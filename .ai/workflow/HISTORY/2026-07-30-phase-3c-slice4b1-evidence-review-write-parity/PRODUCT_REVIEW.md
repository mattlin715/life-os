# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-30-phase-3c-slice4b1-evidence-review-write-parity
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: c7fc0c7a61d3b4f44237a83bf8288d1a7d8ae4ca
- Working-tree digest reviewed: d40d564c2f4788da86f632c022098e12c1581a68e3788a4e74b463ba350c4b01
- Created at: 2026-07-30T04:15:00+09:00
- Updated at: 2026-07-30T04:15:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Close factual Slice 4A promotion drift and decide whether to build the
smallest private disposable-fixture proof that Evidence candidate creation,
pending correction, explicit confirmation, and rejection keep schema-v5
authority and the schema-v4 compatibility projection atomic. This is product
data-integrity work governed by the existing Harness, not production
schema-v5 activation or a user-visible feature.

## Problem Statement

Promoted Slice 4A proves Experience current-state write parity, but ordinary
artifacts still have no post-cutover typed write boundary. The current schema-v4
path rewrites a complete JSON bundle: pending edits mutate payload in place,
confirmation mutates status, and rejection removes the record. That behavior
cannot represent append-only authorship, exact-revision review, rejection
history, or v5/v4 atomicity required by ADR-0011.

## User Value

There is no immediate UI change. The bounded value is trustworthy future
review behavior: an AI/local-mock suggestion remains distinguishable from a
user correction; confirmation applies to one exact revision; rejection does
not become silence; and compatibility storage cannot drift away from the
normalized history.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: user meaning remains user-owned; the system is a
  mirror rather than an oracle.
- `docs/03_Principles.md`: Evidence supports self-understanding but is not
  objective truth; confirmation is a user judgment, not AI authority.
- `docs/06_Memory.md`: durable memory needs provenance, correction, deletion,
  and user control.
- `docs/Reflection.md`: hypotheses and questions remain inspectable and
  non-final.
- `docs/09_AI.md`: AI outputs remain revisable, rejectable hypotheses.
- `docs/10_Privacy.md`: rejected content cannot become a hidden reusable
  archive.
- `docs/appendix/Harness.md`: generated artifacts preserve source, authorship,
  review, and task boundaries.

## Relevant ADRs

- ADR-0007 requires durable reviewed artifacts to preserve artifact type,
  source, AI/user authorship, review state, provenance, revision, correction,
  deletion, and retention distinctions.
- ADR-0009 allows only confirmed Evidence in governed historical packets and
  requires exact-revision staleness/deletion behavior.
- ADR-0011 accepts append-only artifact revisions, exact review events,
  revision-bound dependencies, immediate rejected-content purge, retained
  superseded content, reconfirmation after correction, v4 projection during
  cutover, and no automatic down migration. It does not by itself authorize
  this implementation.

## Current Implementation Context

- **Promoted and verified:** fixed v5 DDL/contract tests; private disposable
  v4-to-v5 migration/restart evidence; private Experience Slice 4A writes;
  production schema-v4 typed artifact bundle writes; Phase 3B eligibility and
  deletion regressions.
- **Founder-approved design:** normalized v5 is authoritative after verified
  cutover; v4 remains a guarded current-state projection; explicit review and
  lifecycle facts are immutable.
- **Production current state:** `SCHEMA_VERSION = 4`; no v5 startup, Tauri,
  renderer, UI, app-data, or real-user write path exists.
- **Proposed only:** a private path/connection-injected Evidence mutation
  boundary exercised exclusively against exact-v5 disposable fixtures.

The Slice 4A promotion facts were corrected minimally in
`docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.

## In Scope

- Decision `PHASE3C-SLICE4B1-001`.
- Exact-v5 fixtures produced through promoted migration evidence.
- New AI/local-mock Evidence candidate creation.
- Correction of an exact current still-pending candidate through a new
  user-authored revision.
- Explicit confirmation or rejection of one exact current pending revision.
- Atomic v5 authority, exact Experience dependency, review/lifecycle history,
  rejected-content purge/tombstone, and v4 compatibility projection.
- Deterministic test inputs, failure injection, read-only reconciliation,
  factual architecture synchronization, canonical verification, Theory
  Alignment Review, archive/reset, and Founder diff review after separate
  approval.

## Out Of Scope

Production schema/user-version 5; real user/app-data databases; runtime
registration; UI; whole-bundle replacement; confirmed-Evidence correction or
deletion; ordinary dependent invalidation; Reflection, Pattern, Context
Recovery, Historical Question or Phase 3B v5 writes; retention jobs; export v2;
production recovery; provider/ContextPacket changes; Phase 4; Harness changes;
Git promotion; PR; deployment; release.

## Product Constraints

- An Evidence artifact remains a hypothesis even when confirmed.
- AI, local mock, and user authorship remain distinct per revision.
- Confirmation/rejection requires an explicit action naming the exact current
  pending revision; silence creates no event.
- Correction never rewrites the prior AI/local-mock revision or its provenance,
  returns the artifact to pending, and never carries confirmation.
- Rejection purges content synchronously and retains only content-free
  history/provenance/dependency facts permitted by ADR-0011.
- V5 is authoritative only inside an exact disposable v5 fixture; v4 is a
  projection. Both commit or roll back together.
- Any dependent requiring unimplemented lifecycle behavior causes a
  fail-closed no-write outcome.

## Evidence And Provenance Constraints

Candidate creation requires complete canonical provenance whose origin is
exactly `ai` or `local_mock`; provider/model/Harness/prompt metadata cannot be
invented or silently changed. A pending correction creates a separate
`authorship=user` revision with user provenance. The prior generated revision,
its content, digest, provenance link, source dependency, and lifecycle history
remain immutable and inspectable. Confirmation does not change authorship or
content.

## Historical Context Constraints

Only confirmed Evidence is eligible under ADR-0009. The proposed commands
create or change only pending Evidence, so a pre-existing Historical Question
dependency on the subject revision is contradictory and must fail closed.
The accepted ADR-0009 cascade remains authoritative for a future separately
authorized operation that invalidates legitimately included confirmed
Evidence; this slice neither creates nor exercises Phase 3B v5 writes.

## Consent Constraints

No selection, packet, consent, transmission, or provider behavior is created
or broadened. A disposable Evidence command cannot infer consent or reuse a
historical packet.

## AI-Role Constraints

The module persists an already supplied candidate; it performs no model call,
inference, diagnosis, taxonomy, identity judgment, summary, or Phase 4
interpretation. Review events record user action without converting the
candidate into truth.

## Privacy Constraints

Pending/superseded content stays local in disposable fixtures. Rejection
clears the current pointer, purges the exact content row in the same
transaction, creates a content-free `rejected_content_purged` tombstone, and
removes the v4 projection. It does not retain rejected text, transmit data, or
start background cleanup.

## User-Agency Constraints

No runtime action is activated. Explicit confirmation/rejection is required;
duplicates, conflicts, stale revisions, malformed provenance, and ambiguous
state do not become inferred user intent. Any future product UI and real-user
activation require a later Founder gate and manual review.

## Acceptance Criteria

1. Exact-v5 disposable fixtures only; v4, malformed, inconsistent, or newer
   databases refuse read-only.
2. AI and local-mock creation each append one exact generated revision,
   provenance link, source dependency, created lifecycle event, pending head,
   and matching v4 candidate projection atomically.
3. Pending correction requires exact current source and artifact revisions,
   appends one user-authored revision, preserves the prior generated revision
   and provenance, appends corrected/superseded events, and remains pending.
4. Confirm inserts one exact user review event, changes only the head/current
   projection to confirmed/eligible, and never rewrites content or authorship.
5. Reject inserts one exact user review event, clears the head pointer, purges
   content, appends content-purged evidence/tombstone, marks the head rejected
   and ineligible, and removes only the v4 current projection.
6. Duplicate or conflicting reviews, stale source/artifact revisions,
   unsupported kind, malformed payload/provenance, and any unauthorized
   dependent write nothing.
7. The exact current source Experience revision is re-read inside
   `BEGIN IMMEDIATE`; dependencies bind to that revision and are never rebound.
8. Injected failure after each meaningful step restores the exact logical
   pre-state, including guard, authority, projection, events, provenance,
   dependencies, content, and tombstones.
9. Commit ambiguity is classified after close/read-only reopen as exact
   committed, exact unchanged, or `recovery_required`; no autonomous action
   follows.
10. Read-only reconciliation proves v5/v4 semantics, immutable receipt,
    schema/contract, manifest, guard emptiness, current-content invariants,
    foreign keys, and integrity.
11. Slice 4A, R1, P1, Phase 3B, production schema-v4, provider, ContextPacket,
    UI, and Harness regressions remain unchanged.
12. Canonical verification and Theory Alignment Review pass before Founder
    diff review.

## Risks

- A compatibility payload cannot represent full lineage; v5 must remain
  authority and reconciliation must compare a defined projection, not infer
  history from v4 JSON.
- Reject ordering can accidentally leave content current or erase review
  history; head clearing, purge, event, tombstone, and projection deletion must
  be one guarded transaction.
- A user correction may accidentally erase generated provenance or be mislabeled
  as objective/user-origin Evidence; immutable prior history and artifact-class
  semantics are mandatory.
- Ordinary dependents need Decision 10B invalidation, which is not implemented
  here. Deleting or rebinding them would exceed authority.
- There is no durable per-write receipt; immediate ambiguous-commit evidence
  does not prove production crash/restart safety.
- Passing disposable tests is not production authorization or real-user safety
  evidence.

## Open Questions

None. The Founder resolved `PHASE3C-SLICE4B1-001` with Option A and accepted
the exact transaction contract. Every stated exclusion remains a mandatory
Engineering Plan condition.

## Human Decision Required

No. `PHASE3C-SLICE4B1-001` is resolved with Option A.

## Recommendation

Proceed to Engineering Planning for Option A only. Copy every authorization
and exclusion into the plan. Production activation, confirmed-Evidence
mutation, ordinary dependent invalidation, Phase 3B v5 writes, later artifact
classes, runtime integration, and Git promotion remain blocked.

## Review Status

approved_with_conditions
