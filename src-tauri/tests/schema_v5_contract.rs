use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use sqlx::{raw_sql, Connection, Executor, Row, SqliteConnection};
use std::collections::{BTreeMap, BTreeSet};

const V2_FIXTURE: &str = include_str!("fixtures/schema_v5/v2.sql");
const V3_FIXTURE: &str = include_str!("fixtures/schema_v5/v3.sql");
const V4_FIXTURE: &str = include_str!("fixtures/schema_v5/v4.sql");
const SCHEMA_V5_DDL: &str = include_str!("fixtures/schema_v5/schema_v5.sql");
const CONTRACT: &str = include_str!("fixtures/schema_v5/contract.json");
const ARCHITECTURE_13: &str =
    include_str!("../../docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md");

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

fn contract() -> Value {
    serde_json::from_str(CONTRACT).expect("Slice 0 contract must be valid JSON")
}

async fn in_memory_connection() -> SqliteConnection {
    let mut conn = SqliteConnection::connect("sqlite::memory:")
        .await
        .expect("in-memory SQLite must open");
    conn.execute("PRAGMA foreign_keys = ON")
        .await
        .expect("foreign keys must be enabled");
    conn
}

async fn apply(conn: &mut SqliteConnection, sql: &str) {
    raw_sql(sql)
        .execute(&mut *conn)
        .await
        .expect("fixed fixture SQL must execute");
}

async fn v4_with_candidate_schema() -> SqliteConnection {
    let mut conn = in_memory_connection().await;
    apply(&mut conn, V4_FIXTURE).await;
    apply(&mut conn, SCHEMA_V5_DDL).await;
    conn
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

fn deterministic_id(prefix: &str, domain: &str, fields: &[Value]) -> String {
    let mut framed = Vec::from(domain.as_bytes());
    framed.push(0);
    for field in fields {
        let field = field
            .as_str()
            .expect("ID vector fields must be strings")
            .as_bytes();
        framed.extend_from_slice(&(field.len() as u64).to_be_bytes());
        framed.extend_from_slice(field);
    }
    format!("{prefix}{}", sha256_hex(&framed))
}

async fn expect_sql_error(conn: &mut SqliteConnection, statement: &str, expected: &str) {
    let error = sqlx::query(statement)
        .execute(&mut *conn)
        .await
        .expect_err("statement must fail closed")
        .to_string();
    assert!(
        error.contains(expected),
        "expected error containing {expected:?}, got {error:?} for {statement}"
    );
}

async fn insert_source_baseline(conn: &mut SqliteConnection, source_id: &str, revision_id: &str) {
    let sql = format!(
        "BEGIN;\n\
         INSERT INTO source_revisions VALUES (\
           '{revision_id}', '{source_id}', 1, NULL, 'legacy_unknown',\
           'legacy_v4_baseline', 'legacy-v4-raw', '{digest}',\
           '2026-01-01T00:00:00.000Z'\
         );\n\
         INSERT INTO source_revision_content VALUES (\
           '{revision_id}', 'fixture content', 15\
         );\n\
         INSERT INTO source_heads VALUES (\
           '{source_id}', '{revision_id}', 'active',\
           '2026-01-01T00:00:00.000Z', '2026-01-01T00:00:00.000Z'\
         );\n\
         COMMIT;",
        digest = "0".repeat(64),
    );
    apply(conn, &sql).await;
}

async fn insert_artifact_baseline(
    conn: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    revision_id: &str,
) {
    let sql = format!(
        "BEGIN;\n\
         INSERT INTO artifact_revisions VALUES (\
           '{revision_id}', '{artifact_id}', '{source_id}', 1, NULL,\
           'legacy_unknown', 'legacy_v4_baseline', 'legacy-v4-raw',\
           '{digest}', '2026-01-01T00:00:01.000Z'\
         );\n\
         INSERT INTO artifact_revision_content VALUES (\
           '{revision_id}', '{{\"text\":\"fixture\"}}', 18\
         );\n\
         INSERT INTO artifact_heads VALUES (\
           '{artifact_id}', '{source_id}', 'historical_question', '{revision_id}',\
           'not_applicable', 'active', 'ineligible', 'legacy_v4_baseline',\
           '2026-01-01T00:00:01.000Z', '2026-01-01T00:00:01.000Z'\
         );\n\
         COMMIT;",
        digest = "1".repeat(64),
    );
    apply(conn, &sql).await;
}

#[tokio::test]
async fn fixed_contract_freezes_ddl_objects_canonicalization_and_ids() {
    let contract = contract();
    let digest_contract = &contract["digest_contract"];
    assert_eq!(contract["contract_version"], "phase-3c-schema-v5-slice0-v1");
    let architecture = ARCHITECTURE_13.replace("\r\n", "\n");
    let approved_ddl = architecture
        .split_once("```sql\n")
        .and_then(|(_, rest)| rest.split_once("\n```").map(|(sql, _)| sql))
        .expect("architecture/13 must retain its approved SQL fence");
    assert_eq!(
        canonical_file_bytes(approved_ddl),
        canonical_file_bytes(SCHEMA_V5_DDL),
        "the fixed SQL fixture must remain byte-aligned with architecture/13"
    );
    assert_eq!(
        sha256_hex(&canonical_file_bytes(SCHEMA_V5_DDL)),
        digest_contract["candidate_ddl_sha256"].as_str().unwrap()
    );

    let actual_error_codes = SCHEMA_V5_DDL
        .split("RAISE(ABORT, '")
        .skip(1)
        .map(|fragment| fragment.split_once('\'').unwrap().0.to_owned())
        .collect::<BTreeSet<_>>();
    let expected_error_codes = contract["ddl_error_codes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_error_codes, expected_error_codes);

    for vector in contract["canonicalization_vectors"].as_array().unwrap() {
        let input: Value = serde_json::from_str(vector["input"].as_str().unwrap()).unwrap();
        let canonical = serde_json::to_string(&canonicalize_json(&input)).unwrap();
        assert_eq!(canonical, vector["canonical"].as_str().unwrap());
        assert_eq!(
            sha256_hex(canonical.as_bytes()),
            vector["sha256"].as_str().unwrap()
        );
    }

    for vector in contract["deterministic_id_vectors"].as_array().unwrap() {
        let kind = vector["kind"].as_str().unwrap();
        let definition = &contract["deterministic_ids"][kind];
        let actual = deterministic_id(
            definition["prefix"].as_str().unwrap(),
            definition["domain"].as_str().unwrap(),
            vector["fields"].as_array().unwrap(),
        );
        assert_eq!(actual, vector["expected"].as_str().unwrap());
    }

    let mut conn = v4_with_candidate_schema().await;
    let rows = sqlx::query(
        "SELECT type, name, sql FROM sqlite_master \
         WHERE type IN ('table','index','trigger') \
           AND name NOT LIKE 'sqlite_%' AND sql IS NOT NULL \
         ORDER BY type, name",
    )
    .fetch_all(&mut conn)
    .await
    .unwrap();

    let actual = rows
        .iter()
        .map(|row| {
            let object_type: String = row.get("type");
            let name: String = row.get("name");
            let sql: String = row.get("sql");
            (
                object_type,
                name,
                sha256_hex(normalize_schema_sql(&sql).as_bytes()),
            )
        })
        .collect::<Vec<_>>();
    let expected = contract["full_schema_objects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|object| {
            (
                object["type"].as_str().unwrap().to_owned(),
                object["name"].as_str().unwrap().to_owned(),
                object["body_sha256"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        actual, expected,
        "every fixed object name and body must match"
    );

    let framing = actual
        .iter()
        .map(|(object_type, name, body)| format!("{object_type}\0{name}\0{body}\n"))
        .collect::<String>();
    assert_eq!(
        sha256_hex(framing.as_bytes()),
        digest_contract["full_schema_object_manifest_sha256"]
            .as_str()
            .unwrap()
    );

    let candidate_names = contract["candidate_objects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|object| {
            (
                object["type"].as_str().unwrap().to_owned(),
                object["name"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    for object_type in ["table", "index", "trigger"] {
        let count = candidate_names
            .iter()
            .filter(|(kind, _)| kind == object_type)
            .count() as u64;
        assert_eq!(
            count,
            contract["candidate_object_counts"][object_type]
                .as_u64()
                .unwrap()
        );
    }

    let version: i64 = sqlx::query_scalar("PRAGMA user_version")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(version, 4, "test-only DDL must not activate schema v5");
}

#[tokio::test]
async fn synthetic_v2_v3_v4_fixtures_are_fixed_and_only_v4_is_ddl_ready() {
    let contract = contract();
    for (name, fixture, expected_table) in [
        ("v2", V2_FIXTURE, "evidence_candidates"),
        ("v3", V3_FIXTURE, "persisted_artifacts"),
        ("v4", V4_FIXTURE, "historical_question_artifacts"),
    ] {
        assert_eq!(
            sha256_hex(&canonical_file_bytes(fixture)),
            contract["fixtures"][name]["sha256"].as_str().unwrap()
        );
        let mut conn = in_memory_connection().await;
        apply(&mut conn, fixture).await;
        let version: i64 = sqlx::query_scalar("PRAGMA user_version")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(
            version,
            contract["fixtures"][name]["user_version"].as_i64().unwrap()
        );
        let exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?",
        )
        .bind(expected_table)
        .fetch_one(&mut conn)
        .await
        .unwrap();
        assert_eq!(exists, 1);
    }

    for fixture in [V2_FIXTURE, V3_FIXTURE] {
        let mut conn = in_memory_connection().await;
        apply(&mut conn, fixture).await;
        assert!(
            raw_sql(SCHEMA_V5_DDL).execute(&mut conn).await.is_err(),
            "older fixtures must stabilize to v4 before candidate DDL"
        );
    }

    let mut conn = v4_with_candidate_schema().await;
    let foreign_key_errors = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    assert!(foreign_key_errors.is_empty());
    let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(integrity, "ok");
}

#[tokio::test]
async fn current_revision_content_invariants_fail_closed() {
    let mut conn = v4_with_candidate_schema().await;
    insert_source_baseline(&mut conn, "source-1", "source-revision-1").await;
    insert_artifact_baseline(&mut conn, "source-1", "artifact-1", "artifact-revision-1").await;

    apply(
        &mut conn,
        &format!(
            "INSERT INTO source_revisions VALUES (\
             'source-revision-2', 'source-1', 2, 'source-revision-1', 'user',\
             'corrected', 'utf8-text-v1', '{}', '2026-01-02T00:00:00.000Z'\
             );",
            "2".repeat(64)
        ),
    )
    .await;
    expect_sql_error(
        &mut conn,
        "UPDATE source_heads SET current_revision_id = 'source-revision-2' WHERE id = 'source-1'",
        "current_source_revision_content_missing",
    )
    .await;

    apply(
        &mut conn,
        &format!(
            "INSERT INTO artifact_revisions VALUES (\
             'artifact-revision-2', 'artifact-1', 'source-1', 2, 'artifact-revision-1',\
             'user', 'corrected', 'canonical-json-v1', '{}',\
             '2026-01-02T00:00:01.000Z'\
             );",
            "3".repeat(64)
        ),
    )
    .await;
    expect_sql_error(
        &mut conn,
        "UPDATE artifact_heads SET current_revision_id = 'artifact-revision-2' WHERE id = 'artifact-1'",
        "current_artifact_revision_content_missing",
    )
    .await;

    let source_insert_error = raw_sql(&format!(
        "BEGIN;\n\
         INSERT INTO source_revisions VALUES (\
           'missing-source-revision', 'missing-source', 1, NULL, 'user', 'created',\
           'utf8-text-v1', '{}', '2026-01-03T00:00:00.000Z'\
         );\n\
         INSERT INTO source_heads VALUES (\
           'missing-source', 'missing-source-revision', 'active',\
           '2026-01-03T00:00:00.000Z', '2026-01-03T00:00:00.000Z'\
         );\n\
         COMMIT;",
        "4".repeat(64)
    ))
    .execute(&mut conn)
    .await
    .expect_err("source head without content must fail")
    .to_string();
    assert!(source_insert_error.contains("current_source_revision_content_missing"));
    let _ = conn.execute("ROLLBACK").await;

    let artifact_insert_error = raw_sql(&format!(
        "BEGIN;\n\
         INSERT INTO artifact_revisions VALUES (\
           'missing-artifact-revision', 'missing-artifact', 'source-1', 1, NULL,\
           'user', 'created', 'canonical-json-v1', '{}',\
           '2026-01-03T00:00:01.000Z'\
         );\n\
         INSERT INTO artifact_heads VALUES (\
           'missing-artifact', 'source-1', 'evidence', 'missing-artifact-revision',\
           'pending', 'active', 'ineligible', 'missing_content',\
           '2026-01-03T00:00:01.000Z', '2026-01-03T00:00:01.000Z'\
         );\n\
         COMMIT;",
        "5".repeat(64)
    ))
    .execute(&mut conn)
    .await
    .expect_err("artifact head without content must fail")
    .to_string();
    assert!(artifact_insert_error.contains("current_artifact_revision_content_missing"));
    let _ = conn.execute("ROLLBACK").await;

    apply(
        &mut conn,
        "INSERT INTO v5_compatibility_write_guard VALUES ('purge-guard','2026-01-04T00:00:00.000Z');",
    )
    .await;
    expect_sql_error(
        &mut conn,
        "DELETE FROM source_revision_content WHERE revision_id = 'source-revision-1'",
        "cannot_purge_current_source_revision",
    )
    .await;
    expect_sql_error(
        &mut conn,
        "DELETE FROM artifact_revision_content WHERE revision_id = 'artifact-revision-1'",
        "cannot_purge_current_artifact_revision",
    )
    .await;
    apply(
        &mut conn,
        "DELETE FROM v5_compatibility_write_guard WHERE token = 'purge-guard';",
    )
    .await;
}

#[tokio::test]
async fn guarded_non_current_purge_preserves_revision_metadata() {
    let mut conn = v4_with_candidate_schema().await;
    insert_source_baseline(&mut conn, "source-purge", "source-purge-revision-1").await;
    insert_artifact_baseline(
        &mut conn,
        "source-purge",
        "artifact-purge",
        "artifact-purge-revision-1",
    )
    .await;

    apply(
        &mut conn,
        &format!(
            r#"BEGIN;
               INSERT INTO source_revisions VALUES (
                 'source-purge-revision-2', 'source-purge', 2,
                 'source-purge-revision-1', 'user', 'corrected', 'utf8-text-v1',
                 '{source_digest}', '2026-01-08T00:00:00.000Z'
               );
               INSERT INTO source_revision_content VALUES (
                 'source-purge-revision-2', 'corrected source', 16
               );
               UPDATE source_heads SET
                 current_revision_id='source-purge-revision-2',
                 updated_at='2026-01-08T00:00:00.000Z'
               WHERE id='source-purge';
               INSERT INTO artifact_revisions VALUES (
                 'artifact-purge-revision-2', 'artifact-purge', 'source-purge', 2,
                 'artifact-purge-revision-1', 'user', 'corrected',
                 'canonical-json-v1', '{artifact_digest}',
                 '2026-01-08T00:00:01.000Z'
               );
               INSERT INTO artifact_revision_content VALUES (
                 'artifact-purge-revision-2', '{{"text":"corrected"}}', 20
               );
               UPDATE artifact_heads SET
                 current_revision_id='artifact-purge-revision-2',
                 review_state='pending', eligibility_state='ineligible',
                 eligibility_reason='corrected_pending_review',
                 updated_at='2026-01-08T00:00:01.000Z'
               WHERE id='artifact-purge';
               COMMIT;"#,
            source_digest = "7".repeat(64),
            artifact_digest = "8".repeat(64),
        ),
    )
    .await;

    apply(
        &mut conn,
        r#"BEGIN;
           INSERT INTO v5_compatibility_write_guard VALUES (
             'non-current-purge-guard','2026-01-08T00:00:02.000Z'
           );
           DELETE FROM source_revision_content
             WHERE revision_id='source-purge-revision-1';
           DELETE FROM artifact_revision_content
             WHERE revision_id='artifact-purge-revision-1';
           DELETE FROM v5_compatibility_write_guard
             WHERE token='non-current-purge-guard';
           COMMIT;"#,
    )
    .await;

    for (table, id) in [
        ("source_revisions", "source-purge-revision-1"),
        ("artifact_revisions", "artifact-purge-revision-1"),
    ] {
        let statement = format!("SELECT COUNT(*) FROM {table} WHERE id = ?");
        let count: i64 = sqlx::query_scalar(&statement)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(count, 1, "{table} metadata must survive content purge");
    }
    for (table, id) in [
        ("source_revision_content", "source-purge-revision-1"),
        ("artifact_revision_content", "artifact-purge-revision-1"),
    ] {
        let statement = format!("SELECT COUNT(*) FROM {table} WHERE revision_id = ?");
        let count: i64 = sqlx::query_scalar(&statement)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(count, 0, "{table} must remove only non-current content");
    }

    let foreign_key_errors = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    assert!(foreign_key_errors.is_empty());
    let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(integrity, "ok");
}

#[tokio::test]
async fn audit_dependency_and_tombstone_facts_are_immutable_and_guarded() {
    let mut conn = v4_with_candidate_schema().await;
    insert_source_baseline(&mut conn, "source-audit", "source-audit-revision").await;
    insert_artifact_baseline(
        &mut conn,
        "source-audit",
        "artifact-audit",
        "artifact-audit-revision",
    )
    .await;
    apply(
        &mut conn,
        &format!(
            "INSERT INTO artifact_review_events VALUES (\
               'review-audit','artifact-audit','artifact-audit-revision',\
               'confirmed','legacy_import','legacy_v4_baseline',\
               '2026-01-09T00:00:00.000Z','record_updated_at_not_decision_time'\
             );\n\
             INSERT INTO artifact_lifecycle_events VALUES (\
               'lifecycle-audit','artifact-audit','artifact-audit-revision',\
               NULL,NULL,'baseline_imported','legacy_import','legacy_v4_baseline',\
               '2026-01-09T00:00:00.000Z'\
             );\n\
             INSERT INTO artifact_dependencies VALUES (\
               'dependency-audit','artifact-audit','artifact-audit-revision',\
               'derived_from_experience','source-audit-revision',NULL,NULL,\
               '2026-01-09T00:00:00.000Z'\
             );\n\
             INSERT INTO content_tombstones VALUES (\
               'tombstone-audit','artifact_revision',NULL,NULL,\
               'artifact-audit','artifact-audit-revision','{digest}',\
               'rejected_content_purged','2026-01-09T00:00:01.000Z'\
             );",
            digest = "9".repeat(64),
        ),
    )
    .await;

    for (statement, error) in [
        (
            "UPDATE artifact_review_events SET occurred_at='changed' WHERE id='review-audit'",
            "review_event_immutable",
        ),
        (
            "UPDATE artifact_lifecycle_events SET occurred_at='changed' WHERE id='lifecycle-audit'",
            "lifecycle_event_immutable",
        ),
        (
            "UPDATE artifact_dependencies SET created_at='changed' WHERE id='dependency-audit'",
            "dependency_immutable",
        ),
        (
            "UPDATE content_tombstones SET purged_at='changed' WHERE id='tombstone-audit'",
            "content_tombstone_immutable",
        ),
        (
            "DELETE FROM artifact_review_events WHERE id='review-audit'",
            "review_event_delete_requires_lifecycle_transaction",
        ),
        (
            "DELETE FROM artifact_lifecycle_events WHERE id='lifecycle-audit'",
            "lifecycle_event_delete_requires_lifecycle_transaction",
        ),
        (
            "DELETE FROM artifact_dependencies WHERE id='dependency-audit'",
            "dependency_delete_requires_lifecycle_transaction",
        ),
        (
            "DELETE FROM content_tombstones WHERE id='tombstone-audit'",
            "tombstone_delete_requires_lifecycle_transaction",
        ),
    ] {
        expect_sql_error(&mut conn, statement, error).await;
    }

    apply(
        &mut conn,
        "BEGIN;\
         INSERT INTO v5_compatibility_write_guard VALUES (\
           'audit-delete-guard','2026-01-09T00:00:02.000Z'\
         );\
         DELETE FROM artifact_review_events WHERE id='review-audit';\
         DELETE FROM artifact_lifecycle_events WHERE id='lifecycle-audit';\
         DELETE FROM artifact_dependencies WHERE id='dependency-audit';\
         DELETE FROM content_tombstones WHERE id='tombstone-audit';\
         DELETE FROM v5_compatibility_write_guard WHERE token='audit-delete-guard';\
         COMMIT;",
    )
    .await;

    for table in [
        "artifact_review_events",
        "artifact_lifecycle_events",
        "artifact_dependencies",
        "content_tombstones",
    ] {
        let statement = format!("SELECT COUNT(*) FROM {table}");
        let count: i64 = sqlx::query_scalar(&statement)
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(count, 0, "{table} guarded delete must remove the test fact");
    }
    let guard_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(guard_count, 0);
    let foreign_key_errors = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    assert!(foreign_key_errors.is_empty());
    let integrity: String = sqlx::query_scalar("PRAGMA integrity_check")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(integrity, "ok");
}

#[tokio::test]
async fn immutable_records_and_lifecycle_deletes_require_the_guard() {
    let mut conn = v4_with_candidate_schema().await;
    insert_source_baseline(&mut conn, "source-immutable", "source-revision-immutable").await;
    insert_artifact_baseline(
        &mut conn,
        "source-immutable",
        "artifact-immutable",
        "artifact-revision-immutable",
    )
    .await;
    apply(
        &mut conn,
        &format!(
            "INSERT INTO schema_migration_receipts VALUES (\
              'fixture-receipt',4,5,'committed','0.2.0',NULL,'{digest}','{digest}',\
              '2026-01-01T00:00:00.000Z','2026-01-01T00:00:01.000Z'\
            );\n\
            INSERT INTO provenance_records VALUES (\
              'fixture-provenance','{digest}','legacy_unknown',NULL,NULL,NULL,NULL,NULL,\
              '{{}}','2026-01-01T00:00:00.000Z'\
            );\n\
            INSERT INTO source_revision_provenance VALUES (\
              'source-revision-immutable','content','fixture-provenance'\
            );\n\
            INSERT INTO artifact_revision_provenance VALUES (\
              'artifact-revision-immutable','content','fixture-provenance'\
            );",
            digest = "6".repeat(64)
        ),
    )
    .await;

    for (statement, error) in [
        (
            "UPDATE schema_migration_receipts SET application_version='9' WHERE migration_id='fixture-receipt'",
            "schema_migration_receipt_immutable",
        ),
        (
            "DELETE FROM schema_migration_receipts WHERE migration_id='fixture-receipt'",
            "schema_migration_receipt_immutable",
        ),
        (
            "UPDATE provenance_records SET model='changed' WHERE id='fixture-provenance'",
            "provenance_immutable",
        ),
        (
            "UPDATE source_revision_provenance SET provenance_id='changed' WHERE source_revision_id='source-revision-immutable'",
            "source_revision_provenance_immutable",
        ),
        (
            "UPDATE artifact_revision_provenance SET provenance_id='changed' WHERE artifact_revision_id='artifact-revision-immutable'",
            "artifact_revision_provenance_immutable",
        ),
        (
            "DELETE FROM source_revision_provenance WHERE source_revision_id='source-revision-immutable'",
            "source_revision_provenance_delete_requires_lifecycle_transaction",
        ),
        (
            "DELETE FROM artifact_revision_provenance WHERE artifact_revision_id='artifact-revision-immutable'",
            "artifact_revision_provenance_delete_requires_lifecycle_transaction",
        ),
    ] {
        expect_sql_error(&mut conn, statement, error).await;
    }

    apply(
        &mut conn,
        "INSERT INTO v5_compatibility_write_guard VALUES ('delete-guard','2026-01-05T00:00:00.000Z');\
         DELETE FROM source_revision_provenance WHERE source_revision_id='source-revision-immutable';\
         DELETE FROM artifact_revision_provenance WHERE artifact_revision_id='artifact-revision-immutable';",
    )
    .await;
    expect_sql_error(
        &mut conn,
        "UPDATE v5_compatibility_write_guard SET token='changed' WHERE token='delete-guard'",
        "compatibility_guard_token_immutable",
    )
    .await;
    apply(
        &mut conn,
        "DELETE FROM v5_compatibility_write_guard WHERE token='delete-guard';",
    )
    .await;
}

#[tokio::test]
async fn all_v4_projection_mutations_fail_closed_without_a_guard() {
    let mut conn = v4_with_candidate_schema().await;
    let statements = [
        "INSERT INTO experience_entries VALUES ('guard-new-experience','x','2026-01-01','2026-01-01')",
        "UPDATE experience_entries SET content='x' WHERE id='fixture-v4-current'",
        "DELETE FROM experience_entries WHERE id='fixture-v4-history'",
        "INSERT INTO persisted_artifacts VALUES ('guard-new-artifact','fixture-v4-current','evidence','{}','2026-01-01','2026-01-01')",
        "UPDATE persisted_artifacts SET payload='{}' WHERE id='fixture-v4-evidence'",
        "DELETE FROM persisted_artifacts WHERE id='fixture-v4-evidence'",
        "INSERT INTO historical_consent_events VALUES ('guard-new-consent','a','{}','granted','2026-01-01','2026-01-02')",
        "UPDATE historical_consent_events SET payload='{}' WHERE id='fixture-v4-consent'",
        "DELETE FROM historical_consent_events WHERE id='fixture-v4-consent'",
        "INSERT INTO historical_transmission_events VALUES ('guard-new-transmission','fixture-v4-consent','a','gemini','m','sent','2026-01-01','2026-01-02')",
        "UPDATE historical_transmission_events SET model='m' WHERE id='fixture-v4-transmission'",
        "DELETE FROM historical_transmission_events WHERE id='fixture-v4-transmission'",
        "INSERT INTO historical_question_artifacts VALUES ('guard-new-question','fixture-v4-current','a','{}','{}','fixture-v4-consent','fixture-v4-transmission','2026-01-01')",
        "UPDATE historical_question_artifacts SET payload='{}' WHERE id='fixture-v4-question'",
        "DELETE FROM historical_question_artifacts WHERE id='fixture-v4-question'",
        "INSERT INTO historical_artifact_dependencies VALUES ('fixture-v4-question','fixture-v4-current','guard-new-source-artifact','2026-01-01')",
        "UPDATE historical_artifact_dependencies SET source_revision='changed' WHERE historical_artifact_id='fixture-v4-question'",
        "DELETE FROM historical_artifact_dependencies WHERE historical_artifact_id='fixture-v4-question'",
    ];
    for statement in statements {
        expect_sql_error(
            &mut conn,
            statement,
            "schema_v5_requires_compatible_application",
        )
        .await;
    }

    apply(
        &mut conn,
        "BEGIN;\
         INSERT INTO v5_compatibility_write_guard VALUES ('projection-guard','2026-01-06T00:00:00.000Z');\
         INSERT INTO database_contract VALUES (\
           1,5,'0.2.0','enabled','disabled','disabled','2026-01-06T00:00:00.000Z'\
         );\
         UPDATE experience_entries SET content='guarded' WHERE id='fixture-v4-current';\
         DELETE FROM v5_compatibility_write_guard WHERE token='projection-guard';\
         COMMIT;",
    )
    .await;
    let content: String =
        sqlx::query_scalar("SELECT content FROM experience_entries WHERE id='fixture-v4-current'")
            .fetch_one(&mut conn)
            .await
            .unwrap();
    assert_eq!(content, "guarded");
    let guard_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(guard_count, 0);
    for statement in [
        "INSERT INTO database_contract VALUES (1,5,'0.2.0','enabled','disabled','disabled','2026-01-06T00:00:00.000Z')",
        "UPDATE database_contract SET updated_at='changed' WHERE singleton=1",
        "DELETE FROM database_contract WHERE singleton=1",
    ] {
        expect_sql_error(
            &mut conn,
            statement,
            "database_contract_requires_lifecycle_transaction",
        )
        .await;
    }

    apply(
        &mut conn,
        "BEGIN; INSERT INTO v5_compatibility_write_guard VALUES ('rollback-guard','2026-01-06T00:00:01.000Z'); ROLLBACK;",
    )
    .await;
    let guard_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(guard_count, 0);
}

#[tokio::test]
async fn guarded_historical_delete_preserves_the_adr_0009_cascade() {
    let mut conn = v4_with_candidate_schema().await;
    insert_source_baseline(
        &mut conn,
        "fixture-v4-current",
        "fixture-v4-current-revision",
    )
    .await;
    insert_artifact_baseline(
        &mut conn,
        "fixture-v4-current",
        "fixture-v5-question-head",
        "fixture-v5-question-revision",
    )
    .await;
    apply(
        &mut conn,
        "INSERT INTO historical_question_lifecycle_links VALUES (\
           'fixture-v4-question','fixture-v5-question-head'\
         );",
    )
    .await;

    expect_sql_error(
        &mut conn,
        "DELETE FROM historical_question_lifecycle_links WHERE historical_artifact_id='fixture-v4-question'",
        "historical_lifecycle_link_delete_requires_lifecycle_transaction",
    )
    .await;
    expect_sql_error(
        &mut conn,
        "UPDATE historical_question_lifecycle_links SET artifact_id='changed' WHERE historical_artifact_id='fixture-v4-question'",
        "historical_lifecycle_link_immutable",
    )
    .await;

    apply(
        &mut conn,
        "BEGIN;\
         INSERT INTO v5_compatibility_write_guard VALUES ('historical-guard','2026-01-07T00:00:00.000Z');\
         DELETE FROM historical_question_artifacts WHERE id='fixture-v4-question';\
         DELETE FROM v5_compatibility_write_guard WHERE token='historical-guard';\
         COMMIT;",
    )
    .await;
    for (table, id_column, id) in [
        ("historical_question_artifacts", "id", "fixture-v4-question"),
        ("historical_consent_events", "id", "fixture-v4-consent"),
        (
            "historical_transmission_events",
            "id",
            "fixture-v4-transmission",
        ),
        ("artifact_heads", "id", "fixture-v5-question-head"),
        (
            "historical_question_lifecycle_links",
            "historical_artifact_id",
            "fixture-v4-question",
        ),
    ] {
        let statement = format!("SELECT COUNT(*) FROM {table} WHERE {id_column} = ?");
        let count: i64 = sqlx::query_scalar(&statement)
            .bind(id)
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(count, 0, "{table} must participate in the governed cascade");
    }
    let foreign_key_errors = sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    assert!(foreign_key_errors.is_empty());
}
