# Engineering Report

Status: completed

- Sprint ID: 2026-07-25-phase-3c-post-slice2b1-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: a476c38ba5c4a9b19a81fbb14973aacc4adf25bd
- Working-tree digest implemented: 292207300477be3ac144dcb0e3e8c29c5fafb51141927a5452672e8fe7ba8a60
- Created at: 2026-07-25T14:20:00+09:00
- Updated at: 2026-07-25T15:15:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the Founder-authorized Slice 2B-2 as a private compile-time Rust
filesystem-safety module. It defines explicit quiescence, sidecar refusal,
canonical path ownership, collision-resistant create-new operation ownership,
exact candidate evidence, platform-aware volume/durability boundaries,
injected replacement outcomes, content-free restart state, and exact-owned
staging cleanup. Thirteen unit tests exercise it exclusively in synthetic
disposable app-like directories.

The module has no Tauri command, invoke-handler registration, renderer/UI
adapter, startup call, app-data path, real database path, or default replacement
invocation.

Theory review cycle 0 identified and corrected two fail-closed gaps before final
verification:

- canonical ownership validation now rejects a symlink/reparse point in every
  existing ancestor component, not only the supplied root or final file;
- restart inspection now reconciles a `Prepared` state's recorded
  `live_before_sha256` with the current live file and returns
  `recovery_required` if they differ.

## Existing System Areas Inspected

- `src-tauri/src/lib.rs`
- `src-tauri/src/sqlite.rs`
- `src-tauri/tests/schema_v5_backup.rs`
- `src-tauri/tests/fixtures/schema_v5/v4.sql`
- `src-tauri/Cargo.toml`
- ADR-0007, ADR-0009, ADR-0010, ADR-0011
- architecture/01, architecture/11, architecture/12, architecture/13
- archived Slice 2B-1 workflow evidence

## Files Added

- `src-tauri/src/filesystem_safety.rs`

## Files Modified

Product and factual documentation:

- `src-tauri/src/lib.rs`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

Workflow evidence:

- `.ai/workflow/CURRENT_MISSION.md`
- `.ai/workflow/PRODUCT_REVIEW.md`
- `.ai/workflow/DECISION_REQUIRED.md`
- `.ai/workflow/ENGINEERING_PLAN.md`
- `.ai/workflow/ENGINEERING_REPORT.md`
- `.ai/workflow/EVENTS.jsonl`
- `.ai/workflow/WORKFLOW_STATE.json`

## Files Deleted

none.

## Behavior Changed

Compile-time internal behavior only:

- a quiescence guard refuses active or unknown database activity;
- WAL, SHM, and rollback-journal presence refuses without mutation;
- canonical direct-owned paths reject traversal, outside-root paths, hard-link
  aliases, symlink/reparse points in any existing ancestor, cross-volume
  identity, and collisions;
- SHA-256-derived operation IDs reserve backup/staging/state with create-new;
- candidate verification requires exact file digest, source manifest, schema
  version 4, foreign-key success, integrity success, and record digest;
- durability separates staged-file sync from parent-directory support/sync;
- replacement returns committed, failed-unchanged, or outcome-unknown;
- ambiguous/post-commit failure becomes `recovery_required` without retry,
  replay, rollback, repair, sidecar cleanup, or candidate selection;
- restart inspection is read-only and preserves candidates;
- cleanup can remove only exact owned staging.

No user-visible or production runtime behavior changed.

## Data Model Impact

No application data-model change. The internal operation-state schema is
content-free and unactivated. Tests use synthetic disposable SQLite files.

## Migration Impact

None. No DDL or migration exists. Production `SCHEMA_VERSION` and
`user_version` remain 4.

## Provenance Impact

None. The candidate-verifier contract checks exact synthetic record evidence
but does not create, modify, repair, infer, or reconstruct provenance.

## Historical Context Impact

None. Synthetic historical/provenance fixture rows remain opaque storage data.
No retrieval, Context Packet, provider call, Historical Question generation,
cross-experience analysis, or Phase 4 behavior.

## Consent Impact

None. No consent event is created, consumed, reused, or transmitted.

## Provider Transmission Impact

None. Provider and ContextPacket code is unchanged.

## Tests Added

Thirteen Rust tests cover:

1. active/unknown quiescence refusal;
2. WAL/SHM/rollback-journal refusal and immutability;
3. traversal/outside-root/cross-volume/collision refusal;
4. hard-link alias refusal;
5. symlink and reparse fail-closed contract;
6. generated ownership and actual system volume/durability adapters;
7. exact successful disposable replacement and completed restart state;
8. expected digest/manifest/schema/FK/integrity/record mismatch;
9. verifier-returned manifest/FK/integrity/record mismatch;
10. unsupported durability and ambiguous post-commit parent-sync failure;
11. failed-unchanged versus outcome-unknown behavior;
12. activity change immediately before replacement and post-commit validation
    failure;
13. prepared/staged/committed/cleanup/missing/corrupt/contradictory restart
    classification, including prepared-live digest drift, and exact cleanup
    refusal.

## Tests Executed

- `cargo fmt --manifest-path .\src-tauri\Cargo.toml -- --check`
  - passed.
- `cargo check --manifest-path .\src-tauri\Cargo.toml`
  - passed.
- `cargo test --manifest-path .\src-tauri\Cargo.toml filesystem_safety -- --nocapture`
  - passed: 13/13.
- `cargo test --manifest-path .\src-tauri\Cargo.toml -- --nocapture`
  - passed:
    - Rust library 40/40, including Slice 2B-2;
    - Slice 2A/2B-1 integration 12/12;
    - Slice 0 schema contract 8/8.
- `git diff --check`
  - passed.

## Verification Results

- Focused and full Rust verification: passed.
- Scope scan: no Tauri command, invoke registration, UI, renderer, startup,
  app-data, provider, ContextPacket, schema-version, or migration activation.
- Canonical repository verification: passed.
  - workflow contract: 17/17;
  - Vitest: 22 files / 163 tests;
  - Rust library: 40/40, including 13 Slice 2B-2 cases;
  - Slice 2A/2B-1 integration: 12/12;
  - Slice 0 schema contract: 8/8;
  - TypeScript typecheck, frontend build, and Rust check: passed;
  - UTF-8, whitespace, secret, Markdown-link, and Constitution checks: passed.

## Manual Verification Required

Not applicable. No user-visible or production runtime path exists. Founder diff
review remains required after canonical verification and Theory Alignment
Review.

## Documentation Updates

Architecture/13 version 1.5 records:

- Slice 2B-1 factual promotion closeout;
- exact Slice 2B-2 Option A authority;
- nine distinct evidence levels;
- current disposable-path implementation and 13-test evidence;
- continued exclusion of product activation, real user data, cross-platform
  atomicity claims, sidecar recovery, schema v5, later slices, and Phase 4.

## ADR Impact

No ADR added or changed. ADR-0007, ADR-0009, ADR-0010, and ADR-0011 remain
unchanged.

## Deviations From Plan

One bounded dependency detail: Windows stable Rust does not expose volume serial
and link-count metadata through stable `std` APIs. The implementation therefore
uses the already-resolved `windows-sys` 0.59 API for volume-root identity,
reparse attributes, and hard-link count. No product surface or authority
expanded.

Theory review cycle 0 required two bounded corrections within the approved
filesystem-safety contract. Both corrections were applied in the same private
module, covered by the focused 13-test suite, and did not expand product scope.

## Known Limitations

- No replacement adapter is connected to a production platform call.
- Windows honestly reports parent-directory sync unsupported through the
  standard durability adapter; no success claim is made.
- Synthetic tests do not prove process-wide quiescence, antivirus behavior,
  power-loss durability, or cross-platform atomicity.
- The candidate verifier is injected; production schema-v4 manifest/record
  verification is not activated or connected to real data.
- Restart classification preserves ambiguity but provides no repair/recover
  command.
- Backup/state deletion and retention remain unauthorized.

## Remaining Risks

The largest risk is treating compiled reusable code as activated recovery
authority. The private module declaration, lack of call sites, documentation,
and scope scans preserve the distinction. Real app-data integration,
quiescence ownership, platform replacement, user disclosure, retention, and
real-data verification require new Founder authority.

## Git State

- Branch: `codex/phase-3c-slice2b2-filesystem-safety`
- HEAD: `a476c38ba5c4a9b19a81fbb14973aacc4adf25bd`
- Working tree: unstaged authorized implementation, factual documentation, and
  workflow evidence
- Staged files: none
- Upstream: none configured
- Commit/push/merge/PR/deployment: none

## Engineer Completion Status

completed - authorized implementation and cycle 0 corrections pass focused
Rust verification; canonical verification and final Theory Alignment Review
remain.
