# Engineering Plan

Status: approved

- Sprint ID: 2026-07-30-phase-3c-slice4b1-evidence-review-write-parity
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: c7fc0c7a61d3b4f44237a83bf8288d1a7d8ae4ca
- Working-tree digest reviewed: d40d564c2f4788da86f632c022098e12c1581a68e3788a4e74b463ba350c4b01
- Created at: 2026-07-30T04:55:00+09:00
- Updated at: 2026-07-30T04:55:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Founder decision
`PHASE3C-SLICE4B1-001` authorizes only a private, unregistered, disposable
exact-v5 Evidence boundary for generated candidate creation, still-pending
user correction, exact-revision confirmation, and exact-revision rejection.
All production/runtime, confirmed-Evidence mutation, dependent lifecycle,
Phase 3B v5, later artifact, migration activation, Git promotion, deployment,
and release exclusions remain binding.

## Existing Implementation Understanding

- `schema_v5_migration.rs` owns the fixed DDL, canonical JSON/provenance,
  deterministic IDs, exact-v5 migrated fixtures, contract verification, and
  conservative `CommitOutcomeAdapter`.
- Nested `schema_v5_experience_write.rs` proves Slice 4A guard ordering,
  logical manifests, read-only reopen, and ambiguous commit classification.
- Schema v5 separates artifact heads, immutable revisions/content/provenance,
  review/lifecycle events, exact dependencies, tombstones, and guarded v4
  compatibility tables.
- Production `sqlite.rs` remains schema v4 and is not a caller for this module.

## Affected Modules

- New: `src-tauri/src/schema_v5_evidence_write.rs`.
- Minimal private nesting declaration:
  `src-tauri/src/schema_v5_migration.rs`.
- Factual synchronization:
  `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Repository workflow artifacts under `.ai/workflow/`.

No Tauri command, renderer, UI, provider, ContextPacket, production schema, or
startup module is affected.

## Proposed Design

1. Add typed private commands for create, correct-pending, confirm-pending, and
   reject-pending with exact expected source/artifact revision IDs.
2. Reuse the parent migration module's fixed contract, canonicalization,
   provenance fingerprint, deterministic ID, migration fixture, and commit
   adapter primitives.
3. Validate exact-v5/receipt/contract/source/projection/guard/integrity state,
   then use one `BEGIN IMMEDIATE` transaction and one exact guard token.
4. Write v5 immutable authority first, then head and v4 projection; reconcile
   all affected rows before guard removal and commit.
5. Reopen read-only and classify exact post-state, exact pre-state, or
   `recovery_required` without autonomous action.
6. Keep every execution inside module-local synthetic/disposable tests.

## Alternatives Considered

- Contracts-only: rejected by Founder Option A because it does not prove
  transaction ordering or rollback.
- Extend Slice 4A into a generic mutation engine: rejected because it would
  obscure artifact-specific review/purge semantics and broaden a promoted
  boundary.
- Reuse production schema-v4 bundle replacement: rejected because it cannot
  represent append-only review/rejection history and is blocked by v5 guards.

## Data Lifecycle Impact

Disposable data only. Candidate and correction content remain append-only.
Rejection clears the current pointer, deletes the exact content row, records
explicit review/lifecycle facts and a content-free tombstone, and removes the
v4 projection in one transaction.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, startup maximum, or production
`user_version` change. Tests execute only on exact-v5 fixtures created through
the promoted disposable migration core.

## Provenance Impact

AI and local-mock revisions retain exact generated provenance; user correction
gets distinct user provenance. Prior links are immutable. Review never rewrites
content or provenance.

## Historical Context Impact

No Phase 3B write or cascade is implemented. Because only pending Evidence may
be corrected/rejected, any inbound historical or ordinary dependency is
contradictory and fails closed. ADR-0009 remains unchanged for future
separately authorized confirmed-Evidence invalidation.

## Consent Impact

None. No selection, consent, packet, transmission, or actual-use record is
created or reused.

## Provider Transmission Impact

None. The module receives already formed synthetic candidate data and makes no
provider call.

## Import And Export Impact

None. No import, export v2, file output, retention, or backup behavior changes.

## Test Strategy

- AI and local-mock candidate creation with exact authorship/provenance.
- Exact source-revision dependency and v4 pending projection.
- Pending correction appends user revision and corrected/superseded events.
- Confirmation exact-review event without content mutation.
- Rejection content purge, tombstone, retained metadata/history, v4 omission.
- Duplicate/conflicting review, stale source/artifact, malformed provenance,
  unsupported kind, cross-source, and inbound-dependent refusal.
- Failure injection after every meaningful write boundary.
- Exact logical rollback, empty guard, read-only reconciliation, ambiguous
  commit pre/post classification, receipt immutability, FK/integrity.
- Existing Slice 4A and full repository regressions.

## Repository Verification Strategy

Run focused Rust tests and Clippy while iterating, then:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record the exact result in the workflow during validation.

## Manual UI Verification

Not applicable. The module is private, unregistered, and has no desktop,
renderer, UI, startup, app-data, or real-user surface.

## Rollback Or Recovery Strategy

Pre-commit failure rolls back the single transaction. Generic commit errors
are outcome-unknown until read-only reopen proves exact pre/post state; mixed
state returns `recovery_required`. No automatic retry, replay, rollback,
repair, restore, cleanup, or candidate selection exists. Removing the private
module affects no user database.

## Documentation Impact

Update architecture/13 only with factual disposable implementation evidence,
exact tests, and preserved production/later-slice fences. Do not modify Book
Zero, the Constitution, ADR status, or archived workflow history.

## ADR Impact

No new ADR and no ADR status change. The implementation is a bounded proof
under accepted ADR-0007, ADR-0009, ADR-0011 and Founder-approved
architecture/13.

## Risk Level

High for lifecycle correctness, bounded to disposable data. Rejection purge,
projection omission, and exact review history can conflict if ordered
incorrectly; exhaustive injected rollback/reconciliation tests are mandatory.

## Escalation Decision

No additional decision is required. Implement only the exact Founder-approved
Option A contract. Stop and return to `human_decision_required` if repository
constraints make that contract impossible without changing DDL, ADR policy,
production/runtime behavior, dependent lifecycle, or another excluded scope.
