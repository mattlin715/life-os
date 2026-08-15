use super::context_recovery_write::verify_exact_context_recovery_v5;
use super::pattern_write::verify_exact_pattern_v5;
use super::*;
use sqlx::{raw_sql, Row};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExperienceWriteInput {
    pub(crate) id: String,
    pub(crate) content: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ExperienceWriteCommand {
    Create(ExperienceWriteInput),
    Update {
        id: String,
        expected_revision_id: String,
        content: String,
        updated_at: String,
    },
    Delete {
        id: String,
        expected_revision_id: String,
    },
    Import(Vec<ExperienceWriteInput>),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) enum ExperienceWriteFailurePoint {
    #[default]
    None,
    AfterGuard,
    AfterProvenance,
    AfterRevision,
    AfterContent,
    AfterHead,
    AfterOrdinaryConsequences,
    AfterHistoricalParity,
    AfterHistoricalCascade,
    AfterArtifactCascade,
    AfterProjection,
    AfterReconciliation,
    AfterGuardRemoval,
}

#[derive(Clone, Debug)]
pub(super) struct ExperienceWriteContext<'a> {
    pub(super) occurred_at: &'a str,
    pub(super) guard_token: &'a str,
    pub(super) failure_point: ExperienceWriteFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ExperienceWriteStatus {
    Committed,
    StaleRevision,
    NotFound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExperienceWriteOutcome {
    pub(super) status: ExperienceWriteStatus,
    pub(super) revision_id: Option<String>,
    pub(super) imported_count: u64,
    pub(super) skipped_count: u64,
    pub(super) operation_manifest: String,
}

#[derive(Clone, Debug)]
struct CurrentSource {
    revision_id: String,
    revision_number: i64,
    content: String,
    created_at: String,
    updated_at: String,
}

struct PreparedWrite {
    outcome: ExperienceWriteOutcome,
    post_manifest: String,
}

#[derive(Clone, Debug)]
struct OrdinaryDependent {
    artifact_id: String,
    revision_id: String,
    artifact_kind: String,
    lifecycle_state: String,
    eligibility_state: String,
    dependency_id: String,
}

enum PreparedWriteResult {
    Write(PreparedWrite),
    NoChange(ExperienceWriteOutcome),
}

fn write_error(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn recovery_error(code: impl Into<String>) -> MigrationError {
    MigrationError::recovery_required(code)
}

fn validate_identifier(value: &str, field: &str) -> Result<(), MigrationError> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(write_error(format!("experience_{field}_invalid")));
    }
    Ok(())
}

fn validate_content(value: &str) -> Result<(), MigrationError> {
    if value.is_empty() {
        return Err(write_error("experience_content_invalid"));
    }
    Ok(())
}

fn validate_timestamp(value: &str, field: &str) -> Result<(), MigrationError> {
    let bytes = value.as_bytes();
    let canonical_shape = bytes.len() == 24
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'.'
        && bytes[23] == b'Z'
        && bytes.iter().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19 | 23) || byte.is_ascii_digit()
        });
    if !canonical_shape {
        return Err(write_error(format!("experience_{field}_invalid")));
    }

    let parse = |start: usize, end: usize| {
        value[start..end]
            .parse::<u32>()
            .map_err(|_| write_error(format!("experience_{field}_invalid")))
    };
    let year = parse(0, 4)?;
    let month = parse(5, 7)?;
    let day = parse(8, 10)?;
    let hour = parse(11, 13)?;
    let minute = parse(14, 16)?;
    let second = parse(17, 19)?;
    let leap_year =
        year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => 0,
    };
    if year == 0 || day == 0 || day > max_day || hour > 23 || minute > 59 || second > 59 {
        return Err(write_error(format!("experience_{field}_invalid")));
    }
    Ok(())
}

fn validate_input(input: &ExperienceWriteInput) -> Result<(), MigrationError> {
    validate_identifier(&input.id, "id")?;
    validate_content(&input.content)?;
    validate_timestamp(&input.created_at, "created_at")?;
    validate_timestamp(&input.updated_at, "updated_at")?;
    if input.updated_at < input.created_at {
        return Err(write_error("experience_timestamp_order_invalid"));
    }
    Ok(())
}

fn inject(
    actual: ExperienceWriteFailurePoint,
    expected: ExperienceWriteFailurePoint,
) -> Result<(), MigrationError> {
    if actual == expected {
        Err(write_error(format!(
            "injected_experience_write_failure:{expected:?}"
        )))
    } else {
        Ok(())
    }
}

pub(super) async fn operation_manifest(
    connection: &mut SqliteConnection,
) -> Result<String, MigrationError> {
    let source = manifest(connection, &SOURCE_TABLE_MANIFESTS).await?;
    let target = manifest(connection, &TARGET_TABLE_MANIFESTS).await?;
    Ok(sha256_hex(
        format!("life-os/experience-write-manifest-v1\0{source}\0{target}").as_bytes(),
    ))
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

async fn verify_receipt_and_contract(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    let receipts: Vec<(i64, i64, String, String, String, String)> = sqlx::query_as(
        "SELECT from_version, to_version, state, application_version, \
         source_manifest_digest, target_manifest_digest \
         FROM schema_migration_receipts",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_receipt_unreadable", error))?;
    if receipts.len() != 1 {
        return Err(recovery_error("experience_write_receipt_count_mismatch"));
    }
    let receipt = &receipts[0];
    if receipt.0 != SOURCE_SCHEMA_VERSION
        || receipt.1 != TARGET_SCHEMA_VERSION
        || receipt.2 != "committed"
        || receipt.3 != APPLICATION_VERSION
        || !valid_sha256(&receipt.4)
        || !valid_sha256(&receipt.5)
    {
        return Err(recovery_error("experience_write_receipt_mismatch"));
    }

    let contracts: Vec<(i64, String, String, String, String)> = sqlx::query_as(
        "SELECT authoritative_schema, minimum_application_version, \
         compatibility_projection, lifecycle_writes, export_v2 \
         FROM database_contract",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_contract_unreadable", error))?;
    if contracts.len() != 1
        || contracts[0].0 != TARGET_SCHEMA_VERSION
        || contracts[0].1 != APPLICATION_VERSION
        || contracts[0].2 != "enabled"
        || !matches!(contracts[0].3.as_str(), "disabled" | "enabled")
        || contracts[0].4 != "disabled"
    {
        return Err(recovery_error("experience_write_contract_mismatch"));
    }
    Ok(())
}

async fn verify_source_projection(connection: &mut SqliteConnection) -> Result<(), MigrationError> {
    let heads = sqlx::query(
        "SELECT id, current_revision_id, lifecycle_state, created_at, updated_at \
         FROM source_heads ORDER BY id",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_source_heads_unreadable", error))?;
    let mut active_count = 0_i64;

    for head in heads {
        let id: String = head.get("id");
        let current_revision_id: Option<String> = head.get("current_revision_id");
        let lifecycle_state: String = head.get("lifecycle_state");
        let created_at: String = head.get("created_at");
        let updated_at: String = head.get("updated_at");
        let projection: Option<(String, String, String)> = sqlx::query_as(
            "SELECT content, created_at, updated_at FROM experience_entries WHERE id = ?",
        )
        .bind(&id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_write_projection_unreadable", error))?;

        match lifecycle_state.as_str() {
            "active" => {
                active_count += 1;
                let revision_id = current_revision_id
                    .ok_or_else(|| recovery_error("active_source_revision_missing"))?;
                let revision: Option<(String, String, i64)> = sqlx::query_as(
                    "SELECT r.content_digest, c.content, c.byte_length \
                     FROM source_revisions r \
                     JOIN source_revision_content c ON c.revision_id = r.id \
                     WHERE r.id = ? AND r.source_id = ?",
                )
                .bind(&revision_id)
                .bind(&id)
                .fetch_optional(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_write_current_revision_unreadable", error)
                })?;
                let (content_digest, content, byte_length) =
                    revision.ok_or_else(|| recovery_error("active_source_content_missing"))?;
                if content_digest != sha256_hex(content.as_bytes())
                    || byte_length != content.len() as i64
                    || projection != Some((content.clone(), created_at.clone(), updated_at.clone()))
                {
                    return Err(recovery_error(
                        "experience_write_source_projection_mismatch",
                    ));
                }
            }
            "deleted" => {
                if current_revision_id.is_some() || projection.is_some() {
                    return Err(recovery_error(
                        "experience_write_deleted_source_projection_mismatch",
                    ));
                }
            }
            _ => return Err(recovery_error("experience_write_source_lifecycle_unknown")),
        }
    }

    let projection_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM experience_entries")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_write_projection_count_failed", error))?;
    if projection_count != active_count {
        return Err(recovery_error("experience_write_projection_count_mismatch"));
    }
    Ok(())
}

pub(super) async fn verify_exact_v5(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    if user_version(connection).await? != TARGET_SCHEMA_VERSION {
        return Err(write_error("experience_write_requires_exact_v5"));
    }
    if !is_expected_schema_object_manifest(&schema_object_manifest(connection).await?) {
        return Err(recovery_error("experience_write_schema_manifest_mismatch"));
    }
    verify_receipt_and_contract(connection).await?;
    let guard_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_write_guard_unreadable", error))?;
    if guard_count != 0 {
        return Err(recovery_error("experience_write_guard_not_empty"));
    }
    current_content_checks(connection).await?;
    verify_source_projection(connection).await?;
    integrity_checks(connection).await
}

async fn current_source(
    connection: &mut SqliteConnection,
    id: &str,
) -> Result<Option<CurrentSource>, MigrationError> {
    let row = sqlx::query(
        "SELECT h.current_revision_id, r.revision_number, c.content, \
         h.created_at, h.updated_at \
         FROM source_heads h \
         JOIN source_revisions r ON r.id = h.current_revision_id \
         JOIN source_revision_content c ON c.revision_id = r.id \
         WHERE h.id = ? AND h.lifecycle_state = 'active'",
    )
    .bind(id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_current_source_unreadable", error))?;
    Ok(row.map(|row| CurrentSource {
        revision_id: row.get("current_revision_id"),
        revision_number: row.get("revision_number"),
        content: row.get("content"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }))
}

async fn assert_no_external_ordinary_dependents(
    connection: &mut SqliteConnection,
    source_id: &str,
) -> Result<(), MigrationError> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) \
         FROM artifact_dependencies d \
         JOIN source_revisions sr ON sr.id = d.source_revision_id \
         JOIN artifact_heads dependent ON dependent.id = d.dependent_artifact_id \
         WHERE sr.source_id = ? \
           AND dependent.source_id <> ? \
           AND dependent.artifact_kind <> 'historical_question'",
    )
    .bind(source_id)
    .bind(source_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_external_dependency_check_failed", error))?;
    if count != 0 {
        return Err(write_error(
            "external_ordinary_dependency_requires_later_slice",
        ));
    }
    Ok(())
}

pub(super) async fn verify_source_caused_invalidation(
    connection: &mut SqliteConnection,
    artifact_id: &str,
    revision_id: &str,
    source_id: &str,
    head_updated_at: &str,
) -> Result<bool, MigrationError> {
    let rows: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT e.id, d.id, e.occurred_at \
         FROM artifact_lifecycle_events e \
         JOIN artifact_dependencies d ON d.id = e.dependency_id \
         JOIN source_revisions sr ON sr.id = d.source_revision_id \
         JOIN source_heads sh ON sh.id = sr.source_id \
         WHERE e.artifact_id = ? AND e.subject_revision_id = ? \
           AND e.event_type = 'invalidated' AND e.actor = 'system' \
           AND e.reason_code = 'source_experience_revision_superseded' \
           AND d.dependent_artifact_id = ? AND d.dependent_revision_id = ? \
           AND d.relationship_type = 'derived_from_experience' \
           AND d.source_artifact_id IS NULL \
           AND d.source_artifact_revision_id IS NULL \
           AND sr.source_id = ? AND sh.id = ? AND sh.lifecycle_state = 'active' \
           AND sh.current_revision_id <> d.source_revision_id \
         ORDER BY e.id",
    )
    .bind(artifact_id)
    .bind(revision_id)
    .bind(artifact_id)
    .bind(revision_id)
    .bind(source_id)
    .bind(source_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("source_invalidation_facts_unreadable", error))?;
    if rows.is_empty() {
        return Ok(false);
    }
    if rows.len() != 1 {
        return Err(recovery_error("source_invalidation_event_duplicate"));
    }
    let (actual_event_id, dependency_id, occurred_at) = &rows[0];
    let domain =
        format!("life-os/experience-source-dependent-invalidated-event-id-v1:{dependency_id}");
    let expected_event_id = event_id("v5le_", &domain, artifact_id, revision_id);
    if actual_event_id != &expected_event_id || occurred_at != head_updated_at {
        return Err(recovery_error("source_invalidation_event_mismatch"));
    }
    let source_dependencies: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_dependencies \
         WHERE dependent_artifact_id = ? AND dependent_revision_id = ? \
           AND relationship_type = 'derived_from_experience' \
           AND source_revision_id IS NOT NULL \
           AND source_artifact_id IS NULL \
           AND source_artifact_revision_id IS NULL",
    )
    .bind(artifact_id)
    .bind(revision_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("source_invalidation_dependency_count_failed", error))?;
    if source_dependencies != 1 {
        return Err(recovery_error("source_invalidation_dependency_mismatch"));
    }
    Ok(true)
}

async fn collect_ordinary_dependents(
    connection: &mut SqliteConnection,
    source_id: &str,
    expected_source_revision_id: &str,
) -> Result<Vec<OrdinaryDependent>, MigrationError> {
    assert_no_external_ordinary_dependents(connection, source_id).await?;
    let rows = sqlx::query(
        "SELECT h.id, h.current_revision_id, h.artifact_kind, h.lifecycle_state, \
                h.eligibility_state, d.id, d.source_revision_id \
         FROM artifact_heads h \
         LEFT JOIN artifact_dependencies d \
           ON d.dependent_artifact_id = h.id \
          AND d.dependent_revision_id = h.current_revision_id \
          AND d.relationship_type = 'derived_from_experience' \
         WHERE h.source_id = ? AND h.artifact_kind <> 'historical_question' \
         ORDER BY h.id, d.id",
    )
    .bind(source_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_dependents_unreadable", error))?;
    let mut dependents = BTreeMap::new();
    for row in rows {
        let artifact_id: String = row.get(0);
        let revision_id: Option<String> = row.get(1);
        let artifact_kind: String = row.get(2);
        let lifecycle_state: String = row.get(3);
        let eligibility_state: String = row.get(4);
        let dependency_id: Option<String> = row.get(5);
        let source_revision_id: Option<String> = row.get(6);
        for (value, field) in [
            (artifact_id.as_str(), "dependent_artifact_id"),
            (artifact_kind.as_str(), "dependent_artifact_kind"),
        ] {
            validate_identifier(value, field)?;
        }
        if !matches!(
            artifact_kind.as_str(),
            "evidence" | "reflection" | "pattern" | "recovery_turn"
        ) {
            return Err(write_error("experience_dependent_kind_unsupported"));
        }
        match lifecycle_state.as_str() {
            "active" | "invalidated" => {
                let revision_id = revision_id
                    .ok_or_else(|| write_error("experience_dependent_revision_missing"))?;
                let dependency_id = dependency_id
                    .ok_or_else(|| write_error("experience_source_dependency_missing"))?;
                let source_revision_id = source_revision_id
                    .ok_or_else(|| write_error("experience_source_dependency_missing"))?;
                for (value, field) in [
                    (revision_id.as_str(), "dependent_revision_id"),
                    (dependency_id.as_str(), "source_dependency_id"),
                    (source_revision_id.as_str(), "source_revision_id"),
                ] {
                    validate_identifier(value, field)?;
                }
                if lifecycle_state == "active" && source_revision_id != expected_source_revision_id
                {
                    return Err(write_error("experience_source_dependency_stale"));
                }
                let dependent = OrdinaryDependent {
                    artifact_id: artifact_id.clone(),
                    revision_id,
                    artifact_kind,
                    lifecycle_state,
                    eligibility_state,
                    dependency_id,
                };
                if dependents.insert(artifact_id, dependent).is_some() {
                    return Err(write_error("experience_source_dependency_duplicate"));
                }
            }
            "content_purged" | "deleted" => {
                if revision_id.is_some() || dependency_id.is_some() || source_revision_id.is_some()
                {
                    return Err(write_error("experience_terminal_dependency_contradictory"));
                }
                let dependent = OrdinaryDependent {
                    artifact_id: artifact_id.clone(),
                    revision_id: String::new(),
                    artifact_kind,
                    lifecycle_state,
                    eligibility_state,
                    dependency_id: String::new(),
                };
                if dependents.insert(artifact_id, dependent).is_some() {
                    return Err(write_error("experience_terminal_artifact_duplicate"));
                }
            }
            _ => return Err(write_error("experience_dependent_lifecycle_unsupported")),
        }
    }
    Ok(dependents.into_values().collect())
}

async fn append_source_invalidation_event(
    connection: &mut SqliteConnection,
    dependent: &OrdinaryDependent,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let existing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_lifecycle_events \
         WHERE artifact_id = ? AND subject_revision_id = ? \
           AND dependency_id = ? AND event_type = 'invalidated' \
           AND actor = 'system' \
           AND reason_code = 'source_experience_revision_superseded'",
    )
    .bind(&dependent.artifact_id)
    .bind(&dependent.revision_id)
    .bind(&dependent.dependency_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_invalidation_lookup_failed", error))?;
    if existing != 0 {
        return Err(recovery_error("experience_invalidation_event_preexisting"));
    }
    let domain = format!(
        "life-os/experience-source-dependent-invalidated-event-id-v1:{}",
        dependent.dependency_id
    );
    let id = event_id(
        "v5le_",
        &domain,
        &dependent.artifact_id,
        &dependent.revision_id,
    );
    sqlx::query(
        "INSERT INTO artifact_lifecycle_events (id, artifact_id, subject_revision_id, \
           related_revision_id, dependency_id, event_type, actor, reason_code, occurred_at) \
         VALUES (?, ?, ?, NULL, ?, 'invalidated', 'system', \
           'source_experience_revision_superseded', ?)",
    )
    .bind(id)
    .bind(&dependent.artifact_id)
    .bind(&dependent.revision_id)
    .bind(&dependent.dependency_id)
    .bind(occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_invalidation_insert_failed", error))?;
    Ok(())
}

async fn invalidate_ordinary_dependents(
    connection: &mut SqliteConnection,
    dependents: &[OrdinaryDependent],
    context: &ExperienceWriteContext<'_>,
) -> Result<Vec<String>, MigrationError> {
    let mut invalidated = Vec::new();
    for dependent in dependents {
        match dependent.lifecycle_state.as_str() {
            "active" => {
                append_source_invalidation_event(connection, dependent, context.occurred_at)
                    .await?;
                let updated = sqlx::query(
                    "UPDATE artifact_heads SET lifecycle_state = 'invalidated', \
                       eligibility_state = 'ineligible', \
                       eligibility_reason = 'source_experience_revision_superseded', \
                       updated_at = ? \
                     WHERE id = ? AND current_revision_id = ? AND artifact_kind = ? \
                       AND lifecycle_state = 'active'",
                )
                .bind(context.occurred_at)
                .bind(&dependent.artifact_id)
                .bind(&dependent.revision_id)
                .bind(&dependent.artifact_kind)
                .execute(&mut *connection)
                .await
                .map_err(|error| migration_error("experience_dependent_update_failed", error))?;
                if updated.rows_affected() != 1 {
                    return Err(write_error("experience_dependent_revision_stale"));
                }
                let deleted = sqlx::query(
                    "DELETE FROM persisted_artifacts \
                     WHERE id = ? AND artifact_kind = ?",
                )
                .bind(&dependent.artifact_id)
                .bind(&dependent.artifact_kind)
                .execute(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_dependent_projection_delete_failed", error)
                })?;
                if deleted.rows_affected() != 1 {
                    return Err(recovery_error(
                        "experience_dependent_projection_delete_mismatch",
                    ));
                }
                invalidated.push(dependent.artifact_id.clone());
            }
            "invalidated" => {
                if dependent.eligibility_state != "ineligible" {
                    return Err(write_error("experience_invalidated_eligibility_mismatch"));
                }
                let projection: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM persisted_artifacts WHERE id = ? AND artifact_kind = ?",
                )
                .bind(&dependent.artifact_id)
                .bind(&dependent.artifact_kind)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_invalidated_projection_unreadable", error)
                })?;
                if projection != 0 {
                    return Err(recovery_error("experience_invalidated_projection_present"));
                }
            }
            "content_purged" | "deleted" => {
                if dependent.eligibility_state != "ineligible" {
                    return Err(write_error("experience_terminal_eligibility_mismatch"));
                }
            }
            _ => return Err(write_error("experience_dependent_lifecycle_unsupported")),
        }
    }
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterOrdinaryConsequences,
    )?;
    Ok(invalidated)
}

async fn insert_guard(
    connection: &mut SqliteConnection,
    context: &ExperienceWriteContext<'_>,
) -> Result<(), MigrationError> {
    validate_identifier(context.guard_token, "guard_token")?;
    if context.guard_token.len() < 32 {
        return Err(write_error("experience_guard_token_invalid"));
    }
    validate_timestamp(context.occurred_at, "occurred_at")?;
    sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
        .bind(context.guard_token)
        .bind(context.occurred_at)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_write_guard_insert_failed", error))?;
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterGuard,
    )
}

async fn remove_guard(
    connection: &mut SqliteConnection,
    context: &ExperienceWriteContext<'_>,
) -> Result<(), MigrationError> {
    let deleted = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
        .bind(context.guard_token)
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_write_guard_remove_failed", error))?;
    if deleted.rows_affected() != 1 {
        return Err(recovery_error("experience_write_guard_identity_mismatch"));
    }
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_write_guard_count_failed", error))?;
    if count != 0 {
        return Err(recovery_error("experience_write_guard_not_empty"));
    }
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterGuardRemoval,
    )
}

async fn insert_source_revision(
    connection: &mut SqliteConnection,
    input: &ExperienceWriteInput,
    revision_number: i64,
    predecessor_revision_id: Option<&str>,
    revision_reason: &str,
    serialization_version: &str,
    context: &ExperienceWriteContext<'_>,
) -> Result<String, MigrationError> {
    validate_input(input)?;
    let digest = sha256_hex(input.content.as_bytes());
    let revision_id = source_revision_id(&input.id, &input.updated_at, &digest);
    sqlx::query(
        "INSERT INTO source_revisions (\
           id, source_id, revision_number, predecessor_revision_id, authorship,\
           revision_reason, serialization_version, content_digest, created_at\
         ) VALUES (?, ?, ?, ?, 'user', ?, ?, ?, ?)",
    )
    .bind(&revision_id)
    .bind(&input.id)
    .bind(revision_number)
    .bind(predecessor_revision_id)
    .bind(revision_reason)
    .bind(serialization_version)
    .bind(&digest)
    .bind(context.occurred_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_revision_insert_failed", error))?;
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterRevision,
    )?;
    sqlx::query(
        "INSERT INTO source_revision_content (revision_id, content, byte_length) VALUES (?, ?, ?)",
    )
    .bind(&revision_id)
    .bind(&input.content)
    .bind(input.content.len() as i64)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_content_insert_failed", error))?;
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterContent,
    )?;
    Ok(revision_id)
}

async fn link_user_provenance(
    connection: &mut SqliteConnection,
    source_id: &str,
    revision_id: &str,
    context: &ExperienceWriteContext<'_>,
) -> Result<(), MigrationError> {
    let provenance = normalized_provenance(None, source_id, "user");
    let provenance_id = insert_provenance(connection, &provenance, context.occurred_at).await?;
    sqlx::query(
        "INSERT INTO source_revision_provenance \
         (source_revision_id, role, provenance_id) VALUES (?, 'content', ?)",
    )
    .bind(revision_id)
    .bind(provenance_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_provenance_link_failed", error))?;
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterProvenance,
    )
}

async fn create_source(
    connection: &mut SqliteConnection,
    input: &ExperienceWriteInput,
    context: &ExperienceWriteContext<'_>,
    revision_reason: &str,
    serialization_version: &str,
) -> Result<String, MigrationError> {
    validate_input(input)?;
    let head_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM source_heads WHERE id = ?)")
            .bind(&input.id)
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| migration_error("experience_write_identity_check_failed", error))?;
    let projection_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM experience_entries WHERE id = ?)")
            .bind(&input.id)
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("experience_write_projection_identity_check_failed", error)
            })?;
    if head_exists || projection_exists {
        return Err(write_error("experience_identity_already_exists"));
    }

    let revision_id = insert_source_revision(
        connection,
        input,
        1,
        None,
        revision_reason,
        serialization_version,
        context,
    )
    .await?;
    sqlx::query(
        "INSERT INTO source_heads \
         (id, current_revision_id, lifecycle_state, created_at, updated_at) \
         VALUES (?, ?, 'active', ?, ?)",
    )
    .bind(&input.id)
    .bind(&revision_id)
    .bind(&input.created_at)
    .bind(&input.updated_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_head_insert_failed", error))?;
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterHead,
    )?;
    link_user_provenance(connection, &input.id, &revision_id, context).await?;
    sqlx::query(
        "INSERT INTO experience_entries (id, content, created_at, updated_at) \
         VALUES (?, ?, ?, ?)",
    )
    .bind(&input.id)
    .bind(&input.content)
    .bind(&input.created_at)
    .bind(&input.updated_at)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_projection_insert_failed", error))?;
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterProjection,
    )?;
    Ok(revision_id)
}

async fn collect_historical_questions_for_source(
    connection: &mut SqliteConnection,
    source_id: &str,
    expected_source_revision_id: &str,
) -> Result<Vec<String>, MigrationError> {
    let v4_rows: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT q.id FROM historical_question_artifacts q \
         LEFT JOIN historical_artifact_dependencies d \
           ON d.historical_artifact_id = q.id \
         WHERE q.current_experience_id = ? OR d.source_entry_id = ? \
         ORDER BY q.id",
    )
    .bind(source_id)
    .bind(source_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_historical_v4_sources_unreadable", error))?;
    let normalized_rows: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT l.historical_artifact_id \
         FROM historical_question_lifecycle_links l \
         JOIN artifact_heads h ON h.id = l.artifact_id \
         JOIN artifact_dependencies d ON d.dependent_artifact_id = h.id \
         LEFT JOIN source_revisions sr ON sr.id = d.source_revision_id \
         LEFT JOIN artifact_heads source_artifact ON source_artifact.id = d.source_artifact_id \
         WHERE h.artifact_kind = 'historical_question' \
           AND (sr.source_id = ? OR source_artifact.source_id = ?) \
         ORDER BY l.historical_artifact_id",
    )
    .bind(source_id)
    .bind(source_id)
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| {
        migration_error("experience_historical_normalized_sources_unreadable", error)
    })?;
    let v4 = v4_rows.into_iter().collect::<BTreeSet<_>>();
    let normalized = normalized_rows.into_iter().collect::<BTreeSet<_>>();
    if v4 != normalized {
        return Err(write_error("experience_historical_representation_mismatch"));
    }

    for historical_id in &v4 {
        validate_identifier(historical_id, "historical_artifact_id")?;
        let representation: (i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_question_lifecycle_links \
                  WHERE historical_artifact_id = ?), \
               (SELECT COUNT(*) FROM historical_question_lifecycle_links l \
                  JOIN artifact_heads h ON h.id = l.artifact_id \
                  WHERE l.historical_artifact_id = ? \
                    AND h.artifact_kind = 'historical_question' \
                    AND h.lifecycle_state = 'active')",
        )
        .bind(historical_id)
        .bind(historical_id)
        .bind(historical_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| {
            migration_error("experience_historical_representation_unreadable", error)
        })?;
        if representation != (1, 1, 1) {
            return Err(write_error(
                "experience_historical_representation_incomplete",
            ));
        }

        let current_experience_id: String = sqlx::query_scalar(
            "SELECT current_experience_id FROM historical_question_artifacts WHERE id = ?",
        )
        .bind(historical_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_historical_current_unreadable", error))?;
        let normalized_current: Vec<String> = sqlx::query_scalar(
            "SELECT d.source_revision_id FROM historical_question_lifecycle_links l \
             JOIN artifact_dependencies d ON d.dependent_artifact_id = l.artifact_id \
             JOIN source_revisions sr ON sr.id = d.source_revision_id \
             WHERE l.historical_artifact_id = ? \
               AND d.relationship_type = 'historical_current_experience' \
               AND sr.source_id = ? ORDER BY d.id",
        )
        .bind(historical_id)
        .bind(source_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| {
            migration_error("experience_historical_current_dependency_unreadable", error)
        })?;
        if current_experience_id == source_id {
            if normalized_current.as_slice() != [expected_source_revision_id] {
                return Err(write_error(
                    "experience_historical_current_dependency_mismatch",
                ));
            }
        } else if !normalized_current.is_empty() {
            return Err(write_error(
                "experience_historical_current_dependency_unexpected",
            ));
        }

        let v4_items: Vec<(Option<String>, String)> = sqlx::query_as(
            "SELECT source_artifact_id, source_revision \
             FROM historical_artifact_dependencies \
             WHERE historical_artifact_id = ? AND source_entry_id = ? \
             ORDER BY source_artifact_id, source_revision",
        )
        .bind(historical_id)
        .bind(source_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_historical_v4_items_unreadable", error))?;
        let normalized_items: Vec<(Option<String>, String)> = sqlx::query_as(
            "SELECT d.source_artifact_id, \
                    CASE WHEN d.source_revision_id IS NOT NULL THEN sr.created_at \
                         ELSE pa.updated_at END \
             FROM historical_question_lifecycle_links l \
             JOIN artifact_dependencies d ON d.dependent_artifact_id = l.artifact_id \
             LEFT JOIN source_revisions sr ON sr.id = d.source_revision_id \
             LEFT JOIN artifact_heads source_artifact ON source_artifact.id = d.source_artifact_id \
             LEFT JOIN persisted_artifacts pa ON pa.id = d.source_artifact_id \
               AND pa.source_entry_id = source_artifact.source_id \
               AND pa.artifact_kind = source_artifact.artifact_kind \
             WHERE l.historical_artifact_id = ? \
               AND d.relationship_type = 'historical_packet_item' \
               AND (sr.source_id = ? OR source_artifact.source_id = ?) \
             ORDER BY d.source_artifact_id, \
                      CASE WHEN d.source_revision_id IS NOT NULL THEN sr.created_at \
                           ELSE pa.updated_at END",
        )
        .bind(historical_id)
        .bind(source_id)
        .bind(source_id)
        .fetch_all(&mut *connection)
        .await
        .map_err(|error| {
            migration_error("experience_historical_normalized_items_unreadable", error)
        })?;
        if v4_items != normalized_items {
            return Err(write_error(
                "experience_historical_item_dependency_mismatch",
            ));
        }
    }
    Ok(v4.into_iter().collect())
}

async fn delete_historical_questions(
    connection: &mut SqliteConnection,
    historical_ids: &[String],
) -> Result<(), MigrationError> {
    for historical_id in historical_ids {
        let identities: (String, String, String) = sqlx::query_as(
            "SELECT l.artifact_id, q.consent_id, q.transmission_id \
             FROM historical_question_lifecycle_links l \
             JOIN historical_question_artifacts q ON q.id = l.historical_artifact_id \
             WHERE l.historical_artifact_id = ?",
        )
        .bind(historical_id)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_historical_identity_unreadable", error))?;
        if identities.0 != *historical_id {
            return Err(write_error("experience_historical_identity_mismatch"));
        }
        let deleted = sqlx::query("DELETE FROM historical_question_artifacts WHERE id = ?")
            .bind(historical_id)
            .execute(&mut *connection)
            .await
            .map_err(|error| migration_error("experience_historical_delete_failed", error))?;
        if deleted.rows_affected() != 1 {
            return Err(recovery_error(
                "experience_historical_delete_identity_mismatch",
            ));
        }
        let remaining: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM historical_question_artifacts WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_artifact_dependencies \
                  WHERE historical_artifact_id = ?), \
               (SELECT COUNT(*) FROM historical_question_lifecycle_links \
                  WHERE historical_artifact_id = ?), \
               (SELECT COUNT(*) FROM artifact_heads WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_consent_events WHERE id = ?), \
               (SELECT COUNT(*) FROM historical_transmission_events WHERE id = ?)",
        )
        .bind(historical_id)
        .bind(historical_id)
        .bind(historical_id)
        .bind(&identities.0)
        .bind(&identities.1)
        .bind(&identities.2)
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_historical_delete_unreadable", error))?;
        if remaining != (0, 0, 0, 0, 0, 0) {
            return Err(recovery_error("experience_historical_delete_incomplete"));
        }
    }
    Ok(())
}

async fn apply_command(
    connection: &mut SqliteConnection,
    command: &ExperienceWriteCommand,
    context: &ExperienceWriteContext<'_>,
) -> Result<ExperienceWriteOutcome, MigrationError> {
    match command {
        ExperienceWriteCommand::Create(input) => {
            let revision_id =
                create_source(connection, input, context, "created", "utf8-text-v1").await?;
            Ok(ExperienceWriteOutcome {
                status: ExperienceWriteStatus::Committed,
                revision_id: Some(revision_id),
                imported_count: 0,
                skipped_count: 0,
                operation_manifest: String::new(),
            })
        }
        ExperienceWriteCommand::Update {
            id,
            expected_revision_id,
            content,
            updated_at,
        } => {
            validate_identifier(id, "id")?;
            validate_identifier(expected_revision_id, "expected_revision_id")?;
            validate_content(content)?;
            validate_timestamp(updated_at, "updated_at")?;
            let Some(current) = current_source(connection, id).await? else {
                let exists: bool =
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM source_heads WHERE id = ?)")
                        .bind(id)
                        .fetch_one(&mut *connection)
                        .await
                        .map_err(|error| {
                            migration_error("experience_write_source_exists_failed", error)
                        })?;
                return Ok(ExperienceWriteOutcome {
                    status: if exists {
                        ExperienceWriteStatus::StaleRevision
                    } else {
                        ExperienceWriteStatus::NotFound
                    },
                    revision_id: None,
                    imported_count: 0,
                    skipped_count: 0,
                    operation_manifest: String::new(),
                });
            };
            if &current.revision_id != expected_revision_id {
                return Ok(ExperienceWriteOutcome {
                    status: ExperienceWriteStatus::StaleRevision,
                    revision_id: None,
                    imported_count: 0,
                    skipped_count: 0,
                    operation_manifest: String::new(),
                });
            }
            if updated_at <= &current.updated_at {
                return Err(write_error("experience_revision_must_advance"));
            }
            let dependents =
                collect_ordinary_dependents(connection, id, &current.revision_id).await?;
            let historical_ids =
                collect_historical_questions_for_source(connection, id, &current.revision_id)
                    .await?;
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterHistoricalParity,
            )?;

            let input = ExperienceWriteInput {
                id: id.clone(),
                content: content.clone(),
                created_at: current.created_at.clone(),
                updated_at: updated_at.clone(),
            };
            let revision_id = insert_source_revision(
                connection,
                &input,
                current.revision_number + 1,
                Some(&current.revision_id),
                "corrected",
                "utf8-text-v1",
                context,
            )
            .await?;
            let advanced = sqlx::query(
                "UPDATE source_heads SET current_revision_id = ?, updated_at = ? \
                 WHERE id = ? AND current_revision_id = ? AND lifecycle_state = 'active'",
            )
            .bind(&revision_id)
            .bind(updated_at)
            .bind(id)
            .bind(expected_revision_id)
            .execute(&mut *connection)
            .await
            .map_err(|error| migration_error("experience_write_head_update_failed", error))?;
            if advanced.rows_affected() != 1 {
                return Err(recovery_error("experience_write_head_update_mismatch"));
            }
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterHead,
            )?;
            link_user_provenance(connection, id, &revision_id, context).await?;
            invalidate_ordinary_dependents(connection, &dependents, context).await?;
            delete_historical_questions(connection, &historical_ids).await?;
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterHistoricalCascade,
            )?;
            let updated = sqlx::query(
                "UPDATE experience_entries SET content = ?, updated_at = ? \
                 WHERE id = ? AND content = ? AND created_at = ? AND updated_at = ?",
            )
            .bind(content)
            .bind(updated_at)
            .bind(id)
            .bind(&current.content)
            .bind(&current.created_at)
            .bind(&current.updated_at)
            .execute(&mut *connection)
            .await
            .map_err(|error| migration_error("experience_write_projection_update_failed", error))?;
            if updated.rows_affected() != 1 {
                return Err(recovery_error(
                    "experience_write_projection_update_mismatch",
                ));
            }
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterProjection,
            )?;
            Ok(ExperienceWriteOutcome {
                status: ExperienceWriteStatus::Committed,
                revision_id: Some(revision_id),
                imported_count: 0,
                skipped_count: 0,
                operation_manifest: String::new(),
            })
        }
        ExperienceWriteCommand::Delete {
            id,
            expected_revision_id,
        } => {
            validate_identifier(id, "id")?;
            validate_identifier(expected_revision_id, "expected_revision_id")?;
            let Some(current) = current_source(connection, id).await? else {
                let exists: bool =
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM source_heads WHERE id = ?)")
                        .bind(id)
                        .fetch_one(&mut *connection)
                        .await
                        .map_err(|error| {
                            migration_error("experience_write_source_exists_failed", error)
                        })?;
                return Ok(ExperienceWriteOutcome {
                    status: if exists {
                        ExperienceWriteStatus::StaleRevision
                    } else {
                        ExperienceWriteStatus::NotFound
                    },
                    revision_id: None,
                    imported_count: 0,
                    skipped_count: 0,
                    operation_manifest: String::new(),
                });
            };
            if &current.revision_id != expected_revision_id {
                return Ok(ExperienceWriteOutcome {
                    status: ExperienceWriteStatus::StaleRevision,
                    revision_id: None,
                    imported_count: 0,
                    skipped_count: 0,
                    operation_manifest: String::new(),
                });
            }
            if context.occurred_at <= current.updated_at.as_str() {
                return Err(write_error("experience_revision_must_advance"));
            }
            let _dependents =
                collect_ordinary_dependents(connection, id, &current.revision_id).await?;
            let historical_ids =
                collect_historical_questions_for_source(connection, id, &current.revision_id)
                    .await?;
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterHistoricalParity,
            )?;
            delete_historical_questions(connection, &historical_ids).await?;
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterHistoricalCascade,
            )?;
            sqlx::query("DELETE FROM artifact_heads WHERE source_id = ?")
                .bind(id)
                .execute(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_write_artifact_delete_failed", error)
                })?;
            sqlx::query("DELETE FROM persisted_artifacts WHERE source_entry_id = ?")
                .bind(id)
                .execute(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_write_projection_artifact_delete_failed", error)
                })?;
            sqlx::query("DELETE FROM content_tombstones WHERE source_id = ?")
                .bind(id)
                .execute(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_write_source_tombstone_delete_failed", error)
                })?;
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterArtifactCascade,
            )?;
            let cleared = sqlx::query(
                "UPDATE source_heads SET current_revision_id = NULL, \
                 lifecycle_state = 'deleted', updated_at = ? \
                 WHERE id = ? AND current_revision_id = ? AND lifecycle_state = 'active'",
            )
            .bind(context.occurred_at)
            .bind(id)
            .bind(expected_revision_id)
            .execute(&mut *connection)
            .await
            .map_err(|error| migration_error("experience_write_source_delete_failed", error))?;
            if cleared.rows_affected() != 1 {
                return Err(recovery_error("experience_write_source_delete_mismatch"));
            }
            sqlx::query(
                "DELETE FROM source_revision_content \
                 WHERE revision_id IN (SELECT id FROM source_revisions WHERE source_id = ?)",
            )
            .bind(id)
            .execute(&mut *connection)
            .await
            .map_err(|error| migration_error("experience_write_source_purge_failed", error))?;
            let projected = sqlx::query("DELETE FROM experience_entries WHERE id = ?")
                .bind(id)
                .execute(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_write_projection_delete_failed", error)
                })?;
            if projected.rows_affected() != 1 {
                return Err(recovery_error(
                    "experience_write_projection_delete_mismatch",
                ));
            }
            inject(
                context.failure_point,
                ExperienceWriteFailurePoint::AfterProjection,
            )?;
            Ok(ExperienceWriteOutcome {
                status: ExperienceWriteStatus::Committed,
                revision_id: None,
                imported_count: 0,
                skipped_count: 0,
                operation_manifest: String::new(),
            })
        }
        ExperienceWriteCommand::Import(inputs) => {
            let mut imported_count = 0_u64;
            let mut skipped_count = 0_u64;
            let mut seen = HashSet::new();
            for input in inputs {
                validate_input(input)?;
                if !seen.insert(input.id.clone()) {
                    skipped_count += 1;
                    continue;
                }
                let head: Option<(String, Option<String>)> = sqlx::query_as(
                    "SELECT lifecycle_state, current_revision_id FROM source_heads WHERE id = ?",
                )
                .bind(&input.id)
                .fetch_optional(&mut *connection)
                .await
                .map_err(|error| migration_error("experience_import_head_check_failed", error))?;
                let projection_exists: bool = sqlx::query_scalar(
                    "SELECT EXISTS(SELECT 1 FROM experience_entries WHERE id = ?)",
                )
                .bind(&input.id)
                .fetch_one(&mut *connection)
                .await
                .map_err(|error| {
                    migration_error("experience_import_projection_check_failed", error)
                })?;
                match head {
                    Some((state, current_revision_id)) => {
                        let consistent_active =
                            state == "active" && current_revision_id.is_some() && projection_exists;
                        let consistent_deleted = state == "deleted"
                            && current_revision_id.is_none()
                            && !projection_exists;
                        if consistent_active || consistent_deleted {
                            skipped_count += 1;
                        } else {
                            return Err(recovery_error(
                                "experience_import_existing_identity_inconsistent",
                            ));
                        }
                    }
                    None if projection_exists => {
                        return Err(recovery_error(
                            "experience_import_projection_without_authority",
                        ));
                    }
                    None => {
                        create_source(
                            connection,
                            input,
                            context,
                            "legacy_v4_baseline",
                            "legacy-v4-raw",
                        )
                        .await?;
                        imported_count += 1;
                    }
                }
            }
            Ok(ExperienceWriteOutcome {
                status: ExperienceWriteStatus::Committed,
                revision_id: None,
                imported_count,
                skipped_count,
                operation_manifest: String::new(),
            })
        }
    }
}

async fn verify_experience_database(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    verify_exact_v5(connection).await?;
    verify_source_projection(connection).await?;
    verify_exact_pattern_v5(connection).await?;
    verify_exact_context_recovery_v5(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await
}

async fn prepare_write(
    connection: &mut SqliteConnection,
    command: &ExperienceWriteCommand,
    context: &ExperienceWriteContext<'_>,
) -> Result<PreparedWriteResult, MigrationError> {
    verify_experience_database(connection).await?;
    raw_sql("PRAGMA defer_foreign_keys = ON")
        .execute(&mut *connection)
        .await
        .map_err(|error| migration_error("experience_write_deferred_fk_failed", error))?;
    insert_guard(connection, context).await?;
    let mut outcome = apply_command(connection, command, context).await?;
    if outcome.status != ExperienceWriteStatus::Committed {
        return Ok(PreparedWriteResult::NoChange(outcome));
    }
    verify_source_projection(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    inject(
        context.failure_point,
        ExperienceWriteFailurePoint::AfterReconciliation,
    )?;
    remove_guard(connection, context).await?;
    verify_experience_database(connection).await?;
    let post_manifest = operation_manifest(connection).await?;
    outcome.operation_manifest = post_manifest.clone();
    Ok(PreparedWriteResult::Write(PreparedWrite {
        outcome,
        post_manifest,
    }))
}

async fn verify_read_only(path: &Path, expected_manifest: &str) -> Result<(), MigrationError> {
    let mut connection = connect(path, true).await.map_err(|error| {
        recovery_error(format!("experience_write_reopen_failed:{}", error.code))
    })?;
    verify_experience_database(&mut connection)
        .await
        .map_err(|error| {
            recovery_error(format!(
                "experience_write_read_only_verification_failed:{}",
                error.code
            ))
        })?;
    let actual = operation_manifest(&mut connection).await?;
    if actual != expected_manifest {
        return Err(recovery_error(
            "experience_write_operation_manifest_mismatch",
        ));
    }
    Ok(())
}

async fn execute_with_adapter<A: CommitOutcomeAdapter>(
    path: &Path,
    command: ExperienceWriteCommand,
    context: ExperienceWriteContext<'_>,
    adapter: &A,
) -> Result<ExperienceWriteOutcome, MigrationError> {
    if !path.exists() {
        return Err(write_error("experience_write_database_missing"));
    }
    let mut connection = connect(path, false).await?;
    verify_experience_database(&mut connection).await?;
    let pre_manifest = operation_manifest(&mut connection).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("experience_write_begin_failed", error))?;

    let prepared = match prepare_write(&mut connection, &command, &context).await {
        Ok(PreparedWriteResult::Write(prepared)) => prepared,
        Ok(PreparedWriteResult::NoChange(mut outcome)) => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verification| {
                    recovery_error(format!(
                        "experience_write_no_change_state_unverified:{}:{rollback:?}",
                        verification.code
                    ))
                })?;
            outcome.operation_manifest = pre_manifest;
            return Ok(outcome);
        }
        Err(error) => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verification| {
                    recovery_error(format!(
                        "experience_write_precommit_state_unverified:{}:{rollback:?}",
                        verification.code
                    ))
                })?;
            return Err(error);
        }
    };

    match adapter.commit(&mut connection).await {
        CommitAttemptOutcome::Committed => {
            drop(connection);
            verify_read_only(path, &prepared.post_manifest).await?;
            Ok(prepared.outcome)
        }
        CommitAttemptOutcome::DefinitelyNotCommitted { error_class } => {
            let rollback = adapter.rollback(&mut connection).await;
            drop(connection);
            verify_read_only(path, &pre_manifest)
                .await
                .map_err(|verification| {
                    recovery_error(format!(
                    "experience_write_definite_noncommit_unverified:{error_class}:{}:{rollback:?}",
                    verification.code
                ))
                })?;
            Err(write_error(format!(
                "experience_write_commit_definitely_not_committed:{error_class}"
            )))
        }
        CommitAttemptOutcome::OutcomeUnknown { error_class } => {
            drop(connection);
            if verify_read_only(path, &prepared.post_manifest)
                .await
                .is_ok()
            {
                return Ok(prepared.outcome);
            }
            if verify_read_only(path, &pre_manifest).await.is_ok() {
                return Err(write_error(format!(
                    "experience_write_commit_outcome_unknown_unchanged:{error_class}"
                )));
            }
            Err(recovery_error(format!(
                "experience_write_commit_outcome_unknown:{error_class}"
            )))
        }
    }
}

pub(super) async fn execute_disposable(
    path: &Path,
    command: ExperienceWriteCommand,
    context: ExperienceWriteContext<'_>,
) -> Result<ExperienceWriteOutcome, MigrationError> {
    execute_with_adapter(path, command, context, &SqlCommitOutcomeAdapter).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const STARTED_AT: &str = "2026-07-30T00:00:00.000Z";
    const COMMITTED_AT: &str = "2026-07-30T00:00:01.000Z";
    const ORDINARY_STATE_FIXTURE: &str = r#"
INSERT INTO persisted_artifacts VALUES (
  'source-edit-evidence-pending', 'fixture-v4-history', 'evidence',
  '{"id":"source-edit-evidence-pending","sourceEntryId":"fixture-v4-history","text":"pending observation","originalText":"pending observation","kind":"observation","userEditable":true,"status":"candidate","provenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":[],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:02.000Z"},"createdAt":"2026-01-01T00:00:02.000Z","updatedAt":"2026-01-01T00:00:02.000Z"}',
  '2026-01-01T00:00:02.000Z', '2026-01-01T00:00:02.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-reflection-suggested', 'fixture-v4-history', 'reflection',
  '{"id":"source-edit-reflection-suggested","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"question":"What stayed?","status":"suggested","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence"],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:03.000Z"},"createdAt":"2026-01-01T00:00:03.000Z","updatedAt":"2026-01-01T00:00:03.000Z"}',
  '2026-01-01T00:00:03.000Z', '2026-01-01T00:00:03.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-reflection-answered', 'fixture-v4-history', 'reflection',
  '{"id":"source-edit-reflection-answered","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"question":"What changed?","status":"answered","response":"My exact answer","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence"],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:04.000Z"},"responseProvenance":{"origin":"user","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["source-edit-reflection-answered"],"provider":null,"model":null,"harnessVersion":null,"promptVersion":null,"generatedAt":"2026-01-01T00:00:05.000Z"},"createdAt":"2026-01-01T00:00:04.000Z","updatedAt":"2026-01-01T00:00:05.000Z"}',
  '2026-01-01T00:00:04.000Z', '2026-01-01T00:00:05.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-reflection-skipped', 'fixture-v4-history', 'reflection',
  '{"id":"source-edit-reflection-skipped","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"question":"Skip this?","status":"skipped","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence"],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:06.000Z"},"createdAt":"2026-01-01T00:00:06.000Z","updatedAt":"2026-01-01T00:00:06.000Z"}',
  '2026-01-01T00:00:06.000Z', '2026-01-01T00:00:06.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-pattern-candidate', 'fixture-v4-history', 'pattern',
  '{"id":"source-edit-pattern-candidate","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"sourceReflectionPromptIds":[],"text":"tentative candidate","status":"candidate","provenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence"],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:07.000Z"},"createdAt":"2026-01-01T00:00:07.000Z","updatedAt":"2026-01-01T00:00:07.000Z"}',
  '2026-01-01T00:00:07.000Z', '2026-01-01T00:00:07.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-pattern-confirmed', 'fixture-v4-history', 'pattern',
  '{"id":"source-edit-pattern-confirmed","sourceEntryId":"fixture-v4-history","sourceEvidenceIds":["fixture-v4-evidence"],"sourceReflectionPromptIds":["source-edit-reflection-answered"],"text":"tentative confirmed pattern","status":"confirmed","provenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["fixture-v4-evidence","source-edit-reflection-answered"],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:08.000Z"},"createdAt":"2026-01-01T00:00:08.000Z","updatedAt":"2026-01-01T00:00:08.000Z"}',
  '2026-01-01T00:00:08.000Z', '2026-01-01T00:00:08.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-recovery-suggested', 'fixture-v4-history', 'recovery_turn',
  '{"id":"source-edit-recovery-suggested","sourceEntryId":"fixture-v4-history","question":"What happened?","status":"suggested","locale":"en","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":[],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:09.000Z"},"createdAt":"2026-01-01T00:00:09.000Z","updatedAt":"2026-01-01T00:00:09.000Z"}',
  '2026-01-01T00:00:09.000Z', '2026-01-01T00:00:09.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-recovery-answered', 'fixture-v4-history', 'recovery_turn',
  '{"id":"source-edit-recovery-answered","sourceEntryId":"fixture-v4-history","question":"What helped?","response":"A pause.","status":"answered","locale":"en","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":[],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:10.000Z"},"responseProvenance":{"origin":"user","sourceEntryId":"fixture-v4-history","sourceArtifactIds":["source-edit-recovery-answered"],"provider":null,"model":null,"harnessVersion":null,"promptVersion":null,"generatedAt":"2026-01-01T00:00:11.000Z"},"createdAt":"2026-01-01T00:00:10.000Z","updatedAt":"2026-01-01T00:00:11.000Z"}',
  '2026-01-01T00:00:10.000Z', '2026-01-01T00:00:11.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'source-edit-recovery-skipped', 'fixture-v4-history', 'recovery_turn',
  '{"id":"source-edit-recovery-skipped","sourceEntryId":"fixture-v4-history","question":"Skip now?","status":"skipped","locale":"en","promptProvenance":{"origin":"local_mock","sourceEntryId":"fixture-v4-history","sourceArtifactIds":[],"provider":"mock","model":null,"harnessVersion":"h1","promptVersion":"p1","generatedAt":"2026-01-01T00:00:12.000Z"},"createdAt":"2026-01-01T00:00:12.000Z","updatedAt":"2026-01-01T00:00:12.000Z"}',
  '2026-01-01T00:00:12.000Z', '2026-01-01T00:00:12.000Z'
);
"#;

    async fn exact_v5_fixture(customize_v4: Option<&str>) -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        if let Some(sql) = customize_v4 {
            raw_sql(sql).execute(&mut connection).await.unwrap();
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
            backup_id: Some("slice4a-fixture-backup"),
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        (directory, path)
    }

    fn input(id: &str, content: &str, at: &str) -> ExperienceWriteInput {
        ExperienceWriteInput {
            id: id.into(),
            content: content.into(),
            created_at: at.into(),
            updated_at: at.into(),
        }
    }

    fn context<'a>(
        occurred_at: &'a str,
        guard_token: &'a str,
        failure_point: ExperienceWriteFailurePoint,
    ) -> ExperienceWriteContext<'a> {
        ExperienceWriteContext {
            occurred_at,
            guard_token,
            failure_point,
        }
    }

    async fn scalar_i64(path: &Path, query: &str) -> i64 {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar(query)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    async fn current_revision(path: &Path, id: &str) -> String {
        let mut connection = connect(path, true).await.unwrap();
        sqlx::query_scalar("SELECT current_revision_id FROM source_heads WHERE id = ?")
            .bind(id)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn create_writes_one_v5_revision_and_matching_v4_projection() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let created = input(
            "slice4a-created",
            "Exact user-authored content.\nSecond line remains exact.",
            "2026-07-30T01:00:00.000Z",
        );
        let outcome = execute_disposable(
            &path,
            ExperienceWriteCommand::Create(created.clone()),
            context(
                "2026-07-30T01:00:00.000Z",
                "guard-create-000000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(outcome.status, ExperienceWriteStatus::Committed);
        let revision_id = outcome.revision_id.unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let revision: (i64, Option<String>, String, String, String) = sqlx::query_as(
            "SELECT revision_number, predecessor_revision_id, authorship, \
             revision_reason, serialization_version FROM source_revisions WHERE id = ?",
        )
        .bind(&revision_id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            revision,
            (
                1,
                None,
                "user".into(),
                "created".into(),
                "utf8-text-v1".into()
            )
        );
        let projection: (String, String, String) = sqlx::query_as(
            "SELECT content, created_at, updated_at FROM experience_entries WHERE id = ?",
        )
        .bind(&created.id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            projection,
            (
                created.content.clone(),
                created.created_at,
                created.updated_at
            )
        );
        let content: (String, i64) = sqlx::query_as(
            "SELECT content, byte_length FROM source_revision_content WHERE revision_id = ?",
        )
        .bind(&revision_id)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            content,
            (created.content.clone(), created.content.len() as i64)
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM source_revision_provenance \
                 WHERE source_revision_id = ? AND role = 'content'"
            )
            .bind(&revision_id)
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            1
        );
        assert_eq!(
            scalar_i64(&path, "SELECT COUNT(*) FROM v5_compatibility_write_guard").await,
            0
        );
    }

    #[tokio::test]
    async fn update_appends_correction_and_preserves_prior_revision() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let prior = current_revision(&path, "fixture-v4-history").await;
        let outcome = execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior.clone(),
                content: "corrected user content".into(),
                updated_at: "2026-07-30T02:00:00.000Z".into(),
            },
            context(
                "2026-07-30T02:00:00.000Z",
                "guard-update-000000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let next = outcome.revision_id.unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let revision: (i64, String, String, String) = sqlx::query_as(
            "SELECT revision_number, predecessor_revision_id, authorship, revision_reason \
             FROM source_revisions WHERE id = ?",
        )
        .bind(&next)
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            revision,
            (2, prior.clone(), "user".into(), "corrected".into())
        );
        assert_eq!(
            sqlx::query_scalar::<_, String>(
                "SELECT content FROM source_revision_content WHERE revision_id = ?"
            )
            .bind(&prior)
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            "historical experience"
        );
        let projection: (String, String) = sqlx::query_as(
            "SELECT content, updated_at FROM experience_entries \
             WHERE id = 'fixture-v4-history'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            projection,
            (
                "corrected user content".into(),
                "2026-07-30T02:00:00.000Z".into()
            )
        );
    }

    #[tokio::test]
    async fn stale_update_rolls_back_without_revision_or_projection_change() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let outcome = execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: "stale".into(),
                content: "must not persist".into(),
                updated_at: "2026-07-30T03:00:00.000Z".into(),
            },
            context(
                "2026-07-30T03:00:00.000Z",
                "guard-stale-000000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(outcome.status, ExperienceWriteStatus::StaleRevision);
        assert_eq!(outcome.operation_manifest, before);
        verify_read_only(&path, &before).await.unwrap();
    }

    #[tokio::test]
    async fn update_invalidates_ordinary_artifact_without_rewriting_or_rebinding() {
        let (_directory, path) = exact_v5_fixture(None).await;
        let prior = current_revision(&path, "fixture-v4-history").await;
        let before: (String, String, String, String, String) = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_as(
                "SELECT r.content_digest, c.payload, p.canonical_payload, d.id, d.source_revision_id \
                 FROM artifact_heads h \
                 JOIN artifact_revisions r ON r.id = h.current_revision_id \
                 JOIN artifact_revision_content c ON c.revision_id = r.id \
                 JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id \
                 JOIN provenance_records p ON p.id = rp.provenance_id \
                 JOIN artifact_dependencies d ON d.dependent_artifact_id = h.id \
                   AND d.dependent_revision_id = r.id \
                   AND d.relationship_type = 'derived_from_experience' \
                 WHERE h.id = 'fixture-v4-evidence'",
            )
            .fetch_one(&mut connection)
            .await
            .unwrap()
        };
        let outcome = execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior.clone(),
                content: "corrected without rewriting dependent evidence".into(),
                updated_at: "2026-07-30T03:30:00.000Z".into(),
            },
            context(
                "2026-07-30T03:30:00.000Z",
                "guard-artifact-000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(outcome.status, ExperienceWriteStatus::Committed);
        let mut connection = connect(&path, true).await.unwrap();
        let after: (String, String, String, String, String) = sqlx::query_as(
            "SELECT r.content_digest, c.payload, p.canonical_payload, d.id, d.source_revision_id \
             FROM artifact_heads h \
             JOIN artifact_revisions r ON r.id = h.current_revision_id \
             JOIN artifact_revision_content c ON c.revision_id = r.id \
             JOIN artifact_revision_provenance rp ON rp.artifact_revision_id = r.id \
             JOIN provenance_records p ON p.id = rp.provenance_id \
             JOIN artifact_dependencies d ON d.dependent_artifact_id = h.id \
               AND d.dependent_revision_id = r.id \
               AND d.relationship_type = 'derived_from_experience' \
             WHERE h.id = 'fixture-v4-evidence'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(after, before);
        assert_eq!(after.4, prior);
        let head: (String, String, String, String) = sqlx::query_as(
            "SELECT review_state, lifecycle_state, eligibility_state, eligibility_reason \
             FROM artifact_heads WHERE id = 'fixture-v4-evidence'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            head,
            (
                "confirmed".into(),
                "invalidated".into(),
                "ineligible".into(),
                "source_experience_revision_superseded".into()
            )
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM artifact_lifecycle_events \
                 WHERE artifact_id = 'fixture-v4-evidence' \
                   AND event_type = 'invalidated' AND actor = 'system' \
                   AND reason_code = 'source_experience_revision_superseded'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM persisted_artifacts \
                 WHERE id = 'fixture-v4-evidence'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn update_invalidates_every_supported_active_ordinary_state_and_cascades_history() {
        let (_directory, path) = exact_v5_fixture(Some(ORDINARY_STATE_FIXTURE)).await;
        let prior = current_revision(&path, "fixture-v4-history").await;
        let before: Vec<(String, String, String, String, String, i64, String)> = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_as(
                "SELECT h.id, h.current_revision_id, h.review_state, r.content_digest, \
                        c.payload, \
                        (SELECT COUNT(*) FROM artifact_revision_provenance rp \
                          WHERE rp.artifact_revision_id = h.current_revision_id), \
                        d.source_revision_id \
                 FROM artifact_heads h \
                 JOIN artifact_revisions r ON r.id = h.current_revision_id \
                 JOIN artifact_revision_content c ON c.revision_id = r.id \
                 JOIN artifact_dependencies d ON d.dependent_artifact_id = h.id \
                   AND d.dependent_revision_id = h.current_revision_id \
                   AND d.relationship_type = 'derived_from_experience' \
                 WHERE h.source_id = 'fixture-v4-history' \
                   AND h.artifact_kind <> 'historical_question' ORDER BY h.id",
            )
            .fetch_all(&mut connection)
            .await
            .unwrap()
        };
        assert_eq!(before.len(), 10);
        execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior.clone(),
                content: "corrected source with complete ordinary consequences".into(),
                updated_at: "2026-07-30T03:40:00.000Z".into(),
            },
            context(
                "2026-07-30T03:40:00.000Z",
                "guard-all-ordinary-states-00000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let after: Vec<(String, String, String, String, String, i64, String)> = sqlx::query_as(
            "SELECT h.id, h.current_revision_id, h.review_state, r.content_digest, \
                    c.payload, \
                    (SELECT COUNT(*) FROM artifact_revision_provenance rp \
                      WHERE rp.artifact_revision_id = h.current_revision_id), \
                    d.source_revision_id \
             FROM artifact_heads h \
             JOIN artifact_revisions r ON r.id = h.current_revision_id \
             JOIN artifact_revision_content c ON c.revision_id = r.id \
             JOIN artifact_dependencies d ON d.dependent_artifact_id = h.id \
               AND d.dependent_revision_id = h.current_revision_id \
               AND d.relationship_type = 'derived_from_experience' \
             WHERE h.source_id = 'fixture-v4-history' \
               AND h.artifact_kind <> 'historical_question' ORDER BY h.id",
        )
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert_eq!(after, before);
        assert!(after.iter().all(|row| row.6 == prior));
        let state_counts: (i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM artifact_heads \
                 WHERE source_id = 'fixture-v4-history' \
                   AND artifact_kind <> 'historical_question' \
                   AND lifecycle_state = 'invalidated' \
                   AND eligibility_state = 'ineligible' \
                   AND eligibility_reason = 'source_experience_revision_superseded'), \
               (SELECT COUNT(*) FROM persisted_artifacts \
                 WHERE source_entry_id = 'fixture-v4-history'), \
               (SELECT COUNT(*) FROM artifact_lifecycle_events e \
                 JOIN artifact_heads h ON h.id = e.artifact_id \
                 WHERE h.source_id = 'fixture-v4-history' \
                   AND h.artifact_kind <> 'historical_question' \
                   AND e.event_type = 'invalidated' \
                   AND e.reason_code = 'source_experience_revision_superseded'), \
               (SELECT COUNT(*) FROM historical_question_artifacts)",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(state_counts, (10, 0, 10, 0));
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM historical_artifact_dependencies d \
                 JOIN artifact_heads h ON h.id = d.source_artifact_id \
                 WHERE h.artifact_kind = 'recovery_turn'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn later_source_correction_preserves_already_invalidated_artifacts_without_duplicate_events(
    ) {
        let (_directory, path) = exact_v5_fixture(Some(ORDINARY_STATE_FIXTURE)).await;
        let original = current_revision(&path, "fixture-v4-history").await;
        let first = execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: original,
                content: "first correction".into(),
                updated_at: "2026-07-30T03:41:00.000Z".into(),
            },
            context(
                "2026-07-30T03:41:00.000Z",
                "guard-first-source-correction-00000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let before: Vec<(String, String, String, String, String)> = {
            let mut connection = connect(&path, true).await.unwrap();
            sqlx::query_as(
                "SELECT h.id, h.current_revision_id, h.updated_at, r.content_digest, d.id \
                 FROM artifact_heads h \
                 JOIN artifact_revisions r ON r.id = h.current_revision_id \
                 JOIN artifact_dependencies d ON d.dependent_artifact_id = h.id \
                   AND d.dependent_revision_id = h.current_revision_id \
                   AND d.relationship_type = 'derived_from_experience' \
                 WHERE h.source_id = 'fixture-v4-history' \
                   AND h.artifact_kind <> 'historical_question' ORDER BY h.id",
            )
            .fetch_all(&mut connection)
            .await
            .unwrap()
        };
        execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: first.revision_id.unwrap(),
                content: "second correction".into(),
                updated_at: "2026-07-30T03:42:00.000Z".into(),
            },
            context(
                "2026-07-30T03:42:00.000Z",
                "guard-second-source-correction-0000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let after: Vec<(String, String, String, String, String)> = sqlx::query_as(
            "SELECT h.id, h.current_revision_id, h.updated_at, r.content_digest, d.id \
             FROM artifact_heads h \
             JOIN artifact_revisions r ON r.id = h.current_revision_id \
             JOIN artifact_dependencies d ON d.dependent_artifact_id = h.id \
               AND d.dependent_revision_id = h.current_revision_id \
               AND d.relationship_type = 'derived_from_experience' \
             WHERE h.source_id = 'fixture-v4-history' \
               AND h.artifact_kind <> 'historical_question' ORDER BY h.id",
        )
        .fetch_all(&mut connection)
        .await
        .unwrap();
        assert_eq!(after, before);
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM artifact_lifecycle_events e \
                 JOIN artifact_heads h ON h.id = e.artifact_id \
                 WHERE h.source_id = 'fixture-v4-history' \
                   AND e.reason_code = 'source_experience_revision_superseded'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            10
        );
    }

    #[tokio::test]
    async fn ordinary_consequence_failure_rolls_back_every_artifact_and_source_fact() {
        let (_directory, path) = exact_v5_fixture(Some(ORDINARY_STATE_FIXTURE)).await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let prior = current_revision(&path, "fixture-v4-history").await;
        let error = execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior.clone(),
                content: "must roll back all consequences".into(),
                updated_at: "2026-07-30T03:43:00.000Z".into(),
            },
            context(
                "2026-07-30T03:43:00.000Z",
                "guard-ordinary-rollback-000000000001",
                ExperienceWriteFailurePoint::AfterOrdinaryConsequences,
            ),
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("injected_experience_write_failure"));
        verify_read_only(&path, &before).await.unwrap();
    }

    #[tokio::test]
    async fn historical_representation_mismatch_fails_closed_before_source_mutation() {
        let (_directory, path) = exact_v5_fixture(None).await;
        {
            let mut connection = connect(&path, false).await.unwrap();
            raw_sql("BEGIN IMMEDIATE")
                .execute(&mut connection)
                .await
                .unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard (token, created_at) \
                 VALUES ('guard-test-historical-drift-00000001', \
                         '2026-07-30T03:43:30.000Z')",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query(
                "UPDATE historical_artifact_dependencies \
                 SET source_revision = '2026-01-01T00:00:00.001Z' \
                 WHERE historical_artifact_id = 'fixture-v4-question' \
                   AND source_artifact_id = 'fixture-v4-evidence'",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query(
                "DELETE FROM v5_compatibility_write_guard \
                 WHERE token = 'guard-test-historical-drift-00000001'",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            raw_sql("COMMIT").execute(&mut connection).await.unwrap();
        }
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let prior = current_revision(&path, "fixture-v4-history").await;
        let error = execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior.clone(),
                content: "must not pass mismatched history".into(),
                updated_at: "2026-07-30T03:43:31.000Z".into(),
            },
            context(
                "2026-07-30T03:43:31.000Z",
                "guard-history-mismatch-refusal-000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "experience_historical_item_dependency_mismatch");
        let mut connection = connect(&path, true).await.unwrap();
        assert_eq!(operation_manifest(&mut connection).await.unwrap(), before);
        assert_eq!(
            sqlx::query_scalar::<_, String>(
                "SELECT current_revision_id FROM source_heads \
                 WHERE id = 'fixture-v4-history'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            prior
        );
    }

    #[tokio::test]
    async fn parent_delete_exhaustively_purges_every_ordinary_state_fixture() {
        let (_directory, path) = exact_v5_fixture(Some(ORDINARY_STATE_FIXTURE)).await;
        let prior = current_revision(&path, "fixture-v4-history").await;
        execute_disposable(
            &path,
            ExperienceWriteCommand::Delete {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior,
            },
            context(
                "2026-07-30T03:44:00.000Z",
                "guard-exhaustive-parent-delete-0000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let remaining: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
            "SELECT \
               (SELECT COUNT(*) FROM artifact_heads WHERE source_id = 'fixture-v4-history'), \
               (SELECT COUNT(*) FROM artifact_revisions WHERE source_id = 'fixture-v4-history'), \
               (SELECT COUNT(*) FROM artifact_revision_content c \
                  JOIN artifact_revisions r ON r.id = c.revision_id \
                  WHERE r.source_id = 'fixture-v4-history'), \
               (SELECT COUNT(*) FROM artifact_dependencies d \
                  JOIN artifact_revisions r ON r.id = d.dependent_revision_id \
                  WHERE r.source_id = 'fixture-v4-history'), \
               (SELECT COUNT(*) FROM content_tombstones WHERE source_id = 'fixture-v4-history'), \
               (SELECT COUNT(*) FROM persisted_artifacts \
                  WHERE source_entry_id = 'fixture-v4-history')",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(remaining, (0, 0, 0, 0, 0, 0));
        let source: (Option<String>, String, i64) = sqlx::query_as(
            "SELECT current_revision_id, lifecycle_state, \
                    (SELECT COUNT(*) FROM source_revision_content c \
                      JOIN source_revisions r ON r.id = c.revision_id \
                      WHERE r.source_id = source_heads.id) \
             FROM source_heads WHERE id = 'fixture-v4-history'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(source, (None, "deleted".into(), 0));
    }

    #[tokio::test]
    async fn update_preserves_adr_0009_cascade_without_rebinding() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM persisted_artifacts; \
             UPDATE historical_artifact_dependencies \
             SET source_artifact_id = NULL, source_revision = '2026-01-01T00:00:00.000Z';",
        ))
        .await;
        let prior = current_revision(&path, "fixture-v4-history").await;
        execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior,
                content: "corrected source".into(),
                updated_at: "2026-07-30T04:00:00.000Z".into(),
            },
            context(
                "2026-07-30T04:00:00.000Z",
                "guard-cascade-0000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        for table in [
            "historical_question_artifacts",
            "historical_artifact_dependencies",
            "historical_consent_events",
            "historical_transmission_events",
            "historical_question_lifecycle_links",
        ] {
            assert_eq!(
                scalar_i64(&path, &format!("SELECT COUNT(*) FROM {table}")).await,
                0,
                "{table}"
            );
        }
    }

    #[tokio::test]
    async fn parent_delete_purges_content_and_parent_scoped_records() {
        let (_directory, path) = exact_v5_fixture(None).await;
        let prior = current_revision(&path, "fixture-v4-history").await;
        execute_disposable(
            &path,
            ExperienceWriteCommand::Delete {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior,
            },
            context(
                "2026-07-30T05:00:00.000Z",
                "guard-delete-000000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        let mut connection = connect(&path, true).await.unwrap();
        let head: (Option<String>, String) = sqlx::query_as(
            "SELECT current_revision_id, lifecycle_state FROM source_heads \
             WHERE id = 'fixture-v4-history'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(head, (None, "deleted".into()));
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM source_revision_content c \
                 JOIN source_revisions r ON r.id = c.revision_id \
                 WHERE r.source_id = 'fixture-v4-history'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            0
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM experience_entries \
                 WHERE id = 'fixture-v4-history'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            0
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM artifact_heads \
                 WHERE source_id = 'fixture-v4-history'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn import_is_atomic_duplicate_skipping_and_honest() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let first = input(
            "import-new",
            "imported exact current state",
            "2026-07-30T06:00:00.000Z",
        );
        let outcome = execute_disposable(
            &path,
            ExperienceWriteCommand::Import(vec![
                first.clone(),
                first,
                input(
                    "fixture-v4-history",
                    "must not overwrite",
                    "2026-07-30T06:00:01.000Z",
                ),
            ]),
            context(
                "2026-07-30T06:00:02.000Z",
                "guard-import-000000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(outcome.imported_count, 1);
        assert_eq!(outcome.skipped_count, 2);
        let mut connection = connect(&path, true).await.unwrap();
        let revision: (String, String, i64) = sqlx::query_as(
            "SELECT revision_reason, serialization_version, revision_number \
             FROM source_revisions WHERE source_id = 'import-new'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            revision,
            ("legacy_v4_baseline".into(), "legacy-v4-raw".into(), 1)
        );
        assert_eq!(
            sqlx::query_scalar::<_, String>(
                "SELECT content FROM experience_entries \
                 WHERE id = 'fixture-v4-history'"
            )
            .fetch_one(&mut connection)
            .await
            .unwrap(),
            "historical experience"
        );
    }

    #[tokio::test]
    async fn stale_delete_and_missing_source_return_explicit_unchanged_outcomes() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };

        let stale = execute_disposable(
            &path,
            ExperienceWriteCommand::Delete {
                id: "fixture-v4-history".into(),
                expected_revision_id: "not-current".into(),
            },
            context(
                "2026-07-30T06:10:00.000Z",
                "guard-stale-delete-000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(stale.status, ExperienceWriteStatus::StaleRevision);
        assert_eq!(stale.operation_manifest, before);

        let missing = execute_disposable(
            &path,
            ExperienceWriteCommand::Delete {
                id: "missing-source".into(),
                expected_revision_id: "not-current".into(),
            },
            context(
                "2026-07-30T06:11:00.000Z",
                "guard-missing-delete-00000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
        assert_eq!(missing.status, ExperienceWriteStatus::NotFound);
        assert_eq!(missing.operation_manifest, before);
        verify_read_only(&path, &before).await.unwrap();
    }

    #[tokio::test]
    async fn import_failure_rolls_back_the_entire_new_batch() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_disposable(
            &path,
            ExperienceWriteCommand::Import(vec![
                input(
                    "import-rollback-first",
                    "first exact content",
                    "2026-07-30T06:20:00.000Z",
                ),
                input(
                    "import-rollback-second",
                    "second exact content",
                    "2026-07-30T06:20:01.000Z",
                ),
            ]),
            context(
                "2026-07-30T06:20:02.000Z",
                "guard-import-rollback-0000000000000001",
                ExperienceWriteFailurePoint::AfterProjection,
            ),
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("injected_experience_write_failure"));
        verify_read_only(&path, &before).await.unwrap();
        assert_eq!(
            scalar_i64(
                &path,
                "SELECT COUNT(*) FROM source_heads \
                 WHERE id IN ('import-rollback-first', 'import-rollback-second')"
            )
            .await,
            0
        );
    }

    #[tokio::test]
    async fn update_and_delete_cascade_failures_roll_back_exactly() {
        let (_directory, update_path) = exact_v5_fixture(Some(
            "DELETE FROM persisted_artifacts; \
             UPDATE historical_artifact_dependencies \
             SET source_artifact_id = NULL, source_revision = '2026-01-01T00:00:00.000Z';",
        ))
        .await;
        let update_before = {
            let mut connection = connect(&update_path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let prior = current_revision(&update_path, "fixture-v4-history").await;
        execute_disposable(
            &update_path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior,
                content: "must roll back with historical rows".into(),
                updated_at: "2026-07-30T06:30:00.000Z".into(),
            },
            context(
                "2026-07-30T06:30:00.000Z",
                "guard-update-cascade-rollback-00000001",
                ExperienceWriteFailurePoint::AfterHistoricalCascade,
            ),
        )
        .await
        .unwrap_err();
        verify_read_only(&update_path, &update_before)
            .await
            .unwrap();
        assert_eq!(
            scalar_i64(
                &update_path,
                "SELECT COUNT(*) FROM historical_question_artifacts"
            )
            .await,
            1
        );

        let (_directory, delete_path) = exact_v5_fixture(None).await;
        let delete_before = {
            let mut connection = connect(&delete_path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let prior = current_revision(&delete_path, "fixture-v4-history").await;
        execute_disposable(
            &delete_path,
            ExperienceWriteCommand::Delete {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior,
            },
            context(
                "2026-07-30T06:31:00.000Z",
                "guard-delete-cascade-rollback-00000001",
                ExperienceWriteFailurePoint::AfterArtifactCascade,
            ),
        )
        .await
        .unwrap_err();
        verify_read_only(&delete_path, &delete_before)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn all_create_failure_points_roll_back_authority_projection_and_guard() {
        for (index, failure_point) in [
            ExperienceWriteFailurePoint::AfterGuard,
            ExperienceWriteFailurePoint::AfterProvenance,
            ExperienceWriteFailurePoint::AfterRevision,
            ExperienceWriteFailurePoint::AfterContent,
            ExperienceWriteFailurePoint::AfterHead,
            ExperienceWriteFailurePoint::AfterProjection,
            ExperienceWriteFailurePoint::AfterReconciliation,
            ExperienceWriteFailurePoint::AfterGuardRemoval,
        ]
        .into_iter()
        .enumerate()
        {
            let (_directory, path) = exact_v5_fixture(Some(
                "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
            ))
            .await;
            let before = {
                let mut connection = connect(&path, true).await.unwrap();
                operation_manifest(&mut connection).await.unwrap()
            };
            let error = execute_disposable(
                &path,
                ExperienceWriteCommand::Create(input(
                    "must-rollback",
                    "must not persist",
                    "2026-07-30T07:00:00.000Z",
                )),
                context(
                    "2026-07-30T07:00:00.000Z",
                    &format!("guard-failure-{index:032}"),
                    failure_point,
                ),
            )
            .await
            .unwrap_err();
            assert!(error.code.contains("injected_experience_write_failure"));
            verify_read_only(&path, &before).await.unwrap();
        }
    }

    struct InjectedCommitAdapter {
        outcome: CommitAttemptOutcome,
        commit_first: bool,
        rollback_calls: Cell<usize>,
    }

    impl CommitOutcomeAdapter for InjectedCommitAdapter {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            if self.commit_first {
                raw_sql("COMMIT").execute(connection).await.unwrap();
            }
            self.outcome.clone()
        }

        async fn rollback(&self, connection: &mut SqliteConnection) -> RollbackAttemptOutcome {
            self.rollback_calls.set(self.rollback_calls.get() + 1);
            raw_sql("ROLLBACK").execute(connection).await.unwrap();
            RollbackAttemptOutcome::RolledBack
        }
    }

    #[tokio::test]
    async fn ambiguous_commit_classifies_exact_post_or_pre_state_without_retry() {
        let (_directory, committed_path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let committed = execute_with_adapter(
            &committed_path,
            ExperienceWriteCommand::Create(input(
                "ambiguous-committed",
                "committed",
                "2026-07-30T08:00:00.000Z",
            )),
            context(
                "2026-07-30T08:00:00.000Z",
                "guard-ambiguous-commit-00000000000001",
                ExperienceWriteFailurePoint::None,
            ),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_after_commit".into(),
                },
                commit_first: true,
                rollback_calls: Cell::new(0),
            },
        )
        .await
        .unwrap();
        assert_eq!(committed.status, ExperienceWriteStatus::Committed);

        let (_directory, unchanged_path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        let error = execute_with_adapter(
            &unchanged_path,
            ExperienceWriteCommand::Create(input(
                "ambiguous-unchanged",
                "unchanged",
                "2026-07-30T08:01:00.000Z",
            )),
            context(
                "2026-07-30T08:01:00.000Z",
                "guard-ambiguous-pre-0000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_without_commit".into(),
                },
                commit_first: false,
                rollback_calls: Cell::new(0),
            },
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("outcome_unknown_unchanged"));
        assert_eq!(
            scalar_i64(
                &unchanged_path,
                "SELECT COUNT(*) FROM source_heads WHERE id='ambiguous-unchanged'"
            )
            .await,
            0
        );
    }

    #[tokio::test]
    async fn ordinary_consequence_commit_ambiguity_accepts_only_exact_post_or_pre_state() {
        let (_directory, committed_path) = exact_v5_fixture(Some(ORDINARY_STATE_FIXTURE)).await;
        let committed_prior = current_revision(&committed_path, "fixture-v4-history").await;
        let committed = execute_with_adapter(
            &committed_path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: committed_prior,
                content: "ambiguous committed source correction".into(),
                updated_at: "2026-07-30T08:10:00.000Z".into(),
            },
            context(
                "2026-07-30T08:10:00.000Z",
                "guard-ambiguous-consequence-post-000001",
                ExperienceWriteFailurePoint::None,
            ),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_after_consequence_commit".into(),
                },
                commit_first: true,
                rollback_calls: Cell::new(0),
            },
        )
        .await
        .unwrap();
        assert_eq!(committed.status, ExperienceWriteStatus::Committed);
        assert_eq!(
            scalar_i64(
                &committed_path,
                "SELECT COUNT(*) FROM artifact_heads \
                 WHERE source_id='fixture-v4-history' \
                   AND artifact_kind <> 'historical_question' \
                   AND lifecycle_state='invalidated'"
            )
            .await,
            10
        );

        let (_directory, unchanged_path) = exact_v5_fixture(Some(ORDINARY_STATE_FIXTURE)).await;
        let unchanged_prior = current_revision(&unchanged_path, "fixture-v4-history").await;
        let before = {
            let mut connection = connect(&unchanged_path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_with_adapter(
            &unchanged_path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: unchanged_prior,
                content: "ambiguous unchanged source correction".into(),
                updated_at: "2026-07-30T08:11:00.000Z".into(),
            },
            context(
                "2026-07-30T08:11:00.000Z",
                "guard-ambiguous-consequence-pre-0000001",
                ExperienceWriteFailurePoint::None,
            ),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_without_consequence_commit".into(),
                },
                commit_first: false,
                rollback_calls: Cell::new(0),
            },
        )
        .await
        .unwrap_err();
        assert!(error.code.contains("outcome_unknown_unchanged"));
        verify_read_only(&unchanged_path, &before).await.unwrap();
    }

    #[tokio::test]
    async fn malformed_v4_inconsistent_v5_and_newer_database_fail_closed() {
        let directory = tempfile::tempdir().unwrap();
        let malformed = directory.path().join("malformed.db");
        std::fs::write(&malformed, b"not sqlite").unwrap();
        assert!(execute_disposable(
            &malformed,
            ExperienceWriteCommand::Create(input("x", "x", "2026-07-30T09:00:00.000Z")),
            context(
                "2026-07-30T09:00:00.000Z",
                "guard-malformed-00000000000000000001",
                ExperienceWriteFailurePoint::None
            )
        )
        .await
        .is_err());

        let (_v4_dir, v4_path) = {
            let directory = tempfile::tempdir().unwrap();
            let path = directory.path().join("v4.db");
            let options = SqliteConnectOptions::new()
                .filename(&path)
                .create_if_missing(true)
                .foreign_keys(true);
            let mut connection = SqliteConnection::connect_with(&options).await.unwrap();
            raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
            drop(connection);
            (directory, path)
        };
        let error = execute_disposable(
            &v4_path,
            ExperienceWriteCommand::Create(input("v4", "v4", "2026-07-30T09:01:00.000Z")),
            context(
                "2026-07-30T09:01:00.000Z",
                "guard-v4-refusal-0000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "experience_write_requires_exact_v5");

        let (_directory, inconsistent_path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        {
            let mut connection = connect(&inconsistent_path, false).await.unwrap();
            raw_sql("PRAGMA foreign_keys = OFF")
                .execute(&mut connection)
                .await
                .unwrap();
            sqlx::query(
                "UPDATE experience_entries SET content='drift' \
                 WHERE id='fixture-v4-history'",
            )
            .execute(&mut connection)
            .await
            .unwrap_err();
            drop(connection);
        }
        {
            let mut connection = connect(&inconsistent_path, false).await.unwrap();
            raw_sql("BEGIN IMMEDIATE")
                .execute(&mut connection)
                .await
                .unwrap();
            sqlx::query(
                "INSERT INTO v5_compatibility_write_guard VALUES \
                 ('manual-drift','2026-07-30T09:02:00.000Z')",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query(
                "UPDATE experience_entries SET content='drift' \
                 WHERE id='fixture-v4-history'",
            )
            .execute(&mut connection)
            .await
            .unwrap();
            sqlx::query("DELETE FROM v5_compatibility_write_guard")
                .execute(&mut connection)
                .await
                .unwrap();
            raw_sql("COMMIT").execute(&mut connection).await.unwrap();
        }
        let error = execute_disposable(
            &inconsistent_path,
            ExperienceWriteCommand::Create(input("blocked", "blocked", "2026-07-30T09:03:00.000Z")),
            context(
                "2026-07-30T09:03:00.000Z",
                "guard-drift-refusal-00000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert!(error.recovery_required);

        let (_directory, newer_path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
        {
            let mut connection = connect(&newer_path, false).await.unwrap();
            raw_sql("PRAGMA user_version = 6")
                .execute(&mut connection)
                .await
                .unwrap();
        }
        let error = execute_disposable(
            &newer_path,
            ExperienceWriteCommand::Create(input("newer", "newer", "2026-07-30T09:04:00.000Z")),
            context(
                "2026-07-30T09:04:00.000Z",
                "guard-newer-refusal-0000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "experience_write_requires_exact_v5");
    }

    #[tokio::test]
    async fn deterministic_fixture_inputs_repeat_revision_and_manifest() {
        let mut results = Vec::new();
        for _ in 0..2 {
            let (_directory, path) = exact_v5_fixture(Some(
                "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
            ))
            .await;
            let outcome = execute_disposable(
                &path,
                ExperienceWriteCommand::Create(input(
                    "deterministic",
                    "same exact bytes",
                    "2026-07-30T10:00:00.000Z",
                )),
                context(
                    "2026-07-30T10:00:00.000Z",
                    "guard-deterministic-0000000000000001",
                    ExperienceWriteFailurePoint::None,
                ),
            )
            .await
            .unwrap();
            results.push((outcome.revision_id, outcome.operation_manifest));
        }
        assert_eq!(results[0], results[1]);
    }

    #[tokio::test]
    async fn migration_receipt_manifests_remain_immutable_after_current_write() {
        let (_directory, path) = exact_v5_fixture(Some(
            "DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;",
        ))
        .await;
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
        execute_disposable(
            &path,
            ExperienceWriteCommand::Create(input(
                "receipt-safe",
                "new current data",
                "2026-07-30T11:00:00.000Z",
            )),
            context(
                "2026-07-30T11:00:00.000Z",
                "guard-receipt-0000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap();
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
    }
}
