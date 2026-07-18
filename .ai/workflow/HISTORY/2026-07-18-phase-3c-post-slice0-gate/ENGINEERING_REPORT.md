# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-07-18-phase-3c-post-slice0-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 7921affde544a5852aa58782a1acb0a4f189520e
- Working-tree digest implemented: 723d569cde853fee71de028f3909e3b653b15763e097b5cae8e3278836a0a7f7
- Created at: 2026/07/18
- Updated at: 2026/07/18

## Implementation Summary

Implemented only the Founder-authorized Phase 3C Slice 1A startup-safety foundation. Rust now inspects database presence and `PRAGMA user_version` through a read-only connection before writable initialization, refuses versions above the supported maximum 4, and revalidates the maximum before migration DDL or Rust-owned transaction execution. The renderer exposes an explicit local startup state and does not construct a SQLite store, run audit cleanup, read, or write when inspection blocks. Application declarations are synchronized to `0.2.0` without changing schema v4.

## Existing System Areas Inspected

- `src-tauri/src/sqlite.rs` initialization, v2/v3/v4 migration, transaction commands, and existing SQLite tests.
- `src-tauri/src/lib.rs` Tauri command registration.
- `src/shared/storage/` store construction, SQLite plugin loading, cleanup, reads, and writes.
- `src/app/App.tsx` startup effects and timeline refresh.
- `src/app/i18n.ts` and parity tests.
- `package.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and `src-tauri/tauri.conf.json` version declarations.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md` Slice authority and factual evidence.

## Files Added

- `src/shared/storage/createLocalEvidenceStore.test.ts`

## Files Modified

- `package.json`
- `src-tauri/Cargo.lock`
- `src-tauri/Cargo.toml`
- `src-tauri/src/lib.rs`
- `src-tauri/src/sqlite.rs`
- `src-tauri/tauri.conf.json`
- `src/app/App.tsx`
- `src/app/i18n.test.ts`
- `src/app/i18n.ts`
- `src/shared/storage/createLocalEvidenceStore.ts`
- `src/shared/storage/index.ts`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

The workflow and factual Harness closeout artifacts are recorded separately by the repository workflow.

## Files Deleted

none

## Behavior Changed

- Missing databases are classified without file creation, then follow existing fresh-v4 initialization.
- Existing versions 0 through 4 may initialize through existing migration behavior; v4 retains the existing `experience_entries` create-if-missing behavior.
- Existing versions above 4 fail closed before production migration DDL.
- Malformed/unreadable databases show an explicit blocked local state rather than exposing the normal timeline.
- A blocked state does not construct the SQLite store or expose cleanup, read, or write operations.
- English, Traditional Chinese, and Japanese disclose the same local refusal authority.

## Data Model Impact

None. Production `SCHEMA_VERSION` remains 4, no schema-v5 table or trigger is installed, and no domain model changed.

## Migration Impact

No production schema-v5 migration exists. Existing fresh/v2/v3 compatibility to v4 is retained. All new migration/refusal tests use synthetic or disposable files; no live user database was opened or mutated.

## Provenance Impact

None. Artifact and historical actual-use provenance are unchanged.

## Historical Context Impact

None. Historical selection, packet, consent, transport, persistence, and generated-artifact behavior are unchanged.

## Consent Impact

None.

## Provider Transmission Impact

None. No provider or ContextPacket file changed.

## Tests Added

- Five read-only/refusal/immutability startup tests and one v4 behavior-preservation test in the Rust SQLite unit suite.
- One renderer runtime regression proving newer-schema blocking prevents store construction plus cleanup/read/write access.
- One EN/zh-TW/JA refusal-copy parity regression.

## Tests Executed

- `cargo test --manifest-path .\\src-tauri\\Cargo.toml --lib`: passed, 14 tests.
- `cargo test --manifest-path .\\src-tauri\\Cargo.toml --test schema_v5_contract`: passed, 8 tests.
- `pnpm exec vitest run src/shared/storage/createLocalEvidenceStore.test.ts src/app/i18n.test.ts`: passed, 2 files / 7 tests.
- `pnpm exec tsc --noEmit`: passed.
- `cargo fmt --manifest-path .\\src-tauri\\Cargo.toml` and `git diff --check`: passed.

## Verification Results

Focused implementation checks pass. Canonical `scripts/verify.ps1` is intentionally recorded in the repository workflow validation phase after this implementation report; it is not pre-claimed here.

## Manual Verification Required

Founder diff review should inspect the exact compatibility copy and confirm the calm blocked UI in EN/zh-TW/JA against disposable newer-version and malformed fixtures. No live user database manual test has been performed or authorized. Promotion remains a separate Founder decision.

## Documentation Updates

`architecture/13` is synchronized to Slice 0 promotion facts, the exact Slice 1A authorization, current unpromoted evidence, application version `0.2.0`, schema-v4 preservation, and the still-unauthorized Slice 1B/later boundary.

## ADR Impact

No ADR status or decision changed. ADR-0011 remains Accepted; Slice 1A implements a bounded startup-safety prerequisite without activating schema v5.

## Deviations From Plan

One additional v4 create-if-missing regression was added during implementation review to prove that moving the version guard before DDL did not remove existing v4 startup behavior. This remains inside the approved synthetic/disposable test scope.

## Known Limitations

- Slice 1B typed Rust mutation parity remains unimplemented, so the existing renderer generic-SQL design remains for compatible v4 databases.
- Slice 1A prevents store construction from the inspected incompatible startup state and rechecks before Rust initialization. It does not claim protection against an external process replacing or mutating the database file after successful startup; typed command ownership and v5 database guards remain later gates.
- The UI does not offer repair, downgrade, restore, or migration actions.

## Remaining Risks

- Founder manual UI verification is outstanding.
- Live OS/file-lock behavior and adversarial post-start file replacement are not exercised by disposable unit tests.
- Application version `0.2.0` could be misread as schema-v5 readiness; documentation and copy explicitly state schema remains v4.

## Git State

Branch `codex/phase-3c-post-slice0-gate`, HEAD `7921affde544a5852aa58782a1acb0a4f189520e`. Working tree is unstaged and contains only this governed sprint's workflow/factual closeout plus Slice 1A files. No staged files, commit, push, merge, PR, or deployment occurred.

## Engineer Completion Status

completed_with_follow_up
