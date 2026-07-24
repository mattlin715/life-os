use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{raw_sql, Connection, Executor, Row, SqliteConnection};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const V4_FIXTURE: &str = include_str!("fixtures/schema_v5/v4.sql");
const APPLICATION_VERSION: &str = "0.2.0";
const CREATED_AT: &str = "2026-07-19T00:00:00.000Z";
const EXPIRES_AT: &str = "2026-08-18T00:00:00.000Z";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum FailureInjection {
    #[default]
    None,
    AfterVacuum,
    CorruptBackup,
    SourceManifestMismatch,
    ForeignKeyViolation,
    IntegrityFailure,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct BackupManifest {
    path_relative_filename: String,
    backup_id: String,
    database_sha256: String,
    source_manifest_digest: String,
    schema_version: i64,
    application_version: String,
    created_at: String,
    expires_at: String,
    verification_result: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CanonicalV4Record {
    table: &'static str,
    values: Vec<Option<String>>,
}

#[derive(Clone, Debug)]
struct RestoreExpectations {
    database_sha256: String,
    source_manifest_digest: String,
    records: Vec<CanonicalV4Record>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum RestoreFailureInjection {
    #[default]
    None,
    PreReplacementDigestMismatch,
    PermissionDenied,
    ReplacementFailure,
    InterruptedAfterReplacement,
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

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn frame_bytes(digest: &mut Sha256, bytes: &[u8]) {
    digest.update((bytes.len() as u64).to_be_bytes());
    digest.update(bytes);
}

fn frame_optional_text(digest: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            digest.update([1]);
            frame_bytes(digest, value.as_bytes());
        }
        None => digest.update([0]),
    }
}

async fn connect(
    path: &Path,
    create_if_missing: bool,
    read_only: bool,
) -> Result<SqliteConnection, String> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(create_if_missing)
        .read_only(read_only)
        .foreign_keys(true);
    SqliteConnection::connect_with(&options)
        .await
        .map_err(|error| format!("database_open_failed:{error}"))
}

async fn create_v4_fixture(path: &Path) {
    let mut connection = connect(path, true, false)
        .await
        .expect("disposable fixture must open");
    raw_sql(V4_FIXTURE)
        .execute(&mut connection)
        .await
        .expect("fixed v4 fixture must execute");
    connection.close().await.expect("fixture must close");
}

async fn create_version_fixture(path: &Path, version: i64) {
    let mut connection = connect(path, true, false)
        .await
        .expect("version fixture must open");
    connection
        .execute(format!("PRAGMA user_version = {version}").as_str())
        .await
        .expect("version fixture must set user_version");
    connection.close().await.expect("fixture must close");
}

async fn v4_record_snapshot(
    connection: &mut SqliteConnection,
) -> Result<Vec<CanonicalV4Record>, String> {
    let mut records = Vec::new();
    for spec in TABLE_MANIFESTS {
        let rows = sqlx::query(spec.query)
            .fetch_all(&mut *connection)
            .await
            .map_err(|error| format!("record_snapshot_query_failed:{}:{error}", spec.name))?;
        for row in rows {
            let mut values = Vec::with_capacity(spec.field_count);
            for index in 0..spec.field_count {
                values.push(row.try_get(index).map_err(|error| {
                    format!(
                        "record_snapshot_decode_failed:{}:{index}:{error}",
                        spec.name
                    )
                })?);
            }
            records.push(CanonicalV4Record {
                table: spec.name,
                values,
            });
        }
    }
    Ok(records)
}

async fn v4_source_manifest(connection: &mut SqliteConnection) -> Result<String, String> {
    let mut digest = Sha256::new();
    digest.update(b"life-os/v4-source-manifest-v1\0");

    for spec in TABLE_MANIFESTS {
        frame_bytes(&mut digest, spec.name.as_bytes());
        let rows = sqlx::query(spec.query)
            .fetch_all(&mut *connection)
            .await
            .map_err(|error| format!("source_manifest_query_failed:{}:{error}", spec.name))?;
        digest.update((rows.len() as u64).to_be_bytes());
        for row in rows {
            for index in 0..spec.field_count {
                let value: Option<String> = row.try_get(index).map_err(|error| {
                    format!(
                        "source_manifest_decode_failed:{}:{index}:{error}",
                        spec.name
                    )
                })?;
                frame_optional_text(&mut digest, value.as_deref());
            }
        }
    }

    frame_bytes(&mut digest, b"persisted_artifacts.count_by_kind");
    let kind_counts = sqlx::query(
        "SELECT artifact_kind, COUNT(*) FROM persisted_artifacts GROUP BY artifact_kind ORDER BY artifact_kind",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| format!("source_manifest_kind_count_failed:{error}"))?;
    digest.update((kind_counts.len() as u64).to_be_bytes());
    for row in kind_counts {
        let kind: String = row
            .try_get(0)
            .map_err(|error| format!("source_manifest_kind_decode_failed:{error}"))?;
        let count: i64 = row
            .try_get(1)
            .map_err(|error| format!("source_manifest_count_decode_failed:{error}"))?;
        frame_bytes(&mut digest, kind.as_bytes());
        digest.update(count.to_be_bytes());
    }

    Ok(format!("{:x}", digest.finalize()))
}

async fn require_v4(connection: &mut SqliteConnection, label: &str) -> Result<(), String> {
    let version: i64 = sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| format!("{label}_schema_version_read_failed:{error}"))?;
    if version != 4 {
        return Err(format!("{label}_schema_version_mismatch:{version}"));
    }
    Ok(())
}

async fn require_foreign_keys(connection: &mut SqliteConnection) -> Result<(), String> {
    let rows = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| format!("backup_foreign_key_check_failed:{error}"))?;
    if !rows.is_empty() {
        return Err(format!("backup_foreign_key_check_failed:{}", rows.len()));
    }
    Ok(())
}

async fn require_integrity(connection: &mut SqliteConnection) -> Result<(), String> {
    let result: String = sqlx::query_scalar("PRAGMA integrity_check")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| format!("backup_integrity_check_failed:{error}"))?;
    if result != "ok" {
        return Err(format!("backup_integrity_check_failed:{result}"));
    }
    Ok(())
}

fn backup_id(source_manifest: &str, database_digest: &str, created_at: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"life-os/pre-v5-backup-v1\0");
    frame_bytes(&mut digest, source_manifest.as_bytes());
    frame_bytes(&mut digest, database_digest.as_bytes());
    frame_bytes(&mut digest, created_at.as_bytes());
    format!("v4b_{:x}", digest.finalize())
}

fn remove_incomplete_destination(destination: &Path) {
    if destination.is_file() {
        let _ = fs::remove_file(destination);
    }
}

async fn inject_foreign_key_violation(destination: &Path) -> Result<(), String> {
    let options = SqliteConnectOptions::new()
        .filename(destination)
        .create_if_missing(false)
        .foreign_keys(false);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|error| format!("injection_open_failed:{error}"))?;
    sqlx::query("INSERT INTO historical_artifact_dependencies (historical_artifact_id, source_entry_id, source_artifact_id, source_revision) VALUES ('missing-question', 'missing-source', NULL, 'missing-revision')")
        .execute(&mut connection)
        .await
        .map_err(|error| format!("injection_foreign_key_failed:{error}"))?;
    connection.close().await.map_err(|error| error.to_string())
}

async fn create_verified_backup_inner(
    source: &Path,
    destination: &Path,
    injection: FailureInjection,
) -> Result<BackupManifest, String> {
    if !source.is_file() {
        return Err("source_database_missing".into());
    }
    let source_before = fs::read(source).map_err(|error| format!("source_read_failed:{error}"))?;
    let mut source_connection = connect(source, false, false).await?;
    require_v4(&mut source_connection, "source").await?;
    let source_manifest = v4_source_manifest(&mut source_connection).await?;

    sqlx::query("VACUUM INTO ?")
        .bind(destination.to_string_lossy().to_string())
        .execute(&mut source_connection)
        .await
        .map_err(|error| format!("backup_vacuum_failed:{error}"))?;
    source_connection
        .close()
        .await
        .map_err(|error| format!("source_close_failed:{error}"))?;

    if injection == FailureInjection::AfterVacuum {
        return Err("injected_failure_after_vacuum".into());
    }
    if injection == FailureInjection::CorruptBackup {
        fs::write(destination, b"not a sqlite database")
            .map_err(|error| format!("corruption_injection_failed:{error}"))?;
    }
    if injection == FailureInjection::ForeignKeyViolation {
        inject_foreign_key_violation(destination).await?;
    }

    let source_after = fs::read(source).map_err(|error| format!("source_read_failed:{error}"))?;
    if source_before != source_after {
        return Err("source_database_changed".into());
    }

    let mut backup_connection = connect(destination, false, true)
        .await
        .map_err(|error| format!("backup_open_failed:{error}"))?;
    let verification = async {
        require_v4(&mut backup_connection, "backup").await?;
        require_foreign_keys(&mut backup_connection).await?;
        require_integrity(&mut backup_connection).await?;
        if injection == FailureInjection::IntegrityFailure {
            return Err("backup_integrity_check_failed:injected".into());
        }
        v4_source_manifest(&mut backup_connection).await
    }
    .await;
    backup_connection
        .close()
        .await
        .map_err(|error| format!("backup_close_failed:{error}"))?;
    let backup_manifest = verification?;

    let expected_manifest = if injection == FailureInjection::SourceManifestMismatch {
        "0".repeat(64)
    } else {
        source_manifest.clone()
    };
    if backup_manifest != expected_manifest {
        return Err("backup_source_manifest_mismatch".into());
    }

    let backup_bytes =
        fs::read(destination).map_err(|error| format!("backup_digest_read_failed:{error}"))?;
    let database_sha256 = sha256_hex(&backup_bytes);
    let relative_filename = destination
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "backup_relative_filename_invalid".to_string())?
        .to_owned();

    Ok(BackupManifest {
        path_relative_filename: relative_filename,
        backup_id: backup_id(&source_manifest, &database_sha256, CREATED_AT),
        database_sha256,
        source_manifest_digest: source_manifest,
        schema_version: 4,
        application_version: APPLICATION_VERSION.into(),
        created_at: CREATED_AT.into(),
        expires_at: EXPIRES_AT.into(),
        verification_result: "verified".into(),
    })
}

async fn create_verified_backup(
    source: &Path,
    destination: &Path,
    injection: FailureInjection,
) -> Result<BackupManifest, String> {
    if destination.exists() {
        return Err("backup_destination_exists".into());
    }
    let result = create_verified_backup_inner(source, destination, injection).await;
    if result.is_err() {
        remove_incomplete_destination(destination);
    }
    result
}

async fn restore_expectations(backup: &Path, manifest: &BackupManifest) -> RestoreExpectations {
    let mut connection = connect(backup, false, true)
        .await
        .expect("verified backup must open");
    let records = v4_record_snapshot(&mut connection)
        .await
        .expect("verified backup records must decode");
    connection
        .close()
        .await
        .expect("verified backup must close");
    RestoreExpectations {
        database_sha256: manifest.database_sha256.clone(),
        source_manifest_digest: manifest.source_manifest_digest.clone(),
        records,
    }
}

async fn verify_restore_candidate(
    path: &Path,
    label: &str,
    expectations: &RestoreExpectations,
) -> Result<(), String> {
    let mut connection = connect(path, false, true)
        .await
        .map_err(|error| format!("{label}_open_failed:{error}"))?;
    let verification = async {
        require_v4(&mut connection, label).await?;
        require_foreign_keys(&mut connection).await?;
        require_integrity(&mut connection).await?;
        let manifest = v4_source_manifest(&mut connection).await?;
        if manifest != expectations.source_manifest_digest {
            return Err(format!("{label}_source_manifest_mismatch"));
        }
        let records = v4_record_snapshot(&mut connection).await?;
        if records != expectations.records {
            return Err(format!("{label}_record_snapshot_mismatch"));
        }
        Ok(())
    }
    .await;
    connection
        .close()
        .await
        .map_err(|error| format!("{label}_close_failed:{error}"))?;
    verification
}

async fn restore_verified_backup_inner(
    backup: &Path,
    live: &Path,
    staging: &Path,
    expectations: &RestoreExpectations,
    injection: RestoreFailureInjection,
) -> Result<(), String> {
    let backup_bytes =
        fs::read(backup).map_err(|error| format!("restore_backup_read_failed:{error}"))?;
    if sha256_hex(&backup_bytes) != expectations.database_sha256 {
        return Err("restore_backup_digest_mismatch".into());
    }
    verify_restore_candidate(backup, "restore_backup", expectations).await?;

    let mut staging_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(staging)
        .map_err(|error| format!("restore_staging_create_failed:{error}"))?;
    staging_file
        .write_all(&backup_bytes)
        .map_err(|error| format!("restore_staging_write_failed:{error}"))?;
    staging_file
        .sync_all()
        .map_err(|error| format!("restore_staging_sync_failed:{error}"))?;
    drop(staging_file);
    verify_restore_candidate(staging, "restore_staging", expectations).await?;

    let pre_replacement_bytes =
        fs::read(backup).map_err(|error| format!("restore_pre_replacement_read_failed:{error}"))?;
    let pre_replacement_expected =
        if injection == RestoreFailureInjection::PreReplacementDigestMismatch {
            "0".repeat(64)
        } else {
            expectations.database_sha256.clone()
        };
    if sha256_hex(&pre_replacement_bytes) != pre_replacement_expected {
        return Err("restore_pre_replacement_digest_mismatch".into());
    }
    verify_restore_candidate(backup, "restore_pre_replacement_backup", expectations).await?;

    match injection {
        RestoreFailureInjection::PermissionDenied => {
            return Err("restore_permission_denied:injected".into());
        }
        RestoreFailureInjection::ReplacementFailure => {
            return Err("restore_replacement_failed:injected".into());
        }
        _ => {}
    }

    // This is a fixture-only logical replacement simulation. It deliberately
    // does not claim production filesystem atomicity or crash durability.
    let staged_bytes =
        fs::read(staging).map_err(|error| format!("restore_staging_read_failed:{error}"))?;
    fs::write(live, staged_bytes)
        .map_err(|error| format!("restore_replacement_write_failed:{error}"))?;

    if injection == RestoreFailureInjection::InterruptedAfterReplacement {
        return Err("restore_interrupted_after_replacement:injected".into());
    }

    verify_restore_candidate(live, "restored_live", expectations).await?;
    fs::remove_file(staging).map_err(|error| format!("restore_staging_cleanup_failed:{error}"))?;
    Ok(())
}

async fn restore_verified_backup(
    backup: &Path,
    live: &Path,
    staging: &Path,
    expectations: &RestoreExpectations,
    injection: RestoreFailureInjection,
) -> Result<(), String> {
    if backup == live || backup == staging || live == staging {
        return Err("restore_paths_must_be_distinct".into());
    }
    if !backup.is_file() {
        return Err("restore_backup_missing".into());
    }
    if !live.is_file() {
        return Err("restore_live_fixture_missing".into());
    }
    if staging.exists() {
        return Err("restore_staging_destination_exists".into());
    }

    let live_before =
        fs::read(live).map_err(|error| format!("restore_live_read_failed:{error}"))?;
    let backup_before =
        fs::read(backup).map_err(|error| format!("restore_backup_read_failed:{error}"))?;
    let result =
        restore_verified_backup_inner(backup, live, staging, expectations, injection).await;

    if result.is_ok() {
        if fs::read(backup).map_err(|error| format!("restore_backup_read_failed:{error}"))?
            != backup_before
        {
            fs::write(live, &live_before)
                .map_err(|error| format!("restore_live_rollback_failed:{error}"))?;
            return Err("restore_backup_changed".into());
        }
        return Ok(());
    }

    let original_error = result.unwrap_err();
    if fs::read(live).map_err(|error| format!("restore_live_read_failed:{error}"))? != live_before {
        fs::write(live, &live_before)
            .map_err(|error| format!("restore_live_rollback_failed:{error}"))?;
    }
    if staging.exists() {
        fs::remove_file(staging)
            .map_err(|error| format!("restore_staging_cleanup_failed:{error}"))?;
    }
    if fs::read(live).map_err(|error| format!("restore_live_read_failed:{error}"))? != live_before {
        return Err(format!(
            "restore_live_not_unchanged_after_failure:{original_error}"
        ));
    }
    if fs::read(backup).map_err(|error| format!("restore_backup_read_failed:{error}"))?
        != backup_before
    {
        return Err(format!("restore_backup_changed:{original_error}"));
    }
    Err(original_error)
}

async fn new_fixture() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let directory = tempfile::tempdir().expect("temporary directory must exist");
    let source = directory.path().join("life-os-v4.db");
    let destination = directory.path().join("life-os-before-v5.db");
    create_v4_fixture(&source).await;
    (directory, source, destination)
}

async fn new_restore_fixture() -> (
    tempfile::TempDir,
    PathBuf,
    PathBuf,
    PathBuf,
    BackupManifest,
    RestoreExpectations,
) {
    let (directory, source, backup) = new_fixture().await;
    let manifest = create_verified_backup(&source, &backup, FailureInjection::None)
        .await
        .expect("restore fixture backup must verify");
    let expectations = restore_expectations(&backup, &manifest).await;
    let live = directory.path().join("life-os-live.db");
    let staging = directory.path().join("life-os-restore-staging.db");
    create_v4_fixture(&live).await;
    let mut live_connection = connect(&live, false, false)
        .await
        .expect("disposable live fixture must open");
    sqlx::query("UPDATE experience_entries SET content = ? WHERE id = ?")
        .bind("live content before restore")
        .bind("fixture-v4-current")
        .execute(&mut live_connection)
        .await
        .expect("disposable live fixture must differ from backup");
    live_connection
        .close()
        .await
        .expect("disposable live fixture must close");
    (directory, backup, live, staging, manifest, expectations)
}

#[tokio::test]
async fn verified_vacuum_backup_has_content_free_manifest_and_preserves_source() {
    let (_directory, source, destination) = new_fixture().await;
    let source_before = fs::read(&source).unwrap();

    let manifest = create_verified_backup(&source, &destination, FailureInjection::None)
        .await
        .expect("valid v4 fixture must produce a verified backup");

    assert_eq!(fs::read(&source).unwrap(), source_before);
    assert!(destination.is_file());
    assert_eq!(manifest.path_relative_filename, "life-os-before-v5.db");
    assert_eq!(manifest.schema_version, 4);
    assert_eq!(manifest.application_version, APPLICATION_VERSION);
    assert_eq!(manifest.verification_result, "verified");
    assert_eq!(manifest.database_sha256.len(), 64);
    assert_eq!(manifest.source_manifest_digest.len(), 64);
    assert!(manifest.backup_id.starts_with("v4b_"));
    assert_eq!(manifest.backup_id.len(), 68);
    assert_eq!(
        manifest.database_sha256,
        sha256_hex(&fs::read(&destination).unwrap())
    );

    let mut source_connection = connect(&source, false, true).await.unwrap();
    assert_eq!(
        manifest.source_manifest_digest,
        v4_source_manifest(&mut source_connection).await.unwrap()
    );
    source_connection.close().await.unwrap();

    let json = serde_json::to_string(&manifest).unwrap();
    for forbidden in [
        "current experience",
        "historical experience",
        "observed",
        "What should I notice?",
        "packetSnapshot",
        "payload",
        "credential",
        "providerError",
    ] {
        assert!(!json.contains(forbidden), "manifest leaked {forbidden:?}");
    }
    let value: Value = serde_json::from_str(&json).unwrap();
    let keys = value
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(
        keys,
        vec![
            "applicationVersion",
            "backupId",
            "createdAt",
            "databaseSha256",
            "expiresAt",
            "pathRelativeFilename",
            "schemaVersion",
            "sourceManifestDigest",
            "verificationResult",
        ]
    );
}

#[tokio::test]
async fn destination_conflict_fails_without_overwriting() {
    let (_directory, source, destination) = new_fixture().await;
    fs::write(&destination, b"preserve-existing").unwrap();

    let error = create_verified_backup(&source, &destination, FailureInjection::None)
        .await
        .unwrap_err();

    assert_eq!(error, "backup_destination_exists");
    assert_eq!(fs::read(&destination).unwrap(), b"preserve-existing");
}

#[tokio::test]
async fn missing_and_malformed_sources_fail_without_destination() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("missing.db");
    let malformed = directory.path().join("malformed.db");
    let destination = directory.path().join("backup.db");

    assert_eq!(
        create_verified_backup(&missing, &destination, FailureInjection::None)
            .await
            .unwrap_err(),
        "source_database_missing"
    );
    assert!(!destination.exists());

    fs::write(&malformed, b"not a sqlite database").unwrap();
    let error = create_verified_backup(&malformed, &destination, FailureInjection::None)
        .await
        .unwrap_err();
    assert!(
        error.contains("schema_version_read_failed") || error.contains("database_open_failed"),
        "unexpected malformed-source error: {error}"
    );
    assert!(!destination.exists());
}

#[tokio::test]
async fn schema_version_mismatch_fails_closed_for_v3_and_v5() {
    for version in [3, 5] {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join(format!("v{version}.db"));
        let destination = directory.path().join("backup.db");
        create_version_fixture(&source, version).await;

        let error = create_verified_backup(&source, &destination, FailureInjection::None)
            .await
            .unwrap_err();
        assert_eq!(error, format!("source_schema_version_mismatch:{version}"));
        assert!(!destination.exists());
    }
}

#[tokio::test]
async fn injected_post_vacuum_failure_removes_incomplete_destination() {
    let (_directory, source, destination) = new_fixture().await;

    let error = create_verified_backup(&source, &destination, FailureInjection::AfterVacuum)
        .await
        .unwrap_err();

    assert_eq!(error, "injected_failure_after_vacuum");
    assert!(!destination.exists());
}

#[tokio::test]
async fn corrupted_backup_and_manifest_mismatch_fail_closed_and_cleanup() {
    for injection in [
        FailureInjection::CorruptBackup,
        FailureInjection::SourceManifestMismatch,
    ] {
        let (_directory, source, destination) = new_fixture().await;

        let error = create_verified_backup(&source, &destination, injection)
            .await
            .unwrap_err();

        assert!(
            error.contains("backup_open_failed")
                || error.contains("backup_schema_version_read_failed")
                || error == "backup_source_manifest_mismatch",
            "unexpected corruption error: {error}"
        );
        assert!(!destination.exists());
    }
}

#[tokio::test]
async fn foreign_key_and_integrity_failures_never_produce_verified_backup() {
    for injection in [
        FailureInjection::ForeignKeyViolation,
        FailureInjection::IntegrityFailure,
    ] {
        let (_directory, source, destination) = new_fixture().await;

        let error = create_verified_backup(&source, &destination, injection)
            .await
            .unwrap_err();

        assert!(
            error.starts_with("backup_foreign_key_check_failed")
                || error == "backup_integrity_check_failed:injected",
            "unexpected verification error: {error}"
        );
        assert!(!destination.exists());
    }
}

#[tokio::test]
async fn source_manifest_changes_when_any_governed_v4_row_changes() {
    let (_directory, source, _destination) = new_fixture().await;
    let mut connection = connect(&source, false, false).await.unwrap();
    let before = v4_source_manifest(&mut connection).await.unwrap();
    sqlx::query("UPDATE historical_question_artifacts SET packet_snapshot = '{\"changed\":true}' WHERE id = 'fixture-v4-question'")
        .execute(&mut connection)
        .await
        .unwrap();
    let after = v4_source_manifest(&mut connection).await.unwrap();

    assert_ne!(before, after);
}

#[tokio::test]
async fn verified_restore_simulation_replaces_only_the_disposable_live_fixture() {
    let (_directory, backup, live, staging, manifest, expectations) = new_restore_fixture().await;
    let backup_before = fs::read(&backup).unwrap();
    let live_before = fs::read(&live).unwrap();
    assert_ne!(live_before, backup_before);

    restore_verified_backup(
        &backup,
        &live,
        &staging,
        &expectations,
        RestoreFailureInjection::None,
    )
    .await
    .expect("verified fixture restore must succeed");

    assert_eq!(fs::read(&backup).unwrap(), backup_before);
    assert_eq!(fs::read(&live).unwrap(), backup_before);
    assert!(!staging.exists());
    assert_eq!(
        sha256_hex(&fs::read(&live).unwrap()),
        manifest.database_sha256
    );
    let mut restored = connect(&live, false, true).await.unwrap();
    assert_eq!(
        v4_source_manifest(&mut restored).await.unwrap(),
        manifest.source_manifest_digest
    );
    assert_eq!(
        v4_record_snapshot(&mut restored).await.unwrap(),
        expectations.records
    );
    require_foreign_keys(&mut restored).await.unwrap();
    require_integrity(&mut restored).await.unwrap();
    restored.close().await.unwrap();
}

#[tokio::test]
async fn restore_expectation_mismatches_leave_live_and_backup_byte_identical() {
    let (_directory, backup, live, staging, _manifest, expectations) = new_restore_fixture().await;
    let backup_before = fs::read(&backup).unwrap();
    let live_before = fs::read(&live).unwrap();

    let mut digest_mismatch = expectations.clone();
    digest_mismatch.database_sha256 = "0".repeat(64);
    assert_eq!(
        restore_verified_backup(
            &backup,
            &live,
            &staging,
            &digest_mismatch,
            RestoreFailureInjection::None,
        )
        .await
        .unwrap_err(),
        "restore_backup_digest_mismatch"
    );

    let mut manifest_mismatch = expectations.clone();
    manifest_mismatch.source_manifest_digest = "0".repeat(64);
    assert_eq!(
        restore_verified_backup(
            &backup,
            &live,
            &staging,
            &manifest_mismatch,
            RestoreFailureInjection::None,
        )
        .await
        .unwrap_err(),
        "restore_backup_source_manifest_mismatch"
    );

    let mut record_mismatch = expectations.clone();
    record_mismatch.records[0].values[0] = Some("unexpected-record".into());
    assert_eq!(
        restore_verified_backup(
            &backup,
            &live,
            &staging,
            &record_mismatch,
            RestoreFailureInjection::None,
        )
        .await
        .unwrap_err(),
        "restore_backup_record_snapshot_mismatch"
    );

    assert_eq!(fs::read(&backup).unwrap(), backup_before);
    assert_eq!(fs::read(&live).unwrap(), live_before);
    assert!(!staging.exists());
}

#[tokio::test]
async fn malformed_corrupt_and_wrong_version_backups_fail_before_replacement() {
    for version in [None, Some(3), Some(5)] {
        let directory = tempfile::tempdir().unwrap();
        let backup = directory.path().join("candidate-backup.db");
        let live = directory.path().join("live.db");
        let staging = directory.path().join("staging.db");
        create_v4_fixture(&live).await;
        if let Some(version) = version {
            create_version_fixture(&backup, version).await;
        } else {
            fs::write(&backup, b"corrupt-not-sqlite").unwrap();
        }
        let live_before = fs::read(&live).unwrap();
        let backup_before = fs::read(&backup).unwrap();
        let expectations = RestoreExpectations {
            database_sha256: sha256_hex(&backup_before),
            source_manifest_digest: "0".repeat(64),
            records: Vec::new(),
        };

        let error = restore_verified_backup(
            &backup,
            &live,
            &staging,
            &expectations,
            RestoreFailureInjection::None,
        )
        .await
        .unwrap_err();

        if let Some(version) = version {
            assert_eq!(
                error,
                format!("restore_backup_schema_version_mismatch:{version}")
            );
        } else {
            assert!(
                error.contains("restore_backup_schema_version_read_failed")
                    || error.contains("restore_backup_open_failed"),
                "unexpected corrupt-backup error: {error}"
            );
        }
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert_eq!(fs::read(&backup).unwrap(), backup_before);
        assert!(!staging.exists());
    }
}

#[tokio::test]
async fn conflicts_and_injected_replacement_failures_clean_only_owned_staging() {
    let (directory, backup, live, staging, _manifest, expectations) = new_restore_fixture().await;
    let live_before = fs::read(&live).unwrap();
    let backup_before = fs::read(&backup).unwrap();
    let unrelated = directory.path().join("unrelated.db");
    fs::write(&unrelated, b"preserve-unrelated").unwrap();

    fs::write(&staging, b"preserve-existing-staging").unwrap();
    assert_eq!(
        restore_verified_backup(
            &backup,
            &live,
            &staging,
            &expectations,
            RestoreFailureInjection::None,
        )
        .await
        .unwrap_err(),
        "restore_staging_destination_exists"
    );
    assert_eq!(fs::read(&staging).unwrap(), b"preserve-existing-staging");
    fs::remove_file(&staging).unwrap();

    assert_eq!(
        restore_verified_backup(
            &backup,
            &live,
            &live,
            &expectations,
            RestoreFailureInjection::None,
        )
        .await
        .unwrap_err(),
        "restore_paths_must_be_distinct"
    );

    for injection in [
        RestoreFailureInjection::PreReplacementDigestMismatch,
        RestoreFailureInjection::PermissionDenied,
        RestoreFailureInjection::ReplacementFailure,
        RestoreFailureInjection::InterruptedAfterReplacement,
    ] {
        let error = restore_verified_backup(&backup, &live, &staging, &expectations, injection)
            .await
            .unwrap_err();
        assert!(
            error.starts_with("restore_pre_replacement_digest_mismatch")
                || error.starts_with("restore_permission_denied")
                || error.starts_with("restore_replacement_failed")
                || error.starts_with("restore_interrupted_after_replacement"),
            "unexpected injected restore error: {error}"
        );
        assert_eq!(fs::read(&backup).unwrap(), backup_before);
        assert_eq!(fs::read(&live).unwrap(), live_before);
        assert!(!staging.exists());
        assert_eq!(fs::read(&unrelated).unwrap(), b"preserve-unrelated");
    }
}
