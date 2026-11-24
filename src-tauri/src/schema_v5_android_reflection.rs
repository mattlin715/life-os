//! Synthetic M2-C direct-origin adapter. No alternate schema or lifecycle policy.
use super::evidence_write::{
    self, EvidenceCandidateInput, EvidenceProvenanceInput, EvidenceWriteCommand as E,
    EvidenceWriteContext, EvidenceWriteFailurePoint,
};
use super::reflection_write::{
    self, EvidenceRevisionRef, PromptProvenanceInput, ReflectionWriteCommand as R,
    ReflectionWriteContext, ReflectionWriteFailurePoint, SuggestedPromptInput,
};
use super::{connect, sha256_hex, MigrationError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Connection, Row};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EvidenceRef {
    pub(crate) artifact_id: String,
    pub(crate) revision_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ArtifactRequest {
    pub(crate) request_id: String,
    pub(crate) operation: String,
    pub(crate) source_id: String,
    pub(crate) expected_source_revision_id: String,
    pub(crate) artifact_id: String,
    pub(crate) expected_artifact_revision_id: Option<String>,
    pub(crate) evidence: Vec<EvidenceRef>,
    pub(crate) text: Option<String>,
    pub(crate) locale: String,
    pub(crate) occurred_at: String,
}

fn fail(code: &str) -> MigrationError {
    MigrationError::fail_closed(code)
}

pub(crate) fn request_identity(r: &ArtifactRequest) -> String {
    let refs: Vec<(&str, &str)> = r
        .evidence
        .iter()
        .map(|v| (v.artifact_id.as_str(), v.revision_id.as_str()))
        .collect();
    let tuple = json!([
        "life-os/m2c-artifact-request-v1",
        r.operation,
        r.source_id,
        r.expected_source_revision_id,
        r.artifact_id,
        r.expected_artifact_revision_id,
        refs,
        r.text.as_deref().map(|v| sha256_hex(v.as_bytes())),
        r.locale,
        r.occurred_at
    ]);
    format!(
        "m2c_{}",
        sha256_hex(serde_json::to_string(&tuple).unwrap().as_bytes())
    )
}

fn validate(r: &ArtifactRequest) -> Result<(), MigrationError> {
    if request_identity(r) != r.request_id {
        return Err(fail("m2c_request_identity_conflict"));
    }
    let mut identifiers = vec![&r.source_id, &r.artifact_id, &r.expected_source_revision_id];
    if let Some(id) = &r.expected_artifact_revision_id {
        identifiers.push(id)
    }
    for edge in &r.evidence {
        identifiers.push(&edge.artifact_id);
        identifiers.push(&edge.revision_id)
    }
    for id in identifiers {
        if id.is_empty()
            || id.len() > 100
            || !id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err(fail("m2c_identifier_invalid"));
        }
    }
    let format = time::macros::format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
    );
    if r.occurred_at.len() != 24 || time::PrimitiveDateTime::parse(&r.occurred_at, format).is_err()
    {
        return Err(fail("m2c_timestamp_invalid"));
    }
    if !matches!(r.locale.as_str(), "en" | "zh-TW" | "ja") {
        return Err(fail("m2c_locale_invalid"));
    }
    if r.text
        .as_ref()
        .is_some_and(|v| v.trim().is_empty() || v.len() > 65536)
    {
        return Err(fail("m2c_text_invalid"));
    }
    if r.evidence.len() > 1 {
        return Err(fail("m2c_evidence_scope_refused"));
    }
    let create = matches!(r.operation.as_str(), "candidate" | "question");
    if create != r.expected_artifact_revision_id.is_none() {
        return Err(fail("m2c_revision_shape_invalid"));
    }
    match r.operation.as_str() {
        "candidate" if r.text.is_none() && r.evidence.is_empty() => (),
        "correct" if r.text.is_some() && r.evidence.is_empty() => (),
        "confirm" | "reject" if r.text.is_none() && r.evidence.is_empty() => (),
        "question" if r.text.is_some() && r.evidence.len() == 1 => (),
        #[cfg(feature = "android-pilot-online")]
        "correctResponse" if r.text.is_some() && r.evidence.is_empty() => (),
        "answer" | "correctResponse" if r.text.is_some() && r.evidence.len() == 1 => (),
        "skip" if r.text.is_none() && r.evidence.len() == 1 => (),
        _ => return Err(fail("m2c_operation_shape_refused")),
    }
    Ok(())
}

#[cfg(feature = "android-pilot-online")]
pub(crate) const MODEL_TRIAL_MODEL: &str = "gemini-3.1-flash-lite";
#[cfg(feature = "android-pilot-online")]
const MODEL_TRIAL_PROMPT: &str = "android-gemini-reflection-v1";
#[cfg(feature = "android-pilot-online")]
pub(crate) const MODEL_SOURCE_PROMPT: &str = "android-gemini-reflection-v2";

/// Refuse unknown/cross-source dependency graphs before any mutation. This is
/// scope admission, not a substitute for canonical projection verification.
pub(crate) async fn verify_scope(path: &Path) -> Result<(), MigrationError> {
    let mut c = connect(path, true).await?;
    super::pattern_write::verify_exact_pattern_v5_direct(&mut c).await?;
    super::context_recovery_write::verify_exact_context_recovery_v5_direct(&mut c).await?;
    let unsupported: i64=sqlx::query_scalar(
        "SELECT (SELECT COUNT(*) FROM artifact_heads WHERE artifact_kind NOT IN ('evidence','reflection')) + \
         (SELECT COUNT(*) FROM historical_question_artifacts) + (SELECT COUNT(*) FROM historical_artifact_dependencies) + \
         (SELECT COUNT(*) FROM historical_question_lifecycle_links) + \
         (SELECT COUNT(*) FROM artifact_dependencies d \
          LEFT JOIN artifact_heads h ON h.id=d.dependent_artifact_id \
          LEFT JOIN artifact_heads a ON a.id=d.source_artifact_id \
          LEFT JOIN source_revisions s ON s.id=d.source_revision_id \
          WHERE h.id IS NULL OR \
          (d.source_revision_id IS NOT NULL AND (s.id IS NULL OR s.source_id<>h.source_id)) OR \
          (d.source_artifact_id IS NOT NULL AND (a.id IS NULL OR a.source_id<>h.source_id OR h.artifact_kind<>'reflection' OR NOT \
              ((d.relationship_type='uses_evidence' AND a.artifact_kind='evidence') OR \
               (d.relationship_type='answers_prompt' AND a.id=h.id AND a.artifact_kind='reflection')))) OR \
          (d.source_revision_id IS NOT NULL AND d.relationship_type<>'derived_from_experience') OR \
          (d.source_revision_id IS NULL AND d.source_artifact_id IS NULL))")
        .fetch_one(&mut c).await.map_err(|_|fail("m2c_scope_inventory_failed"))?;
    if unsupported != 0 {
        return Err(fail("m2c_unsupported_dependency_preserved"));
    }
    #[cfg(feature = "android-pilot-online")]
    let unexpected_trial_role: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM artifact_revision_provenance ap
         JOIN provenance_records p ON p.id=ap.provenance_id
         JOIN artifact_revisions r ON r.id=ap.artifact_revision_id
         JOIN artifact_heads h ON h.id=r.artifact_id
         WHERE json_extract(p.canonical_payload,'$.origin')='ai'
         AND (ap.role<>'prompt' OR h.artifact_kind<>'reflection')",
    ).fetch_one(&mut c).await.map_err(|_| fail("m2c_provenance_inventory_failed"))?;
    #[cfg(feature = "android-pilot-online")]
    if unexpected_trial_role != 0 {
        return Err(fail("m2c_provenance_unknown_preserved"));
    }
    let provenance: Vec<String> =
        sqlx::query_scalar("SELECT canonical_payload FROM provenance_records")
            .fetch_all(&mut c)
            .await
            .map_err(|_| fail("m2c_provenance_inventory_failed"))?;
    for raw in provenance {
        let p: Value =
            serde_json::from_str(&raw).map_err(|_| fail("m2c_provenance_unknown_preserved"))?;
        match p["origin"].as_str() {
            Some("user") => (),
            Some("local_mock")
                if p["provider"] == "mock"
                    && p["model"].is_null()
                    && p["harnessVersion"] == "harness-v1"
                    && p["promptVersion"] == "v1" =>
            {
                ()
            }
            #[cfg(feature = "android-pilot-online")]
            Some("ai")
                if p["provider"] == "gemini" && p["model"] == MODEL_TRIAL_MODEL
                    && p["harnessVersion"] == "harness-v1" && matches!(p["promptVersion"].as_str(), Some(MODEL_TRIAL_PROMPT | MODEL_SOURCE_PROMPT)) =>
            {
                ()
            }
            _ => return Err(fail("m2c_provenance_unknown_preserved")),
        }
    }
    c.close().await.map_err(|_| fail("m2c_read_close_failed"))?;
    Ok(())
}

/// Heads qualify compatibility payloads. Invalidated/rejected/deleted content
/// can never be inferred eligible from a stale domain status alone.
pub(crate) async fn snapshot(path: &Path, source_id: &str) -> Result<Value, MigrationError> {
    verify_scope(path).await?;
    let mut c = connect(path, true).await?;
    let rows=sqlx::query("SELECT h.id,h.artifact_kind,h.current_revision_id,h.review_state,h.lifecycle_state,h.eligibility_state,p.payload \
        FROM artifact_heads h LEFT JOIN persisted_artifacts p ON p.id=h.id WHERE h.source_id=? ORDER BY h.created_at,h.id")
        .bind(source_id).fetch_all(&mut c).await.map_err(|_|fail("m2c_snapshot_failed"))?;
    let mut result = Vec::new();
    for row in rows {
        let raw: Option<String> = row.get(6);
        let payload = raw
            .map(|v| serde_json::from_str::<Value>(&v))
            .transpose()
            .map_err(|_| fail("m2c_projection_invalid"))?;
        result.push(json!({"id":row.get::<String,_>(0),"kind":row.get::<String,_>(1),"revisionId":row.get::<Option<String>,_>(2),
            "reviewState":row.get::<String,_>(3),"lifecycleState":row.get::<String,_>(4),"eligibilityState":row.get::<String,_>(5),"payload":payload}));
    }
    c.close().await.map_err(|_| fail("m2c_read_close_failed"))?;
    Ok(Value::Array(result))
}

#[cfg(feature = "android-pilot-online")]
pub(crate) struct ModelTrialWrite<'a> {
    pub source_id: &'a str,
    pub source_revision: &'a str,
    pub clue: Option<(&'a str, &'a str)>,
    pub artifact_id: &'a str,
    pub question: &'a str,
    pub generated_at: &'a str,
    pub response: &'a str,
    pub saved_at: &'a str,
}

#[cfg(feature = "android-pilot-online")]
pub(crate) async fn save_model_trial(path: &Path, write: ModelTrialWrite<'_>, rollback: bool) -> Result<(), MigrationError> {
    let rows = snapshot(path, write.source_id).await?;
    let rows = rows.as_array().ok_or_else(|| fail("m2c_model_snapshot_invalid"))?;
    let current = super::runtime::direct_source_snapshot(path, write.source_id).await?
        .ok_or_else(|| fail("m2c_stale_source_preserved"))?;
    if current.revision_id != write.source_revision ||
        write.clue.is_some_and(|(id, revision)| !rows.iter().any(|v| v["id"] == id && v["revisionId"] == revision
            && v["kind"] == "evidence" && v["reviewState"] == "confirmed"
            && v["lifecycleState"] == "active" && v["eligibilityState"] == "eligible")) {
        return Err(fail("m2c_stale_revision_preserved"));
    }
    // A retry must match the original pair, never a different answer or a
    // response edited later. Check the writer's role-qualified projection.
    let refs: Vec<&str> = write.clue.iter().map(|(id, _)| *id).collect();
    let prompt_version = if write.clue.is_some() { MODEL_TRIAL_PROMPT } else { MODEL_SOURCE_PROMPT };
    let exact_answer = |v: &Value| {
        let p = &v["payload"];
        v["id"] == write.artifact_id && v["kind"] == "reflection" && v["lifecycleState"] == "active"
            && v["reviewState"] == "not_applicable" && v["eligibilityState"] == "eligible"
            && p["status"] == "answered" && p["question"] == write.question && p["response"] == write.response
            && p["sourceEvidenceIds"] == json!(refs)
            && p["promptProvenance"]["origin"] == "ai" && p["promptProvenance"]["provider"] == "gemini"
            && p["promptProvenance"]["model"] == MODEL_TRIAL_MODEL
            && p["promptProvenance"]["promptVersion"] == prompt_version
            && p["promptProvenance"]["generatedAt"] == write.generated_at
            && p["responseProvenance"]["origin"] == "user"
    };
    if let Some(existing) = rows.iter().find(|v| v["id"] == write.artifact_id) {
        return if exact_answer(existing) { Ok(()) } else { Err(fail("m2c_model_identity_conflict")) };
    }
    let prompt = SuggestedPromptInput {
        id: write.artifact_id.into(), source_id: write.source_id.into(),
        question: write.question.into(), created_at: write.generated_at.into(),
        evidence: write.clue.into_iter().map(|(id, revision)| EvidenceRevisionRef { artifact_id: id.into(), revision_id: revision.into() }).collect(),
        provenance: PromptProvenanceInput {
            origin: "ai".into(), provider: "gemini".into(), model: Some(MODEL_TRIAL_MODEL.into()),
            harness_version: "harness-v1".into(), prompt_version: prompt_version.into(),
            generated_at: write.generated_at.into(), source_artifact_ids: refs.iter().map(|value| (*value).to_owned()).collect(),
        },
    };
    let guard = format!("m3-model-save-{}", sha256_hex(write.artifact_id.as_bytes()));
    reflection_write::execute_direct_fresh(path, R::CreateAnsweredModelTrial {
        expected_source_revision_id: write.source_revision.into(), prompt, response: write.response.into(),
    }, ReflectionWriteContext {
        occurred_at: write.saved_at, guard_token: &guard,
        failure_point: if rollback { ReflectionWriteFailurePoint::AfterReconciliation } else { ReflectionWriteFailurePoint::None },
    }).await?;
    if !snapshot(path, write.source_id).await?.as_array().is_some_and(|rows| rows.iter().any(exact_answer)) {
        return Err(fail("m2c_model_save_not_verified"));
    }
    Ok(())
}

fn quote(body: &str, locale: &str) -> String {
    let label = match locale {
        "zh-TW" => "你寫下的原文",
        "ja" => "あなたが書いた原文",
        _ => "Your own words",
    };
    format!("{label}:\n{}", body.trim())
}
fn question(locale: &str) -> &'static str {
    match locale {
        "zh-TW" => "再次閱讀這段紀錄時，什麼最讓你留意？",
        "ja" => "この記録を読み返すと、何が最も気になりますか？",
        _ => "What stands out when you read this evidence again?",
    }
}

fn suggested_input(r: &ArtifactRequest) -> SuggestedPromptInput {
    SuggestedPromptInput {
        id: r.artifact_id.clone(),
        source_id: r.source_id.clone(),
        question: question(&r.locale).into(),
        created_at: r.occurred_at.clone(),
        evidence: r
            .evidence
            .iter()
            .map(|v| EvidenceRevisionRef {
                artifact_id: v.artifact_id.clone(),
                revision_id: v.revision_id.clone(),
            })
            .collect(),
        provenance: PromptProvenanceInput {
            origin: "local_mock".into(),
            provider: "mock".into(),
            model: None,
            harness_version: "harness-v1".into(),
            prompt_version: "v1".into(),
            generated_at: r.occurred_at.clone(),
            source_artifact_ids: r.evidence.iter().map(|v| v.artifact_id.clone()).collect(),
        },
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct QuestionBatchItem {
    artifact_id: String,
    evidence: EvidenceRef,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct QuestionBatchRequest {
    request_id: String,
    source_id: String,
    expected_source_revision_id: String,
    expected_reflection_set: String,
    items: Vec<QuestionBatchItem>,
    locale: String,
    occurred_at: String,
}

fn question_batch_identity(r: &QuestionBatchRequest) -> String {
    let items: Vec<_> = r
        .items
        .iter()
        .map(|v| {
            (
                &v.artifact_id,
                &v.evidence.artifact_id,
                &v.evidence.revision_id,
            )
        })
        .collect();
    let tuple = json!([
        "life-os/m2c-question-batch-v1",
        r.source_id,
        r.expected_source_revision_id,
        r.expected_reflection_set,
        items,
        r.locale,
        r.occurred_at
    ]);
    format!(
        "m2c_{}",
        sha256_hex(serde_json::to_string(&tuple).unwrap().as_bytes())
    )
}

fn batch_members(r: &QuestionBatchRequest) -> Result<Vec<ArtifactRequest>, MigrationError> {
    let unique: std::collections::HashSet<_> = r.items.iter().map(|v| &v.artifact_id).collect();
    if !(1..=3).contains(&r.items.len()) || unique.len() != r.items.len() {
        return Err(fail("m2c_batch_shape_refused"));
    }
    if question_batch_identity(r) != r.request_id {
        return Err(fail("m2c_request_identity_conflict"));
    }
    r.items
        .iter()
        .map(|item| {
            let mut member = ArtifactRequest {
                request_id: String::new(),
                operation: "question".into(),
                source_id: r.source_id.clone(),
                expected_source_revision_id: r.expected_source_revision_id.clone(),
                artifact_id: item.artifact_id.clone(),
                expected_artifact_revision_id: None,
                evidence: vec![item.evidence.clone()],
                text: Some(question(&r.locale).into()),
                locale: r.locale.clone(),
                occurred_at: r.occurred_at.clone(),
            };
            member.request_id = request_identity(&member);
            validate(&member)?;
            Ok(member)
        })
        .collect()
}

fn reflection_set(rows: &[Value]) -> String {
    let set: Vec<_> = rows
        .iter()
        .filter(|row| row["kind"] == "reflection")
        .map(|row| {
            json!([
                row["id"],
                row["revisionId"],
                row["reviewState"],
                row["lifecycleState"],
                row["eligibilityState"],
                row["payload"]["status"]
            ])
        })
        .collect();
    serde_json::to_string(&set).expect("serializing a JSON array of values")
}

async fn batch_snapshot(
    path: &Path,
    r: &QuestionBatchRequest,
) -> Result<(Vec<Value>, Vec<Value>), MigrationError> {
    let source = super::runtime::direct_source_snapshot(path, &r.source_id)
        .await?
        .ok_or_else(|| fail("m2c_stale_source_preserved"))?;
    if source.revision_id != r.expected_source_revision_id
        || r.occurred_at <= source.entry.updated_at
    {
        return Err(fail("m2c_stale_source_preserved"));
    }
    let Value::Array(rows) = snapshot(path, &r.source_id).await? else {
        return Err(fail("m2c_batch_snapshot_invalid"));
    };
    // Same ordering and bound as the existing local mock: one question per
    // eligible confirmed Evidence, at most the first three in snapshot order.
    let evidence = rows
        .iter()
        .filter(|row| {
            row["kind"] == "evidence"
                && row["lifecycleState"] == "active"
                && row["reviewState"] == "confirmed"
                && row["eligibilityState"] == "eligible"
                && row["revisionId"].is_string()
                && !row["payload"].is_null()
        })
        .take(3);
    if !evidence
        .map(|row| (row["id"].as_str(), row["revisionId"].as_str()))
        .eq(r.items.iter().map(|v| {
            (
                Some(v.evidence.artifact_id.as_str()),
                Some(v.evidence.revision_id.as_str()),
            )
        }))
    {
        return Err(fail("m2c_stale_evidence_preserved"));
    }
    let (added, retained): (Vec<_>, Vec<_>) = rows
        .into_iter()
        .partition(|row| r.items.iter().any(|v| row["id"] == v.artifact_id));
    if reflection_set(&retained) != r.expected_reflection_set {
        return Err(fail("m2c_stale_reflection_set_preserved"));
    }
    Ok((retained, added))
}

async fn verify_batch_items(
    path: &Path,
    added: &[Value],
    members: &[ArtifactRequest],
) -> Result<(), MigrationError> {
    if added.len() != members.len() {
        return Err(fail("m2c_batch_outcome_unconfirmed"));
    }
    for member in members {
        let Some(row) = added.iter().find(|v| v["id"] == member.artifact_id) else {
            return Err(fail("m2c_batch_outcome_unconfirmed"));
        };
        let refs = [&member.evidence[0].artifact_id];
        let expected = json!({
            "id": member.artifact_id, "sourceEntryId": member.source_id,
            "question": member.text, "sourceEvidenceIds": refs, "status": "suggested",
            "createdAt": member.occurred_at, "updatedAt": member.occurred_at,
            "promptProvenance": {
                "origin": "local_mock", "provider": "mock", "model": null,
                "harnessVersion": "harness-v1", "promptVersion": "v1",
                "generatedAt": member.occurred_at, "sourceEntryId": member.source_id, "sourceArtifactIds": refs
            }
        });
        if row["kind"] != "reflection"
            || row["lifecycleState"] != "active"
            || row["reviewState"] != "pending"
            || row["eligibilityState"] != "ineligible"
            || row["payload"] != expected
            || reconcile(path, member).await? != Some("alreadyCommitted")
        {
            return Err(fail("m2c_batch_outcome_unconfirmed"));
        }
    }
    Ok(())
}

/// Caller holds the M2-C operation lock from preflight through verified reread.
/// Replay never creates missing members of a partially visible or stale batch.
pub(crate) async fn generate_questions(
    path: &Path,
    r: &QuestionBatchRequest,
    rollback: bool,
) -> Result<Value, MigrationError> {
    let members = batch_members(r)?;
    let (retained, already_visible) = batch_snapshot(path, r).await?;
    if !already_visible.is_empty() {
        verify_batch_items(path, &already_visible, &members).await?;
        return Ok(json!({"acknowledgement":"alreadyCommitted","requestId":r.request_id}));
    }

    reflection_write::execute_direct_fresh(
        path,
        R::CreateSuggestedBatch {
            expected_source_revision_id: r.expected_source_revision_id.clone(),
            prompts: members.iter().map(suggested_input).collect(),
        },
        ReflectionWriteContext {
            occurred_at: &r.occurred_at,
            guard_token: &r.request_id,
            failure_point: if rollback {
                ReflectionWriteFailurePoint::AfterProjection
            } else {
                ReflectionWriteFailurePoint::None
            },
        },
    )
    .await?;

    let (actual_retained, added) = batch_snapshot(path, r).await?;
    if actual_retained != retained {
        return Err(fail("m2c_batch_retained_projection_changed"));
    }
    verify_batch_items(path, &added, &members).await?;
    Ok(json!({"acknowledgement":"committed","requestId":r.request_id}))
}

async fn exact_dependencies(
    c: &mut sqlx::SqliteConnection,
    r: &ArtifactRequest,
    revision: &str,
) -> Result<bool, MigrationError> {
    let sources:Vec<String>=sqlx::query_scalar("SELECT source_revision_id FROM artifact_dependencies WHERE dependent_revision_id=? AND source_revision_id IS NOT NULL")
        .bind(revision).fetch_all(&mut *c).await.map_err(|_|fail("m2c_dependencies_unreadable"))?;
    let refs:Vec<(String,String)>=sqlx::query_as("SELECT source_artifact_id,source_artifact_revision_id FROM artifact_dependencies WHERE dependent_revision_id=? AND relationship_type='uses_evidence' ORDER BY source_artifact_id")
        .bind(revision).fetch_all(&mut *c).await.map_err(|_|fail("m2c_dependencies_unreadable"))?;
    Ok(sources == vec![r.expected_source_revision_id.clone()]
        && refs
            == r.evidence
                .iter()
                .map(|v| (v.artifact_id.clone(), v.revision_id.clone()))
                .collect::<Vec<_>>())
}

/// Frozen semantic identity is checked against immutable revisions/review
/// events and exact dependencies. No content is returned in acknowledgements.
async fn reconcile(
    path: &Path,
    r: &ArtifactRequest,
) -> Result<Option<&'static str>, MigrationError> {
    let mut c = connect(path, true).await?;
    if matches!(r.operation.as_str(), "confirm" | "reject" | "skip") {
        let decision = match r.operation.as_str() {
            "confirm" => "confirmed",
            "reject" => "rejected",
            _ => "skipped",
        };
        let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM artifact_review_events e JOIN artifact_heads h ON h.id=e.artifact_id \
            WHERE e.artifact_id=? AND h.source_id=? AND e.subject_revision_id=? AND e.decision=? AND e.occurred_at=? AND e.actor='user' AND e.event_origin='explicit_user_action'")
            .bind(&r.artifact_id).bind(&r.source_id).bind(&r.expected_artifact_revision_id).bind(decision).bind(&r.occurred_at)
            .fetch_one(&mut c).await.map_err(|_|fail("m2c_reconciliation_failed"))?;
        if count == 1
            && exact_dependencies(
                &mut c,
                r,
                r.expected_artifact_revision_id.as_deref().unwrap(),
            )
            .await?
        {
            return Ok(Some("alreadyCommitted"));
        }
        return Ok(None);
    }
    let rows=sqlx::query("SELECT r.id,r.predecessor_revision_id,c.payload,h.current_revision_id,h.lifecycle_state \
        FROM artifact_revisions r JOIN artifact_heads h ON h.id=r.artifact_id LEFT JOIN artifact_revision_content c ON c.revision_id=r.id \
        WHERE r.artifact_id=? AND h.source_id=? AND r.created_at=?")
        .bind(&r.artifact_id).bind(&r.source_id).bind(&r.occurred_at).fetch_all(&mut c).await.map_err(|_|fail("m2c_reconciliation_failed"))?;
    for row in rows {
        let revision: String = row.get(0);
        let prior: Option<String> = row.get(1);
        if prior != r.expected_artifact_revision_id
            || !exact_dependencies(&mut c, r, &revision).await?
        {
            continue;
        }
        let raw: Option<String> = row.get(2);
        let Some(raw) = raw else {
            return Err(fail("m2c_prior_outcome_unconfirmed_preserved"));
        };
        let payload: Value =
            serde_json::from_str(&raw).map_err(|_| fail("m2c_reconciliation_failed"))?;
        let matches = match r.operation.as_str() {
            "candidate" => {
                let source: Option<String> = sqlx::query_scalar(
                    "SELECT content FROM source_revision_content WHERE revision_id=?",
                )
                .bind(&r.expected_source_revision_id)
                .fetch_optional(&mut c)
                .await
                .map_err(|_| fail("m2c_reconciliation_failed"))?;
                source.is_some_and(|v| payload["text"] == quote(&v, &r.locale))
            }
            "correct" => payload["text"].as_str() == r.text.as_deref(),
            "question" => payload["question"].as_str() == r.text.as_deref(),
            "answer" | "correctResponse" => {
                // Do not reconcile an initial save as a correction (or vice versa).
                let metadata: (String, String) = sqlx::query_as(
                    "SELECT revision_reason,authorship FROM artifact_revisions WHERE id=?",
                )
                .bind(&revision)
                .fetch_one(&mut c)
                .await
                .map_err(|_| fail("m2c_reconciliation_failed"))?;
                metadata.0
                    == if r.operation == "answer" {
                        "answered"
                    } else {
                        "corrected"
                    }
                    && metadata.1 == "mixed"
                    && payload["response"].as_str() == r.text.as_deref().map(str::trim)
            }
            _ => false,
        };
        if matches {
            let current: Option<String> = row.get(3);
            let state: String = row.get(4);
            return Ok(Some(
                if current.as_deref() == Some(&revision) && state == "active" {
                    "alreadyCommitted"
                } else {
                    "committedNotCurrent"
                },
            ));
        }
        return Err(fail("m2c_request_identity_conflict"));
    }
    Ok(None)
}

pub(crate) async fn mutate(
    path: &Path,
    r: &ArtifactRequest,
    rollback: bool,
) -> Result<Value, MigrationError> {
    validate(r)?;
    verify_scope(path).await?;
    if let Some(ack) = reconcile(path, r).await? {
        return Ok(json!({"acknowledgement":ack,"requestId":r.request_id}));
    }
    let source = super::runtime::direct_source_snapshot(path, &r.source_id)
        .await?
        .ok_or_else(|| fail("m2c_source_missing"))?;
    if source.revision_id != r.expected_source_revision_id
        || r.occurred_at <= source.entry.updated_at
    {
        return Err(fail("m2c_stale_source_preserved"));
    }
    let ec = EvidenceWriteContext {
        occurred_at: &r.occurred_at,
        guard_token: &r.request_id,
        failure_point: if rollback {
            EvidenceWriteFailurePoint::AfterProjection
        } else {
            EvidenceWriteFailurePoint::None
        },
    };
    let rc = ReflectionWriteContext {
        occurred_at: &r.occurred_at,
        guard_token: &r.request_id,
        failure_point: if rollback {
            ReflectionWriteFailurePoint::AfterProjection
        } else {
            ReflectionWriteFailurePoint::None
        },
    };
    let expected = r.expected_artifact_revision_id.clone().unwrap_or_default();
    match r.operation.as_str() {
        "candidate" => {
            let text = quote(&source.entry.body, &r.locale);
            // A verbatim self-report demonstration, not independently verified fact.
            evidence_write::execute_direct_fresh(
                path,
                E::Create {
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    candidate: EvidenceCandidateInput {
                        id: r.artifact_id.clone(),
                        source_id: r.source_id.clone(),
                        text: text.clone(),
                        original_text: text,
                        kind: "other".into(),
                        user_editable: true,
                        created_at: r.occurred_at.clone(),
                        provenance: EvidenceProvenanceInput {
                            origin: "local_mock".into(),
                            provider: "mock".into(),
                            model: None,
                            harness_version: "harness-v1".into(),
                            prompt_version: "v1".into(),
                            generated_at: r.occurred_at.clone(),
                            source_artifact_ids: vec![],
                        },
                    },
                },
                ec,
            )
            .await?;
        }
        "correct" => {
            evidence_write::execute_direct_fresh(
                path,
                E::CorrectPending {
                    source_id: r.source_id.clone(),
                    artifact_id: r.artifact_id.clone(),
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    expected_artifact_revision_id: expected,
                    text: r.text.clone().unwrap(),
                },
                ec,
            )
            .await?;
        }
        "confirm" => {
            evidence_write::execute_direct_fresh(
                path,
                E::ConfirmPending {
                    source_id: r.source_id.clone(),
                    artifact_id: r.artifact_id.clone(),
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    expected_artifact_revision_id: expected,
                },
                ec,
            )
            .await?;
        }
        "reject" => {
            evidence_write::execute_direct_fresh(
                path,
                E::RejectPending {
                    source_id: r.source_id.clone(),
                    artifact_id: r.artifact_id.clone(),
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    expected_artifact_revision_id: expected,
                },
                ec,
            )
            .await?;
        }
        "question" => {
            if r.text.as_deref() != Some(question(&r.locale)) {
                return Err(fail("m2c_mock_question_refused"));
            }
            reflection_write::execute_direct_fresh(
                path,
                R::CreateSuggested {
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    prompt: suggested_input(r),
                },
                rc,
            )
            .await?;
        }
        "answer" => {
            reflection_write::execute_direct_fresh(
                path,
                R::SaveResponse {
                    source_id: r.source_id.clone(),
                    artifact_id: r.artifact_id.clone(),
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    expected_artifact_revision_id: expected,
                    expected_evidence: r
                        .evidence
                        .iter()
                        .map(|v| EvidenceRevisionRef {
                            artifact_id: v.artifact_id.clone(),
                            revision_id: v.revision_id.clone(),
                        })
                        .collect(),
                    response: r.text.as_deref().unwrap().trim().into(),
                },
                rc,
            )
            .await?;
        }
        "skip" => {
            reflection_write::execute_direct_fresh(
                path,
                R::SkipSuggested {
                    source_id: r.source_id.clone(),
                    artifact_id: r.artifact_id.clone(),
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    expected_artifact_revision_id: expected,
                    expected_evidence: r
                        .evidence
                        .iter()
                        .map(|v| EvidenceRevisionRef {
                            artifact_id: v.artifact_id.clone(),
                            revision_id: v.revision_id.clone(),
                        })
                        .collect(),
                },
                rc,
            )
            .await?;
        }
        "correctResponse" => {
            reflection_write::execute_direct_fresh(
                path,
                R::CorrectResponse {
                    source_id: r.source_id.clone(),
                    artifact_id: r.artifact_id.clone(),
                    expected_source_revision_id: r.expected_source_revision_id.clone(),
                    expected_artifact_revision_id: expected,
                    expected_evidence: r
                        .evidence
                        .iter()
                        .map(|v| EvidenceRevisionRef {
                            artifact_id: v.artifact_id.clone(),
                            revision_id: v.revision_id.clone(),
                        })
                        .collect(),
                    response: r.text.as_deref().unwrap().trim().into(),
                },
                rc,
            )
            .await?;
        }
        _ => return Err(fail("m2c_operation_refused")),
    }
    verify_scope(path).await?;
    if reconcile(path, r).await?.is_none() {
        return Err(fail("m2c_commit_reread_unconfirmed"));
    }
    Ok(json!({"acknowledgement":"committed","requestId":r.request_id}))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SOURCE: &str = "m2c-synthetic-source";
    const BODY: &str =
        "  今天我感到緊張。\n誰かが『あなたは失敗する』と言った。I paused before replying.  ";
    async fn fixture() -> (tempfile::TempDir, std::path::PathBuf) {
        let root = tempfile::tempdir().unwrap();
        let path = crate::android_m2a::prepare_m2c_fixture(root.path().to_path_buf())
            .await
            .unwrap();
        super::super::runtime::create_experience_direct_fresh(
            &path,
            SOURCE,
            BODY,
            "2026-10-04T00:00:00.000Z",
            "m2c-fixture-source-create-000000000000001",
        )
        .await
        .unwrap();
        (root, path)
    }
    async fn request(
        path: &Path,
        op: &str,
        id: &str,
        second: u8,
        text: Option<&str>,
    ) -> ArtifactRequest {
        let source = super::super::runtime::direct_source_snapshot(path, SOURCE)
            .await
            .unwrap()
            .unwrap();
        let bundle = snapshot(path, SOURCE).await.unwrap();
        let target = bundle.as_array().unwrap().iter().find(|v| v["id"] == id);
        let refs = if matches!(op, "question" | "answer" | "correctResponse" | "skip") {
            bundle
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| {
                    v["kind"] == "evidence"
                        && v["reviewState"] == "confirmed"
                        && v["eligibilityState"] == "eligible"
                        && v["lifecycleState"] == "active"
                })
                .take(1)
                .map(|v| EvidenceRef {
                    artifact_id: v["id"].as_str().unwrap().into(),
                    revision_id: v["revisionId"].as_str().unwrap().into(),
                })
                .collect()
        } else {
            vec![]
        };
        let mut r = ArtifactRequest {
            request_id: String::new(),
            operation: op.into(),
            source_id: SOURCE.into(),
            expected_source_revision_id: source.revision_id,
            artifact_id: id.into(),
            expected_artifact_revision_id: target
                .and_then(|v| v["revisionId"].as_str())
                .map(str::to_string),
            evidence: refs,
            text: text.map(str::to_string),
            locale: "en".into(),
            occurred_at: format!("2026-10-04T00:00:{second:02}.000Z"),
        };
        r.request_id = request_identity(&r);
        r
    }
    async fn scalar(path: &Path, sql: &str) -> i64 {
        let mut c = connect(path, true).await.unwrap();
        sqlx::query_scalar(sql).fetch_one(&mut c).await.unwrap()
    }
    async fn review(path: &Path) {
        let r = request(path, "candidate", "m2c-evidence-001", 1, None).await;
        mutate(path, &r, false).await.unwrap();
        let r = request(path, "confirm", "m2c-evidence-001", 2, None).await;
        mutate(path, &r, false).await.unwrap();
    }

    #[cfg(feature = "android-pilot-online")]
    #[tokio::test]
    async fn model_trial_is_atomic_verified_and_idempotent() {
        let (_root, path) = fixture().await;
        review(&path).await;
        let binding = request(&path, "question", "m3-model-trial-001", 3, None).await;
        let make_write = |response| ModelTrialWrite {
            source_id: SOURCE, source_revision: &binding.expected_source_revision_id,
            clue: Some((&binding.evidence[0].artifact_id, &binding.evidence[0].revision_id)),
            artifact_id: &binding.artifact_id, question: "What felt different in that moment?",
            generated_at: "2026-10-04T00:00:03.000Z", response,
            saved_at: "2026-10-04T00:00:04.000Z",
        };
        let counts = |sql: &'static str| scalar(&path, sql);
        let before = snapshot(&path, SOURCE).await.unwrap();
        let revisions = counts("SELECT COUNT(*) FROM artifact_revisions").await;
        assert!(save_model_trial(&path, make_write("My answer."), true).await.is_err());
        assert_eq!(snapshot(&path, SOURCE).await.unwrap(), before);
        assert_eq!(counts("SELECT COUNT(*) FROM artifact_revisions").await, revisions);
        save_model_trial(&path, make_write("My answer."), false).await.unwrap();
        save_model_trial(&path, make_write("My answer."), false).await.unwrap();
        assert!(save_model_trial(&path, make_write("A different answer."), false).await.is_err());
        let rows = snapshot(&path, SOURCE).await.unwrap();
        let saved = rows.as_array().unwrap().iter().find(|v| v["id"] == binding.artifact_id).unwrap();
        assert_eq!(saved["payload"]["response"], "My answer.");
        assert_eq!(saved["payload"]["promptProvenance"]["provider"], "gemini");
        assert_eq!(saved["payload"]["responseProvenance"]["origin"], "user");
        assert_eq!(counts("SELECT COUNT(*) FROM artifact_revisions").await, revisions + 2);
        let wrong = ModelTrialWrite { clue: Some((&binding.evidence[0].artifact_id, "wrong-revision")), ..make_write("My answer.") };
        assert!(save_model_trial(&path, wrong, false).await.is_err());
    }
    #[cfg(feature = "android-pilot-online")]
    #[tokio::test]
    async fn model_trial_source_only_survives_reread_and_correction_then_invalidates_and_deletes() {
        let (_root, path) = fixture().await;
        let source = super::super::runtime::direct_source_snapshot(&path, SOURCE).await.unwrap().unwrap();
        let id = "m3-model-source-only-001";
        let input = || ModelTrialWrite {
            source_id: SOURCE, source_revision: &source.revision_id, clue: None, artifact_id: id,
            question: "What did you notice?", generated_at: "2026-10-04T00:00:01.000Z",
            response: "I felt relief.", saved_at: "2026-10-04T00:00:02.000Z",
        };
        assert!(save_model_trial(&path, input(), true).await.is_err());
        assert!(snapshot(&path, SOURCE).await.unwrap().as_array().unwrap().is_empty());
        save_model_trial(&path, input(), false).await.unwrap();
        save_model_trial(&path, input(), false).await.unwrap();
        let rows = snapshot(&path, SOURCE).await.unwrap();
        let pair = &rows[0];
        assert_eq!(rows.as_array().unwrap().len(), 1);
        assert_eq!(pair["payload"]["response"], "I felt relief.");
        assert_eq!(pair["payload"]["sourceEvidenceIds"], json!([]));
        assert_eq!(pair["payload"]["promptProvenance"]["promptVersion"], MODEL_SOURCE_PROMPT);
        assert_eq!(pair["payload"]["responseProvenance"]["origin"], "user");
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM artifact_dependencies WHERE relationship_type='derived_from_experience'").await, 2);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM artifact_dependencies WHERE relationship_type='uses_evidence'").await, 0);
        let mut r = request(&path, "correctResponse", id, 3, Some("I felt surprised.")).await;
        r.evidence.clear();
        r.request_id = request_identity(&r);
        assert_eq!(mutate(&path, &r, false).await.unwrap()["acknowledgement"], "committed");
        assert_eq!(snapshot(&path, SOURCE).await.unwrap()[0]["payload"]["response"], "I felt surprised.");
        assert!(save_model_trial(&path, input(), false).await.is_err());
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path, SOURCE, &source.revision_id, Some("The revised synthetic moment."),
            "2026-10-04T00:00:04.000Z", "m3-source-only-correction-guard-000001", false
        ).await.unwrap());
        assert_eq!(snapshot(&path, SOURCE).await.unwrap()[0]["lifecycleState"], "invalidated");
        assert!(save_model_trial(&path, input(), false).await.is_err());
        let revised = super::super::runtime::direct_source_snapshot(&path, SOURCE).await.unwrap().unwrap();
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path, SOURCE, &revised.revision_id, None,
            "2026-10-04T00:00:05.000Z", "m3-source-only-delete-guard-000001", false
        ).await.unwrap());
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM persisted_artifacts").await, 0);
        assert_eq!(scalar(&path, "SELECT COUNT(*) FROM artifact_revision_content").await, 0);
    }

    async fn answered_fixture() -> (tempfile::TempDir, std::path::PathBuf) {
        let (root, path) = fixture().await;
        review(&path).await;
        let q = request(
            &path,
            "question",
            "m2c-reflection-001",
            3,
            Some(question("en")),
        )
        .await;
        mutate(&path, &q, false).await.unwrap();
        let a = request(
            &path,
            "answer",
            "m2c-reflection-001",
            4,
            Some("Initial saved answer."),
        )
        .await;
        mutate(&path, &a, false).await.unwrap();
        (root, path)
    }
    async fn manifest(path: &Path) -> String {
        let mut c = connect(path, true).await.unwrap();
        let value = super::super::experience_write::operation_manifest(&mut c)
            .await
            .unwrap();
        c.close().await.unwrap();
        value
    }
    async fn batch_request(path: &Path, ids: &[&str], second: u8) -> QuestionBatchRequest {
        let source = super::super::runtime::direct_source_snapshot(path, SOURCE)
            .await
            .unwrap()
            .unwrap();
        let rows = snapshot(path, SOURCE).await.unwrap();
        let rows = rows.as_array().unwrap();
        let items = ids
            .iter()
            .zip(
                rows.iter()
                    .filter(|row| row["kind"] == "evidence" && row["reviewState"] == "confirmed"),
            )
            .map(|(id, row)| QuestionBatchItem {
                artifact_id: (*id).into(),
                evidence: EvidenceRef {
                    artifact_id: row["id"].as_str().unwrap().into(),
                    revision_id: row["revisionId"].as_str().unwrap().into(),
                },
            })
            .collect();
        let mut r = QuestionBatchRequest {
            request_id: String::new(),
            source_id: SOURCE.into(),
            expected_source_revision_id: source.revision_id,
            expected_reflection_set: reflection_set(rows),
            items,
            locale: "en".into(),
            occurred_at: format!("2026-10-04T00:00:{second:02}.000Z"),
        };
        r.request_id = question_batch_identity(&r);
        r
    }

    async fn review_second(path: &Path) {
        let r = request(path, "candidate", "m2c-evidence-002", 5, None).await;
        mutate(path, &r, false).await.unwrap();
        let r = request(path, "confirm", "m2c-evidence-002", 6, None).await;
        mutate(path, &r, false).await.unwrap();
    }

    #[tokio::test]
    async fn batch_create_append_reopen_and_parent_delete() {
        let (_root, path) = fixture().await;
        review(&path).await;
        review_second(&path).await;
        let first = batch_request(&path, &["m2c-prompt-z", "m2c-prompt-a"], 7).await;
        assert_eq!(
            generate_questions(&path, &first, false).await.unwrap()["acknowledgement"],
            "committed"
        );
        let first_manifest = manifest(&path).await;
        assert_eq!(
            generate_questions(&path, &first, false).await.unwrap()["acknowledgement"],
            "alreadyCommitted"
        );
        assert_eq!(manifest(&path).await, first_manifest);
        let first_read = snapshot(&path, SOURCE).await.unwrap();
        assert_eq!(first_read[2]["id"], "m2c-prompt-a"); // equal createdAt: SQLite id tie-break
        assert_eq!(first_read[3]["id"], "m2c-prompt-z");
        let save = request(
            &path,
            "answer",
            "m2c-prompt-z",
            8,
            Some("My original answer."),
        )
        .await;
        mutate(&path, &save, false).await.unwrap();

        let append = batch_request(&path, &["m2c-append-z", "m2c-append-a"], 9).await;
        let before = snapshot(&path, SOURCE).await.unwrap();
        let before_manifest = manifest(&path).await;
        assert!(generate_questions(&path, &append, true)
            .await
            .unwrap_err()
            .code
            .contains("injected"));
        assert_eq!(manifest(&path).await, before_manifest);
        assert_eq!(
            generate_questions(&path, &append, false).await.unwrap()["acknowledgement"],
            "committed"
        );
        let after = snapshot(&path, SOURCE).await.unwrap();
        let rows = after.as_array().unwrap();
        assert_eq!(&rows[..4], before.as_array().unwrap());
        assert_eq!(rows[4]["id"], "m2c-append-a");
        assert_eq!(rows[5]["id"], "m2c-append-z");
        assert_eq!(rows[3]["payload"]["response"], "My original answer.");
        assert_eq!(rows[3]["payload"]["responseProvenance"]["origin"], "user");
        assert_eq!(
            rows[3]["payload"]["promptProvenance"]["origin"],
            "local_mock"
        );

        let correction = request(
            &path,
            "correctResponse",
            "m2c-prompt-z",
            10,
            Some("My corrected answer."),
        )
        .await;
        mutate(&path, &correction, false).await.unwrap();
        assert_eq!(
            snapshot(&path, SOURCE).await.unwrap()[3]["payload"]["response"],
            "My corrected answer."
        );
        assert!(generate_questions(&path, &append, false)
            .await
            .unwrap_err()
            .code
            .contains("stale"));
        let rev = correction.expected_source_revision_id;
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path,
            SOURCE,
            &rev,
            None,
            "2026-10-04T00:00:11.000Z",
            "m2c-fixture-batch-parent-delete-001",
            false,
        )
        .await
        .unwrap());
        assert!(super::super::runtime::direct_source_snapshot(&path, SOURCE)
            .await
            .unwrap()
            .is_none());
        let deleted = snapshot(&path, SOURCE).await.unwrap();
        assert!(deleted
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["payload"].is_null()));
        assert!(generate_questions(&path, &append, false)
            .await
            .unwrap_err()
            .code
            .contains("stale"));
    }

    #[tokio::test]
    async fn batch_rejects_stale_skip_changed_evidence_and_partial_visibility() {
        let (_root, path) = fixture().await;
        review(&path).await;
        let first = batch_request(&path, &["m2c-first-prompt"], 3).await;
        let stale = batch_request(&path, &["m2c-next-prompt"], 7).await;
        generate_questions(&path, &first, false).await.unwrap();
        let before_skip = batch_request(&path, &["m2c-after-skip"], 7).await;
        let skip = request(&path, "skip", "m2c-first-prompt", 4, None).await;
        mutate(&path, &skip, false).await.unwrap();
        let baseline = manifest(&path).await;
        for r in [&stale, &before_skip] {
            assert_eq!(
                generate_questions(&path, r, false).await.unwrap_err().code,
                "m2c_stale_reflection_set_preserved"
            );
        }
        assert_eq!(manifest(&path).await, baseline);

        let before_evidence = batch_request(&path, &["m2c-next-prompt"], 7).await;
        review_second(&path).await;
        let baseline = manifest(&path).await;
        assert_eq!(
            generate_questions(&path, &before_evidence, false)
                .await
                .unwrap_err()
                .code,
            "m2c_stale_evidence_preserved"
        );
        assert_eq!(manifest(&path).await, baseline);

        let foreign = "m2c-synthetic-foreign";
        super::super::runtime::create_experience_direct_fresh(
            &path,
            foreign,
            BODY,
            "2026-10-04T00:00:00.000Z",
            "m2c-fixture-foreign-source-create-001",
        )
        .await
        .unwrap();
        let mut foreign_candidate =
            request(&path, "candidate", "m2c-other-source-id", 1, None).await;
        foreign_candidate.source_id = foreign.into();
        foreign_candidate.expected_source_revision_id =
            super::super::runtime::direct_source_snapshot(&path, foreign)
                .await
                .unwrap()
                .unwrap()
                .revision_id;
        foreign_candidate.request_id = request_identity(&foreign_candidate);
        mutate(&path, &foreign_candidate, false).await.unwrap();
        let collision =
            batch_request(&path, &["m2c-must-roll-back", "m2c-other-source-id"], 7).await;
        let before_collision = manifest(&path).await;
        // First member is legal, second collides globally. Nothing may commit.
        let error = generate_questions(&path, &collision, false)
            .await
            .unwrap_err();
        assert_eq!(error.code, "reflection_identity_conflict");
        assert_eq!(manifest(&path).await, before_collision);

        let batch = batch_request(&path, &["m2c-part-present", "m2c-part-absent"], 7).await;
        let members = batch_members(&batch).unwrap();
        mutate(&path, &members[0], false).await.unwrap();
        let partial_manifest = manifest(&path).await;
        assert_eq!(
            generate_questions(&path, &batch, false)
                .await
                .unwrap_err()
                .code,
            "m2c_batch_outcome_unconfirmed"
        );
        assert_eq!(manifest(&path).await, partial_manifest);
    }

    #[test]
    fn response_correction_identity_matches_typescript_contract() {
        let r = ArtifactRequest {
            request_id: String::new(),
            operation: "correctResponse".into(),
            source_id: "m2c-source-synthetic".into(),
            expected_source_revision_id: "v5sr_exact-source".into(),
            artifact_id: "m2c-response-synthetic".into(),
            expected_artifact_revision_id: Some("v5ar_answer".into()),
            evidence: vec![EvidenceRef {
                artifact_id: "m2c-evidence-synthetic".into(),
                revision_id: "v5ar_exact-evidence".into(),
            }],
            text: Some("changed".into()),
            locale: "en".into(),
            occurred_at: "2026-10-04T00:00:02.000Z".into(),
        };
        assert_eq!(
            request_identity(&r),
            "m2c_81dc40dbb5a20883d6413b865c2eb805e486b16c6c042b9af3757502c6bd5fcd"
        );
    }
    #[tokio::test]
    async fn response_correction_repeated_reopen_lineage_and_authorship() {
        let (root, path) = answered_fixture().await;
        let original = snapshot(&path, SOURCE).await.unwrap();
        let mut prior = original[1]["revisionId"].as_str().unwrap().to_string();
        let mut c = connect(&path, true).await.unwrap();
        let prompt_revision: String = sqlx::query_scalar("SELECT source_artifact_revision_id FROM artifact_dependencies WHERE dependent_revision_id=? AND relationship_type='answers_prompt'")
            .bind(&prior).fetch_one(&mut c).await.unwrap();
        c.close().await.unwrap();
        for (second, text) in [
            (5, "  第一次修正。First correction.  "),
            (6, "Second correction. 再び見直した。"),
        ] {
            let before = manifest(&path).await;
            snapshot(&path, SOURCE).await.unwrap(); // Opening/cancelling calls no writer.
            assert_eq!(manifest(&path).await, before);
            let r = request(
                &path,
                "correctResponse",
                "m2c-reflection-001",
                second,
                Some(text),
            )
            .await;
            assert_eq!(
                r.expected_artifact_revision_id.as_deref(),
                Some(prior.as_str())
            );
            assert_eq!(
                mutate(&path, &r, false).await.unwrap()["acknowledgement"],
                "committed"
            );
            let committed = manifest(&path).await;
            // Lost acknowledgement: the exact immutable request reconciles without a new revision.
            assert_eq!(
                mutate(&path, &r, false).await.unwrap()["acknowledgement"],
                "alreadyCommitted"
            );
            assert_eq!(manifest(&path).await, committed);
            assert_eq!(
                crate::android_m2a::prepare_m2c_fixture(root.path().to_path_buf())
                    .await
                    .unwrap(),
                path
            );
            let bundle = snapshot(&path, SOURCE).await.unwrap();
            assert_eq!(bundle[0], original[0]);
            for field in ["id", "reviewState", "eligibilityState"] {
                assert_eq!(bundle[1][field], original[1][field]);
            }
            for field in [
                "question",
                "promptProvenance",
                "sourceEvidenceIds",
                "createdAt",
            ] {
                assert_eq!(bundle[1]["payload"][field], original[1]["payload"][field]);
            }
            assert_eq!(bundle[1]["payload"]["response"], text.trim());
            assert_eq!(bundle[1]["payload"]["status"], "answered");
            assert_eq!(bundle[1]["payload"]["responseProvenance"]["origin"], "user");
            let source = super::super::runtime::direct_source_snapshot(&path, SOURCE)
                .await
                .unwrap()
                .unwrap();
            assert_eq!(source.revision_id, r.expected_source_revision_id);
            assert_eq!(source.entry.body, BODY);
            let current = bundle[1]["revisionId"].as_str().unwrap().to_string();
            let mut c = connect(&path, true).await.unwrap();
            let revision: (i64, String, String) = sqlx::query_as("SELECT revision_number,predecessor_revision_id,authorship FROM artifact_revisions WHERE id=?")
                .bind(&current).fetch_one(&mut c).await.unwrap();
            assert_eq!(
                revision,
                (i64::from(second) - 2, prior.clone(), "mixed".into())
            );
            let edge: String = sqlx::query_scalar("SELECT source_artifact_revision_id FROM artifact_dependencies WHERE dependent_revision_id=? AND relationship_type='answers_prompt'")
                .bind(&current).fetch_one(&mut c).await.unwrap();
            assert_eq!(edge, prompt_revision);
            assert!(exact_dependencies(&mut c, &r, &current).await.unwrap());
            c.close().await.unwrap();
            prior = current;
        }
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_revisions WHERE artifact_id='m2c-reflection-001'"
            )
            .await,
            4
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_revision_content").await,
            5
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM schema_migration_receipts").await,
            0
        );
        let stale = request(
            &path,
            "correctResponse",
            "m2c-reflection-001",
            9,
            Some("Draft before source changed."),
        )
        .await;
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path,
            SOURCE,
            &stale.expected_source_revision_id,
            Some("Changed original after two answer corrections."),
            "2026-10-04T00:00:07.000Z",
            "m2c-source-after-corrections-guard-00001",
            false
        )
        .await
        .unwrap());
        let invalidated = snapshot(&path, SOURCE).await.unwrap();
        assert!(
            invalidated
                .as_array()
                .unwrap()
                .iter()
                .all(|v| v["lifecycleState"] == "invalidated"
                    && v["eligibilityState"] == "ineligible")
        );
        let before = manifest(&path).await;
        assert!(mutate(&path, &stale, false).await.is_err());
        assert_eq!(manifest(&path).await, before);
        let source = super::super::runtime::direct_source_snapshot(&path, SOURCE)
            .await
            .unwrap()
            .unwrap();
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path,
            SOURCE,
            &source.revision_id,
            None,
            "2026-10-04T00:00:08.000Z",
            "m2c-delete-after-corrections-guard-00001",
            false
        )
        .await
        .unwrap());
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_revision_content").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM source_revision_content").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM persisted_artifacts").await,
            0
        );
    }
    #[tokio::test]
    async fn response_correction_rollback_unchanged_blank_stale_and_identity_guards() {
        let (_root, path) = answered_fixture().await;
        let before = manifest(&path).await;
        let r = request(
            &path,
            "correctResponse",
            "m2c-reflection-001",
            5,
            Some("Changed answer."),
        )
        .await;
        assert!(mutate(&path, &r, true)
            .await
            .unwrap_err()
            .code
            .contains("injected"));
        assert_eq!(manifest(&path).await, before);
        for text in [
            "Initial saved answer.",
            "  Initial saved answer.  ",
            "  \n  ",
        ] {
            let bad = request(
                &path,
                "correctResponse",
                "m2c-reflection-001",
                5,
                Some(text),
            )
            .await;
            assert!(mutate(&path, &bad, false).await.is_err());
            assert_eq!(manifest(&path).await, before);
        }
        for coordinate in ["source", "evidence", "artifact"] {
            let mut bad = r.clone();
            match coordinate {
                "source" => bad.expected_source_revision_id = "wrong-source-revision".into(),
                "artifact" => {
                    bad.expected_artifact_revision_id = Some("wrong-artifact-revision".into())
                }
                _ => bad.evidence[0].revision_id = "wrong-evidence-revision".into(),
            }
            bad.request_id = request_identity(&bad);
            assert!(mutate(&path, &bad, false).await.is_err());
            assert_eq!(manifest(&path).await, before);
        }
        let mut bad = r.clone();
        bad.operation = "answer".into();
        assert_eq!(
            mutate(&path, &bad, false).await.unwrap_err().code,
            "m2c_request_identity_conflict"
        );
        let stale = request(
            &path,
            "correctResponse",
            "m2c-reflection-001",
            6,
            Some("Stale later draft."),
        )
        .await;
        mutate(&path, &r, false).await.unwrap();
        let committed = manifest(&path).await;
        assert!(mutate(&path, &stale, false).await.is_err());
        assert_eq!(manifest(&path).await, committed);
        let mut bad = r.clone();
        bad.operation = "answer".into();
        bad.request_id = request_identity(&bad);
        assert!(mutate(&path, &bad, false).await.is_err());
        assert_eq!(manifest(&path).await, committed);
        let next = request(
            &path,
            "correctResponse",
            "m2c-reflection-001",
            7,
            Some("New explicit correction."),
        )
        .await;
        mutate(&path, &next, false).await.unwrap();
        let after = manifest(&path).await;
        assert_eq!(
            mutate(&path, &r, false).await.unwrap()["acknowledgement"],
            "committedNotCurrent"
        );
        assert_eq!(manifest(&path).await, after);
    }
    #[tokio::test]
    async fn response_correction_refused_after_evidence_loss_or_source_change() {
        let (_root, path) = answered_fixture().await;
        let r = request(
            &path,
            "correctResponse",
            "m2c-reflection-001",
            8,
            Some("Do not rebind draft."),
        )
        .await;
        // Owned adversarial fixture: no supported adopted-Evidence correction
        // is exposed by M2-C. Eligibility loss must fail closed, not rebind.
        let mut c = connect(&path, false).await.unwrap();
        sqlx::query("INSERT INTO v5_compatibility_write_guard(token,created_at) VALUES ('m2c-evidence-loss-fixture-000000001','2026-10-04T00:00:05.000Z')").execute(&mut c).await.unwrap();
        sqlx::query(
            "UPDATE artifact_heads SET eligibility_state='ineligible' WHERE id='m2c-evidence-001'",
        )
        .execute(&mut c)
        .await
        .unwrap();
        sqlx::query("DELETE FROM v5_compatibility_write_guard")
            .execute(&mut c)
            .await
            .unwrap();
        c.close().await.unwrap();
        let before = manifest(&path).await;
        assert!(mutate(&path, &r, false).await.is_err());
        assert_eq!(manifest(&path).await, before);
        let (_root, path) = answered_fixture().await;
        let r = request(
            &path,
            "correctResponse",
            "m2c-reflection-001",
            8,
            Some("Do not rebind draft."),
        )
        .await;
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path,
            SOURCE,
            &r.expected_source_revision_id,
            Some("Changed original source."),
            "2026-10-04T00:00:06.000Z",
            "m2c-source-change-guard-000000000000001",
            false
        )
        .await
        .unwrap());
        let before = manifest(&path).await;
        assert!(mutate(&path, &r, false).await.is_err());
        assert_eq!(manifest(&path).await, before);
    }
    #[tokio::test]
    async fn complete_direct_journey_restart_provenance_and_double_submissions() {
        let (root, path) = fixture().await;
        let candidate = request(&path, "candidate", "m2c-evidence-001", 1, None).await;
        assert_eq!(
            mutate(&path, &candidate, false).await.unwrap()["acknowledgement"],
            "committed"
        );
        assert_eq!(
            mutate(&path, &candidate, false).await.unwrap()["acknowledgement"],
            "alreadyCommitted"
        );
        let candidate_text = snapshot(&path, SOURCE).await.unwrap()[0]["payload"]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert_eq!(candidate_text, quote(BODY, "en"));
        assert!(!candidate_text.contains("Directly observable"));
        let confirm = request(&path, "confirm", "m2c-evidence-001", 2, None).await;
        mutate(&path, &confirm, false).await.unwrap();
        assert_eq!(
            mutate(&path, &confirm, false).await.unwrap()["acknowledgement"],
            "alreadyCommitted"
        );
        let q = request(
            &path,
            "question",
            "m2c-reflection-001",
            3,
            Some(question("en")),
        )
        .await;
        mutate(&path, &q, false).await.unwrap();
        let answer = request(
            &path,
            "answer",
            "m2c-reflection-001",
            4,
            Some("  我的意思：I chose a calm reply. 静かな選択。  "),
        )
        .await;
        mutate(&path, &answer, false).await.unwrap();
        assert_eq!(
            mutate(&path, &answer, false).await.unwrap()["acknowledgement"],
            "alreadyCommitted"
        );
        assert_eq!(
            crate::android_m2a::prepare_m2c_fixture(root.path().to_path_buf())
                .await
                .unwrap(),
            path
        );
        let bundle = snapshot(&path, SOURCE).await.unwrap();
        let prompt = &bundle[1]["payload"];
        assert_eq!(prompt["status"], "answered");
        assert_eq!(
            prompt["response"],
            "我的意思：I chose a calm reply. 静かな選択。"
        );
        assert_eq!(prompt["promptProvenance"]["origin"], "local_mock");
        assert_eq!(prompt["promptProvenance"]["provider"], "mock");
        assert!(prompt["promptProvenance"]["model"].is_null());
        assert_eq!(prompt["responseProvenance"]["origin"], "user");
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM schema_migration_receipts").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_revisions").await,
            3
        );
    }
    #[tokio::test]
    async fn pending_correction_fresh_review_stale_confirmation_and_rejection_purge() {
        let (_root, path) = fixture().await;
        let create = request(&path, "candidate", "m2c-evidence-001", 1, None).await;
        mutate(&path, &create, false).await.unwrap();
        let stale = request(&path, "confirm", "m2c-evidence-001", 3, None).await;
        let correction = request(
            &path,
            "correct",
            "m2c-evidence-001",
            2,
            Some("I felt nervous; this is my report, not a verified outside fact."),
        )
        .await;
        mutate(&path, &correction, false).await.unwrap();
        assert_eq!(
            snapshot(&path, SOURCE).await.unwrap()[0]["reviewState"],
            "pending"
        );
        assert!(mutate(&path, &stale, false).await.is_err());
        let reject = request(&path, "reject", "m2c-evidence-001", 4, None).await;
        mutate(&path, &reject, false).await.unwrap();
        assert_eq!(
            mutate(&path, &reject, false).await.unwrap()["acknowledgement"],
            "alreadyCommitted"
        );
        // ADR-0011 purges the rejected exact revision; the earlier superseded
        // revision stays context-ineligible until parent deletion.
        assert_eq!(scalar(&path,"SELECT COUNT(*) FROM artifact_revision_content c JOIN artifact_revisions r ON r.id=c.revision_id WHERE r.revision_number=2").await,0);
        assert_eq!(scalar(&path,"SELECT COUNT(*) FROM artifact_revision_content c JOIN artifact_revisions r ON r.id=c.revision_id WHERE r.revision_number=1").await,1);
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM persisted_artifacts").await,
            0
        );
        let q = request(
            &path,
            "question",
            "m2c-reflection-001",
            5,
            Some(question("en")),
        )
        .await;
        assert!(mutate(&path, &q, false).await.is_err());
    }
    #[tokio::test]
    async fn skip_drafts_and_no_action_are_not_user_response_context() {
        let (_root, path) = fixture().await;
        review(&path).await;
        let q = request(
            &path,
            "question",
            "m2c-reflection-001",
            3,
            Some(question("en")),
        )
        .await;
        mutate(&path, &q, false).await.unwrap();
        let before = scalar(&path, "SELECT COUNT(*) FROM artifact_revisions").await;
        // Reading/reopening and a local draft do not call a writer.
        snapshot(&path, SOURCE).await.unwrap();
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_revisions").await,
            before
        );
        let skip = request(&path, "skip", "m2c-reflection-001", 4, None).await;
        mutate(&path, &skip, false).await.unwrap();
        let bundle = snapshot(&path, SOURCE).await.unwrap();
        assert_eq!(bundle[1]["payload"]["status"], "skipped");
        assert!(bundle[1]["payload"].get("response").is_none());
        assert!(bundle[1]["payload"].get("responseProvenance").is_none());
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_revision_provenance WHERE role='response'"
            )
            .await,
            0
        );
    }
    #[tokio::test]
    async fn source_edit_invalidates_and_delete_atomically_purges_dependents() {
        let (_root, path) = fixture().await;
        review(&path).await;
        let q = request(
            &path,
            "question",
            "m2c-reflection-001",
            3,
            Some(question("en")),
        )
        .await;
        mutate(&path, &q, false).await.unwrap();
        let answer = request(
            &path,
            "answer",
            "m2c-reflection-001",
            4,
            Some("synthetic user response"),
        )
        .await;
        mutate(&path, &answer, false).await.unwrap();
        let source = super::super::runtime::direct_source_snapshot(&path, SOURCE)
            .await
            .unwrap()
            .unwrap();
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path,
            SOURCE,
            &source.revision_id,
            Some("corrected source"),
            "2026-10-04T00:00:05.000Z",
            "m2c-source-edit-guard-000000000000001",
            false
        )
        .await
        .unwrap());
        let bundle = snapshot(&path, SOURCE).await.unwrap();
        assert!(
            bundle
                .as_array()
                .unwrap()
                .iter()
                .all(|v| v["lifecycleState"] == "invalidated"
                    && v["eligibilityState"] == "ineligible")
        );
        assert!(mutate(&path, &q, false).await.is_ok()); // committed immutable request, no new generation
        let source = super::super::runtime::direct_source_snapshot(&path, SOURCE)
            .await
            .unwrap()
            .unwrap();
        assert!(super::super::runtime::mutate_experience_direct_fresh(
            &path,
            SOURCE,
            &source.revision_id,
            None,
            "2026-10-04T00:00:06.000Z",
            "m2c-source-delete-guard-000000000000001",
            false
        )
        .await
        .unwrap());
        verify_scope(&path).await.unwrap();
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_revision_content").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM source_revision_content").await,
            0
        );
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM persisted_artifacts").await,
            0
        );
    }
    #[tokio::test]
    async fn rollback_exact_prior_graph_and_request_identity_refusal() {
        let (_root, path) = fixture().await;
        review(&path).await;
        let q = request(
            &path,
            "question",
            "m2c-reflection-001",
            3,
            Some(question("en")),
        )
        .await;
        let before = snapshot(&path, SOURCE).await.unwrap();
        assert!(mutate(&path, &q, true).await.is_err());
        assert_eq!(snapshot(&path, SOURCE).await.unwrap(), before);
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM v5_compatibility_write_guard").await,
            0
        );
        let mut tampered = q.clone();
        tampered.text = Some("different content".into());
        assert_eq!(
            mutate(&path, &tampered, false).await.unwrap_err().code,
            "m2c_request_identity_conflict"
        );
        mutate(&path, &q, false).await.unwrap();
        let answer = request(
            &path,
            "answer",
            "m2c-reflection-001",
            4,
            Some("rollback synthetic answer"),
        )
        .await;
        let before = snapshot(&path, SOURCE).await.unwrap();
        assert!(mutate(&path, &answer, true).await.is_err());
        assert_eq!(snapshot(&path, SOURCE).await.unwrap(), before);
        mutate(&path, &answer, false).await.unwrap();
    }
    #[tokio::test]
    async fn foreign_evidence_and_cross_source_graph_refused_without_mutation() {
        let (_root, path) = fixture().await;
        review(&path).await;
        let other = "m2c-foreign-synthetic";
        super::super::runtime::create_experience_direct_fresh(
            &path,
            other,
            "Another synthetic source with enough context.",
            "2026-10-04T00:00:00.000Z",
            "m2c-foreign-source-create-000000000000001",
        )
        .await
        .unwrap();
        let foreign = super::super::runtime::direct_source_snapshot(&path, other)
            .await
            .unwrap()
            .unwrap();
        let mut q = request(
            &path,
            "question",
            "m2c-reflection-001",
            3,
            Some(question("en")),
        )
        .await;
        q.source_id = other.into();
        q.expected_source_revision_id = foreign.revision_id.clone();
        q.request_id = request_identity(&q);
        let before = snapshot(&path, SOURCE).await.unwrap();
        assert!(mutate(&path, &q, false).await.is_err());
        assert_eq!(snapshot(&path, SOURCE).await.unwrap(), before);
        let q = request(
            &path,
            "question",
            "m2c-reflection-001",
            4,
            Some(question("en")),
        )
        .await;
        mutate(&path, &q, false).await.unwrap();
        let rows = snapshot(&path, SOURCE).await.unwrap();
        let reflection_revision = rows[1]["revisionId"].as_str().unwrap();
        // An exact owned adversarial fixture, never a repair of unknown state.
        let mut c = connect(&path, false).await.unwrap();
        sqlx::query("INSERT INTO v5_compatibility_write_guard(token,created_at) VALUES ('m2c-cross-source-fixture-0000000000001','2026-10-04T00:00:05.000Z')").execute(&mut c).await.unwrap();
        sqlx::query("INSERT INTO artifact_dependencies(id,dependent_artifact_id,dependent_revision_id,relationship_type,source_revision_id,source_artifact_id,source_artifact_revision_id,created_at) VALUES ('m2c-cross-source-dependency','m2c-reflection-001',?,'derived_from_experience',?,NULL,NULL,'2026-10-04T00:00:05.000Z')")
            .bind(reflection_revision).bind(&foreign.revision_id).execute(&mut c).await.unwrap();
        sqlx::query("DELETE FROM v5_compatibility_write_guard")
            .execute(&mut c)
            .await
            .unwrap();
        c.close().await.unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert!(verify_scope(&path).await.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert_eq!(
            scalar(
                &path,
                "SELECT COUNT(*) FROM artifact_dependencies WHERE id='m2c-cross-source-dependency'"
            )
            .await,
            1
        );
    }
    #[tokio::test]
    async fn unsupported_question_and_stale_evidence_do_not_write_or_reassign() {
        let (_root, path) = fixture().await;
        review(&path).await;
        let mut q = request(
            &path,
            "question",
            "m2c-reflection-001",
            3,
            Some("This reveals who you really are."),
        )
        .await;
        let before = snapshot(&path, SOURCE).await.unwrap();
        assert_eq!(
            mutate(&path, &q, false).await.unwrap_err().code,
            "m2c_mock_question_refused"
        );
        assert_eq!(snapshot(&path, SOURCE).await.unwrap(), before);
        q.text = Some(question("en").into());
        q.evidence[0].revision_id = "v5ar_stale-evidence".into();
        q.request_id = request_identity(&q);
        assert!(mutate(&path, &q, false).await.is_err());
        assert_eq!(snapshot(&path, SOURCE).await.unwrap(), before);
        let mut unsupported = q.clone();
        unsupported.operation = "correctConfirmed".into();
        unsupported.request_id = request_identity(&unsupported);
        assert!(mutate(&path, &unsupported, false).await.is_err());
        assert_eq!(snapshot(&path, SOURCE).await.unwrap(), before);
    }
    #[tokio::test]
    async fn direct_origin_does_not_relax_migration_default_verification() {
        let (_root, path) = fixture().await;
        let source = super::super::runtime::direct_source_snapshot(&path, SOURCE)
            .await
            .unwrap()
            .unwrap();
        let candidate = EvidenceCandidateInput {
            id: "m2c-default-origin-refused".into(),
            source_id: SOURCE.into(),
            text: "synthetic quote".into(),
            original_text: "synthetic quote".into(),
            kind: "other".into(),
            user_editable: true,
            created_at: "2026-10-04T00:00:01.000Z".into(),
            provenance: EvidenceProvenanceInput {
                origin: "local_mock".into(),
                provider: "mock".into(),
                model: None,
                harness_version: "harness-v1".into(),
                prompt_version: "v1".into(),
                generated_at: "2026-10-04T00:00:01.000Z".into(),
                source_artifact_ids: vec![],
            },
        };
        let bytes = std::fs::read(&path).unwrap();
        assert!(evidence_write::execute_disposable(
            &path,
            E::Create {
                expected_source_revision_id: source.revision_id,
                candidate
            },
            EvidenceWriteContext {
                occurred_at: "2026-10-04T00:00:01.000Z",
                guard_token: "m2c-default-origin-guard-0000000000001",
                failure_point: EvidenceWriteFailurePoint::None
            }
        )
        .await
        .is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert_eq!(
            scalar(&path, "SELECT COUNT(*) FROM artifact_heads").await,
            0
        );
    }
}
