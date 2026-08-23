use super::experience_write::operation_manifest;
use super::pattern_write::verify_exact_pattern_v5;
use super::*;
use sqlx::{raw_sql, Row};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
enum EvidenceLifecycleCommand {
    CorrectConfirmed {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
        text: String,
    },
    DeleteConfirmed {
        source_id: String,
        artifact_id: String,
        expected_source_revision_id: String,
        expected_artifact_revision_id: String,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum EvidenceLifecycleFailurePoint {
    #[default]
    None,
    AfterGuard,
    AfterEvidenceRevision,
    AfterEvidenceLifecycle,
    AfterEvidenceHead,
    AfterEvidenceContentPurge,
    AfterOrdinaryInvalidation,
    AfterProjection,
    AfterHistoricalCascade,
    AfterReconciliation,
    AfterGuardRemoval,
}

#[derive(Clone, Debug)]
struct EvidenceLifecycleContext<'a> {
    occurred_at: &'a str,
    guard_token: &'a str,
    failure_point: EvidenceLifecycleFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EvidenceLifecycleStatus {
    Committed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EvidenceLifecycleOutcome {
    status: EvidenceLifecycleStatus,
    artifact_id: String,
    revision_id: Option<String>,
    invalidated_artifact_ids: Vec<String>,
    deleted_historical_artifact_ids: Vec<String>,
    operation_manifest: String,
}

#[derive(Clone, Debug)]
struct ConfirmedEvidence {
    source_id: String,
    revision_id: String,
    revision_number: i64,
    payload: Value,
    created_at: String,
    projection_updated_at: String,
    generated_provenance: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DependentEdge {
    dependency_id: String,
    artifact_id: String,
    revision_id: String,
    artifact_kind: String,
    source_id: String,
    current_revision_id: Option<String>,
    lifecycle_state: String,
    eligibility_state: String,
}

#[derive(Clone, Debug, Default)]
struct DependencyClosure {
    ordinary_edges: BTreeMap<String, DependentEdge>,
    historical_ids: BTreeSet<String>,
}

fn lifecycle_error(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn lifecycle_recovery(code: impl Into<String>) -> MigrationError {
    MigrationError::recovery_required(code)
}

fn validate_identifier(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        Err(lifecycle_error(format!(
            "evidence_lifecycle_{field}_invalid"
        )))
    } else {
        Ok(())
    }
}

fn validate_timestamp(value: &str) -> Result<(), MigrationError> {
    validate_identifier(value, "occurred_at")?;
    if !value.contains('T') || !value.ends_with('Z') {
        return Err(lifecycle_error("evidence_lifecycle_occurred_at_invalid"));
    }
    Ok(())
}

fn validate_text(value: &str) -> Result<(), MigrationError> {
    if value.trim().is_empty() || value.len() > 100_000 {
        Err(lifecycle_error("evidence_lifecycle_text_invalid"))
    } else {
        Ok(())
    }
}

fn inject(
    context: &EvidenceLifecycleContext<'_>,
    point: EvidenceLifecycleFailurePoint,
) -> Result<(), MigrationError> {
    if context.failure_point == point {
        Err(lifecycle_error(format!(
            "injected_evidence_lifecycle_failure:{point:?}"
        )))
    } else {
        Ok(())
    }
}

async fn insert_guard(
    connection: &mut SqliteConnection,
    context: &EvidenceLifecycleContext<'_>,
) -> Result<(), MigrationError> {
    validate_identifier(context.guard_token, "guard_token")?;
    validate_timestamp(context.occurred_at)?;
    if context.guard_token.len() < 32 {
        return Err(lifecycle_error("evidence_lifecycle_guard_token_invalid"));
    }
    sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
        .bind(context.guard_token)
        .bind(context.occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_lifecycle_guard_insert_failed", error))?;
    inject(context, EvidenceLifecycleFailurePoint::AfterGuard)
}

async fn remove_guard(
    connection: &mut SqliteConnection,
    context: &EvidenceLifecycleContext<'_>,
) -> Result<(), MigrationError> {
    let result = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
        .bind(context.guard_token)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_lifecycle_guard_remove_failed", error))?;
    if result.rows_affected() != 1 {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_guard_identity_mismatch",
        ));
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_lifecycle_guard_count_failed", error))?;
    if count != 0 {
        return Err(lifecycle_recovery("evidence_lifecycle_guard_not_empty"));
    }
    inject(context, EvidenceLifecycleFailurePoint::AfterGuardRemoval)
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
    .map_err(|error| migration_error("evidence_lifecycle_source_lookup_failed", error))?;
    let Some((revision_id, lifecycle, content, projection)) = row else {
        return Err(lifecycle_error("evidence_lifecycle_source_not_found"));
    };
    if revision_id != expected_revision_id || lifecycle != "active" || content != projection {
        return Err(lifecycle_error("evidence_lifecycle_source_revision_stale"));
    }
    Ok(())
}

async fn confirmed_evidence(
    connection: &mut SqliteConnection,
    source_id: &str,
    artifact_id: &str,
    expected_revision_id: &str,
) -> Result<ConfirmedEvidence, MigrationError> {
    validate_identifier(artifact_id, "artifact_id")?;
    validate_identifier(expected_revision_id, "artifact_revision_id")?;
    let row = sqlx::query(
        "SELECT h.source_id, h.current_revision_id, h.review_state, h.lifecycle_state, \
                h.eligibility_state, h.created_at, r.revision_number, \
                r.serialization_version, r.content_digest, c.payload, c.byte_length, \
                pa.payload, pa.updated_at, p.canonical_payload \
         FROM artifact_heads h \
         JOIN artifact_revisions r ON r.id = h.current_revision_id AND r.artifact_id = h.id \
         JOIN artifact_revision_content c ON c.revision_id = r.id \
         JOIN persisted_artifacts pa ON pa.id = h.id AND pa.source_entry_id = h.source_id \
           AND pa.artifact_kind = 'evidence' \
         JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id \
           AND rp.role = 'content' \
         JOIN provenance_records p ON p.id = rp.provenance_id \
         WHERE h.id = ? AND h.artifact_kind = 'evidence'",
    )
    .bind(artifact_id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_current_lookup_failed", error))?
    .ok_or_else(|| lifecycle_error("evidence_lifecycle_artifact_not_found"))?;
    let actual_source: String = row.get(0);
    let revision_id: String = row.get(1);
    if actual_source != source_id || revision_id != expected_revision_id {
        return Err(lifecycle_error(
            "evidence_lifecycle_artifact_revision_stale",
        ));
    }
    let review: String = row.get(2);
    let lifecycle: String = row.get(3);
    let eligibility: String = row.get(4);
    if review != "confirmed" || lifecycle != "active" || eligibility != "eligible" {
        return Err(lifecycle_error(
            "evidence_lifecycle_confirmed_state_required",
        ));
    }
    let serialization: String = row.get(7);
    if !matches!(
        serialization.as_str(),
        "legacy-v4-raw" | "canonical-json-v1"
    ) {
        return Err(lifecycle_error(
            "evidence_lifecycle_serialization_unsupported",
        ));
    }
    let raw_payload: String = row.get(9);
    let byte_length: i64 = row.get(10);
    let digest: String = row.get(8);
    if digest != sha256_hex(raw_payload.as_bytes()) || byte_length != raw_payload.len() as i64 {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_content_digest_mismatch",
        ));
    }
    let payload: Value = serde_json::from_str(&raw_payload)
        .map_err(|error| migration_error("evidence_lifecycle_payload_malformed", error))?;
    let projection_raw: String = row.get(11);
    let projection: Value = serde_json::from_str(&projection_raw)
        .map_err(|error| migration_error("evidence_lifecycle_projection_malformed", error))?;
    for (key, expected) in [("id", artifact_id), ("sourceEntryId", source_id)] {
        if payload.get(key).and_then(Value::as_str) != Some(expected)
            || projection.get(key).and_then(Value::as_str) != Some(expected)
        {
            return Err(lifecycle_error(format!(
                "evidence_lifecycle_{key}_mismatch"
            )));
        }
    }
    for key in ["text", "originalText", "kind", "userEditable", "createdAt"] {
        if payload.get(key).is_none() || payload.get(key) != projection.get(key) {
            return Err(lifecycle_error(format!(
                "evidence_lifecycle_{key}_missing_or_mismatch"
            )));
        }
    }
    if projection.get("status").and_then(Value::as_str) != Some("confirmed") {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_projection_not_confirmed",
        ));
    }
    let generated_provenance_raw: String = row.get(13);
    let generated_provenance: Value =
        serde_json::from_str(&generated_provenance_raw).map_err(|error| {
            migration_error("evidence_lifecycle_generated_provenance_malformed", error)
        })?;
    if projection.get("provenance") != Some(&generated_provenance) {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_projection_provenance_mismatch",
        ));
    }
    Ok(ConfirmedEvidence {
        source_id: actual_source,
        revision_id,
        revision_number: row.get(6),
        payload,
        created_at: row.get(5),
        projection_updated_at: row.get(12),
        generated_provenance,
    })
}

fn corrected_content(
    current: &ConfirmedEvidence,
    artifact_id: &str,
    text: &str,
) -> Result<Value, MigrationError> {
    validate_text(text)?;
    let mut object = Map::new();
    for key in [
        "createdAt",
        "id",
        "kind",
        "originalText",
        "sourceEntryId",
        "userEditable",
    ] {
        object.insert(
            key.to_string(),
            current
                .payload
                .get(key)
                .cloned()
                .ok_or_else(|| lifecycle_error(format!("evidence_lifecycle_{key}_missing")))?,
        );
    }
    if object.get("id").and_then(Value::as_str) != Some(artifact_id)
        || object.get("sourceEntryId").and_then(Value::as_str) != Some(current.source_id.as_str())
        || object.get("userEditable").and_then(Value::as_bool) != Some(true)
    {
        return Err(lifecycle_error(
            "evidence_lifecycle_correctable_content_invalid",
        ));
    }
    object.insert("text".to_string(), Value::String(text.to_string()));
    Ok(Value::Object(object))
}

fn candidate_projection(
    content: &Value,
    provenance: &Value,
    updated_at: &str,
) -> Result<Value, MigrationError> {
    let object = content
        .as_object()
        .ok_or_else(|| lifecycle_error("evidence_lifecycle_content_not_object"))?;
    Ok(json!({
        "createdAt": object.get("createdAt").cloned().ok_or_else(|| lifecycle_error("evidence_lifecycle_createdAt_missing"))?,
        "id": object.get("id").cloned().ok_or_else(|| lifecycle_error("evidence_lifecycle_id_missing"))?,
        "kind": object.get("kind").cloned().ok_or_else(|| lifecycle_error("evidence_lifecycle_kind_missing"))?,
        "originalText": object.get("originalText").cloned().ok_or_else(|| lifecycle_error("evidence_lifecycle_originalText_missing"))?,
        "provenance": provenance,
        "sourceEntryId": object.get("sourceEntryId").cloned().ok_or_else(|| lifecycle_error("evidence_lifecycle_sourceEntryId_missing"))?,
        "status": "candidate",
        "text": object.get("text").cloned().ok_or_else(|| lifecycle_error("evidence_lifecycle_text_missing"))?,
        "updatedAt": updated_at,
        "userEditable": object.get("userEditable").cloned().ok_or_else(|| lifecycle_error("evidence_lifecycle_userEditable_missing"))?
    }))
}

fn user_provenance(source_id: &str, artifact_id: &str, occurred_at: &str) -> Value {
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

async fn collect_dependency_closure(
    connection: &mut SqliteConnection,
    evidence: &ConfirmedEvidence,
    evidence_artifact_id: &str,
) -> Result<DependencyClosure, MigrationError> {
    let rows = sqlx::query(
        "SELECT d.id, d.relationship_type, d.dependent_artifact_id, \
                d.dependent_revision_id, h.artifact_kind, h.source_id, \
                h.current_revision_id, h.lifecycle_state, h.eligibility_state \
         FROM artifact_dependencies d \
         JOIN artifact_heads h ON h.id = d.dependent_artifact_id \
         WHERE d.source_artifact_id = ? AND d.source_artifact_revision_id = ? \
         ORDER BY d.id",
    )
    .bind(evidence_artifact_id)
    .bind(&evidence.revision_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_dependencies_unreadable", error))?;

    let mut closure = DependencyClosure::default();
    let mut reflection_sources = Vec::new();
    for row in rows {
        let dependency_id: String = row.get(0);
        let relationship: String = row.get(1);
        let dependent_artifact_id: String = row.get(2);
        let revision_id: String = row.get(3);
        let artifact_kind: String = row.get(4);
        let source_id: String = row.get(5);
        for (value, field) in [
            (&dependency_id, "dependency_id"),
            (&dependent_artifact_id, "dependent_artifact_id"),
            (&revision_id, "dependent_revision_id"),
        ] {
            validate_identifier(value, field)?;
        }
        if artifact_kind == "historical_question" {
            if relationship != "historical_packet_item" {
                return Err(lifecycle_error(
                    "evidence_lifecycle_historical_relationship_unsupported",
                ));
            }
            let _historical_id: String = sqlx::query_scalar(
                "SELECT historical_artifact_id FROM historical_question_lifecycle_links \
                 WHERE artifact_id = ?",
            )
            .bind(&dependent_artifact_id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("evidence_lifecycle_historical_link_unreadable", error)
            })?
            .ok_or_else(|| lifecycle_error("evidence_lifecycle_historical_link_missing"))?;
            continue;
        }
        if relationship != "uses_evidence"
            || !matches!(artifact_kind.as_str(), "reflection" | "pattern")
            || source_id != evidence.source_id
            || dependent_artifact_id == evidence_artifact_id
        {
            return Err(lifecycle_error(
                "evidence_lifecycle_ordinary_dependency_unsupported",
            ));
        }
        let edge = DependentEdge {
            dependency_id: dependency_id.clone(),
            artifact_id: dependent_artifact_id.clone(),
            revision_id: revision_id.clone(),
            artifact_kind: artifact_kind.clone(),
            source_id,
            current_revision_id: row.get(6),
            lifecycle_state: row.get(7),
            eligibility_state: row.get(8),
        };
        if closure.ordinary_edges.insert(dependency_id, edge).is_some() {
            return Err(lifecycle_error(
                "evidence_lifecycle_dependency_id_duplicate",
            ));
        }
        if artifact_kind == "reflection" {
            reflection_sources.push((dependent_artifact_id, revision_id));
        }
    }

    for (reflection_id, reflection_revision_id) in reflection_sources {
        let rows = sqlx::query(
            "SELECT d.id, d.dependent_artifact_id, d.dependent_revision_id, \
                    h.artifact_kind, h.source_id, h.current_revision_id, \
                    h.lifecycle_state, h.eligibility_state \
             FROM artifact_dependencies d \
             JOIN artifact_heads h ON h.id = d.dependent_artifact_id \
             WHERE d.source_artifact_id = ? AND d.source_artifact_revision_id = ? \
               AND d.relationship_type = 'uses_reflection_response' ORDER BY d.id",
        )
        .bind(&reflection_id)
        .bind(&reflection_revision_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| {
            migration_error(
                "evidence_lifecycle_transitive_dependencies_unreadable",
                error,
            )
        })?;
        for row in rows {
            let dependency_id: String = row.get(0);
            let dependent_id: String = row.get(1);
            let dependent_revision_id: String = row.get(2);
            let kind: String = row.get(3);
            let source_id: String = row.get(4);
            if kind != "pattern" || source_id != evidence.source_id {
                return Err(lifecycle_error(
                    "evidence_lifecycle_transitive_dependency_unsupported",
                ));
            }
            for (value, field) in [
                (&dependency_id, "dependency_id"),
                (&dependent_id, "dependent_artifact_id"),
                (&dependent_revision_id, "dependent_revision_id"),
            ] {
                validate_identifier(value, field)?;
            }
            let edge = DependentEdge {
                dependency_id: dependency_id.clone(),
                artifact_id: dependent_id,
                revision_id: dependent_revision_id,
                artifact_kind: kind,
                source_id,
                current_revision_id: row.get(5),
                lifecycle_state: row.get(6),
                eligibility_state: row.get(7),
            };
            if closure.ordinary_edges.insert(dependency_id, edge).is_some() {
                return Err(lifecycle_error(
                    "evidence_lifecycle_dependency_id_duplicate",
                ));
            }
        }
    }

    let mut historical_sources = BTreeMap::new();
    historical_sources.insert(
        evidence_artifact_id.to_string(),
        (
            evidence.revision_id.clone(),
            evidence.projection_updated_at.clone(),
        ),
    );
    for edge in closure.ordinary_edges.values() {
        let existing = historical_sources
            .entry(edge.artifact_id.clone())
            .or_insert_with(|| (edge.revision_id.clone(), String::new()));
        if existing.0 != edge.revision_id {
            return Err(lifecycle_error(
                "evidence_lifecycle_historical_source_revision_conflict",
            ));
        }
    }

    let mut affected_historical = BTreeSet::new();
    for (source_artifact_id, (source_revision_id, expected_v4_revision)) in &mut historical_sources
    {
        let normalized_rows: Vec<String> = sqlx::query_scalar(
            "SELECT l.historical_artifact_id FROM artifact_dependencies d \
             JOIN artifact_heads h ON h.id = d.dependent_artifact_id \
             JOIN historical_question_lifecycle_links l ON l.artifact_id = h.id \
             WHERE d.source_artifact_id = ? AND d.source_artifact_revision_id = ? \
               AND d.relationship_type = 'historical_packet_item' \
               AND h.artifact_kind = 'historical_question' ORDER BY l.historical_artifact_id",
        )
        .bind(source_artifact_id.as_str())
        .bind(source_revision_id.as_str())
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| {
            migration_error(
                "evidence_lifecycle_normalized_historical_sources_unreadable",
                error,
            )
        })?;
        let normalized = normalized_rows.into_iter().collect::<BTreeSet<_>>();
        let v4_rows: Vec<(String, String, Option<String>, String)> = sqlx::query_as(
            "SELECT historical_artifact_id, source_entry_id, source_artifact_id, source_revision \
             FROM historical_artifact_dependencies WHERE source_artifact_id = ? \
             ORDER BY historical_artifact_id",
        )
        .bind(source_artifact_id.as_str())
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_lifecycle_v4_history_unreadable", error))?;
        if (!normalized.is_empty() || !v4_rows.is_empty()) && expected_v4_revision.is_empty() {
            *expected_v4_revision = sqlx::query_scalar(
                "SELECT pa.updated_at FROM artifact_heads h \
                 JOIN persisted_artifacts pa ON pa.id = h.id \
                   AND pa.source_entry_id = h.source_id AND pa.artifact_kind = h.artifact_kind \
                 WHERE h.id = ? AND h.current_revision_id = ? \
                   AND h.lifecycle_state = 'active'",
            )
            .bind(source_artifact_id.as_str())
            .bind(source_revision_id.as_str())
            .fetch_optional(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("evidence_lifecycle_historical_source_unreadable", error)
            })?
            .ok_or_else(|| lifecycle_error("evidence_lifecycle_historical_source_not_current"))?;
        }
        let mut v4 = BTreeSet::new();
        for (historical_id, source_id, actual_artifact_id, source_revision) in v4_rows {
            validate_identifier(&historical_id, "historical_artifact_id")?;
            if source_id != evidence.source_id
                || actual_artifact_id.as_deref() != Some(source_artifact_id.as_str())
                || source_revision != *expected_v4_revision
                || !v4.insert(historical_id)
            {
                return Err(lifecycle_error(
                    "evidence_lifecycle_v4_historical_dependency_mismatch",
                ));
            }
        }
        if normalized != v4 {
            return Err(lifecycle_error(
                "evidence_lifecycle_historical_representation_mismatch",
            ));
        }
        affected_historical.extend(v4);
    }
    for historical_id in &affected_historical {
        let represented: (i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_question_lifecycle_links WHERE historical_artifact_id = ?), \
               (SELECT COUNT(*) FROM historical_question_lifecycle_links l \
                JOIN artifact_heads h ON h.id = l.artifact_id \
                WHERE l.historical_artifact_id = ? AND h.artifact_kind = 'historical_question' \
                  AND h.lifecycle_state = 'active')",
        )
        .bind(historical_id)
        .bind(historical_id)
        .bind(historical_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| {
            migration_error("evidence_lifecycle_historical_representation_unreadable", error)
        })?;
        if represented != (1, 1, 1) {
            return Err(lifecycle_error(
                "evidence_lifecycle_historical_representation_incomplete",
            ));
        }
    }
    closure.historical_ids = affected_historical;
    Ok(closure)
}

async fn append_invalidation_event(
    connection: &mut SqliteConnection,
    edge: &DependentEdge,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let existing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_lifecycle_events \
         WHERE artifact_id = ? AND subject_revision_id = ? \
           AND dependency_id = ? AND event_type = 'invalidated'",
    )
    .bind(&edge.artifact_id)
    .bind(&edge.revision_id)
    .bind(&edge.dependency_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_invalidation_lookup_failed", error))?;
    if existing == 1 {
        return Ok(());
    }
    if existing != 0 {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_invalidation_duplicate",
        ));
    }
    let domain = format!(
        "life-os/exact-dependent-invalidated-event-id-v1:{}",
        edge.dependency_id
    );
    let id = event_id("v5le_", &domain, &edge.artifact_id, &edge.revision_id);
    sqlx::query(
        "INSERT INTO artifact_lifecycle_events ( \
           id, artifact_id, subject_revision_id, related_revision_id, dependency_id, \
           event_type, actor, reason_code, occurred_at \
         ) VALUES (?, ?, ?, NULL, ?, 'invalidated', 'system', \
           'exact_source_revision_no_longer_current', ?)",
    )
    .bind(id)
    .bind(&edge.artifact_id)
    .bind(&edge.revision_id)
    .bind(&edge.dependency_id)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_invalidation_insert_failed", error))?;
    Ok(())
}

async fn invalidate_ordinary_dependents(
    connection: &mut SqliteConnection,
    closure: &DependencyClosure,
    context: &EvidenceLifecycleContext<'_>,
) -> Result<Vec<String>, MigrationError> {
    let mut transitioned = BTreeSet::new();
    for edge in closure.ordinary_edges.values() {
        if !matches!(edge.artifact_kind.as_str(), "reflection" | "pattern") {
            return Err(lifecycle_error(
                "evidence_lifecycle_dependency_cycle_or_kind_invalid",
            ));
        }
        append_invalidation_event(connection, edge, context.occurred_at).await?;
        if edge.current_revision_id.as_deref() != Some(edge.revision_id.as_str()) {
            continue;
        }
        match (
            edge.lifecycle_state.as_str(),
            edge.eligibility_state.as_str(),
        ) {
            ("active", _) => {
                if transitioned.insert(edge.artifact_id.clone()) {
                    let updated = sqlx::query(
                        "UPDATE artifact_heads SET lifecycle_state = 'invalidated', \
                           eligibility_state = 'ineligible', \
                           eligibility_reason = 'exact_dependency_invalidated', updated_at = ? \
                         WHERE id = ? AND current_revision_id = ? AND artifact_kind = ? \
                           AND lifecycle_state = 'active'",
                    )
                    .bind(context.occurred_at)
                    .bind(&edge.artifact_id)
                    .bind(&edge.revision_id)
                    .bind(&edge.artifact_kind)
                    .execute(&mut *connection)
                    .await
                    .map_err(|error| {
                        migration_error("evidence_lifecycle_dependent_head_update_failed", error)
                    })?;
                    if updated.rows_affected() != 1 {
                        return Err(lifecycle_error(
                            "evidence_lifecycle_dependent_revision_stale",
                        ));
                    }
                    let deleted = sqlx::query(
                        "DELETE FROM persisted_artifacts WHERE id = ? \
                           AND source_entry_id = ? AND artifact_kind = ?",
                    )
                    .bind(&edge.artifact_id)
                    .bind(&edge.source_id)
                    .bind(&edge.artifact_kind)
                    .execute(&mut *connection)
                    .await
                    .map_err(|error| {
                        migration_error(
                            "evidence_lifecycle_dependent_projection_delete_failed",
                            error,
                        )
                    })?;
                    if deleted.rows_affected() != 1 {
                        return Err(lifecycle_recovery(
                            "evidence_lifecycle_dependent_projection_mismatch",
                        ));
                    }
                }
            }
            ("invalidated", "ineligible") => {
                let projection: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM persisted_artifacts WHERE id = ? AND artifact_kind = ?",
                )
                .bind(&edge.artifact_id)
                .bind(&edge.artifact_kind)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error(
                        "evidence_lifecycle_invalidated_projection_unreadable",
                        error,
                    )
                })?;
                if projection != 0 {
                    return Err(lifecycle_recovery(
                        "evidence_lifecycle_invalidated_projection_present",
                    ));
                }
                transitioned.insert(edge.artifact_id.clone());
            }
            _ => {
                return Err(lifecycle_error(
                    "evidence_lifecycle_dependent_state_unsupported",
                ))
            }
        }
    }
    inject(
        context,
        EvidenceLifecycleFailurePoint::AfterOrdinaryInvalidation,
    )?;
    Ok(transitioned.into_iter().collect())
}

async fn cascade_historical_questions(
    connection: &mut SqliteConnection,
    closure: &DependencyClosure,
    context: &EvidenceLifecycleContext<'_>,
) -> Result<Vec<String>, MigrationError> {
    for historical_id in &closure.historical_ids {
        let identities: (String, String, String) = sqlx::query_as(
            "SELECT l.artifact_id, q.consent_id, q.transmission_id \
             FROM historical_question_lifecycle_links l \
             JOIN historical_question_artifacts q ON q.id = l.historical_artifact_id \
             WHERE l.historical_artifact_id = ?",
        )
        .bind(historical_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| {
            migration_error("evidence_lifecycle_historical_identity_unreadable", error)
        })?;
        let deleted = sqlx::query("DELETE FROM historical_question_artifacts WHERE id = ?")
            .bind(historical_id)
            .execute(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("evidence_lifecycle_historical_delete_failed", error)
            })?;
        if deleted.rows_affected() != 1 {
            return Err(lifecycle_recovery(
                "evidence_lifecycle_historical_delete_identity_mismatch",
            ));
        }
        let remaining: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_artifact_dependencies WHERE historical_artifact_id = ?), \
               (SELECT COUNT(*) FROM historical_question_lifecycle_links WHERE historical_artifact_id = ?), \
               (SELECT COUNT(*) FROM artifact_heads WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_consent_events WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_transmission_events WHERE id = ?)",
        )
        .bind(&identities.0)
        .bind(&identities.1)
        .bind(&identities.2)
        .bind(historical_id)
        .bind(historical_id)
        .bind(historical_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| {
            migration_error("evidence_lifecycle_historical_delete_reconcile_failed", error)
        })?;
        if remaining != (0, 0, 0, 0, 0, 0) {
            return Err(lifecycle_recovery(
                "evidence_lifecycle_historical_delete_incomplete",
            ));
        }
    }
    inject(
        context,
        EvidenceLifecycleFailurePoint::AfterHistoricalCascade,
    )?;
    Ok(closure.historical_ids.iter().cloned().collect())
}

async fn correct_confirmed(
    connection: &mut SqliteConnection,
    evidence: &ConfirmedEvidence,
    artifact_id: &str,
    text: &str,
    context: &EvidenceLifecycleContext<'_>,
) -> Result<String, MigrationError> {
    let content = corrected_content(evidence, artifact_id, text)?;
    let payload = canonical_json(&content)?;
    let digest = sha256_hex(payload.as_bytes());
    let revision_id = artifact_revision_id(artifact_id, "evidence", context.occurred_at, &digest);
    let provenance = user_provenance(&evidence.source_id, artifact_id, context.occurred_at);
    let provenance_id = insert_provenance(connection, &provenance, context.occurred_at).await?;
    sqlx::query(
        "INSERT INTO artifact_revisions ( \
           id, artifact_id, source_id, revision_number, predecessor_revision_id, authorship, \
           revision_reason, serialization_version, content_digest, created_at \
         ) VALUES (?, ?, ?, ?, ?, 'user', 'corrected', 'canonical-json-v1', ?, ?)",
    )
    .bind(&revision_id)
    .bind(artifact_id)
    .bind(&evidence.source_id)
    .bind(evidence.revision_number + 1)
    .bind(&evidence.revision_id)
    .bind(&digest)
    .bind(context.occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_revision_insert_failed", error))?;
    sqlx::query(
        "INSERT INTO artifact_revision_content (revision_id, payload, byte_length) VALUES (?, ?, ?)",
    )
    .bind(&revision_id)
    .bind(&payload)
    .bind(payload.len() as i64)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_content_insert_failed", error))?;
    sqlx::query(
        "INSERT INTO artifact_revision_provenance (artifact_revision_id, role, provenance_id) \
         VALUES (?, 'content', ?)",
    )
    .bind(&revision_id)
    .bind(provenance_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_provenance_link_failed", error))?;
    let source_revision_id: String =
        sqlx::query_scalar("SELECT current_revision_id FROM source_heads WHERE id = ?")
            .bind(&evidence.source_id)
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("evidence_lifecycle_source_revision_unreadable", error)
            })?;
    let dependency = dependency_id(
        artifact_id,
        &revision_id,
        "derived_from_experience",
        &evidence.source_id,
        &source_revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_dependencies ( \
           id, dependent_artifact_id, dependent_revision_id, relationship_type, \
           source_revision_id, source_artifact_id, source_artifact_revision_id, created_at \
         ) VALUES (?, ?, ?, 'derived_from_experience', ?, NULL, NULL, ?)",
    )
    .bind(dependency)
    .bind(artifact_id)
    .bind(&revision_id)
    .bind(source_revision_id)
    .bind(context.occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| {
        migration_error("evidence_lifecycle_source_dependency_insert_failed", error)
    })?;
    inject(
        context,
        EvidenceLifecycleFailurePoint::AfterEvidenceRevision,
    )?;
    for (domain, event_type, subject, related, reason) in [
        (
            "life-os/evidence-confirmed-corrected-event-id-v1",
            "corrected",
            revision_id.as_str(),
            evidence.revision_id.as_str(),
            "confirmed_evidence_user_corrected",
        ),
        (
            "life-os/evidence-confirmed-superseded-event-id-v1",
            "superseded",
            evidence.revision_id.as_str(),
            revision_id.as_str(),
            "replaced_by_user_correction",
        ),
    ] {
        let id = event_id("v5le_", domain, artifact_id, subject);
        sqlx::query(
            "INSERT INTO artifact_lifecycle_events ( \
               id, artifact_id, subject_revision_id, related_revision_id, dependency_id, \
               event_type, actor, reason_code, occurred_at \
             ) VALUES (?, ?, ?, ?, NULL, ?, 'user', ?, ?)",
        )
        .bind(id)
        .bind(artifact_id)
        .bind(subject)
        .bind(related)
        .bind(event_type)
        .bind(reason)
        .bind(context.occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("evidence_lifecycle_event_insert_failed", error))?;
    }
    inject(
        context,
        EvidenceLifecycleFailurePoint::AfterEvidenceLifecycle,
    )?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = ?, review_state = 'pending', \
           lifecycle_state = 'active', eligibility_state = 'ineligible', \
           eligibility_reason = 'correction_requires_confirmation', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'evidence' AND review_state = 'confirmed' \
           AND lifecycle_state = 'active' AND eligibility_state = 'eligible'",
    )
    .bind(&revision_id)
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(&evidence.source_id)
    .bind(&evidence.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_head_update_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(lifecycle_error(
            "evidence_lifecycle_artifact_revision_stale",
        ));
    }
    inject(context, EvidenceLifecycleFailurePoint::AfterEvidenceHead)?;
    let projection = candidate_projection(
        &content,
        &evidence.generated_provenance,
        context.occurred_at,
    )?;
    let serialized = canonical_json(&projection)?;
    let projected = sqlx::query(
        "UPDATE persisted_artifacts SET payload = ?, updated_at = ? \
         WHERE id = ? AND source_entry_id = ? AND artifact_kind = 'evidence'",
    )
    .bind(serialized)
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(&evidence.source_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_projection_update_failed", error))?;
    if projected.rows_affected() != 1 {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_projection_identity_mismatch",
        ));
    }
    inject(context, EvidenceLifecycleFailurePoint::AfterProjection)?;
    Ok(revision_id)
}

async fn delete_confirmed(
    connection: &mut SqliteConnection,
    evidence: &ConfirmedEvidence,
    artifact_id: &str,
    context: &EvidenceLifecycleContext<'_>,
) -> Result<(), MigrationError> {
    let event_id = event_id(
        "v5le_",
        "life-os/evidence-user-deleted-event-id-v1",
        artifact_id,
        &evidence.revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_lifecycle_events ( \
           id, artifact_id, subject_revision_id, related_revision_id, dependency_id, \
           event_type, actor, reason_code, occurred_at \
         ) VALUES (?, ?, ?, NULL, NULL, 'deleted', 'user', 'explicit_user_deletion', ?)",
    )
    .bind(event_id)
    .bind(artifact_id)
    .bind(&evidence.revision_id)
    .bind(context.occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_delete_event_insert_failed", error))?;
    inject(
        context,
        EvidenceLifecycleFailurePoint::AfterEvidenceLifecycle,
    )?;
    let updated = sqlx::query(
        "UPDATE artifact_heads SET current_revision_id = NULL, review_state = 'confirmed', \
           lifecycle_state = 'deleted', eligibility_state = 'ineligible', \
           eligibility_reason = 'explicit_user_deletion', updated_at = ? \
         WHERE id = ? AND source_id = ? AND current_revision_id = ? \
           AND artifact_kind = 'evidence' AND review_state = 'confirmed' \
           AND lifecycle_state = 'active' AND eligibility_state = 'eligible'",
    )
    .bind(context.occurred_at)
    .bind(artifact_id)
    .bind(&evidence.source_id)
    .bind(&evidence.revision_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_delete_head_failed", error))?;
    if updated.rows_affected() != 1 {
        return Err(lifecycle_error(
            "evidence_lifecycle_artifact_revision_stale",
        ));
    }
    inject(context, EvidenceLifecycleFailurePoint::AfterEvidenceHead)?;
    let tombstone_id = format!(
        "v5ts_{}",
        sha256_hex(
            format!("life-os/evidence-user-deleted-artifact-tombstone-id-v1\0{artifact_id}")
                .as_bytes()
        )
    );
    sqlx::query(
        "INSERT INTO content_tombstones ( \
           id, subject_type, source_id, source_revision_id, artifact_id, \
           artifact_revision_id, content_digest, reason_code, purged_at \
         ) VALUES (?, 'artifact', NULL, NULL, ?, NULL, NULL, \
           'user_deleted_artifact', ?)",
    )
    .bind(tombstone_id)
    .bind(artifact_id)
    .bind(context.occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_tombstone_insert_failed", error))?;
    let purged = sqlx::query(
        "DELETE FROM artifact_revision_content WHERE revision_id IN ( \
           SELECT id FROM artifact_revisions WHERE artifact_id = ?)",
    )
    .bind(artifact_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_content_purge_failed", error))?;
    if purged.rows_affected() == 0 {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_content_purge_identity_mismatch",
        ));
    }
    inject(
        context,
        EvidenceLifecycleFailurePoint::AfterEvidenceContentPurge,
    )?;
    let projected = sqlx::query(
        "DELETE FROM persisted_artifacts WHERE id = ? AND source_entry_id = ? \
           AND artifact_kind = 'evidence'",
    )
    .bind(artifact_id)
    .bind(&evidence.source_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_projection_delete_failed", error))?;
    if projected.rows_affected() != 1 {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_projection_identity_mismatch",
        ));
    }
    inject(context, EvidenceLifecycleFailurePoint::AfterProjection)?;
    Ok(())
}

async fn verify_historical_link_integrity(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        "SELECT \
           (SELECT COUNT(*) FROM historical_question_artifacts), \
           (SELECT COUNT(*) FROM historical_question_lifecycle_links), \
           (SELECT COUNT(*) FROM artifact_heads WHERE artifact_kind = 'historical_question'), \
           (SELECT COUNT(*) FROM historical_question_lifecycle_links l \
             JOIN historical_question_artifacts q ON q.id = l.historical_artifact_id \
             JOIN artifact_heads h ON h.id = l.artifact_id \
             WHERE h.artifact_kind = 'historical_question')",
    )
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("evidence_lifecycle_historical_links_unreadable", error))?;
    if counts.0 != counts.1 || counts.1 != counts.2 || counts.2 != counts.3 {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_historical_links_inconsistent",
        ));
    }
    Ok(())
}

async fn verify_lifecycle_database(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    verify_exact_pattern_v5(connection).await?;
    verify_historical_link_integrity(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await
}

async fn prepare_lifecycle(
    connection: &mut SqliteConnection,
    command: EvidenceLifecycleCommand,
    context: &EvidenceLifecycleContext<'_>,
) -> Result<EvidenceLifecycleOutcome, MigrationError> {
    let (source_id, artifact_id, expected_source_revision_id, expected_revision_id) = match &command
    {
        EvidenceLifecycleCommand::CorrectConfirmed {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
            ..
        }
        | EvidenceLifecycleCommand::DeleteConfirmed {
            source_id,
            artifact_id,
            expected_source_revision_id,
            expected_artifact_revision_id,
        } => (
            source_id.as_str(),
            artifact_id.as_str(),
            expected_source_revision_id.as_str(),
            expected_artifact_revision_id.as_str(),
        ),
    };
    exact_current_source(connection, source_id, expected_source_revision_id).await?;
    let evidence =
        confirmed_evidence(connection, source_id, artifact_id, expected_revision_id).await?;
    let closure = collect_dependency_closure(connection, &evidence, artifact_id).await?;
    insert_guard(connection, context).await?;
    let revision_id = match &command {
        EvidenceLifecycleCommand::CorrectConfirmed { text, .. } => {
            Some(correct_confirmed(connection, &evidence, artifact_id, text, context).await?)
        }
        EvidenceLifecycleCommand::DeleteConfirmed { .. } => {
            delete_confirmed(connection, &evidence, artifact_id, context).await?;
            None
        }
    };
    let invalidated = invalidate_ordinary_dependents(connection, &closure, context).await?;
    let deleted_historical = cascade_historical_questions(connection, &closure, context).await?;
    verify_historical_link_integrity(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    inject(context, EvidenceLifecycleFailurePoint::AfterReconciliation)?;
    remove_guard(connection, context).await?;
    verify_lifecycle_database(connection).await?;
    let post_manifest = operation_manifest(connection).await?;
    Ok(EvidenceLifecycleOutcome {
        status: EvidenceLifecycleStatus::Committed,
        artifact_id: artifact_id.to_string(),
        revision_id,
        invalidated_artifact_ids: invalidated,
        deleted_historical_artifact_ids: deleted_historical,
        operation_manifest: post_manifest,
    })
}

async fn verify_read_only(path: &Path, expected_manifest: &str) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await?;
    verify_lifecycle_database(&mut connection).await?;
    let actual = operation_manifest(&mut connection).await?;
    if actual != expected_manifest {
        return Err(lifecycle_recovery(
            "evidence_lifecycle_operation_manifest_mismatch",
        ));
    }
    Ok(())
}

async fn execute_with_adapter<A: CommitOutcomeAdapter>(
    path: &Path,
    command: EvidenceLifecycleCommand,
    context: EvidenceLifecycleContext<'_>,
    adapter: &A,
) -> Result<EvidenceLifecycleOutcome, MigrationError> {
    let mut connection = connect(path, false).await?;
    verify_lifecycle_database(&mut connection).await?;
    let pre_manifest = operation_manifest(&mut connection).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("evidence_lifecycle_begin_failed", error))?;
    let prepared = match prepare_lifecycle(&mut connection, command, &context).await {
        Ok(prepared) => prepared,
        Err(error) => {
            let rollback = adapter.rollback(&mut connection).await;
            connection.close().await.map_err(|close_error| {
                lifecycle_recovery(format!("evidence_lifecycle_close_failed:{close_error}"))
            })?;
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verify| {
                    lifecycle_recovery(format!(
                        "evidence_lifecycle_precommit_state_unverified:{}:{rollback:?}",
                        verify.code
                    ))
                })?;
            return Err(error);
        }
    };
    match adapter.commit(&mut connection).await {
        CommitAttemptOutcome::Committed => {
            connection.close().await.map_err(|error| {
                lifecycle_recovery(format!("evidence_lifecycle_close_failed:{error}"))
            })?;
            verify_read_only(path, &prepared.operation_manifest).await?;
            Ok(prepared)
        }
        CommitAttemptOutcome::DefinitelyNotCommitted { error_class } => {
            let rollback = adapter.rollback(&mut connection).await;
            connection.close().await.map_err(|error| {
                lifecycle_recovery(format!("evidence_lifecycle_close_failed:{error}"))
            })?;
            verify_read_only(path, &pre_manifest).await.map_err(|verify| {
                lifecycle_recovery(format!(
                    "evidence_lifecycle_definite_noncommit_unverified:{error_class}:{}:{rollback:?}",
                    verify.code
                ))
            })?;
            Err(lifecycle_error(format!(
                "evidence_lifecycle_commit_definitely_not_committed:{error_class}"
            )))
        }
        CommitAttemptOutcome::OutcomeUnknown { error_class } => {
            connection.close().await.map_err(|error| {
                lifecycle_recovery(format!("evidence_lifecycle_close_failed:{error}"))
            })?;
            if verify_read_only(path, &prepared.operation_manifest)
                .await
                .is_ok()
            {
                return Ok(prepared);
            }
            if verify_read_only(path, &pre_manifest).await.is_ok() {
                return Err(lifecycle_error(format!(
                    "evidence_lifecycle_commit_outcome_unknown_unchanged:{error_class}"
                )));
            }
            Err(lifecycle_recovery(format!(
                "evidence_lifecycle_commit_outcome_unknown:{error_class}"
            )))
        }
    }
}

async fn execute_disposable(
    path: &Path,
    command: EvidenceLifecycleCommand,
    context: EvidenceLifecycleContext<'_>,
) -> Result<EvidenceLifecycleOutcome, MigrationError> {
    execute_with_adapter(path, command, context, &SqlCommitOutcomeAdapter).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const SOURCE_ID: &str = "fixture-v4-history";
    const EVIDENCE_ID: &str = "fixture-v4-evidence";
    const STARTED_AT: &str = "2026-08-02T01:00:00.000Z";
    const COMMITTED_AT: &str = "2026-08-02T01:00:01.000Z";
    const CORRECTED_AT: &str = "2026-08-02T02:00:00.000Z";
    const DELETED_AT: &str = "2026-08-02T02:01:00.000Z";

    async fn exact_v5_fixture(
        ordinary_dependents: bool,
        historical: bool,
    ) -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        let evidence = json!({
            "createdAt": "2026-01-01T00:00:01.000Z",
            "id": EVIDENCE_ID,
            "kind": "observation",
            "originalText": "observed",
            "provenance": {
                "generatedAt": "2026-01-01T00:00:01.000Z",
                "harnessVersion": "harness-v1",
                "model": "fixture-model",
                "origin": "ai",
                "promptVersion": "evidence-v1",
                "provider": "gemini",
                "sourceArtifactIds": [],
                "sourceEntryId": SOURCE_ID
            },
            "sourceEntryId": SOURCE_ID,
            "status": "confirmed",
            "text": "observed",
            "updatedAt": "2026-01-01T00:00:01.000Z",
            "userEditable": true
        });
        sqlx::query("UPDATE persisted_artifacts SET payload = ? WHERE id = ?")
            .bind(canonical_json(&evidence).unwrap())
            .bind(EVIDENCE_ID)
            .execute(&mut connection)
            .await
            .unwrap();
        if ordinary_dependents {
            let reflection = json!({
                "createdAt": "2026-01-01T00:00:03.000Z",
                "id": "fixture-reflection-answered",
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
                "question": "What stayed?",
                "response": "My words",
                "responseProvenance": {
                    "generatedAt": "2026-01-01T00:00:04.000Z",
                    "origin": "user",
                    "sourceArtifactIds": ["fixture-reflection-answered"],
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
                 VALUES ('fixture-reflection-answered', ?, 'reflection', ?, \
                   '2026-01-01T00:00:03.000Z', '2026-01-01T00:00:04.000Z')",
            )
            .bind(SOURCE_ID)
            .bind(canonical_json(&reflection).unwrap())
            .execute(&mut connection)
            .await
            .unwrap();
            let pattern = json!({
                "createdAt": "2026-01-01T00:00:06.000Z",
                "id": "fixture-pattern-confirmed",
                "provenance": {
                    "generatedAt": "2026-01-01T00:00:06.000Z",
                    "harnessVersion": "harness-v1",
                    "model": "fixture-model",
                    "origin": "ai",
                    "promptVersion": "pattern-v1",
                    "provider": "gemini",
                    "sourceArtifactIds": [EVIDENCE_ID, "fixture-reflection-answered"],
                    "sourceEntryId": SOURCE_ID
                },
                "sourceEntryId": SOURCE_ID,
                "sourceEvidenceIds": [EVIDENCE_ID],
                "sourceReflectionPromptIds": ["fixture-reflection-answered"],
                "status": "confirmed",
                "text": "Tentative pattern",
                "updatedAt": "2026-01-01T00:00:06.000Z"
            });
            sqlx::query(
                "INSERT INTO persisted_artifacts \
                 (id, source_entry_id, artifact_kind, payload, created_at, updated_at) \
                 VALUES ('fixture-pattern-confirmed', ?, 'pattern', ?, \
                   '2026-01-01T00:00:06.000Z', '2026-01-01T00:00:06.000Z')",
            )
            .bind(SOURCE_ID)
            .bind(canonical_json(&pattern).unwrap())
            .execute(&mut connection)
            .await
            .unwrap();
        }
        if !historical {
            raw_sql("DELETE FROM historical_question_artifacts;")
                .execute(&mut connection)
                .await
                .unwrap();
        } else if ordinary_dependents {
            raw_sql(
                "INSERT INTO historical_consent_events VALUES ( \
                   'fixture-reflection-consent', \
                   'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb', \
                   '{}', 'consumed', '2026-01-03T00:01:01.000Z', '2026-02-02T00:01:01.000Z'); \
                 INSERT INTO historical_transmission_events VALUES ( \
                   'fixture-reflection-transmission', 'fixture-reflection-consent', \
                   'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb', \
                   'gemini', 'fixture-model', 'sent', \
                   '2026-01-03T00:01:02.000Z', '2026-02-02T00:01:02.000Z'); \
                 INSERT INTO historical_question_artifacts VALUES ( \
                   'fixture-reflection-question', 'fixture-v4-current', \
                   'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb', \
                   '{\"question\":\"What did my response add?\"}', '{}', \
                   'fixture-reflection-consent', 'fixture-reflection-transmission', \
                   '2026-01-03T00:01:03.000Z'); \
                 INSERT INTO historical_artifact_dependencies VALUES ( \
                   'fixture-reflection-question', 'fixture-v4-history', \
                   'fixture-reflection-answered', '2026-01-01T00:00:04.000Z');",
            )
            .execute(&mut connection)
            .await
            .unwrap();
        }
        let expected_source_manifest_digest = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        drop(connection);
        migrate_disposable_v4(MigrationRequest {
            path: &path,
            expected_source_manifest_digest,
            started_at: STARTED_AT,
            committed_at: COMMITTED_AT,
            backup_id: Some("slice4c1-disposable-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        (directory, path)
    }

    async fn exact_revisions(path: &Path) -> (String, String) {
        let mut connection = connect(path, true).await.unwrap();
        let source =
            sqlx::query_scalar("SELECT current_revision_id FROM source_heads WHERE id = ?")
                .bind(SOURCE_ID)
                .fetch_one(&mut connection)
                .await
                .unwrap();
        let evidence =
            sqlx::query_scalar("SELECT current_revision_id FROM artifact_heads WHERE id = ?")
                .bind(EVIDENCE_ID)
                .fetch_one(&mut connection)
                .await
                .unwrap();
        (source, evidence)
    }

    fn context(
        occurred_at: &'static str,
        failure_point: EvidenceLifecycleFailurePoint,
    ) -> EvidenceLifecycleContext<'static> {
        EvidenceLifecycleContext {
            occurred_at,
            guard_token: "guard-evidence-lifecycle-slice4c1-000000000001",
            failure_point,
        }
    }

    fn correct(source: String, evidence: String) -> EvidenceLifecycleCommand {
        EvidenceLifecycleCommand::CorrectConfirmed {
            source_id: SOURCE_ID.into(),
            artifact_id: EVIDENCE_ID.into(),
            expected_source_revision_id: source,
            expected_artifact_revision_id: evidence,
            text: "corrected by the user".into(),
        }
    }

    fn delete(source: String, evidence: String) -> EvidenceLifecycleCommand {
        EvidenceLifecycleCommand::DeleteConfirmed {
            source_id: SOURCE_ID.into(),
            artifact_id: EVIDENCE_ID.into(),
            expected_source_revision_id: source,
            expected_artifact_revision_id: evidence,
        }
    }

    async fn scalar(path: &Path, query: &str) -> i64 {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar(query)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn confirmed_correction_returns_pending_without_rebinding_or_history_loss() {
        let (_directory, path) = exact_v5_fixture(false, false).await;
        let (source, old_revision) = exact_revisions(&path).await;
        let outcome = execute_disposable(
            &path,
            correct(source, old_revision.clone()),
            context(CORRECTED_AT, EvidenceLifecycleFailurePoint::None),
        )
        .await
        .unwrap();
        let new_revision = outcome.revision_id.unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let head: (String, String, String, String) = sqlx::query_as(
            "SELECT current_revision_id, review_state, lifecycle_state, eligibility_state \
             FROM artifact_heads WHERE id = ?",
        )
        .bind(EVIDENCE_ID)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            head,
            (
                new_revision.clone(),
                "pending".into(),
                "active".into(),
                "ineligible".into()
            )
        );
        let history: (i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id = ?), \
               (SELECT COUNT(*) FROM artifact_revision_content c JOIN artifact_revisions r ON r.id=c.revision_id WHERE r.artifact_id = ?), \
               (SELECT COUNT(*) FROM artifact_revisions WHERE id = ? AND predecessor_revision_id = ? AND authorship = 'user' AND revision_reason = 'corrected'), \
               (SELECT COUNT(*) FROM artifact_review_events WHERE artifact_id = ? AND subject_revision_id = ? AND decision = 'confirmed')",
        )
        .bind(EVIDENCE_ID)
        .bind(EVIDENCE_ID)
        .bind(&new_revision)
        .bind(&old_revision)
        .bind(EVIDENCE_ID)
        .bind(&new_revision)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(history, (2, 2, 1, 0));
        let projection_status: String = sqlx::query_scalar(
            "SELECT json_extract(payload, '$.status') FROM persisted_artifacts WHERE id = ?",
        )
        .bind(EVIDENCE_ID)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(projection_status, "candidate");
    }

    #[tokio::test]
    async fn mixed_dependents_are_invalidated_or_cascade_deleted_in_one_correction() {
        let (_directory, path) = exact_v5_fixture(true, true).await;
        let (source, old_revision) = exact_revisions(&path).await;
        let outcome = execute_disposable(
            &path,
            correct(source, old_revision),
            context(CORRECTED_AT, EvidenceLifecycleFailurePoint::None),
        )
        .await
        .unwrap();
        assert_eq!(
            outcome.invalidated_artifact_ids,
            vec![
                "fixture-pattern-confirmed".to_string(),
                "fixture-reflection-answered".to_string()
            ]
        );
        assert_eq!(
            outcome.deleted_historical_artifact_ids,
            vec![
                "fixture-reflection-question".to_string(),
                "fixture-v4-question".to_string()
            ]
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_heads WHERE id IN ('fixture-reflection-answered','fixture-pattern-confirmed') AND lifecycle_state='invalidated' AND eligibility_state='ineligible'").await,
            2
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM persisted_artifacts WHERE id IN ('fixture-reflection-answered','fixture-pattern-confirmed')").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_lifecycle_events WHERE event_type='invalidated' AND artifact_id='fixture-pattern-confirmed'").await,
            2
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM historical_question_artifacts WHERE id='fixture-v4-question'"
            )
            .await,
            0
        );
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM historical_consent_events WHERE id='fixture-v4-consent'"
            )
            .await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM historical_transmission_events WHERE id='fixture-v4-transmission'").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM historical_question_artifacts WHERE id='fixture-reflection-question'").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM historical_consent_events WHERE id='fixture-reflection-consent'").await,
            0
        );
        let new_revision = outcome.revision_id.unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let rebound: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM artifact_dependencies WHERE source_artifact_id = ? \
             AND source_artifact_revision_id = ? AND dependent_artifact_id IN \
             ('fixture-reflection-answered','fixture-pattern-confirmed')",
        )
        .bind(EVIDENCE_ID)
        .bind(new_revision)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(rebound, 0);
    }

    #[tokio::test]
    async fn explicit_deletion_purges_evidence_content_and_retains_only_content_free_tombstone() {
        let (_directory, path) = exact_v5_fixture(true, true).await;
        let (source, revision) = exact_revisions(&path).await;
        let outcome = execute_disposable(
            &path,
            delete(source, revision),
            context(DELETED_AT, EvidenceLifecycleFailurePoint::None),
        )
        .await
        .unwrap();
        assert!(outcome.revision_id.is_none());
        let mut connection = connect(&path, true).await.unwrap();
        let head: (Option<String>, String, String, String) = sqlx::query_as(
            "SELECT current_revision_id, review_state, lifecycle_state, eligibility_state \
             FROM artifact_heads WHERE id = ?",
        )
        .bind(EVIDENCE_ID)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            head,
            (
                None,
                "confirmed".into(),
                "deleted".into(),
                "ineligible".into()
            )
        );
        let retained: (i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id = ?), \
               (SELECT COUNT(*) FROM artifact_revision_content c JOIN artifact_revisions r ON r.id=c.revision_id WHERE r.artifact_id = ?), \
               (SELECT COUNT(*) FROM content_tombstones WHERE artifact_id = ? AND subject_type='artifact' AND content_digest IS NULL AND reason_code='user_deleted_artifact'), \
               (SELECT COUNT(*) FROM persisted_artifacts WHERE id = ?)",
        )
        .bind(EVIDENCE_ID)
        .bind(EVIDENCE_ID)
        .bind(EVIDENCE_ID)
        .bind(EVIDENCE_ID)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(retained, (1, 0, 1, 0));
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM artifact_revision_provenance rp JOIN artifact_revisions r ON r.id=rp.artifact_revision_id WHERE r.artifact_id = ?")
                .bind(EVIDENCE_ID)
                .fetch_one(&mut connection)
                .await
                .unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn stale_source_or_evidence_and_incomplete_historical_parity_fail_without_mutation() {
        let (_directory, stale_path) = exact_v5_fixture(false, false).await;
        let (source, _revision) = exact_revisions(&stale_path).await;
        let before = {
            let mut connection = connect(&stale_path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_disposable(
            &stale_path,
            correct(source, "stale-revision".into()),
            context(CORRECTED_AT, EvidenceLifecycleFailurePoint::None),
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("revision_stale"));
        let mut connection = connect(&stale_path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        drop(connection);

        let (_directory, parity_path) = exact_v5_fixture(false, true).await;
        let (source, revision) = exact_revisions(&parity_path).await;
        let mut connection = connect(&parity_path, false).await.unwrap();
        raw_sql("BEGIN IMMEDIATE")
            .execute(&mut connection)
            .await
            .unwrap();
        sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES ('guard-parity-corruption-0000000000001','2026-08-02T01:30:00.000Z')")
            .execute(&mut connection).await.unwrap();
        sqlx::query("DELETE FROM historical_question_lifecycle_links WHERE historical_artifact_id='fixture-v4-question'")
            .execute(&mut connection).await.unwrap();
        sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token='guard-parity-corruption-0000000000001'")
            .execute(&mut connection).await.unwrap();
        raw_sql("COMMIT").execute(&mut connection).await.unwrap();
        drop(connection);
        let error = execute_disposable(
            &parity_path,
            delete(source, revision),
            context(DELETED_AT, EvidenceLifecycleFailurePoint::None),
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("historical_links_inconsistent"));
    }

    #[tokio::test]
    async fn every_correction_boundary_rolls_back_to_the_exact_logical_prestate() {
        let points = [
            EvidenceLifecycleFailurePoint::AfterGuard,
            EvidenceLifecycleFailurePoint::AfterEvidenceRevision,
            EvidenceLifecycleFailurePoint::AfterEvidenceLifecycle,
            EvidenceLifecycleFailurePoint::AfterEvidenceHead,
            EvidenceLifecycleFailurePoint::AfterOrdinaryInvalidation,
            EvidenceLifecycleFailurePoint::AfterProjection,
            EvidenceLifecycleFailurePoint::AfterHistoricalCascade,
            EvidenceLifecycleFailurePoint::AfterReconciliation,
            EvidenceLifecycleFailurePoint::AfterGuardRemoval,
        ];
        for point in points {
            let (_directory, path) = exact_v5_fixture(true, true).await;
            let (source, revision) = exact_revisions(&path).await;
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            assert!(execute_disposable(
                &path,
                correct(source, revision),
                context(CORRECTED_AT, point),
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
    async fn every_deletion_boundary_rolls_back_to_the_exact_logical_prestate() {
        let points = [
            EvidenceLifecycleFailurePoint::AfterGuard,
            EvidenceLifecycleFailurePoint::AfterEvidenceLifecycle,
            EvidenceLifecycleFailurePoint::AfterEvidenceHead,
            EvidenceLifecycleFailurePoint::AfterEvidenceContentPurge,
            EvidenceLifecycleFailurePoint::AfterOrdinaryInvalidation,
            EvidenceLifecycleFailurePoint::AfterProjection,
            EvidenceLifecycleFailurePoint::AfterHistoricalCascade,
            EvidenceLifecycleFailurePoint::AfterReconciliation,
            EvidenceLifecycleFailurePoint::AfterGuardRemoval,
        ];
        for point in points {
            let (_directory, path) = exact_v5_fixture(true, true).await;
            let (source, revision) = exact_revisions(&path).await;
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            assert!(execute_disposable(
                &path,
                delete(source, revision),
                context(DELETED_AT, point),
            )
            .await
            .is_err());
            let mut connection = connect(&path, true).await.unwrap();
            assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
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
    async fn ambiguous_commit_is_classified_from_exact_read_only_manifests_without_retry() {
        let (_directory, committed_path) = exact_v5_fixture(true, true).await;
        let (source, revision) = exact_revisions(&committed_path).await;
        let committed = execute_with_adapter(
            &committed_path,
            correct(source, revision),
            context(CORRECTED_AT, EvidenceLifecycleFailurePoint::None),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_after_commit".into(),
                },
                commit_first: true,
            },
        )
        .await
        .unwrap();
        assert_eq!(committed.status, EvidenceLifecycleStatus::Committed);

        let (_directory, unchanged_path) = exact_v5_fixture(true, true).await;
        let (source, revision) = exact_revisions(&unchanged_path).await;
        let before = {
            let mut connection = connect(&unchanged_path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_with_adapter(
            &unchanged_path,
            delete(source, revision),
            context(DELETED_AT, EvidenceLifecycleFailurePoint::None),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_without_commit".into(),
                },
                commit_first: false,
            },
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("outcome_unknown_unchanged"));
        let mut connection = connect(&unchanged_path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
    }
}
