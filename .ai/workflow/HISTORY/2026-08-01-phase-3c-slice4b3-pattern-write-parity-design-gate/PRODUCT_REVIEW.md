# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-01-phase-3c-slice4b3-pattern-write-parity-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: c2f518a409c4308682302f505da275b621f16708
- Working-tree digest reviewed: 154c7df11570746d907af04a0ad44656be201c0ba8528ebc0add5c84a190b70c
- Created at: 2026-07-31T16:30:27.107Z
- Updated at: 2026-07-31T17:16:00.000Z

## Mission Interpretation

Close the factual post-promotion drift for Slice 4B-2 and decide whether the
next bounded Phase 3C step may implement disposable exact-v5 write parity for
the existing single-Experience Pattern candidate lifecycle. This is a storage
and lifecycle evidence slice, not a new Pattern theory, provider behavior,
production migration, runtime activation, or Phase 4 capability.

## Problem Statement

The production schema-v4 product already generates, locally persists, confirms,
and rejects source-scoped Pattern candidates. The promoted exact-v5 disposable
evidence now covers Experience, Evidence, and Reflection writes, but no typed
exact-v5 Pattern create/confirm/reject boundary exists. Architecture/13
explicitly withholds that implementation authority.

Implementing it without a separate decision could silently change what Pattern
confirmation means, omit exact dependency revisions, treat a one-source clue as
recurrence, lose immutable AI provenance, or bypass rejected-content purge.
Those are product-governance decisions rather than routine coding details.

## User Value

The bounded evidence would reduce cutover risk for a future append-only local
store by proving that a tentative Pattern remains visibly an AI/local-mock
hypothesis, binds to the exact source material actually used, and preserves an
explicit user review decision without becoming Evidence, Identity, diagnosis,
or objective truth. It produces no current user-visible behavior because the
proposed module remains private, unregistered, and disposable-only.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: We Build Mirrors, Not Oracles; Human before AI;
  Reflection before Answer; Evidence before Conclusion; Privacy before Profit.
- `docs/03_Principles.md`: an inference must expose evidence and uncertainty;
  the user owns final interpretation.
- `docs/05_Identity.md`: Pattern is routed here by `docs/00_Index.md`; identity
  emerges through long evidence and is never finalized by one hypothesis.
- `docs/06_Memory.md`: reviewed continuity remains revisable, provenance-aware,
  deletion-aware, and free of silent identity accumulation.
- `docs/Reflection.md`: AI may invite reflection but cannot automate or own
  meaning.
- `docs/07_Awareness.md`: one Experience may provide only a clue; repeated
  evidence may strengthen a hypothesis but never prove identity; no-pattern and
  insufficient-context outcomes remain valid.
- `docs/09_AI.md`: AI may observe and propose a tentative Pattern while remaining
  uncertain, evidence-bound, and subordinate to user judgment.
- `docs/10_Privacy.md`: local data remains user-owned; more context grants more
  responsibility, not broader inference authority.
- `docs/appendix/Harness.md`: confirmation means useful for continued
  reflection, not objective truth; inference and user-confirmed Evidence remain
  distinct.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: a
  reviewed Pattern may persist locally only with source, authorship, provider,
  model, version, review, revision, and lifecycle provenance. Confirmation does
  not make it fact.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`:
  Pattern is excluded from the Phase 3B Historical Reflection Question packet;
  this slice cannot change that task, packet, consent, or deletion chain.
- `docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md`:
  Phase 4 remains blocked by Phase 3 exit dependencies. A confirmed
  single-Experience Pattern may later appear only as a prior hypothesis, never
  as Evidence or silent support.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`:
  exact revisions, explicit review events, exact dependencies, rejected-content
  purge, content-free tombstones, and no silent rebinding are accepted design
  policy. Exact implementation still needs a separate Founder gate.

## Current Implementation Context

Implemented and promoted:

- production remains SQLite schema v4 and already supports the current
  single-Experience Pattern candidate UI and source-scoped persistence;
- exact-v5 disposable migration/restart evidence exists privately;
- Slice 4A Experience, Slice 4B-1 Evidence, and Slice 4B-2 Reflection private
  write boundaries are implemented, verified, Founder-reviewed, and promoted;
- Slice 4B-2 was promoted through feature commit
  `353a6c1d910c45d9978add7699c1037b8f94a7a1` and non-fast-forward merge
  `c2f518a409c4308682302f505da275b621f16708`;
- `verify_exact_reflection_v5` currently reuses the complete promoted Evidence
  verifier, which in turn reuses the complete Experience verifier.

Founder-approved design but not production implementation:

- normalized schema-v5 direction, append-only lifecycle, exact dependencies,
  guarded schema-v4 projection, and rejected-content purge in ADR-0011 and
  architecture/12-13.

Proposed only in this sprint:

- a private exact-v5 disposable Pattern create/confirm/reject write boundary.

Not authorized or implemented:

- production schema/user_version 5, migration, fresh-v5 startup, runtime
  registration, real-user writes, confirmed Pattern correction/deletion,
  Context Recovery parity, Historical Question/Phase 3B v5 parity, or Phase 4.

## In Scope

If the Founder selects Option A, only the following implementation may proceed:

1. Exact-v5 disposable fixtures produced through the promoted migration core.
2. One private, unregistered, path/connection-injected Rust Pattern module.
3. Create one AI/local-mock single-Experience candidate.
4. Confirm one exact current pending candidate revision.
5. Reject one exact current pending candidate revision with synchronous content
   purge and a content-free tombstone.
6. Exact Experience, confirmed Evidence, and actually used answered Reflection
   revision dependencies.
7. Immutable Pattern content provenance and one guarded v4 projection in the
   same transaction as v5 authority, review/lifecycle facts, and dependencies.
8. Deterministic synthetic/disposable tests, factual Book One synchronization,
   Clippy, canonical verification, Theory Alignment Review, archive/reset, and
   a stop at Founder diff review.

## Out Of Scope

- Production schema/user_version 5, migration, fresh-v5 initialization, real
  user data, app-data, Tauri, renderer, UI, startup, provider, ContextPacket, or
  consent changes.
- Pattern content generation changes or a new semantic-output evaluator.
- Confirmed Pattern correction/deletion; confirmed Evidence correction/deletion;
  background invalidation/cascade; silent dependency rebinding.
- Context Recovery write parity or a new `uses_recovery_turn` schema edge.
- Historical Question/Phase 3B v5 writes, Cross-Experience Reflection,
  recurrence, contradiction, change-over-time, summary, Identity, sensitive
  inference, diagnosis, advice, or Phase 4.
- Export v2, retention jobs, production backup/restore, automatic recovery,
  generic mutation framework, Harness expansion, Git promotion, PR, deployment,
  or release.

## Product Constraints

- A Pattern is a tentative source-scoped hypothesis before and after
  confirmation.
- The phrase `single-Experience Pattern` describes provenance scope, not proof
  that a pattern recurs. One Experience may supply only a clue.
- Confirmation records only that the user finds the exact hypothesis useful for
  continued reflection. It does not change authorship or epistemic authority.
- Rejection applies only to an explicit action on one exact current pending
  revision. Silence, close, timeout, or navigation is not rejection.
- Rejected content is synchronously purged; content-free digest, review,
  lifecycle, revision, and tombstone facts remain under ADR-0011.
- No candidate, confirmed Pattern, or review event becomes Evidence, Identity,
  Awareness, Growth, or Phase 4 support automatically.

## Evidence And Provenance Constraints

- Creation requires one exact current active Experience revision and one or
  more exact current confirmed, eligible, same-source Evidence revisions.
- Zero or more exact current answered, eligible, same-source Reflection
  revisions may be included only when actually used.
- Each Reflection dependency must itself retain a non-empty user response,
  exact prompt lineage, response provenance, and current Evidence dependencies.
- Pattern provenance origin is exactly `ai` or `local_mock`; provider, model,
  Harness version, prompt version, generated time, source Experience ID, and
  source artifact IDs remain immutable.
- The provenance source-artifact ID set must exactly equal the declared
  Evidence plus Reflection artifact set. Duplicates, omissions, extras, or a
  Context Recovery ID fail closed. Context Recovery cannot be silently dropped
  because the fixed DDL has no authorized `uses_recovery_turn` edge.
- The v5 Pattern content is canonical JSON containing stable ID, source ID,
  sorted exact Evidence IDs, sorted exact Reflection IDs, bounded hypothesis
  text, and creation time. Review status and provenance remain separate facts.
- The guarded v4 projection preserves the existing PatternNote shape, including
  candidate/confirmed status and immutable provenance; a rejected Pattern has
  no v4 projection.

## Historical Context Constraints

No historical retrieval, packet assembly, provider transmission, or whole
history is introduced. A Pattern created here is source-scoped to one current
Experience and cannot consume a Historical Question, historical packet, prior
Pattern, or Phase 4 hypothesis.

## Consent Constraints

This private disposable writer creates no provider call and changes no consent
policy. Existing Pattern generation consent/transport behavior is unchanged.
ADR-0009 historical consent cannot authorize this slice or be reused by it.

## AI-Role Constraints

- AI/local mock authors the candidate; user review never relabels it as user
  content.
- Candidate text must be represented and tested as a tentative hypothesis. The
  storage slice does not claim semantic enforcement of arbitrary model text and
  therefore does not establish production output safety.
- No claim of recurrence, cause, diagnosis, moral character, immutable
  personality, identity finalization, or Cross-Experience meaning is authorized.
- `insufficient context` and no candidate remain valid upstream outcomes; the
  write boundary never forces depth.

## Privacy Constraints

- All execution is limited to synthetic/disposable exact-v5 fixtures.
- No real journal content, app-data path, provider payload, raw error body,
  credential, or user database may be used.
- Rejected content must not remain in content rows, v4 projections, logs, test
  snapshots, or retained workflow artifacts.

## User-Agency Constraints

- Only explicit confirm or reject changes review state.
- Confirmation targets one exact current revision and is revisable only through
  a later separately authorized correction lifecycle.
- No automatic confirmation, rejection, retry, replay, correction, cascade,
  rebinding, candidate selection, or repair is permitted.

## Local-First Data Architecture Analysis

### Typed command contract

`CreateCandidate` should require the expected source revision, candidate ID,
source ID, text, creation time, exact Evidence revision references, exact
Reflection revision references, and immutable generated provenance.

`ConfirmPending` and `RejectPending` should require source ID, artifact ID,
expected source revision, expected Pattern revision, and the exact expected
Evidence/Reflection revision sets. A mismatch must write nothing.

### Atomic state mapping

- Create: append revision `created`; authorship `ai` or `local_mock`; review
  `pending`; lifecycle `active`; eligibility `ineligible`; append exact
  dependencies and `created`; write guarded v4 `candidate` projection.
- Confirm: append one explicit-user `confirmed` review event for the unchanged
  exact current revision; keep content/provenance unchanged; set review
  `confirmed` and eligibility `eligible`; update guarded v4 status to
  `confirmed`.
- Reject: append explicit-user `rejected`, remove the v4 projection, purge the
  exact content row, append `content_purged`, clear the current revision pointer,
  set rejected/content-purged/ineligible, and add a content-free
  `rejected_content_purged` tombstone with the prior digest in one transaction.

Every operation uses one `BEGIN IMMEDIATE`, the existing compatibility guard,
deterministic injected inputs, current-content checks, foreign-key and integrity
checks, read-only post-transaction reconciliation, and the existing
conservative COMMIT outcome contract. Generic SQL COMMIT error is
outcome-unknown unless exact durable manifests prove the full pre-state or
post-state; otherwise return `recovery_required` without retry or repair.

### Complete verifier reuse

The complete verifier chain is sufficient:

1. expose `verify_exact_reflection_v5` only as `pub(super)` inside the private
   migration module;
2. make the Pattern verifier call it before Pattern-specific reconciliation;
3. because Reflection already calls the complete Evidence verifier and Evidence
   calls the complete Experience verifier, the chain validates full content,
   projection, provenance, lifecycle, dependency, and schema invariants;
4. separately validate only the exact Pattern-selected Evidence and Reflection
   revision sets and Pattern projection.

No generic mutation framework or public verifier API is justified. The only
proposed extraction is the smallest private visibility change needed for full
verifier reuse. Local exact-reference structs may remain module-specific; their
small shape is not a proven shared mutation abstraction.

## Acceptance Criteria

If Option A is authorized, implementation evidence must prove:

1. exact-v5 disposable-only module; no production registration or caller;
2. AI and local-mock candidate creation preserves exact content provenance;
3. at least one confirmed current same-source Evidence dependency is required;
4. zero/one/multiple answered Reflection dependencies are exact and current;
5. full Reflection -> Evidence -> Experience verifier chain is reused;
6. Context Recovery/extra/duplicate provenance source IDs fail closed;
7. candidate remains ineligible; exact confirmation makes only that hypothesis
   eligible without rewriting content or provenance;
8. rejection synchronously purges content/projection and leaves content-free
   review/lifecycle/tombstone history;
9. stale, deleted, rejected, cross-source, orphaned, malformed, duplicate,
   conflicting, unsupported, and inbound-dependent states write nothing;
10. every meaningful create/confirm/reject boundary has deterministic injected
    rollback coverage;
11. ambiguous COMMIT is classified only from read-only exact manifests with no
    autonomous action;
12. v5 authority and v4 projection agree after every committed operation;
13. foreign-key, integrity, guard emptiness, deterministic ID/fingerprint, and
    operation-manifest checks pass;
14. Pattern never becomes Evidence, Identity, recurrence proof, Phase 4, or a
    historical packet input;
15. focused tests, Clippy `-D warnings`, and canonical `verify.ps1` pass;
16. documentation states implemented/verified/private separately from promoted
    and production-authorized.

## Risks

- A single Experience can tempt the UI or future reader to interpret a clue as
  recurrence. Mitigation: exact source scope, tentative semantics, and no Phase
  4 or identity authority.
- The current provider provenance may include Context Recovery IDs. The fixed
  v5 DDL has no authorized exact dependency type for them. Mitigation: this
  slice fails closed rather than dropping them or inventing a loose edge.
- Confirmation can be misread as truth. Mitigation: immutable AI authorship and
  explicit review semantics.
- A narrow head/status check could accept corrupt Evidence or Reflection
  content. Mitigation: reuse the complete promoted verifier chain.
- Rejection purge can leave partial state if transaction ordering is wrong.
  Mitigation: one transaction plus injected failures at each boundary.
- Private fixture evidence may be overclaimed as production readiness.
  Mitigation: no registration, no real data, no production schema activation,
  and explicit residual-risk wording.

## Open Questions

- None within Slice 4B-3. `PHASE3C-SLICE4B3-001` was resolved by the Founder
  with Option A and the exact bounded implementation contract recorded in
  `DECISION_REQUIRED.md`.

## Human Decision Required

No. Decision `PHASE3C-SLICE4B3-001` is resolved. All production activation,
later-slice, Phase 4, Git-promotion, deployment, and release authority remains
withheld.

## Recommendation

Recommend Option A: implement only bounded disposable create/confirm/reject
parity with exact Experience/Evidence/Reflection dependencies, strict rejection
purge, complete verifier reuse, and fail-closed refusal when Context Recovery or
other unsupported sources appear. It is the smallest coherent next step and
does not require a new schema object, provider change, public API, or generic
mutation framework.

## Review Status

approved_with_conditions
