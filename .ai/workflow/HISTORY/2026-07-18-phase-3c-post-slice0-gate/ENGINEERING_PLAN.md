# Engineering Plan

Status: approved

- Sprint ID: 2026-07-18-phase-3c-post-slice0-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 7921affde544a5852aa58782a1acb0a4f189520e
- Working-tree digest reviewed: 3f952218bcf689dadc1ea3fbf45c4e0032ce4f585f48c452c8f7cdb396cf765d
- Created at: 2026-07-18T13:25:10.628Z
- Updated at: 2026-07-18T13:25:10.628Z

## Approved Product Boundary

Product Review status is pproved_with_conditions after exact Founder resolution PHASE3C-SLICE1A-001, Option A. Implement Slice 1A only: inspect database presence/version before writable initialization; refuse user_version > 4; expose a typed local startup compatibility state; block production SQLite store construction, cleanup, timeline reads, and mutations when incompatible; preserve fresh/v2/v3/v4 behavior at schema v4; synchronize application declarations to 0.2.0; use synthetic/disposable fixtures; update factual documentation; verify and stop at Founder diff review.

Every exclusion remains binding: no Slice 1B typed mutation rewrite, schema-v5 production DDL, user_version = 5, live Founder database access, backup/restore/cleanup, later slices, Phase 4, provider/ContextPacket change, Harness expansion, staging, commit, push, merge, PR, or deployment.

## Existing Implementation Understanding

src-tauri/src/sqlite.rs currently opens writable connections with create_if_missing(true). migrate_connection enables foreign keys and executes CREATE TABLE IF NOT EXISTS experience_entries before reading PRAGMA user_version; it accepts every version at least 4. initialize_sqlite_database creates the app-data directory and immediately opens/migrates. Generic and Historical Rust transactions open the same path without a maximum-version check.

src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts invokes initialization while the store is constructed. App.tsx constructs the store during render and its first refresh immediately performs audit cleanup and timeline reads. There is no typed startup state. Current v2/v3-to-v4 migration tests and eight Slice 0 contract tests protect schema-v4 compatibility and the fixed test-only v5 contract.

## Affected Modules

- src-tauri/src/sqlite.rs: read-only inspection, maximum-version guard, initialization ordering, disposable tests.
- src-tauri/src/lib.rs: register the inspection command.
- src/shared/storage/createLocalEvidenceStore.ts: deferred startup-gated runtime and typed state.
- src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts: accept an already initialized database promise without double initialization.
- src/shared/storage/index.ts: export startup runtime/types.
- src/app/App.tsx: wait for the typed startup state and render a local checking/refusal gate.
- src/app/i18n.ts and src/app/i18n.test.ts: EN/zh-TW/JA parity for checking and refusal meaning.
- package.json, src-tauri/Cargo.toml, src-tauri/Cargo.lock, src-tauri/tauri.conf.json: synchronized 0.2.0 application declarations.
- docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md: factual Slice 1A evidence only after implementation.

## Proposed Design

1. Add a Rust DatabaseStartupState returned by inspect_sqlite_database.
2. For a missing database, report initialization-required without creating a directory or file. For an existing file, open SQLite read-only and read PRAGMA user_version. Return locked/newer_schema for any version above 4. Return errors without retry or repair.
3. Revalidate the maximum version inside migrate_connection before any DDL. Keep existing fresh/v2/v3-to-v4 logic and SCHEMA_VERSION = 4 unchanged.
4. Reuse the same maximum-version guard before Rust generic/Historical transactions so a direct command cannot write a newer database.
5. Add a frontend storage runtime containing a deferred LocalEvidenceStore plus startup promise. It inspects first; only a ready result initializes and constructs the SQLite store. Browser development immediately receives an in-memory ready state.
6. App remains on an explicit checking screen until initialization succeeds. A newer or uninspectable database receives a calm local blocked screen; the normal store and timeline never become active.
7. Synchronize the three application manifests and Cargo lock package entry to 0.2.0. This version does not imply schema-v5 support; the supported maximum remains 4.

## Alternatives Considered

- **Only move the Rust version query before DDL:** rejected because the renderer would still lack an explicit state and could attempt cleanup/read paths.
- **Construct the SQLite store and surface initialization rejection as the existing generic storage error:** rejected because store construction and startup work would already have begun and the user-facing state would be ambiguous.
- **Make App hold a nullable store and add null checks to every callback:** rejected as a broad UI diff. A small deferred store proxy keeps existing mutation semantics while preventing the real SQLite store from existing before readiness.
- **Implement typed Rust mutation commands now:** rejected as unauthorized Slice 1B.

## Data Lifecycle Impact

No new artifact, revision, review, deletion, retention, or cleanup lifecycle. Existing fresh/v2/v3-to-v4 initialization remains unchanged after the new safety gate. A blocked database is left untouched.

## SQLite Or Migration Impact

Production remains SCHEMA_VERSION = 4 and writes only PRAGMA user_version = 4 on existing authorized compatibility paths. No v5 DDL, backfill, receipt, guard trigger, backup, restore, or down migration is added. Read-only inspection must not create or mutate the database. All tests use 	empfile paths or synthetic fixtures; the live application database is not opened.

## Provenance Impact

No provenance schema or meaning changes. Blocking unsupported versions protects existing provenance from writes by this binary. ADR-0009 tables and transaction checks remain unchanged.

## Historical Context Impact

None. Historical retrieval, packet assembly, consent, transmission, successful packet snapshots, and dependencies are unchanged. The existing Historical Rust transaction receives only the shared maximum-version precondition.

## Consent Impact

None. Local compatibility inspection is not migration consent or historical provider-use consent.

## Provider Transmission Impact

None. No provider code, prompt, packet, model, request, or fallback changes.

## Import And Export Impact

None. Experience import/export formats and behavior remain unchanged. Import is unavailable while the entire database startup is blocked because the product store is not active.

## Test Strategy

Rust disposable tests will prove: missing-file inspection creates nothing; v4 inspection preserves file bytes and reports ready; newer-version inspection and migrate_connection refuse before DDL; malformed database inspection fails without altering bytes; direct generic transaction refuses a newer database; existing v2/v3-to-v4 and injected rollback tests remain passing. Frontend tests will assert equivalent EN/zh-TW/JA checking, newer-schema, and inspection-failure meanings. Existing 150 Vitest, 8 SQLite, and 8 Slice 0 tests remain regression baselines.

## Repository Verification Strategy

Run focused Vitest and Cargo tests during implementation, then the canonical command: powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1. Inspect exact diff, production SCHEMA_VERSION, absence of user_version = 5, Constitution diff, staged files, and workflow state.

## Manual UI Verification

Owner: Founder after this sprint reaches Founder diff review. Use only a disposable app-data/database copy to check normal v4 startup, newer-version refusal with no writes, restart consistency, fresh/v3-to-v4 compatibility without v5 activation, and EN/zh-TW/JA refusal meaning. Do not point the test at the Founder live database.

## Rollback Or Recovery Strategy

Before schema-v5 activation, this source change is non-destructively reversible: revert the startup runtime, inspection command, version declarations, and copy. No database rollback is required because Slice 1A creates no v5 state. A blocked database remains unchanged and must be opened with a compatible application; there is no repair or downgrade action.

## Documentation Impact

Update architecture/13 factually to distinguish implemented-but-unpromoted Slice 1A from deferred Slice 1B and production migration. Do not create a new design document or alter Book Zero.

## ADR Impact

No new ADR. This is a separately Founder-authorized implementation slice under Accepted ADR-0011 and Founder-approved architecture/13. ADR-0009 behavior remains unchanged.

## Risk Level

medium. The change is bounded and reversible but touches startup ordering, fresh/legacy initialization, and the condition that protects all local data access. Focused failure and compatibility tests are mandatory.

## Escalation Decision

No new escalation. The exact Founder response resolves the only authority question. Any need for typed mutation commands, schema-v5 objects, live database testing, backup/restore, or broader UI behavior must stop and return to Founder review.