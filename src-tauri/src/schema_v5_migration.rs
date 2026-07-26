use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{raw_sql, Connection, Row, SqliteConnection};
use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

const APPLICATION_VERSION: &str = "0.2.0";
const SOURCE_SCHEMA_VERSION: i64 = 4;
const TARGET_SCHEMA_VERSION: i64 = 5;
const SCHEMA_V5_DDL: &str = include_str!("../schema/schema_v5.sql");
const EXPECTED_DDL_SHA256: &str =
    "396c06634ab871f892be36280468726cda1777b6bdf18039ea165d6b54e126dc";
const EXPECTED_SCHEMA_OBJECT_MANIFEST_SHA256: &str =
    "bc92f25e12f8a6829c152d4b51fcf741872d94724afaccdb2236d399f5b8abe8";
const PROJECTION_GUARD_MARKER: &str =
    "-- Existing v4/current-state tables become guarded compatibility projections.";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MigrationError {
    pub(crate) code: String,
    pub(crate) recovery_required: bool,
}

impl MigrationError {
    fn fail_closed(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            recovery_required: false,
        }
    }

    fn recovery_required(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            recovery_required: true,
        }
    }
}

impl fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.code)
    }
}

impl std::error::Error for MigrationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MigrationReceipt {
    pub(crate) migration_id: String,
    pub(crate) source_manifest_digest: String,
    pub(crate) target_manifest_digest: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FailurePoint {
    #[default]
    None,
    AfterDdlStatement(usize),
    AfterExperienceBackfill,
    AfterPersistedArtifactBackfill,
    AfterHistoricalQuestionBackfill,
    AfterReconciliation,
    AfterDatabaseContract,
    AfterMigrationReceipt,
    AfterProjectionGuards,
    AfterGuardRemoval,
    AfterVersionMutation,
    PostCommitVerification,
}

#[derive(Clone, Debug)]
pub(crate) struct MigrationRequest<'a> {
    pub(crate) path: &'a Path,
    pub(crate) expected_source_manifest_digest: String,
    pub(crate) started_at: &'a str,
    pub(crate) committed_at: &'a str,
    pub(crate) backup_id: Option<&'a str>,
    pub(crate) failure_point: FailurePoint,
}

#[derive(Clone)]
struct ExperienceRow {
    id: String,
    content: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone)]
struct ArtifactRow {
    id: String,
    source_id: String,
    kind: String,
    payload: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone)]
struct HistoricalQuestionRow {
    id: String,
    current_experience_id: String,
    packet_digest: String,
    payload: String,
    consent_id: String,
    transmission_id: String,
    created_at: String,
}

#[derive(Clone)]
struct HistoricalDependencyRow {
    historical_artifact_id: String,
    source_entry_id: String,
    source_artifact_id: Option<String>,
    source_revision: String,
}

#[derive(Clone, Default)]
struct BackfillStats {
    source_count: i64,
    persisted_artifact_count: i64,
    historical_question_count: i64,
    review_event_count: i64,
    dependency_count: i64,
}

#[derive(Clone)]
struct TableManifestSpec {
    name: &'static str,
    query: &'static str,
    field_count: usize,
}

const SOURCE_TABLE_MANIFESTS: [TableManifestSpec; 6] = [
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

const TARGET_TABLE_MANIFESTS: [TableManifestSpec; 14] = [
    TableManifestSpec {
        name: "provenance_records",
        query: "SELECT id, fingerprint, origin, provider, model, harness_version, prompt_version, generated_at, canonical_payload, created_at FROM provenance_records ORDER BY id",
        field_count: 10,
    },
    TableManifestSpec {
        name: "source_revisions",
        query: "SELECT id, source_id, CAST(revision_number AS TEXT), predecessor_revision_id, authorship, revision_reason, serialization_version, content_digest, created_at FROM source_revisions ORDER BY id",
        field_count: 9,
    },
    TableManifestSpec {
        name: "source_heads",
        query: "SELECT id, current_revision_id, lifecycle_state, created_at, updated_at FROM source_heads ORDER BY id",
        field_count: 5,
    },
    TableManifestSpec {
        name: "source_revision_content",
        query: "SELECT revision_id, content, CAST(byte_length AS TEXT) FROM source_revision_content ORDER BY revision_id",
        field_count: 3,
    },
    TableManifestSpec {
        name: "source_revision_provenance",
        query: "SELECT source_revision_id, role, provenance_id FROM source_revision_provenance ORDER BY source_revision_id, role",
        field_count: 3,
    },
    TableManifestSpec {
        name: "artifact_revisions",
        query: "SELECT id, artifact_id, source_id, CAST(revision_number AS TEXT), predecessor_revision_id, authorship, revision_reason, serialization_version, content_digest, created_at FROM artifact_revisions ORDER BY id",
        field_count: 10,
    },
    TableManifestSpec {
        name: "artifact_heads",
        query: "SELECT id, source_id, artifact_kind, current_revision_id, review_state, lifecycle_state, eligibility_state, eligibility_reason, created_at, updated_at FROM artifact_heads ORDER BY id",
        field_count: 10,
    },
    TableManifestSpec {
        name: "artifact_revision_content",
        query: "SELECT revision_id, payload, CAST(byte_length AS TEXT) FROM artifact_revision_content ORDER BY revision_id",
        field_count: 3,
    },
    TableManifestSpec {
        name: "artifact_revision_provenance",
        query: "SELECT artifact_revision_id, role, provenance_id FROM artifact_revision_provenance ORDER BY artifact_revision_id, role",
        field_count: 3,
    },
    TableManifestSpec {
        name: "artifact_review_events",
        query: "SELECT id, artifact_id, subject_revision_id, decision, actor, event_origin, occurred_at, timestamp_quality FROM artifact_review_events ORDER BY id",
        field_count: 8,
    },
    TableManifestSpec {
        name: "artifact_lifecycle_events",
        query: "SELECT id, artifact_id, subject_revision_id, related_revision_id, dependency_id, event_type, actor, reason_code, occurred_at FROM artifact_lifecycle_events ORDER BY id",
        field_count: 9,
    },
    TableManifestSpec {
        name: "artifact_dependencies",
        query: "SELECT id, dependent_artifact_id, dependent_revision_id, relationship_type, source_revision_id, source_artifact_id, source_artifact_revision_id, created_at FROM artifact_dependencies ORDER BY id",
        field_count: 8,
    },
    TableManifestSpec {
        name: "content_tombstones",
        query: "SELECT id, subject_type, source_id, source_revision_id, artifact_id, artifact_revision_id, content_digest, reason_code, purged_at FROM content_tombstones ORDER BY id",
        field_count: 9,
    },
    TableManifestSpec {
        name: "historical_question_lifecycle_links",
        query: "SELECT historical_artifact_id, artifact_id FROM historical_question_lifecycle_links ORDER BY historical_artifact_id",
        field_count: 2,
    },
];

fn migration_error(context: &str, error: impl fmt::Display) -> MigrationError {
    MigrationError::fail_closed(format!("{context}:{error}"))
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn canonical_file_bytes(text: &str) -> Vec<u8> {
    format!("{}\n", text.replace("\r\n", "\n").trim_end()).into_bytes()
}

fn normalize_schema_sql(sql: &str) -> String {
    sql.replace("\r\n", "\n")
        .trim()
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
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

fn deterministic_id(prefix: &str, domain: &str, fields: &[&str]) -> String {
    let mut framed = Vec::from(domain.as_bytes());
    framed.push(0);
    for field in fields {
        framed.extend_from_slice(&(field.len() as u64).to_be_bytes());
        framed.extend_from_slice(field.as_bytes());
    }
    format!("{prefix}{}", sha256_hex(&framed))
}

fn source_revision_id(source_id: &str, updated_at: &str, digest: &str) -> String {
    deterministic_id(
        "v5sr_",
        "life-os/source-revision-id-v1",
        &[source_id, updated_at, digest],
    )
}

fn artifact_revision_id(artifact_id: &str, kind: &str, updated_at: &str, digest: &str) -> String {
    deterministic_id(
        "v5ar_",
        "life-os/artifact-revision-id-v1",
        &[artifact_id, kind, updated_at, digest],
    )
}

fn event_id(prefix: &str, domain: &str, artifact_id: &str, revision_id: &str) -> String {
    deterministic_id(prefix, domain, &[artifact_id, revision_id])
}

fn dependency_id(
    artifact_id: &str,
    revision_id: &str,
    relationship: &str,
    source_id: &str,
    source_revision_id: &str,
) -> String {
    deterministic_id(
        "v5dp_",
        "life-os/artifact-dependency-id-v1",
        &[
            artifact_id,
            revision_id,
            relationship,
            source_id,
            source_revision_id,
        ],
    )
}

fn register_deterministic_fact(
    registry: &mut BTreeMap<String, String>,
    id: &str,
    fact_digest: &str,
) -> Result<(), MigrationError> {
    if let Some(existing) = registry.get(id) {
        if existing != fact_digest {
            return Err(MigrationError::fail_closed("deterministic_id_collision"));
        }
        return Ok(());
    }
    registry.insert(id.to_owned(), fact_digest.to_owned());
    Ok(())
}

fn canonicalize_json(value: &Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.iter().map(canonicalize_json).collect()),
        Value::Object(object) => {
            let mut sorted = BTreeMap::new();
            for (key, value) in object {
                sorted.insert(key, canonicalize_json(value));
            }
            let mut canonical = Map::new();
            for (key, value) in sorted {
                canonical.insert(key.clone(), value);
            }
            Value::Object(canonical)
        }
        _ => value.clone(),
    }
}

fn canonical_json(value: &Value) -> Result<String, MigrationError> {
    serde_json::to_string(&canonicalize_json(value))
        .map_err(|error| migration_error("canonical_json_failed", error))
}

fn split_fixed_ddl() -> Result<(Vec<&'static str>, Vec<&'static str>), MigrationError> {
    if sha256_hex(&canonical_file_bytes(SCHEMA_V5_DDL)) != EXPECTED_DDL_SHA256 {
        return Err(MigrationError::fail_closed(
            "schema_v5_fixed_ddl_digest_mismatch",
        ));
    }

    let mut statements = Vec::new();
    let mut start = 0;
    let mut offset = 0;
    let mut in_trigger = false;
    for line in SCHEMA_V5_DDL.split_inclusive('\n') {
        let trimmed = line.trim();
        if trimmed.starts_with("CREATE TRIGGER ") {
            in_trigger = true;
        }
        offset += line.len();
        let complete = if in_trigger {
            trimmed == "END;"
        } else {
            trimmed.ends_with(';') && !trimmed.starts_with("--")
        };
        if complete {
            statements.push(&SCHEMA_V5_DDL[start..offset]);
            start = offset;
            in_trigger = false;
        }
    }
    if start < SCHEMA_V5_DDL.len() {
        let tail = &SCHEMA_V5_DDL[start..];
        if !tail.trim().is_empty() {
            return Err(MigrationError::fail_closed(
                "schema_v5_fixed_ddl_unparsed_tail",
            ));
        }
        if let Some(last) = statements.last_mut() {
            *last = &SCHEMA_V5_DDL
                [last.as_ptr() as usize - SCHEMA_V5_DDL.as_ptr() as usize..SCHEMA_V5_DDL.len()];
        }
    }
    if statements.concat() != SCHEMA_V5_DDL {
        return Err(MigrationError::fail_closed(
            "schema_v5_fixed_ddl_reconstruction_failed",
        ));
    }
    let guard_index = statements
        .iter()
        .position(|statement| statement.contains(PROJECTION_GUARD_MARKER))
        .ok_or_else(|| MigrationError::fail_closed("schema_v5_guard_marker_missing"))?;
    Ok((
        statements[..guard_index].to_vec(),
        statements[guard_index..].to_vec(),
    ))
}

async fn connect(path: &Path, read_only: bool) -> Result<SqliteConnection, MigrationError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(false)
        .read_only(read_only)
        .foreign_keys(true);
    SqliteConnection::connect_with(&options)
        .await
        .map_err(|error| migration_error("database_open_failed", error))
}

async fn user_version(connection: &mut SqliteConnection) -> Result<i64, MigrationError> {
    sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("database_version_unreadable", error))
}

async fn manifest(
    connection: &mut SqliteConnection,
    specs: &[TableManifestSpec],
) -> Result<String, MigrationError> {
    let mut digest = Sha256::new();
    for spec in specs {
        frame_bytes(&mut digest, spec.name.as_bytes());
        let rows = sqlx::query(spec.query)
            .fetch_all(&mut *connection)
            .await
            .map_err(|error| migration_error("manifest_query_failed", error))?;
        digest.update((rows.len() as u64).to_be_bytes());
        for row in rows {
            for index in 0..spec.field_count {
                let value = row
                    .try_get::<Option<String>, _>(index)
                    .map_err(|error| migration_error("manifest_decode_failed", error))?;
                frame_optional_text(&mut digest, value.as_deref());
            }
        }
    }
    Ok(format!("{:x}", digest.finalize()))
}

async fn schema_object_manifest(
    connection: &mut SqliteConnection,
) -> Result<String, MigrationError> {
    let rows = sqlx::query(
        "SELECT type, name, sql FROM sqlite_master \
         WHERE type IN ('table','index','trigger') \
           AND name NOT LIKE 'sqlite_%' AND sql IS NOT NULL \
         ORDER BY type, name",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("schema_manifest_query_failed", error))?;
    let framing = rows
        .iter()
        .map(|row| {
            let object_type: String = row.get("type");
            let name: String = row.get("name");
            let sql: String = row.get("sql");
            format!(
                "{object_type}\0{name}\0{}\n",
                sha256_hex(normalize_schema_sql(&sql).as_bytes())
            )
        })
        .collect::<String>();
    Ok(sha256_hex(framing.as_bytes()))
}

async fn integrity_checks(connection: &mut SqliteConnection) -> Result<(), MigrationError> {
    let foreign_keys = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("foreign_key_check_failed", error))?;
    if !foreign_keys.is_empty() {
        return Err(MigrationError::fail_closed(
            "schema_v5_reconciliation_failed:foreign_keys",
        ));
    }
    let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("integrity_check_failed", error))?;
    if integrity != "ok" {
        return Err(MigrationError::fail_closed(
            "schema_v5_reconciliation_failed:integrity",
        ));
    }
    Ok(())
}

async fn current_content_checks(connection: &mut SqliteConnection) -> Result<(), MigrationError> {
    let missing_sources: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM source_heads h \
         WHERE h.current_revision_id IS NOT NULL AND NOT EXISTS (\
           SELECT 1 FROM source_revision_content c \
           WHERE c.revision_id = h.current_revision_id\
         )",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("source_content_check_failed", error))?;
    let missing_artifacts: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_heads h \
         WHERE h.current_revision_id IS NOT NULL AND NOT EXISTS (\
           SELECT 1 FROM artifact_revision_content c \
           WHERE c.revision_id = h.current_revision_id\
         )",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("artifact_content_check_failed", error))?;
    if missing_sources != 0 || missing_artifacts != 0 {
        return Err(MigrationError::fail_closed(
            "schema_v5_reconciliation_failed:current_content",
        ));
    }
    Ok(())
}

async fn load_experiences(
    connection: &mut SqliteConnection,
) -> Result<Vec<ExperienceRow>, MigrationError> {
    let rows = sqlx::query(
        "SELECT id, content, created_at, updated_at \
         FROM experience_entries ORDER BY id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_read_failed", error))?;
    Ok(rows
        .into_iter()
        .map(|row| ExperienceRow {
            id: row.get("id"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect())
}

async fn load_artifacts(
    connection: &mut SqliteConnection,
) -> Result<Vec<ArtifactRow>, MigrationError> {
    let rows = sqlx::query(
        "SELECT id, source_entry_id, artifact_kind, payload, created_at, updated_at \
         FROM persisted_artifacts \
         ORDER BY CASE artifact_kind \
           WHEN 'evidence' THEN 1 \
           WHEN 'reflection' THEN 2 \
           WHEN 'recovery_turn' THEN 3 \
           WHEN 'pattern' THEN 4 \
           ELSE 5 END, id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("artifact_read_failed", error))?;
    Ok(rows
        .into_iter()
        .map(|row| ArtifactRow {
            id: row.get("id"),
            source_id: row.get("source_entry_id"),
            kind: row.get("artifact_kind"),
            payload: row.get("payload"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect())
}

async fn load_historical_questions(
    connection: &mut SqliteConnection,
) -> Result<Vec<HistoricalQuestionRow>, MigrationError> {
    let rows = sqlx::query(
        "SELECT id, current_experience_id, packet_digest, payload, consent_id, transmission_id, created_at \
         FROM historical_question_artifacts ORDER BY id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_question_read_failed", error))?;
    Ok(rows
        .into_iter()
        .map(|row| HistoricalQuestionRow {
            id: row.get("id"),
            current_experience_id: row.get("current_experience_id"),
            packet_digest: row.get("packet_digest"),
            payload: row.get("payload"),
            consent_id: row.get("consent_id"),
            transmission_id: row.get("transmission_id"),
            created_at: row.get("created_at"),
        })
        .collect())
}

async fn load_historical_dependencies(
    connection: &mut SqliteConnection,
) -> Result<Vec<HistoricalDependencyRow>, MigrationError> {
    let rows = sqlx::query(
        "SELECT historical_artifact_id, source_entry_id, source_artifact_id, source_revision \
         FROM historical_artifact_dependencies \
         ORDER BY historical_artifact_id, source_entry_id, source_artifact_id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_dependency_read_failed", error))?;
    Ok(rows
        .into_iter()
        .map(|row| HistoricalDependencyRow {
            historical_artifact_id: row.get("historical_artifact_id"),
            source_entry_id: row.get("source_entry_id"),
            source_artifact_id: row.get("source_artifact_id"),
            source_revision: row.get("source_revision"),
        })
        .collect())
}

fn normalized_provenance(
    provenance: Option<&Value>,
    source_id: &str,
    fallback_origin: &str,
) -> Value {
    let object = provenance.and_then(Value::as_object);
    let origin = object
        .and_then(|value| value.get("origin"))
        .and_then(Value::as_str)
        .unwrap_or(fallback_origin);
    let mut artifact_ids = object
        .and_then(|value| value.get("sourceArtifactIds"))
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    artifact_ids.sort();
    json!({
        "origin": origin,
        "sourceEntryId": object
            .and_then(|value| value.get("sourceEntryId"))
            .and_then(Value::as_str)
            .unwrap_or(source_id),
        "sourceArtifactIds": artifact_ids,
        "provider": object
            .and_then(|value| value.get("provider"))
            .cloned()
            .unwrap_or(Value::Null),
        "model": object
            .and_then(|value| value.get("model"))
            .cloned()
            .unwrap_or(Value::Null),
        "harnessVersion": object
            .and_then(|value| value.get("harnessVersion"))
            .cloned()
            .unwrap_or(Value::Null),
        "promptVersion": object
            .and_then(|value| value.get("promptVersion"))
            .cloned()
            .unwrap_or(Value::Null),
        "generatedAt": object
            .and_then(|value| value.get("generatedAt"))
            .cloned()
            .unwrap_or(Value::Null),
    })
}

async fn insert_provenance(
    connection: &mut SqliteConnection,
    value: &Value,
    created_at: &str,
) -> Result<String, MigrationError> {
    let canonical_payload = canonical_json(value)?;
    let fingerprint = sha256_hex(
        [
            b"life-os/provenance-v1\0".as_slice(),
            canonical_payload.as_bytes(),
        ]
        .concat()
        .as_slice(),
    );
    let id = format!("v5pv_{fingerprint}");
    let origin = value["origin"]
        .as_str()
        .ok_or_else(|| MigrationError::fail_closed("provenance_origin_missing"))?;
    let provider = value["provider"].as_str();
    let model = value["model"].as_str();
    let harness_version = value["harnessVersion"].as_str();
    let prompt_version = value["promptVersion"].as_str();
    let generated_at = value["generatedAt"].as_str();

    sqlx::query(
        "INSERT INTO provenance_records (\
           id, fingerprint, origin, provider, model, harness_version,\
           prompt_version, generated_at, canonical_payload, created_at\
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)\
         ON CONFLICT(fingerprint) DO NOTHING",
    )
    .bind(&id)
    .bind(&fingerprint)
    .bind(origin)
    .bind(provider)
    .bind(model)
    .bind(harness_version)
    .bind(prompt_version)
    .bind(generated_at)
    .bind(&canonical_payload)
    .bind(created_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("provenance_insert_failed", error))?;

    let existing: (String, String) = sqlx::query_as(
        "SELECT id, canonical_payload FROM provenance_records WHERE fingerprint = ?",
    )
    .bind(&fingerprint)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("provenance_collision_check_failed", error))?;
    if existing.0 != id || existing.1 != canonical_payload {
        return Err(MigrationError::fail_closed(
            "provenance_fingerprint_collision",
        ));
    }
    Ok(id)
}

async fn insert_dependency_for_source(
    connection: &mut SqliteConnection,
    dependent_artifact_id: &str,
    dependent_revision_id: &str,
    relationship: &str,
    source_id: &str,
    created_at: &str,
) -> Result<(), MigrationError> {
    let source_revision: Option<(String,)> =
        sqlx::query_as("SELECT current_revision_id FROM source_heads WHERE id = ?")
            .bind(source_id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(|error| migration_error("source_dependency_lookup_failed", error))?;
    let source_revision_id = source_revision
        .map(|value| value.0)
        .ok_or_else(|| MigrationError::fail_closed("source_dependency_unresolved"))?;
    let id = dependency_id(
        dependent_artifact_id,
        dependent_revision_id,
        relationship,
        source_id,
        &source_revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_dependencies (\
           id, dependent_artifact_id, dependent_revision_id, relationship_type,\
           source_revision_id, source_artifact_id, source_artifact_revision_id, created_at\
         ) VALUES (?, ?, ?, ?, ?, NULL, NULL, ?)",
    )
    .bind(id)
    .bind(dependent_artifact_id)
    .bind(dependent_revision_id)
    .bind(relationship)
    .bind(source_revision_id)
    .bind(created_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("source_dependency_insert_failed", error))?;
    Ok(())
}

async fn insert_dependency_for_artifact(
    connection: &mut SqliteConnection,
    dependent_artifact_id: &str,
    dependent_revision_id: &str,
    relationship: &str,
    source_artifact_id: &str,
    created_at: &str,
) -> Result<(), MigrationError> {
    let source: Option<(String,)> =
        sqlx::query_as("SELECT current_revision_id FROM artifact_heads WHERE id = ?")
            .bind(source_artifact_id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(|error| migration_error("artifact_dependency_lookup_failed", error))?;
    let source_revision_id = source
        .map(|value| value.0)
        .ok_or_else(|| MigrationError::fail_closed("artifact_dependency_unresolved"))?;
    let id = dependency_id(
        dependent_artifact_id,
        dependent_revision_id,
        relationship,
        source_artifact_id,
        &source_revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_dependencies (\
           id, dependent_artifact_id, dependent_revision_id, relationship_type,\
           source_revision_id, source_artifact_id, source_artifact_revision_id, created_at\
         ) VALUES (?, ?, ?, ?, NULL, ?, ?, ?)",
    )
    .bind(id)
    .bind(dependent_artifact_id)
    .bind(dependent_revision_id)
    .bind(relationship)
    .bind(source_artifact_id)
    .bind(source_revision_id)
    .bind(created_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("artifact_dependency_insert_failed", error))?;
    Ok(())
}

async fn backfill_experiences(
    connection: &mut SqliteConnection,
    experiences: &[ExperienceRow],
) -> Result<(), MigrationError> {
    let mut generated_ids = BTreeMap::new();
    for row in experiences {
        let digest = sha256_hex(row.content.as_bytes());
        let revision_id = source_revision_id(&row.id, &row.updated_at, &digest);
        let fact_digest = sha256_hex(
            [
                row.id.as_bytes(),
                row.updated_at.as_bytes(),
                digest.as_bytes(),
            ]
            .concat()
            .as_slice(),
        );
        register_deterministic_fact(&mut generated_ids, &revision_id, &fact_digest)?;
        sqlx::query(
            "INSERT INTO source_revisions (\
               id, source_id, revision_number, predecessor_revision_id, authorship,\
               revision_reason, serialization_version, content_digest, created_at\
             ) VALUES (?, ?, 1, NULL, 'user', 'legacy_v4_baseline',\
               'legacy-v4-raw', ?, ?)",
        )
        .bind(&revision_id)
        .bind(&row.id)
        .bind(&digest)
        .bind(&row.created_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("source_revision_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO source_revision_content (revision_id, content, byte_length)\
             VALUES (?, ?, ?)",
        )
        .bind(&revision_id)
        .bind(&row.content)
        .bind(row.content.len() as i64)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("source_content_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO source_heads (id, current_revision_id, lifecycle_state, created_at, updated_at)\
             VALUES (?, ?, 'active', ?, ?)",
        )
        .bind(&row.id)
        .bind(&revision_id)
        .bind(&row.created_at)
        .bind(&row.updated_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("source_head_insert_failed", error))?;

        let provenance = normalized_provenance(None, &row.id, "user");
        let provenance_id = insert_provenance(connection, &provenance, &row.created_at).await?;
        sqlx::query(
            "INSERT INTO source_revision_provenance (source_revision_id, role, provenance_id)\
             VALUES (?, 'content', ?)",
        )
        .bind(&revision_id)
        .bind(provenance_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("source_provenance_link_failed", error))?;
    }
    Ok(())
}

struct ArtifactInterpretation {
    payload: Value,
    review_state: &'static str,
    eligibility_state: &'static str,
    eligibility_reason: &'static str,
    authorship: &'static str,
    review_decision: Option<&'static str>,
}

fn interpret_artifact(row: &ArtifactRow) -> Result<ArtifactInterpretation, MigrationError> {
    if !matches!(
        row.kind.as_str(),
        "evidence" | "reflection" | "pattern" | "recovery_turn"
    ) {
        return Err(MigrationError::fail_closed("unsupported_v4_artifact_kind"));
    }
    let payload: Value = serde_json::from_str(&row.payload)
        .map_err(|error| migration_error("malformed_v4_artifact_payload", error))?;
    let status = payload
        .get("status")
        .and_then(Value::as_str)
        .ok_or_else(|| MigrationError::fail_closed("v4_artifact_status_missing"))?;
    let origin = match row.kind.as_str() {
        "reflection" | "recovery_turn" => payload
            .get("promptProvenance")
            .or_else(|| payload.get("provenance"))
            .and_then(|value| value.get("origin"))
            .and_then(Value::as_str)
            .unwrap_or("legacy_unknown"),
        _ => payload
            .get("provenance")
            .and_then(|value| value.get("origin"))
            .and_then(Value::as_str)
            .unwrap_or("legacy_unknown"),
    };
    let has_user_response = payload
        .get("responseProvenance")
        .and_then(|value| value.get("origin"))
        .and_then(Value::as_str)
        == Some("user");
    let authorship = match row.kind.as_str() {
        "reflection" | "recovery_turn" if status == "answered" && has_user_response => "mixed",
        "reflection" | "recovery_turn" if status == "answered" => "legacy_unknown",
        _ => match origin {
            "user" => "user",
            "ai" => "ai",
            "local_mock" => "local_mock",
            _ => "legacy_unknown",
        },
    };

    let (review_state, eligibility_state, eligibility_reason, review_decision) =
        match (row.kind.as_str(), status) {
            ("evidence" | "pattern", "rejected") => {
                return Err(MigrationError::fail_closed(
                    "unexpected_rejected_v4_payload",
                ))
            }
            ("evidence" | "pattern", "confirmed") => (
                "confirmed",
                "eligible",
                "legacy_v4_confirmed",
                Some("confirmed"),
            ),
            ("evidence" | "pattern", "candidate") => {
                ("pending", "ineligible", "legacy_v4_pending", None)
            }
            ("reflection" | "recovery_turn", "skipped") => (
                "skipped",
                "ineligible",
                "legacy_v4_skipped",
                Some("skipped"),
            ),
            ("reflection" | "recovery_turn", "answered") if has_user_response => (
                "not_applicable",
                "eligible",
                "legacy_v4_user_response",
                None,
            ),
            ("reflection" | "recovery_turn", "answered") => (
                "not_applicable",
                "ineligible",
                "legacy_v4_response_authorship_unknown",
                None,
            ),
            ("reflection" | "recovery_turn", "suggested") => {
                ("pending", "ineligible", "legacy_v4_pending", None)
            }
            _ => return Err(MigrationError::fail_closed("invalid_v4_artifact_status")),
        };

    Ok(ArtifactInterpretation {
        payload,
        review_state,
        eligibility_state,
        eligibility_reason,
        authorship,
        review_decision,
    })
}

async fn link_artifact_provenance(
    connection: &mut SqliteConnection,
    row: &ArtifactRow,
    revision_id: &str,
    interpretation: &ArtifactInterpretation,
) -> Result<(), MigrationError> {
    let roles: Vec<(&str, Option<&Value>, &str)> = match row.kind.as_str() {
        "evidence" | "pattern" => vec![(
            "content",
            interpretation.payload.get("provenance"),
            "legacy_unknown",
        )],
        "reflection" | "recovery_turn" => {
            let mut roles = vec![(
                "prompt",
                interpretation
                    .payload
                    .get("promptProvenance")
                    .or_else(|| interpretation.payload.get("provenance")),
                "legacy_unknown",
            )];
            if let Some(response) = interpretation.payload.get("responseProvenance") {
                roles.push(("response", Some(response), "legacy_unknown"));
            }
            roles
        }
        _ => Vec::new(),
    };
    for (role, raw, fallback) in roles {
        let normalized = normalized_provenance(raw, &row.source_id, fallback);
        let provenance_id = insert_provenance(connection, &normalized, &row.created_at).await?;
        sqlx::query(
            "INSERT INTO artifact_revision_provenance \
             (artifact_revision_id, role, provenance_id) VALUES (?, ?, ?)",
        )
        .bind(revision_id)
        .bind(role)
        .bind(provenance_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("artifact_provenance_link_failed", error))?;
    }
    Ok(())
}

async fn backfill_persisted_artifacts(
    connection: &mut SqliteConnection,
    artifacts: &[ArtifactRow],
    stats: &mut BackfillStats,
) -> Result<(), MigrationError> {
    let mut generated_ids = BTreeMap::new();
    for row in artifacts {
        let interpretation = interpret_artifact(row)?;
        let digest = sha256_hex(row.payload.as_bytes());
        let revision_id = artifact_revision_id(&row.id, &row.kind, &row.updated_at, &digest);
        let fact_digest = sha256_hex(
            [
                row.id.as_bytes(),
                row.kind.as_bytes(),
                row.updated_at.as_bytes(),
                digest.as_bytes(),
            ]
            .concat()
            .as_slice(),
        );
        register_deterministic_fact(&mut generated_ids, &revision_id, &fact_digest)?;
        sqlx::query(
            "INSERT INTO artifact_revisions (\
               id, artifact_id, source_id, revision_number, predecessor_revision_id,\
               authorship, revision_reason, serialization_version, content_digest, created_at\
             ) VALUES (?, ?, ?, 1, NULL, ?, 'legacy_v4_baseline',\
               'legacy-v4-raw', ?, ?)",
        )
        .bind(&revision_id)
        .bind(&row.id)
        .bind(&row.source_id)
        .bind(interpretation.authorship)
        .bind(&digest)
        .bind(&row.created_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("artifact_revision_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO artifact_revision_content (revision_id, payload, byte_length)\
             VALUES (?, ?, ?)",
        )
        .bind(&revision_id)
        .bind(&row.payload)
        .bind(row.payload.len() as i64)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("artifact_content_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO artifact_heads (\
               id, source_id, artifact_kind, current_revision_id, review_state,\
               lifecycle_state, eligibility_state, eligibility_reason, created_at, updated_at\
             ) VALUES (?, ?, ?, ?, ?, 'active', ?, ?, ?, ?)",
        )
        .bind(&row.id)
        .bind(&row.source_id)
        .bind(&row.kind)
        .bind(&revision_id)
        .bind(interpretation.review_state)
        .bind(interpretation.eligibility_state)
        .bind(interpretation.eligibility_reason)
        .bind(&row.created_at)
        .bind(&row.updated_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("artifact_head_insert_failed", error))?;
        link_artifact_provenance(connection, row, &revision_id, &interpretation).await?;
        insert_dependency_for_source(
            connection,
            &row.id,
            &revision_id,
            "derived_from_experience",
            &row.source_id,
            &row.created_at,
        )
        .await?;
        stats.dependency_count += 1;

        let source_evidence_ids = interpretation
            .payload
            .get("sourceEvidenceIds")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for source in source_evidence_ids {
            let source = source
                .as_str()
                .ok_or_else(|| MigrationError::fail_closed("invalid_source_evidence_id"))?;
            insert_dependency_for_artifact(
                connection,
                &row.id,
                &revision_id,
                "uses_evidence",
                source,
                &row.created_at,
            )
            .await?;
            stats.dependency_count += 1;
        }
        let source_reflection_ids = interpretation
            .payload
            .get("sourceReflectionPromptIds")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for source in source_reflection_ids {
            let source = source
                .as_str()
                .ok_or_else(|| MigrationError::fail_closed("invalid_source_reflection_id"))?;
            insert_dependency_for_artifact(
                connection,
                &row.id,
                &revision_id,
                "uses_reflection_response",
                source,
                &row.created_at,
            )
            .await?;
            stats.dependency_count += 1;
        }

        let lifecycle_id = event_id(
            "v5le_",
            "life-os/artifact-lifecycle-event-id-v1",
            &row.id,
            &revision_id,
        );
        sqlx::query(
            "INSERT INTO artifact_lifecycle_events (\
               id, artifact_id, subject_revision_id, related_revision_id, dependency_id,\
               event_type, actor, reason_code, occurred_at\
             ) VALUES (?, ?, ?, NULL, NULL, 'baseline_imported', 'legacy_import',\
               'legacy_v4_baseline', ?)",
        )
        .bind(lifecycle_id)
        .bind(&row.id)
        .bind(&revision_id)
        .bind(&row.updated_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("lifecycle_event_insert_failed", error))?;

        if let Some(decision) = interpretation.review_decision {
            let review_id = event_id(
                "v5re_",
                "life-os/artifact-review-event-id-v1",
                &row.id,
                &revision_id,
            );
            sqlx::query(
                "INSERT INTO artifact_review_events (\
                   id, artifact_id, subject_revision_id, decision, actor, event_origin,\
                   occurred_at, timestamp_quality\
                 ) VALUES (?, ?, ?, ?, 'legacy_import', 'legacy_v4_baseline', ?,\
                   'record_updated_at_not_decision_time')",
            )
            .bind(review_id)
            .bind(&row.id)
            .bind(&revision_id)
            .bind(decision)
            .bind(&row.updated_at)
            .execute(&mut *connection)
            .await
            .map_err(|error| migration_error("review_event_insert_failed", error))?;
            stats.review_event_count += 1;
        }
    }
    Ok(())
}

async fn backfill_historical_questions(
    connection: &mut SqliteConnection,
    questions: &[HistoricalQuestionRow],
    dependencies: &[HistoricalDependencyRow],
    stats: &mut BackfillStats,
) -> Result<(), MigrationError> {
    let mut generated_ids = BTreeMap::new();
    for row in questions {
        let _: Value = serde_json::from_str(&row.payload)
            .map_err(|error| migration_error("malformed_historical_payload", error))?;
        let digest = sha256_hex(row.payload.as_bytes());
        let revision_id =
            artifact_revision_id(&row.id, "historical_question", &row.created_at, &digest);
        let fact_digest = sha256_hex(
            [
                row.id.as_bytes(),
                b"historical_question".as_slice(),
                row.created_at.as_bytes(),
                digest.as_bytes(),
            ]
            .concat()
            .as_slice(),
        );
        register_deterministic_fact(&mut generated_ids, &revision_id, &fact_digest)?;
        sqlx::query(
            "INSERT INTO artifact_revisions (\
               id, artifact_id, source_id, revision_number, predecessor_revision_id,\
               authorship, revision_reason, serialization_version, content_digest, created_at\
             ) VALUES (?, ?, ?, 1, NULL, 'ai', 'legacy_v4_baseline',\
               'legacy-v4-raw', ?, ?)",
        )
        .bind(&revision_id)
        .bind(&row.id)
        .bind(&row.current_experience_id)
        .bind(&digest)
        .bind(&row.created_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_revision_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO artifact_revision_content (revision_id, payload, byte_length)\
             VALUES (?, ?, ?)",
        )
        .bind(&revision_id)
        .bind(&row.payload)
        .bind(row.payload.len() as i64)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_content_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO artifact_heads (\
               id, source_id, artifact_kind, current_revision_id, review_state,\
               lifecycle_state, eligibility_state, eligibility_reason, created_at, updated_at\
             ) VALUES (?, ?, 'historical_question', ?, 'not_applicable', 'active',\
               'ineligible', 'legacy_v4_historical_question', ?, ?)",
        )
        .bind(&row.id)
        .bind(&row.current_experience_id)
        .bind(&revision_id)
        .bind(&row.created_at)
        .bind(&row.created_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_head_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO historical_question_lifecycle_links \
             (historical_artifact_id, artifact_id) VALUES (?, ?)",
        )
        .bind(&row.id)
        .bind(&row.id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_link_insert_failed", error))?;

        let transmission: (String, String) = sqlx::query_as(
            "SELECT t.provider, t.model \
             FROM historical_transmission_events t \
             JOIN historical_consent_events c ON c.id = t.consent_id \
             WHERE t.id = ? AND t.consent_id = ? \
               AND t.outcome = 'sent' AND c.state = 'consumed' \
               AND t.packet_digest = ? AND c.packet_digest = ?",
        )
        .bind(&row.transmission_id)
        .bind(&row.consent_id)
        .bind(&row.packet_digest)
        .bind(&row.packet_digest)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_transmission_unresolved", error))?;
        let provider = match transmission.0.as_str() {
            "openai" | "gemini" | "mock" => transmission.0.as_str(),
            _ => "legacy_unknown",
        };
        let source_artifact_ids = dependencies
            .iter()
            .filter(|dependency| dependency.historical_artifact_id == row.id)
            .filter_map(|dependency| dependency.source_artifact_id.clone())
            .collect::<Vec<_>>();
        let provenance = normalized_provenance(
            Some(&json!({
                "origin": "ai",
                "sourceEntryId": row.current_experience_id,
                "sourceArtifactIds": source_artifact_ids,
                "provider": provider,
                "model": transmission.1,
                "harnessVersion": Value::Null,
                "promptVersion": Value::Null,
                "generatedAt": row.created_at,
            })),
            &row.current_experience_id,
            "ai",
        );
        let provenance_id = insert_provenance(connection, &provenance, &row.created_at).await?;
        sqlx::query(
            "INSERT INTO artifact_revision_provenance \
             (artifact_revision_id, role, provenance_id) VALUES (?, 'content', ?)",
        )
        .bind(&revision_id)
        .bind(provenance_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_provenance_link_failed", error))?;

        insert_dependency_for_source(
            connection,
            &row.id,
            &revision_id,
            "historical_current_experience",
            &row.current_experience_id,
            &row.created_at,
        )
        .await?;
        stats.dependency_count += 1;

        for dependency in dependencies
            .iter()
            .filter(|dependency| dependency.historical_artifact_id == row.id)
        {
            if let Some(source_artifact_id) = &dependency.source_artifact_id {
                let source_state: Option<(String, String)> = sqlx::query_as(
                    "SELECT artifact_kind, eligibility_state \
                     FROM artifact_heads WHERE id = ?",
                )
                .bind(source_artifact_id)
                .fetch_optional(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("historical_artifact_eligibility_lookup_failed", error)
                })?;
                if !matches!(
                    source_state
                        .as_ref()
                        .map(|(kind, state)| (kind.as_str(), state.as_str())),
                    Some(("evidence" | "reflection", "eligible"))
                ) {
                    return Err(MigrationError::fail_closed(
                        "historical_artifact_ineligible",
                    ));
                }
                let actual_updated_at: Option<(String,)> =
                    sqlx::query_as("SELECT updated_at FROM persisted_artifacts WHERE id = ? AND source_entry_id = ?")
                        .bind(source_artifact_id)
                        .bind(&dependency.source_entry_id)
                        .fetch_optional(&mut *connection)
                        .await
                        .map_err(|error| migration_error("historical_artifact_revision_lookup_failed", error))?;
                if actual_updated_at.map(|value| value.0)
                    != Some(dependency.source_revision.clone())
                {
                    return Err(MigrationError::fail_closed(
                        "historical_artifact_revision_mismatch",
                    ));
                }
                insert_dependency_for_artifact(
                    connection,
                    &row.id,
                    &revision_id,
                    "historical_packet_item",
                    source_artifact_id,
                    &row.created_at,
                )
                .await?;
            } else {
                let actual_updated_at: Option<(String,)> =
                    sqlx::query_as("SELECT updated_at FROM experience_entries WHERE id = ?")
                        .bind(&dependency.source_entry_id)
                        .fetch_optional(&mut *connection)
                        .await
                        .map_err(|error| {
                            migration_error("historical_source_revision_lookup_failed", error)
                        })?;
                if actual_updated_at.map(|value| value.0)
                    != Some(dependency.source_revision.clone())
                {
                    return Err(MigrationError::fail_closed(
                        "historical_source_revision_mismatch",
                    ));
                }
                insert_dependency_for_source(
                    connection,
                    &row.id,
                    &revision_id,
                    "historical_packet_item",
                    &dependency.source_entry_id,
                    &row.created_at,
                )
                .await?;
            }
            stats.dependency_count += 1;
        }

        let lifecycle_id = event_id(
            "v5le_",
            "life-os/artifact-lifecycle-event-id-v1",
            &row.id,
            &revision_id,
        );
        sqlx::query(
            "INSERT INTO artifact_lifecycle_events (\
               id, artifact_id, subject_revision_id, related_revision_id, dependency_id,\
               event_type, actor, reason_code, occurred_at\
             ) VALUES (?, ?, ?, NULL, NULL, 'baseline_imported', 'legacy_import',\
               'legacy_v4_baseline', ?)",
        )
        .bind(lifecycle_id)
        .bind(&row.id)
        .bind(&revision_id)
        .bind(&row.created_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_lifecycle_insert_failed", error))?;
    }
    Ok(())
}

async fn reconcile(
    connection: &mut SqliteConnection,
    stats: &BackfillStats,
    source_manifest: &str,
) -> Result<String, MigrationError> {
    let counts = [
        ("source_heads", stats.source_count),
        ("source_revisions", stats.source_count),
        ("source_revision_content", stats.source_count),
        (
            "artifact_heads",
            stats.persisted_artifact_count + stats.historical_question_count,
        ),
        (
            "artifact_revisions",
            stats.persisted_artifact_count + stats.historical_question_count,
        ),
        (
            "artifact_revision_content",
            stats.persisted_artifact_count + stats.historical_question_count,
        ),
        (
            "historical_question_lifecycle_links",
            stats.historical_question_count,
        ),
        ("artifact_review_events", stats.review_event_count),
        ("artifact_dependencies", stats.dependency_count),
    ];
    for (table, expected) in counts {
        let query = format!("SELECT COUNT(*) FROM {table}");
        let actual: i64 = sqlx::query_scalar(&query)
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| migration_error("reconciliation_count_failed", error))?;
        if actual != expected {
            return Err(MigrationError::fail_closed(format!(
                "schema_v5_reconciliation_failed:{table}:{actual}:{expected}"
            )));
        }
    }

    let changed_projection = manifest(connection, &SOURCE_TABLE_MANIFESTS).await?;
    if changed_projection != source_manifest {
        return Err(MigrationError::fail_closed(
            "schema_v5_reconciliation_failed:adr_0009_or_v4_projection_changed",
        ));
    }
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    manifest(connection, &TARGET_TABLE_MANIFESTS).await
}

fn injected(point: FailurePoint, expected: FailurePoint) -> Result<(), MigrationError> {
    if point == expected {
        Err(MigrationError::fail_closed(format!(
            "injected_failure:{expected:?}"
        )))
    } else {
        Ok(())
    }
}

async fn rollback(
    connection: &mut SqliteConnection,
    error: MigrationError,
) -> Result<MigrationReceipt, MigrationError> {
    let _ = raw_sql("ROLLBACK").execute(&mut *connection).await;
    Err(error)
}

pub(crate) async fn migrate_disposable_v4(
    request: MigrationRequest<'_>,
) -> Result<MigrationReceipt, MigrationError> {
    if !request.path.exists() {
        return Err(MigrationError::fail_closed("database_path_missing"));
    }
    let (core_ddl, projection_guards) = split_fixed_ddl()?;
    let mut connection = connect(request.path, false).await?;
    let version = user_version(&mut connection).await?;
    if version != SOURCE_SCHEMA_VERSION {
        return Err(MigrationError::fail_closed(format!(
            "schema_v5_requires_exact_v4:{version}"
        )));
    }

    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("migration_begin_failed", error))?;

    let transaction_result = async {
        if user_version(&mut connection).await? != SOURCE_SCHEMA_VERSION {
            return Err(MigrationError::fail_closed(
                "schema_version_changed_before_migration",
            ));
        }
        raw_sql("PRAGMA defer_foreign_keys = ON")
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("deferred_foreign_keys_failed", error))?;
        let source_manifest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS).await?;
        if source_manifest != request.expected_source_manifest_digest {
            return Err(MigrationError::fail_closed(
                "schema_v5_source_manifest_mismatch",
            ));
        }
        let experiences = load_experiences(&mut connection).await?;
        let artifacts = load_artifacts(&mut connection).await?;
        let questions = load_historical_questions(&mut connection).await?;
        let dependencies = load_historical_dependencies(&mut connection).await?;

        let mut ddl_index = 0;
        for statement in &core_ddl {
            raw_sql(statement)
                .execute(&mut connection)
                .await
                .map_err(|error| migration_error("schema_v5_ddl_failed", error))?;
            injected(
                request.failure_point,
                FailurePoint::AfterDdlStatement(ddl_index),
            )?;
            ddl_index += 1;
        }

        let guard_token = deterministic_id(
            "v5gd_",
            "life-os/schema-v5-migration-guard-v1",
            &[&source_manifest, request.started_at],
        );
        sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
            .bind(&guard_token)
            .bind(request.started_at)
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("migration_guard_insert_failed", error))?;

        backfill_experiences(&mut connection, &experiences).await?;
        injected(request.failure_point, FailurePoint::AfterExperienceBackfill)?;
        let mut stats = BackfillStats {
            source_count: experiences.len() as i64,
            persisted_artifact_count: artifacts.len() as i64,
            historical_question_count: questions.len() as i64,
            ..BackfillStats::default()
        };
        backfill_persisted_artifacts(&mut connection, &artifacts, &mut stats).await?;
        injected(
            request.failure_point,
            FailurePoint::AfterPersistedArtifactBackfill,
        )?;
        backfill_historical_questions(&mut connection, &questions, &dependencies, &mut stats)
            .await?;
        injected(
            request.failure_point,
            FailurePoint::AfterHistoricalQuestionBackfill,
        )?;

        let target_manifest = reconcile(&mut connection, &stats, &source_manifest).await?;
        injected(request.failure_point, FailurePoint::AfterReconciliation)?;

        sqlx::query(
            "INSERT INTO database_contract (\
               singleton, authoritative_schema, minimum_application_version,\
               compatibility_projection, lifecycle_writes, export_v2, updated_at\
             ) VALUES (1, 5, ?, 'enabled', 'disabled', 'disabled', ?)",
        )
        .bind(APPLICATION_VERSION)
        .bind(request.committed_at)
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("database_contract_insert_failed", error))?;
        injected(request.failure_point, FailurePoint::AfterDatabaseContract)?;

        let migration_id = deterministic_id(
            "v5mg_",
            "life-os/schema-v5-migration-receipt-v1",
            &[
                &source_manifest,
                &target_manifest,
                request.started_at,
                request.committed_at,
            ],
        );
        sqlx::query(
            "INSERT INTO schema_migration_receipts (\
               migration_id, from_version, to_version, state, application_version,\
               backup_id, source_manifest_digest, target_manifest_digest,\
               started_at, committed_at\
             ) VALUES (?, 4, 5, 'committed', ?, ?, ?, ?, ?, ?)",
        )
        .bind(&migration_id)
        .bind(APPLICATION_VERSION)
        .bind(request.backup_id)
        .bind(&source_manifest)
        .bind(&target_manifest)
        .bind(request.started_at)
        .bind(request.committed_at)
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("migration_receipt_insert_failed", error))?;
        injected(request.failure_point, FailurePoint::AfterMigrationReceipt)?;

        for statement in &projection_guards {
            raw_sql(statement)
                .execute(&mut connection)
                .await
                .map_err(|error| migration_error("projection_guard_ddl_failed", error))?;
            injected(
                request.failure_point,
                FailurePoint::AfterDdlStatement(ddl_index),
            )?;
            ddl_index += 1;
        }
        injected(request.failure_point, FailurePoint::AfterProjectionGuards)?;

        sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
            .bind(&guard_token)
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("migration_guard_remove_failed", error))?;
        let guard_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
                .fetch_one(&mut connection)
                .await
                .map_err(|error| migration_error("migration_guard_check_failed", error))?;
        if guard_count != 0 {
            return Err(MigrationError::fail_closed(
                "schema_v5_reconciliation_failed:guard_not_empty",
            ));
        }
        injected(request.failure_point, FailurePoint::AfterGuardRemoval)?;
        current_content_checks(&mut connection).await?;
        integrity_checks(&mut connection).await?;

        raw_sql("PRAGMA user_version = 5")
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("schema_version_write_failed", error))?;
        injected(request.failure_point, FailurePoint::AfterVersionMutation)?;
        Ok(MigrationReceipt {
            migration_id,
            source_manifest_digest: source_manifest,
            target_manifest_digest: target_manifest,
        })
    }
    .await;

    let receipt = match transaction_result {
        Ok(receipt) => receipt,
        Err(error) => return rollback(&mut connection, error).await,
    };
    if let Err(error) = raw_sql("COMMIT").execute(&mut connection).await {
        return rollback(
            &mut connection,
            migration_error("migration_commit_failed", error),
        )
        .await;
    }
    drop(connection);

    if request.failure_point == FailurePoint::PostCommitVerification {
        return Err(MigrationError::recovery_required(
            "injected_post_commit_verification_failure",
        ));
    }
    verify_committed_v5(request.path, &receipt).await?;
    Ok(receipt)
}

async fn verify_committed_v5(
    path: &Path,
    expected: &MigrationReceipt,
) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await.map_err(|error| {
        MigrationError::recovery_required(format!("post_commit_open_failed:{}", error.code))
    })?;
    let result = async {
        if user_version(&mut connection).await? != TARGET_SCHEMA_VERSION {
            return Err(MigrationError::recovery_required(
                "post_commit_version_mismatch",
            ));
        }
        let receipt: (String, String, String) = sqlx::query_as(
            "SELECT migration_id, source_manifest_digest, target_manifest_digest \
             FROM schema_migration_receipts WHERE migration_id = ?",
        )
        .bind(&expected.migration_id)
        .fetch_one(&mut connection)
        .await
        .map_err(|error| {
            MigrationError::recovery_required(format!("post_commit_receipt_missing:{error}"))
        })?;
        if receipt
            != (
                expected.migration_id.clone(),
                expected.source_manifest_digest.clone(),
                expected.target_manifest_digest.clone(),
            )
        {
            return Err(MigrationError::recovery_required(
                "post_commit_receipt_mismatch",
            ));
        }
        let contract: (i64, String, String, String, String) = sqlx::query_as(
            "SELECT authoritative_schema, minimum_application_version,\
             compatibility_projection, lifecycle_writes, export_v2 \
             FROM database_contract WHERE singleton = 1",
        )
        .fetch_one(&mut connection)
        .await
        .map_err(|error| {
            MigrationError::recovery_required(format!("post_commit_contract_missing:{error}"))
        })?;
        if contract
            != (
                5,
                APPLICATION_VERSION.to_string(),
                "enabled".to_string(),
                "disabled".to_string(),
                "disabled".to_string(),
            )
        {
            return Err(MigrationError::recovery_required(
                "post_commit_contract_mismatch",
            ));
        }
        let guard_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
                .fetch_one(&mut connection)
                .await
                .map_err(|error| {
                    MigrationError::recovery_required(format!(
                        "post_commit_guard_unreadable:{error}"
                    ))
                })?;
        if guard_count != 0 {
            return Err(MigrationError::recovery_required(
                "post_commit_guard_not_empty",
            ));
        }
        if schema_object_manifest(&mut connection).await? != EXPECTED_SCHEMA_OBJECT_MANIFEST_SHA256
        {
            return Err(MigrationError::recovery_required(
                "post_commit_schema_manifest_mismatch",
            ));
        }
        if manifest(&mut connection, &SOURCE_TABLE_MANIFESTS).await?
            != expected.source_manifest_digest
        {
            return Err(MigrationError::recovery_required(
                "post_commit_source_manifest_mismatch",
            ));
        }
        if manifest(&mut connection, &TARGET_TABLE_MANIFESTS).await?
            != expected.target_manifest_digest
        {
            return Err(MigrationError::recovery_required(
                "post_commit_target_manifest_mismatch",
            ));
        }
        current_content_checks(&mut connection).await?;
        integrity_checks(&mut connection).await?;
        Ok(())
    }
    .await;
    result.map_err(|error| {
        if error.recovery_required {
            error
        } else {
            MigrationError::recovery_required(format!(
                "post_commit_verification_failed:{}",
                error.code
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::raw_sql;
    use std::fs;
    use tempfile::TempDir;

    const V2_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v2.sql");
    const V3_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v3.sql");
    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const CONTRACT: &str = include_str!("../tests/fixtures/schema_v5/contract.json");
    const STARTED_AT: &str = "2026-07-26T00:00:00.000Z";
    const COMMITTED_AT: &str = "2026-07-26T00:00:01.000Z";

    async fn create_fixture(sql: &str) -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(sql).execute(&mut connection).await.unwrap();
        drop(connection);
        (directory, path)
    }

    async fn insert_artifact(
        connection: &mut SqliteConnection,
        id: &str,
        kind: &str,
        payload: &str,
        updated_at: &str,
    ) {
        sqlx::query(
            "INSERT INTO persisted_artifacts \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
             VALUES (?, 'fixture-v4-history', ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(kind)
        .bind(payload)
        .bind(updated_at)
        .bind(updated_at)
        .execute(connection)
        .await
        .unwrap();
    }

    async fn expand_v4(path: &Path) {
        let mut connection = connect(path, false).await.unwrap();
        insert_artifact(
            &mut connection,
            "fixture-evidence-pending",
            "evidence",
            r#"{ "id":"fixture-evidence-pending", "sourceEntryId":"fixture-v4-history", "text":"exact\r\n内容", "kind":"observation", "status":"candidate", "provenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":[],"provider":"mock","model":"fixture","harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:02.000Z"}, "createdAt":"2026-01-01T00:00:02.000Z", "updatedAt":"2026-01-01T00:00:02.000Z" }"#,
            "2026-01-01T00:00:02.000Z",
        )
        .await;
        insert_artifact(
            &mut connection,
            "fixture-reflection-answered",
            "reflection",
            r#"{"id":"fixture-reflection-answered","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"question":"What stayed?","status":"answered","response":"My words","promptProvenance":{"origin":"ai","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence"],"provider":"gemini","model":"fixture","harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:03.000Z"},"responseProvenance":{"origin":"user","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-reflection-answered"],"generatedAt":"2026-01-01T00:00:04.000Z"},"createdAt":"2026-01-01T00:00:03.000Z","updatedAt":"2026-01-01T00:00:04.000Z"}"#,
            "2026-01-01T00:00:04.000Z",
        )
        .await;
        insert_artifact(
            &mut connection,
            "fixture-reflection-skipped",
            "reflection",
            r#"{"id":"fixture-reflection-skipped","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"question":"Skip?","status":"skipped","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence"],"provider":"mock","generatedAt":"2026-01-01T00:00:05.000Z"},"createdAt":"2026-01-01T00:00:05.000Z","updatedAt":"2026-01-01T00:00:05.000Z"}"#,
            "2026-01-01T00:00:05.000Z",
        )
        .await;
        insert_artifact(
            &mut connection,
            "fixture-pattern-confirmed",
            "pattern",
            r#"{"id":"fixture-pattern-confirmed","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"sourceReflectionPromptIds":["fixture-reflection-answered"],"text":"Tentative pattern","status":"confirmed","provenance":{"origin":"ai","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence","fixture-reflection-answered"],"provider":"gemini","model":"fixture","generatedAt":"2026-01-01T00:00:06.000Z"},"createdAt":"2026-01-01T00:00:06.000Z","updatedAt":"2026-01-01T00:00:06.000Z"}"#,
            "2026-01-01T00:00:06.000Z",
        )
        .await;
        insert_artifact(
            &mut connection,
            "fixture-recovery-answered",
            "recovery_turn",
            r#"{"id":"fixture-recovery-answered","sourceEntryId":"fixture-v4-history","question":"What happened?","response":"A meeting.","status":"answered","locale":"en","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":[],"provider":"mock","generatedAt":"2026-01-01T00:00:07.000Z"},"responseProvenance":{"origin":"user","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-recovery-answered"],"generatedAt":"2026-01-01T00:00:08.000Z"},"createdAt":"2026-01-01T00:00:07.000Z","updatedAt":"2026-01-01T00:00:08.000Z"}"#,
            "2026-01-01T00:00:08.000Z",
        )
        .await;
    }

    fn request_with_expected(
        path: &Path,
        failure_point: FailurePoint,
        expected_source_manifest_digest: String,
    ) -> MigrationRequest<'_> {
        MigrationRequest {
            path,
            expected_source_manifest_digest,
            started_at: STARTED_AT,
            committed_at: COMMITTED_AT,
            backup_id: Some("fixture-backup"),
            failure_point,
        }
    }

    async fn request(path: &Path, failure_point: FailurePoint) -> MigrationRequest<'_> {
        let mut connection = connect(path, true).await.unwrap();
        let expected = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        request_with_expected(path, failure_point, expected)
    }

    async fn v4_logical_snapshot(path: &Path) -> (i64, String, Vec<(String, String, String)>) {
        let mut connection = connect(path, true).await.unwrap();
        let version = user_version(&mut connection).await.unwrap();
        let manifest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        let objects = sqlx::query(
            "SELECT type, name, sql FROM sqlite_master \
             WHERE name NOT LIKE 'sqlite_%' AND sql IS NOT NULL ORDER BY type, name",
        )
        .fetch_all(&mut connection)
        .await
        .unwrap()
        .into_iter()
        .map(|row| (row.get("type"), row.get("name"), row.get("sql")))
        .collect();
        (version, manifest, objects)
    }

    #[test]
    fn shared_ddl_matches_the_fixed_contract_and_orders_guards_last() {
        let contract: Value = serde_json::from_str(CONTRACT).unwrap();
        assert_eq!(
            contract["digest_contract"]["candidate_ddl_sha256"],
            EXPECTED_DDL_SHA256
        );
        assert_eq!(
            contract["digest_contract"]["full_schema_object_manifest_sha256"],
            EXPECTED_SCHEMA_OBJECT_MANIFEST_SHA256
        );
        let (core, guards) = split_fixed_ddl().unwrap();
        assert_eq!(
            [core.clone(), guards.clone()].concat().concat(),
            SCHEMA_V5_DDL
        );
        assert!(guards[0].contains(PROJECTION_GUARD_MARKER));
        assert!(guards
            .iter()
            .all(|statement| statement.contains("CREATE TRIGGER")));

        let mut registry = BTreeMap::new();
        register_deterministic_fact(&mut registry, "same-id", "same-fact").unwrap();
        register_deterministic_fact(&mut registry, "same-id", "same-fact").unwrap();
        assert_eq!(
            register_deterministic_fact(&mut registry, "same-id", "different-fact")
                .unwrap_err()
                .code,
            "deterministic_id_collision"
        );
    }

    #[tokio::test]
    async fn exact_v4_migrates_all_baselines_without_changing_raw_or_adr_0009_bytes() {
        let (_directory, path) = create_fixture(V4_FIXTURE).await;
        expand_v4(&path).await;
        let before = v4_logical_snapshot(&path).await;
        let receipt = migrate_disposable_v4(request(&path, FailurePoint::None).await)
            .await
            .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(user_version(&mut connection).await.unwrap(), 5);
        assert_eq!(
            manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
                .await
                .unwrap(),
            before.1
        );
        assert_eq!(
            manifest(&mut connection, &TARGET_TABLE_MANIFESTS)
                .await
                .unwrap(),
            receipt.target_manifest_digest
        );
        let source_serializations: Vec<String> =
            sqlx::query_scalar("SELECT serialization_version FROM source_revisions")
                .fetch_all(&mut connection)
                .await
                .unwrap();
        assert!(
            source_serializations
                .iter()
                .all(|version| version == "legacy-v4-raw"),
            "v4 Experience baselines must retain an honest legacy serialization label"
        );
        let artifact_serializations: Vec<String> =
            sqlx::query_scalar("SELECT serialization_version FROM artifact_revisions")
                .fetch_all(&mut connection)
                .await
                .unwrap();
        assert!(
            artifact_serializations
                .iter()
                .all(|version| version == "legacy-v4-raw"),
            "v4 artifact baselines must retain an honest legacy serialization label"
        );
        let states = sqlx::query(
            "SELECT id, review_state, eligibility_state FROM artifact_heads ORDER BY id",
        )
        .fetch_all(&mut connection)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            (
                row.get::<String, _>("id"),
                row.get::<String, _>("review_state"),
                row.get::<String, _>("eligibility_state"),
            )
        })
        .collect::<Vec<_>>();
        assert!(states.contains(&(
            "fixture-v4-evidence".to_string(),
            "confirmed".to_string(),
            "eligible".to_string()
        )));
        assert!(states.contains(&(
            "fixture-evidence-pending".to_string(),
            "pending".to_string(),
            "ineligible".to_string()
        )));
        assert!(states.contains(&(
            "fixture-reflection-skipped".to_string(),
            "skipped".to_string(),
            "ineligible".to_string()
        )));
        assert!(states.contains(&(
            "fixture-reflection-answered".to_string(),
            "not_applicable".to_string(),
            "eligible".to_string()
        )));
        let original_payload: String = sqlx::query_scalar(
            "SELECT payload FROM persisted_artifacts WHERE id = 'fixture-evidence-pending'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        let migrated_payload: String = sqlx::query_scalar(
            "SELECT c.payload FROM artifact_revision_content c \
             JOIN artifact_heads h ON h.current_revision_id = c.revision_id \
             WHERE h.id = 'fixture-evidence-pending'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(migrated_payload.as_bytes(), original_payload.as_bytes());
        let review_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM artifact_review_events")
            .fetch_one(&mut connection)
            .await
            .unwrap();
        assert_eq!(review_count, 3);
        let guard_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
                .fetch_one(&mut connection)
                .await
                .unwrap();
        assert_eq!(guard_count, 0);
        integrity_checks(&mut connection).await.unwrap();
    }

    #[tokio::test]
    async fn deterministic_ids_and_manifests_repeat_across_independent_fixtures() {
        let (_first_dir, first) = create_fixture(V4_FIXTURE).await;
        let (_second_dir, second) = create_fixture(V4_FIXTURE).await;
        expand_v4(&first).await;
        expand_v4(&second).await;
        let first_receipt = migrate_disposable_v4(request(&first, FailurePoint::None).await)
            .await
            .unwrap();
        let second_receipt = migrate_disposable_v4(request(&second, FailurePoint::None).await)
            .await
            .unwrap();
        assert_eq!(first_receipt, second_receipt);
        let mut first_connection = connect(&first, true).await.unwrap();
        let mut second_connection = connect(&second, true).await.unwrap();
        let first_ids: Vec<String> =
            sqlx::query_scalar("SELECT id FROM source_revisions UNION ALL SELECT id FROM artifact_revisions ORDER BY id")
                .fetch_all(&mut first_connection)
                .await
                .unwrap();
        let second_ids: Vec<String> =
            sqlx::query_scalar("SELECT id FROM source_revisions UNION ALL SELECT id FROM artifact_revisions ORDER BY id")
                .fetch_all(&mut second_connection)
                .await
                .unwrap();
        assert_eq!(first_ids, second_ids);
    }

    #[tokio::test]
    async fn every_ddl_and_transaction_boundary_rolls_back_to_logical_exact_v4() {
        let (core, guards) = split_fixed_ddl().unwrap();
        let mut points = (0..core.len() + guards.len())
            .map(FailurePoint::AfterDdlStatement)
            .collect::<Vec<_>>();
        points.extend([
            FailurePoint::AfterExperienceBackfill,
            FailurePoint::AfterPersistedArtifactBackfill,
            FailurePoint::AfterHistoricalQuestionBackfill,
            FailurePoint::AfterReconciliation,
            FailurePoint::AfterDatabaseContract,
            FailurePoint::AfterMigrationReceipt,
            FailurePoint::AfterProjectionGuards,
            FailurePoint::AfterGuardRemoval,
            FailurePoint::AfterVersionMutation,
        ]);
        for point in points {
            let (_directory, path) = create_fixture(V4_FIXTURE).await;
            expand_v4(&path).await;
            let before = v4_logical_snapshot(&path).await;
            let error = migrate_disposable_v4(request(&path, point).await)
                .await
                .unwrap_err();
            assert!(!error.recovery_required, "{point:?}: {error}");
            let after = v4_logical_snapshot(&path).await;
            assert_eq!(after, before, "{point:?}");
        }
    }

    #[tokio::test]
    async fn malformed_old_existing_and_newer_versions_refuse_without_mutation() {
        for sql in [V2_FIXTURE, V3_FIXTURE] {
            let (_directory, path) = create_fixture(sql).await;
            let before = fs::read(&path).unwrap();
            let error = migrate_disposable_v4(request_with_expected(
                &path,
                FailurePoint::None,
                "0".repeat(64),
            ))
            .await
            .unwrap_err();
            assert!(error.code.contains("requires_exact_v4"));
            assert_eq!(fs::read(&path).unwrap(), before);
        }
        for version in [5_i64, 6_i64] {
            let (_directory, path) = create_fixture(V4_FIXTURE).await;
            let mut connection = connect(&path, false).await.unwrap();
            raw_sql(&format!("PRAGMA user_version = {version}"))
                .execute(&mut connection)
                .await
                .unwrap();
            drop(connection);
            let before = fs::read(&path).unwrap();
            assert!(migrate_disposable_v4(request_with_expected(
                &path,
                FailurePoint::None,
                "0".repeat(64),
            ))
            .await
            .is_err());
            assert_eq!(fs::read(&path).unwrap(), before);
        }
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("malformed.db");
        fs::write(&path, b"not sqlite").unwrap();
        let before = fs::read(&path).unwrap();
        assert!(migrate_disposable_v4(request_with_expected(
            &path,
            FailurePoint::None,
            "0".repeat(64),
        ))
        .await
        .is_err());
        assert_eq!(fs::read(&path).unwrap(), before);
    }

    #[tokio::test]
    async fn rejected_or_stale_dependency_fails_closed_with_v4_unchanged() {
        for mutation in [
            "rejected",
            "stale_dependency",
            "ineligible_dependency",
            "consent_digest_mismatch",
        ] {
            let (_directory, path) = create_fixture(V4_FIXTURE).await;
            let mut connection = connect(&path, false).await.unwrap();
            match mutation {
                "rejected" => {
                    sqlx::query(
                        "UPDATE persisted_artifacts SET payload = ? \
                         WHERE id = 'fixture-v4-evidence'",
                    )
                    .bind(r#"{"status":"rejected"}"#)
                    .execute(&mut connection)
                    .await
                    .unwrap();
                }
                "stale_dependency" => {
                    sqlx::query(
                        "UPDATE historical_artifact_dependencies \
                         SET source_revision = 'stale' \
                         WHERE historical_artifact_id = 'fixture-v4-question'",
                    )
                    .execute(&mut connection)
                    .await
                    .unwrap();
                }
                "ineligible_dependency" => {
                    sqlx::query(
                        "UPDATE persisted_artifacts SET payload = ? \
                         WHERE id = 'fixture-v4-evidence'",
                    )
                    .bind(r#"{"status":"candidate","text":"observed"}"#)
                    .execute(&mut connection)
                    .await
                    .unwrap();
                }
                "consent_digest_mismatch" => {
                    sqlx::query(
                        "UPDATE historical_question_artifacts \
                         SET packet_digest = ? WHERE id = 'fixture-v4-question'",
                    )
                    .bind("b".repeat(64))
                    .execute(&mut connection)
                    .await
                    .unwrap();
                }
                _ => unreachable!(),
            }
            drop(connection);
            let before = v4_logical_snapshot(&path).await;
            assert!(
                migrate_disposable_v4(request(&path, FailurePoint::None).await)
                    .await
                    .is_err()
            );
            assert_eq!(v4_logical_snapshot(&path).await, before);
        }
    }

    #[tokio::test]
    async fn source_payload_or_timestamp_change_after_manifest_capture_is_refused() {
        for column in ["content", "updated_at"] {
            let (_directory, path) = create_fixture(V4_FIXTURE).await;
            let mut connection = connect(&path, true).await.unwrap();
            let expected = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
                .await
                .unwrap();
            drop(connection);

            let mut connection = connect(&path, false).await.unwrap();
            let statement = format!(
                "UPDATE experience_entries SET {column} = ? \
                 WHERE id = 'fixture-v4-history'"
            );
            sqlx::query(&statement)
                .bind(if column == "content" {
                    "changed after disclosure"
                } else {
                    "2026-01-01T09:00:00.000Z"
                })
                .execute(&mut connection)
                .await
                .unwrap();
            drop(connection);
            let changed_v4 = v4_logical_snapshot(&path).await;

            let error =
                migrate_disposable_v4(request_with_expected(&path, FailurePoint::None, expected))
                    .await
                    .unwrap_err();
            assert_eq!(error.code, "schema_v5_source_manifest_mismatch");
            assert_eq!(v4_logical_snapshot(&path).await, changed_v4);
        }
    }

    #[tokio::test]
    async fn post_commit_verification_failure_is_recovery_required_without_repair() {
        let (_directory, path) = create_fixture(V4_FIXTURE).await;
        let error =
            migrate_disposable_v4(request(&path, FailurePoint::PostCommitVerification).await)
                .await
                .unwrap_err();
        assert!(error.recovery_required);
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(user_version(&mut connection).await.unwrap(), 5);
        let receipt_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM schema_migration_receipts")
                .fetch_one(&mut connection)
                .await
                .unwrap();
        assert_eq!(receipt_count, 1);
    }

    #[tokio::test]
    async fn actual_post_commit_schema_drift_is_blocked_without_automatic_action() {
        let (_directory, path) = create_fixture(V4_FIXTURE).await;
        let receipt = migrate_disposable_v4(request(&path, FailurePoint::None).await)
            .await
            .unwrap();
        let mut connection = connect(&path, false).await.unwrap();
        raw_sql("DROP TRIGGER migration_receipts_immutable_update")
            .execute(&mut connection)
            .await
            .unwrap();
        drop(connection);

        let error = verify_committed_v5(&path, &receipt).await.unwrap_err();
        assert!(error.recovery_required);
        assert!(error.code.contains("schema_manifest_mismatch"));
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(user_version(&mut connection).await.unwrap(), 5);
        let trigger_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master \
             WHERE type = 'trigger' AND name = 'migration_receipts_immutable_update'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            trigger_count, 0,
            "verification must not repair schema drift"
        );
    }

    #[tokio::test]
    async fn receipt_is_immutable_and_lifecycle_export_remain_disabled() {
        let (_directory, path) = create_fixture(V4_FIXTURE).await;
        migrate_disposable_v4(request(&path, FailurePoint::None).await)
            .await
            .unwrap();
        let mut connection = connect(&path, false).await.unwrap();
        for sql in [
            "UPDATE schema_migration_receipts SET state = 'committed'",
            "DELETE FROM schema_migration_receipts",
        ] {
            let error = raw_sql(sql)
                .execute(&mut connection)
                .await
                .unwrap_err()
                .to_string();
            assert!(error.contains("schema_migration_receipt_immutable"));
        }
        let flags: (String, String) =
            sqlx::query_as("SELECT lifecycle_writes, export_v2 FROM database_contract")
                .fetch_one(&mut connection)
                .await
                .unwrap();
        assert_eq!(flags, ("disabled".to_string(), "disabled".to_string()));
    }
}
