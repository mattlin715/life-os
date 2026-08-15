# Sprint Report

Status: completed

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-16T15:40:00+09:00
- Updated at: 2026-08-16T15:40:00+09:00

## Sprint ID

2026-08-16-founder-v5-context-recovery-runtime-correction-r1

## Mission

Correct only the isolated Founder schema-v5 Context Recovery runtime artifact
kind and local mutation-error disclosure, verify the real typed facade, build a
new unsigned package, and repeat affected manual Step 11D-3.

## Starting Commit

`9a226f7081aabc071571f4a745e1343dbdb7d927` on
`codex/desktop-schema-v5-founder-dogfood-activation-r1`.

## Ending Commit Or Working-Tree State

HEAD is unchanged at `9a226f7081aabc071571f4a745e1343dbdb7d927`.
All Candidate and correction work remains unstaged. No commit, push, merge, PR,
distribution, deployment, or release occurred.

## Final Status

completed

## Product Decision

Founder Decision
`FOUNDER-SCHEMA-V5-CANDIDATE-R1-CONTEXT-RECOVERY-FOLLOWUP-004` authorized the
separate bounded correction. Founder manual decision
`FOUNDER-V5-CONTEXT-RECOVERY-R1-MANUAL-001` selected Option A with exact
response `Manual now`.

## Engineering Summary

The real schema-v5 facade now resolves Context Recovery using the governed
`recovery_turn` kind. It strictly validates user response provenance and
canonicalizes only absent optional provider fields to the promoted writer's
null representation. The visible Context Recovery area now shows a calm,
session-only localized mutation error after an explicit failed Save or Skip.

## Behavior Changed

Explicit Context Recovery Save/Skip can reach the existing private writer and
reconcile the exact current head. Failures are visible beside the action.
There is no automatic retry, repair, replay, profile action, or new product
action.

## Files Changed

Correction-specific product and factual paths:

- `src-tauri/src/schema_v5_runtime.rs`
- `src/app/App.tsx`
- `src/app/ContextRecoveryPanel.tsx`
- `src/app/ContextRecoveryPanel.test.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `scripts/founder-dogfood-package.mjs`
- `docs/architecture/17_Desktop_Schema_v5_Founder_Dogfood_Activation_Candidate_R1.md`
- repository-required current workflow artifacts and the truthful archived
  incomplete parent sprint.

The broader unstaged Candidate working tree predates and remains outside this
follow-up's change authority.

## Tests

Focused runtime tests: 4/4. Focused Vitest: 2 files / 20 tests. Package tests:
8 schema-v4 Founder package tests and 1 Candidate contract test. Clippy with
warnings denied and TypeScript typecheck passed.

## Repository Verification

Canonical `scripts/verify.ps1` passed: 17 workflow tests, 46 Vitest files / 329
tests, 193 Rust library tests, 12 backup/restore integration tests, 8 schema
contract tests, 6 Founder activation tests, 4 Founder runtime tests, frontend
build, Rust check, UTF-8, whitespace, secret, Markdown-link, and Constitution
checks.

## Manual Verification

Passed on the isolated disposable Founder profile using the exact new unsigned
package. The Founder verified package SHA-256, installed without deleting app
data, confirmed preserved profile state, saved one Context Recovery response,
observed Pattern unlock and one candidate, closed and restarted, and confirmed
that the response and candidate persisted without mutation error or duplicate.
A post-close read-only check showed schema v5, integrity `ok`, zero foreign-key
errors, no SQLite sidecars, one recovery head at answered/eligible, two recovery
revisions, and one pending Pattern head.

## Architecture Updates

Architecture/17 v0.7 records the truthful incomplete parent closeout, exact
runtime defect, bounded correction, package evidence, and unchanged authority
fences.

## ADR Updates

None. ADR-0009 and ADR-0011 remain unchanged.

## Documentation Synchronization

Only factual Candidate architecture and workflow evidence changed.

## Data And Migration Impact

No schema, DDL, migration policy, ordinary profile, or real-user operation was
changed. The Founder explicitly saved one response and generated one candidate
inside the isolated disposable profile during manual review.

## Provenance And Consent Impact

The required response provenance is validated exactly; only absent optional fields
are normalized to existing null representation. Context Recovery remains
historically ineligible. Provider, ContextPacket, and consent behavior are
unchanged.

## Risks

This is still an unsigned, isolated Founder-only Candidate. It does not prove
ordinary-profile schema-v5 readiness, real-user migration safety, distribution,
deployment, or release readiness.

## Deferred Items

Remaining Candidate R1 Founder diff review and any separately authorized
promotion. Ordinary-profile schema v5, real-user migration, Phase 4, Android,
distribution, deployment, and release remain unauthorized.

## Human Decisions

Decision 004 Option A authorized the correction sprint. Manual Decision 001
Option A authorized only the one-step packaged retest; it did not expand product
or promotion authority.

## Review Cycles

Cycle 0 fixed the artifact-kind lookup and strict optional-null provenance
adapter mismatch exposed by the real-facade test. Cycle 1 synchronized the exact
Founder package active-successor allowlist after canonical verification failed
closed. Final canonical and manual evidence passed.

## Workflow Lessons

A real facade regression is necessary even when a private writer is already
well tested: integration discriminators and representation adapters can fail
before or after the writer boundary. Package allowlists must be synchronized
when authorized product paths are extracted.

## Recommended Next Sprint

None automatically. Stop for Founder diff review of this correction together
with the still-unstaged Candidate R1 package. Promotion requires separate exact
Founder authorization.

## Git Status

Branch `codex/desktop-schema-v5-founder-dogfood-activation-r1`; HEAD
`9a226f7081aabc071571f4a745e1343dbdb7d927`; no staged files; no upstream;
working tree intentionally contains the unpromoted Candidate implementation,
this bounded correction, and workflow archives.
