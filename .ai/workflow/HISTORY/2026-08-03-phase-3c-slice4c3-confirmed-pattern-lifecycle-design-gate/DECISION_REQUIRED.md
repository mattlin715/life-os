# Decision Required

Status: resolved
- Sprint ID: 2026-08-03-phase-3c-slice4c3-confirmed-pattern-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-02T16:15:00.000Z
- Updated at: 2026-08-02T16:15:00.000Z

## Decision ID

`PHASE3C-SLICE4C3-001`

## Sprint ID

`2026-08-03-phase-3c-slice4c3-confirmed-pattern-lifecycle-design-gate`

## Decision Summary

Choose whether and how to implement confirmed single-Experience Pattern
correction/deletion in the private disposable schema-v5 evidence boundary.

Repository evidence establishes that a Pattern has exact outgoing dependencies
on one Experience revision, one-or-more confirmed Evidence revisions, and
zero-or-more answered Reflection revisions. No currently authorized artifact
may depend on a Pattern: the DDL has no `uses_pattern` relationship, Context
Recovery does not consume Pattern, and ADR-0009 explicitly excludes Pattern.

## Why Automation Stopped

Implementation authority is consequential and withheld. ADR-0011 accepts the
lifecycle policy but does not authorize this slice, production schema v5, or a
choice among exact-set preservation, speculative dependent handling, partial
implementation, and deferral. Silence is not approval.

## Relevant Constitution Clauses

- The AI remains a mirror rather than an oracle.
- The user retains authority over meaning, correction, and deletion.
- Privacy, provenance, reversibility, and refusal boundaries are not optional.
- A Pattern may not become Evidence, diagnosis, sensitive inference, or final
  Identity through confirmation or correction.

## Relevant Primary Definitions

- `docs/05_Identity.md` and `docs/07_Awareness.md`: Pattern is a revisable
  hypothesis, not identity truth.
- `docs/06_Memory.md`: durable memory must support provenance, correction, and
  deletion.
- `docs/Reflection.md`: meaning is co-created but user-owned.
- `docs/09_AI.md`, `docs/10_Privacy.md`, `docs/appendix/Harness.md`: exact
  inputs, authorship, bounded use, and fail-closed behavior remain inspectable.

## Relevant ADRs

- ADR-0007: persist reviewed artifacts with provenance.
- ADR-0009: Pattern is ineligible for historical context and cannot be a legal
  Historical Question source.
- ADR-0011: correction is append-only and requires reconfirmation; old
  authorship is immutable; dependents never rebind; deletion purges content and
  retains only approved content-free facts.

## Available Options

### Option A — Exact-source preservation plus zero legal inbound (recommended)

Authorize only a private, unregistered, path/connection-injected disposable
Pattern lifecycle extension in `src-tauri/src/schema_v5_pattern_write.rs`:

- correct exact-current confirmed Pattern by appending a user-authored
  successor revision;
- require the caller to supply the complete Experience/Evidence/Reflection
  source set and prove exact equality with the predecessor before copying those
  exact revision edges to the successor;
- preserve predecessor AI/local-mock authorship/provenance and superseded
  visibility; make the successor pending/ineligible and require reconfirmation;
- delete an exact-current Pattern by purging all retained Pattern revision
  content, clearing the head, removing the v4 projection, and retaining only
  authorized immutable metadata plus one content-free tombstone;
- require zero normalized and historical inbound edges; any inbound edge fails
  closed without invalidation, cascade, or rebinding;
- add deterministic rollback and conservative ambiguous-COMMIT tests; keep
  production schema/startup at v4.

### Option B — Generic ordinary dependent invalidation

Implement correction/deletion plus a generic invalidation engine for hypothetical
future artifacts that might depend on Pattern.

### Option C — Correction only

Implement exact correction and reconfirmation behavior but defer explicit
Pattern deletion and full purge/tombstone behavior.

### Option D — Defer complete slice

Make no lifecycle implementation until production lifecycle UI is designed.

## Benefits

- **A:** closes the exact known lifecycle gap without inventing ontology;
  preserves user correction/deletion, authorship, provenance, and fail-closed
  behavior.
- **B:** could reduce later implementation work if a legal Pattern consumer is
  eventually authorized.
- **C:** delivers user-owned revision semantics sooner with less code.
- **D:** avoids private evidence that could later diverge from UI needs.

## Risks

- **A:** later legal Pattern consumers would require a new Founder-gated
  consequence design; disposable evidence does not prove production safety.
- **B:** creates speculative relationship semantics, invalidation authority,
  and possible Phase 4 drift without a current product consumer.
- **C:** leaves the accepted deletion/privacy gap open and cannot be treated as
  complete Pattern lifecycle parity.
- **D:** prolongs the Phase 3C post-review correction/deletion gap and delays
  production-v5 readiness evidence.

## Reversibility

All options remain production-neutral at this gate. Option A implementation
would be private, unregistered, disposable-only, and removable before runtime
activation. No real database, migration, DDL, user_version, or product UI is
changed. Production activation would require a separate Founder decision.

## Data And Privacy Impact

Option A touches only synthetic/disposable fixtures. Its deletion contract is
privacy-positive: all retained Pattern text is purged and the tombstone contains
no reusable text or raw content digest. No provider call, transmission,
consent, app-data access, real user data, or historical eligibility is added.

## Orchestrator Recommendation

Select **Option A**. Complete dependency closure is known: exact outgoing
Experience/Evidence/Reflection sources and zero legal inbound consumers.
Malformed future-looking edges must fail closed rather than authorize a generic
framework.

## Default Safe Action

Remain at `human_decision_required`. Do not create an Engineering Plan or edit
implementation files until an exact Founder resolution is recorded.

## Blocked Files Or Phases

- `src-tauri/src/schema_v5_pattern_write.rs`
- all implementation planning and implementation phases
- production schema/user_version 5, migration, real-data/runtime activation,
  Phase 4, Git promotion, PR, deployment, and release

Only the factual architecture/13 correction and workflow decision artifacts are
currently permitted changes.

## Exact Founder Response Needed

To authorize the recommended bounded implementation, respond exactly or with
equivalent explicit scope:

> I resolve PHASE3C-SLICE4C3-001 by selecting Option A. I authorize Phase 3C
> Slice 4C-3 only: exact-v5 disposable fixtures produced through the promoted
> migration core; extension of the existing private, unregistered,
> path/connection-injected Rust Pattern boundary for exact-current confirmed
> Pattern correction and explicit Pattern deletion; correction appends one
> immutable user-authored successor revision, preserves the predecessor's exact
> AI/local-mock authorship and provenance, records corrected and superseded
> lifecycle facts, returns the successor to pending/ineligible, and requires
> later exact-revision reconfirmation; the caller must explicitly supply the
> complete Experience, confirmed Evidence, and answered Reflection source
> dependency set and it must exactly equal the predecessor set before those
> exact revision edges are preserved, with no add, remove, inference, source
> reselection, or rebinding; deletion remains distinct from rejection, records
> explicit user deletion, purges all retained Pattern revision content, clears
> the head, removes the guarded v4 projection, and retains only authorized
> immutable content-free metadata and one minimal tombstone containing no
> reusable text or raw content digest; every normalized or historical inbound
> Pattern dependency fails closed under a zero-legal-inbound invariant, including
> any malformed ADR-0009 Historical Question edge; exact v4/v5 parity,
> deterministic failure injection, full logical rollback, conservative
> ambiguous-COMMIT pre/post/third-state classification, read-only
> reconciliation, focused synthetic/disposable tests, factual Book One
> documentation, Clippy, canonical verification, Theory Alignment Review,
> archive/reset, and stop at Founder diff review. I accept that this does not
> prove production restart recovery, real-user safety, or production schema-v5
> readiness. I do not authorize production SCHEMA_VERSION 5, production
> user_version 5, migration or fresh-v5 initialization, real user databases or
> app-data, startup/Tauri/renderer/UI activation, Pattern generation or source
> reselection, Context Recovery or Historical Question dependence on Pattern,
> generic future-dependent invalidation, provider or ContextPacket changes,
> consent changes, export v2, retention, production backup/restore/recovery,
> Phase 4, identity or sensitive inference, Harness expansion, staging, commit,
> push, merge, PR, deployment, or release.

For Options B, C, or D, name the option and explicitly state the authorized and
withheld scope. A partial or silent response does not authorize implementation.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4C3-001 by selecting Option A. I authorize Phase 3C Slice 4C-3 only: exact-v5 disposable fixtures produced through the promoted migration core; extension of the existing private, unregistered, path/connection-injected Rust Pattern boundary for exact-current confirmed Pattern correction and explicit Pattern deletion; correction appends one immutable user-authored successor revision, preserves the predecessor's exact AI/local-mock authorship and provenance, records corrected and superseded lifecycle facts, returns the successor to pending/ineligible, and requires later exact-revision reconfirmation; the caller must explicitly supply the complete Experience, confirmed Evidence, and answered Reflection source dependency set and it must exactly equal the predecessor set before those exact revision edges are preserved, with no add, remove, inference, source reselection, or rebinding; deletion remains distinct from rejection, records explicit user deletion, purges all retained Pattern revision content, clears the head, removes the guarded v4 projection, and retains only authorized immutable content-free metadata and one minimal tombstone containing no reusable text or raw content digest; every normalized or historical inbound Pattern dependency fails closed under a zero-legal-inbound invariant, including any malformed ADR-0009 Historical Question edge; exact v4/v5 parity, deterministic failure injection, full logical rollback, conservative ambiguous-COMMIT pre/post/third-state classification, read-only reconciliation, focused synthetic/disposable tests, factual Book One documentation, Clippy, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I accept that this does not prove production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, Pattern generation or source reselection, Context Recovery or Historical Question dependence on Pattern, generic future-dependent invalidation, provider or ContextPacket changes, consent changes, export v2, retention, production backup/restore/recovery, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Slice 4C-3 Option A exactly as recorded: private unregistered disposable-only Pattern correction/deletion in the existing Rust boundary, exact source-set preservation, zero inbound invariant, rollback/ambiguous-COMMIT evidence, factual docs, verification, theory review, archive/reset, and stop at Founder diff review; all production, runtime, real-data, Phase 4, Harness, Git promotion, deployment, and release authority remains withheld.

## Decided At And Evidence Reference

- Decided at: 2026-08-02T17:03:54.461Z
- Evidence reference: PHASE3C-SLICE4C3-001

## Resume Phase

product_review
