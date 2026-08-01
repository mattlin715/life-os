use super::evidence_write::verify_exact_evidence_v5;
use super::experience_write::operation_manifest;
use super::*;
use sqlx::{raw_sql, Row};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct PromptProvenanceInput {
    origin: String,
    provider: String,
    model: Option<String>,
    harness_version: String,
    prompt_version: String,
    generated_at: String,
    source_artifact_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct EvidenceRevisionRef {
    artifact_id: String,
    revision_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SuggestedPromptInput {
    id: String,
    source_id: String,
    question: String,
    created_at: String,
    evidence: Vec<EvidenceRevisionRef>,
    provenance: PromptProvenanceInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ReflectionWriteCommand {
    CreateSuggested {
        expected_source_revision_id: String,
        prompt: SuggestedPromptInput,
    },
    SaveResponse {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        expected_evidence: Vec<EvidenceRevisionRef>,
        response: String,
    },
    CorrectResponse {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        expected_evidence: Vec<EvidenceRevisionRef>,
        response: String,
    },
    SkipSuggested {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        expected_evidence: Vec<EvidenceRevisionRef>,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ReflectionWriteFailurePoint {
    #[default]
    None,
    AfterGuard,
    AfterProvenance,
    AfterRevision,
    AfterContent,
    AfterHead,
    AfterDependency,
    AfterLifecycle,
    AfterReview,
    AfterProjection,
    AfterReconciliation,
    AfterGuardRemoval,
}

#[derive(Clone, Debug)]
struct ReflectionWriteContext<'a> {
    occurred_at: &'a str,
    guard_token: &'a str,
    failure_point: ReflectionWriteFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReflectionWriteOutcome {
    artifact_id: String,
    revision_id: String,
    operation_manifest: String,
}

#[derive(Clone, Debug)]
struct CurrentReflection {
    artifact_id: String,
    source_id: String,
    revision_id: String,
    revision_number: i64,
    review_state: String,
    lifecycle_state: String,
    eligibility_state: String,
    serialization_version: String,
    authorship: String,
    payload: Value,
    prompt_provenance_id: String,
    prompt_provenance: Value,
    response_provenance: Option<Value>,
    created_at: String,
}

fn write_error(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn recovery_error(code: impl Into<String>) -> MigrationError {
    MigrationError::recovery_required(code)
}

fn validate_identifier(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        Err(write_error(format!("reflection_{field}_invalid")))
    } else {
        Ok(())
    }
}

fn validate_timestamp(value: &str, field: &str) -> Result<(), MigrationError> {
    validate_identifier(value, field)?;
    if !value.contains('T') || !value.ends_with('Z') {
        return Err(write_error(format!("reflection_{field}_invalid")));
    }
    Ok(())
}

fn validate_text(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 100_000 {
        Err(write_error(format!("reflection_{field}_invalid")))
    } else {
        Ok(())
    }
}

fn normalize_evidence(
    evidence: &[EvidenceRevisionRef],
) -> Result<Vec<EvidenceRevisionRef>, MigrationError> {
    if evidence.is_empty() {
        return Err(write_error("reflection_evidence_required"));
    }
    let mut by_artifact = BTreeMap::new();
    for item in evidence {
        validate_identifier(&item.artifact_id, "evidence_artifact_id")?;
        validate_identifier(&item.revision_id, "evidence_revision_id")?;
        if by_artifact
            .insert(item.artifact_id.clone(), item.revision_id.clone())
            .is_some()
        {
            return Err(write_error("reflection_evidence_duplicate"));
        }
    }
    Ok(by_artifact
        .into_iter()
        .map(|(artifact_id, revision_id)| EvidenceRevisionRef {
            artifact_id,
            revision_id,
        })
        .collect())
}

fn validate_prompt(
    prompt: &SuggestedPromptInput,
) -> Result<Vec<EvidenceRevisionRef>, MigrationError> {
    validate_identifier(&prompt.id, "artifact_id")?;
    validate_identifier(&prompt.source_id, "source_id")?;
    validate_text(&prompt.question, "question")?;
    validate_timestamp(&prompt.created_at, "created_at")?;
    let evidence = normalize_evidence(&prompt.evidence)?;
    if !matches!(prompt.provenance.origin.as_str(), "ai" | "local_mock") {
        return Err(write_error("reflection_prompt_origin_invalid"));
    }
    let provider_valid = match prompt.provenance.origin.as_str() {
        "ai" => matches!(prompt.provenance.provider.as_str(), "openai" | "gemini"),
        "local_mock" => prompt.provenance.provider == "mock",
        _ => false,
    };
    if !provider_valid {
        return Err(write_error("reflection_prompt_provider_invalid"));
    }
    if prompt.provenance.origin == "ai"
        && prompt
            .provenance
            .model
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .is_none()
    {
        return Err(write_error("reflection_prompt_model_missing"));
    }
    validate_identifier(&prompt.provenance.harness_version, "harness_version")?;
    validate_identifier(&prompt.provenance.prompt_version, "prompt_version")?;
    validate_timestamp(&prompt.provenance.generated_at, "generated_at")?;
    let expected = evidence
        .iter()
        .map(|item| item.artifact_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeSet::new();
    for id in &prompt.provenance.source_artifact_ids {
        validate_identifier(id, "source_artifact_id")?;
        if !actual.insert(id.as_str()) {
            return Err(write_error("reflection_prompt_sources_duplicate"));
        }
    }
    if actual != expected {
        return Err(write_error("reflection_prompt_sources_mismatch"));
    }
    Ok(evidence)
}

fn inject(
    context: &ReflectionWriteContext<'_>,
    point: ReflectionWriteFailurePoint,
) -> Result<(), MigrationError> {
    if context.failure_point == point {
        Err(write_error(format!(
            "injected_reflection_write_failure:{point:?}"
        )))
    } else {
        Ok(())
    }
}

fn prompt_provenance_value(source_id: &str, provenance: &PromptProvenanceInput) -> Value {
    let mut source_artifact_ids = provenance.source_artifact_ids.clone();
    source_artifact_ids.sort();
    json!({
        "generatedAt": provenance.generated_at,
        "harnessVersion": provenance.harness_version,
        "model": provenance.model,
        "origin": provenance.origin,
        "promptVersion": provenance.prompt_version,
        "provider": provenance.provider,
        "sourceArtifactIds": source_artifact_ids,
        "sourceEntryId": source_id
    })
}

fn response_provenance_value(source_id: &str, artifact_id: &str, occurred_at: &str) -> Value {
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

fn content_value(
    id: &str,
    source_id: &str,
    evidence: &[EvidenceRevisionRef],
    question: &str,
    response: Option<&str>,
    created_at: &str,
) -> Value {
    json!({
        "createdAt": created_at,
        "id": id,
        "question": question,
        "response": response,
        "sourceEntryId": source_id,
        "sourceEvidenceIds": evidence.iter().map(|item| item.artifact_id.clone()).collect::<Vec<_>>()
    })
}

fn v4_projection_value(
    content: &Value,
    status: &str,
    updated_at: &str,
    prompt_provenance: &Value,
    response_provenance: Option<&Value>,
) -> Result<Value, MigrationError> {
    let object = content
        .as_object()
        .ok_or_else(|| write_error("reflection_content_not_object"))?;
    let mut projection = Map::new();
    for key in [
        "createdAt",
        "id",
        "question",
        "sourceEntryId",
        "sourceEvidenceIds",
    ] {
        projection.insert(
            key.into(),
            object
                .get(key)
                .cloned()
                .ok_or_else(|| write_error(format!("reflection_{key}_missing")))?,
        );
    }
    projection.insert("status".into(), Value::String(status.into()));
    projection.insert("updatedAt".into(), Value::String(updated_at.into()));
    projection.insert("promptProvenance".into(), prompt_provenance.clone());
    if let Some(response) = object.get("response").filter(|value| !value.is_null()) {
        projection.insert("response".into(), response.clone());
    }
    if let Some(provenance) = response_provenance {
        projection.insert("responseProvenance".into(), provenance.clone());
    }
    Ok(Value::Object(projection))
}

async fn exact_current_source(
    connection: &mut SqliteConnection,
    source_id: &str,
    expected_revision_id: &str,
) -> Result<(), MigrationError> {
    validate_identifier(source_id, "source_id")?;
    validate_identifier(expected_revision_id, "source_revision_id")?;
    let row: Option<(String, String, String, String)> = sqlx::query_as(
        "SELECT h.current_revision_id, h.lifecycle_state, c.content, e.content \
         FROM source_heads h \
         JOIN source_revision_content c ON c.revision_id = h.current_revision_id \
         JOIN experience_entries e ON e.id = h.id WHERE h.id = ?",
    )
    .bind(source_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_source_lookup_failed", error))?;
    let Some((revision, lifecycle, content, projection)) = row else {
        return Err(write_error("reflection_source_not_found"));
    };
    if revision != expected_revision_id || lifecycle != "active" || content != projection {
        return Err(write_error("reflection_source_revision_stale"));
    }
    Ok(())
}

async fn validate_current_evidence(
    connection: &mut SqliteConnection,
    source_id: &str,
    evidence: &[EvidenceRevisionRef],
) -> Result<Vec<EvidenceRevisionRef>, MigrationError> {
    let evidence = normalize_evidence(evidence)?;
    for item in &evidence {
        let row: Option<(String, String, String, String, String, String)> = sqlx::query_as(
            "SELECT h.source_id, h.current_revision_id, h.review_state, h.lifecycle_state, \
                    h.eligibility_state, pa.payload \
             FROM artifact_heads h \
             JOIN artifact_revisions r ON r.id = h.current_revision_id AND r.artifact_id = h.id \
             JOIN artifact_revision_content c ON c.revision_id = r.id \
             JOIN persisted_artifacts pa ON pa.id = h.id \
               AND pa.source_entry_id = h.source_id AND pa.artifact_kind = 'evidence' \
             WHERE h.id = ? AND h.artifact_kind = 'evidence'",
        )
        .bind(&item.artifact_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_evidence_lookup_failed", error))?;
        let Some((actual_source, revision, review, lifecycle, eligibility, projection)) = row
        else {
            return Err(write_error("reflection_evidence_not_found"));
        };
        let projected: Value = serde_json::from_str(&projection)
            .map_err(|error| migration_error("reflection_evidence_projection_malformed", error))?;
        if actual_source != source_id {
            return Err(write_error("reflection_evidence_cross_source"));
        }
        if revision != item.revision_id {
            return Err(write_error("reflection_evidence_revision_stale"));
        }
        if review != "confirmed"
            || lifecycle != "active"
            || eligibility != "eligible"
            || projected.get("status").and_then(Value::as_str) != Some("confirmed")
        {
            return Err(write_error("reflection_evidence_ineligible"));
        }
    }
    Ok(evidence)
}

async fn insert_guard(
    connection: &mut SqliteConnection,
    context: &ReflectionWriteContext<'_>,
) -> Result<(), MigrationError> {
    validate_identifier(context.guard_token, "guard_token")?;
    validate_timestamp(context.occurred_at, "occurred_at")?;
    if context.guard_token.len() < 32 {
        return Err(write_error("reflection_guard_token_invalid"));
    }
    sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
        .bind(context.guard_token)
        .bind(context.occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_guard_insert_failed", error))?;
    inject(context, ReflectionWriteFailurePoint::AfterGuard)
}

async fn remove_guard(
    connection: &mut SqliteConnection,
    context: &ReflectionWriteContext<'_>,
) -> Result<(), MigrationError> {
    let result = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
        .bind(context.guard_token)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_guard_remove_failed", error))?;
    if result.rows_affected() != 1 {
        return Err(recovery_error("reflection_guard_identity_mismatch"));
    }
    let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_guard_count_failed", error))?;
    if remaining != 0 {
        return Err(recovery_error("reflection_guard_not_empty"));
    }
    inject(context, ReflectionWriteFailurePoint::AfterGuardRemoval)
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
    sqlx::query(
        "INSERT INTO artifact_revisions (id, artifact_id, source_id, revision_number, \
           predecessor_revision_id, authorship, revision_reason, serialization_version, \
           content_digest, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, 'canonical-json-v1', ?, ?)",
    )
    .bind(revision_id)
    .bind(artifact_id)
    .bind(source_id)
    .bind(revision_number)
    .bind(predecessor)
    .bind(authorship)
    .bind(reason)
    .bind(sha256_hex(payload.as_bytes()))
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_revision_insert_failed", error))?;
    Ok(())
}

async fn insert_content(
    connection: &mut SqliteConnection,
    revision_id: &str,
    payload: &str,
) -> Result<(), MigrationError> {
    sqlx::query(
        "INSERT INTO artifact_revision_content (revision_id, payload, byte_length) VALUES (?, ?, ?)",
    )
    .bind(revision_id)
    .bind(payload)
    .bind(payload.len() as i64)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_content_insert_failed", error))?;
    Ok(())
}

async fn link_provenance(
    connection: &mut SqliteConnection,
    revision_id: &str,
    role: &str,
    provenance_id: &str,
) -> Result<(), MigrationError> {
    sqlx::query(
        "INSERT INTO artifact_revision_provenance (artifact_revision_id, role, provenance_id) \
         VALUES (?, ?, ?)",
    )
    .bind(revision_id)
    .bind(role)
    .bind(provenance_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_provenance_link_failed", error))?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn insert_dependencies(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
    source_id: &str,
    source_revision_id: &str,
    evidence: &[EvidenceRevisionRef],
    prompt_revision_id: Option<&str>,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let source_dependency = dependency_id(
        artifact_id,
        revision_id,
        "derived_from_experience",
        source_id,
        source_revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_dependencies (id, dependent_artifact_id, dependent_revision_id, \
           relationship_type, source_revision_id, source_artifact_id, \
           source_artifact_revision_id, created_at) \
         VALUES (?, ?, ?, 'derived_from_experience', ?, NULL, NULL, ?)",
    )
    .bind(source_dependency)
    .bind(artifact_id)
    .bind(revision_id)
    .bind(source_revision_id)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_source_dependency_insert_failed", error))?;

    for item in evidence {
        let id = dependency_id(
            artifact_id,
            revision_id,
            "uses_evidence",
            &item.artifact_id,
            &item.revision_id,
        );
        sqlx::query(
            "INSERT INTO artifact_dependencies (id, dependent_artifact_id, dependent_revision_id, \
               relationship_type, source_revision_id, source_artifact_id, \
               source_artifact_revision_id, created_at) \
             VALUES (?, ?, ?, 'uses_evidence', NULL, ?, ?, ?)",
        )
        .bind(id)
        .bind(artifact_id)
        .bind(revision_id)
        .bind(&item.artifact_id)
        .bind(&item.revision_id)
        .bind(occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_evidence_dependency_insert_failed", error))?;
    }
    if let Some(prompt_revision_id) = prompt_revision_id {
        let id = dependency_id(
            artifact_id,
            revision_id,
            "answers_prompt",
            artifact_id,
            prompt_revision_id,
        );
        sqlx::query(
            "INSERT INTO artifact_dependencies (id, dependent_artifact_id, dependent_revision_id, \
               relationship_type, source_revision_id, source_artifact_id, \
               source_artifact_revision_id, created_at) \
             VALUES (?, ?, ?, 'answers_prompt', NULL, ?, ?, ?)",
        )
        .bind(id)
        .bind(artifact_id)
        .bind(revision_id)
        .bind(artifact_id)
        .bind(prompt_revision_id)
        .bind(occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_prompt_dependency_insert_failed", error))?;
    }
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
        "INSERT INTO artifact_lifecycle_events (id, artifact_id, subject_revision_id, \
           related_revision_id, dependency_id, event_type, actor, reason_code, occurred_at) \
         VALUES (?, ?, ?, ?, NULL, ?, ?, ?, ?)",
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
    .map_err(|error| migration_error("reflection_lifecycle_insert_failed", error))?;
    Ok(())
}

async fn insert_skip_event(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let id = event_id(
        "v5re_",
        "life-os/reflection-skipped-review-event-id-v1",
        artifact_id,
        revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_review_events (id, artifact_id, subject_revision_id, decision, \
           actor, event_origin, occurred_at, timestamp_quality) \
         VALUES (?, ?, ?, 'skipped', 'user', 'explicit_user_action', ?, 'exact_action_time')",
    )
    .bind(id)
    .bind(artifact_id)
    .bind(revision_id)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_skip_event_insert_failed", error))?;
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
    let payload = canonical_json(payload)?;
    if insert {
        sqlx::query(
            "INSERT INTO persisted_artifacts (id, source_entry_id, artifact_kind, payload, \
               created_at, updated_at) VALUES (?, ?, 'reflection', ?, ?, ?)",
        )
        .bind(artifact_id)
        .bind(source_id)
        .bind(payload)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_projection_insert_failed", error))?;
    } else {
        let changed = sqlx::query(
            "UPDATE persisted_artifacts SET payload = ?, updated_at = ? \
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'reflection'",
        )
        .bind(payload)
        .bind(updated_at)
        .bind(artifact_id)
        .bind(source_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_projection_update_failed", error))?;
        if changed.rows_affected() != 1 {
            return Err(recovery_error("reflection_projection_identity_mismatch"));
        }
    }
    Ok(())
}

async fn current_reflection(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_revision_id: &str,
) -> Result<CurrentReflection, MigrationError> {
    validate_identifier(artifact_id, "artifact_id")?;
    validate_identifier(expected_revision_id, "artifact_revision_id")?;
    let row = sqlx::query(
        "SELECT h.source_id, h.current_revision_id, h.review_state, h.lifecycle_state, \
                h.eligibility_state, h.created_at, r.revision_number, r.serialization_version, \
                r.authorship, r.content_digest, c.payload, pa.payload, pp.provenance_id, \
                p.canonical_payload, rp.canonical_payload \
         FROM artifact_heads h \
         JOIN artifact_revisions r ON r.id = h.current_revision_id AND r.artifact_id = h.id \
         JOIN artifact_revision_content c ON c.revision_id = r.id \
         JOIN persisted_artifacts pa ON pa.id = h.id AND pa.source_entry_id = h.source_id \
           AND pa.artifact_kind = 'reflection' \
         JOIN artifact_revision_provenance pp ON pp.artifact_revision_id = r.id AND pp.role = 'prompt' \
         JOIN provenance_records p ON p.id = pp.provenance_id \
         LEFT JOIN artifact_revision_provenance rpl ON rpl.artifact_revision_id = r.id \
           AND rpl.role = 'response' \
         LEFT JOIN provenance_records rp ON rp.id = rpl.provenance_id \
         WHERE h.id = ? AND h.artifact_kind = 'reflection'",
    )
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_current_lookup_failed", error))?
    .ok_or_else(|| write_error("reflection_artifact_not_found"))?;
    let actual_source: String = row.get(0);
    let revision_id: String = row.get(1);
    if actual_source != source_id || revision_id != expected_revision_id {
        return Err(write_error("reflection_artifact_revision_stale"));
    }
    let raw_payload: String = row.get(10);
    let digest: String = row.get(9);
    if digest != sha256_hex(raw_payload.as_bytes()) {
        return Err(recovery_error("reflection_current_digest_mismatch"));
    }
    Ok(CurrentReflection {
        artifact_id: artifact_id.to_string(),
        source_id: actual_source,
        revision_id,
        revision_number: row.get(6),
        review_state: row.get(2),
        lifecycle_state: row.get(3),
        eligibility_state: row.get(4),
        serialization_version: row.get(7),
        authorship: row.get(8),
        payload: serde_json::from_str(&raw_payload)
            .map_err(|error| migration_error("reflection_current_payload_malformed", error))?,
        prompt_provenance_id: row.get(12),
        prompt_provenance: serde_json::from_str(&row.get::<String, _>(13))
            .map_err(|error| migration_error("reflection_prompt_provenance_malformed", error))?,
        response_provenance: row
            .try_get::<Option<String>, _>(14)
            .map_err(|error| migration_error("reflection_response_provenance_unreadable", error))?
            .map(|raw| serde_json::from_str(&raw))
            .transpose()
            .map_err(|error| migration_error("reflection_response_provenance_malformed", error))?,
        created_at: row.get(5),
    })
}

async fn revision_evidence(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
) -> Result<Vec<EvidenceRevisionRef>, MigrationError> {
    let rows = sqlx::query(
        "SELECT source_artifact_id, source_artifact_revision_id \
         FROM artifact_dependencies WHERE dependent_artifact_id = ? \
           AND dependent_revision_id = ? AND relationship_type = 'uses_evidence' \
         ORDER BY source_artifact_id",
    )
    .bind(artifact_id)
    .bind(revision_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_dependency_read_failed", error))?;
    normalize_evidence(
        &rows
            .into_iter()
            .map(|row| EvidenceRevisionRef {
                artifact_id: row.get(0),
                revision_id: row.get(1),
            })
            .collect::<Vec<_>>(),
    )
}

async fn initial_prompt_revision(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    current_revision_id: &str,
) -> Result<String, MigrationError> {
    let answer: Option<String> = sqlx::query_scalar(
        "SELECT source_artifact_revision_id FROM artifact_dependencies \
         WHERE dependent_artifact_id = ? AND dependent_revision_id = ? \
           AND relationship_type = 'answers_prompt' AND source_artifact_id = ?",
    )
    .bind(artifact_id)
    .bind(current_revision_id)
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_prompt_link_lookup_failed", error))?;
    if let Some(revision) = answer {
        return Ok(revision);
    }
    let revision: Option<String> = sqlx::query_scalar(
        "SELECT id FROM artifact_revisions WHERE artifact_id = ? AND revision_number = 1",
    )
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_initial_revision_lookup_failed", error))?;
    revision.ok_or_else(|| recovery_error("reflection_initial_prompt_missing"))
}

async fn validate_prompt_lineage(
    connection: &mut SqliteConnection,
    current: &CurrentReflection,
) -> Result<String, MigrationError> {
    let prompt_revision =
        initial_prompt_revision(connection, &current.artifact_id, &current.revision_id).await?;
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT c.payload, rp.provenance_id FROM artifact_revisions r \
         JOIN artifact_revision_content c ON c.revision_id = r.id \
         JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id \
           AND rp.role = 'prompt' \
         WHERE r.id = ? AND r.artifact_id = ? AND r.revision_number = 1 \
           AND r.predecessor_revision_id IS NULL",
    )
    .bind(&prompt_revision)
    .bind(&current.artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_initial_prompt_lookup_failed", error))?;
    let (prompt_payload, prompt_provenance_id) =
        row.ok_or_else(|| recovery_error("reflection_initial_prompt_missing"))?;
    let prompt_payload: Value = serde_json::from_str(&prompt_payload)
        .map_err(|error| migration_error("reflection_initial_prompt_malformed", error))?;
    for key in [
        "createdAt",
        "id",
        "question",
        "sourceEntryId",
        "sourceEvidenceIds",
    ] {
        if prompt_payload.get(key) != current.payload.get(key) {
            return Err(recovery_error(format!(
                "reflection_prompt_lineage_mismatch:{key}"
            )));
        }
    }
    if prompt_payload
        .get("response")
        .is_some_and(|value| !value.is_null())
        || prompt_provenance_id != current.prompt_provenance_id
    {
        return Err(recovery_error("reflection_prompt_lineage_mismatch"));
    }
    Ok(prompt_revision)
}

async fn inbound_dependency_count(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
) -> Result<i64, MigrationError> {
    let normalized: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_dependencies \
         WHERE source_artifact_id = ? AND source_artifact_revision_id = ? \
           AND NOT (dependent_artifact_id = ? AND relationship_type = 'answers_prompt')",
    )
    .bind(artifact_id)
    .bind(revision_id)
    .bind(artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_inbound_dependency_lookup_failed", error))?;
    let historical: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM historical_artifact_dependencies WHERE source_artifact_id = ?",
    )
    .bind(artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_historical_dependency_lookup_failed", error))?;
    Ok(normalized + historical)
}

async fn ensure_expected_evidence(
    connection: &mut SqliteConnection,
    current: &CurrentReflection,
    expected: &[EvidenceRevisionRef],
) -> Result<Vec<EvidenceRevisionRef>, MigrationError> {
    let expected = validate_current_evidence(connection, &current.source_id, expected).await?;
    let recorded =
        revision_evidence(connection, &current.artifact_id, &current.revision_id).await?;
    if expected != recorded {
        return Err(write_error("reflection_evidence_dependency_mismatch"));
    }
    Ok(expected)
}

async fn create_suggested(
    connection: &mut SqliteConnection,
    expected_source_revision_id: &str,
    prompt: SuggestedPromptInput,
    context: &ReflectionWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    let evidence = validate_prompt(&prompt)?;
    exact_current_source(connection, &prompt.source_id, expected_source_revision_id).await?;
    validate_current_evidence(connection, &prompt.source_id, &evidence).await?;
    let conflict: i64 = sqlx::query_scalar(
        "SELECT (SELECT COUNT(*) FROM artifact_heads WHERE id = ?) + \
                (SELECT COUNT(*) FROM persisted_artifacts WHERE id = ?)",
    )
    .bind(&prompt.id)
    .bind(&prompt.id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_identity_check_failed", error))?;
    if conflict != 0 {
        return Err(write_error("reflection_identity_conflict"));
    }

    let content = content_value(
        &prompt.id,
        &prompt.source_id,
        &evidence,
        &prompt.question,
        None,
        &prompt.created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id = artifact_revision_id(&prompt.id, "reflection", &prompt.created_at, &digest);
    let provenance = prompt_provenance_value(&prompt.source_id, &prompt.provenance);
    let provenance_id = insert_provenance(connection, &provenance, &prompt.created_at).await?;
    inject(context, ReflectionWriteFailurePoint::AfterProvenance)?;
    insert_revision(
        connection,
        &prompt.id,
        &prompt.source_id,
        &revision_id,
        1,
        None,
        &prompt.provenance.origin,
        "created",
        &payload,
        &prompt.created_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterRevision)?;
    insert_content(connection, &revision_id, &payload).await?;
    inject(context, ReflectionWriteFailurePoint::AfterContent)?;
    sqlx::query(
        "INSERT INTO artifact_heads (id, source_id, artifact_kind, current_revision_id, \
           review_state, lifecycle_state, eligibility_state, eligibility_reason, created_at, updated_at) \
         VALUES (?, ?, 'reflection', ?, 'pending', 'active', 'ineligible', \
           'suggested_prompt_requires_user_response', ?, ?)",
    )
    .bind(&prompt.id)
    .bind(&prompt.source_id)
    .bind(&revision_id)
    .bind(&prompt.created_at)
    .bind(&prompt.created_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_head_insert_failed", error))?;
    link_provenance(connection, &revision_id, "prompt", &provenance_id).await?;
    inject(context, ReflectionWriteFailurePoint::AfterHead)?;
    insert_dependencies(
        connection,
        &prompt.id,
        &revision_id,
        &prompt.source_id,
        expected_source_revision_id,
        &evidence,
        None,
        &prompt.created_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/reflection-suggested-event-id-v1",
        "created",
        "system",
        "reflection_prompt_suggested",
        &prompt.id,
        &revision_id,
        None,
        &prompt.created_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterLifecycle)?;
    let projection =
        v4_projection_value(&content, "suggested", &prompt.created_at, &provenance, None)?;
    write_projection(
        connection,
        &prompt.id,
        &prompt.source_id,
        &projection,
        &prompt.created_at,
        &prompt.created_at,
        true,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterProjection)?;
    Ok((prompt.id, revision_id))
}

#[allow(clippy::too_many_arguments)]
async fn save_response(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    expected_evidence: &[EvidenceRevisionRef],
    response: &str,
    context: &ReflectionWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    validate_text(response, "response")?;
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_reflection(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
        || current.serialization_version != "canonical-json-v1"
        || !matches!(current.authorship.as_str(), "ai" | "local_mock")
        || current
            .payload
            .get("response")
            .is_some_and(|value| !value.is_null())
        || current.response_provenance.is_some()
    {
        return Err(write_error("reflection_first_response_not_allowed"));
    }
    let evidence = ensure_expected_evidence(connection, &current, expected_evidence).await?;
    let question = current
        .payload
        .get("question")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("reflection_question_missing"))?;
    let created_at = current
        .payload
        .get("createdAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("reflection_created_at_missing"))?;
    let content = content_value(
        artifact_id,
        source_id,
        &evidence,
        question,
        Some(response),
        created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id = artifact_revision_id(artifact_id, "reflection", context.occurred_at, &digest);
    let response_provenance =
        response_provenance_value(source_id, artifact_id, context.occurred_at);
    let response_provenance_id =
        insert_provenance(connection, &response_provenance, context.occurred_at).await?;
    inject(context, ReflectionWriteFailurePoint::AfterProvenance)?;
    insert_revision(
        connection,
        artifact_id,
        source_id,
        &revision_id,
        current.revision_number + 1,
        Some(&current.revision_id),
        "mixed",
        "answered",
        &payload,
        context.occurred_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterRevision)?;
    insert_content(connection, &revision_id, &payload).await?;
    link_provenance(
        connection,
        &revision_id,
        "prompt",
        &current.prompt_provenance_id,
    )
    .await?;
    link_provenance(
        connection,
        &revision_id,
        "response",
        &response_provenance_id,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterContent)?;
    insert_dependencies(
        connection,
        artifact_id,
        &revision_id,
        source_id,
        expected_source_revision_id,
        &evidence,
        Some(&current.revision_id),
        context.occurred_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/reflection-answered-event-id-v1",
        "created",
        "user",
        "reflection_response_saved",
        artifact_id,
        &revision_id,
        Some(&current.revision_id),
        context.occurred_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterLifecycle)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = ?, review_state = 'not_applicable', \
           lifecycle_state = 'active', eligibility_state = 'eligible', \
           eligibility_reason = 'answered_with_current_exact_dependencies', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'reflection' AND review_state = 'pending' \
           AND lifecycle_state = 'active' AND eligibility_state = 'ineligible'",
    )
    .bind(&revision_id)
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_answer_head_update_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("reflection_artifact_revision_stale"));
    }
    inject(context, ReflectionWriteFailurePoint::AfterHead)?;
    let projection = v4_projection_value(
        &content,
        "answered",
        context.occurred_at,
        &current.prompt_provenance,
        Some(&response_provenance),
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
    inject(context, ReflectionWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.into(), revision_id))
}

#[allow(clippy::too_many_arguments)]
async fn correct_response(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    expected_evidence: &[EvidenceRevisionRef],
    response: &str,
    context: &ReflectionWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    validate_text(response, "response")?;
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_reflection(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "not_applicable"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "eligible"
        || current.serialization_version != "canonical-json-v1"
        || current.authorship != "mixed"
        || current.response_provenance.is_none()
    {
        return Err(write_error("reflection_response_correction_not_allowed"));
    }
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "reflection_inbound_dependency_requires_later_slice",
        ));
    }
    let current_response = current
        .payload
        .get("response")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("reflection_response_missing"))?;
    if current_response == response {
        return Err(write_error("reflection_response_unchanged"));
    }
    let evidence = ensure_expected_evidence(connection, &current, expected_evidence).await?;
    let question = current
        .payload
        .get("question")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("reflection_question_missing"))?;
    let created_at = current
        .payload
        .get("createdAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("reflection_created_at_missing"))?;
    let prompt_revision = validate_prompt_lineage(connection, &current).await?;
    let content = content_value(
        artifact_id,
        source_id,
        &evidence,
        question,
        Some(response),
        created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id = artifact_revision_id(artifact_id, "reflection", context.occurred_at, &digest);
    let response_provenance =
        response_provenance_value(source_id, artifact_id, context.occurred_at);
    let response_provenance_id =
        insert_provenance(connection, &response_provenance, context.occurred_at).await?;
    inject(context, ReflectionWriteFailurePoint::AfterProvenance)?;
    insert_revision(
        connection,
        artifact_id,
        source_id,
        &revision_id,
        current.revision_number + 1,
        Some(&current.revision_id),
        "mixed",
        "corrected",
        &payload,
        context.occurred_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterRevision)?;
    insert_content(connection, &revision_id, &payload).await?;
    link_provenance(
        connection,
        &revision_id,
        "prompt",
        &current.prompt_provenance_id,
    )
    .await?;
    link_provenance(
        connection,
        &revision_id,
        "response",
        &response_provenance_id,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterContent)?;
    insert_dependencies(
        connection,
        artifact_id,
        &revision_id,
        source_id,
        expected_source_revision_id,
        &evidence,
        Some(&prompt_revision),
        context.occurred_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/reflection-corrected-event-id-v1",
        "corrected",
        "user",
        "reflection_response_corrected",
        artifact_id,
        &revision_id,
        Some(&current.revision_id),
        context.occurred_at,
    )
    .await?;
    insert_lifecycle_event(
        connection,
        "life-os/reflection-superseded-event-id-v1",
        "superseded",
        "user",
        "reflection_response_superseded",
        artifact_id,
        &current.revision_id,
        Some(&revision_id),
        context.occurred_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterLifecycle)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = ?, review_state = 'not_applicable', \
           lifecycle_state = 'active', eligibility_state = 'eligible', \
           eligibility_reason = 'answered_with_current_exact_dependencies', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'reflection' AND review_state = 'not_applicable' \
           AND lifecycle_state = 'active' AND eligibility_state = 'eligible'",
    )
    .bind(&revision_id)
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_correction_head_update_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("reflection_artifact_revision_stale"));
    }
    inject(context, ReflectionWriteFailurePoint::AfterHead)?;
    let projection = v4_projection_value(
        &content,
        "answered",
        context.occurred_at,
        &current.prompt_provenance,
        Some(&response_provenance),
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
    inject(context, ReflectionWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.into(), revision_id))
}

#[allow(clippy::too_many_arguments)]
async fn skip_suggested(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    expected_evidence: &[EvidenceRevisionRef],
    context: &ReflectionWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_reflection(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
        || current.serialization_version != "canonical-json-v1"
        || !matches!(current.authorship.as_str(), "ai" | "local_mock")
        || current.response_provenance.is_some()
    {
        return Err(write_error("reflection_skip_not_allowed"));
    }
    ensure_expected_evidence(connection, &current, expected_evidence).await?;
    insert_skip_event(
        connection,
        artifact_id,
        &current.revision_id,
        context.occurred_at,
    )
    .await?;
    inject(context, ReflectionWriteFailurePoint::AfterReview)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET review_state = 'skipped', lifecycle_state = 'active', \
           eligibility_state = 'ineligible', eligibility_reason = 'explicitly_skipped', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'reflection' AND review_state = 'pending' \
           AND lifecycle_state = 'active' AND eligibility_state = 'ineligible'",
    )
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_skip_head_update_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("reflection_artifact_revision_stale"));
    }
    inject(context, ReflectionWriteFailurePoint::AfterHead)?;
    let projection = v4_projection_value(
        &current.payload,
        "skipped",
        context.occurred_at,
        &current.prompt_provenance,
        None,
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
    inject(context, ReflectionWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.into(), current.revision_id))
}

async fn apply_command(
    connection: &mut SqliteConnection,
    command: ReflectionWriteCommand,
    context: &ReflectionWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    match command {
        ReflectionWriteCommand::CreateSuggested {
            expected_source_revision_id,
            prompt,
        } => create_suggested(connection, &expected_source_revision_id, prompt, context).await,
        ReflectionWriteCommand::SaveResponse {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            expected_evidence,
            response,
        } => {
            save_response(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                &expected_evidence,
                &response,
                context,
            )
            .await
        }
        ReflectionWriteCommand::CorrectResponse {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            expected_evidence,
            response,
        } => {
            correct_response(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                &expected_evidence,
                &response,
                context,
            )
            .await
        }
        ReflectionWriteCommand::SkipSuggested {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            expected_evidence,
        } => {
            skip_suggested(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                &expected_evidence,
                context,
            )
            .await
        }
    }
}

async fn verify_reflection_projection(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    let heads = sqlx::query(
        "SELECT h.id, h.source_id, h.current_revision_id, h.review_state, h.lifecycle_state, \
                h.eligibility_state, h.created_at, h.updated_at \
         FROM artifact_heads h WHERE h.artifact_kind = 'reflection' ORDER BY h.id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_heads_unreadable", error))?;
    let mut projected = 0_i64;
    for head in heads {
        let artifact_id: String = head.get(0);
        let source_id: String = head.get(1);
        let revision_id: Option<String> = head.get(2);
        let review_state: String = head.get(3);
        let lifecycle_state: String = head.get(4);
        let eligibility_state: String = head.get(5);
        if lifecycle_state == "invalidated" {
            let revision_id = revision_id
                .ok_or_else(|| recovery_error("reflection_invalidated_revision_missing"))?;
            if eligibility_state != "ineligible" {
                return Err(recovery_error(
                    "reflection_invalidated_eligibility_mismatch",
                ));
            }
            let retained: Option<(String, String, i64)> = sqlx::query_as(
                "SELECT r.content_digest, c.payload, c.byte_length \
                 FROM artifact_revisions r \
                 JOIN artifact_revision_content c ON c.revision_id = r.id \
                 WHERE r.id = ? AND r.artifact_id = ?",
            )
            .bind(&revision_id)
            .bind(&artifact_id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(|error| migration_error("reflection_invalidated_content_unreadable", error))?;
            let (digest, content, byte_length) =
                retained.ok_or_else(|| recovery_error("reflection_invalidated_content_missing"))?;
            if digest != sha256_hex(content.as_bytes()) || byte_length != content.len() as i64 {
                return Err(recovery_error(
                    "reflection_invalidated_content_digest_mismatch",
                ));
            }
            let facts: (i64, i64, i64) = sqlx::query_as(
                "SELECT \
                   (SELECT COUNT(*) FROM persisted_artifacts \
                     WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'reflection'), \
                   (SELECT COUNT(*) FROM artifact_revision_provenance \
                     WHERE artifact_revision_id = ?), \
                   (SELECT COUNT(*) FROM artifact_lifecycle_events e \
                     JOIN artifact_dependencies d ON d.id = e.dependency_id \
                     JOIN artifact_heads s ON s.id = d.source_artifact_id \
                     WHERE e.artifact_id = ? AND e.subject_revision_id = ? \
                       AND e.event_type = 'invalidated' \
                       AND d.dependent_artifact_id = ? AND d.dependent_revision_id = ? \
                       AND d.relationship_type = 'uses_evidence' \
                       AND (s.current_revision_id <> d.source_artifact_revision_id \
                         OR s.current_revision_id IS NULL OR s.lifecycle_state <> 'active' \
                         OR s.eligibility_state <> 'eligible'))",
            )
            .bind(&artifact_id)
            .bind(&source_id)
            .bind(&revision_id)
            .bind(&artifact_id)
            .bind(&revision_id)
            .bind(&artifact_id)
            .bind(&revision_id)
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| migration_error("reflection_invalidated_facts_unreadable", error))?;
            if facts.0 != 0 || facts.1 == 0 || facts.2 == 0 {
                return Err(recovery_error("reflection_invalidated_facts_mismatch"));
            }
            continue;
        }
        if lifecycle_state != "active" {
            return Err(recovery_error("reflection_lifecycle_unsupported"));
        }
        let revision_id =
            revision_id.ok_or_else(|| recovery_error("reflection_active_revision_missing"))?;
        let revision: Option<(String, String, String, String, i64)> = sqlx::query_as(
            "SELECT r.serialization_version, r.authorship, r.content_digest, c.payload, c.byte_length \
             FROM artifact_revisions r JOIN artifact_revision_content c ON c.revision_id = r.id \
             WHERE r.id = ? AND r.artifact_id = ?",
        )
        .bind(&revision_id)
        .bind(&artifact_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_revision_unreadable", error))?;
        let (serialization, authorship, digest, content, byte_length) =
            revision.ok_or_else(|| recovery_error("reflection_active_content_missing"))?;
        if digest != sha256_hex(content.as_bytes()) || byte_length != content.len() as i64 {
            return Err(recovery_error("reflection_content_digest_mismatch"));
        }
        let projection: Option<(String, String, String)> = sqlx::query_as(
            "SELECT payload, created_at, updated_at FROM persisted_artifacts \
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'reflection'",
        )
        .bind(&artifact_id)
        .bind(&source_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_projection_unreadable", error))?;
        let (projection, created_at, updated_at) =
            projection.ok_or_else(|| recovery_error("reflection_projection_missing"))?;
        projected += 1;
        if created_at != head.get::<String, _>(6) || updated_at != head.get::<String, _>(7) {
            return Err(recovery_error("reflection_projection_timestamp_mismatch"));
        }
        if serialization == "legacy-v4-raw" {
            if content.as_bytes() != projection.as_bytes() {
                return Err(recovery_error("reflection_legacy_projection_mismatch"));
            }
            continue;
        }
        if serialization != "canonical-json-v1" {
            return Err(recovery_error(
                "reflection_serialization_version_unsupported",
            ));
        }
        let content_value: Value = serde_json::from_str(&content)
            .map_err(|error| migration_error("reflection_content_malformed", error))?;
        let projection_value: Value = serde_json::from_str(&projection)
            .map_err(|error| migration_error("reflection_projection_malformed", error))?;
        for key in [
            "createdAt",
            "id",
            "question",
            "sourceEntryId",
            "sourceEvidenceIds",
        ] {
            if content_value.get(key) != projection_value.get(key) {
                return Err(recovery_error(format!(
                    "reflection_projection_field_mismatch:{key}"
                )));
            }
        }
        let prompt_rows = sqlx::query(
            "SELECT p.canonical_payload FROM artifact_revision_provenance rp \
             JOIN provenance_records p ON p.id = rp.provenance_id \
             WHERE rp.artifact_revision_id = ? AND rp.role = 'prompt'",
        )
        .bind(&revision_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_prompt_provenance_unreadable", error))?;
        if prompt_rows.len() != 1 {
            return Err(recovery_error(
                "reflection_prompt_provenance_count_mismatch",
            ));
        }
        let prompt_provenance: Value = serde_json::from_str(&prompt_rows[0].get::<String, _>(0))
            .map_err(|error| migration_error("reflection_prompt_provenance_malformed", error))?;
        if projection_value.get("promptProvenance") != Some(&prompt_provenance) {
            return Err(recovery_error(
                "reflection_prompt_provenance_projection_mismatch",
            ));
        }
        let response_rows = sqlx::query(
            "SELECT p.canonical_payload FROM artifact_revision_provenance rp \
             JOIN provenance_records p ON p.id = rp.provenance_id \
             WHERE rp.artifact_revision_id = ? AND rp.role = 'response'",
        )
        .bind(&revision_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_response_provenance_unreadable", error))?;
        let expected_status = match (review_state.as_str(), eligibility_state.as_str()) {
            ("pending", "ineligible") => {
                if !matches!(authorship.as_str(), "ai" | "local_mock")
                    || content_value
                        .get("response")
                        .is_some_and(|value| !value.is_null())
                    || !response_rows.is_empty()
                {
                    return Err(recovery_error("reflection_suggested_state_mismatch"));
                }
                "suggested"
            }
            ("skipped", "ineligible") => {
                if !matches!(authorship.as_str(), "ai" | "local_mock")
                    || content_value
                        .get("response")
                        .is_some_and(|value| !value.is_null())
                    || !response_rows.is_empty()
                {
                    return Err(recovery_error("reflection_skipped_state_mismatch"));
                }
                let reviews: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM artifact_review_events WHERE artifact_id = ? \
                     AND subject_revision_id = ? AND decision = 'skipped'",
                )
                .bind(&artifact_id)
                .bind(&revision_id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| migration_error("reflection_skip_review_unreadable", error))?;
                if reviews != 1 {
                    return Err(recovery_error("reflection_skip_review_missing"));
                }
                "skipped"
            }
            ("not_applicable", "eligible") => {
                let response = content_value
                    .get("response")
                    .and_then(Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                    .ok_or_else(|| recovery_error("reflection_answer_response_missing"))?;
                if response.is_empty() || authorship != "mixed" || response_rows.len() != 1 {
                    return Err(recovery_error("reflection_answer_state_mismatch"));
                }
                let response_provenance: Value =
                    serde_json::from_str(&response_rows[0].get::<String, _>(0)).map_err(
                        |error| migration_error("reflection_response_provenance_malformed", error),
                    )?;
                if response_provenance.get("origin").and_then(Value::as_str) != Some("user")
                    || projection_value.get("responseProvenance") != Some(&response_provenance)
                    || projection_value.get("response") != content_value.get("response")
                {
                    return Err(recovery_error(
                        "reflection_response_provenance_projection_mismatch",
                    ));
                }
                let prompt_targets: Vec<String> = sqlx::query_scalar(
                    "SELECT source_artifact_revision_id FROM artifact_dependencies \
                     WHERE dependent_artifact_id = ? \
                     AND dependent_revision_id = ? AND relationship_type = 'answers_prompt' \
                     AND source_artifact_id = ?",
                )
                .bind(&artifact_id)
                .bind(&revision_id)
                .bind(&artifact_id)
                .fetch_all(&mut *connection)
                .await
                .map_err(|error| migration_error("reflection_answers_prompt_unreadable", error))?;
                if prompt_targets.len() != 1 {
                    return Err(recovery_error("reflection_answers_prompt_mismatch"));
                }
                let initial: Option<(String, String)> = sqlx::query_as(
                    "SELECT c.payload, rp.provenance_id FROM artifact_revisions r \
                     JOIN artifact_revision_content c ON c.revision_id = r.id \
                     JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id \
                       AND rp.role = 'prompt' \
                     WHERE r.id = ? AND r.artifact_id = ? AND r.revision_number = 1",
                )
                .bind(&prompt_targets[0])
                .bind(&artifact_id)
                .fetch_optional(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("reflection_initial_prompt_verification_failed", error)
                })?;
                let (initial_payload, initial_provenance_id) =
                    initial.ok_or_else(|| recovery_error("reflection_initial_prompt_missing"))?;
                let initial_payload: Value =
                    serde_json::from_str(&initial_payload).map_err(|error| {
                        migration_error("reflection_initial_prompt_malformed", error)
                    })?;
                let current_prompt_provenance_id: String = sqlx::query_scalar(
                    "SELECT provenance_id FROM artifact_revision_provenance \
                     WHERE artifact_revision_id = ? AND role = 'prompt'",
                )
                .bind(&revision_id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("reflection_prompt_provenance_id_unreadable", error)
                })?;
                for key in [
                    "createdAt",
                    "id",
                    "question",
                    "sourceEntryId",
                    "sourceEvidenceIds",
                ] {
                    if initial_payload.get(key) != content_value.get(key) {
                        return Err(recovery_error(format!(
                            "reflection_prompt_lineage_mismatch:{key}"
                        )));
                    }
                }
                if initial_payload
                    .get("response")
                    .is_some_and(|value| !value.is_null())
                    || initial_provenance_id != current_prompt_provenance_id
                {
                    return Err(recovery_error("reflection_prompt_lineage_mismatch"));
                }
                "answered"
            }
            _ => return Err(recovery_error("reflection_review_eligibility_mismatch")),
        };
        if projection_value.get("status").and_then(Value::as_str) != Some(expected_status) {
            return Err(recovery_error("reflection_projection_status_mismatch"));
        }
        let source_dependencies: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM artifact_dependencies d JOIN source_heads h \
             ON h.current_revision_id = d.source_revision_id AND h.id = ? \
             WHERE d.dependent_artifact_id = ? AND d.dependent_revision_id = ? \
               AND d.relationship_type = 'derived_from_experience'",
        )
        .bind(&source_id)
        .bind(&artifact_id)
        .bind(&revision_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_source_dependency_unreadable", error))?;
        if source_dependencies != 1 {
            return Err(recovery_error("reflection_source_dependency_mismatch"));
        }
        let evidence_dependencies: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM artifact_dependencies d JOIN artifact_heads h \
               ON h.id = d.source_artifact_id AND h.current_revision_id = d.source_artifact_revision_id \
             WHERE d.dependent_artifact_id = ? AND d.dependent_revision_id = ? \
               AND d.relationship_type = 'uses_evidence' AND h.source_id = ? \
               AND h.artifact_kind = 'evidence' AND h.review_state = 'confirmed' \
               AND h.lifecycle_state = 'active' AND h.eligibility_state = 'eligible'",
        )
        .bind(&artifact_id)
        .bind(&revision_id)
        .bind(&source_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("reflection_evidence_dependencies_unreadable", error))?;
        let represented = content_value
            .get("sourceEvidenceIds")
            .and_then(Value::as_array)
            .map(|items| items.len() as i64)
            .ok_or_else(|| recovery_error("reflection_evidence_ids_missing"))?;
        if evidence_dependencies == 0 || evidence_dependencies != represented {
            return Err(recovery_error("reflection_evidence_dependencies_mismatch"));
        }
    }
    let projection_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM persisted_artifacts WHERE artifact_kind = 'reflection'",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("reflection_projection_count_failed", error))?;
    if projection_count != projected {
        return Err(recovery_error("reflection_projection_count_mismatch"));
    }
    Ok(())
}

pub(super) async fn verify_exact_reflection_v5(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    verify_exact_evidence_v5(connection).await?;
    verify_reflection_projection(connection).await
}

async fn prepare_write(
    connection: &mut SqliteConnection,
    command: ReflectionWriteCommand,
    context: &ReflectionWriteContext<'_>,
) -> Result<ReflectionWriteOutcome, MigrationError> {
    insert_guard(connection, context).await?;
    let (artifact_id, revision_id) = apply_command(connection, command, context).await?;
    verify_reflection_projection(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    inject(context, ReflectionWriteFailurePoint::AfterReconciliation)?;
    remove_guard(connection, context).await?;
    verify_exact_reflection_v5(connection).await?;
    let post_manifest = operation_manifest(connection).await?;
    Ok(ReflectionWriteOutcome {
        artifact_id,
        revision_id,
        operation_manifest: post_manifest,
    })
}

async fn verify_read_only(path: &Path, expected_manifest: &str) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await?;
    verify_exact_reflection_v5(&mut connection).await?;
    let actual = operation_manifest(&mut connection).await?;
    if actual != expected_manifest {
        return Err(recovery_error("reflection_operation_manifest_mismatch"));
    }
    Ok(())
}

async fn execute_with_adapter<A: CommitOutcomeAdapter>(
    path: &Path,
    command: ReflectionWriteCommand,
    context: ReflectionWriteContext<'_>,
    adapter: &A,
) -> Result<ReflectionWriteOutcome, MigrationError> {
    let mut connection = connect(path, false).await?;
    verify_exact_reflection_v5(&mut connection).await?;
    let pre_manifest = operation_manifest(&mut connection).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("reflection_begin_failed", error))?;
    let prepared = match prepare_write(&mut connection, command, &context).await {
        Ok(prepared) => prepared,
        Err(error) => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verify| {
                    recovery_error(format!(
                        "reflection_precommit_state_unverified:{}:{rollback:?}",
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
                        "reflection_definite_noncommit_unverified:{error_class}:{}:{rollback:?}",
                        verify.code
                    ))
                })?;
            Err(write_error(format!(
                "reflection_commit_definitely_not_committed:{error_class}"
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
                    "reflection_commit_outcome_unknown_unchanged:{error_class}"
                )));
            }
            Err(recovery_error(format!(
                "reflection_commit_outcome_unknown:{error_class}"
            )))
        }
    }
}

async fn execute_disposable(
    path: &Path,
    command: ReflectionWriteCommand,
    context: ReflectionWriteContext<'_>,
) -> Result<ReflectionWriteOutcome, MigrationError> {
    execute_with_adapter(path, command, context, &SqlCommitOutcomeAdapter).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const SOURCE_ID: &str = "fixture-v4-history";
    const EVIDENCE_ID: &str = "fixture-v4-evidence";
    const STARTED_AT: &str = "2026-07-31T01:00:00.000Z";
    const COMMITTED_AT: &str = "2026-07-31T01:00:01.000Z";

    async fn exact_v5_fixture() -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        let expected_source_manifest_digest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        drop(connection);
        migrate_disposable_v4(MigrationRequest {
            path: &path,
            expected_source_manifest_digest,
            started_at: STARTED_AT,
            committed_at: COMMITTED_AT,
            backup_id: Some("slice4b2-fixture-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        (directory, path)
    }

    async fn source_revision(path: &Path) -> String {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar("SELECT current_revision_id FROM source_heads WHERE id = ?")
            .bind(SOURCE_ID)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    async fn evidence_ref(path: &Path) -> EvidenceRevisionRef {
        let mut connection = connect(path, true).await.unwrap();
        EvidenceRevisionRef {
            artifact_id: EVIDENCE_ID.into(),
            revision_id: sqlx::query_scalar(
                "SELECT current_revision_id FROM artifact_heads WHERE id = ? \
                 AND artifact_kind = 'evidence' AND review_state = 'confirmed' \
                 AND lifecycle_state = 'active' AND eligibility_state = 'eligible'",
            )
            .bind(EVIDENCE_ID)
            .fetch_one(&mut connection)
            .await
            .unwrap(),
        }
    }

    fn provenance(origin: &str) -> PromptProvenanceInput {
        PromptProvenanceInput {
            origin: origin.into(),
            provider: if origin == "ai" { "gemini" } else { "mock" }.into(),
            model: if origin == "ai" {
                Some("fixture-model".into())
            } else {
                None
            },
            harness_version: "harness-v1".into(),
            prompt_version: "reflection-v1".into(),
            generated_at: "2026-07-31T02:00:00.000Z".into(),
            source_artifact_ids: vec![EVIDENCE_ID.into()],
        }
    }

    fn prompt(id: &str, origin: &str, evidence: EvidenceRevisionRef) -> SuggestedPromptInput {
        SuggestedPromptInput {
            id: id.into(),
            source_id: SOURCE_ID.into(),
            question: "What feels important to notice?".into(),
            created_at: "2026-07-31T02:00:00.000Z".into(),
            evidence: vec![evidence],
            provenance: provenance(origin),
        }
    }

    fn context(
        occurred_at: &'static str,
        guard_token: &'static str,
        failure_point: ReflectionWriteFailurePoint,
    ) -> ReflectionWriteContext<'static> {
        ReflectionWriteContext {
            occurred_at,
            guard_token,
            failure_point,
        }
    }

    async fn create(
        path: &Path,
        id: &str,
        origin: &str,
    ) -> (ReflectionWriteOutcome, EvidenceRevisionRef, String) {
        let source = source_revision(path).await;
        let evidence = evidence_ref(path).await;
        let outcome = execute_disposable(
            path,
            ReflectionWriteCommand::CreateSuggested {
                expected_source_revision_id: source.clone(),
                prompt: prompt(id, origin, evidence.clone()),
            },
            context(
                "2026-07-31T02:00:00.000Z",
                "guard-reflection-create-00000000000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        (outcome, evidence, source)
    }

    async fn answer(
        path: &Path,
        id: &str,
        prompt_revision: &str,
        evidence: &EvidenceRevisionRef,
        source: &str,
    ) -> ReflectionWriteOutcome {
        execute_disposable(
            path,
            ReflectionWriteCommand::SaveResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source.into(),
                expected_artifact_revision_id: prompt_revision.into(),
                expected_evidence: vec![evidence.clone()],
                response: "I want to respond in my own words.".into(),
            },
            context(
                "2026-07-31T02:01:00.000Z",
                "guard-reflection-answer-00000000000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn ai_and_local_mock_suggestions_preserve_exact_authorship_and_dependencies() {
        for origin in ["ai", "local_mock"] {
            let (_directory, path) = exact_v5_fixture().await;
            let id = format!("reflection-{origin}");
            let (outcome, evidence, source) = create(&path, &id, origin).await;
            let mut connection = connect(&path, true).await.unwrap();
            let row: (String, String, String, String) = sqlx::query_as(
                "SELECT r.authorship, h.review_state, h.eligibility_state, pa.payload \
                 FROM artifact_heads h JOIN artifact_revisions r ON r.id = h.current_revision_id \
                 JOIN persisted_artifacts pa ON pa.id = h.id WHERE h.id = ?",
            )
            .bind(&id)
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(row.0, origin);
            assert_eq!((row.1.as_str(), row.2.as_str()), ("pending", "ineligible"));
            let projected: Value = serde_json::from_str(&row.3).unwrap();
            assert_eq!(projected["status"], "suggested");
            assert!(projected.get("response").is_none());
            let dependencies: Vec<(String, Option<String>, Option<String>)> = sqlx::query_as(
                "SELECT relationship_type, source_revision_id, source_artifact_revision_id \
                 FROM artifact_dependencies WHERE dependent_revision_id = ? \
                 ORDER BY relationship_type",
            )
            .bind(&outcome.revision_id)
            .fetch_all(&mut connection)
            .await
            .unwrap();
            assert!(dependencies.contains(&("derived_from_experience".into(), Some(source), None)));
            assert!(dependencies.contains(&(
                "uses_evidence".into(),
                None,
                Some(evidence.revision_id)
            )));
            verify_exact_reflection_v5(&mut connection).await.unwrap();
        }
    }

    #[tokio::test]
    async fn answer_and_correction_append_mixed_revisions_with_split_provenance() {
        let (_directory, path) = exact_v5_fixture().await;
        let id = "reflection-answer-correct";
        let (created, evidence, source) = create(&path, id, "ai").await;
        let answered = answer(&path, id, &created.revision_id, &evidence, &source).await;
        let corrected = execute_disposable(
            &path,
            ReflectionWriteCommand::CorrectResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: answered.revision_id.clone(),
                expected_evidence: vec![evidence],
                response: "I revised only my response.".into(),
            },
            context(
                "2026-07-31T02:02:00.000Z",
                "guard-reflection-correct-0000000000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let revisions: Vec<(i64, String, String, Option<String>)> = sqlx::query_as(
            "SELECT revision_number, authorship, revision_reason, predecessor_revision_id \
             FROM artifact_revisions WHERE artifact_id = ? ORDER BY revision_number",
        )
        .bind(id)
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert_eq!(revisions.len(), 3);
        assert_eq!(
            (revisions[1].1.as_str(), revisions[1].2.as_str()),
            ("mixed", "answered")
        );
        assert_eq!(
            (revisions[2].1.as_str(), revisions[2].2.as_str()),
            ("mixed", "corrected")
        );
        let roles: Vec<String> = sqlx::query_scalar(
            "SELECT role FROM artifact_revision_provenance WHERE artifact_revision_id = ? ORDER BY role",
        )
        .bind(&corrected.revision_id)
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert_eq!(roles, vec!["prompt", "response"]);
        let prompt_target: String = sqlx::query_scalar(
            "SELECT source_artifact_revision_id FROM artifact_dependencies \
             WHERE dependent_revision_id = ? AND relationship_type = 'answers_prompt'",
        )
        .bind(&corrected.revision_id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(prompt_target, created.revision_id);
        let lifecycle: Vec<String> = sqlx::query_scalar(
            "SELECT event_type FROM artifact_lifecycle_events WHERE artifact_id = ? \
             AND event_type IN ('corrected','superseded') ORDER BY event_type",
        )
        .bind(id)
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert_eq!(lifecycle, vec!["corrected", "superseded"]);
        let previous_content: String = sqlx::query_scalar(
            "SELECT payload FROM artifact_revision_content WHERE revision_id = ?",
        )
        .bind(answered.revision_id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert!(previous_content.contains("I want to respond in my own words."));
        verify_exact_reflection_v5(&mut connection).await.unwrap();
    }

    #[tokio::test]
    async fn explicit_skip_keeps_prompt_revision_and_creates_no_response_provenance() {
        let (_directory, path) = exact_v5_fixture().await;
        let id = "reflection-skip";
        let (created, evidence, source) = create(&path, id, "local_mock").await;
        let skipped = execute_disposable(
            &path,
            ReflectionWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: created.revision_id.clone(),
                expected_evidence: vec![evidence.clone()],
            },
            context(
                "2026-07-31T02:01:00.000Z",
                "guard-reflection-skip-000000000000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(skipped.revision_id, created.revision_id);
        let mut connection = connect(&path, true).await.unwrap();
        let row: (String, String, i64, i64) = sqlx::query_as(
            "SELECT h.review_state, h.eligibility_state, \
               (SELECT COUNT(*) FROM artifact_revisions r WHERE r.artifact_id = h.id), \
               (SELECT COUNT(*) FROM artifact_revision_provenance rp \
                  WHERE rp.artifact_revision_id = h.current_revision_id AND rp.role = 'response') \
             FROM artifact_heads h WHERE h.id = ?",
        )
        .bind(id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(row, ("skipped".into(), "ineligible".into(), 1, 0));
        let duplicate = execute_disposable(
            &path,
            ReflectionWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: created.revision_id,
                expected_evidence: vec![evidence],
            },
            context(
                "2026-07-31T02:02:00.000Z",
                "guard-reflection-skip-duplicate-0000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(duplicate.code, "reflection_skip_not_allowed");
    }

    #[tokio::test]
    async fn stale_cross_source_ineligible_duplicate_and_blank_inputs_fail_closed() {
        let (_directory, path) = exact_v5_fixture().await;
        let evidence = evidence_ref(&path).await;
        let source = source_revision(&path).await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let cases = vec![
            SuggestedPromptInput {
                evidence: Vec::new(),
                ..prompt("reflection-invalid-empty", "ai", evidence.clone())
            },
            SuggestedPromptInput {
                question: "  ".into(),
                ..prompt("reflection-invalid-blank", "ai", evidence.clone())
            },
            SuggestedPromptInput {
                evidence: vec![evidence.clone(), evidence.clone()],
                ..prompt("reflection-invalid-duplicate", "ai", evidence.clone())
            },
            SuggestedPromptInput {
                source_id: "different-source".into(),
                ..prompt("reflection-invalid-cross", "ai", evidence.clone())
            },
        ];
        for (index, candidate) in cases.into_iter().enumerate() {
            let result = execute_disposable(
                &path,
                ReflectionWriteCommand::CreateSuggested {
                    expected_source_revision_id: source.clone(),
                    prompt: candidate,
                },
                context(
                    "2026-07-31T02:03:00.000Z",
                    match index {
                        0 => "guard-reflection-invalid-0000000000001",
                        1 => "guard-reflection-invalid-0000000000002",
                        2 => "guard-reflection-invalid-0000000000003",
                        _ => "guard-reflection-invalid-0000000000004",
                    },
                    ReflectionWriteFailurePoint::None,
                ),
            )
            .await;
            assert!(result.is_err());
            let mut connection = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        }
    }

    #[tokio::test]
    async fn stale_rejected_deleted_and_cross_source_evidence_fail_closed() {
        for condition in ["stale", "rejected", "deleted", "cross_source"] {
            let (_directory, path) = exact_v5_fixture().await;
            let evidence = evidence_ref(&path).await;
            let mut source_id = SOURCE_ID.to_string();
            let mut source_revision_id = source_revision(&path).await;
            let mut expected_evidence = evidence.clone();
            match condition {
                "stale" => expected_evidence.revision_id = "stale-evidence-revision".into(),
                "rejected" => {
                    let mut connection = connect(&path, false).await.unwrap();
                    sqlx::query(
                        "UPDATE artifact_heads SET review_state = 'rejected', \
                         eligibility_state = 'ineligible', eligibility_reason = 'test_rejected' \
                         WHERE id = ?",
                    )
                    .bind(EVIDENCE_ID)
                    .execute(&mut connection)
                    .await
                    .unwrap();
                }
                "deleted" => {
                    let mut connection = connect(&path, false).await.unwrap();
                    raw_sql("BEGIN IMMEDIATE")
                        .execute(&mut connection)
                        .await
                        .unwrap();
                    sqlx::query(
                        "INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)",
                    )
                    .bind("guard-reflection-delete-fixture-000001")
                    .bind("2026-07-31T02:03:00.000Z")
                    .execute(&mut connection)
                    .await
                    .unwrap();
                    sqlx::query("DELETE FROM persisted_artifacts WHERE id = ?")
                        .bind(EVIDENCE_ID)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                    sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
                        .bind("guard-reflection-delete-fixture-000001")
                        .execute(&mut connection)
                        .await
                        .unwrap();
                    raw_sql("COMMIT").execute(&mut connection).await.unwrap();
                }
                "cross_source" => {
                    source_id = "fixture-v4-current".into();
                    let mut connection = connect(&path, true).await.unwrap();
                    source_revision_id = sqlx::query_scalar(
                        "SELECT current_revision_id FROM source_heads WHERE id = ?",
                    )
                    .bind(&source_id)
                    .fetch_one(&mut connection)
                    .await
                    .unwrap();
                }
                _ => unreachable!(),
            }
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            let mut candidate = prompt(
                &format!("reflection-evidence-{condition}"),
                "ai",
                expected_evidence,
            );
            candidate.source_id = source_id;
            let error = execute_disposable(
                &path,
                ReflectionWriteCommand::CreateSuggested {
                    expected_source_revision_id: source_revision_id,
                    prompt: candidate,
                },
                context(
                    "2026-07-31T02:04:00.000Z",
                    match condition {
                        "stale" => "guard-reflection-state-stale-000000001",
                        "rejected" => "guard-reflection-state-rejected-00001",
                        "deleted" => "guard-reflection-state-deleted-000001",
                        _ => "guard-reflection-state-cross-00000001",
                    },
                    ReflectionWriteFailurePoint::None,
                ),
            )
            .await
            .unwrap_err();
            assert!(
                error.code.contains("evidence"),
                "{condition}: {}",
                error.code
            );
            let mut connection = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        }
    }

    #[tokio::test]
    async fn answer_and_skip_are_mutually_exclusive_and_stale_heads_do_not_advance() {
        let (_directory, path) = exact_v5_fixture().await;
        let (created, evidence, source) = create(&path, "reflection-terminal-skip", "ai").await;
        execute_disposable(
            &path,
            ReflectionWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: "reflection-terminal-skip".into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: created.revision_id.clone(),
                expected_evidence: vec![evidence.clone()],
            },
            context(
                "2026-07-31T02:01:00.000Z",
                "guard-reflection-terminal-skip-000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let answer_after_skip = execute_disposable(
            &path,
            ReflectionWriteCommand::SaveResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: "reflection-terminal-skip".into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: created.revision_id,
                expected_evidence: vec![evidence.clone()],
                response: "Must not save after skip.".into(),
            },
            context(
                "2026-07-31T02:02:00.000Z",
                "guard-reflection-after-skip-000000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(
            answer_after_skip.code,
            "reflection_first_response_not_allowed"
        );

        let (created, evidence, source) =
            create(&path, "reflection-terminal-answer", "local_mock").await;
        let answered = answer(
            &path,
            "reflection-terminal-answer",
            &created.revision_id,
            &evidence,
            &source,
        )
        .await;
        let skip_after_answer = execute_disposable(
            &path,
            ReflectionWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: "reflection-terminal-answer".into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: answered.revision_id,
                expected_evidence: vec![evidence],
            },
            context(
                "2026-07-31T02:03:00.000Z",
                "guard-reflection-after-answer-0000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(skip_after_answer.code, "reflection_skip_not_allowed");
    }

    #[tokio::test]
    async fn correction_with_inbound_dependency_fails_closed_without_rebinding() {
        let (_directory, path) = exact_v5_fixture().await;
        let id = "reflection-dependent";
        let (created, evidence, source) = create(&path, id, "ai").await;
        let answered = answer(&path, id, &created.revision_id, &evidence, &source).await;
        let mut connection = connect(&path, false).await.unwrap();
        let dependent_revision: String = sqlx::query_scalar(
            "SELECT current_revision_id FROM artifact_heads WHERE id = 'fixture-v4-question'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        let dependency = dependency_id(
            "fixture-v4-question",
            &dependent_revision,
            "uses_reflection_response",
            id,
            &answered.revision_id,
        );
        sqlx::query(
            "INSERT INTO artifact_dependencies (id, dependent_artifact_id, dependent_revision_id, \
               relationship_type, source_revision_id, source_artifact_id, \
               source_artifact_revision_id, created_at) \
             VALUES (?, 'fixture-v4-question', ?, 'uses_reflection_response', NULL, ?, ?, ?)",
        )
        .bind(dependency)
        .bind(dependent_revision)
        .bind(id)
        .bind(&answered.revision_id)
        .bind("2026-07-31T02:02:00.000Z")
        .execute(&mut connection)
        .await
        .unwrap();
        drop(connection);
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_disposable(
            &path,
            ReflectionWriteCommand::CorrectResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: answered.revision_id,
                expected_evidence: vec![evidence],
                response: "This must not silently rebind.".into(),
            },
            context(
                "2026-07-31T02:03:00.000Z",
                "guard-reflection-dependent-0000000001",
                ReflectionWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(
            error.code,
            "reflection_inbound_dependency_requires_later_slice"
        );
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
    }

    #[tokio::test]
    async fn every_create_write_boundary_rolls_back_to_exact_logical_prestate() {
        let points = [
            ReflectionWriteFailurePoint::AfterGuard,
            ReflectionWriteFailurePoint::AfterProvenance,
            ReflectionWriteFailurePoint::AfterRevision,
            ReflectionWriteFailurePoint::AfterContent,
            ReflectionWriteFailurePoint::AfterHead,
            ReflectionWriteFailurePoint::AfterDependency,
            ReflectionWriteFailurePoint::AfterLifecycle,
            ReflectionWriteFailurePoint::AfterProjection,
            ReflectionWriteFailurePoint::AfterReconciliation,
            ReflectionWriteFailurePoint::AfterGuardRemoval,
        ];
        for (index, point) in points.into_iter().enumerate() {
            let (_directory, path) = exact_v5_fixture().await;
            let source = source_revision(&path).await;
            let evidence = evidence_ref(&path).await;
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            let id = format!("reflection-injected-{index}");
            let error = execute_disposable(
                &path,
                ReflectionWriteCommand::CreateSuggested {
                    expected_source_revision_id: source,
                    prompt: prompt(&id, "ai", evidence),
                },
                ReflectionWriteContext {
                    occurred_at: "2026-07-31T02:04:00.000Z",
                    guard_token: "guard-reflection-injected-00000000001",
                    failure_point: point,
                },
            )
            .await
            .unwrap_err();
            assert!(!error.recovery_required, "{point:?}: {error}");
            let mut connection = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
            let guards: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
                    .fetch_one(&mut connection)
                    .await
                    .unwrap();
            assert_eq!(guards, 0);
        }
    }

    #[tokio::test]
    async fn injected_skip_review_failure_rolls_back_review_head_and_projection() {
        let (_directory, path) = exact_v5_fixture().await;
        let id = "reflection-skip-rollback";
        let (created, evidence, source) = create(&path, id, "local_mock").await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_disposable(
            &path,
            ReflectionWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: created.revision_id.clone(),
                expected_evidence: vec![evidence],
            },
            context(
                "2026-07-31T02:06:00.000Z",
                "guard-reflection-skip-rollback-000001",
                ReflectionWriteFailurePoint::AfterReview,
            ),
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("AfterReview"));
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        let state: (String, String, i64) = sqlx::query_as(
            "SELECT review_state, current_revision_id, \
               (SELECT COUNT(*) FROM artifact_review_events WHERE artifact_id = ?) \
             FROM artifact_heads WHERE id = ?",
        )
        .bind(id)
        .bind(id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(state, ("pending".into(), created.revision_id, 0));
    }

    #[derive(Clone, Copy)]
    enum InjectedCommit {
        DefinitelyNotCommitted,
        UnknownWithoutCommit,
        UnknownAfterCommit,
    }

    struct InjectedCommitAdapter {
        mode: InjectedCommit,
        commits: Cell<usize>,
        rollbacks: Cell<usize>,
    }

    impl CommitOutcomeAdapter for InjectedCommitAdapter {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            self.commits.set(self.commits.get() + 1);
            match self.mode {
                InjectedCommit::DefinitelyNotCommitted => {
                    CommitAttemptOutcome::DefinitelyNotCommitted {
                        error_class: "injected_definite_noncommit".into(),
                    }
                }
                InjectedCommit::UnknownWithoutCommit => CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_unknown_without_commit".into(),
                },
                InjectedCommit::UnknownAfterCommit => {
                    raw_sql("COMMIT").execute(connection).await.unwrap();
                    CommitAttemptOutcome::OutcomeUnknown {
                        error_class: "injected_unknown_after_commit".into(),
                    }
                }
            }
        }

        async fn rollback(&self, connection: &mut SqliteConnection) -> RollbackAttemptOutcome {
            self.rollbacks.set(self.rollbacks.get() + 1);
            raw_sql("ROLLBACK").execute(connection).await.unwrap();
            RollbackAttemptOutcome::RolledBack
        }
    }

    #[tokio::test]
    async fn commit_outcomes_are_classified_from_read_only_exact_manifests_without_retry() {
        for (mode, succeeds, rollback_count) in [
            (InjectedCommit::DefinitelyNotCommitted, false, 1),
            (InjectedCommit::UnknownWithoutCommit, false, 0),
            (InjectedCommit::UnknownAfterCommit, true, 0),
        ] {
            let (_directory, path) = exact_v5_fixture().await;
            let source = source_revision(&path).await;
            let evidence = evidence_ref(&path).await;
            let adapter = InjectedCommitAdapter {
                mode,
                commits: Cell::new(0),
                rollbacks: Cell::new(0),
            };
            let result = execute_with_adapter(
                &path,
                ReflectionWriteCommand::CreateSuggested {
                    expected_source_revision_id: source,
                    prompt: prompt("reflection-commit", "ai", evidence),
                },
                context(
                    "2026-07-31T02:05:00.000Z",
                    "guard-reflection-commit-0000000000001",
                    ReflectionWriteFailurePoint::None,
                ),
                &adapter,
            )
            .await;
            assert_eq!(result.is_ok(), succeeds);
            assert_eq!(adapter.commits.get(), 1);
            assert_eq!(adapter.rollbacks.get(), rollback_count);
            let mut connection = connect(&path, true).await.unwrap();
            verify_exact_reflection_v5(&mut connection).await.unwrap();
        }
    }

    #[tokio::test]
    async fn deterministic_identifiers_and_manifests_repeat_across_disposable_fixtures() {
        let (_first_directory, first) = exact_v5_fixture().await;
        let (_second_directory, second) = exact_v5_fixture().await;
        let first_created = create(&first, "reflection-deterministic", "ai").await.0;
        let second_created = create(&second, "reflection-deterministic", "ai").await.0;
        assert_eq!(first_created.revision_id, second_created.revision_id);
        assert_eq!(
            first_created.operation_manifest,
            second_created.operation_manifest
        );
    }
}
