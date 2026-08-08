# Decision Required

Status: resolved
- Sprint ID: 2026-08-09-phase-3c-remaining-lifecycle-parity-closure-audit
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-08T15:50:01.998Z
- Updated at: 2026-08-08T15:50:01.998Z

## Decision ID

`PHASE3C-SLICE4C5-001`

## Sprint ID

`2026-08-09-phase-3c-remaining-lifecycle-parity-closure-audit`

## Decision Summary

Choose the next single bounded action after the remaining lifecycle parity
audit. The recommendation is a private disposable Slice 4C-5 that completes
Experience correction consequences for every currently persisted ordinary
artifact kind while preserving the promoted parent-deletion and ADR-0009
cascade rules.

## Why Automation Stopped

The repository proves that current UI Experience edit is reachable while
`schema_v5_experience_write.rs` refuses the same operation whenever ordinary
artifacts exist. Deciding what source correction does to pending, confirmed,
answered, skipped, candidate, and supporting-conversation artifacts changes
durable lifecycle policy. It therefore requires explicit Founder authority.

## Relevant Constitution Clauses

- `docs/00_Constitution.md`: We Build Mirrors, Not Oracles; Human Before AI;
  Evidence Before Conclusion; Privacy Before Profit; documentation and human
  authority govern lower-level implementation.

## Relevant Primary Definitions

- `docs/02_Philosophy.md`
- `docs/03_Principles.md`
- `docs/06_Memory.md`
- `docs/Reflection.md`
- `docs/09_AI.md`
- `docs/10_Privacy.md`

These require user-owned meaning, exact evidence/provenance distinctions,
revisability, deletion, no silent identity accumulation, and no AI authority.

## Relevant ADRs

- ADR-0007: reviewed AI artifacts retain provenance and user review state.
- ADR-0009: exact stale historical sources cascade-delete dependent generated
  content and successful packet snapshots.
- ADR-0011 Decisions 6B, 10B, 11A, and 12A: append-only correction, no
  rebinding, ordinary dependent invalidation, Historical Question cascade, and
  content-free deletion facts.

## Available Options

### Option A — Authorize Slice 4C-5 Experience source-mutation consequences

Authorize only the existing private, unregistered, path/connection-injected
disposable schema-v5 Experience boundary and exact artifact verifiers to:

- correct an exact-current Experience in one `BEGIN IMMEDIATE` transaction;
- retain byte-exact content, provenance, review state, and old source dependency
  for every active same-source Evidence, Reflection, Pattern, and Context
  Recovery artifact;
- mark those artifacts invalidated/ineligible and remove guarded v4 projections;
- preserve already invalidated, rejected/content-purged, and deleted states
  without duplicate events or resurrection;
- require exact v4/v5 Historical Question dependency parity and apply ADR-0009
  cascade deletion;
- keep Context Recovery historically excluded;
- fail closed on cross-source ordinary, malformed, incomplete, duplicate,
  cyclic, contradictory, or unsupported state;
- preserve and regression-test the existing complete parent-deletion behavior;
- add deterministic failure, exact rollback, and conservative ambiguous-COMMIT
  pre/post/third-state evidence;
- update only the exact Product Review allowlist, factual architecture/13, and
  workflow archive; run Clippy and canonical verification; complete Theory
  Alignment Review; archive/reset; stop at Founder diff review.

This option does not include legacy-v4 baseline action parity. That remains one
later bounded cutover blocker.

### Option B — Context Recovery correction/deletion next

Design/implement answered Context Recovery correction and standalone deletion
before source mutation.

### Option C — Combine Experience and Context Recovery lifecycle work

Implement both independent lifecycle boundaries in one slice.

### Option D — Treat remaining lifecycle gaps as non-blocking

Proceed to a production cutover authorization gate without either slice.

### Option E — Stop for a broader lifecycle redesign

Do not add another writer until a new general lifecycle architecture is
approved.

## Benefits

- **A:** removes the broadest reachable current-UI blocker and implements the
  accepted artifact-specific source-mutation contract without data deletion.
- **B:** advances future user control for supporting conversation.
- **C:** reduces the number of named slices.
- **D:** reaches cutover planning sooner.
- **E:** offers a chance to simplify APIs if a real architectural contradiction
  exists.

## Risks

- **A:** five exact verifiers must accept only valid source-caused invalidation;
  missing one artifact state could cause stale projection or hidden retention.
- **B:** leaves current Experience edit blocked and works on commands absent from
  the current UI.
- **C:** couples independent risks and makes rollback evidence harder to review.
- **D:** migrated users would encounter fail-closed current actions; not parity.
- **E:** risks speculative generic infrastructure despite an accepted ADR and
  working artifact-specific model.

## Reversibility

- **A/B/C:** private disposable code only if exact scope is honored; no schema,
  runtime, or real data. A is the smallest reversible useful step.
- **D:** reversible as a planning choice, but unsafe to use as production
  authority.
- **E:** reversible before implementation but delays product progress.

## Data And Privacy Impact

Option A touches only synthetic/disposable fixtures. Its proposed durable rule
is non-destructive on Experience correction: retained artifacts become
ineligible without content/provenance rewrite. Explicit parent deletion remains
destructive only within the disposable fixture and purges all scoped content.
No provider call, consent, packet use, real database, or app-data path is
authorized.

## Orchestrator Recommendation

Select **Option A**. Then gate one legacy-v4 baseline current-action parity
slice before considering production cutover. Do not combine Context Recovery
standalone lifecycle work.

## Default Safe Action

Remain at `human_decision_required`; do not implement, stage, commit, push,
merge, activate schema v5, or access real data.

## Blocked Files Or Phases

Implementation, validation, Theory Alignment Review, workflow completion, and
all proposed Rust changes remain blocked. Production schema/runtime integration,
legacy parity, Context Recovery correction/deletion, Phase 4, and Git promotion
remain separately blocked even if Option A is selected.

## Exact Founder Response Needed

To authorize the recommendation, reply exactly or equivalently:

```text
I resolve PHASE3C-SLICE4C5-001 by selecting Option A. I authorize Phase 3C Slice 4C-5 only: exact-v5 disposable fixtures produced through the promoted migration core; extension of the existing private, unregistered, path/connection-injected Rust Experience boundary for exact-current Experience correction with complete same-source ordinary artifact consequences and exhaustive parent-deletion regression; retention of byte-exact content, provenance, review state, and old exact source-revision dependencies for pending or confirmed Evidence, suggested or answered or skipped Reflection, candidate or confirmed Pattern, and suggested or answered or skipped Context Recovery, while marking each active dependent invalidated and ineligible and removing its guarded schema-v4 projection without rejection, deletion, regeneration, recalculation, source reselection, confirmation, or dependency rebinding; preservation without duplicate events or resurrection of already invalidated, rejected, content-purged, or deleted states; exact normalized/schema-v4 Historical Question parity followed by ADR-0009 cascade deletion; continued categorical exclusion of Context Recovery from historical eligibility; complete exact verifier support for source-caused invalidation only; cross-source ordinary, malformed, stale, incomplete, duplicate, cyclic, contradictory, unsupported, or unexpected inbound state failing closed; one BEGIN IMMEDIATE transaction, deterministic failure injection, exact logical rollback, conservative ambiguous-COMMIT exact pre-state/post-state/third-state classification, and read-only reconciliation; focused synthetic/disposable tests; factual architecture/13 synchronization; Clippy; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that Slice 4C-5 does not complete legacy-v4 baseline current-action parity and does not prove production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, legacy-baseline action parity, standalone Context Recovery correction/deletion, suggested/skipped prompt deletion, per-revision purge UI, export v2, provider or ContextPacket changes, consent or retention changes, production backup/restore/recovery, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.
```

The Founder may instead select B, C, D, or E and state the exact authorized
scope. Silence is not a decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4C5-001 by selecting Option A. I authorize Phase 3C Slice 4C-5 only: exact-v5 disposable fixtures produced through the promoted migration core; extension of the existing private, unregistered, path/connection-injected Rust Experience boundary for exact-current Experience correction with complete same-source ordinary artifact consequences and exhaustive parent-deletion regression; retention of byte-exact content, provenance, review state, and old exact source-revision dependencies for pending or confirmed Evidence, suggested or answered or skipped Reflection, candidate or confirmed Pattern, and suggested or answered or skipped Context Recovery, while marking each active dependent invalidated and ineligible and removing its guarded schema-v4 projection without rejection, deletion, regeneration, recalculation, source reselection, confirmation, or dependency rebinding; preservation without duplicate events or resurrection of already invalidated, rejected, content-purged, or deleted states; exact normalized/schema-v4 Historical Question parity followed by ADR-0009 cascade deletion; continued categorical exclusion of Context Recovery from historical eligibility; complete exact verifier support for source-caused invalidation only; cross-source ordinary, malformed, stale, incomplete, duplicate, cyclic, contradictory, unsupported, or unexpected inbound state failing closed; one BEGIN IMMEDIATE transaction, deterministic failure injection, exact logical rollback, conservative ambiguous-COMMIT exact pre-state/post-state/third-state classification, and read-only reconciliation; focused synthetic/disposable tests; factual architecture/13 synchronization; Clippy; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that Slice 4C-5 does not complete legacy-v4 baseline current-action parity and does not prove production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, legacy-baseline action parity, standalone Context Recovery correction/deletion, suggested/skipped prompt deletion, per-revision purge UI, export v2, provider or ContextPacket changes, consent or retention changes, production backup/restore/recovery, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Phase 3C Slice 4C-5 only: private unregistered disposable exact-v5 Experience source-correction consequences, exact artifact verifier support, ADR-0009 cascade, parent-deletion regression, deterministic failure and ambiguous-COMMIT evidence, factual architecture/13 synchronization, canonical verification, Theory Alignment Review, archive/reset, and Founder diff stop; all exclusions in the exact Founder response remain binding.

## Decided At And Evidence Reference

- Decided at: 2026-08-08T16:36:53.589Z
- Evidence reference: founder-message-2026-08-09-PHASE3C-SLICE4C5-001

## Resume Phase

product_review
