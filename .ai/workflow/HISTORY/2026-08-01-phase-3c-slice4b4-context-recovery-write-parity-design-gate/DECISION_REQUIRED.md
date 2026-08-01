# Decision Required

Status: resolved
- Sprint ID: 2026-08-01-phase-3c-slice4b4-context-recovery-write-parity-design-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-01T20:16:00+09:00
- Updated at: 2026-08-01T20:16:00+09:00

## Decision ID

PHASE3C-SLICE4B4-001

## Sprint ID

2026-08-01-phase-3c-slice4b4-context-recovery-write-parity-design-gate

## Decision Summary

Select whether and how Life OS may implement private disposable schema-v5
write parity for the existing Context Recovery create, first-answer and
explicit-skip flow.

## Why Automation Stopped

Architecture/13 has fixed schema-v5 lifecycle authority, but it does not grant
this artifact-specific implementation. Context Recovery affects user-authored
supporting context, prompt/response provenance, eligibility and the meaning of
skip. The Founder must explicitly choose the operations before implementation.

## Observed Repository Evidence

- Production creates a localized deterministic `local_mock` question, although
  the domain provenance type also represents exact AI prompts.
- Only a `suggested` turn can be edited, answered or skipped in the UI.
- There is no response-correction UI or command after answer.
- Save requires a non-empty response and creates separate user provenance.
- Skip is explicit, removes response/provenance, and does not deepen context.
- A second open suggestion is refused; a later explicit opportunity can follow
  a terminal prior turn.
- Only answered, non-empty, same-source turns enter the existing current-task
  ContextPacket. Context Recovery is not eligible for Phase 3B historical use.
- Schema v5 already supports `recovery_turn`, prompt/response provenance,
  append-only revisions, exact dependencies, review/lifecycle facts and a
  guarded v4 projection, but no typed recovery writer exists.
- Production schema and startup support remain v4.

## Relevant Constitution Clauses

Human before AI; Context/Reflection before Answer; Evidence before Conclusion;
Privacy before Profit; AI must strengthen rather than replace user agency.
No Constitution edit is proposed.

## Relevant Primary Definitions

- `docs/02_Philosophy.md`: optional invitation, not interrogation.
- `docs/03_Principles.md`: Context Before Insight.
- `docs/Reflection.md`: skip reduces inference depth; it is not truth or
  rejection feedback.
- `docs/06_Memory.md`, `docs/09_AI.md`, `docs/10_Privacy.md`: provenance,
  source control, local-first privacy and separation of persistence from
  provider authority.

## Relevant ADRs

- ADR-0007: reviewed-artifact provenance and source relationships.
- ADR-0009: separate historical eligibility, consent and transmission.
- ADR-0010: Phase 4 remains outside this slice.
- ADR-0011: append-only lifecycle, exact dependencies, purgeable content and
  compatibility projections that are not authority.

## Available Options

### Option A — Create, first answer and explicit skip only

Authorize one private, unregistered, path/connection-injected disposable Rust
boundary with three typed commands:

1. create an exact AI/local-mock suggested prompt;
2. save the first explicit non-empty user response;
3. explicitly skip an unanswered suggested prompt.

It requires one exact current Experience revision; immutable prompt provenance;
separate user response provenance; suggested/answered/skipped artifact-specific
states; an `answers_prompt` link for the answered revision; current-task-only
eligibility; categorical Phase 3B historical exclusion; one guarded v4
projection transaction; failure injection; conservative ambiguous-COMMIT
classification; and full read-only reconciliation. No correction or delete.

### Option B — Add append-only response correction

Option A plus correction of an answered response as another mixed append-only
revision, retaining immutable prompt provenance and superseding the prior
response. Any inbound dependent would fail closed because invalidation and
rebinding remain outside the slice.

### Option C — Contracts and fixtures only

Record command/data/evaluation contracts without implementing a Rust writer.

### Option D — Defer Context Recovery and design Phase 3B v5 parity first

Do no Context Recovery implementation; move to the separate historical
question/consent/transmission persistence problem.

## Benefits

- **A:** exact parity with current product behavior; smallest safe production-v5
  readiness step; clear authorship and skip semantics; no dependent correction
  problem.
- **B:** earlier user correction capability and stronger long-term lifecycle
  model.
- **C:** lowest implementation risk and retains a complete decision record.
- **D:** prioritizes the more complex historical governance chain.

## Risks

- **A:** response correction remains deferred; generic v5 eligibility must not
  be mistaken for historical eligibility.
- **B:** exceeds actual current UI behavior and introduces correction,
  supersession and inbound-dependent policy before those product controls are
  designed.
- **C:** does not produce executable parity evidence.
- **D:** leaves an existing source-scoped artifact behind while attempting the
  more consequential Phase 3B v5 chain.

All implementation options risk being overclaimed as production-ready even
though they remain private, disposable and unregistered.

## Reversibility

- A/B are new private modules exercised only in disposable fixtures and can be
  disabled by non-registration; no user database or schema version changes.
- C/D are documentation-only and fully reversible through a later decision.
- None authorize destructive down migration, cleanup, retry, repair or real
  user-data mutation.

## Data And Privacy Impact

Option A/B tests only synthetic/disposable exact-v5 fixtures. The proposed
answer remains user-authored local current-task context. It is not Evidence,
historical memory, consent or provider authorization. No app-data, real user
database, new provider call, retention job or export is permitted.

## Orchestrator Recommendation

**Option A.** It matches actual current behavior, preserves the distinct
purpose of Context Recovery, reuses the complete Experience verifier without a
generic framework, and avoids prematurely designing correction/invalidation.

## Default Safe Action

Remain at `human_decision_required`; do not implement any writer.

## Blocked Files Or Phases

Blocked pending resolution:

- any new `src-tauri/src/schema_v5_context_recovery_write.rs`;
- any modification that nests/registers such a module;
- Engineering Plan and implementation phases;
- all production schema-v5, runtime, provider, historical, later-slice and Git
  promotion actions listed in the Founder prompt.

## Exact Founder Response Needed

To authorize the recommended bounded option, reply exactly:

```text
I resolve PHASE3C-SLICE4B4-001 by selecting Option A. I authorize Phase 3C Slice 4B-4 only: exact-v5 disposable fixtures produced through the promoted migration core; one private, unregistered, path/connection-injected Rust Context Recovery mutation boundary for exact AI/local-mock suggested-prompt creation, first explicit non-empty user response, and explicit skip of an unanswered suggested prompt; one exact current active Experience revision dependency with complete promoted Experience verifier reuse; immutable prompt content and AI/local-mock provenance; separate exact user response provenance without relabeling or confirming the prompt; at most one open suggested turn per source while preserving later explicit opportunities after terminal turns; suggested/pending/ineligible, answered/not-applicable/current-Experience-task-only eligible, and skipped/explicit-user-action/ineligible semantics; an exact answers_prompt dependency from the answered revision to the immutable initial prompt revision; categorical exclusion from Phase 3B historical eligibility, consent, transmission, and durable longitudinal memory; synchronized v5 authority and guarded v4 projection in one transaction; stale, deleted, invalidated, malformed, blank, duplicate, conflicting, cross-source, unsupported, and unexpected inbound-dependent states failing closed without rebinding or cascade; well-formed unique durable ID validation before set equality; deterministic injected fixture clocks, identifiers, guard tokens, failure points, commit outcomes, and manifests; read-only post-transaction reconciliation; focused synthetic/disposable tests; factual Book One documentation; Clippy; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that current production creates local-mock prompts only, that accepting exact AI provenance in this private writer does not authorize any AI call, and that this slice does not implement response correction, standalone deletion, dependent invalidation, semantic validation of arbitrary prompt text, production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, provider or ContextPacket behavior changes, new AI calls, historical transmission, consent-policy changes, Evidence/Reflection/Pattern correction or deletion, ordinary dependent invalidation/cascade, Context Recovery response correction/deletion, Phase 3B v5 persistence, export v2, retention, backup/restore activation, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.
```

The Founder may instead select B, C or D, but must state the exact authorized
scope. Silence does not resolve the decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4B4-001 by selecting Option A. I authorize Phase 3C Slice 4B-4 only: exact-v5 disposable fixtures produced through the promoted migration core; one private, unregistered, path/connection-injected Rust Context Recovery mutation boundary for exact AI/local-mock suggested-prompt creation, first explicit non-empty user response, and explicit skip of an unanswered suggested prompt; one exact current active Experience revision dependency with complete promoted Experience verifier reuse; immutable prompt content and AI/local-mock provenance; separate exact user response provenance without relabeling or confirming the prompt; at most one open suggested turn per source while preserving later explicit opportunities after terminal turns; suggested/pending/ineligible, answered/not-applicable/current-Experience-task-only eligible, and skipped/explicit-user-action/ineligible semantics; an exact answers_prompt dependency from the answered revision to the immutable initial prompt revision; categorical exclusion from Phase 3B historical eligibility, consent, transmission, and durable longitudinal memory; synchronized v5 authority and guarded v4 projection in one transaction; stale, deleted, invalidated, malformed, blank, duplicate, conflicting, cross-source, unsupported, and unexpected inbound-dependent states failing closed without rebinding or cascade; well-formed unique durable ID validation before set equality; deterministic injected fixture clocks, identifiers, guard tokens, failure points, commit outcomes, and manifests; read-only post-transaction reconciliation; focused synthetic/disposable tests; factual Book One documentation; Clippy; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that current production creates local-mock prompts only, that accepting exact AI provenance in this private writer does not authorize any AI call, and that this slice does not implement response correction, standalone deletion, dependent invalidation, semantic validation of arbitrary prompt text, production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, provider or ContextPacket behavior changes, new AI calls, historical transmission, consent-policy changes, Evidence/Reflection/Pattern correction or deletion, ordinary dependent invalidation/cascade, Context Recovery response correction/deletion, Phase 3B v5 persistence, export v2, retention, backup/restore activation, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Slice 4B-4 Option A only: private unregistered disposable exact-v5 Context Recovery create, first-answer, and explicit-skip parity with complete Experience verifier reuse, exact provenance/dependencies, current-task-only eligibility, historical exclusion, guarded v4 projection, deterministic tests, factual docs, verification, Theory Review, archive/reset, and Founder diff stop; all production, correction/deletion, Phase 3B/4, Harness, Git promotion, deployment, and release authority remains withheld.

## Decided At And Evidence Reference

- Decided at: 2026-08-01T15:15:10.870Z
- Evidence reference: PHASE3C-SLICE4B4-001

## Resume Phase

product_review
