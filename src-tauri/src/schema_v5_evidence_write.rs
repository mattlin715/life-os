use super::experience_write::{operation_manifest, verify_exact_v5};
use super::*;
use sqlx::{raw_sql, Row};
use std::collections::BTreeSet;

const ALLOWED_EVIDENCE_KINDS: [&str; 6] = [
    "observation",
    "emotion",
    "decision",
    "contradiction",
    "self_description",
    "other",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct EvidenceProvenanceInput {
    origin: String,
    provider: String,
    model: Option<String>,
    harness_version: String,
    prompt_version: String,
    generated_at: String,
    source_artifact_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EvidenceCandidateInput {
    id: String,
    source_id: String,
    text: String,
    original_text: String,
    kind: String,
    user_editable: bool,
    created_at: String,
    provenance: EvidenceProvenanceInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
enum EvidenceWriteCommand {
    Create {
        expected_source_revision_id: String,
        candidate: EvidenceCandidateInput,
    },
    CorrectPending {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        text: String,
    },
    ConfirmPending {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
    },
    RejectPending {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum EvidenceWriteFailurePoint {
    #[default]
    None,
    AfterGuard,
    AfterProvenance,
    AfterRevision,
    AfterContent,
    AfterDependency,
    AfterLifecycle,
    AfterReview,
    AfterHead,
    AfterTombstone,
    AfterPurge,
    AfterProjection,
    AfterReconciliation,
    AfterGuardRemoval,
}

#[derive(Clone, Debug)]
struct EvidenceWriteContext<'a> {
    occurred_at: &'a str,
    guard_token: &'a str,
    failure_point: EvidenceWriteFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EvidenceWriteStatus {
    Committed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EvidenceWriteOutcome {
    status: EvidenceWriteStatus,
    artifact_id: String,
    revision_id: String,
    operation_manifest: String,
}

#[derive(Clone, Debug)]
struct CurrentEvidence {
    source_id: String,
    revision_id: String,
    revision_number: i64,
    review_state: String,
    lifecycle_state: String,
    eligibility_state: String,
    serialization_version: String,
    authorship: String,
    revision_reason: String,
    predecessor_revision_id: Option<String>,
    content_digest: String,
    payload: Value,
    created_at: String,
    projection_payload: Value,
    generated_provenance: Value,
}

fn validate_legacy_provenance(
    raw: Option<&Value>,
    normalized: &Value,
    source_id: &str,
) -> Result<(), MigrationError> {
    let normalized = normalized
        .as_object()
        .ok_or_else(|| recovery_error("evidence_legacy_provenance_not_object"))?;
    let normalized_keys = [
        "origin",
        "sourceEntryId",
        "sourceArtifactIds",
        "provider",
        "model",
        "harnessVersion",
        "promptVersion",
        "generatedAt",
    ];
    if normalized.len() != normalized_keys.len()
        || normalized
            .keys()
            .any(|key| !normalized_keys.contains(&key.as_str()))
    {
        return Err(recovery_error("evidence_legacy_provenance_shape_mismatch"));
    }
    if normalized.get("sourceEntryId").and_then(Value::as_str) != Some(source_id) {
        return Err(recovery_error("evidence_legacy_provenance_source_mismatch"));
    }
    let normalized_sources = normalized
        .get("sourceArtifactIds")
        .and_then(Value::as_array)
        .ok_or_else(|| recovery_error("evidence_legacy_provenance_sources_missing"))?;
    if !normalized_sources.is_empty() {
        return Err(recovery_error(
            "evidence_legacy_provenance_sources_unexpected",
        ));
    }

    let normalized_origin = normalized
        .get("origin")
        .and_then(Value::as_str)
        .ok_or_else(|| recovery_error("evidence_legacy_provenance_origin_missing"))?;
    match normalized_origin {
        "ai" | "local_mock" | "user" | "legacy_unknown" => {}
        _ => return Err(write_error("evidence_legacy_provenance_origin_unsupported")),
    }

    if let Some(raw) = raw {
        let raw = raw
            .as_object()
            .ok_or_else(|| write_error("evidence_legacy_provenance_not_object"))?;
        if raw
            .keys()
            .any(|key| !normalized_keys.contains(&key.as_str()))
        {
            return Err(write_error("evidence_legacy_provenance_unknown_field"));
        }
        if raw.get("origin").and_then(Value::as_str) != Some(normalized_origin)
            || raw.get("sourceEntryId").and_then(Value::as_str) != Some(source_id)
        {
            return Err(recovery_error(
                "evidence_legacy_provenance_projection_mismatch",
            ));
        }
        let raw_sources = raw
            .get("sourceArtifactIds")
            .and_then(Value::as_array)
            .ok_or_else(|| write_error("evidence_legacy_provenance_sources_missing"))?;
        if !raw_sources.is_empty() {
            return Err(write_error("evidence_legacy_provenance_sources_unexpected"));
        }
        for key in [
            "provider",
            "model",
            "harnessVersion",
            "promptVersion",
            "generatedAt",
        ] {
            if raw.get(key).unwrap_or(&Value::Null) != normalized.get(key).unwrap_or(&Value::Null) {
                return Err(recovery_error(format!(
                    "evidence_legacy_provenance_field_mismatch:{key}"
                )));
            }
        }
    } else if normalized_origin != "legacy_unknown"
        || [
            "provider",
            "model",
            "harnessVersion",
            "promptVersion",
            "generatedAt",
        ]
        .iter()
        .any(|key| normalized.get(*key) != Some(&Value::Null))
    {
        return Err(recovery_error(
            "evidence_legacy_unknown_provenance_mismatch",
        ));
    }

    Ok(())
}

fn validate_legacy_review_candidate(
    current: &CurrentEvidence,
    source_id: &str,
    artifact_id: &str,
) -> Result<(), MigrationError> {
    if current.serialization_version != "legacy-v4-raw"
        || current.revision_reason != "legacy_v4_baseline"
        || current.revision_number != 1
        || current.predecessor_revision_id.is_some()
    {
        return Err(write_error("evidence_legacy_baseline_contract_mismatch"));
    }
    let object = current
        .payload
        .as_object()
        .ok_or_else(|| write_error("evidence_legacy_payload_not_object"))?;
    let allowed = [
        "id",
        "sourceEntryId",
        "text",
        "originalText",
        "kind",
        "status",
        "userEditable",
        "provenance",
        "createdAt",
        "updatedAt",
    ];
    let required = [
        "id",
        "sourceEntryId",
        "text",
        "kind",
        "status",
        "userEditable",
        "createdAt",
        "updatedAt",
    ];
    if object.keys().any(|key| !allowed.contains(&key.as_str())) {
        return Err(write_error("evidence_legacy_payload_unknown_field"));
    }
    if required.iter().any(|key| !object.contains_key(*key)) {
        return Err(write_error("evidence_legacy_payload_field_missing"));
    }
    if object.get("id").and_then(Value::as_str) != Some(artifact_id)
        || object.get("sourceEntryId").and_then(Value::as_str) != Some(source_id)
        || object.get("status").and_then(Value::as_str) != Some("candidate")
        || object.get("userEditable").and_then(Value::as_bool) != Some(true)
    {
        return Err(write_error(
            "evidence_legacy_payload_identity_or_state_mismatch",
        ));
    }
    let text = object
        .get("text")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("evidence_legacy_text_missing"))?;
    validate_text(text)?;
    if let Some(original) = object.get("originalText") {
        validate_text(
            original
                .as_str()
                .ok_or_else(|| write_error("evidence_legacy_original_text_malformed"))?,
        )?;
    }
    let kind = object
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("evidence_legacy_kind_missing"))?;
    if !ALLOWED_EVIDENCE_KINDS.contains(&kind) {
        return Err(write_error("evidence_legacy_kind_unsupported"));
    }
    let created_at = object
        .get("createdAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("evidence_legacy_created_at_missing"))?;
    let updated_at = object
        .get("updatedAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("evidence_legacy_updated_at_missing"))?;
    validate_timestamp(created_at, "legacy_created_at")?;
    validate_timestamp(updated_at, "legacy_updated_at")?;
    if created_at != current.created_at || current.source_id != source_id {
        return Err(recovery_error(
            "evidence_legacy_timestamp_or_source_mismatch",
        ));
    }
    validate_legacy_provenance(
        object.get("provenance"),
        &current.generated_provenance,
        source_id,
    )?;
    let origin = current
        .generated_provenance
        .get("origin")
        .and_then(Value::as_str)
        .ok_or_else(|| recovery_error("evidence_legacy_provenance_origin_missing"))?;
    let expected_authorship = match origin {
        "ai" => "ai",
        "local_mock" => "local_mock",
        "user" => "user",
        _ => "legacy_unknown",
    };
    if current.authorship != expected_authorship || current.projection_payload != current.payload {
        return Err(recovery_error(
            "evidence_legacy_authority_projection_mismatch",
        ));
    }
    Ok(())
}

fn legacy_review_projection(
    payload: &Value,
    status: &str,
    occurred_at: &str,
) -> Result<Value, MigrationError> {
    let mut projected = payload.clone();
    let object = projected
        .as_object_mut()
        .ok_or_else(|| write_error("evidence_legacy_payload_not_object"))?;
    object.insert("status".into(), Value::String(status.into()));
    object.insert("updatedAt".into(), Value::String(occurred_at.into()));
    Ok(projected)
}

fn write_error(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn recovery_error(code: impl Into<String>) -> MigrationError {
    MigrationError::recovery_required(code)
}

fn validate_identifier(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        Err(write_error(format!("evidence_{field}_invalid")))
    } else {
        Ok(())
    }
}

fn validate_timestamp(value: &str, field: &str) -> Result<(), MigrationError> {
    validate_identifier(value, field)?;
    if !value.contains('T') || !value.ends_with('Z') {
        return Err(write_error(format!("evidence_{field}_invalid")));
    }
    Ok(())
}

fn validate_text(value: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 100_000 {
        Err(write_error("evidence_text_invalid"))
    } else {
        Ok(())
    }
}

fn validate_provenance(
    provenance: &EvidenceProvenanceInput,
    source_id: &str,
) -> Result<(), MigrationError> {
    if !matches!(provenance.origin.as_str(), "ai" | "local_mock") {
        return Err(write_error("evidence_generated_origin_invalid"));
    }
    let provider_valid = match provenance.origin.as_str() {
        "ai" => matches!(provenance.provider.as_str(), "openai" | "gemini"),
        "local_mock" => provenance.provider == "mock",
        _ => false,
    };
    if !provider_valid {
        return Err(write_error("evidence_generated_provider_invalid"));
    }
    if provenance.origin == "ai"
        && provenance
            .model
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .is_none()
    {
        return Err(write_error("evidence_generated_model_missing"));
    }
    validate_identifier(&provenance.harness_version, "harness_version")?;
    validate_identifier(&provenance.prompt_version, "prompt_version")?;
    validate_timestamp(&provenance.generated_at, "generated_at")?;
    let mut sources = BTreeSet::new();
    for source in &provenance.source_artifact_ids {
        validate_identifier(source, "source_artifact_id")?;
        if source == source_id || !sources.insert(source) {
            return Err(write_error("evidence_generated_sources_invalid"));
        }
    }
    Ok(())
}

fn validate_candidate(candidate: &EvidenceCandidateInput) -> Result<(), MigrationError> {
    validate_identifier(&candidate.id, "artifact_id")?;
    validate_identifier(&candidate.source_id, "source_id")?;
    validate_text(&candidate.text)?;
    validate_text(&candidate.original_text)?;
    validate_timestamp(&candidate.created_at, "created_at")?;
    if !ALLOWED_EVIDENCE_KINDS.contains(&candidate.kind.as_str()) {
        return Err(write_error("evidence_kind_unsupported"));
    }
    if !candidate.user_editable {
        return Err(write_error("evidence_candidate_must_be_user_editable"));
    }
    if candidate.text != candidate.original_text {
        return Err(write_error("evidence_created_text_original_mismatch"));
    }
    validate_provenance(&candidate.provenance, &candidate.source_id)
}

fn inject(
    context: &EvidenceWriteContext<'_>,
    point: EvidenceWriteFailurePoint,
) -> Result<(), MigrationError> {
    if context.failure_point == point {
        Err(write_error(format!(
            "injected_evidence_write_failure:{point:?}"
        )))
    } else {
        Ok(())
    }
}

fn content_value(
    id: &str,
    source_id: &str,
    text: &str,
    original_text: &str,
    kind: &str,
    user_editable: bool,
    created_at: &str,
) -> Value {
    json!({
        "createdAt": created_at,
        "id": id,
        "kind": kind,
        "originalText": original_text,
        "sourceEntryId": source_id,
        "text": text,
        "userEditable": user_editable
    })
}

fn generated_provenance_value(source_id: &str, provenance: &EvidenceProvenanceInput) -> Value {
    json!({
        "generatedAt": provenance.generated_at,
        "harnessVersion": provenance.harness_version,
        "model": provenance.model,
        "origin": provenance.origin,
        "promptVersion": provenance.prompt_version,
        "provider": provenance.provider,
        "sourceArtifactIds": provenance.source_artifact_ids,
        "sourceEntryId": source_id
    })
}

fn user_provenance_value(source_id: &str, artifact_id: &str, occurred_at: &str) -> Value {
    json!({
        "generatedAt": occurred_at,
        "harnessVersion": null,
        "model": null,
        "origin": "user",
        "promptVersion": null,
        "provider": null,
        "sourceArtifactIds": [artifact_id],
        "sourceEntryId": source_id
    })
}

fn v4_projection_value(
    content: &Value,
    status: &str,
    updated_at: &str,
    generated_provenance: &Value,
) -> Result<Value, MigrationError> {
    let object = content
        .as_object()
        .ok_or_else(|| write_error("evidence_content_not_object"))?;
    Ok(json!({
        "createdAt": object.get("createdAt").cloned().ok_or_else(|| write_error("evidence_created_at_missing"))?,
        "id": object.get("id").cloned().ok_or_else(|| write_error("evidence_id_missing"))?,
        "kind": object.get("kind").cloned().ok_or_else(|| write_error("evidence_kind_missing"))?,
        "originalText": object.get("originalText").cloned().ok_or_else(|| write_error("evidence_original_text_missing"))?,
        "provenance": generated_provenance,
        "sourceEntryId": object.get("sourceEntryId").cloned().ok_or_else(|| write_error("evidence_source_missing"))?,
        "status": status,
        "text": object.get("text").cloned().ok_or_else(|| write_error("evidence_text_missing"))?,
        "updatedAt": updated_at,
        "userEditable": object.get("userEditable").cloned().ok_or_else(|| write_error("evidence_editability_missing"))?
    }))
}

async fn exact_current_source(
    connection: &mut SqliteConnection,
    source_id: &str,
    expected_revision_id: &str,
) -> Result<(), MigrationError> {
    validate_identifier(source_id, "source_id")?;
    validate_identifier(expected_revision_id, "source_revision_id")?;
    let row: Option<(String, String, String, String)> = sqlx::query_as(
        "SELECT h.current_revision_id, h.lifecycle_state, c.content, e.content  \
         FROM source_heads h  \
         JOIN source_revision_content c ON c.revision_id = h.current_revision_id  \
         JOIN experience_entries e ON e.id = h.id  \
         WHERE h.id = ?",
    )
    .bind(source_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_source_lookup_failed", error))?;
    let Some((revision_id, lifecycle, content, projection)) = row else {
        return Err(write_error("evidence_source_not_found"));
    };
    if revision_id != expected_revision_id || lifecycle != "active" || content != projection {
        return Err(write_error("evidence_source_revision_stale"));
    }
    Ok(())
}

async fn insert_guard(
    connection: &mut SqliteConnection,
    context: &EvidenceWriteContext<'_>,
) -> Result<(), MigrationError> {
    validate_identifier(context.guard_token, "guard_token")?;
    validate_timestamp(context.occurred_at, "occurred_at")?;
    if context.guard_token.len() < 32 {
        return Err(write_error("evidence_guard_token_invalid"));
    }
    sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
        .bind(context.guard_token)
        .bind(context.occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_guard_insert_failed", error))?;
    inject(context, EvidenceWriteFailurePoint::AfterGuard)
}

async fn remove_guard(
    connection: &mut SqliteConnection,
    context: &EvidenceWriteContext<'_>,
) -> Result<(), MigrationError> {
    let result = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
        .bind(context.guard_token)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_guard_remove_failed", error))?;
    if result.rows_affected() != 1 {
        return Err(recovery_error("evidence_guard_identity_mismatch"));
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_guard_count_failed", error))?;
    if count != 0 {
        return Err(recovery_error("evidence_guard_not_empty"));
    }
    inject(context, EvidenceWriteFailurePoint::AfterGuardRemoval)
}

async fn inbound_dependency_count(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
) -> Result<i64, MigrationError> {
    let normalized: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_dependencies  \
         WHERE source_artifact_id = ? AND source_artifact_revision_id = ?",
    )
    .bind(artifact_id)
    .bind(revision_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_inbound_dependency_lookup_failed", error))?;
    let historical: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM historical_artifact_dependencies  \
         WHERE source_artifact_id = ?",
    )
    .bind(artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_historical_dependency_lookup_failed", error))?;
    Ok(normalized + historical)
}

async fn current_evidence(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_revision_id: &str,
) -> Result<CurrentEvidence, MigrationError> {
    validate_identifier(artifact_id, "artifact_id")?;
    validate_identifier(expected_revision_id, "artifact_revision_id")?;
    let row = sqlx::query(
        "SELECT h.source_id, h.current_revision_id, h.review_state,  \
                h.lifecycle_state, h.eligibility_state, h.created_at,  \
                r.revision_number, r.serialization_version, r.authorship,  \
                r.revision_reason, r.predecessor_revision_id, r.content_digest, \
                c.payload, p.canonical_payload, pa.payload  \
         FROM artifact_heads h  \
         JOIN artifact_revisions r ON r.id = h.current_revision_id  \
           AND r.artifact_id = h.id  \
         JOIN artifact_revision_content c ON c.revision_id = r.id  \
         JOIN artifact_revision_provenance rp  \
           ON rp.artifact_revision_id = r.id AND rp.role = 'content'  \
         JOIN provenance_records p ON p.id = rp.provenance_id  \
         JOIN persisted_artifacts pa ON pa.id = h.id  \
           AND pa.source_entry_id = h.source_id AND pa.artifact_kind = 'evidence'  \
         WHERE h.id = ? AND h.artifact_kind = 'evidence'",
    )
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_current_lookup_failed", error))?
    .ok_or_else(|| write_error("evidence_artifact_not_found"))?;

    let actual_source: String = row.get(0);
    let revision_id: String = row.get(1);
    if actual_source != source_id || revision_id != expected_revision_id {
        return Err(write_error("evidence_artifact_revision_stale"));
    }
    let raw_payload: String = row.get(12);
    let payload: Value = serde_json::from_str(&raw_payload)
        .map_err(|error| migration_error("evidence_current_payload_malformed", error))?;
    let provenance_raw: String = row.get(13);
    let generated_provenance: Value = serde_json::from_str(&provenance_raw)
        .map_err(|error| migration_error("evidence_current_provenance_malformed", error))?;
    let projection_raw: String = row.get(14);
    let projection_payload: Value = serde_json::from_str(&projection_raw)
        .map_err(|error| migration_error("evidence_projection_payload_malformed", error))?;
    let content_digest: String = row.get(11);
    if content_digest != sha256_hex(raw_payload.as_bytes()) {
        return Err(recovery_error("evidence_current_digest_mismatch"));
    }
    Ok(CurrentEvidence {
        source_id: actual_source,
        revision_id,
        revision_number: row.get(6),
        review_state: row.get(2),
        lifecycle_state: row.get(3),
        eligibility_state: row.get(4),
        serialization_version: row.get(7),
        authorship: row.get(8),
        revision_reason: row.get(9),
        predecessor_revision_id: row.get(10),
        content_digest,
        payload,
        created_at: row.get(5),
        projection_payload,
        generated_provenance,
    })
}

#[allow(clippy::too_many_arguments)]
async fn insert_revision(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    source_id: &str,
    revision_id: &str,
    revision_number: i64,
    predecessor: Option<&str>,
    authorship: &str,
    reason: &str,
    payload: &str,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let digest = sha256_hex(payload.as_bytes());
    sqlx::query(
        "INSERT INTO artifact_revisions ( \
           id, artifact_id, source_id, revision_number, predecessor_revision_id, \
           authorship, revision_reason, serialization_version, content_digest, created_at \
         ) VALUES (?, ?, ?, ?, ?, ?, ?, 'canonical-json-v1', ?, ?)",
    )
    .bind(revision_id)
    .bind(artifact_id)
    .bind(source_id)
    .bind(revision_number)
    .bind(predecessor)
    .bind(authorship)
    .bind(reason)
    .bind(digest)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_revision_insert_failed", error))?;
    Ok(())
}

async fn insert_content(
    connection: &mut SqliteConnection,
    revision_id: &str,
    payload: &str,
) -> Result<(), MigrationError> {
    sqlx::query(
        "INSERT INTO artifact_revision_content (revision_id, payload, byte_length)  \
         VALUES (?, ?, ?)",
    )
    .bind(revision_id)
    .bind(payload)
    .bind(payload.len() as i64)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_content_insert_failed", error))?;
    Ok(())
}

async fn insert_source_dependency(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
    source_id: &str,
    source_revision_id: &str,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let id = dependency_id(
        artifact_id,
        revision_id,
        "derived_from_experience",
        source_id,
        source_revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_dependencies ( \
           id, dependent_artifact_id, dependent_revision_id, relationship_type, \
           source_revision_id, source_artifact_id, source_artifact_revision_id, created_at \
         ) VALUES (?, ?, ?, 'derived_from_experience', ?, NULL, NULL, ?)",
    )
    .bind(id)
    .bind(artifact_id)
    .bind(revision_id)
    .bind(source_revision_id)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_dependency_insert_failed", error))?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn insert_lifecycle_event(
    connection: &mut SqliteConnection,
    domain: &str,
    event_type: &str,
    actor: &str,
    reason: &str,
    artifact_id: &str,
    subject_revision_id: &str,
    related_revision_id: Option<&str>,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let id = event_id("v5le_", domain, artifact_id, subject_revision_id);
    sqlx::query(
        "INSERT INTO artifact_lifecycle_events ( \
           id, artifact_id, subject_revision_id, related_revision_id, dependency_id, \
           event_type, actor, reason_code, occurred_at \
         ) VALUES (?, ?, ?, ?, NULL, ?, ?, ?, ?)",
    )
    .bind(id)
    .bind(artifact_id)
    .bind(subject_revision_id)
    .bind(related_revision_id)
    .bind(event_type)
    .bind(actor)
    .bind(reason)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_insert_failed", error))?;
    Ok(())
}

async fn insert_review_event(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
    decision: &str,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let id = event_id(
        "v5re_",
        &format!("life-os/evidence-{decision}-review-event-id-v1"),
        artifact_id,
        revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_review_events ( \
           id, artifact_id, subject_revision_id, decision, actor, event_origin, \
           occurred_at, timestamp_quality \
         ) VALUES (?, ?, ?, ?, 'user', 'explicit_user_action', ?, 'exact_action_time')",
    )
    .bind(id)
    .bind(artifact_id)
    .bind(revision_id)
    .bind(decision)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_review_insert_failed", error))?;
    Ok(())
}

async fn write_projection(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    source_id: &str,
    payload: &Value,
    created_at: &str,
    updated_at: &str,
    insert: bool,
) -> Result<(), MigrationError> {
    let serialized = canonical_json(payload)?;
    if insert {
        sqlx::query(
            "INSERT INTO persisted_artifacts  \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at)  \
             VALUES (?, ?, 'evidence', ?, ?, ?)",
        )
        .bind(artifact_id)
        .bind(source_id)
        .bind(serialized)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_projection_insert_failed", error))?;
    } else {
        let result = sqlx::query(
            "UPDATE persisted_artifacts SET payload = ?, updated_at = ?  \
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'evidence'",
        )
        .bind(serialized)
        .bind(updated_at)
        .bind(artifact_id)
        .bind(source_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_projection_update_failed", error))?;
        if result.rows_affected() != 1 {
            return Err(recovery_error("evidence_projection_identity_mismatch"));
        }
    }
    Ok(())
}

async fn verify_evidence_projection(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    let heads = sqlx::query(
        "SELECT h.id, h.source_id, h.current_revision_id, h.review_state,  \
                h.lifecycle_state, h.eligibility_state, h.created_at, h.updated_at  \
         FROM artifact_heads h WHERE h.artifact_kind = 'evidence' ORDER BY h.id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_heads_unreadable", error))?;
    let mut projected = 0_i64;
    for head in heads {
        let artifact_id: String = head.get(0);
        let source_id: String = head.get(1);
        let current_revision_id: Option<String> = head.get(2);
        let review_state: String = head.get(3);
        let lifecycle_state: String = head.get(4);
        let eligibility_state: String = head.get(5);
        let projection: Option<(String, String, String)> = sqlx::query_as(
            "SELECT payload, created_at, updated_at FROM persisted_artifacts  \
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'evidence'",
        )
        .bind(&artifact_id)
        .bind(&source_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_projection_unreadable", error))?;

        match lifecycle_state.as_str() {
            "active" => {
                projected += 1;
                let revision_id = current_revision_id
                    .ok_or_else(|| recovery_error("evidence_active_revision_missing"))?;
                let revision: Option<(String, String, String, i64)> = sqlx::query_as(
                    "SELECT r.serialization_version, r.content_digest, c.payload, c.byte_length  \
                     FROM artifact_revisions r  \
                     JOIN artifact_revision_content c ON c.revision_id = r.id  \
                     WHERE r.id = ? AND r.artifact_id = ?",
                )
                .bind(&revision_id)
                .bind(&artifact_id)
                .fetch_optional(&mut *connection)
                .await
                .map_err(|error| migration_error("evidence_revision_unreadable", error))?;
                let (serialization, digest, content, byte_length) =
                    revision.ok_or_else(|| recovery_error("evidence_active_content_missing"))?;
                if digest != sha256_hex(content.as_bytes()) || byte_length != content.len() as i64 {
                    return Err(recovery_error("evidence_content_digest_mismatch"));
                }
                let (projection_payload, created_at, updated_at) =
                    projection.ok_or_else(|| recovery_error("evidence_projection_missing"))?;
                if created_at != head.get::<String, _>(6) || updated_at != head.get::<String, _>(7)
                {
                    return Err(recovery_error("evidence_projection_timestamp_mismatch"));
                }
                let projected_value: Value =
                    serde_json::from_str(&projection_payload).map_err(|error| {
                        migration_error("evidence_projection_payload_malformed", error)
                    })?;
                let expected_status = match review_state.as_str() {
                    "pending" if eligibility_state == "ineligible" => "candidate",
                    "confirmed" if eligibility_state == "eligible" => "confirmed",
                    _ => return Err(recovery_error("evidence_review_eligibility_mismatch")),
                };
                if projected_value.get("status").and_then(Value::as_str) != Some(expected_status) {
                    return Err(recovery_error("evidence_projection_status_mismatch"));
                }
                if serialization == "legacy-v4-raw" {
                    if projection_payload.as_bytes() != content.as_bytes() {
                        if review_state != "confirmed" || eligibility_state != "eligible" {
                            return Err(recovery_error("evidence_legacy_projection_mismatch"));
                        }
                        let content_value: Value =
                            serde_json::from_str(&content).map_err(|error| {
                                migration_error("evidence_legacy_content_malformed", error)
                            })?;
                        let expected = legacy_review_projection(
                            &content_value,
                            "confirmed",
                            &head.get::<String, _>(7),
                        )?;
                        if projected_value != expected {
                            return Err(recovery_error(
                                "evidence_legacy_review_projection_mismatch",
                            ));
                        }
                        let explicit_review: i64 = sqlx::query_scalar(
                            "SELECT COUNT(*) FROM artifact_review_events \
                             WHERE artifact_id = ? AND subject_revision_id = ? \
                               AND decision = 'confirmed' AND actor = 'user' \
                               AND event_origin = 'explicit_user_action' AND occurred_at = ? \
                               AND timestamp_quality = 'exact_action_time'",
                        )
                        .bind(&artifact_id)
                        .bind(&revision_id)
                        .bind(head.get::<String, _>(7))
                        .fetch_one(&mut *connection)
                        .await
                        .map_err(|error| {
                            migration_error("evidence_legacy_review_event_unreadable", error)
                        })?;
                        if explicit_review != 1 {
                            return Err(recovery_error("evidence_legacy_review_event_mismatch"));
                        }
                    }
                } else if serialization == "canonical-json-v1" {
                    let content_value: Value = serde_json::from_str(&content)
                        .map_err(|error| migration_error("evidence_content_malformed", error))?;
                    for key in [
                        "id",
                        "sourceEntryId",
                        "text",
                        "originalText",
                        "kind",
                        "userEditable",
                        "createdAt",
                    ] {
                        if content_value.get(key) != projected_value.get(key) {
                            return Err(recovery_error(format!(
                                "evidence_projection_field_mismatch:{key}"
                            )));
                        }
                    }
                } else {
                    return Err(recovery_error("evidence_serialization_version_unsupported"));
                }
            }
            "invalidated" => {
                let revision_id = current_revision_id
                    .ok_or_else(|| recovery_error("evidence_invalidated_revision_missing"))?;
                if eligibility_state != "ineligible"
                    || !matches!(review_state.as_str(), "pending" | "confirmed")
                    || projection.is_some()
                {
                    return Err(recovery_error("evidence_invalidated_head_mismatch"));
                }
                let retained: Option<(String, String, i64)> = sqlx::query_as(
                    "SELECT r.content_digest, c.payload, c.byte_length \
                     FROM artifact_revisions r \
                     JOIN artifact_revision_content c ON c.revision_id = r.id \
                     WHERE r.id = ? AND r.artifact_id = ? AND r.source_id = ?",
                )
                .bind(&revision_id)
                .bind(&artifact_id)
                .bind(&source_id)
                .fetch_optional(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("evidence_invalidated_content_unreadable", error)
                })?;
                let (digest, content, byte_length) = retained
                    .ok_or_else(|| recovery_error("evidence_invalidated_content_missing"))?;
                if digest != sha256_hex(content.as_bytes()) || byte_length != content.len() as i64 {
                    return Err(recovery_error(
                        "evidence_invalidated_content_digest_mismatch",
                    ));
                }
                let provenance_count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM artifact_revision_provenance \
                     WHERE artifact_revision_id = ?",
                )
                .bind(&revision_id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("evidence_invalidated_provenance_unreadable", error)
                })?;
                if provenance_count == 0 {
                    return Err(recovery_error("evidence_invalidated_provenance_missing"));
                }
                let eligibility_reason: String = sqlx::query_scalar(
                    "SELECT eligibility_reason FROM artifact_heads WHERE id = ?",
                )
                .bind(&artifact_id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("evidence_invalidated_reason_unreadable", error)
                })?;
                if eligibility_reason != "source_experience_revision_superseded"
                    || !super::experience_write::verify_source_caused_invalidation(
                        connection,
                        &artifact_id,
                        &revision_id,
                        &source_id,
                        &head.get::<String, _>(7),
                    )
                    .await?
                {
                    return Err(recovery_error("evidence_invalidated_facts_mismatch"));
                }
            }
            "content_purged" => {
                if current_revision_id.is_some()
                    || review_state != "rejected"
                    || eligibility_state != "ineligible"
                    || projection.is_some()
                {
                    return Err(recovery_error("evidence_rejected_projection_mismatch"));
                }
            }
            "deleted" => {
                if current_revision_id.is_some()
                    || review_state != "confirmed"
                    || eligibility_state != "ineligible"
                    || projection.is_some()
                {
                    return Err(recovery_error("evidence_deleted_projection_mismatch"));
                }
                let retained: (i64, i64, i64, i64) = sqlx::query_as(
                    "SELECT \
                       (SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id = ?), \
                       (SELECT COUNT(*) FROM artifact_revision_content c \
                         JOIN artifact_revisions r ON r.id = c.revision_id \
                         WHERE r.artifact_id = ?), \
                       (SELECT COUNT(*) FROM artifact_lifecycle_events \
                         WHERE artifact_id = ? AND event_type = 'deleted' \
                           AND actor = 'user'), \
                       (SELECT COUNT(*) FROM content_tombstones \
                         WHERE artifact_id = ? AND subject_type = 'artifact' \
                           AND artifact_revision_id IS NULL AND content_digest IS NULL \
                           AND reason_code = 'user_deleted_artifact')",
                )
                .bind(&artifact_id)
                .bind(&artifact_id)
                .bind(&artifact_id)
                .bind(&artifact_id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| migration_error("evidence_deletion_history_unreadable", error))?;
                if retained.0 == 0 || retained.1 != 0 || retained.2 != 1 || retained.3 != 1 {
                    return Err(recovery_error("evidence_deletion_history_mismatch"));
                }
            }
            _ => return Err(recovery_error("evidence_lifecycle_unsupported")),
        }
    }
    let projection_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM persisted_artifacts WHERE artifact_kind = 'evidence'",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_projection_count_failed", error))?;
    if projection_count != projected {
        return Err(recovery_error("evidence_projection_count_mismatch"));
    }
    Ok(())
}

pub(super) async fn verify_exact_evidence_v5(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    verify_exact_v5(connection).await?;
    verify_evidence_projection(connection).await
}

async fn create_candidate(
    connection: &mut SqliteConnection,
    expected_source_revision_id: &str,
    candidate: EvidenceCandidateInput,
    context: &EvidenceWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    validate_candidate(&candidate)?;
    exact_current_source(
        connection,
        &candidate.source_id,
        expected_source_revision_id,
    )
    .await?;
    let conflict: i64 = sqlx::query_scalar(
        "SELECT  \
           (SELECT COUNT(*) FROM artifact_heads WHERE id = ?) +  \
           (SELECT COUNT(*) FROM persisted_artifacts WHERE id = ?)",
    )
    .bind(&candidate.id)
    .bind(&candidate.id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_identity_check_failed", error))?;
    if conflict != 0 {
        return Err(write_error("evidence_identity_conflict"));
    }

    let content = content_value(
        &candidate.id,
        &candidate.source_id,
        &candidate.text,
        &candidate.original_text,
        &candidate.kind,
        candidate.user_editable,
        &candidate.created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id =
        artifact_revision_id(&candidate.id, "evidence", &candidate.created_at, &digest);
    let provenance = generated_provenance_value(&candidate.source_id, &candidate.provenance);
    // The role link requires the revision; insert provenance now and link after
    // revision creation without rebinding either record.
    let provenance_id = insert_provenance(connection, &provenance, &candidate.created_at).await?;
    inject(context, EvidenceWriteFailurePoint::AfterProvenance)?;
    insert_revision(
        connection,
        &candidate.id,
        &candidate.source_id,
        &revision_id,
        1,
        None,
        &candidate.provenance.origin,
        "created",
        &payload,
        &candidate.created_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterRevision)?;
    insert_content(connection, &revision_id, &payload).await?;
    inject(context, EvidenceWriteFailurePoint::AfterContent)?;
    sqlx::query(
        "INSERT INTO artifact_heads ( \
           id, source_id, artifact_kind, current_revision_id, review_state, \
           lifecycle_state, eligibility_state, eligibility_reason, created_at, updated_at \
         ) VALUES (?, ?, 'evidence', ?, 'pending', 'active', 'ineligible', \
           'pending_explicit_review', ?, ?)",
    )
    .bind(&candidate.id)
    .bind(&candidate.source_id)
    .bind(&revision_id)
    .bind(&candidate.created_at)
    .bind(&candidate.created_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_head_insert_failed", error))?;
    sqlx::query(
        "INSERT INTO artifact_revision_provenance  \
         (artifact_revision_id, role, provenance_id) VALUES (?, 'content', ?)",
    )
    .bind(&revision_id)
    .bind(&provenance_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_provenance_link_failed", error))?;
    inject(context, EvidenceWriteFailurePoint::AfterHead)?;
    insert_source_dependency(
        connection,
        &candidate.id,
        &revision_id,
        &candidate.source_id,
        expected_source_revision_id,
        &candidate.created_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/evidence-created-event-id-v1",
        "created",
        "system",
        "evidence_candidate_created",
        &candidate.id,
        &revision_id,
        None,
        &candidate.created_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterLifecycle)?;
    let projection =
        v4_projection_value(&content, "candidate", &candidate.created_at, &provenance)?;
    write_projection(
        connection,
        &candidate.id,
        &candidate.source_id,
        &projection,
        &candidate.created_at,
        &candidate.created_at,
        true,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterProjection)?;
    Ok((candidate.id, revision_id))
}

async fn correct_pending(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    text: &str,
    context: &EvidenceWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    validate_text(text)?;
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_evidence(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
        || !matches!(current.authorship.as_str(), "ai" | "local_mock" | "user")
        || current.serialization_version != "canonical-json-v1"
    {
        return Err(write_error("evidence_pending_correction_not_allowed"));
    }
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "evidence_inbound_dependency_requires_later_slice",
        ));
    }
    let original_text = current
        .payload
        .get("originalText")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("evidence_original_text_missing"))?;
    let kind = current
        .payload
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("evidence_kind_missing"))?;
    let user_editable = current
        .payload
        .get("userEditable")
        .and_then(Value::as_bool)
        .ok_or_else(|| write_error("evidence_editability_missing"))?;
    let created_at = current
        .payload
        .get("createdAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("evidence_created_at_missing"))?;
    let content = content_value(
        artifact_id,
        source_id,
        text,
        original_text,
        kind,
        user_editable,
        created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id = artifact_revision_id(artifact_id, "evidence", context.occurred_at, &digest);
    let user_provenance = user_provenance_value(source_id, artifact_id, context.occurred_at);
    let provenance_id =
        insert_provenance(connection, &user_provenance, context.occurred_at).await?;
    inject(context, EvidenceWriteFailurePoint::AfterProvenance)?;
    insert_revision(
        connection,
        artifact_id,
        source_id,
        &revision_id,
        current.revision_number + 1,
        Some(&current.revision_id),
        "user",
        "corrected",
        &payload,
        context.occurred_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterRevision)?;
    insert_content(connection, &revision_id, &payload).await?;
    sqlx::query(
        "INSERT INTO artifact_revision_provenance  \
         (artifact_revision_id, role, provenance_id) VALUES (?, 'content', ?)",
    )
    .bind(&revision_id)
    .bind(provenance_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_provenance_link_failed", error))?;
    inject(context, EvidenceWriteFailurePoint::AfterContent)?;
    insert_source_dependency(
        connection,
        artifact_id,
        &revision_id,
        source_id,
        expected_source_revision_id,
        context.occurred_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/evidence-corrected-event-id-v1",
        "corrected",
        "user",
        "pending_candidate_corrected",
        artifact_id,
        &revision_id,
        Some(&current.revision_id),
        context.occurred_at,
    )
    .await?;
    insert_lifecycle_event(
        connection,
        "life-os/evidence-superseded-event-id-v1",
        "superseded",
        "user",
        "replaced_by_user_correction",
        artifact_id,
        &current.revision_id,
        Some(&revision_id),
        context.occurred_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterLifecycle)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = ?, review_state = 'pending', \
         lifecycle_state = 'active', eligibility_state = 'ineligible', \
         eligibility_reason = 'correction_requires_confirmation', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'evidence' AND review_state = 'pending' \
           AND lifecycle_state = 'active'",
    )
    .bind(&revision_id)
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_head_update_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("evidence_artifact_revision_stale"));
    }
    inject(context, EvidenceWriteFailurePoint::AfterHead)?;
    let projection = v4_projection_value(
        &content,
        "candidate",
        context.occurred_at,
        &current.generated_provenance,
    )?;
    write_projection(
        connection,
        artifact_id,
        source_id,
        &projection,
        &current.created_at,
        context.occurred_at,
        false,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.to_string(), revision_id))
}

async fn confirm_pending(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    context: &EvidenceWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_evidence(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
    {
        return Err(write_error("evidence_pending_review_not_allowed"));
    }
    if current.serialization_version == "legacy-v4-raw" {
        validate_legacy_review_candidate(&current, source_id, artifact_id)?;
    } else if current.serialization_version != "canonical-json-v1" {
        return Err(write_error("evidence_review_serialization_unsupported"));
    }
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "evidence_inbound_dependency_requires_later_slice",
        ));
    }
    insert_review_event(
        connection,
        artifact_id,
        &current.revision_id,
        "confirmed",
        context.occurred_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterReview)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET review_state = 'confirmed', \
         eligibility_state = 'eligible', \
         eligibility_reason = 'explicitly_confirmed_current_revision', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'evidence' AND review_state = 'pending' \
           AND lifecycle_state = 'active'",
    )
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_confirm_head_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("evidence_artifact_revision_stale"));
    }
    inject(context, EvidenceWriteFailurePoint::AfterHead)?;
    let projection = if current.serialization_version == "legacy-v4-raw" {
        legacy_review_projection(&current.payload, "confirmed", context.occurred_at)?
    } else {
        v4_projection_value(
            &current.payload,
            "confirmed",
            context.occurred_at,
            &current.generated_provenance,
        )?
    };
    write_projection(
        connection,
        artifact_id,
        source_id,
        &projection,
        &current.created_at,
        context.occurred_at,
        false,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.to_string(), current.revision_id))
}

async fn reject_pending(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    context: &EvidenceWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_evidence(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
    {
        return Err(write_error("evidence_pending_review_not_allowed"));
    }
    if current.serialization_version == "legacy-v4-raw" {
        validate_legacy_review_candidate(&current, source_id, artifact_id)?;
    } else if current.serialization_version != "canonical-json-v1" {
        return Err(write_error("evidence_review_serialization_unsupported"));
    }
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "evidence_inbound_dependency_requires_later_slice",
        ));
    }
    insert_review_event(
        connection,
        artifact_id,
        &current.revision_id,
        "rejected",
        context.occurred_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterReview)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = NULL, \
         review_state = 'rejected', lifecycle_state = 'content_purged', \
         eligibility_state = 'ineligible', \
         eligibility_reason = 'explicitly_rejected_content_purged', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'evidence' AND review_state = 'pending' \
           AND lifecycle_state = 'active'",
    )
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_reject_head_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("evidence_artifact_revision_stale"));
    }
    inject(context, EvidenceWriteFailurePoint::AfterHead)?;
    insert_lifecycle_event(
        connection,
        "life-os/evidence-rejected-content-purged-event-id-v1",
        "content_purged",
        "user",
        "rejected_content_purged",
        artifact_id,
        &current.revision_id,
        None,
        context.occurred_at,
    )
    .await?;
    inject(context, EvidenceWriteFailurePoint::AfterLifecycle)?;
    let tombstone_id = format!(
        "v5ts_{}",
        sha256_hex(
            format!(
                "life-os/evidence-rejection-tombstone-id-v1\0{artifact_id}\0{}\0{}",
                current.revision_id, current.content_digest
            )
            .as_bytes()
        )
    );
    sqlx::query(
        "INSERT INTO content_tombstones ( \
           id, subject_type, source_id, source_revision_id, artifact_id, \
           artifact_revision_id, content_digest, reason_code, purged_at \
         ) VALUES (?, 'artifact_revision', NULL, NULL, ?, ?, ?, \
           'rejected_content_purged', ?)",
    )
    .bind(tombstone_id)
    .bind(artifact_id)
    .bind(&current.revision_id)
    .bind(&current.content_digest)
    .bind(context.occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_tombstone_insert_failed", error))?;
    inject(context, EvidenceWriteFailurePoint::AfterTombstone)?;
    let purged = sqlx::query("DELETE FROM artifact_revision_content WHERE revision_id = ?")
        .bind(&current.revision_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_content_purge_failed", error))?;
    if purged.rows_affected() != 1 {
        return Err(recovery_error("evidence_content_purge_identity_mismatch"));
    }
    inject(context, EvidenceWriteFailurePoint::AfterPurge)?;
    let deleted = sqlx::query(
        "DELETE FROM persisted_artifacts  \
         WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'evidence'",
    )
    .bind(artifact_id)
    .bind(source_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_projection_delete_failed", error))?;
    if deleted.rows_affected() != 1 {
        return Err(recovery_error("evidence_projection_identity_mismatch"));
    }
    inject(context, EvidenceWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.to_string(), current.revision_id))
}

async fn apply_command(
    connection: &mut SqliteConnection,
    command: EvidenceWriteCommand,
    context: &EvidenceWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    match command {
        EvidenceWriteCommand::Create {
            expected_source_revision_id,
            candidate,
        } => create_candidate(connection, &expected_source_revision_id, candidate, context).await,
        EvidenceWriteCommand::CorrectPending {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            text,
        } => {
            correct_pending(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                &text,
                context,
            )
            .await
        }
        EvidenceWriteCommand::ConfirmPending {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
        } => {
            confirm_pending(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                context,
            )
            .await
        }
        EvidenceWriteCommand::RejectPending {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
        } => {
            reject_pending(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                context,
            )
            .await
        }
    }
}

async fn prepare_write(
    connection: &mut SqliteConnection,
    command: EvidenceWriteCommand,
    context: &EvidenceWriteContext<'_>,
) -> Result<EvidenceWriteOutcome, MigrationError> {
    insert_guard(connection, context).await?;
    let (artifact_id, revision_id) = apply_command(connection, command, context).await?;
    verify_evidence_projection(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    inject(context, EvidenceWriteFailurePoint::AfterReconciliation)?;
    remove_guard(connection, context).await?;
    verify_exact_evidence_v5(connection).await?;
    let post_manifest = operation_manifest(connection).await?;
    Ok(EvidenceWriteOutcome {
        status: EvidenceWriteStatus::Committed,
        artifact_id,
        revision_id,
        operation_manifest: post_manifest,
    })
}

async fn verify_read_only(path: &Path, expected_manifest: &str) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await?;
    verify_exact_evidence_v5(&mut connection).await?;
    let actual = operation_manifest(&mut connection).await?;
    if actual != expected_manifest {
        return Err(recovery_error("evidence_operation_manifest_mismatch"));
    }
    Ok(())
}

async fn execute_with_adapter<A: CommitOutcomeAdapter>(
    path: &Path,
    command: EvidenceWriteCommand,
    context: EvidenceWriteContext<'_>,
    adapter: &A,
) -> Result<EvidenceWriteOutcome, MigrationError> {
    let mut connection = connect(path, false).await?;
    verify_exact_evidence_v5(&mut connection).await?;
    let pre_manifest = operation_manifest(&mut connection).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("evidence_begin_failed", error))?;
    let prepared = match prepare_write(&mut connection, command, &context).await {
        Ok(prepared) => prepared,
        Err(error) => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verify| {
                    recovery_error(format!(
                        "evidence_precommit_state_unverified:{}:{rollback:?}",
                        verify.code
                    ))
                })?;
            return Err(error);
        }
    };
    match adapter.commit(&mut connection).await {
        CommitAttemptOutcome::Committed => {
            drop(connection);
            verify_read_only(path, &prepared.operation_manifest).await?;
            Ok(prepared)
        }
        CommitAttemptOutcome::DefinitelyNotCommitted { error_class } => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verify| {
                    recovery_error(format!(
                        "evidence_definite_noncommit_unverified:{error_class}:{}:{rollback:?}",
                        verify.code
                    ))
                })?;
            Err(write_error(format!(
                "evidence_commit_definitely_not_committed:{error_class}"
            )))
        }
        CommitAttemptOutcome::OutcomeUnknown { error_class } => {
            drop(connection);
            if verify_read_only(path, &prepared.operation_manifest)
                .await
                .is_ok()
            {
                return Ok(prepared);
            }
            if verify_read_only(path, &pre_manifest).await.is_ok() {
                return Err(write_error(format!(
                    "evidence_commit_outcome_unknown_unchanged:{error_class}"
                )));
            }
            Err(recovery_error(format!(
                "evidence_commit_outcome_unknown:{error_class}"
            )))
        }
    }
}

async fn execute_disposable(
    path: &Path,
    command: EvidenceWriteCommand,
    context: EvidenceWriteContext<'_>,
) -> Result<EvidenceWriteOutcome, MigrationError> {
    execute_with_adapter(path, command, context, &SqlCommitOutcomeAdapter).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const STARTED_AT: &str = "2026-07-30T05:00:00.000Z";
    const COMMITTED_AT: &str = "2026-07-30T05:00:01.000Z";
    const SOURCE_ID: &str = "fixture-v4-history";

    async fn exact_v5_fixture() -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        raw_sql("DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;")
            .execute(&mut connection)
            .await
            .unwrap();
        let expected_source_manifest_digest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        drop(connection);
        migrate_disposable_v4(MigrationRequest {
            path: &path,
            expected_source_manifest_digest,
            started_at: STARTED_AT,
            committed_at: COMMITTED_AT,
            backup_id: Some("slice4b1-fixture-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        (directory, path)
    }

    fn legacy_evidence_payload(id: &str, include_provenance: bool, extra_field: bool) -> String {
        let mut payload = json!({
            "createdAt": "2026-01-01T00:00:02.000Z",
            "id": id,
            "kind": "observation",
            "originalText": "Legacy observation",
            "sourceEntryId": SOURCE_ID,
            "status": "candidate",
            "text": "Legacy observation",
            "updatedAt": "2026-01-01T00:00:02.000Z",
            "userEditable": true
        });
        if include_provenance {
            payload.as_object_mut().unwrap().insert(
                "provenance".into(),
                json!({
                    "generatedAt": "2026-01-01T00:00:02.000Z",
                    "harnessVersion": "harness-v1",
                    "model": null,
                    "origin": "local_mock",
                    "promptVersion": "evidence-v1",
                    "provider": "mock",
                    "sourceArtifactIds": [],
                    "sourceEntryId": SOURCE_ID
                }),
            );
        }
        if extra_field {
            payload
                .as_object_mut()
                .unwrap()
                .insert("futureField".into(), Value::String("unsupported".into()));
        }
        serde_json::to_string_pretty(&payload).unwrap()
    }

    async fn exact_v5_legacy_evidence_fixture(
        id: &str,
        include_provenance: bool,
        extra_field: bool,
    ) -> (TempDir, std::path::PathBuf, String) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        raw_sql("DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;")
            .execute(&mut connection)
            .await
            .unwrap();
        let raw = legacy_evidence_payload(id, include_provenance, extra_field);
        sqlx::query(
            "INSERT INTO persisted_artifacts \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
             VALUES (?, ?, 'evidence', ?, '2026-01-01T00:00:02.000Z', \
               '2026-01-01T00:00:02.000Z')",
        )
        .bind(id)
        .bind(SOURCE_ID)
        .bind(&raw)
        .execute(&mut connection)
        .await
        .unwrap();
        let mut imported_review_payload: Value = serde_json::from_str(&legacy_evidence_payload(
            "legacy-imported-review-evidence",
            true,
            false,
        ))
        .unwrap();
        imported_review_payload["status"] = Value::String("confirmed".into());
        let imported_review_raw = serde_json::to_string_pretty(&imported_review_payload).unwrap();
        sqlx::query(
            "INSERT INTO persisted_artifacts \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
             VALUES ('legacy-imported-review-evidence', ?, 'evidence', ?, \
               '2026-01-01T00:00:02.000Z', '2026-01-01T00:00:02.000Z')",
        )
        .bind(SOURCE_ID)
        .bind(imported_review_raw)
        .execute(&mut connection)
        .await
        .unwrap();
        let expected_source_manifest_digest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        drop(connection);
        migrate_disposable_v4(MigrationRequest {
            path: &path,
            expected_source_manifest_digest,
            started_at: STARTED_AT,
            committed_at: COMMITTED_AT,
            backup_id: Some("slice4c6a-evidence-fixture-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        let mut read_only = connect(&path, true).await.unwrap();
        verify_exact_evidence_v5(&mut read_only).await.unwrap();
        (directory, path, raw)
    }

    async fn legacy_revision(path: &Path, id: &str) -> String {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar("SELECT current_revision_id FROM artifact_heads WHERE id = ?")
            .bind(id)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    fn provenance(origin: &str) -> EvidenceProvenanceInput {
        EvidenceProvenanceInput {
            origin: origin.into(),
            provider: if origin == "ai" { "gemini" } else { "mock" }.into(),
            model: if origin == "ai" {
                Some("fixture-model".into())
            } else {
                None
            },
            harness_version: "harness-v1".into(),
            prompt_version: "evidence-v1".into(),
            generated_at: "2026-07-30T06:00:00.000Z".into(),
            source_artifact_ids: Vec::new(),
        }
    }

    fn candidate(id: &str, origin: &str) -> EvidenceCandidateInput {
        EvidenceCandidateInput {
            id: id.into(),
            source_id: SOURCE_ID.into(),
            text: "A bounded observation.".into(),
            original_text: "A bounded observation.".into(),
            kind: "observation".into(),
            user_editable: true,
            created_at: "2026-07-30T06:00:00.000Z".into(),
            provenance: provenance(origin),
        }
    }

    fn context(
        occurred_at: &'static str,
        token: &'static str,
        failure_point: EvidenceWriteFailurePoint,
    ) -> EvidenceWriteContext<'static> {
        EvidenceWriteContext {
            occurred_at,
            guard_token: token,
            failure_point,
        }
    }

    async fn source_revision(path: &Path) -> String {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar("SELECT current_revision_id FROM source_heads WHERE id = ?")
            .bind(SOURCE_ID)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    async fn create(path: &Path, id: &str, origin: &str) -> EvidenceWriteOutcome {
        let source = source_revision(path).await;
        execute_disposable(
            path,
            EvidenceWriteCommand::Create {
                expected_source_revision_id: source,
                candidate: candidate(id, origin),
            },
            context(
                "2026-07-30T06:00:00.000Z",
                "guard-evidence-create-000000000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap()
    }

    async fn scalar(path: &Path, query: &str) -> i64 {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar(query)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn ai_and_local_mock_creation_preserve_authorship_provenance_and_source_revision() {
        for origin in ["ai", "local_mock"] {
            let (_directory, path) = exact_v5_fixture().await;
            let id = format!("evidence-{origin}");
            let outcome = create(&path, &id, origin).await;
            let mut connection = connect(&path, true).await.unwrap();
            let row: (String, String, String, String, String) = sqlx::query_as(
                "SELECT r.authorship, p.origin, h.review_state, h.eligibility_state, \
                 d.source_revision_id FROM artifact_revisions r \
                 JOIN artifact_heads h ON h.current_revision_id = r.id \
                 JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id \
                 JOIN provenance_records p ON p.id = rp.provenance_id \
                 JOIN artifact_dependencies d ON d.dependent_revision_id = r.id \
                 WHERE r.id = ?",
            )
            .bind(&outcome.revision_id)
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(row.0, origin);
            assert_eq!(row.1, origin);
            assert_eq!(row.2, "pending");
            assert_eq!(row.3, "ineligible");
            assert_eq!(row.4, source_revision(&path).await);
            let projected_status: String = sqlx::query_scalar(
                "SELECT json_extract(payload, '$.status') FROM persisted_artifacts WHERE id = ?",
            )
            .bind(&id)
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(projected_status, "candidate");
            assert_eq!(
                1,
                scalar(&path, "SELECT COUNT(*) FROM artifact_heads").await
            );
        }
    }

    #[tokio::test]
    async fn pending_correction_appends_user_revision_and_preserves_generated_history() {
        let (_directory, path) = exact_v5_fixture().await;
        let created = create(&path, "evidence-correct", "ai").await;
        let source = source_revision(&path).await;
        let corrected = execute_disposable(
            &path,
            EvidenceWriteCommand::CorrectPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-correct".into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: created.revision_id.clone(),
                text: "User-corrected observation.".into(),
            },
            context(
                "2026-07-30T06:01:00.000Z",
                "guard-evidence-correct-00000000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let revisions: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
            "SELECT id, authorship, revision_reason, predecessor_revision_id \
             FROM artifact_revisions WHERE artifact_id = 'evidence-correct' \
             ORDER BY revision_number",
        )
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert_eq!(revisions.len(), 2);
        assert_eq!(revisions[0].1, "ai");
        assert_eq!(revisions[1].1, "user");
        assert_eq!(revisions[1].2, "corrected");
        assert_eq!(
            revisions[1].3.as_deref(),
            Some(created.revision_id.as_str())
        );
        assert_eq!(revisions[1].0, corrected.revision_id);
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_lifecycle_events \
                 WHERE artifact_id='evidence-correct' \
                 AND event_type IN ('corrected','superseded')"
            )
            .await,
            2
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_revision_content \
                 WHERE revision_id IN (SELECT id FROM artifact_revisions \
                 WHERE artifact_id='evidence-correct')"
            )
            .await,
            2
        );
        let head: (String, String) = sqlx::query_as(
            "SELECT review_state, eligibility_state FROM artifact_heads \
             WHERE id='evidence-correct'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(head, ("pending".into(), "ineligible".into()));
    }

    #[tokio::test]
    async fn confirmation_targets_exact_revision_without_rewriting_content_or_provenance() {
        let (_directory, path) = exact_v5_fixture().await;
        let created = create(&path, "evidence-confirm", "local_mock").await;
        let mut before = connect(&path, true).await.unwrap();
        let bytes: (String, String) = sqlx::query_as(
            "SELECT c.payload, p.canonical_payload FROM artifact_revision_content c \
             JOIN artifact_revision_provenance rp ON rp.artifact_revision_id=c.revision_id \
             JOIN provenance_records p ON p.id=rp.provenance_id \
             WHERE c.revision_id=?",
        )
        .bind(&created.revision_id)
        .fetch_one(&mut before)
        .await
        .unwrap();
        drop(before);
        execute_disposable(
            &path,
            EvidenceWriteCommand::ConfirmPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-confirm".into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: created.revision_id.clone(),
            },
            context(
                "2026-07-30T06:02:00.000Z",
                "guard-evidence-confirm-00000000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut after = connect(&path, true).await.unwrap();
        let after_bytes: (String, String) = sqlx::query_as(
            "SELECT c.payload, p.canonical_payload FROM artifact_revision_content c \
             JOIN artifact_revision_provenance rp ON rp.artifact_revision_id=c.revision_id \
             JOIN provenance_records p ON p.id=rp.provenance_id \
             WHERE c.revision_id=?",
        )
        .bind(&created.revision_id)
        .fetch_one(&mut after)
        .await
        .unwrap();
        assert_eq!(bytes, after_bytes);
        let review: (String, String, String) = sqlx::query_as(
            "SELECT subject_revision_id, decision, actor FROM artifact_review_events \
             WHERE artifact_id='evidence-confirm'",
        )
        .fetch_one(&mut after)
        .await
        .unwrap();
        assert_eq!(
            review,
            (created.revision_id, "confirmed".into(), "user".into())
        );
        let projected: String = sqlx::query_scalar(
            "SELECT json_extract(payload, '$.status') FROM persisted_artifacts \
             WHERE id='evidence-confirm'",
        )
        .fetch_one(&mut after)
        .await
        .unwrap();
        assert_eq!(projected, "confirmed");
    }

    #[tokio::test]
    async fn rejection_purges_content_but_retains_content_free_history() {
        let (_directory, path) = exact_v5_fixture().await;
        let created = create(&path, "evidence-reject", "ai").await;
        execute_disposable(
            &path,
            EvidenceWriteCommand::RejectPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-reject".into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: created.revision_id.clone(),
            },
            context(
                "2026-07-30T06:03:00.000Z",
                "guard-evidence-reject-000000000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_revision_content \
                 WHERE revision_id IN (SELECT id FROM artifact_revisions \
                 WHERE artifact_id='evidence-reject')"
            )
            .await,
            0
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_review_events \
                 WHERE artifact_id='evidence-reject' AND decision='rejected'"
            )
            .await,
            1
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM content_tombstones \
                 WHERE artifact_id='evidence-reject' \
                 AND reason_code='rejected_content_purged'"
            )
            .await,
            1
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM persisted_artifacts WHERE id='evidence-reject'"
            )
            .await,
            0
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_revision_provenance \
                 WHERE artifact_revision_id IN (SELECT id FROM artifact_revisions \
                 WHERE artifact_id='evidence-reject')"
            )
            .await,
            1
        );
    }

    #[tokio::test]
    async fn duplicate_conflicting_and_stale_reviews_fail_closed() {
        let (_directory, path) = exact_v5_fixture().await;
        let created = create(&path, "evidence-review-once", "ai").await;
        let source = source_revision(&path).await;
        execute_disposable(
            &path,
            EvidenceWriteCommand::ConfirmPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-review-once".into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: created.revision_id.clone(),
            },
            context(
                "2026-07-30T06:04:00.000Z",
                "guard-evidence-review-once-00000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        for command in [
            EvidenceWriteCommand::ConfirmPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-review-once".into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: created.revision_id.clone(),
            },
            EvidenceWriteCommand::RejectPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-review-once".into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: created.revision_id.clone(),
            },
            EvidenceWriteCommand::ConfirmPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-review-once".into(),
                expected_source_revision_id: "stale-source".into(),
                expected_artifact_revision_id: created.revision_id.clone(),
            },
        ] {
            assert!(execute_disposable(
                &path,
                command,
                context(
                    "2026-07-30T06:05:00.000Z",
                    "guard-evidence-review-refuse-000000001",
                    EvidenceWriteFailurePoint::None,
                ),
            )
            .await
            .is_err());
        }
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_review_events").await,
            1
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM v5_compatibility_write_guard").await,
            0
        );
    }

    #[tokio::test]
    async fn malformed_unsupported_and_inbound_dependency_states_fail_closed() {
        let (_directory, path) = exact_v5_fixture().await;
        let source = source_revision(&path).await;
        let mut malformed = candidate("bad-provider", "ai");
        malformed.provenance.provider = "mock".into();
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        assert!(execute_disposable(
            &path,
            EvidenceWriteCommand::Create {
                expected_source_revision_id: source.clone(),
                candidate: malformed,
            },
            context(
                "2026-07-30T06:06:00.000Z",
                "guard-evidence-malformed-0000000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());
        let mut unsupported = candidate("bad-kind", "local_mock");
        unsupported.kind = "diagnosis".into();
        assert!(execute_disposable(
            &path,
            EvidenceWriteCommand::Create {
                expected_source_revision_id: source,
                candidate: unsupported,
            },
            context(
                "2026-07-30T06:06:30.000Z",
                "guard-evidence-unsupported-00000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());
        let created = create(&path, "evidence-dependent", "ai").await;
        {
            let mut connection = connect(&path, false).await.unwrap();
            raw_sql("BEGIN IMMEDIATE")
                .execute(&mut connection)
                .await
                .unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard VALUES \
                 ('manual-dependent-fixture-000000000000001','2026-07-30T06:07:00.000Z')",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            // A self-edge is semantically invalid for pending Evidence but is
            // structurally sufficient to prove inbound-dependent refusal.
            sqlx::query(
                "INSERT INTO artifact_dependencies VALUES \
                 ('fixture-inbound','evidence-dependent',?, 'uses_evidence', \
                  NULL,'evidence-dependent',?,'2026-07-30T06:07:00.000Z')",
            )
            .bind(&created.revision_id)
            .bind(&created.revision_id)
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query("DELETE FROM v5_compatibility_write_guard")
                .execute(&mut connection)
                .await
                .unwrap();
            raw_sql("COMMIT").execute(&mut connection).await.unwrap();
        }
        assert!(execute_disposable(
            &path,
            EvidenceWriteCommand::RejectPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "evidence-dependent".into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: created.revision_id,
            },
            context(
                "2026-07-30T06:08:00.000Z",
                "guard-evidence-dependent-0000000000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());
        let mut connection = connect(&path, true).await.unwrap();
        assert_ne!(operation_manifest(&mut connection).await.unwrap(), before);
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_review_events \
                 WHERE artifact_id='evidence-dependent'"
            )
            .await,
            0
        );
    }

    #[tokio::test]
    async fn injected_create_failures_roll_back_authority_projection_and_guard() {
        let points = [
            EvidenceWriteFailurePoint::AfterGuard,
            EvidenceWriteFailurePoint::AfterProvenance,
            EvidenceWriteFailurePoint::AfterRevision,
            EvidenceWriteFailurePoint::AfterContent,
            EvidenceWriteFailurePoint::AfterHead,
            EvidenceWriteFailurePoint::AfterDependency,
            EvidenceWriteFailurePoint::AfterLifecycle,
            EvidenceWriteFailurePoint::AfterProjection,
            EvidenceWriteFailurePoint::AfterReconciliation,
            EvidenceWriteFailurePoint::AfterGuardRemoval,
        ];
        for (index, point) in points.into_iter().enumerate() {
            let (_directory, path) = exact_v5_fixture().await;
            let source = source_revision(&path).await;
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            let id = format!("evidence-failure-{index}");
            assert!(execute_disposable(
                &path,
                EvidenceWriteCommand::Create {
                    expected_source_revision_id: source,
                    candidate: candidate(&id, "ai"),
                },
                EvidenceWriteContext {
                    occurred_at: "2026-07-30T06:09:00.000Z",
                    guard_token: "guard-evidence-failure-000000000000001",
                    failure_point: point,
                },
            )
            .await
            .is_err());
            let mut connection = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM v5_compatibility_write_guard")
                    .fetch_one(&mut connection)
                    .await
                    .unwrap(),
                0
            );
        }
    }

    struct InjectedCommitAdapter {
        outcome: CommitAttemptOutcome,
        commit_first: bool,
    }

    impl CommitOutcomeAdapter for InjectedCommitAdapter {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            if self.commit_first {
                raw_sql("COMMIT").execute(connection).await.unwrap();
            }
            self.outcome.clone()
        }

        async fn rollback(&self, connection: &mut SqliteConnection) -> RollbackAttemptOutcome {
            match raw_sql("ROLLBACK").execute(connection).await {
                Ok(_) => RollbackAttemptOutcome::RolledBack,
                Err(error) => RollbackAttemptOutcome::Failed {
                    error_class: error.to_string(),
                },
            }
        }
    }

    #[tokio::test]
    async fn migrated_legacy_review_ambiguous_commit_classifies_exact_pre_and_post_state() {
        for commit_first in [false, true] {
            let id = format!("legacy-evidence-ambiguous-{commit_first}");
            let (_directory, path, raw) =
                exact_v5_legacy_evidence_fixture(&id, true, false).await;
            let revision = legacy_revision(&path, &id).await;
            let result = execute_with_adapter(
                &path,
                EvidenceWriteCommand::ConfirmPending {
                    source_id: SOURCE_ID.into(),
                    artifact_id: id,
                    expected_source_revision_id: source_revision(&path).await,
                    expected_artifact_revision_id: revision.clone(),
                },
                context(
                    "2026-08-09T01:04:00.000Z",
                    "guard-legacy-evidence-ambiguous-001",
                    EvidenceWriteFailurePoint::None,
                ),
                &InjectedCommitAdapter {
                    outcome: CommitAttemptOutcome::OutcomeUnknown {
                        error_class: "injected_legacy_review_commit_ambiguity".into(),
                    },
                    commit_first,
                },
            )
            .await;
            if commit_first {
                assert_eq!(result.unwrap().status, EvidenceWriteStatus::Committed);
            } else {
                assert!(result
                    .unwrap_err()
                    .code
                    .contains("evidence_commit_outcome_unknown_unchanged"));
                let mut connection = connect(&path, true).await.unwrap();
                assert_eq!(
                    sqlx::query_scalar::<_, String>(
                        "SELECT payload FROM artifact_revision_content WHERE revision_id=?",
                    )
                    .bind(&revision)
                    .fetch_one(&mut connection)
                    .await
                    .unwrap(),
                    raw
                );
            }
        }
    }

    #[tokio::test]
    async fn ambiguous_commit_classifies_exact_post_or_pre_state_without_retry() {
        let (_directory, committed_path) = exact_v5_fixture().await;
        let committed_source = source_revision(&committed_path).await;
        let committed = execute_with_adapter(
            &committed_path,
            EvidenceWriteCommand::Create {
                expected_source_revision_id: committed_source,
                candidate: candidate("evidence-ambiguous-committed", "ai"),
            },
            context(
                "2026-07-30T06:10:00.000Z",
                "guard-evidence-ambiguous-commit-0000001",
                EvidenceWriteFailurePoint::None,
            ),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_after_commit".into(),
                },
                commit_first: true,
            },
        )
        .await
        .unwrap();
        assert_eq!(committed.status, EvidenceWriteStatus::Committed);

        let (_directory, unchanged_path) = exact_v5_fixture().await;
        let unchanged_source = source_revision(&unchanged_path).await;
        let error = execute_with_adapter(
            &unchanged_path,
            EvidenceWriteCommand::Create {
                expected_source_revision_id: unchanged_source,
                candidate: candidate("evidence-ambiguous-unchanged", "local_mock"),
            },
            context(
                "2026-07-30T06:11:00.000Z",
                "guard-evidence-ambiguous-pre-0000000001",
                EvidenceWriteFailurePoint::None,
            ),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_without_commit".into(),
                },
                commit_first: false,
            },
        )
        .await
        .unwrap_err();
        assert!(error
            .code
            .contains("evidence_commit_outcome_unknown_unchanged"));
        assert_eq!(
            scalar(
                &unchanged_path,
                "SELECT COUNT(*) FROM v5_compatibility_write_guard"
            )
            .await,
            0
        );
    }

    #[tokio::test]
    async fn migrated_legacy_candidate_confirmation_preserves_raw_history_and_unknown_provenance() {
        for (index, include_provenance) in [true, false].into_iter().enumerate() {
            let id = format!("legacy-evidence-confirm-{index}");
            let (_directory, path, raw) =
                exact_v5_legacy_evidence_fixture(&id, include_provenance, false).await;
            let revision = legacy_revision(&path, &id).await;
            let imported_review_before: (String, String, String, String) = {
                let mut connection = connect(&path, true).await.unwrap();
                sqlx::query_as(
                    "SELECT decision, actor, event_origin, timestamp_quality \
                     FROM artifact_review_events \
                     WHERE artifact_id='legacy-imported-review-evidence'",
                )
                .fetch_one(&mut connection)
                .await
                .unwrap()
            };
            let before: (String, String, String) = {
                let mut connection = connect(&path, true).await.unwrap();
                sqlx::query_as(
                    "SELECT c.payload, r.content_digest, p.canonical_payload \
                     FROM artifact_revision_content c \
                     JOIN artifact_revisions r ON r.id=c.revision_id \
                     JOIN artifact_revision_provenance rp \
                       ON rp.artifact_revision_id=r.id AND rp.role='content' \
                     JOIN provenance_records p ON p.id=rp.provenance_id \
                     WHERE r.id=?",
                )
                .bind(&revision)
                .fetch_one(&mut connection)
                .await
                .unwrap()
            };
            assert_eq!(before.0, raw);
            execute_disposable(
                &path,
                EvidenceWriteCommand::ConfirmPending {
                    source_id: SOURCE_ID.into(),
                    artifact_id: id.clone(),
                    expected_source_revision_id: source_revision(&path).await,
                    expected_artifact_revision_id: revision.clone(),
                },
                context(
                    "2026-08-09T01:00:00.000Z",
                    "guard-legacy-evidence-confirm-0000001",
                    EvidenceWriteFailurePoint::None,
                ),
            )
            .await
            .unwrap();
            let mut connection = connect(&path, true).await.unwrap();
            let after: (String, String, String) = sqlx::query_as(
                "SELECT c.payload, r.content_digest, p.canonical_payload \
                 FROM artifact_revision_content c \
                 JOIN artifact_revisions r ON r.id=c.revision_id \
                 JOIN artifact_revision_provenance rp \
                   ON rp.artifact_revision_id=r.id AND rp.role='content' \
                 JOIN provenance_records p ON p.id=rp.provenance_id \
                 WHERE r.id=?",
            )
            .bind(&revision)
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(after, before);
            let review: (String, String, String, String, String) = sqlx::query_as(
                "SELECT subject_revision_id, decision, actor, event_origin, occurred_at \
                 FROM artifact_review_events WHERE artifact_id=?",
            )
            .bind(&id)
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(
                review,
                (
                    revision,
                    "confirmed".into(),
                    "user".into(),
                    "explicit_user_action".into(),
                    "2026-08-09T01:00:00.000Z".into(),
                )
            );
            let head: (String, String) = sqlx::query_as(
                "SELECT review_state, eligibility_state FROM artifact_heads WHERE id=?",
            )
            .bind(&id)
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(head, ("confirmed".into(), "eligible".into()));
            let imported_review_after: (String, String, String, String) = sqlx::query_as(
                "SELECT decision, actor, event_origin, timestamp_quality \
                 FROM artifact_review_events \
                 WHERE artifact_id='legacy-imported-review-evidence'",
            )
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(imported_review_after, imported_review_before);
            if !include_provenance {
                let projection: Value = serde_json::from_str(
                    &sqlx::query_scalar::<_, String>(
                        "SELECT payload FROM persisted_artifacts WHERE id=?",
                    )
                    .bind(&id)
                    .fetch_one(&mut connection)
                    .await
                    .unwrap(),
                )
                .unwrap();
                assert!(projection.get("provenance").is_none());
                assert_eq!(
                    after.2.parse::<Value>().unwrap()["origin"],
                    "legacy_unknown"
                );
            }
        }
    }

    #[tokio::test]
    async fn migrated_legacy_candidate_rejection_purges_content_and_projection() {
        let id = "legacy-evidence-reject";
        let (_directory, path, raw) = exact_v5_legacy_evidence_fixture(id, true, false).await;
        let revision = legacy_revision(&path, id).await;
        execute_disposable(
            &path,
            EvidenceWriteCommand::RejectPending {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: revision.clone(),
            },
            context(
                "2026-08-09T01:01:00.000Z",
                "guard-legacy-evidence-reject-00000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM artifact_revision_content WHERE revision_id=?",
            )
            .bind(&revision)
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            0
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM persisted_artifacts WHERE id=?")
                .bind(id)
                .fetch_one(&mut connection)
                .await
                .unwrap(),
            0
        );
        let facts: (String, String, i64) = sqlx::query_as(
            "SELECT h.review_state, h.lifecycle_state, \
                    (SELECT COUNT(*) FROM content_tombstones WHERE artifact_id=h.id) \
             FROM artifact_heads h WHERE h.id=?",
        )
        .bind(id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(facts, ("rejected".into(), "content_purged".into(), 1));
        let retained_payloads: Vec<String> = sqlx::query_scalar(
            "SELECT payload FROM artifact_revision_content UNION ALL \
             SELECT payload FROM persisted_artifacts",
        )
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert!(!retained_payloads.iter().any(|payload| payload == &raw));
    }

    #[tokio::test]
    async fn migrated_legacy_review_rejects_unknown_shape_and_rolls_back_every_boundary() {
        let (_directory, malformed_path, _) =
            exact_v5_legacy_evidence_fixture("legacy-evidence-unknown", true, true).await;
        let malformed_revision = legacy_revision(&malformed_path, "legacy-evidence-unknown").await;
        let before = {
            let mut connection = connect(&malformed_path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_disposable(
            &malformed_path,
            EvidenceWriteCommand::ConfirmPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "legacy-evidence-unknown".into(),
                expected_source_revision_id: source_revision(&malformed_path).await,
                expected_artifact_revision_id: malformed_revision,
            },
            context(
                "2026-08-09T01:02:00.000Z",
                "guard-legacy-evidence-unknown-0000001",
                EvidenceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("evidence_legacy_payload_unknown_field"));
        let mut connection = connect(&malformed_path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        drop(connection);

        for (reject, points) in [
            (
                false,
                vec![
                    EvidenceWriteFailurePoint::AfterReview,
                    EvidenceWriteFailurePoint::AfterHead,
                    EvidenceWriteFailurePoint::AfterProjection,
                    EvidenceWriteFailurePoint::AfterReconciliation,
                    EvidenceWriteFailurePoint::AfterGuardRemoval,
                ],
            ),
            (
                true,
                vec![
                    EvidenceWriteFailurePoint::AfterReview,
                    EvidenceWriteFailurePoint::AfterHead,
                    EvidenceWriteFailurePoint::AfterLifecycle,
                    EvidenceWriteFailurePoint::AfterTombstone,
                    EvidenceWriteFailurePoint::AfterPurge,
                    EvidenceWriteFailurePoint::AfterProjection,
                    EvidenceWriteFailurePoint::AfterReconciliation,
                    EvidenceWriteFailurePoint::AfterGuardRemoval,
                ],
            ),
        ] {
            for (index, point) in points.into_iter().enumerate() {
                let id = format!("legacy-evidence-rollback-{reject}-{index}");
                let (_directory, path, raw) =
                    exact_v5_legacy_evidence_fixture(&id, true, false).await;
                let revision = legacy_revision(&path, &id).await;
                let before = {
                    let mut connection = connect(&path, true).await.unwrap();
                    operation_manifest(&mut connection).await.unwrap()
                };
                let command = if reject {
                    EvidenceWriteCommand::RejectPending {
                        source_id: SOURCE_ID.into(),
                        artifact_id: id.clone(),
                        expected_source_revision_id: source_revision(&path).await,
                        expected_artifact_revision_id: revision.clone(),
                    }
                } else {
                    EvidenceWriteCommand::ConfirmPending {
                        source_id: SOURCE_ID.into(),
                        artifact_id: id.clone(),
                        expected_source_revision_id: source_revision(&path).await,
                        expected_artifact_revision_id: revision.clone(),
                    }
                };
                assert!(execute_disposable(
                    &path,
                    command,
                    EvidenceWriteContext {
                        occurred_at: "2026-08-09T01:03:00.000Z",
                        guard_token: "guard-legacy-evidence-rollback-0000001",
                        failure_point: point,
                    },
                )
                .await
                .is_err());
                let mut connection = connect(&path, true).await.unwrap();
                assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
                assert_eq!(
                    sqlx::query_scalar::<_, String>(
                        "SELECT payload FROM artifact_revision_content WHERE revision_id=?",
                    )
                    .bind(&revision)
                    .fetch_one(&mut connection)
                    .await
                    .unwrap(),
                    raw
                );
            }
        }
    }

    #[tokio::test]
    async fn migration_receipt_and_production_schema_boundary_remain_unchanged() {
        let (_directory, path) = exact_v5_fixture().await;
        let before: (String, String) = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_as(
                "SELECT source_manifest_digest, target_manifest_digest \
                 FROM schema_migration_receipts",
            )
            .fetch_one(&mut connection)
            .await
            .unwrap()
        };
        create(&path, "evidence-receipt", "ai").await;
        let after: (String, String) = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_as(
                "SELECT source_manifest_digest, target_manifest_digest \
                 FROM schema_migration_receipts",
            )
            .fetch_one(&mut connection)
            .await
            .unwrap()
        };
        assert_eq!(before, after);
        assert_eq!(TARGET_SCHEMA_VERSION, 5);
        assert_eq!(SOURCE_SCHEMA_VERSION, 4);
    }
}
