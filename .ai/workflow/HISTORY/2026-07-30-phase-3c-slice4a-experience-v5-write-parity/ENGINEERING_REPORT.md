# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-07-30-phase-3c-slice4a-experience-v5-write-parity
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: `cf7633780a1a0a72efcad7558e463ceb094468c4`
- Working-tree digest implemented:
  `eba523c4a395ebe94986ee1cc496b45d2db479872664d36c4bc389eecbb41f9a`
- Created at: 2026-07-30T01:42:00+09:00
- Updated at: 2026-07-30T01:46:00+09:00

## Implementation Summary

Implemented the Founder-authorized Slice 4A boundary as one private nested Rust
module. Exact-v5 disposable fixtures are produced by the promoted migration
core, then a path-injected transaction exercises Experience create,
exact-current-revision correction, parent delete, and atomic
duplicate-skipping v4-format import. Normalized v5 authority and the guarded v4
projection are written and reconciled together. The module is unregistered and
has no production, Tauri, renderer, startup, UI, app-data, or real-user caller.

The implementation also corrected stale post-promotion P1 wording in three
Book One documents. No P1 behavior changed.

## Existing System Areas Inspected

- `src-tauri/src/schema_v5_migration.rs`
- `src-tauri/src/schema_v5.sql`
- `src-tauri/src/sqlite.rs`
- `src-tauri/tests/fixtures/schema_v5/`
- `src-tauri/tests/schema_v5_contract.rs`
- `src-tauri/tests/schema_v5_backup.rs`
- `docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- ADR-0007, ADR-0009, and ADR-0011
- current P1 and Phase 3B documentation

## Files Added

- `src-tauri/src/schema_v5_experience_write.rs`

## Files Modified

- `src-tauri/src/schema_v5_migration.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `docs/11_MVP.md`
- `docs/12_Roadmap.md`
- `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`
- current `.ai/workflow/` sprint artifacts

## Files Deleted

none

## Behavior Changed

No production or user-visible behavior changed. The private disposable-only
boundary now proves:

- create writes one exact user-authored revision, content row, provenance link,
  active head, and v4 projection;
- correction requires the exact current revision, appends an immutable
  predecessor-linked revision, advances the head/projection, and applies
  ADR-0009 deletion without rebinding;
- parent delete applies the approved cascades, clears the head, purges source
  content, removes the v4 row, and preserves content-free metadata/provenance;
- import uses one batch transaction, creates honest `legacy_v4_baseline`
  revisions, and skips consistent/repeated IDs without overwrite;
- stale and missing outcomes roll back and return explicit unchanged states;
- ordinary artifact lifecycle requirements fail closed.

## Data Model Impact

No DDL or table change. Synthetic/disposable exact-v5 databases receive only
the already-approved normalized rows. Production remains schema v4.

## Migration Impact

No migration change or activation. Tests invoke the promoted disposable
schema-v4-to-v5 migration core only to construct exact-v5 fixtures.
`SCHEMA_VERSION` and production startup support remain 4.

## Provenance Impact

New fixture writes reuse the promoted domain-separated source revision ID and
canonical user provenance fingerprint. Provenance is linked to the exact
revision and only exact fingerprints may deduplicate. Migration receipt
manifests remain immutable.

## Historical Context Impact

Correction and parent deletion remove affected Historical Questions through
the existing ADR-0009 v4 cascade and v5 bridge inside the same guarded
transaction. Exact dependencies are never rebound. Ordinary artifact
correction/invalidation is not simulated and fails closed.

## Consent Impact

No consent is created, reused, or transmitted. Existing consent and
transmission evidence is deleted only when the already-approved ADR-0009
Historical Question cascade requires it.

## Provider Transmission Impact

None. No provider, ContextPacket, network, or transmission path changed.

## Tests Added

Fifteen module-local disposable tests cover:

- exact create authority/projection/provenance and multiline UTF-8 bytes;
- correction lineage and prior revision retention;
- stale update, stale delete, missing source, and non-advancing refusal;
- ordinary artifact lifecycle refusal;
- ADR-0009 correction cascade and no rebinding;
- parent deletion and content purge;
- honest duplicate-skipping import and full-batch rollback;
- every create write/failure boundary plus cascade-boundary rollback;
- conservative ambiguous-commit pre/post classification;
- malformed, v4, inconsistent-v5, and newer-version refusal;
- deterministic IDs/manifests;
- immutable migration receipt manifests.

## Tests Executed

- `cargo test --manifest-path src-tauri/Cargo.toml --lib
  schema_v5_migration::experience_write::tests --no-fail-fast`
  - passed: 15/15
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets
  --no-fail-fast`
  - passed: 85 Rust library tests, 12 backup/restore integration tests, and 8
    schema-contract tests
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --
  -D warnings`
  - passed
- `cargo fmt --all --manifest-path src-tauri/Cargo.toml -- --check`
  - passed
- `git diff --check`
  - passed

## Verification Results

Canonical repository verification passed:

- workflow contract: 17/17;
- Vitest: 26 files / 204 tests;
- Rust: 85 library tests, 12 backup/restore integration tests, and 8
  schema-contract tests;
- TypeScript typecheck and frontend production build;
- Rust check;
- unstaged/staged whitespace, UTF-8, secret-file, and Markdown-link checks;
- Constitution diff: none.

The recurring PowerShell profile `conda-script.py ... initialize` diagnostic
occurred after the successful commands and did not change the canonical exit
code (`0`) or results.

## Manual Verification Required

No desktop runtime check applies because the module is private, unregistered,
and has no runtime/UI surface. Founder exact diff review remains required.

## Documentation Updates

- architecture/13 version 2.4 records the exact Slice 4A authority,
  disposable-only implementation evidence, transaction/cascade boundaries,
  and remaining production/lifecycle/restart fences.
- MVP, Roadmap, and architecture/10 now factually record the already-promoted
  P1 feature and merge commits without claiming deployment or release.

## ADR Impact

No ADR status or decision changed. The implementation exercises the exact
Founder-authorized ADR-0011 exception, preserves ADR-0007 provenance
separation, and preserves ADR-0009 source invalidation.

## Deviations From Plan

- Content validation was corrected during implementation to preserve exact
  multiline UTF-8 Experience bytes rather than rejecting all control
  characters. IDs and guard tokens still reject control characters.
- Stale and missing requests return explicit unchanged outcomes after rollback
  and read-only verification, instead of being collapsed into generic errors.
- Provenance linking was ordered after source head advancement to match the
  accepted transaction contract.

No authorized scope was expanded.

## Known Limitations

- There is no durable per-write operation receipt. Real process-loss restart
  recovery is unproved.
- The code never executes against real user data.
- Ordinary artifact/Reflection/Pattern and Phase 3B write parity is absent.
- Production schema-v5 startup, Tauri/UI registration, migration, lifecycle
  UI, export v2, retention, and recovery remain absent.

## Remaining Risks

The disposable tests provide logical SQLite transaction and read-only reopen
evidence, not production crash durability or real-user safety. A future slice
must design and separately authorize durable restart handling and ordinary
artifact lifecycle parity before any runtime activation.

## Git State

- Branch: `codex/phase-3c-slice4a-experience-v5-write-parity`
- HEAD: `cf7633780a1a0a72efcad7558e463ceb094468c4`
- Working tree: modified workflow/docs/Rust plus one untracked Rust module
- Staged files: none
- Commit/push/merge/PR/deployment/release: none

## Engineer Completion Status

`completed_with_follow_up`: implementation and canonical validation are
complete; Theory Alignment Review, archive/reset, and Founder diff review
remain.
