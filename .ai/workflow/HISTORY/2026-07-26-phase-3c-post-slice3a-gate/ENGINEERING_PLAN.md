# Engineering Plan

Status: approved

- Sprint ID: 2026-07-26-phase-3c-post-slice3a-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `7e0e5c44e03e58769f834243477c901cb01771ab`
- Working-tree digest reviewed:
  `61d38f19aae1e1a91cd8b924f84eac1ac53527832e86f2652eb3bbeae3835b7c`
- Created at: 2026-07-26T12:38:15Z
- Updated at: 2026-07-26T12:38:15Z

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions` after exact Founder resolution
`PHASE3C-SLICE3B-001` Option A. Implement only private, unregistered,
synthetic/disposable migration restart and COMMIT-ambiguity orchestration.
Extend the existing owned-operation state contract; do not create a parallel
state authority. Closed read-only database and exact verified-backup evidence
remain authoritative. No production/runtime surface or automatic recovery
action is permitted.

## Existing Implementation Understanding

- `filesystem_safety.rs` owns collision-resistant operation directories,
  canonical live/backup/staging/state paths, exact verified schema-v4 backup
  evidence, replacement outcomes, and fail-closed restart inspection.
- Its state currently records only backup/replacement phases and only the
  expected database digest; source-manifest and exact-record evidence returned
  by backup verification are not durable in the state record.
- `schema_v5_migration.rs` performs exact-v4 DDL/backfill/reconciliation in one
  `BEGIN IMMEDIATE`, writes `user_version = 5` last, directly executes COMMIT,
  and verifies committed v5 read-only.
- Any COMMIT error currently calls a rollback whose result is ignored and
  returns an ordinary fail-closed error. There is no definite/unknown COMMIT
  adapter or restart classifier that can recognize a valid committed v5 after
  a lost response.
- The private modules have no runtime caller. Production `SCHEMA_VERSION` is 4.

## Affected Modules

- `src-tauri/src/filesystem_safety.rs`
- `src-tauri/src/schema_v5_migration.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- repository workflow artifacts only

No Tauri command, TypeScript, UI, provider, ContextPacket, production SQLite
initialization, app-data, backup/restore activation, or schema constant file is
in scope.

## Proposed Design

### One owned-operation state contract

Evolve the current private state schema and retain its exact operation identity
and path bindings. Add the migration phases:

- `Migrating`
- `CommitOutcomeUnknown`
- `V5Verifying`
- `V5Ready`
- `V4ReadyWithBackup`
- `V5BlockedRestoreAvailable`

Existing `Prepared`, `BackupVerified`, and `RecoveryRequired` remain. Existing
replacement phases remain for promoted tests but are not a second migration
authority.

Persist content-free evidence required for restart:

- backup database SHA-256;
- backup/source manifest digest;
- backup exact-record digest;
- backup schema/FK/integrity facts;
- migration ID and source/target manifests when durably observed;
- outcome class.

Add crate-private transition/snapshot helpers that revalidate operation
identity. State is evidence context only and never authorizes action.

### Conservative transaction adapter

Introduce an injectable adapter whose COMMIT result is:

- `Committed`;
- `DefinitelyNotCommitted`;
- `OutcomeUnknown`.

The system SQL adapter maps success to `Committed` and every generic SQL error
to `OutcomeUnknown`. Only deterministic test evidence may report
`DefinitelyNotCommitted`.

Pre-commit errors and definite non-commit may attempt rollback. Rollback result
is recorded only as diagnostic evidence; after rollback success or failure the
writable connection is closed and the outcome is established by read-only
inspection. Outcome-unknown never retries COMMIT and does not automatically
rollback or restore.

### Read-only restart classifier

Accept an explicitly injected set of owned-operation candidates. Zero,
multiple, malformed, identity-mismatched, or contradictory candidates become
`RecoveryRequired`; the classifier does not discover or choose a candidate
autonomously.

For exactly one candidate:

- validate the state and exact verified backup;
- inspect closed live database user_version and schema objects;
- exact valid v4 plus verified backup -> `V4ReadyWithBackup`;
- user_version 5 -> `V5Verifying`, then verify exactly one immutable receipt,
  database contract, fixed schema-object manifest, source/target manifests,
  empty guard, current-content invariants, foreign keys, and integrity;
- complete v5 evidence -> `V5Ready`;
- inconsistent v5 plus verified backup -> `V5BlockedRestoreAvailable`;
- anything not safely classifiable -> `RecoveryRequired`.

No classifier path mutates state, live database, backup, or candidate files.

### Orchestration entry

Add one private function that requires:

- exact `OwnedOperation`;
- exact `ExpectedCandidate` returned by promoted backup verification;
- final backup and live-v4 revalidation;
- Slice 3A migration request;
- injected transaction adapter.

It records `Migrating`, invokes the existing Slice 3A transaction, closes the
connection, and returns the read-only classifier result. It never retries,
restores, repairs, cleans, or selects.

## Alternatives Considered

- New independent migration-state file/module: rejected because it duplicates
  operation ownership and backup authority.
- Infer commit failure from SQL error text: rejected because transport/driver
  errors do not prove durable commit outcome.
- Automatically restore verified backup after ambiguous commit: rejected
  because it can overwrite valid durable v5 and violates explicit user control.
- Production startup activation: rejected by Founder scope and missing
  disclosure/quiescence/durability/real-data evidence.

## Data Lifecycle Impact

Synthetic/disposable fixtures only. Verified backup files are preserved.
Operation state remains content-free. No automatic cleanup or data selection.
No real user lifecycle changes.

## SQLite Or Migration Impact

Private fixture-only schema-v5 execution gains conservative commit adapters and
restart classification. `user_version = 5` remains limited to disposable test
databases. Production `SCHEMA_VERSION` and user_version remain 4. No fresh-v5
initialization, v2/v3-to-v5 path, or production migration.

## Provenance Impact

No provenance semantics change. Durable v5 verification reuses Slice 3A exact
receipt, source/target manifests, database contract, current-content and
ADR-0009 preservation checks. State stores digests/IDs only, never content.

## Historical Context Impact

None. Existing Historical Question packet, consent, transmission, provenance,
and dependency rows are only verified as migration data. No retrieval or
generation behavior changes.

## Consent Impact

None. No migration UI or consent action is implemented. Historical provider
consent remains separate and unchanged.

## Provider Transmission Impact

None. No provider or ContextPacket files are modified and no transmission
occurs.

## Import And Export Impact

None. Export v2 remains disabled and imports remain schema-v4 product behavior.

## Test Strategy

Add deterministic private Rust tests for:

1. interruption before backup -> `Prepared`;
2. interruption after verified backup -> `BackupVerified`;
3. connection loss during transaction;
4. rollback success and injected rollback failure;
5. COMMIT definite non-commit -> read-only exact v4;
6. COMMIT outcome unknown with durable v4;
7. COMMIT outcome unknown with valid durable v5;
8. commit success before state update;
9. post-commit verification failure with preserved backup;
10. missing/malformed/contradictory state;
11. exact v4 with verified backup;
12. exact v5 receipt/manifests;
13. v5 missing receipt/nonempty guard/schema drift;
14. backup missing/altered;
15. multiple candidates;
16. byte/digest proof that restart inspection is read-only;
17. no automatic restore, repair, cleanup, retry, or candidate selection.

Preserve all existing Slice 2 and Slice 3A tests.

## Repository Verification Strategy

- `cargo fmt --all -- --check`
- focused Rust library tests for filesystem and migration modules
- `cargo test --all-targets --no-fail-fast`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- canonical `scripts/verify.ps1`
- workflow validation at every transition

## Manual UI Verification

Not applicable. The implementation remains private, unregistered, and has no
desktop runtime surface. Founder diff review remains required.

## Rollback Or Recovery Strategy

No autonomous recovery is implemented. A known pre-commit failure is classified
only after closed read-only proof. Ambiguous COMMIT, rollback failure, malformed
state, or contradictory durable evidence preserves the backup and returns a
blocked state. Logical exact-v4 proof does not claim physical SQLite file-byte
identity.

## Documentation Impact

Update architecture/13 factually with the exact authorized Slice 3B boundary,
implemented private state/adapter/classifier evidence, test counts, and
continued production fences. Do not add a new architecture or ADR document.

## ADR Impact

No ADR change. Implementation remains within ADR-0011 and preserves ADR-0007,
ADR-0009, and ADR-0010 boundaries.

## Risk Level

High conceptually because commit ambiguity can affect durable user data, but
bounded operational risk because all execution is private and disposable-only.
Any implementation contradiction returns to Product Review or
`human_decision_required`.

## Escalation Decision

No additional escalation is required before implementation. Exact Founder
Option A is recorded. Any need for real user data, production activation,
automatic action, a parallel state authority, or broader schema behavior must
stop and request new Founder authority.
