use super::experience_write::operation_manifest;
use super::reflection_write::verify_exact_reflection_v5;
use super::*;
use serde::{Deserialize, Serialize};
use sqlx::raw_sql;
use std::collections::{BTreeMap, BTreeSet};

const HISTORICAL_TASK: &str = "historical_reflection_questions";
const HISTORICAL_PURPOSE: &str = "invite_user_comparison_without_cross_time_conclusions";
const HISTORICAL_PACKET_SCHEMA: &str = "historical-packet-v1";
const MAX_HISTORICAL_SOURCES: usize = 3;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CurrentExperience {
    id: String,
    revision: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Destination {
    provider: String,
    model: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PacketVersions {
    harness: String,
    prompt: String,
    output_schema: String,
    safety_contract: String,
}

#[derive(Clone, Debug, Deserialize)]
struct PacketConsent {
    reference: String,
    scope: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct IncludedItem {
    item_type: String,
    source_experience_id: String,
    artifact_id: Option<String>,
    revision: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoricalPacket {
    packet_id: String,
    packet_digest: String,
    schema_version: String,
    current_experience: CurrentExperience,
    task: String,
    purpose: String,
    destination: Destination,
    versions: PacketVersions,
    included_items: Vec<IncludedItem>,
    consent: PacketConsent,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConsentSourceRevision {
    source_experience_id: String,
    revision: String,
    artifact_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConsentPayload {
    id: String,
    packet_digest: String,
    task: String,
    purpose: String,
    provider: String,
    model: String,
    source_revisions: Vec<ConsentSourceRevision>,
    state: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct HistoricalQuestion {
    id: String,
    text: String,
    source_experience_ids: Vec<String>,
}

#[derive(Clone, Debug)]
struct HistoricalQuestionWriteRequest {
    artifact_id: String,
    current_experience_id: String,
    questions: Vec<HistoricalQuestion>,
    packet_snapshot: String,
    consent_id: String,
    transmission_id: String,
    generated_at: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum HistoricalWriteFailurePoint {
    #[default]
    None,
    AfterGuard,
    AfterV4Artifact,
    AfterV4Dependency,
    AfterProvenance,
    AfterRevision,
    AfterContent,
    AfterHead,
    AfterProvenanceLink,
    AfterV5Dependency,
    AfterLifecycle,
    AfterLink,
    AfterReconciliation,
    AfterGuardRemoval,
}

#[derive(Clone, Debug)]
struct HistoricalWriteContext<'a> {
    guard_token: &'a str,
    failure_point: HistoricalWriteFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum HistoricalWriteStatus {
    Committed,
    AlreadyCommitted,
    NoQuestion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HistoricalWriteOutcome {
    status: HistoricalWriteStatus,
    artifact_id: Option<String>,
    revision_id: Option<String>,
    operation_manifest: String,
}

#[derive(Clone, Debug)]
struct PreparedQuestion {
    packet: HistoricalPacket,
    packet_value: Value,
    payload: String,
    content_digest: String,
    revision_id: String,
    provenance: Value,
    included: Vec<IncludedItem>,
}

fn write_error(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn recovery_error(code: impl Into<String>) -> MigrationError {
    MigrationError::recovery_required(code)
}

fn inject(
    context: &HistoricalWriteContext<'_>,
    expected: HistoricalWriteFailurePoint,
) -> Result<(), MigrationError> {
    if context.failure_point == expected {
        Err(write_error(format!(
            "injected_historical_question_write_failure:{expected:?}"
        )))
    } else {
        Ok(())
    }
}

fn validate_identifier(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        Err(write_error(format!("historical_question_{field}_invalid")))
    } else {
        Ok(())
    }
}

fn validate_timestamp(value: &str, field: &str) -> Result<(), MigrationError> {
    validate_identifier(value, field)?;
    if !value.contains('T') || !value.ends_with('Z') {
        return Err(write_error(format!("historical_question_{field}_invalid")));
    }
    Ok(())
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn unique_strings(values: &[String], field: &str) -> Result<BTreeSet<String>, MigrationError> {
    let mut result = BTreeSet::new();
    for value in values {
        validate_identifier(value, field)?;
        if !result.insert(value.clone()) {
            return Err(write_error(format!("historical_question_{field}_duplicate")));
        }
    }
    Ok(result)
}

fn packet_digest(packet_value: &Value) -> Result<String, MigrationError> {
    let mut without_digest = packet_value
        .as_object()
        .cloned()
        .ok_or_else(|| write_error("historical_packet_malformed"))?;
    without_digest
        .remove("packetDigest")
        .ok_or_else(|| write_error("historical_packet_digest_missing"))?;
    Ok(sha256_hex(
        canonical_json(&Value::Object(without_digest))?.as_bytes(),
    ))
}

fn normalize_items(items: &[IncludedItem]) -> Result<Vec<IncludedItem>, MigrationError> {
    if items.is_empty() {
        return Err(write_error("historical_packet_items_required"));
    }
    let mut identities = BTreeSet::new();
    let mut sources = BTreeSet::new();
    let mut experience_sources = BTreeSet::new();
    let mut result = Vec::new();
    for item in items {
        validate_identifier(&item.source_experience_id, "source_experience_id")?;
        validate_timestamp(&item.revision, "source_revision")?;
        sources.insert(item.source_experience_id.clone());
        let identity = (
            item.source_experience_id.clone(),
            item.artifact_id.clone(),
        );
        if !identities.insert(identity) {
            return Err(write_error("historical_packet_dependency_duplicate"));
        }
        match item.item_type.as_str() {
            "experience" => {
                if item.artifact_id.is_some()
                    || !experience_sources.insert(item.source_experience_id.clone())
                {
                    return Err(write_error("historical_packet_experience_invalid"));
                }
            }
            "evidence" | "reflection_response" => {
                validate_identifier(
                    item.artifact_id
                        .as_deref()
                        .ok_or_else(|| write_error("historical_packet_artifact_id_missing"))?,
                    "artifact_id",
                )?;
            }
            _ => return Err(write_error("historical_packet_item_type_invalid")),
        }
        result.push(item.clone());
    }
    if sources.len() > MAX_HISTORICAL_SOURCES || sources != experience_sources {
        return Err(write_error("historical_packet_source_boundary_invalid"));
    }
    result.sort();
    Ok(result)
}

fn validate_questions(
    questions: &[HistoricalQuestion],
    current_experience_id: &str,
    historical_sources: &BTreeSet<String>,
) -> Result<(), MigrationError> {
    if questions.is_empty() || questions.len() > 3 {
        return Err(write_error("historical_question_count_invalid"));
    }
    let mut question_ids = BTreeSet::new();
    for question in questions {
        validate_identifier(&question.id, "question_id")?;
        if !question_ids.insert(question.id.clone())
            || question.text.trim().is_empty()
            || question.text.len() > 4_000
        {
            return Err(write_error("historical_question_output_invalid"));
        }
        let citations = unique_strings(&question.source_experience_ids, "citation")?;
        if citations.is_empty()
            || citations
                .iter()
                .any(|id| id != current_experience_id && !historical_sources.contains(id))
            || !citations.iter().any(|id| historical_sources.contains(id))
        {
            return Err(write_error("historical_question_sources_invalid"));
        }
    }
    Ok(())
}

fn prepare_request(
    request: &HistoricalQuestionWriteRequest,
) -> Result<Option<PreparedQuestion>, MigrationError> {
    validate_identifier(&request.artifact_id, "artifact_id")?;
    validate_identifier(&request.current_experience_id, "current_experience_id")?;
    validate_identifier(&request.consent_id, "consent_id")?;
    validate_identifier(&request.transmission_id, "transmission_id")?;
    validate_timestamp(&request.generated_at, "generated_at")?;
    let packet_value: Value = serde_json::from_str(&request.packet_snapshot)
        .map_err(|error| migration_error("historical_packet_malformed", error))?;
    let packet: HistoricalPacket = serde_json::from_value(packet_value.clone())
        .map_err(|error| migration_error("historical_packet_malformed", error))?;
    if packet.schema_version != HISTORICAL_PACKET_SCHEMA
        || packet.task != HISTORICAL_TASK
        || packet.purpose != HISTORICAL_PURPOSE
        || packet.consent.scope != "one_generation_one_purpose"
        || packet.consent.reference != request.consent_id
        || packet.current_experience.id != request.current_experience_id
        || !matches!(packet.destination.provider.as_str(), "openai" | "gemini")
        || packet.destination.model.trim().is_empty()
        || packet.versions.output_schema != "historical-question-output-v1"
        || packet.versions.safety_contract != "phase-3b-safety-v1"
        || !valid_sha256(&packet.packet_digest)
        || packet_digest(&packet_value)? != packet.packet_digest
    {
        return Err(write_error("historical_packet_contract_mismatch"));
    }
    validate_identifier(&packet.packet_id, "packet_id")?;
    validate_timestamp(&packet.current_experience.revision, "current_revision")?;
    validate_identifier(&packet.versions.harness, "harness_version")?;
    validate_identifier(&packet.versions.prompt, "prompt_version")?;
    validate_identifier(&packet.versions.output_schema, "output_schema_version")?;
    validate_identifier(&packet.versions.safety_contract, "safety_contract_version")?;

    let included = normalize_items(&packet.included_items)?;
    if request.questions.is_empty() {
        return Ok(None);
    }
    let historical_sources = included
        .iter()
        .map(|item| item.source_experience_id.clone())
        .collect::<BTreeSet<_>>();
    validate_questions(
        &request.questions,
        &request.current_experience_id,
        &historical_sources,
    )?;

    let payload_value = json!({
        "id": request.artifact_id,
        "currentExperienceId": request.current_experience_id,
        "questions": request.questions,
        "consentId": request.consent_id,
        "transmissionId": request.transmission_id,
        "generatedAt": request.generated_at,
    });
    let payload = canonical_json(&payload_value)?;
    let content_digest = sha256_hex(payload.as_bytes());
    let revision_id = artifact_revision_id(
        &request.artifact_id,
        "historical_question",
        &request.generated_at,
        &content_digest,
    );
    let source_artifact_ids = included
        .iter()
        .filter_map(|item| item.artifact_id.clone())
        .collect::<Vec<_>>();
    let provenance = json!({
        "origin": "ai",
        "sourceEntryId": request.current_experience_id,
        "sourceArtifactIds": source_artifact_ids,
        "provider": packet.destination.provider,
        "model": packet.destination.model,
        "harnessVersion": packet.versions.harness,
        "promptVersion": packet.versions.prompt,
        "generatedAt": request.generated_at,
        "purpose": packet.purpose,
        "packetId": packet.packet_id,
        "packetDigest": packet.packet_digest,
        "consentId": request.consent_id,
        "transmissionId": request.transmission_id,
        "outputSchemaVersion": packet.versions.output_schema,
        "safetyContractVersion": packet.versions.safety_contract,
    });
    Ok(Some(PreparedQuestion {
        packet,
        packet_value,
        payload,
        content_digest,
        revision_id,
        provenance,
        included,
    }))
}

async fn validate_current_source(
    connection: &mut SqliteConnection,
    source_id: &str,
    v4_revision: &str,
) -> Result<String, MigrationError> {
    let state: Option<(String, String, String)> = sqlx::query_as(
        "SELECT e.updated_at, h.current_revision_id, h.lifecycle_state \
         FROM experience_entries e JOIN source_heads h ON h.id = e.id WHERE e.id = ?",
    )
    .bind(source_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_source_lookup_failed", error))?;
    match state {
        Some((updated_at, revision_id, lifecycle))
            if updated_at == v4_revision && lifecycle == "active" => Ok(revision_id),
        _ => Err(write_error("historical_source_revision_stale")),
    }
}

async fn validate_artifact_item(
    connection: &mut SqliteConnection,
    item: &IncludedItem,
) -> Result<String, MigrationError> {
    let artifact_id = item
        .artifact_id
        .as_deref()
        .ok_or_else(|| write_error("historical_artifact_id_missing"))?;
    let expected_kind = if item.item_type == "evidence" {
        "evidence"
    } else {
        "reflection"
    };
    let state: Option<(String, String, String, String, String, String)> = sqlx::query_as(
        "SELECT p.updated_at, h.source_id, h.artifact_kind, h.current_revision_id, \
                h.lifecycle_state, h.eligibility_state \
         FROM persisted_artifacts p JOIN artifact_heads h ON h.id = p.id \
         WHERE p.id = ? AND p.source_entry_id = ? AND p.artifact_kind = ?",
    )
    .bind(artifact_id)
    .bind(&item.source_experience_id)
    .bind(expected_kind)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_artifact_lookup_failed", error))?;
    let Some((updated_at, source_id, kind, revision_id, lifecycle, eligibility)) = state else {
        return Err(write_error("historical_artifact_missing"));
    };
    if updated_at != item.revision
        || source_id != item.source_experience_id
        || kind != expected_kind
        || lifecycle != "active"
        || eligibility != "eligible"
    {
        return Err(write_error("historical_artifact_revision_stale"));
    }
    let payload: String = sqlx::query_scalar("SELECT payload FROM persisted_artifacts WHERE id = ?")
        .bind(artifact_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_artifact_payload_failed", error))?;
    let payload: Value = serde_json::from_str(&payload)
        .map_err(|error| migration_error("historical_artifact_payload_malformed", error))?;
    let eligible = if expected_kind == "evidence" {
        payload.get("status").and_then(Value::as_str) == Some("confirmed")
    } else {
        payload.get("status").and_then(Value::as_str) == Some("answered")
            && payload
                .get("response")
                .and_then(Value::as_str)
                .is_some_and(|text| !text.trim().is_empty())
            && payload
                .get("responseProvenance")
                .and_then(|value| value.get("origin"))
                .and_then(Value::as_str)
                == Some("user")
    };
    if !eligible {
        return Err(write_error("historical_artifact_ineligible"));
    }
    if expected_kind == "reflection" {
        let source_evidence = payload
            .get("sourceEvidenceIds")
            .and_then(Value::as_array)
            .filter(|values| !values.is_empty())
            .ok_or_else(|| write_error("historical_reflection_evidence_required"))?;
        let mut expected = BTreeSet::new();
        for value in source_evidence {
            let evidence_id = value
                .as_str()
                .ok_or_else(|| write_error("historical_reflection_evidence_malformed"))?;
            validate_identifier(evidence_id, "reflection_evidence_id")?;
            if !expected.insert(evidence_id.to_owned()) {
                return Err(write_error("historical_reflection_evidence_duplicate"));
            }
        }
        let dependencies: Vec<(String, String)> = sqlx::query_as(
            "SELECT d.source_artifact_id, d.source_artifact_revision_id \
             FROM artifact_dependencies d WHERE d.dependent_artifact_id = ? \
               AND d.dependent_revision_id = ? AND d.relationship_type = 'uses_evidence' \
             ORDER BY d.source_artifact_id",
        )
        .bind(artifact_id)
        .bind(&revision_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_reflection_dependency_read_failed", error))?;
        let actual = dependencies
            .iter()
            .map(|value| value.0.clone())
            .collect::<BTreeSet<_>>();
        if actual.len() != dependencies.len() || actual != expected {
            return Err(write_error("historical_reflection_dependency_mismatch"));
        }
        for (evidence_id, evidence_revision_id) in dependencies {
            let evidence: Option<(String, String, String, String)> = sqlx::query_as(
                "SELECT source_id, current_revision_id, review_state, eligibility_state \
                 FROM artifact_heads WHERE id = ? AND artifact_kind = 'evidence' \
                   AND lifecycle_state = 'active'",
            )
            .bind(&evidence_id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(|error| migration_error("historical_reflection_evidence_read_failed", error))?;
            if evidence
                .as_ref()
                .map(|value| {
                    (
                        value.0.as_str(),
                        value.1.as_str(),
                        value.2.as_str(),
                        value.3.as_str(),
                    )
                })
                != Some((
                    item.source_experience_id.as_str(),
                    evidence_revision_id.as_str(),
                    "confirmed",
                    "eligible",
                ))
            {
                return Err(write_error("historical_reflection_evidence_ineligible"));
            }
        }
    }
    Ok(revision_id)
}

async fn validate_consent_and_transmission(
    connection: &mut SqliteConnection,
    request: &HistoricalQuestionWriteRequest,
    prepared: &PreparedQuestion,
) -> Result<(), MigrationError> {
    let consent: Option<(String, String, String)> = sqlx::query_as(
        "SELECT packet_digest, state, payload FROM historical_consent_events WHERE id = ?",
    )
    .bind(&request.consent_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_consent_lookup_failed", error))?;
    let Some((digest, state, payload)) = consent else {
        return Err(write_error("historical_consent_missing"));
    };
    let consent: ConsentPayload = serde_json::from_str(&payload)
        .map_err(|error| migration_error("historical_consent_malformed", error))?;
    if digest != prepared.packet.packet_digest
        || state != "consumed"
        || consent.id != request.consent_id
        || consent.packet_digest != prepared.packet.packet_digest
        || consent.task != HISTORICAL_TASK
        || consent.purpose != HISTORICAL_PURPOSE
        || consent.provider != prepared.packet.destination.provider
        || consent.model != prepared.packet.destination.model
        || consent.state != "consumed"
    {
        return Err(write_error("historical_consent_mismatch"));
    }

    let mut expected = BTreeMap::<String, (String, BTreeSet<String>)>::new();
    for item in &prepared.included {
        let entry = expected
            .entry(item.source_experience_id.clone())
            .or_insert_with(|| (String::new(), BTreeSet::new()));
        if item.item_type == "experience" {
            entry.0.clone_from(&item.revision);
        } else if let Some(id) = &item.artifact_id {
            entry.1.insert(id.clone());
        }
    }
    let mut actual = BTreeMap::new();
    for source in consent.source_revisions {
        validate_identifier(&source.source_experience_id, "consent_source_id")?;
        validate_timestamp(&source.revision, "consent_source_revision")?;
        let artifacts = unique_strings(&source.artifact_ids, "consent_artifact_id")?;
        if actual
            .insert(source.source_experience_id, (source.revision, artifacts))
            .is_some()
        {
            return Err(write_error("historical_consent_source_duplicate"));
        }
    }
    if actual != expected {
        return Err(write_error("historical_consent_sources_mismatch"));
    }

    let transmission: Option<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT consent_id, packet_digest, provider, model, outcome \
         FROM historical_transmission_events WHERE id = ?",
    )
    .bind(&request.transmission_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_transmission_lookup_failed", error))?;
    if transmission
        .as_ref()
        .map(|value| {
            (
                value.0.as_str(),
                value.1.as_str(),
                value.2.as_str(),
                value.3.as_str(),
                value.4.as_str(),
            )
        })
        != Some((
            request.consent_id.as_str(),
            prepared.packet.packet_digest.as_str(),
            prepared.packet.destination.provider.as_str(),
            prepared.packet.destination.model.as_str(),
            "sent",
        ))
    {
        return Err(write_error("historical_transmission_mismatch"));
    }
    Ok(())
}

async fn validate_all_sources(
    connection: &mut SqliteConnection,
    request: &HistoricalQuestionWriteRequest,
    prepared: &PreparedQuestion,
) -> Result<Vec<(IncludedItem, String)>, MigrationError> {
    validate_current_source(
        connection,
        &request.current_experience_id,
        &prepared.packet.current_experience.revision,
    )
    .await?;
    let mut exact = Vec::new();
    for item in &prepared.included {
        let revision_id = if item.item_type == "experience" {
            validate_current_source(connection, &item.source_experience_id, &item.revision).await?
        } else {
            validate_artifact_item(connection, item).await?
        };
        exact.push((item.clone(), revision_id));
    }
    validate_consent_and_transmission(connection, request, prepared).await?;
    Ok(exact)
}

async fn insert_guard(
    connection: &mut SqliteConnection,
    context: &HistoricalWriteContext<'_>,
) -> Result<(), MigrationError> {
    validate_identifier(context.guard_token, "guard_token")?;
    sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
        .bind(context.guard_token)
        .bind("2026-08-02T00:00:00.000Z")
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_guard_insert_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterGuard)
}

async fn remove_guard(
    connection: &mut SqliteConnection,
    context: &HistoricalWriteContext<'_>,
) -> Result<(), MigrationError> {
    let deleted = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
        .bind(context.guard_token)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_guard_delete_failed", error))?;
    if deleted.rows_affected() != 1 {
        return Err(recovery_error("historical_guard_identity_mismatch"));
    }
    inject(context, HistoricalWriteFailurePoint::AfterGuardRemoval)
}

async fn exact_existing(
    connection: &mut SqliteConnection,
    request: &HistoricalQuestionWriteRequest,
    prepared: &PreparedQuestion,
    exact_sources: &[(IncludedItem, String)],
) -> Result<bool, MigrationError> {
    let counts: (i64, i64, i64) = sqlx::query_as(
        "SELECT \
          (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?), \
          (SELECT COUNT(*) FROM artifact_heads WHERE id = ?), \
          (SELECT COUNT(*) FROM historical_question_lifecycle_links \
             WHERE historical_artifact_id = ? AND artifact_id = ?)",
    )
    .bind(&request.artifact_id)
    .bind(&request.artifact_id)
    .bind(&request.artifact_id)
    .bind(&request.artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_identity_lookup_failed", error))?;
    if counts == (0, 0, 0) {
        return Ok(false);
    }
    if counts != (1, 1, 1) {
        return Err(recovery_error("historical_partial_identity_conflict"));
    }
    let v4: (String, String, String, String, String, String, String) = sqlx::query_as(
        "SELECT current_experience_id, packet_digest, payload, packet_snapshot, \
                consent_id, transmission_id, created_at \
         FROM historical_question_artifacts WHERE id = ?",
    )
    .bind(&request.artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_existing_v4_read_failed", error))?;
    if v4
        != (
            request.current_experience_id.clone(),
            prepared.packet.packet_digest.clone(),
            prepared.payload.clone(),
            request.packet_snapshot.clone(),
            request.consent_id.clone(),
            request.transmission_id.clone(),
            request.generated_at.clone(),
        )
    {
        return Err(write_error("historical_duplicate_conflict"));
    }
    let v5: (String, String, String, String, String, String, String) = sqlx::query_as(
        "SELECT h.source_id, h.artifact_kind, h.current_revision_id, h.review_state, \
                h.lifecycle_state, h.eligibility_state, c.payload \
         FROM artifact_heads h JOIN artifact_revision_content c \
           ON c.revision_id = h.current_revision_id WHERE h.id = ?",
    )
    .bind(&request.artifact_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_existing_v5_read_failed", error))?;
    if v5
        != (
            request.current_experience_id.clone(),
            "historical_question".into(),
            prepared.revision_id.clone(),
            "not_applicable".into(),
            "active".into(),
            "ineligible".into(),
            prepared.payload.clone(),
        )
    {
        return Err(write_error("historical_duplicate_conflict"));
    }
    verify_exact_dependencies(connection, request, prepared, exact_sources).await?;
    let expected_provenance = canonical_json(&prepared.provenance)?;
    let actual_provenance: String = sqlx::query_scalar(
        "SELECT p.canonical_payload FROM provenance_records p \
         JOIN artifact_revision_provenance rp ON rp.provenance_id = p.id \
         WHERE rp.artifact_revision_id = ? AND rp.role = 'content'",
    )
    .bind(&prepared.revision_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_existing_provenance_failed", error))?;
    if actual_provenance != expected_provenance {
        return Err(write_error("historical_duplicate_conflict"));
    }
    Ok(true)
}

async fn verify_exact_dependencies(
    connection: &mut SqliteConnection,
    request: &HistoricalQuestionWriteRequest,
    prepared: &PreparedQuestion,
    exact_sources: &[(IncludedItem, String)],
) -> Result<(), MigrationError> {
    let mut expected_v4 = prepared
        .included
        .iter()
        .map(|item| {
            (
                item.source_experience_id.clone(),
                item.artifact_id.clone(),
                item.revision.clone(),
            )
        })
        .collect::<Vec<_>>();
    expected_v4.sort();
    let actual_v4: Vec<(String, Option<String>, String)> = sqlx::query_as(
        "SELECT source_entry_id, source_artifact_id, source_revision \
         FROM historical_artifact_dependencies WHERE historical_artifact_id = ? \
         ORDER BY source_entry_id, source_artifact_id",
    )
    .bind(&request.artifact_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v4_dependencies_read_failed", error))?;
    if actual_v4 != expected_v4 {
        return Err(recovery_error("historical_v4_dependencies_mismatch"));
    }
    let current_revision: String = sqlx::query_scalar(
        "SELECT current_revision_id FROM source_heads WHERE id = ?",
    )
    .bind(&request.current_experience_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_current_dependency_read_failed", error))?;
    let mut expected_v5 = vec![(
        "historical_current_experience".to_string(),
        request.current_experience_id.clone(),
        current_revision,
    )];
    expected_v5.extend(exact_sources.iter().map(|(item, revision_id)| {
        (
            "historical_packet_item".to_string(),
            item.artifact_id
                .clone()
                .unwrap_or_else(|| item.source_experience_id.clone()),
            revision_id.clone(),
        )
    }));
    expected_v5.sort();
    let actual_v5: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT relationship_type, \
                COALESCE(source_artifact_id, (SELECT source_id FROM source_revisions \
                  WHERE id = source_revision_id)), \
                COALESCE(source_artifact_revision_id, source_revision_id) \
         FROM artifact_dependencies WHERE dependent_artifact_id = ? \
           AND dependent_revision_id = ? \
         ORDER BY relationship_type, 2, 3",
    )
    .bind(&request.artifact_id)
    .bind(&prepared.revision_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v5_dependencies_read_failed", error))?;
    if actual_v5 != expected_v5 {
        return Err(recovery_error("historical_v5_dependencies_mismatch"));
    }
    Ok(())
}

async fn write_new(
    connection: &mut SqliteConnection,
    request: &HistoricalQuestionWriteRequest,
    prepared: &PreparedQuestion,
    exact_sources: &[(IncludedItem, String)],
    context: &HistoricalWriteContext<'_>,
) -> Result<(), MigrationError> {
    sqlx::query(
        "INSERT INTO historical_question_artifacts (id, current_experience_id, \
         packet_digest, payload, packet_snapshot, consent_id, transmission_id, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&request.artifact_id)
    .bind(&request.current_experience_id)
    .bind(&prepared.packet.packet_digest)
    .bind(&prepared.payload)
    .bind(&request.packet_snapshot)
    .bind(&request.consent_id)
    .bind(&request.transmission_id)
    .bind(&request.generated_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v4_artifact_insert_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterV4Artifact)?;
    for item in &prepared.included {
        sqlx::query(
            "INSERT INTO historical_artifact_dependencies \
             (historical_artifact_id, source_entry_id, source_artifact_id, source_revision) \
             VALUES (?, ?, ?, ?)",
        )
        .bind(&request.artifact_id)
        .bind(&item.source_experience_id)
        .bind(&item.artifact_id)
        .bind(&item.revision)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("historical_v4_dependency_insert_failed", error))?;
    }
    inject(context, HistoricalWriteFailurePoint::AfterV4Dependency)?;

    let provenance_id = insert_provenance(connection, &prepared.provenance, &request.generated_at)
        .await?;
    inject(context, HistoricalWriteFailurePoint::AfterProvenance)?;
    sqlx::query(
        "INSERT INTO artifact_revisions (id, artifact_id, source_id, revision_number, \
         predecessor_revision_id, authorship, revision_reason, serialization_version, \
         content_digest, created_at) \
         VALUES (?, ?, ?, 1, NULL, 'ai', 'created', 'canonical-json-v1', ?, ?)",
    )
    .bind(&prepared.revision_id)
    .bind(&request.artifact_id)
    .bind(&request.current_experience_id)
    .bind(&prepared.content_digest)
    .bind(&request.generated_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v5_revision_insert_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterRevision)?;
    sqlx::query(
        "INSERT INTO artifact_revision_content (revision_id, payload, byte_length) VALUES (?, ?, ?)",
    )
    .bind(&prepared.revision_id)
    .bind(&prepared.payload)
    .bind(prepared.payload.len() as i64)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v5_content_insert_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterContent)?;
    sqlx::query(
        "INSERT INTO artifact_heads (id, source_id, artifact_kind, current_revision_id, \
         review_state, lifecycle_state, eligibility_state, eligibility_reason, \
         created_at, updated_at) VALUES (?, ?, 'historical_question', ?, \
         'not_applicable', 'active', 'ineligible', \
         'historical_question_not_source_eligible', ?, ?)",
    )
    .bind(&request.artifact_id)
    .bind(&request.current_experience_id)
    .bind(&prepared.revision_id)
    .bind(&request.generated_at)
    .bind(&request.generated_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v5_head_insert_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterHead)?;
    sqlx::query(
        "INSERT INTO artifact_revision_provenance (artifact_revision_id, role, provenance_id) \
         VALUES (?, 'content', ?)",
    )
    .bind(&prepared.revision_id)
    .bind(provenance_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v5_provenance_link_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterProvenanceLink)?;

    insert_dependency_for_source(
        connection,
        &request.artifact_id,
        &prepared.revision_id,
        "historical_current_experience",
        &request.current_experience_id,
        &request.generated_at,
    )
    .await?;
    for (item, _) in exact_sources {
        if let Some(artifact_id) = &item.artifact_id {
            insert_dependency_for_artifact(
                connection,
                &request.artifact_id,
                &prepared.revision_id,
                "historical_packet_item",
                artifact_id,
                &request.generated_at,
            )
            .await?;
        } else {
            insert_dependency_for_source(
                connection,
                &request.artifact_id,
                &prepared.revision_id,
                "historical_packet_item",
                &item.source_experience_id,
                &request.generated_at,
            )
            .await?;
        }
    }
    inject(context, HistoricalWriteFailurePoint::AfterV5Dependency)?;
    let lifecycle_id = event_id(
        "v5le_",
        "life-os/artifact-lifecycle-event-id-v1",
        &request.artifact_id,
        &prepared.revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_lifecycle_events (id, artifact_id, subject_revision_id, \
         related_revision_id, dependency_id, event_type, actor, reason_code, occurred_at) \
         VALUES (?, ?, ?, NULL, NULL, 'created', 'system', \
         'authorized_historical_question_generated', ?)",
    )
    .bind(lifecycle_id)
    .bind(&request.artifact_id)
    .bind(&prepared.revision_id)
    .bind(&request.generated_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v5_lifecycle_insert_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterLifecycle)?;
    sqlx::query(
        "INSERT INTO historical_question_lifecycle_links \
         (historical_artifact_id, artifact_id) VALUES (?, ?)",
    )
    .bind(&request.artifact_id)
    .bind(&request.artifact_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_v5_link_insert_failed", error))?;
    inject(context, HistoricalWriteFailurePoint::AfterLink)?;
    Ok(())
}

async fn reconcile(
    connection: &mut SqliteConnection,
    request: &HistoricalQuestionWriteRequest,
    prepared: &PreparedQuestion,
    exact_sources: &[(IncludedItem, String)],
) -> Result<(), MigrationError> {
    if !exact_existing(connection, request, prepared, exact_sources).await? {
        return Err(recovery_error("historical_reconciliation_missing"));
    }
    let parity: (i64, i64, i64) = sqlx::query_as(
        "SELECT \
          (SELECT COUNT(*) FROM historical_question_artifacts), \
          (SELECT COUNT(*) FROM artifact_heads WHERE artifact_kind = 'historical_question'), \
          (SELECT COUNT(*) FROM historical_question_lifecycle_links)",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("historical_parity_count_failed", error))?;
    if parity.0 != parity.1 || parity.0 != parity.2 {
        return Err(recovery_error("historical_projection_parity_mismatch"));
    }
    if prepared.packet_value["packetDigest"].as_str()
        != Some(prepared.packet.packet_digest.as_str())
    {
        return Err(recovery_error("historical_packet_identity_mismatch"));
    }
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    Ok(())
}

async fn verify_read_only(
    path: &Path,
    expected_manifest: &str,
) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await?;
    verify_exact_reflection_v5(&mut connection).await?;
    let actual = operation_manifest(&mut connection).await?;
    if actual != expected_manifest {
        return Err(recovery_error("historical_operation_manifest_mismatch"));
    }
    Ok(())
}

async fn execute_with_adapter<A: CommitOutcomeAdapter>(
    path: &Path,
    request: HistoricalQuestionWriteRequest,
    context: HistoricalWriteContext<'_>,
    adapter: &A,
) -> Result<HistoricalWriteOutcome, MigrationError> {
    let Some(prepared) = prepare_request(&request)? else {
        let mut connection = connect(path, true).await?;
        verify_exact_reflection_v5(&mut connection).await?;
        return Ok(HistoricalWriteOutcome {
            status: HistoricalWriteStatus::NoQuestion,
            artifact_id: None,
            revision_id: None,
            operation_manifest: operation_manifest(&mut connection).await?,
        });
    };
    let mut connection = connect(path, false).await?;
    verify_exact_reflection_v5(&mut connection).await?;
    let pre_manifest = operation_manifest(&mut connection).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("historical_begin_failed", error))?;
    let prepared_result = async {
        insert_guard(&mut connection, &context).await?;
        let exact_sources = validate_all_sources(&mut connection, &request, &prepared).await?;
        let already = exact_existing(&mut connection, &request, &prepared, &exact_sources).await?;
        if !already {
            write_new(
                &mut connection,
                &request,
                &prepared,
                &exact_sources,
                &context,
            )
            .await?;
        }
        reconcile(&mut connection, &request, &prepared, &exact_sources).await?;
        inject(&context, HistoricalWriteFailurePoint::AfterReconciliation)?;
        remove_guard(&mut connection, &context).await?;
        verify_exact_reflection_v5(&mut connection).await?;
        let post_manifest = operation_manifest(&mut connection).await?;
        Ok::<_, MigrationError>((already, post_manifest))
    }
    .await;

    let (already, post_manifest) = match prepared_result {
        Ok(value) => value,
        Err(error) => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest).await.map_err(|verify| {
                recovery_error(format!(
                    "historical_rollback_verification_failed:{}:{}",
                    error.code, verify.code
                ))
            })?;
            return match rollback {
                RollbackAttemptOutcome::RolledBack => Err(error),
                RollbackAttemptOutcome::Failed { error_class } => Err(recovery_error(format!(
                    "historical_rollback_outcome_unknown:{}:{error_class}",
                    error.code
                ))),
            };
        }
    };

    match adapter.commit(&mut connection).await {
        CommitAttemptOutcome::Committed => {
            drop(connection);
            verify_read_only(path, &post_manifest).await?;
            Ok(HistoricalWriteOutcome {
                status: if already {
                    HistoricalWriteStatus::AlreadyCommitted
                } else {
                    HistoricalWriteStatus::Committed
                },
                artifact_id: Some(request.artifact_id),
                revision_id: Some(prepared.revision_id),
                operation_manifest: post_manifest,
            })
        }
        CommitAttemptOutcome::DefinitelyNotCommitted { error_class } => {
            drop(connection);
            verify_read_only(path, &pre_manifest).await?;
            Err(write_error(format!(
                "historical_commit_definitely_not_committed:{error_class}"
            )))
        }
        CommitAttemptOutcome::OutcomeUnknown { error_class } => {
            drop(connection);
            let mut read_only = connect(path, true).await?;
            verify_exact_reflection_v5(&mut read_only).await?;
            let durable = operation_manifest(&mut read_only).await?;
            if durable == post_manifest {
                Ok(HistoricalWriteOutcome {
                    status: if already {
                        HistoricalWriteStatus::AlreadyCommitted
                    } else {
                        HistoricalWriteStatus::Committed
                    },
                    artifact_id: Some(request.artifact_id),
                    revision_id: Some(prepared.revision_id),
                    operation_manifest: post_manifest,
                })
            } else if durable == pre_manifest {
                Err(write_error(format!(
                    "historical_commit_definitely_not_committed:{error_class}"
                )))
            } else {
                Err(recovery_error(format!(
                    "historical_commit_outcome_unknown:{error_class}"
                )))
            }
        }
    }
}

async fn write_disposable_historical_question(
    path: &Path,
    request: HistoricalQuestionWriteRequest,
    context: HistoricalWriteContext<'_>,
) -> Result<HistoricalWriteOutcome, MigrationError> {
    execute_with_adapter(path, request, context, &SqlCommitOutcomeAdapter).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::Connection;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const STARTED_AT: &str = "2026-08-02T00:00:00.000Z";
    const COMMITTED_AT: &str = "2026-08-02T00:00:01.000Z";
    const GENERATED_AT: &str = "2026-08-02T00:00:05.000Z";
    const CONSENT_ID: &str = "slice4c2-consent";
    const TRANSMISSION_ID: &str = "slice4c2-transmission";
    const ARTIFACT_ID: &str = "slice4c2-question";

    async fn create_v5_fixture() -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        sqlx::query(
            "INSERT INTO persisted_artifacts \
             (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
             VALUES ('fixture-reflection-answered', 'fixture-v4-history', 'reflection', ?, \
             '2026-01-01T00:00:03.000Z', '2026-01-01T00:00:04.000Z')",
        )
        .bind(r#"{"id":"fixture-reflection-answered","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"question":"What stayed?","status":"answered","response":"My words","promptProvenance":{"origin":"ai","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence"],"provider":"gemini","model":"fixture-model","harnessVersion":"harness-v1","promptVersion":"historical-reflection-question-v1","generatedAt":"2026-01-01T00:00:03.000Z"},"responseProvenance":{"origin":"user","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-reflection-answered"],"generatedAt":"2026-01-01T00:00:04.000Z"},"createdAt":"2026-01-01T00:00:03.000Z","updatedAt":"2026-01-01T00:00:04.000Z"}"#)
        .execute(&mut connection)
        .await
        .unwrap();
        let source_manifest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        drop(connection);
        migrate_disposable_v4(MigrationRequest {
            path: &path,
            expected_source_manifest_digest: source_manifest,
            started_at: STARTED_AT,
            committed_at: COMMITTED_AT,
            backup_id: Some("slice4c2-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        seed_successful_audit(&path).await;
        (directory, path)
    }

    fn packet_without_digest() -> Value {
        json!({
            "packetId": "slice4c2-packet",
            "schemaVersion": HISTORICAL_PACKET_SCHEMA,
            "assembledAt": "2026-08-02T00:00:02.000Z",
            "expiresAt": "2026-08-02T00:10:02.000Z",
            "currentExperience": {
                "id": "fixture-v4-current",
                "revision": "2026-01-03T00:00:00.000Z",
                "content": "current experience"
            },
            "task": HISTORICAL_TASK,
            "purpose": HISTORICAL_PURPOSE,
            "locale": "en",
            "responseLanguage": "en",
            "destination": {
                "provider": "gemini",
                "model": "fixture-model",
                "retentionDisclosure": "fixture disclosure"
            },
            "versions": {
                "harness": "harness-v1",
                "prompt": "historical-reflection-question-v1",
                "outputSchema": "historical-question-output-v1",
                "safetyContract": "phase-3b-safety-v1"
            },
            "includedItems": [
                {
                    "itemType": "experience",
                    "sourceExperienceId": "fixture-v4-history",
                    "artifactId": Value::Null,
                    "revision": "2026-01-01T00:00:00.000Z",
                    "authorship": "user",
                    "reviewState": "persisted",
                    "content": "historical experience",
                    "relevanceReason": "fixture"
                },
                {
                    "itemType": "evidence",
                    "sourceExperienceId": "fixture-v4-history",
                    "artifactId": "fixture-v4-evidence",
                    "revision": "2026-01-01T00:00:01.000Z",
                    "authorship": "user_confirmed_ai_candidate",
                    "reviewState": "confirmed",
                    "content": "observed",
                    "relevanceReason": "fixture"
                },
                {
                    "itemType": "reflection_response",
                    "sourceExperienceId": "fixture-v4-history",
                    "artifactId": "fixture-reflection-answered",
                    "revision": "2026-01-01T00:00:04.000Z",
                    "authorship": "user",
                    "reviewState": "answered",
                    "content": "My words",
                    "relevanceReason": "fixture"
                }
            ],
            "consent": {"reference": CONSENT_ID, "scope": "one_generation_one_purpose"},
            "limits": {"maxSources": 3, "maxContentCharacters": 6000}
        })
    }

    fn packet_snapshot() -> (String, String) {
        let mut packet = packet_without_digest();
        let digest = sha256_hex(canonical_json(&packet).unwrap().as_bytes());
        packet
            .as_object_mut()
            .unwrap()
            .insert("packetDigest".into(), json!(digest));
        (canonical_json(&packet).unwrap(), digest)
    }

    fn consent_payload(digest: &str) -> String {
        canonical_json(&json!({
            "id": CONSENT_ID,
            "packetDigest": digest,
            "task": HISTORICAL_TASK,
            "purpose": HISTORICAL_PURPOSE,
            "provider": "gemini",
            "model": "fixture-model",
            "sourceRevisions": [{
                "sourceExperienceId": "fixture-v4-history",
                "revision": "2026-01-01T00:00:00.000Z",
                "artifactIds": ["fixture-reflection-answered", "fixture-v4-evidence"]
            }],
            "state": "consumed",
            "createdAt": "2026-08-02T00:00:03.000Z",
            "expiresAt": "2026-09-01T00:00:03.000Z"
        }))
        .unwrap()
    }

    async fn seed_successful_audit(path: &Path) {
        let (_, digest) = packet_snapshot();
        let mut connection = connect(path, false).await.unwrap();
        raw_sql("BEGIN IMMEDIATE")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO v5_compatibility_write_guard (token, created_at) \
             VALUES ('slice4c2-seed-guard', '2026-08-02T00:00:03.000Z')",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO historical_consent_events \
             (id, packet_digest, payload, state, created_at, expires_at) \
             VALUES (?, ?, ?, 'consumed', '2026-08-02T00:00:03.000Z', \
             '2026-09-01T00:00:03.000Z')",
        )
        .bind(CONSENT_ID)
        .bind(&digest)
        .bind(consent_payload(&digest))
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO historical_transmission_events \
             (id, consent_id, packet_digest, provider, model, outcome, created_at, expires_at) \
             VALUES (?, ?, ?, 'gemini', 'fixture-model', 'sent', \
             '2026-08-02T00:00:04.000Z', '2026-09-01T00:00:04.000Z')",
        )
        .bind(TRANSMISSION_ID)
        .bind(CONSENT_ID)
        .bind(digest)
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query(
            "DELETE FROM v5_compatibility_write_guard WHERE token = 'slice4c2-seed-guard'",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        raw_sql("COMMIT").execute(&mut connection).await.unwrap();
    }

    fn request() -> HistoricalQuestionWriteRequest {
        let (packet_snapshot, _) = packet_snapshot();
        HistoricalQuestionWriteRequest {
            artifact_id: ARTIFACT_ID.into(),
            current_experience_id: "fixture-v4-current".into(),
            questions: vec![
                HistoricalQuestion {
                    id: "question-en".into(),
                    text: "What details feel similar or different to you?".into(),
                    source_experience_ids: vec![
                        "fixture-v4-current".into(),
                        "fixture-v4-history".into(),
                    ],
                },
                HistoricalQuestion {
                    id: "question-zh".into(),
                    text: "哪些細節讓你感到相似或不同？".into(),
                    source_experience_ids: vec!["fixture-v4-history".into()],
                },
                HistoricalQuestion {
                    id: "question-ja".into(),
                    text: "どの細部が似ている、または異なると感じますか？".into(),
                    source_experience_ids: vec!["fixture-v4-history".into()],
                },
            ],
            packet_snapshot,
            consent_id: CONSENT_ID.into(),
            transmission_id: TRANSMISSION_ID.into(),
            generated_at: GENERATED_AT.into(),
        }
    }

    fn context(failure_point: HistoricalWriteFailurePoint) -> HistoricalWriteContext<'static> {
        HistoricalWriteContext {
            guard_token: "slice4c2-write-guard",
            failure_point,
        }
    }

    async fn manifest_at(path: &Path) -> String {
        let mut connection = connect(path, true).await.unwrap();
        operation_manifest(&mut connection).await.unwrap()
    }

    #[tokio::test]
    async fn successful_creation_is_exact_and_duplicate_is_idempotent() {
        let (_directory, path) = create_v5_fixture().await;
        let first = write_disposable_historical_question(
            &path,
            request(),
            context(HistoricalWriteFailurePoint::None),
        )
        .await
        .unwrap();
        assert_eq!(first.status, HistoricalWriteStatus::Committed);
        let second = write_disposable_historical_question(
            &path,
            request(),
            context(HistoricalWriteFailurePoint::None),
        )
        .await
        .unwrap();
        assert_eq!(second.status, HistoricalWriteStatus::AlreadyCommitted);
        assert_eq!(first.operation_manifest, second.operation_manifest);
        let stable = manifest_at(&path).await;
        let mut conflicting = request();
        conflicting.questions[0].text = "What other detail would you compare?".into();
        assert_eq!(
            write_disposable_historical_question(
                &path,
                conflicting,
                context(HistoricalWriteFailurePoint::None),
            )
            .await
            .unwrap_err()
            .code,
            "historical_duplicate_conflict"
        );
        assert_eq!(manifest_at(&path).await, stable);

        let mut connection = connect(&path, true).await.unwrap();
        let parity: (i64, i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
             (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?), \
             (SELECT COUNT(*) FROM historical_question_lifecycle_links WHERE artifact_id = ?), \
             (SELECT COUNT(*) FROM artifact_heads WHERE id = ? AND artifact_kind = 'historical_question'), \
             (SELECT COUNT(*) FROM historical_artifact_dependencies WHERE historical_artifact_id = ?), \
             (SELECT COUNT(*) FROM artifact_dependencies WHERE dependent_artifact_id = ?)",
        )
        .bind(ARTIFACT_ID)
        .bind(ARTIFACT_ID)
        .bind(ARTIFACT_ID)
        .bind(ARTIFACT_ID)
        .bind(ARTIFACT_ID)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(parity, (1, 1, 1, 3, 4));
        let stored_packet: String = sqlx::query_scalar(
            "SELECT packet_snapshot FROM historical_question_artifacts WHERE id = ?",
        )
        .bind(ARTIFACT_ID)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(stored_packet.as_bytes(), request().packet_snapshot.as_bytes());
    }

    #[tokio::test]
    async fn no_question_and_unsuccessful_transport_create_no_actual_use_artifact() {
        let (_directory, path) = create_v5_fixture().await;
        let before = manifest_at(&path).await;
        let mut no_question = request();
        no_question.questions.clear();
        let result = write_disposable_historical_question(
            &path,
            no_question,
            context(HistoricalWriteFailurePoint::None),
        )
        .await
        .unwrap();
        assert_eq!(result.status, HistoricalWriteStatus::NoQuestion);
        assert_eq!(before, result.operation_manifest);

        let mut connection = connect(&path, false).await.unwrap();
        raw_sql("BEGIN IMMEDIATE")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO v5_compatibility_write_guard VALUES \
             ('slice4c2-failed-guard', '2026-08-02T00:00:04.500Z')",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query(
            "UPDATE historical_transmission_events SET outcome = 'failed' WHERE id = ?",
        )
        .bind(TRANSMISSION_ID)
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query("DELETE FROM v5_compatibility_write_guard")
            .execute(&mut connection)
            .await
            .unwrap();
        raw_sql("COMMIT").execute(&mut connection).await.unwrap();
        drop(connection);
        let before_failed = manifest_at(&path).await;
        let error = write_disposable_historical_question(
            &path,
            request(),
            context(HistoricalWriteFailurePoint::None),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "historical_transmission_mismatch");
        assert_eq!(manifest_at(&path).await, before_failed);
        let mut read_only = connect(&path, true).await.unwrap();
        let audit: (i64, i64) = sqlx::query_as(
            "SELECT \
             (SELECT COUNT(*) FROM historical_transmission_events \
                WHERE id = ? AND outcome = 'failed'), \
             (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?)",
        )
        .bind(TRANSMISSION_ID)
        .bind(ARTIFACT_ID)
        .fetch_one(&mut read_only)
        .await
        .unwrap();
        assert_eq!(audit, (1, 0));
    }

    #[tokio::test]
    async fn stale_malformed_duplicate_and_output_contract_inputs_fail_closed() {
        let cases = ["duplicate_dependency", "stale_source", "safety_contract"];
        for case in cases {
            let (_directory, path) = create_v5_fixture().await;
            let before = manifest_at(&path).await;
            let mut request = request();
            if case == "duplicate_dependency" {
                let mut packet: Value = serde_json::from_str(&request.packet_snapshot).unwrap();
                let duplicate = packet["includedItems"][0].clone();
                packet["includedItems"].as_array_mut().unwrap().push(duplicate);
                packet.as_object_mut().unwrap().remove("packetDigest");
                let digest = sha256_hex(canonical_json(&packet).unwrap().as_bytes());
                packet["packetDigest"] = json!(digest);
                request.packet_snapshot = canonical_json(&packet).unwrap();
            } else if case == "stale_source" {
                let mut packet: Value = serde_json::from_str(&request.packet_snapshot).unwrap();
                packet["includedItems"][0]["revision"] = json!("2026-01-01T00:00:09.000Z");
                packet.as_object_mut().unwrap().remove("packetDigest");
                let digest = sha256_hex(canonical_json(&packet).unwrap().as_bytes());
                packet["packetDigest"] = json!(digest);
                request.packet_snapshot = canonical_json(&packet).unwrap();
            } else {
                let mut packet: Value = serde_json::from_str(&request.packet_snapshot).unwrap();
                packet["versions"]["safetyContract"] = json!("phase-4-cross-experience-v1");
                packet.as_object_mut().unwrap().remove("packetDigest");
                let digest = sha256_hex(canonical_json(&packet).unwrap().as_bytes());
                packet["packetDigest"] = json!(digest);
                request.packet_snapshot = canonical_json(&packet).unwrap();
            }
            assert!(write_disposable_historical_question(
                &path,
                request,
                context(HistoricalWriteFailurePoint::None),
            )
            .await
            .is_err());
            assert_eq!(manifest_at(&path).await, before);
        }
    }

    #[tokio::test]
    async fn consent_scope_source_and_reflection_dependency_drift_fail_closed() {
        for case in ["missing_consent", "source_edit", "reflection_dependency"] {
            let (_directory, path) = create_v5_fixture().await;
            let mut connection = connect(&path, false).await.unwrap();
            raw_sql("BEGIN IMMEDIATE")
                .execute(&mut connection)
                .await
                .unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard VALUES \
                 ('slice4c2-drift-guard', '2026-08-02T00:00:04.500Z')",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            match case {
                "missing_consent" => {
                    sqlx::query("DELETE FROM historical_transmission_events WHERE id = ?")
                        .bind(TRANSMISSION_ID)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                    sqlx::query("DELETE FROM historical_consent_events WHERE id = ?")
                        .bind(CONSENT_ID)
                        .execute(&mut connection)
                        .await
                        .unwrap();
                }
                "source_edit" => {
                    sqlx::query(
                        "UPDATE experience_entries SET updated_at = \
                         '2026-01-01T00:00:09.000Z' WHERE id = 'fixture-v4-history'",
                    )
                    .execute(&mut connection)
                    .await
                    .unwrap();
                }
                "reflection_dependency" => {
                    sqlx::query(
                        "DELETE FROM artifact_dependencies WHERE dependent_artifact_id = \
                         'fixture-reflection-answered' AND relationship_type = 'uses_evidence'",
                    )
                    .execute(&mut connection)
                    .await
                    .unwrap();
                }
                _ => unreachable!(),
            }
            sqlx::query("DELETE FROM v5_compatibility_write_guard")
                .execute(&mut connection)
                .await
                .unwrap();
            raw_sql("COMMIT").execute(&mut connection).await.unwrap();
            drop(connection);
            let before = manifest_at(&path).await;
            assert!(write_disposable_historical_question(
                &path,
                request(),
                context(HistoricalWriteFailurePoint::None),
            )
            .await
            .is_err(), "drift case {case} must fail closed");
            assert_eq!(manifest_at(&path).await, before, "drift case {case}");
        }
    }

    #[tokio::test]
    async fn source_cap_unsupported_kind_and_missing_historical_citation_are_refused() {
        for case in ["source_cap", "unsupported_kind", "citation"] {
            let (_directory, path) = create_v5_fixture().await;
            let before = manifest_at(&path).await;
            let mut request = request();
            if case == "citation" {
                request.questions[0].source_experience_ids = vec!["fixture-v4-current".into()];
            } else {
                let mut packet: Value = serde_json::from_str(&request.packet_snapshot).unwrap();
                if case == "unsupported_kind" {
                    packet["includedItems"][1]["itemType"] = json!("pattern");
                } else {
                    for index in 2..=4 {
                        packet["includedItems"].as_array_mut().unwrap().push(json!({
                            "itemType": "experience",
                            "sourceExperienceId": format!("extra-source-{index}"),
                            "artifactId": Value::Null,
                            "revision": "2026-01-01T00:00:00.000Z"
                        }));
                    }
                }
                packet.as_object_mut().unwrap().remove("packetDigest");
                let digest = sha256_hex(canonical_json(&packet).unwrap().as_bytes());
                packet["packetDigest"] = json!(digest);
                request.packet_snapshot = canonical_json(&packet).unwrap();
            }
            assert!(write_disposable_historical_question(
                &path,
                request,
                context(HistoricalWriteFailurePoint::None),
            )
            .await
            .is_err());
            assert_eq!(manifest_at(&path).await, before, "boundary case {case}");
        }
    }

    #[tokio::test]
    async fn every_meaningful_write_boundary_rolls_back() {
        for point in [
            HistoricalWriteFailurePoint::AfterGuard,
            HistoricalWriteFailurePoint::AfterV4Artifact,
            HistoricalWriteFailurePoint::AfterV4Dependency,
            HistoricalWriteFailurePoint::AfterProvenance,
            HistoricalWriteFailurePoint::AfterRevision,
            HistoricalWriteFailurePoint::AfterContent,
            HistoricalWriteFailurePoint::AfterHead,
            HistoricalWriteFailurePoint::AfterProvenanceLink,
            HistoricalWriteFailurePoint::AfterV5Dependency,
            HistoricalWriteFailurePoint::AfterLifecycle,
            HistoricalWriteFailurePoint::AfterLink,
            HistoricalWriteFailurePoint::AfterReconciliation,
            HistoricalWriteFailurePoint::AfterGuardRemoval,
        ] {
            let (_directory, path) = create_v5_fixture().await;
            let before = manifest_at(&path).await;
            assert!(write_disposable_historical_question(&path, request(), context(point))
                .await
                .is_err());
            assert_eq!(manifest_at(&path).await, before, "failure point {point:?}");
        }
    }

    struct CommitThenUnknown;

    impl CommitOutcomeAdapter for CommitThenUnknown {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            raw_sql("COMMIT").execute(connection).await.unwrap();
            CommitAttemptOutcome::OutcomeUnknown {
                error_class: "injected_unknown_after_commit".into(),
            }
        }

        async fn rollback(&self, connection: &mut SqliteConnection) -> RollbackAttemptOutcome {
            raw_sql("ROLLBACK").execute(connection).await.unwrap();
            RollbackAttemptOutcome::RolledBack
        }
    }

    struct RollbackThenUnknown;

    impl CommitOutcomeAdapter for RollbackThenUnknown {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            raw_sql("ROLLBACK").execute(connection).await.unwrap();
            CommitAttemptOutcome::OutcomeUnknown {
                error_class: "injected_unknown_after_rollback".into(),
            }
        }

        async fn rollback(&self, connection: &mut SqliteConnection) -> RollbackAttemptOutcome {
            raw_sql("ROLLBACK").execute(connection).await.unwrap();
            RollbackAttemptOutcome::RolledBack
        }
    }

    struct CommitThenUnrelatedDurableAuditUnknown;

    impl CommitOutcomeAdapter for CommitThenUnrelatedDurableAuditUnknown {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            raw_sql("COMMIT").execute(&mut *connection).await.unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard VALUES \
                 ('slice4c2-ambiguous-drift-guard', '2026-08-02T00:00:05.500Z')",
            )
            .execute(&mut *connection)
            .await
            .unwrap();
            sqlx::query(
                "INSERT INTO historical_transmission_events \
                 (id, consent_id, packet_digest, provider, model, outcome, created_at, expires_at) \
                 VALUES ('slice4c2-unrelated-failed-audit', 'fixture-v4-consent', \
                 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa', \
                 'gemini', 'fixture-model', 'failed', \
                 '2026-08-02T00:00:05.500Z', '2026-09-01T00:00:05.500Z')",
            )
            .execute(&mut *connection)
            .await
            .unwrap();
            sqlx::query(
                "DELETE FROM v5_compatibility_write_guard \
                 WHERE token = 'slice4c2-ambiguous-drift-guard'",
            )
            .execute(connection)
            .await
            .unwrap();
            CommitAttemptOutcome::OutcomeUnknown {
                error_class: "injected_unknown_with_third_state".into(),
            }
        }

        async fn rollback(&self, connection: &mut SqliteConnection) -> RollbackAttemptOutcome {
            raw_sql("ROLLBACK").execute(connection).await.unwrap();
            RollbackAttemptOutcome::RolledBack
        }
    }

    #[tokio::test]
    async fn ambiguous_commit_is_classified_from_exact_read_only_state() {
        let (_committed_dir, committed_path) = create_v5_fixture().await;
        let committed = execute_with_adapter(
            &committed_path,
            request(),
            context(HistoricalWriteFailurePoint::None),
            &CommitThenUnknown,
        )
        .await
        .unwrap();
        assert_eq!(committed.status, HistoricalWriteStatus::Committed);

        let (_rolled_dir, rolled_path) = create_v5_fixture().await;
        let before = manifest_at(&rolled_path).await;
        let error = execute_with_adapter(
            &rolled_path,
            request(),
            context(HistoricalWriteFailurePoint::None),
            &RollbackThenUnknown,
        )
        .await
        .unwrap_err();
        assert!(error.code.starts_with("historical_commit_definitely_not_committed:"));
        assert_eq!(manifest_at(&rolled_path).await, before);

        let (_unknown_dir, unknown_path) = create_v5_fixture().await;
        let unknown = execute_with_adapter(
            &unknown_path,
            request(),
            context(HistoricalWriteFailurePoint::None),
            &CommitThenUnrelatedDurableAuditUnknown,
        )
        .await
        .unwrap_err();
        assert!(unknown.recovery_required);
        assert!(unknown.code.starts_with("historical_commit_outcome_unknown:"));
    }

    #[tokio::test]
    async fn created_shape_is_reached_by_existing_guarded_adr0009_cascade() {
        let (_directory, path) = create_v5_fixture().await;
        write_disposable_historical_question(
            &path,
            request(),
            context(HistoricalWriteFailurePoint::None),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, false).await.unwrap();
        raw_sql("BEGIN IMMEDIATE")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO v5_compatibility_write_guard VALUES \
             ('slice4c2-delete-guard', '2026-08-02T00:00:06.000Z')",
        )
        .execute(&mut connection)
        .await
        .unwrap();
        sqlx::query("DELETE FROM experience_entries WHERE id = 'fixture-v4-history'")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query("DELETE FROM v5_compatibility_write_guard")
            .execute(&mut connection)
            .await
            .unwrap();
        raw_sql("COMMIT").execute(&mut connection).await.unwrap();
        let remaining: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
             (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?), \
             (SELECT COUNT(*) FROM historical_question_lifecycle_links WHERE artifact_id = ?), \
             (SELECT COUNT(*) FROM artifact_heads WHERE id = ?), \
             (SELECT COUNT(*) FROM historical_artifact_dependencies WHERE historical_artifact_id = ?), \
             (SELECT COUNT(*) FROM historical_consent_events WHERE id = ?), \
             (SELECT COUNT(*) FROM historical_transmission_events WHERE id = ?)",
        )
        .bind(ARTIFACT_ID)
        .bind(ARTIFACT_ID)
        .bind(ARTIFACT_ID)
        .bind(ARTIFACT_ID)
        .bind(CONSENT_ID)
        .bind(TRANSMISSION_ID)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(remaining, (0, 0, 0, 0, 0, 0));
        let orphan_links: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM artifact_revision_provenance rp \
             LEFT JOIN artifact_revisions r ON r.id = rp.artifact_revision_id \
             WHERE r.id IS NULL",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(orphan_links, 0);
    }
}
