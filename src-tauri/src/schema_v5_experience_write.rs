use super::*;
use sqlx::{raw_sql, Row};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExperienceWriteInput {
    id: String,
    content: String,
    created_at: String,
    updated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ExperienceWriteCommand {
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
enum ExperienceWriteFailurePoint {
    #[default]
    None,
    AfterGuard,
    AfterProvenance,
    AfterRevision,
    AfterContent,
    AfterHead,
    AfterHistoricalCascade,
    AfterArtifactCascade,
    AfterProjection,
    AfterReconciliation,
    AfterGuardRemoval,
}

#[derive(Clone, Debug)]
struct ExperienceWriteContext<'a> {
    occurred_at: &'a str,
    guard_token: &'a str,
    failure_point: ExperienceWriteFailurePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ExperienceWriteStatus {
    Committed,
    StaleRevision,
    NotFound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExperienceWriteOutcome {
    status: ExperienceWriteStatus,
    revision_id: Option<String>,
    imported_count: u64,
    skipped_count: u64,
    operation_manifest: String,
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

async fn operation_manifest(connection: &mut SqliteConnection) -> Result<String, MigrationError> {
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
    if contracts
        != vec![(
            TARGET_SCHEMA_VERSION,
            APPLICATION_VERSION.to_string(),
            "enabled".to_string(),
            "disabled".to_string(),
            "disabled".to_string(),
        )]
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

async fn verify_exact_v5(connection: &mut SqliteConnection) -> Result<(), MigrationError> {
    if user_version(connection).await? != TARGET_SCHEMA_VERSION {
        return Err(write_error("experience_write_requires_exact_v5"));
    }
    if schema_object_manifest(connection).await? != EXPECTED_SCHEMA_OBJECT_MANIFEST_SHA256 {
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

async fn assert_no_ordinary_update_effects(
    connection: &mut SqliteConnection,
    source_id: &str,
) -> Result<(), MigrationError> {
    let source_artifacts: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_heads \
         WHERE source_id = ? AND artifact_kind <> 'historical_question'",
    )
    .bind(source_id)
    .fetch_one(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_artifact_scope_check_failed", error))?;
    let projection_artifacts: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM persisted_artifacts WHERE source_entry_id = ?")
            .bind(source_id)
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| {
                migration_error("experience_write_projection_artifact_check_failed", error)
            })?;
    if source_artifacts != 0 || projection_artifacts != 0 {
        return Err(write_error(
            "ordinary_artifact_lifecycle_requires_later_slice",
        ));
    }
    assert_no_external_ordinary_dependents(connection, source_id).await
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

async fn delete_historical_questions_for_source(
    connection: &mut SqliteConnection,
    source_id: &str,
) -> Result<(), MigrationError> {
    sqlx::query(
        "DELETE FROM historical_question_artifacts \
         WHERE current_experience_id = ? OR id IN (\
           SELECT historical_artifact_id FROM historical_artifact_dependencies \
           WHERE source_entry_id = ?\
         )",
    )
    .bind(source_id)
    .bind(source_id)
    .execute(&mut *connection)
    .await
    .map_err(|error| migration_error("experience_write_historical_cascade_failed", error))?;
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
            assert_no_ordinary_update_effects(connection, id).await?;

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
            delete_historical_questions_for_source(connection, id).await?;
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
            assert_no_external_ordinary_dependents(connection, id).await?;
            delete_historical_questions_for_source(connection, id).await?;
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

async fn prepare_write(
    connection: &mut SqliteConnection,
    command: &ExperienceWriteCommand,
    context: &ExperienceWriteContext<'_>,
) -> Result<PreparedWriteResult, MigrationError> {
    verify_exact_v5(connection).await?;
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
    verify_source_projection(connection).await?;
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
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
    verify_exact_v5(&mut connection).await.map_err(|error| {
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
    verify_exact_v5(&mut connection).await?;
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

async fn execute_disposable(
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
    async fn update_fails_closed_when_ordinary_artifact_lifecycle_is_required() {
        let (_directory, path) = exact_v5_fixture(None).await;
        let prior = current_revision(&path, "fixture-v4-history").await;
        let before = {
            let mut connection = connect(&path, true).await.unwrap();
            operation_manifest(&mut connection).await.unwrap()
        };
        let error = execute_disposable(
            &path,
            ExperienceWriteCommand::Update {
                id: "fixture-v4-history".into(),
                expected_revision_id: prior,
                content: "must not replace artifact lifecycle".into(),
                updated_at: "2026-07-30T03:30:00.000Z".into(),
            },
            context(
                "2026-07-30T03:30:00.000Z",
                "guard-artifact-000000000000000000001",
                ExperienceWriteFailurePoint::None,
            ),
        )
        .await
        .unwrap_err();
        assert_eq!(
            error.code,
            "ordinary_artifact_lifecycle_requires_later_slice"
        );
        verify_read_only(&path, &before).await.unwrap();
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
