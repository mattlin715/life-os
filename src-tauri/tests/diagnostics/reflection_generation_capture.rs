//! Authorized, opt-in host diagnostic. No production routing or policy changes.
use crate::schema_v5_fresh_base::EMPTY_V4_BASE_SCHEMA;
use crate::schema_v5_migration::{
    activate_lifecycle_writes, migrate_disposable_v4, runtime, source_manifest_for_path,
    FailurePoint, MigrationRequest,
};
use serde_json::{json, Value};
use sqlx::{sqlite::SqliteConnectOptions, Connection};
use std::{fs::OpenOptions, io::Write, path::Path};

fn record(directory: &Path, event: Value) {
    let mut output = OpenOptions::new()
        .append(true)
        .create(true)
        .open(directory.join("trace.jsonl"))
        .unwrap();
    writeln!(output, "{event}").unwrap();
    output.sync_all().unwrap();
    println!("REFLECTION_SQLITE_CAPTURE {event}");
}

#[tokio::test]
#[ignore = "Explicit synthetic capture only; preserves its new DB and trace under .artifacts"]
async fn capture_fresh_then_replace_reflection_generation() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".artifacts/reflection-generation-capture-r1");
    std::fs::create_dir_all(&root).unwrap();
    let directory = tempfile::Builder::new()
        .prefix("case-")
        .tempdir_in(&root)
        .unwrap()
        .keep();
    println!("PRESERVED_SYNTHETIC_CASE {}", directory.display());
    let path = directory.join("life-os.db");
    let fixture: Value = serde_json::from_str(include_str!(
        "../fixtures/schema_v5/reflection_generation_capture.json"
    ))
    .unwrap();
    let source = &fixture["source"];
    let id = source["id"].as_str().unwrap();
    let at = source["createdAt"].as_str().unwrap();
    // Same empty-v4 -> migration -> activation primitives as the Founder fresh initializer.
    // This does NOT exercise AppHandle, installation, publication rename, or the real profile.
    let mut connection = sqlx::SqliteConnection::connect_with(
        &SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true),
    )
    .await
    .unwrap();
    sqlx::raw_sql(EMPTY_V4_BASE_SCHEMA)
        .execute(&mut connection)
        .await
        .unwrap();
    connection.close().await.unwrap();
    let source_manifest = source_manifest_for_path(&path).await.unwrap();
    let receipt = migrate_disposable_v4(MigrationRequest {
        path: &path,
        expected_source_manifest_digest: source_manifest,
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
        source["body"].as_str().unwrap(),
        at,
        "capture-source-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    let mut bundle = runtime::list_artifacts(&path, id).await.unwrap();
    let mut candidate = fixture["evidence"].clone();
    candidate["status"] = json!("candidate");
    candidate["updatedAt"] = json!(at);
    bundle["evidence"] = json!([candidate]);
    runtime::save_artifacts(
        &path,
        id,
        bundle,
        Some(at),
        at,
        "capture-evidence-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();
    let mut confirmed = runtime::list_artifacts(&path, id).await.unwrap();
    confirmed["evidence"] = json!([fixture["evidence"]]);
    runtime::save_artifacts(
        &path,
        id,
        confirmed,
        Some(at),
        fixture["evidence"]["updatedAt"].as_str().unwrap(),
        "capture-confirm-0123456789abcdef0123456789abcdef",
    )
    .await
    .unwrap();

    for index in 0..2 {
        let before = runtime::list_artifacts(&path, id).await.unwrap();
        let bytes_before = std::fs::read(&path).unwrap();
        let mut desired = before.clone();
        desired["reflections"] = json!([fixture["prompts"][index]]);
        record(
            &directory,
            json!({"step":index + 1, "phase":"before_save", "before":before, "desired":desired}),
        );
        let result = runtime::save_artifacts(
            &path,
            id,
            desired,
            Some(at),
            fixture["prompts"][index]["updatedAt"].as_str().unwrap(),
            &format!("capture-question-{index}-0123456789abcdef0123456789abcdef"),
        )
        .await;
        let after = runtime::list_artifacts(&path, id).await.unwrap();
        let error = result.as_ref().err().map(|error| error.code.clone());
        let status = result.as_ref().ok().map(|result| result.status);
        let unchanged = bytes_before == std::fs::read(&path).unwrap();
        record(
            &directory,
            json!({"step":index + 1, "phase":"after_save_and_reopen_read", "status":status,
            "error":error, "after":after, "db_bytes_unchanged":unchanged}),
        );
        if index == 0 {
            assert!(result.is_ok(), "first generation: {error:?}");
            assert_eq!(after["reflections"], json!([fixture["prompts"][0]]));
        } else {
            assert_eq!(
                error.as_deref(),
                Some("founder_runtime_mutation_not_single_action")
            );
            assert_eq!(after, before);
            assert!(unchanged);
        }
    }
    // Control: adding Q2 while retaining Q1 is a different, accepted batch shape.
    // Diagnostic only: this is NOT a proposed product behavior or a retry of the failed request.
    // Preserve the refused case DB at Q1; only the separate quiescent synthetic copy is changed.
    for suffix in ["-wal", "-shm", "-journal"] {
        assert!(!std::path::PathBuf::from(format!("{}{}", path.display(), suffix)).exists());
    }
    let control_path = directory.join("append-control.db");
    assert!(!control_path.exists());
    std::fs::copy(&path, &control_path).unwrap();
    let before = runtime::list_artifacts(&control_path, id).await.unwrap();
    let mut desired = before.clone();
    desired["reflections"] = fixture["prompts"].clone();
    record(
        &directory,
        json!({"phase":"control_append_before", "before":before, "desired":desired}),
    );
    let result = runtime::save_artifacts(
        &control_path,
        id,
        desired,
        Some(at),
        fixture["prompts"][1]["updatedAt"].as_str().unwrap(),
        "capture-append-control-0123456789abcdef0123456789abcdef",
    )
    .await;
    let after = runtime::list_artifacts(&control_path, id).await.unwrap();
    record(
        &directory,
        json!({"phase":"control_append_after", "error":result.as_ref().err().map(|error| &error.code), "after":after}),
    );
    assert!(result.is_ok());
    assert_eq!(after["reflections"].as_array().unwrap().len(), 2);
}
