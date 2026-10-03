//! Synthetic-only M2-B lifecycle. Existing direct initializer and canonical
//! Experience writer remain authoritative; acknowledgements contain no text.
use crate::android_m2a::with_m2b_storage;
use crate::schema_v5_migration::runtime as runtime;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, time::Duration};
use tauri::AppHandle;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LifecycleRequest {
    pub(crate) request_id: String,
    pub(crate) operation: String,
    pub(crate) id: String,
    pub(crate) expected_revision_id: String,
    pub(crate) occurred_at: String,
    pub(crate) body: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MutationAcknowledgement {
    acknowledgement: &'static str,
    request_id: String,
    source_id: String,
}

fn identifier(value: &str) -> Result<(), String> {
    if !(8..=100).contains(&value.len()) || !value.bytes().all(|b|
        b.is_ascii_alphanumeric() || b == b'-' || b == b'_') {
        return Err("m2b_identifier_invalid".into());
    }
    Ok(())
}

fn body(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 64 * 1024 {
        return Err("m2b_body_invalid".into());
    }
    Ok(())
}

fn request_identity(request: &LifecycleRequest) -> Result<String, String> {
    let digest = request.body.as_ref().map(|v| format!("{:x}", Sha256::digest(v.as_bytes())))
        .unwrap_or_default();
    let encoded = serde_json::to_vec(&[
        "life-os/m2b-request-v1", &request.operation, &request.id,
        &request.expected_revision_id, &request.occurred_at, &digest,
    ]).map_err(|_| "m2b_request_encoding_failed")?;
    Ok(format!("m2b_{:x}", Sha256::digest(encoded)))
}

fn validate(request: &LifecycleRequest) -> Result<(), String> {
    identifier(&request.id)?;
    identifier(&request.expected_revision_id)?;
    // The canonical writer also validates actual date/time and monotonicity.
    if request.occurred_at.len() != 24 || !request.occurred_at.is_ascii() {
        return Err("m2b_timestamp_invalid".into());
    }
    match (request.operation.as_str(), &request.body) {
        ("update", Some(text)) => body(text)?,
        ("delete", None) => (),
        _ => return Err("m2b_operation_invalid".into()),
    }
    if request.request_id != request_identity(request)? {
        return Err("m2b_request_identity_conflict".into());
    }
    Ok(())
}

fn sanitized_failure(code: &str) -> String {
    // Never pass SQL error details, caller text or raw paths to UI/logs.
    if code.contains("outcome_unknown_unchanged") {
        "m2b_outcome_unchanged_same_request_only"
    } else if code.contains("definitely_not_committed") || code.starts_with("injected_") {
        "m2b_not_committed"
    } else if code == "m2b_unexpected_dependency_preserved" {
        "m2b_unexpected_dependency_preserved"
    } else { "m2b_lifecycle_refused_preserved" }.into()
}

fn hold(path: &Path, phase: Option<&str>, target: &str) -> Result<(), String> {
    if phase != Some(target) { return Ok(()) }
    if !cfg!(debug_assertions) { return Err("m2b_debug_unavailable".into()) }
    fs::write(path.with_file_name(format!(".m2b-{target}.hold")), target)
        .map_err(|_| "m2b_debug_marker_failed")?;
    std::thread::sleep(Duration::from_secs(30));
    Ok(())
}

fn mutate_at(path: &Path, request: &LifecycleRequest, debug_phase: Option<&str>)
    -> Result<MutationAcknowledgement, String> {
    validate(request)?;
    if debug_phase.is_some() && (!cfg!(debug_assertions) || !matches!(debug_phase,
        Some("beforeCommit" | "afterCommitBeforeAck" | "rollbackAfterProjection"))) {
        return Err("m2b_debug_phase_invalid".into());
    }
    hold(path, debug_phase, "beforeCommit")?;
    let acknowledgement = tauri::async_runtime::block_on(async {
        if let Some(state) = runtime::reconcile_direct_mutation(path, &request.id,
            &request.expected_revision_id, request.body.as_deref(), &request.occurred_at)
            .await.map_err(|e| sanitized_failure(&e.code))? {
            return Ok::<&'static str, String>(state);
        }
        let current = runtime::direct_source_snapshot(path, &request.id).await
            .map_err(|e| sanitized_failure(&e.code))?;
        if current.as_ref().map(|s| s.revision_id.as_str()) != Some(&request.expected_revision_id) {
            return Err("m2b_stale_revision_preserved".into());
        }
        runtime::refuse_non_synthetic_dependencies(path).await
            .map_err(|e| sanitized_failure(&e.code))?;
        let committed = runtime::mutate_experience_direct_fresh(path, &request.id,
            &request.expected_revision_id, request.body.as_deref(), &request.occurred_at,
            &request.request_id, debug_phase == Some("rollbackAfterProjection"))
            .await.map_err(|e| sanitized_failure(&e.code))?;
        if !committed { return Err("m2b_stale_revision_preserved".into()) }
        let state = runtime::reconcile_direct_mutation(path, &request.id,
            &request.expected_revision_id, request.body.as_deref(), &request.occurred_at)
            .await.map_err(|e| sanitized_failure(&e.code))?;
        if state != Some("alreadyCommitted") { return Err("m2b_postcommit_unverified_preserved".into()) }
        // Verified runtime read after canonical transaction verification.
        let actual = runtime::direct_source_snapshot(path, &request.id).await
            .map_err(|e| sanitized_failure(&e.code))?;
        if let Some(expected) = request.body.as_deref() {
            let Some(actual) = actual else { return Err("m2b_postcommit_unverified_preserved".into()) };
            if actual.entry.body != expected || actual.predecessor_revision_id.as_deref() != Some(&request.expected_revision_id)
                || actual.entry.updated_at != request.occurred_at || actual.authorship != "user" {
                return Err("m2b_postcommit_unverified_preserved".into());
            }
        } else if actual.is_some() { return Err("m2b_postdelete_unverified_preserved".into()) }
        Ok("committed")
    })?;
    hold(path, debug_phase, "afterCommitBeforeAck")?;
    Ok(MutationAcknowledgement { acknowledgement, request_id: request.request_id.clone(), source_id: request.id.clone() })
}

#[tauri::command]
pub(crate) fn m2b_storage_status(app: AppHandle) -> Result<serde_json::Value, String> {
    with_m2b_storage(&app, |_| Ok(serde_json::json!({
        "state":"ready", "schemaVersion":5, "applicationId":"com.lifeos.review.m2b",
        "databaseFilename":"android-m2b-disposable-v5.db",
        "receiptFilename":"android-m2b-direct-fresh-v5.receipt.json",
        "initializationOrigin":"directFreshV5", "syntheticOnly":true,
        "supportedOperations":["createExperience","listExperiences","getExperience","updateExperience","deleteExperience"],
        "unsupportedOperations":["artifacts","AI","importExport","backupRestore","migrationRecovery","sync"]
    })))
}

#[tauri::command]
pub(crate) fn m2b_create_experience(app: AppHandle, id: String, text: String) -> Result<(), String> {
    identifier(&id)?; body(&text)?;
    with_m2b_storage(&app, |path| tauri::async_runtime::block_on(async {
        runtime::refuse_non_synthetic_dependencies(path).await.map_err(|e| sanitized_failure(&e.code))?;
        if let Some(current) = runtime::get_experience_direct_fresh(path, &id).await.map_err(|e| sanitized_failure(&e.code))? {
            if current.body == text { return Ok(()) }
            return Err("m2b_create_identity_conflict".into());
        }
        let now = time::OffsetDateTime::now_utc().format(time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"))
            .map_err(|_| "m2b_clock_invalid")?;
        runtime::create_experience_direct_fresh(path, &id, &text, &now,
            &format!("m2b-create-{:x}", Sha256::digest(id.as_bytes())))
            .await.map_err(|e| sanitized_failure(&e.code))?;
        Ok(())
    }))
}

#[tauri::command]
pub(crate) fn m2b_list_experiences(app: AppHandle) -> Result<Vec<runtime::ExperienceRow>, String> {
    with_m2b_storage(&app, |path| tauri::async_runtime::block_on(runtime::list_experiences_direct_fresh(path))
        .map_err(|e| sanitized_failure(&e.code)))
}

#[tauri::command]
pub(crate) fn m2b_get_experience(app: AppHandle, id: String) -> Result<Option<runtime::DirectSourceSnapshot>, String> {
    identifier(&id)?;
    with_m2b_storage(&app, |path| tauri::async_runtime::block_on(runtime::direct_source_snapshot(path, &id))
        .map_err(|e| sanitized_failure(&e.code)))
}

#[tauri::command]
pub(crate) fn m2b_mutate_experience(app: AppHandle, request: LifecycleRequest, debug_phase: Option<String>)
    -> Result<MutationAcknowledgement, String> {
    with_m2b_storage(&app, |path| mutate_at(path, &request, debug_phase.as_deref()))
}

#[tauri::command]
pub(crate) fn m2b_get_locale_preference(app: AppHandle) -> Result<Option<String>, String> {
    with_m2b_storage(&app, |path| {
        let preference = path.with_file_name("android-m2b-locale.pref");
        if !preference.exists() { return Ok(None) }
        let value = fs::read_to_string(preference).map_err(|_| "m2b_locale_unreadable")?;
        if !matches!(value.as_str(), "en" | "zh-TW" | "ja") { return Err("m2b_locale_invalid".into()) }
        Ok(Some(value))
    })
}

#[tauri::command]
pub(crate) fn m2b_set_locale_preference(app: AppHandle, locale: String) -> Result<(), String> {
    if !matches!(locale.as_str(), "en" | "zh-TW" | "ja") { return Err("m2b_locale_invalid".into()) }
    with_m2b_storage(&app, |path| {
        use std::io::Write;
        let mut file = fs::File::create(path.with_file_name("android-m2b-locale.pref"))
            .map_err(|_| "m2b_locale_write_failed")?;
        file.write_all(locale.as_bytes()).map_err(|_| "m2b_locale_write_failed")?;
        file.sync_all().map_err(|_| "m2b_locale_sync_failed".to_string())
            .map(|_| ())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Connection, SqliteConnection};

    fn fixture() -> (tempfile::TempDir, std::path::PathBuf) {
        let root = tempfile::tempdir().unwrap();
        let path = tauri::async_runtime::block_on(crate::android_m2a::prepare_m2b_fixture(root.path().join("owned"))).unwrap();
        tauri::async_runtime::block_on(runtime::create_experience_direct_fresh(&path, "m2b-fixture-001",
            "  合成テスト\n第一行  ", "2026-10-03T00:00:00.000Z", "m2b-fixture-create-guard-000000000000")).unwrap();
        (root, path)
    }

    fn snapshot(path: &Path) -> runtime::DirectSourceSnapshot {
        tauri::async_runtime::block_on(runtime::direct_source_snapshot(path, "m2b-fixture-001")).unwrap().unwrap()
    }

    fn request(path: &Path, text: Option<&str>, time: &str) -> LifecycleRequest {
        let mut request = LifecycleRequest { request_id: String::new(), operation: if text.is_some() { "update" } else { "delete" }.into(),
            id: "m2b-fixture-001".into(), expected_revision_id: snapshot(path).revision_id,
            occurred_at: time.into(), body: text.map(str::to_string) };
        request.request_id = request_identity(&request).unwrap(); request
    }

    fn scalar(path: &Path, query: &str) -> i64 {
        tauri::async_runtime::block_on(async {
            let mut connection = SqliteConnection::connect(&format!("sqlite:{}?mode=ro", path.display())).await.unwrap();
            sqlx::query_scalar(query).fetch_one(&mut connection).await.unwrap()
        })
    }

    #[test]
    fn exact_cjk_revision_identity_predecessor_and_same_text_contract() {
        let (_root, path) = fixture(); let prior = snapshot(&path);
        let text = "  今天，静かな勇気。\n第二行\n  ";
        let update = request(&path, Some(text), "2026-10-03T00:00:01.000Z");
        assert_eq!(mutate_at(&path, &update, None).unwrap().acknowledgement, "committed");
        let current = snapshot(&path);
        assert_eq!(current.entry.id, prior.entry.id); assert_eq!(current.entry.created_at, prior.entry.created_at);
        assert_eq!(current.entry.body, text); assert_eq!(current.revision_number, 2);
        assert_eq!(current.predecessor_revision_id.as_deref(), Some(prior.revision_id.as_str())); assert_eq!(current.authorship, "user");
        assert_eq!(mutate_at(&path, &update, None).unwrap().acknowledgement, "alreadyCommitted");
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revisions"), 2);
        let same = request(&path, Some(text), "2026-10-03T00:00:02.000Z"); mutate_at(&path, &same, None).unwrap();
        assert_eq!(snapshot(&path).revision_number, 3);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revision_content"), 3);
        assert_eq!(mutate_at(&path, &update, None).unwrap().acknowledgement, "committedNotCurrent");
        tauri::async_runtime::block_on(crate::android_m2a::prepare_m2b_fixture(path.parent().unwrap().to_path_buf())).unwrap();
        assert_eq!(snapshot(&path).entry.body, text);
    }

    #[test]
    fn two_editors_duplicate_identity_conflict_and_edit_delete_race() {
        let (_root, path) = fixture();
        let first = request(&path, Some("first corrected"), "2026-10-03T00:00:01.000Z");
        let second = request(&path, Some("second stale"), "2026-10-03T00:00:02.000Z");
        let deletion = request(&path, None, "2026-10-03T00:00:03.000Z");
        mutate_at(&path, &first, None).unwrap();
        assert_eq!(mutate_at(&path, &second, None).unwrap_err(), "m2b_stale_revision_preserved");
        assert_eq!(mutate_at(&path, &deletion, None).unwrap_err(), "m2b_stale_revision_preserved");
        let mut conflicting = first.clone(); conflicting.body = Some("other".into());
        assert_eq!(mutate_at(&path, &conflicting, None).unwrap_err(), "m2b_request_identity_conflict");
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revisions"), 2);
        let deletion = request(&path, None, "2026-10-03T00:00:04.000Z"); mutate_at(&path, &deletion, None).unwrap();
        assert_eq!(mutate_at(&path, &second, None).unwrap_err(), "m2b_stale_revision_preserved");
        assert_eq!(mutate_at(&path, &first, None).unwrap().acknowledgement, "committedNotCurrent");
        assert_eq!(mutate_at(&path, &deletion, None).unwrap().acknowledgement, "alreadyCommitted");
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revision_content"), 0);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM experience_entries"), 0);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revisions"), 2);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_heads WHERE lifecycle_state='deleted' AND current_revision_id IS NULL"), 1);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revision_provenance"), 2);
        tauri::async_runtime::block_on(crate::android_m2a::prepare_m2b_fixture(path.parent().unwrap().to_path_buf())).unwrap();
        assert!(tauri::async_runtime::block_on(runtime::get_experience_direct_fresh(&path, "m2b-fixture-001")).unwrap().is_none());
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM schema_migration_receipts"), 0);
        let acknowledgement = serde_json::to_string(&mutate_at(&path, &deletion, None).unwrap()).unwrap();
        assert!(!acknowledgement.contains("corrected") && !acknowledgement.contains("合成"));
    }

    #[test]
    fn update_and_delete_rollback_preserve_exact_prior_state() {
        for text in [Some("rollback text"), None] {
            let (_root, path) = fixture(); let prior = snapshot(&path);
            let command = request(&path, text, "2026-10-03T00:00:01.000Z");
            assert_eq!(mutate_at(&path, &command, Some("rollbackAfterProjection")).unwrap_err(), "m2b_not_committed");
            assert_eq!(snapshot(&path).revision_id, prior.revision_id);
            assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revisions"), 1);
            assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revision_content"), 1);
            assert_eq!(scalar(&path, "SELECT COUNT(*) FROM v5_compatibility_write_guard"), 0);
            assert!(tauri::async_runtime::block_on(runtime::reconcile_direct_mutation(&path,
                &command.id, &command.expected_revision_id, command.body.as_deref(), &command.occurred_at)).unwrap().is_none());
            mutate_at(&path, &command, None).unwrap();
        }
    }

    #[test]
    fn dependency_inventory_is_read_only_and_error_text_is_content_free() {
        let (_root, path) = fixture();
        let command = request(&path, Some("secret-canary"), "2026-10-03T00:00:01.000Z");
        tauri::async_runtime::block_on(async {
            let mut connection = SqliteConnection::connect(&format!("sqlite:{}", path.display())).await.unwrap();
            sqlx::query("INSERT INTO v5_compatibility_write_guard(token,created_at) VALUES ('m2b-synthetic-fixture-guard-000000000000','2026-10-03T00:00:00.000Z')").execute(&mut connection).await.unwrap();
            sqlx::query("INSERT INTO persisted_artifacts(id,source_entry_id,artifact_kind,payload,created_at,updated_at) VALUES ('unexpected-artifact','m2b-fixture-001','evidence','unexpected-raw-content','2026-10-03T00:00:00.000Z','2026-10-03T00:00:00.000Z')")
                .execute(&mut connection).await.unwrap();
            sqlx::query("DELETE FROM v5_compatibility_write_guard").execute(&mut connection).await.unwrap();
        });
        assert_eq!(tauri::async_runtime::block_on(runtime::refuse_non_synthetic_dependencies(&path)).unwrap_err().code, "m2b_unexpected_dependency_preserved");
        assert!(mutate_at(&path, &command, None).is_err());
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM source_revisions"), 1);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM persisted_artifacts"), 1);
        assert_eq!(sanitized_failure("sql failure:secret-canary"), "m2b_lifecycle_refused_preserved");
    }
}
