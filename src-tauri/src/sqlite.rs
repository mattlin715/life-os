use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Connection, Executor, Row, SqliteConnection};
use std::{path::Path, str::FromStr};
use tauri::{AppHandle, Manager};

const SCHEMA_VERSION: i64 = 3;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlStatement { query: String, values: Vec<Value> }

async fn connect(path: &Path) -> Result<SqliteConnection, String> {
    let options = sqlx::sqlite::SqliteConnectOptions::from_str(&format!("sqlite://{}", path.display()))
        .map_err(|e| e.to_string())?.create_if_missing(true).foreign_keys(true);
    SqliteConnection::connect_with(&options).await.map_err(|e| e.to_string())
}

pub async fn migrate_connection(conn: &mut SqliteConnection, inject_failure: bool) -> Result<(), String> {
    conn.execute("PRAGMA foreign_keys = ON").await.map_err(|e| e.to_string())?;
    conn.execute("CREATE TABLE IF NOT EXISTS experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)").await.map_err(|e| e.to_string())?;
    let version: i64 = sqlx::query("PRAGMA user_version").fetch_one(&mut *conn).await.map_err(|e| e.to_string())?.get(0);
    if version >= SCHEMA_VERSION { return Ok(()); }
    let mut tx = conn.begin().await.map_err(|e| e.to_string())?;
    tx.execute("CREATE TABLE IF NOT EXISTS persisted_artifacts (id TEXT PRIMARY KEY NOT NULL, source_entry_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, payload TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE)").await.map_err(|e| e.to_string())?;
    tx.execute("CREATE INDEX IF NOT EXISTS idx_persisted_artifacts_source ON persisted_artifacts(source_entry_id, artifact_kind, created_at)").await.map_err(|e| e.to_string())?;
    for (table, kind) in [("evidence_candidates", "evidence"), ("reflection_prompts", "reflection"), ("pattern_notes", "pattern")] {
        let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?").bind(table).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        if exists > 0 {
            let query = format!("INSERT INTO persisted_artifacts (id, source_entry_id, artifact_kind, payload, created_at, updated_at) SELECT id, source_entry_id, '{kind}', payload, created_at, updated_at FROM {table} WHERE status != 'rejected'");
            tx.execute(query.as_str()).await.map_err(|e| e.to_string())?;
        }
    }
    if inject_failure { return Err("injected migration failure".into()); }
    tx.execute("PRAGMA user_version = 3").await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn initialize_sqlite_database(app: AppHandle) -> Result<(), String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    migrate_connection(&mut connect(&dir.join("life-os.db")).await?, false).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResult { status: String }

async fn execute_transaction(conn: &mut SqliteConnection, statements: Vec<SqlStatement>, expected_experience_updated_at: Option<String>) -> Result<TransactionResult, String> {
    let mut tx = conn.begin().await.map_err(|e| e.to_string())?;
    if let Some(expected) = expected_experience_updated_at {
        let entry_id = statements.first().and_then(|statement| statement.values.first()).and_then(|value| value.as_str()).ok_or_else(|| "Expected-version transaction requires entry id as first bind value".to_string())?;
        let current: Option<String> = sqlx::query_scalar("SELECT updated_at FROM experience_entries WHERE id = ?").bind(entry_id).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;
        if current.as_deref() != Some(expected.as_str()) { return Ok(TransactionResult { status: "stale_generation".into() }); }
    }
    for statement in statements {
        let mut query = sqlx::query(&statement.query);
        for value in statement.values {
            query = match value { Value::String(v) => query.bind(v), Value::Null => query.bind(None::<String>), Value::Number(v) if v.is_i64() => query.bind(v.as_i64()), Value::Bool(v) => query.bind(v), _ => return Err("Unsupported SQLite bind value".into()) };
        }
        query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(TransactionResult { status: "committed".into() })
}

#[tauri::command]
pub async fn execute_sqlite_transaction(app: AppHandle, statements: Vec<SqlStatement>, expected_experience_updated_at: Option<String>) -> Result<TransactionResult, String> {
    let path = app.path().app_data_dir().map_err(|e| e.to_string())?.join("life-os.db");
    execute_transaction(&mut connect(&path).await?, statements, expected_experience_updated_at).await
}

#[cfg(test)]
mod tests {
    use super::*;
    async fn fixture(path: &Path) -> SqliteConnection {
        let mut conn = connect(path).await.unwrap();
        conn.execute("CREATE TABLE experience_entries (id TEXT PRIMARY KEY NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL); INSERT INTO experience_entries VALUES ('entry','body','t','t'); CREATE TABLE evidence_candidates (id TEXT PRIMARY KEY, source_entry_id TEXT, status TEXT, payload TEXT, created_at TEXT, updated_at TEXT); INSERT INTO evidence_candidates VALUES ('keep','entry','confirmed','{\"id\":\"keep\"}','t','t'); INSERT INTO evidence_candidates VALUES ('drop','entry','rejected','{\"id\":\"drop\"}','t','t'); PRAGMA user_version = 2;").await.unwrap();
        conn
    }
    #[tokio::test]
    async fn migrates_v2_and_excludes_rejected() {
        let file = tempfile::NamedTempFile::new().unwrap(); let mut conn = fixture(file.path()).await;
        migrate_connection(&mut conn, false).await.unwrap();
        let version: i64 = sqlx::query_scalar("PRAGMA user_version").fetch_one(&mut conn).await.unwrap();
        let ids: Vec<String> = sqlx::query_scalar("SELECT id FROM persisted_artifacts ORDER BY id").fetch_all(&mut conn).await.unwrap();
        assert_eq!(version, 3); assert_eq!(ids, vec!["keep"]);
    }
    #[tokio::test]
    async fn injected_failure_rolls_back_and_keeps_version() {
        let file = tempfile::NamedTempFile::new().unwrap(); let mut conn = fixture(file.path()).await;
        assert!(migrate_connection(&mut conn, true).await.is_err());
        let version: i64 = sqlx::query_scalar("PRAGMA user_version").fetch_one(&mut conn).await.unwrap();
        let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='persisted_artifacts'").fetch_one(&mut conn).await.unwrap();
        let legacy: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM evidence_candidates").fetch_one(&mut conn).await.unwrap();
        assert_eq!(version, 2); assert_eq!(exists, 0); assert_eq!(legacy, 2);
    }    #[tokio::test]
    async fn wrong_expected_version_keeps_existing_artifacts() {
        let file = tempfile::NamedTempFile::new().unwrap(); let mut conn = fixture(file.path()).await;
        migrate_connection(&mut conn, false).await.unwrap();
        let result = execute_transaction(&mut conn, vec![SqlStatement { query: "DELETE FROM persisted_artifacts WHERE source_entry_id = $1".into(), values: vec![Value::String("entry".into())] }], Some("wrong-version".into())).await.unwrap();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM persisted_artifacts").fetch_one(&mut conn).await.unwrap();
        assert_eq!(result.status, "stale_generation"); assert_eq!(count, 1);
    }

}
