# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-09-phase-3c-remaining-lifecycle-parity-closure-audit
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `1428f610c161eb89b57d4c9d44da6fd99be7682b`
- Working-tree digest reviewed: `0d0d5280b4b8415712cfffbab3461ec36820217da4209cca03853741353bd1d3`
- Created at: 2026-08-08T15:50:01.998Z
- Updated at: 2026-08-08T16:36:53.589Z

## Mission Interpretation

This is a closure audit, not a lifecycle implementation sprint. It asks which
remaining schema-v5 lifecycle gaps are reachable through current product
behavior and therefore block a later production cutover. Passing disposable
tests is evidence for transaction contracts only; it is not production
activation or real-user safety.

The audit must end with exactly one next bounded recommendation. The Founder
must decide whether to authorize that implementation. Silence, the accepted
ADR, prior promotion, or this recommendation is not authority.

## Problem Statement

The private schema-v5 writers now cover canonical Experience, Evidence,
Reflection, Pattern, Context Recovery, and Historical Question paths, but three
different kinds of remaining work were being discussed as if they were equal:

1. a reachable source-edit failure in `schema_v5_experience_write.rs`;
2. legacy-v4 baseline actions that a migrated current UI would still invoke;
3. future lifecycle controls that the current UI does not expose.

Only the first two classes are production-cutover blockers. Future controls,
Phase 4 relationships, malformed-state refusal, and conservative
`recovery_required` outcomes must not manufacture an endless micro-slice
sequence.

## User Value

The user must be able to edit or delete their Experience without silent
artifact rewriting, dependency rebinding, accidental history retention, or an
opaque refusal caused by artifacts that Life OS itself created. Existing user
content and reviewed hypotheses must remain understandable and user-owned,
while ADR-0009 Historical Questions must still be deleted when an exact source
becomes stale.

## Relevant Primary Definitions

- `docs/02_Philosophy.md`: context precedes insight and meaning remains
  user-owned.
- `docs/03_Principles.md`: evidence remains distinguishable from interpretation;
  agency and reversibility are design constraints.
- `docs/06_Memory.md`: memory requires provenance, correction, deletion,
  consent, and no silent identity accumulation.
- `docs/Reflection.md`: Reflection is not an automated conclusion and its cost
  must be earned.
- `docs/09_AI.md`: AI output remains a hypothesis and never becomes authority.
- `docs/10_Privacy.md`: local data belongs to the user; hidden retention and
  silent reuse are prohibited.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md` requires
  durable reviewed artifacts to retain provenance and user review distinctions.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`
  requires exact stale-source revalidation and deletion of dependent generated
  artifacts/packet snapshots.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`
  requires append-only revisions, exact dependencies, content-free deletion
  facts, no silent rebinding, ordinary dependent invalidation, and artifact-
  specific Historical Question deletion.

## Current Implementation Context

### Promoted evidence

- Slice 4A: Experience create, simple correction, parent deletion, and import.
- Slice 4B-1: canonical Evidence candidate create/correct/confirm/reject.
- Slice 4B-2: canonical Reflection suggest/answer/correct/skip.
- Slice 4B-3: canonical Pattern create/confirm/reject.
- Slice 4B-4: canonical Context Recovery suggest/first-answer/skip.
- Slice 4C-1: confirmed Evidence correction/deletion and exact dependent
  consequences.
- Slice 4C-2: Historical Question schema-v5 creation parity.
- Slice 4C-3: confirmed Pattern correction/deletion.
- Slice 4C-4: answered Reflection correction/deletion, Pattern invalidation,
  and ADR-0009 cascade. It was promoted by feature commit `07fa1f6` and merge
  commit `1428f61`; architecture/13 has been factually synchronized.

All of the above remain private, unregistered, disposable-only evidence.
Production `SCHEMA_VERSION` and startup maximum remain 4.

### Reachable production behavior

The current renderer can edit and delete Experiences, edit/confirm/reject
pending Evidence, answer/correct/skip Reflection, confirm/reject Pattern, and
answer/skip Context Recovery. Production schema v4 currently performs these
through typed Rust commands, but `saveArtifacts` remains a whole current-bundle
replacement contract. Experience edit currently invalidates/removes all v4
artifacts atomically.

### Classification vocabulary

Every matrix cell uses exactly one repository-requested classification:

- **disposable parity verified**
- **production behavior already implemented**
- **intentionally prohibited**
- **genuine implementation gap**
- **Founder product-policy decision required**
- **safely deferrable until after production cutover**
- **Phase 4 and therefore out of Phase 3C**

## Lifecycle Parity Matrix

The evidence column records qualifications; the classification remains the
single bold phrase in each row.

### Experience

| Required area | Classification | Evidence |
| --- | --- | --- |
| 1. Creation | **disposable parity verified** | Canonical create writes v5 authority and guarded v4 projection. |
| 2. First user response | **intentionally prohibited** | Not an Experience command. |
| 3. Confirm/reject/skip | **intentionally prohibited** | Experience text is user-authored source content, not an AI review candidate. |
| 4. Correction | **genuine implementation gap** | Simple correction works only when no ordinary artifacts exist; current UI can edit an Experience that has them. |
| 5. Deletion | **disposable parity verified** | Parent delete purges source content and all source-scoped child state, and applies ADR-0009 cascade. |
| 6. Prior-revision content purge | **safely deferrable until after production cutover** | Decision 9A requires a future explicit control, but preserving superseded content does not lose data and the current UI exposes no per-revision purge. |
| 7. Source mutation | **genuine implementation gap** | `ordinary_artifact_lifecycle_requires_later_slice` is reachable. |
| 8. Direct dependent consequences | **genuine implementation gap** | Evidence, Reflection, Pattern, and Context Recovery exact source edges are not handled by Experience correction. |
| 9. Transitive dependent consequences | **genuine implementation gap** | Pattern and Historical Question closure must be proven with the direct source consequences in one transaction. |
| 10. v4 projection | **genuine implementation gap** | Invalidated ordinary artifacts need projection removal while the Experience projection advances atomically. |
| 11. v5 authority | **genuine implementation gap** | Source head can advance only when no ordinary artifacts exist. |
| 12. Rollback/restart classification | **disposable parity verified** | Failure injection and exact read-only pre/post/third-state classification exist; this is not real-user restart proof. |
| 13. Production/runtime activation | **intentionally prohibited** | No production-v5 caller or authority exists. |

### Evidence

| Required area | Classification | Evidence |
| --- | --- | --- |
| 1. Creation | **disposable parity verified** | AI/local-mock candidate creation preserves exact provenance and source revision. |
| 2. First user response | **intentionally prohibited** | Not an Evidence command. |
| 3. Confirm/reject/skip | **disposable parity verified** | Exact confirmation and rejection are implemented; skip is not an Evidence review decision. |
| 4. Correction | **genuine implementation gap** | Canonical pending and confirmed correction exist, but a reachable migrated legacy pending candidate cannot be edited. |
| 5. Deletion | **disposable parity verified** | Confirmed deletion and pending rejection purge the authorized content and retain bounded content-free facts. |
| 6. Prior-revision content purge | **safely deferrable until after production cutover** | No current UI control; retained superseded content remains ineligible. |
| 7. Source mutation | **genuine implementation gap** | Experience correction cannot yet mark exact dependent Evidence invalidated/ineligible. |
| 8. Direct dependent consequences | **disposable parity verified** | Evidence correction/deletion invalidates Reflection/Pattern and deletes Historical Questions. |
| 9. Transitive dependent consequences | **disposable parity verified** | Evidence to Reflection to Pattern and Historical Question closure is verified. |
| 10. v4 projection | **disposable parity verified** | Canonical lifecycle writes keep or remove the guarded projection as required. |
| 11. v5 authority | **disposable parity verified** | Exact revision, review, lifecycle, provenance, and dependency facts are authoritative in disposable v5. |
| 12. Rollback/restart classification | **disposable parity verified** | Exact rollback and conservative ambiguous-COMMIT classification exist. |
| 13. Production/runtime activation | **intentionally prohibited** | Private unregistered boundary only. |

### Reflection

| Required area | Classification | Evidence |
| --- | --- | --- |
| 1. Creation | **disposable parity verified** | Canonical suggested prompt creation preserves prompt provenance and exact dependencies. |
| 2. First user response | **genuine implementation gap** | Canonical first answer works; a reachable migrated legacy suggested prompt cannot be answered. |
| 3. Confirm/reject/skip | **genuine implementation gap** | Canonical skip works; a migrated legacy suggested prompt cannot be skipped. Confirmation/rejection are intentionally not Reflection semantics. |
| 4. Correction | **genuine implementation gap** | Canonical answered correction works, but current UI can edit a migrated legacy answered response and the writer refuses it. |
| 5. Deletion | **safely deferrable until after production cutover** | Answered deletion is verified; suggested/skipped standalone deletion has no current UI path. Parent Experience deletion still purges it. |
| 6. Prior-revision content purge | **safely deferrable until after production cutover** | Approved future control, not a current runtime action. |
| 7. Source mutation | **genuine implementation gap** | Experience correction cannot yet invalidate the exact stale `derived_from_experience` edge. |
| 8. Direct dependent consequences | **disposable parity verified** | Answered correction/deletion invalidates Pattern and deletes Historical Questions. |
| 9. Transitive dependent consequences | **disposable parity verified** | Reflection-to-Pattern and ADR-0009 closure is verified. |
| 10. v4 projection | **disposable parity verified** | Active, invalidated-by-Evidence, and deleted states have guarded projection evidence. |
| 11. v5 authority | **disposable parity verified** | Canonical prompt/response lineage is exact in disposable v5. |
| 12. Rollback/restart classification | **disposable parity verified** | Exact rollback and pre/post/third-state classification exist. |
| 13. Production/runtime activation | **intentionally prohibited** | Private unregistered boundary only. |

### Pattern

| Required area | Classification | Evidence |
| --- | --- | --- |
| 1. Creation | **disposable parity verified** | Canonical single-Experience candidate creation preserves exact sources and provenance. |
| 2. First user response | **intentionally prohibited** | Not a Pattern command. |
| 3. Confirm/reject/skip | **genuine implementation gap** | Canonical confirm/reject works, but migrated legacy pending Pattern review is explicitly refused. Skip is not a Pattern decision. |
| 4. Correction | **disposable parity verified** | Confirmed correction appends a user revision and requires exact reconfirmation. |
| 5. Deletion | **disposable parity verified** | Explicit deletion purges all Pattern content and retains only authorized content-free facts. |
| 6. Prior-revision content purge | **safely deferrable until after production cutover** | No current UI control and all prior content remains ineligible. |
| 7. Source mutation | **genuine implementation gap** | Experience correction does not yet invalidate a Pattern through its exact source revision. |
| 8. Direct dependent consequences | **intentionally prohibited** | No Phase 3 artifact or Historical Question may currently depend on Pattern. |
| 9. Transitive dependent consequences | **Phase 4 and therefore out of Phase 3C** | Cross-experience Pattern relationships would be Phase 4, not speculative Phase 3C infrastructure. |
| 10. v4 projection | **disposable parity verified** | Active/rejected/invalidated/deleted canonical states are reconciled. |
| 11. v5 authority | **disposable parity verified** | Exact provenance and dependency-set equality are verified. |
| 12. Rollback/restart classification | **disposable parity verified** | Exact rollback and conservative commit ambiguity evidence exist. |
| 13. Production/runtime activation | **intentionally prohibited** | Private unregistered boundary only. |

### Context Recovery

| Required area | Classification | Evidence |
| --- | --- | --- |
| 1. Creation | **disposable parity verified** | Canonical local-mock/AI suggested prompt creation is exact and historically excluded. |
| 2. First user response | **genuine implementation gap** | Canonical first response works; a migrated legacy suggested turn cannot be answered. |
| 3. Confirm/reject/skip | **genuine implementation gap** | Canonical skip works; migrated legacy suggested skip is refused. Confirmation/rejection are intentionally not Context Recovery semantics. |
| 4. Correction | **safely deferrable until after production cutover** | ADR-0011 already supplies append-only/no-rebinding policy, but the current UI disables answered turns and exposes no correction action. |
| 5. Deletion | **safely deferrable until after production cutover** | User ownership requires an eventual explicit deletion path; the current UI has none and parent deletion already purges the turn. |
| 6. Prior-revision content purge | **safely deferrable until after production cutover** | No correction history or current UI control yet. |
| 7. Source mutation | **genuine implementation gap** | An answered or pending turn has an exact Experience dependency but no invalidated-state consequence. |
| 8. Direct dependent consequences | **intentionally prohibited** | Context Recovery is supporting conversation, not Evidence; zero legal inbound dependencies remain the current rule. |
| 9. Transitive dependent consequences | **Phase 4 and therefore out of Phase 3C** | Context Recovery must not become a longitudinal dependency graph. |
| 10. v4 projection | **disposable parity verified** | Canonical suggested/answered/skipped states remain synchronized. |
| 11. v5 authority | **disposable parity verified** | Prompt and response provenance remain separate and task-only. |
| 12. Rollback/restart classification | **disposable parity verified** | Exact rollback and conservative commit ambiguity evidence exist. |
| 13. Production/runtime activation | **intentionally prohibited** | Private unregistered boundary only. |

### Historical Question

| Required area | Classification | Evidence |
| --- | --- | --- |
| 1. Creation | **disposable parity verified** | Exact consumed consent, successful transmission, packet, output, v4 authority, and normalized v5 parity are verified. |
| 2. First user response | **intentionally prohibited** | Historical Question is a generated prompt artifact, not a stored response thread. |
| 3. Confirm/reject/skip | **intentionally prohibited** | ADR-0009 authorizes generation and deletion, not a reusable hypothesis review lifecycle. |
| 4. Correction | **intentionally prohibited** | Generated content is immutable; regeneration requires a new packet, consent, and transmission. |
| 5. Deletion | **disposable parity verified** | Explicit deletion and exact source mutation cascades remove content, packet snapshot, actual-use provenance, links, and dependencies. |
| 6. Prior-revision content purge | **intentionally prohibited** | No Historical Question correction/revision series is authorized. |
| 7. Source mutation | **disposable parity verified** | Evidence, Reflection, and no-ordinary-artifact Experience source cascades are verified; the remaining transaction blocker belongs to Experience ordinary consequences. |
| 8. Direct dependent consequences | **intentionally prohibited** | No artifact may depend on a Historical Question in Phase 3. |
| 9. Transitive dependent consequences | **Phase 4 and therefore out of Phase 3C** | It must not seed inferred cross-time relationships. |
| 10. v4 projection | **disposable parity verified** | Schema-v4 authority and guarded normalized v5 representation reconcile exactly. |
| 11. v5 authority | **disposable parity verified** | Normalized representation preserves the exact ADR-0009 identity chain without replacing v4 authority before cutover. |
| 12. Rollback/restart classification | **disposable parity verified** | Exact rollback, idempotent byte-equal duplicate, and conservative commit ambiguity evidence exist. |
| 13. Production/runtime activation | **intentionally prohibited** | Existing Phase 3B remains production schema v4; no v5 runtime caller exists. |

## Remaining Fail-Closed Fence Inventory

| Fence | Reachability and classification | Cutover effect |
| --- | --- | --- |
| `ordinary_artifact_lifecycle_requires_later_slice` | Legitimate current Product Harness state; **genuine implementation gap**. Later artifact slices did not supersede source mutation. | Blocks current UI Experience edit after v5 cutover. |
| `external_ordinary_dependency_requires_later_slice` | No current legal cross-source ordinary relationship; synthetic malformed or future Phase 4 state; **intentionally prohibited**. The name now over-promises a later slice. | Does not block cutover; keep fail closed and later rename only when the file is already touched. |
| Evidence `inbound_dependency_requires_later_slice` | Pending Evidence is ineligible and cannot legally have dependents; **intentionally prohibited** malformed-state refusal. The error name is stale. | Does not block cutover. |
| Context Recovery inbound/`unexpected_inbound_dependency` | Zero-legal-inbound invariant; **intentionally prohibited** malformed/future relationship. | Does not block cutover. |
| Pattern inbound dependency fences | No authorized Pattern consumer; **intentionally prohibited**. | Does not block cutover. |
| `pattern_legacy_review_requires_later_slice` | Migrated v4 candidate plus current confirm/reject UI is reachable; **genuine implementation gap**. | Blocks legacy cutover parity; schedule after Slice 4C-5. |
| Reflection canonical-only first response/skip/correction | Migrated suggested/answered rows plus current UI actions are reachable; **genuine implementation gap**. | Blocks legacy cutover parity. |
| Context Recovery canonical-only first response/skip | Migrated suggested turns plus current UI actions are reachable; **genuine implementation gap**. | Blocks legacy cutover parity. |
| Evidence canonical-only pending correction | Migrated candidate edit is reachable; **genuine implementation gap**. | Blocks legacy cutover parity. |
| `*_lifecycle_unsupported` | Correctly rejects unknown lifecycle states today, but Evidence/Reflection/Pattern/Context Recovery verifiers need exact source-invalidation support before Experience correction can commit; **genuine implementation gap** only for that named state extension. | Part of Slice 4C-5, not a generic lifecycle framework. |
| `recovery_required` | Conservative result for contradictory durable evidence or an ambiguous third state; **intentionally prohibited** autonomous recovery. | Required safety behavior, not a parity defect. Production disclosure/caller handling remains a later activation gate. |
| `database_contract.lifecycle_writes = disabled` | Honest activation marker; private tests do not grant production authority; **intentionally prohibited** until cutover authorization. | Separate activation gate, not a reason to add another lifecycle writer. |
| `database_contract.export_v2 = disabled` | Export v2 is separately sequenced; **safely deferrable until after production cutover**. | Does not block safe current-behavior cutover. |

## Experience Source-Mutation Consequence Model

### Recommended correction contract for a future Slice 4C-5

| Existing dependent state | Exact correction consequence |
| --- | --- |
| Pending or confirmed Evidence | Retain exact content/provenance/review state; mark current revision `invalidated` and `ineligible`; remove v4 projection; never rebind. |
| Suggested, answered, or skipped Reflection | Retain exact prompt/response/provenance/review state; mark invalidated/ineligible; remove v4 projection; never regenerate or rebind. |
| Candidate or confirmed Pattern | Retain exact content/provenance/review state; mark invalidated/ineligible; remove v4 projection; never recalculate, reselect, or rebind. |
| Suggested, answered, or skipped Context Recovery | Retain exact prompt/response/provenance/review state; mark invalidated/ineligible; remove v4 projection; keep categorical historical exclusion. |
| Historical Question using the Experience or any affected artifact | Require exact v4/v5 dependency parity, then apply ADR-0009 cascade deletion in the same transaction. |
| Already invalidated ordinary artifact | Preserve it without duplicate lifecycle events; prove its exact stale dependency remains. |
| Rejected/content-purged/deleted artifact | Preserve its authorized content-free facts; do not resurrect content or a head. |
| Cross-source ordinary dependency | Fail closed. No such Phase 3 relationship is currently legal. |

Retaining pending/suggested ordinary artifacts on correction is the recommended
Founder policy because the data is already persisted, correction is not an
explicit rejection/deletion action, and silent deletion would be less
reversible. All retained artifacts become visibly unusable for context. This
policy is part of the Founder decision; it is not inferred as already approved.

### Parent deletion

The promoted parent delete already purges all source revision content,
source-scoped artifact heads/revisions/content/projections/tombstones and
dependency copies, while preserving bounded source revision metadata and a
deleted source head. It applies ADR-0009 cascade first. External ordinary
dependents remain fail closed because no current Phase 3 relationship permits
them. Slice 4C-5 should add exhaustive mixed-state regression evidence rather
than redesign this behavior.

## Context Recovery Lifecycle Assessment

Answered Context Recovery correction/deletion is already directionally implied
by ADR-0011 Decisions 1B, 6B, and 12A: correction would append a mixed-authorship
revision with unchanged prompt/provenance and new user-response provenance;
deletion would purge all prompt/response content and leave only authorized
content-free facts. It is not Evidence, is never confirmed, is task-only, has
an exact Experience and `answers_prompt` lineage, and remains excluded from
historical packets.

However, the current UI disables answered Context Recovery turns and exposes
neither correction nor standalone deletion. Parent Experience deletion already
purges them. Therefore these commands are useful user-ownership work but safely
deferrable until a separate lifecycle UI is intentionally authorized. They are
not the next production-cutover blocker and must not be combined with source
mutation.

## Alternatives

### Option A — Experience source-mutation consequences next

Implement only the recommended correction consequence table and exhaustive
parent-deletion regression in the existing private disposable boundary.

- Benefit: removes the broadest reachable current-UI refusal and completes the
  accepted 6B/10B/11A/12A source-mutation transaction.
- Risk: artifact-specific verifier changes must agree exactly; a missed state
  could preserve a stale projection or delete data silently.
- Reversibility: private disposable code only; no schema or real data.

### Option B — Context Recovery correction/deletion next

- Benefit: advances future granular user control.
- Risk: does not remove any current UI cutover blocker and delays the reachable
  Experience refusal.
- Reversibility: private disposable code, but still the wrong ordering.

### Option C — Combine Experience and Context Recovery lifecycle work

- Benefit: fewer apparent sprints.
- Risk: mixes source-wide dependency closure with an independent artifact
  correction/deletion policy, increasing audit and rollback surface.
- Recommendation: reject.

### Option D — Declare remaining gaps non-blocking and proceed to cutover gate

- Benefit: fastest path to activation planning.
- Risk: current Experience edits and multiple migrated legacy actions would
  fail closed after cutover. This is not behavioral parity.
- Recommendation: reject.

### Option E — Stop for a broader lifecycle redesign

- Benefit: could unify APIs.
- Risk: ADR-0011 already supplies the required distinctions; a generic redesign
  would add speculative infrastructure and slow product delivery.
- Recommendation: reject unless implementation exposes a real contradiction.

## Proposed Slice 4C-5 Boundary And Allowlist

If the Founder selects Option A, authorize only:

1. `src-tauri/src/schema_v5_experience_write.rs`
2. `src-tauri/src/schema_v5_evidence_write.rs`
3. `src-tauri/src/schema_v5_reflection_write.rs`
4. `src-tauri/src/schema_v5_pattern_write.rs`
5. `src-tauri/src/schema_v5_context_recovery_write.rs`
6. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
7. the eleven terminal archive files under
   `.ai/workflow/HISTORY/2026-08-09-phase-3c-remaining-lifecycle-parity-closure-audit/`
   (`ARCHIVE_MANIFEST.json`, `CURRENT_MISSION.md`, `DECISION_REQUIRED.md`,
   `ENGINEERING_PLAN.md`, `ENGINEERING_REPORT.md`, `EVENTS.jsonl`,
   `PRODUCT_REVIEW.md`, `SPRINT_REPORT.md`, `THEORY_ALIGNMENT_REVIEW.md`,
   `WORKFLOW_CONTRACT.json`, and `WORKFLOW_STATE.json`).

No DDL, migration core, Historical Question writer, production SQLite/Tauri,
TypeScript, renderer, UI, provider, ContextPacket, or Harness source is in the
proposed allowlist. Any evidence requiring another file must return to the
Founder gate rather than expand scope silently.

## Updated Phase 3C Completion Roadmap

1. **Slice 4C-5 (recommended now):** Experience correction source-mutation
   consequences across all current ordinary artifact kinds, plus exhaustive
   parent-delete regression.
2. **One later legacy-baseline parity slice:** only current runtime actions that
   remain reachable after v4-to-v5 migration: pending Evidence edit, pending
   Pattern confirm/reject, suggested Reflection answer/skip, answered Reflection
   correction, and suggested Context Recovery answer/skip. Do not add new UI.
3. **Separate production cutover authorization gate:** decide real startup
   migration, disclosure, verified backup/restart handling, production command
   registration/integration, and manual disposable app-like tests. Passing
   private writers does not authorize it.
4. **Safely deferred product lifecycle controls:** standalone Context Recovery
   correction/deletion, suggested/skipped prompt deletion, explicit prior-
   revision purge, lifecycle UI, full provenance/dependency inspector, and
   export v2. Gate these by actual product value, not string-count completion.
5. **Phase 4:** cross-experience interpretation remains entirely separate.

## In Scope

- This audit, factual Slice 4C-4 promotion synchronization, the matrix, fence
  inventory, source-mutation model, Context Recovery assessment, alternatives,
  roadmap, and Founder decision package.

## Out Of Scope

- Any lifecycle mutation implementation, DDL, migration, v5 activation, real
  user data, runtime integration, UI, provider/ContextPacket/consent/retention
  changes, export v2, Phase 4, Harness changes, Git promotion, deployment, or
  release.

## Product Constraints

- Preserve Mirrors, Not Oracles; exact provenance; no silent rebinding;
  artifact-specific lifecycle rules; and ADR-0009 deletion rather than retained
  Phase 4-style Historical Question hypotheses.

## Evidence And Provenance Constraints

- Retained invalidated artifacts keep exact immutable content/provenance and
  exact stale dependency edges. No actor, review, or source fact is invented.
- A source correction event never becomes user confirmation/rejection of an AI
  artifact.

## Historical Context Constraints

- Every affected Historical Question requires exact normalized/schema-v4 parity
  before ADR-0009 cascade. No packet is reused or rehydrated.

## Consent Constraints

- No new consent occurs. Existing consent/transmission facts are deleted only
  through the already-approved successful-artifact cascade; unrelated failed
  audit metadata remains unchanged.

## AI-Role Constraints

- Invalidation is a data-lifecycle fact, not an AI conclusion. No artifact is
  regenerated, summarized, reinterpreted, or promoted to identity.

## Privacy Constraints

- Source correction must not silently delete retained user-authored responses.
- Source deletion must purge all scoped content as already approved.
- Invalidated content is never eligible for retrieval, provider transmission,
  profiling, or Harness learning.

## User-Agency Constraints

- Correction, rejection, skip, and deletion remain distinct explicit actions.
- Source edit cannot be treated as artifact rejection.
- Every consequence must be deterministic, inspectable, and reversible until
  an explicit deletion/purge action.

## Acceptance Criteria

For an authorized Option A implementation:

1. Exact-current Experience correction with any mixture of pending/confirmed
   Evidence, suggested/answered/skipped Reflection, candidate/confirmed Pattern,
   and suggested/answered/skipped Context Recovery commits in one transaction.
2. Each active ordinary artifact retains byte-exact content/provenance/review
   facts, becomes invalidated/ineligible, loses its v4 projection, and keeps its
   old exact source revision dependency.
3. Already invalidated, rejected/content-purged, and deleted states remain
   unchanged without duplicate events or resurrection.
4. All affected Historical Questions pass exact v4/v5 parity and are deleted by
   ADR-0009 cascade; unrelated failed audit metadata remains.
5. No ordinary dependency is rebound to the new Experience revision.
6. Cross-source ordinary, malformed, duplicate, cyclic, missing, contradictory,
   or unsupported evidence fails closed with no mutation.
7. Parent deletion still purges every source-scoped content/tombstone/dependency
   and retains only authorized source metadata.
8. Evidence, Reflection, Pattern, and Context Recovery verifiers accept only an
   exact invalidation caused by the stale `derived_from_experience` edge.
9. Failure injection at every new boundary proves exact logical rollback.
10. Ambiguous COMMIT accepts only exact pre-state or post-state; any third state
    returns `recovery_required` without autonomous action.
11. Production `SCHEMA_VERSION`, startup maximum, DDL, and user databases remain
    v4; no registration or runtime caller is added.
12. Focused tests, Clippy with warnings denied, canonical verification, Theory
    Alignment Review, archive/reset, and Founder diff review all complete.

## Risks

- Treating source edit as rejection could silently erase unreviewed AI output;
  the recommendation instead retains it invalidated and ineligible.
- Treating every stale edge identically could weaken artifact-specific policy;
  Historical Questions must still cascade-delete.
- A verifier that accepts any invalidated state without proving the exact stale
  source edge would weaken provenance.
- Completing Slice 4C-5 alone does not solve migrated legacy current-action
  parity and must not be described as production-v5 readiness.

## Resolved Founder Decision

- Founder decision `PHASE3C-SLICE4C5-001`: **Option A**.
- The authorized policy is that source correction retains all already-
  persisted ordinary artifact states as invalidated/ineligible rather than
  silently deleting pending/suggested/skipped content.
- All exclusions in the exact Founder response remain binding, including no
  production schema-v5 activation, legacy-v4 baseline action parity, real user
  data, runtime integration, Phase 4, Git promotion, deployment, or release.

## Human Decision Required

No. `PHASE3C-SLICE4C5-001` was resolved as Option A. New consequential scope
outside the exact authorization must return to `human_decision_required`.

## Recommendation

Implement only the Founder-approved Option A boundary. It removes the broadest
reachable current-behavior blocker without mixing independent Context Recovery
lifecycle controls or speculative Phase 4 infrastructure. After a separate
Founder-reviewed promotion, perform one bounded legacy-v4 baseline current-
action parity gate before any production cutover request.

## Review Status

approved_with_conditions
