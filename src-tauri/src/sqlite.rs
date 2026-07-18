use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Connection, Executor, SqliteConnection};
use std::{path::Path, str::FromStr};
use tauri::{AppHandle, Manager};

const SCHEMA_VERSION: i64 = 4;
const NEWER_SCHEMA_ERROR: &str = "database_schema_newer_than_supported";

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlStatement {
    query: String,
    values: Vec<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedExperienceRevision {
    id: String,
    updated_at: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedArtifactRevision {
    id: String,
    source_entry_id: String,
    updated_at: String,
    artifact_kind: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpectedHistoricalProvenance {
    consent_id: String,
    transmission_id: String,
    packet_digest: String,
    provider: String,
    model: String,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResult {
    status: String,
}

async fn execute_transaction(
    conn: &mut SqliteConnection,
    statements: Vec<SqlStatement>,
    expected_experience_updated_at: Option<String>,
) -> Result<TransactionResult, String> {
    ensure_supported_schema(conn).await?;
    let mut tx = conn.begin().await.map_err(|e| e.to_string())?;
    if let Some(expected) = expected_experience_updated_at {
        let entry_id = statements
            .first()
            .and_then(|statement| statement.values.first())
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                "Expected-version transaction requires entry id as first bind value".to_string()
            })?;
        let current: Option<String> =
            sqlx::query_scalar("SELECT updated_at FROM experience_entries WHERE id = ?")
                .bind(entry_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        if current.as_deref() != Some(expected.as_str()) {
            return Ok(TransactionResult {
                status: "stale_generation".into(),
            });
        }
    }
    for statement in statements {
        let mut query = sqlx::query(&statement.query);
        for value in statement.values {
            query = match value {
                Value::String(v) => query.bind(v),
                Value::Null => query.bind(None::<String>),
                Value::Number(v) if v.is_i64() => query.bind(v.as_i64()),
                Value::Bool(v) => query.bind(v),
                _ => return Err("Unsupported SQLite bind value".into()),
            };
        }
        query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

#[tauri::command]
pub async fn execute_sqlite_transaction(
    app: AppHandle,
    statements: Vec<SqlStatement>,
    expected_experience_updated_at: Option<String>,
) -> Result<TransactionResult, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("life-os.db");
    execute_transaction(
        &mut connect(&path).await?,
        statements,
        expected_experience_updated_at,
    )
    .await
}

async fn execute_historical_transaction(
    conn: &mut SqliteConnection,
    statements: Vec<SqlStatement>,
    expected_revisions: Vec<ExpectedExperienceRevision>,
    expected_artifacts: Vec<ExpectedArtifactRevision>,
    expected_provenance: ExpectedHistoricalProvenance,
) -> Result<TransactionResult, String> {
    ensure_supported_schema(conn).await?;
    let mut tx = conn.begin().await.map_err(|e| e.to_string())?;
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
    for statement in statements {
        let mut query = sqlx::query(&statement.query);
        for value in statement.values {
            query = match value {
                Value::String(v) => query.bind(v),
                Value::Null => query.bind(None::<String>),
                Value::Number(v) if v.is_i64() => query.bind(v.as_i64()),
                Value::Bool(v) => query.bind(v),
                _ => return Err("Unsupported SQLite bind value".into()),
            };
        }
        query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult {
        status: "committed".into(),
    })
}

#[tauri::command]
pub async fn execute_sqlite_historical_transaction(
    app: AppHandle,
    statements: Vec<SqlStatement>,
    expected_revisions: Vec<ExpectedExperienceRevision>,
    expected_artifacts: Vec<ExpectedArtifactRevision>,
    expected_provenance: ExpectedHistoricalProvenance,
) -> Result<TransactionResult, String> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("life-os.db");
    execute_historical_transaction(
        &mut connect(&path).await?,
        statements,
        expected_revisions,
        expected_artifacts,
        expected_provenance,
    )
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

    #[tokio::test]
    async fn generic_transaction_refuses_a_newer_schema_without_writing() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE marker (value TEXT NOT NULL); INSERT INTO marker VALUES ('before'); PRAGMA user_version = 5;")
            .await
            .unwrap();

        let error = match execute_transaction(
            &mut conn,
            vec![SqlStatement {
                query: "UPDATE marker SET value = 'after'".into(),
                values: vec![],
            }],
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
        let result = execute_transaction(
            &mut conn,
            vec![SqlStatement {
                query: "DELETE FROM persisted_artifacts WHERE source_entry_id = $1".into(),
                values: vec![Value::String("entry".into())],
            }],
            Some("wrong-version".into()),
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

    #[tokio::test]
    async fn historical_persistence_revalidates_artifact_eligibility_inside_transaction() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO persisted_artifacts VALUES ('evidence','entry','evidence','{\"status\":\"rejected\"}','t','t'); PRAGMA user_version = 3;").await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        let result = execute_historical_transaction(
            &mut conn,
            vec![SqlStatement { query: "INSERT INTO historical_consent_events (id,packet_digest,payload,state,created_at,expires_at) VALUES ('must-not-write','d','{}','granted','t','z')".into(), values: vec![] }],
            vec![ExpectedExperienceRevision { id: "entry".into(), updated_at: "t".into() }],
            vec![ExpectedArtifactRevision { id: "evidence".into(), source_entry_id: "entry".into(), updated_at: "t".into(), artifact_kind: "evidence".into() }],
            ExpectedHistoricalProvenance { consent_id: "missing-consent".into(), transmission_id: "missing-transmission".into(), packet_digest: "d".into(), provider: "openai".into(), model: "model".into() },
        ).await.unwrap();
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM historical_consent_events WHERE id='must-not-write'",
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
        let result = execute_historical_transaction(
            &mut conn,
            vec![SqlStatement { query: "INSERT INTO historical_question_artifacts (id,current_experience_id,packet_digest,payload,packet_snapshot,consent_id,transmission_id,created_at) VALUES ('must-not-write','entry','wrong','{}','{}','consent','transmission','t')".into(), values: vec![] }],
            vec![ExpectedExperienceRevision { id: "entry".into(), updated_at: "t".into() }],
            vec![],
            ExpectedHistoricalProvenance { consent_id: "consent".into(), transmission_id: "transmission".into(), packet_digest: "wrong".into(), provider: "openai".into(), model: "model".into() },
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
    async fn historical_persistence_rejects_consent_destination_mismatch() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); PRAGMA user_version = 3;").await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        conn.execute(r#"INSERT INTO historical_consent_events VALUES ('consent','digest','{"id":"consent","packetDigest":"digest","provider":"gemini","model":"model"}','consumed','t','z'); INSERT INTO historical_transmission_events VALUES ('transmission','consent','digest','openai','model','sent','t','z');"#).await.unwrap();
        let result = execute_historical_transaction(
            &mut conn,
            vec![SqlStatement { query: "INSERT INTO historical_question_artifacts (id,current_experience_id,packet_digest,payload,packet_snapshot,consent_id,transmission_id,created_at) VALUES ('must-not-write','entry','digest','{}','{}','consent','transmission','t')".into(), values: vec![] }],
            vec![ExpectedExperienceRevision { id: "entry".into(), updated_at: "t".into() }],
            vec![],
            ExpectedHistoricalProvenance { consent_id: "consent".into(), transmission_id: "transmission".into(), packet_digest: "digest".into(), provider: "openai".into(), model: "model".into() },
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
    async fn historical_persistence_revalidates_reflection_evidence_dependency() {
        let file = tempfile::NamedTempFile::new().unwrap();
        let mut conn = connect(file.path()).await.unwrap();
        conn.execute(r#"CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO persisted_artifacts VALUES ('evidence','entry','evidence','{"status":"rejected"}','t','t'); INSERT INTO persisted_artifacts VALUES ('reflection','entry','reflection','{"status":"answered","response":"User answer","responseProvenance":{"origin":"user"},"sourceEvidenceIds":["evidence"]}','t','t'); PRAGMA user_version = 3;"#).await.unwrap();
        migrate_connection(&mut conn, false).await.unwrap();
        conn.execute(r#"INSERT INTO historical_consent_events VALUES ('consent','digest','{"id":"consent","packetDigest":"digest","provider":"openai","model":"model"}','consumed','t','z'); INSERT INTO historical_transmission_events VALUES ('transmission','consent','digest','openai','model','sent','t','z');"#).await.unwrap();
        let result = execute_historical_transaction(
            &mut conn,
            vec![SqlStatement { query: "INSERT INTO historical_question_artifacts (id,current_experience_id,packet_digest,payload,packet_snapshot,consent_id,transmission_id,created_at) VALUES ('must-not-write','entry','digest','{}','{}','consent','transmission','t')".into(), values: vec![] }],
            vec![ExpectedExperienceRevision { id: "entry".into(), updated_at: "t".into() }],
            vec![ExpectedArtifactRevision { id: "reflection".into(), source_entry_id: "entry".into(), updated_at: "t".into(), artifact_kind: "reflection".into() }],
            ExpectedHistoricalProvenance { consent_id: "consent".into(), transmission_id: "transmission".into(), packet_digest: "digest".into(), provider: "openai".into(), model: "model".into() },
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
}
