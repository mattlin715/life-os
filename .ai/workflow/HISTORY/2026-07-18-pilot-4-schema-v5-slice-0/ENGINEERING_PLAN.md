# Engineering Plan

Status: approved

- Sprint ID: 2026-07-18-pilot-4-schema-v5-slice-0
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: b781071fbf726cde69e93cb9cd98c74abdff0ba3
- Working-tree digest reviewed: 527acca8dc06433f4206f5435337fed0afd48ded23fd3c65956081c3ad304849
- Created at: 2026-07-18T10:26:47.7445382Z
- Updated at: 2026-07-18T10:26:47.7445382Z

## Approved Product Boundary

Product Review is `approved_with_conditions`. This is a recovery plan for
implementation that already existed before intake. The plan does not claim to
have preceded that work. It authorizes only: preserve/audit the existing Slice
0 diff, align remote CI with local full Rust tests, add demonstrably missing
test-only invariant coverage, make factual architecture/13 evidence current,
complete the workflow, and stop before promotion.

All eight Product Review conditions are binding: truthful chronology,
preservation, complete Rust CI, bounded missing-regression audit, factual state
separation, no prohibited files, no `WORKFLOW_EVALUATION.md` change, and stop
after archive/reset with no staged files.

## Existing Implementation Understanding

- `src-tauri/tests/fixtures/schema_v5/schema_v5.sql` is a fixed copy of the
  architecture/13 candidate DDL, not production migration SQL.
- `contract.json` freezes file/object digests, all object names/body hashes,
  canonicalization, deterministic IDs, manifests, and error codes.
- Synthetic v2/v3/v4 fixtures execute only in memory.
- `schema_v5_contract.rs` currently has six integration tests for contract
  alignment, fixture readiness, content invariants, immutability/guards,
  eighteen v4 projection guards, and the ADR-0009 cascade.
- `sha2` is a dev dependency already present transitively in Cargo.lock; the
  package dependency list now exposes it only to tests.
- `scripts/verify.ps1` was already changed before intake from a filtered Rust
  test invocation to the complete Cargo test suite.
- `.github/workflows/check.yml` still uses the old `sqlite::tests` filter and is
  the confirmed parity defect.
- Existing tests reject purge of current content, but do not yet prove the
  approved positive path that guarded purge of non-current content preserves
  immutable metadata.
- Trigger-body snapshots freeze event/dependency/tombstone immutability, but a
  focused behavioral regression for those record families is missing.

## Affected Modules

- Preserve existing: `scripts/verify.ps1`, `src-tauri/Cargo.toml`,
  `src-tauri/Cargo.lock`, `src-tauri/tests/schema_v5_contract.rs`, and
  `src-tauri/tests/fixtures/schema_v5/*`.
- Correct: `.github/workflows/check.yml`.
- Extend only if confirmed missing: `src-tauri/tests/schema_v5_contract.rs`.
- Synchronize factually: `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Record workflow: `.ai/workflow/*` and terminal Pilot 4 archive.
- Prohibited/read-only: `src-tauri/src/sqlite.rs`, `src/`, startup, provider,
  ContextPacket, UI, package/application versions, and `WORKFLOW_EVALUATION.md`.

## Proposed Design

1. Change the GitHub Rust test step to `cargo test --manifest-path
   src-tauri/Cargo.toml -- --nocapture`, matching `verify.ps1`.
2. Keep the fixed DDL and contract manifest unchanged unless the byte-alignment
   or digest test exposes a real mismatch.
3. Add one bounded regression proving a guard-authorized non-current source and
   artifact content purge deletes only content and preserves revision metadata.
4. Add one bounded regression proving review events, lifecycle events,
   dependencies, and tombstones reject UPDATE and unguarded DELETE and permit
   only an explicitly guarded delete path.
5. Re-run the focused integration test and canonical verifier. Do not add
   typed migration/backfill/runtime logic to satisfy later-slice matrix rows.
6. Update architecture/13 validation evidence from "not yet implemented" to a
   precise working-diff state with test counts and a promotion/production fence.
7. Complete Engineering Report, independent validation, theory review,
   Sprint Report, archive/reset, and Founder diff review.

## Alternatives Considered

- **Leave CI filtered:** rejected because remote CI would report success while
  skipping the new contract suite.
- **Add a second separate CI command:** valid but duplicates test selection and
  could drift again; complete `cargo test` is simpler and matches local truth.
- **Move integration tests into production `sqlite.rs`:** rejected because it
  would mix test contracts into a prohibited production file.
- **Implement dependency-kind/cross-source validation now:** rejected because
  those checks belong to later typed migration/write commands unless the fixed
  DDL explicitly supports them; Slice 0 cannot invent production semantics.
- **Rewrite existing Slice 0 files after intake:** rejected because the founder
  requires preservation and truthful recovery.

## Data Lifecycle Impact

Test-only. Synthetic rows exercise immutable metadata, separately purgeable
content, guarded deletion, and cascade behavior. No real user record is opened,
persisted, changed, backed up, restored, imported, exported, or transmitted.

## SQLite Or Migration Impact

Candidate DDL runs only against in-memory synthetic fixtures. Production
`SCHEMA_VERSION` and `PRAGMA user_version = 4` remain unchanged. There is no
migration entry point, startup hook, database path, backfill, receipt, backup,
restore, or schema activation.

## Provenance Impact

Only fixed synthetic provenance constraints and immutable-role behavior are
tested. No production provenance is created or rewritten.

## Historical Context Impact

Only ADR-0009 synthetic deletion-cascade compatibility is exercised. No
historical retrieval, packet assembly, consent reuse, provider call, or new
Historical Question is produced.

## Consent Impact

None in product behavior. Synthetic consent rows are referential test data only.

## Provider Transmission Impact

None. Provider and ContextPacket files are prohibited from the diff.

## Import And Export Impact

None. Slice 0 neither activates nor changes import or export.

## Test Strategy

- Existing six `schema_v5_contract` tests remain the baseline.
- Add non-current purge/metadata survival coverage.
- Add behavioral immutability/guard coverage for event, dependency, and
  tombstone families.
- Require `foreign_key_check` empty and `integrity_check = ok` after positive
  paths.
- Confirm v2/v3 direct application fails, v4 application succeeds, DDL remains
  architecture-aligned, and object/file digests remain unchanged.
- Run all Rust tests through both focused Cargo and canonical verification.

## Repository Verification Strategy

Run:

```powershell
. .\scripts\use-local-dev-env.ps1
cargo test --manifest-path .\src-tauri\Cargo.toml --test schema_v5_contract -- --nocapture
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

Then inspect exact Git status, staged state, prohibited-path diff,
`SCHEMA_VERSION`, workflow validation, event sequence/hash, and CI command.

## Manual UI Verification

Not applicable. No UI or runtime application behavior changes. Founder diff
review remains required; no manual UI pass will be claimed.

## Rollback Or Recovery Strategy

No production data exists in scope. Before promotion, rollback is omission of
the unstaged corrective files; however this sprint will not discard them.
Workflow recovery uses repository state and event-chain validation only. No
autonomous repair, replay, reset, or file restoration is authorized.

## Documentation Impact

Update only architecture/13 factual validation evidence. Do not create a new
design document or update `WORKFLOW_EVALUATION.md`. Workflow artifacts and the
Pilot 4 archive record operational evidence without declaring Stage 1 reliable.

## ADR Impact

No ADR change. ADR-0011 remains Accepted with production implementation
withheld; ADR-0009 behavior is regression-tested but not changed; ADR-0008
workflow authority is applied without changing its decision.

## Risk Level

Medium. The code changes are test-only, but they freeze a future migration
contract. A false positive could mislead later production authorization, so CI
parity, exact digests, behavioral invariants, and explicit limitations are
required.

## Escalation Decision

No founder decision is required inside this plan. Any DDL change, production
path, dependency-kind policy, later slice, or promotion request must stop and
return to the founder.
