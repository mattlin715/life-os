# Engineering Report

Status: completed

- Sprint ID: 2026-07-18-pilot-2-interruption-recovery
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: af2d8736a20cc1820dacc566e25c5accd6b41d59
- Working-tree digest implemented: bc59d2da148232efc47b4e4bdb4887beecd6c6544696135884105a97f467725d
- Created at: 2026-07-17T17:40:41Z
- Updated at: 2026-07-17T18:18:06Z

## Implementation Summary

Added exactly two Node regression tests for torn workflow projections. Both
tests create valid before/after checkpoints through the real workflow CLI in
an OS-temporary copied repository, then combine the journal and state snapshots
to prove the validator fails closed when either projection is ahead.

Review cycle 1 followed a real terminal-transition `EPERM`. The failed rename
left live revision 13 consistent but orphaned its candidate event temp file.
After a read-only audit captured the orphan's exact size, SHA-256, and content
relationship, the verified orphan was removed, `writeAtomic` gained temp cleanup,
and event/state writes gained captured-pair rollback. Two deterministic
fault-injection tests cover failure on each write position.

## Existing System Areas Inspected

- `.ai/workflow/WORKFLOW_CONTRACT.json`
- `.ai/workflow/WORKFLOW_STATE.json`
- `.ai/workflow/EVENTS.jsonl`
- `scripts/ai-workflow.mjs`
- `scripts/ai-workflow.node-test.mjs`
- `.ai/workflow/PRODUCT_REVIEW.md`
- `.ai/workflow/ENGINEERING_PLAN.md`

## Files Added

None.

## Files Modified

- `scripts/ai-workflow.node-test.mjs`
- `scripts/ai-workflow.mjs`
- Current `.ai/workflow/` control-plane artifacts through guarded workflow
  commands and this report

## Files Deleted

None.

## Behavior Changed

No Life OS product/runtime behavior changed. Regression coverage now explicitly
protects existing fail-closed detection for:

- five journal events paired with revision-4 product-review state;
- revision-5 engineering-planning state paired with four journal events.

Each case asserts revision-count, last-event, and status/sprint disagreement.

Workflow-kernel failure behavior also changed: a failed atomic rename removes
only its command-owned temp and rethrows; if either event/state write fails,
both captured originals are restored before the failure returns. This is
command-local rollback, not reconciliation of an unknown mismatch.

## Data Model Impact

None. Synthetic workflow metadata exists only inside disposable OS-temp test
directories.

## Migration Impact

None. SQLite, schema versions, migrations, and runtime databases are unchanged.

## Provenance Impact

No product provenance impact. Test evidence strengthens the Engineering Harness
rule that the journal and current projection must agree before progress.

## Historical Context Impact

None.

## Consent Impact

None.

## Provider Transmission Impact

None. No provider path is invoked.

## Tests Added

- `validator fails closed when the event journal is ahead of workflow state`
- `validator fails closed when workflow state is ahead of the event journal`
- `first atomic rename failure removes its temp and preserves the workflow pair`
- `second atomic rename failure restores events and state without orphan temps`

Shared helpers create valid CLI-generated snapshots without duplicating
production event hashing. Fault injection is process-local and active only when
`NODE_ENV=test` and the explicit test environment variable are both set.

## Tests Executed

- Initial `pnpm run test:workflow`: 15/15 passed.
- Review-cycle-1 `pnpm run test:workflow`: 17/17 passed.
- `pnpm workflow:validate`: passed against the live control plane after tests.
- No workflow temp file remained after fault tests.
- Git inspection confirmed implementation diff is limited to
  `scripts/ai-workflow.mjs` and `scripts/ai-workflow.node-test.mjs`.
- Full canonical verification is deliberately deferred to the validation
  micro-block and is not represented as passed here.

## Verification Results

Targeted implementation verification passed. Both mismatch projections were
rejected with all three planned diagnostic categories. Both injected rename
positions returned non-zero, preserved exact original state/event bytes,
removed all temp files, and left copied workflows valid. The live workflow is
valid at implementation revision 19. Canonical verification for the corrected
snapshot remains pending.

## Manual Verification Required

No UI verification applies. The later validation review must still confirm
repository status, event tail, architecture/13 exclusion, and canonical checks.

## Documentation Updates

Current Product Review and Engineering Plan record the real EPERM evidence and
bounded cycle-1 correction. No durable architecture or ADR document changed.

## ADR Impact

None. ADR-0008 remains Accepted and unchanged. The implementation applies the
existing Stage 1 fail-closed boundary without extending authority.

## Deviations From Plan

Review cycle 1 expanded the original test-only plan after real evidence
invalidated the assumption that validator coverage alone was sufficient. The
expansion was routed through Product Review and Engineering Planning before the
kernel changed.

## Known Limitations

- Tests prove detection, not repair.
- Synthetic torn projections do not prove arbitrary process-crash recovery.
- No multi-writer, lease, replay, or Stage 2/3 behavior is tested or added.
- Pair rollback can itself fail under persistent filesystem denial; such a case
  preserves error evidence and remains fail closed rather than guaranteeing
  recovery.

## Remaining Risks

Canonical verification must be rerun because the implementation digest changed
and earlier verification is stale by design. Terminal completion must not reuse
the failed temp candidate event or its idempotency key.

## Git State

- Branch: `codex/orchestration-pilot-2-interruption-recovery`
- HEAD: `af2d8736a20cc1820dacc566e25c5accd6b41d59`
- Upstream: none configured
- Staged files: none
- Commit, push, merge: not performed
- Unrelated untracked architecture/13: preserved and untouched

## Engineer Completion Status

completed
