# Engineering Plan

Status: approved

- Sprint ID: 2026-07-19-phase-3c-slice-1b2-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 1ef3aa0acd57756f7593e3fa792c321f0e164dcc
- Working-tree digest reviewed: cd038cd913ae41d697be7a9fd3cfee2556fd678ff437edbbb1cf5ac6e450270e
- Created at: 2026-07-18T20:40:00.000Z
- Updated at: 2026-07-18T20:40:00.000Z

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE1B2-001` as Option A. Implement only typed Rust commands and typed
TypeScript adapters for the six remaining schema-v4 mutation paths. Preserve
all current product, consent, retention, deletion, provenance, provider, and
startup behavior. Stop at Founder diff review; Git promotion is not authorized.

## Existing Implementation Understanding

- `sqliteLocalEvidenceStore.ts` still creates SQL for artifact replacement,
  consent, transmission, Historical Question create/delete, and audit cleanup.
- `execute_sqlite_transaction` executes arbitrary renderer statements and
  derives an Experience ID from the first bind value.
- `execute_sqlite_historical_transaction` performs strong persistence-time
  revalidation but still executes renderer-selected statements.
- Schema-v4 triggers delete Historical Question transmission/consent provenance
  with the artifact and invalidate questions when a source is deleted.
- `validateArtifactBundle` remains the public product validation boundary.
- All writable Rust paths already call `ensure_supported_schema`; production
  `SCHEMA_VERSION` is 4.

## Affected Modules

- `src-tauri/src/sqlite.rs`
- `src-tauri/src/lib.rs`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.test.ts`
- Focused existing storage and governed-packet tests if required
- `docs/architecture/01_Local_Evidence_Store.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- Current workflow artifacts and final sprint archive

## Proposed Design

Implement five capacity-resilient micro-blocks in one authorized sprint:

1. **Artifact bundle:** add a named Rust command that accepts entry ID, validated
   typed bundle records, and optional expected Experience revision. Rust owns
   `BEGIN IMMEDIATE`, revision/missing-source checks, dependent Historical
   Question deletion, whole-bundle replacement, and rollback. Renderer sends no
   SQL.
2. **Consent and transmission:** add typed event DTOs. Rust serializes the
   consent payload, prevents an ID conflict from rebinding packet/timing scope,
   and persists transmission plus consent consumption atomically with exact
   outcome rules.
3. **Historical Question create/delete:** accept the typed artifact and packet,
   derive current/source revisions, eligible artifact expectations, destination,
   provenance, and dependency rows from the packet inside Rust; reuse the
   current fail-closed eligibility/provenance checks; Rust owns INSERT/DELETE.
4. **Audit cleanup and bypass removal:** add a typed timestamp-only cleanup
   command preserving current expiry/reference rules. Remove `SqlStatement`,
   generic transaction execution, and both generic Tauri registrations after no
   callers remain.
5. **Integrated regression/documentation:** prove all typed mutations refuse a
   newer schema, roll back on failure, preserve schema-v4 cascades and audit
   retention, and leave reads/provider behavior unchanged.

Outer command DTOs may contain JSON domain payloads where schema v4 stores JSON,
but no DTO may contain SQL text or arbitrary statement arrays. Rust validates
relational metadata against payload/packet identifiers before mutation.

## Alternatives Considered

- Artifact-only Slice 1B-2A: rejected by the Founder in favor of Option A.
- Keep generic commands registered but unused: rejected because it leaves a
  renderer mutation bypass.
- Reimplement schema-v5 lifecycle writes now: outside authority.
- Convert read-only renderer queries: unnecessary for mutation parity and
  outside the minimum slice.

## Data Lifecycle Impact

No new record family or lifecycle state. Existing v4 whole-bundle replacement,
source invalidation, Historical Question deletion, provenance cascade, consent
consumption, and expired unreferenced audit cleanup remain unchanged.

## SQLite Or Migration Impact

No DDL, migration, startup change, or `user_version` change. Every command uses
the existing schema-v4 tables, `BEGIN IMMEDIATE` where multi-step atomicity is
required, `ensure_supported_schema`, and disposable databases in tests.

## Provenance Impact

ADR-0009 actual-use provenance becomes safer because Rust derives and persists
the artifact/dependency rows after in-transaction checks. No provenance field,
retention rule, or deletion policy changes.

## Historical Context Impact

Persistence implementation only. Retrieval, selection, disclosure, packet
digest, provider transport, output evaluator, and Phase 4 fences do not change.

## Consent Impact

No policy change. Consent remains exact, one-generation, one-purpose, and tied
to packet/provider/model. Transmission and the existing consumed-state update
become one explicitly typed Rust transaction.

## Provider Transmission Impact

None. No provider or ContextPacket code will change.

## Import And Export Impact

None. Experience import remains the already promoted Slice 1B-1 typed command;
export and import-v2 remain untouched.

## Test Strategy

- Extend Rust temporary-file tests for exact artifact replacement, missing and
  stale source, dependency cascade, injected rollback, and newer-schema refusal.
- Test consent idempotency and conflicting ID fail-closed behavior.
- Test transmission/consent atomicity for every outcome and injected failure.
- Preserve and adapt all Historical Question revision, artifact eligibility,
  Reflection-to-confirmed-Evidence, consent, transmission, digest,
  provider/model, dependency, deletion, and stale-response regressions.
- Test cleanup keeps referenced records and removes only expired unreferenced
  records.
- Assert `foreign_key_check` is empty and `integrity_check` is `ok`.
- Extend TypeScript adapter tests so every mutation invokes the expected named
  command with typed data and no `query`, `values`, or `statements` fields.
- Repository search must find no renderer mutation SQL and no registered generic
  statement-array command.

## Repository Verification Strategy

Run focused Vitest and Rust tests after each micro-block. At the final digest run
`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`,
record the result in the workflow, and re-run after any Theory Review correction.

## Manual UI Verification

No UI or provider behavior changes, so desktop smoke is not required for
engineering completion. The Founder may request a disposable schema-v4 smoke at
diff review; no real user database may be used.

## Rollback Or Recovery Strategy

Changes are application-code-only against schema v4. On interruption, preserve
the working tree and workflow event chain and resume from the last completed
micro-block. A code revert before promotion restores the prior command boundary;
no destructive data rollback is needed or authorized.

## Documentation Impact

Record Slice 1B-1 as promoted, then describe Slice 1B-2 as implemented only after
verified completion. Keep schema-v5 and later slices explicitly unauthorized.

## ADR Impact

No new ADR and no ADR status change. This implements the separately authorized
schema-v4 trust-boundary preparation under ADR-0007, ADR-0009, ADR-0011, and
architecture/13.

## Risk Level

High: the code is schema-preserving but touches exact consent, transmission,
actual-use provenance, and deletion transactions. Risk is contained through
named commands, derived metadata, synthetic failure injection, and bounded
micro-block verification.

## Escalation Decision

No unresolved design decision exists. Proceed to implementation. Stop at
`human_decision_required` if implementation would require schema changes,
retention-policy changes, provider behavior changes, weaker ADR-0009 checks, or
any file outside the authorized boundary that cannot be justified factually.
