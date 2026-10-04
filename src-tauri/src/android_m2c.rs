//! Synthetic-only M2-C lifecycle. Existing direct initializer and canonical
//! Experience writer remain authoritative; acknowledgements contain no text.
use crate::android_m2a::with_m2c_storage;
use crate::schema_v5_migration::runtime;
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
    if !(8..=100).contains(&value.len())
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err("m2c_identifier_invalid".into());
    }
    Ok(())
}

fn body(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.len() > 64 * 1024 {
        return Err("m2c_body_invalid".into());
    }
    Ok(())
}

fn request_identity(request: &LifecycleRequest) -> Result<String, String> {
    let digest = request
        .body
        .as_ref()
        .map(|v| format!("{:x}", Sha256::digest(v.as_bytes())))
        .unwrap_or_default();
    let encoded = serde_json::to_vec(&[
        "life-os/m2c-request-v1",
        &request.operation,
        &request.id,
        &request.expected_revision_id,
        &request.occurred_at,
        &digest,
    ])
    .map_err(|_| "m2c_request_encoding_failed")?;
    Ok(format!("m2c_{:x}", Sha256::digest(encoded)))
}

fn validate(request: &LifecycleRequest) -> Result<(), String> {
    identifier(&request.id)?;
    identifier(&request.expected_revision_id)?;
    // The canonical writer also validates actual date/time and monotonicity.
    if request.occurred_at.len() != 24 || !request.occurred_at.is_ascii() {
        return Err("m2c_timestamp_invalid".into());
    }
    match (request.operation.as_str(), &request.body) {
        ("update", Some(text)) => body(text)?,
        ("delete", None) => (),
        _ => return Err("m2c_operation_invalid".into()),
    }
    if request.request_id != request_identity(request)? {
        return Err("m2c_request_identity_conflict".into());
    }
    Ok(())
}

fn sanitized_failure(code: &str) -> String {
    // Never pass SQL error details, caller text or raw paths to UI/logs.
    if code.contains("outcome_unknown_unchanged") {
        "m2c_outcome_unchanged_same_request_only"
    } else if code.contains("definitely_not_committed") || code.starts_with("injected_") {
        "m2c_not_committed"
    } else if code == "m2c_unexpected_dependency_preserved" {
        "m2c_unexpected_dependency_preserved"
    } else {
        "m2c_lifecycle_refused_preserved"
    }
    .into()
}

fn hold(path: &Path, phase: Option<&str>, target: &str) -> Result<(), String> {
    if phase != Some(target) {
        return Ok(());
    }
    if !cfg!(debug_assertions) {
        return Err("m2c_debug_unavailable".into());
    }
    fs::write(path.with_file_name(format!(".m2c-{target}.hold")), target)
        .map_err(|_| "m2c_debug_marker_failed")?;
    std::thread::sleep(Duration::from_secs(30));
    Ok(())
}

fn mutate_at(
    path: &Path,
    request: &LifecycleRequest,
    debug_phase: Option<&str>,
) -> Result<MutationAcknowledgement, String> {
    validate(request)?;
    if debug_phase.is_some()
        && (!cfg!(debug_assertions)
            || !matches!(
                debug_phase,
                Some("beforeCommit" | "afterCommitBeforeAck" | "rollbackAfterProjection")
            ))
    {
        return Err("m2c_debug_phase_invalid".into());
    }
    hold(path, debug_phase, "beforeCommit")?;
    let acknowledgement = tauri::async_runtime::block_on(async {
        if let Some(state) = runtime::reconcile_direct_mutation(
            path,
            &request.id,
            &request.expected_revision_id,
            request.body.as_deref(),
            &request.occurred_at,
        )
        .await
        .map_err(|e| sanitized_failure(&e.code))?
        {
            return Ok::<&'static str, String>(state);
        }
        let current = runtime::direct_source_snapshot(path, &request.id)
            .await
            .map_err(|e| sanitized_failure(&e.code))?;
        if current.as_ref().map(|s| s.revision_id.as_str()) != Some(&request.expected_revision_id) {
            return Err("m2c_stale_revision_preserved".into());
        }
        crate::schema_v5_migration::android_reflection::verify_scope(path)
            .await
            .map_err(|e| sanitized_failure(&e.code))?;
        let committed = runtime::mutate_experience_direct_fresh(
            path,
            &request.id,
            &request.expected_revision_id,
            request.body.as_deref(),
            &request.occurred_at,
            &request.request_id,
            debug_phase == Some("rollbackAfterProjection"),
        )
        .await
        .map_err(|e| sanitized_failure(&e.code))?;
        if !committed {
            return Err("m2c_stale_revision_preserved".into());
        }
        let state = runtime::reconcile_direct_mutation(
            path,
            &request.id,
            &request.expected_revision_id,
            request.body.as_deref(),
            &request.occurred_at,
        )
        .await
        .map_err(|e| sanitized_failure(&e.code))?;
        if state != Some("alreadyCommitted") {
            return Err("m2c_postcommit_unverified_preserved".into());
        }
        // Verified runtime read after canonical transaction verification.
        let actual = runtime::direct_source_snapshot(path, &request.id)
            .await
            .map_err(|e| sanitized_failure(&e.code))?;
        if let Some(expected) = request.body.as_deref() {
            let Some(actual) = actual else {
                return Err("m2c_postcommit_unverified_preserved".into());
            };
            if actual.entry.body != expected
                || actual.predecessor_revision_id.as_deref() != Some(&request.expected_revision_id)
                || actual.entry.updated_at != request.occurred_at
                || actual.authorship != "user"
            {
                return Err("m2c_postcommit_unverified_preserved".into());
            }
        } else if actual.is_some() {
            return Err("m2c_postdelete_unverified_preserved".into());
        }
        Ok("committed")
    })?;
    hold(path, debug_phase, "afterCommitBeforeAck")?;
    Ok(MutationAcknowledgement {
        acknowledgement,
        request_id: request.request_id.clone(),
        source_id: request.id.clone(),
    })
}

#[tauri::command]
pub(crate) fn m2c_storage_status(app: AppHandle) -> Result<serde_json::Value, String> {
    with_m2c_storage(&app, |_| {
        Ok(serde_json::json!({
            "state":"ready", "schemaVersion":5, "applicationId":"com.lifeos.review.m2c",
            "databaseFilename":"android-m2c-disposable-v5.db",
            "receiptFilename":"android-m2c-direct-fresh-v5.receipt.json",
            "initializationOrigin":"directFreshV5", "syntheticOnly":true,
            "supportedOperations":["createExperience","listExperiences","getExperience","updateExperience","deleteExperience"],
            "unsupportedOperations":["provider","history","Pattern","importExport","backupRestore","migrationRecovery","sync"]
        }))
    })
}

#[tauri::command]
pub(crate) fn m2c_create_experience(
    app: AppHandle,
    id: String,
    text: String,
) -> Result<(), String> {
    identifier(&id)?;
    body(&text)?;
    with_m2c_storage(&app, |path| {
        tauri::async_runtime::block_on(async {
            crate::schema_v5_migration::android_reflection::verify_scope(path)
                .await
                .map_err(|e| sanitized_failure(&e.code))?;
            if let Some(current) = runtime::get_experience_direct_fresh(path, &id)
                .await
                .map_err(|e| sanitized_failure(&e.code))?
            {
                if current.body == text {
                    return Ok(());
                }
                return Err("m2c_create_identity_conflict".into());
            }
            let now = time::OffsetDateTime::now_utc()
                .format(time::macros::format_description!(
                    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
                ))
                .map_err(|_| "m2c_clock_invalid")?;
            runtime::create_experience_direct_fresh(
                path,
                &id,
                &text,
                &now,
                &format!("m2c-create-{:x}", Sha256::digest(id.as_bytes())),
            )
            .await
            .map_err(|e| sanitized_failure(&e.code))?;
            Ok(())
        })
    })
}

#[tauri::command]
pub(crate) fn m2c_list_experiences(app: AppHandle) -> Result<Vec<runtime::ExperienceRow>, String> {
    with_m2c_storage(&app, |path| {
        tauri::async_runtime::block_on(runtime::list_experiences_direct_fresh(path))
            .map_err(|e| sanitized_failure(&e.code))
    })
}

#[tauri::command]
pub(crate) fn m2c_get_experience(
    app: AppHandle,
    id: String,
) -> Result<Option<runtime::DirectSourceSnapshot>, String> {
    identifier(&id)?;
    with_m2c_storage(&app, |path| {
        tauri::async_runtime::block_on(runtime::direct_source_snapshot(path, &id))
            .map_err(|e| sanitized_failure(&e.code))
    })
}

#[tauri::command]
pub(crate) fn m2c_mutate_experience(
    app: AppHandle,
    request: LifecycleRequest,
    debug_phase: Option<String>,
) -> Result<MutationAcknowledgement, String> {
    with_m2c_storage(&app, |path| {
        mutate_at(path, &request, debug_phase.as_deref())
    })
}

#[tauri::command]
pub(crate) fn m2c_get_locale_preference(app: AppHandle) -> Result<Option<String>, String> {
    with_m2c_storage(&app, |path| {
        let preference = path.with_file_name("android-m2c-locale.pref");
        if !preference.exists() {
            return Ok(None);
        }
        let value = fs::read_to_string(preference).map_err(|_| "m2c_locale_unreadable")?;
        if !matches!(value.as_str(), "en" | "zh-TW" | "ja") {
            return Err("m2c_locale_invalid".into());
        }
        Ok(Some(value))
    })
}

#[tauri::command]
pub(crate) fn m2c_set_locale_preference(app: AppHandle, locale: String) -> Result<(), String> {
    if !matches!(locale.as_str(), "en" | "zh-TW" | "ja") {
        return Err("m2c_locale_invalid".into());
    }
    with_m2c_storage(&app, |path| {
        use std::io::Write;
        let mut file = fs::File::create(path.with_file_name("android-m2c-locale.pref"))
            .map_err(|_| "m2c_locale_write_failed")?;
        file.write_all(locale.as_bytes())
            .map_err(|_| "m2c_locale_write_failed")?;
        file.sync_all()
            .map_err(|_| "m2c_locale_sync_failed".to_string())
            .map(|_| ())
    })
}

#[tauri::command]
pub(crate) fn m2c_artifact_snapshot(
    app: AppHandle,
    id: String,
) -> Result<serde_json::Value, String> {
    identifier(&id)?;
    with_m2c_storage(&app, |path| {
        tauri::async_runtime::block_on(crate::schema_v5_migration::android_reflection::snapshot(
            path, &id,
        ))
        .map_err(|_| "m2c_artifact_read_failed_preserved".into())
    })
}

#[tauri::command]
pub(crate) fn m2c_mutate_artifact(
    app: AppHandle,
    request: crate::schema_v5_migration::android_reflection::ArtifactRequest,
    debug_phase: Option<String>,
) -> Result<serde_json::Value, String> {
    if debug_phase.is_some() && !cfg!(debug_assertions) {
        return Err("m2c_debug_unavailable".into());
    }
    if debug_phase.as_deref().is_some_and(|v| {
        !matches!(
            v,
            "beforeCommit" | "afterCommitBeforeAck" | "rollbackAfterProjection"
        )
    }) {
        return Err("m2c_debug_phase_invalid".into());
    }
    with_m2c_storage(&app, |path| {
        hold(path, debug_phase.as_deref(), "beforeCommit")?;
        let outcome =
            tauri::async_runtime::block_on(crate::schema_v5_migration::android_reflection::mutate(
                path,
                &request,
                debug_phase.as_deref() == Some("rollbackAfterProjection"),
            ))
            .map_err(|error| {
                if error.code.contains("stale") {
                    "m2c_stale_revision_preserved".to_string()
                } else if error.code.contains("request_identity") {
                    "m2c_request_identity_conflict".into()
                } else {
                    "m2c_artifact_outcome_unconfirmed_preserved".into()
                }
            })?;
        hold(path, debug_phase.as_deref(), "afterCommitBeforeAck")?;
        Ok(outcome)
    })
}
