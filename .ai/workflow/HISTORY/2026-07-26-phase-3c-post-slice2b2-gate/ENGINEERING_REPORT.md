# Engineering Report

Status: completed

- Sprint ID: 2026-07-26-phase-3c-post-slice2b2-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 277c4b5b2031d5bf88dc2a33b03765c103c62629
- Working-tree digest implemented: d9406fcad00d5e5f5dc2f3fba9c1af162a32480d7044c9f96f26d4376bf6c81a
- Created at: 2026-07-26T03:28:00+09:00
- Updated at: 2026-07-26T03:28:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the Founder-authorized Slice 2B-3 inside the existing private,
unregistered Rust filesystem-safety module. The change integrates the promoted
Slice 2A `VACUUM INTO` creation behavior with the Slice 2B-2 ownership and
restart boundary instead of introducing a parallel backup system.

A collision-resistant create-new operation directory and content-free state
claim the exact backup child pathname while leaving that pathname absent for
SQLite. The implementation revalidates the approved path, identity,
quiescence, sidecar, same-volume, and collision boundaries immediately before
one backup-creator invocation, then proves direct single-link operation
ownership and exact schema-v4 evidence after the database is closed.

A private Windows `ReplaceFileW` adapter classifies committed,
failed-unchanged, and outcome-unknown results conservatively. Normal Windows
execution remains fail-closed before replacement because the required
parent-directory durability operation is not supported.

## Existing System Areas Inspected

- `src-tauri/src/filesystem_safety.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/sqlite.rs`
- `src-tauri/tests/schema_v5_backup.rs`
- `src-tauri/tests/fixtures/schema_v5/v4.sql`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- ADR-0007, ADR-0009, ADR-0010, and ADR-0011
- archived Slice 2A, Slice 2B-1, and Slice 2B-2 workflow evidence

## Files Added

none.

## Files Modified

Product implementation and factual documentation:

- `src-tauri/src/filesystem_safety.rs`
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

Private compile-time behavior only:

- each operation now owns a unique create-new directory under the validated
  disposable root;
- staging and state remain create-new, while the exact backup child stays
  absent until SQLite executes one parameter-bound `VACUUM INTO`;
- the source file identity and governed schema-v4 evidence are captured and
  revalidated immediately before creation;
- activity, sidecars, aliases, collisions, identity drift, or volume drift
  refuse before SQLite is invoked;
- after close, the output must be a direct, regular, single-link,
  exact-operation-owned file with matching SHA-256, source manifest, schema
  version 4, foreign keys, integrity, and exact records;
- ambiguous output ownership or validity becomes `recovery_required` and is
  preserved;
- only a positively proved exact-owned incomplete output may be deleted after a
  failed creation;
- restart inspection understands `BackupVerified` and treats a contradictory
  `Prepared` state with output present as recovery-required;
- a private Windows `ReplaceFileW` adapter supplies conservative result
  classification, including documented partial-failure codes.

No user-visible or production runtime behavior changed.

## Data Model Impact

No application data-model change. The private content-free operation-state
schema advances from 1 to 2 to record the exact operation-relative directory
and `BackupVerified` phase. It is unregistered and used only with synthetic
disposable files.

## Migration Impact

None. No schema-v5 DDL is executed. Production `SCHEMA_VERSION` and SQLite
`user_version` remain 4.

## Provenance Impact

None. The verifier compares synthetic exact-record and governed
source-manifest evidence but does not create, rewrite, infer, or reconstruct
product provenance.

## Historical Context Impact

None. No retrieval, Context Packet, Historical Question generation,
cross-experience interpretation, or Phase 4 behavior.

## Consent Impact

None. No consent is created, reused, consumed, persisted, or transmitted.

## Provider Transmission Impact

None. Provider and ContextPacket code are unchanged.

## Tests Added

Nine focused tests were added:

1. exact operation ownership with one real `VACUUM INTO`;
2. collision after pathname claim, with no SQLite invocation;
3. source-identity replacement before creation, with no SQLite invocation;
4. activity and sidecar changes after claim, with no SQLite invocation;
5. final-preflight volume drift, with no SQLite invocation;
6. exact-owned incomplete-output cleanup versus ambiguous hard-link
   preservation;
7. post-close evidence failure preservation as `recovery_required`;
8. conservative Windows replacement error classification;
9. Windows `ReplaceFileW` changes only exact disposable paths.

Existing Slice 2B-2 cases were updated for absent backup ownership and
`BackupVerified` restart state. The focused filesystem-safety suite now has 22
tests.

## Tests Executed

- `cargo fmt --manifest-path .\src-tauri\Cargo.toml -- --check`
  - passed.
- `cargo clippy --manifest-path .\src-tauri\Cargo.toml --all-targets -- -D warnings`
  - passed.
- `cargo test --manifest-path .\src-tauri\Cargo.toml filesystem_safety::tests`
  - passed: 22/22.
- `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`
  - passed:
    - workflow contract 17/17;
    - Vitest 22 files / 163 tests;
    - Rust library 49/49, including 22 filesystem-safety cases;
    - Slice 2A/2B-1 integration 12/12;
    - Slice 0 schema contract 8/8;
    - TypeScript typecheck, frontend build, and Rust check;
    - UTF-8, whitespace, secret, Markdown-link, workflow-state, and
      Constitution checks.
- `git diff --check`
  - passed.

## Verification Results

All automated verification passed. The actual Windows adapter test operated
only inside a Rust temporary directory and proved that an outside sentinel was
unchanged. No application or system path was supplied.

## Manual Verification Required

Not applicable for runtime behavior because no Tauri, renderer, UI, startup,
app-data, or production caller exists. Founder diff review remains required.

## Documentation Updates

Architecture/13 version 1.7 records:

- exact Slice 2B-3 Founder authority;
- the existing Slice 2A integration rather than a parallel backup path;
- operation-directory and absent-backup ownership;
- exact pre-creation and post-close evidence;
- private conservative Windows replacement classification;
- the continuing Windows durability refusal and non-activation boundaries;
- ten evidence levels, with Slice 2B-3 still current-working-tree evidence.

## ADR Impact

No ADR added or changed. Accepted ADR decisions and statuses are unchanged.

## Deviations From Plan

none.

## Known Limitations

- The implementation does not defend against a malicious same-user process.
- No process-wide production quiescence mechanism exists.
- Windows parent-directory and power-loss durability are not established.
- The Windows adapter is directly tested only in disposable temporary paths and
  is not reachable from the product.
- No production replacement safety or real-user recovery claim is made.
- No autonomous retry, replay, rollback, repair, sidecar cleanup, or candidate
  selection exists.

## Remaining Risks

The primary risk is confusing a private compiled primitive with authorized
product recovery. The lack of registration and callers, fail-closed durability
boundary, explicit documentation, and synthetic-only tests preserve the
distinction. Production activation requires a separate Founder decision and
new real-path, quiescence, disclosure, and recovery evidence.

## Git State

- Branch: `codex/phase-3c-slice2b3-backup-windows-integration`
- HEAD: `277c4b5b2031d5bf88dc2a33b03765c103c62629`
- Working tree: unstaged authorized implementation, factual documentation, and
  workflow evidence
- Staged files: none
- Upstream: none configured
- Commit/push/merge/PR/deployment: none

## Engineer Completion Status

completed - Slice 2B-3 implementation and canonical automated verification are
complete; Theory Alignment Review and Founder diff review remain.
