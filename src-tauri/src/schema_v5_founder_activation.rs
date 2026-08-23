#[cfg(windows)]
use crate::filesystem_safety::WindowsReplacement;
use crate::filesystem_safety::{
    create_owned_verified_backup, execute_replacement, inspect_readiness_filesystem,
    prepare_explicit_restore, read_migration_state_evidence, record_migration_state,
    DatabaseActivity, ExclusiveOperationGuard, ExpectedCandidate, MigrationOperationPhase,
    OwnedOperation, QuiescenceProbe, ReadinessFilesystemFailure, ReadinessOperationEvidence,
    SystemDurability, SystemVacuumInto, SystemVolumeProbe,
};
use crate::schema_v5_migration::{
    activate_lifecycle_writes, migrate_disposable_v4, orchestrate_disposable_v4_migration,
    source_manifest_for_path, verify_any_committed_v5, ExactV4CandidateVerifier, FailurePoint,
    MigrationRequest, SqlCommitOutcomeAdapter,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Connection, SqliteConnection};
use std::fs;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use time::{Duration, OffsetDateTime};
#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;

const FOUNDER_IDENTIFIER: &str = "com.lifeos.founderdogfood";
const ORDINARY_IDENTIFIER: &str = "com.lifeos.app";
const DATABASE_FILENAME: &str = "life-os.db";
const V4_SCHEMA_VERSION: i64 = 4;
const V5_SCHEMA_VERSION: i64 = 5;
const EMPTY_V4_BASE_SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;
CREATE TABLE experience_entries (
  id TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE TABLE persisted_artifacts (
  id TEXT PRIMARY KEY NOT NULL,
  source_entry_id TEXT NOT NULL,
  artifact_kind TEXT NOT NULL,
  payload TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE
);
CREATE INDEX idx_persisted_artifacts_source
  ON persisted_artifacts(source_entry_id, artifact_kind, created_at);
CREATE TABLE historical_consent_events (
  id TEXT PRIMARY KEY NOT NULL,
  packet_digest TEXT NOT NULL,
  payload TEXT NOT NULL,
  state TEXT NOT NULL CHECK(state IN ('granted','consumed','invalidated')),
  created_at TEXT NOT NULL,
  expires_at TEXT NOT NULL
);
CREATE TABLE historical_transmission_events (
  id TEXT PRIMARY KEY NOT NULL,
  consent_id TEXT NOT NULL,
  packet_digest TEXT NOT NULL,
  provider TEXT NOT NULL,
  model TEXT NOT NULL,
  outcome TEXT NOT NULL CHECK(outcome IN (
    'sent','failed','refused','cancelled_before_send','cancelled_after_send'
  )),
  created_at TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  FOREIGN KEY(consent_id) REFERENCES historical_consent_events(id) ON DELETE CASCADE
);
CREATE TABLE historical_question_artifacts (
  id TEXT PRIMARY KEY NOT NULL,
  current_experience_id TEXT NOT NULL,
  packet_digest TEXT NOT NULL,
  payload TEXT NOT NULL,
  packet_snapshot TEXT NOT NULL,
  consent_id TEXT NOT NULL,
  transmission_id TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY(current_experience_id) REFERENCES experience_entries(id) ON DELETE CASCADE,
  FOREIGN KEY(consent_id) REFERENCES historical_consent_events(id) ON DELETE CASCADE,
  FOREIGN KEY(transmission_id) REFERENCES historical_transmission_events(id) ON DELETE CASCADE
);
CREATE TABLE historical_artifact_dependencies (
  historical_artifact_id TEXT NOT NULL,
  source_entry_id TEXT NOT NULL,
  source_artifact_id TEXT,
  source_revision TEXT NOT NULL,
  PRIMARY KEY(historical_artifact_id, source_entry_id, source_artifact_id),
  FOREIGN KEY(historical_artifact_id) REFERENCES historical_question_artifacts(id) ON DELETE CASCADE,
  FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE
);
CREATE INDEX idx_historical_dependencies_source
  ON historical_artifact_dependencies(source_entry_id, source_artifact_id);
CREATE INDEX idx_historical_questions_current
  ON historical_question_artifacts(current_experience_id, created_at);
CREATE TRIGGER delete_historical_artifacts_before_source_delete
BEFORE DELETE ON experience_entries
BEGIN
  DELETE FROM historical_question_artifacts
  WHERE id IN (
    SELECT historical_artifact_id FROM historical_artifact_dependencies
    WHERE source_entry_id = OLD.id
  );
END;
CREATE TRIGGER delete_historical_provenance_after_artifact_delete
AFTER DELETE ON historical_question_artifacts
BEGIN
  DELETE FROM historical_transmission_events WHERE id = OLD.transmission_id;
  DELETE FROM historical_consent_events WHERE id = OLD.consent_id;
END;
PRAGMA user_version = 4;
"#;
static EXCLUSIVE_OPERATION: AtomicBool = AtomicBool::new(false);

struct OperationLock;
impl Drop for OperationLock {
    fn drop(&mut self) {
        EXCLUSIVE_OPERATION.store(false, Ordering::Release);
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FounderSchemaV5State {
    state: String,
    reason: Option<String>,
    detected_schema_version: Option<i64>,
    supported_schema_version: i64,
    initialization_required: bool,
    migration_available: bool,
    backup_available: bool,
    restore_available: bool,
    backup_relative_path: Option<String>,
    backup_retention_days: u16,
    backup_created_at: Option<String>,
    backup_expires_at: Option<String>,
}

impl FounderSchemaV5State {
    fn state(name: &str, version: Option<i64>) -> Self {
        Self {
            state: name.into(),
            reason: None,
            detected_schema_version: version,
            supported_schema_version: V5_SCHEMA_VERSION,
            initialization_required: name == "missing",
            migration_available: name == "migration_required",
            backup_available: false,
            restore_available: false,
            backup_relative_path: None,
            backup_retention_days: 30,
            backup_created_at: None,
            backup_expires_at: None,
        }
    }

    fn blocked(reason: impl Into<String>, version: Option<i64>) -> Self {
        Self {
            state: "blocked".into(),
            reason: Some(reason.into()),
            detected_schema_version: version,
            supported_schema_version: V5_SCHEMA_VERSION,
            initialization_required: false,
            migration_available: false,
            backup_available: false,
            restore_available: false,
            backup_relative_path: None,
            backup_retention_days: 30,
            backup_created_at: None,
            backup_expires_at: None,
        }
    }

    fn with_backup(mut self, operation: &OwnedOperation) -> Self {
        self.backup_available = operation.backup.is_file();
        self.backup_relative_path = if self.backup_available {
            operation
                .root
                .file_name()
                .map(|name| format!("{}/backup.db", name.to_string_lossy()))
        } else {
            None
        };
        self
    }

    fn with_restore_available(mut self) -> Self {
        self.restore_available = self.backup_available;
        self
    }
}

fn canonical_now() -> Result<String, String> {
    OffsetDateTime::now_utc()
        .format(time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
        ))
        .map_err(|_| "founder_clock_invalid".into())
}

async fn backup_dates(path: &Path) -> Result<(String, String), String> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .read_only(true)
        .create_if_missing(false)
        .immutable(true)
        .foreign_keys(true);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|_| "founder_database_unreadable")?;
    let created: String = sqlx::query_scalar(
        "SELECT committed_at FROM schema_migration_receipts WHERE state = 'committed'",
    )
    .fetch_one(&mut connection)
    .await
    .map_err(|_| "founder_migration_receipt_unreadable")?;
    connection
        .close()
        .await
        .map_err(|_| "founder_database_unreadable")?;
    let parsed = OffsetDateTime::parse(&created, &time::format_description::well_known::Rfc3339)
        .map_err(|_| "founder_migration_receipt_timestamp_invalid")?;
    let expires = (parsed + Duration::days(30))
        .format(time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
        ))
        .map_err(|_| "founder_migration_receipt_timestamp_invalid")?;
    Ok((created, expires))
}

struct Quiescent;
impl QuiescenceProbe for Quiescent {
    fn database_activity(&self) -> DatabaseActivity {
        DatabaseActivity::Quiescent
    }
}

fn require_schema_v5_identity(app: &AppHandle) -> Result<(), String> {
    let identifier = app.config().identifier.as_str();
    let founder_allowed = cfg!(feature = "founder-schema-v5") && identifier == FOUNDER_IDENTIFIER;
    let ordinary_allowed = cfg!(feature = "desktop-schema-v5") && identifier == ORDINARY_IDENTIFIER;
    if founder_allowed || ordinary_allowed {
        Ok(())
    } else {
        Err("desktop_schema_v5_identity_refused".into())
    }
}

fn paths(app: &AppHandle) -> Result<(PathBuf, PathBuf), String> {
    require_schema_v5_identity(app)?;
    let root = app
        .path()
        .app_data_dir()
        .map_err(|_| "founder_profile_path_unavailable")?;
    Ok((root.clone(), root.join(DATABASE_FILENAME)))
}

fn operation_lock() -> Result<OperationLock, String> {
    EXCLUSIVE_OPERATION
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .map(|_| OperationLock)
        .map_err(|_| "founder_database_operation_active".into())
}

#[cfg(windows)]
fn ensure_disk_capacity(root: &Path, source: Option<&Path>) -> Result<(), String> {
    let mut wide: Vec<u16> = root.as_os_str().encode_wide().collect();
    wide.push(0);
    let mut available = 0_u64;
    let ok = unsafe {
        GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut available,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        return Err("founder_disk_capacity_unknown".into());
    }
    let source_size = source
        .and_then(|path| fs::metadata(path).ok())
        .map_or(0, |metadata| metadata.len());
    let required = source_size
        .saturating_mul(3)
        .saturating_add(64 * 1024 * 1024);
    if available < required {
        Err("founder_disk_capacity_insufficient".into())
    } else {
        Ok(())
    }
}

#[cfg(not(windows))]
fn ensure_disk_capacity(_root: &Path, _source: Option<&Path>) -> Result<(), String> {
    Ok(())
}

fn operation_candidates(root: &Path, live: &Path) -> Result<Vec<OwnedOperation>, String> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut operations = Vec::new();
    let entries = fs::read_dir(root).map_err(|_| "founder_operation_evidence_unreadable")?;
    for entry in entries {
        let entry = entry.map_err(|_| "founder_operation_evidence_unreadable")?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("life-os-") || !name.ends_with(".operation") {
            continue;
        }
        let operation_root = entry.path();
        let operation_id = name
            .strip_prefix("life-os-")
            .and_then(|value| value.strip_suffix(".operation"))
            .ok_or("founder_operation_identity_invalid")?
            .to_string();
        operations.push(OwnedOperation {
            operation_id,
            owned_root: fs::canonicalize(root).map_err(|_| "founder_owned_root_invalid")?,
            root: fs::canonicalize(&operation_root)
                .map_err(|_| "founder_operation_evidence_unreadable")?,
            live: fs::canonicalize(live).unwrap_or_else(|_| live.to_path_buf()),
            backup: operation_root.join("backup.db"),
            staging: operation_root.join("staging.db"),
            state: operation_root.join("state.json"),
        });
    }
    operations.sort_by(|left, right| left.operation_id.cmp(&right.operation_id));
    Ok(operations)
}

async fn read_user_version(path: &Path) -> Result<i64, String> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .read_only(true)
        .create_if_missing(false)
        .immutable(true)
        .foreign_keys(true);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|_| "founder_database_unreadable")?;
    let result = sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(&mut connection)
        .await
        .map_err(|_| "founder_database_malformed".to_string());
    connection
        .close()
        .await
        .map_err(|_| "founder_database_close_failed".to_string())?;
    result
}

async fn classify(root: &Path, live: &Path) -> FounderSchemaV5State {
    if !root.exists() {
        return FounderSchemaV5State::state("missing", None);
    }
    let filesystem = match inspect_readiness_filesystem(root, live) {
        Ok(value) => value,
        Err(ReadinessFilesystemFailure::PathUnsafe) => {
            return FounderSchemaV5State::blocked("path_unsafe", None)
        }
        Err(ReadinessFilesystemFailure::Unreadable) => {
            return FounderSchemaV5State::blocked("unreadable", None)
        }
        Err(ReadinessFilesystemFailure::RecoveryRequired) => {
            return FounderSchemaV5State::blocked("recovery_required", None)
        }
    };
    if !filesystem.database_exists {
        return if filesystem.operation_evidence == ReadinessOperationEvidence::None {
            FounderSchemaV5State::state("missing", None)
        } else {
            FounderSchemaV5State::blocked("orphaned_operation_evidence", None)
        };
    }
    if filesystem.wal_present || filesystem.shm_present || filesystem.rollback_journal_present {
        return FounderSchemaV5State::blocked("sqlite_sidecar_present", None);
    }
    let version = match read_user_version(live).await {
        Ok(version) => version,
        Err(reason) => return FounderSchemaV5State::blocked(reason, None),
    };
    let operations = match operation_candidates(root, live) {
        Ok(value) => value,
        Err(reason) => return FounderSchemaV5State::blocked(reason, Some(version)),
    };
    if operations.len() > 1 {
        return FounderSchemaV5State::blocked("multiple_operation_candidates", Some(version));
    }
    let operation = operations.first();
    match version {
        0..=3 => FounderSchemaV5State::blocked("older_schema_unsupported", Some(version)),
        V4_SCHEMA_VERSION => {
            if let Some(operation) = operation {
                let evidence = match read_migration_state_evidence(operation) {
                    Ok(value) => value,
                    Err(_) => {
                        return FounderSchemaV5State::blocked(
                            "operation_evidence_malformed",
                            Some(version),
                        )
                    }
                };
                match evidence.phase {
                    MigrationOperationPhase::BackupVerified
                    | MigrationOperationPhase::V4ReadyWithBackup => {
                        FounderSchemaV5State::state("migration_required", Some(version))
                            .with_backup(operation)
                    }
                    MigrationOperationPhase::Prepared => {
                        FounderSchemaV5State::blocked("migration_prepared", Some(version))
                            .with_backup(operation)
                    }
                    _ => FounderSchemaV5State::blocked("recovery_required", Some(version))
                        .with_backup(operation),
                }
            } else {
                FounderSchemaV5State::state("migration_required", Some(version))
            }
        }
        V5_SCHEMA_VERSION => match verify_any_committed_v5(live).await {
            Ok(receipt) => {
                let mut state = FounderSchemaV5State::state("ready", Some(version));
                if let Some(operation) = operation {
                    let evidence = match read_migration_state_evidence(operation) {
                        Ok(value) => value,
                        Err(_) => {
                            return FounderSchemaV5State::blocked(
                                "operation_evidence_malformed",
                                Some(version),
                            )
                            .with_backup(operation)
                        }
                    };
                    if evidence.phase != MigrationOperationPhase::V5Ready
                        || evidence.migration_id.as_deref() != Some(receipt.migration_id.as_str())
                        || evidence.migration_source_manifest_digest.as_deref()
                            != Some(receipt.source_manifest_digest.as_str())
                        || evidence.migration_target_manifest_digest.as_deref()
                            != Some(receipt.target_manifest_digest.as_str())
                        || receipt.backup_id.as_deref() != Some(operation.operation_id.as_str())
                    {
                        return FounderSchemaV5State::blocked(
                            "operation_evidence_contradictory",
                            Some(version),
                        )
                        .with_backup(operation);
                    }
                    let expected = match evidence.verified_backup {
                        Some(value) => value,
                        None => {
                            return FounderSchemaV5State::blocked(
                                "migration_backup_evidence_missing",
                                Some(version),
                            )
                        }
                    };
                    if crate::filesystem_safety::verify_owned_backup_for_migration(
                        operation,
                        &expected,
                        &ExactV4CandidateVerifier,
                    )
                    .await
                    .is_err()
                    {
                        return FounderSchemaV5State::blocked(
                            "migration_backup_verification_failed",
                            Some(version),
                        )
                        .with_backup(operation);
                    }
                    state = state.with_backup(operation);
                    match backup_dates(live).await {
                        Ok((created, expires)) => {
                            state.backup_created_at = Some(created);
                            state.backup_expires_at = Some(expires);
                        }
                        Err(reason) => {
                            return FounderSchemaV5State::blocked(reason, Some(version))
                                .with_backup(operation)
                        }
                    }
                } else if receipt.backup_id.is_some() {
                    return FounderSchemaV5State::blocked(
                        "migration_backup_operation_missing",
                        Some(version),
                    );
                }
                state
            }
            Err(error) => {
                let mut state = FounderSchemaV5State::blocked(error.code.clone(), Some(version))
                    .with_optional_backup(operation);
                if let Some(operation) = operation {
                    if let Ok(evidence) = read_migration_state_evidence(operation) {
                        let exact_blocked_restore = evidence.phase
                            == MigrationOperationPhase::V5BlockedRestoreAvailable
                            && evidence.migration_id.is_some()
                            && evidence.migration_source_manifest_digest.is_some()
                            && evidence.migration_target_manifest_digest.is_some()
                            && evidence.outcome_class.is_some();
                        if exact_blocked_restore {
                            if let Some(expected) = evidence.verified_backup {
                                if crate::filesystem_safety::verify_owned_backup_for_migration(
                                    operation,
                                    &expected,
                                    &ExactV4CandidateVerifier,
                                )
                                .await
                                .is_ok()
                                {
                                    state = state.with_restore_available();
                                }
                            }
                        }
                    }
                }
                state
            }
        },
        _ => FounderSchemaV5State::blocked("newer_schema_unsupported", Some(version)),
    }
}

trait OptionalBackup {
    fn with_optional_backup(self, operation: Option<&OwnedOperation>) -> Self;
}
impl OptionalBackup for FounderSchemaV5State {
    fn with_optional_backup(self, operation: Option<&OwnedOperation>) -> Self {
        operation.map_or(self.clone(), |operation| self.with_backup(operation))
    }
}

#[tauri::command]
pub fn inspect_founder_schema_v5_startup(app: AppHandle) -> Result<FounderSchemaV5State, String> {
    tauri::async_runtime::block_on(async move {
        let (root, live) = paths(&app)?;
        Ok(classify(&root, &live).await)
    })
}

async fn initialize_fresh(root: &Path, live: &Path) -> Result<(), String> {
    if live.exists() {
        return Err("founder_fresh_initialization_destination_exists".into());
    }
    fs::create_dir_all(root).map_err(|_| "founder_profile_directory_create_failed")?;
    ensure_disk_capacity(root, None)?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "founder_clock_invalid")?
        .as_nanos();
    let staging = root.join(format!(".life-os-fresh-v5-{nonce}.db"));
    if staging.exists() {
        return Err("founder_fresh_staging_conflict".into());
    }
    let result = async {
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&staging)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options)
            .await
            .map_err(|_| "founder_fresh_database_create_failed".to_string())?;
        sqlx::raw_sql(EMPTY_V4_BASE_SCHEMA)
            .execute(&mut connection)
            .await
            .map_err(|_| "founder_fresh_base_schema_failed".to_string())?;
        connection
            .close()
            .await
            .map_err(|_| "founder_fresh_base_close_failed".to_string())?;
        let source_manifest = source_manifest_for_path(&staging)
            .await
            .map_err(|error| error.code)?;
        let now = canonical_now()?;
        let receipt = migrate_disposable_v4(MigrationRequest {
            path: &staging,
            expected_source_manifest_digest: source_manifest,
            started_at: &now,
            committed_at: &now,
            backup_id: None,
            failure_point: FailurePoint::None,
        })
        .await
        .map_err(|error| error.code)?;
        activate_lifecycle_writes(&staging, &receipt, &now)
            .await
            .map_err(|error| error.code)?;
        verify_any_committed_v5(&staging)
            .await
            .map_err(|error| error.code)?;
        fs::rename(&staging, live)
            .map_err(|error| format!("founder_fresh_publish_failed:{}", error.kind()))?;
        verify_any_committed_v5(live)
            .await
            .map_err(|error| error.code)?;
        Ok(())
    }
    .await;
    if result.is_err() && staging.exists() {
        let _ = fs::remove_file(&staging);
    }
    result
}

#[tauri::command]
pub fn initialize_founder_schema_v5_database(
    app: AppHandle,
) -> Result<FounderSchemaV5State, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let (root, live) = paths(&app)?;
        let before = classify(&root, &live).await;
        if before.state != "missing" {
            return Err("founder_fresh_initialization_state_changed".into());
        }
        initialize_fresh(&root, &live).await?;
        let after = classify(&root, &live).await;
        if after.state != "ready" {
            return Err("founder_fresh_initialization_verification_failed".into());
        }
        Ok(after)
    })
}

async fn prove_exclusive_v4(path: &Path) -> Result<(), String> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(false)
        .foreign_keys(true);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|_| "founder_database_activity_unknown")?;
    sqlx::query("BEGIN EXCLUSIVE")
        .execute(&mut connection)
        .await
        .map_err(|_| "founder_database_activity_active")?;
    sqlx::query("ROLLBACK")
        .execute(&mut connection)
        .await
        .map_err(|_| "founder_database_activity_unknown")?;
    connection
        .close()
        .await
        .map_err(|_| "founder_database_activity_unknown".to_string())
}

#[tauri::command]
pub fn authorize_founder_schema_v5_migration(
    app: AppHandle,
) -> Result<FounderSchemaV5State, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let (root, live) = paths(&app)?;
        authorize_founder_schema_v5_migration_at(&root, &live).await
    })
}

async fn authorize_founder_schema_v5_migration_at(
    root: &Path,
    live: &Path,
) -> Result<FounderSchemaV5State, String> {
    let before = classify(root, live).await;
    if before.state != "migration_required" || before.detected_schema_version != Some(4) {
        return Err("founder_migration_authorization_state_changed".into());
    }
    prove_exclusive_v4(live).await?;
    ensure_disk_capacity(root, Some(live))?;
    let operations = operation_candidates(root, live)?;
    let verifier = ExactV4CandidateVerifier;
    let quiescence = Quiescent;
    let guard = ExclusiveOperationGuard::acquire(&quiescence).map_err(|error| error.code)?;
    let volume = SystemVolumeProbe;
    let operation = if let Some(existing) = operations.into_iter().next() {
        existing
    } else {
        crate::filesystem_safety::prepare_operation(root, live, &guard, &volume)
            .map_err(|error| error.code)?
    };
    let evidence = read_migration_state_evidence(&operation).map_err(|error| error.code)?;
    let expected = match evidence.verified_backup {
        Some(expected) => expected,
        None => create_owned_verified_backup(
            &operation,
            &guard,
            &volume,
            &verifier,
            &SystemVacuumInto,
            &SystemDurability,
        )
        .await
        .map_err(|error| error.code)?,
    };
    let now = canonical_now()?;
    let classification = orchestrate_disposable_v4_migration(
        &operation,
        &expected,
        MigrationRequest {
            // `operation_candidates` and `prepare_operation` bind the durable
            // operation to the canonical live path. Reuse that exact identity
            // here rather than the app-data spelling returned by Tauri (which
            // differs from `fs::canonicalize` on Windows).
            path: &operation.live,
            expected_source_manifest_digest: expected.source_manifest_digest.clone(),
            started_at: &now,
            committed_at: &now,
            backup_id: Some(&operation.operation_id),
            failure_point: FailurePoint::None,
        },
        &verifier,
        &SqlCommitOutcomeAdapter,
    )
    .await;
    if classification.phase != MigrationOperationPhase::V5Ready {
        return Ok(FounderSchemaV5State::blocked(
            classification
                .error_class
                .unwrap_or_else(|| "recovery_required".into()),
            read_user_version(live).await.ok(),
        )
        .with_backup(&operation));
    }
    let receipt = classification
        .receipt
        .ok_or("founder_migration_receipt_missing")?;
    activate_lifecycle_writes(live, &receipt, &now)
        .await
        .map_err(|error| error.code)?;
    record_migration_state(
        &operation,
        MigrationOperationPhase::V5Ready,
        Some("lifecycle_writes_enabled".into()),
        None,
    )
    .map_err(|error| error.code)?;
    Ok(classify(root, live).await)
}

#[tauri::command]
pub fn inspect_founder_schema_v5_backup(app: AppHandle) -> Result<FounderSchemaV5State, String> {
    inspect_founder_schema_v5_startup(app)
}

#[tauri::command]
pub fn delete_founder_schema_v5_backup(app: AppHandle) -> Result<FounderSchemaV5State, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let (root, live) = paths(&app)?;
        let state = classify(&root, &live).await;
        if state.state != "ready" || !state.backup_available {
            return Err("founder_backup_delete_state_changed".into());
        }
        let operations = operation_candidates(&root, &live)?;
        if operations.len() != 1 {
            return Err("founder_backup_delete_ownership_ambiguous".into());
        }
        let operation = &operations[0];
        let evidence = read_migration_state_evidence(operation).map_err(|error| error.code)?;
        let expected = evidence
            .verified_backup
            .ok_or("founder_backup_evidence_missing")?;
        crate::filesystem_safety::verify_owned_backup_for_migration(
            operation,
            &expected,
            &ExactV4CandidateVerifier,
        )
        .await
        .map_err(|error| error.code)?;
        fs::remove_file(&operation.backup).map_err(|_| "founder_backup_delete_failed")?;
        for path in [&operation.staging, &operation.state] {
            if path.exists() {
                fs::remove_file(path).map_err(|_| "founder_backup_cleanup_failed")?;
            }
        }
        fs::remove_dir(&operation.root).map_err(|_| "founder_backup_cleanup_failed")?;
        Ok(classify(&root, &live).await)
    })
}

#[tauri::command]
pub fn restore_founder_schema_v4_backup(app: AppHandle) -> Result<FounderSchemaV5State, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let (root, live) = paths(&app)?;
        restore_founder_schema_v4_backup_at(&root, &live).await
    })
}

async fn restore_founder_schema_v4_backup_at(
    root: &Path,
    live: &Path,
) -> Result<FounderSchemaV5State, String> {
    let state = classify(root, live).await;
    let restore_authorized_state = (state.state == "ready" && state.backup_available)
        || (state.state == "blocked" && state.restore_available);
    if !restore_authorized_state {
        return Err("founder_restore_state_changed".into());
    }
    let operations = operation_candidates(root, live)?;
    if operations.len() != 1 {
        return Err("founder_restore_ownership_ambiguous".into());
    }
    let operation = &operations[0];
    let evidence = read_migration_state_evidence(operation).map_err(|error| error.code)?;
    let expected: ExpectedCandidate = evidence
        .verified_backup
        .ok_or("founder_restore_backup_missing")?;
    prove_exclusive_v4(live).await?;
    let quiescence = Quiescent;
    let guard = ExclusiveOperationGuard::acquire(&quiescence).map_err(|error| error.code)?;
    prepare_explicit_restore(operation, &expected).map_err(|error| error.code)?;
    #[cfg(windows)]
    execute_replacement(
        operation,
        &guard,
        &expected,
        &ExactV4CandidateVerifier,
        &SystemDurability,
        &WindowsReplacement,
    )
    .await
    .map_err(|error| error.code)?;
    #[cfg(not(windows))]
    return Err("founder_restore_windows_only".into());
    // A successful explicit restore consumes the exact-owned operation and its
    // backup. Retaining v5 migration evidence beside the restored v4 database
    // would create contradictory restart evidence.
    for path in [&operation.backup, &operation.staging, &operation.state] {
        if path.exists() {
            fs::remove_file(path).map_err(|_| "founder_restore_cleanup_failed")?;
        }
    }
    fs::remove_dir(&operation.root).map_err(|_| "founder_restore_cleanup_failed")?;
    let restored = classify(root, live).await;
    if restored.state != "migration_required" || restored.detected_schema_version != Some(4) {
        return Err("founder_restore_postcondition_failed".into());
    }
    Ok(restored)
}

fn runtime_path(app: &AppHandle) -> Result<PathBuf, String> {
    let (_, live) = paths(app)?;
    Ok(live)
}

fn runtime_guard_token() -> Result<String, String> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "founder_clock_invalid")?
        .as_nanos();
    Ok(format!("founder-runtime-{nonce}"))
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FounderImportEntry {
    id: String,
    body: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FounderImportResult {
    imported: u64,
    skipped: u64,
}

#[tauri::command]
pub fn list_founder_v5_experiences(
    app: AppHandle,
) -> Result<Vec<crate::schema_v5_migration::runtime::ExperienceRow>, String> {
    tauri::async_runtime::block_on(async move {
        let live = runtime_path(&app)?;
        crate::schema_v5_migration::runtime::list_experiences(&live)
            .await
            .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn get_founder_v5_experience(
    app: AppHandle,
    id: String,
) -> Result<Option<crate::schema_v5_migration::runtime::ExperienceRow>, String> {
    tauri::async_runtime::block_on(async move {
        let live = runtime_path(&app)?;
        crate::schema_v5_migration::runtime::get_experience(&live, &id)
            .await
            .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn list_founder_v5_artifacts(app: AppHandle, entry_id: String) -> Result<Value, String> {
    tauri::async_runtime::block_on(async move {
        let live = runtime_path(&app)?;
        crate::schema_v5_migration::runtime::list_artifacts(&live, &entry_id)
            .await
            .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn create_founder_v5_experience(
    app: AppHandle,
    id: String,
    body: String,
    occurred_at: String,
) -> Result<crate::schema_v5_migration::runtime::ExperienceRow, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        crate::schema_v5_migration::runtime::create_experience(
            &live,
            &id,
            &body,
            &occurred_at,
            &guard,
        )
        .await
        .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn update_founder_v5_experience(
    app: AppHandle,
    id: String,
    expected_updated_at: String,
    body: String,
    occurred_at: String,
) -> Result<Option<crate::schema_v5_migration::runtime::ExperienceRow>, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        crate::schema_v5_migration::runtime::update_experience(
            &live,
            &id,
            &expected_updated_at,
            &body,
            &occurred_at,
            &guard,
        )
        .await
        .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn delete_founder_v5_experience(
    app: AppHandle,
    id: String,
    expected_updated_at: String,
) -> Result<bool, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        let occurred_at = canonical_now()?;
        crate::schema_v5_migration::runtime::delete_experience(
            &live,
            &id,
            &expected_updated_at,
            &occurred_at,
            &guard,
        )
        .await
        .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn import_founder_v5_experiences(
    app: AppHandle,
    entries: Vec<FounderImportEntry>,
) -> Result<FounderImportResult, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        let occurred_at = canonical_now()?;
        let entries = entries
            .into_iter()
            .map(|entry| crate::schema_v5_migration::runtime::ExperienceRow {
                id: entry.id,
                body: entry.body,
                created_at: entry.created_at,
                updated_at: entry.updated_at,
            })
            .collect();
        let (imported, skipped) = crate::schema_v5_migration::runtime::import_experiences(
            &live,
            entries,
            &occurred_at,
            &guard,
        )
        .await
        .map_err(|error| error.code)?;
        Ok(FounderImportResult { imported, skipped })
    })
}

#[tauri::command]
pub fn save_founder_v5_artifacts(
    app: AppHandle,
    entry_id: String,
    bundle: Value,
    expected_experience_updated_at: Option<String>,
    occurred_at: String,
) -> Result<crate::schema_v5_migration::runtime::SaveArtifactsResult, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        crate::schema_v5_migration::runtime::save_artifacts(
            &live,
            &entry_id,
            bundle,
            expected_experience_updated_at.as_deref(),
            &occurred_at,
            &guard,
        )
        .await
        .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn save_founder_v5_historical_consent(app: AppHandle, event: Value) -> Result<(), String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        crate::schema_v5_migration::runtime::save_historical_consent(&live, event, &guard)
            .await
            .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn save_founder_v5_historical_transmission(app: AppHandle, event: Value) -> Result<(), String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        crate::schema_v5_migration::runtime::save_historical_transmission(&live, event, &guard)
            .await
            .map_err(|error| error.code)
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct FounderHistoricalSaveResult {
    status: &'static str,
}

#[tauri::command]
pub fn save_founder_v5_historical_question(
    app: AppHandle,
    artifact: Value,
) -> Result<FounderHistoricalSaveResult, String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        let status =
            crate::schema_v5_migration::runtime::save_historical_question(&live, artifact, &guard)
                .await
                .map_err(|error| error.code)?;
        Ok(FounderHistoricalSaveResult { status })
    })
}

#[tauri::command]
pub fn list_founder_v5_historical_questions(
    app: AppHandle,
    current_experience_id: String,
) -> Result<Vec<Value>, String> {
    tauri::async_runtime::block_on(async move {
        let live = runtime_path(&app)?;
        crate::schema_v5_migration::runtime::list_historical_questions(
            &live,
            &current_experience_id,
        )
        .await
        .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn delete_founder_v5_historical_question(app: AppHandle, id: String) -> Result<(), String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        crate::schema_v5_migration::runtime::delete_historical_question(&live, &id, &guard)
            .await
            .map_err(|error| error.code)
    })
}

#[tauri::command]
pub fn purge_founder_v5_expired_historical_audit(
    app: AppHandle,
    timestamp: String,
) -> Result<(), String> {
    tauri::async_runtime::block_on(async move {
        let _lock = operation_lock()?;
        let live = runtime_path(&app)?;
        let guard = runtime_guard_token()?;
        crate::schema_v5_migration::runtime::purge_expired_historical_audit(
            &live, &timestamp, &guard,
        )
        .await
        .map_err(|error| error.code)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");

    async fn exact_v4_fixture() -> (TempDir, PathBuf) {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&live)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        sqlx::raw_sql(V4_FIXTURE)
            .execute(&mut connection)
            .await
            .unwrap();
        connection.close().await.unwrap();
        (directory, live)
    }

    async fn production_initialized_v4_fixture() -> (TempDir, PathBuf) {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&live)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        crate::sqlite::migrate_connection(&mut connection, false)
            .await
            .unwrap();
        connection.close().await.unwrap();
        (directory, live)
    }

    fn sidecar(path: &Path, suffix: &str) -> PathBuf {
        PathBuf::from(format!("{}{}", path.display(), suffix))
    }

    async fn persistent_wal_v4_fixture() -> (TempDir, PathBuf) {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&live)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        crate::sqlite::migrate_connection(&mut connection, false)
            .await
            .unwrap();
        let journal_mode: String = sqlx::query_scalar("PRAGMA journal_mode = WAL")
            .fetch_one(&mut connection)
            .await
            .unwrap();
        assert_eq!(journal_mode, "wal");
        sqlx::query("PRAGMA user_version = 4")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
            .fetch_all(&mut connection)
            .await
            .unwrap();
        connection.close().await.unwrap();

        let header = fs::read(&live).unwrap();
        assert_eq!(&header[18..20], &[2, 2]);
        assert!(!sidecar(&live, "-wal").exists());
        assert!(!sidecar(&live, "-shm").exists());
        assert!(!sidecar(&live, "-journal").exists());
        (directory, live)
    }

    #[tokio::test]
    async fn exact_v4_activation_uses_the_owned_canonical_live_identity() {
        let (directory, live) = exact_v4_fixture().await;

        let state = authorize_founder_schema_v5_migration_at(directory.path(), &live)
            .await
            .unwrap();

        assert_eq!(state.state, "ready");
        assert_eq!(read_user_version(&live).await.unwrap(), V5_SCHEMA_VERSION);
        let operations = operation_candidates(directory.path(), &live).unwrap();
        assert_eq!(operations.len(), 1);
        assert_eq!(
            read_user_version(&operations[0].backup).await.unwrap(),
            V4_SCHEMA_VERSION
        );
        verify_any_committed_v5(&live).await.unwrap();
    }

    #[tokio::test]
    async fn exact_v4_activation_resumes_only_after_explicit_action_from_verified_backup() {
        let (directory, live) = exact_v4_fixture().await;
        let quiescence = Quiescent;
        let guard = ExclusiveOperationGuard::acquire(&quiescence).unwrap();
        let volume = SystemVolumeProbe;
        let operation =
            crate::filesystem_safety::prepare_operation(directory.path(), &live, &guard, &volume)
                .unwrap();
        let expected = create_owned_verified_backup(
            &operation,
            &guard,
            &volume,
            &ExactV4CandidateVerifier,
            &SystemVacuumInto,
            &SystemDurability,
        )
        .await
        .unwrap();
        assert_eq!(
            read_migration_state_evidence(&operation)
                .unwrap()
                .verified_backup,
            Some(expected)
        );
        assert_eq!(read_user_version(&live).await.unwrap(), V4_SCHEMA_VERSION);

        let state = authorize_founder_schema_v5_migration_at(directory.path(), &live)
            .await
            .unwrap();

        assert_eq!(state.state, "ready");
        assert_eq!(read_user_version(&live).await.unwrap(), V5_SCHEMA_VERSION);
        assert_eq!(
            read_user_version(&operation.backup).await.unwrap(),
            V4_SCHEMA_VERSION
        );
        verify_any_committed_v5(&live).await.unwrap();
    }

    #[tokio::test]
    async fn promoted_runtime_v4_schema_manifest_migrates_and_verifies_exact_v5() {
        let (directory, live) = production_initialized_v4_fixture().await;

        let state = authorize_founder_schema_v5_migration_at(directory.path(), &live)
            .await
            .unwrap();

        assert_eq!(state.state, "ready");
        assert_eq!(read_user_version(&live).await.unwrap(), V5_SCHEMA_VERSION);
        verify_any_committed_v5(&live).await.unwrap();
        let operations = operation_candidates(directory.path(), &live).unwrap();
        assert_eq!(operations.len(), 1);
        assert_eq!(
            read_user_version(&operations[0].backup).await.unwrap(),
            V4_SCHEMA_VERSION
        );

        let created = crate::schema_v5_migration::runtime::create_experience(
            &live,
            "runtime-v4-migrated-create",
            "A post-migration runtime moment",
            "2026-08-15T00:00:00.000Z",
            "runtime-v4-migrated-create-guard-0123456789abcdef",
        )
        .await
        .unwrap();
        assert_eq!(created.id, "runtime-v4-migrated-create");
        assert_eq!(created.body, "A post-migration runtime moment");
        verify_any_committed_v5(&live).await.unwrap();
    }

    #[tokio::test]
    async fn persistent_wal_exact_v4_migrates_with_verified_backup_without_source_sidecars() {
        let (directory, live) = persistent_wal_v4_fixture().await;

        let state = authorize_founder_schema_v5_migration_at(directory.path(), &live)
            .await
            .unwrap();

        assert_eq!(state.state, "ready", "{state:?}");
        assert_eq!(read_user_version(&live).await.unwrap(), V5_SCHEMA_VERSION);
        assert!(!sidecar(&live, "-wal").exists());
        assert!(!sidecar(&live, "-shm").exists());
        assert!(!sidecar(&live, "-journal").exists());
        let operations = operation_candidates(directory.path(), &live).unwrap();
        assert_eq!(operations.len(), 1);
        assert_eq!(
            read_user_version(&operations[0].backup).await.unwrap(),
            V4_SCHEMA_VERSION
        );
        verify_any_committed_v5(&live).await.unwrap();

        assert!(crate::schema_v5_migration::runtime::list_experiences(&live)
            .await
            .unwrap()
            .is_empty());
        assert!(!sidecar(&live, "-wal").exists());
        assert!(!sidecar(&live, "-shm").exists());

        let created = crate::schema_v5_migration::runtime::create_experience(
            &live,
            "persistent-wal-runtime-create",
            "A typed write after persistent-WAL migration",
            "2026-08-23T00:00:00.000Z",
            "persistent-wal-runtime-create-guard",
        )
        .await
        .unwrap();
        assert_eq!(created.id, "persistent-wal-runtime-create");
        assert_eq!(
            crate::schema_v5_migration::runtime::list_experiences(&live)
                .await
                .unwrap()
                .len(),
            1
        );
        assert!(!sidecar(&live, "-wal").exists());
        assert!(!sidecar(&live, "-shm").exists());
        assert!(!sidecar(&live, "-journal").exists());
    }

    #[tokio::test]
    async fn persistent_wal_with_uncheckpointed_sidecars_fails_closed_before_operation() {
        let (directory, live) = persistent_wal_v4_fixture().await;
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&live)
            .create_if_missing(false)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        sqlx::query(
            "INSERT INTO experience_entries (id, content, created_at, updated_at) \
             VALUES ('uncheckpointed', 'synthetic', 't', 't')",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        assert!(sidecar(&live, "-wal").exists());
        assert!(sidecar(&live, "-shm").exists());

        let state = classify(directory.path(), &live).await;
        assert_eq!(state.state, "blocked");
        assert_eq!(state.reason.as_deref(), Some("sqlite_sidecar_present"));
        assert!(
            authorize_founder_schema_v5_migration_at(directory.path(), &live)
                .await
                .is_err()
        );
        assert!(operation_candidates(directory.path(), &live)
            .unwrap()
            .is_empty());
        connection.close().await.unwrap();
    }

    #[tokio::test]
    async fn malformed_source_fails_closed_before_operation() {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        fs::write(&live, b"not-a-sqlite-database").unwrap();

        let state = classify(directory.path(), &live).await;
        assert_eq!(state.state, "blocked");
        assert!(
            authorize_founder_schema_v5_migration_at(directory.path(), &live)
                .await
                .is_err()
        );
        assert!(operation_candidates(directory.path(), &live)
            .unwrap()
            .is_empty());
        assert_eq!(fs::read(&live).unwrap(), b"not-a-sqlite-database");
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn exact_blocked_v5_evidence_exposes_only_explicit_verified_restore() {
        let (directory, live) = exact_v4_fixture().await;
        let quiescence = Quiescent;
        let guard = ExclusiveOperationGuard::acquire(&quiescence).unwrap();
        let volume = SystemVolumeProbe;
        let operation =
            crate::filesystem_safety::prepare_operation(directory.path(), &live, &guard, &volume)
                .unwrap();
        let expected = create_owned_verified_backup(
            &operation,
            &guard,
            &volume,
            &ExactV4CandidateVerifier,
            &SystemVacuumInto,
            &SystemDurability,
        )
        .await
        .unwrap();
        let classification = orchestrate_disposable_v4_migration(
            &operation,
            &expected,
            MigrationRequest {
                path: &operation.live,
                expected_source_manifest_digest: expected.source_manifest_digest.clone(),
                started_at: "2026-08-15T00:00:00.000Z",
                committed_at: "2026-08-15T00:00:00.000Z",
                backup_id: Some(&operation.operation_id),
                failure_point: FailurePoint::PostCommitVerification,
            },
            &ExactV4CandidateVerifier,
            &SqlCommitOutcomeAdapter,
        )
        .await;
        assert_eq!(
            classification.phase,
            MigrationOperationPhase::V5BlockedRestoreAvailable
        );

        let blocked = classify(directory.path(), &live).await;
        assert_eq!(blocked.state, "blocked");
        assert_eq!(
            blocked.reason.as_deref(),
            Some("runtime_v5_contract_mismatch")
        );
        assert!(blocked.backup_available);
        assert!(blocked.restore_available);

        let restored = restore_founder_schema_v4_backup_at(directory.path(), &live)
            .await
            .unwrap();
        assert_eq!(restored.state, "migration_required");
        assert_eq!(read_user_version(&live).await.unwrap(), V4_SCHEMA_VERSION);
        assert!(operation_candidates(directory.path(), &live)
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn missing_disposable_profile_initializes_exact_v5() {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        initialize_fresh(directory.path(), &live).await.unwrap();
        assert_eq!(read_user_version(&live).await.unwrap(), 5);
        verify_any_committed_v5(&live).await.unwrap();
    }

    #[tokio::test]
    async fn existing_destination_refuses_fresh_initialization_without_change() {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        fs::write(&live, b"sentinel").unwrap();
        let before = fs::read(&live).unwrap();
        assert_eq!(
            initialize_fresh(directory.path(), &live).await.unwrap_err(),
            "founder_fresh_initialization_destination_exists"
        );
        assert_eq!(fs::read(&live).unwrap(), before);
    }
}
