//! Typed schema-v5 runtime routing for the isolated Founder candidate.
//!
//! This module deliberately accepts domain values, never SQL or filesystem
//! paths from the renderer. Every mutation is delegated to the promoted v5
//! writer that owns the relevant lifecycle contract.

use super::context_recovery_write::{
    execute_disposable as execute_recovery, ContextRecoveryWriteCommand,
    ContextRecoveryWriteContext, ContextRecoveryWriteFailurePoint,
    PromptProvenanceInput as RecoveryProvenance, SuggestedPromptInput as RecoveryPrompt,
};
use super::evidence_write::{
    execute_disposable as execute_evidence, EvidenceCandidateInput, EvidenceProvenanceInput,
    EvidenceWriteCommand, EvidenceWriteContext, EvidenceWriteFailurePoint,
};
use super::experience_write::{
    execute_disposable as execute_experience, ExperienceWriteCommand, ExperienceWriteContext,
    ExperienceWriteFailurePoint, ExperienceWriteInput, ExperienceWriteStatus,
};
use super::historical_question_write::{
    write_disposable_historical_question, HistoricalQuestion, HistoricalQuestionWriteRequest,
    HistoricalWriteContext, HistoricalWriteFailurePoint, HistoricalWriteStatus,
};
use super::pattern_write::{
    execute_disposable as execute_pattern, ArtifactRevisionRef, PatternCandidateInput,
    PatternProvenanceInput, PatternWriteCommand, PatternWriteContext, PatternWriteFailurePoint,
};
use super::reflection_write::{
    execute_disposable as execute_reflection, EvidenceRevisionRef, PromptProvenanceInput,
    ReflectionWriteCommand, ReflectionWriteContext, ReflectionWriteFailurePoint,
    SuggestedPromptInput,
};
use super::{connect, verify_activated_v5_runtime, MigrationError};
use serde::Serialize;
use serde_json::Value;
use sqlx::{raw_sql, Connection, Row};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceRow {
    pub(crate) id: String,
    pub(crate) body: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveArtifactsResult {
    pub(crate) status: &'static str,
    pub(crate) bundle: Value,
}

fn fail(code: impl Into<String>) -> MigrationError {
    MigrationError::fail_closed(code)
}

fn field<'a>(value: &'a Value, key: &str) -> Result<&'a str, MigrationError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| fail(format!("founder_runtime_{key}_invalid")))
}

fn optional_string(value: &Value, key: &str) -> Result<Option<String>, MigrationError> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        _ => Err(fail(format!("founder_runtime_{key}_invalid"))),
    }
}

fn strings(value: &Value, key: &str) -> Result<Vec<String>, MigrationError> {
    let values = value
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| fail(format!("founder_runtime_{key}_invalid")))?;
    let mut result = Vec::with_capacity(values.len());
    for value in values {
        let value = value
            .as_str()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| fail(format!("founder_runtime_{key}_invalid")))?;
        result.push(value.to_string());
    }
    if result.iter().collect::<BTreeSet<_>>().len() != result.len() {
        return Err(fail(format!("founder_runtime_{key}_duplicate")));
    }
    Ok(result)
}

fn bool_field(value: &Value, key: &str) -> Result<bool, MigrationError> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| fail(format!("founder_runtime_{key}_invalid")))
}

type ParsedProvenance = (
    String,
    String,
    Option<String>,
    String,
    String,
    String,
    Vec<String>,
);

fn provenance(value: &Value, key: &str) -> Result<ParsedProvenance, MigrationError> {
    let value = value
        .get(key)
        .ok_or_else(|| fail(format!("founder_runtime_{key}_missing")))?;
    Ok((
        field(value, "origin")?.to_string(),
        field(value, "provider")?.to_string(),
        optional_string(value, "model")?,
        field(value, "harnessVersion")?.to_string(),
        field(value, "promptVersion")?.to_string(),
        field(value, "generatedAt")?.to_string(),
        strings(value, "sourceArtifactIds")?,
    ))
}

async fn source_revision(
    path: &Path,
    source_id: &str,
    expected_updated_at: Option<&str>,
) -> Result<String, MigrationError> {
    let mut connection = connect(path, true).await?;
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT h.current_revision_id, e.updated_at FROM source_heads h \
         JOIN experience_entries e ON e.id = h.id WHERE h.id = ? AND h.lifecycle_state = 'active'",
    )
    .bind(source_id)
    .fetch_optional(&mut connection)
    .await
    .map_err(|error| fail(format!("founder_runtime_source_unreadable:{error}")))?;
    connection
        .close()
        .await
        .map_err(|error| fail(format!("founder_runtime_close_failed:{error}")))?;
    let (revision, updated_at) = row.ok_or_else(|| fail("founder_runtime_source_missing"))?;
    if expected_updated_at.is_some_and(|expected| expected != updated_at) {
        return Err(fail("stale_generation"));
    }
    Ok(revision)
}

async fn artifact_revision(
    path: &Path,
    artifact_id: &str,
    kind: &str,
) -> Result<String, MigrationError> {
    let mut connection = connect(path, true).await?;
    let revision: Option<String> = sqlx::query_scalar(
        "SELECT current_revision_id FROM artifact_heads \
         WHERE id = ? AND artifact_kind = ? AND current_revision_id IS NOT NULL",
    )
    .bind(artifact_id)
    .bind(kind)
    .fetch_optional(&mut connection)
    .await
    .map_err(|error| fail(format!("founder_runtime_artifact_unreadable:{error}")))?;
    connection
        .close()
        .await
        .map_err(|error| fail(format!("founder_runtime_close_failed:{error}")))?;
    revision.ok_or_else(|| fail("founder_runtime_artifact_missing"))
}

async fn revision_refs<T>(
    path: &Path,
    ids: &[String],
    kind: &str,
    make: impl Fn(String, String) -> T,
) -> Result<Vec<T>, MigrationError> {
    let mut result = Vec::with_capacity(ids.len());
    for id in ids {
        result.push(make(id.clone(), artifact_revision(path, id, kind).await?));
    }
    Ok(result)
}

pub(crate) async fn list_experiences(path: &Path) -> Result<Vec<ExperienceRow>, MigrationError> {
    verify_activated_v5_runtime(path).await?;
    let mut connection = connect(path, true).await?;
    let rows = sqlx::query(
        "SELECT e.id, e.content, e.created_at, e.updated_at FROM experience_entries e \
         JOIN source_heads h ON h.id = e.id \
         WHERE h.lifecycle_state = 'active' ORDER BY e.created_at DESC, e.id",
    )
    .fetch_all(&mut connection)
    .await
    .map_err(|error| fail(format!("founder_runtime_experiences_unreadable:{error}")))?;
    let result = rows
        .into_iter()
        .map(|row| ExperienceRow {
            id: row.get(0),
            body: row.get(1),
            created_at: row.get(2),
            updated_at: row.get(3),
        })
        .collect();
    connection
        .close()
        .await
        .map_err(|error| fail(format!("founder_runtime_close_failed:{error}")))?;
    Ok(result)
}

pub(crate) async fn get_experience(
    path: &Path,
    id: &str,
) -> Result<Option<ExperienceRow>, MigrationError> {
    Ok(list_experiences(path)
        .await?
        .into_iter()
        .find(|row| row.id == id))
}

pub(crate) async fn list_artifacts(path: &Path, source_id: &str) -> Result<Value, MigrationError> {
    verify_activated_v5_runtime(path).await?;
    let mut connection = connect(path, true).await?;
    let rows = sqlx::query(
        "SELECT artifact_kind, payload FROM persisted_artifacts \
         WHERE source_entry_id = ? ORDER BY created_at, id",
    )
    .bind(source_id)
    .fetch_all(&mut connection)
    .await
    .map_err(|error| fail(format!("founder_runtime_artifacts_unreadable:{error}")))?;
    let mut evidence = Vec::new();
    let mut reflections = Vec::new();
    let mut patterns = Vec::new();
    let mut recovery = Vec::new();
    for row in rows {
        let kind: String = row.get(0);
        let raw: String = row.get(1);
        let value: Value =
            serde_json::from_str(&raw).map_err(|_| fail("founder_runtime_projection_malformed"))?;
        match kind.as_str() {
            "evidence" => evidence.push(value),
            "reflection" => reflections.push(value),
            "pattern" => patterns.push(value),
            "recovery_turn" => recovery.push(value),
            _ => return Err(fail("founder_runtime_projection_kind_unsupported")),
        }
    }
    connection
        .close()
        .await
        .map_err(|error| fail(format!("founder_runtime_close_failed:{error}")))?;
    Ok(serde_json::json!({
        "evidence": evidence,
        "reflections": reflections,
        "patterns": patterns,
        "recoveryTurns": recovery,
    }))
}

fn category_map(bundle: &Value, key: &str) -> Result<BTreeMap<String, Value>, MigrationError> {
    let values = bundle
        .get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| fail(format!("founder_runtime_{key}_invalid")))?;
    let mut result = BTreeMap::new();
    for value in values {
        let id = field(value, "id")?.to_string();
        if result.insert(id, value.clone()).is_some() {
            return Err(fail(format!("founder_runtime_{key}_duplicate")));
        }
    }
    Ok(result)
}

fn changed_categories(
    current: &Value,
    desired: &Value,
) -> Result<Vec<&'static str>, MigrationError> {
    let mut result = Vec::new();
    for key in ["evidence", "reflections", "patterns", "recoveryTurns"] {
        if category_map(current, key)? != category_map(desired, key)? {
            result.push(key);
        }
    }
    Ok(result)
}

fn evidence_input(value: &Value) -> Result<EvidenceCandidateInput, MigrationError> {
    let (
        origin,
        provider,
        model,
        harness_version,
        prompt_version,
        generated_at,
        source_artifact_ids,
    ) = provenance(value, "provenance")?;
    Ok(EvidenceCandidateInput {
        id: field(value, "id")?.into(),
        source_id: field(value, "sourceEntryId")?.into(),
        text: field(value, "text")?.into(),
        original_text: field(value, "originalText")?.into(),
        kind: field(value, "kind")?.into(),
        user_editable: bool_field(value, "userEditable")?,
        created_at: field(value, "createdAt")?.into(),
        provenance: EvidenceProvenanceInput {
            origin,
            provider,
            model,
            harness_version,
            prompt_version,
            generated_at,
            source_artifact_ids,
        },
    })
}

fn prompt_input(
    value: &Value,
    evidence: Vec<EvidenceRevisionRef>,
) -> Result<SuggestedPromptInput, MigrationError> {
    let (
        origin,
        provider,
        model,
        harness_version,
        prompt_version,
        generated_at,
        source_artifact_ids,
    ) = provenance(value, "promptProvenance")?;
    Ok(SuggestedPromptInput {
        id: field(value, "id")?.into(),
        source_id: field(value, "sourceEntryId")?.into(),
        question: field(value, "question")?.into(),
        created_at: field(value, "createdAt")?.into(),
        evidence,
        provenance: PromptProvenanceInput {
            origin,
            provider,
            model,
            harness_version,
            prompt_version,
            generated_at,
            source_artifact_ids,
        },
    })
}

fn pattern_input(
    value: &Value,
    evidence: Vec<ArtifactRevisionRef>,
    reflections: Vec<ArtifactRevisionRef>,
) -> Result<PatternCandidateInput, MigrationError> {
    let (
        origin,
        provider,
        model,
        harness_version,
        prompt_version,
        generated_at,
        source_artifact_ids,
    ) = provenance(value, "provenance")?;
    Ok(PatternCandidateInput {
        id: field(value, "id")?.into(),
        source_id: field(value, "sourceEntryId")?.into(),
        text: field(value, "text")?.into(),
        created_at: field(value, "createdAt")?.into(),
        evidence,
        reflections,
        provenance: PatternProvenanceInput {
            origin,
            provider,
            model,
            harness_version,
            prompt_version,
            generated_at,
            source_artifact_ids,
        },
    })
}

fn recovery_input(value: &Value) -> Result<RecoveryPrompt, MigrationError> {
    let (
        origin,
        provider,
        model,
        harness_version,
        prompt_version,
        generated_at,
        source_artifact_ids,
    ) = provenance(value, "promptProvenance")?;
    Ok(RecoveryPrompt {
        id: field(value, "id")?.into(),
        source_id: field(value, "sourceEntryId")?.into(),
        question: field(value, "question")?.into(),
        locale: field(value, "locale")?.into(),
        created_at: field(value, "createdAt")?.into(),
        provenance: RecoveryProvenance {
            origin,
            provider,
            model,
            harness_version,
            prompt_version,
            generated_at,
            source_artifact_ids,
        },
    })
}

fn canonicalize_recovery_response_provenance(
    bundle: &mut Value,
    artifact_id: &str,
    source_id: &str,
    occurred_at: &str,
) -> Result<(), MigrationError> {
    let turns = bundle
        .get_mut("recoveryTurns")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| fail("founder_runtime_recoveryTurns_invalid"))?;
    let turn = turns
        .iter_mut()
        .find(|value| value.get("id").and_then(Value::as_str) == Some(artifact_id))
        .ok_or_else(|| fail("founder_runtime_recovery_projection_missing"))?;
    let provenance = turn
        .get("responseProvenance")
        .and_then(Value::as_object)
        .ok_or_else(|| fail("founder_runtime_recovery_response_provenance_invalid"))?;
    let allowed: BTreeSet<_> = [
        "origin",
        "sourceEntryId",
        "sourceArtifactIds",
        "provider",
        "model",
        "harnessVersion",
        "promptVersion",
        "generatedAt",
    ]
    .into_iter()
    .collect();
    if provenance.keys().any(|key| !allowed.contains(key.as_str()))
        || field(turn, "status")? != "answered"
        || field(&Value::Object(provenance.clone()), "origin")? != "user"
        || field(&Value::Object(provenance.clone()), "sourceEntryId")? != source_id
        || field(&Value::Object(provenance.clone()), "generatedAt")? != occurred_at
        || strings(&Value::Object(provenance.clone()), "sourceArtifactIds")?
            != vec![artifact_id.to_string()]
        || ["provider", "model", "harnessVersion", "promptVersion"]
            .iter()
            .any(|key| provenance.get(*key).is_some_and(|value| !value.is_null()))
    {
        return Err(fail(
            "founder_runtime_recovery_response_provenance_mismatch",
        ));
    }
    turn["responseProvenance"] = serde_json::json!({
        "generatedAt": occurred_at,
        "harnessVersion": null,
        "model": null,
        "origin": "user",
        "promptVersion": null,
        "provider": null,
        "sourceArtifactIds": [artifact_id],
        "sourceEntryId": source_id
    });
    Ok(())
}

fn one_changed<'a>(
    current: &'a BTreeMap<String, Value>,
    desired: &'a BTreeMap<String, Value>,
) -> Result<(&'a str, Option<&'a Value>, Option<&'a Value>), MigrationError> {
    let ids: BTreeSet<_> = current.keys().chain(desired.keys()).collect();
    let changed: Vec<_> = ids
        .into_iter()
        .filter(|id| current.get(*id) != desired.get(*id))
        .collect();
    if changed.len() != 1 {
        return Err(fail("founder_runtime_mutation_not_single_action"));
    }
    let id = changed[0].as_str();
    Ok((id, current.get(id), desired.get(id)))
}

pub(crate) async fn save_artifacts(
    path: &Path,
    source_id: &str,
    mut desired: Value,
    expected_updated_at: Option<&str>,
    occurred_at: &str,
    guard_token: &str,
) -> Result<SaveArtifactsResult, MigrationError> {
    let current = list_artifacts(path, source_id).await?;
    let categories = changed_categories(&current, &desired)?;
    if categories.is_empty() {
        return Ok(SaveArtifactsResult {
            status: "committed",
            bundle: current,
        });
    }
    if categories.len() != 1 {
        return Err(fail("founder_runtime_cross_kind_bundle_refused"));
    }
    let source_revision = match source_revision(path, source_id, expected_updated_at).await {
        Ok(value) => value,
        Err(error) if error.code == "stale_generation" => {
            return Ok(SaveArtifactsResult {
                status: "stale_generation",
                bundle: current,
            })
        }
        Err(error) => return Err(error),
    };
    match categories[0] {
        "evidence" => {
            let before = category_map(&current, "evidence")?;
            let after = category_map(&desired, "evidence")?;
            let additions: Vec<_> = after
                .iter()
                .filter(|(id, _)| !before.contains_key(*id))
                .collect();
            let command = if !additions.is_empty()
                && before
                    .iter()
                    .all(|(id, value)| after.get(id) == Some(value))
            {
                EvidenceWriteCommand::CreateBatch {
                    expected_source_revision_id: source_revision,
                    candidates: additions
                        .into_iter()
                        .map(|(_, value)| evidence_input(value))
                        .collect::<Result<_, _>>()?,
                }
            } else {
                let (id, old, new) = one_changed(&before, &after)?;
                let old = old.ok_or_else(|| fail("founder_runtime_evidence_action_unsupported"))?;
                let revision = artifact_revision(path, id, "evidence").await?;
                match new {
                    None if field(old, "status")? == "candidate" => {
                        EvidenceWriteCommand::RejectPending {
                            source_id: source_id.into(),
                            artifact_id: id.into(),
                            expected_source_revision_id: source_revision,
                            expected_artifact_revision_id: revision,
                        }
                    }
                    Some(value)
                        if field(old, "status")? == "candidate"
                            && field(value, "status")? == "confirmed" =>
                    {
                        EvidenceWriteCommand::ConfirmPending {
                            source_id: source_id.into(),
                            artifact_id: id.into(),
                            expected_source_revision_id: source_revision,
                            expected_artifact_revision_id: revision,
                        }
                    }
                    Some(value)
                        if field(old, "status")? == "candidate"
                            && field(value, "status")? == "candidate" =>
                    {
                        EvidenceWriteCommand::CorrectPending {
                            source_id: source_id.into(),
                            artifact_id: id.into(),
                            expected_source_revision_id: source_revision,
                            expected_artifact_revision_id: revision,
                            text: field(value, "text")?.into(),
                        }
                    }
                    _ => return Err(fail("founder_runtime_evidence_action_unsupported")),
                }
            };
            execute_evidence(
                path,
                command,
                EvidenceWriteContext {
                    occurred_at,
                    guard_token,
                    failure_point: EvidenceWriteFailurePoint::None,
                },
            )
            .await?;
        }
        "reflections" => {
            let before = category_map(&current, "reflections")?;
            let after = category_map(&desired, "reflections")?;
            let additions: Vec<_> = after
                .iter()
                .filter(|(id, _)| !before.contains_key(*id))
                .collect();
            let command = if !additions.is_empty()
                && before
                    .iter()
                    .all(|(id, value)| after.get(id) == Some(value))
            {
                let mut prompts = Vec::new();
                for (_, value) in additions {
                    let ids = strings(value, "sourceEvidenceIds")?;
                    let refs = revision_refs(path, &ids, "evidence", |artifact_id, revision_id| {
                        EvidenceRevisionRef {
                            artifact_id,
                            revision_id,
                        }
                    })
                    .await?;
                    prompts.push(prompt_input(value, refs)?);
                }
                ReflectionWriteCommand::CreateSuggestedBatch {
                    expected_source_revision_id: source_revision,
                    prompts,
                }
            } else {
                let (id, old, new) = one_changed(&before, &after)?;
                let old =
                    old.ok_or_else(|| fail("founder_runtime_reflection_action_unsupported"))?;
                let value =
                    new.ok_or_else(|| fail("founder_runtime_reflection_delete_unsupported"))?;
                let revision = artifact_revision(path, id, "reflection").await?;
                let ids = strings(old, "sourceEvidenceIds")?;
                let refs = revision_refs(path, &ids, "evidence", |artifact_id, revision_id| {
                    EvidenceRevisionRef {
                        artifact_id,
                        revision_id,
                    }
                })
                .await?;
                match (field(old, "status")?, field(value, "status")?) {
                    ("suggested", "answered") => ReflectionWriteCommand::SaveResponse {
                        source_id: source_id.into(),
                        artifact_id: id.into(),
                        expected_source_revision_id: source_revision,
                        expected_artifact_revision_id: revision,
                        expected_evidence: refs,
                        response: field(value, "response")?.into(),
                    },
                    ("answered", "answered") => ReflectionWriteCommand::CorrectResponse {
                        source_id: source_id.into(),
                        artifact_id: id.into(),
                        expected_source_revision_id: source_revision,
                        expected_artifact_revision_id: revision,
                        expected_evidence: refs,
                        response: field(value, "response")?.into(),
                    },
                    ("suggested", "skipped") => ReflectionWriteCommand::SkipSuggested {
                        source_id: source_id.into(),
                        artifact_id: id.into(),
                        expected_source_revision_id: source_revision,
                        expected_artifact_revision_id: revision,
                        expected_evidence: refs,
                    },
                    _ => return Err(fail("founder_runtime_reflection_action_unsupported")),
                }
            };
            execute_reflection(
                path,
                command,
                ReflectionWriteContext {
                    occurred_at,
                    guard_token,
                    failure_point: ReflectionWriteFailurePoint::None,
                },
            )
            .await?;
        }
        "patterns" => {
            let before = category_map(&current, "patterns")?;
            let after = category_map(&desired, "patterns")?;
            let (id, old, new) = one_changed(&before, &after)?;
            let command = match (old, new) {
                (None, Some(value)) => {
                    let evidence_ids = strings(value, "sourceEvidenceIds")?;
                    let reflection_ids = strings(value, "sourceReflectionPromptIds")?;
                    let evidence = revision_refs(
                        path,
                        &evidence_ids,
                        "evidence",
                        |artifact_id, revision_id| ArtifactRevisionRef {
                            artifact_id,
                            revision_id,
                        },
                    )
                    .await?;
                    let reflections = revision_refs(
                        path,
                        &reflection_ids,
                        "reflection",
                        |artifact_id, revision_id| ArtifactRevisionRef {
                            artifact_id,
                            revision_id,
                        },
                    )
                    .await?;
                    PatternWriteCommand::Create {
                        expected_source_revision_id: source_revision,
                        candidate: pattern_input(value, evidence, reflections)?,
                    }
                }
                (Some(old), next) => {
                    let evidence_ids = strings(old, "sourceEvidenceIds")?;
                    let reflection_ids = strings(old, "sourceReflectionPromptIds")?;
                    let evidence = revision_refs(
                        path,
                        &evidence_ids,
                        "evidence",
                        |artifact_id, revision_id| ArtifactRevisionRef {
                            artifact_id,
                            revision_id,
                        },
                    )
                    .await?;
                    let reflections = revision_refs(
                        path,
                        &reflection_ids,
                        "reflection",
                        |artifact_id, revision_id| ArtifactRevisionRef {
                            artifact_id,
                            revision_id,
                        },
                    )
                    .await?;
                    let revision = artifact_revision(path, id, "pattern").await?;
                    match next {
                        None if field(old, "status")? == "candidate" => {
                            PatternWriteCommand::RejectPending {
                                source_id: source_id.into(),
                                artifact_id: id.into(),
                                expected_source_revision_id: source_revision,
                                expected_artifact_revision_id: revision,
                                expected_evidence: evidence,
                                expected_reflections: reflections,
                            }
                        }
                        Some(value)
                            if field(old, "status")? == "candidate"
                                && field(value, "status")? == "confirmed" =>
                        {
                            PatternWriteCommand::ConfirmPending {
                                source_id: source_id.into(),
                                artifact_id: id.into(),
                                expected_source_revision_id: source_revision,
                                expected_artifact_revision_id: revision,
                                expected_evidence: evidence,
                                expected_reflections: reflections,
                            }
                        }
                        _ => return Err(fail("founder_runtime_pattern_action_unsupported")),
                    }
                }
                _ => return Err(fail("founder_runtime_pattern_action_unsupported")),
            };
            execute_pattern(
                path,
                command,
                PatternWriteContext {
                    occurred_at,
                    guard_token,
                    failure_point: PatternWriteFailurePoint::None,
                },
            )
            .await?;
        }
        "recoveryTurns" => {
            let before = category_map(&current, "recoveryTurns")?;
            let after = category_map(&desired, "recoveryTurns")?;
            let (id, old, new) = one_changed(&before, &after)?;
            let mut canonicalize_answer = false;
            let command = match (old, new) {
                (None, Some(value)) => ContextRecoveryWriteCommand::CreateSuggested {
                    expected_source_revision_id: source_revision,
                    prompt: recovery_input(value)?,
                },
                (Some(old), Some(value)) => {
                    let revision = artifact_revision(path, id, "recovery_turn").await?;
                    match (field(old, "status")?, field(value, "status")?) {
                        ("suggested", "answered") => {
                            canonicalize_answer = true;
                            ContextRecoveryWriteCommand::SaveFirstResponse {
                                source_id: source_id.into(),
                                artifact_id: id.into(),
                                expected_source_revision_id: source_revision,
                                expected_artifact_revision_id: revision,
                                response: field(value, "response")?.into(),
                            }
                        }
                        ("suggested", "skipped") => ContextRecoveryWriteCommand::SkipSuggested {
                            source_id: source_id.into(),
                            artifact_id: id.into(),
                            expected_source_revision_id: source_revision,
                            expected_artifact_revision_id: revision,
                        },
                        _ => return Err(fail("founder_runtime_recovery_action_unsupported")),
                    }
                }
                _ => return Err(fail("founder_runtime_recovery_action_unsupported")),
            };
            if canonicalize_answer {
                canonicalize_recovery_response_provenance(
                    &mut desired,
                    id,
                    source_id,
                    occurred_at,
                )?;
            }
            execute_recovery(
                path,
                command,
                ContextRecoveryWriteContext {
                    occurred_at,
                    guard_token,
                    failure_point: ContextRecoveryWriteFailurePoint::None,
                },
            )
            .await?;
        }
        _ => return Err(fail("founder_runtime_category_unsupported")),
    }
    let actual = list_artifacts(path, source_id).await?;
    if actual != desired {
        return Err(MigrationError::recovery_required(
            "founder_runtime_projection_postcondition_mismatch",
        ));
    }
    Ok(SaveArtifactsResult {
        status: "committed",
        bundle: actual,
    })
}

pub(crate) async fn create_experience(
    path: &Path,
    id: &str,
    body: &str,
    occurred_at: &str,
    guard_token: &str,
) -> Result<ExperienceRow, MigrationError> {
    let outcome = execute_experience(
        path,
        ExperienceWriteCommand::Create(ExperienceWriteInput {
            id: id.into(),
            content: body.into(),
            created_at: occurred_at.into(),
            updated_at: occurred_at.into(),
        }),
        ExperienceWriteContext {
            occurred_at,
            guard_token,
            failure_point: ExperienceWriteFailurePoint::None,
        },
    )
    .await?;
    if outcome.status != ExperienceWriteStatus::Committed {
        return Err(fail("founder_runtime_create_not_committed"));
    }
    get_experience(path, id)
        .await?
        .ok_or_else(|| MigrationError::recovery_required("founder_runtime_create_missing"))
}

pub(crate) async fn update_experience(
    path: &Path,
    id: &str,
    expected_updated_at: &str,
    body: &str,
    occurred_at: &str,
    guard_token: &str,
) -> Result<Option<ExperienceRow>, MigrationError> {
    let revision = match source_revision(path, id, Some(expected_updated_at)).await {
        Ok(value) => value,
        Err(error) if error.code == "stale_generation" => return Ok(None),
        Err(error) => return Err(error),
    };
    let outcome = execute_experience(
        path,
        ExperienceWriteCommand::Update {
            id: id.into(),
            expected_revision_id: revision,
            content: body.into(),
            updated_at: occurred_at.into(),
        },
        ExperienceWriteContext {
            occurred_at,
            guard_token,
            failure_point: ExperienceWriteFailurePoint::None,
        },
    )
    .await?;
    if outcome.status == ExperienceWriteStatus::StaleRevision {
        return Ok(None);
    }
    get_experience(path, id).await
}

pub(crate) async fn delete_experience(
    path: &Path,
    id: &str,
    expected_updated_at: &str,
    occurred_at: &str,
    guard_token: &str,
) -> Result<bool, MigrationError> {
    let revision = match source_revision(path, id, Some(expected_updated_at)).await {
        Ok(value) => value,
        Err(error) if error.code == "stale_generation" => return Ok(false),
        Err(error) => return Err(error),
    };
    let outcome = execute_experience(
        path,
        ExperienceWriteCommand::Delete {
            id: id.into(),
            expected_revision_id: revision,
        },
        ExperienceWriteContext {
            occurred_at,
            guard_token,
            failure_point: ExperienceWriteFailurePoint::None,
        },
    )
    .await?;
    Ok(outcome.status == ExperienceWriteStatus::Committed)
}

pub(crate) async fn import_experiences(
    path: &Path,
    entries: Vec<ExperienceRow>,
    occurred_at: &str,
    guard_token: &str,
) -> Result<(u64, u64), MigrationError> {
    let entries = entries
        .into_iter()
        .map(|entry| ExperienceWriteInput {
            id: entry.id,
            content: entry.body,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        })
        .collect();
    let outcome = execute_experience(
        path,
        ExperienceWriteCommand::Import(entries),
        ExperienceWriteContext {
            occurred_at,
            guard_token,
            failure_point: ExperienceWriteFailurePoint::None,
        },
    )
    .await?;
    Ok((outcome.imported_count, outcome.skipped_count))
}

async fn guarded_projection_transaction<F, T>(
    path: &Path,
    guard_token: &str,
    operation: F,
) -> Result<T, MigrationError>
where
    F: for<'a> FnOnce(
        &'a mut sqlx::SqliteConnection,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<T, MigrationError>> + 'a>,
    >,
{
    verify_activated_v5_runtime(path).await?;
    let mut connection = connect(path, false).await?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| fail(format!("founder_runtime_begin_failed:{error}")))?;
    let prepared = async {
        sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, strftime('%Y-%m-%dT%H:%M:%fZ','now'))")
            .bind(guard_token)
            .execute(&mut connection)
            .await
            .map_err(|error| fail(format!("founder_runtime_guard_failed:{error}")))?;
        let value = operation(&mut connection).await?;
        let removed = sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
            .bind(guard_token)
            .execute(&mut connection)
            .await
            .map_err(|error| fail(format!("founder_runtime_guard_cleanup_failed:{error}")))?;
        if removed.rows_affected() != 1 {
            return Err(MigrationError::recovery_required("founder_runtime_guard_cleanup_mismatch"));
        }
        Ok(value)
    }
    .await;
    let value = match prepared {
        Ok(value) => value,
        Err(error) => {
            let _ = raw_sql("ROLLBACK").execute(&mut connection).await;
            return Err(error);
        }
    };
    raw_sql("COMMIT")
        .execute(&mut connection)
        .await
        .map_err(|error| {
            MigrationError::recovery_required(format!(
                "founder_runtime_commit_outcome_unknown:{error}"
            ))
        })?;
    connection.close().await.map_err(|error| {
        MigrationError::recovery_required(format!("founder_runtime_close_failed:{error}"))
    })?;
    verify_activated_v5_runtime(path).await?;
    Ok(value)
}

pub(crate) async fn save_historical_consent(
    path: &Path,
    event: Value,
    guard_token: &str,
) -> Result<(), MigrationError> {
    let id = field(&event, "id")?.to_string();
    let digest = field(&event, "packetDigest")?.to_string();
    let task = field(&event, "task")?.to_string();
    let purpose = field(&event, "purpose")?.to_string();
    let provider = field(&event, "provider")?.to_string();
    let _model = field(&event, "model")?.to_string();
    let state = field(&event, "state")?.to_string();
    let created_at = field(&event, "createdAt")?.to_string();
    let expires_at = field(&event, "expiresAt")?.to_string();
    if task != "historical_reflection_questions"
        || purpose != "invite_user_comparison_without_cross_time_conclusions"
        || !matches!(provider.as_str(), "openai" | "gemini")
        || !matches!(state.as_str(), "granted" | "invalidated")
    {
        return Err(fail("invalid_historical_consent_scope"));
    }
    let payload = super::canonical_json(&event)?;
    guarded_projection_transaction(path, guard_token, move |connection| Box::pin(async move {
        let existing: Option<(String, String, String)> = sqlx::query_as(
            "SELECT packet_digest, payload, state FROM historical_consent_events WHERE id = ?",
        )
        .bind(&id)
        .fetch_optional(&mut *connection)
        .await
        .map_err(|error| fail(format!("historical_consent_read_failed:{error}")))?;
        match existing {
            None => {
                sqlx::query("INSERT INTO historical_consent_events (id, packet_digest, payload, state, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?)")
                    .bind(&id).bind(&digest).bind(&payload).bind(&state).bind(&created_at).bind(&expires_at)
                    .execute(&mut *connection).await.map_err(|error| fail(format!("historical_consent_write_failed:{error}")))?;
            }
            Some((existing_digest, existing_payload, existing_state))
                if existing_digest == digest && existing_payload == payload && existing_state == state => {}
            Some((existing_digest, _, existing_state))
                if existing_digest == digest && existing_state == "granted" && state == "invalidated" => {
                    sqlx::query("UPDATE historical_consent_events SET payload = ?, state = ? WHERE id = ? AND state = 'granted'")
                        .bind(&payload).bind(&state).bind(&id).execute(&mut *connection).await
                        .map_err(|error| fail(format!("historical_consent_update_failed:{error}")))?;
            }
            _ => return Err(fail("historical_consent_conflict")),
        }
        Ok(())
    })).await
}

pub(crate) async fn save_historical_transmission(
    path: &Path,
    event: Value,
    guard_token: &str,
) -> Result<(), MigrationError> {
    let id = field(&event, "id")?.to_string();
    let consent_id = field(&event, "consentId")?.to_string();
    let digest = field(&event, "packetDigest")?.to_string();
    let provider = field(&event, "provider")?.to_string();
    let model = field(&event, "model")?.to_string();
    let outcome = field(&event, "outcome")?.to_string();
    let created_at = field(&event, "createdAt")?.to_string();
    let expires_at = field(&event, "expiresAt")?.to_string();
    if !matches!(provider.as_str(), "openai" | "gemini")
        || !matches!(
            outcome.as_str(),
            "sent" | "failed" | "refused" | "cancelled_before_send" | "cancelled_after_send"
        )
    {
        return Err(fail("invalid_historical_transmission"));
    }
    guarded_projection_transaction(path, guard_token, move |connection| Box::pin(async move {
        let consent: Option<(String, String, String, String)> = sqlx::query_as(
            "SELECT packet_digest, state, payload, expires_at FROM historical_consent_events WHERE id = ?",
        ).bind(&consent_id).fetch_optional(&mut *connection).await
            .map_err(|error| fail(format!("historical_consent_read_failed:{error}")))?;
        let (consent_digest, consent_state, consent_payload, _) = consent.ok_or_else(|| fail("historical_transmission_consent_missing"))?;
        let consent_value: Value = serde_json::from_str(&consent_payload).map_err(|_| fail("historical_transmission_consent_malformed"))?;
        if consent_digest != digest || field(&consent_value, "provider")? != provider || field(&consent_value, "model")? != model {
            return Err(fail("historical_transmission_consent_mismatch"));
        }
        let existing: Option<(String, String, String, String, String)> = sqlx::query_as(
            "SELECT consent_id, packet_digest, provider, model, outcome FROM historical_transmission_events WHERE id = ?",
        ).bind(&id).fetch_optional(&mut *connection).await
            .map_err(|error| fail(format!("historical_transmission_read_failed:{error}")))?;
        if let Some(existing) = existing {
            if existing == (consent_id.clone(), digest.clone(), provider.clone(), model.clone(), outcome.clone()) { return Ok(()); }
            return Err(fail("historical_transmission_conflict"));
        }
        if consent_state != "granted" { return Err(fail("historical_transmission_consent_not_granted")); }
        sqlx::query("INSERT INTO historical_transmission_events (id, consent_id, packet_digest, provider, model, outcome, created_at, expires_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&id).bind(&consent_id).bind(&digest).bind(&provider).bind(&model).bind(&outcome).bind(&created_at).bind(&expires_at)
            .execute(&mut *connection).await.map_err(|error| fail(format!("historical_transmission_write_failed:{error}")))?;
        if outcome == "sent" {
            let consumed = sqlx::query("UPDATE historical_consent_events SET state = 'consumed', payload = json_set(payload, '$.state', 'consumed') WHERE id = ? AND state = 'granted'")
                .bind(&consent_id).execute(&mut *connection).await
                .map_err(|error| fail(format!("historical_consent_consume_failed:{error}")))?;
            if consumed.rows_affected() != 1 { return Err(fail("historical_consent_consume_mismatch")); }
        }
        Ok(())
    })).await
}

pub(crate) async fn save_historical_question(
    path: &Path,
    artifact: Value,
    guard_token: &str,
) -> Result<&'static str, MigrationError> {
    verify_activated_v5_runtime(path).await?;
    let questions_value = artifact
        .get("questions")
        .and_then(Value::as_array)
        .ok_or_else(|| fail("historical_question_questions_invalid"))?;
    let mut questions = Vec::with_capacity(questions_value.len());
    for question in questions_value {
        questions.push(HistoricalQuestion {
            id: field(question, "id")?.into(),
            text: field(question, "text")?.into(),
            source_experience_ids: strings(question, "sourceExperienceIds")?,
        });
    }
    let packet = artifact
        .get("packet")
        .cloned()
        .ok_or_else(|| fail("historical_question_packet_missing"))?;
    let request = HistoricalQuestionWriteRequest {
        artifact_id: field(&artifact, "id")?.into(),
        current_experience_id: field(&artifact, "currentExperienceId")?.into(),
        questions,
        packet_snapshot: super::canonical_json(&packet)?,
        consent_id: field(&artifact, "consentId")?.into(),
        transmission_id: field(&artifact, "transmissionId")?.into(),
        generated_at: field(&artifact, "generatedAt")?.into(),
    };
    let outcome = write_disposable_historical_question(
        path,
        request,
        HistoricalWriteContext {
            guard_token,
            failure_point: HistoricalWriteFailurePoint::None,
        },
    )
    .await?;
    Ok(match outcome.status {
        HistoricalWriteStatus::Committed | HistoricalWriteStatus::AlreadyCommitted => "committed",
        HistoricalWriteStatus::NoQuestion => "stale_generation",
    })
}

pub(crate) async fn list_historical_questions(
    path: &Path,
    current_experience_id: &str,
) -> Result<Vec<Value>, MigrationError> {
    verify_activated_v5_runtime(path).await?;
    let mut connection = connect(path, true).await?;
    let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT payload, packet_snapshot FROM historical_question_artifacts WHERE current_experience_id = ? ORDER BY created_at DESC",
    ).bind(current_experience_id).fetch_all(&mut connection).await
        .map_err(|error| fail(format!("historical_question_read_failed:{error}")))?;
    connection
        .close()
        .await
        .map_err(|error| fail(format!("founder_runtime_close_failed:{error}")))?;
    rows.into_iter()
        .map(|(payload, packet)| {
            let mut value: Value = serde_json::from_str(&payload)
                .map_err(|_| fail("historical_question_payload_malformed"))?;
            let packet: Value = serde_json::from_str(&packet)
                .map_err(|_| fail("historical_question_packet_malformed"))?;
            value
                .as_object_mut()
                .ok_or_else(|| fail("historical_question_payload_malformed"))?
                .insert("packet".into(), packet);
            Ok(value)
        })
        .collect()
}

pub(crate) async fn delete_historical_question(
    path: &Path,
    id: &str,
    guard_token: &str,
) -> Result<(), MigrationError> {
    let id = id.to_string();
    guarded_projection_transaction(path, guard_token, move |connection| {
        Box::pin(async move {
            sqlx::query("DELETE FROM historical_question_artifacts WHERE id = ?")
                .bind(&id)
                .execute(&mut *connection)
                .await
                .map_err(|error| fail(format!("historical_question_delete_failed:{error}")))?;
            Ok(())
        })
    })
    .await
}

pub(crate) async fn purge_expired_historical_audit(
    path: &Path,
    timestamp: &str,
    guard_token: &str,
) -> Result<(), MigrationError> {
    let timestamp = timestamp.to_string();
    guarded_projection_transaction(path, guard_token, move |connection| Box::pin(async move {
        sqlx::query("DELETE FROM historical_transmission_events WHERE expires_at <= ? AND outcome != 'sent' AND id NOT IN (SELECT transmission_id FROM historical_question_artifacts)")
            .bind(&timestamp).execute(&mut *connection).await
            .map_err(|error| fail(format!("historical_audit_cleanup_failed:{error}")))?;
        sqlx::query("DELETE FROM historical_consent_events WHERE expires_at <= ? AND id NOT IN (SELECT consent_id FROM historical_transmission_events) AND id NOT IN (SELECT consent_id FROM historical_question_artifacts)")
            .bind(&timestamp).execute(&mut *connection).await
            .map_err(|error| fail(format!("historical_consent_cleanup_failed:{error}")))?;
        Ok(())
    })).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema_v5_migration::{
        activate_lifecycle_writes, manifest, migrate_disposable_v4, FailurePoint, MigrationRequest,
        SOURCE_TABLE_MANIFESTS,
    };
    use sqlx::sqlite::SqliteConnectOptions;
    use tempfile::TempDir;

    const V4_FIXTURE: &str = include_str!("../tests/fixtures/schema_v5/v4.sql");
    const SOURCE_ID: &str = "fixture-v4-history";
    const AT: &str = "2026-08-13T12:00:00.000Z";

    async fn fixture() -> (TempDir, std::path::PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("life-os.db");
        let options = SqliteConnectOptions::new()
            .filename(&path)
            .create_if_missing(true)
            .foreign_keys(true);
        let mut connection = sqlx::SqliteConnection::connect_with(&options)
            .await
            .unwrap();
        raw_sql(V4_FIXTURE).execute(&mut connection).await.unwrap();
        raw_sql("DELETE FROM historical_question_artifacts; DELETE FROM persisted_artifacts;")
            .execute(&mut connection)
            .await
            .unwrap();
        let source = manifest(&mut connection, &SOURCE_TABLE_MANIFESTS)
            .await
            .unwrap();
        let journal_mode: String = sqlx::query_scalar("PRAGMA journal_mode = WAL")
            .fetch_one(&mut connection)
            .await
            .unwrap();
        assert_eq!(journal_mode, "wal");
        sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
            .fetch_all(&mut connection)
            .await
            .unwrap();
        connection.close().await.unwrap();
        assert_eq!(&std::fs::read(&path).unwrap()[18..20], &[2, 2]);
        for suffix in ["-wal", "-shm", "-journal"] {
            assert!(!std::path::PathBuf::from(format!("{}{}", path.display(), suffix)).exists());
        }
        let receipt = migrate_disposable_v4(MigrationRequest {
            path: &path,
            expected_source_manifest_digest: source,
            started_at: AT,
            committed_at: AT,
            backup_id: None,
            failure_point: FailurePoint::None,
        })
        .await
        .unwrap();
        activate_lifecycle_writes(&path, &receipt, AT)
            .await
            .unwrap();
        for suffix in ["-wal", "-shm", "-journal"] {
            assert!(!std::path::PathBuf::from(format!("{}{}", path.display(), suffix)).exists());
        }
        (directory, path)
    }

    fn add_runtime_recovery(mut bundle: Value) -> Value {
        bundle["recoveryTurns"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "id": "runtime-recovery",
                "sourceEntryId": SOURCE_ID,
                "question": "What happened before you returned to the task?",
                "status": "suggested",
                "locale": "en",
                "promptProvenance": {
                    "origin": "local_mock", "sourceEntryId": SOURCE_ID,
                    "sourceArtifactIds": [], "provider": "mock", "model": null,
                    "harnessVersion": "harness-v1", "promptVersion": "v1",
                    "generatedAt": AT
                },
                "createdAt": AT, "updatedAt": AT
            }));
        bundle
    }

    #[tokio::test]
    async fn typed_experience_route_preserves_v5_authority_and_projection() {
        let (_directory, path) = fixture().await;
        let created = create_experience(
            &path,
            "runtime-created",
            "A real moment",
            AT,
            "guard-create-0123456789abcdef0123456789abcdef",
        )
        .await
        .unwrap();
        assert_eq!(created.body, "A real moment");
        let updated_at = "2026-08-13T12:00:01.000Z";
        let updated = update_experience(
            &path,
            &created.id,
            &created.updated_at,
            "A corrected moment",
            updated_at,
            "guard-update-0123456789abcdef0123456789abcdef",
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(updated.body, "A corrected moment");
        assert!(update_experience(
            &path,
            &created.id,
            &created.updated_at,
            "stale",
            "2026-08-13T12:00:02.000Z",
            "guard-stale-0123456789abcdef0123456789abcdef"
        )
        .await
        .unwrap()
        .is_none());
        verify_activated_v5_runtime(&path).await.unwrap();
    }

    #[tokio::test]
    async fn typed_evidence_batch_and_review_route_is_atomic_and_stale_safe() {
        let (_directory, path) = fixture().await;
        let current = list_artifacts(&path, SOURCE_ID).await.unwrap();
        let mut desired = current.clone();
        desired["evidence"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "id": "runtime-evidence",
                "sourceEntryId": SOURCE_ID,
                "text": "A bounded observation",
                "originalText": "A bounded observation",
                "kind": "observation",
                "userEditable": true,
                "status": "candidate",
                "provenance": {
                    "origin": "local_mock", "sourceEntryId": SOURCE_ID,
                    "sourceArtifactIds": [], "provider": "mock", "model": null,
                    "harnessVersion": "harness-v1", "promptVersion": "evidence-v1",
                    "generatedAt": AT
                },
                "createdAt": AT, "updatedAt": AT
            }));
        let source = get_experience(&path, SOURCE_ID).await.unwrap().unwrap();
        let result = save_artifacts(
            &path,
            SOURCE_ID,
            desired.clone(),
            Some(&source.updated_at),
            AT,
            "guard-evidence-0123456789abcdef0123456789abcdef",
        )
        .await
        .unwrap();
        assert_eq!(result.status, "committed");
        let confirmed = desired["evidence"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["id"] == "runtime-evidence")
            .unwrap();
        confirmed["status"] = Value::String("confirmed".into());
        confirmed["updatedAt"] = Value::String("2026-08-13T12:00:01.000Z".into());
        let result = save_artifacts(
            &path,
            SOURCE_ID,
            desired,
            Some(&source.updated_at),
            "2026-08-13T12:00:01.000Z",
            "guard-confirm-0123456789abcdef0123456789abcdef",
        )
        .await
        .unwrap();
        assert_eq!(result.status, "committed");
        verify_activated_v5_runtime(&path).await.unwrap();
    }

    #[tokio::test]
    async fn typed_context_recovery_route_creates_and_answers_through_real_facade() {
        let (_directory, path) = fixture().await;
        let source = get_experience(&path, SOURCE_ID).await.unwrap().unwrap();
        let suggested = add_runtime_recovery(list_artifacts(&path, SOURCE_ID).await.unwrap());
        let created = save_artifacts(
            &path,
            SOURCE_ID,
            suggested,
            Some(&source.updated_at),
            AT,
            "guard-recovery-create-0123456789abcdef01234567",
        )
        .await
        .unwrap();
        assert_eq!(created.status, "committed");

        let answer_at = "2026-08-13T12:00:01.000Z";
        let mut answered = list_artifacts(&path, SOURCE_ID).await.unwrap();
        let turn = answered["recoveryTurns"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["id"] == "runtime-recovery")
            .unwrap();
        turn["status"] = Value::String("answered".into());
        turn["response"] = Value::String("I paused to remember where I put it.".into());
        turn["responseProvenance"] = serde_json::json!({
            "origin": "user", "sourceEntryId": SOURCE_ID,
            "sourceArtifactIds": ["runtime-recovery"], "generatedAt": answer_at
        });
        turn["updatedAt"] = Value::String(answer_at.into());
        let result = save_artifacts(
            &path,
            SOURCE_ID,
            answered.clone(),
            Some(&source.updated_at),
            answer_at,
            "guard-recovery-answer-0123456789abcdef01234567",
        )
        .await
        .unwrap();
        assert_eq!(result.status, "committed");
        let projected = &result.bundle["recoveryTurns"][0];
        assert_eq!(projected["status"], "answered");
        assert_eq!(
            projected["response"],
            "I paused to remember where I put it."
        );
        assert_eq!(projected["responseProvenance"]["origin"], "user");
        assert!(projected["responseProvenance"]["provider"].is_null());

        let mut connection = connect(&path, true).await.unwrap();
        let head: (String, String, String) = sqlx::query_as(
            "SELECT review_state, eligibility_state, eligibility_reason FROM artifact_heads \
             WHERE id = 'runtime-recovery' AND artifact_kind = 'recovery_turn'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(
            head,
            (
                "not_applicable".into(),
                "eligible".into(),
                "context_recovery_answered_current_experience_task_only".into(),
            )
        );
        let revision_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id = 'runtime-recovery'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(revision_count, 2);
        connection.close().await.unwrap();
        verify_activated_v5_runtime(&path).await.unwrap();
    }

    #[tokio::test]
    async fn typed_context_recovery_route_rejects_contradictory_user_provenance_without_write() {
        let (_directory, path) = fixture().await;
        let source = get_experience(&path, SOURCE_ID).await.unwrap().unwrap();
        let suggested = add_runtime_recovery(list_artifacts(&path, SOURCE_ID).await.unwrap());
        save_artifacts(
            &path,
            SOURCE_ID,
            suggested,
            Some(&source.updated_at),
            AT,
            "guard-recovery-create-abcdef0123456789abcdef01",
        )
        .await
        .unwrap();

        let answer_at = "2026-08-13T12:00:01.000Z";
        let mut answered = list_artifacts(&path, SOURCE_ID).await.unwrap();
        let turn = &mut answered["recoveryTurns"][0];
        turn["status"] = Value::String("answered".into());
        turn["response"] = Value::String("A bounded answer".into());
        turn["responseProvenance"] = serde_json::json!({
            "origin": "user", "sourceEntryId": "different-source",
            "sourceArtifactIds": ["runtime-recovery"], "generatedAt": answer_at
        });
        turn["updatedAt"] = Value::String(answer_at.into());
        let error = save_artifacts(
            &path,
            SOURCE_ID,
            answered,
            Some(&source.updated_at),
            answer_at,
            "guard-recovery-invalid-0123456789abcdef012345",
        )
        .await
        .unwrap_err();
        assert_eq!(
            error.code,
            "founder_runtime_recovery_response_provenance_mismatch"
        );
        let durable = list_artifacts(&path, SOURCE_ID).await.unwrap();
        assert_eq!(durable["recoveryTurns"][0]["status"], "suggested");
        let mut connection = connect(&path, true).await.unwrap();
        let revision_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id = 'runtime-recovery'",
        )
        .fetch_one(&mut connection)
        .await
        .unwrap();
        assert_eq!(revision_count, 1);
        connection.close().await.unwrap();
    }
}
