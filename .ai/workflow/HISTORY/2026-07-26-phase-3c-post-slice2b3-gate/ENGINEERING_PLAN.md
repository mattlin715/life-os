# Engineering Plan

Status: approved

- Sprint ID: 2026-07-26-phase-3c-post-slice2b3-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: e932ead6da3346d3783da22dc1d1295c31cdc979
- Working-tree digest reviewed: 4cbc1d4c4b2fe7fe8df4830ece66105e894369d126f093b7e492d8966f0e9e39
- Created at: 2026-07-26T05:05:00+09:00
- Updated at: 2026-07-26T05:05:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE3A-001` with Option A. Implement only a private, unregistered,
path/connection-injected migration core exercised against synthetic/disposable
exact-v4 fixtures. Production `SCHEMA_VERSION` remains 4. No runtime caller,
real user path, startup/Tauri/renderer/UI integration, fresh-v5 initialization,
combined v2/v3 migration, production backup/restore/replacement, lifecycle
writes, export v2, later slice, Phase 4, Harness expansion, or Git promotion is
within scope.

## Existing Implementation Understanding

- The executable candidate DDL is byte-aligned between architecture/13 and
  `src-tauri/tests/fixtures/schema_v5/schema_v5.sql` and is frozen by
  `schema_v5_contract.rs` plus `contract.json` object/digest manifests.
- Exact-v4 inputs are `experience_entries`, `persisted_artifacts`, the three
  Historical Question audit/artifact tables, and
  `historical_artifact_dependencies`; their ADR-0009 bytes and cascade
  semantics must remain unchanged.
- The DDL contains core objects and a final compatibility-projection guard
  section beginning at the fixed marker for existing v4/current-state tables.
- Production startup rejects versions greater than 4 and cannot call this
  module. Existing backup/filesystem modules remain separate and unactivated.

## Affected Modules

- Relocate unchanged executable DDL to `src-tauri/schema/schema_v5.sql`.
- Update `src-tauri/tests/schema_v5_contract.rs` to consume that shared file.
- Add private `src-tauri/src/schema_v5_migration.rs` and a private module
  declaration in `src-tauri/src/lib.rs`; no Tauri registration or public API.
- Add focused module tests and only the synthetic fixture data needed to cover
  every v4 artifact/status/dependency class.
- Synchronize factual evidence in architecture/13 and sprint workflow reports.

## Proposed Design

1. Open only the injected disposable path or use an injected connection.
2. Refuse malformed, v2, v3, existing-v5, and greater-than-v5 inputs before
   mutation; exact `user_version = 4` is mandatory.
3. Start `BEGIN IMMEDIATE`, enable deferred foreign-key validation, and compute
   the exact governed-v4 source manifest with Rust ordered iteration and typed
   length framing.
4. Split the one fixed DDL at its fixed compatibility-projection marker. Prove
   normalized core plus guard chunks reconstruct the shared DDL; execute the
   core chunk first and the projection guard chunk last.
5. Insert one migration-scoped compatibility token. Backfill Experiences,
   persisted artifacts, Historical Questions, exact dependencies, immutable
   provenance links, honest legacy review/lifecycle facts, and current heads in
   referential order.
6. Preserve exact Experience UTF-8 and artifact/Historical payload bytes.
   Parsing is validation/metadata inspection only and never reserialization of
   revision content.
7. Derive domain-separated deterministic IDs and provenance fingerprints;
   collision with non-identical facts aborts.
8. Reconcile counts, exact content digests, current pointers, dependencies,
   review events, ADR-0009 bytes, foreign keys, integrity, and ordered target
   manifest inside the transaction.
9. Insert disabled `database_contract`, immutable receipt, install projection
   guards last, delete the guard token, revalidate empty guard/current content,
   then set `user_version = 5` as the final SQL mutation and commit.
10. Close and reopen read-only. Verify exact receipt/contract/schema manifest,
    source/target manifests, version, empty guard, foreign keys, integrity, and
    current content. Any post-commit inconsistency returns blocked
    `recovery_required`; there is no automatic action.
11. A deterministic injection enum exposes every meaningful pre-commit
    boundary. All failures explicitly roll back and tests reopen the fixture to
    prove logical exact-v4 facts, schema objects, version, manifests, and
    ADR-0009 records are unchanged. Physical file-byte identity is not claimed.

## Alternatives Considered

- Production startup/app-data activation: rejected; disclosure, quiescence,
  Windows durability, real-data backup/restore, and user recovery are not
  proven or authorized.
- A parallel/generated DDL copy: rejected; it would create a third schema
  authority and permit drift.
- Partial successful v5 state: rejected; the migration is one transaction or
  exact-v4 logical rollback.
- Structured retrieval first: valuable separate gap, but it does not validate
  the approved lifecycle storage cutover.

## Data Lifecycle Impact

Synthetic fixture records gain one baseline revision and immutable lifecycle
facts inside disposable databases only. No current user data is opened or
changed. Lifecycle writes remain disabled in `database_contract`; no purge,
cleanup, rejection reconstruction, or down migration is introduced.

## SQLite Or Migration Impact

A private migration implementation may set `user_version = 5` only in
synthetic/disposable test databases. Production `SCHEMA_VERSION` and startup
paths remain 4. All pre-commit errors roll back DDL and facts; post-commit
inconsistency blocks without restore or repair.

## Provenance Impact

Legacy payload and ADR-0009 bytes are preserved. Exact existing provenance
objects may be fingerprint-deduplicated, while absent provenance remains
`legacy_unknown`. Authorship roles are not merged or invented.

## Historical Context Impact

Historical Question payload, packet snapshot, consent, transmission, packet
digest, provider/model, source revision strings, and exact dependencies remain
unchanged in v4 tables and gain only an exact linked lifecycle projection.
No provider call or Cross-Experience Reflection occurs.

## Consent Impact

None. No consent is created, reinterpreted, consumed, extended, or reused.
Existing consent-event bytes are reconciliation inputs only.

## Provider Transmission Impact

None. No provider or ContextPacket code changes and no network call exists.

## Import And Export Impact

None. Existing import/export behavior is untouched. Export v2 is explicitly
stored as disabled.

## Test Strategy

- Exact v4 to exact valid v5 with every Experience/artifact kind and Historical
  Question chain.
- Confirmed, pending, skipped, and not-applicable review mappings; surviving
  rejected Evidence/Pattern refusal and no invented rejection history.
- Exact payload byte/digest preservation, deterministic IDs across independent
  migrations, exact provenance role links, and dependency mapping.
- Source/target manifest/count reconciliation and deliberate source,
  timestamp, dependency, deterministic-ID, receipt, and target mismatch
  failures.
- Failure injection after DDL, each backfill family, reconciliation, contract,
  receipt, guards, token removal, and version mutation; every pre-commit case
  reopens as logical exact v4.
- Malformed/v2/v3/v5/>v5 refusal; post-commit injected verification failure
  returns `recovery_required` without mutation or autonomous recovery.
- Empty guard, immutable receipt, schema-object contract, foreign-key and
  integrity checks.

## Repository Verification Strategy

Run focused Rust tests during implementation, then the canonical repository
command:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record the passed result only in workflow validation after the final diff is
stable.

## Manual UI Verification

Not applicable. The module is private, unregistered, has no runtime/UI caller,
and is exercised only through disposable automated fixtures. Founder review is
a diff/evidence review, not a desktop smoke test.

## Rollback Or Recovery Strategy

Before commit, issue explicit `ROLLBACK` on every natural or injected failure
and reopen the database to verify exact governed-v4 logical state. After commit,
verification failure returns `recovery_required` and preserves the disposable
v5 fixture. No automatic retry, replay, restore, repair, candidate selection,
down migration, or file replacement exists.

## Documentation Impact

Update architecture/13 only with factual Slice 3A authorization and verified
working-tree evidence. Do not change an ADR status or add a new design
document.

## ADR Impact

No new ADR and no ADR status change. Implementation remains within accepted
ADR-0011 plus the exact Founder resolution recorded in this sprint.

## Risk Level

Medium. Data-integrity sensitivity is high, but exposure is bounded to private,
unregistered code and synthetic disposable fixtures. The primary residual risk
is false confidence or accidental future activation; explicit module privacy,
production-version assertions, and documentation fences mitigate it.

## Escalation Decision

No further decision is needed before implementation. Stop and return to
`human_decision_required` if the fixed DDL cannot be executed in the approved
order without modifying its bytes, if an honest legacy mapping is ambiguous,
or if any implementation would require a production/runtime caller.
