# Engineering Plan

Status: approved

- Sprint ID: 2026-07-24-phase-3c-post-slice2a-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 9e0ff74f3fcdbe64d060e40d800078d90d4d9dc6
- Working-tree digest reviewed: 03993f812b030fb68c6dc9959e5a2c23618421c7f2a35271f3a5039e2383ff38
- Created at: 2026-07-24T01:30:00+09:00
- Updated at: 2026-07-24T01:30:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE2B1-001` as Option A. Implement only a private,
integration-test-local verified restore and test-injected replacement
simulation over synthetic/disposable schema-v4 fixtures.

Every Product Review condition is binding:

- require a previously verified backup and exact expected database digest,
  governed source-manifest digest, schema 4, foreign-key integrity, database
  integrity, and exact expected records;
- use an exact owned staging path and refuse conflicts;
- revalidate immediately before the simulated replacement boundary;
- inject digest, corruption, version, manifest, record, conflict,
  interruption, permission, and replacement failures;
- ensure every failed restore leaves the disposable live fixture
  byte-identical;
- clean only a staging file created and owned by the current call;
- label replacement as simulation evidence, never production OS atomicity;
- preserve every production, real-data, migration, retention, Phase 4,
  provider, Harness, Git-promotion, PR, and deployment exclusion.

## Existing Implementation Understanding

`src-tauri/tests/schema_v5_backup.rs` contains the promoted Slice 2A private
test harness. It creates disposable schema-v4 fixtures, produces a backup with
SQLite `VACUUM INTO`, closes handles, computes SHA-256 and a governed
source-manifest, verifies schema/FK/integrity, and removes only a newly created
incomplete backup destination. Nothing is registered as a Tauri command or
compiled into production source.

The six governed schema-v4 tables are already enumerated in deterministic
order. That enumeration can be reused to construct an in-memory exact-record
snapshot for restore assertions without adding content to the backup manifest.

## Affected Modules

- `src-tauri/tests/schema_v5_backup.rs`: test-only restore simulation and
  focused regressions.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`:
  factual Slice 2B-1 authority/evidence synchronization after verification.
- `.ai/workflow/*`: repository-native plan, report, verification, theory
  review, sprint report, event chain, and archive/reset.

No production module or dependency file is affected.

## Proposed Design

1. Add a deterministic in-memory `CanonicalV4Record` snapshot derived from the
   existing governed table order and exact nullable text fields.
2. Define restore expectations containing the already verified backup's exact
   SHA-256, governed source-manifest digest, and canonical fixture records.
3. Add a private `restore_verified_backup` test primitive:
   - validate distinct backup/live/staging paths, file presence, and staging
     nonexistence;
   - capture the live fixture bytes and backup bytes before work;
   - validate backup digest, schema 4, FK, integrity, manifest, and exact
     records;
   - create the staging file with create-new semantics and copy the verified
     backup bytes;
   - close and revalidate the staging database;
   - immediately re-read and revalidate the backup digest and manifest;
   - invoke a test-only replacement simulation;
   - validate the restored live fixture;
   - on any error, restore the captured live fixture bytes if the simulation
     changed them and remove only the staging file created by this call.
4. Model bounded failure points with a private enum. Permission and replacement
   failures occur before live mutation; interruption after simulated
   replacement exercises rollback.
5. The success path may overwrite only the disposable live fixture. The code
   and comments explicitly state that this is logical simulation evidence and
   not a production filesystem atomicity implementation or guarantee.

## Alternatives Considered

- Use platform-native `ReplaceFileW`/rename primitives: rejected because it
  would introduce platform-specific production-adjacent behavior and still
  would not prove cross-platform crash durability.
- Register a production Tauri restore command: prohibited by Founder scope.
- Copy directly over the live fixture without staging/revalidation: rejected
  because it cannot prove fail-closed ordering or owned cleanup.
- Compare only the content-free manifest: rejected because the approved scope
  requires exact expected record verification too.

## Data Lifecycle Impact

Synthetic fixture files only. A successful test replaces one disposable live
fixture with the exact verified backup content. Failed calls restore the
captured live bytes and delete only their owned staging file. The verified
backup and unrelated files remain unchanged. No durable user artifact or
retention policy changes.

## SQLite Or Migration Impact

No production SQLite or migration impact. Production `SCHEMA_VERSION` and
`user_version` remain 4. The test accepts only schema-v4 fixtures and never
executes schema-v5 DDL, migration, backfill, startup, or app-data behavior.

## Provenance Impact

No provenance semantics change. Exact Experience, artifact, consent,
transmission, Historical Question, and dependency records are compared as
stored synthetic data. No record is re-authored, normalized, inferred, or
elevated.

## Historical Context Impact

None. Historical tables are opaque governed fixture rows. No retrieval,
selection, packet assembly, provider use, summary, recurrence, contradiction,
or Phase 4 analysis occurs.

## Consent Impact

None. No consent is created, consumed, reused, or interpreted. Synthetic
ADR-0009 consent rows are checked only for exact restoration.

## Provider Transmission Impact

None. No provider or ContextPacket file changes and no network behavior.

## Import And Export Impact

None. This is not product import/export. It neither changes the existing export
format nor activates `life-os-export-v2`.

## Test Strategy

Focused Rust integration coverage will prove:

1. verified backup success restores exact schema-v4 records, digests, manifest,
   FK/integrity state, and leaves backup unchanged;
2. digest mismatch;
3. malformed/corrupt backup;
4. schema v3/v5 mismatch;
5. governed source-manifest mismatch;
6. exact-record mismatch;
7. occupied staging destination without deletion or overwrite;
8. injected interruption after simulated replacement with byte-identical live
   rollback;
9. injected permission and replacement failures before mutation;
10. cleanup removes only the exact owned staging file and preserves backup,
    live fixture, and unrelated sibling.

Existing Slice 2A and Slice 0 tests must continue to pass.

## Repository Verification Strategy

Run the focused Rust integration test during development, then execute:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Confirm workflow validation, Vitest, typecheck, frontend build, all Rust tests,
Rust check, UTF-8, whitespace, secret, link, and Constitution checks.

## Manual UI Verification

Not applicable. The authorized primitive has no production registration,
renderer, UI, startup, or app-data path. Automated disposable-fixture evidence
is the complete authorized observable surface.

## Rollback Or Recovery Strategy

Before promotion, remove the test-only code/document diff. During each test
call, preserve original live and backup bytes, use one exact staging path,
restore live bytes after any simulated post-replacement failure, and delete
only a staging file created by that call. This is not a production recovery
strategy.

## Documentation Impact

Update architecture/13 only after tests pass to record Slice 2B-1 as
Founder-authorized and implemented in the working tree, while retaining that
promotion, production restore, platform atomicity, schema v5, and later slices
remain unauthorized.

## ADR Impact

No new ADR and no ADR status change. This is a bounded implementation of the
existing Founder-approved architecture/13 direction and exact Founder Option A
authority.

## Risk Level

Medium within the test harness because replacement ordering and cleanup are
destructive concepts, but no real or production path is reachable. Residual
risk is that readers overgeneralize simulation results; code and documentation
must state the evidence limit repeatedly.

## Escalation Decision

No further Founder decision is required for this exact plan. Escalate and stop
if implementation requires production source, platform-native replacement,
new dependencies, SQLite sidecars, real paths/data, schema-v5 activation, or
any weakening of the byte-identical failure guarantee.
