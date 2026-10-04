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
        "answer" if r.text.is_some() && r.evidence.len() == 1 => (),
        "skip" if r.text.is_none() && r.evidence.len() == 1 => (),
        _ => return Err(fail("m2c_operation_shape_refused")),
    }
    Ok(())
}

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
            "answer" => payload["response"].as_str() == r.text.as_deref().map(str::trim),
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
                    prompt: SuggestedPromptInput {
                        id: r.artifact_id.clone(),
                        source_id: r.source_id.clone(),
                        question: r.text.clone().unwrap(),
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
                            source_artifact_ids: r
                                .evidence
                                .iter()
                                .map(|v| v.artifact_id.clone())
                                .collect(),
                        },
                    },
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
        let refs = if matches!(op, "question" | "answer" | "skip") {
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
