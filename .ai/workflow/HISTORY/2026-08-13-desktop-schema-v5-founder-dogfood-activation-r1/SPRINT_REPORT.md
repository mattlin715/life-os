# Sprint Report

Status: failed

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-16T12:20:00+09:00
- Updated at: 2026-08-16T12:20:00+09:00

## Sprint ID

2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1

## Mission

Implement and evaluate an isolated Windows Founder-only schema-v5 activation
Candidate without changing the ordinary schema-v4 application or authorizing
real-user migration, distribution, deployment, or release.

## Starting Commit

`9a226f7081aabc071571f4a745e1343dbdb7d927`

## Ending Commit Or Working-Tree State

The same repository HEAD with unstaged implementation, documentation, tests,
and workflow evidence. No staged files, commit, push, merge, PR, deployment,
distribution, or release occurred.

## Final Status

failed: the bounded three-cycle Candidate sprint produced substantial verified
and Founder-reviewed evidence, but a currently reachable Context Recovery answer
path remains defective and the Harness forbids a fourth revision attempt.

## Product Decision

Founder Decision
`FOUNDER-SCHEMA-V5-CANDIDATE-R1-CONTEXT-RECOVERY-FOLLOWUP-004` Option A accepts
truthful incomplete closeout and authorizes a separate minimal corrective sprint.

## Engineering Summary

Implemented the isolated Candidate identity, explicit exact-v4 migration,
verified backup/restart evidence, exact-v5 initialization, typed runtime routes,
three-language disclosure, and private unsigned package. Three bounded
correction cycles addressed path identity, runtime-v4 schema-manifest
compatibility, and Experience writer manifest parity. Continued manual review
then found a separate Context Recovery artifact-kind lookup mismatch.

## Behavior Changed

Only the isolated Founder Candidate code path changed. The ordinary application
remains schema v4. The unresolved Candidate defect prevents saving a reachable
Context Recovery answer because the runtime lookup uses `context_recovery`
instead of governed `recovery_turn`.

## Files Changed

Candidate-only Rust, TypeScript, UI, package scripts/configuration, focused
tests, architecture/17 and factual Book One synchronization, plus repository
workflow artifacts. Exact paths remain visible in the archived Engineering
Report and Git working tree.

## Tests

Last canonical verification before the newly observed manual defect passed:
17 workflow tests, 8 Founder package tests, 1 Candidate package test, 45 Vitest
files/323 tests, 191 ordinary-feature Rust tests, 12 backup/restore tests, 8
schema-contract tests, 6 Founder activation tests, 2 runtime-focused tests,
typecheck, frontend build, Rust checks, Clippy with warnings denied, hygiene,
and Constitution checks.

## Repository Verification

Passed before Cycle 3 packaging. It does not override the later Founder manual
failure. The Context Recovery defect was confirmed by read-only database and
source inspection; no corrective implementation was attempted after discovery.

## Manual Verification

Founder evidence passed exact-v4 migration/restart, explicit restore and repeat
migration, fresh-v5 typed Experience, Evidence candidate/correction/confirmation,
and Reflection prompt/answer/correction. Context Recovery prompt creation
passed. Saving its first response failed visibly as an inert action. After
normal close, schema v5 remained valid with no partial response, no sidecars,
valid foreign keys, and integrity `ok`.

## Architecture Updates

Architecture/17 documents the isolated Candidate. Architecture/13/15/16 and the
Founder package runbook contain factual boundary synchronization. No new
constitutional or ADR authority was created.

## ADR Updates

None. ADR-0009 and ADR-0011 remain authoritative and unchanged.

## Documentation Synchronization

Completed for the bounded Candidate evidence and known limitations. The
separate corrective sprint must add only factual follow-up evidence.

## Data And Migration Impact

Only disposable `com.lifeos.founderdogfood` data was manually exercised. The
ordinary `com.lifeos.app` database remained byte-identical. Production
`SCHEMA_VERSION` and startup maximum remain 4. No real-user migration occurred.

## Provenance And Consent Impact

No provider, ContextPacket, consent-policy, historical eligibility, or Phase 4
change. Context Recovery remains current-task-scoped and historically ineligible.

## Risks

Context Recovery answer save is unusable in the Candidate until corrected.
Remaining Pattern and package manual checks are incomplete. Candidate evidence
does not authorize ordinary-profile schema v5, real-user migration,
distribution, deployment, or release.

## Deferred Items

The exact runtime discriminator correction, local three-language error
visibility, affected manual Step 11D-3 retest, remaining manual matrix, Founder
diff review, and any later promotion.

## Human Decisions

The Founder resolved all prior bounded Candidate decisions and selected Option A
for Decision 004: close this sprint truthfully and continue only in one separate
minimal corrective sprint.

## Review Cycles

Three of three consumed. A fourth correction was not attempted.

## Workflow Lessons

A runtime facade must use the persisted artifact discriminator, not a conceptual
module label. Focused tests must traverse the real facade for every reachable
artifact action, and mutation failures must be visible at the point of action.

## Recommended Next Sprint

`2026-08-16-founder-v5-context-recovery-runtime-correction-r1`, limited exactly
to the Founder-authorized discriminator, facade regression, local error
visibility, verification, unsigned package, and affected manual retest.

## Git Status

Branch `codex/desktop-schema-v5-founder-dogfood-activation-r1`; HEAD
`9a226f7081aabc071571f4a745e1343dbdb7d927`; unstaged work only; no upstream;
no stage, commit, push, merge, PR, deployment, distribution, or release.
