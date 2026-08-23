use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Connection, Executor, SqliteConnection};
use std::{
    collections::HashSet,
    fs::File,
    path::Path,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

use crate::filesystem_safety::{
    inspect_readiness_filesystem, ReadinessFilesystemFailure, ReadinessOperationEvidence,
};

const SCHEMA_VERSION: i64 = 4;
const NEWER_SCHEMA_ERROR: &str = "database_schema_newer_than_supported";

fn readiness_supported_schema_version() -> i64 {
    if cfg!(any(
        feature = "desktop-schema-v5",
        feature = "founder-schema-v5"
    )) {
        5
    } else {
        SCHEMA_VERSION
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStartupState {
    state: String,
    reason: Option<String>,
    detected_schema_version: Option<i64>,
    supported_schema_version: i64,
    initialization_required: bool,
}

impl DatabaseStartupState {
    fn ready(detected_schema_version: Option<i64>) -> Self {
        Self {
            state: "ready".into(),
            reason: None,
            detected_schema_version,
            supported_schema_version: SCHEMA_VERSION,
            initialization_required: detected_schema_version.unwrap_or(0) < SCHEMA_VERSION,
        }
    }

    fn newer_schema(detected_schema_version: i64) -> Self {
        Self {
            state: "blocked".into(),
            reason: Some("newer_schema".into()),
            detected_schema_version: Some(detected_schema_version),
            supported_schema_version: SCHEMA_VERSION,
            initialization_required: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseReadinessResult {
    classification: String,
    database_exists: Option<bool>,
    detected_schema_version: Option<i64>,
    supported_schema_version: i64,
    wal_present: Option<bool>,
    shm_present: Option<bool>,
    rollback_journal_present: Option<bool>,
    quiescence: String,
    operation_evidence: String,
    schema_v5_available: bool,
    inspected_at_unix_ms: u64,
}

impl DatabaseReadinessResult {
    fn bounded(classification: &str) -> Self {
        let supported_schema_version = readiness_supported_schema_version();
        Self {
            classification: classification.into(),
            database_exists: None,
            detected_schema_version: None,
            supported_schema_version,
            wal_present: None,
            shm_present: None,
            rollback_journal_present: None,
            quiescence: "not_proven".into(),
            operation_evidence: "unknown".into(),
            schema_v5_available: supported_schema_version >= 5,
            inspected_at_unix_ms: inspection_time_ms(),
        }
    }
}

fn inspection_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

#[derive(Debug, Clone)]
struct ExpectedExperienceRevision {
    id: String,
    updated_at: String,
}

#[derive(Debug, Clone)]
struct ExpectedArtifactRevision {
    id: String,
    source_entry_id: String,
    updated_at: String,
    artifact_kind: String,
}

#[derive(Debug, Clone)]
struct ExpectedHistoricalProvenance {
    consent_id: String,
    transmission_id: String,
    packet_digest: String,
    provider: String,
    model: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceRecord {
    id: String,
    content: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedArtifactBundleRecord {
    evidence: Vec<Value>,
    reflections: Vec<Value>,
    patterns: Vec<Value>,
    recovery_turns: Vec<Value>,
}

#[derive(Debug, Clone)]
struct PersistedArtifactRecord {
    id: String,
    artifact_kind: &'static str,
    payload: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalConsentRecord {
    id: String,
    packet_digest: String,
    task: String,
    purpose: String,
    provider: String,
    model: String,
    source_revisions: Vec<HistoricalConsentSourceRevisionRecord>,
    state: String,
    created_at: String,
    expires_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalConsentSourceRevisionRecord {
    source_experience_id: String,
    revision: String,
    artifact_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalTransmissionRecord {
    id: String,
    consent_id: String,
    packet_digest: String,
    provider: String,
    model: String,
    outcome: String,
    created_at: String,
    expires_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoricalCurrentExperienceRecord {
    id: String,
    revision: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoricalDestinationRecord {
    provider: String,
    model: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoricalPacketConsentRecord {
    reference: String,
    scope: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoricalIncludedItemRecord {
    item_type: String,
    source_experience_id: String,
    artifact_id: Option<String>,
    revision: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoricalPacketPersistenceRecord {
    packet_digest: String,
    current_experience: HistoricalCurrentExperienceRecord,
    task: String,
    purpose: String,
    destination: HistoricalDestinationRecord,
    included_items: Vec<HistoricalIncludedItemRecord>,
    consent: HistoricalPacketConsentRecord,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalQuestionArtifactRecord {
    id: String,
    current_experience_id: String,
    questions: Vec<HistoricalReflectionQuestionRecord>,
    packet: Value,
    consent_id: String,
    transmission_id: String,
    generated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalReflectionQuestionRecord {
    id: String,
    text: String,
    source_experience_ids: Vec<String>,
}

async fn connect(path: &Path) -> Result<SqliteConnection, String> {
    let options =
        sqlx::sqlite::SqliteConnectOptions::from_str(&format!("sqlite://{}", path.display()))
            .map_err(|e| e.to_string())?
            .create_if_missing(true)
            .foreign_keys(true);
    SqliteConnection::connect_with(&options)
        .await
        .map_err(|e| e.to_string())
}

async fn read_schema_version(conn: &mut SqliteConnection) -> Result<i64, String> {
    sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| e.to_string())
}

fn newer_schema_error(version: i64) -> String {
    format!("{NEWER_SCHEMA_ERROR}: detected={version} supported={SCHEMA_VERSION}")
}

async fn ensure_supported_schema(conn: &mut SqliteConnection) -> Result<i64, String> {
    let version = read_schema_version(conn).await?;
    if version > SCHEMA_VERSION {
        return Err(newer_schema_error(version));
    }
    Ok(version)
}

async fn inspect_database_path(path: &Path) -> Result<DatabaseStartupState, String> {
    if !path.exists() {
        return Ok(DatabaseStartupState::ready(None));
    }

    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .read_only(true)
        .create_if_missing(false)
        .foreign_keys(true);
    let mut conn = SqliteConnection::connect_with(&options)
        .await
        .map_err(|e| format!("database_inspection_failed: {e}"))?;
    let version = read_schema_version(&mut conn)
        .await
        .map_err(|e| format!("database_inspection_failed: {e}"))?;

    if version > SCHEMA_VERSION {
        Ok(DatabaseStartupState::newer_schema(version))
    } else {
        Ok(DatabaseStartupState::ready(Some(version)))
    }
}

async fn inspect_database_readiness_path(root: &Path, path: &Path) -> DatabaseReadinessResult {
    let supported_schema_version = readiness_supported_schema_version();
    let filesystem = match inspect_readiness_filesystem(root, path) {
        Ok(snapshot) => snapshot,
        Err(ReadinessFilesystemFailure::PathUnsafe) => {
            return DatabaseReadinessResult::bounded("path_unsafe")
        }
        Err(ReadinessFilesystemFailure::Unreadable) => {
            return DatabaseReadinessResult::bounded("unreadable")
        }
        Err(ReadinessFilesystemFailure::RecoveryRequired) => {
            return DatabaseReadinessResult::bounded("recovery_required")
        }
    };

    let mut result = DatabaseReadinessResult {
        classification: "missing".into(),
        database_exists: Some(filesystem.database_exists),
        detected_schema_version: None,
        supported_schema_version,
        wal_present: Some(filesystem.wal_present),
        shm_present: Some(filesystem.shm_present),
        rollback_journal_present: Some(filesystem.rollback_journal_present),
        quiescence: "not_proven".into(),
        operation_evidence: match filesystem.operation_evidence {
            ReadinessOperationEvidence::None => "none",
            ReadinessOperationEvidence::Present => "present",
        }
        .into(),
        schema_v5_available: supported_schema_version >= 5,
        inspected_at_unix_ms: inspection_time_ms(),
    };

    if filesystem.wal_present
        || filesystem.shm_present
        || filesystem.rollback_journal_present
        || filesystem.operation_evidence == ReadinessOperationEvidence::Present
    {
        result.classification = "recovery_required".into();
        return result;
    }
    if !filesystem.database_exists {
        return result;
    }

    if File::open(path).is_err() {
        result.classification = "unreadable".into();
        return result;
    }
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .read_only(true)
        .create_if_missing(false)
        .immutable(true)
        .foreign_keys(true);
    let mut connection = match SqliteConnection::connect_with(&options).await {
        Ok(connection) => connection,
        Err(_) => {
            result.classification = "malformed".into();
            return result;
        }
    };
    let version = match read_schema_version(&mut connection).await {
        Ok(version) => version,
        Err(_) => {
            result.classification = "malformed".into();
            return result;
        }
    };
    result.detected_schema_version = Some(version);
    result.classification = if version < SCHEMA_VERSION {
        "older_supported"
    } else if version == SCHEMA_VERSION {
        "exact_v4"
    } else if version == 5 && supported_schema_version >= 5 {
        "exact_v5"
    } else {
        "newer_unsupported"
    }
    .into();
    result
}

pub async fn migrate_connection(
    conn: &mut SqliteConnection,
    inject_failure: bool,
) -> Result<(), String> {
    conn.execute("PRAGMA foreign_keys = ON")
        .await
        .map_err(|e| e.to_string())?;
    let version = ensure_supported_schema(conn).await?;
    conn.execute("CREATE TABLE IF NOT EXISTS experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)").await.map_err(|e| e.to_string())?;
    if version == SCHEMA_VERSION {
        return Ok(());
    }
    let mut tx = conn.begin().await.map_err(|e| e.to_string())?;
    if version < 3 {
        tx.execute("CREATE TABLE IF NOT EXISTS persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE)").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_persisted_artifacts_source ON persisted_artifacts(source_entry_id, artifact_kind, created_at)").await.map_err(|e| e.to_string())?;
        for (table, kind) in [
            ("evidence_candidates", "evidence"),
            ("reflection_prompts", "reflection"),
            ("pattern_notes", "pattern"),
        ] {
            let exists: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?",
            )
            .bind(table)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
            if exists > 0 {
                let query = format!("INSERT INTO persisted_artifacts (id, source_entry_id, artifact_kind, payload, created_at, updated_at) SELECT id, source_entry_id, '{kind}', payload, created_at, updated_at FROM {table} WHERE status != 'rejected'");
                tx.execute(query.as_str())
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    if version < 4 {
        tx.execute("CREATE TABLE historical_consent_events (id TEXT PRIMARY KEY NOT NULL, packet_digest TEXT NOT NULL, payload TEXT NOT NULL, state TEXT NOT NULL CHECK(state IN ('granted','consumed','invalidated')), created_at TEXT NOT NULL, expires_at TEXT NOT NULL)").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE TABLE historical_transmission_events (id TEXT PRIMARY KEY NOT NULL, consent_id TEXT NOT NULL, packet_digest TEXT NOT NULL, provider TEXT NOT NULL, model TEXT NOT NULL, outcome TEXT NOT NULL CHECK(outcome IN ('sent','failed','refused','cancelled_before_send','cancelled_after_send')), created_at TEXT NOT NULL, expires_at TEXT NOT NULL, FOREIGN KEY(consent_id) REFERENCES historical_consent_events(id) ON DELETE CASCADE)").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE TABLE historical_question_artifacts (id TEXT PRIMARY KEY NOT NULL, current_experience_id TEXT NOT NULL, packet_digest TEXT NOT NULL, payload TEXT NOT NULL, packet_snapshot TEXT NOT NULL, consent_id TEXT NOT NULL, transmission_id TEXT NOT NULL, created_at TEXT NOT NULL, FOREIGN KEY(current_experience_id) REFERENCES experience_entries(id) ON DELETE CASCADE, FOREIGN KEY(consent_id) REFERENCES historical_consent_events(id) ON DELETE CASCADE, FOREIGN KEY(transmission_id) REFERENCES historical_transmission_events(id) ON DELETE CASCADE)").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE TABLE historical_artifact_dependencies (historical_artifact_id TEXT NOT NULL, source_entry_id TEXT NOT NULL, source_artifact_id TEXT, source_revision TEXT NOT NULL, PRIMARY KEY(historical_artifact_id, source_entry_id, source_artifact_id), FOREIGN KEY(historical_artifact_id) REFERENCES historical_question_artifacts(id) ON DELETE CASCADE, FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE)").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE INDEX idx_historical_dependencies_source ON historical_artifact_dependencies(source_entry_id, source_artifact_id)").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE INDEX idx_historical_questions_current ON historical_question_artifacts(current_experience_id, created_at)").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE TRIGGER delete_historical_artifacts_before_source_delete BEFORE DELETE ON experience_entries BEGIN DELETE FROM historical_question_artifacts WHERE id IN (SELECT historical_artifact_id FROM historical_artifact_dependencies WHERE source_entry_id = OLD.id); END").await.map_err(|e| e.to_string())?;
        tx.execute("CREATE TRIGGER delete_historical_provenance_after_artifact_delete AFTER DELETE ON historical_question_artifacts BEGIN DELETE FROM historical_transmission_events WHERE id = OLD.transmission_id; DELETE FROM historical_consent_events WHERE id = OLD.consent_id; END").await.map_err(|e| e.to_string())?;
    }
    if inject_failure {
        return Err("injected migration failure".into());
    }
    tx.execute("PRAGMA user_version = 4")
        .await
        .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn inspect_sqlite_database(app: AppHandle) -> Result<DatabaseStartupState, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("life-os.db");
    inspect_database_path(&path).await
}

#[tauri::command]
pub async fn inspect_database_readiness(app: AppHandle) -> DatabaseReadinessResult {
    let root = match app.path().app_data_dir() {
        Ok(root) => root,
        Err(_) => return DatabaseReadinessResult::bounded("unreadable"),
    };
    inspect_database_readiness_path(&root, &root.join("life-os.db")).await
}

#[tauri::command]
pub async fn initialize_sqlite_database(app: AppHandle) -> Result<(), String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let path = dir.join("life-os.db");
    let startup = inspect_database_path(&path).await?;
    if startup.state == "blocked" {
        return Err(newer_schema_error(
            startup
                .detected_schema_version
                .unwrap_or(SCHEMA_VERSION + 1),
        ));
    }
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    migrate_connection(&mut connect(&path).await?, false).await
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResult {
    status: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceImportResult {
    imported_count: u64,
    skipped_count: u64,
}

async fn create_experience_record(
    conn: &mut SqliteConnection,
    experience: ExperienceRecord,
) -> Result<TransactionResult, String> {
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query(
        "INSERT INTO experience_entries (id, content, created_at, updated_at) VALUES (?, ?, ?, ?)",
    )
    .bind(experience.id)
    .bind(experience.content)
    .bind(experience.created_at)
    .bind(experience.updated_at)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

async fn update_experience_record(
    conn: &mut SqliteConnection,
    id: String,
    content: String,
    expected_updated_at: String,
    updated_at: String,
) -> Result<TransactionResult, String> {
    ensure_supported_schema(conn).await?;
    if updated_at == expected_updated_at {
        return Err("invalid_revision: updated_at must advance".into());
    }
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    let update = sqlx::query(
        "UPDATE experience_entries SET content = ?, updated_at = ? WHERE id = ? AND updated_at = ?",
    )
    .bind(content)
    .bind(updated_at)
    .bind(&id)
    .bind(expected_updated_at)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if update.rows_affected() == 0 {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM experience_entries WHERE id = ?)")
                .bind(&id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        tx.rollback().await.map_err(|e| e.to_string())?;
        return Ok(TransactionResult {
            status: if exists {
                "stale_generation".into()
            } else {
                "not_found".into()
            },
        });
    }

    sqlx::query("DELETE FROM historical_question_artifacts WHERE current_experience_id = ? OR id IN (SELECT historical_artifact_id FROM historical_artifact_dependencies WHERE source_entry_id = ?)")
        .bind(&id)
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM persisted_artifacts WHERE source_entry_id = ?")
        .bind(&id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

async fn delete_experience_record(
    conn: &mut SqliteConnection,
    id: String,
) -> Result<TransactionResult, String> {
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    let deleted = sqlx::query("DELETE FROM experience_entries WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: if deleted.rows_affected() == 0 {
            "not_found".into()
        } else {
            "committed".into()
        },
    })
}

async fn import_experience_records(
    conn: &mut SqliteConnection,
    experiences: Vec<ExperienceRecord>,
    inject_failure_after: Option<usize>,
) -> Result<ExperienceImportResult, String> {
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    let mut imported_count = 0_u64;
    let mut skipped_count = 0_u64;
    for (index, experience) in experiences.into_iter().enumerate() {
        if inject_failure_after == Some(index) {
            tx.rollback().await.map_err(|e| e.to_string())?;
            return Err("injected experience import failure".into());
        }
        let inserted = sqlx::query("INSERT INTO experience_entries (id, content, created_at, updated_at) VALUES (?, ?, ?, ?) ON CONFLICT(id) DO NOTHING")
            .bind(experience.id)
            .bind(experience.content)
            .bind(experience.created_at)
            .bind(experience.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        if inserted.rows_affected() == 1 {
            imported_count += 1;
        } else {
            skipped_count += 1;
        }
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(ExperienceImportResult {
        imported_count,
        skipped_count,
    })
}

fn database_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("life-os.db"))
}

#[tauri::command]
pub async fn create_sqlite_experience(
    app: AppHandle,
    experience: ExperienceRecord,
) -> Result<TransactionResult, String> {
    create_experience_record(&mut connect(&database_path(&app)?).await?, experience).await
}

#[tauri::command]
pub async fn update_sqlite_experience(
    app: AppHandle,
    id: String,
    content: String,
    expected_updated_at: String,
    updated_at: String,
) -> Result<TransactionResult, String> {
    update_experience_record(
        &mut connect(&database_path(&app)?).await?,
        id,
        content,
        expected_updated_at,
        updated_at,
    )
    .await
}

#[tauri::command]
pub async fn delete_sqlite_experience(
    app: AppHandle,
    id: String,
) -> Result<TransactionResult, String> {
    delete_experience_record(&mut connect(&database_path(&app)?).await?, id).await
}

#[tauri::command]
pub async fn import_sqlite_experiences(
    app: AppHandle,
    experiences: Vec<ExperienceRecord>,
) -> Result<ExperienceImportResult, String> {
    import_experience_records(
        &mut connect(&database_path(&app)?).await?,
        experiences,
        None,
    )
    .await
}

fn json_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| format!("invalid_artifact_field:{field}"))
}

fn json_string_array<'a>(value: &'a Value, field: &str) -> Result<Vec<&'a str>, String> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("invalid_artifact_field:{field}"))?
        .iter()
        .map(|item| {
            item.as_str()
                .filter(|text| !text.is_empty())
                .ok_or_else(|| format!("invalid_artifact_field:{field}"))
        })
        .collect()
}

fn artifact_record(
    value: &Value,
    entry_id: &str,
    artifact_kind: &'static str,
) -> Result<PersistedArtifactRecord, String> {
    let id = json_string(value, "id")?.to_owned();
    if json_string(value, "sourceEntryId")? != entry_id {
        return Err(format!("artifact_source_mismatch:{id}"));
    }
    Ok(PersistedArtifactRecord {
        id,
        artifact_kind,
        payload: serde_json::to_string(value).map_err(|e| e.to_string())?,
        created_at: json_string(value, "createdAt")?.to_owned(),
        updated_at: json_string(value, "updatedAt")?.to_owned(),
    })
}

fn prepare_artifact_records(
    entry_id: &str,
    bundle: &PersistedArtifactBundleRecord,
) -> Result<Vec<PersistedArtifactRecord>, String> {
    let confirmed_evidence = bundle
        .evidence
        .iter()
        .filter(|value| value.get("status").and_then(Value::as_str) == Some("confirmed"))
        .map(|value| json_string(value, "id").map(str::to_owned))
        .collect::<Result<HashSet<_>, _>>()?;
    let answered_reflections = bundle
        .reflections
        .iter()
        .filter(|value| {
            value.get("status").and_then(Value::as_str) == Some("answered")
                && value
                    .get("response")
                    .and_then(Value::as_str)
                    .is_some_and(|response| !response.trim().is_empty())
        })
        .map(|value| json_string(value, "id").map(str::to_owned))
        .collect::<Result<HashSet<_>, _>>()?;

    let mut records = Vec::new();
    let mut ids = HashSet::new();
    for value in &bundle.evidence {
        if value.get("status").and_then(Value::as_str) == Some("rejected") {
            return Err(format!(
                "rejected_artifact_not_persistable:{}",
                json_string(value, "id")?
            ));
        }
        records.push(artifact_record(value, entry_id, "evidence")?);
    }
    for value in &bundle.reflections {
        let source_ids = json_string_array(value, "sourceEvidenceIds")?;
        if source_ids.is_empty()
            || source_ids
                .iter()
                .any(|source_id| !confirmed_evidence.contains(*source_id))
        {
            return Err(format!(
                "invalid_reflection_dependency:{}",
                json_string(value, "id")?
            ));
        }
        records.push(artifact_record(value, entry_id, "reflection")?);
    }
    for value in &bundle.patterns {
        if value.get("status").and_then(Value::as_str) == Some("rejected") {
            return Err(format!(
                "rejected_artifact_not_persistable:{}",
                json_string(value, "id")?
            ));
        }
        let evidence_ids = json_string_array(value, "sourceEvidenceIds")?;
        if evidence_ids
            .iter()
            .any(|source_id| !confirmed_evidence.contains(*source_id))
        {
            return Err(format!(
                "invalid_pattern_dependency:{}",
                json_string(value, "id")?
            ));
        }
        if let Some(reflection_ids) = value.get("sourceReflectionPromptIds") {
            let reflection_ids = reflection_ids
                .as_array()
                .ok_or_else(|| "invalid_artifact_field:sourceReflectionPromptIds".to_string())?;
            if reflection_ids.iter().any(|source_id| {
                source_id
                    .as_str()
                    .is_none_or(|id| !answered_reflections.contains(id))
            }) {
                return Err(format!(
                    "invalid_pattern_dependency:{}",
                    json_string(value, "id")?
                ));
            }
        }
        records.push(artifact_record(value, entry_id, "pattern")?);
    }
    for value in &bundle.recovery_turns {
        records.push(artifact_record(value, entry_id, "recovery_turn")?);
    }
    for record in &records {
        if !ids.insert(record.id.clone()) {
            return Err(format!("duplicate_artifact_id:{}", record.id));
        }
    }
    Ok(records)
}

async fn save_artifact_bundle_records(
    conn: &mut SqliteConnection,
    entry_id: String,
    bundle: PersistedArtifactBundleRecord,
    expected_experience_updated_at: Option<String>,
    inject_failure_after: Option<usize>,
) -> Result<TransactionResult, String> {
    let records = prepare_artifact_records(&entry_id, &bundle)?;
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    let current: Option<String> =
        sqlx::query_scalar("SELECT updated_at FROM experience_entries WHERE id = ?")
            .bind(&entry_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    let Some(current) = current else {
        tx.rollback().await.map_err(|e| e.to_string())?;
        return Ok(TransactionResult {
            status: "not_found".into(),
        });
    };
    if expected_experience_updated_at
        .as_deref()
        .is_some_and(|expected| expected != current)
    {
        tx.rollback().await.map_err(|e| e.to_string())?;
        return Ok(TransactionResult {
            status: "stale_generation".into(),
        });
    }

    sqlx::query("DELETE FROM historical_question_artifacts WHERE id IN (SELECT historical_artifact_id FROM historical_artifact_dependencies WHERE source_entry_id = ?)")
        .bind(&entry_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM persisted_artifacts WHERE source_entry_id = ?")
        .bind(&entry_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    for (index, record) in records.into_iter().enumerate() {
        if inject_failure_after == Some(index) {
            tx.rollback().await.map_err(|e| e.to_string())?;
            return Err("injected artifact bundle failure".into());
        }
        sqlx::query("INSERT INTO persisted_artifacts (id, source_entry_id, artifact_kind, payload, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(record.id)
            .bind(&entry_id)
            .bind(record.artifact_kind)
            .bind(record.payload)
            .bind(record.created_at)
            .bind(record.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

fn same_consent_scope(
    existing: &HistoricalConsentRecord,
    candidate: &HistoricalConsentRecord,
) -> bool {
    existing.id == candidate.id
        && existing.packet_digest == candidate.packet_digest
        && existing.task == candidate.task
        && existing.purpose == candidate.purpose
        && existing.provider == candidate.provider
        && existing.model == candidate.model
        && existing.source_revisions == candidate.source_revisions
        && existing.created_at == candidate.created_at
        && existing.expires_at == candidate.expires_at
}

fn valid_consent_transition(from: &str, to: &str) -> bool {
    from == to || (from == "granted" && matches!(to, "consumed" | "invalidated"))
}

async fn save_historical_consent_record(
    conn: &mut SqliteConnection,
    event: HistoricalConsentRecord,
) -> Result<TransactionResult, String> {
    if event.task != "historical_reflection_questions"
        || event.purpose != "invite_user_comparison_without_cross_time_conclusions"
        || !matches!(event.provider.as_str(), "openai" | "gemini")
        || event.model.trim().is_empty()
    {
        return Err("invalid_historical_consent_scope".into());
    }
    if !matches!(event.state.as_str(), "granted" | "consumed" | "invalidated") {
        return Err("invalid_historical_consent_state".into());
    }
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    let existing_payload: Option<String> =
        sqlx::query_scalar("SELECT payload FROM historical_consent_events WHERE id = ?")
            .bind(&event.id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    let payload = serde_json::to_string(&event).map_err(|e| e.to_string())?;
    if let Some(existing_payload) = existing_payload {
        let existing: HistoricalConsentRecord =
            serde_json::from_str(&existing_payload).map_err(|e| e.to_string())?;
        if !same_consent_scope(&existing, &event)
            || !valid_consent_transition(&existing.state, &event.state)
        {
            tx.rollback().await.map_err(|e| e.to_string())?;
            return Err("historical_consent_conflict".into());
        }
        sqlx::query("UPDATE historical_consent_events SET payload = ?, state = ? WHERE id = ?")
            .bind(payload)
            .bind(&event.state)
            .bind(&event.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        sqlx::query("INSERT INTO historical_consent_events (id, packet_digest, payload, state, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&event.id)
            .bind(&event.packet_digest)
            .bind(payload)
            .bind(&event.state)
            .bind(&event.created_at)
            .bind(&event.expires_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

async fn save_historical_transmission_record(
    conn: &mut SqliteConnection,
    event: HistoricalTransmissionRecord,
    inject_failure_before_consent: bool,
) -> Result<TransactionResult, String> {
    if !matches!(
        event.outcome.as_str(),
        "sent" | "failed" | "refused" | "cancelled_before_send" | "cancelled_after_send"
    ) {
        return Err("invalid_historical_transmission_outcome".into());
    }
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    let consent_payload: Option<String> =
        sqlx::query_scalar("SELECT payload FROM historical_consent_events WHERE id = ?")
            .bind(&event.consent_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    let Some(consent_payload) = consent_payload else {
        tx.rollback().await.map_err(|e| e.to_string())?;
        return Err("historical_transmission_consent_missing".into());
    };
    let consent: HistoricalConsentRecord =
        serde_json::from_str(&consent_payload).map_err(|e| e.to_string())?;
    if consent.packet_digest != event.packet_digest
        || consent.provider != event.provider
        || consent.model != event.model
        || !matches!(consent.state.as_str(), "granted" | "consumed")
    {
        tx.rollback().await.map_err(|e| e.to_string())?;
        return Err("historical_transmission_consent_mismatch".into());
    }
    let existing: Option<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT consent_id, packet_digest, provider, model, created_at FROM historical_transmission_events WHERE id = ?",
    )
    .bind(&event.id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    if let Some((consent_id, digest, provider, model, created_at)) = existing {
        if consent_id != event.consent_id
            || digest != event.packet_digest
            || provider != event.provider
            || model != event.model
            || created_at != event.created_at
        {
            tx.rollback().await.map_err(|e| e.to_string())?;
            return Err("historical_transmission_conflict".into());
        }
        sqlx::query(
            "UPDATE historical_transmission_events SET outcome = ?, expires_at = ? WHERE id = ?",
        )
        .bind(&event.outcome)
        .bind(&event.expires_at)
        .bind(&event.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    } else {
        sqlx::query("INSERT INTO historical_transmission_events (id, consent_id, packet_digest, provider, model, outcome, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&event.id)
            .bind(&event.consent_id)
            .bind(&event.packet_digest)
            .bind(&event.provider)
            .bind(&event.model)
            .bind(&event.outcome)
            .bind(&event.created_at)
            .bind(&event.expires_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }
    if inject_failure_before_consent {
        tx.rollback().await.map_err(|e| e.to_string())?;
        return Err("injected historical transmission failure".into());
    }
    if matches!(
        event.outcome.as_str(),
        "sent" | "failed" | "cancelled_after_send"
    ) {
        let mut consumed = consent;
        consumed.state = "consumed".into();
        let consumed_payload = serde_json::to_string(&consumed).map_err(|e| e.to_string())?;
        sqlx::query(
            "UPDATE historical_consent_events SET payload = ?, state = 'consumed' WHERE id = ?",
        )
        .bind(consumed_payload)
        .bind(&event.consent_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

fn historical_expectations(
    artifact: &HistoricalQuestionArtifactRecord,
) -> Result<
    (
        HistoricalPacketPersistenceRecord,
        Vec<ExpectedExperienceRevision>,
        Vec<ExpectedArtifactRevision>,
        ExpectedHistoricalProvenance,
    ),
    String,
> {
    let packet: HistoricalPacketPersistenceRecord =
        serde_json::from_value(artifact.packet.clone()).map_err(|e| e.to_string())?;
    if packet.current_experience.id != artifact.current_experience_id {
        return Err("historical_current_experience_mismatch".into());
    }
    if packet.task != "historical_reflection_questions"
        || packet.purpose != "invite_user_comparison_without_cross_time_conclusions"
        || packet.consent.reference != artifact.consent_id
        || packet.consent.scope != "one_generation_one_purpose"
        || !matches!(packet.destination.provider.as_str(), "openai" | "gemini")
        || packet.destination.model.trim().is_empty()
    {
        return Err("historical_packet_scope_mismatch".into());
    }
    let historical_sources = packet
        .included_items
        .iter()
        .map(|item| item.source_experience_id.as_str())
        .collect::<HashSet<_>>();
    let mut question_ids = HashSet::new();
    for question in &artifact.questions {
        if question.id.trim().is_empty()
            || question.text.trim().is_empty()
            || !question_ids.insert(question.id.as_str())
            || question.source_experience_ids.is_empty()
            || question.source_experience_ids.iter().any(|source_id| {
                source_id != &packet.current_experience.id
                    && !historical_sources.contains(source_id.as_str())
            })
            || !question
                .source_experience_ids
                .iter()
                .any(|source_id| historical_sources.contains(source_id.as_str()))
        {
            return Err("historical_question_sources_invalid".into());
        }
    }
    let mut expected_revisions = vec![ExpectedExperienceRevision {
        id: packet.current_experience.id.clone(),
        updated_at: packet.current_experience.revision.clone(),
    }];
    let mut expected_artifacts = Vec::new();
    let mut dependency_keys = HashSet::new();
    for item in &packet.included_items {
        let dependency_key = (item.source_experience_id.clone(), item.artifact_id.clone());
        if !dependency_keys.insert(dependency_key) {
            return Err("duplicate_historical_dependency".into());
        }
        match item.item_type.as_str() {
            "experience" => {
                if item.artifact_id.is_some() {
                    return Err("historical_experience_artifact_id_invalid".into());
                }
                expected_revisions.push(ExpectedExperienceRevision {
                    id: item.source_experience_id.clone(),
                    updated_at: item.revision.clone(),
                });
            }
            "evidence" | "reflection_response" => {
                let id = item
                    .artifact_id
                    .clone()
                    .ok_or_else(|| "historical_artifact_id_missing".to_string())?;
                expected_artifacts.push(ExpectedArtifactRevision {
                    id,
                    source_entry_id: item.source_experience_id.clone(),
                    updated_at: item.revision.clone(),
                    artifact_kind: if item.item_type == "evidence" {
                        "evidence".into()
                    } else {
                        "reflection".into()
                    },
                });
            }
            _ => return Err("historical_item_type_invalid".into()),
        }
    }
    let provenance = ExpectedHistoricalProvenance {
        consent_id: artifact.consent_id.clone(),
        transmission_id: artifact.transmission_id.clone(),
        packet_digest: packet.packet_digest.clone(),
        provider: packet.destination.provider.clone(),
        model: packet.destination.model.clone(),
    };
    Ok((packet, expected_revisions, expected_artifacts, provenance))
}

async fn persist_historical_question_record(
    conn: &mut SqliteConnection,
    artifact: HistoricalQuestionArtifactRecord,
) -> Result<TransactionResult, String> {
    let (packet, expected_revisions, expected_artifacts, expected_provenance) =
        historical_expectations(&artifact)?;
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    for expected in expected_revisions {
        let current: Option<String> =
            sqlx::query_scalar("SELECT updated_at FROM experience_entries WHERE id = ?")
                .bind(&expected.id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        if current.as_deref() != Some(expected.updated_at.as_str()) {
            return Ok(TransactionResult {
                status: "stale_generation".into(),
            });
        }
    }
    for expected in expected_artifacts {
        let current: Option<(String, String)> = sqlx::query_as("SELECT updated_at, payload FROM persisted_artifacts WHERE id = ? AND source_entry_id = ? AND artifact_kind = ?").bind(&expected.id).bind(&expected.source_entry_id).bind(&expected.artifact_kind).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;
        let Some((updated_at, payload)) = current else {
            return Ok(TransactionResult {
                status: "stale_generation".into(),
            });
        };
        if updated_at != expected.updated_at {
            return Ok(TransactionResult {
                status: "stale_generation".into(),
            });
        }
        let value: Value = serde_json::from_str(&payload).map_err(|e| e.to_string())?;
        let mut eligible = match expected.artifact_kind.as_str() {
            "evidence" => value.get("status").and_then(Value::as_str) == Some("confirmed"),
            "reflection" => {
                value.get("status").and_then(Value::as_str) == Some("answered")
                    && value
                        .get("response")
                        .and_then(Value::as_str)
                        .map(|text| !text.trim().is_empty())
                        .unwrap_or(false)
                    && value
                        .get("responseProvenance")
                        .and_then(|item| item.get("origin"))
                        .and_then(Value::as_str)
                        == Some("user")
            }
            _ => false,
        };
        if eligible && expected.artifact_kind == "reflection" {
            let evidence_ids = value
                .get("sourceEvidenceIds")
                .and_then(Value::as_array)
                .filter(|ids| !ids.is_empty());
            let Some(evidence_ids) = evidence_ids else {
                return Ok(TransactionResult {
                    status: "stale_generation".into(),
                });
            };
            for evidence_id in evidence_ids {
                let Some(evidence_id) = evidence_id.as_str() else {
                    eligible = false;
                    break;
                };
                let evidence_payload: Option<String> = sqlx::query_scalar(
                    "SELECT payload FROM persisted_artifacts WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'evidence'",
                )
                .bind(evidence_id)
                .bind(&expected.source_entry_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
                let evidence_is_confirmed = evidence_payload
                    .as_deref()
                    .and_then(|payload| serde_json::from_str::<Value>(payload).ok())
                    .and_then(|record| {
                        record
                            .get("status")
                            .and_then(Value::as_str)
                            .map(str::to_owned)
                    })
                    .as_deref()
                    == Some("confirmed");
                if !evidence_is_confirmed {
                    eligible = false;
                    break;
                }
            }
        }
        if !eligible {
            return Ok(TransactionResult {
                status: "stale_generation".into(),
            });
        }
    }
    let consent: Option<(String, String, String)> = sqlx::query_as(
        "SELECT packet_digest, state, payload FROM historical_consent_events WHERE id = ?",
    )
    .bind(&expected_provenance.consent_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    let consent_matches = consent.as_ref().is_some_and(|(digest, state, payload)| {
        let Ok(event) = serde_json::from_str::<Value>(payload) else {
            return false;
        };
        digest == &expected_provenance.packet_digest
            && state == "consumed"
            && event.get("id").and_then(Value::as_str)
                == Some(expected_provenance.consent_id.as_str())
            && event.get("packetDigest").and_then(Value::as_str)
                == Some(expected_provenance.packet_digest.as_str())
            && event.get("provider").and_then(Value::as_str)
                == Some(expected_provenance.provider.as_str())
            && event.get("model").and_then(Value::as_str)
                == Some(expected_provenance.model.as_str())
    });
    if !consent_matches {
        return Ok(TransactionResult {
            status: "stale_generation".into(),
        });
    }
    let transmission: Option<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT consent_id, packet_digest, provider, model, outcome FROM historical_transmission_events WHERE id = ?",
    )
    .bind(&expected_provenance.transmission_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    if transmission
        .as_ref()
        .map(|(consent_id, digest, provider, model, outcome)| {
            (
                consent_id.as_str(),
                digest.as_str(),
                provider.as_str(),
                model.as_str(),
                outcome.as_str(),
            )
        })
        != Some((
            expected_provenance.consent_id.as_str(),
            expected_provenance.packet_digest.as_str(),
            expected_provenance.provider.as_str(),
            expected_provenance.model.as_str(),
            "sent",
        ))
    {
        return Ok(TransactionResult {
            status: "stale_generation".into(),
        });
    }
    let payload = serde_json::json!({
        "id": &artifact.id,
        "currentExperienceId": &artifact.current_experience_id,
        "questions": &artifact.questions,
        "consentId": &artifact.consent_id,
        "transmissionId": &artifact.transmission_id,
        "generatedAt": &artifact.generated_at,
    });
    sqlx::query("INSERT INTO historical_question_artifacts (id, current_experience_id, packet_digest, payload, packet_snapshot, consent_id, transmission_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&artifact.id)
        .bind(&artifact.current_experience_id)
        .bind(&packet.packet_digest)
        .bind(serde_json::to_string(&payload).map_err(|e| e.to_string())?)
        .bind(serde_json::to_string(&artifact.packet).map_err(|e| e.to_string())?)
        .bind(&artifact.consent_id)
        .bind(&artifact.transmission_id)
        .bind(&artifact.generated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    for item in &packet.included_items {
        sqlx::query("INSERT INTO historical_artifact_dependencies (historical_artifact_id, source_entry_id, source_artifact_id, source_revision) VALUES (?, ?, ?, ?)")
            .bind(&artifact.id)
            .bind(&item.source_experience_id)
            .bind(&item.artifact_id)
            .bind(&item.revision)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

async fn delete_historical_question_record(
    conn: &mut SqliteConnection,
    id: String,
) -> Result<TransactionResult, String> {
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    let deleted = sqlx::query("DELETE FROM historical_question_artifacts WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: if deleted.rows_affected() == 0 {
            "not_found".into()
        } else {
            "committed".into()
        },
    })
}

async fn purge_expired_historical_audit_records(
    conn: &mut SqliteConnection,
    timestamp: String,
) -> Result<TransactionResult, String> {
    ensure_supported_schema(conn).await?;
    let mut tx = conn
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM historical_transmission_events WHERE expires_at <= ? AND id NOT IN (SELECT transmission_id FROM historical_question_artifacts)")
        .bind(&timestamp)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM historical_consent_events WHERE expires_at <= ? AND id NOT IN (SELECT consent_id FROM historical_question_artifacts)")
        .bind(&timestamp)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

#[tauri::command]
pub async fn save_sqlite_artifacts(
    app: AppHandle,
    entry_id: String,
    bundle: PersistedArtifactBundleRecord,
    expected_experience_updated_at: Option<String>,
) -> Result<TransactionResult, String> {
    save_artifact_bundle_records(
        &mut connect(&database_path(&app)?).await?,
        entry_id,
        bundle,
        expected_experience_updated_at,
        None,
    )
    .await
}

#[tauri::command]
pub async fn save_sqlite_historical_consent(
    app: AppHandle,
    event: HistoricalConsentRecord,
) -> Result<TransactionResult, String> {
    save_historical_consent_record(&mut connect(&database_path(&app)?).await?, event).await
}

#[tauri::command]
pub async fn save_sqlite_historical_transmission(
    app: AppHandle,
    event: HistoricalTransmissionRecord,
) -> Result<TransactionResult, String> {
    save_historical_transmission_record(&mut connect(&database_path(&app)?).await?, event, false)
        .await
}

#[tauri::command]
pub async fn save_sqlite_historical_question_artifact(
    app: AppHandle,
    artifact: HistoricalQuestionArtifactRecord,
) -> Result<TransactionResult, String> {
    persist_historical_question_record(&mut connect(&database_path(&app)?).await?, artifact).await
}

#[tauri::command]
pub async fn delete_sqlite_historical_question_artifact(
    app: AppHandle,
    id: String,
) -> Result<TransactionResult, String> {
    delete_historical_question_record(&mut connect(&database_path(&app)?).await?, id).await
}

#[tauri::command]
pub async fn purge_sqlite_expired_historical_audit_records(
    app: AppHandle,
    timestamp: String,
) -> Result<TransactionResult, String> {
    purge_expired_historical_audit_records(&mut connect(&database_path(&app)?).await?, timestamp)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn missing_database_inspection_does_not_create_a_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing.db");

        let state = inspect_database_path(&path).await.unwrap();

        assert_eq!(state, DatabaseStartupState::ready(None));
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn v4_inspection_is_read_only_and_reports_ready() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE marker (value TEXT NOT NULL); INSERT INTO marker VALUES ('preserve-me'); PRAGMA user_version = 4;")
            .await
            .unwrap();
        drop(conn);
        let before = std::fs::read(file.path()).unwrap();

        let state = inspect_database_path(file.path()).await.unwrap();
        let after = std::fs::read(file.path()).unwrap();

        assert_eq!(state, DatabaseStartupState::ready(Some(4)));
        assert_eq!(after, before);
    }

    #[tokio::test]
    async fn v4_migration_preserves_existing_create_if_missing_behavior() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("PRAGMA user_version = 4;").await.unwrap();

        migrate_connection(&mut conn, false).await.unwrap();

        let experience_table: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='experience_entries'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        assert_eq!(experience_table, 1);
        assert_eq!(read_schema_version(&mut conn).await.unwrap(), 4);
    }

    #[tokio::test]
    async fn newer_schema_is_refused_before_migration_ddl() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE marker (value TEXT NOT NULL); INSERT INTO marker VALUES ('preserve-me'); PRAGMA user_version = 5;")
            .await
            .unwrap();
        drop(conn);

        let state = inspect_database_path(file.path()).await.unwrap();
        assert_eq!(state, DatabaseStartupState::newer_schema(5));

        let mut conn = connect(file.path()).await.unwrap();
        let error = migrate_connection(&mut conn, false).await.unwrap_err();
        assert!(error.contains(NEWER_SCHEMA_ERROR));
        let version: i64 = read_schema_version(&mut conn).await.unwrap();
        let experience_table: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='experience_entries'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        let marker: String = sqlx::query_scalar("SELECT value FROM marker")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(version, 5);
        assert_eq!(experience_table, 0);
        assert_eq!(marker, "preserve-me");
    }

    #[tokio::test]
    async fn malformed_database_inspection_fails_without_changing_bytes() {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), b"not-a-sqlite-database").unwrap();
        let before = std::fs::read(file.path()).unwrap();

        let error = inspect_database_path(file.path()).await.unwrap_err();
        let after = std::fs::read(file.path()).unwrap();

        assert!(error.contains("database_inspection_failed"));
        assert_eq!(after, before);
    }

    async fn readiness_fixture(root: &Path, version: i64) -> std::path::PathBuf {
        let path = root.join("life-os.db");
        let mut conn = connect(&path).await.unwrap();
        conn.execute(
            "CREATE TABLE marker (value TEXT NOT NULL); INSERT INTO marker VALUES ('preserve-me');",
        )
        .await
        .unwrap();
        conn.execute(format!("PRAGMA user_version = {version}").as_str())
            .await
            .unwrap();
        drop(conn);
        path
    }

    #[tokio::test]
    async fn readiness_missing_path_does_not_create_a_directory_or_database() {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join("missing-app-data");
        let path = root.join("life-os.db");

        let result = inspect_database_readiness_path(&root, &path).await;

        assert_eq!(result.classification, "missing");
        assert_eq!(result.database_exists, Some(false));
        assert!(!root.exists());
        assert!(!path.exists());
        assert_eq!(
            result.schema_v5_available,
            readiness_supported_schema_version() >= 5
        );
    }

    #[tokio::test]
    async fn readiness_classifies_supported_exact_and_newer_versions_without_mutation() {
        let expected_v5 = if readiness_supported_schema_version() >= 5 {
            "exact_v5"
        } else {
            "newer_unsupported"
        };
        for (version, expected) in [
            (3, "older_supported"),
            (4, "exact_v4"),
            (5, expected_v5),
            (6, "newer_unsupported"),
        ] {
            let root = tempfile::tempdir().unwrap();
            let path = readiness_fixture(root.path(), version).await;
            let before = std::fs::read(&path).unwrap();
            let modified_before = std::fs::metadata(&path).unwrap().modified().unwrap();

            let first = inspect_database_readiness_path(root.path(), &path).await;
            let second = inspect_database_readiness_path(root.path(), &path).await;

            assert_eq!(first.classification, expected);
            assert_eq!(first.detected_schema_version, Some(version));
            assert_eq!(
                first.supported_schema_version,
                readiness_supported_schema_version()
            );
            assert_eq!(first.quiescence, "not_proven");
            assert_eq!(first.operation_evidence, "none");
            assert_eq!(
                first.schema_v5_available,
                readiness_supported_schema_version() >= 5
            );
            assert_eq!(second.classification, expected);
            assert_eq!(std::fs::read(&path).unwrap(), before);
            assert_eq!(
                std::fs::metadata(&path).unwrap().modified().unwrap(),
                modified_before
            );
        }
        assert_eq!(SCHEMA_VERSION, 4);
    }

    #[tokio::test]
    async fn readiness_malformed_database_fails_closed_without_raw_error_or_mutation() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("life-os.db");
        std::fs::write(&path, b"not-a-sqlite-database").unwrap();
        let before = std::fs::read(&path).unwrap();

        let result = inspect_database_readiness_path(root.path(), &path).await;

        assert_eq!(result.classification, "malformed");
        assert_eq!(result.database_exists, Some(true));
        assert_eq!(result.detected_schema_version, None);
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }

    #[tokio::test]
    async fn readiness_sidecars_are_disclosed_and_left_byte_identical() {
        let root = tempfile::tempdir().unwrap();
        let path = readiness_fixture(root.path(), 4).await;
        let wal = root.path().join("life-os.db-wal");
        let shm = root.path().join("life-os.db-shm");
        let journal = root.path().join("life-os.db-journal");
        std::fs::write(&wal, b"wal-evidence").unwrap();
        std::fs::write(&shm, b"shm-evidence").unwrap();
        std::fs::write(&journal, b"journal-evidence").unwrap();

        let result = inspect_database_readiness_path(root.path(), &path).await;

        assert_eq!(result.classification, "recovery_required");
        assert_eq!(result.wal_present, Some(true));
        assert_eq!(result.shm_present, Some(true));
        assert_eq!(result.rollback_journal_present, Some(true));
        assert_eq!(std::fs::read(&wal).unwrap(), b"wal-evidence");
        assert_eq!(std::fs::read(&shm).unwrap(), b"shm-evidence");
        assert_eq!(std::fs::read(&journal).unwrap(), b"journal-evidence");
    }

    #[tokio::test]
    async fn readiness_malformed_or_multiple_owned_operations_require_recovery_without_cleanup() {
        let root = tempfile::tempdir().unwrap();
        let path = readiness_fixture(root.path(), 4).await;
        let first = root
            .path()
            .join("life-os-0123456789abcdef0123456789abcdef.operation");
        std::fs::create_dir(&first).unwrap();

        let missing_state = inspect_database_readiness_path(root.path(), &path).await;
        assert_eq!(missing_state.classification, "recovery_required");
        assert!(first.exists());

        std::fs::write(first.join("state.json"), b"{malformed").unwrap();
        let malformed = inspect_database_readiness_path(root.path(), &path).await;
        assert_eq!(malformed.classification, "recovery_required");
        assert_eq!(
            std::fs::read(first.join("state.json")).unwrap(),
            b"{malformed"
        );

        let second = root
            .path()
            .join("life-os-1123456789abcdef0123456789abcdef.operation");
        std::fs::create_dir(&second).unwrap();
        let multiple = inspect_database_readiness_path(root.path(), &path).await;
        assert_eq!(multiple.classification, "recovery_required");
        assert!(first.exists());
        assert!(second.exists());
    }

    #[tokio::test]
    async fn readiness_refuses_traversal_and_hard_link_aliases() {
        let root = tempfile::tempdir().unwrap();
        let path = readiness_fixture(root.path(), 4).await;
        let aliased_root = root.path().join("nested").join("..");
        let aliased_path = aliased_root.join("life-os.db");
        assert_eq!(
            inspect_database_readiness_path(&aliased_root, &aliased_path)
                .await
                .classification,
            "path_unsafe"
        );

        let alias = root.path().join("alias.db");
        std::fs::hard_link(&path, &alias).unwrap();
        assert_eq!(
            inspect_database_readiness_path(root.path(), &path)
                .await
                .classification,
            "path_unsafe"
        );
        assert!(alias.exists());
        assert!(path.exists());
    }

    #[tokio::test]
    async fn typed_artifact_save_refuses_a_newer_schema_without_writing() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE marker (value TEXT NOT NULL); INSERT INTO marker VALUES ('before'); PRAGMA user_version = 5;")
            .await
            .unwrap();

        let error = match save_artifact_bundle_records(
            &mut conn,
            "entry".into(),
            PersistedArtifactBundleRecord {
                evidence: vec![],
                reflections: vec![],
                patterns: vec![],
                recovery_turns: vec![],
            },
            None,
            None,
        )
        .await
        {
            Err(error) => error,
            Ok(_) => panic!("newer schema transaction must fail closed"),
        };
        let marker: String = sqlx::query_scalar("SELECT value FROM marker")
            .fetch_one(&mut conn)
            .await
            .unwrap();

        assert!(error.contains(NEWER_SCHEMA_ERROR));
        assert_eq!(marker, "before");
    }

    async fn fixture(path: &Path) -> SqliteConnection {
        let mut conn = connect(path).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE evidence_candidates (id TEXT PRIMARY KEY, source_entry_id TEXT, status TEXT, payload TEXT, created_at TEXT, updated_at TEXT); INSERT INTO evidence_candidates VALUES ('keep','entry','confirmed','{\"id\":\"keep\"}','t','t'); INSERT INTO evidence_candidates VALUES ('drop','entry','rejected','{\"id\":\"drop\"}','t','t'); PRAGMA user_version = 2;").await.unwrap();
        conn
    }
    #[tokio::test]
    async fn migrates_v2_and_excludes_rejected() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = fixture(file.path()).await;
        migrate_connection(&mut conn, false).await.unwrap();
        let version: i64 = sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        let ids: Vec<String> = sqlx::query_scalar("SELECT id FROM persisted_artifacts ORDER BY id")
            .fetch_all(&mut conn)
            .await
            .unwrap();
        assert_eq!(version, 4);
        assert_eq!(ids, vec!["keep"]);
    }
    #[tokio::test]
    async fn injected_failure_rolls_back_and_keeps_version() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = fixture(file.path()).await;
        assert!(migrate_connection(&mut conn, true).await.is_err());
        let version: i64 = sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        let exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='persisted_artifacts'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        let legacy: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM evidence_candidates")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(version, 2);
        assert_eq!(exists, 0);
        assert_eq!(legacy, 2);
    }
    #[tokio::test]
    async fn wrong_expected_version_keeps_existing_artifacts() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = fixture(file.path()).await;
        migrate_connection(&mut conn, false).await.unwrap();
        let result = save_artifact_bundle_records(
            &mut conn,
            "entry".into(),
            PersistedArtifactBundleRecord {
                evidence: vec![],
                reflections: vec![],
                patterns: vec![],
                recovery_turns: vec![],
            },
            Some("wrong-version".into()),
            None,
        )
        .await
        .unwrap();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM persisted_artifacts")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(result.status, "stale_generation");
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn v4_adds_governance_tables_without_removing_v3_data() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO persisted_artifacts VALUES ('kept','entry','evidence','{}','t','t'); PRAGMA user_version = 3;").await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        let version: i64 = sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        let kept: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM persisted_artifacts WHERE id='kept'")
                .fetch_one(&mut conn)
                .await
                .unwrap();
        let tables: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name LIKE 'historical_%'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        assert_eq!(version, 4);
        assert_eq!(kept, 1);
        assert_eq!(tables, 4);
    }

    fn historical_artifact(
        digest: &str,
        provider: &str,
        model: &str,
        included_items: Value,
    ) -> HistoricalQuestionArtifactRecord {
        HistoricalQuestionArtifactRecord {
            id: "must-not-write".into(),
            current_experience_id: "entry".into(),
            questions: vec![],
            packet: serde_json::json!({
                "packetDigest": digest,
                "currentExperience": { "id": "entry", "revision": "t" },
                "task": "historical_reflection_questions",
                "purpose": "invite_user_comparison_without_cross_time_conclusions",
                "destination": { "provider": provider, "model": model },
                "includedItems": included_items,
                "consent": { "reference": "consent", "scope": "one_generation_one_purpose" },
            }),
            consent_id: "consent".into(),
            transmission_id: "transmission".into(),
            generated_at: "t".into(),
        }
    }

    #[tokio::test]
    async fn historical_persistence_revalidates_artifact_eligibility_inside_transaction() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO persisted_artifacts VALUES ('evidence','entry','evidence','{\"status\":\"rejected\"}','t','t'); PRAGMA user_version = 3;").await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        let result = persist_historical_question_record(
            &mut conn,
            historical_artifact("d", "openai", "model", serde_json::json!([
                { "itemType": "evidence", "sourceExperienceId": "entry", "artifactId": "evidence", "revision": "t" }
            ])),
        ).await.unwrap();
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM historical_question_artifacts WHERE id='must-not-write'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        assert_eq!(result.status, "stale_generation");
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn historical_persistence_revalidates_actual_use_provenance_inside_transaction() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); PRAGMA user_version = 3;").await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        conn.execute("INSERT INTO historical_consent_events VALUES ('consent','digest','{}','consumed','t','z'); INSERT INTO historical_transmission_events VALUES ('transmission','consent','digest','openai','model','sent','t','z');").await.unwrap();
        let result = persist_historical_question_record(
            &mut conn,
            historical_artifact("wrong", "openai", "model", serde_json::json!([])),
        )
        .await
        .unwrap();
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM historical_question_artifacts WHERE id='must-not-write'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        assert_eq!(result.status, "stale_generation");
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn historical_persistence_rejects_consent_destination_mismatch() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); PRAGMA user_version = 3;").await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        conn.execute(r#"INSERT INTO historical_consent_events VALUES ('consent','digest','{"id":"consent","packetDigest":"digest","provider":"gemini","model":"model"}','consumed','t','z'); INSERT INTO historical_transmission_events VALUES ('transmission','consent','digest','openai','model','sent','t','z');"#).await.unwrap();
        let result = persist_historical_question_record(
            &mut conn,
            historical_artifact("digest", "openai", "model", serde_json::json!([])),
        )
        .await
        .unwrap();
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM historical_question_artifacts WHERE id='must-not-write'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        assert_eq!(result.status, "stale_generation");
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn historical_persistence_revalidates_reflection_evidence_dependency() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute(r#"CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO persisted_artifacts VALUES ('evidence','entry','evidence','{"status":"rejected"}','t','t'); INSERT INTO persisted_artifacts VALUES ('reflection','entry','reflection','{"status":"answered","response":"User answer","responseProvenance":{"origin":"user"},"sourceEvidenceIds":["evidence"]}','t','t'); PRAGMA user_version = 3;"#).await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        conn.execute(r#"INSERT INTO historical_consent_events VALUES ('consent','digest','{"id":"consent","packetDigest":"digest","provider":"openai","model":"model"}','consumed','t','z'); INSERT INTO historical_transmission_events VALUES ('transmission','consent','digest','openai','model','sent','t','z');"#).await.unwrap();
        let result = persist_historical_question_record(
            &mut conn,
            historical_artifact("digest", "openai", "model", serde_json::json!([
                { "itemType": "reflection_response", "sourceExperienceId": "entry", "artifactId": "reflection", "revision": "t" }
            ])),
        ).await.unwrap();
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM historical_question_artifacts WHERE id='must-not-write'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();
        assert_eq!(result.status, "stale_generation");
        assert_eq!(count, 0);
    }

    fn experience(id: &str, content: &str, updated_at: &str) -> ExperienceRecord {
        ExperienceRecord {
            id: id.into(),
            content: content.into(),
            created_at: "created".into(),
            updated_at: updated_at.into(),
        }
    }

    async fn empty_v4_database(path: &Path) -> SqliteConnection {
        let mut conn = connect(path).await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        assert_eq!(read_schema_version(&mut conn).await.unwrap(), 4);
        conn
    }

    async fn insert_historical_dependency_fixture(conn: &mut SqliteConnection) {
        conn.execute(
            r#"
            INSERT INTO experience_entries VALUES ('source','before','created','revision-1');
            INSERT INTO experience_entries VALUES ('current','current','created','current-revision');
            INSERT INTO persisted_artifacts VALUES ('evidence','source','evidence','{"status":"confirmed"}','created','artifact-revision');
            INSERT INTO historical_consent_events VALUES ('consent','digest','{}','consumed','created','expires');
            INSERT INTO historical_transmission_events VALUES ('transmission','consent','digest','openai','model','sent','created','expires');
            INSERT INTO historical_question_artifacts VALUES ('question','current','digest','{}','{}','consent','transmission','created');
            INSERT INTO historical_artifact_dependencies VALUES ('question','source','evidence','artifact-revision');
            "#,
        )
        .await
        .unwrap();
    }

    async fn count(conn: &mut SqliteConnection, table: &str) -> i64 {
        sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {table}"))
            .fetch_one(&mut *conn)
            .await
            .unwrap()
    }

    async fn assert_database_integrity(conn: &mut SqliteConnection) {
        let foreign_key_errors: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM pragma_foreign_key_check")
                .fetch_one(&mut *conn)
                .await
                .unwrap();
        let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&mut *conn)
            .await
            .unwrap();
        assert_eq!(foreign_key_errors, 0);
        assert_eq!(integrity, "ok");
    }

    fn artifact_bundle(entry_id: &str, artifact_id: &str) -> PersistedArtifactBundleRecord {
        PersistedArtifactBundleRecord {
            evidence: vec![serde_json::json!({
                "id": artifact_id,
                "sourceEntryId": entry_id,
                "text": "confirmed observation",
                "kind": "observation",
                "status": "confirmed",
                "createdAt": "created",
                "updatedAt": "artifact-revision"
            })],
            reflections: vec![],
            patterns: vec![],
            recovery_turns: vec![],
        }
    }

    fn consent(id: &str, digest: &str, state: &str) -> HistoricalConsentRecord {
        HistoricalConsentRecord {
            id: id.into(),
            packet_digest: digest.into(),
            task: "historical_reflection_questions".into(),
            purpose: "invite_user_comparison_without_cross_time_conclusions".into(),
            provider: "openai".into(),
            model: "model".into(),
            source_revisions: vec![],
            state: state.into(),
            created_at: "created".into(),
            expires_at: "expires".into(),
        }
    }

    fn transmission(
        id: &str,
        consent_id: &str,
        digest: &str,
        outcome: &str,
    ) -> HistoricalTransmissionRecord {
        HistoricalTransmissionRecord {
            id: id.into(),
            consent_id: consent_id.into(),
            packet_digest: digest.into(),
            provider: "openai".into(),
            model: "model".into(),
            outcome: outcome.into(),
            created_at: "created".into(),
            expires_at: "expires".into(),
        }
    }

    #[tokio::test]
    async fn typed_create_and_import_preserve_exact_records_and_duplicate_counts() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        let created = experience("created-id", "created body", "created-revision");

        let result = create_experience_record(&mut conn, created.clone())
            .await
            .unwrap();
        let import = import_experience_records(
            &mut conn,
            vec![
                created,
                experience("imported-id", "imported body", "imported-revision"),
            ],
            None,
        )
        .await
        .unwrap();
        let rows: Vec<(String, String, String, String)> = sqlx::query_as(
            "SELECT id, content, created_at, updated_at FROM experience_entries ORDER BY id",
        )
        .fetch_all(&mut conn)
        .await
        .unwrap();

        assert_eq!(result.status, "committed");
        assert_eq!(
            import,
            ExperienceImportResult {
                imported_count: 1,
                skipped_count: 1
            }
        );
        assert_eq!(
            rows,
            vec![
                (
                    "created-id".into(),
                    "created body".into(),
                    "created".into(),
                    "created-revision".into()
                ),
                (
                    "imported-id".into(),
                    "imported body".into(),
                    "created".into(),
                    "imported-revision".into()
                ),
            ]
        );
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_import_failure_rolls_back_the_entire_batch() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;

        let error = import_experience_records(
            &mut conn,
            vec![
                experience("first", "one", "r1"),
                experience("second", "two", "r2"),
            ],
            Some(1),
        )
        .await
        .unwrap_err();

        assert!(error.contains("injected experience import failure"));
        assert_eq!(count(&mut conn, "experience_entries").await, 0);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_update_revalidates_revision_before_invalidating_dependents() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        insert_historical_dependency_fixture(&mut conn).await;

        let result = update_experience_record(
            &mut conn,
            "source".into(),
            "must-not-commit".into(),
            "wrong-revision".into(),
            "revision-2".into(),
        )
        .await
        .unwrap();
        let source: (String, String) = sqlx::query_as(
            "SELECT content, updated_at FROM experience_entries WHERE id = 'source'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();

        assert_eq!(result.status, "stale_generation");
        assert_eq!(source, ("before".into(), "revision-1".into()));
        assert_eq!(count(&mut conn, "persisted_artifacts").await, 1);
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 1);
        assert_eq!(count(&mut conn, "historical_consent_events").await, 1);
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 1);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_update_refuses_a_non_advancing_revision_without_invalidating_dependents() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        insert_historical_dependency_fixture(&mut conn).await;

        let error = update_experience_record(
            &mut conn,
            "source".into(),
            "must-not-commit".into(),
            "revision-1".into(),
            "revision-1".into(),
        )
        .await
        .unwrap_err();
        let source: (String, String) = sqlx::query_as(
            "SELECT content, updated_at FROM experience_entries WHERE id = 'source'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();

        assert!(error.contains("invalid_revision"));
        assert_eq!(source, ("before".into(), "revision-1".into()));
        assert_eq!(count(&mut conn, "persisted_artifacts").await, 1);
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 1);
        assert_eq!(count(&mut conn, "historical_consent_events").await, 1);
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 1);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_update_commits_content_and_existing_provenance_cascades_atomically() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        insert_historical_dependency_fixture(&mut conn).await;

        let result = update_experience_record(
            &mut conn,
            "source".into(),
            "after".into(),
            "revision-1".into(),
            "revision-2".into(),
        )
        .await
        .unwrap();
        let source: (String, String) = sqlx::query_as(
            "SELECT content, updated_at FROM experience_entries WHERE id = 'source'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();

        assert_eq!(result.status, "committed");
        assert_eq!(source, ("after".into(), "revision-2".into()));
        assert_eq!(count(&mut conn, "persisted_artifacts").await, 0);
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 0);
        assert_eq!(count(&mut conn, "historical_consent_events").await, 0);
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 0);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_delete_uses_the_existing_schema_v4_full_cascade() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        insert_historical_dependency_fixture(&mut conn).await;

        let result = delete_experience_record(&mut conn, "source".into())
            .await
            .unwrap();

        assert_eq!(result.status, "committed");
        assert_eq!(count(&mut conn, "experience_entries").await, 1);
        assert_eq!(count(&mut conn, "persisted_artifacts").await, 0);
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 0);
        assert_eq!(count(&mut conn, "historical_consent_events").await, 0);
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 0);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_artifact_bundle_replacement_preserves_v4_cascades_and_rolls_back_on_failure() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        insert_historical_dependency_fixture(&mut conn).await;

        let error = save_artifact_bundle_records(
            &mut conn,
            "source".into(),
            artifact_bundle("source", "replacement"),
            Some("revision-1".into()),
            Some(0),
        )
        .await
        .unwrap_err();
        assert!(error.contains("injected artifact bundle failure"));
        assert_eq!(count(&mut conn, "persisted_artifacts").await, 1);
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 1);
        assert_eq!(count(&mut conn, "historical_consent_events").await, 1);
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 1);

        let result = save_artifact_bundle_records(
            &mut conn,
            "source".into(),
            artifact_bundle("source", "replacement"),
            Some("revision-1".into()),
            None,
        )
        .await
        .unwrap();
        let ids: Vec<String> = sqlx::query_scalar(
            "SELECT id FROM persisted_artifacts WHERE source_entry_id = 'source' ORDER BY id",
        )
        .fetch_all(&mut conn)
        .await
        .unwrap();

        assert_eq!(result.status, "committed");
        assert_eq!(ids, vec!["replacement"]);
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 0);
        assert_eq!(count(&mut conn, "historical_consent_events").await, 0);
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 0);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_consent_is_monotonic_and_cannot_rebind_scope() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;

        save_historical_consent_record(&mut conn, consent("consent", "digest", "granted"))
            .await
            .unwrap();
        save_historical_consent_record(&mut conn, consent("consent", "digest", "invalidated"))
            .await
            .unwrap();
        let error =
            save_historical_consent_record(&mut conn, consent("consent", "different", "granted"))
                .await
                .unwrap_err();
        let row: (String, String) = sqlx::query_as(
            "SELECT packet_digest, state FROM historical_consent_events WHERE id = 'consent'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();

        assert!(error.contains("historical_consent_conflict"));
        assert_eq!(row, ("digest".into(), "invalidated".into()));
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_transmission_and_consent_consumption_are_atomic() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        save_historical_consent_record(&mut conn, consent("consent", "digest", "granted"))
            .await
            .unwrap();

        let error = save_historical_transmission_record(
            &mut conn,
            transmission("transmission", "consent", "digest", "sent"),
            true,
        )
        .await
        .unwrap_err();
        let state_after_failure: String =
            sqlx::query_scalar("SELECT state FROM historical_consent_events WHERE id = 'consent'")
                .fetch_one(&mut conn)
                .await
                .unwrap();
        assert!(error.contains("injected historical transmission failure"));
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 0);
        assert_eq!(state_after_failure, "granted");

        save_historical_transmission_record(
            &mut conn,
            transmission("transmission", "consent", "digest", "sent"),
            false,
        )
        .await
        .unwrap();
        save_historical_transmission_record(
            &mut conn,
            transmission("transmission", "consent", "digest", "failed"),
            false,
        )
        .await
        .unwrap();
        let row: (String, String) = sqlx::query_as(
            "SELECT t.outcome, c.state FROM historical_transmission_events t JOIN historical_consent_events c ON c.id = t.consent_id WHERE t.id = 'transmission'",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();

        assert_eq!(row, ("failed".into(), "consumed".into()));
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_historical_question_derives_dependencies_and_delete_cascades_actual_use() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        conn.execute(
            "INSERT INTO experience_entries VALUES ('current','current','created','current-revision'); INSERT INTO experience_entries VALUES ('source','source','created','source-revision');",
        )
        .await
        .unwrap();
        save_artifact_bundle_records(
            &mut conn,
            "source".into(),
            artifact_bundle("source", "evidence"),
            Some("source-revision".into()),
            None,
        )
        .await
        .unwrap();
        save_historical_consent_record(&mut conn, consent("consent", "digest", "granted"))
            .await
            .unwrap();
        save_historical_transmission_record(
            &mut conn,
            transmission("transmission", "consent", "digest", "sent"),
            false,
        )
        .await
        .unwrap();
        let artifact = HistoricalQuestionArtifactRecord {
            id: "question".into(),
            current_experience_id: "current".into(),
            questions: vec![HistoricalReflectionQuestionRecord {
                id: "q".into(),
                text: "What do you notice?".into(),
                source_experience_ids: vec!["source".into()],
            }],
            packet: serde_json::json!({
                "packetDigest": "digest",
                "currentExperience": { "id": "current", "revision": "current-revision" },
                "task": "historical_reflection_questions",
                "purpose": "invite_user_comparison_without_cross_time_conclusions",
                "destination": { "provider": "openai", "model": "model" },
                "includedItems": [
                    { "itemType": "experience", "sourceExperienceId": "source", "artifactId": null, "revision": "source-revision" },
                    { "itemType": "evidence", "sourceExperienceId": "source", "artifactId": "evidence", "revision": "artifact-revision" }
                ],
                "consent": { "reference": "consent", "scope": "one_generation_one_purpose" }
            }),
            consent_id: "consent".into(),
            transmission_id: "transmission".into(),
            generated_at: "generated".into(),
        };

        let mut invalid_sources = artifact.clone();
        invalid_sources.questions[0].source_experience_ids = vec!["unrelated".into()];
        let error = persist_historical_question_record(&mut conn, invalid_sources)
            .await
            .unwrap_err();
        assert!(error.contains("historical_question_sources_invalid"));
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 0);

        let result = persist_historical_question_record(&mut conn, artifact)
            .await
            .unwrap();
        assert_eq!(result.status, "committed");
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 1);
        assert_eq!(
            count(&mut conn, "historical_artifact_dependencies").await,
            2
        );

        let deleted = delete_historical_question_record(&mut conn, "question".into())
            .await
            .unwrap();
        assert_eq!(deleted.status, "committed");
        assert_eq!(count(&mut conn, "historical_question_artifacts").await, 0);
        assert_eq!(
            count(&mut conn, "historical_artifact_dependencies").await,
            0
        );
        assert_eq!(count(&mut conn, "historical_consent_events").await, 0);
        assert_eq!(count(&mut conn, "historical_transmission_events").await, 0);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn typed_audit_cleanup_removes_only_expired_unreferenced_records() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = empty_v4_database(file.path()).await;
        conn.execute(
            r#"
            INSERT INTO experience_entries VALUES ('current','body','created','revision');
            INSERT INTO historical_consent_events VALUES ('kept-consent','kept','{}','consumed','created','2020');
            INSERT INTO historical_transmission_events VALUES ('kept-transmission','kept-consent','kept','openai','model','sent','created','2020');
            INSERT INTO historical_question_artifacts VALUES ('kept-question','current','kept','{}','{}','kept-consent','kept-transmission','created');
            INSERT INTO historical_consent_events VALUES ('expired-consent','expired','{}','invalidated','created','2020');
            INSERT INTO historical_transmission_events VALUES ('expired-transmission','expired-consent','expired','openai','model','failed','created','2020');
            INSERT INTO historical_consent_events VALUES ('future-consent','future','{}','invalidated','created','2030');
            "#,
        )
        .await
        .unwrap();

        purge_expired_historical_audit_records(&mut conn, "2026".into())
            .await
            .unwrap();
        let consent_ids: Vec<String> =
            sqlx::query_scalar("SELECT id FROM historical_consent_events ORDER BY id")
                .fetch_all(&mut conn)
                .await
                .unwrap();
        let transmission_ids: Vec<String> =
            sqlx::query_scalar("SELECT id FROM historical_transmission_events ORDER BY id")
                .fetch_all(&mut conn)
                .await
                .unwrap();

        assert_eq!(consent_ids, vec!["future-consent", "kept-consent"]);
        assert_eq!(transmission_ids, vec!["kept-transmission"]);
        assert_database_integrity(&mut conn).await;
    }

    #[tokio::test]
    async fn all_typed_experience_mutations_refuse_a_newer_schema_without_writing() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE marker (value TEXT NOT NULL); INSERT INTO marker VALUES ('before'); PRAGMA user_version = 5;")
            .await
            .unwrap();

        assert!(
            create_experience_record(&mut conn, experience("new", "body", "r"))
                .await
                .unwrap_err()
                .contains(NEWER_SCHEMA_ERROR)
        );
        assert!(update_experience_record(
            &mut conn,
            "new".into(),
            "body".into(),
            "r".into(),
            "r2".into()
        )
        .await
        .unwrap_err()
        .contains(NEWER_SCHEMA_ERROR));
        assert!(delete_experience_record(&mut conn, "new".into())
            .await
            .unwrap_err()
            .contains(NEWER_SCHEMA_ERROR));
        assert!(import_experience_records(
            &mut conn,
            vec![experience("import", "body", "r")],
            None
        )
        .await
        .unwrap_err()
        .contains(NEWER_SCHEMA_ERROR));
        let marker: String = sqlx::query_scalar("SELECT value FROM marker")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(marker, "before");
        assert_eq!(read_schema_version(&mut conn).await.unwrap(), 5);
    }

    #[tokio::test]
    async fn all_typed_historical_mutations_refuse_a_newer_schema_without_writing() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE marker (value TEXT NOT NULL); INSERT INTO marker VALUES ('before'); PRAGMA user_version = 5;")
            .await
            .unwrap();

        assert!(
            save_historical_consent_record(&mut conn, consent("consent", "digest", "granted"))
                .await
                .unwrap_err()
                .contains(NEWER_SCHEMA_ERROR)
        );
        assert!(save_historical_transmission_record(
            &mut conn,
            transmission("transmission", "consent", "digest", "sent"),
            false
        )
        .await
        .unwrap_err()
        .contains(NEWER_SCHEMA_ERROR));
        assert!(persist_historical_question_record(
            &mut conn,
            historical_artifact("digest", "openai", "model", serde_json::json!([]))
        )
        .await
        .unwrap_err()
        .contains(NEWER_SCHEMA_ERROR));
        assert!(
            delete_historical_question_record(&mut conn, "question".into())
                .await
                .unwrap_err()
                .contains(NEWER_SCHEMA_ERROR)
        );
        assert!(
            purge_expired_historical_audit_records(&mut conn, "timestamp".into())
                .await
                .unwrap_err()
                .contains(NEWER_SCHEMA_ERROR)
        );

        let marker: String = sqlx::query_scalar("SELECT value FROM marker")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(marker, "before");
        assert_eq!(read_schema_version(&mut conn).await.unwrap(), 5);
    }
}
