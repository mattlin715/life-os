# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-07-19-phase-3c-slice-1b2-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented against: 1ef3aa0acd57756f7593e3fa792c321f0e164dcc
- Working-tree digest reviewed: 463f798caa77198068e7326c495263f44d69f9aedc86fafb9ee475767108050b
- Created at: 2026-07-18T21:11:05.8400011Z
- Updated at: 2026-07-18T21:21:00.0000000Z

## Implementation Summary

Implemented the Founder-authorized Slice 1B-2 schema-v4 mutation trust
boundary:

- `saveArtifacts` now sends a validated typed bundle to a named Rust command.
  Rust checks source metadata and dependencies, revalidates the optional
  Experience revision before mutation, replaces the bundle, and invalidates
  dependent Historical Questions in one `BEGIN IMMEDIATE` transaction.
- Historical consent uses a typed event. ID conflicts cannot rebind packet,
  provider/model, source-revision, or timing scope; state transitions are
  monotonic and preserve the existing granted/consumed/invalidated policy.
- Historical transmission uses a typed event. Transmission persistence and
  consent consumption are atomic; a same-ID event cannot rebind its scope.
- Historical Question persistence accepts the typed artifact, derives exact
  revisions, artifact eligibility expectations, destination provenance, and
  dependency rows from the packet, reuses the existing ADR-0009 in-transaction
  checks, and owns the fixed INSERT statements in Rust.
- Historical Question deletion and expired audit cleanup use named typed Rust
  commands and preserve schema-v4 triggers/reference retention.
- Both generic statement-array Tauri commands and the renderer `SqlStatement`
  boundary were removed. Renderer read-only SQL remains outside this slice.
- Production `SCHEMA_VERSION` and `user_version` remain 4.

## Files Changed

Product implementation and tests:

- `src-tauri/src/sqlite.rs`
- `src-tauri/src/lib.rs`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.test.ts`

Factual documentation:

- `docs/architecture/01_Local_Evidence_Store.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

Repository workflow artifacts are also changed for the current sprint.

## Tests Executed

Focused evidence completed:

- Rust library suite: 27/27 passed.
- SQLite adapter Vitest: 11/11 passed.
- TypeScript typecheck: passed.
- Source search: no renderer mutation SQL, `SqlStatement`, generic statement
  array call, `execute_sqlite_transaction`, or
  `execute_sqlite_historical_transaction` remains.
- `git diff --check`: passed.

New regressions cover artifact rollback/cascade, consent monotonicity and scope
conflict, transmission/consent atomicity, packet-derived dependencies, invalid
question sources, Historical deletion actual-use cascade, audit cleanup,
newer-schema refusal, `foreign_key_check`, and `integrity_check`.

## Verification Results

Canonical `scripts/verify.ps1` passed with exit code 0 against the completed
implementation working tree:

- repository workflow tests: 17/17;
- workflow state validation: passed;
- Vitest: 22 files / 163 tests;
- Rust SQLite library tests: 27/27;
- Phase 3C Slice 0 schema-contract integration tests: 8/8;
- TypeScript typecheck, production frontend build, and Rust check: passed;
- whitespace, UTF-8, secret-file, and Markdown-link checks: passed;
- Constitution diff: none.

## Product Behavior

No UI, retrieval, preflight, provider, ContextPacket, output evaluator,
retention-policy, or Phase 4 behavior changed. The product continues to persist
the same schema-v4 records; only Rust now owns every mutation statement and
transaction invariant.

## Data And Migration State

- `SCHEMA_VERSION = 4`.
- No DDL or migration change.
- No user database was opened or mutated for testing.
- No backup, restore, cleanup-policy, export-v2, or lifecycle activation.

## Known Risks Or Follow-Up

- Theory Alignment Review requested one bounded factual-documentation
  correction; no implementation file changed in that correction cycle.
- Desktop smoke is not required because no UI contract changed, but the Founder
  may request a disposable schema-v4 smoke during diff review.
- The Rust diff is intentionally substantial because six mutation families and
  their synthetic invariant tests are consolidated into one trusted boundary.

## Git State

- Branch: `codex/phase-3c-slice-1b2-gate`
- HEAD: `1ef3aa0acd57756f7593e3fa792c321f0e164dcc`
- Working changes: unstaged
- Staged files: none
- Commit/push/merge/PR/deployment: not performed

## Engineer Completion Status

`completed_with_follow_up`: implementation is complete; the documentation-only
correction is ready for final canonical verification and repeated Theory
Alignment Review.
