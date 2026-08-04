# Decision Required

Status: resolved
- Sprint ID: 2026-08-04-phase-3c-slice4c4-answered-reflection-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-04T11:52:36.308Z
- Updated at: 2026-08-04T11:52:36.308Z

## Decision ID

`PHASE3C-SLICE4C4-001`

## Sprint ID

`2026-08-04-phase-3c-slice4c4-answered-reflection-lifecycle-design-gate`

## Decision Summary

Choose whether and how to extend the existing private disposable Reflection
writer so exact-current answered-response correction and explicit answered
Reflection deletion apply all already-governed Pattern and Historical Question
consequences atomically.

Repository evidence establishes a bounded legal closure: a Reflection has
exact outgoing Experience/Evidence/prompt dependencies; a Pattern may depend on
its exact answered revision through `uses_reflection_response`; an ADR-0009
Historical Question may select that exact revision through matched v4/v5
historical dependencies. Context Recovery and every other artifact kind may not
depend on Reflection, and Pattern has no legal inbound consumer.

## Why Automation Stopped

ADR-0011 fixes correction, deletion, invalidation, and tombstone policy, while
ADR-0009 fixes the Historical Question cascade. Neither acceptance nor prior
disposable evidence authorizes this implementation slice. Choosing complete
correction/deletion, a partial lifecycle, or deferral is consequential Founder
authority. Silence is not approval.

## Relevant Constitution Clauses

- **We Build Mirrors, Not Oracles.**
- User-authored meaning and correction remain under user control.
- Evidence, provenance, privacy, reversibility, and deletion are not optional.
- A saved or corrected Reflection response is not AI truth, Evidence
  confirmation, diagnosis, or Identity.

## Relevant Primary Definitions

- `docs/03_Principles.md`: Evidence before Conclusion, Reflection before
  Answer, visible uncertainty, and user control.
- `docs/06_Memory.md`: durable records must remain provenance-preserving,
  revisable, and deletable.
- `docs/Reflection.md`: the user remains the subject of Reflection and owns the
  meaning.
- `docs/09_AI.md`, `docs/10_Privacy.md`, and `docs/appendix/Harness.md`: AI
  authorship, exact sources, local control, historical consent, and deletion
  boundaries remain explicit.

## Relevant ADRs

- ADR-0007: preserve reviewed artifacts and exact AI/user provenance.
- ADR-0009: source correction/deletion invalidates the exact historical use and
  cascade-deletes dependent Historical Question content and successful packet
  snapshot; no consent reuse.
- ADR-0011: append-only correction, no dependency rebinding, ordinary Pattern
  invalidation, all-content artifact deletion, and content-free tombstones.

## Available Options

### Option A — Complete answered lifecycle and exact consequences (recommended)

Authorize only the existing private, unregistered, path/connection-injected
Reflection module against migrated exact-v5 disposable fixtures to:

- extend exact-current answered response correction so it appends a mixed
  successor, preserves immutable prompt bytes/provenance and exact outgoing
  source set, creates new user response provenance, keeps eligibility only
  while exact dependencies remain current, and adds no confirmation step;
- explicitly delete only an exact-current active eligible answered Reflection,
  purge every retained prompt/response content row, clear the head, remove the
  v4 projection, and retain only authorized content-free facts plus one minimal
  tombstone;
- invalidate every legal exact Pattern dependent without changing its content
  or provenance, remove its v4 projection, and never regenerate or rebind it;
- cascade-delete every exact ADR-0009 Historical Question use across its
  matched v4/v5 representation while preserving unrelated unsuccessful audit
  metadata;
- fail closed on unsupported or incomplete closure, then prove rollback and
  conservative ambiguous-COMMIT classification.

Prospective product-file allowlist:

1. `src-tauri/src/schema_v5_reflection_write.rs`
2. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

Standard current workflow artifacts and the standard sprint archive may also
change. Any additional product path requires a new Founder decision.

### Option B — Dependent-aware correction only

Authorize correction plus Pattern invalidation and ADR-0009 cascade, but keep
explicit answered Reflection deletion unsupported.

### Option C — Explicit deletion only

Authorize deletion plus Pattern invalidation and ADR-0009 cascade, but preserve
the current fail-closed correction behavior whenever an inbound dependent
exists.

### Option D — Defer to production lifecycle UI design

Make no new lifecycle implementation and leave both gaps open until user-facing
production controls are designed.

## Benefits

- **A:** closes the complete known answered-Reflection lifecycle gap in one
  coherent transaction and directly exercises accepted ADR-0009/0011 policy.
- **B:** smaller implementation while enabling correction of already-used
  responses.
- **C:** prioritizes the strongest privacy action with less correction logic.
- **D:** avoids disposable evidence that might later diverge from UI needs.

## Risks

- **A:** has the widest transaction surface and requires exact all-revision
  dependency audit; disposable evidence still does not prove production safety.
- **B:** leaves the Founder-approved deletion/privacy gap unresolved and would
  require another near-identical consequence implementation later.
- **C:** users still cannot correct an answered response with dependents, and
  correction/deletion paths may drift.
- **D:** prolongs Phase 3C exit gaps and postpones evidence needed before any
  production-v5 decision.

## Reversibility

All options remain production-neutral. Option A would be private, unregistered,
path/connection-injected, disposable-only, and removable without changing the
production v4 database or runtime. Definite pre-commit failures roll back the
logical fixture state. Ambiguous outcomes are classified from read-only
evidence, never reversed or repaired automatically.

## Data And Privacy Impact

Option A uses synthetic/disposable fixtures only. Correction intentionally
retains superseded content under ADR-0011 Decision 9A. Explicit deletion purges
all retained Reflection question/response content and leaves only authorized
content-free facts. Historical generated content and packet snapshots follow
ADR-0009 cascade. No real user data, network, provider call, consent, telemetry,
clipboard, or runtime path is involved.

## Orchestrator Recommendation

Select **Option A**. Repository authority already defines every legal
consequence, so a partial slice would create two lifecycle paths without
reducing the eventual governance surface. Keep the implementation narrow to the
existing Reflection module and exact known dependents; do not create generic
future-dependent infrastructure.

## Default Safe Action

Remain at `human_decision_required`. Do not create an Engineering Plan or edit
implementation files until the Founder records an exact option and scope.

## Blocked Files Or Phases

- `src-tauri/src/schema_v5_reflection_write.rs`
- Engineering Planning and implementation
- Any additional product file beyond the two-path allowlist
- Production schema/user_version 5, real data/runtime activation, Phase 4,
  Git promotion, PR, deployment, and release

Only the factual architecture/13 correction and current workflow decision
artifacts are changed at this gate.

## Exact Founder Response Needed

To authorize the recommended bounded implementation, respond exactly or with
equivalent explicit scope:

> I resolve PHASE3C-SLICE4C4-001 by selecting Option A. I authorize Phase 3C
> Slice 4C-4 only: exact-v5 disposable fixtures produced through the promoted
> migration core; extension of the existing private, unregistered,
> path/connection-injected Rust Reflection boundary for dependent-aware
> correction of an exact-current active eligible answered user response and
> explicit deletion of an exact-current active eligible answered Reflection;
> complete promoted Experience, confirmed-Evidence, immutable initial-prompt,
> prompt-provenance, response-provenance, Reflection projection, Pattern, and
> Historical Question verifier reuse; durable identifier uniqueness before
> exact-set equality; correction appends one immutable mixed-authorship
> successor with unchanged prompt bytes and prompt provenance, new exact user
> response provenance, the same exact Experience/Evidence/answers-prompt
> dependency set, corrected and superseded lifecycle facts, no additional
> confirmation requirement, and eligibility only while every exact source
> remains current; explicit deletion remains distinct from skip and
> invalidation, records explicit user deletion, purges all retained Reflection
> prompt and response content, clears the head, removes the guarded v4
> projection, and retains only authorized immutable content-free metadata and
> one minimal tombstone with no reusable text; complete direct
> Reflection-to-Pattern consequence closure retains exact Pattern content and
> provenance, marks affected current Patterns invalidated/ineligible, removes
> their guarded v4 projections, and performs no regeneration, source
> reselection, cascade, or dependency rebinding; complete exact
> Reflection-to-Historical-Question v4/v5 parity followed by ADR-0009 cascade
> deletion of generated content, successful packet snapshot, actual-use
> provenance, lifecycle link, and exact dependencies while unrelated
> unsuccessful audit metadata remains unchanged; unsupported Context Recovery,
> other future inbound relationships, malformed, stale, duplicate,
> contradictory, cross-source, cyclic, or incomplete evidence fails closed;
> one BEGIN IMMEDIATE transaction, deterministic failure injection, exact
> logical rollback, conservative ambiguous-COMMIT exact pre-state/post-state/
> third-state classification, read-only reconciliation, focused
> synthetic/disposable tests, factual Book One documentation, Clippy, canonical
> verification, Theory Alignment Review, archive/reset, and stop at Founder
> diff review. I accept that this does not prove production restart recovery,
> real-user safety, or production schema-v5 readiness. I do not authorize
> production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5
> initialization, real user databases or app-data, startup/Tauri/renderer/UI
> activation, suggested/skipped Reflection deletion, prompt generation,
> Evidence or historical eligibility changes, provider or ContextPacket
> changes, consent or retention changes, Pattern regeneration or rebinding,
> generic future-dependent infrastructure, production backup/restore/recovery,
> export v2, Phase 4, identity or sensitive inference, Harness expansion,
> staging, commit, push, merge, PR, deployment, or release.

For Options B, C, or D, name the option and explicitly state the authorized and
withheld scope. A partial response or silence does not authorize implementation.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4C4-001 by selecting Option A. I authorize Phase 3C Slice 4C-4 only: exact-v5 disposable fixtures produced through the promoted migration core; extension of the existing private, unregistered, path/connection-injected Rust Reflection boundary for dependent-aware correction of an exact-current active eligible answered user response and explicit deletion of an exact-current active eligible answered Reflection; complete promoted Experience, confirmed-Evidence, immutable initial-prompt, prompt-provenance, response-provenance, Reflection projection, Pattern, and Historical Question verifier reuse; durable identifier uniqueness before exact-set equality; correction appends one immutable mixed-authorship successor with unchanged prompt bytes and prompt provenance, new exact user response provenance, the same exact Experience/Evidence/answers-prompt dependency set, corrected and superseded lifecycle facts, no additional confirmation requirement, and eligibility only while every exact source remains current; explicit deletion remains distinct from skip and invalidation, records explicit user deletion, purges all retained Reflection prompt and response content, clears the head, removes the guarded v4 projection, and retains only authorized immutable content-free metadata and one minimal tombstone with no reusable text; complete direct Reflection-to-Pattern consequence closure retains exact Pattern content and provenance, marks affected current Patterns invalidated/ineligible, removes their guarded v4 projections, and performs no regeneration, source reselection, cascade, or dependency rebinding; complete exact Reflection-to-Historical-Question v4/v5 parity followed by ADR-0009 cascade deletion of generated content, successful packet snapshot, actual-use provenance, lifecycle link, and exact dependencies while unrelated unsuccessful audit metadata remains unchanged; unsupported Context Recovery, other future inbound relationships, malformed, stale, duplicate, contradictory, cross-source, cyclic, or incomplete evidence fails closed; one BEGIN IMMEDIATE transaction, deterministic failure injection, exact logical rollback, conservative ambiguous-COMMIT exact pre-state/post-state/third-state classification, read-only reconciliation, focused synthetic/disposable tests, factual Book One documentation, Clippy, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I accept that this does not prove production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, suggested/skipped Reflection deletion, prompt generation, Evidence or historical eligibility changes, provider or ContextPacket changes, consent or retention changes, Pattern regeneration or rebinding, generic future-dependent infrastructure, production backup/restore/recovery, export v2, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Slice 4C-4 Option A exactly as recorded: private unregistered disposable-only answered Reflection correction/deletion with exact Pattern invalidation, ADR-0009 Historical Question cascade, deterministic rollback/reconciliation evidence, factual docs, verification, theory review, archive/reset, and stop at Founder diff review; every production, runtime, real-data, Phase 4, Harness, Git promotion, deployment, and release authority remains withheld.

## Decided At And Evidence Reference

- Decided at: 2026-08-04T14:29:58.374Z
- Evidence reference: PHASE3C-SLICE4C4-001

## Resume Phase

product_review
