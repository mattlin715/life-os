use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::Connection;
use std::{
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Component, Path, PathBuf},
    process,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

const SUPPORTED_SCHEMA_VERSION: i64 = 4;
const OPERATION_STATE_SCHEMA: u8 = 3;
const MAX_OPERATION_ID_ATTEMPTS: usize = 16;
static OPERATION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReadinessFilesystemFailure {
    PathUnsafe,
    Unreadable,
    RecoveryRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReadinessOperationEvidence {
    None,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReadinessFilesystemSnapshot {
    pub(crate) database_exists: bool,
    pub(crate) wal_present: bool,
    pub(crate) shm_present: bool,
    pub(crate) rollback_journal_present: bool,
    pub(crate) operation_evidence: ReadinessOperationEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SafetyError {
    pub(crate) code: String,
    pub(crate) recovery_required: bool,
}

impl SafetyError {
    pub(crate) fn fail_closed(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            recovery_required: false,
        }
    }

    pub(crate) fn recovery_required(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            recovery_required: true,
        }
    }
}

impl fmt::Display for SafetyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.code)
    }
}

impl std::error::Error for SafetyError {}

impl From<io::Error> for SafetyError {
    fn from(error: io::Error) -> Self {
        Self::fail_closed(format!("filesystem_io_failed:{}", error.kind()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DatabaseActivity {
    Quiescent,
    Active,
    Unknown,
}

pub(crate) trait QuiescenceProbe {
    fn database_activity(&self) -> DatabaseActivity;
}

pub(crate) struct ExclusiveOperationGuard<'a, Q: QuiescenceProbe> {
    probe: &'a Q,
}

impl<'a, Q: QuiescenceProbe> ExclusiveOperationGuard<'a, Q> {
    pub(crate) fn acquire(probe: &'a Q) -> Result<Self, SafetyError> {
        let guard = Self { probe };
        guard.require_quiescent()?;
        Ok(guard)
    }

    fn require_quiescent(&self) -> Result<(), SafetyError> {
        match self.probe.database_activity() {
            DatabaseActivity::Quiescent => Ok(()),
            DatabaseActivity::Active => Err(SafetyError::fail_closed("database_activity_active")),
            DatabaseActivity::Unknown => Err(SafetyError::fail_closed("database_activity_unknown")),
        }
    }
}

pub(crate) trait VolumeProbe {
    fn volume_id(&self, path: &Path) -> Result<u64, SafetyError>;
}

pub(crate) struct SystemVolumeProbe;

impl VolumeProbe for SystemVolumeProbe {
    fn volume_id(&self, path: &Path) -> Result<u64, SafetyError> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let metadata = fs::metadata(path).map_err(|error| {
                SafetyError::fail_closed(format!("volume_probe_failed:{error}"))
            })?;
            Ok(metadata.dev())
        }

        #[cfg(windows)]
        {
            windows_volume_id(path)
        }

        #[cfg(not(any(unix, windows)))]
        {
            let _ = path;
            Err(SafetyError::fail_closed("volume_identity_unsupported"))
        }
    }
}

#[cfg(windows)]
fn windows_volume_id(path: &Path) -> Result<u64, SafetyError> {
    use std::{os::windows::ffi::OsStrExt, slice};
    use windows_sys::Win32::Storage::FileSystem::GetVolumePathNameW;

    let canonical = fs::canonicalize(path)
        .map_err(|error| SafetyError::fail_closed(format!("volume_probe_failed:{error}")))?;
    let mut input: Vec<u16> = canonical.as_os_str().encode_wide().collect();
    input.push(0);
    let mut volume_path = vec![0_u16; 32_768];
    let succeeded = unsafe {
        GetVolumePathNameW(
            input.as_ptr(),
            volume_path.as_mut_ptr(),
            volume_path.len() as u32,
        )
    };
    if succeeded == 0 {
        return Err(SafetyError::fail_closed("volume_identity_unavailable"));
    }
    let length = volume_path
        .iter()
        .position(|value| *value == 0)
        .ok_or_else(|| SafetyError::fail_closed("volume_identity_unterminated"))?;
    let bytes = unsafe {
        slice::from_raw_parts(
            volume_path.as_ptr().cast::<u8>(),
            length * std::mem::size_of::<u16>(),
        )
    };
    let digest = Sha256::digest(bytes);
    Ok(u64::from_be_bytes(digest[..8].try_into().unwrap()))
}

#[derive(Debug)]
pub(crate) struct OwnedOperation {
    pub(crate) operation_id: String,
    pub(crate) owned_root: PathBuf,
    pub(crate) root: PathBuf,
    pub(crate) live: PathBuf,
    pub(crate) backup: PathBuf,
    pub(crate) staging: PathBuf,
    pub(crate) state: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpectedCandidate {
    pub(crate) database_sha256: String,
    pub(crate) source_manifest_digest: String,
    pub(crate) schema_version: i64,
    pub(crate) foreign_keys_valid: bool,
    pub(crate) integrity_valid: bool,
    pub(crate) exact_record_digest: String,
}

impl ExpectedCandidate {
    pub(crate) fn validate_contract(&self) -> Result<(), SafetyError> {
        if self.schema_version != SUPPORTED_SCHEMA_VERSION {
            return Err(SafetyError::fail_closed(
                "expected_schema_version_not_supported",
            ));
        }
        for (label, digest) in [
            ("database", &self.database_sha256),
            ("source_manifest", &self.source_manifest_digest),
            ("exact_record", &self.exact_record_digest),
        ] {
            if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return Err(SafetyError::fail_closed(format!(
                    "expected_{label}_digest_invalid"
                )));
            }
        }
        if !self.foreign_keys_valid {
            return Err(SafetyError::fail_closed(
                "expected_foreign_key_state_invalid",
            ));
        }
        if !self.integrity_valid {
            return Err(SafetyError::fail_closed("expected_integrity_state_invalid"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CandidateEvidence {
    pub(crate) source_manifest_digest: String,
    pub(crate) schema_version: i64,
    pub(crate) foreign_keys_valid: bool,
    pub(crate) integrity_valid: bool,
    pub(crate) exact_record_digest: String,
}

pub(crate) trait CandidateVerifier {
    async fn inspect(&self, path: &Path) -> Result<CandidateEvidence, SafetyError>;
}

pub(crate) trait BackupCreator {
    async fn create(&self, source: &Path, destination: &Path) -> Result<(), SafetyError>;
}

pub(crate) struct SystemVacuumInto;

impl BackupCreator for SystemVacuumInto {
    async fn create(&self, source: &Path, destination: &Path) -> Result<(), SafetyError> {
        let destination = destination
            .to_str()
            .ok_or_else(|| SafetyError::fail_closed("backup_destination_not_unicode"))?;
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(source)
            .create_if_missing(false)
            .foreign_keys(true);
        let mut connection = sqlx::SqliteConnection::connect_with(&options)
            .await
            .map_err(|error| {
                SafetyError::fail_closed(format!("backup_source_open_failed:{error}"))
            })?;
        let result = sqlx::query("VACUUM INTO ?")
            .bind(destination)
            .execute(&mut connection)
            .await
            .map_err(|error| SafetyError::fail_closed(format!("backup_vacuum_failed:{error}")));
        let close_result = connection.close().await.map_err(|error| {
            SafetyError::recovery_required(format!("backup_source_close_failed:{error}"))
        });
        result?;
        close_result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParentDirectorySupport {
    Supported,
    Unsupported,
}

pub(crate) trait DurabilityAdapter {
    fn sync_file(&self, path: &Path) -> Result<(), SafetyError>;
    fn parent_directory_support(&self) -> ParentDirectorySupport;
    fn sync_parent(&self, parent: &Path) -> Result<(), SafetyError>;
}

pub(crate) struct SystemDurability;

impl DurabilityAdapter for SystemDurability {
    fn sync_file(&self, path: &Path) -> Result<(), SafetyError> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .and_then(|file| file.sync_all())
            .map_err(|error| SafetyError::fail_closed(format!("file_sync_failed:{}", error.kind())))
    }

    fn parent_directory_support(&self) -> ParentDirectorySupport {
        #[cfg(unix)]
        {
            ParentDirectorySupport::Supported
        }
        #[cfg(not(unix))]
        {
            ParentDirectorySupport::Unsupported
        }
    }

    fn sync_parent(&self, parent: &Path) -> Result<(), SafetyError> {
        #[cfg(unix)]
        {
            File::open(parent)
                .and_then(|directory| directory.sync_all())
                .map_err(|error| {
                    SafetyError::recovery_required(format!(
                        "parent_directory_sync_failed:{}",
                        error.kind()
                    ))
                })
        }

        #[cfg(not(unix))]
        {
            let _ = parent;
            Err(SafetyError::fail_closed(
                "parent_directory_sync_unsupported",
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReplacementOutcome {
    Committed,
    FailedUnchanged { error_class: String },
    OutcomeUnknown { error_class: String },
}

pub(crate) trait ReplacementAdapter {
    fn requires_parent_directory_sync(&self) -> bool;
    fn replace(&self, staging: &Path, live: &Path) -> ReplacementOutcome;
}

#[cfg(windows)]
pub(crate) struct WindowsReplacement;

#[cfg(windows)]
impl ReplacementAdapter for WindowsReplacement {
    fn requires_parent_directory_sync(&self) -> bool {
        true
    }

    fn replace(&self, staging: &Path, live: &Path) -> ReplacementOutcome {
        use std::{os::windows::ffi::OsStrExt, ptr};
        use windows_sys::Win32::{Foundation::GetLastError, Storage::FileSystem::ReplaceFileW};

        fn wide_path(path: &Path) -> Option<Vec<u16>> {
            let mut value: Vec<u16> = path.as_os_str().encode_wide().collect();
            if value.contains(&0) {
                return None;
            }
            value.push(0);
            Some(value)
        }

        let Some(staging) = wide_path(staging) else {
            return ReplacementOutcome::OutcomeUnknown {
                error_class: "windows_replacement_path_invalid".into(),
            };
        };
        let Some(live) = wide_path(live) else {
            return ReplacementOutcome::OutcomeUnknown {
                error_class: "windows_live_path_invalid".into(),
            };
        };

        let succeeded = unsafe {
            ReplaceFileW(
                live.as_ptr(),
                staging.as_ptr(),
                ptr::null(),
                0,
                ptr::null(),
                ptr::null(),
            )
        };
        if succeeded != 0 {
            ReplacementOutcome::Committed
        } else {
            classify_windows_replace_error(unsafe { GetLastError() })
        }
    }
}

#[cfg(windows)]
fn classify_windows_replace_error(error: u32) -> ReplacementOutcome {
    use windows_sys::Win32::Foundation::ERROR_UNABLE_TO_REMOVE_REPLACED;

    if error == ERROR_UNABLE_TO_REMOVE_REPLACED {
        ReplacementOutcome::FailedUnchanged {
            error_class: format!("windows_replace_error:{error}"),
        }
    } else {
        ReplacementOutcome::OutcomeUnknown {
            error_class: format!("windows_replace_error:{error}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExecutionResult {
    Completed,
    CompletedWithCleanupRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RestartInspection {
    Prepared,
    BackupVerified,
    Staged,
    ReplacementCommitted,
    Completed,
    CompletedWithCleanupRequired,
    RecoveryRequired { error_class: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MigrationOperationPhase {
    Prepared,
    BackupVerified,
    Migrating,
    CommitOutcomeUnknown,
    V5Verifying,
    V5Ready,
    V4ReadyWithBackup,
    V5BlockedRestoreAvailable,
    RecoveryRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MigrationStateEvidence {
    pub(crate) phase: MigrationOperationPhase,
    pub(crate) verified_backup: Option<ExpectedCandidate>,
    pub(crate) live_before_sha256: Option<String>,
    pub(crate) migration_id: Option<String>,
    pub(crate) migration_source_manifest_digest: Option<String>,
    pub(crate) migration_target_manifest_digest: Option<String>,
    pub(crate) outcome_class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MigrationReceiptEvidence<'a> {
    pub(crate) migration_id: &'a str,
    pub(crate) source_manifest_digest: &'a str,
    pub(crate) target_manifest_digest: &'a str,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum OperationPhase {
    Prepared,
    BackupVerified,
    Migrating,
    CommitOutcomeUnknown,
    V5Verifying,
    V5Ready,
    V4ReadyWithBackup,
    V5BlockedRestoreAvailable,
    Staged,
    ReplacementCommitted,
    Completed,
    CompletedWithCleanupRequired,
    RecoveryRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct OperationState {
    schema_version: u8,
    operation_id: String,
    operation_relative_directory: String,
    live_relative_filename: String,
    backup_relative_filename: String,
    staging_relative_filename: String,
    phase: OperationPhase,
    live_before_sha256: Option<String>,
    expected_database_sha256: Option<String>,
    backup_source_manifest_digest: Option<String>,
    backup_schema_version: Option<i64>,
    backup_foreign_keys_valid: Option<bool>,
    backup_integrity_valid: Option<bool>,
    backup_exact_record_digest: Option<String>,
    migration_id: Option<String>,
    migration_source_manifest_digest: Option<String>,
    migration_target_manifest_digest: Option<String>,
    outcome_class: Option<String>,
}

pub(crate) fn prepare_operation<Q: QuiescenceProbe, V: VolumeProbe>(
    root: &Path,
    live: &Path,
    guard: &ExclusiveOperationGuard<'_, Q>,
    volume_probe: &V,
) -> Result<OwnedOperation, SafetyError> {
    for _ in 0..MAX_OPERATION_ID_ATTEMPTS {
        let operation_id = generate_operation_id(root);
        match prepare_operation_with_id(root, live, guard, volume_probe, &operation_id) {
            Ok(operation) => return Ok(operation),
            Err(error) if error.code == "operation_destination_conflict" => continue,
            Err(error) => return Err(error),
        }
    }
    Err(SafetyError::fail_closed(
        "operation_id_collision_limit_reached",
    ))
}

fn prepare_operation_with_id<Q: QuiescenceProbe, V: VolumeProbe>(
    root: &Path,
    live: &Path,
    guard: &ExclusiveOperationGuard<'_, Q>,
    volume_probe: &V,
    operation_id: &str,
) -> Result<OwnedOperation, SafetyError> {
    guard.require_quiescent()?;
    validate_operation_id(operation_id)?;
    reject_lexical_alias(root)?;
    reject_lexical_alias(live)?;
    reject_reparse_chain(root)?;
    let canonical_root = fs::canonicalize(root)
        .map_err(|error| SafetyError::fail_closed(format!("owned_root_invalid:{error}")))?;
    if !canonical_root.is_dir() {
        return Err(SafetyError::fail_closed("owned_root_not_directory"));
    }

    let canonical_live = validate_direct_owned_existing_file(&canonical_root, live, "live")?;
    reject_multiple_links(&canonical_live)?;
    ensure_no_sidecars(&canonical_live)?;

    if volume_probe.volume_id(&canonical_root)? != volume_probe.volume_id(&canonical_live)? {
        return Err(SafetyError::fail_closed("cross_volume_replacement_refused"));
    }

    let operation_root = canonical_root.join(format!("life-os-{operation_id}.operation"));
    match fs::create_dir(&operation_root) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            return Err(SafetyError::fail_closed("operation_destination_conflict"));
        }
        Err(error) => {
            return Err(SafetyError::fail_closed(format!(
                "operation_directory_create_failed:{}",
                error.kind()
            )));
        }
    }
    let prepared_root = (|| {
        reject_reparse_chain(&operation_root)?;
        let prepared_root = fs::canonicalize(&operation_root).map_err(|error| {
            SafetyError::fail_closed(format!("operation_directory_invalid:{error}"))
        })?;
        if prepared_root.parent() != Some(canonical_root.as_path()) {
            return Err(SafetyError::fail_closed(
                "operation_directory_outside_owned_root",
            ));
        }
        if volume_probe.volume_id(&prepared_root)? != volume_probe.volume_id(&canonical_live)? {
            return Err(SafetyError::fail_closed("cross_volume_replacement_refused"));
        }
        Ok(prepared_root)
    })();
    let prepared_root = match prepared_root {
        Ok(root) => root,
        Err(error) => {
            let _ = fs::remove_dir(&operation_root);
            return Err(error);
        }
    };

    let backup = prepared_root.join("backup.db");
    let staging = prepared_root.join("staging.db");
    let state = prepared_root.join("state.json");
    if let Err(error) = ensure_distinct_paths(&[&canonical_live, &backup, &staging, &state]) {
        let _ = fs::remove_dir(&prepared_root);
        return Err(error);
    }

    let mut created = Vec::new();
    for path in [&staging, &state] {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)
        {
            Ok(_) => created.push(path.clone()),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                remove_created_paths(&created);
                let _ = fs::remove_dir(&prepared_root);
                return Err(SafetyError::fail_closed("operation_destination_conflict"));
            }
            Err(error) => {
                remove_created_paths(&created);
                let _ = fs::remove_dir(&prepared_root);
                return Err(SafetyError::fail_closed(format!(
                    "operation_destination_create_failed:{}",
                    error.kind()
                )));
            }
        }
    }

    let operation = OwnedOperation {
        operation_id: operation_id.to_string(),
        owned_root: canonical_root,
        root: prepared_root,
        live: canonical_live,
        backup,
        staging,
        state,
    };
    let state_record = operation.initial_state()?;
    if let Err(error) = write_state(&operation, &state_record) {
        remove_created_paths(&[operation.staging.clone(), operation.state.clone()]);
        let _ = fs::remove_dir(&operation.root);
        return Err(error);
    }
    Ok(operation)
}

impl OwnedOperation {
    fn initial_state(&self) -> Result<OperationState, SafetyError> {
        Ok(OperationState {
            schema_version: OPERATION_STATE_SCHEMA,
            operation_id: self.operation_id.clone(),
            operation_relative_directory: relative_filename(&self.root)?,
            live_relative_filename: relative_filename(&self.live)?,
            backup_relative_filename: relative_filename(&self.backup)?,
            staging_relative_filename: relative_filename(&self.staging)?,
            phase: OperationPhase::Prepared,
            live_before_sha256: None,
            expected_database_sha256: None,
            backup_source_manifest_digest: None,
            backup_schema_version: None,
            backup_foreign_keys_valid: None,
            backup_integrity_valid: None,
            backup_exact_record_digest: None,
            migration_id: None,
            migration_source_manifest_digest: None,
            migration_target_manifest_digest: None,
            outcome_class: None,
        })
    }

    fn verify_owned_path(&self, path: &Path) -> bool {
        path == self.backup || path == self.staging || path == self.state
    }
}

pub(crate) async fn create_owned_verified_backup<
    Q: QuiescenceProbe,
    P: VolumeProbe,
    V: CandidateVerifier,
    B: BackupCreator,
    D: DurabilityAdapter,
>(
    operation: &OwnedOperation,
    guard: &ExclusiveOperationGuard<'_, Q>,
    volume_probe: &P,
    verifier: &V,
    creator: &B,
    durability: &D,
) -> Result<ExpectedCandidate, SafetyError> {
    guard.require_quiescent()?;
    validate_backup_creation_preflight(operation)?;
    ensure_no_sidecars(&operation.live)?;
    let state = read_owned_state(operation)?;
    if state.phase != OperationPhase::Prepared {
        return Err(SafetyError::fail_closed(
            "backup_creation_requires_prepared_state",
        ));
    }
    validate_operation_volumes(operation, volume_probe)?;

    let source_before_sha256 = sha256_file(&operation.live)?;
    let source_before_identity = file_identity(&operation.live)?;
    let source_evidence = verifier.inspect(&operation.live).await?;
    validate_source_evidence(&source_evidence)?;

    // This is the final application-owned preflight immediately before the
    // single SQLite VACUUM INTO call. The pathname remains absent; ownership is
    // carried by the create-new operation directory and state record.
    guard.require_quiescent()?;
    validate_backup_creation_preflight(operation)?;
    ensure_no_sidecars(&operation.live)?;
    validate_operation_volumes(operation, volume_probe)?;
    let final_state = read_owned_state(operation)?;
    if final_state.phase != OperationPhase::Prepared {
        return Err(SafetyError::fail_closed("backup_creation_state_changed"));
    }
    if file_identity(&operation.live)? != source_before_identity {
        return Err(SafetyError::fail_closed("backup_source_identity_changed"));
    }
    if let Err(error) = creator.create(&operation.live, &operation.backup).await {
        return Err(handle_backup_creation_failure(operation, error));
    }

    if let Err(error) = guard.require_quiescent() {
        return Err(record_ambiguous_backup(operation, error.code));
    }
    if let Err(error) = ensure_no_sidecars(&operation.live) {
        return Err(record_ambiguous_backup(operation, error.code));
    }
    if let Err(error) = validate_operation_paths(operation, BackupPathRequirement::Existing) {
        return Err(record_ambiguous_backup(operation, error.code));
    }
    let source_after_sha256 = sha256_file(&operation.live)
        .map_err(|error| record_ambiguous_backup(operation, error.code))?;
    let source_after_identity = file_identity(&operation.live)
        .map_err(|error| record_ambiguous_backup(operation, error.code))?;
    if source_after_sha256 != source_before_sha256
        || source_after_identity != source_before_identity
    {
        return Err(record_ambiguous_backup(operation, "backup_source_changed"));
    }
    if let Err(error) = durability.sync_file(&operation.backup) {
        return Err(record_ambiguous_backup(operation, error.code));
    }

    let backup_evidence = verifier
        .inspect(&operation.backup)
        .await
        .map_err(|error| record_ambiguous_backup(operation, error.code))?;
    if backup_evidence != source_evidence {
        return Err(record_ambiguous_backup(
            operation,
            "backup_evidence_mismatch",
        ));
    }
    let expected = ExpectedCandidate {
        database_sha256: sha256_file(&operation.backup)
            .map_err(|error| record_ambiguous_backup(operation, error.code))?,
        source_manifest_digest: source_evidence.source_manifest_digest,
        schema_version: source_evidence.schema_version,
        foreign_keys_valid: source_evidence.foreign_keys_valid,
        integrity_valid: source_evidence.integrity_valid,
        exact_record_digest: source_evidence.exact_record_digest,
    };
    expected.validate_contract()?;
    verify_candidate(&operation.backup, &expected, verifier)
        .await
        .map_err(|error| record_ambiguous_backup(operation, error.code))?;
    record_verified_backup_state(operation, source_before_sha256, &expected)?;
    Ok(expected)
}

fn validate_operation_volumes<P: VolumeProbe>(
    operation: &OwnedOperation,
    volume_probe: &P,
) -> Result<(), SafetyError> {
    let root_volume = volume_probe.volume_id(&operation.owned_root)?;
    if volume_probe.volume_id(&operation.root)? != root_volume
        || volume_probe.volume_id(&operation.live)? != root_volume
    {
        return Err(SafetyError::fail_closed("cross_volume_backup_refused"));
    }
    Ok(())
}

fn validate_backup_creation_preflight(operation: &OwnedOperation) -> Result<(), SafetyError> {
    match validate_operation_paths(operation, BackupPathRequirement::Absent) {
        Ok(()) => Ok(()),
        Err(error) if fs::symlink_metadata(&operation.backup).is_ok() => {
            Err(record_ambiguous_backup(
                operation,
                format!("backup_preflight_refused:{}", error.code),
            ))
        }
        Err(error) => Err(error),
    }
}

fn validate_source_evidence(evidence: &CandidateEvidence) -> Result<(), SafetyError> {
    if evidence.schema_version != SUPPORTED_SCHEMA_VERSION {
        return Err(SafetyError::fail_closed(
            "source_schema_version_not_supported",
        ));
    }
    if !evidence.foreign_keys_valid {
        return Err(SafetyError::fail_closed("source_foreign_key_check_failed"));
    }
    if !evidence.integrity_valid {
        return Err(SafetyError::fail_closed("source_integrity_check_failed"));
    }
    for (label, digest) in [
        ("source_manifest", &evidence.source_manifest_digest),
        ("exact_record", &evidence.exact_record_digest),
    ] {
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(SafetyError::fail_closed(format!("{label}_digest_invalid")));
        }
    }
    Ok(())
}

fn handle_backup_creation_failure(
    operation: &OwnedOperation,
    original_error: SafetyError,
) -> SafetyError {
    match fs::symlink_metadata(&operation.backup) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if update_state(
                operation,
                OperationPhase::Prepared,
                None,
                None,
                Some(format!("backup_creation_refused:{}", original_error.code)),
            )
            .is_err()
            {
                SafetyError::recovery_required("backup_failure_state_record_failed")
            } else {
                original_error
            }
        }
        Err(_) => record_ambiguous_backup(operation, "backup_output_identity_unreadable"),
        Ok(_) => {
            let exact_owned =
                validate_direct_owned_existing_file(&operation.root, &operation.backup, "backup")
                    .and_then(|_| reject_multiple_links(&operation.backup));
            if exact_owned.is_err() {
                return record_ambiguous_backup(operation, "backup_output_ownership_ambiguous");
            }
            if fs::remove_file(&operation.backup).is_err() {
                return record_ambiguous_backup(
                    operation,
                    "owned_incomplete_backup_cleanup_failed",
                );
            }
            if update_state(
                operation,
                OperationPhase::Prepared,
                None,
                None,
                Some(format!(
                    "owned_incomplete_backup_removed:{}",
                    original_error.code
                )),
            )
            .is_err()
            {
                SafetyError::recovery_required("backup_cleanup_state_record_failed")
            } else {
                original_error
            }
        }
    }
}

fn record_ambiguous_backup(
    operation: &OwnedOperation,
    error_class: impl Into<String>,
) -> SafetyError {
    let error_class = error_class.into();
    if mark_recovery_required(operation, &error_class).is_err() {
        SafetyError::recovery_required("backup_recovery_state_record_failed")
    } else {
        SafetyError::recovery_required(error_class)
    }
}

pub(crate) async fn execute_replacement<
    Q: QuiescenceProbe,
    V: CandidateVerifier,
    D: DurabilityAdapter,
    R: ReplacementAdapter,
>(
    operation: &OwnedOperation,
    guard: &ExclusiveOperationGuard<'_, Q>,
    expected: &ExpectedCandidate,
    verifier: &V,
    durability: &D,
    replacement: &R,
) -> Result<ExecutionResult, SafetyError> {
    let prepared = prepare_staged_candidate(operation, guard, expected, verifier, durability).await;
    let live_before_sha256 = match prepared {
        Ok(digest) => digest,
        Err(error) => return Err(record_known_precommit_failure(operation, error)),
    };
    let live_before_identity = file_identity(&operation.live)?;

    if let Err(error) = guard.require_quiescent() {
        return Err(record_known_precommit_failure(operation, error));
    }
    if let Err(error) = ensure_no_sidecars(&operation.live) {
        return Err(record_known_precommit_failure(operation, error));
    }
    if let Err(error) = verify_candidate(&operation.backup, expected, verifier).await {
        return Err(record_known_precommit_failure(operation, error));
    }

    if replacement.requires_parent_directory_sync()
        && durability.parent_directory_support() == ParentDirectorySupport::Unsupported
    {
        return Err(record_known_precommit_failure(
            operation,
            SafetyError::fail_closed("parent_directory_sync_unsupported"),
        ));
    }

    match replacement.replace(&operation.staging, &operation.live) {
        ReplacementOutcome::FailedUnchanged { error_class } => {
            let current_live = sha256_file(&operation.live)?;
            let current_identity = file_identity(&operation.live)?;
            if current_live != live_before_sha256 || current_identity != live_before_identity {
                mark_recovery_required(operation, "replacement_failed_but_live_changed")?;
                return Err(SafetyError::recovery_required(
                    "replacement_failed_but_live_changed",
                ));
            }
            Err(record_known_precommit_failure(
                operation,
                SafetyError::fail_closed(format!("replacement_failed_unchanged:{error_class}")),
            ))
        }
        ReplacementOutcome::OutcomeUnknown { error_class } => {
            mark_recovery_required(
                operation,
                &format!("replacement_outcome_unknown:{error_class}"),
            )?;
            Err(SafetyError::recovery_required(format!(
                "replacement_outcome_unknown:{error_class}"
            )))
        }
        ReplacementOutcome::Committed => {
            update_state(
                operation,
                OperationPhase::ReplacementCommitted,
                Some(live_before_sha256),
                Some(expected.database_sha256.clone()),
                Some("committed".into()),
            )
            .map_err(|_| SafetyError::recovery_required("committed_state_record_failed"))?;

            if let Err(error) = durability.sync_parent(&operation.root) {
                mark_recovery_required(operation, "parent_directory_sync_failed")?;
                return Err(SafetyError::recovery_required(error.code));
            }
            if let Err(error) = verify_candidate(&operation.live, expected, verifier).await {
                mark_recovery_required(operation, "post_commit_verification_failed")?;
                return Err(SafetyError::recovery_required(format!(
                    "post_commit_verification_failed:{}",
                    error.code
                )));
            }

            match cleanup_owned_temporary(operation, &operation.staging) {
                Ok(()) => {
                    update_state(
                        operation,
                        OperationPhase::Completed,
                        None,
                        Some(expected.database_sha256.clone()),
                        Some("committed_and_verified".into()),
                    )
                    .map_err(|_| SafetyError::recovery_required("completed_state_record_failed"))?;
                    Ok(ExecutionResult::Completed)
                }
                Err(_) => {
                    update_state(
                        operation,
                        OperationPhase::CompletedWithCleanupRequired,
                        None,
                        Some(expected.database_sha256.clone()),
                        Some("cleanup_required".into()),
                    )
                    .map_err(|_| {
                        SafetyError::recovery_required("completed_cleanup_state_record_failed")
                    })?;
                    Ok(ExecutionResult::CompletedWithCleanupRequired)
                }
            }
        }
    }
}

fn record_known_precommit_failure(
    operation: &OwnedOperation,
    original_error: SafetyError,
) -> SafetyError {
    if let Err(cleanup_error) = cleanup_owned_temporary(operation, &operation.staging) {
        return cleanup_error;
    }
    if update_state(
        operation,
        OperationPhase::BackupVerified,
        None,
        None,
        Some(format!("precommit_refused:{}", original_error.code)),
    )
    .is_err()
    {
        return SafetyError::recovery_required("precommit_state_record_failed");
    }
    original_error
}

async fn prepare_staged_candidate<
    Q: QuiescenceProbe,
    V: CandidateVerifier,
    D: DurabilityAdapter,
>(
    operation: &OwnedOperation,
    guard: &ExclusiveOperationGuard<'_, Q>,
    expected: &ExpectedCandidate,
    verifier: &V,
    durability: &D,
) -> Result<String, SafetyError> {
    guard.require_quiescent()?;
    expected.validate_contract()?;
    validate_operation_paths(operation, BackupPathRequirement::Existing)?;
    let state = read_owned_state(operation)?;
    if state.phase != OperationPhase::BackupVerified
        || state.expected_database_sha256.as_deref() != Some(expected.database_sha256.as_str())
    {
        return Err(SafetyError::fail_closed(
            "replacement_requires_verified_backup_state",
        ));
    }
    ensure_no_sidecars(&operation.live)?;
    let live_before_sha256 = sha256_file(&operation.live)?;
    verify_candidate(&operation.backup, expected, verifier).await?;

    {
        let mut source = File::open(&operation.backup).map_err(|error| {
            SafetyError::fail_closed(format!("backup_open_failed:{}", error.kind()))
        })?;
        let mut staging = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&operation.staging)
            .map_err(|error| {
                SafetyError::fail_closed(format!("staging_open_failed:{}", error.kind()))
            })?;
        io::copy(&mut source, &mut staging).map_err(|error| {
            SafetyError::fail_closed(format!("staging_copy_failed:{}", error.kind()))
        })?;
        staging.flush().map_err(|error| {
            SafetyError::fail_closed(format!("staging_flush_failed:{}", error.kind()))
        })?;
    }
    durability.sync_file(&operation.staging)?;
    verify_candidate(&operation.staging, expected, verifier).await?;
    update_state(
        operation,
        OperationPhase::Staged,
        Some(live_before_sha256.clone()),
        Some(expected.database_sha256.clone()),
        Some("staged_and_verified".into()),
    )?;
    Ok(live_before_sha256)
}

async fn verify_candidate<V: CandidateVerifier>(
    path: &Path,
    expected: &ExpectedCandidate,
    verifier: &V,
) -> Result<(), SafetyError> {
    if sha256_file(path)? != expected.database_sha256 {
        return Err(SafetyError::fail_closed(
            "candidate_database_digest_mismatch",
        ));
    }
    let evidence = verifier.inspect(path).await?;
    if evidence.source_manifest_digest != expected.source_manifest_digest {
        return Err(SafetyError::fail_closed(
            "candidate_source_manifest_mismatch",
        ));
    }
    if evidence.schema_version != expected.schema_version {
        return Err(SafetyError::fail_closed(
            "candidate_schema_version_mismatch",
        ));
    }
    if evidence.foreign_keys_valid != expected.foreign_keys_valid || !evidence.foreign_keys_valid {
        return Err(SafetyError::fail_closed(
            "candidate_foreign_key_check_failed",
        ));
    }
    if evidence.integrity_valid != expected.integrity_valid || !evidence.integrity_valid {
        return Err(SafetyError::fail_closed("candidate_integrity_check_failed"));
    }
    if evidence.exact_record_digest != expected.exact_record_digest {
        return Err(SafetyError::fail_closed(
            "candidate_exact_record_digest_mismatch",
        ));
    }
    Ok(())
}

pub(crate) fn inspect_restart_state(
    operation: &OwnedOperation,
) -> Result<RestartInspection, SafetyError> {
    let bytes = match fs::read(&operation.state) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(RestartInspection::RecoveryRequired {
                error_class: "operation_state_missing".into(),
            });
        }
        Err(error) => {
            return Ok(RestartInspection::RecoveryRequired {
                error_class: format!("operation_state_unreadable:{}", error.kind()),
            });
        }
    };
    let state: OperationState = match serde_json::from_slice(&bytes) {
        Ok(state) => state,
        Err(_) => {
            return Ok(RestartInspection::RecoveryRequired {
                error_class: "operation_state_malformed".into(),
            });
        }
    };
    if !state_matches_operation(&state, operation) {
        return Ok(RestartInspection::RecoveryRequired {
            error_class: "operation_state_identity_mismatch".into(),
        });
    }

    let live_digest = sha256_file(&operation.live).ok();
    let backup_digest = sha256_file(&operation.backup).ok();
    let backup_exists = fs::symlink_metadata(&operation.backup).is_ok();
    let staging_exists = operation.staging.exists();
    let expected_matches = state
        .expected_database_sha256
        .as_ref()
        .zip(live_digest.as_ref())
        .map(|(expected, actual)| expected == actual)
        .unwrap_or(false);
    let backup_matches = state
        .expected_database_sha256
        .as_ref()
        .zip(backup_digest.as_ref())
        .map(|(expected, actual)| expected == actual)
        .unwrap_or(false);
    let original_matches = state
        .live_before_sha256
        .as_ref()
        .zip(live_digest.as_ref())
        .map(|(expected, actual)| expected == actual)
        .unwrap_or(false);

    Ok(match state.phase {
        OperationPhase::Prepared
            if !backup_exists && (state.live_before_sha256.is_none() || original_matches) =>
        {
            RestartInspection::Prepared
        }
        OperationPhase::Prepared => RestartInspection::RecoveryRequired {
            error_class: "prepared_state_contradictory".into(),
        },
        OperationPhase::BackupVerified if backup_matches && original_matches => {
            RestartInspection::BackupVerified
        }
        OperationPhase::BackupVerified => RestartInspection::RecoveryRequired {
            error_class: "backup_verified_state_contradictory".into(),
        },
        OperationPhase::Staged if staging_exists && original_matches => RestartInspection::Staged,
        OperationPhase::Staged => RestartInspection::RecoveryRequired {
            error_class: "staged_state_contradictory".into(),
        },
        OperationPhase::ReplacementCommitted if expected_matches => {
            RestartInspection::ReplacementCommitted
        }
        OperationPhase::ReplacementCommitted => RestartInspection::RecoveryRequired {
            error_class: "committed_state_unverified".into(),
        },
        OperationPhase::Completed if expected_matches && !staging_exists => {
            RestartInspection::Completed
        }
        OperationPhase::CompletedWithCleanupRequired if expected_matches => {
            RestartInspection::CompletedWithCleanupRequired
        }
        OperationPhase::Completed | OperationPhase::CompletedWithCleanupRequired => {
            RestartInspection::RecoveryRequired {
                error_class: "completed_state_contradictory".into(),
            }
        }
        OperationPhase::Migrating
        | OperationPhase::CommitOutcomeUnknown
        | OperationPhase::V5Verifying
        | OperationPhase::V5Ready
        | OperationPhase::V4ReadyWithBackup
        | OperationPhase::V5BlockedRestoreAvailable => RestartInspection::RecoveryRequired {
            error_class: "migration_state_requires_migration_classifier".into(),
        },
        OperationPhase::RecoveryRequired => RestartInspection::RecoveryRequired {
            error_class: state
                .outcome_class
                .unwrap_or_else(|| "recovery_required".into()),
        },
    })
}

pub(crate) fn cleanup_owned_temporary(
    operation: &OwnedOperation,
    candidate: &Path,
) -> Result<(), SafetyError> {
    if !operation.verify_owned_path(candidate) {
        return Err(SafetyError::fail_closed("cleanup_path_not_owned"));
    }
    if candidate != operation.staging {
        return Err(SafetyError::fail_closed("cleanup_not_authorized"));
    }
    match fs::remove_file(candidate) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(SafetyError::fail_closed(format!(
            "owned_staging_cleanup_failed:{}",
            error.kind()
        ))),
    }
}

#[derive(Clone, Copy)]
enum BackupPathRequirement {
    Absent,
    Existing,
}

fn validate_operation_paths(
    operation: &OwnedOperation,
    backup_requirement: BackupPathRequirement,
) -> Result<(), SafetyError> {
    reject_reparse_chain(&operation.owned_root)?;
    let owned_root = fs::canonicalize(&operation.owned_root)
        .map_err(|error| SafetyError::fail_closed(format!("owned_root_invalid:{error}")))?;
    if owned_root != operation.owned_root {
        return Err(SafetyError::fail_closed("owned_root_identity_changed"));
    }
    reject_reparse_chain(&operation.root)?;
    let operation_root = fs::canonicalize(&operation.root).map_err(|error| {
        SafetyError::fail_closed(format!("operation_directory_invalid:{error}"))
    })?;
    if operation_root != operation.root
        || operation_root.parent() != Some(operation.owned_root.as_path())
        || !operation_root.is_dir()
    {
        return Err(SafetyError::fail_closed(
            "operation_directory_identity_changed",
        ));
    }
    validate_direct_owned_existing_file(&owned_root, &operation.live, "live")?;
    reject_multiple_links(&operation.live)?;
    for (label, path) in [("staging", &operation.staging), ("state", &operation.state)] {
        validate_direct_owned_existing_file(&operation_root, path, label)?;
        reject_multiple_links(path)?;
    }
    match backup_requirement {
        BackupPathRequirement::Absent => {
            validate_direct_owned_absent_path(&operation_root, &operation.backup, "backup")?;
        }
        BackupPathRequirement::Existing => {
            validate_direct_owned_existing_file(&operation_root, &operation.backup, "backup")?;
            reject_multiple_links(&operation.backup)?;
        }
    }
    ensure_distinct_paths(&[
        &operation.live,
        &operation.backup,
        &operation.staging,
        &operation.state,
    ])
}

fn validate_direct_owned_absent_path(
    canonical_root: &Path,
    path: &Path,
    label: &str,
) -> Result<(), SafetyError> {
    reject_lexical_alias(path)?;
    reject_reparse_chain(path)?;
    if path.parent() != Some(canonical_root) {
        return Err(SafetyError::fail_closed(format!(
            "{label}_path_outside_owned_root"
        )));
    }
    if path.exists() {
        return Err(SafetyError::fail_closed(format!(
            "{label}_destination_exists"
        )));
    }
    Ok(())
}

fn validate_direct_owned_existing_file(
    canonical_root: &Path,
    path: &Path,
    label: &str,
) -> Result<PathBuf, SafetyError> {
    reject_lexical_alias(path)?;
    reject_reparse_chain(path)?;
    let canonical = fs::canonicalize(path).map_err(|error| {
        SafetyError::fail_closed(format!("{label}_path_invalid:{}", error.kind()))
    })?;
    if canonical.parent() != Some(canonical_root) {
        return Err(SafetyError::fail_closed(format!(
            "{label}_path_outside_owned_root"
        )));
    }
    let metadata = fs::metadata(&canonical).map_err(|error| {
        SafetyError::fail_closed(format!("{label}_metadata_failed:{}", error.kind()))
    })?;
    if !metadata.is_file() {
        return Err(SafetyError::fail_closed(format!("{label}_path_not_file")));
    }
    Ok(canonical)
}

fn reject_lexical_alias(path: &Path) -> Result<(), SafetyError> {
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
    {
        return Err(SafetyError::fail_closed("path_alias_or_traversal_refused"));
    }
    Ok(())
}

fn reject_reparse_point(path: &Path) -> Result<(), SafetyError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| SafetyError::fail_closed(format!("path_metadata_failed:{error}")))?;
    reject_link_flags(
        metadata.file_type().is_symlink(),
        platform_reparse_point(&metadata),
    )
}

fn reject_reparse_chain(path: &Path) -> Result<(), SafetyError> {
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component.as_os_str());
        if matches!(component, Component::Prefix(_) | Component::RootDir) {
            continue;
        }
        if current.exists() {
            reject_reparse_point(&current)?;
        }
    }
    Ok(())
}

fn reject_link_flags(is_symlink: bool, is_reparse_point: bool) -> Result<(), SafetyError> {
    if is_symlink || is_reparse_point {
        return Err(SafetyError::fail_closed("symlink_or_reparse_point_refused"));
    }
    Ok(())
}

#[cfg(windows)]
fn platform_reparse_point(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(windows)]
fn windows_file_information(
    path: &Path,
) -> Result<windows_sys::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION, SafetyError> {
    use std::{mem::MaybeUninit, os::windows::io::AsRawHandle};
    use windows_sys::Win32::{
        Foundation::HANDLE,
        Storage::FileSystem::{GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION},
    };

    let file = File::open(path).map_err(|error| {
        SafetyError::fail_closed(format!(
            "windows_file_identity_open_failed:{}",
            error.kind()
        ))
    })?;
    let mut information = MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::zeroed();
    let succeeded = unsafe {
        GetFileInformationByHandle(file.as_raw_handle() as HANDLE, information.as_mut_ptr())
    };
    if succeeded == 0 {
        return Err(SafetyError::fail_closed(
            "windows_file_identity_unavailable",
        ));
    }
    Ok(unsafe { information.assume_init() })
}

#[cfg(not(windows))]
fn platform_reparse_point(_metadata: &fs::Metadata) -> bool {
    false
}

fn reject_multiple_links(path: &Path) -> Result<(), SafetyError> {
    let metadata = fs::metadata(path)
        .map_err(|error| SafetyError::fail_closed(format!("file_identity_failed:{error}")))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() > 1 {
            return Err(SafetyError::fail_closed("file_alias_refused"));
        }
    }

    #[cfg(windows)]
    {
        let _ = metadata;
        if windows_file_information(path)?.nNumberOfLinks > 1 {
            return Err(SafetyError::fail_closed("file_alias_refused"));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileIdentity {
    volume: u64,
    file: u128,
}

fn file_identity(path: &Path) -> Result<FileIdentity, SafetyError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::metadata(path)
            .map_err(|error| SafetyError::fail_closed(format!("file_identity_failed:{error}")))?;
        Ok(FileIdentity {
            volume: metadata.dev(),
            file: metadata.ino() as u128,
        })
    }

    #[cfg(windows)]
    {
        let information = windows_file_information(path)?;
        Ok(FileIdentity {
            volume: information.dwVolumeSerialNumber as u64,
            file: ((information.nFileIndexHigh as u128) << 32) | information.nFileIndexLow as u128,
        })
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = path;
        Err(SafetyError::fail_closed("file_identity_unsupported"))
    }
}

fn ensure_distinct_paths(paths: &[&Path]) -> Result<(), SafetyError> {
    for (index, path) in paths.iter().enumerate() {
        if paths.iter().skip(index + 1).any(|other| *other == *path) {
            return Err(SafetyError::fail_closed("operation_paths_must_be_distinct"));
        }
    }
    Ok(())
}

fn ensure_no_sidecars(live: &Path) -> Result<(), SafetyError> {
    for (suffix, label) in [
        ("-wal", "wal"),
        ("-shm", "shm"),
        ("-journal", "rollback_journal"),
    ] {
        let sidecar = append_suffix(live, suffix);
        if sidecar.exists() {
            return Err(SafetyError::fail_closed(format!(
                "sqlite_{label}_sidecar_present"
            )));
        }
    }
    Ok(())
}

/// Observes only the exact app-owned database boundary needed by the explicit
/// readiness inspector. It never creates a path, opens a writable handle, or
/// cleans up sidecars/operation evidence.
pub(crate) fn inspect_readiness_filesystem(
    root: &Path,
    live: &Path,
) -> Result<ReadinessFilesystemSnapshot, ReadinessFilesystemFailure> {
    reject_lexical_alias(root).map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;
    reject_lexical_alias(live).map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;
    if live.parent() != Some(root) || live.file_name().and_then(|name| name.to_str()) != Some("life-os.db") {
        return Err(ReadinessFilesystemFailure::PathUnsafe);
    }
    reject_reparse_chain(root).map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;

    match fs::symlink_metadata(root) {
        Ok(metadata) => {
            reject_link_flags(
                metadata.file_type().is_symlink(),
                platform_reparse_point(&metadata),
            )
            .map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;
            if !metadata.is_dir() {
                return Err(ReadinessFilesystemFailure::PathUnsafe);
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(ReadinessFilesystemSnapshot {
                database_exists: false,
                wal_present: false,
                shm_present: false,
                rollback_journal_present: false,
                operation_evidence: ReadinessOperationEvidence::None,
            });
        }
        Err(_) => return Err(ReadinessFilesystemFailure::Unreadable),
    }

    let canonical_root = fs::canonicalize(root)
        .map_err(|_| ReadinessFilesystemFailure::Unreadable)?;
    let root_metadata = fs::metadata(&canonical_root)
        .map_err(|_| ReadinessFilesystemFailure::Unreadable)?;
    if !root_metadata.is_dir() {
        return Err(ReadinessFilesystemFailure::PathUnsafe);
    }

    let database_exists = match fs::symlink_metadata(live) {
        Ok(metadata) => {
            reject_link_flags(
                metadata.file_type().is_symlink(),
                platform_reparse_point(&metadata),
            )
            .map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;
            if !metadata.is_file() {
                return Err(ReadinessFilesystemFailure::PathUnsafe);
            }
            let canonical_live = fs::canonicalize(live)
                .map_err(|_| ReadinessFilesystemFailure::Unreadable)?;
            if canonical_live.parent() != Some(canonical_root.as_path()) {
                return Err(ReadinessFilesystemFailure::PathUnsafe);
            }
            reject_multiple_links(&canonical_live)
                .map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;
            true
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(_) => return Err(ReadinessFilesystemFailure::Unreadable),
    };

    let sidecar_present = |suffix: &str| -> Result<bool, ReadinessFilesystemFailure> {
        let path = append_suffix(live, suffix);
        match fs::symlink_metadata(&path) {
            Ok(metadata) => {
                reject_link_flags(
                    metadata.file_type().is_symlink(),
                    platform_reparse_point(&metadata),
                )
                .map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;
                if !metadata.is_file() {
                    return Err(ReadinessFilesystemFailure::PathUnsafe);
                }
                let canonical_sidecar = fs::canonicalize(&path)
                    .map_err(|_| ReadinessFilesystemFailure::Unreadable)?;
                if canonical_sidecar.parent() != Some(canonical_root.as_path()) {
                    return Err(ReadinessFilesystemFailure::PathUnsafe);
                }
                reject_multiple_links(&canonical_sidecar)
                    .map_err(|_| ReadinessFilesystemFailure::PathUnsafe)?;
                Ok(true)
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(_) => Err(ReadinessFilesystemFailure::Unreadable),
        }
    };

    let mut operation_directories = Vec::new();
    for entry in fs::read_dir(&canonical_root).map_err(|_| ReadinessFilesystemFailure::Unreadable)? {
        let entry = entry.map_err(|_| ReadinessFilesystemFailure::Unreadable)?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            return Err(ReadinessFilesystemFailure::RecoveryRequired);
        };
        if name.starts_with("life-os-") && name.ends_with(".operation") {
            operation_directories.push(entry.path());
        }
    }
    if operation_directories.len() > 1 {
        return Err(ReadinessFilesystemFailure::RecoveryRequired);
    }
    let operation_evidence = if let Some(operation_root) = operation_directories.pop() {
        let metadata = fs::symlink_metadata(&operation_root)
            .map_err(|_| ReadinessFilesystemFailure::RecoveryRequired)?;
        if metadata.file_type().is_symlink()
            || platform_reparse_point(&metadata)
            || !metadata.is_dir()
        {
            return Err(ReadinessFilesystemFailure::RecoveryRequired);
        }
        let canonical_operation = fs::canonicalize(&operation_root)
            .map_err(|_| ReadinessFilesystemFailure::RecoveryRequired)?;
        if canonical_operation.parent() != Some(canonical_root.as_path()) {
            return Err(ReadinessFilesystemFailure::RecoveryRequired);
        }
        let name = canonical_operation
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or(ReadinessFilesystemFailure::RecoveryRequired)?;
        let operation_id = name
            .strip_prefix("life-os-")
            .and_then(|value| value.strip_suffix(".operation"))
            .ok_or(ReadinessFilesystemFailure::RecoveryRequired)?;
        validate_operation_id(operation_id)
            .map_err(|_| ReadinessFilesystemFailure::RecoveryRequired)?;
        let operation = OwnedOperation {
            operation_id: operation_id.to_string(),
            owned_root: canonical_root.clone(),
            root: canonical_operation.clone(),
            live: fs::canonicalize(live)
                .map_err(|_| ReadinessFilesystemFailure::RecoveryRequired)?,
            backup: canonical_operation.join("backup.db"),
            staging: canonical_operation.join("staging.db"),
            state: canonical_operation.join("state.json"),
        };
        read_owned_state(&operation)
            .map_err(|_| ReadinessFilesystemFailure::RecoveryRequired)?;
        ReadinessOperationEvidence::Present
    } else {
        ReadinessOperationEvidence::None
    };

    Ok(ReadinessFilesystemSnapshot {
        database_exists,
        wal_present: sidecar_present("-wal")?,
        shm_present: sidecar_present("-shm")?,
        rollback_journal_present: sidecar_present("-journal")?,
        operation_evidence,
    })
}

fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut value = path.as_os_str().to_os_string();
    value.push(suffix);
    PathBuf::from(value)
}

fn generate_operation_id(root: &Path) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let counter = OPERATION_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut digest = Sha256::new();
    digest.update(b"life-os/filesystem-operation-v1\0");
    digest.update(process::id().to_be_bytes());
    digest.update(now.to_be_bytes());
    digest.update(counter.to_be_bytes());
    digest.update(root.as_os_str().to_string_lossy().as_bytes());
    format!("{:x}", digest.finalize())[..32].to_string()
}

fn validate_operation_id(operation_id: &str) -> Result<(), SafetyError> {
    if operation_id.len() != 32
        || !operation_id
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(SafetyError::fail_closed("operation_id_invalid"));
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, SafetyError> {
    let mut file = File::open(path).map_err(|error| {
        SafetyError::fail_closed(format!("database_digest_open_failed:{}", error.kind()))
    })?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer).map_err(|error| {
            SafetyError::fail_closed(format!("database_digest_read_failed:{}", error.kind()))
        })?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn relative_filename(path: &Path) -> Result<String, SafetyError> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .ok_or_else(|| SafetyError::fail_closed("relative_filename_invalid"))
}

fn write_state(operation: &OwnedOperation, state: &OperationState) -> Result<(), SafetyError> {
    if !state_matches_operation(state, operation) {
        return Err(SafetyError::fail_closed(
            "operation_state_identity_mismatch",
        ));
    }
    let bytes = serde_json::to_vec(state)
        .map_err(|_| SafetyError::fail_closed("operation_state_encode_failed"))?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&operation.state)
        .map_err(|error| {
            SafetyError::fail_closed(format!("operation_state_open_failed:{}", error.kind()))
        })?;
    file.seek(SeekFrom::Start(0)).map_err(|error| {
        SafetyError::fail_closed(format!("operation_state_seek_failed:{}", error.kind()))
    })?;
    file.write_all(&bytes).map_err(|error| {
        SafetyError::fail_closed(format!("operation_state_write_failed:{}", error.kind()))
    })?;
    file.sync_all().map_err(|error| {
        SafetyError::fail_closed(format!("operation_state_sync_failed:{}", error.kind()))
    })
}

fn update_state(
    operation: &OwnedOperation,
    phase: OperationPhase,
    live_before_sha256: Option<String>,
    expected_database_sha256: Option<String>,
    outcome_class: Option<String>,
) -> Result<(), SafetyError> {
    let mut state = read_owned_state(operation)?;
    state.phase = phase;
    if live_before_sha256.is_some() {
        state.live_before_sha256 = live_before_sha256;
    }
    if expected_database_sha256.is_some() {
        state.expected_database_sha256 = expected_database_sha256;
    }
    state.outcome_class = outcome_class;
    write_state(operation, &state)
}

fn record_verified_backup_state(
    operation: &OwnedOperation,
    live_before_sha256: String,
    expected: &ExpectedCandidate,
) -> Result<(), SafetyError> {
    expected.validate_contract()?;
    let mut state = read_owned_state(operation)?;
    if state.phase != OperationPhase::Prepared {
        return Err(SafetyError::fail_closed("backup_state_requires_prepared"));
    }
    state.phase = OperationPhase::BackupVerified;
    state.live_before_sha256 = Some(live_before_sha256);
    state.expected_database_sha256 = Some(expected.database_sha256.clone());
    state.backup_source_manifest_digest = Some(expected.source_manifest_digest.clone());
    state.backup_schema_version = Some(expected.schema_version);
    state.backup_foreign_keys_valid = Some(expected.foreign_keys_valid);
    state.backup_integrity_valid = Some(expected.integrity_valid);
    state.backup_exact_record_digest = Some(expected.exact_record_digest.clone());
    state.outcome_class = Some("backup_verified".into());
    write_state(operation, &state)
}

fn migration_phase(phase: OperationPhase) -> Result<MigrationOperationPhase, SafetyError> {
    match phase {
        OperationPhase::Prepared => Ok(MigrationOperationPhase::Prepared),
        OperationPhase::BackupVerified => Ok(MigrationOperationPhase::BackupVerified),
        OperationPhase::Migrating => Ok(MigrationOperationPhase::Migrating),
        OperationPhase::CommitOutcomeUnknown => Ok(MigrationOperationPhase::CommitOutcomeUnknown),
        OperationPhase::V5Verifying => Ok(MigrationOperationPhase::V5Verifying),
        OperationPhase::V5Ready => Ok(MigrationOperationPhase::V5Ready),
        OperationPhase::V4ReadyWithBackup => Ok(MigrationOperationPhase::V4ReadyWithBackup),
        OperationPhase::V5BlockedRestoreAvailable => {
            Ok(MigrationOperationPhase::V5BlockedRestoreAvailable)
        }
        OperationPhase::RecoveryRequired => Ok(MigrationOperationPhase::RecoveryRequired),
        OperationPhase::Staged
        | OperationPhase::ReplacementCommitted
        | OperationPhase::Completed
        | OperationPhase::CompletedWithCleanupRequired => Err(SafetyError::recovery_required(
            "operation_state_not_migration",
        )),
    }
}

fn expected_backup_from_state(
    state: &OperationState,
) -> Result<Option<ExpectedCandidate>, SafetyError> {
    let fields_present = [
        state.expected_database_sha256.is_some(),
        state.backup_source_manifest_digest.is_some(),
        state.backup_schema_version.is_some(),
        state.backup_foreign_keys_valid.is_some(),
        state.backup_integrity_valid.is_some(),
        state.backup_exact_record_digest.is_some(),
    ];
    if fields_present.iter().all(|present| !present) {
        return Ok(None);
    }
    if !fields_present.iter().all(|present| *present) {
        return Err(SafetyError::recovery_required(
            "migration_backup_state_incomplete",
        ));
    }
    let expected = ExpectedCandidate {
        database_sha256: state.expected_database_sha256.clone().unwrap(),
        source_manifest_digest: state.backup_source_manifest_digest.clone().unwrap(),
        schema_version: state.backup_schema_version.unwrap(),
        foreign_keys_valid: state.backup_foreign_keys_valid.unwrap(),
        integrity_valid: state.backup_integrity_valid.unwrap(),
        exact_record_digest: state.backup_exact_record_digest.clone().unwrap(),
    };
    expected
        .validate_contract()
        .map_err(|error| SafetyError::recovery_required(error.code))?;
    Ok(Some(expected))
}

pub(crate) fn read_migration_state_evidence(
    operation: &OwnedOperation,
) -> Result<MigrationStateEvidence, SafetyError> {
    let state = read_owned_state(operation)?;
    let phase = migration_phase(state.phase)?;
    let verified_backup = expected_backup_from_state(&state)?;
    if phase != MigrationOperationPhase::Prepared && verified_backup.is_none() {
        return Err(SafetyError::recovery_required(
            "migration_backup_evidence_missing",
        ));
    }
    let migration_fields_present = [
        state.migration_id.is_some(),
        state.migration_source_manifest_digest.is_some(),
        state.migration_target_manifest_digest.is_some(),
    ];
    if !migration_fields_present.iter().all(|present| *present)
        && migration_fields_present.iter().any(|present| *present)
    {
        return Err(SafetyError::recovery_required(
            "migration_receipt_state_incomplete",
        ));
    }
    if matches!(
        phase,
        MigrationOperationPhase::V5Verifying
            | MigrationOperationPhase::V5Ready
            | MigrationOperationPhase::V5BlockedRestoreAvailable
    ) && !migration_fields_present.iter().all(|present| *present)
    {
        return Err(SafetyError::recovery_required(
            "migration_receipt_state_missing",
        ));
    }
    Ok(MigrationStateEvidence {
        phase,
        verified_backup,
        live_before_sha256: state.live_before_sha256,
        migration_id: state.migration_id,
        migration_source_manifest_digest: state.migration_source_manifest_digest,
        migration_target_manifest_digest: state.migration_target_manifest_digest,
        outcome_class: state.outcome_class,
    })
}

pub(crate) fn record_migration_state(
    operation: &OwnedOperation,
    phase: MigrationOperationPhase,
    outcome_class: Option<String>,
    receipt: Option<MigrationReceiptEvidence<'_>>,
) -> Result<(), SafetyError> {
    let mut state = read_owned_state(operation)?;
    let current = migration_phase(state.phase)?;
    let transition_allowed = match phase {
        MigrationOperationPhase::Prepared => current == MigrationOperationPhase::Prepared,
        MigrationOperationPhase::BackupVerified => {
            current == MigrationOperationPhase::BackupVerified
        }
        MigrationOperationPhase::Migrating => current == MigrationOperationPhase::BackupVerified,
        MigrationOperationPhase::CommitOutcomeUnknown => {
            current == MigrationOperationPhase::Migrating
        }
        MigrationOperationPhase::V5Verifying => matches!(
            current,
            MigrationOperationPhase::Migrating
                | MigrationOperationPhase::CommitOutcomeUnknown
                | MigrationOperationPhase::V5Verifying
        ),
        MigrationOperationPhase::V5Ready => matches!(
            current,
            MigrationOperationPhase::Migrating
                | MigrationOperationPhase::CommitOutcomeUnknown
                | MigrationOperationPhase::V5Verifying
                | MigrationOperationPhase::V5Ready
        ),
        MigrationOperationPhase::V4ReadyWithBackup => matches!(
            current,
            MigrationOperationPhase::BackupVerified
                | MigrationOperationPhase::Migrating
                | MigrationOperationPhase::CommitOutcomeUnknown
                | MigrationOperationPhase::V4ReadyWithBackup
        ),
        MigrationOperationPhase::V5BlockedRestoreAvailable => matches!(
            current,
            MigrationOperationPhase::Migrating
                | MigrationOperationPhase::CommitOutcomeUnknown
                | MigrationOperationPhase::V5Verifying
                | MigrationOperationPhase::V5BlockedRestoreAvailable
        ),
        MigrationOperationPhase::RecoveryRequired => true,
    };
    if !transition_allowed {
        return Err(SafetyError::recovery_required(format!(
            "migration_state_transition_refused:{current:?}:{phase:?}"
        )));
    }
    if phase != MigrationOperationPhase::Prepared && expected_backup_from_state(&state)?.is_none() {
        return Err(SafetyError::recovery_required(
            "migration_backup_evidence_missing",
        ));
    }
    state.phase = match phase {
        MigrationOperationPhase::Prepared => OperationPhase::Prepared,
        MigrationOperationPhase::BackupVerified => OperationPhase::BackupVerified,
        MigrationOperationPhase::Migrating => OperationPhase::Migrating,
        MigrationOperationPhase::CommitOutcomeUnknown => OperationPhase::CommitOutcomeUnknown,
        MigrationOperationPhase::V5Verifying => OperationPhase::V5Verifying,
        MigrationOperationPhase::V5Ready => OperationPhase::V5Ready,
        MigrationOperationPhase::V4ReadyWithBackup => OperationPhase::V4ReadyWithBackup,
        MigrationOperationPhase::V5BlockedRestoreAvailable => {
            OperationPhase::V5BlockedRestoreAvailable
        }
        MigrationOperationPhase::RecoveryRequired => OperationPhase::RecoveryRequired,
    };
    if let Some(receipt) = receipt {
        state.migration_id = Some(receipt.migration_id.to_string());
        state.migration_source_manifest_digest = Some(receipt.source_manifest_digest.to_string());
        state.migration_target_manifest_digest = Some(receipt.target_manifest_digest.to_string());
    }
    state.outcome_class = outcome_class;
    write_state(operation, &state)
}

pub(crate) async fn verify_owned_backup_for_migration<V: CandidateVerifier>(
    operation: &OwnedOperation,
    expected: &ExpectedCandidate,
    verifier: &V,
) -> Result<(), SafetyError> {
    expected.validate_contract()?;
    validate_operation_paths(operation, BackupPathRequirement::Existing)?;
    let state = read_migration_state_evidence(operation)?;
    if state.verified_backup.as_ref() != Some(expected) {
        return Err(SafetyError::recovery_required(
            "migration_backup_state_mismatch",
        ));
    }
    verify_candidate(&operation.backup, expected, verifier)
        .await
        .map_err(|error| SafetyError::recovery_required(error.code))
}

fn mark_recovery_required(
    operation: &OwnedOperation,
    error_class: &str,
) -> Result<(), SafetyError> {
    update_state(
        operation,
        OperationPhase::RecoveryRequired,
        None,
        None,
        Some(error_class.to_string()),
    )
}

fn read_owned_state(operation: &OwnedOperation) -> Result<OperationState, SafetyError> {
    let bytes = fs::read(&operation.state).map_err(|error| {
        SafetyError::recovery_required(format!("operation_state_read_failed:{}", error.kind()))
    })?;
    let state: OperationState = serde_json::from_slice(&bytes)
        .map_err(|_| SafetyError::recovery_required("operation_state_malformed"))?;
    if !state_matches_operation(&state, operation) {
        return Err(SafetyError::recovery_required(
            "operation_state_identity_mismatch",
        ));
    }
    Ok(state)
}

fn state_matches_operation(state: &OperationState, operation: &OwnedOperation) -> bool {
    state.schema_version == OPERATION_STATE_SCHEMA
        && state.operation_id == operation.operation_id
        && Some(state.operation_relative_directory.as_str())
            == operation.root.file_name().and_then(|name| name.to_str())
        && Some(state.live_relative_filename.as_str())
            == operation.live.file_name().and_then(|name| name.to_str())
        && Some(state.backup_relative_filename.as_str())
            == operation.backup.file_name().and_then(|name| name.to_str())
        && Some(state.staging_relative_filename.as_str())
            == operation.staging.file_name().and_then(|name| name.to_str())
}

fn remove_created_paths(paths: &[PathBuf]) {
    for path in paths.iter().rev() {
        let _ = fs::remove_file(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use sqlx::{Connection, Executor, Row, SqliteConnection};
    use std::sync::atomic::{AtomicU8, AtomicUsize};
    use tempfile::TempDir;

    const FIXED_ID: &str = "0123456789abcdef0123456789abcdef";

    struct MutableQuiescence(AtomicU8);

    impl MutableQuiescence {
        fn new(activity: DatabaseActivity) -> Self {
            Self(AtomicU8::new(match activity {
                DatabaseActivity::Quiescent => 0,
                DatabaseActivity::Active => 1,
                DatabaseActivity::Unknown => 2,
            }))
        }

        fn set(&self, activity: DatabaseActivity) {
            self.0.store(
                match activity {
                    DatabaseActivity::Quiescent => 0,
                    DatabaseActivity::Active => 1,
                    DatabaseActivity::Unknown => 2,
                },
                Ordering::SeqCst,
            );
        }
    }

    impl QuiescenceProbe for MutableQuiescence {
        fn database_activity(&self) -> DatabaseActivity {
            match self.0.load(Ordering::SeqCst) {
                0 => DatabaseActivity::Quiescent,
                1 => DatabaseActivity::Active,
                _ => DatabaseActivity::Unknown,
            }
        }
    }

    struct FixedVolumeProbe {
        root_volume: u64,
        live_volume: u64,
    }

    impl VolumeProbe for FixedVolumeProbe {
        fn volume_id(&self, path: &Path) -> Result<u64, SafetyError> {
            if path.is_dir() {
                Ok(self.root_volume)
            } else {
                Ok(self.live_volume)
            }
        }
    }

    struct ChangingVolumeProbe {
        calls: AtomicUsize,
    }

    impl VolumeProbe for ChangingVolumeProbe {
        fn volume_id(&self, path: &Path) -> Result<u64, SafetyError> {
            let call = self.calls.fetch_add(1, Ordering::SeqCst);
            if call >= 3 && path.is_file() {
                Ok(8)
            } else {
                Ok(7)
            }
        }
    }

    struct SqliteV4Verifier {
        calls: AtomicUsize,
        fail_on_call: Option<usize>,
        override_evidence: Option<CandidateEvidence>,
    }

    impl SqliteV4Verifier {
        fn exact() -> Self {
            Self {
                calls: AtomicUsize::new(0),
                fail_on_call: None,
                override_evidence: None,
            }
        }

        fn failing_on(call: usize) -> Self {
            Self {
                calls: AtomicUsize::new(0),
                fail_on_call: Some(call),
                override_evidence: None,
            }
        }

        fn overriding(evidence: CandidateEvidence) -> Self {
            Self {
                calls: AtomicUsize::new(0),
                fail_on_call: None,
                override_evidence: Some(evidence),
            }
        }
    }

    impl CandidateVerifier for SqliteV4Verifier {
        async fn inspect(&self, path: &Path) -> Result<CandidateEvidence, SafetyError> {
            let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
            if self.fail_on_call == Some(call) {
                return Err(SafetyError::fail_closed("injected_candidate_failure"));
            }
            if let Some(evidence) = &self.override_evidence {
                return Ok(evidence.clone());
            }
            inspect_v4(path).await
        }
    }

    struct CountingSystemVacuum {
        calls: AtomicUsize,
    }

    impl CountingSystemVacuum {
        fn new() -> Self {
            Self {
                calls: AtomicUsize::new(0),
            }
        }
    }

    impl BackupCreator for CountingSystemVacuum {
        async fn create(&self, source: &Path, destination: &Path) -> Result<(), SafetyError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            SystemVacuumInto.create(source, destination).await
        }
    }

    enum FailingBackupCreator {
        ExactOwnedOutput,
        AmbiguousHardLink,
    }

    impl BackupCreator for FailingBackupCreator {
        async fn create(&self, source: &Path, destination: &Path) -> Result<(), SafetyError> {
            match self {
                Self::ExactOwnedOutput => {
                    fs::write(destination, b"incomplete").map_err(SafetyError::from)?
                }
                Self::AmbiguousHardLink => {
                    fs::hard_link(source, destination).map_err(SafetyError::from)?
                }
            }
            Err(SafetyError::fail_closed("injected_backup_creation_failure"))
        }
    }

    struct SourceReplacingVerifier {
        calls: AtomicUsize,
    }

    impl CandidateVerifier for SourceReplacingVerifier {
        async fn inspect(&self, path: &Path) -> Result<CandidateEvidence, SafetyError> {
            let evidence = inspect_v4(path).await?;
            if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
                let replacement = path.with_extension("replacement");
                fs::copy(path, &replacement).map_err(SafetyError::from)?;
                fs::remove_file(path).map_err(SafetyError::from)?;
                fs::rename(replacement, path).map_err(SafetyError::from)?;
            }
            Ok(evidence)
        }
    }

    struct ActivityChangingVerifier<'a> {
        calls: AtomicUsize,
        probe: &'a MutableQuiescence,
    }

    impl CandidateVerifier for ActivityChangingVerifier<'_> {
        async fn inspect(&self, path: &Path) -> Result<CandidateEvidence, SafetyError> {
            let evidence = inspect_v4(path).await?;
            if self.calls.fetch_add(1, Ordering::SeqCst) + 1 == 2 {
                self.probe.set(DatabaseActivity::Active);
            }
            Ok(evidence)
        }
    }

    #[derive(Clone, Copy)]
    struct TestDurability {
        support: ParentDirectorySupport,
        fail_file: bool,
        fail_parent: bool,
    }

    impl TestDurability {
        fn supported() -> Self {
            Self {
                support: ParentDirectorySupport::Supported,
                fail_file: false,
                fail_parent: false,
            }
        }
    }

    impl DurabilityAdapter for TestDurability {
        fn sync_file(&self, path: &Path) -> Result<(), SafetyError> {
            if self.fail_file {
                return Err(SafetyError::fail_closed("injected_file_sync_failure"));
            }
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .and_then(|file| file.sync_all())
                .map_err(SafetyError::from)
        }

        fn parent_directory_support(&self) -> ParentDirectorySupport {
            self.support
        }

        fn sync_parent(&self, _parent: &Path) -> Result<(), SafetyError> {
            if self.fail_parent {
                Err(SafetyError::recovery_required(
                    "injected_parent_sync_failure",
                ))
            } else {
                Ok(())
            }
        }
    }

    enum TestReplacement {
        CommitByCopy,
        FailedUnchanged,
        UnknownAfterWrite,
    }

    impl ReplacementAdapter for TestReplacement {
        fn requires_parent_directory_sync(&self) -> bool {
            true
        }

        fn replace(&self, staging: &Path, live: &Path) -> ReplacementOutcome {
            match self {
                Self::CommitByCopy => {
                    fs::copy(staging, live).unwrap();
                    ReplacementOutcome::Committed
                }
                Self::FailedUnchanged => ReplacementOutcome::FailedUnchanged {
                    error_class: "injected_permission".into(),
                },
                Self::UnknownAfterWrite => {
                    fs::copy(staging, live).unwrap();
                    ReplacementOutcome::OutcomeUnknown {
                        error_class: "injected_interruption".into(),
                    }
                }
            }
        }
    }

    async fn fixture(content: &str) -> (TempDir, PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        create_v4_database(&path, content).await;
        (directory, path)
    }

    async fn create_v4_database(path: &Path, content: &str) {
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        connection
            .execute(include_str!("../tests/fixtures/schema_v5/v4.sql"))
            .await
            .unwrap();
        sqlx::query("INSERT INTO experience_entries (id, content, created_at, updated_at) VALUES ('exp-1', ?, '2026-07-25T00:00:00Z', '2026-07-25T00:00:00Z')")
            .bind(content)
            .execute(&mut connection)
            .await
            .unwrap();
        connection.execute("PRAGMA user_version = 4").await.unwrap();
        connection.close().await.unwrap();
    }

    #[derive(Clone, Copy)]
    struct TableManifestSpec {
        name: &'static str,
        query: &'static str,
        field_count: usize,
    }

    const TABLE_MANIFESTS: [TableManifestSpec; 6] = [
        TableManifestSpec {
            name: "experience_entries",
            query: "SELECT id, content, created_at, updated_at FROM experience_entries ORDER BY id",
            field_count: 4,
        },
        TableManifestSpec {
            name: "persisted_artifacts",
            query: "SELECT id, source_entry_id, artifact_kind, payload, created_at, updated_at FROM persisted_artifacts ORDER BY id",
            field_count: 6,
        },
        TableManifestSpec {
            name: "historical_consent_events",
            query: "SELECT id, packet_digest, payload, state, created_at, expires_at FROM historical_consent_events ORDER BY id",
            field_count: 6,
        },
        TableManifestSpec {
            name: "historical_transmission_events",
            query: "SELECT id, consent_id, packet_digest, provider, model, outcome, created_at, expires_at FROM historical_transmission_events ORDER BY id",
            field_count: 8,
        },
        TableManifestSpec {
            name: "historical_question_artifacts",
            query: "SELECT id, current_experience_id, packet_digest, payload, packet_snapshot, consent_id, transmission_id, created_at FROM historical_question_artifacts ORDER BY id",
            field_count: 8,
        },
        TableManifestSpec {
            name: "historical_artifact_dependencies",
            query: "SELECT historical_artifact_id, source_entry_id, source_artifact_id, source_revision FROM historical_artifact_dependencies ORDER BY historical_artifact_id, source_entry_id, source_artifact_id",
            field_count: 4,
        },
    ];

    fn frame_test_bytes(digest: &mut Sha256, bytes: &[u8]) {
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }

    fn frame_test_optional_text(digest: &mut Sha256, value: Option<&str>) {
        match value {
            Some(value) => {
                digest.update([1]);
                frame_test_bytes(digest, value.as_bytes());
            }
            None => digest.update([0]),
        }
    }

    async fn inspect_v4(path: &Path) -> Result<CandidateEvidence, SafetyError> {
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(path)
            .read_only(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options)
            .await
            .map_err(|_| SafetyError::fail_closed("candidate_open_failed"))?;
        let schema_version: i64 = sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&mut connection)
            .await
            .map_err(|_| SafetyError::fail_closed("candidate_schema_read_failed"))?;
        let foreign_key_rows = sqlx::query("PRAGMA foreign_key_check")
            .fetch_all(&mut connection)
            .await
            .map_err(|_| SafetyError::fail_closed("candidate_foreign_key_read_failed"))?;
        let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&mut connection)
            .await
            .map_err(|_| SafetyError::fail_closed("candidate_integrity_read_failed"))?;
        let mut manifest_digest = Sha256::new();
        manifest_digest.update(b"life-os/v4-source-manifest-v1\0");
        let mut record_digest = Sha256::new();
        record_digest.update(b"life-os/v4-exact-record-v1\0");
        for spec in TABLE_MANIFESTS {
            frame_test_bytes(&mut manifest_digest, spec.name.as_bytes());
            frame_test_bytes(&mut record_digest, spec.name.as_bytes());
            let rows = sqlx::query(spec.query)
                .fetch_all(&mut connection)
                .await
                .map_err(|_| SafetyError::fail_closed("candidate_record_read_failed"))?;
            manifest_digest.update((rows.len() as u64).to_be_bytes());
            record_digest.update((rows.len() as u64).to_be_bytes());
            for row in rows {
                for index in 0..spec.field_count {
                    let value: Option<String> = row
                        .try_get(index)
                        .map_err(|_| SafetyError::fail_closed("candidate_record_decode_failed"))?;
                    frame_test_optional_text(&mut manifest_digest, value.as_deref());
                    frame_test_optional_text(&mut record_digest, value.as_deref());
                }
            }
        }
        let exact_record_digest = format!("{:x}", record_digest.finalize());
        frame_test_bytes(&mut manifest_digest, b"persisted_artifacts.count_by_kind");
        let kind_counts = sqlx::query(
            "SELECT artifact_kind, COUNT(*) FROM persisted_artifacts GROUP BY artifact_kind ORDER BY artifact_kind",
        )
        .fetch_all(&mut connection)
        .await
        .map_err(|_| SafetyError::fail_closed("candidate_kind_count_read_failed"))?;
        manifest_digest.update((kind_counts.len() as u64).to_be_bytes());
        for row in kind_counts {
            let kind: String = row
                .try_get(0)
                .map_err(|_| SafetyError::fail_closed("candidate_kind_decode_failed"))?;
            let count: i64 = row
                .try_get(1)
                .map_err(|_| SafetyError::fail_closed("candidate_count_decode_failed"))?;
            frame_test_bytes(&mut manifest_digest, kind.as_bytes());
            manifest_digest.update(count.to_be_bytes());
        }
        connection
            .close()
            .await
            .map_err(|_| SafetyError::fail_closed("candidate_close_failed"))?;
        Ok(CandidateEvidence {
            source_manifest_digest: format!("{:x}", manifest_digest.finalize()),
            schema_version,
            foreign_keys_valid: foreign_key_rows.is_empty(),
            integrity_valid: integrity == "ok",
            exact_record_digest,
        })
    }

    async fn expected_for(path: &Path) -> ExpectedCandidate {
        let evidence = inspect_v4(path).await.unwrap();
        ExpectedCandidate {
            database_sha256: sha256_file(path).unwrap(),
            source_manifest_digest: evidence.source_manifest_digest,
            schema_version: evidence.schema_version,
            foreign_keys_valid: evidence.foreign_keys_valid,
            integrity_valid: evidence.integrity_valid,
            exact_record_digest: evidence.exact_record_digest,
        }
    }

    fn same_volume() -> FixedVolumeProbe {
        FixedVolumeProbe {
            root_volume: 7,
            live_volume: 7,
        }
    }

    fn prepare_fixed<'a>(
        root: &Path,
        live: &Path,
        guard: &ExclusiveOperationGuard<'a, MutableQuiescence>,
        id: &str,
    ) -> OwnedOperation {
        prepare_operation_with_id(root, live, guard, &same_volume(), id).unwrap()
    }

    fn fill_owned_backup(operation: &OwnedOperation, source: &Path) {
        let bytes = fs::read(source).unwrap();
        let mut backup = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&operation.backup)
            .unwrap();
        backup.write_all(&bytes).unwrap();
        backup.sync_all().unwrap();
        update_state(
            operation,
            OperationPhase::BackupVerified,
            Some(sha256_file(&operation.live).unwrap()),
            Some(sha256_file(&operation.backup).unwrap()),
            Some("backup_verified".into()),
        )
        .unwrap();
    }

    #[tokio::test]
    async fn quiescence_and_sidecars_fail_closed_without_modification() {
        let (directory, live) = fixture("live").await;
        let live_before = fs::read(&live).unwrap();
        for activity in [DatabaseActivity::Active, DatabaseActivity::Unknown] {
            let probe = MutableQuiescence::new(activity);
            let error = ExclusiveOperationGuard::acquire(&probe).err().unwrap();
            assert_eq!(
                error.code,
                if activity == DatabaseActivity::Active {
                    "database_activity_active"
                } else {
                    "database_activity_unknown"
                }
            );
        }

        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        for (index, suffix) in ["-wal", "-shm", "-journal"].iter().enumerate() {
            let sidecar = append_suffix(&live, suffix);
            fs::write(&sidecar, format!("sidecar-{index}")).unwrap();
            let before = fs::read(&sidecar).unwrap();
            let error = prepare_operation_with_id(
                directory.path(),
                &live,
                &guard,
                &same_volume(),
                FIXED_ID,
            )
            .unwrap_err();
            assert!(error.code.contains("sidecar_present"));
            assert_eq!(fs::read(&sidecar).unwrap(), before);
            assert_eq!(fs::read(&live).unwrap(), live_before);
            fs::remove_file(sidecar).unwrap();
        }
    }

    #[test]
    fn symlink_and_reparse_flags_are_both_fail_closed() {
        assert!(reject_link_flags(false, false).is_ok());
        assert_eq!(
            reject_link_flags(true, false).unwrap_err().code,
            "symlink_or_reparse_point_refused"
        );
        assert_eq!(
            reject_link_flags(false, true).unwrap_err().code,
            "symlink_or_reparse_point_refused"
        );
    }

    #[tokio::test]
    async fn owned_paths_refuse_traversal_aliases_cross_volume_and_collisions() {
        let (directory, live) = fixture("live").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();

        let traversal = directory.path().join("sub").join("..").join("life-os.db");
        assert_eq!(
            prepare_operation_with_id(
                directory.path(),
                &traversal,
                &guard,
                &same_volume(),
                FIXED_ID
            )
            .unwrap_err()
            .code,
            "path_alias_or_traversal_refused"
        );

        let other = tempfile::tempdir().unwrap();
        let outside = other.path().join("outside.db");
        fs::write(&outside, b"outside").unwrap();
        assert_eq!(
            prepare_operation_with_id(directory.path(), &outside, &guard, &same_volume(), FIXED_ID)
                .unwrap_err()
                .code,
            "live_path_outside_owned_root"
        );

        let different_volume = FixedVolumeProbe {
            root_volume: 1,
            live_volume: 2,
        };
        assert_eq!(
            prepare_operation_with_id(directory.path(), &live, &guard, &different_volume, FIXED_ID)
                .unwrap_err()
                .code,
            "cross_volume_replacement_refused"
        );

        let collision = directory
            .path()
            .join(format!("life-os-{FIXED_ID}.operation"));
        fs::create_dir(&collision).unwrap();
        let sentinel = collision.join("owned-by-someone-else");
        fs::write(&sentinel, b"preserve").unwrap();
        let before = fs::read(&sentinel).unwrap();
        assert_eq!(
            prepare_operation_with_id(directory.path(), &live, &guard, &same_volume(), FIXED_ID)
                .unwrap_err()
                .code,
            "operation_destination_conflict"
        );
        assert_eq!(fs::read(sentinel).unwrap(), before);
    }

    #[tokio::test]
    async fn generated_ownership_and_system_adapters_are_bounded_to_the_disposable_root() {
        let (directory, live) = fixture("live").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation =
            prepare_operation(directory.path(), &live, &guard, &SystemVolumeProbe).unwrap();

        assert_eq!(operation.operation_id.len(), 32);
        assert!(operation
            .operation_id
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));
        for path in [&operation.backup, &operation.staging, &operation.state] {
            assert_eq!(path.parent(), Some(operation.root.as_path()));
        }
        assert!(!operation.backup.exists());
        assert!(operation.staging.exists());
        assert!(operation.state.exists());
        assert_eq!(
            operation.root.parent(),
            Some(operation.owned_root.as_path())
        );
        assert_ne!(operation.backup, operation.staging);
        assert_ne!(operation.staging, operation.state);

        assert_eq!(
            SystemVolumeProbe.volume_id(&operation.root).unwrap(),
            SystemVolumeProbe.volume_id(&operation.live).unwrap()
        );
        assert!(SystemDurability.sync_file(&operation.staging).is_ok());

        #[cfg(windows)]
        {
            assert_eq!(
                SystemDurability.parent_directory_support(),
                ParentDirectorySupport::Unsupported
            );
            assert_eq!(
                SystemDurability
                    .sync_parent(&operation.root)
                    .unwrap_err()
                    .code,
                "parent_directory_sync_unsupported"
            );
        }
        #[cfg(unix)]
        {
            assert_eq!(
                SystemDurability.parent_directory_support(),
                ParentDirectorySupport::Supported
            );
            assert!(SystemDurability.sync_parent(&operation.root).is_ok());
        }
    }

    #[tokio::test]
    async fn owned_backup_handoff_keeps_the_claimed_path_absent_until_one_verified_vacuum() {
        let (directory, live) = fixture("governed-source").await;
        let outside = directory.path().join("outside-sentinel");
        fs::write(&outside, b"outside").unwrap();
        let live_before = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation =
            prepare_operation(directory.path(), &live, &guard, &SystemVolumeProbe).unwrap();
        assert!(!operation.backup.exists());
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::Prepared
        );

        let creator = CountingSystemVacuum::new();
        let expected = create_owned_verified_backup(
            &operation,
            &guard,
            &SystemVolumeProbe,
            &SqliteV4Verifier::exact(),
            &creator,
            &SystemDurability,
        )
        .await
        .unwrap();

        assert_eq!(creator.calls.load(Ordering::SeqCst), 1);
        assert!(operation.backup.is_file());
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert_eq!(fs::read(&outside).unwrap(), b"outside");
        assert_eq!(
            expected.database_sha256,
            sha256_file(&operation.backup).unwrap()
        );
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::BackupVerified
        );
    }

    #[tokio::test]
    async fn backup_collision_after_claim_is_preserved_without_invoking_sqlite() {
        let (directory, live) = fixture("source").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fs::write(&operation.backup, b"not-owned-by-operation").unwrap();
        let before = fs::read(&operation.backup).unwrap();
        let creator = CountingSystemVacuum::new();

        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &same_volume(),
            &SqliteV4Verifier::exact(),
            &creator,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();

        assert!(error.recovery_required);
        assert_eq!(creator.calls.load(Ordering::SeqCst), 0);
        assert_eq!(fs::read(&operation.backup).unwrap(), before);
        assert!(matches!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired { .. }
        ));
    }

    #[tokio::test]
    async fn source_identity_change_before_vacuum_fails_without_invoking_sqlite() {
        let (directory, live) = fixture("source").await;
        let bytes = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        let creator = CountingSystemVacuum::new();

        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &same_volume(),
            &SourceReplacingVerifier {
                calls: AtomicUsize::new(0),
            },
            &creator,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();

        assert_eq!(error.code, "backup_source_identity_changed");
        assert_eq!(creator.calls.load(Ordering::SeqCst), 0);
        assert!(!operation.backup.exists());
        assert_eq!(fs::read(&live).unwrap(), bytes);
    }

    #[tokio::test]
    async fn activity_and_sidecar_changes_after_claim_refuse_before_vacuum() {
        let (directory, live) = fixture("source").await;
        let live_before = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        let creator = CountingSystemVacuum::new();

        probe.set(DatabaseActivity::Active);
        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &same_volume(),
            &SqliteV4Verifier::exact(),
            &creator,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "database_activity_active");
        assert_eq!(creator.calls.load(Ordering::SeqCst), 0);

        probe.set(DatabaseActivity::Quiescent);
        let wal = append_suffix(&live, "-wal");
        fs::write(&wal, b"preserve-wal").unwrap();
        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &same_volume(),
            &SqliteV4Verifier::exact(),
            &creator,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "sqlite_wal_sidecar_present");
        assert_eq!(creator.calls.load(Ordering::SeqCst), 0);
        assert_eq!(fs::read(&wal).unwrap(), b"preserve-wal");
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert!(!operation.backup.exists());
    }

    #[tokio::test]
    async fn volume_change_at_final_preflight_refuses_before_vacuum() {
        let (directory, live) = fixture("source").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        let creator = CountingSystemVacuum::new();

        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &ChangingVolumeProbe {
                calls: AtomicUsize::new(0),
            },
            &SqliteV4Verifier::exact(),
            &creator,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();

        assert_eq!(error.code, "cross_volume_backup_refused");
        assert_eq!(creator.calls.load(Ordering::SeqCst), 0);
        assert!(!operation.backup.exists());
    }

    #[tokio::test]
    async fn failed_backup_creation_cleans_only_exact_owned_output_and_preserves_ambiguity() {
        let (directory, live) = fixture("source").await;
        let live_before = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);

        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &same_volume(),
            &SqliteV4Verifier::exact(),
            &FailingBackupCreator::ExactOwnedOutput,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "injected_backup_creation_failure");
        assert!(!error.recovery_required);
        assert!(!operation.backup.exists());
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::Prepared
        );

        let second_id = "1123456789abcdef0123456789abcdef";
        let operation = prepare_fixed(directory.path(), &live, &guard, second_id);
        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &same_volume(),
            &SqliteV4Verifier::exact(),
            &FailingBackupCreator::AmbiguousHardLink,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();
        assert!(error.recovery_required);
        assert!(operation.backup.exists());
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert!(matches!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired { .. }
        ));
    }

    #[tokio::test]
    async fn post_close_backup_verification_failure_preserves_output_for_recovery() {
        let (directory, live) = fixture("source").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        let creator = CountingSystemVacuum::new();

        let error = create_owned_verified_backup(
            &operation,
            &guard,
            &same_volume(),
            &SqliteV4Verifier::failing_on(2),
            &creator,
            &TestDurability::supported(),
        )
        .await
        .unwrap_err();

        assert!(error.recovery_required);
        assert_eq!(creator.calls.load(Ordering::SeqCst), 1);
        assert!(operation.backup.exists());
        assert!(matches!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired { .. }
        ));
    }

    #[cfg(windows)]
    #[test]
    fn windows_replace_classifier_is_conservative_for_documented_partial_failures() {
        use windows_sys::Win32::Foundation::{
            ERROR_ACCESS_DENIED, ERROR_UNABLE_TO_MOVE_REPLACEMENT,
            ERROR_UNABLE_TO_MOVE_REPLACEMENT_2, ERROR_UNABLE_TO_REMOVE_REPLACED,
        };

        assert!(matches!(
            classify_windows_replace_error(ERROR_UNABLE_TO_REMOVE_REPLACED),
            ReplacementOutcome::FailedUnchanged { .. }
        ));
        for error in [
            ERROR_UNABLE_TO_MOVE_REPLACEMENT,
            ERROR_UNABLE_TO_MOVE_REPLACEMENT_2,
            ERROR_ACCESS_DENIED,
        ] {
            assert!(matches!(
                classify_windows_replace_error(error),
                ReplacementOutcome::OutcomeUnknown { .. }
            ));
        }
    }

    #[cfg(windows)]
    #[test]
    fn windows_replace_file_adapter_changes_only_disposable_exact_paths() {
        let directory = tempfile::tempdir().unwrap();
        let live = directory.path().join("live.db");
        let staging = directory.path().join("staging.db");
        let sentinel = directory.path().join("sentinel");
        fs::write(&live, b"old-live").unwrap();
        fs::write(&staging, b"verified-staging").unwrap();
        fs::write(&sentinel, b"outside-replacement").unwrap();

        assert_eq!(
            WindowsReplacement.replace(&staging, &live),
            ReplacementOutcome::Committed
        );
        assert_eq!(fs::read(&live).unwrap(), b"verified-staging");
        assert!(!staging.exists());
        assert_eq!(fs::read(&sentinel).unwrap(), b"outside-replacement");
        assert!(WindowsReplacement.requires_parent_directory_sync());
        assert_eq!(
            SystemDurability.parent_directory_support(),
            ParentDirectorySupport::Unsupported
        );
    }

    #[tokio::test]
    async fn hard_link_alias_is_refused_without_touching_either_name() {
        let (directory, live) = fixture("live").await;
        let alias = directory.path().join("alias.db");
        fs::hard_link(&live, &alias).unwrap();
        let before = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        assert_eq!(
            prepare_operation_with_id(directory.path(), &live, &guard, &same_volume(), FIXED_ID)
                .unwrap_err()
                .code,
            "file_alias_refused"
        );
        assert_eq!(fs::read(live).unwrap(), before);
        assert_eq!(fs::read(alias).unwrap(), before);
    }

    #[tokio::test]
    async fn successful_disposable_replacement_validates_exact_evidence_and_restart_state() {
        let (directory, live) = fixture("live-before").await;
        let source_dir = tempfile::tempdir().unwrap();
        let source = source_dir.path().join("source.db");
        create_v4_database(&source, "backup-content").await;
        let expected = expected_for(&source).await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fill_owned_backup(&operation, &source);
        let backup_before = fs::read(&operation.backup).unwrap();

        let result = execute_replacement(
            &operation,
            &guard,
            &expected,
            &SqliteV4Verifier::exact(),
            &TestDurability::supported(),
            &TestReplacement::CommitByCopy,
        )
        .await
        .unwrap();

        assert_eq!(result, ExecutionResult::Completed);
        assert_eq!(sha256_file(&live).unwrap(), expected.database_sha256);
        assert_eq!(fs::read(&operation.backup).unwrap(), backup_before);
        assert!(!operation.staging.exists());
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::Completed
        );
    }

    #[tokio::test]
    async fn every_candidate_evidence_mismatch_fails_before_replacement() {
        let variants = [
            "database",
            "source_manifest",
            "schema",
            "foreign_key",
            "integrity",
            "records",
        ];
        for (index, variant) in variants.iter().enumerate() {
            let (directory, live) = fixture("live-before").await;
            let source_dir = tempfile::tempdir().unwrap();
            let source = source_dir.path().join("source.db");
            create_v4_database(&source, "backup-content").await;
            let mut expected = expected_for(&source).await;
            let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
            let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
            let id = format!("{index:032x}");
            let operation = prepare_fixed(directory.path(), &live, &guard, &id);
            fill_owned_backup(&operation, &source);
            let live_before = fs::read(&live).unwrap();
            match *variant {
                "database" => expected.database_sha256 = "0".repeat(64),
                "source_manifest" => expected.source_manifest_digest = "0".repeat(64),
                "schema" => expected.schema_version = 3,
                "foreign_key" => expected.foreign_keys_valid = false,
                "integrity" => expected.integrity_valid = false,
                "records" => expected.exact_record_digest = "0".repeat(64),
                _ => unreachable!(),
            }
            let error = execute_replacement(
                &operation,
                &guard,
                &expected,
                &SqliteV4Verifier::exact(),
                &TestDurability::supported(),
                &TestReplacement::CommitByCopy,
            )
            .await
            .unwrap_err();
            assert!(!error.recovery_required, "{variant}: {error}");
            assert_eq!(fs::read(&live).unwrap(), live_before);
            assert!(!operation.staging.exists());
        }
    }

    #[tokio::test]
    async fn verifier_detects_manifest_fk_integrity_and_record_mismatch() {
        let (directory, live) = fixture("live-before").await;
        let source_dir = tempfile::tempdir().unwrap();
        let source = source_dir.path().join("source.db");
        create_v4_database(&source, "backup-content").await;
        let expected = expected_for(&source).await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();

        let mut variants = Vec::new();
        let mut manifest = inspect_v4(&source).await.unwrap();
        manifest.source_manifest_digest = "0".repeat(64);
        variants.push(("candidate_source_manifest_mismatch", manifest));
        let mut foreign_key = inspect_v4(&source).await.unwrap();
        foreign_key.foreign_keys_valid = false;
        variants.push(("candidate_foreign_key_check_failed", foreign_key));
        let mut integrity = inspect_v4(&source).await.unwrap();
        integrity.integrity_valid = false;
        variants.push(("candidate_integrity_check_failed", integrity));
        let mut records = inspect_v4(&source).await.unwrap();
        records.exact_record_digest = "0".repeat(64);
        variants.push(("candidate_exact_record_digest_mismatch", records));

        for (index, (expected_code, evidence)) in variants.into_iter().enumerate() {
            let id = format!("{:032x}", index + 32);
            let operation = prepare_fixed(directory.path(), &live, &guard, &id);
            fill_owned_backup(&operation, &source);
            let live_before = fs::read(&live).unwrap();
            let error = execute_replacement(
                &operation,
                &guard,
                &expected,
                &SqliteV4Verifier::overriding(evidence),
                &TestDurability::supported(),
                &TestReplacement::CommitByCopy,
            )
            .await
            .unwrap_err();
            assert_eq!(error.code, expected_code);
            assert_eq!(fs::read(&live).unwrap(), live_before);
        }
    }

    #[tokio::test]
    async fn durability_refusal_and_failure_distinguish_precommit_from_ambiguous_commit() {
        let source_dir = tempfile::tempdir().unwrap();
        let source = source_dir.path().join("source.db");
        create_v4_database(&source, "backup-content").await;
        let expected = expected_for(&source).await;

        let (directory, live) = fixture("live-before").await;
        let live_before = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fill_owned_backup(&operation, &source);
        let unsupported = TestDurability {
            support: ParentDirectorySupport::Unsupported,
            fail_file: false,
            fail_parent: false,
        };
        let error = execute_replacement(
            &operation,
            &guard,
            &expected,
            &SqliteV4Verifier::exact(),
            &unsupported,
            &TestReplacement::CommitByCopy,
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "parent_directory_sync_unsupported");
        assert!(!error.recovery_required);
        assert_eq!(fs::read(&live).unwrap(), live_before);

        let (directory, live) = fixture("live-before").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fill_owned_backup(&operation, &source);
        let parent_failure = TestDurability {
            support: ParentDirectorySupport::Supported,
            fail_file: false,
            fail_parent: true,
        };
        let error = execute_replacement(
            &operation,
            &guard,
            &expected,
            &SqliteV4Verifier::exact(),
            &parent_failure,
            &TestReplacement::CommitByCopy,
        )
        .await
        .unwrap_err();
        assert!(error.recovery_required);
        assert!(error.code.contains("parent_sync_failure"));
        assert!(matches!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired { .. }
        ));
    }

    #[tokio::test]
    async fn replacement_outcomes_preserve_known_failure_and_flag_unknown_state() {
        let source_dir = tempfile::tempdir().unwrap();
        let source = source_dir.path().join("source.db");
        create_v4_database(&source, "backup-content").await;
        let expected = expected_for(&source).await;

        let (directory, live) = fixture("live-before").await;
        let live_before = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fill_owned_backup(&operation, &source);
        let error = execute_replacement(
            &operation,
            &guard,
            &expected,
            &SqliteV4Verifier::exact(),
            &TestDurability::supported(),
            &TestReplacement::FailedUnchanged,
        )
        .await
        .unwrap_err();
        assert!(!error.recovery_required);
        assert_eq!(fs::read(&live).unwrap(), live_before);

        let (directory, live) = fixture("live-before").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fill_owned_backup(&operation, &source);
        let error = execute_replacement(
            &operation,
            &guard,
            &expected,
            &SqliteV4Verifier::exact(),
            &TestDurability::supported(),
            &TestReplacement::UnknownAfterWrite,
        )
        .await
        .unwrap_err();
        assert!(error.recovery_required);
        assert!(matches!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired { .. }
        ));
        assert!(operation.state.exists());
        assert!(operation.backup.exists());
    }

    #[tokio::test]
    async fn activity_change_and_post_commit_verification_fail_without_automatic_repair() {
        let source_dir = tempfile::tempdir().unwrap();
        let source = source_dir.path().join("source.db");
        create_v4_database(&source, "backup-content").await;
        let expected = expected_for(&source).await;

        let (directory, live) = fixture("live-before").await;
        let live_before = fs::read(&live).unwrap();
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fill_owned_backup(&operation, &source);
        let error = execute_replacement(
            &operation,
            &guard,
            &expected,
            &ActivityChangingVerifier {
                calls: AtomicUsize::new(0),
                probe: &probe,
            },
            &TestDurability::supported(),
            &TestReplacement::CommitByCopy,
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "database_activity_active");
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert!(!operation.staging.exists());
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::BackupVerified
        );

        let (directory, live) = fixture("live-before").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        fill_owned_backup(&operation, &source);
        let error = execute_replacement(
            &operation,
            &guard,
            &expected,
            &SqliteV4Verifier::failing_on(4),
            &TestDurability::supported(),
            &TestReplacement::CommitByCopy,
        )
        .await
        .unwrap_err();
        assert!(error.recovery_required);
        assert!(error.code.starts_with("post_commit_verification_failed"));
        assert_eq!(sha256_file(&live).unwrap(), expected.database_sha256);
        assert!(matches!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired { .. }
        ));
    }

    #[tokio::test]
    async fn restart_inspection_is_read_only_and_fails_closed_on_corrupt_or_contradictory_state() {
        let (directory, live) = fixture("live-before").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        let live_before = fs::read(&live).unwrap();
        assert!(!operation.backup.exists());
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::Prepared
        );
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert!(!operation.backup.exists());

        update_state(
            &operation,
            OperationPhase::Prepared,
            Some(sha256_file(&live).unwrap()),
            None,
            Some("known_precommit_failure".into()),
        )
        .unwrap();
        fs::write(&live, b"externally-changed").unwrap();
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired {
                error_class: "prepared_state_contradictory".into()
            }
        );
        fs::write(&live, &live_before).unwrap();
        write_state(&operation, &operation.initial_state().unwrap()).unwrap();

        let source_dir = tempfile::tempdir().unwrap();
        let source = source_dir.path().join("source.db");
        create_v4_database(&source, "replacement").await;
        let expected = expected_for(&source).await;
        fill_owned_backup(&operation, &source);
        fs::copy(&operation.backup, &operation.staging).unwrap();
        update_state(
            &operation,
            OperationPhase::Staged,
            Some(sha256_file(&live).unwrap()),
            Some(expected.database_sha256.clone()),
            Some("staged_and_verified".into()),
        )
        .unwrap();
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::Staged
        );

        fs::copy(&operation.staging, &live).unwrap();
        update_state(
            &operation,
            OperationPhase::ReplacementCommitted,
            None,
            None,
            Some("committed".into()),
        )
        .unwrap();
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::ReplacementCommitted
        );

        update_state(
            &operation,
            OperationPhase::CompletedWithCleanupRequired,
            None,
            None,
            Some("cleanup_required".into()),
        )
        .unwrap();
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::CompletedWithCleanupRequired
        );
        assert!(operation.staging.exists());
        let live_after_replace = fs::read(&live).unwrap();

        fs::remove_file(&operation.state).unwrap();
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired {
                error_class: "operation_state_missing".into()
            }
        );

        File::create(&operation.state).unwrap();
        fs::write(&operation.state, b"{malformed").unwrap();
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired {
                error_class: "operation_state_malformed".into()
            }
        );
        assert_eq!(fs::read(&live).unwrap(), live_after_replace);

        let mut state = operation.initial_state().unwrap();
        state.operation_id = "ffffffffffffffffffffffffffffffff".into();
        let bytes = serde_json::to_vec(&state).unwrap();
        fs::write(&operation.state, bytes).unwrap();
        assert_eq!(
            inspect_restart_state(&operation).unwrap(),
            RestartInspection::RecoveryRequired {
                error_class: "operation_state_identity_mismatch".into()
            }
        );
    }

    #[tokio::test]
    async fn cleanup_is_limited_to_exact_owned_staging() {
        let (directory, live) = fixture("live-before").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        let unrelated = directory.path().join("unrelated.db");
        let manifest = directory.path().join("manifest.json");
        let sidecar = append_suffix(&live, "-wal");
        fs::write(&operation.backup, b"backup").unwrap();
        fs::write(&unrelated, b"unrelated").unwrap();
        fs::write(&manifest, b"manifest").unwrap();
        fs::write(&sidecar, b"wal").unwrap();
        let protected = [
            operation.live.clone(),
            operation.backup.clone(),
            operation.state.clone(),
            unrelated.clone(),
            manifest.clone(),
            sidecar.clone(),
        ];
        let before: Vec<_> = protected
            .iter()
            .map(|path| fs::read(path).unwrap())
            .collect();

        for path in &protected {
            let error = cleanup_owned_temporary(&operation, path).unwrap_err();
            assert!(matches!(
                error.code.as_str(),
                "cleanup_path_not_owned" | "cleanup_not_authorized"
            ));
        }
        assert!(cleanup_owned_temporary(&operation, &operation.staging).is_ok());
        assert!(!operation.staging.exists());
        for (path, bytes) in protected.iter().zip(before) {
            assert_eq!(fs::read(path).unwrap(), bytes);
        }
    }

    #[tokio::test]
    async fn readiness_observes_valid_owned_operation_without_mutating_it() {
        let (directory, live) = fixture("live-before").await;
        let probe = MutableQuiescence::new(DatabaseActivity::Quiescent);
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation = prepare_fixed(directory.path(), &live, &guard, FIXED_ID);
        let state_before = fs::read(&operation.state).unwrap();
        let staging_before = fs::read(&operation.staging).unwrap();

        let snapshot = inspect_readiness_filesystem(directory.path(), &live).unwrap();

        assert!(snapshot.database_exists);
        assert_eq!(snapshot.operation_evidence, ReadinessOperationEvidence::Present);
        assert!(!snapshot.wal_present);
        assert_eq!(fs::read(&operation.state).unwrap(), state_before);
        assert_eq!(fs::read(&operation.staging).unwrap(), staging_before);
    }
}
