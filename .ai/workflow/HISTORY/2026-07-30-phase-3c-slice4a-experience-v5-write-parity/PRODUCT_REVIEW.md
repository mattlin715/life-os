# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-30-phase-3c-slice4a-experience-v5-write-parity
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: cf7633780a1a0a72efcad7558e463ceb094468c4
- Working-tree digest reviewed: 3beab4235b0c7f3a885d684ae5032b89808fcc0c17c34b7285a2b788aea8a1d3
- Created at: 2026-07-30T01:15:00+09:00
- Updated at: 2026-07-30T01:19:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Close factual Provenance Inspector P1 promotion drift and decide whether to
build the smallest private disposable-fixture proof that Experience
current-state mutations keep schema-v5 authority and the schema-v4
compatibility projection atomic. This is product data-integrity work governed
by the existing Harness, not a Harness feature or production schema-v5
activation.

## Problem Statement

Promoted Slice 3A can migrate an exact disposable v4 fixture to v5 and promoted
Slice 3B can classify its durable migration outcome, but no post-cutover
Experience mutation exists. The current production typed commands support only
schema v4. Directly reusing them on v5 is blocked by compatibility triggers and
would update only the projection even if bypassed.

## User Value

There is no immediate visible feature. The value is a bounded prerequisite for
honest correction and deletion: user-authored Experience changes must become
append-only revisions without allowing the compatibility projection to drift
or weakening dependent Historical Question deletion.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: user meaning and content remain user-owned; local
  control, transparency, correction, deletion, and consent are constitutional
  constraints.
- `docs/03_Principles.md`: evidence and uncertainty distinctions cannot be
  erased by storage convenience.
- `docs/06_Memory.md`: memory requires provenance, correction, deletion, and
  user control.
- `docs/09_AI.md`: Experience content is not AI output.
- `docs/10_Privacy.md`: data minimization and fail-closed local behavior apply
  to lifecycle storage.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md` requires
  durable reviewable provenance and revision/deletion distinctions.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`
  keeps exact Historical Question actual-use records tied to their sources and
  requires cascade deletion when an included source changes or is deleted.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`
  accepts normalized v5, append-only Experience revisions, exact dependencies,
  content separation, and no automatic down migration. It does not by itself
  authorize this implementation.

## Current Implementation Context

- **Promoted and verified:** fixed v5 DDL/contract tests; private disposable
  v4-to-v5 migration and restart classification; typed schema-v4 Experience
  CRUD/import; R1 retrieval; P1 local actual-use inspector.
- **Founder-approved design:** v5 becomes authoritative after verified cutover;
  v4 remains a guarded compatibility projection; writes update both in one
  transaction; no invented pre-v5 history.
- **Production current state:** `SCHEMA_VERSION = 4`; no v5 startup, Tauri,
  renderer, UI, app-data, or real-user write path.
- **Proposed only in this sprint:** a private path/connection-injected
  Experience v5 write boundary exercised solely against disposable fixtures.

The exact P1 promotion facts were corrected in `docs/11_MVP.md`,
`docs/12_Roadmap.md`, and
`docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`.

## In Scope

- Decision `PHASE3C-SLICE4A-001`.
- Exact-v5 fixtures produced by the promoted migration core.
- Private Experience create, expected-revision update, delete, and atomic
  duplicate-skipping import proposal.
- Atomic v5 authority plus v4 projection, deterministic fixture clock/ID
  inputs, failure injection, read-only reconciliation, and ADR-0009 cascade.
- Factual Book One synchronization, tests, canonical verification, Theory
  Alignment Review, archive/reset, and Founder diff review only after explicit
  authorization.

## Out Of Scope

Production schema-v5 activation, real user/app-data databases, Tauri or
renderer registration, startup/UI changes, general artifact/Reflection/Pattern
or Phase 3B write parity, lifecycle UI, export v2, retention cleanup, automatic
recovery, provider/ContextPacket changes, Phase 4, Harness changes, Git
promotion, PR, deployment, and release.

## Product Constraints

- V5 is authoritative only in an exact disposable v5 fixture.
- V4 is a projection and is never independently merged back into v5.
- Stale or inconsistent state writes nothing.
- The compatibility guard is transaction-local and empty at commit/reopen.
- Update never overwrites prior revision metadata/content.
- Ordinary artifact invalidation/reconfirmation is deferred; a mutation that
  would require unimplemented ordinary-artifact lifecycle behavior must fail
  closed in Slice 4A.

## Evidence And Provenance Constraints

New Experience revisions are explicitly user-authored. Create uses `created`,
correction uses `corrected`, and v4-format import uses
`legacy_v4_baseline`; none may be substituted for another. Exact UTF-8 content
digests and domain-separated provenance fingerprints are required. Migration
receipts remain immutable migration evidence rather than being rewritten as a
current-state manifest.

## Historical Context Constraints

Existing exact-revision dependencies are never rebound. Update/delete must
delete affected ADR-0009 Historical Questions, packet snapshots, consent and
transmission records through the accepted cascade. Ordinary non-historical
dependents that require Decision 10B invalidation remain outside Slice 4A and
therefore block the mutation instead of being silently deleted or rebound.

## Consent Constraints

No selection, consent, packet, transmission, or provider behavior is created or
broadened. The only consent effect is deletion of already persisted records
when ADR-0009 requires source invalidation.

## AI-Role Constraints

No AI call, inference, summary, taxonomy, identity claim, or Phase 4
interpretation occurs. Experience revisions remain user content.

## Privacy Constraints

Prior corrected Experience content remains locally retained because Decision
9A applies the approved revision-content boundary. Parent Experience deletion
purges source content and source-scoped child records; it does not preserve
hidden content. No backup, export, telemetry, or provider transmission is
introduced.

## User-Agency Constraints

No production-facing action is activated. Any future production mutation and
its disclosure require later Founder authority and manual review. Failure and
post-commit ambiguity return a blocked result; the module does not retry,
repair, restore, or select a database autonomously.

## Acceptance Criteria

1. Exact-v5 disposable fixtures only; v4, malformed, inconsistent v5, and
   newer databases refuse without mutation.
2. Create writes one user-authored `created` source revision/content/provenance
   and matching v4 row atomically.
3. Update requires the exact current v5 revision ID, appends one
   user-authored `corrected` revision, preserves the predecessor, and advances
   both projections atomically.
4. Stale update/delete and projection drift write nothing.
5. Delete clears the source head, purges source content and parent-scoped
   records under the accepted deletion boundary, and preserves no hidden
   content.
6. Affected Historical Questions and actual-use records follow ADR-0009 in the
   same transaction; exact dependencies are not rebound.
7. V4-format import creates only one honest baseline per new ID, skips
   consistent duplicate IDs, and rolls back the whole batch on any error.
8. Injected failure after every meaningful authority/projection step restores
   the exact pre-transaction logical manifests.
9. Guard count is zero after success, rollback, commit-outcome classification,
   and read-only reopen.
10. Read-only post-write verification proves schema/receipt/contract shape,
    source-current/projection parity, exact expected manifests, current-content
    invariants, foreign keys, and integrity.
11. Existing R1/P1 behavior and production schema-v4 code remain unchanged.
12. Canonical verification passes and Theory Alignment Review approves the
    exact diff before Founder review.

## Risks

- Source deletion semantics are privacy-sensitive; keeping content or child
  tombstones past parent deletion would violate the accepted policy.
- Updating a source with ordinary artifacts requires lifecycle invalidation
  not authorized here; accidental delete-all would violate Decision 10B.
- Migration receipt source/target manifests describe cutover, not mutable
  current state; treating them as live manifests would falsely block every
  valid post-cutover write.
- A process loss after commit has no durable per-write operation receipt in
  this slice. In-process ambiguous-commit classification is testable, but
  production crash recovery remains unproved.
- A guard token is not an authentication boundary.
- Passing disposable tests is not production authorization or real-user
  safety evidence.

## Open Questions

None. The Founder resolved `PHASE3C-SLICE4A-001` with Option A and accepted
the exact transaction contract. The authorized-scope fences remain mandatory
Engineering Plan conditions.

## Human Decision Required

No. `PHASE3C-SLICE4A-001` is resolved with Option A.

## Recommendation

Proceed to Engineering Planning for Option A only. Copy every authorization
and exclusion into the plan. In particular, ordinary artifact lifecycle,
production activation, real data, runtime registration, and Git promotion
remain outside the sprint.

## Review Status

approved_with_conditions
