# Engineering Plan

Status: approved

- Sprint ID: 2026-07-19-phase-3c-post-slice1b2-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 3c84d4660d425a65f1173f3f8501a76ba2b3262a
- Working-tree digest reviewed: 6652abf7cfe46328d71423884191651decb4e4132ae580b6a336e231c83589ab
- Created at: 2026-07-19T12:58:00.0000000Z
- Updated at: 2026-07-19T12:58:00.0000000Z

## Approved Product Boundary

Product Review is `approved_with_conditions` after the exact Founder resolution
of `PHASE3C-SLICE2A-001` as Option A. Implement only a Rust integration-test
harness that creates and verifies a backup of a synthetic disposable schema-v4
fixture. It must have no production command, module registration, startup/UI,
app-data, or real database connection. Restore and all later slices remain
blocked.

## Existing Implementation Understanding

- Production SQLite is owned by `src-tauri/src/sqlite.rs`, remains schema v4,
  and exposes typed mutation commands plus startup compatibility refusal.
- `src-tauri/tests/schema_v5_contract.rs` and fixed v2/v3/v4 fixtures provide the
  existing test-only schema foundation.
- `cargo test --manifest-path src-tauri/Cargo.toml` runs every integration test
  locally and in GitHub Actions.
- `sha2`, `tempfile`, `tokio`, `sqlx`, `serde`, and `serde_json` are already
  available for the integration-test target; no dependency or production code
  change is required.
- No current backup, manifest, restore, retention, or v5 startup path exists.

## Affected Modules

- Add `src-tauri/tests/schema_v5_backup.rs` only for implementation/test logic.
- Synchronize factual Slice 1B-2 promotion and Slice 2A working-tree evidence in
  `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Current `.ai/workflow/` artifacts record the three-role sprint.

No `src-tauri/src/`, renderer, UI, provider, ContextPacket, migration fixture,
Cargo dependency, verification script, or CI workflow change is planned.

## Proposed Design

Create one integration-test-local backup harness:

1. Build a disposable file-backed v4 database from the promoted fixed v4 SQL
   fixture and close it before backup.
2. Reopen the injected source path with SQLite foreign keys enabled; refuse
   missing/malformed databases and any `user_version` other than 4.
3. Require the injected destination path not to exist.
4. Compute the architecture/13 v4 source-manifest digest by hashing
   domain-separated table/count/length-prefixed row fields in fixed primary-key
   order for all six v4 data tables.
5. Execute fixed `VACUUM INTO ?` against the injected destination.
6. Close the source connection, open the backup without create-if-missing, and
   require schema version 4, empty `foreign_key_check`, `integrity_check = ok`,
   and an identical source-manifest digest.
7. Close the backup, compute exact database-file SHA-256, and construct an
   in-memory content-free manifest with the architecture/13 fields.
8. Return a verified result only after every check passes. On every failure,
   drop open handles and remove a newly created incomplete destination
   best-effort; never overwrite a pre-existing destination.
9. Keep all helper functions private to the integration-test crate. Do not add
   a Tauri command, `mod` registration, production feature, or app-data path.

Timestamps, backup ID, application version, relative filename, and expiry are
injected deterministic values so tests do not depend on locale or wall clock.

## Alternatives Considered

- Production Rust module without command registration: rejected for Slice 2A
  because dormant production code expands the runtime surface without user
  value and would require a production SHA-256 dependency.
- Full fixture restore harness: deferred to separately authorized Slice 2B.
- Raw filesystem copy: rejected by Founder-approved architecture/13 Decision
  9A in favor of SQLite `VACUUM INTO`.

## Data Lifecycle Impact

Only synthetic fixture data is duplicated under an OS temporary directory.
Successful files live only until test teardown. Incomplete destinations are
removed best-effort. No Life OS user data or app-data directory is opened.

## SQLite Or Migration Impact

No production SQLite or migration impact. The source fixture remains
`user_version = 4`; the backup must also be 4. Existing test-only schema-v5 DDL
is not executed, and no `user_version = 5` statement is added.

## Provenance Impact

No production provenance change. Synthetic ADR-0009 rows participate only in
the source-manifest equality proof.

## Historical Context Impact

No product behavior change. The fixture manifest covers consent,
transmission, Historical Question, packet snapshot, and dependency rows as raw
preservation evidence without interpreting them.

## Consent Impact

none; no consent event is created, consumed, transmitted, or changed.

## Provider Transmission Impact

none; no provider code or network request is involved.

## Import And Export Impact

none; backup fixtures are not export-v2 and no import path is changed.

## Test Strategy

Focused integration tests will cover:

1. verified v4 backup and exact content-free manifest;
2. source bytes/logical manifest unchanged;
3. destination conflict refusal without overwrite;
4. missing/malformed source refusal;
5. schema-version mismatch refusal for v3 and v5 fixtures;
6. injected failure after `VACUUM INTO` with destination cleanup;
7. corrupted backup/digest/source-manifest mismatch refusal and cleanup;
8. foreign-key violation refusal;
9. malformed/integrity failure refusal;
10. manifest JSON excludes fixture content and sensitive payload fields.

Run the new integration test directly before canonical verification.

## Repository Verification Strategy

Run:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Confirm workflow validation, Vitest, all Rust unit/integration tests, typecheck,
frontend build, Rust check, encoding, whitespace, secret, link, and Constitution
checks.

## Manual UI Verification

Not applicable. No executable product path or UI is changed. Founder review is
diff/evidence review only; it must not be reported as a desktop backup test.

## Rollback Or Recovery Strategy

Before promotion, remove the new integration-test file and factual doc diff to
return exactly to the promoted Slice 1B-2 baseline. Tests never replace a live
file. Failure cleanup targets only the exact injected temporary destination and
never recursively deletes a directory.

## Documentation Impact

Architecture/13 only: preserve promotion facts, record the exact fixture-only
Slice 2A boundary/evidence, and keep Slice 2B, production backup/restore,
cleanup, and migration unauthorized.

## ADR Impact

No new ADR and no ADR status change. The plan implements evidence under existing
ADR-0011 and architecture/13 authority without changing policy.

## Risk Level

medium. The data is synthetic and the code is test-only, but backup semantics,
content-free manifests, cleanup targeting, and preservation claims are
load-bearing prerequisites for any future migration.

## Escalation Decision

No further Founder decision is required for this exact plan. Stop immediately
if implementation would require a production module/dependency, Tauri command,
app-data path, real database, restore/file replacement, retention behavior,
schema-v5 activation, or a change to the fixed source-manifest policy.
