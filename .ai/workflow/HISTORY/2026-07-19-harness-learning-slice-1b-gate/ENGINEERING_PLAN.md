# Engineering Plan

Status: approved

- Sprint ID: 2026-07-19-harness-learning-slice-1b-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 6e9dd6615cb7f99556d258080f6a45140fd55b68
- Working-tree digest reviewed: 1cf2402bb2f71de13212b9f8acc950921562af7893b893774626f7e402a9daff
- Created at: 2026-07-18T19:27:52.832Z
- Updated at: 2026-07-18T19:27:52.832Z

## Approved Product Boundary

Product Review is `approved_with_conditions`. The Founder selected
`PHASE3C-SLICE1B-001` Option B: Slice 1B-1 only. Implement typed Rust commands
and typed TypeScript adapters for Experience create, update, delete, and atomic
import. Preserve schema v4, startup safety, current user behavior, dependent
deletion/provenance guarantees, and stop at Founder diff review.

The Founder did not authorize Slice 1B-2, schema v5, migration, backup, restore,
retention, later slices, Phase 4, provider/ContextPacket behavior, Harness
expansion, Stage 2/3, real user-database testing, or Git promotion.

## Existing Implementation Understanding

- `sqliteLocalEvidenceStore.ts` executes renderer-authored SQL directly for
  create/import and passes generic statement arrays for update/delete.
- Rust `execute_sqlite_transaction` remains necessary for the deferred artifact
  and audit paths and is not removed or broadened here.
- Experience update currently reads outside the transaction, then invalidates
  historical/artifact dependents without supplying its available expected
  revision to Rust.
- Schema-v4 foreign keys delete source-scoped persisted artifacts. A BEFORE
  DELETE trigger removes Historical Questions that depend on a source, and an
  AFTER DELETE trigger removes their consent/transmission provenance.
- The in-memory adapter already expresses the intended local behavior and does
  not author SQL.

## Affected Modules

- `src-tauri/src/sqlite.rs`
- `src-tauri/src/lib.rs`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.test.ts` (new)
- Factual implementation-aligned sections of `docs/architecture/13...`,
  `docs/architecture/01...`, `.ai/workflow/WORKFLOW_EVALUATION.md`, and
  `docs/dev/08_Engineering_Harness.md` only where the final diff requires it.
- Repository workflow reports and archive artifacts.

## Proposed Design

1. Add a typed serializable Experience record DTO in Rust with id, content,
   created timestamp, and updated timestamp.
2. Add typed Tauri commands for create, update, delete, and import. Commands
   resolve the application database path, enforce supported schema, and invoke
   internal transaction functions.
3. Create inserts exactly one supplied typed record in a Rust transaction.
4. Update performs a conditional `UPDATE ... WHERE id = ? AND updated_at = ?`
   before any invalidation. Zero affected rows becomes `not_found` or
   `stale_generation`; no dependent row is changed. A successful update then
   deletes dependent Historical Questions and source-scoped artifacts in the
   same transaction.
5. Delete issues a typed source deletion in one transaction and relies on the
   existing schema-v4 foreign keys/triggers for the authoritative cascade.
6. Import inserts all typed records in one transaction using conflict-ignore on
   Experience ID, counts inserted versus skipped records, and commits only when
   the whole batch succeeds.
7. Internal import accepts test-only failure injection; the production command
   never exposes that parameter.
8. TypeScript continues generating Experience IDs/timestamps to preserve public
   behavior but sends typed data rather than SQL. A stale update rejects with
   `stale_generation`, preserving the open edit and existing error surface.
9. Generic renderer SQL remains only in explicitly deferred non-Experience
   mutation paths.

## Alternatives Considered

- Full Slice 1B was rejected by the Founder because it couples ADR-0009-sensitive
  artifact/historical mutations to the Experience conversion.
- Deferral leaves renderer-owned Experience SQL and the fragile statement-order
  revision convention.
- Generating IDs/timestamps in Rust was not selected because it would change
  current adapter semantics without a product need.
- Adding new schema objects or a mutation queue was rejected as outside the
  authorized schema-v4 parity slice.

## Data Lifecycle Impact

Experience content lifecycle is unchanged. Update invalidates all derived
source artifacts and Historical Questions that use the changed Experience.
Delete removes the Experience and existing dependent graph. Import adds only
missing IDs and remains local. The new boundary makes these effects atomic and
stale-safe; it creates no new retention state.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, or `user_version` change. All commands
refuse schemas newer than supported maximum 4. Only schema-v4 typed DML is
added. Tests use temporary synthetic databases.

## Provenance Impact

No provenance model changes. Existing schema-v4 cascade triggers remain the
authority for Historical Question consent/transmission deletion. Tests must
prove update/delete do not strand or silently preserve stale provenance.

## Historical Context Impact

No retrieval, packet, selection, consent, transmission, or output behavior
changes. Only existing invalidation on Experience revision/deletion is
preserved through the typed transaction.

## Consent Impact

No consent policy or transmission change. Existing ADR-0009 provenance cascade
must remain exact under update/delete tests.

## Provider Transmission Impact

None. Provider contracts, `store: false`, models, preflight, and ContextPacket
are untouched.

## Import And Export Impact

Experience import becomes one typed Rust transaction. Duplicate IDs are counted
as skipped; any non-conflict failure rolls back the entire batch. Import format
and parsing remain unchanged. Export is untouched.

## Test Strategy

Rust temporary-file tests:

- typed create writes the exact record under schema v4;
- typed update commits with the expected revision and removes source artifacts,
  dependent Historical Questions, consent, and transmission provenance;
- stale update leaves Experience and every dependent record unchanged;
- typed delete preserves the schema-v4 full cascade;
- atomic import inserts missing IDs, skips duplicates, and returns exact counts;
- injected import failure rolls back all inserts;
- every typed command path refuses a newer schema without mutation;
- `foreign_key_check` and `integrity_check` remain clean.

Vitest adapter tests:

- create/import/update/delete invoke only the corresponding typed commands;
- no Experience mutation calls `Database.execute` or sends a `statements` SQL
  payload;
- stale update rejects and does not pretend success;
- committed payloads preserve ID, content, and timestamps.

Existing storage, Historical Context, startup, workflow, and schema contract
regressions remain canonical.

## Repository Verification Strategy

Run focused Vitest and Rust tests during implementation, then run:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record verification only for the final HEAD and non-workflow working-tree
digest before Theory Alignment Review.

## Manual UI Verification

Owner: Founder at diff review if requested. The Slice 1A manual startup review
already completed with a recorded procedure correction. Slice 1B-1 should
receive a disposable create/edit/delete/import smoke checklist, but this sprint
does not use the real user database or browser automation.

## Rollback Or Recovery Strategy

Rollback is a source-code revert because schema remains v4 and no migration or
new persisted structure exists. Failed typed transactions roll back. The old
generic command remains for deferred artifact/audit paths, not as a silent
Experience fallback.

## Documentation Impact

Record the Founder-accepted Stage 1 conclusion with its caveats. Mark only Slice
1B-1 implementation facts after tests pass. Keep Slice 1B-2, schema v5, and later
slices unauthorized.

## ADR Impact

No new ADR and no ADR status change. This implements the bounded authority
already selected under architecture/13 and must preserve ADR-0007, ADR-0009,
and ADR-0011.

## Risk Level

Medium. The schema is unchanged and rollback is straightforward, but
Experience update/delete touch artifact and historical-provenance cascades.
Transaction ordering and failure injection therefore require direct regression
coverage.

## Escalation Decision

No new Founder decision is required. Escalate and stop if implementation needs
schema changes, a public UI contract change, artifact/historical typed-command
conversion, new consent behavior, or any authority outside Slice 1B-1.
