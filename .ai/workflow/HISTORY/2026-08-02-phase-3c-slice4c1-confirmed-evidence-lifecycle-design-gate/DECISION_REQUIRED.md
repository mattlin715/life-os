# Decision Required

Status: resolved
- Sprint ID: 2026-08-02-phase-3c-slice4c1-confirmed-evidence-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-01T17:46:10.475Z
- Updated at: 2026-08-01T17:46:10.475Z

## Decision ID

PHASE3C-SLICE4C1-001

## Sprint ID

2026-08-02-phase-3c-slice4c1-confirmed-evidence-lifecycle-design-gate

## Decision Summary

Choose whether and how to implement the next private disposable schema-v5
confirmed-Evidence lifecycle slice. The consequential boundary is whether one
transaction may correct or delete confirmed Evidence while completely applying
ordinary Reflection/Pattern invalidation, ADR-0009 Historical Question cascade
deletion, and guarded schema-v4 projection changes.

## Why Automation Stopped

ADR-0011 fixes the lifecycle policy, but it does not itself authorize this
implementation. The proposed transaction introduces destructive content purge,
transitive dependency invalidation, and coordinated schema-v4/v5 Historical
Question deletion. Only the Founder may authorize the exact slice and cascade
boundary. Slice 4B-4 promotion and passing tests do not imply this authority.

## Relevant Constitution Clauses

- Preserve **We Build Mirrors, Not Oracles**.
- The AI must not silently convert hypotheses into user-owned facts.
- User control, provenance, revision, rejection, and deletion remain visible.
- This decision does not propose a Constitution change.

## Relevant Primary Definitions

- `docs/03_Principles.md`: Evidence before Conclusion; Reflection before Answer;
  user agency and visible uncertainty.
- `docs/06_Memory.md`: provenance-preserving, revisable, deletable memory with
  no silent identity accumulation.
- `docs/Reflection.md`: no authoritative or automatically rewritten meaning.
- `docs/09_AI.md` and `docs/10_Privacy.md`: bounded AI role, local-first
  deletion, and governed historical use.

## Relevant ADRs

- ADR-0007: reviewed artifacts preserve provenance and authorship.
- ADR-0009: source invalidation cascade-deletes dependent Historical Question
  generated content and successful actual-use snapshot; consent is not reused.
- ADR-0010: Phase 4 cross-time interpretation remains outside scope.
- ADR-0011 Decisions 6B, 7B, 9A, 10B, 11A, and 12A: append-only correction,
  pending reconfirmation, retained superseded content, ordinary invalidation,
  Historical Question cascade deletion, and content-free deletion tombstone.

## Available Options

### Option A — Confirmed Evidence correction only

Implement correction only and refuse every inbound-dependent state. Do not
implement deletion or dependent consequences.

### Option B — Complete bounded lifecycle slice

Implement correction and deletion with complete ordinary direct/transitive
invalidation, ADR-0009 Historical Question schema-v4/v5 cascade deletion, and
guarded schema-v4 projection changes in one private, unregistered,
disposable-only transaction. Accept only exact migrated-v5 fixtures with
one-to-one Historical Question representation; fail closed otherwise. Keep
Phase 3B v5 runtime creation parity as a blocker for production activation.

### Option C — Phase 3B v5 parity prerequisite first

Do not implement confirmed-Evidence lifecycle yet. First design and implement a
separately authorized private Phase 3B schema-v5 runtime write-parity slice,
then return to this lifecycle.

### Option D — Contracts and evaluation only

Retain this design and add only separately authorized contracts/fixtures/tests;
do not add a lifecycle writer.

### Option E — Defer to production cutover planning

Do not implement lifecycle or Phase 3B parity now; move to a later production
cutover planning gate.

## Benefits

- **A:** smallest code surface; proves pending reconfirmation without cascade.
- **B:** smallest semantically complete user-control lifecycle; verifies all
  already-accepted dependency consequences together.
- **C:** closes the Phase 3B normalized-write prerequisite before lifecycle and
  better resembles eventual production state.
- **D:** lowest mutation risk; improves specifications without executable writes.
- **E:** no immediate implementation cost.

## Risks

- **A:** leaves deletion and real dependent lifecycle gaps unresolved; can be
  mistaken for general confirmed-Evidence support.
- **B:** larger transaction and test matrix; incomplete graph handling could
  violate both ordinary-retention and Historical Question-deletion policy.
- **C:** delays the correction/deletion user-control gap and expands scope into
  Phase 3B persistence before it is required for disposable proof.
- **D:** infrastructure evidence does not progress executable lifecycle safety.
- **E:** cutover planning is premature while lifecycle and Phase 3B write gaps
  remain known blockers.

## Reversibility

Options A-D can remain private, unregistered, and fixture-only, with no
production schema or user-data effect. Option B's definite pre-commit failures
must roll back atomically; ambiguous COMMIT must stop for read-only
classification rather than retry. Option E changes no code but risks planning
around unresolved requirements. Any production activation requires a separate
Founder gate.

## Data And Privacy Impact

Option B exercises only synthetic/disposable exact-v5 fixtures. Correction
retains superseded ordinary Evidence content locally but makes it permanently
context-ineligible, as ADR-0011 requires. Deletion purges all Evidence content
and leaves only minimal content-free metadata/tombstone. Ordinary dependent
content remains visibly invalidated; Historical Question generated and
actual-used content is deleted under ADR-0009. No provider call, consent reuse,
real-user data, app-data access, or transmission is authorized.

## Orchestrator Recommendation

Select **Option B**. It prioritizes semantic completeness over line count and is
independently verifiable against exact migrated disposable fixtures. Do not add
Phase 3B v5 creation parity to this slice, but keep that parity as an explicit
precondition for any later production schema-v5 activation.

## Default Safe Action

Remain at `human_decision_required`. Make no implementation, schema, runtime,
provider, UI, Git, deployment, or release change.

## Blocked Files Or Phases

- Any new or modified Rust lifecycle implementation or integration tests.
- Production `SCHEMA_VERSION`/`user_version` 5 and real-user migration.
- Phase 3B v5 runtime creation parity unless separately selected and authorized.
- Tauri, renderer, UI, startup, app-data, provider, ContextPacket, consent,
  retention, export v2, backup/restore activation, Phase 4, and deployment.
- Engineering Planning and implementation until this decision is resolved.

## Exact Founder Response Needed

To authorize the recommendation, reply exactly or equivalently:

> I resolve PHASE3C-SLICE4C1-001 by selecting Option B. I authorize Phase 3C
> Slice 4C-1 only: exact-v5 disposable fixtures produced through the promoted
> migration core; one private, unregistered, path/connection-injected Rust
> confirmed-Evidence lifecycle boundary for exact-current confirmed Evidence
> correction and explicit deletion; correction as one new immutable
> user-authored revision with exact predecessor, retained prior AI/local-mock
> content and provenance, corrected/superseded facts, pending/ineligible head,
> no confirmation carry-forward, and guarded v4 candidate projection; deletion
> as explicit lifecycle fact, purge of all Evidence content, deleted/ineligible
> head with no current revision, minimal content-free artifact tombstone, and v4
> projection removal; complete promoted Experience/Evidence/Reflection/Pattern
> verifier reuse; well-formed unique durable IDs before set equality; complete
> exact direct and transitive Evidence-to-Reflection-to-Pattern dependency
> closure; edge-specific invalidation facts, current dependent heads retained
> but invalidated/ineligible, v4 projection omission, and no content rewrite,
> regeneration, recalculation, reconfirmation, or dependency rebinding; complete
> ADR-0009 cascade deletion for every affected migrated Historical Question
> across its one-to-one schema-v4 and normalized-v5 representation, including
> generated content, successful packet snapshot and actual-used provenance while
> preserving only already-permitted minimal audit metadata and the existing
> 30-day unsuccessful-attempt boundary; exact one-to-one Historical Question
> representation as a hard input precondition with missing, extra,
> contradictory, malformed, cross-source, cyclic, duplicate, unsupported, or
> partial lifecycle state failing closed; synchronized schema-v5 authority and
> guarded schema-v4 projection in one BEGIN IMMEDIATE transaction; deterministic
> clocks, identifiers, guards, failure points, commit outcomes, exact pre/post
> manifests, foreign-key and integrity checks; read-only ambiguous-COMMIT
> reconciliation with no retry, replay, rollback, repair, cleanup, rebind, or
> candidate selection; focused synthetic/disposable tests; factual Book One
> documentation; Clippy; canonical verification; Theory Alignment Review;
> archive/reset; and stop at Founder diff review. I accept that Phase 3B v5
> runtime creation parity is not part of this disposable slice but remains a
> blocker for production schema-v5 activation. I do not authorize production
> SCHEMA_VERSION or user_version 5, real user databases or app-data, migration
> or fresh-v5 initialization, startup/Tauri/renderer/UI activation, Phase 3B v5
> write parity, provider or ContextPacket changes, consent-policy changes, new
> transmission, automatic generation or reconfirmation, ordinary content
> rewriting, Context Recovery correction/deletion, retention, export v2,
> backup/restore activation, Phase 4, sensitive inference, Harness expansion,
> staging, commit, push, merge, PR, deployment, or release.

The Founder may instead select A, C, D, or E and must state the exact authorized
scope. Silence does not resolve this decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4C1-001 by selecting Option B. I authorize Phase 3C Slice 4C-1 only: one private, unregistered, path/connection-injected disposable schema-v5 confirmed-Evidence lifecycle boundary for exact-current confirmed Evidence correction and explicit deletion; correction creates an immutable user-authored successor revision, retains prior content and provenance, returns the head to pending/ineligible, and carries no confirmation forward; deletion purges all Evidence content and retains only minimal content-free metadata and tombstone; complete exact direct and transitive Evidence-to-Reflection-to-Pattern invalidation without rewriting, regeneration, reconfirmation or dependency rebinding; complete ADR-0009 cascade deletion across each affected Historical Question’s one-to-one schema-v4 and normalized-v5 representation; guarded schema-v4 projection synchronization; malformed, stale, incomplete, duplicate, cross-source, cyclic, unsupported or contradictory state fails closed; deterministic disposable tests, exact reconciliation, Clippy, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review.

I accept that Phase 3B v5 runtime creation parity is not included but remains required before production schema-v5 activation. I do not authorize production schema/user_version 5, real user data, migration, runtime/Tauri/UI activation, Phase 3B v5 write parity, provider or ContextPacket changes, consent changes, Phase 4, Harness expansion, staging, commit, push, merge, PR, deployment or release.

## Selected Option And Authorized Scope

- Selected option: B
- Authorized scope: Phase 3C Slice 4C-1 Option B only: private, unregistered, path/connection-injected disposable confirmed-Evidence correction/deletion; complete exact ordinary direct/transitive invalidation; complete ADR-0009 migrated one-to-one Historical Question cascade; guarded v4 projection; deterministic disposable tests and exact reconciliation; documentation, Clippy, canonical verification, Theory Alignment Review, archive/reset, stop at Founder diff review. All production schema/runtime/real-data/Phase 3B-v5/provider/consent/Phase-4/Harness/Git/deployment/release authority remains withheld.

## Decided At And Evidence Reference

- Decided at: 2026-08-01T18:15:45.827Z
- Evidence reference: PHASE3C-SLICE4C1-001

## Resume Phase

product_review
