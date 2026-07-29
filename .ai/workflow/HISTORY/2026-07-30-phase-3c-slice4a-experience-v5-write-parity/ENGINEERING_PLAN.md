# Engineering Plan

Status: approved

- Sprint ID: 2026-07-30-phase-3c-slice4a-experience-v5-write-parity
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `cf7633780a1a0a72efcad7558e463ceb094468c4`
- Working-tree digest reviewed:
  `3beab4235b0c7f3a885d684ae5032b89808fcc0c17c34b7285a2b788aea8a1d3`
- Created at: 2026-07-30T01:22:00+09:00
- Updated at: 2026-07-30T01:22:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE4A-001` with Option A. Implement only a private, unregistered,
path/connection-injected Experience mutation boundary against exact-v5
synthetic/disposable fixtures. The accepted transaction contract in
`DECISION_REQUIRED.md` is binding.

The implementation must remain unreachable from Tauri, TypeScript, startup,
UI, app-data, and real user databases. Production schema and user version stay
at 4. Ordinary artifact lifecycle behavior fails closed except for the exact
parent-delete and ADR-0009 cascade consequences expressly accepted. No later
slice, production recovery, Git promotion, deployment, or release is implied.

## Existing Implementation Understanding

- `schema_v5_migration.rs` owns the fixed DDL, exact v4-to-v5 disposable
  migration, canonical manifests, deterministic baseline IDs/provenance,
  receipt/contract verification, current-content checks, and conservative
  migration commit/restart evidence.
- `schema_v5.sql` makes v5 heads/revisions authoritative, separates immutable
  metadata from purgeable content, and guards all v4 projection mutations.
- The migration receipt manifests describe the cutover snapshot. They are
  immutable and cannot be treated as mutable post-write current-state
  manifests.
- `sqlite.rs` implements production schema-v4 typed Experience CRUD/import.
  Update uses v4 `updated_at`, removes current source-scoped artifacts, and
  preserves ADR-0009 cascades. These production commands must not be changed.
- Schema-v5 ordinary artifact invalidation/reconfirmation is not implemented.
  Source correction therefore must fail closed when non-historical artifact
  lifecycle effects would be required.
- The fixed bridge trigger removes a linked v5 Historical Question projection
  when a guarded v4 Historical Question row is deleted.

## Affected Modules

- `src-tauri/src/schema_v5_migration.rs`: declare one nested private Experience
  write module so it can reuse promoted private canonical/migration helpers.
- `src-tauri/src/schema_v5_experience_write.rs`: typed private commands,
  validators, transaction/commit adapter, reconciliation, failure injection,
  and module-local disposable tests.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`:
  factual Slice 4A authorization and implemented-evidence synchronization after
  verification.
- Existing P1 factual corrections and repository workflow artifacts.

No production SQLite, Tauri handler, TypeScript, React, provider,
ContextPacket, schema SQL, Cargo dependency, or application version file is
affected.

## Proposed Design

### Nested private module

Declare `schema_v5_experience_write` as a private child of
`schema_v5_migration`. It may reuse the exact DDL digest, migration fixture
entry point, source-revision ID, provenance, manifests, and integrity helpers
without widening their crate API. It remains compiled but unregistered and has
no product caller.

### Typed operations

Define private typed requests and outcomes for:

- create;
- update with `expected_revision_id`;
- delete with `expected_revision_id`;
- ordered duplicate-skipping import.

Inject one deterministic fixture context containing operation time and a
unique guard token. New source-revision IDs reuse the fixed
domain-separated `(source ID, updated_at, exact content digest)` derivation.
Import uses `legacy_v4_baseline`; create uses `created`; update uses
`corrected`.

### Transaction entry validation

Before and inside `BEGIN IMMEDIATE`, require:

- exact `user_version = 5`;
- exact schema-object digest;
- one immutable migration receipt;
- exact database contract with compatibility enabled and lifecycle/export
  disabled;
- empty guard;
- source/current-content/v4 projection reconciliation;
- foreign keys and integrity.

Any malformed, older, newer, or inconsistent database is refused before
mutation.

### Authority/projection transaction

After revalidating expected state, insert one guard token. Apply v5 authority
in ordered revision/content/head/provenance steps, then authorized dependency
effects, then the v4 projection. Reconcile exact affected rows, delete the exact
guard, require the table empty, and run integrity checks before commit.

Create adds one active head and one `created` revision. Update appends one
`corrected` revision with the exact predecessor and preserves prior content.
Import adds one honest baseline per new ID and skips consistent existing or
earlier-batch IDs. Delete applies the accepted ADR-0009 cascade, removes
parent-scoped artifact records, clears the source head, purges all source
content, and removes the v4 source row. External ordinary dependents and
ordinary update lifecycle needs fail closed.

### Commit and read-only classification

Use a private injectable commit adapter. Ordinary SQL commit success is
`Committed`; a generic error is `OutcomeUnknown`; tests may prove
`DefinitelyNotCommitted`.

Capture versioned pre/post operation manifests in memory. Close the writable
connection and reopen read-only:

- exact post state -> committed;
- exact pre state after definite/unknown failure -> failed unchanged;
- any other durable state -> recovery required.

The verifier checks exact operation outcome, v5/v4 parity, schema/receipt/
contract shape, empty guard, current content, foreign keys, and integrity. It
does not update migration receipt manifests or perform recovery.

### Failure injection

Expose test-only boundaries after every meaningful validation, guard,
authority, cascade, projection, reconciliation, guard removal, and
pre-commit step. Each injected pre-commit failure closes/reopens and proves the
logical pre-manifest and empty guard.

## Alternatives Considered

- Add v5 behavior to production `sqlite.rs`: rejected because production
  activation and real user databases are unauthorized.
- Copy canonical migration helpers into a sibling module: rejected because
  duplicated ID/manifest contracts could drift.
- Delete ordinary artifacts during correction to mimic current v4: rejected;
  it violates ADR-0011 Decision 10B and the Slice 5 boundary.
- Persist a per-write recovery journal: rejected as new production restart
  architecture outside Slice 4A.
- Treat migration receipt manifests as live state: rejected because any valid
  post-cutover mutation changes current state while the receipt is immutable.

## Data Lifecycle Impact

Synthetic/disposable fixtures only. Correction retains superseded source
content. Parent delete purges all source content and parent-scoped child
records, leaving only content-free source revision metadata/provenance and a
deleted head. No hidden deleted content, real data, backup, cleanup, or
retention job is introduced.

## SQLite Or Migration Impact

No DDL or migration change. Tests create exact v4 fixtures and invoke the
promoted disposable migration to obtain exact-v5 fixtures. Only those fixture
databases receive post-cutover writes. Production `SCHEMA_VERSION` and
`user_version` remain 4.

## Provenance Impact

Create/correction/import write exact user provenance using the promoted
canonical fingerprint. Provenance is bound to the exact source revision.
Migration receipts and prior provenance remain immutable. No provenance record
is invented for an AI action.

## Historical Context Impact

Source update/delete atomically delete Historical Questions where the source is
current or included, preserving their existing packet/consent/transmission
cascade and v5 lifecycle bridge. No dependency is rebound. Non-historical
dependency behavior outside the accepted parent-delete consequence fails
closed.

## Consent Impact

No consent is created or reused. Existing consent/transmission records are only
deleted when ADR-0009 source invalidation requires it.

## Provider Transmission Impact

None. No network path, provider, ContextPacket, packet assembly, or transmission
code is modified or invoked.

## Import And Export Impact

Only private disposable proof of existing v4-format Experience import is in
scope. New imports create one honest `legacy_v4_baseline`; duplicates skip
without overwrite and any non-conflict failure rolls back the batch. Production
import is unchanged. Export v2 remains disabled and untouched.

## Test Strategy

Focused Rust tests will prove:

1. create writes exactly one correct v5 revision/content/provenance/head and
   matching v4 projection;
2. update appends one user-authored correction, exact predecessor, and advances
   the head/projection while retaining immutable prior metadata/content;
3. stale update/delete and non-advancing timestamps write nothing;
4. update refuses ordinary artifact lifecycle requirements;
5. update/delete preserve ADR-0009 Historical Question, consent,
   transmission, dependency, and linked v5 projection cascades;
6. delete purges content/child state, leaves the exact content-free source
   state, and refuses external ordinary dependents;
7. import preserves first occurrence, skips consistent active/deleted/input
   duplicates, creates honest baselines, and rolls back the full batch;
8. failure after each meaningful step rolls back authority and projection;
9. guard is absent after success, rollback, ambiguous/definite commit evidence,
   and reopen;
10. projection drift is recovery-required and never repaired;
11. malformed, v4, inconsistent v5, and newer schemas fail closed;
12. deterministic fixture inputs reproduce IDs/manifests;
13. read-only reopen verifies exact source/projection/ADR outcome;
14. migration receipt manifests remain immutable while current operation
    manifests change;
15. existing schema-v5 migration/contract, R1, P1, and schema-v4 tests remain
    green.

## Repository Verification Strategy

- `cargo fmt --all -- --check`
- focused `cargo test schema_v5_experience_write`
- `cargo test --all-targets --no-fail-fast`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- canonical `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`
- workflow validation at each phase transition

## Manual UI Verification

Not applicable. Slice 4A has no Tauri, renderer, startup, or UI surface and only
executes against disposable fixtures. Founder exact-diff review is required.

## Rollback Or Recovery Strategy

Every pre-commit failure rolls back and is checked after read-only reopen.
Commit ambiguity is classified only from exact pre/post in-memory manifests; no
retry, repair, restore, cleanup, or database selection occurs. A real
process-loss restart has no durable per-write receipt and remains explicitly
unproved and unauthorized.

## Documentation Impact

Preserve the P1 promotion corrections. After implementation evidence exists,
update only architecture/13 factually with the exact Slice 4A authorization,
module/test evidence, continued production schema-v4 boundary, and residual
restart/artifact-lifecycle gaps. Do not create a new design document.

## ADR Impact

No ADR status or decision changes. The implementation exercises ADR-0011
within the exact Founder exception and preserves ADR-0007 and ADR-0009.

## Risk Level

High conceptually because revision/deletion and dual-projection transactions
are data-integrity boundaries. Operational risk is bounded because the module
is private, unregistered, path-injected, and exercised only with disposable
fixtures.

## Escalation Decision

No further escalation is required before implementation. The exact Founder
Option A response is recorded. Any need for runtime registration, real data,
ordinary artifact lifecycle, durable write recovery, new DDL/dependency, or a
change to the accepted delete contract must stop at
`human_decision_required`.
