# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-07-19-harness-learning-slice-1b-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 6e9dd6615cb7f99556d258080f6a45140fd55b68
- Working-tree digest implemented: 34eb902d6a4be3fcfbd496655657d0d2d82f40b98ef03af9a5b4c370fda70569
- Created at: 2026-07-18T19:43:14.638Z
- Updated at: 2026-07-18T19:59:00.000Z

## Implementation Summary

Implemented only Founder-authorized Phase 3C Slice 1B-1. Experience create,
expected-revision update, delete, and duplicate-skipping atomic import now cross
the Tauri boundary as typed data and execute SQL inside typed Rust commands.
Schema remains v4. Artifact/historical generic mutation paths remain deferred.

## Existing System Areas Inspected

- `src/shared/storage/types.ts`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `src/shared/storage/inMemoryLocalEvidenceStore.ts`
- Experience mutation/import call sites in `src/app/App.tsx`
- `src-tauri/src/sqlite.rs` initialization, generic/historical transactions,
  schema-v4 foreign keys/triggers, and existing tests
- `src-tauri/src/lib.rs` Tauri command registration
- ADR-0007, ADR-0009, ADR-0011 and architecture/01, /13

## Files Added

- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.test.ts`

## Files Modified

- `src-tauri/src/sqlite.rs`
- `src-tauri/src/lib.rs`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `docs/architecture/01_Local_Evidence_Store.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `docs/architecture/14_AI_Orchestration_Evolution.md`
- `docs/dev/08_Engineering_Harness.md`
- `.ai/workflow/WORKFLOW_EVALUATION.md`
- Current repository workflow artifacts and event/state projections

## Files Deleted

none

## Behavior Changed

- Renderer code no longer authors or directly executes SQL for Experience
  create/update/delete/import.
- Typed Rust writes use one `BEGIN IMMEDIATE` transaction.
- Update conditionally matches `id + expected updated_at` before invalidating any
  dependent data. Stale writes return `stale_generation`; the adapter rejects
  instead of pretending success.
- A committed update must advance `updated_at`. The adapter produces a
  millisecond-later revision when wall-clock time equals the current revision,
  and Rust refuses a non-advancing proposed revision without writing.
- Successful update changes content/revision and removes source artifacts and
  dependent Historical Questions/provenance in one transaction.
- Delete relies on existing schema-v4 foreign keys and deletion triggers.
- Import is one atomic typed transaction, skips duplicate IDs, reports exact
  counts, and fully rolls back on non-conflict failure.

## Data Model Impact

No schema or persisted-shape change. Existing Experience fields and timestamps
are preserved. Only transaction ownership and stale-write behavior changed.

## Migration Impact

None. `SCHEMA_VERSION = 4`; production `user_version` remains 4. No DDL,
migration, backup, restore, or user-database operation was added.

## Provenance Impact

No provenance model change. Rust tests prove update/delete preserve existing
schema-v4 Historical Question, consent, and transmission cascades.

## Historical Context Impact

No retrieval, packet, selection, consent, transmission, or generation behavior
changed. Existing invalidation on Experience update/delete is now directly
covered under the typed transaction.

## Consent Impact

No policy or transport change. Existing ADR-0009 deletion semantics are
preserved.

## Provider Transmission Impact

None. Provider and ContextPacket files are unchanged.

## Tests Added

- Six Vitest adapter tests for typed create/import/update/stale-update/delete,
  same-tick revision advancement, and absence of renderer SQL execution.
- Seven Rust tests for exact create/import records, duplicate counts, full import
  rollback, stale-before-invalidation, successful update cascade, delete
  cascade, non-advancing revision refusal, newer-schema refusal, foreign-key
  integrity, and database integrity.

## Tests Executed

- `pnpm exec vitest run src/shared/storage/sqlite/sqliteLocalEvidenceStore.test.ts`
  - 6 passed after the bounded revision correction.
- `cargo test --manifest-path .\src-tauri\Cargo.toml --lib typed_update`
  - 3 focused update tests passed after the bounded revision correction.
- `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`
  - final post-correction rerun passed completely.

## Verification Results

Passed final post-correction canonical verification:

- 17 workflow tests;
- workflow validation;
- 22 Vitest files / 158 tests;
- TypeScript typecheck;
- production frontend build;
- 21 Rust unit tests;
- 8 schema-v5 test-only contract tests;
- Rust check;
- whitespace, UTF-8, secret-like file, Markdown-link checks;
- no Constitution diff.

An earlier canonical invocation was terminated by the shell's 120-second tool
timeout while Rust tests were listing, producing a broken-pipe exit 101. It was
not represented as a product failure or pass. The same canonical command was
rerun with sufficient time and passed.

## Manual Verification Required

Slice 1B-1 disposable create/edit/delete/import smoke verification remains a
Founder diff-review option and is not marked passed. No real user database was
used. The separate Slice 1A startup manual review completed with its documented
isolation-procedure correction and evidence limit.

## Documentation Updates

- Recorded `HL-001` Founder acceptance and retained no-independent-review/no
  Stage-2/3 caveats.
- Corrected architecture/13 to distinguish current unpromoted Slice 1B-1 from
  unauthorized Slice 1B-2 and production migration.
- Synchronized architecture/01 with the typed Experience mutation boundary.

## ADR Impact

No new ADR and no ADR status change. ADR-0008 remains Accepted and unchanged in
decision. ADR-0007, ADR-0009, and ADR-0011 behavior is preserved.

## Deviations From Plan

No scope deviation. Theory review opened one bounded revision cycle because a
same-tick update could reuse its expected `updated_at` token. Implementation now
advances the adapter timestamp and Rust rejects equality before any write.

## Known Limitations

- Artifact, historical, consent, transmission, and audit mutation paths still
  use the existing generic transaction boundary pending Slice 1B-2.
- Reads still use the Tauri SQL plugin.
- Stale Experience edits use the existing generic storage-error surface; no new
  UI behavior was authorized.

## Remaining Risks

- Runtime smoke verification for typed Experience mutations is not yet run.
- This unpromoted diff still requires Theory Alignment Review and Founder diff
  review.
- Schema-v5 activation, backup/restore, and all later Phase 3C gaps remain open.

## Git State

- Branch: `codex/harness-learning-slice-1b-gate`
- HEAD: `6e9dd6615cb7f99556d258080f6a45140fd55b68`
- Working tree: modified with authorized implementation, documentation, and
  workflow artifacts plus one untracked authorized test file
- Staged files: none
- Commit/push/merge/PR: none

## Engineer Completion Status

completed_with_follow_up
