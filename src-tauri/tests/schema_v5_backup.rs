use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{raw_sql, Connection, Executor, Row, SqliteConnection};
use std::fs;
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

async fn new_fixture() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let directory = tempfile::tempdir().expect("temporary directory must exist");
    let source = directory.path().join("life-os-v4.db");
    let destination = directory.path().join("life-os-before-v5.db");
    create_v4_fixture(&source).await;
    (directory, source, destination)
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
