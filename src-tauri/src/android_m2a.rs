//! Disposable Android M2-A app-private persistence façade.
//!
//! This module deliberately uses the canonical direct schema-v5 initializer
//! and typed Experience writer while refusing every migration, desktop,
//! profile, recovery, provider, and network route.

use crate::schema_v5_migration::direct_init::{
    initialize_direct_fresh_v5, verify_direct_fresh_v5, DirectFreshV5Receipt,
    DirectInitFailurePoint, DirectInitRequest,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use time::OffsetDateTime;

const APPLICATION_ID: &str = "com.lifeos.review.m2a";
const DATABASE_FILENAME: &str = "android-m2a-disposable-v5.db";
const PENDING_FILENAME: &str = ".android-m2a-fresh-v5.pending.db";
const RECEIPT_FILENAME: &str = "android-m2a-direct-fresh-v5.receipt.json";
const PENDING_RECEIPT_FILENAME: &str = ".android-m2a-direct-fresh-v5.pending.receipt.json";
const LOCALE_PREFERENCE_FILENAME: &str = "android-m2a-locale.pref";
const MAX_BODY_BYTES: usize = 64 * 1024;
const MAX_DEBUG_HOLD_MS: u64 = 30_000;
static OPERATION_LOCK: Mutex<()> = Mutex::new(());
static LOCALE_PREFERENCE_LOCK: Mutex<()> = Mutex::new(());
static RETIREMENT_DURABILITY_UNCERTAIN: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct M2AStorageStatus {
    state: &'static str,
    reason: Option<String>,
    schema_version: i64,
    application_id: &'static str,
    database_filename: &'static str,
    receipt_filename: &'static str,
    initialization_origin: &'static str,
    synthetic_only: bool,
    supported_operations: [&'static str; 3],
    unsupported_operations: [&'static str; 7],
}

impl M2AStorageStatus {
    fn ready() -> Self {
        Self {
            state: "ready",
            reason: None,
            schema_version: 5,
            application_id: APPLICATION_ID,
            database_filename: DATABASE_FILENAME,
            receipt_filename: RECEIPT_FILENAME,
            initialization_origin: "directFreshV5",
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
pub(crate) struct M2ASaveReceipt {
    acknowledgement: &'static str,
    entry: crate::schema_v5_migration::runtime::ExperienceRow,
}

fn canonical_now() -> Result<String, String> {
    OffsetDateTime::now_utc()
        .format(time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
        ))
        .map_err(|_| "m2a_clock_invalid".into())
}

fn validate_request_id(value: &str) -> Result<(), String> {
    if !(8..=100).contains(&value.len())
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Err("m2a_request_id_invalid".into());
    }
    Ok(())
}

fn validate_body(value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err("m2a_experience_empty".into());
    }
    if value.len() > MAX_BODY_BYTES {
        return Err("m2a_experience_too_large".into());
    }
    Ok(())
}

fn require_identity(app: &AppHandle) -> Result<(), String> {
    if app.config().identifier == APPLICATION_ID {
        Ok(())
    } else {
        Err("m2a_application_identity_refused".into())
    }
}

#[derive(Clone, Debug)]
struct StoragePaths {
    application_id: &'static str,
    root: PathBuf,
    live: PathBuf,
    pending: PathBuf,
    receipt: PathBuf,
    pending_receipt: PathBuf,
}

fn paths_at(root: PathBuf) -> StoragePaths {
    StoragePaths {
        application_id: APPLICATION_ID,
        live: root.join(DATABASE_FILENAME),
        pending: root.join(PENDING_FILENAME),
        receipt: root.join(RECEIPT_FILENAME),
        pending_receipt: root.join(PENDING_RECEIPT_FILENAME),
        root,
    }
}

fn storage_paths(app: &AppHandle) -> Result<StoragePaths, String> {
    require_identity(app)?;
    let root = app
        .path()
        .app_data_dir()
        .map_err(|_| "m2a_app_private_directory_unavailable")?;
    Ok(paths_at(root))
}

fn read_locale_preference_at(path: &Path) -> Result<Option<String>, String> {
    let value = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("m2a_locale_preference_read_failed".into()),
    };
    Ok(match value.as_str() {
        "en" | "zh-TW" | "ja" => Some(value),
        _ => None,
    })
}

fn write_locale_preference_at(path: &Path, locale: &str) -> Result<(), String> {
    if !matches!(locale, "en" | "zh-TW" | "ja") {
        return Err("m2a_locale_preference_invalid".into());
    }
    let root = path
        .parent()
        .ok_or_else(|| "m2a_app_private_directory_unavailable".to_string())?;
    fs::create_dir_all(root).map_err(|_| "m2a_app_private_directory_create_failed".to_string())?;
    fs::write(path, locale).map_err(|_| "m2a_locale_preference_write_failed".to_string())
}

#[tauri::command]
pub(crate) fn m2a_get_locale_preference(app: AppHandle) -> Result<Option<String>, String> {
    let _guard = LOCALE_PREFERENCE_LOCK
        .lock()
        .map_err(|_| "m2a_locale_preference_lock_poisoned".to_string())?;
    let paths = storage_paths(&app)?;
    read_locale_preference_at(&paths.root.join(LOCALE_PREFERENCE_FILENAME))
}

#[tauri::command]
pub(crate) fn m2a_set_locale_preference(app: AppHandle, locale: String) -> Result<(), String> {
    let _guard = LOCALE_PREFERENCE_LOCK
        .lock()
        .map_err(|_| "m2a_locale_preference_lock_poisoned".to_string())?;
    let paths = storage_paths(&app)?;
    write_locale_preference_at(&paths.root.join(LOCALE_PREFERENCE_FILENAME), &locale)
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum PublicationFailurePoint {
    #[default]
    None,
    AfterLiveDatabasePublish,
    AfterLiveReceiptPublish,
}

fn sqlite_sidecars(path: &Path) -> [PathBuf; 3] {
    ["-wal", "-shm", "-journal"].map(|suffix| {
        let mut value = path.as_os_str().to_os_string();
        value.push(suffix);
        PathBuf::from(value)
    })
}

fn refuse_sidecars(paths: &StoragePaths) -> Result<(), String> {
    for database in [&paths.live, &paths.pending] {
        if sqlite_sidecars(database).iter().any(|path| path.exists()) {
            return Err("m2a_sqlite_sidecar_preserved".into());
        }
    }
    Ok(())
}

fn read_receipt(path: &Path) -> Result<DirectFreshV5Receipt, String> {
    let bytes = fs::read(path).map_err(|_| "m2a_receipt_read_failed".to_string())?;
    if bytes.len() > 16 * 1024 {
        return Err("m2a_receipt_too_large".into());
    }
    serde_json::from_slice(&bytes).map_err(|_| "m2a_receipt_malformed".to_string())
}

fn write_pending_receipt(path: &Path, receipt: &DirectFreshV5Receipt) -> Result<(), String> {
    let mut bytes =
        serde_json::to_vec(receipt).map_err(|_| "m2a_receipt_serialization_failed".to_string())?;
    bytes.push(b'\n');
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| "m2a_pending_receipt_create_failed".to_string())?;
    file.write_all(&bytes)
        .map_err(|_| "m2a_pending_receipt_write_failed".to_string())?;
    file.sync_all()
        .map_err(|_| "m2a_pending_receipt_sync_failed".to_string())
}

fn publish_no_replace(source: &Path, destination: &Path, code: &str) -> Result<(), String> {
    let mut source_file =
        fs::File::open(source).map_err(|error| format!("{code}_source_open:{}", error.kind()))?;
    let mut destination_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)
        .map_err(|error| format!("{code}_exclusive_create:{}", error.kind()))?;
    io::copy(&mut source_file, &mut destination_file)
        .map_err(|error| format!("{code}_copy:{}", error.kind()))?;
    destination_file
        .sync_all()
        .map_err(|error| format!("{code}_sync:{}", error.kind()))
}

#[cfg(unix)]
fn sync_directory(path: &Path, code: &str) -> Result<(), String> {
    let directory =
        fs::File::open(path).map_err(|error| format!("{code}_open:{}", error.kind()))?;
    directory
        .sync_all()
        .map_err(|error| format!("{code}_sync:{}", error.kind()))
}

fn retirement_durability_unknown(root: &Path) -> Result<bool, String> {
    RETIREMENT_DURABILITY_UNCERTAIN
        .lock()
        .map(|roots| roots.iter().any(|candidate| candidate == root))
        .map_err(|_| "m2a_retirement_durability_lock_poisoned".to_string())
}

fn mark_retirement_durability_unknown(root: &Path) -> Result<(), String> {
    let mut roots = RETIREMENT_DURABILITY_UNCERTAIN
        .lock()
        .map_err(|_| "m2a_retirement_durability_lock_poisoned".to_string())?;
    if !roots.iter().any(|candidate| candidate == root) {
        roots.push(root.to_path_buf());
    }
    Ok(())
}

fn clear_retirement_durability_unknown(root: &Path) -> Result<(), String> {
    let mut roots = RETIREMENT_DURABILITY_UNCERTAIN
        .lock()
        .map_err(|_| "m2a_retirement_durability_lock_poisoned".to_string())?;
    roots.retain(|candidate| candidate != root);
    Ok(())
}

// Windows host tests cannot open a directory through std::fs::File. The
// durability contract is exercised by the Android-native restart regression;
// host tests still validate that the target is the expected directory.
#[cfg(not(unix))]
fn sync_directory(path: &Path, code: &str) -> Result<(), String> {
    if path.is_dir() {
        Ok(())
    } else {
        Err(format!("{code}_open:not_a_directory"))
    }
}

fn publication_injected(
    actual: PublicationFailurePoint,
    expected: PublicationFailurePoint,
) -> Result<(), String> {
    if actual == expected {
        Err(format!("injected_m2a_publication_failure:{expected:?}"))
    } else {
        Ok(())
    }
}

async fn verify_ready(paths: &StoragePaths) -> Result<(), String> {
    if retirement_durability_unknown(&paths.root)? {
        return Err("m2a_pending_retirement_durability_unknown_preserved".into());
    }
    refuse_sidecars(paths)?;
    if !paths.live.is_file()
        || !paths.receipt.is_file()
        || paths.pending.exists()
        || paths.pending_receipt.exists()
    {
        return Err("m2a_publication_state_incomplete_preserved".into());
    }
    let receipt = read_receipt(&paths.receipt)?;
    verify_direct_fresh_v5(&paths.live, &receipt, paths.application_id)
        .await
        .map_err(|error| format!("m2a_existing_database_refused:{}", error.code))?;
    crate::schema_v5_migration::runtime::list_experiences_direct_fresh(&paths.live)
        .await
        .map_err(|error| format!("m2a_existing_runtime_refused:{}", error.code))?;
    Ok(())
}

async fn create_fresh_exact_v5(
    paths: &StoragePaths,
    direct_failure: DirectInitFailurePoint,
    publication_failure: PublicationFailurePoint,
) -> Result<(), String> {
    let any_state_exists = paths.live.exists()
        || paths.pending.exists()
        || paths.receipt.exists()
        || paths.pending_receipt.exists();
    if any_state_exists {
        return Err("m2a_fresh_initialization_state_changed".into());
    }
    fs::create_dir_all(&paths.root)
        .map_err(|_| "m2a_app_private_directory_create_failed".to_string())?;
    refuse_sidecars(paths)?;
    if paths.live.exists()
        || paths.pending.exists()
        || paths.receipt.exists()
        || paths.pending_receipt.exists()
    {
        return Err("m2a_fresh_initialization_state_changed".into());
    }

    let now = canonical_now()?;
    let receipt = initialize_direct_fresh_v5(DirectInitRequest {
        path: &paths.pending,
        application_id: paths.application_id,
        initialized_at: &now,
        failure_point: direct_failure,
    })
    .await
    .map_err(|error| format!("m2a_direct_fresh_v5_failed:{}", error.code))?;
    refuse_sidecars(paths)?;
    write_pending_receipt(&paths.pending_receipt, &receipt)?;
    verify_direct_fresh_v5(
        &paths.pending,
        &read_receipt(&paths.pending_receipt)?,
        paths.application_id,
    )
    .await
    .map_err(|error| format!("m2a_pending_verification_failed:{}", error.code))?;

    // Android app-private storage can refuse hard links even for the owning
    // UID. Publish by exclusively creating and syncing each live destination
    // from its already-verified pending source. O_EXCL prevents replacement;
    // any partial copy or interruption leaves the pending source plus a mixed
    // live/pending state that the next launch preserves and refuses.
    publish_no_replace(
        &paths.pending,
        &paths.live,
        "m2a_live_database_publish_failed",
    )?;
    publication_injected(
        publication_failure,
        PublicationFailurePoint::AfterLiveDatabasePublish,
    )?;
    publish_no_replace(
        &paths.pending_receipt,
        &paths.receipt,
        "m2a_live_receipt_publish_failed",
    )?;
    publication_injected(
        publication_failure,
        PublicationFailurePoint::AfterLiveReceiptPublish,
    )?;

    // File sync makes the copied contents durable; the directory barrier makes
    // both exclusive live-name creations durable before their pending sources
    // are retired. Without the second barrier below, a shutdown can resurrect
    // successfully removed pending names and make the next launch fail closed.
    sync_directory(&paths.root, "m2a_live_publication_directory_sync_failed")?;
    mark_retirement_durability_unknown(&paths.root)?;
    fs::remove_file(&paths.pending_receipt)
        .map_err(|_| "m2a_pending_receipt_retire_failed".to_string())?;
    fs::remove_file(&paths.pending)
        .map_err(|_| "m2a_pending_database_retire_failed".to_string())?;
    sync_directory(&paths.root, "m2a_pending_retirement_directory_sync_failed")?;
    clear_retirement_durability_unknown(&paths.root)?;
    verify_ready(paths).await
}

async fn ensure_ready_at(paths: &StoragePaths) -> Result<(), String> {
    refuse_sidecars(paths)?;
    let no_state = !paths.live.exists()
        && !paths.pending.exists()
        && !paths.receipt.exists()
        && !paths.pending_receipt.exists();
    if no_state {
        create_fresh_exact_v5(
            paths,
            DirectInitFailurePoint::None,
            PublicationFailurePoint::None,
        )
        .await?;
    }
    verify_ready(paths).await
}

fn with_storage<T>(
    app: &AppHandle,
    operation: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    let _guard = OPERATION_LOCK
        .lock()
        .map_err(|_| "m2a_operation_lock_poisoned".to_string())?;
    let paths = storage_paths(app)?;
    tauri::async_runtime::block_on(ensure_ready_at(&paths))?;
    operation(&paths.live)
}

#[tauri::command]
pub(crate) fn m2a_storage_status(app: AppHandle) -> Result<M2AStorageStatus, String> {
    with_storage(&app, |_| Ok(M2AStorageStatus::ready()))
}

// Fixed review profiles share the exact publication/readiness algorithm.
// No renderer-supplied path or identity can reach this constructor.
fn m2b_paths_at(root: PathBuf) -> StoragePaths {
    StoragePaths {
        application_id: "com.lifeos.review.m2b",
        live: root.join("android-m2b-disposable-v5.db"),
        pending: root.join(".android-m2b-fresh-v5.pending.db"),
        receipt: root.join("android-m2b-direct-fresh-v5.receipt.json"),
        pending_receipt: root.join(".android-m2b-direct-fresh-v5.pending.receipt.json"),
        root,
    }
}

pub(crate) fn with_m2b_storage<T>(
    app: &AppHandle,
    operation: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    if app.config().identifier != "com.lifeos.review.m2b" {
        return Err("m2b_application_identity_refused".into());
    }
    let _lock = OPERATION_LOCK.lock().map_err(|_| "m2b_operation_lock_poisoned")?;
    let root = app.path().app_data_dir().map_err(|_| "m2b_app_private_path_failed")?;
    let paths = m2b_paths_at(root);
    tauri::async_runtime::block_on(ensure_ready_at(&paths))
        .map_err(|_| "m2b_storage_blocked_preserved")?;
    tauri::async_runtime::block_on(
        crate::schema_v5_migration::runtime::refuse_non_synthetic_dependencies(&paths.live)
    ).map_err(|_| "m2b_unexpected_dependency_preserved")?;
    operation(&paths.live)
}

#[cfg(test)]
pub(crate) async fn prepare_m2b_fixture(root: PathBuf) -> Result<PathBuf, String> {
    let paths = m2b_paths_at(root);
    ensure_ready_at(&paths).await?;
    Ok(paths.live)
}

fn m2c_paths_at(root: PathBuf) -> StoragePaths {
    StoragePaths {
        application_id: "com.lifeos.review.m2c",
        live: root.join("android-m2c-disposable-v5.db"),
        pending: root.join(".android-m2c-fresh-v5.pending.db"),
        receipt: root.join("android-m2c-direct-fresh-v5.receipt.json"),
        pending_receipt: root.join(".android-m2c-direct-fresh-v5.pending.receipt.json"),
        root,
    }
}

pub(crate) fn with_m2c_storage<T>(app: &AppHandle, operation: impl FnOnce(&Path) -> Result<T,String>) -> Result<T,String> {
    if app.config().identifier != "com.lifeos.review.m2c" {return Err("m2c_application_identity_refused".into())}
    let _lock=OPERATION_LOCK.lock().map_err(|_| "m2c_operation_lock_poisoned")?;
    let root=app.path().app_data_dir().map_err(|_| "m2c_app_private_path_failed")?;
    let paths=m2c_paths_at(root);
    tauri::async_runtime::block_on(ensure_ready_at(&paths)).map_err(|_| "m2c_storage_blocked_preserved")?;
    tauri::async_runtime::block_on(crate::schema_v5_migration::android_reflection::verify_scope(&paths.live))
        .map_err(|_| "m2c_unsupported_dependency_preserved")?;
    operation(&paths.live)
}

#[cfg(test)]
pub(crate) async fn prepare_m2c_fixture(root: PathBuf) -> Result<PathBuf,String> {
    let paths=m2c_paths_at(root);ensure_ready_at(&paths).await?;Ok(paths.live)
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
        return Err("m2a_debug_hold_unavailable".into());
    }
    let milliseconds = milliseconds.unwrap_or(10_000);
    if milliseconds == 0 || milliseconds > MAX_DEBUG_HOLD_MS {
        return Err("m2a_debug_hold_invalid".into());
    }
    let marker = live.with_file_name(format!(".m2a-{expected}.hold"));
    fs::write(&marker, expected).map_err(|_| "m2a_debug_hold_marker_failed".to_string())?;
    std::thread::sleep(Duration::from_millis(milliseconds));
    Ok(())
}

fn save_at(
    live: &Path,
    request_id: &str,
    body: &str,
    debug_phase: Option<&str>,
    debug_hold_ms: Option<u64>,
) -> Result<M2ASaveReceipt, String> {
    validate_request_id(request_id)?;
    validate_body(body)?;
    debug_hold(live, debug_phase, "beforeCommit", debug_hold_ms)?;

    tauri::async_runtime::block_on(async {
        if let Some(existing) =
            crate::schema_v5_migration::runtime::get_experience_direct_fresh(live, request_id)
                .await
                .map_err(|error| format!("m2a_existing_request_unreadable:{}", error.code))?
        {
            if existing.body == body {
                return Ok(M2ASaveReceipt {
                    acknowledgement: "alreadyCommitted",
                    entry: existing,
                });
            }
            return Err("m2a_request_id_content_conflict".into());
        }

        let occurred_at = canonical_now()?;
        let guard_token = format!("m2a-create-{:x}", Sha256::digest(request_id.as_bytes()));
        let created = match crate::schema_v5_migration::runtime::create_experience_direct_fresh(
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
                    crate::schema_v5_migration::runtime::get_experience_direct_fresh(
                        live, request_id,
                    )
                    .await
                {
                    if existing.body == body {
                        return Ok(M2ASaveReceipt {
                            acknowledgement: "alreadyCommitted",
                            entry: existing,
                        });
                    }
                }
                return Err(format!("m2a_create_failed:{}", error.code));
            }
        };
        Ok(M2ASaveReceipt {
            acknowledgement: "committed",
            entry: created,
        })
    })
    .and_then(|receipt| {
        debug_hold(live, debug_phase, "afterCommitBeforeAck", debug_hold_ms)?;
        Ok(receipt)
    })
}

#[tauri::command]
pub(crate) fn m2a_create_experience(
    app: AppHandle,
    request_id: String,
    body: String,
    debug_phase: Option<String>,
    debug_hold_ms: Option<u64>,
) -> Result<M2ASaveReceipt, String> {
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
pub(crate) fn m2a_list_experiences(
    app: AppHandle,
) -> Result<Vec<crate::schema_v5_migration::runtime::ExperienceRow>, String> {
    with_storage(&app, |live| {
        tauri::async_runtime::block_on(
            crate::schema_v5_migration::runtime::list_experiences_direct_fresh(live),
        )
        .map_err(|error| format!("m2a_list_failed:{}", error.code))
    })
}

#[tauri::command]
pub(crate) fn m2a_get_experience(
    app: AppHandle,
    id: String,
) -> Result<Option<crate::schema_v5_migration::runtime::ExperienceRow>, String> {
    validate_request_id(&id)?;
    with_storage(&app, |live| {
        tauri::async_runtime::block_on(
            crate::schema_v5_migration::runtime::get_experience_direct_fresh(live, &id),
        )
        .map_err(|error| format!("m2a_get_failed:{}", error.code))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Connection, Row, SqliteConnection};
    use std::sync::Arc;
    use std::thread;
    use tempfile::TempDir;

    fn test_paths(directory: &TempDir) -> StoragePaths {
        paths_at(directory.path().to_path_buf())
    }

    async fn scalar(path: &Path, sql: &str) -> i64 {
        let mut connection = SqliteConnection::connect(&format!("sqlite://{}", path.display()))
            .await
            .unwrap();
        let value = sqlx::query(sql)
            .fetch_one(&mut connection)
            .await
            .unwrap()
            .get::<i64, _>(0);
        connection.close().await.unwrap();
        value
    }

    #[test]
    fn locale_preference_is_app_private_non_content_and_bounded() {
        let directory = TempDir::new().unwrap();
        let path = directory.path().join(LOCALE_PREFERENCE_FILENAME);
        assert_eq!(read_locale_preference_at(&path).unwrap(), None);
        write_locale_preference_at(&path, "zh-TW").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "zh-TW");
        assert_eq!(
            read_locale_preference_at(&path).unwrap().as_deref(),
            Some("zh-TW")
        );
        assert_eq!(
            write_locale_preference_at(&path, "synthetic Experience text").unwrap_err(),
            "m2a_locale_preference_invalid"
        );
        fs::write(&path, "invalid").unwrap();
        assert_eq!(read_locale_preference_at(&path).unwrap(), None);
    }

    #[test]
    fn direct_fresh_v5_has_truthful_origin_and_preserves_exact_cjk() {
        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        tauri::async_runtime::block_on(ensure_ready_at(&paths)).unwrap();
        assert!(!paths.pending.exists());
        assert!(!paths.pending_receipt.exists());
        assert!(!retirement_durability_unknown(&paths.root).unwrap());
        let receipt = read_receipt(&paths.receipt).unwrap();
        assert_eq!(receipt.origin, "direct_fresh_v5");
        assert_eq!(receipt.application_id, APPLICATION_ID);
        assert_eq!(
            tauri::async_runtime::block_on(scalar(&paths.live, "PRAGMA user_version")),
            5
        );
        assert_eq!(
            tauri::async_runtime::block_on(scalar(
                &paths.live,
                "SELECT COUNT(*) FROM schema_migration_receipts"
            )),
            0
        );

        let body = "今天我想記住：静かな勇気。\nExact spacing stays.";
        let first = save_at(&paths.live, "request-cjk-0001", body, None, None).unwrap();
        assert_eq!(first.acknowledgement, "committed");
        let repeated = save_at(&paths.live, "request-cjk-0001", body, None, None).unwrap();
        assert_eq!(repeated.acknowledgement, "alreadyCommitted");
        assert_eq!(
            tauri::async_runtime::block_on(
                crate::schema_v5_migration::runtime::get_experience_direct_fresh(
                    &paths.live,
                    "request-cjk-0001",
                )
            )
            .unwrap()
            .unwrap()
            .body,
            body
        );
        tauri::async_runtime::block_on(ensure_ready_at(&paths)).unwrap();
        assert_eq!(
            tauri::async_runtime::block_on(
                crate::schema_v5_migration::runtime::list_experiences_direct_fresh(&paths.live)
            )
            .unwrap()
            .len(),
            1
        );
    }

    #[tokio::test]
    async fn interrupted_direct_transactions_remain_pending_and_never_claim_ready() {
        for point in [
            DirectInitFailurePoint::AfterCompatibilityDdl,
            DirectInitFailurePoint::AfterFinalDdlStatement(0),
            DirectInitFailurePoint::AfterDatabaseContract,
            DirectInitFailurePoint::AfterVersionMutation,
            DirectInitFailurePoint::PostCommitVerification,
        ] {
            let directory = TempDir::new().unwrap();
            let paths = test_paths(&directory);
            let error = create_fresh_exact_v5(&paths, point, PublicationFailurePoint::None)
                .await
                .unwrap_err();
            assert!(error.contains("direct_fresh_v5_failed"));
            assert!(paths.pending.exists());
            assert!(!paths.receipt.exists());
            assert!(ensure_ready_at(&paths).await.is_err());
        }
    }

    #[tokio::test]
    async fn publication_failures_preserve_every_observed_name_and_block_reopen() {
        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        let error = create_fresh_exact_v5(
            &paths,
            DirectInitFailurePoint::None,
            PublicationFailurePoint::AfterLiveDatabasePublish,
        )
        .await
        .unwrap_err();
        assert!(error.contains("injected_m2a_publication_failure"));
        assert!(paths.live.exists());
        assert!(paths.pending.exists());
        assert!(paths.pending_receipt.exists());
        assert!(!paths.receipt.exists());
        assert_eq!(
            ensure_ready_at(&paths).await.unwrap_err(),
            "m2a_publication_state_incomplete_preserved"
        );

        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        let error = create_fresh_exact_v5(
            &paths,
            DirectInitFailurePoint::None,
            PublicationFailurePoint::AfterLiveReceiptPublish,
        )
        .await
        .unwrap_err();
        assert!(error.contains("injected_m2a_publication_failure"));
        assert!(paths.live.exists() && paths.receipt.exists());
        assert!(paths.pending.exists() && paths.pending_receipt.exists());
        assert_eq!(
            ensure_ready_at(&paths).await.unwrap_err(),
            "m2a_publication_state_incomplete_preserved"
        );
    }

    #[tokio::test]
    async fn pending_partial_malformed_newer_receipt_and_sidecars_are_preserved() {
        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        fs::write(&paths.pending, b"preserve pending").unwrap();
        assert!(ensure_ready_at(&paths).await.is_err());
        assert_eq!(fs::read(&paths.pending).unwrap(), b"preserve pending");

        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        fs::write(&paths.live, b"not sqlite").unwrap();
        fs::write(&paths.receipt, b"not json").unwrap();
        let before = fs::read(&paths.live).unwrap();
        assert!(ensure_ready_at(&paths).await.is_err());
        assert_eq!(fs::read(&paths.live).unwrap(), before);

        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        ensure_ready_at(&paths).await.unwrap();
        let mut receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(&paths.receipt).unwrap()).unwrap();
        receipt["origin"] = serde_json::Value::String("migration".into());
        fs::write(&paths.receipt, serde_json::to_vec(&receipt).unwrap()).unwrap();
        assert!(ensure_ready_at(&paths).await.is_err());
        assert_eq!(receipt["origin"], "migration");

        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        ensure_ready_at(&paths).await.unwrap();
        let wal = sqlite_sidecars(&paths.live)[0].clone();
        fs::write(&wal, b"preserve sidecar").unwrap();
        assert_eq!(
            ensure_ready_at(&paths).await.unwrap_err(),
            "m2a_sqlite_sidecar_preserved"
        );
        assert_eq!(fs::read(&wal).unwrap(), b"preserve sidecar");

        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        fs::create_dir_all(&paths.root).unwrap();
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&paths.live)
            .create_if_missing(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        sqlx::query("PRAGMA user_version = 6")
            .execute(&mut connection)
            .await
            .unwrap();
        connection.close().await.unwrap();
        fs::write(&paths.receipt, b"{}").unwrap();
        assert!(ensure_ready_at(&paths).await.is_err());
        assert_eq!(scalar(&paths.live, "PRAGMA user_version").await, 6);
    }

    #[test]
    fn concurrent_initialization_serializes_to_one_direct_database() {
        let directory = Arc::new(TempDir::new().unwrap());
        let mut threads = Vec::new();
        for _ in 0..2 {
            let directory = directory.clone();
            threads.push(thread::spawn(move || {
                let _guard = OPERATION_LOCK.lock().unwrap();
                let paths = test_paths(&directory);
                tauri::async_runtime::block_on(ensure_ready_at(&paths))
            }));
        }
        for handle in threads {
            handle.join().unwrap().unwrap();
        }
        let paths = test_paths(&directory);
        let receipt = read_receipt(&paths.receipt).unwrap();
        tauri::async_runtime::block_on(verify_direct_fresh_v5(
            &paths.live,
            &receipt,
            APPLICATION_ID,
        ))
        .unwrap();
    }

    #[test]
    fn no_replace_publication_preserves_existing_destination() {
        let directory = TempDir::new().unwrap();
        let source = directory.path().join("source");
        let destination = directory.path().join("destination");
        fs::write(&source, b"candidate").unwrap();
        fs::write(&destination, b"existing").unwrap();
        assert!(publish_no_replace(&source, &destination, "test").is_err());
        assert_eq!(fs::read(&source).unwrap(), b"candidate");
        assert_eq!(fs::read(&destination).unwrap(), b"existing");
    }

    #[tokio::test]
    async fn database_open_failure_is_truthful_and_non_destructive() {
        let directory = TempDir::new().unwrap();
        let paths = test_paths(&directory);
        fs::create_dir(&paths.live).unwrap();
        fs::write(&paths.receipt, b"{}").unwrap();
        let error = ensure_ready_at(&paths).await.unwrap_err();
        assert!(error.contains("preserved") || error.contains("receipt"));
        assert!(paths.live.is_dir());
    }
}
