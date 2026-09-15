use crate::filesystem_safety::{
    inspect_exact_prepared_operation_for_recovery, read_migration_state_evidence,
    record_migration_state, verify_owned_backup_for_migration, MigrationOperationPhase,
    MigrationReceiptEvidence, OwnedOperation,
};
use crate::schema_v5_founder_activation::{operation_candidates, operation_lock, paths};
use crate::schema_v5_migration::{
    activate_exact_historical_frontend_lifecycle_writes, verify_any_committed_v5,
    verify_any_committed_v5_after_empty_sidecar_proof,
    verify_exact_historical_frontend_post_commit_v5, ExactV4CandidateVerifier, MigrationReceipt,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tauri::AppHandle;
use time::OffsetDateTime;

const ORDINARY_IDENTIFIER: &str = "com.lifeos.app";
const DATABASE_FILENAME: &str = "life-os.db";
const WAL_FILENAME: &str = "life-os.db-wal";
const SHM_FILENAME: &str = "life-os.db-shm";
const JOURNAL_FILENAME: &str = "life-os.db-journal";
const RECOVERY_CLASSIFICATION: &str = "exact_prepared_v4_empty_wal_v1";
const V5_SIDECAR_RECOVERY_CLASSIFICATION: &str = "exact_v5_ready_legacy_empty_sidecar_v1";
const POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION: &str =
    "exact_historical_frontend_v4_post_commit_manifest_v1";
const RECOVERY_RECEIPT_SCHEMA: u8 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecoveryFileFact {
    relative_path: String,
    size: u64,
    sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreparedRecoveryInspection {
    eligible: bool,
    reason: Option<String>,
    classification: Option<String>,
    operation_id: Option<String>,
    database: Option<RecoveryFileFact>,
    database_identity: Option<String>,
    evidence_files: Vec<RecoveryFileFact>,
    preserved_files: Vec<RecoveryFileFact>,
    claim_digest: Option<String>,
}

impl PreparedRecoveryInspection {
    fn refused(reason: impl Into<String>) -> Self {
        Self {
            eligible: false,
            reason: Some(reason.into()),
            classification: None,
            operation_id: None,
            database: None,
            database_identity: None,
            evidence_files: Vec::new(),
            preserved_files: Vec::new(),
            claim_digest: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreparedRecoveryResult {
    classification: String,
    operation_id: String,
    receipt_relative_path: String,
    database_sha256: String,
    restart_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PreparedRecoveryReceipt {
    schema_version: u8,
    classification: String,
    operation_id: String,
    application_version: String,
    database_schema_version: i64,
    recovered_at: String,
    database: RecoveryFileFact,
    quarantined_files: Vec<RecoveryFileFact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct V5SidecarRecoveryReceipt {
    schema_version: u8,
    classification: String,
    operation_id: String,
    application_version: String,
    database_schema_version: i64,
    recovered_at: String,
    database: RecoveryFileFact,
    preserved_files: Vec<RecoveryFileFact>,
    quarantined_files: Vec<RecoveryFileFact>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PostCommitManifestRecoveryReceipt {
    schema_version: u8,
    classification: String,
    operation_id: String,
    application_version: String,
    database_schema_version: i64,
    recovered_at: String,
    migration_id: String,
    source_manifest_digest: String,
    target_manifest_digest: String,
    database: RecoveryFileFact,
    operation_state_before: RecoveryFileFact,
    preserved_files: Vec<RecoveryFileFact>,
    lifecycle_writes: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecoveryReceiptSummary {
    pub(crate) relative_path: String,
    pub(crate) operation_id: String,
    pub(crate) recovered_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecoveryFailurePoint {
    None,
    AfterQuarantineCreate,
    AfterWalMove,
    AfterShmMove,
    AfterStagingMove,
    AfterStateMove,
    AfterOperationCleanup,
    BeforeReceiptWrite,
    AfterReceiptWrite,
}

fn inject(point: RecoveryFailurePoint, expected: RecoveryFailurePoint) -> Result<(), String> {
    if point == expected {
        Err(format!("prepared_recovery_injected_failure_{expected:?}"))
    } else {
        Ok(())
    }
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|_| "prepared_recovery_file_unreadable")?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|_| "prepared_recovery_file_unreadable")?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn direct_file(path: &Path, root: &Path, label: &str) -> Result<(), String> {
    let metadata =
        fs::symlink_metadata(path).map_err(|_| format!("prepared_recovery_{label}_unreadable"))?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(format!("prepared_recovery_{label}_not_direct_file"));
    }
    let canonical_root =
        fs::canonicalize(root).map_err(|_| "prepared_recovery_root_unreadable".to_string())?;
    let canonical =
        fs::canonicalize(path).map_err(|_| format!("prepared_recovery_{label}_unreadable"))?;
    if canonical.parent() != Some(canonical_root.as_path()) {
        return Err(format!("prepared_recovery_{label}_outside_owned_root"));
    }
    #[cfg(windows)]
    {
        use std::os::windows::{fs::MetadataExt, io::AsRawHandle};
        use windows_sys::Win32::Storage::FileSystem::{
            GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
        };
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        let file = File::open(path)
            .map_err(|_| format!("prepared_recovery_{label}_identity_unreadable"))?;
        let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
        let succeeded =
            unsafe { GetFileInformationByHandle(file.as_raw_handle().cast(), &mut info) };
        if succeeded == 0
            || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
            || info.nNumberOfLinks != 1
        {
            return Err(format!("prepared_recovery_{label}_identity_ambiguous"));
        }
    }
    Ok(())
}

fn file_fact(
    path: &Path,
    root: &Path,
    relative: &str,
    label: &str,
) -> Result<RecoveryFileFact, String> {
    direct_file(path, root, label)?;
    Ok(RecoveryFileFact {
        relative_path: relative.replace('\\', "/"),
        size: fs::metadata(path)
            .map_err(|_| format!("prepared_recovery_{label}_unreadable"))?
            .len(),
        sha256: sha256_file(path)?,
    })
}

#[cfg(windows)]
fn file_identity(path: &Path) -> Result<String, String> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
    };
    let file = File::open(path).map_err(|_| "prepared_recovery_database_identity_unreadable")?;
    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
    if unsafe { GetFileInformationByHandle(file.as_raw_handle().cast(), &mut info) } == 0 {
        return Err("prepared_recovery_database_identity_unreadable".into());
    }
    Ok(format!(
        "windows-volume-{:08X}-file-{:08X}{:08X}",
        info.dwVolumeSerialNumber, info.nFileIndexHigh, info.nFileIndexLow
    ))
}

#[cfg(unix)]
fn file_identity(path: &Path) -> Result<String, String> {
    use std::os::unix::fs::MetadataExt;
    let metadata =
        fs::metadata(path).map_err(|_| "prepared_recovery_database_identity_unreadable")?;
    Ok(format!(
        "unix-device-{}-inode-{}",
        metadata.dev(),
        metadata.ino()
    ))
}

fn read_user_version_without_sql(path: &Path) -> Result<i64, String> {
    let mut bytes = [0_u8; 64];
    File::open(path)
        .and_then(|mut file| file.read_exact(&mut bytes))
        .map_err(|_| "prepared_recovery_database_header_unreadable")?;
    if &bytes[..16] != b"SQLite format 3\0" {
        return Err("prepared_recovery_database_header_invalid".into());
    }
    Ok(u32::from_be_bytes(bytes[60..64].try_into().unwrap()) as i64)
}

fn prove_empty_wal(path: &Path) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|_| "prepared_recovery_wal_unreadable")?;
    if bytes.is_empty() {
        return Ok(());
    }
    if bytes.len() != 32 {
        return Err("prepared_recovery_wal_contains_frames".into());
    }
    let magic = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    if !matches!(magic, 0x377f_0682 | 0x377f_0683) {
        return Err("prepared_recovery_wal_header_invalid".into());
    }
    let page_size = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
    if page_size != 1 && (!page_size.is_power_of_two() || !(512..=65_536).contains(&page_size)) {
        return Err("prepared_recovery_wal_header_invalid".into());
    }
    Ok(())
}

fn prove_matching_zero_frame_shm(path: &Path) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|_| "prepared_recovery_shm_unreadable")?;
    if bytes.len() != 32_768 || bytes[0..48] != bytes[48..96] {
        return Err("prepared_recovery_shm_contract_mismatch".into());
    }
    let read_u32 =
        |offset: usize| u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    if read_u32(0) != 3_007_000
        || bytes[12] != 1
        || read_u32(16) != 0
        || read_u32(20) != 0
        || read_u32(96) != 0
    {
        return Err("prepared_recovery_shm_not_zero_frame".into());
    }
    Ok(())
}

fn claim_digest(
    classification: &str,
    operation_id: &str,
    database: &RecoveryFileFact,
    database_identity: &str,
    files: &[RecoveryFileFact],
    preserved_files: &[RecoveryFileFact],
) -> String {
    let value = serde_json::json!({
        "classification": classification,
        "operationId": operation_id,
        "database": database,
        "databaseIdentity": database_identity,
        "evidenceFiles": files,
        "preservedFiles": preserved_files,
    });
    sha256_bytes(serde_json::to_string(&value).unwrap().as_bytes())
}

fn canonical_now() -> Result<String, String> {
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|_| "prepared_recovery_timestamp_failed".into())
}

fn receipt_path(root: &Path, operation_id: &str) -> PathBuf {
    root.join(format!("prepared-recovery-{operation_id}.receipt.json"))
}

fn quarantine_path(root: &Path, operation_id: &str) -> PathBuf {
    root.join(format!("life-os-{operation_id}.prepared-recovery"))
}

fn v5_sidecar_receipt_path(root: &Path, operation_id: &str) -> PathBuf {
    root.join(format!("v5-sidecar-recovery-{operation_id}.receipt.json"))
}

fn v5_sidecar_quarantine_path(root: &Path, operation_id: &str) -> PathBuf {
    root.join(format!("life-os-{operation_id}.v5-sidecar-recovery"))
}

fn post_commit_manifest_receipt_path(root: &Path, operation_id: &str) -> PathBuf {
    root.join(format!(
        "post-commit-manifest-recovery-{operation_id}.receipt.json"
    ))
}

fn existing_receipts(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut receipts = Vec::new();
    for entry in fs::read_dir(root).map_err(|_| "prepared_recovery_root_unreadable")? {
        let entry = entry.map_err(|_| "prepared_recovery_root_unreadable")?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("prepared-recovery-") && name.ends_with(".receipt.json") {
            receipts.push(entry.path());
        }
    }
    receipts.sort();
    Ok(receipts)
}

fn existing_v5_sidecar_receipts(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut receipts = Vec::new();
    for entry in fs::read_dir(root).map_err(|_| "v5_sidecar_recovery_root_unreadable")? {
        let entry = entry.map_err(|_| "v5_sidecar_recovery_root_unreadable")?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("v5-sidecar-recovery-") && name.ends_with(".receipt.json") {
            receipts.push(entry.path());
        }
    }
    receipts.sort();
    Ok(receipts)
}

fn existing_post_commit_manifest_receipts(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut receipts = Vec::new();
    for entry in fs::read_dir(root).map_err(|_| "post_commit_recovery_root_unreadable")? {
        let entry = entry.map_err(|_| "post_commit_recovery_root_unreadable")?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("post-commit-manifest-recovery-") && name.ends_with(".receipt.json") {
            receipts.push(entry.path());
        }
    }
    receipts.sort();
    Ok(receipts)
}

fn collect_v5_preserved_files(
    root: &Path,
    operation: &OwnedOperation,
    prior_summary: &RecoveryReceiptSummary,
) -> Result<Vec<RecoveryFileFact>, String> {
    let operation_name = operation.root.file_name().unwrap().to_string_lossy();
    let mut preserved_files = vec![
        file_fact(
            &operation.state,
            &operation.root,
            &format!("{operation_name}/state.json"),
            "state",
        )?,
        file_fact(
            &operation.staging,
            &operation.root,
            &format!("{operation_name}/staging.db"),
            "staging",
        )?,
        file_fact(
            &operation.backup,
            &operation.root,
            &format!("{operation_name}/backup.db"),
            "backup",
        )?,
    ];
    let prior_receipt = root.join(&prior_summary.relative_path);
    preserved_files.push(file_fact(
        &prior_receipt,
        root,
        &prior_summary.relative_path,
        "prior_receipt",
    )?);
    let prior_quarantine = quarantine_path(root, &prior_summary.operation_id);
    for entry in fs::read_dir(&prior_quarantine)
        .map_err(|_| "v5_sidecar_recovery_prior_quarantine_unreadable")?
    {
        let entry = entry.map_err(|_| "v5_sidecar_recovery_prior_quarantine_unreadable")?;
        let name = entry.file_name().to_string_lossy().to_string();
        preserved_files.push(file_fact(
            &entry.path(),
            &prior_quarantine,
            &format!(
                "{}/{}",
                prior_quarantine.file_name().unwrap().to_string_lossy(),
                name
            ),
            "prior_quarantine",
        )?);
    }
    preserved_files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(preserved_files)
}

fn collect_post_commit_preserved_files(
    root: &Path,
    operation: &OwnedOperation,
    prior_summary: &RecoveryReceiptSummary,
) -> Result<Vec<RecoveryFileFact>, String> {
    let mut files = collect_v5_preserved_files(root, operation, prior_summary)?;
    let operation_state = format!(
        "{}/state.json",
        operation.root.file_name().unwrap().to_string_lossy()
    );
    files.retain(|fact| fact.relative_path != operation_state);
    Ok(files)
}

fn exact_post_commit_receipt(
    operation: &OwnedOperation,
    evidence: &crate::filesystem_safety::MigrationStateEvidence,
) -> Option<MigrationReceipt> {
    if evidence.phase != MigrationOperationPhase::V5BlockedRestoreAvailable
        || evidence.outcome_class.as_deref() != Some("post_commit_schema_manifest_mismatch")
    {
        return None;
    }
    Some(MigrationReceipt {
        migration_id: evidence.migration_id.clone()?,
        backup_id: Some(operation.operation_id.clone()),
        source_manifest_digest: evidence.migration_source_manifest_digest.clone()?,
        target_manifest_digest: evidence.migration_target_manifest_digest.clone()?,
    })
}

async fn inspect_post_commit_manifest_at(
    root: &Path,
    live: &Path,
) -> Result<PreparedRecoveryInspection, String> {
    if !root.is_dir() || !live.is_file() {
        return Ok(PreparedRecoveryInspection::refused(
            "post_commit_recovery_database_missing",
        ));
    }
    if [WAL_FILENAME, SHM_FILENAME, JOURNAL_FILENAME]
        .iter()
        .any(|name| root.join(name).exists())
    {
        return Ok(PreparedRecoveryInspection::refused(
            "post_commit_recovery_sidecar_present",
        ));
    }
    let operations = operation_candidates(root, live)?;
    if operations.len() != 1 {
        return Ok(PreparedRecoveryInspection::refused(
            if operations.is_empty() {
                "post_commit_recovery_operation_missing"
            } else {
                "post_commit_recovery_operation_not_unique"
            },
        ));
    }
    let operation = &operations[0];
    let evidence = match read_migration_state_evidence(operation) {
        Ok(value) => value,
        Err(error) => return Ok(PreparedRecoveryInspection::refused(error.code)),
    };
    let receipt = match exact_post_commit_receipt(operation, &evidence) {
        Some(value) => value,
        None => {
            return Ok(PreparedRecoveryInspection::refused(
                "post_commit_recovery_operation_not_exact",
            ))
        }
    };
    let expected_backup = match evidence.verified_backup.as_ref() {
        Some(value) => value,
        None => {
            return Ok(PreparedRecoveryInspection::refused(
                "post_commit_recovery_backup_evidence_missing",
            ))
        }
    };
    if verify_owned_backup_for_migration(operation, expected_backup, &ExactV4CandidateVerifier)
        .await
        .is_err()
    {
        return Ok(PreparedRecoveryInspection::refused(
            "post_commit_recovery_backup_verification_failed",
        ));
    }
    let lifecycle_writes =
        if verify_exact_historical_frontend_post_commit_v5(live, &receipt, "disabled")
            .await
            .is_ok()
        {
            "disabled"
        } else if verify_exact_historical_frontend_post_commit_v5(live, &receipt, "enabled")
            .await
            .is_ok()
        {
            "enabled"
        } else {
            return Ok(PreparedRecoveryInspection::refused(
                "post_commit_recovery_database_not_exact",
            ));
        };
    let operation_names = fs::read_dir(&operation.root)
        .map_err(|_| "post_commit_recovery_operation_unreadable")?
        .map(|entry| {
            entry
                .map(|value| value.file_name().to_string_lossy().to_string())
                .map_err(|_| "post_commit_recovery_operation_unreadable".to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if operation_names
        != BTreeSet::from([
            "backup.db".to_string(),
            "staging.db".to_string(),
            "state.json".to_string(),
        ])
        || fs::metadata(&operation.staging)
            .map_err(|_| "post_commit_recovery_staging_unreadable")?
            .len()
            != 0
    {
        return Ok(PreparedRecoveryInspection::refused(
            "post_commit_recovery_operation_files_invalid",
        ));
    }
    let prior_summary = match recovery_receipt_summary(root) {
        Some(value) => value,
        None => {
            return Ok(PreparedRecoveryInspection::refused(
                "post_commit_recovery_prior_receipt_invalid",
            ))
        }
    };
    let post_receipts = existing_post_commit_manifest_receipts(root)?;
    if post_receipts.len() > 1
        || post_receipts.first().is_some_and(|path| {
            *path != post_commit_manifest_receipt_path(root, &operation.operation_id)
        })
    {
        return Ok(PreparedRecoveryInspection::refused(
            "post_commit_recovery_receipt_conflict",
        ));
    }

    direct_file(live, root, "database")?;
    let database = file_fact(live, root, DATABASE_FILENAME, "database")?;
    let database_identity = file_identity(live)?;
    let operation_name = operation.root.file_name().unwrap().to_string_lossy();
    let state = file_fact(
        &operation.state,
        &operation.root,
        &format!("{operation_name}/state.json"),
        "state",
    )?;
    let preserved_files = collect_post_commit_preserved_files(root, operation, &prior_summary)?;
    if let Some(path) = post_receipts.first() {
        direct_file(path, root, "post_commit_receipt")?;
        let record: PostCommitManifestRecoveryReceipt = serde_json::from_slice(
            &fs::read(path).map_err(|_| "post_commit_recovery_receipt_unreadable")?,
        )
        .map_err(|_| "post_commit_recovery_receipt_malformed")?;
        if record.schema_version != RECOVERY_RECEIPT_SCHEMA
            || record.classification != POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION
            || record.operation_id != operation.operation_id
            || record.application_version != env!("CARGO_PKG_VERSION")
            || record.database_schema_version != 5
            || record.migration_id != receipt.migration_id
            || record.source_manifest_digest != receipt.source_manifest_digest
            || record.target_manifest_digest != receipt.target_manifest_digest
            || record.database != database
            || record.operation_state_before != state
            || record.preserved_files != preserved_files
            || record.lifecycle_writes != "enabled"
            || lifecycle_writes != "enabled"
            || OffsetDateTime::parse(
                &record.recovered_at,
                &time::format_description::well_known::Rfc3339,
            )
            .is_err()
        {
            return Ok(PreparedRecoveryInspection::refused(
                "post_commit_recovery_receipt_invalid",
            ));
        }
    }
    let evidence_files = vec![state];
    Ok(PreparedRecoveryInspection {
        eligible: true,
        reason: None,
        classification: Some(POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION.into()),
        operation_id: Some(operation.operation_id.clone()),
        claim_digest: Some(claim_digest(
            POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION,
            &operation.operation_id,
            &database,
            &database_identity,
            &evidence_files,
            &preserved_files,
        )),
        database: Some(database),
        database_identity: Some(database_identity),
        evidence_files,
        preserved_files,
    })
}

pub(crate) async fn exact_post_commit_manifest_recovery_available(
    root: &Path,
    live: &Path,
) -> bool {
    inspect_post_commit_manifest_at(root, live)
        .await
        .map(|inspection| inspection.eligible)
        .unwrap_or(false)
}

fn inspect_at(root: &Path, live: &Path) -> Result<PreparedRecoveryInspection, String> {
    if !root.is_dir() || !live.is_file() {
        return Ok(PreparedRecoveryInspection::refused(
            "prepared_recovery_database_missing",
        ));
    }
    if root.join(JOURNAL_FILENAME).exists() {
        return Ok(PreparedRecoveryInspection::refused(
            "prepared_recovery_journal_present",
        ));
    }
    if read_user_version_without_sql(live)? != 4 {
        return Ok(PreparedRecoveryInspection::refused(
            "prepared_recovery_schema_not_v4",
        ));
    }
    let operations = operation_candidates(root, live)?;
    if operations.len() != 1 {
        return Ok(PreparedRecoveryInspection::refused(
            if operations.is_empty() {
                "prepared_recovery_operation_missing"
            } else {
                "prepared_recovery_operation_not_unique"
            },
        ));
    }
    let operation = &operations[0];
    let prepared = match inspect_exact_prepared_operation_for_recovery(operation) {
        Ok(value) => value,
        Err(error) => return Ok(PreparedRecoveryInspection::refused(error.code)),
    };
    if !existing_receipts(root)?.is_empty()
        || quarantine_path(root, &operation.operation_id).exists()
    {
        return Ok(PreparedRecoveryInspection::refused(
            "prepared_recovery_destination_conflict",
        ));
    }
    let wal = root.join(WAL_FILENAME);
    let shm = root.join(SHM_FILENAME);
    if !wal.is_file() || !shm.is_file() {
        return Ok(PreparedRecoveryInspection::refused(
            "prepared_recovery_sidecar_pair_missing",
        ));
    }
    direct_file(live, root, "database")?;
    direct_file(&wal, root, "wal")?;
    direct_file(&shm, root, "shm")?;
    prove_empty_wal(&wal)?;
    prove_matching_zero_frame_shm(&shm)?;

    let database = file_fact(live, root, DATABASE_FILENAME, "database")?;
    let database_identity = file_identity(live)?;
    let operation_name = operation.root.file_name().unwrap().to_string_lossy();
    let mut files = vec![
        file_fact(
            &operation.state,
            &operation.root,
            &format!("{operation_name}/state.json"),
            "state",
        )?,
        file_fact(
            &operation.staging,
            &operation.root,
            &format!("{operation_name}/staging.db"),
            "staging",
        )?,
        file_fact(&wal, root, WAL_FILENAME, "wal")?,
        file_fact(&shm, root, SHM_FILENAME, "shm")?,
    ];
    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    if prepared.operation_id != operation.operation_id
        || prepared.state_size
            != files
                .iter()
                .find(|fact| fact.relative_path.ends_with("state.json"))
                .unwrap()
                .size
        || prepared.state_sha256
            != files
                .iter()
                .find(|fact| fact.relative_path.ends_with("state.json"))
                .unwrap()
                .sha256
        || prepared.staging_size != 0
        || prepared.staging_sha256
            != files
                .iter()
                .find(|fact| fact.relative_path.ends_with("staging.db"))
                .unwrap()
                .sha256
    {
        return Ok(PreparedRecoveryInspection::refused(
            "prepared_recovery_operation_changed",
        ));
    }
    Ok(PreparedRecoveryInspection {
        eligible: true,
        reason: None,
        classification: Some(RECOVERY_CLASSIFICATION.into()),
        operation_id: Some(operation.operation_id.clone()),
        claim_digest: Some(claim_digest(
            RECOVERY_CLASSIFICATION,
            &operation.operation_id,
            &database,
            &database_identity,
            &files,
            &[],
        )),
        database: Some(database),
        database_identity: Some(database_identity),
        evidence_files: files,
        preserved_files: Vec::new(),
    })
}

async fn inspect_v5_sidecar_at(
    root: &Path,
    live: &Path,
) -> Result<PreparedRecoveryInspection, String> {
    if !root.is_dir() || !live.is_file() {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_database_missing",
        ));
    }
    if root.join(JOURNAL_FILENAME).exists() {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_journal_present",
        ));
    }
    if read_user_version_without_sql(live)? != 5 {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_schema_not_v5",
        ));
    }
    let operations = operation_candidates(root, live)?;
    if operations.len() != 1 {
        return Ok(PreparedRecoveryInspection::refused(
            if operations.is_empty() {
                "v5_sidecar_recovery_operation_missing"
            } else {
                "v5_sidecar_recovery_operation_not_unique"
            },
        ));
    }
    let operation = &operations[0];
    let evidence = match read_migration_state_evidence(operation) {
        Ok(value) => value,
        Err(error) => return Ok(PreparedRecoveryInspection::refused(error.code)),
    };
    if evidence.phase != MigrationOperationPhase::V5Ready
        || evidence.outcome_class.as_deref() != Some("lifecycle_writes_enabled")
    {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_operation_not_v5_ready",
        ));
    }
    let wal = root.join(WAL_FILENAME);
    let shm = root.join(SHM_FILENAME);
    if !wal.is_file() || !shm.is_file() {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_sidecar_pair_missing",
        ));
    }
    direct_file(live, root, "database")?;
    direct_file(&wal, root, "wal")?;
    direct_file(&shm, root, "shm")?;
    prove_empty_wal(&wal)?;
    prove_matching_zero_frame_shm(&shm)?;

    let receipt = match verify_any_committed_v5_after_empty_sidecar_proof(live).await {
        Ok(value) => value,
        Err(error) => return Ok(PreparedRecoveryInspection::refused(error.code)),
    };
    if evidence.migration_id.as_deref() != Some(receipt.migration_id.as_str())
        || evidence.migration_source_manifest_digest.as_deref()
            != Some(receipt.source_manifest_digest.as_str())
        || evidence.migration_target_manifest_digest.as_deref()
            != Some(receipt.target_manifest_digest.as_str())
        || receipt.backup_id.as_deref() != Some(operation.operation_id.as_str())
    {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_operation_contradictory",
        ));
    }
    let expected_backup = match evidence.verified_backup {
        Some(value) => value,
        None => {
            return Ok(PreparedRecoveryInspection::refused(
                "v5_sidecar_recovery_backup_evidence_missing",
            ))
        }
    };
    if verify_owned_backup_for_migration(operation, &expected_backup, &ExactV4CandidateVerifier)
        .await
        .is_err()
    {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_backup_verification_failed",
        ));
    }
    let operation_names = fs::read_dir(&operation.root)
        .map_err(|_| "v5_sidecar_recovery_operation_unreadable")?
        .map(|entry| {
            entry
                .map(|value| value.file_name().to_string_lossy().to_string())
                .map_err(|_| "v5_sidecar_recovery_operation_unreadable".to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if operation_names
        != BTreeSet::from([
            "backup.db".to_string(),
            "staging.db".to_string(),
            "state.json".to_string(),
        ])
        || fs::metadata(&operation.staging)
            .map_err(|_| "v5_sidecar_recovery_staging_unreadable")?
            .len()
            != 0
    {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_operation_files_invalid",
        ));
    }
    let prior_receipts = existing_receipts(root)?;
    let prior_summary = if prior_receipts.len() == 1 {
        recovery_receipt_summary(root)
    } else {
        None
    };
    let prior_summary = match prior_summary {
        Some(value) => value,
        None => {
            return Ok(PreparedRecoveryInspection::refused(
                "v5_sidecar_recovery_prior_receipt_invalid",
            ))
        }
    };
    if !existing_v5_sidecar_receipts(root)?.is_empty()
        || v5_sidecar_receipt_path(root, &operation.operation_id).exists()
        || v5_sidecar_quarantine_path(root, &operation.operation_id).exists()
    {
        return Ok(PreparedRecoveryInspection::refused(
            "v5_sidecar_recovery_destination_conflict",
        ));
    }

    let database = file_fact(live, root, DATABASE_FILENAME, "database")?;
    let database_identity = file_identity(live)?;
    let mut moved_files = vec![
        file_fact(&wal, root, WAL_FILENAME, "wal")?,
        file_fact(&shm, root, SHM_FILENAME, "shm")?,
    ];
    moved_files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    let preserved_files = collect_v5_preserved_files(root, operation, &prior_summary)?;

    Ok(PreparedRecoveryInspection {
        eligible: true,
        reason: None,
        classification: Some(V5_SIDECAR_RECOVERY_CLASSIFICATION.into()),
        operation_id: Some(operation.operation_id.clone()),
        claim_digest: Some(claim_digest(
            V5_SIDECAR_RECOVERY_CLASSIFICATION,
            &operation.operation_id,
            &database,
            &database_identity,
            &moved_files,
            &preserved_files,
        )),
        database: Some(database),
        database_identity: Some(database_identity),
        evidence_files: moved_files,
        preserved_files,
    })
}

fn inspect_any_at(root: &Path, live: &Path) -> Result<PreparedRecoveryInspection, String> {
    match read_user_version_without_sql(live)? {
        4 => inspect_at(root, live),
        5 if root.join(WAL_FILENAME).exists() || root.join(SHM_FILENAME).exists() => {
            tauri::async_runtime::block_on(inspect_v5_sidecar_at(root, live))
        }
        5 => tauri::async_runtime::block_on(inspect_post_commit_manifest_at(root, live)),
        _ => Ok(PreparedRecoveryInspection::refused(
            "prepared_recovery_schema_unsupported",
        )),
    }
}

#[cfg(windows)]
struct DatabaseActivityGuard(File);

#[cfg(windows)]
impl Drop for DatabaseActivityGuard {
    fn drop(&mut self) {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::{
            Win32::Storage::FileSystem::UnlockFileEx, Win32::System::IO::OVERLAPPED,
        };
        let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
        overlapped.Anonymous.Anonymous.Offset = 0x4000_0000;
        unsafe {
            UnlockFileEx(self.0.as_raw_handle().cast(), 0, 512, 0, &mut overlapped);
        }
    }
}

#[cfg(windows)]
fn open_database_exclusive(path: &Path) -> Result<DatabaseActivityGuard, String> {
    use std::os::windows::{fs::OpenOptionsExt, io::AsRawHandle};
    use windows_sys::{
        Win32::Storage::FileSystem::{
            LockFileEx, LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY,
        },
        Win32::System::IO::OVERLAPPED,
    };
    let file = OpenOptions::new()
        .read(true)
        // Existing SQLite writers request write sharing. By granting read
        // sharing only, this handle can coexist with our own bounded reads
        // while excluding write and delete access for the recovery window.
        .share_mode(1)
        .open(path)
        .map_err(|_| "prepared_recovery_database_activity_not_exclusive")?;
    let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
    // SQLite's Win32 locking page starts at PENDING_BYTE (0x40000000).
    // Locking its complete 512-byte coordination range conflicts with active
    // SQLite shared/reserved/pending/exclusive locks while leaving ordinary
    // closed-file reads of database content below that range possible.
    overlapped.Anonymous.Anonymous.Offset = 0x4000_0000;
    let succeeded = unsafe {
        LockFileEx(
            file.as_raw_handle().cast(),
            LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY,
            0,
            512,
            0,
            &mut overlapped,
        )
    };
    if succeeded == 0 {
        Err("prepared_recovery_database_activity_not_exclusive".into())
    } else {
        Ok(DatabaseActivityGuard(file))
    }
}

#[cfg(windows)]
fn move_with_write_through(source: &Path, destination: &Path) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_WRITE_THROUGH};
    let mut source_wide: Vec<u16> = source.as_os_str().encode_wide().collect();
    source_wide.push(0);
    let mut destination_wide: Vec<u16> = destination.as_os_str().encode_wide().collect();
    destination_wide.push(0);
    let succeeded = unsafe {
        MoveFileExW(
            source_wide.as_ptr(),
            destination_wide.as_ptr(),
            MOVEFILE_WRITE_THROUGH,
        )
    };
    if succeeded == 0 {
        Err("prepared_recovery_quarantine_move_failed".into())
    } else {
        Ok(())
    }
}

#[cfg(not(windows))]
fn move_with_write_through(source: &Path, destination: &Path) -> Result<(), String> {
    fs::rename(source, destination).map_err(|_| "prepared_recovery_quarantine_move_failed".into())
}

#[cfg(not(windows))]
struct DatabaseActivityGuard(File);

#[cfg(not(windows))]
fn open_database_exclusive(path: &Path) -> Result<DatabaseActivityGuard, String> {
    File::open(path)
        .map(DatabaseActivityGuard)
        .map_err(|_| "prepared_recovery_database_activity_not_exclusive".into())
}

fn execute_at(
    root: &Path,
    live: &Path,
    expected_claim_digest: &str,
    expected_database_sha256: &str,
) -> Result<PreparedRecoveryResult, String> {
    execute_at_with_failure(
        root,
        live,
        expected_claim_digest,
        expected_database_sha256,
        RecoveryFailurePoint::None,
    )
}

fn execute_at_with_failure(
    root: &Path,
    live: &Path,
    expected_claim_digest: &str,
    expected_database_sha256: &str,
    failure_point: RecoveryFailurePoint,
) -> Result<PreparedRecoveryResult, String> {
    let _database_guard = open_database_exclusive(live)?;
    let inspection = inspect_at(root, live)?;
    if !inspection.eligible {
        return Err(inspection
            .reason
            .unwrap_or_else(|| "prepared_recovery_ineligible".into()));
    }
    let actual_claim = inspection.claim_digest.as_deref().unwrap();
    let database = inspection.database.clone().unwrap();
    if actual_claim != expected_claim_digest || database.sha256 != expected_database_sha256 {
        return Err("prepared_recovery_claim_changed".into());
    }
    let operation_id = inspection.operation_id.clone().unwrap();
    let receipt = receipt_path(root, &operation_id);
    let quarantine = quarantine_path(root, &operation_id);
    if receipt.exists() || quarantine.exists() || !existing_receipts(root)?.is_empty() {
        return Err("prepared_recovery_destination_conflict".into());
    }
    fs::create_dir(&quarantine).map_err(|_| "prepared_recovery_quarantine_create_failed")?;
    inject(failure_point, RecoveryFailurePoint::AfterQuarantineCreate)?;
    let operation = operation_candidates(root, live)?
        .into_iter()
        .next()
        .ok_or("prepared_recovery_operation_missing")?;
    let moves = [
        (root.join(WAL_FILENAME), quarantine.join(WAL_FILENAME)),
        (root.join(SHM_FILENAME), quarantine.join(SHM_FILENAME)),
        (operation.staging.clone(), quarantine.join("staging.db")),
        (operation.state.clone(), quarantine.join("state.json")),
    ];
    for (index, (source, destination)) in moves.iter().enumerate() {
        move_with_write_through(source, destination)?;
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(destination)
            .and_then(|file| file.sync_all())
            .map_err(|_| "prepared_recovery_quarantine_sync_failed")?;
        inject(
            failure_point,
            match index {
                0 => RecoveryFailurePoint::AfterWalMove,
                1 => RecoveryFailurePoint::AfterShmMove,
                2 => RecoveryFailurePoint::AfterStagingMove,
                _ => RecoveryFailurePoint::AfterStateMove,
            },
        )?;
    }
    fs::remove_dir(&operation.root).map_err(|_| "prepared_recovery_operation_cleanup_failed")?;
    inject(failure_point, RecoveryFailurePoint::AfterOperationCleanup)?;
    #[cfg(unix)]
    File::open(root)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "prepared_recovery_root_sync_failed")?;

    let mut quarantined_files = Vec::new();
    for (_, destination) in &moves {
        let name = destination
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        quarantined_files.push(file_fact(
            destination,
            &quarantine,
            &format!(
                "{}/{}",
                quarantine.file_name().unwrap().to_string_lossy(),
                name
            ),
            "quarantine",
        )?);
    }
    quarantined_files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    for quarantined in &quarantined_files {
        let name = Path::new(&quarantined.relative_path)
            .file_name()
            .ok_or("prepared_recovery_quarantine_fact_invalid")?;
        let source = inspection
            .evidence_files
            .iter()
            .find(|fact| Path::new(&fact.relative_path).file_name() == Some(name))
            .ok_or("prepared_recovery_quarantine_fact_missing")?;
        if source.size != quarantined.size || source.sha256 != quarantined.sha256 {
            return Err("prepared_recovery_evidence_changed_during_move".into());
        }
    }
    let record = PreparedRecoveryReceipt {
        schema_version: RECOVERY_RECEIPT_SCHEMA,
        classification: RECOVERY_CLASSIFICATION.into(),
        operation_id: operation_id.clone(),
        application_version: env!("CARGO_PKG_VERSION").into(),
        database_schema_version: 4,
        recovered_at: canonical_now()?,
        database: database.clone(),
        quarantined_files,
    };
    let encoded = serde_json::to_vec_pretty(&record)
        .map_err(|_| "prepared_recovery_receipt_encode_failed")?;
    inject(failure_point, RecoveryFailurePoint::BeforeReceiptWrite)?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&receipt)
        .map_err(|_| "prepared_recovery_receipt_create_failed")?;
    output
        .write_all(&encoded)
        .map_err(|_| "prepared_recovery_receipt_write_failed")?;
    output
        .sync_all()
        .map_err(|_| "prepared_recovery_receipt_sync_failed")?;
    inject(failure_point, RecoveryFailurePoint::AfterReceiptWrite)?;
    if sha256_file(live)? != expected_database_sha256
        || read_user_version_without_sql(live)? != 4
        || root.join(WAL_FILENAME).exists()
        || root.join(SHM_FILENAME).exists()
        || root.join(JOURNAL_FILENAME).exists()
        || !operation_candidates(root, live)?.is_empty()
    {
        return Err("prepared_recovery_postcondition_failed".into());
    }
    Ok(PreparedRecoveryResult {
        classification: RECOVERY_CLASSIFICATION.into(),
        operation_id,
        receipt_relative_path: receipt.file_name().unwrap().to_string_lossy().to_string(),
        database_sha256: database.sha256,
        restart_required: true,
    })
}

fn execute_v5_sidecar_at(
    root: &Path,
    live: &Path,
    expected_claim_digest: &str,
    expected_database_sha256: &str,
) -> Result<PreparedRecoveryResult, String> {
    execute_v5_sidecar_at_with_failure(
        root,
        live,
        expected_claim_digest,
        expected_database_sha256,
        RecoveryFailurePoint::None,
    )
}

fn execute_v5_sidecar_at_with_failure(
    root: &Path,
    live: &Path,
    expected_claim_digest: &str,
    expected_database_sha256: &str,
    failure_point: RecoveryFailurePoint,
) -> Result<PreparedRecoveryResult, String> {
    let _database_guard = open_database_exclusive(live)?;
    let inspection = tauri::async_runtime::block_on(inspect_v5_sidecar_at(root, live))?;
    if !inspection.eligible
        || inspection.classification.as_deref() != Some(V5_SIDECAR_RECOVERY_CLASSIFICATION)
    {
        return Err(inspection
            .reason
            .unwrap_or_else(|| "v5_sidecar_recovery_ineligible".into()));
    }
    let actual_claim = inspection.claim_digest.as_deref().unwrap();
    let database = inspection.database.clone().unwrap();
    if actual_claim != expected_claim_digest || database.sha256 != expected_database_sha256 {
        return Err("v5_sidecar_recovery_claim_changed".into());
    }
    let operation_id = inspection.operation_id.clone().unwrap();
    let receipt = v5_sidecar_receipt_path(root, &operation_id);
    let quarantine = v5_sidecar_quarantine_path(root, &operation_id);
    if receipt.exists() || quarantine.exists() || !existing_v5_sidecar_receipts(root)?.is_empty() {
        return Err("v5_sidecar_recovery_destination_conflict".into());
    }
    fs::create_dir(&quarantine).map_err(|_| "v5_sidecar_recovery_quarantine_create_failed")?;
    inject(failure_point, RecoveryFailurePoint::AfterQuarantineCreate)?;
    let moves = [
        (root.join(WAL_FILENAME), quarantine.join(WAL_FILENAME)),
        (root.join(SHM_FILENAME), quarantine.join(SHM_FILENAME)),
    ];
    for (index, (source, destination)) in moves.iter().enumerate() {
        move_with_write_through(source, destination)?;
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(destination)
            .and_then(|file| file.sync_all())
            .map_err(|_| "v5_sidecar_recovery_quarantine_sync_failed")?;
        inject(
            failure_point,
            if index == 0 {
                RecoveryFailurePoint::AfterWalMove
            } else {
                RecoveryFailurePoint::AfterShmMove
            },
        )?;
    }
    #[cfg(unix)]
    File::open(root)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "v5_sidecar_recovery_root_sync_failed")?;

    let mut quarantined_files = Vec::new();
    for (_, destination) in &moves {
        let name = destination
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        quarantined_files.push(file_fact(
            destination,
            &quarantine,
            &format!(
                "{}/{}",
                quarantine.file_name().unwrap().to_string_lossy(),
                name
            ),
            "v5_sidecar_quarantine",
        )?);
    }
    quarantined_files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    for quarantined in &quarantined_files {
        let name = Path::new(&quarantined.relative_path)
            .file_name()
            .ok_or("v5_sidecar_recovery_quarantine_fact_invalid")?;
        let source = inspection
            .evidence_files
            .iter()
            .find(|fact| Path::new(&fact.relative_path).file_name() == Some(name))
            .ok_or("v5_sidecar_recovery_quarantine_fact_missing")?;
        if source.size != quarantined.size || source.sha256 != quarantined.sha256 {
            return Err("v5_sidecar_recovery_evidence_changed_during_move".into());
        }
    }

    let operations = operation_candidates(root, live)?;
    if operations.len() != 1 || operations[0].operation_id != operation_id {
        return Err("v5_sidecar_recovery_operation_changed".into());
    }
    let prior_summary =
        recovery_receipt_summary(root).ok_or("v5_sidecar_recovery_prior_receipt_changed")?;
    let preserved_files = collect_v5_preserved_files(root, &operations[0], &prior_summary)?;
    if preserved_files != inspection.preserved_files {
        return Err("v5_sidecar_recovery_preserved_evidence_changed".into());
    }
    let record = V5SidecarRecoveryReceipt {
        schema_version: RECOVERY_RECEIPT_SCHEMA,
        classification: V5_SIDECAR_RECOVERY_CLASSIFICATION.into(),
        operation_id: operation_id.clone(),
        application_version: env!("CARGO_PKG_VERSION").into(),
        database_schema_version: 5,
        recovered_at: canonical_now()?,
        database: database.clone(),
        preserved_files: preserved_files.clone(),
        quarantined_files,
    };
    let encoded = serde_json::to_vec_pretty(&record)
        .map_err(|_| "v5_sidecar_recovery_receipt_encode_failed")?;
    inject(failure_point, RecoveryFailurePoint::BeforeReceiptWrite)?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&receipt)
        .map_err(|_| "v5_sidecar_recovery_receipt_create_failed")?;
    output
        .write_all(&encoded)
        .map_err(|_| "v5_sidecar_recovery_receipt_write_failed")?;
    output
        .sync_all()
        .map_err(|_| "v5_sidecar_recovery_receipt_sync_failed")?;
    inject(failure_point, RecoveryFailurePoint::AfterReceiptWrite)?;

    if sha256_file(live)? != expected_database_sha256
        || read_user_version_without_sql(live)? != 5
        || root.join(WAL_FILENAME).exists()
        || root.join(SHM_FILENAME).exists()
        || root.join(JOURNAL_FILENAME).exists()
        || collect_v5_preserved_files(root, &operations[0], &prior_summary)? != preserved_files
    {
        return Err("v5_sidecar_recovery_postcondition_failed".into());
    }
    let durable_receipt = tauri::async_runtime::block_on(verify_any_committed_v5(live))
        .map_err(|error| error.code)?;
    let durable_evidence =
        read_migration_state_evidence(&operations[0]).map_err(|error| error.code)?;
    if durable_evidence.phase != MigrationOperationPhase::V5Ready
        || durable_evidence.migration_id.as_deref() != Some(durable_receipt.migration_id.as_str())
        || durable_receipt.backup_id.as_deref() != Some(operation_id.as_str())
    {
        return Err("v5_sidecar_recovery_postcondition_failed".into());
    }
    let expected_backup = durable_evidence
        .verified_backup
        .ok_or("v5_sidecar_recovery_backup_evidence_missing")?;
    tauri::async_runtime::block_on(verify_owned_backup_for_migration(
        &operations[0],
        &expected_backup,
        &ExactV4CandidateVerifier,
    ))
    .map_err(|error| error.code)?;

    Ok(PreparedRecoveryResult {
        classification: V5_SIDECAR_RECOVERY_CLASSIFICATION.into(),
        operation_id,
        receipt_relative_path: receipt.file_name().unwrap().to_string_lossy().to_string(),
        database_sha256: database.sha256,
        restart_required: true,
    })
}

fn execute_post_commit_manifest_at(
    root: &Path,
    live: &Path,
    expected_claim_digest: &str,
    expected_database_sha256: &str,
) -> Result<PreparedRecoveryResult, String> {
    let inspection = tauri::async_runtime::block_on(inspect_post_commit_manifest_at(root, live))?;
    if !inspection.eligible {
        return Err(inspection
            .reason
            .unwrap_or_else(|| "post_commit_recovery_ineligible".into()));
    }
    let database_before = inspection.database.clone().unwrap();
    if inspection.claim_digest.as_deref() != Some(expected_claim_digest)
        || database_before.sha256 != expected_database_sha256
    {
        return Err("post_commit_recovery_claim_changed".into());
    }
    let operation_id = inspection.operation_id.clone().unwrap();
    let operations = operation_candidates(root, live)?;
    if operations.len() != 1 || operations[0].operation_id != operation_id {
        return Err("post_commit_recovery_operation_changed".into());
    }
    let operation = &operations[0];
    let evidence = read_migration_state_evidence(operation).map_err(|error| error.code)?;
    let migration_receipt = exact_post_commit_receipt(operation, &evidence)
        .ok_or("post_commit_recovery_operation_changed")?;
    let state_before = inspection
        .evidence_files
        .first()
        .cloned()
        .ok_or("post_commit_recovery_state_fact_missing")?;
    let prior_summary =
        recovery_receipt_summary(root).ok_or("post_commit_recovery_prior_receipt_changed")?;
    if collect_post_commit_preserved_files(root, operation, &prior_summary)?
        != inspection.preserved_files
    {
        return Err("post_commit_recovery_preserved_evidence_changed".into());
    }

    let lifecycle_already_enabled = tauri::async_runtime::block_on(
        verify_exact_historical_frontend_post_commit_v5(live, &migration_receipt, "enabled"),
    )
    .is_ok();
    if !lifecycle_already_enabled {
        tauri::async_runtime::block_on(verify_exact_historical_frontend_post_commit_v5(
            live,
            &migration_receipt,
            "disabled",
        ))
        .map_err(|error| error.code)?;
        let now = canonical_now()?;
        tauri::async_runtime::block_on(activate_exact_historical_frontend_lifecycle_writes(
            live,
            &migration_receipt,
            &now,
        ))
        .map_err(|error| error.code)?;
    }
    tauri::async_runtime::block_on(verify_exact_historical_frontend_post_commit_v5(
        live,
        &migration_receipt,
        "enabled",
    ))
    .map_err(|error| error.code)?;

    let database_after = file_fact(live, root, DATABASE_FILENAME, "database")?;
    let receipt_path = post_commit_manifest_receipt_path(root, &operation_id);
    if !receipt_path.exists() {
        let record = PostCommitManifestRecoveryReceipt {
            schema_version: RECOVERY_RECEIPT_SCHEMA,
            classification: POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION.into(),
            operation_id: operation_id.clone(),
            application_version: env!("CARGO_PKG_VERSION").into(),
            database_schema_version: 5,
            recovered_at: canonical_now()?,
            migration_id: migration_receipt.migration_id.clone(),
            source_manifest_digest: migration_receipt.source_manifest_digest.clone(),
            target_manifest_digest: migration_receipt.target_manifest_digest.clone(),
            database: database_after.clone(),
            operation_state_before: state_before,
            preserved_files: inspection.preserved_files.clone(),
            lifecycle_writes: "enabled".into(),
        };
        let encoded = serde_json::to_vec_pretty(&record)
            .map_err(|_| "post_commit_recovery_receipt_encode_failed")?;
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&receipt_path)
            .map_err(|_| "post_commit_recovery_receipt_create_failed")?;
        output
            .write_all(&encoded)
            .map_err(|_| "post_commit_recovery_receipt_write_failed")?;
        output
            .sync_all()
            .map_err(|_| "post_commit_recovery_receipt_sync_failed")?;
    }

    record_migration_state(
        operation,
        MigrationOperationPhase::V5Ready,
        Some("lifecycle_writes_enabled".into()),
        Some(MigrationReceiptEvidence {
            migration_id: &migration_receipt.migration_id,
            source_manifest_digest: &migration_receipt.source_manifest_digest,
            target_manifest_digest: &migration_receipt.target_manifest_digest,
        }),
    )
    .map_err(|error| error.code)?;

    let durable_receipt = tauri::async_runtime::block_on(verify_any_committed_v5(live))
        .map_err(|error| error.code)?;
    let durable_evidence = read_migration_state_evidence(operation).map_err(|error| error.code)?;
    if durable_receipt != migration_receipt
        || durable_evidence.phase != MigrationOperationPhase::V5Ready
        || durable_evidence.outcome_class.as_deref() != Some("lifecycle_writes_enabled")
        || collect_post_commit_preserved_files(root, operation, &prior_summary)?
            != inspection.preserved_files
    {
        return Err("post_commit_recovery_postcondition_failed".into());
    }

    Ok(PreparedRecoveryResult {
        classification: POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION.into(),
        operation_id,
        receipt_relative_path: receipt_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        database_sha256: database_after.sha256,
        restart_required: true,
    })
}

fn require_ordinary(app: &AppHandle) -> Result<(), String> {
    if app.config().identifier == ORDINARY_IDENTIFIER && cfg!(feature = "desktop-schema-v5") {
        Ok(())
    } else {
        Err("prepared_recovery_ordinary_identity_required".into())
    }
}

#[tauri::command]
pub fn inspect_prepared_state_recovery(
    app: AppHandle,
) -> Result<PreparedRecoveryInspection, String> {
    require_ordinary(&app)?;
    let (root, live) = paths(&app)?;
    match inspect_any_at(&root, &live) {
        Ok(value) => Ok(value),
        Err(reason) => Ok(PreparedRecoveryInspection::refused(reason)),
    }
}

#[tauri::command]
pub fn execute_prepared_state_recovery(
    app: AppHandle,
    expected_claim_digest: String,
    expected_database_sha256: String,
) -> Result<PreparedRecoveryResult, String> {
    require_ordinary(&app)?;
    let _operation_guard = operation_lock()?;
    let (root, live) = paths(&app)?;
    match read_user_version_without_sql(&live)? {
        4 => execute_at(
            &root,
            &live,
            &expected_claim_digest,
            &expected_database_sha256,
        ),
        5 if root.join(WAL_FILENAME).exists() || root.join(SHM_FILENAME).exists() => {
            execute_v5_sidecar_at(
                &root,
                &live,
                &expected_claim_digest,
                &expected_database_sha256,
            )
        }
        5 => execute_post_commit_manifest_at(
            &root,
            &live,
            &expected_claim_digest,
            &expected_database_sha256,
        ),
        _ => Err("prepared_recovery_schema_unsupported".into()),
    }
}

pub(crate) fn recovery_receipt_summary(root: &Path) -> Option<RecoveryReceiptSummary> {
    let receipts = existing_receipts(root).ok()?;
    if receipts.len() != 1 {
        return None;
    }
    let path = &receipts[0];
    direct_file(path, root, "receipt").ok()?;
    let record: PreparedRecoveryReceipt = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
    if record.schema_version != RECOVERY_RECEIPT_SCHEMA
        || record.classification != RECOVERY_CLASSIFICATION
        || record.database_schema_version != 4
        || record.application_version != env!("CARGO_PKG_VERSION")
        || path.file_name()?.to_string_lossy()
            != format!("prepared-recovery-{}.receipt.json", record.operation_id)
        || record.operation_id.len() != 32
        || !record
            .operation_id
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || OffsetDateTime::parse(
            &record.recovered_at,
            &time::format_description::well_known::Rfc3339,
        )
        .is_err()
        || record.database.relative_path != DATABASE_FILENAME
        || record.database.sha256.len() != 64
        || !record
            .database
            .sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return None;
    }
    let quarantine = quarantine_path(root, &record.operation_id);
    let canonical_root = fs::canonicalize(root).ok()?;
    let canonical_quarantine = fs::canonicalize(&quarantine).ok()?;
    if canonical_quarantine.parent() != Some(canonical_root.as_path())
        || !canonical_quarantine.is_dir()
        || record.quarantined_files.len() != 4
    {
        return None;
    }
    let expected_names = BTreeSet::from([
        SHM_FILENAME.to_string(),
        WAL_FILENAME.to_string(),
        "staging.db".to_string(),
        "state.json".to_string(),
    ]);
    let actual_names = record
        .quarantined_files
        .iter()
        .map(|fact| {
            Path::new(&fact.relative_path)
                .file_name()
                .map(|value| value.to_string_lossy().to_string())
        })
        .collect::<Option<BTreeSet<_>>>()?;
    if actual_names != expected_names {
        return None;
    }
    for fact in &record.quarantined_files {
        let expected_relative = format!(
            "{}/{}",
            quarantine.file_name()?.to_string_lossy(),
            Path::new(&fact.relative_path)
                .file_name()?
                .to_string_lossy()
        );
        if fact.relative_path != expected_relative {
            return None;
        }
        let candidate = quarantine.join(Path::new(&fact.relative_path).file_name()?);
        let current = file_fact(&candidate, &quarantine, &fact.relative_path, "quarantine").ok()?;
        if current != *fact {
            return None;
        }
    }
    Some(RecoveryReceiptSummary {
        relative_path: path.file_name()?.to_string_lossy().to_string(),
        operation_id: record.operation_id,
        recovered_at: record.recovered_at,
    })
}

fn delete_receipt_at(root: &Path, relative_path: &str) -> Result<(), String> {
    let summary = recovery_receipt_summary(root).ok_or("prepared_recovery_receipt_unavailable")?;
    if summary.relative_path != relative_path || relative_path.contains(['/', '\\']) {
        return Err("prepared_recovery_receipt_identity_mismatch".into());
    }
    let path = root.join(relative_path);
    direct_file(&path, root, "receipt")?;
    fs::remove_file(path).map_err(|_| "prepared_recovery_receipt_delete_failed")?;
    Ok(())
}

#[tauri::command]
pub fn delete_prepared_recovery_receipt(
    app: AppHandle,
    relative_path: String,
) -> Result<(), String> {
    require_ordinary(&app)?;
    let _operation_guard = operation_lock()?;
    let (root, _) = paths(&app)?;
    delete_receipt_at(&root, &relative_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filesystem_safety::{
        prepare_operation, DatabaseActivity, ExclusiveOperationGuard, OwnedOperation,
        QuiescenceProbe, SystemVolumeProbe,
    };
    use sqlx::{Connection, SqliteConnection};
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");

    type FixtureMutation = Box<dyn Fn(&TempDir, &Path, &OwnedOperation)>;

    struct Quiescent;
    impl QuiescenceProbe for Quiescent {
        fn database_activity(&self) -> DatabaseActivity {
            DatabaseActivity::Quiescent
        }
    }

    fn write_empty_sidecars(root: &Path) {
        fs::write(root.join(WAL_FILENAME), []).unwrap();
        let mut shm = vec![0_u8; 32_768];
        shm[0..4].copy_from_slice(&3_007_000_u32.to_le_bytes());
        shm[12] = 1;
        let first = shm[0..48].to_vec();
        shm[48..96].copy_from_slice(&first);
        fs::write(root.join(SHM_FILENAME), shm).unwrap();
    }

    fn fixture() -> (TempDir, PathBuf, OwnedOperation) {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        let mut bytes = vec![0_u8; 4096];
        bytes[..16].copy_from_slice(b"SQLite format 3\0");
        bytes[60..64].copy_from_slice(&4_u32.to_be_bytes());
        fs::write(&live, bytes).unwrap();
        let probe = Quiescent;
        let guard = ExclusiveOperationGuard::acquire(&probe).unwrap();
        let operation =
            prepare_operation(directory.path(), &live, &guard, &SystemVolumeProbe).unwrap();
        write_empty_sidecars(directory.path());
        (directory, live, operation)
    }

    async fn v5_sidecar_fixture() -> (TempDir, PathBuf, OwnedOperation) {
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
        let v4_database =
            file_fact(&live, directory.path(), DATABASE_FILENAME, "database").unwrap();

        crate::schema_v5_founder_activation::authorize_founder_schema_v5_migration_at(
            directory.path(),
            &live,
        )
        .await
        .unwrap();
        let operation = operation_candidates(directory.path(), &live)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let prior_id = "a".repeat(32);
        let prior_quarantine = quarantine_path(directory.path(), &prior_id);
        fs::create_dir(&prior_quarantine).unwrap();
        fs::write(prior_quarantine.join(WAL_FILENAME), []).unwrap();
        fs::write(prior_quarantine.join(SHM_FILENAME), b"prior-shm").unwrap();
        fs::write(prior_quarantine.join("staging.db"), []).unwrap();
        fs::write(prior_quarantine.join("state.json"), b"{}").unwrap();
        let mut quarantined_files = [WAL_FILENAME, SHM_FILENAME, "staging.db", "state.json"]
            .into_iter()
            .map(|name| {
                file_fact(
                    &prior_quarantine.join(name),
                    &prior_quarantine,
                    &format!(
                        "{}/{}",
                        prior_quarantine.file_name().unwrap().to_string_lossy(),
                        name
                    ),
                    "quarantine",
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        quarantined_files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        let prior_receipt = PreparedRecoveryReceipt {
            schema_version: RECOVERY_RECEIPT_SCHEMA,
            classification: RECOVERY_CLASSIFICATION.into(),
            operation_id: prior_id.clone(),
            application_version: env!("CARGO_PKG_VERSION").into(),
            database_schema_version: 4,
            recovered_at: "2026-08-30T00:00:00Z".into(),
            database: v4_database,
            quarantined_files,
        };
        fs::write(
            receipt_path(directory.path(), &prior_id),
            serde_json::to_vec_pretty(&prior_receipt).unwrap(),
        )
        .unwrap();
        assert!(recovery_receipt_summary(directory.path()).is_some());
        write_empty_sidecars(directory.path());
        (directory, live, operation)
    }

    async fn historical_post_commit_fixture() -> (TempDir, PathBuf, OwnedOperation) {
        let directory = TempDir::new().unwrap();
        let live = directory.path().join(DATABASE_FILENAME);
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&live)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        sqlx::raw_sql(
            r#"CREATE TABLE IF NOT EXISTS experience_entries (
      id TEXT PRIMARY KEY NOT NULL,
      content TEXT NOT NULL,
      created_at TEXT NOT NULL,
      updated_at TEXT NOT NULL
    )"#,
        )
        .execute(&mut connection)
        .await
        .unwrap();
        crate::sqlite::migrate_connection(&mut connection, false)
            .await
            .unwrap();
        connection.close().await.unwrap();
        let v4_database =
            file_fact(&live, directory.path(), DATABASE_FILENAME, "database").unwrap();

        crate::schema_v5_founder_activation::authorize_founder_schema_v5_migration_at(
            directory.path(),
            &live,
        )
        .await
        .unwrap();
        let operation = operation_candidates(directory.path(), &live)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        let prior_id = "b".repeat(32);
        let prior_quarantine = quarantine_path(directory.path(), &prior_id);
        fs::create_dir(&prior_quarantine).unwrap();
        fs::write(prior_quarantine.join(WAL_FILENAME), []).unwrap();
        fs::write(prior_quarantine.join(SHM_FILENAME), b"prior-shm").unwrap();
        fs::write(prior_quarantine.join("staging.db"), []).unwrap();
        fs::write(prior_quarantine.join("state.json"), b"{}").unwrap();
        let mut quarantined_files = [WAL_FILENAME, SHM_FILENAME, "staging.db", "state.json"]
            .into_iter()
            .map(|name| {
                file_fact(
                    &prior_quarantine.join(name),
                    &prior_quarantine,
                    &format!(
                        "{}/{}",
                        prior_quarantine.file_name().unwrap().to_string_lossy(),
                        name
                    ),
                    "quarantine",
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        quarantined_files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        let prior_receipt = PreparedRecoveryReceipt {
            schema_version: RECOVERY_RECEIPT_SCHEMA,
            classification: RECOVERY_CLASSIFICATION.into(),
            operation_id: prior_id.clone(),
            application_version: env!("CARGO_PKG_VERSION").into(),
            database_schema_version: 4,
            recovered_at: "2026-09-01T00:00:00Z".into(),
            database: v4_database,
            quarantined_files,
        };
        fs::write(
            receipt_path(directory.path(), &prior_id),
            serde_json::to_vec_pretty(&prior_receipt).unwrap(),
        )
        .unwrap();

        let mut connection = SqliteConnection::connect_with(
            &sqlx::sqlite::SqliteConnectOptions::new()
                .filename(&live)
                .foreign_keys(true),
        )
        .await
        .unwrap();
        sqlx::raw_sql("BEGIN IMMEDIATE")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES ('synthetic-reset', '2026-09-01T00:00:00Z')",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query(
            "UPDATE database_contract SET lifecycle_writes = 'disabled' WHERE singleton = 1",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = 'synthetic-reset'")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::raw_sql("COMMIT")
            .execute(&mut connection)
            .await
            .unwrap();
        connection.close().await.unwrap();
        let mut state: serde_json::Value =
            serde_json::from_slice(&fs::read(&operation.state).unwrap()).unwrap();
        state["phase"] = "v5_blocked_restore_available".into();
        state["outcome_class"] = "post_commit_schema_manifest_mismatch".into();
        fs::write(&operation.state, serde_json::to_vec_pretty(&state).unwrap()).unwrap();
        assert!(recovery_receipt_summary(directory.path()).is_some());
        (directory, live, operation)
    }

    fn protected_bytes(root: &Path, live: &Path, operation: &OwnedOperation) -> Vec<Vec<u8>> {
        let prior = recovery_receipt_summary(root).unwrap();
        let mut bytes = vec![
            fs::read(live).unwrap(),
            fs::read(&operation.state).unwrap(),
            fs::read(&operation.staging).unwrap(),
            fs::read(&operation.backup).unwrap(),
            fs::read(root.join(&prior.relative_path)).unwrap(),
        ];
        let prior_quarantine = quarantine_path(root, &prior.operation_id);
        let mut files = fs::read_dir(prior_quarantine)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        files.sort();
        bytes.extend(files.into_iter().map(|path| fs::read(path).unwrap()));
        bytes
    }

    #[test]
    fn exact_v5_ready_legacy_sidecars_are_explicitly_recoverable() {
        let (directory, live, operation) = tauri::async_runtime::block_on(v5_sidecar_fixture());
        let before = protected_bytes(directory.path(), &live, &operation);
        let normal_verifier_error =
            tauri::async_runtime::block_on(verify_any_committed_v5(&live)).unwrap_err();
        assert_eq!(normal_verifier_error.code, "sqlite_sidecar_present");
        let inspection =
            tauri::async_runtime::block_on(inspect_v5_sidecar_at(directory.path(), &live)).unwrap();
        assert!(inspection.eligible, "{:?}", inspection.reason);
        assert_eq!(
            inspection.classification.as_deref(),
            Some(V5_SIDECAR_RECOVERY_CLASSIFICATION)
        );
        assert_eq!(inspection.evidence_files.len(), 2);
        assert_eq!(inspection.preserved_files.len(), 8);

        let result = execute_v5_sidecar_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256,
        )
        .unwrap();

        assert_eq!(protected_bytes(directory.path(), &live, &operation), before);
        assert!(!directory.path().join(WAL_FILENAME).exists());
        assert!(!directory.path().join(SHM_FILENAME).exists());
        assert!(operation.root.exists());
        let receipt = directory.path().join(&result.receipt_relative_path);
        let receipt_value: serde_json::Value =
            serde_json::from_slice(&fs::read(receipt).unwrap()).unwrap();
        assert_eq!(
            receipt_value["classification"],
            V5_SIDECAR_RECOVERY_CLASSIFICATION
        );
        assert_eq!(receipt_value["databaseSchemaVersion"], 5);
        assert_eq!(
            receipt_value["quarantinedFiles"].as_array().unwrap().len(),
            2
        );
        assert_eq!(receipt_value["preservedFiles"].as_array().unwrap().len(), 8);
        assert!(!serde_json::to_string(&receipt_value)
            .unwrap()
            .contains("experience"));
    }

    #[test]
    fn v5_sidecar_inspection_and_cancel_are_write_free() {
        let (directory, live, operation) = tauri::async_runtime::block_on(v5_sidecar_fixture());
        let before = protected_bytes(directory.path(), &live, &operation);
        let wal = fs::read(directory.path().join(WAL_FILENAME)).unwrap();
        let shm = fs::read(directory.path().join(SHM_FILENAME)).unwrap();
        let inspection =
            tauri::async_runtime::block_on(inspect_v5_sidecar_at(directory.path(), &live)).unwrap();
        assert!(inspection.eligible, "{:?}", inspection.reason);
        assert_eq!(protected_bytes(directory.path(), &live, &operation), before);
        assert_eq!(fs::read(directory.path().join(WAL_FILENAME)).unwrap(), wal);
        assert_eq!(fs::read(directory.path().join(SHM_FILENAME)).unwrap(), shm);
        assert!(existing_v5_sidecar_receipts(directory.path())
            .unwrap()
            .is_empty());
    }

    #[test]
    fn v5_sidecar_claim_and_contract_drift_fail_closed() {
        for mutation in ["wal", "state", "backup", "prior-receipt", "journal"] {
            let (directory, live, operation) = tauri::async_runtime::block_on(v5_sidecar_fixture());
            let inspection =
                tauri::async_runtime::block_on(inspect_v5_sidecar_at(directory.path(), &live))
                    .unwrap();
            let database_before = fs::read(&live).unwrap();
            match mutation {
                "wal" => fs::write(directory.path().join(WAL_FILENAME), b"frame").unwrap(),
                "state" => fs::write(&operation.state, b"changed").unwrap(),
                "backup" => fs::write(&operation.backup, b"changed").unwrap(),
                "prior-receipt" => {
                    let prior = recovery_receipt_summary(directory.path()).unwrap();
                    fs::write(directory.path().join(prior.relative_path), b"changed").unwrap();
                }
                _ => fs::write(directory.path().join(JOURNAL_FILENAME), b"journal").unwrap(),
            }
            let result = execute_v5_sidecar_at(
                directory.path(),
                &live,
                inspection.claim_digest.as_deref().unwrap(),
                &inspection.database.as_ref().unwrap().sha256,
            );
            assert!(result.is_err(), "{mutation}");
            assert_eq!(fs::read(&live).unwrap(), database_before, "{mutation}");
            assert!(
                !v5_sidecar_quarantine_path(directory.path(), &operation.operation_id).exists(),
                "{mutation}"
            );
        }
    }

    #[test]
    fn v5_sidecar_partial_failures_never_change_protected_evidence() {
        for point in [
            RecoveryFailurePoint::AfterQuarantineCreate,
            RecoveryFailurePoint::AfterWalMove,
            RecoveryFailurePoint::AfterShmMove,
            RecoveryFailurePoint::BeforeReceiptWrite,
            RecoveryFailurePoint::AfterReceiptWrite,
        ] {
            let (directory, live, operation) = tauri::async_runtime::block_on(v5_sidecar_fixture());
            let before = protected_bytes(directory.path(), &live, &operation);
            let inspection =
                tauri::async_runtime::block_on(inspect_v5_sidecar_at(directory.path(), &live))
                    .unwrap();
            let result = execute_v5_sidecar_at_with_failure(
                directory.path(),
                &live,
                inspection.claim_digest.as_deref().unwrap(),
                &inspection.database.as_ref().unwrap().sha256,
                point,
            );
            assert!(result.is_err(), "{point:?}");
            assert_eq!(
                protected_bytes(directory.path(), &live, &operation),
                before,
                "{point:?}"
            );
        }
    }

    #[test]
    fn exact_historical_post_commit_state_is_explicitly_recoverable() {
        let (directory, live, operation) =
            tauri::async_runtime::block_on(historical_post_commit_fixture());
        let startup = tauri::async_runtime::block_on(
            crate::schema_v5_founder_activation::classify(directory.path(), &live),
        );
        let startup = serde_json::to_value(startup).unwrap();
        assert_eq!(startup["state"], "blocked");
        assert_eq!(
            startup["reason"],
            "post_commit_schema_manifest_recovery_available"
        );
        assert_eq!(startup["backupAvailable"], true);
        assert_eq!(startup["restoreAvailable"], false);
        let inspection = tauri::async_runtime::block_on(inspect_post_commit_manifest_at(
            directory.path(),
            &live,
        ))
        .unwrap();
        assert!(inspection.eligible, "{:?}", inspection.reason);
        assert_eq!(
            inspection.classification.as_deref(),
            Some(POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION)
        );
        assert_eq!(inspection.evidence_files.len(), 1);
        assert_eq!(inspection.preserved_files.len(), 7);
        let preserved_before = inspection
            .preserved_files
            .iter()
            .map(|fact| (fact.relative_path.clone(), fact.sha256.clone()))
            .collect::<Vec<_>>();

        let result = execute_post_commit_manifest_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256,
        )
        .unwrap();

        assert_eq!(
            result.classification,
            POST_COMMIT_MANIFEST_RECOVERY_CLASSIFICATION
        );
        let evidence = read_migration_state_evidence(&operation).unwrap();
        assert_eq!(evidence.phase, MigrationOperationPhase::V5Ready);
        assert_eq!(
            evidence.outcome_class.as_deref(),
            Some("lifecycle_writes_enabled")
        );
        tauri::async_runtime::block_on(verify_any_committed_v5(&live)).unwrap();
        let prior = recovery_receipt_summary(directory.path()).unwrap();
        let preserved_after =
            collect_post_commit_preserved_files(directory.path(), &operation, &prior)
                .unwrap()
                .into_iter()
                .map(|fact| (fact.relative_path, fact.sha256))
                .collect::<Vec<_>>();
        assert_eq!(preserved_after, preserved_before);

        let recovery_receipt =
            fs::read_to_string(directory.path().join(&result.receipt_relative_path)).unwrap();
        assert!(!recovery_receipt.contains("synthetic body"));
        assert!(!recovery_receipt.contains("experience_entries"));
        let created =
            tauri::async_runtime::block_on(crate::schema_v5_migration::runtime::create_experience(
                &live,
                "synthetic-post-commit-review",
                "synthetic body",
                "2026-09-02T00:00:00.000Z",
                "synthetic-post-commit-review-guard-0123456789abcdef",
            ))
            .unwrap();
        assert_eq!(created.id, "synthetic-post-commit-review");
    }

    #[test]
    fn post_commit_inspection_and_cancel_are_write_free() {
        let (directory, live, operation) =
            tauri::async_runtime::block_on(historical_post_commit_fixture());
        let before = protected_bytes(directory.path(), &live, &operation);
        let inspection = tauri::async_runtime::block_on(inspect_post_commit_manifest_at(
            directory.path(),
            &live,
        ))
        .unwrap();
        assert!(inspection.eligible, "{:?}", inspection.reason);
        assert_eq!(protected_bytes(directory.path(), &live, &operation), before);
        assert!(existing_post_commit_manifest_receipts(directory.path())
            .unwrap()
            .is_empty());
    }

    #[test]
    fn post_commit_claim_drift_fails_before_activation() {
        let (directory, live, operation) =
            tauri::async_runtime::block_on(historical_post_commit_fixture());
        let inspection = tauri::async_runtime::block_on(inspect_post_commit_manifest_at(
            directory.path(),
            &live,
        ))
        .unwrap();
        let database_before = fs::read(&live).unwrap();
        let mut state: serde_json::Value =
            serde_json::from_slice(&fs::read(&operation.state).unwrap()).unwrap();
        state["outcome_class"] = "changed".into();
        fs::write(&operation.state, serde_json::to_vec_pretty(&state).unwrap()).unwrap();
        let result = execute_post_commit_manifest_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256,
        );
        assert!(result.is_err());
        assert_eq!(fs::read(&live).unwrap(), database_before);
        assert!(existing_post_commit_manifest_receipts(directory.path())
            .unwrap()
            .is_empty());
    }

    #[test]
    fn exact_post_commit_activation_reverifies_under_the_write_lock() {
        let (directory, live, operation) =
            tauri::async_runtime::block_on(historical_post_commit_fixture());
        let evidence = read_migration_state_evidence(&operation).unwrap();
        let receipt = exact_post_commit_receipt(&operation, &evidence).unwrap();
        tauri::async_runtime::block_on(async {
            let mut connection = SqliteConnection::connect_with(
                &sqlx::sqlite::SqliteConnectOptions::new()
                    .filename(&live)
                    .foreign_keys(true),
            )
            .await
            .unwrap();
            sqlx::query(
                "CREATE INDEX unexpected_post_commit_drift ON experience_entries(created_at)",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            connection.close().await.unwrap();
        });
        let database_before = fs::read(&live).unwrap();

        let error =
            tauri::async_runtime::block_on(activate_exact_historical_frontend_lifecycle_writes(
                &live,
                &receipt,
                "2026-09-02T00:00:00Z",
            ))
            .unwrap_err();

        assert_eq!(error.code, "post_commit_schema_manifest_mismatch");
        assert_eq!(fs::read(&live).unwrap(), database_before);
        assert!(existing_post_commit_manifest_receipts(directory.path())
            .unwrap()
            .is_empty());
    }

    #[test]
    fn exact_prepared_state_is_eligible_and_recovery_preserves_database() {
        let (directory, live, operation) = fixture();
        let before = fs::read(&live).unwrap();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        assert!(inspection.eligible, "{:?}", inspection.reason);
        let result = execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256,
        )
        .unwrap();
        assert_eq!(fs::read(&live).unwrap(), before);
        assert!(result.restart_required);
        assert!(!operation.root.exists());
        assert!(!directory.path().join(WAL_FILENAME).exists());
        assert!(!directory.path().join(SHM_FILENAME).exists());
        assert!(recovery_receipt_summary(directory.path()).is_some());
    }

    #[test]
    fn cancellation_inspection_is_write_free() {
        let (directory, live, operation) = fixture();
        let before = [
            fs::read(&live).unwrap(),
            fs::read(&operation.state).unwrap(),
            fs::read(&operation.staging).unwrap(),
            fs::read(directory.path().join(WAL_FILENAME)).unwrap(),
            fs::read(directory.path().join(SHM_FILENAME)).unwrap(),
        ];
        let inspection = inspect_at(directory.path(), &live).unwrap();
        assert!(inspection.eligible, "{:?}", inspection.reason);
        let after = [
            fs::read(&live).unwrap(),
            fs::read(&operation.state).unwrap(),
            fs::read(&operation.staging).unwrap(),
            fs::read(directory.path().join(WAL_FILENAME)).unwrap(),
            fs::read(directory.path().join(SHM_FILENAME)).unwrap(),
        ];
        assert_eq!(before, after);
    }

    #[test]
    fn malformed_or_ambiguous_states_are_refused() {
        let cases: Vec<FixtureMutation> = vec![
            Box::new(|_, live, _| {
                let mut b = fs::read(live).unwrap();
                b[63] = 5;
                fs::write(live, b).unwrap();
            }),
            Box::new(|d, _, _| {
                fs::write(d.path().join(JOURNAL_FILENAME), b"x").unwrap();
            }),
            Box::new(|d, _, _| {
                fs::write(d.path().join(WAL_FILENAME), vec![0_u8; 33]).unwrap();
            }),
            Box::new(|d, _, _| {
                let p = d.path().join(SHM_FILENAME);
                let mut b = fs::read(&p).unwrap();
                b[16] = 1;
                fs::write(p, b).unwrap();
            }),
            Box::new(|_, _, o| {
                fs::write(&o.staging, b"x").unwrap();
            }),
            Box::new(|_, _, o| {
                fs::write(o.root.join("backup.db"), b"x").unwrap();
            }),
            Box::new(|_, _, o| {
                fs::write(o.root.join("unexpected"), b"x").unwrap();
            }),
        ];
        for mutate in cases {
            let (directory, live, operation) = fixture();
            mutate(&directory, &live, &operation);
            let result = inspect_at(directory.path(), &live);
            assert!(result.is_err() || !result.unwrap().eligible);
        }
    }

    #[test]
    fn thirty_case_synthetic_recovery_matrix_fails_closed() {
        let preflight_cases: Vec<(&str, FixtureMutation)> = vec![
            (
                "schema-v5",
                Box::new(|_, live, _| {
                    let mut b = fs::read(live).unwrap();
                    b[63] = 5;
                    fs::write(live, b).unwrap();
                }),
            ),
            (
                "invalid-header",
                Box::new(|_, live, _| {
                    let mut b = fs::read(live).unwrap();
                    b[0] = 0;
                    fs::write(live, b).unwrap();
                }),
            ),
            (
                "journal",
                Box::new(|d, _, _| {
                    fs::write(d.path().join(JOURNAL_FILENAME), b"x").unwrap();
                }),
            ),
            (
                "missing-wal",
                Box::new(|d, _, _| {
                    fs::remove_file(d.path().join(WAL_FILENAME)).unwrap();
                }),
            ),
            (
                "missing-shm",
                Box::new(|d, _, _| {
                    fs::remove_file(d.path().join(SHM_FILENAME)).unwrap();
                }),
            ),
            (
                "short-wal",
                Box::new(|d, _, _| {
                    fs::write(d.path().join(WAL_FILENAME), [1]).unwrap();
                }),
            ),
            (
                "framed-wal",
                Box::new(|d, _, _| {
                    fs::write(d.path().join(WAL_FILENAME), [0; 33]).unwrap();
                }),
            ),
            (
                "bad-wal-magic",
                Box::new(|d, _, _| {
                    let mut b = vec![0; 32];
                    b[8..12].copy_from_slice(&4096_u32.to_be_bytes());
                    fs::write(d.path().join(WAL_FILENAME), b).unwrap();
                }),
            ),
            (
                "bad-wal-page",
                Box::new(|d, _, _| {
                    let mut b = vec![0; 32];
                    b[0..4].copy_from_slice(&0x377f_0682_u32.to_be_bytes());
                    b[8..12].copy_from_slice(&3_u32.to_be_bytes());
                    fs::write(d.path().join(WAL_FILENAME), b).unwrap();
                }),
            ),
            (
                "empty-shm",
                Box::new(|d, _, _| {
                    fs::write(d.path().join(SHM_FILENAME), []).unwrap();
                }),
            ),
            (
                "short-shm",
                Box::new(|d, _, _| {
                    fs::write(d.path().join(SHM_FILENAME), vec![0; 32767]).unwrap();
                }),
            ),
            (
                "shm-copy-mismatch",
                Box::new(|d, _, _| {
                    let p = d.path().join(SHM_FILENAME);
                    let mut b = fs::read(&p).unwrap();
                    b[48] = 1;
                    fs::write(p, b).unwrap();
                }),
            ),
            (
                "shm-version",
                Box::new(|d, _, _| {
                    let p = d.path().join(SHM_FILENAME);
                    let mut b = fs::read(&p).unwrap();
                    b[0] = 0;
                    b[48] = 0;
                    fs::write(p, b).unwrap();
                }),
            ),
            (
                "shm-uninitialized",
                Box::new(|d, _, _| {
                    let p = d.path().join(SHM_FILENAME);
                    let mut b = fs::read(&p).unwrap();
                    b[12] = 0;
                    b[60] = 0;
                    fs::write(p, b).unwrap();
                }),
            ),
            (
                "shm-frame",
                Box::new(|d, _, _| {
                    let p = d.path().join(SHM_FILENAME);
                    let mut b = fs::read(&p).unwrap();
                    b[16] = 1;
                    b[64] = 1;
                    fs::write(p, b).unwrap();
                }),
            ),
            (
                "shm-page",
                Box::new(|d, _, _| {
                    let p = d.path().join(SHM_FILENAME);
                    let mut b = fs::read(&p).unwrap();
                    b[20] = 1;
                    b[68] = 1;
                    fs::write(p, b).unwrap();
                }),
            ),
            (
                "shm-backfill",
                Box::new(|d, _, _| {
                    let p = d.path().join(SHM_FILENAME);
                    let mut b = fs::read(&p).unwrap();
                    b[96] = 1;
                    fs::write(p, b).unwrap();
                }),
            ),
            (
                "staging-not-empty",
                Box::new(|_, _, o| {
                    fs::write(&o.staging, b"x").unwrap();
                }),
            ),
            (
                "backup-present",
                Box::new(|_, _, o| {
                    fs::write(&o.backup, b"x").unwrap();
                }),
            ),
            (
                "operation-extra-file",
                Box::new(|_, _, o| {
                    fs::write(o.root.join("extra"), b"x").unwrap();
                }),
            ),
            (
                "state-malformed",
                Box::new(|_, _, o| {
                    fs::write(&o.state, b"{}").unwrap();
                }),
            ),
            (
                "state-missing",
                Box::new(|_, _, o| {
                    fs::remove_file(&o.state).unwrap();
                }),
            ),
            (
                "second-operation",
                Box::new(|d, _, _| {
                    fs::create_dir(d.path().join("life-os-second.operation")).unwrap();
                }),
            ),
            (
                "database-missing",
                Box::new(|_, live, _| {
                    fs::remove_file(live).unwrap();
                }),
            ),
        ];
        for (name, mutate) in &preflight_cases {
            let (directory, live, operation) = fixture();
            mutate(&directory, &live, &operation);
            let result = inspect_at(directory.path(), &live);
            assert!(result.is_err() || !result.unwrap().eligible, "case {name}");
        }

        let mut execute_cases = 0;
        let (directory, live, _) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        assert!(execute_at(
            directory.path(),
            &live,
            &"0".repeat(64),
            &inspection.database.as_ref().unwrap().sha256
        )
        .is_err());
        execute_cases += 1;

        let (directory, live, _) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        assert!(execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &"0".repeat(64)
        )
        .is_err());
        execute_cases += 1;

        let (directory, live, operation) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        fs::write(&operation.staging, b"changed").unwrap();
        assert!(execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256
        )
        .is_err());
        execute_cases += 1;

        let (directory, live, _) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        let mut changed = fs::read(&live).unwrap();
        changed[100] = 1;
        fs::write(&live, changed).unwrap();
        assert!(execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256
        )
        .is_err());
        execute_cases += 1;

        let (directory, live, _) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        fs::write(
            receipt_path(
                directory.path(),
                inspection.operation_id.as_deref().unwrap(),
            ),
            b"conflict",
        )
        .unwrap();
        assert!(execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256
        )
        .is_err());
        execute_cases += 1;

        let (directory, live, _) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        #[cfg(windows)]
        use std::os::windows::fs::OpenOptionsExt;
        let mut active_options = OpenOptions::new();
        active_options.read(true).write(true);
        #[cfg(windows)]
        active_options.share_mode(0);
        let active = active_options.open(&live).unwrap();
        assert!(execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256
        )
        .is_err());
        drop(active);
        execute_cases += 1;

        assert_eq!(preflight_cases.len() + execute_cases, 30);
    }

    #[test]
    fn changed_claim_fails_closed_without_mutation() {
        let (directory, live, operation) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        fs::write(&operation.staging, b"changed").unwrap();
        let before = fs::read(&live).unwrap();
        assert!(execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.unwrap().sha256
        )
        .is_err());
        assert_eq!(fs::read(&live).unwrap(), before);
        assert!(operation.root.exists());
    }

    #[test]
    fn receipt_is_content_free_and_explicitly_deletable() {
        let (directory, live, _) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        let result = execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.unwrap().sha256,
        )
        .unwrap();
        let receipt =
            fs::read_to_string(directory.path().join(&result.receipt_relative_path)).unwrap();
        assert!(!receipt.contains("SQLite format"));
        assert!(!receipt.contains("experience"));
        let value: serde_json::Value = serde_json::from_str(&receipt).unwrap();
        let keys = value
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(
            keys,
            BTreeSet::from([
                "applicationVersion".to_string(),
                "classification".to_string(),
                "database".to_string(),
                "databaseSchemaVersion".to_string(),
                "operationId".to_string(),
                "quarantinedFiles".to_string(),
                "recoveredAt".to_string(),
                "schemaVersion".to_string(),
            ])
        );
        assert!(value.get("databaseIdentity").is_none());
        delete_receipt_at(directory.path(), &result.receipt_relative_path).unwrap();
        assert!(recovery_receipt_summary(directory.path()).is_none());
    }

    #[test]
    fn every_recovery_mutation_boundary_fails_without_database_change() {
        for point in [
            RecoveryFailurePoint::AfterQuarantineCreate,
            RecoveryFailurePoint::AfterWalMove,
            RecoveryFailurePoint::AfterShmMove,
            RecoveryFailurePoint::AfterStagingMove,
            RecoveryFailurePoint::AfterStateMove,
            RecoveryFailurePoint::AfterOperationCleanup,
            RecoveryFailurePoint::BeforeReceiptWrite,
            RecoveryFailurePoint::AfterReceiptWrite,
        ] {
            let (directory, live, _) = fixture();
            let before = fs::read(&live).unwrap();
            let inspection = inspect_at(directory.path(), &live).unwrap();
            let result = execute_at_with_failure(
                directory.path(),
                &live,
                inspection.claim_digest.as_deref().unwrap(),
                &inspection.database.as_ref().unwrap().sha256,
                point,
            );
            assert!(result.is_err(), "{point:?}");
            assert_eq!(fs::read(&live).unwrap(), before, "{point:?}");
        }
    }

    #[cfg(windows)]
    #[test]
    fn active_sqlite_lock_range_is_refused() {
        use std::os::windows::{fs::OpenOptionsExt, io::AsRawHandle};
        use windows_sys::{
            Win32::Storage::FileSystem::{LockFileEx, UnlockFileEx},
            Win32::System::IO::OVERLAPPED,
        };
        let (directory, live, _) = fixture();
        let inspection = inspect_at(directory.path(), &live).unwrap();
        let active = OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(7)
            .open(&live)
            .unwrap();
        let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
        overlapped.Anonymous.Anonymous.Offset = 0x4000_0002;
        assert_ne!(
            unsafe { LockFileEx(active.as_raw_handle().cast(), 0, 0, 1, 0, &mut overlapped) },
            0
        );
        let result = execute_at(
            directory.path(),
            &live,
            inspection.claim_digest.as_deref().unwrap(),
            &inspection.database.as_ref().unwrap().sha256,
        );
        assert_eq!(
            result.unwrap_err(),
            "prepared_recovery_database_activity_not_exclusive"
        );
        unsafe {
            UnlockFileEx(active.as_raw_handle().cast(), 0, 1, 0, &mut overlapped);
        }
    }
}
