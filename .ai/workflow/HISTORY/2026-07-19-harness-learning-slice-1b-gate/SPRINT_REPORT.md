# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-18T20:02:00.000Z
- Updated at: 2026-07-18T20:02:00.000Z

## Sprint ID

2026-07-19-harness-learning-slice-1b-gate

## Mission

Close the promoted Slice 1A evidence truthfully, complete the bounded Stage 1
Harness learning decision, obtain an explicit Founder decision for the next
Phase 3C boundary, and implement only Founder-authorized Slice 1B-1.

## Starting Commit

`6e9dd6615cb7f99556d258080f6a45140fd55b68` on
`codex/harness-learning-slice-1b-gate`; `develop` and `origin/develop` matched.

## Ending Commit Or Working-Tree State

HEAD remains `6e9dd6615cb7f99556d258080f6a45140fd55b68`. Final product working-tree
digest is `34eb902d6a4be3fcfbd496655657d0d2d82f40b98ef03af9a5b4c370fda70569`.
The authorized diff is unstaged and uncommitted for Founder review.

## Final Status

completed_with_follow_up

## Product Decision

- `HL-001`: Founder accepted only the bounded Stage 1 conclusion for manually
  triggered, repository-mediated, single-writer product sprints. This provides
  no independent-review assurance and does not authorize Stage 2 or Stage 3.
- `PHASE3C-SLICE1B-001`: Founder selected Option B and authorized Slice 1B-1
  only. Slice 1B-2 and all later boundaries remain unauthorized.

## Engineering Summary

Experience create, expected-revision update, delete, and duplicate-skipping
atomic import now use typed Rust Tauri commands and typed TypeScript adapters.
The renderer no longer supplies SQL for those four paths. One bounded review
cycle added a mandatory durable-revision advancement guard.

## Behavior Changed

- Typed Experience mutations run under Rust-owned `BEGIN IMMEDIATE`
  transactions.
- Update compares the durable `updated_at` token before any dependent
  invalidation and returns `stale_generation` on mismatch.
- A committed update cannot reuse its expected revision token.
- Update/delete preserve existing schema-v4 artifact, Historical Question,
  consent, transmission, dependency, and actual-use provenance cascades.
- Import skips duplicate IDs, reports inserted/skipped counts, and rolls back
  the whole batch on non-conflict failure.

## Files Changed

Product and factual-documentation paths:

- `src-tauri/src/lib.rs`
- `src-tauri/src/sqlite.rs`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.test.ts` (new)
- `docs/architecture/01_Local_Evidence_Store.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `docs/architecture/14_AI_Orchestration_Evolution.md`
- `docs/dev/08_Engineering_Harness.md`
- `.ai/workflow/WORKFLOW_EVALUATION.md`
- repository workflow artifacts and the eventual sprint archive

## Tests

- 17 workflow contract tests passed.
- 22 Vitest files / 158 tests passed, including 6 typed-adapter tests.
- 21 Rust unit tests passed, including typed mutation, rollback, cascade,
  newer-schema refusal, and non-advancing revision regressions.
- 8 schema-v5 test-only contract tests passed.
- TypeScript typecheck, frontend production build, and Rust check passed.

## Repository Verification

Final post-correction canonical command passed:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

UTF-8, whitespace, secret-file, Markdown-link, workflow-state, and Constitution
checks passed. An earlier 120-second shell timeout produced a broken pipe during
Rust output; it was not treated as verification evidence and the command was
rerun successfully with sufficient time.

## Manual Verification

Slice 1A Founder manual verification completed for fresh schema-v4 startup,
persistence across restart, newer-schema refusal, English/Traditional Chinese/
Japanese disclosure parity, malformed-database refusal, byte preservation after
the guard, and disposable-folder cleanup. The first isolation attempt revealed
that an `APPDATA` override did not redirect Tauri's Windows resolver; testing
stopped, the real database remained unchanged after the captured guard, and all
remaining tests used unique application identifiers. Slice 1B-1 runtime smoke
verification has not been run and remains an optional Founder review check.

## Architecture Updates

- Architecture/01 records the typed schema-v4 Experience mutation boundary.
- Architecture/13 distinguishes implemented-but-unpromoted Slice 1B-1 from
  unauthorized Slice 1B-2 and schema-v5 activation.
- Architecture/14 records the bounded Stage 1 conclusion without Stage 2/3
  readiness claims.

## ADR Updates

No ADR status or decision changed. ADR-0007, ADR-0009, and ADR-0011 semantics
are preserved. ADR-0008 remains Accepted without decision expansion.

## Documentation Synchronization

Engineering Harness and workflow evaluation documents now record five real
product sprints, the Founder-accepted bounded Stage 1 conclusion, and the lack
of independent-review/Stage-2/3 assurance.

## Data And Migration Impact

No schema or migration. `SCHEMA_VERSION` and production SQLite `user_version`
remain 4. No live user database, backup, restore, retention cleanup, schema-v5
DDL, or later Phase 3C slice was used.

## Provenance And Consent Impact

No new policy or shape. Existing ADR-0009 invalidation/deletion semantics are
preserved and regression-tested. Provider and ContextPacket code is unchanged.

## Risks

- Slice 1B-1 desktop runtime smoke has not been run.
- The current diff is not promoted and awaits Founder diff review.
- Generic renderer-supplied SQL remains for deferred artifact/historical/audit
  mutation paths; this is the explicit Slice 1B-2 boundary, not hidden closure.

## Deferred Items

Slice 1B-2; schema-v5 activation and `user_version = 5`; live migration;
backup/restore/retention; Phase 3C Slices 2-6; Phase 4; provider/ContextPacket
changes; Harness Stage 2/3; PR and deployment.

## Human Decisions

Resolved: `HL-001` = HL-A and `PHASE3C-SLICE1B-001` = Option B. Next human gate
is Founder diff review. Promotion requires a separate explicit authorization.

## Review Cycles

One. Theory review found that a same-tick update could reuse `updated_at`. The
adapter now advances the timestamp by one millisecond when required, Rust
rejects equality before writing, and final focused/canonical regressions pass.

## Workflow Lessons

- Manual verification evidence must distinguish operating-system environment
  variables from the actual Tauri application-data resolver.
- A revision token must change on every committed mutation; equality checks on
  the expected token are insufficient unless the replacement token also
  advances.
- The existing Harness supported the product sprint and bounded correction; no
  Harness functionality was added.

## Recommended Next Sprint

First complete Founder diff review and, if desired, a disposable Slice 1B-1
desktop smoke check. If accepted, request a separate narrow promotion
authorization for this exact diff. Do not begin Slice 1B-2 from this report.

## Git Status

Branch `codex/harness-learning-slice-1b-gate`; HEAD/develop/origin-develop all
remain `6e9dd6615cb7f99556d258080f6a45140fd55b68`. No staged files, commit, push,
merge, PR, or deployment occurred.
