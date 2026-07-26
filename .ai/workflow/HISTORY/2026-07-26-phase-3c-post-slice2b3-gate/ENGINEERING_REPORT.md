# Engineering Report

Status: completed

- Sprint ID: 2026-07-26-phase-3c-post-slice2b3-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: e932ead6da3346d3783da22dc1d1295c31cdc979
- Working-tree digest implemented: 087590e9f394a6917dbbf5ed4c7d63ca8396cc5a69754f881ca021877eac5e45
- Created at: 2026-07-26T05:40:00+09:00
- Updated at: 2026-07-26T06:05:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the Founder-authorized Slice 3A disposable migration core. One
private crate-local Rust module consumes the unchanged fixed schema-v5 DDL,
requires an injected exact-v4 path and expected source-manifest digest, runs a
single atomic DDL/backfill/reconciliation/receipt/guard/version transaction,
and performs read-only post-commit verification. The module has no Tauri,
startup, renderer, UI, app-data, or real-user-data caller.

## Existing System Areas Inspected

- `src-tauri/src/sqlite.rs` startup/version and schema-v4 mutation boundary.
- `src-tauri/src/filesystem_safety.rs` unactivated backup/replacement boundary.
- `src-tauri/tests/schema_v5_contract.rs`, `schema_v5_backup.rs`, and all fixed
  v2/v3/v4 fixtures.
- `src/shared/storage/` persisted artifact shapes and ADR-0009 mutation paths.
- ADR-0007, ADR-0009, ADR-0010, ADR-0011, architecture/12, and
  architecture/13.

## Files Added

- `src-tauri/schema/schema_v5.sql` — unchanged executable DDL relocated from
  the test-fixture directory so contract tests and the private module share one
  compile-time source.
- `src-tauri/src/schema_v5_migration.rs` — private migration core plus ten
  synthetic/disposable unit tests.

## Files Modified

- `src-tauri/src/lib.rs` — private module declaration only; no command
  registration.
- `src-tauri/tests/schema_v5_contract.rs` — shared DDL include path and factual
  scope assertion.
- `src-tauri/tests/fixtures/schema_v5/contract.json` — factual Slice 3A scope;
  schema/digest/object contracts unchanged.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md` —
  version 1.9 authorization and working-tree evidence synchronization.
- Repository workflow mission, decision, Product Review, plan, state, and event
  artifacts.

## Files Deleted

- `src-tauri/tests/fixtures/schema_v5/schema_v5.sql` was relocated byte-for-byte
  to `src-tauri/schema/schema_v5.sql`; it was not removed as an authority.

## Behavior Changed

- Private code can migrate only an explicitly injected, existing exact-v4
  SQLite path when its in-transaction source manifest equals the caller's
  expected digest.
- The fixed DDL is digest-checked, parsed into reconstructable statements, and
  executed with compatibility-projection guards last.
- Exact Experience text, legacy artifact/Historical payload bytes, packet
  snapshots, consent/transmission records, and ADR-0009 dependencies remain
  unchanged.
- Honest review mappings import confirmed/skipped facts with uncertain legacy
  action time; pending and not-applicable states create no invented decision.
- Pre-commit errors explicitly roll back; post-commit inconsistency returns
  `recovery_required` without repair.
- No product runtime behavior changed.

## Data Model Impact

Only disposable fixtures receive the approved schema-v5 tables, one baseline
revision per surviving v4 record, immutable provenance/review/lifecycle facts,
dependencies, disabled operational contract, and migration receipt. Production
schema and user data remain v4 and untouched.

## Migration Impact

The private module sets `user_version = 5` only as the final SQL mutation in a
disposable migration transaction. Production `SCHEMA_VERSION` remains 4. No
fresh-v5 initialization, v2/v3 stabilization, production activation, down
migration, backup, restore, or replacement was added.

## Provenance Impact

Exact provenance objects are normalized only for domain-separated fingerprint
records and role links; revision content bytes are never reserialized. Missing
legacy provenance remains `legacy_unknown`. User, AI, local-mock, mixed, and
unknown authorship remain distinct.

## Historical Context Impact

Historical Question payload, packet snapshot, packet digest, consent,
transmission, provider/model, revision strings, and v4 cascade rows remain
byte-preserved. The migration adds only an exact lifecycle link and dependencies
to resolved current baseline revisions. Stale, ineligible, or consent/digest
mismatches abort.

## Consent Impact

None. Existing consent is an immutable reconciliation input. No consent is
created, consumed, reused, extended, or reinterpreted.

## Provider Transmission Impact

None. No provider/ContextPacket source changed and no network path exists.

## Tests Added

Ten Rust unit tests cover:

1. shared DDL digest/reconstruction/guard ordering and deterministic collision
   refusal;
2. complete exact-v4 migration with every artifact kind and review mapping;
3. exact raw and ADR-0009 preservation;
4. deterministic IDs/manifests across independent fixtures;
5. every fixed DDL statement and meaningful transaction boundary rollback;
6. malformed/v2/v3/v5/newer refusal without mutation;
7. rejected, stale, ineligible, and consent-digest mismatch refusal;
8. source content/timestamp change after manifest capture refusal;
9. injected and actual post-commit inconsistency becoming
   `recovery_required` without repair;
10. immutable receipt and disabled lifecycle/export flags.

## Tests Executed

- `cargo test schema_v5_migration --lib --no-fail-fast` — 10/10 passed.
- `cargo test --all-targets --no-fail-fast` — Rust library 59/59 total after
  Slice 3A, backup/restore integration 12/12, schema contract 8/8; passed.
- `cargo clippy --all-targets -- -D warnings` — passed.
- `cargo fmt --all` and `git diff --check` — passed.

## Verification Results

Focused implementation, integration, contract, formatting, and lint checks
passed. Canonical repository verification is intentionally recorded in the
next workflow validation phase after this report is frozen.

## Manual Verification Required

No desktop runtime verification applies because the module is private,
unregistered, path-injected, and has no product caller. Founder diff review is
required after canonical verification and Theory Alignment Review.

## Documentation Updates

Architecture/13 version 1.9 records the exact Founder resolution, shared DDL
location, disposable evidence, and preserved production activation fences.
No new design document was added.

## ADR Impact

No ADR or ADR status changed. ADR-0011 remains Accepted; the implementation is
bounded by the exact `PHASE3C-SLICE3A-001` exception.

## Deviations From Plan

Theory Review Cycle 0 required the migrated Experience baseline serialization
label to be `legacy-v4-raw` rather than `utf8-text-v1`, matching the exact
Founder wording and avoiding any implication that legacy content was newly
serialized. The exact bytes and digests were already preserved; an assertion
now freezes the honest source/artifact labels. Architecture/13 was also updated
with the passed canonical counts. The fixed DDL and object/digest contracts are
unchanged. The contract JSON scope sentence was factually updated because
`user_version = 5` is now exercised by a private disposable test path; no
schema contract field changed.

## Known Limitations

- This does not test real user data, production app-data, startup orchestration,
  process-wide quiescence, backup/restore activation, Windows durability, or
  production recovery.
- Logical exact-v4 rollback proves schema objects, version, governed rows,
  manifests, and ADR-0009 records; it does not claim physical SQLite file-byte
  identity.
- Lifecycle writes, export v2, fresh-v5 initialization, and v5 runtime write
  parity remain disabled/unimplemented.
- Passing fixture tests is not production migration authorization.

## Remaining Risks

Accidental future registration or mistaken promotion as production-ready is
the primary residual risk. Module privacy, no handler/caller, production
version 4, explicit architecture fences, exact source-manifest requirement,
and fail-closed verification reduce but do not eliminate that governance risk.

## Git State

- Branch: `codex/phase-3c-slice3a-disposable-migration-core`.
- HEAD: `e932ead6da3346d3783da22dc1d1295c31cdc979`.
- Upstream: none configured.
- Working tree: expected unstaged Slice 3A/workflow changes only.
- Staged files: none.
- Commit, push, merge, PR, and deployment: not performed.

## Engineer Completion Status

completed — implementation is ready for canonical validation and Theory
Alignment Review, not promotion.
