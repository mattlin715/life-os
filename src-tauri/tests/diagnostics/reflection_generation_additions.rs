//! Real canonical-runtime/SQLite regression, separate from preserved diagnostic evidence.
use crate::schema_v5_fresh_base::EMPTY_V4_BASE_SCHEMA;
use crate::schema_v5_migration::{
    activate_lifecycle_writes, migrate_disposable_v4, runtime, source_manifest_for_path,
    FailurePoint, MigrationRequest,
};
use serde_json::{json, Value};
use sqlx::{sqlite::SqliteConnectOptions, Connection};

#[tokio::test]
async fn additive_generation_answers_correction_and_source_deletion() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".artifacts/reflection-generation-correction-r1");
    std::fs::create_dir_all(&root).unwrap();
    let directory = tempfile::Builder::new()
        .prefix("sqlite-case-")
        .tempdir_in(root)
        .unwrap()
        .keep();
    println!("PRESERVED_SYNTHETIC_CASE {}", directory.display());
    let path = directory.join("life-os.db");
    let at = "2026-08-13T12:00:00.000Z";
    let id = "fixture-v4-history";
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
        "SYNTHETIC: a disposable reflection generation journey.",
        at,
        "additive-source-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    let mut bundle = runtime::list_artifacts(&path, id).await.unwrap();
    bundle["evidence"] = json!([{"id":"runtime-evidence", "sourceEntryId":id, "text":"A synthetic observation", "originalText":"A synthetic observation", "kind":"observation", "userEditable":true, "status":"candidate", "provenance":{"origin":"local_mock", "sourceEntryId":id, "sourceArtifactIds":[], "provider":"mock", "model":null, "harnessVersion":"harness-v1", "promptVersion":"v1", "generatedAt":at}, "createdAt":at,"updatedAt":at}]);
    runtime::save_artifacts(
        &path,
        id,
        bundle,
        Some(at),
        at,
        "additive-evidence-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    let mut bundle = runtime::list_artifacts(&path, id).await.unwrap();
    bundle["evidence"][0]["status"] = json!("confirmed");
    bundle["evidence"][0]["updatedAt"] = json!("2026-08-13T12:00:01.000Z");
    runtime::save_artifacts(
        &path,
        id,
        bundle,
        Some(at),
        "2026-08-13T12:00:01.000Z",
        "additive-confirm-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    // TS product-handler test pins these exact mock-generator payloads and response helper outputs.
    let payloads: Value = serde_json::from_str(include_str!(
        "../fixtures/schema_v5/desktop_reflection_answers.json"
    ))
    .unwrap();
    let q1 = payloads["suggested"].clone();
    let mut q2 = q1.clone();
    q2["id"] = json!("runtime-reflection-2");
    for (index, prompts) in [json!([q1]), json!([q1, q2])].into_iter().enumerate() {
        let mut desired = runtime::list_artifacts(&path, id).await.unwrap();
        desired["reflections"] = prompts;
        let result = runtime::save_artifacts(
            &path,
            id,
            desired,
            Some(at),
            "2026-08-13T12:00:02.000Z",
            &format!("additive-questions-{index}-0123456789abcdef0123456789abcdef"),
        )
        .await
        .unwrap();
        assert_eq!(result.status, "committed");
        assert_eq!(
            runtime::list_artifacts(&path, id).await.unwrap(),
            result.bundle
        );
    }
    let before = runtime::list_artifacts(&path, id).await.unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let mut replacement = before.clone();
    replacement["reflections"] = json!([q2]);
    let error = runtime::save_artifacts(
        &path,
        id,
        replacement,
        Some(at),
        "2026-08-13T12:00:02.000Z",
        "additive-replacement-refused-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap_err();
    // Removing one existing ID is still rejected; the earlier diagnostic also covers remove+add.
    assert_eq!(error.code, "founder_runtime_reflection_delete_unsupported");
    let mut q3 = q2.clone();
    q3["id"] = json!("runtime-reflection-3");
    let mut replacement = before.clone();
    replacement["reflections"] = json!([q3]);
    let error = runtime::save_artifacts(
        &path,
        id,
        replacement,
        Some(at),
        "2026-08-13T12:00:02.000Z",
        "additive-old-replacement-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap_err();
    assert_eq!(error.code, "founder_runtime_mutation_not_single_action");
    assert_eq!(runtime::list_artifacts(&path, id).await.unwrap(), before);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::copy(&path, directory.join("after-refused-replacement.db")).unwrap();
    for key in ["answerA", "answerB"] {
        let mut desired = runtime::list_artifacts(&path, id).await.unwrap();
        let list = desired["reflections"].as_array_mut().unwrap();
        let target = list.iter_mut().find(|item| item["id"] == q1["id"]).unwrap();
        *target = payloads[key].clone();
        let result = runtime::save_artifacts(
            &path,
            id,
            desired,
            Some(at),
            payloads[key]["updatedAt"].as_str().unwrap(),
            &format!("additive-{key}-0123456789abcdef0123456789abcdef"),
        )
        .await
        .unwrap();
        let reopened = runtime::list_artifacts(&path, id).await.unwrap();
        assert_eq!(result.bundle, reopened);
        assert!(reopened["reflections"].as_array().unwrap().contains(&q2));
        let answered = reopened["reflections"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["id"] == q1["id"])
            .unwrap();
        assert_eq!(answered["response"], payloads[key]["response"]);
        assert_eq!(answered["promptProvenance"], q1["promptProvenance"]);
    }
    let mut c = sqlx::SqliteConnection::connect_with(
        &SqliteConnectOptions::new()
            .filename(&path)
            .read_only(true)
            .immutable(true),
    )
    .await
    .unwrap();
    let reasons: Vec<String>=sqlx::query_scalar("SELECT revision_reason FROM artifact_revisions WHERE artifact_id='runtime-reflection' ORDER BY revision_number").fetch_all(&mut c).await.unwrap();
    assert_eq!(&reasons[1..], &["answered", "corrected"]);
    c.close().await.unwrap();
    std::fs::copy(&path, directory.join("before-source-deletion.db")).unwrap();
    assert!(runtime::delete_experience(
        &path,
        id,
        at,
        "2026-08-13T12:00:05.000Z",
        "additive-delete-0123456789abcdef0123456789abcdef"
    )
    .await
    .unwrap());
    assert!(runtime::get_experience(&path, id).await.unwrap().is_none());
    assert_eq!(
        runtime::list_artifacts(&path, id).await.unwrap(),
        json!({"evidence":[],"reflections":[],"patterns":[],"recoveryTurns":[]})
    );
    let mut c = sqlx::SqliteConnection::connect_with(
        &SqliteConnectOptions::new()
            .filename(&path)
            .read_only(true)
            .immutable(true),
    )
    .await
    .unwrap();
    let count:i64=sqlx::query_scalar("SELECT (SELECT COUNT(*) FROM artifact_revision_content)+(SELECT COUNT(*) FROM source_revision_content)+(SELECT COUNT(*) FROM persisted_artifacts)").fetch_one(&mut c).await.unwrap();
    assert_eq!(count, 0);
    c.close().await.unwrap();
}
