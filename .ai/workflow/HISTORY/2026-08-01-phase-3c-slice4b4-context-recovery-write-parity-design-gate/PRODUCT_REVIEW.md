# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-01-phase-3c-slice4b4-context-recovery-write-parity-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: b8205b12a4ac33ef23d20c84d54a2115fdecb830
- Working-tree digest reviewed: cb3e9eff12468fea8f182dcd8e95cecbb3fa020cfaaa8894b9b92738bdbe2805
- Created at: 2026-08-01T20:15:00+09:00
- Updated at: 2026-08-01T20:15:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Define, but do not implement, the smallest artifact-specific schema-v5 write
boundary that faithfully represents the existing source-scoped Context
Recovery interaction. Correct only the stale Slice 4B-3 promotion facts, then
stop for an explicit Founder decision before any Rust implementation begins.

## Problem Statement

Schema v4 currently persists Context Recovery turns as one mutable JSON payload
inside the typed whole-bundle mutation transaction. Schema v5 has an accepted
`recovery_turn` artifact kind, append-only revisions, prompt/response provenance
roles, exact dependencies, review/lifecycle facts, guarded v4 projection and
content eligibility, but no private typed writer proves that the existing
create/answer/skip product behavior can satisfy those contracts.

The design must not copy Reflection blindly. Context Recovery asks for missing
current-Experience context; it is not reflective interpretation, Evidence,
Pattern, identity, historical memory or consent.

## User Value

- Preserves a user's exact clarification as user-authored supporting context.
- Preserves the optional, low-pressure meaning of an explicit skip.
- Makes prompt authorship, response authorship and exact source revision
  inspectable without treating the prompt as truth.
- Moves one existing product behavior toward schema-v5 parity without exposing
  production migration or new AI behavior.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI, Reflection before Answer,
  Evidence before Conclusion, Privacy before Profit and Mirrors not Oracles.
- `docs/02_Philosophy.md`: Context Recovery is an invitation, not an
  interrogation; the user may decline, skip or stop.
- `docs/03_Principles.md`: Context Before Insight requires clarification before
  interpretation when context is insufficient.
- `docs/06_Memory.md`: durable memory requires provenance, correction, deletion
  and user control; persistence is not provider-use permission.
- `docs/Reflection.md`: clarification is bounded and optional; skipped context
  reduces inference depth rather than creating certainty.
- `docs/09_AI.md` and `docs/10_Privacy.md`: AI never owns meaning, diagnosis or
  identity; local storage, provider transmission and consent remain distinct.

## Relevant ADRs

- ADR-0007: durable AI-assisted artifacts preserve authorship, provenance,
  review state and source relationships.
- ADR-0009: historical provider use has a separate exact-content consent and
  eligibility contract. Context Recovery is not an eligible Phase 3B source.
- ADR-0010: no Cross-Experience interpretation, recurrence or Phase 4 meaning
  may be inferred from Context Recovery.
- ADR-0011: stable identity, append-only revision metadata, separate purgeable
  content, exact dependencies, explicit review/lifecycle facts and guarded
  projections are the accepted lifecycle direction.

## Current Implementation Context

### Implemented production schema-v4 behavior

- `ContextRecoveryTurn` stores question, optional response, status
  `suggested|answered|skipped`, locale, separate prompt/response provenance and
  source Experience ID.
- Production creates a localized deterministic `local_mock` prompt. The domain
  contract also permits exact AI provenance, but no current provider method
  generates a recovery prompt.
- `addSuggestedRecoveryTurn` refuses a second open suggestion but permits a new
  explicit opportunity after prior turns are terminal.
- The UI allows editing and Save/Skip only while status is `suggested`.
  Answered/skipped textareas are disabled. There is no current response
  correction operation.
- Save trims a non-empty response, marks it `answered`, and creates distinct
  user response provenance. Explicit Skip removes response/provenance and marks
  it `skipped`. Closing, silence, navigation and timeout perform no mutation.
- Only answered, non-empty, same-source turns enter the existing current-task
  ContextPacket. Suggested/skipped turns do not improve sufficiency.
- Editing or deleting the parent Experience clears its source-scoped v4
  recovery turns through the existing bundle lifecycle.

### Promoted disposable schema-v5 evidence

- Slice 3A provides the exact disposable migration core and fixed v5 DDL.
- Slice 4A/4B-1/4B-2/4B-3 provide private Experience, Evidence, Reflection and
  Pattern mutation/reconciliation patterns.
- Complete verifier reuse and durable-array uniqueness are promoted lessons.
- No Context Recovery v5 writer exists. Production `SCHEMA_VERSION` and startup
  maximum remain 4.

### Factual synchronization

Architecture/13 version 3.0 now records Slice 4B-3 Founder review, canonical
verification, feature commit `bd1fa7482e8cc52a59518eabca5d4d8c5f541c35`
and non-fast-forward merge `b8205b12a4ac33ef23d20c84d54a2115fdecb830`.
It does not claim production readiness or closure of a Phase 3 exit blocker.

## In Scope

Decision and design only for one future private, unregistered,
path/connection-injected disposable exact-v5 Context Recovery writer with:

1. prompt creation;
2. first explicit user response;
3. explicit skip of an unanswered prompt;
4. exact current Experience dependency;
5. normalized v5 authority and guarded v4 projection in one transaction;
6. deterministic failure injection and read-only reconciliation;
7. exact pre/post manifest classification for ambiguous COMMIT outcomes.

## Out Of Scope

Implementation before Founder approval; response correction; provider calls;
ContextPacket changes; Tauri/UI/startup/app-data registration; production v5;
real-user migration; historical eligibility/transmission; consent changes;
Evidence/Reflection/Pattern mutation; dependent invalidation; Phase 3B v5;
Phase 4; export, retention, backup/restore activation; Harness expansion; Git
promotion, PR, deployment and release.

## Product Constraints

- The question requests missing context only; it cannot assert an emotion,
  relationship, motive, diagnosis, identity or truth about the user.
- A response is user-authored supporting context, not confirmed Evidence.
- Skip is an explicit user action and remains ineligible context.
- Silence, close, cancel, timeout and navigation never imply skip.
- A new prompt cannot duplicate an open suggestion.
- No response correction is included because the current product exposes no
  such action and safe dependent invalidation is separately withheld.

## Local-First Data Architecture

### Typed commands

Recommended future private enum:

- `CreateSuggested { expected_source_revision_id, prompt }`
- `SaveFirstResponse { source_id, artifact_id,
  expected_source_revision_id, expected_artifact_revision_id, response }`
- `SkipSuggested { source_id, artifact_id,
  expected_source_revision_id, expected_artifact_revision_id }`

No generic SQL, generic artifact command, response correction, deletion or
automatic transition is proposed.

### Canonical content and provenance

- V5 content uses canonical JSON v1 with stable Context Recovery fields:
  artifact/source identity, exact question, optional response, locale and
  original created time. Provenance remains normalized outside content.
- Prompt provenance is immutable `ai` or `local_mock`. `ai` requires exact
  provider/model/Harness/prompt/generated-time fields; `local_mock` requires
  provider `mock`, null model and the exact deterministic contract versions.
- Prompt provenance `sourceEntryId` must equal the source and
  `sourceArtifactIds` must be an empty, well-formed unique array. Context
  Recovery does not derive from Evidence, Reflection, Pattern or history.
- First answer creates new user response provenance with origin `user`, exact
  source Experience ID, exact recovery-turn ID and action time. It never
  relabels the prompt as user-authored.

### Revisions, lifecycle and eligibility

- Suggested: revision 1, authorship `ai|local_mock`, reason `created`, active,
  review `pending`, ineligible reason `awaiting_optional_response`.
- Answered: revision 2, predecessor revision 1, authorship `mixed`, reason
  `answered`, active, review `not_applicable`, eligibility `eligible` with the
  narrower reason `answered_current_experience_context_only`. It keeps exact
  prompt bytes/provenance and adds the user response/provenance.
- Answered revision has `derived_from_experience` to the exact current source
  revision and `answers_prompt` to the immutable initial prompt revision.
- Skip adds one exact user `skipped` review event against revision 1, leaves
  prompt content/provenance unchanged, writes no response revision/provenance,
  and updates the head to `skipped`/ineligible reason `explicitly_skipped`.
- Suggested and skipped are never ContextPacket-eligible. Answered is eligible
  only for the already-existing current-Experience task packet while its exact
  source revision is current. Artifact kind `recovery_turn` remains
  categorically excluded from Phase 3B historical packets even though the v5
  head uses the generic `eligible` state.
- The guarded v4 projection reconstructs the existing `ContextRecoveryTurn`
  payload exactly; it is never authority.

### Stale, conflicting and dependent states

- Reuse the complete promoted exact Experience verifier; do not replace it
  with partial source-head checks.
- Refuse wrong schema/receipt/contract, stale source or artifact revision,
  missing/deleted/invalidated source, wrong artifact kind, cross-source data,
  malformed IDs/timestamps/locale/content/provenance, blank response,
  duplicate IDs, second open suggestion, duplicate/conflicting terminal action,
  and any unexpected inbound dependency.
- Prove every durable ID array is well formed and unique before set comparison.
- Do not bind, rebind, invalidate or cascade dependents in this slice.

### Atomicity and reconciliation

- One `BEGIN IMMEDIATE` transaction writes v5 authority, dependencies,
  provenance, lifecycle/review state, head, compatibility guard and v4
  projection.
- Inject deterministic failure after every meaningful provenance, revision,
  content, dependency, review/lifecycle, head, projection and guard boundary.
- Every pre-commit failure must preserve the exact logical pre-state.
- Generic COMMIT error is outcome-unknown unless read-only durable evidence
  proves one exact pre-state or post-state. No retry, replay, rollback after
  ambiguity, repair, cleanup or inferred user intent.
- Reconciliation verifies full Experience authority, recovery content,
  provenance roles, exact dependencies, head/review/lifecycle/eligibility,
  v4 projection, guard emptiness, immutable migration receipt,
  foreign-key check and integrity check.

### Deletion and retention

No artifact delete or retention operation is proposed. Existing parent-source
cascade remains the governing eventual boundary. A later slice must govern any
standalone recovery-turn deletion, content purge, correction or dependent
invalidation before those actions exist in production v5.

## Evidence And Provenance Constraints

The AI/local-mock prompt, user response and explicit skip are three different
facts. Saving an answer is not prompt confirmation and does not create Evidence.
Prompt bytes and provenance remain immutable after answer. The exact current
Experience revision is the only external content dependency.

## Historical Context Constraints

Context Recovery remains excluded from Phase 3B eligible historical packet
types. `eligible` in this writer means current-Experience generation only; it
cannot be interpreted as historical selection, consent or transmission.

## Consent Constraints

Local persistence and current-task use do not grant historical provider-use
consent. No consent event, consent reuse, transmission or provider retention
policy changes.

## AI-Role Constraints

The prompt may invite context but cannot diagnose, profile, finalize identity,
infer sensitive traits or make a conclusion. The writer validates structure and
provenance, not the semantic truth of arbitrary model text.

## Privacy Constraints

Only synthetic/disposable exact-v5 fixtures may be touched. No real user data,
app-data path or provider request. The answer remains source-scoped and is not
added to historical memory, export or audit retention by this slice.

## User-Agency Constraints

- Answer and skip require exact explicit actions.
- Silence or absence of action preserves `suggested`.
- Skipping does not count as context or negative feedback.
- The user-authored answer remains distinguishable and does not confirm the
  prompt.

## Acceptance Criteria

If Option A is later authorized, focused disposable evidence must prove:

1. exact-v5 fixtures come only from the promoted migration core;
2. AI and local-mock prompt provenance are structurally exact;
3. only one open suggestion exists per source while later explicit
   opportunities remain possible after terminal turns;
4. exact current Experience revision dependency and complete Experience
   verifier reuse;
5. first non-empty answer appends one mixed answered revision with immutable
   prompt and distinct user provenance;
6. answer links exactly to the initial prompt revision;
7. explicit skip creates one review event, no response content/provenance and
   no new revision;
8. silence/cancel/close have no command and therefore no durable mutation;
9. answered is current-task eligible; suggested/skipped are ineligible;
10. all recovery turns remain historically ineligible under ADR-0009;
11. v5 authority and guarded v4 projection reconcile exactly;
12. stale, deleted, malformed, cross-source, duplicate, conflicting,
    unsupported and inbound-dependent states fail closed;
13. well-formed unique durable arrays are checked before set equality;
14. every injected pre-commit failure preserves the exact logical pre-state;
15. ambiguous COMMIT is classified read-only as exact pre/post/unknown with no
    retry or repair;
16. immutable receipt, guards, foreign keys and integrity reconcile;
17. production schema/startup remain 4 and no runtime registration exists;
18. canonical verification and Clippy with warnings denied pass.

## Risks

- Generic `eligibility_state = eligible` could be misread as historical
  eligibility; the artifact-kind allowlist and narrow reason must remain hard.
- Supporting AI provenance could be mistaken for authorizing an AI call; it
  does not. Current product creation remains local-mock.
- Copying Reflection code could hide artifact-specific semantics or create a
  premature generic framework.
- Omitting correction defers a user-control capability, but adding it now would
  exceed current behavior and require dependent invalidation policy.
- Disposable tests can be overclaimed as production or real-user readiness.

## Open Questions

none within the selected Option A boundary; later production, correction,
deletion, dependent invalidation and historical-parity decisions remain
separately gated.

## Human Decision Required

false — `PHASE3C-SLICE4B4-001` was resolved exactly as Option A. The recorded
scope authorizes only Engineering Planning and the bounded private disposable
implementation.

## Recommendation

Option A: create, first answer and explicit skip only. It exactly matches the
current product surface, closes the smallest artifact-specific v5 parity gap,
and avoids premature response-correction/dependent-invalidation semantics.

## Review Status

approved_with_conditions
