use super::experience_write::operation_manifest;
use super::reflection_write::verify_exact_reflection_v5;
use super::*;
use sqlx::{raw_sql, Row};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct PatternProvenanceInput {
    origin: String,
    provider: String,
    model: Option<String>,
    harness_version: String,
    prompt_version: String,
    generated_at: String,
    source_artifact_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ArtifactRevisionRef {
    artifact_id: String,
    revision_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PatternCandidateInput {
    id: String,
    source_id: String,
    text: String,
    created_at: String,
    evidence: Vec<ArtifactRevisionRef>,
    reflections: Vec<ArtifactRevisionRef>,
    provenance: PatternProvenanceInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
enum PatternWriteCommand {
    Create {
        expected_source_revision_id: String,
        candidate: PatternCandidateInput,
    },
    ConfirmPending {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        expected_evidence: Vec<ArtifactRevisionRef>,
        expected_reflections: Vec<ArtifactRevisionRef>,
    },
    RejectPending {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        expected_evidence: Vec<ArtifactRevisionRef>,
        expected_reflections: Vec<ArtifactRevisionRef>,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum PatternWriteFailurePoint {
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
struct PatternWriteContext<'a> {
    occurred_at: &'a str,
    guard_token: &'a str,
    failure_point: PatternWriteFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PatternWriteStatus {
    Committed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PatternWriteOutcome {
    status: PatternWriteStatus,
    artifact_id: String,
    revision_id: String,
    operation_manifest: String,
}

#[derive(Clone, Debug)]
struct CurrentPattern {
    revision_id: String,
    review_state: String,
    lifecycle_state: String,
    eligibility_state: String,
    serialization_version: String,
    content_digest: String,
    payload: Value,
    created_at: String,
    generated_provenance: Value,
}

fn write_error(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn recovery_error(code: impl Into<String>) -> MigrationError {
    MigrationError::recovery_required(code)
}

fn validate_identifier(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        Err(write_error(format!("pattern_{field}_invalid")))
    } else {
        Ok(())
    }
}

fn validate_timestamp(value: &str, field: &str) -> Result<(), MigrationError> {
    validate_identifier(value, field)?;
    if !value.contains('T') || !value.ends_with('Z') {
        return Err(write_error(format!("pattern_{field}_invalid")));
    }
    Ok(())
}

fn validate_text(value: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 100_000 {
        Err(write_error("pattern_text_invalid"))
    } else {
        Ok(())
    }
}

fn exact_id_set(
    value: Option<&Value>,
    field: &str,
    required: bool,
) -> Result<BTreeSet<String>, MigrationError> {
    let represented = value
        .and_then(Value::as_array)
        .ok_or_else(|| recovery_error(format!("pattern_{field}_missing")))?;
    let mut exact = BTreeSet::new();
    for item in represented {
        let id = item
            .as_str()
            .filter(|id| !id.trim().is_empty())
            .ok_or_else(|| recovery_error(format!("pattern_{field}_malformed")))?;
        if !exact.insert(id.to_owned()) {
            return Err(recovery_error(format!("pattern_{field}_duplicate")));
        }
    }
    if required && exact.is_empty() {
        return Err(recovery_error(format!("pattern_{field}_required")));
    }
    Ok(exact)
}

fn normalize_refs(
    values: &[ArtifactRevisionRef],
    kind: &str,
    required: bool,
) -> Result<Vec<ArtifactRevisionRef>, MigrationError> {
    if required && values.is_empty() {
        return Err(write_error(format!("pattern_{kind}_required")));
    }
    let mut normalized = BTreeMap::new();
    for value in values {
        validate_identifier(&value.artifact_id, &format!("{kind}_artifact_id"))?;
        validate_identifier(&value.revision_id, &format!("{kind}_revision_id"))?;
        if normalized
            .insert(value.artifact_id.clone(), value.revision_id.clone())
            .is_some()
        {
            return Err(write_error(format!("pattern_{kind}_duplicate")));
        }
    }
    Ok(normalized
        .into_iter()
        .map(|(artifact_id, revision_id)| ArtifactRevisionRef {
            artifact_id,
            revision_id,
        })
        .collect())
}

fn validate_provenance(
    provenance: &PatternProvenanceInput,
    source_id: &str,
    evidence: &[ArtifactRevisionRef],
    reflections: &[ArtifactRevisionRef],
) -> Result<(), MigrationError> {
    if !matches!(provenance.origin.as_str(), "ai" | "local_mock") {
        return Err(write_error("pattern_generated_origin_invalid"));
    }
    let provider_valid = match provenance.origin.as_str() {
        "ai" => matches!(provenance.provider.as_str(), "openai" | "gemini"),
        "local_mock" => provenance.provider == "mock",
        _ => false,
    };
    if !provider_valid {
        return Err(write_error("pattern_generated_provider_invalid"));
    }
    if provenance.origin == "ai"
        && provenance
            .model
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .is_none()
    {
        return Err(write_error("pattern_generated_model_missing"));
    }
    validate_identifier(&provenance.harness_version, "harness_version")?;
    validate_identifier(&provenance.prompt_version, "prompt_version")?;
    validate_timestamp(&provenance.generated_at, "generated_at")?;
    let expected = evidence
        .iter()
        .chain(reflections)
        .map(|item| item.artifact_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut sources = BTreeSet::new();
    for source in &provenance.source_artifact_ids {
        validate_identifier(source, "source_artifact_id")?;
        if source == source_id || !sources.insert(source.as_str()) {
            return Err(write_error("pattern_generated_sources_invalid"));
        }
    }
    if sources != expected {
        return Err(write_error("pattern_generated_sources_mismatch"));
    }
    Ok(())
}

fn validate_candidate(
    candidate: &PatternCandidateInput,
) -> Result<(Vec<ArtifactRevisionRef>, Vec<ArtifactRevisionRef>), MigrationError> {
    validate_identifier(&candidate.id, "artifact_id")?;
    validate_identifier(&candidate.source_id, "source_id")?;
    validate_text(&candidate.text)?;
    validate_timestamp(&candidate.created_at, "created_at")?;
    let evidence = normalize_refs(&candidate.evidence, "evidence", true)?;
    let reflections = normalize_refs(&candidate.reflections, "reflection", false)?;
    let evidence_ids = evidence
        .iter()
        .map(|item| item.artifact_id.as_str())
        .collect::<BTreeSet<_>>();
    let reflection_ids = reflections
        .iter()
        .map(|item| item.artifact_id.as_str())
        .collect::<BTreeSet<_>>();
    if evidence_ids.intersection(&reflection_ids).next().is_some() {
        return Err(write_error("pattern_dependency_kind_conflict"));
    }
    validate_provenance(
        &candidate.provenance,
        &candidate.source_id,
        &evidence,
        &reflections,
    )?;
    Ok((evidence, reflections))
}

fn inject(
    context: &PatternWriteContext<'_>,
    point: PatternWriteFailurePoint,
) -> Result<(), MigrationError> {
    if context.failure_point == point {
        Err(write_error(format!(
            "injected_pattern_write_failure:{point:?}"
        )))
    } else {
        Ok(())
    }
}

fn content_value(
    id: &str,
    source_id: &str,
    text: &str,
    evidence: &[ArtifactRevisionRef],
    reflections: &[ArtifactRevisionRef],
    created_at: &str,
) -> Value {
    json!({
        "createdAt": created_at,
        "id": id,
        "sourceEntryId": source_id,
        "sourceEvidenceIds": evidence.iter().map(|item| item.artifact_id.clone()).collect::<Vec<_>>(),
        "sourceReflectionPromptIds": reflections.iter().map(|item| item.artifact_id.clone()).collect::<Vec<_>>(),
        "text": text,
    })
}

fn generated_provenance_value(source_id: &str, provenance: &PatternProvenanceInput) -> Value {
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

fn v4_projection_value(
    content: &Value,
    status: &str,
    updated_at: &str,
    generated_provenance: &Value,
) -> Result<Value, MigrationError> {
    let object = content
        .as_object()
        .ok_or_else(|| write_error("pattern_content_not_object"))?;
    Ok(json!({
        "createdAt": object.get("createdAt").cloned().ok_or_else(|| write_error("pattern_created_at_missing"))?,
        "id": object.get("id").cloned().ok_or_else(|| write_error("pattern_id_missing"))?,
        "provenance": generated_provenance,
        "sourceEntryId": object.get("sourceEntryId").cloned().ok_or_else(|| write_error("pattern_source_missing"))?,
        "sourceEvidenceIds": object.get("sourceEvidenceIds").cloned().ok_or_else(|| write_error("pattern_evidence_ids_missing"))?,
        "sourceReflectionPromptIds": object.get("sourceReflectionPromptIds").cloned().ok_or_else(|| write_error("pattern_reflection_ids_missing"))?,
        "status": status,
        "text": object.get("text").cloned().ok_or_else(|| write_error("pattern_text_missing"))?,
        "updatedAt": updated_at
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
    .map_err(|error| migration_error("pattern_source_lookup_failed", error))?;
    let Some((revision_id, lifecycle, content, projection)) = row else {
        return Err(write_error("pattern_source_not_found"));
    };
    if revision_id != expected_revision_id || lifecycle != "active" || content != projection {
        return Err(write_error("pattern_source_revision_stale"));
    }
    Ok(())
}

async fn validate_current_evidence(
    connection: &mut SqliteConnection,
    source_id: &str,
    evidence: &[ArtifactRevisionRef],
) -> Result<Vec<ArtifactRevisionRef>, MigrationError> {
    let evidence = normalize_refs(evidence, "evidence", true)?;
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
        .map_err(|error| migration_error("pattern_evidence_lookup_failed", error))?;
        let Some((actual_source, revision, review, lifecycle, eligibility, projection)) = row
        else {
            return Err(write_error("pattern_evidence_not_found"));
        };
        let projected: Value = serde_json::from_str(&projection)
            .map_err(|error| migration_error("pattern_evidence_projection_malformed", error))?;
        if actual_source != source_id {
            return Err(write_error("pattern_evidence_cross_source"));
        }
        if revision != item.revision_id {
            return Err(write_error("pattern_evidence_revision_stale"));
        }
        if review != "confirmed"
            || lifecycle != "active"
            || eligibility != "eligible"
            || projected.get("status").and_then(Value::as_str) != Some("confirmed")
        {
            return Err(write_error("pattern_evidence_ineligible"));
        }
    }
    Ok(evidence)
}

async fn validate_current_reflections(
    connection: &mut SqliteConnection,
    source_id: &str,
    reflections: &[ArtifactRevisionRef],
) -> Result<Vec<ArtifactRevisionRef>, MigrationError> {
    let reflections = normalize_refs(reflections, "reflection", false)?;
    for item in &reflections {
        let row: Option<(String, String, String, String, String, String)> = sqlx::query_as(
            "SELECT h.source_id, h.current_revision_id, h.review_state, h.lifecycle_state, \
                    h.eligibility_state, pa.payload \
             FROM artifact_heads h \
             JOIN artifact_revisions r ON r.id = h.current_revision_id AND r.artifact_id = h.id \
             JOIN artifact_revision_content c ON c.revision_id = r.id \
             JOIN persisted_artifacts pa ON pa.id = h.id \
               AND pa.source_entry_id = h.source_id AND pa.artifact_kind = 'reflection' \
             WHERE h.id = ? AND h.artifact_kind = 'reflection'",
        )
        .bind(&item.artifact_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_reflection_lookup_failed", error))?;
        let Some((actual_source, revision, review, lifecycle, eligibility, projection)) = row
        else {
            return Err(write_error("pattern_reflection_not_found"));
        };
        let projected: Value = serde_json::from_str(&projection)
            .map_err(|error| migration_error("pattern_reflection_projection_malformed", error))?;
        if actual_source != source_id {
            return Err(write_error("pattern_reflection_cross_source"));
        }
        if revision != item.revision_id {
            return Err(write_error("pattern_reflection_revision_stale"));
        }
        if review != "not_applicable"
            || lifecycle != "active"
            || eligibility != "eligible"
            || projected.get("status").and_then(Value::as_str) != Some("answered")
            || projected
                .get("response")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .is_none()
        {
            return Err(write_error("pattern_reflection_ineligible"));
        }
    }
    Ok(reflections)
}

async fn insert_guard(
    connection: &mut SqliteConnection,
    context: &PatternWriteContext<'_>,
) -> Result<(), MigrationError> {
    validate_identifier(context.guard_token, "guard_token")?;
    validate_timestamp(context.occurred_at, "occurred_at")?;
    if context.guard_token.len() < 32 {
        return Err(write_error("pattern_guard_token_invalid"));
    }
    sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
        .bind(context.guard_token)
        .bind(context.occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_guard_insert_failed", error))?;
    inject(context, PatternWriteFailurePoint::AfterGuard)
}

async fn remove_guard(
    connection: &mut SqliteConnection,
    context: &PatternWriteContext<'_>,
) -> Result<(), MigrationError> {
    let result = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
        .bind(context.guard_token)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_guard_remove_failed", error))?;
    if result.rows_affected() != 1 {
        return Err(recovery_error("pattern_guard_identity_mismatch"));
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_guard_count_failed", error))?;
    if count != 0 {
        return Err(recovery_error("pattern_guard_not_empty"));
    }
    inject(context, PatternWriteFailurePoint::AfterGuardRemoval)
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
    .map_err(|error| migration_error("pattern_inbound_dependency_lookup_failed", error))?;
    let historical: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM historical_artifact_dependencies  \
         WHERE source_artifact_id = ?",
    )
    .bind(artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_historical_dependency_lookup_failed", error))?;
    Ok(normalized + historical)
}

async fn current_pattern(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_revision_id: &str,
) -> Result<CurrentPattern, MigrationError> {
    validate_identifier(artifact_id, "artifact_id")?;
    validate_identifier(expected_revision_id, "artifact_revision_id")?;
    let row = sqlx::query(
        "SELECT h.source_id, h.current_revision_id, h.review_state,  \
                h.lifecycle_state, h.eligibility_state, h.created_at,  \
                r.serialization_version, r.content_digest, c.payload,  \
                p.canonical_payload  \
         FROM artifact_heads h  \
         JOIN artifact_revisions r ON r.id = h.current_revision_id  \
           AND r.artifact_id = h.id  \
         JOIN artifact_revision_content c ON c.revision_id = r.id  \
         JOIN artifact_revision_provenance rp  \
           ON rp.artifact_revision_id = r.id AND rp.role = 'content'  \
         JOIN provenance_records p ON p.id = rp.provenance_id  \
         JOIN persisted_artifacts pa ON pa.id = h.id  \
           AND pa.source_entry_id = h.source_id AND pa.artifact_kind = 'pattern'  \
         WHERE h.id = ? AND h.artifact_kind = 'pattern'",
    )
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_current_lookup_failed", error))?
    .ok_or_else(|| write_error("pattern_artifact_not_found"))?;

    let actual_source: String = row.get(0);
    let revision_id: String = row.get(1);
    if actual_source != source_id || revision_id != expected_revision_id {
        return Err(write_error("pattern_artifact_revision_stale"));
    }
    let raw_payload: String = row.get(8);
    let payload: Value = serde_json::from_str(&raw_payload)
        .map_err(|error| migration_error("pattern_current_payload_malformed", error))?;
    let provenance_raw: String = row.get(9);
    let generated_provenance: Value = serde_json::from_str(&provenance_raw)
        .map_err(|error| migration_error("pattern_current_provenance_malformed", error))?;
    let content_digest: String = row.get(7);
    if content_digest != sha256_hex(raw_payload.as_bytes()) {
        return Err(recovery_error("pattern_current_digest_mismatch"));
    }
    Ok(CurrentPattern {
        revision_id,
        review_state: row.get(2),
        lifecycle_state: row.get(3),
        eligibility_state: row.get(4),
        serialization_version: row.get(6),
        content_digest,
        payload,
        created_at: row.get(5),
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
    .map_err(|error| migration_error("pattern_revision_insert_failed", error))?;
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
    .map_err(|error| migration_error("pattern_content_insert_failed", error))?;
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
    .map_err(|error| migration_error("pattern_dependency_insert_failed", error))?;
    Ok(())
}

async fn insert_artifact_dependencies(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
    relationship: &str,
    dependencies: &[ArtifactRevisionRef],
    occurred_at: &str,
) -> Result<(), MigrationError> {
    if !matches!(relationship, "uses_evidence" | "uses_reflection_response") {
        return Err(write_error("pattern_dependency_type_unsupported"));
    }
    for dependency in dependencies {
        let id = dependency_id(
            artifact_id,
            revision_id,
            relationship,
            &dependency.artifact_id,
            &dependency.revision_id,
        );
        sqlx::query(
            "INSERT INTO artifact_dependencies ( \
               id, dependent_artifact_id, dependent_revision_id, relationship_type, \
               source_revision_id, source_artifact_id, source_artifact_revision_id, created_at \
             ) VALUES (?, ?, ?, ?, NULL, ?, ?, ?)",
        )
        .bind(id)
        .bind(artifact_id)
        .bind(revision_id)
        .bind(relationship)
        .bind(&dependency.artifact_id)
        .bind(&dependency.revision_id)
        .bind(occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_artifact_dependency_insert_failed", error))?;
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
    .map_err(|error| migration_error("pattern_lifecycle_insert_failed", error))?;
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
        &format!("life-os/pattern-{decision}-review-event-id-v1"),
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
    .map_err(|error| migration_error("pattern_review_insert_failed", error))?;
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
             VALUES (?, ?, 'pattern', ?, ?, ?)",
        )
        .bind(artifact_id)
        .bind(source_id)
        .bind(serialized)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_projection_insert_failed", error))?;
    } else {
        let result = sqlx::query(
            "UPDATE persisted_artifacts SET payload = ?, updated_at = ?  \
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'pattern'",
        )
        .bind(serialized)
        .bind(updated_at)
        .bind(artifact_id)
        .bind(source_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_projection_update_failed", error))?;
        if result.rows_affected() != 1 {
            return Err(recovery_error("pattern_projection_identity_mismatch"));
        }
    }
    Ok(())
}

async fn verify_pattern_projection(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    let heads = sqlx::query(
        "SELECT h.id, h.source_id, h.current_revision_id, h.review_state,  \
                h.lifecycle_state, h.eligibility_state, h.created_at, h.updated_at  \
         FROM artifact_heads h WHERE h.artifact_kind = 'pattern' ORDER BY h.id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_heads_unreadable", error))?;
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
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'pattern'",
        )
        .bind(&artifact_id)
        .bind(&source_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_projection_unreadable", error))?;

        match lifecycle_state.as_str() {
            "active" => {
                projected += 1;
                let revision_id = current_revision_id
                    .ok_or_else(|| recovery_error("pattern_active_revision_missing"))?;
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
                .map_err(|error| migration_error("pattern_revision_unreadable", error))?;
                let (serialization, digest, content, byte_length) =
                    revision.ok_or_else(|| recovery_error("pattern_active_content_missing"))?;
                if digest != sha256_hex(content.as_bytes()) || byte_length != content.len() as i64 {
                    return Err(recovery_error("pattern_content_digest_mismatch"));
                }
                let (projection_payload, created_at, updated_at) =
                    projection.ok_or_else(|| recovery_error("pattern_projection_missing"))?;
                if created_at != head.get::<String, _>(6) || updated_at != head.get::<String, _>(7)
                {
                    return Err(recovery_error("pattern_projection_timestamp_mismatch"));
                }
                let projected_value: Value =
                    serde_json::from_str(&projection_payload).map_err(|error| {
                        migration_error("pattern_projection_payload_malformed", error)
                    })?;
                let expected_status = match review_state.as_str() {
                    "pending" if eligibility_state == "ineligible" => "candidate",
                    "confirmed" if eligibility_state == "eligible" => "confirmed",
                    _ => return Err(recovery_error("pattern_review_eligibility_mismatch")),
                };
                if projected_value.get("status").and_then(Value::as_str) != Some(expected_status) {
                    return Err(recovery_error("pattern_projection_status_mismatch"));
                }
                if serialization == "legacy-v4-raw" {
                    if projection_payload.as_bytes() != content.as_bytes() {
                        return Err(recovery_error("pattern_legacy_projection_mismatch"));
                    }
                } else if serialization == "canonical-json-v1" {
                    let content_value: Value = serde_json::from_str(&content)
                        .map_err(|error| migration_error("pattern_content_malformed", error))?;
                    for key in [
                        "id",
                        "sourceEntryId",
                        "text",
                        "sourceEvidenceIds",
                        "sourceReflectionPromptIds",
                        "createdAt",
                    ] {
                        if content_value.get(key) != projected_value.get(key) {
                            return Err(recovery_error(format!(
                                "pattern_projection_field_mismatch:{key}"
                            )));
                        }
                    }
                    let provenance: Vec<String> = sqlx::query_scalar(
                        "SELECT p.canonical_payload FROM artifact_revision_provenance rp \
                         JOIN provenance_records p ON p.id = rp.provenance_id \
                         WHERE rp.artifact_revision_id = ? AND rp.role = 'content'",
                    )
                    .bind(&revision_id)
                    .fetch_all(&mut *connection)
                    .await
                    .map_err(|error| migration_error("pattern_provenance_unreadable", error))?;
                    if provenance.len() != 1 {
                        return Err(recovery_error("pattern_provenance_count_mismatch"));
                    }
                    let provenance_value: Value = serde_json::from_str(&provenance[0])
                        .map_err(|error| migration_error("pattern_provenance_malformed", error))?;
                    if projected_value.get("provenance") != Some(&provenance_value) {
                        return Err(recovery_error("pattern_provenance_projection_mismatch"));
                    }
                } else {
                    return Err(recovery_error("pattern_serialization_version_unsupported"));
                }

                let source_dependency_count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM artifact_dependencies d \
                     JOIN artifact_heads h ON h.id = d.dependent_artifact_id \
                     JOIN source_heads s ON s.id = h.source_id \
                     WHERE d.dependent_revision_id = ? \
                       AND d.relationship_type = 'derived_from_experience' \
                       AND d.source_revision_id = s.current_revision_id \
                       AND s.id = ? AND s.lifecycle_state = 'active'",
                )
                .bind(&revision_id)
                .bind(&source_id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| migration_error("pattern_source_dependency_unreadable", error))?;
                if source_dependency_count != 1 {
                    return Err(recovery_error("pattern_source_dependency_mismatch"));
                }

                let expected_evidence = exact_id_set(
                    projected_value.get("sourceEvidenceIds"),
                    "evidence_ids",
                    true,
                )?;
                let actual_evidence: Vec<String> = sqlx::query_scalar(
                    "SELECT d.source_artifact_id FROM artifact_dependencies d \
                     JOIN artifact_heads h ON h.id = d.source_artifact_id \
                       AND h.current_revision_id = d.source_artifact_revision_id \
                     WHERE d.dependent_revision_id = ? AND d.relationship_type = 'uses_evidence' \
                       AND h.source_id = ? AND h.artifact_kind = 'evidence' \
                       AND h.review_state = 'confirmed' AND h.lifecycle_state = 'active' \
                       AND h.eligibility_state = 'eligible' ORDER BY d.source_artifact_id",
                )
                .bind(&revision_id)
                .bind(&source_id)
                .fetch_all(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("pattern_evidence_dependencies_unreadable", error)
                })?;
                if actual_evidence.into_iter().collect::<BTreeSet<_>>() != expected_evidence {
                    return Err(recovery_error("pattern_evidence_dependencies_mismatch"));
                }

                let expected_reflections = exact_id_set(
                    projected_value.get("sourceReflectionPromptIds"),
                    "reflection_ids",
                    false,
                )?;
                let actual_reflections: Vec<String> = sqlx::query_scalar(
                    "SELECT d.source_artifact_id FROM artifact_dependencies d \
                     JOIN artifact_heads h ON h.id = d.source_artifact_id \
                       AND h.current_revision_id = d.source_artifact_revision_id \
                     JOIN persisted_artifacts pa ON pa.id = h.id \
                       AND pa.source_entry_id = h.source_id AND pa.artifact_kind = 'reflection' \
                     WHERE d.dependent_revision_id = ? \
                       AND d.relationship_type = 'uses_reflection_response' \
                       AND h.source_id = ? AND h.artifact_kind = 'reflection' \
                       AND h.review_state = 'not_applicable' AND h.lifecycle_state = 'active' \
                       AND h.eligibility_state = 'eligible' \
                       AND json_extract(pa.payload, '$.status') = 'answered' \
                       AND trim(json_extract(pa.payload, '$.response')) <> '' \
                     ORDER BY d.source_artifact_id",
                )
                .bind(&revision_id)
                .bind(&source_id)
                .fetch_all(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("pattern_reflection_dependencies_unreadable", error)
                })?;
                if actual_reflections.into_iter().collect::<BTreeSet<_>>() != expected_reflections {
                    return Err(recovery_error("pattern_reflection_dependencies_mismatch"));
                }

                let declared_sources = expected_evidence
                    .union(&expected_reflections)
                    .cloned()
                    .collect::<BTreeSet<_>>();
                let provenance_sources = exact_id_set(
                    projected_value
                        .get("provenance")
                        .and_then(|value| value.get("sourceArtifactIds")),
                    "provenance_sources",
                    true,
                )?;
                if declared_sources != provenance_sources {
                    return Err(recovery_error("pattern_provenance_sources_mismatch"));
                }
            }
            "content_purged" => {
                if current_revision_id.is_some()
                    || review_state != "rejected"
                    || eligibility_state != "ineligible"
                    || projection.is_some()
                {
                    return Err(recovery_error("pattern_rejected_projection_mismatch"));
                }
                let retained: (i64, i64, i64) = sqlx::query_as(
                    "SELECT \
                       (SELECT COUNT(*) FROM artifact_review_events \
                        WHERE artifact_id = ? AND decision = 'rejected'), \
                       (SELECT COUNT(*) FROM artifact_lifecycle_events \
                        WHERE artifact_id = ? AND event_type = 'content_purged'), \
                       (SELECT COUNT(*) FROM content_tombstones \
                        WHERE artifact_id = ? AND reason_code = 'rejected_content_purged')",
                )
                .bind(&artifact_id)
                .bind(&artifact_id)
                .bind(&artifact_id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| migration_error("pattern_rejection_history_unreadable", error))?;
                if retained != (1, 1, 1) {
                    return Err(recovery_error("pattern_rejection_history_mismatch"));
                }
            }
            _ => return Err(recovery_error("pattern_lifecycle_unsupported")),
        }
    }
    let projection_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM persisted_artifacts WHERE artifact_kind = 'pattern'",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_projection_count_failed", error))?;
    if projection_count != projected {
        return Err(recovery_error("pattern_projection_count_mismatch"));
    }
    Ok(())
}

pub(super) async fn verify_exact_pattern_v5(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    verify_exact_reflection_v5(connection).await?;
    verify_pattern_projection(connection).await
}

async fn create_candidate(
    connection: &mut SqliteConnection,
    expected_source_revision_id: &str,
    candidate: PatternCandidateInput,
    context: &PatternWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    let (evidence, reflections) = validate_candidate(&candidate)?;
    exact_current_source(
        connection,
        &candidate.source_id,
        expected_source_revision_id,
    )
    .await?;
    let evidence = validate_current_evidence(connection, &candidate.source_id, &evidence).await?;
    let reflections =
        validate_current_reflections(connection, &candidate.source_id, &reflections).await?;
    let conflict: i64 = sqlx::query_scalar(
        "SELECT  \
           (SELECT COUNT(*) FROM artifact_heads WHERE id = ?) +  \
           (SELECT COUNT(*) FROM persisted_artifacts WHERE id = ?)",
    )
    .bind(&candidate.id)
    .bind(&candidate.id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_identity_check_failed", error))?;
    if conflict != 0 {
        return Err(write_error("pattern_identity_conflict"));
    }

    let content = content_value(
        &candidate.id,
        &candidate.source_id,
        &candidate.text,
        &evidence,
        &reflections,
        &candidate.created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id =
        artifact_revision_id(&candidate.id, "pattern", &candidate.created_at, &digest);
    let provenance = generated_provenance_value(&candidate.source_id, &candidate.provenance);
    // The role link requires the revision; insert provenance now and link after
    // revision creation without rebinding either record.
    let provenance_id = insert_provenance(connection, &provenance, &candidate.created_at).await?;
    inject(context, PatternWriteFailurePoint::AfterProvenance)?;
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
    inject(context, PatternWriteFailurePoint::AfterRevision)?;
    insert_content(connection, &revision_id, &payload).await?;
    inject(context, PatternWriteFailurePoint::AfterContent)?;
    sqlx::query(
        "INSERT INTO artifact_heads ( \
           id, source_id, artifact_kind, current_revision_id, review_state, \
           lifecycle_state, eligibility_state, eligibility_reason, created_at, updated_at \
         ) VALUES (?, ?, 'pattern', ?, 'pending', 'active', 'ineligible', \
           'pending_explicit_review', ?, ?)",
    )
    .bind(&candidate.id)
    .bind(&candidate.source_id)
    .bind(&revision_id)
    .bind(&candidate.created_at)
    .bind(&candidate.created_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_head_insert_failed", error))?;
    sqlx::query(
        "INSERT INTO artifact_revision_provenance  \
         (artifact_revision_id, role, provenance_id) VALUES (?, 'content', ?)",
    )
    .bind(&revision_id)
    .bind(&provenance_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_provenance_link_failed", error))?;
    inject(context, PatternWriteFailurePoint::AfterHead)?;
    insert_source_dependency(
        connection,
        &candidate.id,
        &revision_id,
        &candidate.source_id,
        expected_source_revision_id,
        &candidate.created_at,
    )
    .await?;
    insert_artifact_dependencies(
        connection,
        &candidate.id,
        &revision_id,
        "uses_evidence",
        &evidence,
        &candidate.created_at,
    )
    .await?;
    insert_artifact_dependencies(
        connection,
        &candidate.id,
        &revision_id,
        "uses_reflection_response",
        &reflections,
        &candidate.created_at,
    )
    .await?;
    inject(context, PatternWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/pattern-created-event-id-v1",
        "created",
        "system",
        "pattern_candidate_created",
        &candidate.id,
        &revision_id,
        None,
        &candidate.created_at,
    )
    .await?;
    inject(context, PatternWriteFailurePoint::AfterLifecycle)?;
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
    inject(context, PatternWriteFailurePoint::AfterProjection)?;
    Ok((candidate.id, revision_id))
}

async fn ensure_expected_dependencies(
    connection: &mut SqliteConnection,
    current: &CurrentPattern,
    expected_evidence: &[ArtifactRevisionRef],
    expected_reflections: &[ArtifactRevisionRef],
) -> Result<(), MigrationError> {
    let expected_evidence = normalize_refs(expected_evidence, "evidence", true)?;
    let expected_reflections = normalize_refs(expected_reflections, "reflection", false)?;
    validate_current_evidence(
        connection,
        current_source_id(&current.payload)?,
        &expected_evidence,
    )
    .await?;
    validate_current_reflections(
        connection,
        current_source_id(&current.payload)?,
        &expected_reflections,
    )
    .await?;

    for (relationship, expected) in [
        ("uses_evidence", &expected_evidence),
        ("uses_reflection_response", &expected_reflections),
    ] {
        let actual: Vec<(String, String)> = sqlx::query_as(
            "SELECT source_artifact_id, source_artifact_revision_id \
             FROM artifact_dependencies WHERE dependent_revision_id = ? \
               AND relationship_type = ? ORDER BY source_artifact_id",
        )
        .bind(&current.revision_id)
        .bind(relationship)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_dependency_read_failed", error))?;
        let expected_pairs = expected
            .iter()
            .map(|item| (item.artifact_id.clone(), item.revision_id.clone()))
            .collect::<Vec<_>>();
        if actual != expected_pairs {
            return Err(write_error("pattern_expected_dependencies_mismatch"));
        }
    }
    Ok(())
}

fn current_source_id(payload: &Value) -> Result<&str, MigrationError> {
    payload
        .get("sourceEntryId")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("pattern_source_missing"))
}

#[allow(clippy::too_many_arguments)]
async fn confirm_pending(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    expected_evidence: &[ArtifactRevisionRef],
    expected_reflections: &[ArtifactRevisionRef],
    context: &PatternWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_pattern(
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
        return Err(write_error("pattern_pending_review_not_allowed"));
    }
    if current.serialization_version != "canonical-json-v1" {
        return Err(write_error("pattern_legacy_review_requires_later_slice"));
    }
    ensure_expected_dependencies(
        connection,
        &current,
        expected_evidence,
        expected_reflections,
    )
    .await?;
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "pattern_inbound_dependency_requires_later_slice",
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
    inject(context, PatternWriteFailurePoint::AfterReview)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET review_state = 'confirmed', \
         eligibility_state = 'eligible', \
         eligibility_reason = 'explicitly_confirmed_useful_for_reflection', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'pattern' AND review_state = 'pending' \
           AND lifecycle_state = 'active'",
    )
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_confirm_head_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("pattern_artifact_revision_stale"));
    }
    inject(context, PatternWriteFailurePoint::AfterHead)?;
    let projection = v4_projection_value(
        &current.payload,
        "confirmed",
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
    inject(context, PatternWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.to_string(), current.revision_id))
}

#[allow(clippy::too_many_arguments)]
async fn reject_pending(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    expected_evidence: &[ArtifactRevisionRef],
    expected_reflections: &[ArtifactRevisionRef],
    context: &PatternWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_pattern(
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
        return Err(write_error("pattern_pending_review_not_allowed"));
    }
    if current.serialization_version != "canonical-json-v1" {
        return Err(write_error("pattern_legacy_review_requires_later_slice"));
    }
    ensure_expected_dependencies(
        connection,
        &current,
        expected_evidence,
        expected_reflections,
    )
    .await?;
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "pattern_inbound_dependency_requires_later_slice",
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
    inject(context, PatternWriteFailurePoint::AfterReview)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = NULL, \
         review_state = 'rejected', lifecycle_state = 'content_purged', \
         eligibility_state = 'ineligible', \
         eligibility_reason = 'explicitly_rejected_content_purged', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'pattern' AND review_state = 'pending' \
           AND lifecycle_state = 'active'",
    )
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(source_id)
    .bind(&current.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_reject_head_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("pattern_artifact_revision_stale"));
    }
    inject(context, PatternWriteFailurePoint::AfterHead)?;
    insert_lifecycle_event(
        connection,
        "life-os/pattern-rejected-content-purged-event-id-v1",
        "content_purged",
        "user",
        "rejected_content_purged",
        artifact_id,
        &current.revision_id,
        None,
        context.occurred_at,
    )
    .await?;
    inject(context, PatternWriteFailurePoint::AfterLifecycle)?;
    let tombstone_id = format!(
        "v5ts_{}",
        sha256_hex(
            format!(
                "life-os/pattern-rejection-tombstone-id-v1\0{artifact_id}\0{}\0{}",
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
    .map_err(|error| migration_error("pattern_tombstone_insert_failed", error))?;
    inject(context, PatternWriteFailurePoint::AfterTombstone)?;
    let purged = sqlx::query("DELETE FROM artifact_revision_content WHERE revision_id = ?")
        .bind(&current.revision_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("pattern_content_purge_failed", error))?;
    if purged.rows_affected() != 1 {
        return Err(recovery_error("pattern_content_purge_identity_mismatch"));
    }
    inject(context, PatternWriteFailurePoint::AfterPurge)?;
    let deleted = sqlx::query(
        "DELETE FROM persisted_artifacts  \
         WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'pattern'",
    )
    .bind(artifact_id)
    .bind(source_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("pattern_projection_delete_failed", error))?;
    if deleted.rows_affected() != 1 {
        return Err(recovery_error("pattern_projection_identity_mismatch"));
    }
    inject(context, PatternWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.to_string(), current.revision_id))
}

async fn apply_command(
    connection: &mut SqliteConnection,
    command: PatternWriteCommand,
    context: &PatternWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    match command {
        PatternWriteCommand::Create {
            expected_source_revision_id,
            candidate,
        } => create_candidate(connection, &expected_source_revision_id, candidate, context).await,
        PatternWriteCommand::ConfirmPending {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            expected_evidence,
            expected_reflections,
        } => {
            confirm_pending(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                &expected_evidence,
                &expected_reflections,
                context,
            )
            .await
        }
        PatternWriteCommand::RejectPending {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            expected_evidence,
            expected_reflections,
        } => {
            reject_pending(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                &expected_evidence,
                &expected_reflections,
                context,
            )
            .await
        }
    }
}

async fn prepare_write(
    connection: &mut SqliteConnection,
    command: PatternWriteCommand,
    context: &PatternWriteContext<'_>,
) -> Result<PatternWriteOutcome, MigrationError> {
    insert_guard(connection, context).await?;
    let (artifact_id, revision_id) = apply_command(connection, command, context).await?;
    verify_pattern_projection(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    inject(context, PatternWriteFailurePoint::AfterReconciliation)?;
    remove_guard(connection, context).await?;
    verify_exact_pattern_v5(connection).await?;
    let post_manifest = operation_manifest(connection).await?;
    Ok(PatternWriteOutcome {
        status: PatternWriteStatus::Committed,
        artifact_id,
        revision_id,
        operation_manifest: post_manifest,
    })
}

async fn verify_read_only(path: &Path, expected_manifest: &str) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await?;
    verify_exact_pattern_v5(&mut connection).await?;
    let actual = operation_manifest(&mut connection).await?;
    if actual != expected_manifest {
        return Err(recovery_error("pattern_operation_manifest_mismatch"));
    }
    Ok(())
}

async fn execute_with_adapter<A: CommitOutcomeAdapter>(
    path: &Path,
    command: PatternWriteCommand,
    context: PatternWriteContext<'_>,
    adapter: &A,
) -> Result<PatternWriteOutcome, MigrationError> {
    let mut connection = connect(path, false).await?;
    verify_exact_pattern_v5(&mut connection).await?;
    let pre_manifest = operation_manifest(&mut connection).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("pattern_begin_failed", error))?;
    let prepared = match prepare_write(&mut connection, command, &context).await {
        Ok(prepared) => prepared,
        Err(error) => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verify| {
                    recovery_error(format!(
                        "pattern_precommit_state_unverified:{}:{rollback:?}",
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
                        "pattern_definite_noncommit_unverified:{error_class}:{}:{rollback:?}",
                        verify.code
                    ))
                })?;
            Err(write_error(format!(
                "pattern_commit_definitely_not_committed:{error_class}"
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
                    "pattern_commit_outcome_unknown_unchanged:{error_class}"
                )));
            }
            Err(recovery_error(format!(
                "pattern_commit_outcome_unknown:{error_class}"
            )))
        }
    }
}

async fn execute_disposable(
    path: &Path,
    command: PatternWriteCommand,
    context: PatternWriteContext<'_>,
) -> Result<PatternWriteOutcome, MigrationError> {
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
    const REFLECTION_ID: &str = "fixture-pattern-reflection";
    const STARTED_AT: &str = "2026-08-01T00:00:00.000Z";
    const COMMITTED_AT: &str = "2026-08-01T00:00:01.000Z";

    async fn insert_answered_reflection(connection: &mut SqliteConnection, id: &str) {
        let payload = json!({
            "createdAt": "2026-01-01T00:00:03.000Z",
            "id": id,
            "promptProvenance": {
                "generatedAt": "2026-01-01T00:00:03.000Z",
                "harnessVersion": "harness-v1",
                "model": "fixture-model",
                "origin": "ai",
                "promptVersion": "reflection-v1",
                "provider": "gemini",
                "sourceArtifactIds": [EVIDENCE_ID],
                "sourceEntryId": SOURCE_ID
            },
            "question": "What feels useful to notice?",
            "response": "I want to keep this in my own words.",
            "responseProvenance": {
                "generatedAt": "2026-01-01T00:00:04.000Z",
                "harnessVersion": null,
                "model": null,
                "origin": "user",
                "promptVersion": null,
                "provider": null,
                "sourceArtifactIds": [id],
                "sourceEntryId": SOURCE_ID
            },
            "sourceEntryId": SOURCE_ID,
            "sourceEvidenceIds": [EVIDENCE_ID],
            "status": "answered",
            "updatedAt": "2026-01-01T00:00:04.000Z"
        });
        sqlx::query(
            "INSERT INTO persisted_artifacts \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
             VALUES (?, ?, 'reflection', ?, '2026-01-01T00:00:03.000Z', \
               '2026-01-01T00:00:04.000Z')",
        )
        .bind(id)
        .bind(SOURCE_ID)
        .bind(canonical_json(&payload).unwrap())
        .execute(connection)
        .await
        .unwrap();
    }

    async fn insert_pending_evidence(connection: &mut SqliteConnection) {
        let payload = json!({
            "createdAt": "2026-01-01T00:00:02.000Z",
            "id": "fixture-pattern-evidence-pending",
            "kind": "observation",
            "originalText": "Pending observation",
            "provenance": {
                "generatedAt": "2026-01-01T00:00:02.000Z",
                "harnessVersion": "harness-v1",
                "model": null,
                "origin": "local_mock",
                "promptVersion": "evidence-v1",
                "provider": "mock",
                "sourceArtifactIds": [],
                "sourceEntryId": SOURCE_ID
            },
            "sourceEntryId": SOURCE_ID,
            "status": "candidate",
            "text": "Pending observation",
            "updatedAt": "2026-01-01T00:00:02.000Z",
            "userEditable": true
        });
        sqlx::query(
            "INSERT INTO persisted_artifacts \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
             VALUES ('fixture-pattern-evidence-pending', ?, 'evidence', ?, \
               '2026-01-01T00:00:02.000Z', '2026-01-01T00:00:02.000Z')",
        )
        .bind(SOURCE_ID)
        .bind(canonical_json(&payload).unwrap())
        .execute(connection)
        .await
        .unwrap();
    }

    async fn exact_v5_fixture() -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        insert_pending_evidence(&mut connection).await;
        insert_answered_reflection(&mut connection, REFLECTION_ID).await;
        insert_answered_reflection(&mut connection, "fixture-pattern-reflection-two").await;
        let expected_source_manifest_digest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        drop(connection);
        migrate_disposable_v4(MigrationRequest {
            path: &path,
            expected_source_manifest_digest,
            started_at: STARTED_AT,
            committed_at: COMMITTED_AT,
            backup_id: Some("slice4b3-fixture-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        let mut read_only = connect(&path, true).await.unwrap();
        verify_exact_reflection_v5(&mut read_only).await.unwrap();
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

    async fn artifact_ref(path: &Path, id: &str, kind: &str) -> ArtifactRevisionRef {
        let mut connection = connect(path, true).await.unwrap();
        ArtifactRevisionRef {
            artifact_id: id.into(),
            revision_id: sqlx::query_scalar(
                "SELECT current_revision_id FROM artifact_heads WHERE id = ? \
                 AND artifact_kind = ? AND lifecycle_state = 'active' \
                 AND eligibility_state = 'eligible'",
            )
            .bind(id)
            .bind(kind)
            .fetch_one(&mut connection)
            .await
            .unwrap(),
        }
    }

    fn provenance(origin: &str, ids: Vec<String>) -> PatternProvenanceInput {
        PatternProvenanceInput {
            origin: origin.into(),
            provider: if origin == "ai" { "gemini" } else { "mock" }.into(),
            model: if origin == "ai" {
                Some("fixture-model".into())
            } else {
                None
            },
            harness_version: "harness-v1".into(),
            prompt_version: "single-experience-pattern-v1".into(),
            generated_at: "2026-08-01T01:00:00.000Z".into(),
            source_artifact_ids: ids,
        }
    }

    fn candidate(
        id: &str,
        origin: &str,
        evidence: Vec<ArtifactRevisionRef>,
        reflections: Vec<ArtifactRevisionRef>,
    ) -> PatternCandidateInput {
        let ids = evidence
            .iter()
            .chain(&reflections)
            .map(|item| item.artifact_id.clone())
            .collect::<Vec<_>>();
        PatternCandidateInput {
            id: id.into(),
            source_id: SOURCE_ID.into(),
            text: "A tentative single-experience hypothesis that may be useful to revisit.".into(),
            created_at: "2026-08-01T01:00:00.000Z".into(),
            evidence,
            reflections,
            provenance: provenance(origin, ids),
        }
    }

    fn context(
        occurred_at: &'static str,
        token: &'static str,
        failure_point: PatternWriteFailurePoint,
    ) -> PatternWriteContext<'static> {
        PatternWriteContext {
            occurred_at,
            guard_token: token,
            failure_point,
        }
    }

    async fn create(
        path: &Path,
        id: &str,
        origin: &str,
        reflections: Vec<ArtifactRevisionRef>,
    ) -> (
        PatternWriteOutcome,
        ArtifactRevisionRef,
        Vec<ArtifactRevisionRef>,
    ) {
        let evidence = artifact_ref(path, EVIDENCE_ID, "evidence").await;
        let outcome = execute_disposable(
            path,
            PatternWriteCommand::Create {
                expected_source_revision_id: source_revision(path).await,
                candidate: candidate(id, origin, vec![evidence.clone()], reflections.clone()),
            },
            context(
                "2026-08-01T01:00:00.000Z",
                "guard-pattern-create-000000000000000001",
                PatternWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        (outcome, evidence, reflections)
    }

    async fn scalar(path: &Path, query: &str) -> i64 {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar(query)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    #[test]
    fn durable_array_reconciliation_rejects_duplicate_or_malformed_ids() {
        assert!(exact_id_set(Some(&json!(["a", "a"])), "evidence_ids", true)
            .unwrap_err()
            .code
            .contains("duplicate"));
        assert!(
            exact_id_set(Some(&json!(["a", 7])), "reflection_ids", false)
                .unwrap_err()
                .code
                .contains("malformed")
        );
        assert!(exact_id_set(Some(&json!([])), "evidence_ids", true)
            .unwrap_err()
            .code
            .contains("required"));
        assert_eq!(
            exact_id_set(Some(&json!(["b", "a"])), "provenance_sources", true)
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>(),
            vec!["a".to_string(), "b".to_string()]
        );
    }

    #[tokio::test]
    async fn ai_and_local_mock_creation_preserve_exact_dependencies_and_provenance() {
        for (origin, reflection_count) in [("ai", 0_usize), ("local_mock", 1), ("ai", 2)] {
            let (_directory, path) = exact_v5_fixture().await;
            let mut reflections = Vec::new();
            if reflection_count >= 1 {
                reflections.push(artifact_ref(&path, REFLECTION_ID, "reflection").await);
            }
            if reflection_count == 2 {
                reflections.push(
                    artifact_ref(&path, "fixture-pattern-reflection-two", "reflection").await,
                );
            }
            let id = format!("pattern-{origin}-{reflection_count}");
            let (outcome, evidence, reflections) = create(&path, &id, origin, reflections).await;
            let mut connection = connect(&path, true).await.unwrap();
            let state: (String, String, String, String) = sqlx::query_as(
                "SELECT r.authorship, h.review_state, h.eligibility_state, pa.payload \
                 FROM artifact_heads h JOIN artifact_revisions r ON r.id=h.current_revision_id \
                 JOIN persisted_artifacts pa ON pa.id=h.id WHERE h.id=?",
            )
            .bind(&id)
            .fetch_one(&mut connection)
            .await
            .unwrap();
            assert_eq!(state.0, origin);
            assert_eq!(
                (state.1.as_str(), state.2.as_str()),
                ("pending", "ineligible")
            );
            let projected: Value = serde_json::from_str(&state.3).unwrap();
            assert_eq!(projected["status"], "candidate");
            assert_eq!(
                projected["sourceReflectionPromptIds"]
                    .as_array()
                    .unwrap()
                    .len(),
                reflection_count
            );
            let dependencies: Vec<(String, Option<String>, Option<String>)> = sqlx::query_as(
                "SELECT relationship_type, source_artifact_id, source_artifact_revision_id \
                 FROM artifact_dependencies WHERE dependent_revision_id=? \
                 ORDER BY relationship_type, source_artifact_id",
            )
            .bind(&outcome.revision_id)
            .fetch_all(&mut connection)
            .await
            .unwrap();
            assert!(dependencies.iter().any(|item| {
                item.0 == "uses_evidence"
                    && item.1.as_deref() == Some(evidence.artifact_id.as_str())
                    && item.2.as_deref() == Some(evidence.revision_id.as_str())
            }));
            for reflection in reflections {
                assert!(dependencies.iter().any(|item| {
                    item.0 == "uses_reflection_response"
                        && item.1.as_deref() == Some(reflection.artifact_id.as_str())
                        && item.2.as_deref() == Some(reflection.revision_id.as_str())
                }));
            }
            verify_exact_pattern_v5(&mut connection).await.unwrap();
        }
    }

    #[tokio::test]
    async fn provenance_mismatch_duplicate_and_context_recovery_sources_fail_closed() {
        let (_directory, path) = exact_v5_fixture().await;
        let evidence = artifact_ref(&path, EVIDENCE_ID, "evidence").await;
        let reflection = artifact_ref(&path, REFLECTION_ID, "reflection").await;
        for (index, sources) in [
            vec![EVIDENCE_ID.into()],
            vec![
                EVIDENCE_ID.into(),
                REFLECTION_ID.into(),
                "fixture-recovery-answered".into(),
            ],
            vec![EVIDENCE_ID.into(), EVIDENCE_ID.into(), REFLECTION_ID.into()],
        ]
        .into_iter()
        .enumerate()
        {
            let mut input = candidate(
                &format!("pattern-bad-source-{index}"),
                "ai",
                vec![evidence.clone()],
                vec![reflection.clone()],
            );
            input.provenance.source_artifact_ids = sources;
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            let error = execute_disposable(
                &path,
                PatternWriteCommand::Create {
                    expected_source_revision_id: source_revision(&path).await,
                    candidate: input,
                },
                context(
                    "2026-08-01T01:01:00.000Z",
                    "guard-pattern-bad-sources-0000000000001",
                    PatternWriteFailurePoint::None,
                ),
            )
            .await
            .unwrap_err();
            assert!(error.code.contains("pattern_generated_sources"));
            let mut connection = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        }
    }

    #[tokio::test]
    async fn malformed_duplicate_orphaned_ineligible_cross_source_and_conflicting_inputs_fail_closed(
    ) {
        let (_directory, path) = exact_v5_fixture().await;
        let source = source_revision(&path).await;
        let evidence = artifact_ref(&path, EVIDENCE_ID, "evidence").await;
        let pending = {
            let mut connection = connect(&path, true).await.unwrap();
            ArtifactRevisionRef {
                artifact_id: "fixture-pattern-evidence-pending".into(),
                revision_id: sqlx::query_scalar(
                    "SELECT current_revision_id FROM artifact_heads \
                     WHERE id='fixture-pattern-evidence-pending'",
                )
                .fetch_one(&mut connection)
                .await
                .unwrap(),
            }
        };
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let mut malformed = candidate(
            "pattern-malformed",
            "ai",
            vec![evidence.clone()],
            Vec::new(),
        );
        malformed.text = "   ".into();
        let mut duplicate = candidate(
            "pattern-duplicate-dependency",
            "ai",
            vec![evidence.clone(), evidence.clone()],
            Vec::new(),
        );
        duplicate.provenance.source_artifact_ids = vec![EVIDENCE_ID.into()];
        let orphan = ArtifactRevisionRef {
            artifact_id: "missing-evidence".into(),
            revision_id: "missing-evidence-revision".into(),
        };
        let cases = [
            (source.clone(), malformed),
            (source.clone(), duplicate),
            (
                source.clone(),
                candidate("pattern-orphan", "ai", vec![orphan], Vec::new()),
            ),
            (
                source.clone(),
                candidate("pattern-ineligible", "ai", vec![pending], Vec::new()),
            ),
        ];
        for (index, (expected_source_revision_id, input)) in cases.into_iter().enumerate() {
            assert!(
                execute_disposable(
                    &path,
                    PatternWriteCommand::Create {
                        expected_source_revision_id,
                        candidate: input,
                    },
                    context(
                        "2026-08-01T01:01:30.000Z",
                        "guard-pattern-invalid-input-00000000001",
                        PatternWriteFailurePoint::None,
                    ),
                )
                .await
                .is_err(),
                "invalid case {index} unexpectedly succeeded"
            );
        }

        let mut cross_source = candidate(
            "pattern-cross-source",
            "ai",
            vec![evidence.clone()],
            Vec::new(),
        );
        cross_source.source_id = "fixture-v4-current".into();
        let current_source_revision = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_scalar(
                "SELECT current_revision_id FROM source_heads WHERE id='fixture-v4-current'",
            )
            .fetch_one(&mut connection)
            .await
            .unwrap()
        };
        assert!(execute_disposable(
            &path,
            PatternWriteCommand::Create {
                expected_source_revision_id: current_source_revision,
                candidate: cross_source,
            },
            context(
                "2026-08-01T01:01:31.000Z",
                "guard-pattern-cross-source-000000000001",
                PatternWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());

        create(&path, "pattern-conflict", "ai", Vec::new()).await;
        assert!(execute_disposable(
            &path,
            PatternWriteCommand::Create {
                expected_source_revision_id: source_revision(&path).await,
                candidate: candidate("pattern-conflict", "ai", vec![evidence], Vec::new()),
            },
            context(
                "2026-08-01T01:01:32.000Z",
                "guard-pattern-identity-conflict-00000001",
                PatternWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());
        let mut connection = connect(&path, true).await.unwrap();
        assert_ne!(operation_manifest(&mut connection).await.unwrap(), before);
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM artifact_heads WHERE id='pattern-conflict'",
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn rejected_and_deleted_dependency_states_fail_closed_without_rebinding() {
        for lifecycle in ["content_purged", "deleted"] {
            let (_directory, path) = exact_v5_fixture().await;
            let revision = {
                let mut connection = connect(&path, true).await.unwrap();
                sqlx::query_scalar::<_, String>(
                    "SELECT current_revision_id FROM artifact_heads \
                     WHERE id='fixture-pattern-evidence-pending'",
                )
                .fetch_one(&mut connection)
                .await
                .unwrap()
            };
            {
                let mut connection = connect(&path, false).await.unwrap();
                raw_sql("BEGIN IMMEDIATE")
                    .execute(&mut connection)
                    .await
                    .unwrap();
                sqlx::query(
                    "INSERT INTO v5_compatibility_write_guard VALUES \
                     ('manual-pattern-dependency-state-0000001','2026-08-01T01:01:40.000Z')",
                )
                .execute(&mut connection)
                .await
                .unwrap();
                sqlx::query(
                    "UPDATE artifact_heads SET current_revision_id=NULL, review_state='rejected', \
                     lifecycle_state=?, eligibility_state='ineligible', \
                     eligibility_reason='fixture_dependency_unavailable' \
                     WHERE id='fixture-pattern-evidence-pending'",
                )
                .bind(lifecycle)
                .execute(&mut connection)
                .await
                .unwrap();
                sqlx::query(
                    "DELETE FROM persisted_artifacts WHERE id='fixture-pattern-evidence-pending'",
                )
                .execute(&mut connection)
                .await
                .unwrap();
                sqlx::query("DELETE FROM artifact_revision_content WHERE revision_id=?")
                    .bind(&revision)
                    .execute(&mut connection)
                    .await
                    .unwrap();
                sqlx::query("DELETE FROM v5_compatibility_write_guard")
                    .execute(&mut connection)
                    .await
                    .unwrap();
                raw_sql("COMMIT").execute(&mut connection).await.unwrap();
            }
            let unavailable = ArtifactRevisionRef {
                artifact_id: "fixture-pattern-evidence-pending".into(),
                revision_id: revision,
            };
            let input = candidate(
                &format!("pattern-unavailable-{lifecycle}"),
                "ai",
                vec![unavailable],
                Vec::new(),
            );
            assert!(execute_disposable(
                &path,
                PatternWriteCommand::Create {
                    expected_source_revision_id: source_revision(&path).await,
                    candidate: input,
                },
                context(
                    "2026-08-01T01:01:41.000Z",
                    "guard-pattern-unavailable-000000000001",
                    PatternWriteFailurePoint::None,
                ),
            )
            .await
            .is_err());
            assert_eq!(
                scalar(
                    &path,
                    "SELECT COUNT(*) FROM artifact_heads WHERE artifact_kind='pattern'"
                )
                .await,
                0
            );
        }
    }

    #[tokio::test]
    async fn confirmation_is_useful_for_reflection_without_content_or_provenance_rewrite() {
        let (_directory, path) = exact_v5_fixture().await;
        let reflection = artifact_ref(&path, REFLECTION_ID, "reflection").await;
        let (created, evidence, reflections) =
            create(&path, "pattern-confirm", "ai", vec![reflection]).await;
        let before: (String, String) = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_as(
                "SELECT c.payload, p.canonical_payload FROM artifact_revision_content c \
                 JOIN artifact_revision_provenance rp ON rp.artifact_revision_id=c.revision_id \
                 JOIN provenance_records p ON p.id=rp.provenance_id \
                 WHERE c.revision_id=? AND rp.role='content'",
            )
            .bind(&created.revision_id)
            .fetch_one(&mut connection)
            .await
            .unwrap()
        };
        execute_disposable(
            &path,
            PatternWriteCommand::ConfirmPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "pattern-confirm".into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: created.revision_id.clone(),
                expected_evidence: vec![evidence],
                expected_reflections: reflections,
            },
            context(
                "2026-08-01T01:02:00.000Z",
                "guard-pattern-confirm-0000000000000001",
                PatternWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let after: (String, String) = sqlx::query_as(
            "SELECT c.payload, p.canonical_payload FROM artifact_revision_content c \
             JOIN artifact_revision_provenance rp ON rp.artifact_revision_id=c.revision_id \
             JOIN provenance_records p ON p.id=rp.provenance_id \
             WHERE c.revision_id=? AND rp.role='content'",
        )
        .bind(&created.revision_id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(before, after);
        let state: (String, String, String) = sqlx::query_as(
            "SELECT review_state, eligibility_state, eligibility_reason FROM artifact_heads \
             WHERE id='pattern-confirm'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(state.0, "confirmed");
        assert_eq!(state.1, "eligible");
        assert_eq!(state.2, "explicitly_confirmed_useful_for_reflection");
    }

    #[tokio::test]
    async fn rejection_purges_content_and_projection_but_retains_content_free_facts() {
        let (_directory, path) = exact_v5_fixture().await;
        let (created, evidence, reflections) =
            create(&path, "pattern-reject", "local_mock", Vec::new()).await;
        execute_disposable(
            &path,
            PatternWriteCommand::RejectPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "pattern-reject".into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: created.revision_id,
                expected_evidence: vec![evidence],
                expected_reflections: reflections,
            },
            context(
                "2026-08-01T01:03:00.000Z",
                "guard-pattern-reject-00000000000000001",
                PatternWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_revision_content WHERE revision_id IN \
                 (SELECT id FROM artifact_revisions WHERE artifact_id='pattern-reject')"
            )
            .await,
            0
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM persisted_artifacts WHERE id='pattern-reject'"
            )
            .await,
            0
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM content_tombstones WHERE artifact_id='pattern-reject' \
                 AND reason_code='rejected_content_purged' AND content_digest IS NOT NULL"
            )
            .await,
            1
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_review_events WHERE artifact_id='pattern-reject' \
                 AND decision='rejected'"
            )
            .await,
            1
        );
    }

    #[tokio::test]
    async fn stale_ineligible_cross_source_and_inbound_dependent_states_fail_closed() {
        let (_directory, path) = exact_v5_fixture().await;
        let (created, evidence, reflections) =
            create(&path, "pattern-refuse", "ai", Vec::new()).await;
        let source = source_revision(&path).await;
        let bad_expected = PatternWriteCommand::ConfirmPending {
            source_id: SOURCE_ID.into(),
            artifact_id: "pattern-refuse".into(),
            expected_source_revision_id: source.clone(),
            expected_artifact_revision_id: created.revision_id.clone(),
            expected_evidence: vec![ArtifactRevisionRef {
                artifact_id: evidence.artifact_id.clone(),
                revision_id: "stale-evidence-revision".into(),
            }],
            expected_reflections: reflections.clone(),
        };
        assert!(execute_disposable(
            &path,
            bad_expected,
            context(
                "2026-08-01T01:04:00.000Z",
                "guard-pattern-stale-dependency-00000001",
                PatternWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());

        {
            let mut connection = connect(&path, false).await.unwrap();
            raw_sql("BEGIN IMMEDIATE")
                .execute(&mut connection)
                .await
                .unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard VALUES \
                 ('manual-pattern-dependent-00000000000001','2026-08-01T01:04:30.000Z')",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query(
                "INSERT INTO artifact_dependencies VALUES \
                 ('fixture-pattern-inbound','pattern-refuse',?, 'uses_evidence', \
                  NULL,'pattern-refuse',?,'2026-08-01T01:04:30.000Z')",
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
            PatternWriteCommand::RejectPending {
                source_id: SOURCE_ID.into(),
                artifact_id: "pattern-refuse".into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: created.revision_id,
                expected_evidence: vec![evidence],
                expected_reflections: reflections,
            },
            context(
                "2026-08-01T01:05:00.000Z",
                "guard-pattern-inbound-refuse-0000000001",
                PatternWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_review_events WHERE artifact_id='pattern-refuse'"
            )
            .await,
            0
        );
    }

    #[tokio::test]
    async fn injected_create_failures_roll_back_authority_projection_and_guard() {
        let points = [
            PatternWriteFailurePoint::AfterGuard,
            PatternWriteFailurePoint::AfterProvenance,
            PatternWriteFailurePoint::AfterRevision,
            PatternWriteFailurePoint::AfterContent,
            PatternWriteFailurePoint::AfterHead,
            PatternWriteFailurePoint::AfterDependency,
            PatternWriteFailurePoint::AfterLifecycle,
            PatternWriteFailurePoint::AfterProjection,
            PatternWriteFailurePoint::AfterReconciliation,
            PatternWriteFailurePoint::AfterGuardRemoval,
        ];
        for (index, point) in points.into_iter().enumerate() {
            let (_directory, path) = exact_v5_fixture().await;
            let evidence = artifact_ref(&path, EVIDENCE_ID, "evidence").await;
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            let input = candidate(
                &format!("pattern-failure-{index}"),
                "ai",
                vec![evidence],
                Vec::new(),
            );
            assert!(execute_disposable(
                &path,
                PatternWriteCommand::Create {
                    expected_source_revision_id: source_revision(&path).await,
                    candidate: input,
                },
                PatternWriteContext {
                    occurred_at: "2026-08-01T01:06:00.000Z",
                    guard_token: "guard-pattern-create-failure-00000000001",
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

    #[tokio::test]
    async fn injected_review_and_purge_failures_roll_back_exact_pending_state() {
        for reject in [false, true] {
            for point in [
                PatternWriteFailurePoint::AfterReview,
                PatternWriteFailurePoint::AfterHead,
                PatternWriteFailurePoint::AfterLifecycle,
                PatternWriteFailurePoint::AfterTombstone,
                PatternWriteFailurePoint::AfterPurge,
                PatternWriteFailurePoint::AfterProjection,
                PatternWriteFailurePoint::AfterReconciliation,
                PatternWriteFailurePoint::AfterGuardRemoval,
            ] {
                if !reject
                    && matches!(
                        point,
                        PatternWriteFailurePoint::AfterLifecycle
                            | PatternWriteFailurePoint::AfterTombstone
                            | PatternWriteFailurePoint::AfterPurge
                    )
                {
                    continue;
                }
                let (_directory, path) = exact_v5_fixture().await;
                let (created, evidence, reflections) =
                    create(&path, "pattern-review-failure", "ai", Vec::new()).await;
                let before = {
                    let mut connection = connect(&path, true).await.unwrap();
                    operation_manifest(&mut connection).await.unwrap()
                };
                let command = if reject {
                    PatternWriteCommand::RejectPending {
                        source_id: SOURCE_ID.into(),
                        artifact_id: "pattern-review-failure".into(),
                        expected_source_revision_id: source_revision(&path).await,
                        expected_artifact_revision_id: created.revision_id,
                        expected_evidence: vec![evidence],
                        expected_reflections: reflections,
                    }
                } else {
                    PatternWriteCommand::ConfirmPending {
                        source_id: SOURCE_ID.into(),
                        artifact_id: "pattern-review-failure".into(),
                        expected_source_revision_id: source_revision(&path).await,
                        expected_artifact_revision_id: created.revision_id,
                        expected_evidence: vec![evidence],
                        expected_reflections: reflections,
                    }
                };
                assert!(execute_disposable(
                    &path,
                    command,
                    PatternWriteContext {
                        occurred_at: "2026-08-01T01:07:00.000Z",
                        guard_token: "guard-pattern-review-failure-0000000001",
                        failure_point: point,
                    },
                )
                .await
                .is_err());
                let mut connection = connect(&path, true).await.unwrap();
                assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
            }
        }
    }

    struct InjectedCommitAdapter {
        outcome: CommitAttemptOutcome,
        commit_first: bool,
        calls: Cell<usize>,
    }

    impl CommitOutcomeAdapter for InjectedCommitAdapter {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            self.calls.set(self.calls.get() + 1);
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
    async fn ambiguous_commit_classifies_exact_post_or_pre_state_once() {
        for commit_first in [true, false] {
            let (_directory, path) = exact_v5_fixture().await;
            let evidence = artifact_ref(&path, EVIDENCE_ID, "evidence").await;
            let adapter = InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_commit_ambiguity".into(),
                },
                commit_first,
                calls: Cell::new(0),
            };
            let result = execute_with_adapter(
                &path,
                PatternWriteCommand::Create {
                    expected_source_revision_id: source_revision(&path).await,
                    candidate: candidate(
                        if commit_first {
                            "pattern-ambiguous-post"
                        } else {
                            "pattern-ambiguous-pre"
                        },
                        "ai",
                        vec![evidence],
                        Vec::new(),
                    ),
                },
                context(
                    "2026-08-01T01:08:00.000Z",
                    "guard-pattern-ambiguous-0000000000001",
                    PatternWriteFailurePoint::None,
                ),
                &adapter,
            )
            .await;
            assert_eq!(adapter.calls.get(), 1);
            if commit_first {
                assert_eq!(result.unwrap().status, PatternWriteStatus::Committed);
            } else {
                assert!(result
                    .unwrap_err()
                    .code
                    .contains("pattern_commit_outcome_unknown_unchanged"));
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
        create(&path, "pattern-receipt", "ai", Vec::new()).await;
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
        assert!(include_str!("sqlite.rs").contains("const SCHEMA_VERSION: i64 = 4;"));
    }
}
