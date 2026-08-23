# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-23-ordinary-v5-real-profile-phase-b-blocked-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest implemented: f50002b19829cf3928ea2ed57d3c81046cc58ade04fc238e08b4a041246d9e17
- Created at: 2026-08-23T00:00:00+09:00
- Updated at: 2026-08-23T23:42:53+09:00

## Implementation Summary

Implemented the Founder-authorized Option A correction for sidecar-free exact
schema-v4 databases whose SQLite header retains persistent WAL mode. Stable
source verification and the verified-backup source read now use immutable
read-only SQLite connections with explicit close. Existing sidecars still fail
closed before open. Added production-shaped disposable success, malformed, and
uncheckpointed regressions. The first packaged review then exposed a bounded
post-commit runtime sidecar defect. Revision cycle 2 now routes all stable
schema-v5 runtime reads through the immutable sidecar-prechecked boundary and
explicitly closes every reachable typed writer before durable verification. No
real-profile action was retried.

## Existing System Areas Inspected

- `src-tauri/src/schema_v5_migration.rs`
- `src-tauri/src/filesystem_safety.rs`
- `src-tauri/src/schema_v5_founder_activation.rs`
- `src-tauri/src/schema_v5_runtime.rs`
- `src-tauri/src/schema_v5_experience_write.rs`
- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_evidence_lifecycle.rs`
- `src-tauri/src/schema_v5_reflection_write.rs`
- `src-tauri/src/schema_v5_pattern_write.rs`
- `src-tauri/src/schema_v5_context_recovery_write.rs`
- `src-tauri/src/schema_v5_historical_question_write.rs`
- `src-tauri/src/sqlite.rs`
- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
- current Engineering Harness workflow artifacts

## Files Added

No new correction-specific source file. The parent R1 sprint's already
anticipated untracked architecture, runbook, package scripts/configuration, and
workflow archives remain part of the unpromoted working tree.

## Files Modified

Correction-specific changes:

- `src-tauri/src/schema_v5_migration.rs`
- `src-tauri/src/filesystem_safety.rs`
- `src-tauri/src/schema_v5_founder_activation.rs`
- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
- `.ai/workflow/CURRENT_MISSION.md`
- `.ai/workflow/PRODUCT_REVIEW.md`
- `.ai/workflow/ENGINEERING_PLAN.md`
- `.ai/workflow/ENGINEERING_REPORT.md`
- `.ai/workflow/DECISION_REQUIRED.md`
- `.ai/workflow/EVENTS.jsonl`
- `.ai/workflow/WORKFLOW_STATE.json`

The remaining modified/untracked paths are the preserved, previously reviewed
ordinary activation R1 implementation and its earlier workflow archives.

## Files Deleted

none

## Behavior Changed

- A sidecar-free, checkpointed exact-v4 source with persistent WAL header bytes
  can be inspected without the verifier itself creating WAL/SHM files.
- The verified backup reads the exact source immutably and writes only the
  exact-owned `VACUUM INTO` destination.
- A WAL, SHM, or rollback-journal file present before inspection remains a
  fail-closed recovery state.
- Malformed sources remain blocked before operation creation.
- Stable activated-v5 reads no longer create WAL/SHM sidecars on a closed,
  sidecar-free persistent-WAL database.
- Every bounded typed writer explicitly closes its writable connection before
  durable verification; a close failure remains recovery-required.

## Data Model Impact

None. No DDL, table, column, manifest, projection, or schema-version contract
changed.

## Migration Impact

The migration policy is unchanged. The correction removes a verifier-created
side effect for an already supported exact-v4 state. It does not checkpoint,
repair, clean up, retry, or relax any source predicate.

## Provenance Impact

None. Artifact provenance, revision dependencies, receipts, and manifests are
unchanged.

## Historical Context Impact

None. ADR-0009 selection, consent, provenance, eligibility, and cascade behavior
are unchanged.

## Consent Impact

None.

## Provider Transmission Impact

None. No provider call or ContextPacket behavior changed.

## Tests Added

- persistent-WAL exact-v4 full activation with verified backup and no source
  sidecars;
- persistent-WAL verified backup source immutability;
- pre-existing uncheckpointed WAL/SHM fail-closed before operation creation;
- malformed source fail-closed before operation creation.
- migrated persistent-WAL typed runtime coverage for Experience, Evidence, and
  Context Recovery, including a contradictory-provenance no-write path;
- complete existing Reflection, Pattern, Historical Question, lifecycle,
  rollback, and ambiguous-commit suites after explicit writer-close changes.

## Tests Executed

- `cargo test --manifest-path src-tauri/Cargo.toml --features desktop-schema-v5 schema_v5_founder_activation` — passed, 9 tests.
- `cargo test --manifest-path src-tauri/Cargo.toml --features desktop-schema-v5 filesystem_safety::tests` — passed, 24 tests.
- Focused `persistent_wal` filter — passed, 3 tests.
- Focused `malformed_source_fails_closed_before_operation` filter — passed, 1 test.
- Persistent-WAL exact activation repeated five times after explicit-close
  correction — passed five of five.
- Revision cycle 2 full Rust library suite — passed, 204 tests.
- Revision cycle 2 migrated persistent-WAL typed runtime suite — passed, 4
  tests.

## Verification Results

The previous focused Rust verification, Clippy, and canonical verification
passed. The first
canonical run exposed a nondeterministic post-migration sidecar after dropping
the writable migration connection; it failed rather than being treated as a
pass. Revision cycle 1 added explicit close and the focused test then passed
five consecutive runs. A fresh canonical run then passed all deterministic
checks, including 349 frontend tests, 204 default Rust tests, 12 backup tests,
8 schema-contract tests, 33 legacy-v4 refusal tests, both ordinary and Founder
activation suites, and the Founder typed-runtime suite. The first packaged
manual migration subsequently exposed the same connection-lifetime issue in
the activated runtime. Revision cycle 2 formatting, all 204 Rust library tests,
and Clippy with warnings denied passed. The first revised canonical run failed
only because the existing Founder package contract allowlist omitted the
already-modified Evidence lifecycle writer; the allowlist was synchronized and
its focused contract test passed. A fresh canonical run then passed all
deterministic checks, including 349 frontend tests, 204 Rust tests, ordinary and
Founder activation suites, four migrated persistent-WAL typed-runtime tests,
legacy-v4 refusal, backup/schema contracts, documentation, security, and
Constitution checks.

## Manual Verification

Founder disposable manual review completed on 2026/08/23 in `LifeOSReviewR1`.
The failed disposable v5 profile was preserved without retry, repair, restore,
checkpoint, or cleanup, and the successor review used a fresh clone of the
untouched disposable exact-v4 fixture. The real Founder profile and its prepared
operation, zero-byte staging file, WAL/SHM evidence, database, and personal data
remained untouched. The failed package is version `0.3.0`, HEAD
`44ec6d56d645829488aa73d0b92bcf72b19487f4`, size 5,682,243 bytes, SHA-256
`E58927F317D573D304915372F0C1257141727B3B5D19DF04733C5D87187F2C51`.
That package is retained as failed manual evidence and must not be retried. The
successor ignored package is version `0.3.0`, HEAD
`44ec6d56d645829488aa73d0b92bcf72b19487f4`, size 5,679,953 bytes, SHA-256
`96BB5C8CE77574572375F15359751CF38D7D4B04CB81D5D8A6F1F752BBA51EBF`.
It was hash-verified, installed, and launched only in the disposable account;
it was not distributed, deployed, or released.

The review proved that the fresh persistent-WAL exact-v4 fixture remained
unchanged through installation and disclosure, one explicit migration reached
durable `v5_ready / lifecycle_writes_enabled` with one verified schema-v4
backup, normal close removed no evidence yet left no sidecars, restart rebuilt
the migrated record, one new typed Experience write succeeded, and a second
normal close/restart rebuilt both records without `sqlite_sidecar_present`,
recovery-required, or manifest mismatch. No third implementation correction was
needed; workflow cycle 3 only synchronizes this completed manual evidence and
refreshes canonical verification.

## Documentation Updates

Architecture 18 records the exact Phase B fail-closed evidence, root cause,
bounded Option A authority, both disposable correction cycles, implementation
semantics, and remaining real-profile gate. Dev runbook 10 records the
disposable persistent-WAL runtime retest boundary.

## ADR Impact

No ADR status or acceptance change. ADR-0007, ADR-0009, and ADR-0011 remain the
governing authority.

## Deviations From Plan

The initial narrow change to the exact-v4 verifier exposed the same normal
read-only side effect in later stable/durable verification, so the correction
was applied to those existing helpers too. Validation then exposed that dropping
the writable migration connection could leave sidecars visible for immediate
classification; revision cycle 1 changed that boundary to explicit close. Both
changes remain in the approved migration-core file and do not change policy.
The first packaged migration then showed that activated runtime reads and
dropped writer connections could recreate the same empty sidecars. Revision
cycle 2 extends only the already-authorized immutable read and explicit-close
mechanics across the existing typed runtime; it does not weaken sidecar refusal.

## Known Limitations

- The real ordinary profile has not successfully migrated.
- Its earlier prepared operation and sidecar evidence have not been cleaned up.
- Revision cycle 2 has both automated and completed disposable Founder manual
  migration/runtime evidence; it has no second real-profile migration evidence.
- No production distribution, deployment, or release evidence exists.

## Remaining Risks

The next real-profile action requires a separate Founder decision that accounts
for the retained operation evidence and sidecars. This sprint does not authorize
or define cleanup, checkpoint, retry, repair, restore, or a second migration.

## Git State

- Branch: `codex/desktop-schema-v5-ordinary-production-activation-r1`
- HEAD: `44ec6d56d645829488aa73d0b92bcf72b19487f4`
- Index: no staged files
- Working tree: modified/untracked parent R1 implementation plus this bounded
  correction; no unrelated path was introduced by the correction
- Upstream: none configured

## Engineer Completion Status

completed_with_follow_up — revision cycle 2 implementation, formatting, Rust
tests, Clippy, fresh canonical validation, and the successor ignored package
build and disposable Founder migration/runtime review are complete. The real
profile remains fail-closed at its earlier exact-v4 evidence and any future
action requires a separate Founder decision.
