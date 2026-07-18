# Engineering Report

Status: completed

- Sprint ID: 2026-07-18-pilot-4-schema-v5-slice-0
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: b781071fbf726cde69e93cb9cd98c74abdff0ba3
- Working-tree digest implemented: 945577a4b9c9afe8dae82a5f790c1222b84898771d266274dfb6c0521883182c
- Created at: 2026-07-18T10:35:39.8915757Z
- Updated at: 2026-07-18T10:35:39.8915757Z

## Implementation Summary

Recovered the Founder-authorized pre-intake Slice 0 work without rewriting its
chronology. Preserved the fixed test-only schema-v5 contract, fixtures, digest
manifest, and six original integration tests. Corrected remote CI to run all
Rust tests, added two missing DDL-invariant regressions, and synchronized
architecture/13 with the current unpromoted evidence while retaining every
production-authorization fence.

## Existing System Areas Inspected

- `AGENTS.md`, `AI_CONTRIBUTOR_GUIDE.md`, and `docs/dev/08_Engineering_Harness.md`
- `docs/00_Constitution.md`, applicable Book Zero definitions, and Product
  Harness boundaries
- `ADR-0007`, `ADR-0008`, `ADR-0009`, and `ADR-0011`
- `docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `.github/workflows/check.yml`, `scripts/verify.ps1`
- `src-tauri/src/sqlite.rs` as a read-only production baseline
- all existing Slice 0 fixtures, manifest, and Rust integration tests
- repository-native workflow contract, roles, state, and prior Pilot archives

## Files Added

These six pre-intake files were preserved and remain untracked pending Founder
review:

- `src-tauri/tests/schema_v5_contract.rs`
- `src-tauri/tests/fixtures/schema_v5/contract.json`
- `src-tauri/tests/fixtures/schema_v5/schema_v5.sql`
- `src-tauri/tests/fixtures/schema_v5/v2.sql`
- `src-tauri/tests/fixtures/schema_v5/v3.sql`
- `src-tauri/tests/fixtures/schema_v5/v4.sql`

Pilot 4 terminal archive files will be added only by the repository-native
archive command after completion.

## Files Modified

- Preserved pre-intake: `scripts/verify.ps1`, `src-tauri/Cargo.toml`, and
  `src-tauri/Cargo.lock`.
- Corrected during Pilot 4: `.github/workflows/check.yml`.
- Extended during Pilot 4: `src-tauri/tests/schema_v5_contract.rs` with two
  bounded regression cases.
- Factual synchronization during Pilot 4:
  `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Workflow evidence: current `.ai/workflow/` mission, review, plan, report,
  state, and event files.

## Files Deleted

None.

## Behavior Changed

- Local and remote deterministic verification now both execute the complete
  Rust test suite, including `schema_v5_contract.rs`.
- Test-only coverage now proves guarded purge of non-current content preserves
  immutable revision metadata.
- Test-only coverage now behaviorally proves review events, lifecycle events,
  dependencies, and tombstones reject updates and unguarded deletes while an
  explicit guard permits the bounded delete path and leaves no guard token.
- No production application or database behavior changed.

## Data Model Impact

None in production. The fixed candidate schema remains test-only and is applied
only to in-memory synthetic fixtures.

## Migration Impact

No migration implementation, user database mutation, `SCHEMA_VERSION` change,
production `user_version` activation, startup path, backup, restore, backfill,
receipt, or cutover exists in the diff.

## Provenance Impact

Only synthetic fixed provenance and immutable lifecycle constraints are tested.
No production provenance is added, changed, or deleted.

## Historical Context Impact

The existing synthetic ADR-0009 deletion-cascade regression remains passing.
No historical content is retrieved, assembled, persisted from a provider call,
or transmitted.

## Consent Impact

None. Synthetic consent rows remain fixture-only referential data.

## Provider Transmission Impact

None. Provider and ContextPacket code have no diff.

## Tests Added

1. `guarded_non_current_purge_preserves_revision_metadata`
2. `audit_dependency_and_tombstone_facts_are_immutable_and_guarded`

The Slice 0 integration suite increases from six to eight tests.

## Tests Executed

- Pre-intake canonical verification:
  `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`
  — passed with 150 Vitest, eight existing Rust SQLite, and six original Slice
  0 tests.
- First focused post-change run — failed one new test because Rust string line
  continuation produced invalid `SETcurrent_revision_id` SQL. No production or
  fixture data was involved.
- Second focused run — failed the same test at the next whitespace boundary
  (`near revision_id`) and confirmed the need to replace the fragile continued
  strings rather than patch individual spaces.
- Final focused run:
  `cargo test --manifest-path .\src-tauri\Cargo.toml --test schema_v5_contract -- --nocapture`
  — eight passed, zero failed.
- Post-change canonical verification is intentionally performed by the
  orchestrator in the Validation phase and is not pre-claimed here.

## Verification Results

- Focused Slice 0 integration suite: passed, 8/8.
- DDL byte alignment and fixed digests: passed inside the focused suite.
- `foreign_key_check` and `integrity_check`: passed in the fixture and both new
  positive-path tests.
- Git diff whitespace check before validation: passed.
- Canonical post-change verification: pending Validation phase.
- Remote GitHub Actions execution: not observed because push is prohibited.

## Manual Verification Required

Founder diff review is required. Manual UI verification is not applicable
because there is no UI or runtime behavior change.

## Documentation Updates

architecture/13 version advances from 0.3 to 0.4 and records the current
unpromoted eight-test Slice 0 evidence. It explicitly states that passing or
promotion cannot authorize production DDL without the separate Founder gate.
No new design document or `WORKFLOW_EVALUATION.md` change was made.

## ADR Impact

No ADR content, status, or decision changed. ADR-0011 and ADR-0009 boundaries
are preserved; ADR-0008 is applied only as workflow governance.

## Deviations From Plan

None. The two initial focused-test failures were bounded implementation defects
inside the planned new regression and were corrected by using explicit raw SQL
whitespace. No approved scope changed.

## Known Limitations

- The suite proves a fixed in-memory DDL contract, not a production migration.
- Remote CI command parity is inspectable but cannot be live-observed without a
  future authorized push.
- Backup, restart, large-database behavior, backfill, reconciliation, typed
  dependency-kind/cross-source validation, startup refusal, and real user data
  remain later-slice work.
- Recovery intake cannot retroactively prove Product Review prevented the
  original implementation; the repository truthfully records that limitation.

## Remaining Risks

- A future SQLite/toolchain change may expose object-body snapshot drift and
  must fail closed for review rather than silently update digests.
- Later production code could still misapply the fixed contract; a separate
  Founder checkpoint and later-slice verification remain mandatory.
- Stage 1 operational reliability remains unevaluated by this product sprint.

## Git State

- Branch: `codex/phase-3c-schema-v5-slice-0-contracts`
- HEAD: `b781071fbf726cde69e93cb9cd98c74abdff0ba3`
- `develop` and `origin/develop`: same HEAD at intake
- Working tree: modified with authorized Slice 0 and Pilot 4 evidence
- Staged files: none
- Branch upstream: none configured
- Commit, push, merge, deployment: not performed

## Engineer Completion Status

completed
