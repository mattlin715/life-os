# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-17T18:04:00Z
- Updated at: 2026-07-17T18:04:00Z

## Sprint ID

2026-07-18-pilot-2-interruption-recovery

## Mission

Evaluate Stage 1 interruption and recovery through repository-only checkpoints,
then protect torn state/event projection detection with bounded regression tests.

## Starting Commit

`af2d8736a20cc1820dacc566e25c5accd6b41d59` on promoted `develop`.

## Ending Commit Or Working-Tree State

Uncommitted bounded working tree on
`codex/orchestration-pilot-2-interruption-recovery`, with repository HEAD
unchanged at `af2d8736a20cc1820dacc566e25c5accd6b41d59`.

## Final Status

completed_with_follow_up

## Product Decision

Stage 1 remains fail-closed: repository facts govern resume, mismatches stop
progress, and no autonomous repair is authorized. Pilot 2 approves test coverage
for two torn projections but does not authorize Stage 2 or Stage 3.

## Engineering Summary

The initial implementation added two torn-projection tests in
`scripts/ai-workflow.node-test.mjs`. A real terminal-transition `EPERM` then
opened review cycle 1: `scripts/ai-workflow.mjs` now removes its own temp after
rename failure and restores the captured event/state pair when either write
fails. Two deterministic fault tests cover both write positions.

## Behavior Changed

No Life OS product behavior changed. Regression coverage protects event-ahead
and state-ahead detection. Engineering workflow failure behavior now cleans
command-owned temps and rolls back only the command's captured pair; it does not
repair an unknown mismatch.

## Files Changed

- `scripts/ai-workflow.node-test.mjs`
- `scripts/ai-workflow.mjs`
- Archived workflow evidence created at terminal archive

Current workflow artifacts are reset to idle templates after archive.
Architecture/13 remains untracked and excluded.

## Tests

- Initial targeted/canonical workflow suite: 15/15 passed.
- Review-cycle-1 targeted/canonical workflow suite: 17/17 passed.
- Vitest: 20 files, 150 tests passed.
- SQLite Rust: 8 tests passed.
- TypeScript typecheck, frontend build, Rust check, UTF-8, whitespace, secret,
  local-link, and Constitution checks passed.

## Repository Verification

Canonical `scripts/verify.ps1` passed for the initial digest
`66088c17260a98d14f46937e28293aee5877df3f0be1626a9a52532addf9612d`.
After cycle-1 correction, it passed again for digest
`bc59d2da148232efc47b4e4bdb4887beecd6c6544696135884105a97f467725d`.
Both results are recorded with exit code 0; only the latter is current.

## Manual Verification

No UI verification applies. Repository-only recovery was independently
performed across sessions and independently re-audited before validation.

## Architecture Updates

None. Architecture/14 remains Implemented with operational reliability still
under pilot evaluation. Architecture/13 remained untracked and excluded; Git
does not independently prove historical byte-for-byte immutability of an
untracked file.

## ADR Updates

None. ADR-0008 remains Accepted and unchanged.

## Documentation Synchronization

No durable documentation update was required. Product Review, Engineering Plan,
Engineering Report, Theory Review, and this archive preserve the evidence.

## Data And Migration Impact

None. No product data, SQLite, schema, migration, retention, import, or export
path changed.

## Provenance And Consent Impact

No product provenance or consent impact. Workflow provenance records exact
checkpoint coordinates, cross-session resume, authorization, verification, and
review events.

## Risks

- Detection still does not provide safe automatic repair.
- One clean repository-only resume does not prove arbitrary crash recovery.
- Temporary mismatch tests do not prove multi-writer or distributed safety.
- Read-only sub-process review improves evidence separation but is not
  independent human assurance.
- Pair rollback can itself fail under persistent filesystem denial; that path
  remains fail closed and preserves error evidence rather than guaranteeing
  recovery.

## Deferred Items

- At least one further bounded Stage 1 pilot before an overall reliability
  conclusion.
- Human-decision stop/resume operational pilot.
- Stage 2 triggers, Stage 3 orchestration, replay, recovery commands, locks,
  leases, deployment, and architecture/13 work.

## Human Decisions

The founder separately authorized Pilot 2A, later planning, and the test-only
implementation phase. No approval was inferred as promotion authority.

## Review Cycles

One corrective cycle. The sprint was intentionally split into capacity-
resilient micro-blocks. Cycle 1 began when terminal completion encountered a
real `EPERM`; live revision 13 stayed consistent, the orphan candidate was
rejected, and correction returned through Product Review and Engineering Plan.

## Workflow Lessons

- A new session recovered the product-review checkpoint without hidden chat
  context and recorded exact coordinates in the event journal.
- A read-only sub-process can independently audit recovery and theory alignment
  while the root remains the sole writer.
- Event/state torn projections are detectable, but the correct Stage 1 response
  remains stop-and-report rather than repair.
- Small durable checkpoints reduce capacity-interruption risk without weakening
  founder gates.
- Per-file atomic replacement is not pair atomicity. Command-local rollback and
  temp cleanup are required even when validation already detects torn state.
- A failed terminal transition must never reuse the orphan candidate event,
  timestamp, hash, or idempotency key.

## Recommended Next Sprint

After Founder diff review and any separately authorized promotion, run Pilot 3
to exercise `human_decision_required`, exact founder resolution evidence, and
repository-only resume. Do not begin Stage 2 or Stage 3.

## Git Status

- Branch: `codex/orchestration-pilot-2-interruption-recovery`
- HEAD: `af2d8736a20cc1820dacc566e25c5accd6b41d59`
- Upstream: none configured
- Staged files: none
- Commit, push, merge: not performed
- Unrelated untracked architecture/13: excluded
