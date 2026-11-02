//! Real runtime replay of payloads captured from the product coordinator/adapter.
use crate::schema_v5_fresh_base::EMPTY_V4_BASE_SCHEMA;
use crate::schema_v5_migration::{
    activate_lifecycle_writes, migrate_disposable_v4, runtime, source_manifest_for_path,
    FailurePoint, MigrationRequest,
};
use serde_json::{json, Value};
use sqlx::{sqlite::SqliteConnectOptions, Connection};
use std::path::{Path, PathBuf};

fn captured() -> Value {
    serde_json::from_str(include_str!(
        "../fixtures/schema_v5/reflection_projection_ordering.json"
    ))
    .unwrap()
}
fn preserve(dir: &Path, name: &str, value: &Value) {
    let path = dir.join(name);
    assert!(!path.exists());
    std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    println!("ORDERING_TRACE {name} {value}");
}
async fn fixture(label: &str) -> (PathBuf, PathBuf) {
    let data = captured();
    let at = data["source"]["createdAt"].as_str().unwrap();
    let id = data["source"]["id"].as_str().unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".artifacts/reflection-projection-ordering-r1/run-001");
    std::fs::create_dir_all(&root).unwrap();
    let dir = tempfile::Builder::new()
        .prefix(label)
        .tempdir_in(root)
        .unwrap()
        .keep();
    println!("PRESERVED_ORDERING_CASE {}", dir.display());
    let path = dir.join("life-os.db");
    let mut c = sqlx::SqliteConnection::connect_with(
        &SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true),
    )
    .await
    .unwrap();
    sqlx::raw_sql(EMPTY_V4_BASE_SCHEMA)
        .execute(&mut c)
        .await
        .unwrap();
    c.close().await.unwrap();
    let receipt = migrate_disposable_v4(MigrationRequest {
        path: &path,
        expected_source_manifest_digest: source_manifest_for_path(&path).await.unwrap(),
        started_at: at,
        committed_at: at,
        backup_id: None,
        failure_point: FailurePoint::None,
    })
    .await
    .unwrap();
    activate_lifecycle_writes(&path, &receipt, at)
        .await
        .unwrap();
    runtime::create_experience(
        &path,
        id,
        data["source"]["body"].as_str().unwrap(),
        at,
        "ordering-source-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    for (n, ev) in data["confirmedEvidence"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let mut bundle = runtime::list_artifacts(&path, id).await.unwrap();
        let mut candidate = ev.clone();
        candidate["status"] = json!("candidate");
        candidate["updatedAt"] = json!(at);
        bundle["evidence"].as_array_mut().unwrap().push(candidate);
        runtime::save_artifacts(
            &path,
            id,
            bundle,
            Some(at),
            at,
            &format!("ordering-evidence-{n}-0123456789abcdef0123456789abcdef"),
        )
        .await
        .unwrap();
        let mut bundle = runtime::list_artifacts(&path, id).await.unwrap();
        *bundle["evidence"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|x| x["id"] == ev["id"])
            .unwrap() = ev.clone();
        runtime::save_artifacts(
            &path,
            id,
            bundle,
            Some(at),
            ev["updatedAt"].as_str().unwrap(),
            &format!("ordering-confirm-{n}-0123456789abcdef0123456789abcdef"),
        )
        .await
        .unwrap();
    }
    (dir, path)
}
async fn replay(name: &str) {
    let data = captured();
    let (dir, path) = fixture(&format!("replay-{name}-")).await;
    let case = &data[name];
    let request = &case["request"];
    let id = request["entryId"].as_str().unwrap();
    let at = request["expectedExperienceUpdatedAt"].as_str().unwrap();
    if name == "append" {
        runtime::save_artifacts(
            &path,
            id,
            case["before"].clone(),
            Some(at),
            request["occurredAt"].as_str().unwrap(),
            "ordering-existing-M-0123456789abcdef0123456789abcdef",
        )
        .await
        .unwrap();
    }
    assert_eq!(
        runtime::list_artifacts(&path, id).await.unwrap(),
        case["before"]
    );
    preserve(&dir, "submitted.json", request);
    let result = runtime::save_artifacts(
        &path,
        id,
        request["bundle"].clone(),
        Some(at),
        request["occurredAt"].as_str().unwrap(),
        "ordering-batch-0123456789abcdef0123456789abcdef",
    )
    .await;
    let reopened = runtime::list_artifacts(&path, id).await.unwrap();
    let error = result.as_ref().err().map(|e| e.code.clone());
    preserve(
        &dir,
        "result-and-reopened.json",
        &json!({"status":result.as_ref().ok().map(|r|r.status),"error":error,"reopened":reopened}),
    );
    let ids: Vec<_> = reopened["reflections"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        if name == "first" {
            vec!["ordering-question-A", "ordering-question-Z"]
        } else {
            vec![
                "ordering-question-A",
                "ordering-question-M",
                "ordering-question-Z",
            ]
        }
    );
    if let Some(code) = &error {
        assert_eq!(code, "founder_runtime_projection_postcondition_mismatch");
    }
    assert!(result.is_ok(),"valid product multi-question payload must return committed, got {error:?}; DB preserved at {}",dir.display());
    assert_eq!(result.unwrap().bundle, reopened);
}
#[tokio::test]
async fn same_time_reverse_ids_first_product_batch() {
    replay("first").await;
}
#[tokio::test]
async fn same_time_reverse_ids_append_product_batch() {
    replay("append").await;
}

async fn with_existing(label: &str) -> (PathBuf, PathBuf) {
    let (dir, path) = fixture(label).await;
    let data = captured();
    runtime::save_artifacts(
        &path,
        "ordering-source",
        data["append"]["before"].clone(),
        Some("2026-10-31T00:00:00.000Z"),
        "2026-10-31T00:00:02.000Z",
        "ordering-existing-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    (dir, path)
}
async fn submit(
    path: &Path,
    bundle: Value,
) -> Result<runtime::SaveArtifactsResult, crate::schema_v5_migration::MigrationError> {
    runtime::save_artifacts(
        path,
        "ordering-source",
        bundle,
        Some("2026-10-31T00:00:00.000Z"),
        "2026-10-31T00:00:02.000Z",
        "ordering-regression-0123456789abcdef0123456789abcdef",
    )
    .await
}
fn ids(bundle: &Value) -> Vec<String> {
    bundle["reflections"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["id"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn every_batch_permutation_has_the_same_complete_projection() {
    let data = captured();
    let original = data["append"]["request"]["bundle"].clone();
    for permutation in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let (dir, path) = with_existing("permutation-").await;
        let mut desired = original.clone();
        desired["reflections"] = json!(permutation.map(|i| original["reflections"][i].clone()));
        let result = submit(&path, desired.clone()).await.unwrap();
        let actual = runtime::list_artifacts(&path, "ordering-source")
            .await
            .unwrap();
        assert_eq!(result.bundle, actual);
        assert_eq!(
            ids(&actual),
            [
                "ordering-question-A",
                "ordering-question-M",
                "ordering-question-Z"
            ]
        );
        for field in ["evidence", "patterns", "recoveryTurns"] {
            assert_eq!(actual[field], original[field]);
        }
        for prompt in actual["reflections"].as_array().unwrap() {
            assert!(original["reflections"].as_array().unwrap().contains(prompt));
        }
        preserve(
            &dir,
            "permutation.json",
            &json!({"submitted":desired,"committed":actual}),
        );
    }
}

#[tokio::test]
async fn timestamp_text_and_binary_ids_match_actual_sqlite_not_locale_or_instant_order() {
    let (dir, path) = with_existing("text-order-").await;
    let mut desired = captured()["append"]["request"]["bundle"].clone();
    for prompt in desired["reflections"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .skip(1)
    {
        let at = if prompt["id"] == "ordering-question-Z" {
            "2026-10-31T00:00:02.500Z"
        } else {
            "2026-10-31T00:00:02Z"
        };
        prompt["createdAt"] = json!(at);
        prompt["updatedAt"] = json!(at);
        prompt["promptProvenance"]["generatedAt"] = json!(at);
    }
    desired["reflections"].as_array_mut().unwrap().reverse();
    let result = submit(&path, desired.clone()).await.unwrap();
    assert_eq!(
        ids(&result.bundle),
        [
            "ordering-question-M",
            "ordering-question-Z",
            "ordering-question-A"
        ]
    );
    let mut c = sqlx::SqliteConnection::connect_with(
        &SqliteConnectOptions::new()
            .filename(&path)
            .read_only(true)
            .immutable(true),
    )
    .await
    .unwrap();
    let encoding: String = sqlx::query_scalar("PRAGMA encoding")
        .fetch_one(&mut c)
        .await
        .unwrap();
    let keys:Vec<(String,String)>=sqlx::query_as("SELECT id,created_at FROM persisted_artifacts WHERE artifact_kind='reflection' ORDER BY created_at,id").fetch_all(&mut c).await.unwrap();
    c.close().await.unwrap();
    assert_eq!(encoding, "UTF-8");
    for (id, at) in &keys {
        assert!(desired["reflections"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p["id"] == *id && p["createdAt"] == *at));
    }
    preserve(
        &dir,
        "literal-time-order.json",
        &json!({"encoding":encoding,"columnKeys":keys,"submitted":desired,"committed":result.bundle}),
    );
    for pair in [["a", "A"], ["𐀀", ""]] {
        let (dir, path) = fixture("binary-id-").await;
        let mut desired = captured()["first"]["request"]["bundle"].clone();
        for (i, id) in pair.iter().enumerate() {
            desired["reflections"][i]["id"] = json!(id);
        }
        let result = submit(&path, desired.clone()).await.unwrap();
        assert_eq!(ids(&result.bundle), [pair[1], pair[0]]);
        preserve(
            &dir,
            "binary-ids.json",
            &json!({"submitted":desired,"committed":result.bundle}),
        );
    }
}

#[tokio::test]
async fn malformed_contradictory_stale_and_replacement_requests_do_not_change_db() {
    let (_dir, path) = with_existing("refusal-").await;
    let data = captured();
    let good = data["append"]["request"]["bundle"].clone();
    let before = runtime::list_artifacts(&path, "ordering-source")
        .await
        .unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let mut variants = Vec::new();
    for (pointer, bad) in [
        ("/reflections/1/id", json!("ordering-question-M")),
        ("/reflections/1/createdAt", json!(42)),
        (
            "/reflections/1/createdAt",
            json!("2026-10-31T00:00:02+00:00"),
        ),
        (
            "/reflections/1/updatedAt",
            json!("2026-10-31T00:00:03.000Z"),
        ),
        ("/reflections/1/status", json!("answered")),
        (
            "/reflections/1/promptProvenance/sourceEntryId",
            json!("foreign"),
        ),
        (
            "/reflections/0/question",
            json!("Do not overwrite this existing question"),
        ),
    ] {
        let mut bad_bundle = good.clone();
        *bad_bundle.pointer_mut(pointer).unwrap() = bad;
        variants.push(bad_bundle);
    }
    let mut extra = good.clone();
    extra["reflections"][1]["unknown"] = json!("must not disappear");
    variants.push(extra);
    let mut extra = good.clone();
    extra["unknown"] = json!("must not disappear");
    variants.push(extra);
    let mut nested = good.clone();
    nested["reflections"][1]["sourceEvidenceIds"] =
        json!(["ordering-evidence-2", "ordering-evidence-1"]);
    nested["reflections"][1]["promptProvenance"]["sourceArtifactIds"] =
        json!(["ordering-evidence-2", "ordering-evidence-1"]);
    variants.push(nested);
    let mut replacement = good.clone();
    replacement["reflections"].as_array_mut().unwrap().remove(0);
    variants.push(replacement);
    for variant in variants {
        assert!(submit(&path, variant).await.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert_eq!(
            runtime::list_artifacts(&path, "ordering-source")
                .await
                .unwrap(),
            before
        );
    }
    let stale = runtime::save_artifacts(
        &path,
        "ordering-source",
        good,
        Some("2099-01-01T00:00:00.000Z"),
        "2026-10-31T00:00:02.000Z",
        "ordering-stale-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    assert_eq!(stale.status, "stale_generation");
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
}

async fn answer(path: &Path, text: &str, at: &str) {
    let mut desired = runtime::list_artifacts(path, "ordering-source")
        .await
        .unwrap();
    let p = desired["reflections"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["id"] == "ordering-question-M")
        .unwrap();
    p["status"] = json!("answered");
    p["response"] = json!(text);
    p["updatedAt"] = json!(at);
    p["responseProvenance"] = json!({"origin":"user","sourceEntryId":"ordering-source","sourceArtifactIds":["ordering-question-M"],"generatedAt":at});
    // Preserve the real frontend's omitted nullable user fields; existing fix owns them.
    desired["reflections"].as_array_mut().unwrap().reverse();
    let result = runtime::save_artifacts(
        path,
        "ordering-source",
        desired,
        Some("2026-10-31T00:00:00.000Z"),
        at,
        "ordering-answer-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    assert_eq!(
        result.bundle,
        runtime::list_artifacts(path, "ordering-source")
            .await
            .unwrap()
    );
}
#[tokio::test]
async fn answered_existing_question_and_other_artifacts_survive_batch_then_correct_and_delete() {
    let (dir, path) = with_existing("answer-lineage-").await;
    answer(&path, "Synthetic answer A", "2026-10-31T00:00:03.000Z").await;
    let mut before = runtime::list_artifacts(&path, "ordering-source")
        .await
        .unwrap();
    before["recoveryTurns"] = json!([{"id":"ordering-recovery","sourceEntryId":"ordering-source","question":"SYNTHETIC optional context?","status":"suggested","locale":"en","promptProvenance":{"origin":"local_mock","sourceEntryId":"ordering-source","sourceArtifactIds":[],"provider":"mock","model":null,"harnessVersion":"harness-v1","promptVersion":"v1","generatedAt":"2026-10-31T00:00:02.000Z"},"createdAt":"2026-10-31T00:00:02.000Z","updatedAt":"2026-10-31T00:00:02.000Z"}]);
    submit(&path, before).await.unwrap();
    let before = runtime::list_artifacts(&path, "ordering-source")
        .await
        .unwrap();
    let answered = before["reflections"][0].clone();
    let mut desired = before.clone();
    desired["reflections"].as_array_mut().unwrap().extend(
        captured()["first"]["request"]["bundle"]["reflections"]
            .as_array()
            .unwrap()
            .iter()
            .cloned(),
    );
    let result = submit(&path, desired).await.unwrap();
    assert_eq!(
        ids(&result.bundle),
        [
            "ordering-question-A",
            "ordering-question-M",
            "ordering-question-Z"
        ]
    );
    assert!(result.bundle["reflections"]
        .as_array()
        .unwrap()
        .contains(&answered));
    for key in ["evidence", "patterns", "recoveryTurns"] {
        assert_eq!(result.bundle[key], before[key]);
    }
    answer(&path, "Synthetic answer B", "2026-10-31T00:00:04.000Z").await;
    let current = runtime::list_artifacts(&path, "ordering-source")
        .await
        .unwrap();
    let q = current["reflections"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == "ordering-question-M")
        .unwrap();
    assert_eq!(q["response"], "Synthetic answer B");
    assert_eq!(q["promptProvenance"], answered["promptProvenance"]);
    let mut c = sqlx::SqliteConnection::connect_with(
        &SqliteConnectOptions::new()
            .filename(&path)
            .read_only(true)
            .immutable(true),
    )
    .await
    .unwrap();
    let lineage:Vec<(String,Option<String>)>=sqlx::query_as("SELECT revision_reason,predecessor_revision_id FROM artifact_revisions WHERE artifact_id='ordering-question-M' ORDER BY revision_number").fetch_all(&mut c).await.unwrap();
    c.close().await.unwrap();
    assert_eq!(
        lineage.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(),
        ["created", "answered", "corrected"]
    );
    assert!(lineage[2].1.is_some());
    preserve(&dir, "answered-after-batch.json", &current);
    std::fs::copy(&path, dir.join("before-delete.db")).unwrap();
    assert!(runtime::delete_experience(
        &path,
        "ordering-source",
        "2026-10-31T00:00:00.000Z",
        "2026-10-31T00:00:05.000Z",
        "ordering-delete-0123456789abcdef0123456789abcdef"
    )
    .await
    .unwrap());
    assert!(runtime::get_experience(&path, "ordering-source")
        .await
        .unwrap()
        .is_none());
    assert_eq!(
        runtime::list_artifacts(&path, "ordering-source")
            .await
            .unwrap(),
        json!({"evidence":[],"reflections":[],"patterns":[],"recoveryTurns":[]})
    );
}

#[tokio::test]
async fn contradictory_projection_column_keys_are_refused_read_only() {
    for column in ["id", "created_at"] {
        let (_dir, path) = with_existing("bad-column-key-").await;
        let mut c =
            sqlx::SqliteConnection::connect_with(&SqliteConnectOptions::new().filename(&path))
                .await
                .unwrap();
        sqlx::query("INSERT INTO v5_compatibility_write_guard(token,created_at) VALUES('ordering-synthetic-corruption-0123456789abcdef','2026-10-31T00:00:02.000Z')").execute(&mut c).await.unwrap();
        let sql = if column == "id" {
            "UPDATE persisted_artifacts SET id='contradictory-column-id' WHERE artifact_kind='reflection'"
        } else {
            "UPDATE persisted_artifacts SET created_at='2026-10-31T00:00:09.000Z' WHERE artifact_kind='reflection'"
        };
        sqlx::query(sql).execute(&mut c).await.unwrap();
        sqlx::query("DELETE FROM v5_compatibility_write_guard")
            .execute(&mut c)
            .await
            .unwrap();
        c.close().await.unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(
            runtime::list_artifacts(&path, "ordering-source")
                .await
                .unwrap_err()
                .code,
            "founder_runtime_reflection_projection_key_mismatch"
        );
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
}

#[tokio::test]
async fn skip_projection_content_is_checked_before_write_without_changing_skip_semantics() {
    let (_dir, path) = with_existing("skip-projection-").await;
    let before = runtime::list_artifacts(&path, "ordering-source")
        .await
        .unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let mut desired = before.clone();
    desired["reflections"][0]["status"] = json!("skipped");
    desired["reflections"][0]["updatedAt"] = json!("2026-10-31T00:00:03.000Z");
    desired["reflections"][0]["question"] = json!("must not rewrite the question when skipping");
    let rejected = runtime::save_artifacts(
        &path,
        "ordering-source",
        desired.clone(),
        Some("2026-10-31T00:00:00.000Z"),
        "2026-10-31T00:00:03.000Z",
        "ordering-skip-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap_err();
    assert_eq!(
        rejected.code,
        "founder_runtime_reflection_skip_projection_mismatch"
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    desired["reflections"][0]["question"] = before["reflections"][0]["question"].clone();
    let committed = runtime::save_artifacts(
        &path,
        "ordering-source",
        desired,
        Some("2026-10-31T00:00:00.000Z"),
        "2026-10-31T00:00:03.000Z",
        "ordering-skip-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    assert_eq!(
        committed.bundle,
        runtime::list_artifacts(&path, "ordering-source")
            .await
            .unwrap()
    );
}
