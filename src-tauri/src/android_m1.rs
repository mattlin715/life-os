//! Disposable Android M1 app-private persistence façade.
//!
//! This module deliberately reuses the canonical schema-v5 migration and typed
//! Experience writer while refusing every desktop/profile/recovery route.

use crate::schema_v5_fresh_base::EMPTY_V4_BASE_SCHEMA;
use crate::schema_v5_migration::{
    activate_lifecycle_writes, migrate_disposable_v4, source_manifest_for_path,
    verify_any_committed_v5, FailurePoint, MigrationRequest,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{Connection, SqliteConnection};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use time::OffsetDateTime;

const APPLICATION_ID: &str = "com.lifeos.review.m1";
const DATABASE_FILENAME: &str = "android-m1-disposable-v5.db";
const PENDING_FILENAME: &str = ".android-m1-fresh-v5.pending.db";
const LOCALE_PREFERENCE_FILENAME: &str = "android-m1-locale.pref";
const MAX_BODY_BYTES: usize = 64 * 1024;
const MAX_DEBUG_HOLD_MS: u64 = 30_000;
static OPERATION_LOCK: Mutex<()> = Mutex::new(());
static LOCALE_PREFERENCE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct M1StorageStatus {
    state: &'static str,
    reason: Option<String>,
    schema_version: i64,
    application_id: &'static str,
    database_filename: &'static str,
    synthetic_only: bool,
    supported_operations: [&'static str; 3],
    unsupported_operations: [&'static str; 7],
}

impl M1StorageStatus {
    fn ready() -> Self {
        Self {
            state: "ready",
            reason: None,
            schema_version: 5,
            application_id: APPLICATION_ID,
            database_filename: DATABASE_FILENAME,
            synthetic_only: true,
            supported_operations: ["createExperience", "listExperiences", "getExperience"],
            unsupported_operations: [
                "updateExperience",
                "deleteExperience",
                "importExperiences",
                "artifacts",
                "historicalContext",
                "backupRestore",
                "migrationRecovery",
            ],
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct M1SaveReceipt {
    acknowledgement: &'static str,
    entry: crate::schema_v5_migration::runtime::ExperienceRow,
}

fn canonical_now() -> Result<String, String> {
    OffsetDateTime::now_utc()
        .format(time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
        ))
        .map_err(|_| "m1_clock_invalid".into())
}

fn validate_request_id(value: &str) -> Result<(), String> {
    if !(8..=100).contains(&value.len())
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Err("m1_request_id_invalid".into());
    }
    Ok(())
}

fn validate_body(value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err("m1_experience_empty".into());
    }
    if value.len() > MAX_BODY_BYTES {
        return Err("m1_experience_too_large".into());
    }
    Ok(())
}

fn require_identity(app: &AppHandle) -> Result<(), String> {
    if app.config().identifier == APPLICATION_ID {
        Ok(())
    } else {
        Err("m1_application_identity_refused".into())
    }
}

fn storage_paths(app: &AppHandle) -> Result<(PathBuf, PathBuf, PathBuf), String> {
    require_identity(app)?;
    let root = app
        .path()
        .app_data_dir()
        .map_err(|_| "m1_app_private_directory_unavailable")?;
    Ok((
        root.clone(),
        root.join(DATABASE_FILENAME),
        root.join(PENDING_FILENAME),
    ))
}

fn read_locale_preference_at(path: &Path) -> Result<Option<String>, String> {
    let value = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("m1_locale_preference_read_failed".into()),
    };
    Ok(match value.as_str() {
        "en" | "zh-TW" | "ja" => Some(value),
        _ => None,
    })
}

fn write_locale_preference_at(path: &Path, locale: &str) -> Result<(), String> {
    if !matches!(locale, "en" | "zh-TW" | "ja") {
        return Err("m1_locale_preference_invalid".into());
    }
    let root = path
        .parent()
        .ok_or_else(|| "m1_app_private_directory_unavailable".to_string())?;
    fs::create_dir_all(root).map_err(|_| "m1_app_private_directory_create_failed".to_string())?;
    fs::write(path, locale).map_err(|_| "m1_locale_preference_write_failed".to_string())
}

#[tauri::command]
pub(crate) fn m1_get_locale_preference(app: AppHandle) -> Result<Option<String>, String> {
    let _guard = LOCALE_PREFERENCE_LOCK
        .lock()
        .map_err(|_| "m1_locale_preference_lock_poisoned".to_string())?;
    let (root, _, _) = storage_paths(&app)?;
    read_locale_preference_at(&root.join(LOCALE_PREFERENCE_FILENAME))
}

#[tauri::command]
pub(crate) fn m1_set_locale_preference(app: AppHandle, locale: String) -> Result<(), String> {
    let _guard = LOCALE_PREFERENCE_LOCK
        .lock()
        .map_err(|_| "m1_locale_preference_lock_poisoned".to_string())?;
    let (root, _, _) = storage_paths(&app)?;
    write_locale_preference_at(&root.join(LOCALE_PREFERENCE_FILENAME), &locale)
}

async fn create_fresh_exact_v5(root: &Path, live: &Path, pending: &Path) -> Result<(), String> {
    if live.exists() || pending.exists() {
        return Err("m1_fresh_initialization_state_changed".into());
    }
    fs::create_dir_all(root).map_err(|_| "m1_app_private_directory_create_failed")?;
    if live.exists() || pending.exists() {
        return Err("m1_fresh_initialization_state_changed".into());
    }

    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(pending)
        .create_if_missing(true)
        .foreign_keys(true);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|_| "m1_fresh_database_create_failed".to_string())?;
    sqlx::raw_sql(EMPTY_V4_BASE_SCHEMA)
        .execute(&mut connection)
        .await
        .map_err(|_| "m1_fresh_base_schema_failed".to_string())?;
    connection
        .close()
        .await
        .map_err(|_| "m1_fresh_base_close_failed".to_string())?;

    let source_manifest = source_manifest_for_path(pending)
        .await
        .map_err(|error| format!("m1_fresh_source_refused:{}", error.code))?;
    let now = canonical_now()?;
    let receipt = migrate_disposable_v4(MigrationRequest {
        path: pending,
        expected_source_manifest_digest: source_manifest,
        started_at: &now,
        committed_at: &now,
        backup_id: None,
        failure_point: FailurePoint::None,
    })
    .await
    .map_err(|error| format!("m1_fresh_v5_failed:{}", error.code))?;
    activate_lifecycle_writes(pending, &receipt, &now)
        .await
        .map_err(|error| format!("m1_fresh_activation_failed:{}", error.code))?;
    verify_any_committed_v5(pending)
        .await
        .map_err(|error| format!("m1_fresh_verification_failed:{}", error.code))?;

    // Android publication semantics remain prototype evidence, not a desktop
    // replacement/durability guarantee. A failed publish leaves the pending
    // file in place so the next launch blocks rather than recreating it.
    fs::rename(pending, live)
        .map_err(|error| format!("m1_fresh_publish_failed:{}", error.kind()))?;
    verify_any_committed_v5(live)
        .await
        .map_err(|error| format!("m1_published_v5_refused:{}", error.code))?;
    crate::schema_v5_migration::runtime::list_experiences(live)
        .await
        .map_err(|error| format!("m1_activated_v5_refused:{}", error.code))?;
    Ok(())
}

async fn ensure_ready_at(root: &Path, live: &Path, pending: &Path) -> Result<(), String> {
    if pending.exists() {
        return Err("m1_pending_initialization_preserved".into());
    }
    if !live.exists() {
        create_fresh_exact_v5(root, live, pending).await?;
    }
    verify_any_committed_v5(live)
        .await
        .map_err(|error| format!("m1_existing_database_refused:{}", error.code))?;
    crate::schema_v5_migration::runtime::list_experiences(live)
        .await
        .map_err(|error| format!("m1_existing_runtime_refused:{}", error.code))?;
    Ok(())
}

fn with_storage<T>(
    app: &AppHandle,
    operation: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    let _guard = OPERATION_LOCK
        .lock()
        .map_err(|_| "m1_operation_lock_poisoned".to_string())?;
    let (root, live, pending) = storage_paths(app)?;
    tauri::async_runtime::block_on(ensure_ready_at(&root, &live, &pending))?;
    operation(&live)
}

#[tauri::command]
pub(crate) fn m1_storage_status(app: AppHandle) -> Result<M1StorageStatus, String> {
    with_storage(&app, |_| Ok(M1StorageStatus::ready()))
}

fn debug_hold(
    live: &Path,
    phase: Option<&str>,
    expected: &str,
    milliseconds: Option<u64>,
) -> Result<(), String> {
    if phase != Some(expected) {
        return Ok(());
    }
    if !cfg!(debug_assertions) {
        return Err("m1_debug_hold_unavailable".into());
    }
    let milliseconds = milliseconds.unwrap_or(10_000);
    if milliseconds == 0 || milliseconds > MAX_DEBUG_HOLD_MS {
        return Err("m1_debug_hold_invalid".into());
    }
    let marker = live.with_file_name(format!(".m1-{expected}.hold"));
    fs::write(&marker, expected).map_err(|_| "m1_debug_hold_marker_failed".to_string())?;
    std::thread::sleep(Duration::from_millis(milliseconds));
    Ok(())
}

fn save_at(
    live: &Path,
    request_id: &str,
    body: &str,
    debug_phase: Option<&str>,
    debug_hold_ms: Option<u64>,
) -> Result<M1SaveReceipt, String> {
    validate_request_id(request_id)?;
    validate_body(body)?;
    debug_hold(live, debug_phase, "beforeCommit", debug_hold_ms)?;

    tauri::async_runtime::block_on(async {
        if let Some(existing) = crate::schema_v5_migration::runtime::get_experience(live, request_id)
            .await
            .map_err(|error| format!("m1_existing_request_unreadable:{}", error.code))?
        {
            if existing.body == body {
                return Ok(M1SaveReceipt {
                    acknowledgement: "alreadyCommitted",
                    entry: existing,
                });
            }
            return Err("m1_request_id_content_conflict".into());
        }

        let occurred_at = canonical_now()?;
        let guard_token = format!("m1-create-{:x}", Sha256::digest(request_id.as_bytes()));
        let created = match crate::schema_v5_migration::runtime::create_experience(
            live,
            request_id,
            body,
            &occurred_at,
            &guard_token,
        )
        .await
        {
            Ok(created) => created,
            Err(error) => {
                // A concurrent identical request may have committed first.
                if let Ok(Some(existing)) =
                    crate::schema_v5_migration::runtime::get_experience(live, request_id).await
                {
                    if existing.body == body {
                        return Ok(M1SaveReceipt {
                            acknowledgement: "alreadyCommitted",
                            entry: existing,
                        });
                    }
                }
                return Err(format!("m1_create_failed:{}", error.code));
            }
        };
        Ok(M1SaveReceipt {
            acknowledgement: "committed",
            entry: created,
        })
    })
    .and_then(|receipt| {
        debug_hold(
            live,
            debug_phase,
            "afterCommitBeforeAck",
            debug_hold_ms,
        )?;
        Ok(receipt)
    })
}

#[tauri::command]
pub(crate) fn m1_create_experience(
    app: AppHandle,
    request_id: String,
    body: String,
    debug_phase: Option<String>,
    debug_hold_ms: Option<u64>,
) -> Result<M1SaveReceipt, String> {
    with_storage(&app, |live| {
        save_at(
            live,
            &request_id,
            &body,
            debug_phase.as_deref(),
            debug_hold_ms,
        )
    })
}

#[tauri::command]
pub(crate) fn m1_list_experiences(
    app: AppHandle,
) -> Result<Vec<crate::schema_v5_migration::runtime::ExperienceRow>, String> {
    with_storage(&app, |live| {
        tauri::async_runtime::block_on(
            crate::schema_v5_migration::runtime::list_experiences(live),
        )
        .map_err(|error| format!("m1_list_failed:{}", error.code))
    })
}

#[tauri::command]
pub(crate) fn m1_get_experience(
    app: AppHandle,
    id: String,
) -> Result<Option<crate::schema_v5_migration::runtime::ExperienceRow>, String> {
    validate_request_id(&id)?;
    with_storage(&app, |live| {
        tauri::async_runtime::block_on(
            crate::schema_v5_migration::runtime::get_experience(live, &id),
        )
        .map_err(|error| format!("m1_get_failed:{}", error.code))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;
    use std::sync::Arc;
    use std::thread;
    use tempfile::TempDir;

    fn paths(directory: &TempDir) -> (PathBuf, PathBuf, PathBuf) {
        let root = directory.path().to_path_buf();
        let live = root.join(DATABASE_FILENAME);
        let pending = root.join(PENDING_FILENAME);
        (root, live, pending)
    }

    #[test]
    fn locale_preference_is_app_private_non_content_and_bounded() {
        let directory = TempDir::new().unwrap();
        let path = directory.path().join(LOCALE_PREFERENCE_FILENAME);
        assert_eq!(read_locale_preference_at(&path).unwrap(), None);
        write_locale_preference_at(&path, "zh-TW").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "zh-TW");
        assert_eq!(read_locale_preference_at(&path).unwrap().as_deref(), Some("zh-TW"));
        assert_eq!(
            write_locale_preference_at(&path, "synthetic Experience text").unwrap_err(),
            "m1_locale_preference_invalid"
        );
        fs::write(&path, "invalid").unwrap();
        assert_eq!(read_locale_preference_at(&path).unwrap(), None);
    }

    #[test]
    fn fresh_exact_v5_reopens_and_preserves_cjk_with_idempotent_request() {
        let directory = TempDir::new().unwrap();
        let (root, live, pending) = paths(&directory);
        tauri::async_runtime::block_on(ensure_ready_at(&root, &live, &pending)).unwrap();
        let body = "今天我想記住：静かな勇気。\nExact spacing stays.";
        let first = save_at(&live, "request-cjk-0001", body, None, None).unwrap();
        assert_eq!(first.acknowledgement, "committed");
        let repeated = save_at(&live, "request-cjk-0001", body, None, None).unwrap();
        assert_eq!(repeated.acknowledgement, "alreadyCommitted");
        assert_eq!(
            tauri::async_runtime::block_on(
                crate::schema_v5_migration::runtime::get_experience(&live, "request-cjk-0001")
            ).unwrap().unwrap().body,
            body
        );
        tauri::async_runtime::block_on(ensure_ready_at(&root, &live, &pending)).unwrap();
        assert_eq!(
            tauri::async_runtime::block_on(
                crate::schema_v5_migration::runtime::list_experiences(&live)
            ).unwrap().len(),
            1
        );
    }

    #[tokio::test]
    async fn pending_malformed_and_newer_states_are_preserved_and_refused() {
        let pending_case = TempDir::new().unwrap();
        let (root, live, pending) = paths(&pending_case);
        fs::write(&pending, b"preserve me").unwrap();
        assert_eq!(
            ensure_ready_at(&root, &live, &pending).await.unwrap_err(),
            "m1_pending_initialization_preserved"
        );
        assert_eq!(fs::read(&pending).unwrap(), b"preserve me");

        let malformed_case = TempDir::new().unwrap();
        let (root, live, pending) = paths(&malformed_case);
        fs::write(&live, b"not sqlite").unwrap();
        assert!(ensure_ready_at(&root, &live, &pending)
            .await
            .unwrap_err()
            .starts_with("m1_existing_database_refused:"));
        assert_eq!(fs::read(&live).unwrap(), b"not sqlite");

        let newer_case = TempDir::new().unwrap();
        let (root, live, pending) = paths(&newer_case);
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&live)
            .create_if_missing(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        sqlx::query("PRAGMA user_version = 6")
            .execute(&mut connection)
            .await
            .unwrap();
        connection.close().await.unwrap();
        let error = ensure_ready_at(&root, &live, &pending).await.unwrap_err();
        assert!(error.contains("refused"));
        let mut connection = SqliteConnection::connect(&format!("sqlite://{}", live.display()))
            .await
            .unwrap();
        let version = sqlx::query("PRAGMA user_version")
            .fetch_one(&mut connection)
            .await
            .unwrap()
            .get::<i64, _>(0);
        assert_eq!(version, 6);
    }

    #[test]
    fn concurrent_initialization_serializes_to_one_exact_database() {
        let directory = Arc::new(TempDir::new().unwrap());
        let mut threads = Vec::new();
        for _ in 0..2 {
            let directory = directory.clone();
            threads.push(thread::spawn(move || {
                let _guard = OPERATION_LOCK.lock().unwrap();
                let (root, live, pending) = paths(&directory);
                tauri::async_runtime::block_on(ensure_ready_at(&root, &live, &pending))
            }));
        }
        for handle in threads {
            handle.join().unwrap().unwrap();
        }
        let (_, live, _) = paths(&directory);
        tauri::async_runtime::block_on(verify_any_committed_v5(&live)).unwrap();
    }

    #[tokio::test]
    async fn interrupted_schema_transaction_rolls_back_without_claiming_v5() {
        let directory = TempDir::new().unwrap();
        let staging = directory.path().join("interrupted.db");
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&staging)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        sqlx::raw_sql(EMPTY_V4_BASE_SCHEMA)
            .execute(&mut connection)
            .await
            .unwrap();
        connection.close().await.unwrap();
        let source = source_manifest_for_path(&staging).await.unwrap();
        let error = migrate_disposable_v4(MigrationRequest {
            path: &staging,
            expected_source_manifest_digest: source,
            started_at: "2026-09-22T00:00:00.000Z",
            committed_at: "2026-09-22T00:00:00.000Z",
            backup_id: None,
            failure_point: FailurePoint::AfterDatabaseContract,
        })
        .await
        .unwrap_err();
        assert!(error.code.contains("injected"));
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&staging)
            .read_only(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        let version: i64 = sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&mut connection)
            .await
            .unwrap();
        assert_eq!(version, 4);
        let v5_objects: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE name = 'database_contract'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(v5_objects, 0);
    }

    #[tokio::test]
    async fn database_open_failure_is_truthful_and_non_destructive() {
        let directory = TempDir::new().unwrap();
        let (root, live, pending) = paths(&directory);
        fs::create_dir(&live).unwrap();
        let error = ensure_ready_at(&root, &live, &pending).await.unwrap_err();
        assert!(error.starts_with("m1_existing_database_refused:"));
        assert!(live.is_dir());
    }
}
