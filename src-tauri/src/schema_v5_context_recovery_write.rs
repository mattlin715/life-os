use super::experience_write::operation_manifest;
use super::experience_write::verify_exact_v5;
use super::*;
use sqlx::{raw_sql, Row};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PromptProvenanceInput {
    pub(super) origin: String,
    pub(super) provider: String,
    pub(super) model: Option<String>,
    pub(super) harness_version: String,
    pub(super) prompt_version: String,
    pub(super) generated_at: String,
    pub(super) source_artifact_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SuggestedPromptInput {
    pub(super) id: String,
    pub(super) source_id: String,
    pub(super) question: String,
    pub(super) locale: String,
    pub(super) created_at: String,
    pub(super) provenance: PromptProvenanceInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ContextRecoveryWriteCommand {
    CreateSuggested {
        expected_source_revision_id: String,
        prompt: SuggestedPromptInput,
    },
    SaveFirstResponse {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        response: String,
    },
    SkipSuggested {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) enum ContextRecoveryWriteFailurePoint {
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
pub(super) struct ContextRecoveryWriteContext<'a> {
    pub(super) occurred_at: &'a str,
    pub(super) guard_token: &'a str,
    pub(super) failure_point: ContextRecoveryWriteFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ContextRecoveryWriteOutcome {
    pub(super) artifact_id: String,
    pub(super) revision_id: String,
    pub(super) operation_manifest: String,
}

#[derive(Clone, Debug)]
struct CurrentRecovery {
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

type RecoveryRevisionRow = (
    String,
    String,
    String,
    String,
    i64,
    i64,
    String,
    Option<String>,
);
type RecoveryDependencyRow = (String, Option<String>, Option<String>, Option<String>);
type LegacyRecoveryMetadataRow = (
    String,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
);

fn write_error(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn recovery_error(code: impl Into<String>) -> MigrationError {
    MigrationError::recovery_required(code)
}

fn validate_identifier(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        Err(write_error(format!("context_recovery_{field}_invalid")))
    } else {
        Ok(())
    }
}

fn validate_timestamp(value: &str, field: &str) -> Result<(), MigrationError> {
    validate_identifier(value, field)?;
    if !value.contains('T') || !value.ends_with('Z') {
        return Err(write_error(format!("context_recovery_{field}_invalid")));
    }
    Ok(())
}

fn validate_text(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 100_000 {
        Err(write_error(format!("context_recovery_{field}_invalid")))
    } else {
        Ok(())
    }
}

fn validate_legacy_prompt_provenance(
    raw: Option<&Value>,
    normalized: &Value,
    source_id: &str,
) -> Result<(), MigrationError> {
    let allowed = [
        "origin",
        "sourceEntryId",
        "sourceArtifactIds",
        "provider",
        "model",
        "harnessVersion",
        "promptVersion",
        "generatedAt",
    ];
    if let Some(raw) = raw {
        let object = raw
            .as_object()
            .ok_or_else(|| write_error("context_recovery_legacy_prompt_provenance_not_object"))?;
        if object.keys().any(|key| !allowed.contains(&key.as_str())) {
            return Err(write_error(
                "context_recovery_legacy_prompt_provenance_unknown_field",
            ));
        }
        if object.get("sourceEntryId").and_then(Value::as_str) != Some(source_id) {
            return Err(recovery_error(
                "context_recovery_legacy_prompt_provenance_source_mismatch",
            ));
        }
        let sources = object
            .get("sourceArtifactIds")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                write_error("context_recovery_legacy_prompt_provenance_sources_missing")
            })?;
        if !sources.is_empty() {
            return Err(write_error(
                "context_recovery_legacy_prompt_provenance_sources_unexpected",
            ));
        }
        if normalized_provenance(Some(raw), source_id, "legacy_unknown") != *normalized {
            return Err(recovery_error(
                "context_recovery_legacy_prompt_provenance_normalization_mismatch",
            ));
        }
    } else if normalized_provenance(None, source_id, "legacy_unknown") != *normalized {
        return Err(recovery_error(
            "context_recovery_legacy_unknown_prompt_provenance_mismatch",
        ));
    }
    if !matches!(
        normalized.get("origin").and_then(Value::as_str),
        Some("ai" | "local_mock" | "legacy_unknown")
    ) {
        return Err(write_error(
            "context_recovery_legacy_prompt_origin_unsupported",
        ));
    }
    Ok(())
}

fn legacy_terminal_projection(
    payload: &Value,
    status: &str,
    occurred_at: &str,
) -> Result<Value, MigrationError> {
    let mut projected = payload.clone();
    let object = projected
        .as_object_mut()
        .ok_or_else(|| write_error("context_recovery_legacy_payload_not_object"))?;
    object.insert("status".into(), Value::String(status.into()));
    object.insert("updatedAt".into(), Value::String(occurred_at.into()));
    Ok(projected)
}

async fn validate_legacy_suggested_recovery(
    connection: &mut SqliteConnection,
    current: &CurrentRecovery,
    expected_source_revision_id: &str,
) -> Result<String, MigrationError> {
    if current.serialization_version != "legacy-v4-raw" || current.revision_number != 1 {
        return Err(write_error(
            "context_recovery_legacy_baseline_contract_mismatch",
        ));
    }
    let metadata: Option<LegacyRecoveryMetadataRow> = sqlx::query_as(
        "SELECT r.revision_reason, r.predecessor_revision_id, r.content_digest, \
                    c.payload, pa.payload, pa.updated_at, h.eligibility_reason \
             FROM artifact_revisions r \
             JOIN artifact_revision_content c ON c.revision_id = r.id \
             JOIN artifact_heads h ON h.id = r.artifact_id AND h.current_revision_id = r.id \
             JOIN persisted_artifacts pa ON pa.id = h.id AND pa.source_entry_id = h.source_id \
               AND pa.artifact_kind = 'recovery_turn' \
             WHERE r.id = ? AND r.artifact_id = ? AND r.source_id = ?",
    )
    .bind(&current.revision_id)
    .bind(&current.artifact_id)
    .bind(&current.source_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_legacy_metadata_unreadable", error))?;
    let (reason, predecessor, digest, raw, projection, updated_at, eligibility_reason) =
        metadata.ok_or_else(|| recovery_error("context_recovery_legacy_metadata_missing"))?;
    if reason != "legacy_v4_baseline"
        || predecessor.is_some()
        || digest != sha256_hex(raw.as_bytes())
        || raw.as_bytes() != projection.as_bytes()
        || updated_at
            != current
                .payload
                .get("updatedAt")
                .and_then(Value::as_str)
                .unwrap_or_default()
    {
        return Err(recovery_error("context_recovery_legacy_authority_mismatch"));
    }
    let object = current
        .payload
        .as_object()
        .ok_or_else(|| write_error("context_recovery_legacy_payload_not_object"))?;
    let allowed = [
        "id",
        "sourceEntryId",
        "question",
        "response",
        "status",
        "locale",
        "promptProvenance",
        "provenance",
        "responseProvenance",
        "createdAt",
        "updatedAt",
    ];
    let required = [
        "id",
        "sourceEntryId",
        "question",
        "status",
        "locale",
        "createdAt",
        "updatedAt",
    ];
    if object.keys().any(|key| !allowed.contains(&key.as_str())) {
        return Err(write_error("context_recovery_legacy_payload_unknown_field"));
    }
    if required.iter().any(|key| !object.contains_key(*key)) {
        return Err(write_error("context_recovery_legacy_payload_field_missing"));
    }
    if object.get("id").and_then(Value::as_str) != Some(&current.artifact_id)
        || object.get("sourceEntryId").and_then(Value::as_str) != Some(&current.source_id)
        || object.get("status").and_then(Value::as_str) != Some("suggested")
        || current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
        || eligibility_reason != "legacy_v4_pending"
        || !matches!(
            current.authorship.as_str(),
            "ai" | "local_mock" | "legacy_unknown"
        )
        || object.get("response").is_some_and(|value| !value.is_null())
        || object.get("responseProvenance").is_some()
        || current.response_provenance.is_some()
    {
        return Err(write_error(
            "context_recovery_legacy_suggested_contract_mismatch",
        ));
    }
    validate_text(
        object
            .get("question")
            .and_then(Value::as_str)
            .ok_or_else(|| write_error("context_recovery_legacy_question_missing"))?,
        "legacy_question",
    )?;
    let locale = object
        .get("locale")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("context_recovery_legacy_locale_missing"))?;
    if !matches!(locale, "en" | "zh-TW" | "ja") {
        return Err(write_error("context_recovery_legacy_locale_unsupported"));
    }
    let created_at = object
        .get("createdAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("context_recovery_legacy_created_at_missing"))?;
    let raw_updated_at = object
        .get("updatedAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("context_recovery_legacy_updated_at_missing"))?;
    validate_timestamp(created_at, "legacy_created_at")?;
    validate_timestamp(raw_updated_at, "legacy_updated_at")?;
    if created_at != current.created_at || raw_updated_at != updated_at {
        return Err(recovery_error("context_recovery_legacy_timestamp_mismatch"));
    }
    validate_legacy_prompt_provenance(
        object
            .get("promptProvenance")
            .or_else(|| object.get("provenance")),
        &current.prompt_provenance,
        &current.source_id,
    )?;
    let source_edges: Vec<String> = sqlx::query_scalar(
        "SELECT source_revision_id FROM artifact_dependencies \
         WHERE dependent_artifact_id = ? AND dependent_revision_id = ? \
           AND relationship_type = 'derived_from_experience' \
           AND source_artifact_id IS NULL AND source_artifact_revision_id IS NULL",
    )
    .bind(&current.artifact_id)
    .bind(&current.revision_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| {
        migration_error(
            "context_recovery_legacy_source_dependency_unreadable",
            error,
        )
    })?;
    if source_edges != [expected_source_revision_id.to_string()]
        || inbound_dependency_count(connection, &current.artifact_id, &current.revision_id).await?
            != 0
    {
        return Err(recovery_error(
            "context_recovery_legacy_dependency_mismatch",
        ));
    }
    let lifecycle_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_lifecycle_events WHERE artifact_id = ? \
           AND subject_revision_id = ? AND event_type = 'baseline_imported' \
           AND actor = 'legacy_import' AND reason_code = 'legacy_v4_baseline'",
    )
    .bind(&current.artifact_id)
    .bind(&current.revision_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_legacy_lifecycle_unreadable", error))?;
    if lifecycle_count != 1 {
        return Err(recovery_error("context_recovery_legacy_lifecycle_mismatch"));
    }
    Ok(current.revision_id.clone())
}

fn validate_prompt(prompt: &SuggestedPromptInput) -> Result<(), MigrationError> {
    validate_identifier(&prompt.id, "artifact_id")?;
    validate_identifier(&prompt.source_id, "source_id")?;
    validate_text(&prompt.question, "question")?;
    validate_timestamp(&prompt.created_at, "created_at")?;
    if !matches!(prompt.locale.as_str(), "en" | "zh-TW" | "ja") {
        return Err(write_error("context_recovery_locale_invalid"));
    }
    if !matches!(prompt.provenance.origin.as_str(), "ai" | "local_mock") {
        return Err(write_error("context_recovery_prompt_origin_invalid"));
    }
    match prompt.provenance.origin.as_str() {
        "ai" => {
            if !matches!(prompt.provenance.provider.as_str(), "openai" | "gemini")
                || prompt
                    .provenance
                    .model
                    .as_deref()
                    .map(str::trim)
                    .filter(|v| !v.is_empty())
                    .is_none()
            {
                return Err(write_error("context_recovery_ai_provenance_invalid"));
            }
        }
        "local_mock" => {
            if prompt.provenance.provider != "mock" || prompt.provenance.model.is_some() {
                return Err(write_error(
                    "context_recovery_local_mock_provenance_invalid",
                ));
            }
        }
        _ => unreachable!(),
    }
    validate_identifier(&prompt.provenance.harness_version, "harness_version")?;
    validate_identifier(&prompt.provenance.prompt_version, "prompt_version")?;
    validate_timestamp(&prompt.provenance.generated_at, "generated_at")?;
    if prompt.provenance.generated_at != prompt.created_at {
        return Err(write_error("context_recovery_generated_time_mismatch"));
    }
    let mut ids = BTreeSet::new();
    for id in &prompt.provenance.source_artifact_ids {
        validate_identifier(id, "source_artifact_id")?;
        if !ids.insert(id.as_str()) {
            return Err(write_error("context_recovery_prompt_sources_duplicate"));
        }
    }
    if !ids.is_empty() {
        return Err(write_error("context_recovery_prompt_sources_unsupported"));
    }
    Ok(())
}

fn inject(
    context: &ContextRecoveryWriteContext<'_>,
    point: ContextRecoveryWriteFailurePoint,
) -> Result<(), MigrationError> {
    if context.failure_point == point {
        Err(write_error(format!(
            "injected_context_recovery_write_failure:{point:?}"
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
    question: &str,
    response: Option<&str>,
    locale: &str,
    created_at: &str,
) -> Value {
    json!({
        "createdAt": created_at,
        "id": id,
        "locale": locale,
        "question": question,
        "response": response,
        "sourceEntryId": source_id
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
        .ok_or_else(|| write_error("context_recovery_content_not_object"))?;
    let mut projection = Map::new();
    for key in ["createdAt", "id", "locale", "question", "sourceEntryId"] {
        projection.insert(
            key.into(),
            object
                .get(key)
                .cloned()
                .ok_or_else(|| write_error(format!("context_recovery_{key}_missing")))?,
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
    .map_err(|error| migration_error("context_recovery_source_lookup_failed", error))?;
    let Some((revision, lifecycle, content, projection)) = row else {
        return Err(write_error("context_recovery_source_not_found"));
    };
    if revision != expected_revision_id || lifecycle != "active" || content != projection {
        return Err(write_error("context_recovery_source_revision_stale"));
    }
    Ok(())
}

async fn insert_guard(
    connection: &mut SqliteConnection,
    context: &ContextRecoveryWriteContext<'_>,
) -> Result<(), MigrationError> {
    validate_identifier(context.guard_token, "guard_token")?;
    validate_timestamp(context.occurred_at, "occurred_at")?;
    if context.guard_token.len() < 32 {
        return Err(write_error("context_recovery_guard_token_invalid"));
    }
    sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
        .bind(context.guard_token)
        .bind(context.occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("context_recovery_guard_insert_failed", error))?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterGuard)
}

async fn remove_guard(
    connection: &mut SqliteConnection,
    context: &ContextRecoveryWriteContext<'_>,
) -> Result<(), MigrationError> {
    let result = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
        .bind(context.guard_token)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("context_recovery_guard_remove_failed", error))?;
    if result.rows_affected() != 1 {
        return Err(recovery_error("context_recovery_guard_identity_mismatch"));
    }
    let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("context_recovery_guard_count_failed", error))?;
    if remaining != 0 {
        return Err(recovery_error("context_recovery_guard_not_empty"));
    }
    inject(context, ContextRecoveryWriteFailurePoint::AfterGuardRemoval)
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
    .map_err(|error| migration_error("context_recovery_revision_insert_failed", error))?;
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
    .map_err(|error| migration_error("context_recovery_content_insert_failed", error))?;
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
    .map_err(|error| migration_error("context_recovery_provenance_link_failed", error))?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn insert_dependencies(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
    source_id: &str,
    source_revision_id: &str,
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
           relationship_type, source_revision_id, source_artifact_id, source_artifact_revision_id, created_at) \
         VALUES (?, ?, ?, 'derived_from_experience', ?, NULL, NULL, ?)",
    )
    .bind(source_dependency).bind(artifact_id).bind(revision_id).bind(source_revision_id).bind(occurred_at)
    .execute(&mut *connection).await
    .map_err(|error| migration_error("context_recovery_source_dependency_insert_failed", error))?;
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
               relationship_type, source_revision_id, source_artifact_id, source_artifact_revision_id, created_at) \
             VALUES (?, ?, ?, 'answers_prompt', NULL, ?, ?, ?)",
        )
        .bind(id).bind(artifact_id).bind(revision_id).bind(artifact_id).bind(prompt_revision_id).bind(occurred_at)
        .execute(&mut *connection).await
        .map_err(|error| migration_error("context_recovery_prompt_dependency_insert_failed", error))?;
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
    .map_err(|error| migration_error("context_recovery_lifecycle_insert_failed", error))?;
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
        "life-os/context-recovery-skipped-review-event-id-v1",
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
    .map_err(|error| migration_error("context_recovery_skip_event_insert_failed", error))?;
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
               created_at, updated_at) VALUES (?, ?, 'recovery_turn', ?, ?, ?)",
        )
        .bind(artifact_id)
        .bind(source_id)
        .bind(payload)
        .bind(created_at)
        .bind(updated_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("context_recovery_projection_insert_failed", error))?;
    } else {
        let changed = sqlx::query(
            "UPDATE persisted_artifacts SET payload = ?, updated_at = ? \
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'recovery_turn'",
        )
        .bind(payload)
        .bind(updated_at)
        .bind(artifact_id)
        .bind(source_id)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("context_recovery_projection_update_failed", error))?;
        if changed.rows_affected() != 1 {
            return Err(recovery_error(
                "context_recovery_projection_identity_mismatch",
            ));
        }
    }
    Ok(())
}

async fn current_recovery(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_revision_id: &str,
) -> Result<CurrentRecovery, MigrationError> {
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
           AND pa.artifact_kind = 'recovery_turn' \
         JOIN artifact_revision_provenance pp ON pp.artifact_revision_id = r.id AND pp.role = 'prompt' \
         JOIN provenance_records p ON p.id = pp.provenance_id \
         LEFT JOIN artifact_revision_provenance rpl ON rpl.artifact_revision_id = r.id \
           AND rpl.role = 'response' \
         LEFT JOIN provenance_records rp ON rp.id = rpl.provenance_id \
         WHERE h.id = ? AND h.artifact_kind = 'recovery_turn'",
    )
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_current_lookup_failed", error))?
    .ok_or_else(|| write_error("context_recovery_artifact_not_found"))?;
    let actual_source: String = row.get(0);
    let revision_id: String = row.get(1);
    if actual_source != source_id || revision_id != expected_revision_id {
        return Err(write_error("context_recovery_artifact_revision_stale"));
    }
    let raw_payload: String = row.get(10);
    let digest: String = row.get(9);
    if digest != sha256_hex(raw_payload.as_bytes()) {
        return Err(recovery_error("context_recovery_current_digest_mismatch"));
    }
    Ok(CurrentRecovery {
        artifact_id: artifact_id.to_string(),
        source_id: actual_source,
        revision_id,
        revision_number: row.get(6),
        review_state: row.get(2),
        lifecycle_state: row.get(3),
        eligibility_state: row.get(4),
        serialization_version: row.get(7),
        authorship: row.get(8),
        payload: serde_json::from_str(&raw_payload).map_err(|error| {
            migration_error("context_recovery_current_payload_malformed", error)
        })?,
        prompt_provenance_id: row.get(12),
        prompt_provenance: serde_json::from_str(&row.get::<String, _>(13)).map_err(|error| {
            migration_error("context_recovery_prompt_provenance_malformed", error)
        })?,
        response_provenance: row
            .try_get::<Option<String>, _>(14)
            .map_err(|error| {
                migration_error("context_recovery_response_provenance_unreadable", error)
            })?
            .map(|raw| serde_json::from_str(&raw))
            .transpose()
            .map_err(|error| {
                migration_error("context_recovery_response_provenance_malformed", error)
            })?,
        created_at: row.get(5),
    })
}

async fn initial_prompt_revision(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    current_revision_id: &str,
) -> Result<String, MigrationError> {
    let linked: Vec<String> = sqlx::query_scalar(
        "SELECT source_artifact_revision_id FROM artifact_dependencies \
         WHERE dependent_artifact_id = ? AND dependent_revision_id = ? \
           AND relationship_type = 'answers_prompt' AND source_artifact_id = ?",
    )
    .bind(artifact_id)
    .bind(current_revision_id)
    .bind(artifact_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_prompt_link_lookup_failed", error))?;
    if linked.len() == 1 {
        return Ok(linked[0].clone());
    }
    if !linked.is_empty() {
        return Err(recovery_error(
            "context_recovery_prompt_link_count_mismatch",
        ));
    }
    sqlx::query_scalar(
        "SELECT id FROM artifact_revisions WHERE artifact_id = ? AND revision_number = 1",
    )
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_initial_revision_lookup_failed", error))?
    .ok_or_else(|| recovery_error("context_recovery_initial_prompt_missing"))
}

async fn validate_prompt_lineage(
    connection: &mut SqliteConnection,
    current: &CurrentRecovery,
) -> Result<String, MigrationError> {
    let prompt_revision =
        initial_prompt_revision(connection, &current.artifact_id, &current.revision_id).await?;
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT c.payload, rp.provenance_id FROM artifact_revisions r \
         JOIN artifact_revision_content c ON c.revision_id = r.id \
         JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id AND rp.role = 'prompt' \
         WHERE r.id = ? AND r.artifact_id = ? AND r.revision_number = 1 AND r.predecessor_revision_id IS NULL",
    )
    .bind(&prompt_revision).bind(&current.artifact_id).fetch_optional(&mut *connection).await
    .map_err(|error| migration_error("context_recovery_initial_prompt_lookup_failed", error))?;
    let (raw, provenance_id) =
        row.ok_or_else(|| recovery_error("context_recovery_initial_prompt_missing"))?;
    let initial: Value = serde_json::from_str(&raw)
        .map_err(|error| migration_error("context_recovery_initial_prompt_malformed", error))?;
    for key in ["createdAt", "id", "locale", "question", "sourceEntryId"] {
        if initial.get(key) != current.payload.get(key) {
            return Err(recovery_error(format!(
                "context_recovery_prompt_lineage_mismatch:{key}"
            )));
        }
    }
    if initial.get("response").is_some_and(|v| !v.is_null())
        || provenance_id != current.prompt_provenance_id
    {
        return Err(recovery_error("context_recovery_prompt_lineage_mismatch"));
    }
    Ok(prompt_revision)
}

async fn inbound_dependency_count(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
) -> Result<i64, MigrationError> {
    let normalized: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_dependencies WHERE source_artifact_id = ? \
           AND source_artifact_revision_id = ? \
           AND NOT (dependent_artifact_id = ? AND relationship_type = 'answers_prompt')",
    )
    .bind(artifact_id)
    .bind(revision_id)
    .bind(artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_inbound_dependency_lookup_failed", error))?;
    let historical: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM historical_artifact_dependencies WHERE source_artifact_id = ?",
    )
    .bind(artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| {
        migration_error(
            "context_recovery_historical_dependency_lookup_failed",
            error,
        )
    })?;
    Ok(normalized + historical)
}

async fn create_suggested(
    connection: &mut SqliteConnection,
    expected_source_revision_id: &str,
    prompt: SuggestedPromptInput,
    context: &ContextRecoveryWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    validate_prompt(&prompt)?;
    exact_current_source(connection, &prompt.source_id, expected_source_revision_id).await?;
    let conflict: i64 = sqlx::query_scalar(
        "SELECT (SELECT COUNT(*) FROM artifact_heads WHERE id = ?) + \
                (SELECT COUNT(*) FROM persisted_artifacts WHERE id = ?)",
    )
    .bind(&prompt.id)
    .bind(&prompt.id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_identity_check_failed", error))?;
    if conflict != 0 {
        return Err(write_error("context_recovery_identity_conflict"));
    }
    let open: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_heads WHERE source_id = ? AND artifact_kind = 'recovery_turn' \
           AND review_state = 'pending' AND lifecycle_state = 'active' AND eligibility_state = 'ineligible'",
    ).bind(&prompt.source_id).fetch_one(&mut *connection).await
      .map_err(|error| migration_error("context_recovery_open_turn_lookup_failed", error))?;
    if open != 0 {
        return Err(write_error("context_recovery_open_turn_exists"));
    }

    let content = content_value(
        &prompt.id,
        &prompt.source_id,
        &prompt.question,
        None,
        &prompt.locale,
        &prompt.created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id =
        artifact_revision_id(&prompt.id, "recovery_turn", &prompt.created_at, &digest);
    let provenance = prompt_provenance_value(&prompt.source_id, &prompt.provenance);
    let provenance_id = insert_provenance(connection, &provenance, &prompt.created_at).await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterProvenance)?;
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
    inject(context, ContextRecoveryWriteFailurePoint::AfterRevision)?;
    insert_content(connection, &revision_id, &payload).await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterContent)?;
    sqlx::query(
        "INSERT INTO artifact_heads (id, source_id, artifact_kind, current_revision_id, review_state, \
           lifecycle_state, eligibility_state, eligibility_reason, created_at, updated_at) \
         VALUES (?, ?, 'recovery_turn', ?, 'pending', 'active', 'ineligible', \
           'context_recovery_suggested_pending_user_response', ?, ?)",
    ).bind(&prompt.id).bind(&prompt.source_id).bind(&revision_id).bind(&prompt.created_at).bind(&prompt.created_at)
      .execute(&mut *connection).await
      .map_err(|error| migration_error("context_recovery_head_insert_failed", error))?;
    link_provenance(connection, &revision_id, "prompt", &provenance_id).await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterHead)?;
    insert_dependencies(
        connection,
        &prompt.id,
        &revision_id,
        &prompt.source_id,
        expected_source_revision_id,
        None,
        &prompt.created_at,
    )
    .await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/context-recovery-suggested-event-id-v1",
        "created",
        "system",
        "context_recovery_prompt_suggested",
        &prompt.id,
        &revision_id,
        None,
        &prompt.created_at,
    )
    .await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterLifecycle)?;
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
    inject(context, ContextRecoveryWriteFailurePoint::AfterProjection)?;
    Ok((prompt.id, revision_id))
}

#[allow(clippy::too_many_arguments)]
async fn save_first_response(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    response: &str,
    context: &ContextRecoveryWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    validate_text(response, "response")?;
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_recovery(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
        || !matches!(
            current.serialization_version.as_str(),
            "canonical-json-v1" | "legacy-v4-raw"
        )
        || !matches!(
            current.authorship.as_str(),
            "ai" | "local_mock" | "legacy_unknown"
        )
        || current
            .payload
            .get("response")
            .is_some_and(|v| !v.is_null())
        || current.response_provenance.is_some()
    {
        return Err(write_error("context_recovery_first_response_not_allowed"));
    }
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "context_recovery_inbound_dependency_requires_later_slice",
        ));
    }
    let prompt_revision = if current.serialization_version == "legacy-v4-raw" {
        validate_legacy_suggested_recovery(connection, &current, expected_source_revision_id)
            .await?
    } else {
        validate_prompt_lineage(connection, &current).await?
    };
    let question = current
        .payload
        .get("question")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("context_recovery_question_missing"))?;
    let locale = current
        .payload
        .get("locale")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("context_recovery_locale_missing"))?;
    let created_at = current
        .payload
        .get("createdAt")
        .and_then(Value::as_str)
        .ok_or_else(|| write_error("context_recovery_created_at_missing"))?;
    let content = content_value(
        artifact_id,
        source_id,
        question,
        Some(response),
        locale,
        created_at,
    );
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id =
        artifact_revision_id(artifact_id, "recovery_turn", context.occurred_at, &digest);
    let response_provenance =
        response_provenance_value(source_id, artifact_id, context.occurred_at);
    let response_provenance_id =
        insert_provenance(connection, &response_provenance, context.occurred_at).await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterProvenance)?;
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
    inject(context, ContextRecoveryWriteFailurePoint::AfterRevision)?;
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
    inject(context, ContextRecoveryWriteFailurePoint::AfterContent)?;
    insert_dependencies(
        connection,
        artifact_id,
        &revision_id,
        source_id,
        expected_source_revision_id,
        Some(&prompt_revision),
        context.occurred_at,
    )
    .await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterDependency)?;
    insert_lifecycle_event(
        connection,
        "life-os/context-recovery-answered-event-id-v1",
        "created",
        "user",
        "context_recovery_response_saved",
        artifact_id,
        &revision_id,
        Some(&current.revision_id),
        context.occurred_at,
    )
    .await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterLifecycle)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = ?, review_state = 'not_applicable', \
           lifecycle_state = 'active', eligibility_state = 'eligible', \
           eligibility_reason = 'context_recovery_answered_current_experience_task_only', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? AND artifact_kind = 'recovery_turn' \
           AND review_state = 'pending' AND lifecycle_state = 'active' AND eligibility_state = 'ineligible'",
    ).bind(&revision_id).bind(context.occurred_at).bind(artifact_id).bind(source_id).bind(&current.revision_id)
      .execute(&mut *connection).await
      .map_err(|error| migration_error("context_recovery_answer_head_update_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("context_recovery_artifact_revision_stale"));
    }
    inject(context, ContextRecoveryWriteFailurePoint::AfterHead)?;
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
    inject(context, ContextRecoveryWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.into(), revision_id))
}

#[allow(clippy::too_many_arguments)]
async fn skip_suggested(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_source_revision_id: &str,
    expected_artifact_revision_id: &str,
    context: &ContextRecoveryWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let current = current_recovery(
        connection,
        source_id,
        artifact_id,
        expected_artifact_revision_id,
    )
    .await?;
    if current.review_state != "pending"
        || current.lifecycle_state != "active"
        || current.eligibility_state != "ineligible"
        || !matches!(
            current.serialization_version.as_str(),
            "canonical-json-v1" | "legacy-v4-raw"
        )
        || !matches!(
            current.authorship.as_str(),
            "ai" | "local_mock" | "legacy_unknown"
        )
        || current
            .payload
            .get("response")
            .is_some_and(|v| !v.is_null())
        || current.response_provenance.is_some()
    {
        return Err(write_error("context_recovery_skip_not_allowed"));
    }
    if inbound_dependency_count(connection, artifact_id, &current.revision_id).await? != 0 {
        return Err(write_error(
            "context_recovery_inbound_dependency_requires_later_slice",
        ));
    }
    if current.serialization_version == "legacy-v4-raw" {
        validate_legacy_suggested_recovery(connection, &current, expected_source_revision_id)
            .await?;
    } else {
        validate_prompt_lineage(connection, &current).await?;
    }
    insert_skip_event(
        connection,
        artifact_id,
        &current.revision_id,
        context.occurred_at,
    )
    .await?;
    inject(context, ContextRecoveryWriteFailurePoint::AfterReview)?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET review_state = 'skipped', lifecycle_state = 'active', \
           eligibility_state = 'ineligible', eligibility_reason = 'context_recovery_explicit_user_skip', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? AND artifact_kind = 'recovery_turn' \
           AND review_state = 'pending' AND lifecycle_state = 'active' AND eligibility_state = 'ineligible'",
    ).bind(context.occurred_at).bind(artifact_id).bind(source_id).bind(&current.revision_id)
      .execute(&mut *connection).await
      .map_err(|error| migration_error("context_recovery_skip_head_update_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(write_error("context_recovery_artifact_revision_stale"));
    }
    inject(context, ContextRecoveryWriteFailurePoint::AfterHead)?;
    let projection = if current.serialization_version == "legacy-v4-raw" {
        legacy_terminal_projection(&current.payload, "skipped", context.occurred_at)?
    } else {
        v4_projection_value(
            &current.payload,
            "skipped",
            context.occurred_at,
            &current.prompt_provenance,
            None,
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
    inject(context, ContextRecoveryWriteFailurePoint::AfterProjection)?;
    Ok((artifact_id.into(), current.revision_id))
}

async fn apply_command(
    connection: &mut SqliteConnection,
    command: ContextRecoveryWriteCommand,
    context: &ContextRecoveryWriteContext<'_>,
) -> Result<(String, String), MigrationError> {
    match command {
        ContextRecoveryWriteCommand::CreateSuggested {
            expected_source_revision_id,
            prompt,
        } => create_suggested(connection, &expected_source_revision_id, prompt, context).await,
        ContextRecoveryWriteCommand::SaveFirstResponse {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            response,
        } => {
            save_first_response(
                connection,
                &source_id,
                &artifact_id,
                &expected_source_revision_id,
                &expected_artifact_revision_id,
                &response,
                context,
            )
            .await
        }
        ContextRecoveryWriteCommand::SkipSuggested {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
        } => {
            skip_suggested(
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

fn durable_id_set(value: &Value, field: &str) -> Result<BTreeSet<String>, MigrationError> {
    let items = value
        .as_array()
        .ok_or_else(|| recovery_error(format!("context_recovery_{field}_not_array")))?;
    let mut ids = BTreeSet::new();
    for item in items {
        let id = item
            .as_str()
            .ok_or_else(|| recovery_error(format!("context_recovery_{field}_id_not_string")))?;
        validate_identifier(id, field)
            .map_err(|_| recovery_error(format!("context_recovery_{field}_id_invalid")))?;
        if !ids.insert(id.to_string()) {
            return Err(recovery_error(format!(
                "context_recovery_{field}_duplicate"
            )));
        }
    }
    Ok(ids)
}

fn verify_prompt_provenance(
    value: &Value,
    source_id: &str,
    created_at: &str,
) -> Result<(), MigrationError> {
    let origin = value
        .get("origin")
        .and_then(Value::as_str)
        .ok_or_else(|| recovery_error("context_recovery_prompt_origin_missing"))?;
    if value.get("sourceEntryId").and_then(Value::as_str) != Some(source_id)
        || value.get("generatedAt").and_then(Value::as_str) != Some(created_at)
        || !durable_id_set(
            value
                .get("sourceArtifactIds")
                .ok_or_else(|| recovery_error("context_recovery_prompt_sources_missing"))?,
            "prompt_sources",
        )?
        .is_empty()
    {
        return Err(recovery_error(
            "context_recovery_prompt_provenance_mismatch",
        ));
    }
    let provider = value.get("provider").and_then(Value::as_str);
    let model = value
        .get("model")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let harness = value
        .get("harnessVersion")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let prompt = value
        .get("promptVersion")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty());
    if harness.is_none() || prompt.is_none() {
        return Err(recovery_error("context_recovery_prompt_contract_missing"));
    }
    match origin {
        "ai" if matches!(provider, Some("openai" | "gemini")) && model.is_some() => Ok(()),
        "local_mock" if provider == Some("mock") && model.is_none() => Ok(()),
        _ => Err(recovery_error("context_recovery_prompt_provenance_invalid")),
    }
}

fn verify_response_provenance(
    value: &Value,
    source_id: &str,
    artifact_id: &str,
) -> Result<(), MigrationError> {
    let expected = BTreeSet::from([artifact_id.to_string()]);
    if value.get("origin").and_then(Value::as_str) != Some("user")
        || value.get("sourceEntryId").and_then(Value::as_str) != Some(source_id)
        || durable_id_set(
            value
                .get("sourceArtifactIds")
                .ok_or_else(|| recovery_error("context_recovery_response_sources_missing"))?,
            "response_sources",
        )? != expected
        || value.get("provider").is_some_and(|v| !v.is_null())
        || value.get("model").is_some_and(|v| !v.is_null())
        || value.get("harnessVersion").is_some_and(|v| !v.is_null())
        || value.get("promptVersion").is_some_and(|v| !v.is_null())
    {
        return Err(recovery_error(
            "context_recovery_response_provenance_invalid",
        ));
    }
    let generated = value
        .get("generatedAt")
        .and_then(Value::as_str)
        .ok_or_else(|| recovery_error("context_recovery_response_generated_at_missing"))?;
    validate_timestamp(generated, "response_generated_at")
        .map_err(|_| recovery_error("context_recovery_response_generated_at_invalid"))
}

async fn verify_context_recovery_projection(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    let heads = sqlx::query(
        "SELECT h.id, h.source_id, h.current_revision_id, h.review_state, h.lifecycle_state, \
                h.eligibility_state, h.eligibility_reason, h.created_at, h.updated_at \
         FROM artifact_heads h WHERE h.artifact_kind = 'recovery_turn' ORDER BY h.id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_heads_unreadable", error))?;
    let mut projected = 0_i64;
    for head in heads {
        let artifact_id: String = head.get(0);
        let source_id: String = head.get(1);
        validate_identifier(&artifact_id, "durable_artifact_id")
            .map_err(|_| recovery_error("context_recovery_durable_artifact_id_invalid"))?;
        validate_identifier(&source_id, "durable_source_id")
            .map_err(|_| recovery_error("context_recovery_durable_source_id_invalid"))?;
        let revision_id: String = head
            .try_get::<Option<String>, _>(2)
            .map_err(|error| migration_error("context_recovery_head_revision_unreadable", error))?
            .ok_or_else(|| recovery_error("context_recovery_active_revision_missing"))?;
        validate_identifier(&revision_id, "durable_revision_id")
            .map_err(|_| recovery_error("context_recovery_durable_revision_id_invalid"))?;
        let review_state: String = head.get(3);
        let lifecycle_state: String = head.get(4);
        let eligibility_state: String = head.get(5);
        let eligibility_reason: String = head.get(6);
        if lifecycle_state == "invalidated" {
            if eligibility_state != "ineligible"
                || eligibility_reason != "source_experience_revision_superseded"
            {
                return Err(recovery_error("context_recovery_invalidated_head_mismatch"));
            }
            let revision: Option<RecoveryRevisionRow> = sqlx::query_as(
                "SELECT r.serialization_version, r.authorship, r.content_digest, c.payload, c.byte_length, \
                        r.revision_number, r.revision_reason, r.predecessor_revision_id \
                 FROM artifact_revisions r JOIN artifact_revision_content c ON c.revision_id = r.id \
                 WHERE r.id = ? AND r.artifact_id = ? AND r.source_id = ?",
            )
            .bind(&revision_id)
            .bind(&artifact_id)
            .bind(&source_id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("context_recovery_invalidated_revision_unreadable", error)
            })?;
            let (
                serialization,
                authorship,
                digest,
                content,
                byte_length,
                revision_number,
                revision_reason,
                predecessor,
            ) = revision
                .ok_or_else(|| recovery_error("context_recovery_invalidated_content_missing"))?;
            if digest != sha256_hex(content.as_bytes()) || byte_length != content.len() as i64 {
                return Err(recovery_error(
                    "context_recovery_invalidated_content_digest_mismatch",
                ));
            }
            let projection_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM persisted_artifacts \
                 WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'recovery_turn'",
            )
            .bind(&artifact_id)
            .bind(&source_id)
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("context_recovery_invalidated_projection_unreadable", error)
            })?;
            if projection_count != 0 {
                return Err(recovery_error(
                    "context_recovery_invalidated_projection_present",
                ));
            }
            let content_value: Value = serde_json::from_str(&content).map_err(|error| {
                migration_error("context_recovery_invalidated_content_malformed", error)
            })?;
            if content_value.get("id").and_then(Value::as_str) != Some(&artifact_id)
                || content_value.get("sourceEntryId").and_then(Value::as_str) != Some(&source_id)
                || !matches!(
                    content_value.get("locale").and_then(Value::as_str),
                    Some("en" | "zh-TW" | "ja")
                )
                || content_value
                    .get("question")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .is_none()
            {
                return Err(recovery_error(
                    "context_recovery_invalidated_content_contract_mismatch",
                ));
            }
            let prompt_rows: Vec<String> = sqlx::query_scalar(
                "SELECT p.canonical_payload FROM artifact_revision_provenance rp \
                 JOIN provenance_records p ON p.id = rp.provenance_id \
                 WHERE rp.artifact_revision_id = ? AND rp.role = 'prompt'",
            )
            .bind(&revision_id)
            .fetch_all(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("context_recovery_invalidated_prompt_unreadable", error)
            })?;
            if prompt_rows.len() != 1 {
                return Err(recovery_error(
                    "context_recovery_invalidated_prompt_count_mismatch",
                ));
            }
            let prompt: Value = serde_json::from_str(&prompt_rows[0]).map_err(|error| {
                migration_error("context_recovery_invalidated_prompt_malformed", error)
            })?;
            let initial_created = content_value
                .get("createdAt")
                .and_then(Value::as_str)
                .ok_or_else(|| recovery_error("context_recovery_created_at_missing"))?;
            verify_prompt_provenance(&prompt, &source_id, initial_created)?;
            let response_rows: Vec<String> = sqlx::query_scalar(
                "SELECT p.canonical_payload FROM artifact_revision_provenance rp \
                 JOIN provenance_records p ON p.id = rp.provenance_id \
                 WHERE rp.artifact_revision_id = ? AND rp.role = 'response'",
            )
            .bind(&revision_id)
            .fetch_all(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("context_recovery_invalidated_response_unreadable", error)
            })?;
            match review_state.as_str() {
                "pending" | "skipped" => {
                    if !response_rows.is_empty()
                        || content_value
                            .get("response")
                            .is_some_and(|value| !value.is_null())
                    {
                        return Err(recovery_error(
                            "context_recovery_invalidated_terminal_mismatch",
                        ));
                    }
                }
                "not_applicable" => {
                    if response_rows.len() != 1
                        || content_value
                            .get("response")
                            .and_then(Value::as_str)
                            .map(str::trim)
                            .filter(|value| !value.is_empty())
                            .is_none()
                    {
                        return Err(recovery_error(
                            "context_recovery_invalidated_answer_mismatch",
                        ));
                    }
                    let response: Value =
                        serde_json::from_str(&response_rows[0]).map_err(|error| {
                            migration_error(
                                "context_recovery_invalidated_response_malformed",
                                error,
                            )
                        })?;
                    verify_response_provenance(&response, &source_id, &artifact_id)?;
                }
                _ => {
                    return Err(recovery_error(
                        "context_recovery_invalidated_review_unsupported",
                    ))
                }
            }
            if serialization == "canonical-json-v1" {
                match review_state.as_str() {
                    "pending" | "skipped"
                        if revision_number == 1
                            && revision_reason == "created"
                            && predecessor.is_none()
                            && matches!(authorship.as_str(), "ai" | "local_mock") => {}
                    "not_applicable"
                        if revision_number == 2
                            && revision_reason == "answered"
                            && predecessor.is_some()
                            && authorship == "mixed" => {}
                    _ => {
                        return Err(recovery_error(
                            "context_recovery_invalidated_revision_contract_mismatch",
                        ))
                    }
                }
            } else if serialization != "legacy-v4-raw" {
                return Err(recovery_error(
                    "context_recovery_invalidated_serialization_unsupported",
                ));
            }
            let dependencies: Vec<RecoveryDependencyRow> = sqlx::query_as(
                "SELECT relationship_type, source_revision_id, source_artifact_id, source_artifact_revision_id \
                 FROM artifact_dependencies WHERE dependent_artifact_id = ? \
                   AND dependent_revision_id = ? ORDER BY relationship_type",
            )
            .bind(&artifact_id)
            .bind(&revision_id)
            .fetch_all(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("context_recovery_invalidated_dependencies_unreadable", error)
            })?;
            let source_count = dependencies
                .iter()
                .filter(
                    |(relationship, source_revision, source_artifact, source_artifact_revision)| {
                        relationship == "derived_from_experience"
                            && source_revision.is_some()
                            && source_artifact.is_none()
                            && source_artifact_revision.is_none()
                    },
                )
                .count();
            let answer_count = dependencies
                .iter()
                .filter(
                    |(relationship, _, source_artifact, source_artifact_revision)| {
                        relationship == "answers_prompt"
                            && source_artifact.as_deref() == Some(&artifact_id)
                            && source_artifact_revision.is_some()
                    },
                )
                .count();
            let dependency_contract_matches = if serialization == "legacy-v4-raw" {
                dependencies.len() == 1 && answer_count == 0
            } else if review_state == "not_applicable" {
                dependencies.len() == 2 && answer_count == 1
            } else {
                dependencies.len() == 1 && answer_count == 0
            };
            if source_count != 1 || !dependency_contract_matches {
                return Err(recovery_error(
                    "context_recovery_invalidated_dependency_contract_mismatch",
                ));
            }
            if !super::experience_write::verify_source_caused_invalidation(
                connection,
                &artifact_id,
                &revision_id,
                &source_id,
                &head.get::<String, _>(8),
            )
            .await?
                || inbound_dependency_count(connection, &artifact_id, &revision_id).await? != 0
            {
                return Err(recovery_error(
                    "context_recovery_invalidated_facts_mismatch",
                ));
            }
            continue;
        }
        if lifecycle_state != "active" {
            return Err(recovery_error("context_recovery_lifecycle_unsupported"));
        }
        let revision: Option<RecoveryRevisionRow> = sqlx::query_as(
            "SELECT r.serialization_version, r.authorship, r.content_digest, c.payload, c.byte_length, \
                    r.revision_number, r.revision_reason, r.predecessor_revision_id \
             FROM artifact_revisions r JOIN artifact_revision_content c ON c.revision_id = r.id \
             WHERE r.id = ? AND r.artifact_id = ? AND r.source_id = ?",
        ).bind(&revision_id).bind(&artifact_id).bind(&source_id).fetch_optional(&mut *connection).await
          .map_err(|error| migration_error("context_recovery_revision_unreadable", error))?;
        let (
            serialization,
            authorship,
            digest,
            content,
            byte_length,
            revision_number,
            revision_reason,
            predecessor,
        ) = revision.ok_or_else(|| recovery_error("context_recovery_active_content_missing"))?;
        if digest != sha256_hex(content.as_bytes()) || byte_length != content.len() as i64 {
            return Err(recovery_error("context_recovery_content_digest_mismatch"));
        }
        let projection: Option<(String, String, String)> = sqlx::query_as(
            "SELECT payload, created_at, updated_at FROM persisted_artifacts \
             WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'recovery_turn'",
        )
        .bind(&artifact_id)
        .bind(&source_id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("context_recovery_projection_unreadable", error))?;
        let (projection, created_at, updated_at) =
            projection.ok_or_else(|| recovery_error("context_recovery_projection_missing"))?;
        projected += 1;
        if created_at != head.get::<String, _>(7) || updated_at != head.get::<String, _>(8) {
            return Err(recovery_error(
                "context_recovery_projection_timestamp_mismatch",
            ));
        }
        if serialization == "legacy-v4-raw" {
            if content.as_bytes() == projection.as_bytes() {
                continue;
            }
            if review_state != "skipped" || eligibility_state != "ineligible" {
                return Err(recovery_error(
                    "context_recovery_legacy_projection_mismatch",
                ));
            }
            let content_value: Value = serde_json::from_str(&content).map_err(|error| {
                migration_error("context_recovery_legacy_content_malformed", error)
            })?;
            let projection_value: Value = serde_json::from_str(&projection).map_err(|error| {
                migration_error("context_recovery_legacy_projection_malformed", error)
            })?;
            let expected =
                legacy_terminal_projection(&content_value, "skipped", &head.get::<String, _>(8))?;
            if projection_value != expected {
                return Err(recovery_error(
                    "context_recovery_legacy_skip_projection_mismatch",
                ));
            }
            let exact_skip: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM artifact_review_events WHERE artifact_id = ? \
                   AND subject_revision_id = ? AND decision = 'skipped' AND actor = 'user' \
                   AND event_origin = 'explicit_user_action' AND occurred_at = ? \
                   AND timestamp_quality = 'exact_action_time'",
            )
            .bind(&artifact_id)
            .bind(&revision_id)
            .bind(head.get::<String, _>(8))
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| migration_error("context_recovery_legacy_skip_unreadable", error))?;
            if exact_skip != 1 {
                return Err(recovery_error(
                    "context_recovery_legacy_skip_event_mismatch",
                ));
            }
            continue;
        }
        if serialization != "canonical-json-v1" {
            return Err(recovery_error(
                "context_recovery_serialization_version_unsupported",
            ));
        }
        let content_value: Value = serde_json::from_str(&content)
            .map_err(|error| migration_error("context_recovery_content_malformed", error))?;
        let projection_value: Value = serde_json::from_str(&projection)
            .map_err(|error| migration_error("context_recovery_projection_malformed", error))?;
        for key in ["createdAt", "id", "locale", "question", "sourceEntryId"] {
            if content_value.get(key) != projection_value.get(key) {
                return Err(recovery_error(format!(
                    "context_recovery_projection_field_mismatch:{key}"
                )));
            }
        }
        if content_value.get("id").and_then(Value::as_str) != Some(&artifact_id)
            || content_value.get("sourceEntryId").and_then(Value::as_str) != Some(&source_id)
            || !matches!(
                content_value.get("locale").and_then(Value::as_str),
                Some("en" | "zh-TW" | "ja")
            )
            || content_value
                .get("question")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .is_none()
        {
            return Err(recovery_error("context_recovery_content_contract_mismatch"));
        }
        let prompt_rows = sqlx::query(
            "SELECT rp.provenance_id, p.canonical_payload FROM artifact_revision_provenance rp \
             JOIN provenance_records p ON p.id = rp.provenance_id \
             WHERE rp.artifact_revision_id = ? AND rp.role = 'prompt'",
        )
        .bind(&revision_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("context_recovery_prompt_provenance_unreadable", error))?;
        if prompt_rows.len() != 1 {
            return Err(recovery_error(
                "context_recovery_prompt_provenance_count_mismatch",
            ));
        }
        let prompt_provenance_id: String = prompt_rows[0].get(0);
        validate_identifier(&prompt_provenance_id, "durable_prompt_provenance_id")
            .map_err(|_| recovery_error("context_recovery_prompt_provenance_id_invalid"))?;
        let prompt_provenance: Value = serde_json::from_str(&prompt_rows[0].get::<String, _>(1))
            .map_err(|error| {
                migration_error("context_recovery_prompt_provenance_malformed", error)
            })?;
        let initial_created = content_value
            .get("createdAt")
            .and_then(Value::as_str)
            .ok_or_else(|| recovery_error("context_recovery_created_at_missing"))?;
        verify_prompt_provenance(&prompt_provenance, &source_id, initial_created)?;
        if projection_value.get("promptProvenance") != Some(&prompt_provenance) {
            return Err(recovery_error(
                "context_recovery_prompt_provenance_projection_mismatch",
            ));
        }
        let response_rows = sqlx::query(
            "SELECT p.canonical_payload FROM artifact_revision_provenance rp JOIN provenance_records p ON p.id = rp.provenance_id \
             WHERE rp.artifact_revision_id = ? AND rp.role = 'response'",
        ).bind(&revision_id).fetch_all(&mut *connection).await
          .map_err(|error| migration_error("context_recovery_response_provenance_unreadable", error))?;
        let expected_status = match (review_state.as_str(), eligibility_state.as_str()) {
            ("pending", "ineligible") => {
                if revision_number != 1
                    || revision_reason != "created"
                    || predecessor.is_some()
                    || eligibility_reason != "context_recovery_suggested_pending_user_response"
                    || !matches!(authorship.as_str(), "ai" | "local_mock")
                    || content_value.get("response").is_some_and(|v| !v.is_null())
                    || !response_rows.is_empty()
                {
                    return Err(recovery_error("context_recovery_suggested_state_mismatch"));
                }
                "suggested"
            }
            ("skipped", "ineligible") => {
                if revision_number != 1
                    || revision_reason != "created"
                    || predecessor.is_some()
                    || eligibility_reason != "context_recovery_explicit_user_skip"
                    || !matches!(authorship.as_str(), "ai" | "local_mock")
                    || content_value.get("response").is_some_and(|v| !v.is_null())
                    || !response_rows.is_empty()
                {
                    return Err(recovery_error("context_recovery_skipped_state_mismatch"));
                }
                let reviews: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM artifact_review_events WHERE artifact_id = ? AND subject_revision_id = ? \
                       AND decision = 'skipped' AND actor = 'user' AND event_origin = 'explicit_user_action'",
                ).bind(&artifact_id).bind(&revision_id).fetch_one(&mut *connection).await
                  .map_err(|error| migration_error("context_recovery_skip_review_unreadable", error))?;
                if reviews != 1 {
                    return Err(recovery_error("context_recovery_skip_review_missing"));
                }
                "skipped"
            }
            ("not_applicable", "eligible") => {
                if revision_number != 2
                    || revision_reason != "answered"
                    || predecessor.is_none()
                    || eligibility_reason
                        != "context_recovery_answered_current_experience_task_only"
                    || authorship != "mixed"
                    || response_rows.len() != 1
                {
                    return Err(recovery_error("context_recovery_answer_state_mismatch"));
                }
                content_value
                    .get("response")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|v| !v.is_empty())
                    .ok_or_else(|| recovery_error("context_recovery_answer_response_missing"))?;
                let response_provenance: Value = serde_json::from_str(
                    &response_rows[0].get::<String, _>(0),
                )
                .map_err(|error| {
                    migration_error("context_recovery_response_provenance_malformed", error)
                })?;
                verify_response_provenance(&response_provenance, &source_id, &artifact_id)?;
                if projection_value.get("response") != content_value.get("response")
                    || projection_value.get("responseProvenance") != Some(&response_provenance)
                {
                    return Err(recovery_error(
                        "context_recovery_response_projection_mismatch",
                    ));
                }
                let prompt_target =
                    initial_prompt_revision(connection, &artifact_id, &revision_id).await?;
                if predecessor.as_deref() != Some(prompt_target.as_str()) {
                    return Err(recovery_error(
                        "context_recovery_answer_predecessor_mismatch",
                    ));
                }
                let initial: Option<(String, String)> = sqlx::query_as(
                    "SELECT c.payload, rp.provenance_id FROM artifact_revisions r \
                     JOIN artifact_revision_content c ON c.revision_id = r.id \
                     JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id AND rp.role = 'prompt' \
                     WHERE r.id = ? AND r.artifact_id = ? AND r.revision_number = 1 AND r.predecessor_revision_id IS NULL",
                ).bind(&prompt_target).bind(&artifact_id).fetch_optional(&mut *connection).await
                  .map_err(|error| migration_error("context_recovery_initial_prompt_verification_failed", error))?;
                let (raw_initial, initial_provenance_id) = initial
                    .ok_or_else(|| recovery_error("context_recovery_initial_prompt_missing"))?;
                let initial_value: Value = serde_json::from_str(&raw_initial).map_err(|error| {
                    migration_error("context_recovery_initial_prompt_malformed", error)
                })?;
                for key in ["createdAt", "id", "locale", "question", "sourceEntryId"] {
                    if initial_value.get(key) != content_value.get(key) {
                        return Err(recovery_error(format!(
                            "context_recovery_prompt_lineage_mismatch:{key}"
                        )));
                    }
                }
                if initial_value.get("response").is_some_and(|v| !v.is_null())
                    || initial_provenance_id != prompt_provenance_id
                {
                    return Err(recovery_error("context_recovery_prompt_lineage_mismatch"));
                }
                "answered"
            }
            _ => {
                return Err(recovery_error(
                    "context_recovery_review_eligibility_mismatch",
                ))
            }
        };
        if projection_value.get("status").and_then(Value::as_str) != Some(expected_status) {
            return Err(recovery_error(
                "context_recovery_projection_status_mismatch",
            ));
        }
        if expected_status != "answered"
            && (projection_value.get("response").is_some()
                || projection_value.get("responseProvenance").is_some())
        {
            return Err(recovery_error(
                "context_recovery_terminal_projection_mismatch",
            ));
        }
        let dependencies: Vec<RecoveryDependencyRow> = sqlx::query_as(
            "SELECT relationship_type, source_revision_id, source_artifact_id, source_artifact_revision_id \
             FROM artifact_dependencies WHERE dependent_artifact_id = ? AND dependent_revision_id = ? ORDER BY relationship_type",
        ).bind(&artifact_id).bind(&revision_id).fetch_all(&mut *connection).await
          .map_err(|error| migration_error("context_recovery_dependencies_unreadable", error))?;
        let source_matches = dependencies
            .iter()
            .filter(|(rel, src_rev, src_art, src_art_rev)| {
                rel == "derived_from_experience"
                    && src_art.is_none()
                    && src_art_rev.is_none()
                    && src_rev.as_deref().is_some()
            })
            .count();
        if source_matches != 1 {
            return Err(recovery_error(
                "context_recovery_source_dependency_mismatch",
            ));
        }
        let actual_source_revision = dependencies
            .iter()
            .find(|(rel, _, _, _)| rel == "derived_from_experience")
            .and_then(|(_, rev, _, _)| rev.as_deref())
            .ok_or_else(|| recovery_error("context_recovery_source_dependency_missing"))?;
        let current_source_revision: String = sqlx::query_scalar(
            "SELECT current_revision_id FROM source_heads WHERE id = ? AND lifecycle_state = 'active'",
        ).bind(&source_id).fetch_optional(&mut *connection).await
          .map_err(|error| migration_error("context_recovery_source_dependency_unreadable", error))?
          .ok_or_else(|| recovery_error("context_recovery_source_dependency_stale"))?;
        if actual_source_revision != current_source_revision {
            return Err(recovery_error("context_recovery_source_dependency_stale"));
        }
        let answer_count = dependencies
            .iter()
            .filter(|(rel, _, art, rev)| {
                rel == "answers_prompt" && art.as_deref() == Some(&artifact_id) && rev.is_some()
            })
            .count();
        if (expected_status == "answered" && (dependencies.len() != 2 || answer_count != 1))
            || (expected_status != "answered" && dependencies.len() != 1)
        {
            return Err(recovery_error(
                "context_recovery_dependency_contract_mismatch",
            ));
        }
        if inbound_dependency_count(connection, &artifact_id, &revision_id).await? != 0 {
            return Err(recovery_error(
                "context_recovery_unexpected_inbound_dependency",
            ));
        }
    }
    let projection_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM persisted_artifacts WHERE artifact_kind = 'recovery_turn'",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_projection_count_failed", error))?;
    if projection_count != projected {
        return Err(recovery_error("context_recovery_projection_count_mismatch"));
    }
    let duplicate_open: Option<String> = sqlx::query_scalar(
        "SELECT source_id FROM artifact_heads WHERE artifact_kind = 'recovery_turn' AND review_state = 'pending' \
           AND lifecycle_state = 'active' AND eligibility_state = 'ineligible' GROUP BY source_id HAVING COUNT(*) > 1 LIMIT 1",
    ).fetch_optional(&mut *connection).await
      .map_err(|error| migration_error("context_recovery_open_turn_count_failed", error))?;
    if duplicate_open.is_some() {
        return Err(recovery_error("context_recovery_multiple_open_turns"));
    }
    let unsupported_inbound: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_dependencies d \
         JOIN artifact_heads source ON source.id = d.source_artifact_id \
         WHERE source.artifact_kind = 'recovery_turn' \
           AND NOT (d.dependent_artifact_id = d.source_artifact_id \
             AND d.relationship_type = 'answers_prompt')",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("context_recovery_inbound_scan_failed", error))?;
    if unsupported_inbound != 0 {
        return Err(recovery_error(
            "context_recovery_unexpected_inbound_dependency",
        ));
    }
    let historical: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM historical_artifact_dependencies d JOIN artifact_heads h ON h.id = d.source_artifact_id \
         WHERE h.artifact_kind = 'recovery_turn'",
    ).fetch_one(&mut *connection).await
      .map_err(|error| migration_error("context_recovery_historical_exclusion_unreadable", error))?;
    if historical != 0 {
        return Err(recovery_error(
            "context_recovery_historical_exclusion_violated",
        ));
    }
    Ok(())
}

pub(super) async fn verify_exact_context_recovery_v5(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    verify_exact_v5(connection).await?;
    verify_context_recovery_projection(connection).await
}

async fn prepare_write(
    connection: &mut SqliteConnection,
    command: ContextRecoveryWriteCommand,
    context: &ContextRecoveryWriteContext<'_>,
) -> Result<ContextRecoveryWriteOutcome, MigrationError> {
    insert_guard(connection, context).await?;
    let (artifact_id, revision_id) = apply_command(connection, command, context).await?;
    verify_context_recovery_projection(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    inject(
        context,
        ContextRecoveryWriteFailurePoint::AfterReconciliation,
    )?;
    remove_guard(connection, context).await?;
    verify_exact_context_recovery_v5(connection).await?;
    let post_manifest = operation_manifest(connection).await?;
    Ok(ContextRecoveryWriteOutcome {
        artifact_id,
        revision_id,
        operation_manifest: post_manifest,
    })
}

async fn verify_read_only(path: &Path, expected_manifest: &str) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await?;
    verify_exact_context_recovery_v5(&mut connection).await?;
    let actual = operation_manifest(&mut connection).await?;
    if actual != expected_manifest {
        return Err(recovery_error(
            "context_recovery_operation_manifest_mismatch",
        ));
    }
    Ok(())
}

async fn execute_with_adapter<A: CommitOutcomeAdapter>(
    path: &Path,
    command: ContextRecoveryWriteCommand,
    context: ContextRecoveryWriteContext<'_>,
    adapter: &A,
) -> Result<ContextRecoveryWriteOutcome, MigrationError> {
    let mut connection = connect(path, false).await?;
    verify_exact_context_recovery_v5(&mut connection).await?;
    let pre_manifest = operation_manifest(&mut connection).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("context_recovery_begin_failed", error))?;
    let prepared = match prepare_write(&mut connection, command, &context).await {
        Ok(prepared) => prepared,
        Err(error) => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verify| {
                    recovery_error(format!(
                        "context_recovery_precommit_state_unverified:{}:{rollback:?}",
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
                        "context_recovery_definite_noncommit_unverified:{error_class}:{}:{rollback:?}",
                        verify.code
                    ))
                })?;
            Err(write_error(format!(
                "context_recovery_commit_definitely_not_committed:{error_class}"
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
                    "context_recovery_commit_outcome_unknown_unchanged:{error_class}"
                )));
            }
            Err(recovery_error(format!(
                "context_recovery_commit_outcome_unknown:{error_class}"
            )))
        }
    }
}

pub(super) async fn execute_disposable(
    path: &Path,
    command: ContextRecoveryWriteCommand,
    context: ContextRecoveryWriteContext<'_>,
) -> Result<ContextRecoveryWriteOutcome, MigrationError> {
    execute_with_adapter(path, command, context, &SqlCommitOutcomeAdapter).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const SOURCE_ID: &str = "fixture-v4-history";
    const STARTED_AT: &str = "2026-08-02T01:00:00.000Z";
    const COMMITTED_AT: &str = "2026-08-02T01:00:01.000Z";

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
            backup_id: Some("slice4b4-fixture-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        (directory, path)
    }

    fn legacy_recovery_payload(id: &str, extra_field: bool) -> String {
        let mut payload = json!({
            "createdAt": "2026-01-01T00:00:03.000Z",
            "id": id,
            "locale": "en",
            "promptProvenance": {
                "generatedAt": "2026-01-01T00:00:03.000Z",
                "harnessVersion": "harness-v1",
                "model": null,
                "origin": "local_mock",
                "promptVersion": "context-recovery-v1",
                "provider": "mock",
                "sourceArtifactIds": [],
                "sourceEntryId": SOURCE_ID
            },
            "question": "What would help clarify this Experience?",
            "sourceEntryId": SOURCE_ID,
            "status": "suggested",
            "updatedAt": "2026-01-01T00:00:03.000Z"
        });
        if extra_field {
            payload
                .as_object_mut()
                .unwrap()
                .insert("futureField".into(), Value::String("unsupported".into()));
        }
        serde_json::to_string_pretty(&payload).unwrap()
    }

    async fn exact_v5_legacy_recovery_fixture(
        id: &str,
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
        let raw = legacy_recovery_payload(id, extra_field);
        sqlx::query(
            "INSERT INTO persisted_artifacts \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
             VALUES (?, ?, 'recovery_turn', ?, '2026-01-01T00:00:03.000Z', \
                     '2026-01-01T00:00:03.000Z')",
        )
        .bind(id)
        .bind(SOURCE_ID)
        .bind(&raw)
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
            backup_id: Some("slice4c6b-context-recovery-fixture-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        let mut read_only = connect(&path, true).await.unwrap();
        verify_exact_context_recovery_v5(&mut read_only)
            .await
            .unwrap();
        (directory, path, raw)
    }

    async fn artifact_revision(path: &Path, id: &str) -> String {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar("SELECT current_revision_id FROM artifact_heads WHERE id=?")
            .bind(id)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    async fn source_revision(path: &Path) -> String {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar("SELECT current_revision_id FROM source_heads WHERE id = ?")
            .bind(SOURCE_ID)
            .fetch_one(&mut connection)
            .await
            .unwrap()
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
            prompt_version: "context-recovery-v1".into(),
            generated_at: "2026-08-02T02:00:00.000Z".into(),
            source_artifact_ids: vec![],
        }
    }

    fn prompt(id: &str, origin: &str) -> SuggestedPromptInput {
        SuggestedPromptInput {
            id: id.into(),
            source_id: SOURCE_ID.into(),
            question: "What would help clarify this Experience?".into(),
            locale: "en".into(),
            created_at: "2026-08-02T02:00:00.000Z".into(),
            provenance: provenance(origin),
        }
    }

    fn context(
        occurred_at: &'static str,
        guard_token: &'static str,
        failure_point: ContextRecoveryWriteFailurePoint,
    ) -> ContextRecoveryWriteContext<'static> {
        ContextRecoveryWriteContext {
            occurred_at,
            guard_token,
            failure_point,
        }
    }

    async fn create(path: &Path, id: &str, origin: &str) -> (ContextRecoveryWriteOutcome, String) {
        let source = source_revision(path).await;
        let outcome = execute_disposable(
            path,
            ContextRecoveryWriteCommand::CreateSuggested {
                expected_source_revision_id: source.clone(),
                prompt: prompt(id, origin),
            },
            context(
                "2026-08-02T02:00:00.000Z",
                "guard-context-recovery-create-00000001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        (outcome, source)
    }

    async fn answer(
        path: &Path,
        id: &str,
        current_revision: &str,
        source: &str,
    ) -> ContextRecoveryWriteOutcome {
        execute_disposable(
            path,
            ContextRecoveryWriteCommand::SaveFirstResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source.into(),
                expected_artifact_revision_id: current_revision.into(),
                response: "I need a little more time to name it.".into(),
            },
            context(
                "2026-08-02T02:01:00.000Z",
                "guard-context-recovery-answer-00000001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap()
    }

    async fn skip(
        path: &Path,
        id: &str,
        current_revision: &str,
        source: &str,
    ) -> ContextRecoveryWriteOutcome {
        execute_disposable(
            path,
            ContextRecoveryWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source.into(),
                expected_artifact_revision_id: current_revision.into(),
            },
            context(
                "2026-08-02T02:01:00.000Z",
                "guard-context-recovery-skip-000000001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn ai_and_local_mock_suggestions_are_exact_and_historically_excluded() {
        for origin in ["ai", "local_mock"] {
            let (_directory, path) = exact_v5_fixture().await;
            let id = format!("recovery-{origin}");
            let (created, source) = create(&path, &id, origin).await;
            let mut connection = connect(&path, true).await.unwrap();
            let row: (String, String, String, String, String) = sqlx::query_as(
                "SELECT r.authorship, h.review_state, h.eligibility_state, h.eligibility_reason, pa.payload \
                 FROM artifact_heads h JOIN artifact_revisions r ON r.id = h.current_revision_id \
                 JOIN persisted_artifacts pa ON pa.id = h.id WHERE h.id = ? AND h.artifact_kind = 'recovery_turn'",
            ).bind(&id).fetch_one(&mut connection).await.unwrap();
            assert_eq!(row.0, origin);
            assert_eq!(
                (&row.1, &row.2),
                (&"pending".to_string(), &"ineligible".to_string())
            );
            assert_eq!(row.3, "context_recovery_suggested_pending_user_response");
            let projected: Value = serde_json::from_str(&row.4).unwrap();
            assert_eq!(projected["status"], "suggested");
            assert_eq!(projected["locale"], "en");
            assert!(projected.get("response").is_none());
            let dependencies: Vec<(String, Option<String>, Option<String>)> = sqlx::query_as(
                "SELECT relationship_type, source_revision_id, source_artifact_id FROM artifact_dependencies \
                 WHERE dependent_revision_id = ?",
            ).bind(&created.revision_id).fetch_all(&mut connection).await.unwrap();
            assert_eq!(
                dependencies,
                vec![("derived_from_experience".into(), Some(source), None)]
            );
            let historical: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM historical_artifact_dependencies WHERE source_artifact_id = ?",
            ).bind(&id).fetch_one(&mut connection).await.unwrap();
            assert_eq!(historical, 0);
            verify_exact_context_recovery_v5(&mut connection)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn only_one_open_turn_exists_but_terminal_turns_allow_later_opportunities() {
        for terminal in ["answered", "skipped"] {
            let (_directory, path) = exact_v5_fixture().await;
            let (first, source) = create(&path, "recovery-first", "local_mock").await;
            let duplicate = execute_disposable(
                &path,
                ContextRecoveryWriteCommand::CreateSuggested {
                    expected_source_revision_id: source.clone(),
                    prompt: prompt("recovery-second", "ai"),
                },
                context(
                    "2026-08-02T02:00:30.000Z",
                    "guard-context-recovery-open-refusal-001",
                    ContextRecoveryWriteFailurePoint::None,
                ),
            )
            .await
            .unwrap_err();
            assert_eq!(duplicate.code, "context_recovery_open_turn_exists");
            if terminal == "answered" {
                answer(&path, "recovery-first", &first.revision_id, &source).await;
            } else {
                skip(&path, "recovery-first", &first.revision_id, &source).await;
            }
            let later = execute_disposable(
                &path,
                ContextRecoveryWriteCommand::CreateSuggested {
                    expected_source_revision_id: source,
                    prompt: prompt("recovery-later", "local_mock"),
                },
                context(
                    "2026-08-02T02:02:00.000Z",
                    "guard-context-recovery-later-create-001",
                    ContextRecoveryWriteFailurePoint::None,
                ),
            )
            .await
            .unwrap();
            assert_eq!(later.artifact_id, "recovery-later");
        }
    }

    #[tokio::test]
    async fn first_response_appends_user_provenance_and_exact_prompt_lineage() {
        let (_directory, path) = exact_v5_fixture().await;
        let id = "recovery-answer";
        let (created, source) = create(&path, id, "ai").await;
        let answered = answer(&path, id, &created.revision_id, &source).await;
        let mut connection = connect(&path, true).await.unwrap();
        let revisions: Vec<(i64, String, String, Option<String>)> = sqlx::query_as(
            "SELECT revision_number, authorship, revision_reason, predecessor_revision_id FROM artifact_revisions \
             WHERE artifact_id = ? ORDER BY revision_number",
        ).bind(id).fetch_all(&mut connection).await.unwrap();
        assert_eq!(revisions.len(), 2);
        assert_eq!(
            (
                revisions[1].0,
                revisions[1].1.as_str(),
                revisions[1].2.as_str()
            ),
            (2, "mixed", "answered")
        );
        assert_eq!(
            revisions[1].3.as_deref(),
            Some(created.revision_id.as_str())
        );
        let roles: Vec<String> = sqlx::query_scalar(
            "SELECT role FROM artifact_revision_provenance WHERE artifact_revision_id = ? ORDER BY role",
        ).bind(&answered.revision_id).fetch_all(&mut connection).await.unwrap();
        assert_eq!(roles, vec!["prompt", "response"]);
        let answer_target: String = sqlx::query_scalar(
            "SELECT source_artifact_revision_id FROM artifact_dependencies WHERE dependent_revision_id = ? \
             AND relationship_type = 'answers_prompt'",
        ).bind(&answered.revision_id).fetch_one(&mut connection).await.unwrap();
        assert_eq!(answer_target, created.revision_id);
        let head: (String, String, String) = sqlx::query_as(
            "SELECT review_state, eligibility_state, eligibility_reason FROM artifact_heads WHERE id = ?",
        ).bind(id).fetch_one(&mut connection).await.unwrap();
        assert_eq!(
            head,
            (
                "not_applicable".into(),
                "eligible".into(),
                "context_recovery_answered_current_experience_task_only".into()
            )
        );
        let initial_payload: String = sqlx::query_scalar(
            "SELECT payload FROM artifact_revision_content WHERE revision_id = ?",
        )
        .bind(created.revision_id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert!(!initial_payload.contains("I need a little more time"));
        verify_exact_context_recovery_v5(&mut connection)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn explicit_skip_is_terminal_without_response_or_new_revision() {
        let (_directory, path) = exact_v5_fixture().await;
        let id = "recovery-skip";
        let (created, source) = create(&path, id, "local_mock").await;
        let skipped = skip(&path, id, &created.revision_id, &source).await;
        assert_eq!(skipped.revision_id, created.revision_id);
        let mut connection = connect(&path, true).await.unwrap();
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id = ?")
                .bind(id)
                .fetch_one(&mut connection)
                .await
                .unwrap();
        let responses: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM artifact_revision_provenance WHERE artifact_revision_id = ? AND role = 'response'",
        ).bind(&skipped.revision_id).fetch_one(&mut connection).await.unwrap();
        let head: (String, String, String) = sqlx::query_as(
            "SELECT review_state, eligibility_state, eligibility_reason FROM artifact_heads WHERE id = ?",
        ).bind(id).fetch_one(&mut connection).await.unwrap();
        assert_eq!(count, 1);
        assert_eq!(responses, 0);
        assert_eq!(
            head,
            (
                "skipped".into(),
                "ineligible".into(),
                "context_recovery_explicit_user_skip".into()
            )
        );
    }

    #[tokio::test]
    async fn malformed_blank_duplicate_cross_source_and_stale_inputs_fail_closed() {
        let cases = [
            "blank_question",
            "blank_response",
            "duplicate_sources",
            "unsupported_locale",
            "cross_source",
            "stale_source",
        ];
        for case in cases {
            let (_directory, path) = exact_v5_fixture().await;
            let source = source_revision(&path).await;
            let before = {
                let mut c = connect(&path, true).await.unwrap();
                operation_manifest(&mut c).await.unwrap()
            };
            let result = match case {
                "blank_question" => {
                    let mut p = prompt("recovery-invalid", "ai");
                    p.question = "   ".into();
                    execute_disposable(
                        &path,
                        ContextRecoveryWriteCommand::CreateSuggested {
                            expected_source_revision_id: source,
                            prompt: p,
                        },
                        context(
                            "2026-08-02T02:03:00.000Z",
                            "guard-context-recovery-invalid-000001",
                            ContextRecoveryWriteFailurePoint::None,
                        ),
                    )
                    .await
                }
                "duplicate_sources" => {
                    let mut p = prompt("recovery-invalid", "ai");
                    p.provenance.source_artifact_ids = vec!["duplicate".into(), "duplicate".into()];
                    execute_disposable(
                        &path,
                        ContextRecoveryWriteCommand::CreateSuggested {
                            expected_source_revision_id: source,
                            prompt: p,
                        },
                        context(
                            "2026-08-02T02:03:00.000Z",
                            "guard-context-recovery-invalid-000002",
                            ContextRecoveryWriteFailurePoint::None,
                        ),
                    )
                    .await
                }
                "unsupported_locale" => {
                    let mut p = prompt("recovery-invalid", "ai");
                    p.locale = "fr".into();
                    execute_disposable(
                        &path,
                        ContextRecoveryWriteCommand::CreateSuggested {
                            expected_source_revision_id: source,
                            prompt: p,
                        },
                        context(
                            "2026-08-02T02:03:00.000Z",
                            "guard-context-recovery-invalid-000003",
                            ContextRecoveryWriteFailurePoint::None,
                        ),
                    )
                    .await
                }
                "cross_source" => {
                    let mut p = prompt("recovery-invalid", "ai");
                    p.source_id = "fixture-v4-current".into();
                    execute_disposable(
                        &path,
                        ContextRecoveryWriteCommand::CreateSuggested {
                            expected_source_revision_id: source,
                            prompt: p,
                        },
                        context(
                            "2026-08-02T02:03:00.000Z",
                            "guard-context-recovery-invalid-000004",
                            ContextRecoveryWriteFailurePoint::None,
                        ),
                    )
                    .await
                }
                "stale_source" => {
                    execute_disposable(
                        &path,
                        ContextRecoveryWriteCommand::CreateSuggested {
                            expected_source_revision_id: "stale-source-revision".into(),
                            prompt: prompt("recovery-invalid", "ai"),
                        },
                        context(
                            "2026-08-02T02:03:00.000Z",
                            "guard-context-recovery-invalid-000005",
                            ContextRecoveryWriteFailurePoint::None,
                        ),
                    )
                    .await
                }
                "blank_response" => {
                    let (created, source) =
                        create(&path, "recovery-invalid-answer", "local_mock").await;
                    execute_disposable(
                        &path,
                        ContextRecoveryWriteCommand::SaveFirstResponse {
                            source_id: SOURCE_ID.into(),
                            artifact_id: "recovery-invalid-answer".into(),
                            expected_source_revision_id: source,
                            expected_artifact_revision_id: created.revision_id,
                            response: "  ".into(),
                        },
                        context(
                            "2026-08-02T02:03:00.000Z",
                            "guard-context-recovery-invalid-000006",
                            ContextRecoveryWriteFailurePoint::None,
                        ),
                    )
                    .await
                }
                _ => unreachable!(),
            };
            assert!(result.is_err(), "{case}");
            if case != "blank_response" {
                let mut c = connect(&path, true).await.unwrap();
                assert_eq!(operation_manifest(&mut c).await.unwrap(), before);
            }
        }
    }

    #[tokio::test]
    async fn terminal_or_stale_head_cannot_be_answered_or_skipped_again() {
        let (_directory, path) = exact_v5_fixture().await;
        let (created, source) = create(&path, "recovery-terminal", "ai").await;
        let answered = answer(&path, "recovery-terminal", &created.revision_id, &source).await;
        let skip_after_answer = execute_disposable(
            &path,
            ContextRecoveryWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: "recovery-terminal".into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: answered.revision_id,
            },
            context(
                "2026-08-02T02:02:00.000Z",
                "guard-context-recovery-terminal-00001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(skip_after_answer.code, "context_recovery_skip_not_allowed");
        let stale_answer = execute_disposable(
            &path,
            ContextRecoveryWriteCommand::SaveFirstResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: "recovery-terminal".into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: created.revision_id,
                response: "Must fail.".into(),
            },
            context(
                "2026-08-02T02:03:00.000Z",
                "guard-context-recovery-stale-head-00001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(
            stale_answer.code,
            "context_recovery_artifact_revision_stale"
        );
    }

    #[tokio::test]
    async fn deleted_source_or_invalidated_turn_fails_closed_without_writer_mutation() {
        for condition in ["deleted_source", "invalidated_turn"] {
            let (_directory, path) = exact_v5_fixture().await;
            let source = source_revision(&path).await;
            let created = if condition == "invalidated_turn" {
                Some(
                    create(&path, "recovery-inactive-turn", "local_mock")
                        .await
                        .0,
                )
            } else {
                None
            };
            let mut connection = connect(&path, false).await.unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)",
            )
            .bind("guard-context-recovery-inactive-source-01")
            .bind("2026-08-02T02:02:00.000Z")
            .execute(&mut connection)
            .await
            .unwrap();
            if condition == "deleted_source" {
                sqlx::query(
                    "UPDATE source_heads SET current_revision_id = NULL, lifecycle_state = 'deleted' WHERE id = ?",
                )
                    .bind(SOURCE_ID)
                    .execute(&mut connection)
                    .await
                    .unwrap();
            } else {
                sqlx::query(
                    "UPDATE artifact_heads SET lifecycle_state = 'invalidated' WHERE id = ?",
                )
                .bind("recovery-inactive-turn")
                .execute(&mut connection)
                .await
                .unwrap();
            }
            sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
                .bind("guard-context-recovery-inactive-source-01")
                .execute(&mut connection)
                .await
                .unwrap();
            drop(connection);
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            let command = if let Some(created) = created {
                ContextRecoveryWriteCommand::SaveFirstResponse {
                    source_id: SOURCE_ID.into(),
                    artifact_id: "recovery-inactive-turn".into(),
                    expected_source_revision_id: source,
                    expected_artifact_revision_id: created.revision_id,
                    response: "Must not save.".into(),
                }
            } else {
                ContextRecoveryWriteCommand::CreateSuggested {
                    expected_source_revision_id: source,
                    prompt: prompt("recovery-inactive-source", "local_mock"),
                }
            };
            let result = execute_disposable(
                &path,
                command,
                context(
                    "2026-08-02T02:03:00.000Z",
                    "guard-context-recovery-inactive-write-01",
                    ContextRecoveryWriteFailurePoint::None,
                ),
            )
            .await;
            assert!(result.is_err(), "{condition}");
            let mut connection = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        }
    }

    #[tokio::test]
    async fn unexpected_inbound_dependency_fails_closed_without_rebinding() {
        let (_directory, path) = exact_v5_fixture().await;
        let id = "recovery-dependent";
        let (created, source) = create(&path, id, "local_mock").await;
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
            &created.revision_id,
        );
        sqlx::query(
            "INSERT INTO artifact_dependencies (id, dependent_artifact_id, dependent_revision_id, relationship_type, \
             source_revision_id, source_artifact_id, source_artifact_revision_id, created_at) \
             VALUES (?, 'fixture-v4-question', ?, 'uses_reflection_response', NULL, ?, ?, ?)",
        ).bind(dependency).bind(dependent_revision).bind(id).bind(&created.revision_id).bind("2026-08-02T02:01:00.000Z")
          .execute(&mut connection).await.unwrap();
        drop(connection);
        let error = execute_disposable(
            &path,
            ContextRecoveryWriteCommand::SaveFirstResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source,
                expected_artifact_revision_id: created.revision_id,
                response: "No silent rebinding.".into(),
            },
            context(
                "2026-08-02T02:02:00.000Z",
                "guard-context-recovery-dependent-0001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert!(
            error.code.contains("inbound_dependency") || error.code.contains("unexpected_inbound")
        );
    }

    #[tokio::test]
    async fn every_create_boundary_rolls_back_to_exact_logical_prestate() {
        let points = [
            ContextRecoveryWriteFailurePoint::AfterGuard,
            ContextRecoveryWriteFailurePoint::AfterProvenance,
            ContextRecoveryWriteFailurePoint::AfterRevision,
            ContextRecoveryWriteFailurePoint::AfterContent,
            ContextRecoveryWriteFailurePoint::AfterHead,
            ContextRecoveryWriteFailurePoint::AfterDependency,
            ContextRecoveryWriteFailurePoint::AfterLifecycle,
            ContextRecoveryWriteFailurePoint::AfterProjection,
            ContextRecoveryWriteFailurePoint::AfterReconciliation,
            ContextRecoveryWriteFailurePoint::AfterGuardRemoval,
        ];
        for (index, point) in points.into_iter().enumerate() {
            let (_directory, path) = exact_v5_fixture().await;
            let source = source_revision(&path).await;
            let before = {
                let mut c = connect(&path, true).await.unwrap();
                operation_manifest(&mut c).await.unwrap()
            };
            let error = execute_disposable(
                &path,
                ContextRecoveryWriteCommand::CreateSuggested {
                    expected_source_revision_id: source,
                    prompt: prompt(&format!("recovery-injected-{index}"), "ai"),
                },
                ContextRecoveryWriteContext {
                    occurred_at: "2026-08-02T02:04:00.000Z",
                    guard_token: "guard-context-recovery-injected-00001",
                    failure_point: point,
                },
            )
            .await
            .unwrap_err();
            assert!(!error.recovery_required, "{point:?}: {error}");
            let mut c = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut c).await.unwrap(), before);
            assert_eq!(
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM v5_compatibility_write_guard")
                    .fetch_one(&mut c)
                    .await
                    .unwrap(),
                0
            );
        }
    }

    #[tokio::test]
    async fn answer_and_skip_failure_boundaries_roll_back() {
        for operation in ["answer", "skip"] {
            for (index, point) in [
                ContextRecoveryWriteFailurePoint::AfterReview,
                ContextRecoveryWriteFailurePoint::AfterProvenance,
                ContextRecoveryWriteFailurePoint::AfterRevision,
                ContextRecoveryWriteFailurePoint::AfterContent,
                ContextRecoveryWriteFailurePoint::AfterDependency,
                ContextRecoveryWriteFailurePoint::AfterLifecycle,
                ContextRecoveryWriteFailurePoint::AfterHead,
                ContextRecoveryWriteFailurePoint::AfterProjection,
            ]
            .into_iter()
            .enumerate()
            {
                if operation == "skip"
                    && !matches!(
                        point,
                        ContextRecoveryWriteFailurePoint::AfterReview
                            | ContextRecoveryWriteFailurePoint::AfterHead
                            | ContextRecoveryWriteFailurePoint::AfterProjection
                    )
                {
                    continue;
                }
                if operation == "answer" && point == ContextRecoveryWriteFailurePoint::AfterReview {
                    continue;
                }
                let (_directory, path) = exact_v5_fixture().await;
                let (created, source) = create(&path, "recovery-op-rollback", "local_mock").await;
                let before = {
                    let mut c = connect(&path, true).await.unwrap();
                    operation_manifest(&mut c).await.unwrap()
                };
                let command = if operation == "answer" {
                    ContextRecoveryWriteCommand::SaveFirstResponse {
                        source_id: SOURCE_ID.into(),
                        artifact_id: "recovery-op-rollback".into(),
                        expected_source_revision_id: source,
                        expected_artifact_revision_id: created.revision_id,
                        response: "Rollback me.".into(),
                    }
                } else {
                    ContextRecoveryWriteCommand::SkipSuggested {
                        source_id: SOURCE_ID.into(),
                        artifact_id: "recovery-op-rollback".into(),
                        expected_source_revision_id: source,
                        expected_artifact_revision_id: created.revision_id,
                    }
                };
                let error = execute_disposable(
                    &path,
                    command,
                    ContextRecoveryWriteContext {
                        occurred_at: "2026-08-02T02:05:00.000Z",
                        guard_token: "guard-context-recovery-op-rollback-001",
                        failure_point: point,
                    },
                )
                .await
                .unwrap_err();
                assert!(
                    error.code.contains("injected"),
                    "{operation}:{index}:{}",
                    error.code
                );
                let mut c = connect(&path, true).await.unwrap();
                assert_eq!(operation_manifest(&mut c).await.unwrap(), before);
            }
        }
    }

    #[tokio::test]
    async fn migrated_legacy_context_recovery_answer_appends_user_successor_and_stays_historically_ineligible(
    ) {
        let id = "legacy-recovery-answer";
        let (_directory, path, raw) = exact_v5_legacy_recovery_fixture(id, false).await;
        let predecessor = artifact_revision(&path, id).await;
        let predecessor_digest: String = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_scalar("SELECT content_digest FROM artifact_revisions WHERE id=?")
                .bind(&predecessor)
                .fetch_one(&mut connection)
                .await
                .unwrap()
        };
        let source = source_revision(&path).await;
        let outcome = execute_disposable(
            &path,
            ContextRecoveryWriteCommand::SaveFirstResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source.clone(),
                expected_artifact_revision_id: predecessor.clone(),
                response: "An explicit task-scoped response.".into(),
            },
            context(
                "2026-08-09T03:00:00.000Z",
                "guard-legacy-context-recovery-answer-01",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let predecessor_after: (String, String, String) = sqlx::query_as(
            "SELECT c.payload, r.content_digest, r.serialization_version \
             FROM artifact_revisions r JOIN artifact_revision_content c ON c.revision_id=r.id \
             WHERE r.id=?",
        )
        .bind(&predecessor)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            predecessor_after,
            (raw, predecessor_digest, "legacy-v4-raw".into())
        );
        let successor: (i64, Option<String>, String, String, String, String) = sqlx::query_as(
            "SELECT r.revision_number, r.predecessor_revision_id, r.authorship, \
                    r.serialization_version, h.review_state, h.eligibility_reason \
             FROM artifact_revisions r JOIN artifact_heads h ON h.current_revision_id=r.id \
             WHERE r.id=? AND h.id=?",
        )
        .bind(&outcome.revision_id)
        .bind(id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            successor,
            (
                2,
                Some(predecessor.clone()),
                "mixed".into(),
                "canonical-json-v1".into(),
                "not_applicable".into(),
                "context_recovery_answered_current_experience_task_only".into(),
            )
        );
        let edges: Vec<RecoveryDependencyRow> = sqlx::query_as(
            "SELECT relationship_type, source_revision_id, source_artifact_id, \
                        source_artifact_revision_id FROM artifact_dependencies \
                 WHERE dependent_artifact_id=? AND dependent_revision_id=? \
                 ORDER BY relationship_type",
        )
        .bind(id)
        .bind(&outcome.revision_id)
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert!(edges.iter().any(|edge| {
            edge.0 == "derived_from_experience" && edge.1.as_deref() == Some(source.as_str())
        }));
        assert!(edges.iter().any(|edge| {
            edge.0 == "answers_prompt"
                && edge.2.as_deref() == Some(id)
                && edge.3.as_deref() == Some(predecessor.as_str())
        }));
        assert!(!edges.iter().any(|edge| edge.0 == "historical_packet_item"));
        let origins: Vec<String> = sqlx::query_scalar(
            "SELECT json_extract(p.canonical_payload, '$.origin') \
             FROM artifact_revision_provenance rp JOIN provenance_records p ON p.id=rp.provenance_id \
             WHERE rp.artifact_revision_id=? ORDER BY rp.role",
        )
        .bind(&outcome.revision_id)
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert_eq!(origins, vec!["local_mock".to_string(), "user".to_string()]);
    }

    #[tokio::test]
    async fn migrated_legacy_context_recovery_skip_is_terminal_without_response_revision() {
        let id = "legacy-recovery-skip";
        let (_directory, path, raw) = exact_v5_legacy_recovery_fixture(id, false).await;
        let predecessor = artifact_revision(&path, id).await;
        let outcome = execute_disposable(
            &path,
            ContextRecoveryWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: predecessor.clone(),
            },
            context(
                "2026-08-09T03:01:00.000Z",
                "guard-legacy-context-recovery-skip-001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(outcome.revision_id, predecessor);
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(
            sqlx::query_scalar::<_, String>(
                "SELECT payload FROM artifact_revision_content WHERE revision_id=?",
            )
            .bind(&outcome.revision_id)
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            raw
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id=?",
            )
            .bind(id)
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            1
        );
        let projection: Value = serde_json::from_str(
            &sqlx::query_scalar::<_, String>("SELECT payload FROM persisted_artifacts WHERE id=?")
                .bind(id)
                .fetch_one(&mut connection)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(projection["status"], "skipped");
        assert!(projection.get("response").is_none());
    }

    #[tokio::test]
    async fn migrated_legacy_context_recovery_unknown_shape_and_stale_source_fail_closed() {
        let id = "legacy-recovery-malformed";
        let (_directory, path, _) = exact_v5_legacy_recovery_fixture(id, true).await;
        let revision = artifact_revision(&path, id).await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_disposable(
            &path,
            ContextRecoveryWriteCommand::SaveFirstResponse {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: source_revision(&path).await,
                expected_artifact_revision_id: revision,
                response: "Must not persist.".into(),
            },
            context(
                "2026-08-09T03:02:00.000Z",
                "guard-legacy-context-recovery-malformed-01",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert!(error
            .code
            .contains("context_recovery_legacy_payload_unknown_field"));
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        drop(connection);

        let id = "legacy-recovery-stale";
        let (_directory, path, _) = exact_v5_legacy_recovery_fixture(id, false).await;
        let revision = artifact_revision(&path, id).await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        assert!(execute_disposable(
            &path,
            ContextRecoveryWriteCommand::SkipSuggested {
                source_id: SOURCE_ID.into(),
                artifact_id: id.into(),
                expected_source_revision_id: "stale-source-revision".into(),
                expected_artifact_revision_id: revision,
            },
            context(
                "2026-08-09T03:03:00.000Z",
                "guard-legacy-context-recovery-stale-001",
                ContextRecoveryWriteFailurePoint::None,
            ),
        )
        .await
        .is_err());
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
    }

    #[derive(Clone, Copy)]
    enum InjectedCommit {
        DefinitelyNotCommitted,
        UnknownWithoutCommit,
        UnknownAfterCommit,
        UnknownAfterCommitWithThirdState,
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
                    raw_sql("COMMIT").execute(&mut *connection).await.unwrap();
                    CommitAttemptOutcome::OutcomeUnknown {
                        error_class: "injected_unknown_after_commit".into(),
                    }
                }
                InjectedCommit::UnknownAfterCommitWithThirdState => {
                    raw_sql("COMMIT").execute(&mut *connection).await.unwrap();
                    raw_sql("PRAGMA user_version = 6")
                        .execute(&mut *connection)
                        .await
                        .unwrap();
                    CommitAttemptOutcome::OutcomeUnknown {
                        error_class: "injected_unknown_third_state".into(),
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
    async fn commit_outcomes_use_read_only_exact_manifests_without_retry() {
        for (mode, succeeds, rollbacks) in [
            (InjectedCommit::DefinitelyNotCommitted, false, 1),
            (InjectedCommit::UnknownWithoutCommit, false, 0),
            (InjectedCommit::UnknownAfterCommit, true, 0),
            (InjectedCommit::UnknownAfterCommitWithThirdState, false, 0),
        ] {
            let (_directory, path) = exact_v5_fixture().await;
            let source = source_revision(&path).await;
            let adapter = InjectedCommitAdapter {
                mode,
                commits: Cell::new(0),
                rollbacks: Cell::new(0),
            };
            let result = execute_with_adapter(
                &path,
                ContextRecoveryWriteCommand::CreateSuggested {
                    expected_source_revision_id: source,
                    prompt: prompt("recovery-commit", "ai"),
                },
                context(
                    "2026-08-02T02:06:00.000Z",
                    "guard-context-recovery-commit-000001",
                    ContextRecoveryWriteFailurePoint::None,
                ),
                &adapter,
            )
            .await;
            assert_eq!(result.is_ok(), succeeds);
            if matches!(mode, InjectedCommit::UnknownAfterCommitWithThirdState) {
                let error = result.unwrap_err();
                assert!(error.recovery_required);
                assert!(error
                    .code
                    .contains("context_recovery_commit_outcome_unknown"));
            }
            assert_eq!(adapter.commits.get(), 1);
            assert_eq!(adapter.rollbacks.get(), rollbacks);
            let mut c = connect(&path, true).await.unwrap();
            if matches!(mode, InjectedCommit::UnknownAfterCommitWithThirdState) {
                assert_eq!(
                    sqlx::query_scalar::<_, i64>("PRAGMA user_version")
                        .fetch_one(&mut c)
                        .await
                        .unwrap(),
                    6
                );
            } else {
                verify_exact_context_recovery_v5(&mut c).await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn migrated_legacy_recovery_answer_commit_classifies_pre_post_and_third_state() {
        for (mode, expected) in [
            (InjectedCommit::UnknownWithoutCommit, "pre"),
            (InjectedCommit::UnknownAfterCommit, "post"),
            (InjectedCommit::UnknownAfterCommitWithThirdState, "third"),
        ] {
            let id = format!("legacy-recovery-commit-{expected}");
            let (_directory, path, _) = exact_v5_legacy_recovery_fixture(&id, false).await;
            let adapter = InjectedCommitAdapter {
                mode,
                commits: Cell::new(0),
                rollbacks: Cell::new(0),
            };
            let result = execute_with_adapter(
                &path,
                ContextRecoveryWriteCommand::SaveFirstResponse {
                    source_id: SOURCE_ID.into(),
                    artifact_id: id.clone(),
                    expected_source_revision_id: source_revision(&path).await,
                    expected_artifact_revision_id: artifact_revision(&path, &id).await,
                    response: "Explicit answer under commit ambiguity.".into(),
                },
                context(
                    "2026-08-09T03:04:00.000Z",
                    "guard-legacy-context-recovery-commit-01",
                    ContextRecoveryWriteFailurePoint::None,
                ),
                &adapter,
            )
            .await;
            match expected {
                "pre" => assert!(result
                    .unwrap_err()
                    .code
                    .contains("context_recovery_commit_outcome_unknown_unchanged")),
                "post" => {
                    result.unwrap();
                }
                "third" => {
                    let error = result.unwrap_err();
                    assert!(error.recovery_required);
                    assert!(error
                        .code
                        .contains("context_recovery_commit_outcome_unknown"));
                }
                _ => unreachable!(),
            }
            assert_eq!(adapter.commits.get(), 1);
        }
    }

    #[tokio::test]
    async fn malformed_durable_source_arrays_and_historical_dependencies_fail_verification() {
        for condition in ["duplicate_prompt_sources", "historical_dependency"] {
            if condition == "duplicate_prompt_sources" {
                let duplicate = json!(["durable-id", "durable-id"]);
                assert_eq!(
                    durable_id_set(&duplicate, "prompt_sources")
                        .unwrap_err()
                        .code,
                    "context_recovery_prompt_sources_duplicate"
                );
                let malformed = json!(["valid-id", 42]);
                assert_eq!(
                    durable_id_set(&malformed, "prompt_sources")
                        .unwrap_err()
                        .code,
                    "context_recovery_prompt_sources_id_not_string"
                );
                continue;
            }
            let (_directory, path) = exact_v5_fixture().await;
            create(&path, "recovery-durable-invalid", "ai").await;
            let mut connection = connect(&path, false).await.unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)",
            )
            .bind("guard-context-recovery-corrupt-fixture-01")
            .bind("2026-08-02T02:07:00.000Z")
            .execute(&mut connection)
            .await
            .unwrap();
            let dependency_id = "v5hd_recovery_forbidden_fixture";
            sqlx::query(
                "INSERT INTO historical_artifact_dependencies (historical_artifact_id, source_entry_id, source_artifact_id, source_revision) \
                 VALUES ('fixture-v4-question', ?, ?, ?)",
            ).bind(SOURCE_ID).bind("recovery-durable-invalid").bind(dependency_id)
              .execute(&mut connection).await.unwrap();
            sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
                .bind("guard-context-recovery-corrupt-fixture-01")
                .execute(&mut connection)
                .await
                .unwrap();
            drop(connection);
            let mut c = connect(&path, true).await.unwrap();
            assert!(
                verify_exact_context_recovery_v5(&mut c).await.is_err(),
                "{condition}"
            );
        }
    }

    #[tokio::test]
    async fn deterministic_ids_and_manifests_repeat_across_disposable_fixtures() {
        let (_first_directory, first) = exact_v5_fixture().await;
        let (_second_directory, second) = exact_v5_fixture().await;
        let first = create(&first, "recovery-deterministic", "local_mock")
            .await
            .0;
        let second = create(&second, "recovery-deterministic", "local_mock")
            .await
            .0;
        assert_eq!(first.revision_id, second.revision_id);
        assert_eq!(first.operation_manifest, second.operation_manifest);
    }
}
