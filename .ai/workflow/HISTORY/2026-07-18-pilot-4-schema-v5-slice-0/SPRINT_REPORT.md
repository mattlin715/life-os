# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-18T10:39:07.9287807Z
- Updated at: 2026-07-18T10:39:07.9287807Z

## Sprint ID

2026-07-18-pilot-4-schema-v5-slice-0

## Mission

Recover the already-existing Founder-authorized Phase 3C Slice 0 test work into
the repository-native Three-Role workflow, correct only CI parity and missing
Slice 0 regressions, validate and theory-review the actual diff, archive the
evidence, and stop before promotion.

## Starting Commit

`b781071fbf726cde69e93cb9cd98c74abdff0ba3` on
`codex/phase-3c-schema-v5-slice-0-contracts`; `develop` and `origin/develop`
matched that commit. Existing Slice 0 work was unstaged and the workflow state
was incorrectly idle.

## Ending Commit Or Working-Tree State

HEAD remains `b781071`. Working-tree digest at verified implementation is
`945577a4b9c9afe8dae82a5f790c1222b84898771d266274dfb6c0521883182c`.
The authorized Slice 0, CI, factual documentation, and Pilot 4 archive changes
remain unstaged for Founder review.

## Final Status

completed_with_follow_up

## Product Decision

Product Review approved the recovery audit with eight conditions. The fixed
schema-v5 SQL remains a test-only candidate contract. Passing tests, completing
Pilot 4, or later promotion cannot authorize production migration.

## Engineering Summary

- Preserved all nine pre-intake Slice 0 files/file groups.
- Aligned GitHub Actions with canonical local verification so all Rust tests run.
- Added two missing Slice 0 DDL-invariant tests: guarded non-current purge with
  metadata survival, and behavioral immutability/guard enforcement for review,
  lifecycle, dependency, and tombstone facts.
- Updated architecture/13 factual evidence to eight integration tests and kept
  the separate production-authorization gate explicit.
- Did not modify `WORKFLOW_EVALUATION.md`; formal Stage 1 evaluation remains a
  separate follow-up sprint.

## Behavior Changed

Only verification and test behavior changed. Local and remote commands select
the complete Rust suite. Production application, database, UI, provider, and
historical-context behavior are unchanged.

## Files Changed

Product/Slice 0 diff:

- `.github/workflows/check.yml`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `scripts/verify.ps1`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/tests/schema_v5_contract.rs`
- `src-tauri/tests/fixtures/schema_v5/contract.json`
- `src-tauri/tests/fixtures/schema_v5/schema_v5.sql`
- `src-tauri/tests/fixtures/schema_v5/v2.sql`
- `src-tauri/tests/fixtures/schema_v5/v3.sql`
- `src-tauri/tests/fixtures/schema_v5/v4.sql`

Workflow diff: current `.ai/workflow/` artifacts during the sprint and terminal
archive `.ai/workflow/HISTORY/2026-07-18-pilot-4-schema-v5-slice-0/` after the
repository-native archive/reset command.

## Tests

- Focused final Slice 0 Rust integration suite: 8 passed, 0 failed.
- Canonical workflow tests: 17 passed.
- Vitest: 20 files, 150 tests passed.
- Existing Rust SQLite tests: 8 passed.
- Slice 0 Rust integration tests: 8 passed.
- TypeScript typecheck, frontend build, Rust check, UTF-8, secrets, Markdown
  links, whitespace, and Constitution diff checks passed.
- Two intermediate failures in the new purge test exposed whitespace loss in a
  Rust continued SQL string; replacement with explicit raw multiline SQL fixed
  the regression without changing the DDL or product boundary.

## Repository Verification

Canonical command passed with exit code 0:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

Verification is bound to HEAD `b781071` and working-tree digest
`945577a4b9c9afe8dae82a5f790c1222b84898771d266274dfb6c0521883182c`.

## Manual Verification

No UI/manual runtime verification applies because no production or UI behavior
changed. Founder diff review is required. Remote GitHub Actions was not run
because push is prohibited.

## Architecture Updates

architecture/13 remains `Founder-approved`, advances factually to version 0.4,
and distinguishes current unpromoted Slice 0 evidence from production migration
authority. No design decision changed.

## ADR Updates

None. ADR-0007, ADR-0008, ADR-0009, and ADR-0011 remain unchanged.

## Documentation Synchronization

Only architecture/13 factual evidence and workflow artifacts changed. No new
design document and no `WORKFLOW_EVALUATION.md` change were made.

## Data And Migration Impact

No production data or migration impact. Candidate SQL ran only in memory against
synthetic fixtures. Production `SCHEMA_VERSION` and `user_version` remain v4.

## Provenance And Consent Impact

Synthetic immutability and ADR-0009 deletion behavior are regression-tested.
No real provenance, consent, packet, provider call, or transmission occurred.

## Risks

- Remote CI execution remains unobserved until an authorized future push.
- In-memory contracts do not prove production backup, restore, restart,
  backfill, reconciliation, performance, or user-data cutover safety.
- Later code must not infer production authority from this completed archive.
- Pilot 4 provides operational evidence but does not complete or approve the
  formal Stage 1 reliability evaluation.

## Deferred Items

- Founder diff review and separate Slice 0 promotion authorization.
- Post-promotion Founder checkpoint for any Slice 1 or production-migration
  authority.
- Live remote CI observation after an authorized push.
- Formal Stage 1 evaluation in a separate sprint.
- All architecture/13 Slices 1-6 and Phase 4 implementation.

## Human Decisions

The founder explicitly authorized recovery of the in-flight Slice 0 work into
Pilot 4, the CI correction, missing Slice 0 regression coverage, factual
documentation synchronization, and archive/reset. The founder expressly
withheld stage, commit, push, merge, production migration, later slices, and
deployment. No new decision was required inside this boundary.

## Review Cycles

Zero workflow revision cycles. The two focused-test corrections occurred inside
the initial implementation phase before handoff.

## Workflow Lessons

1. Recovery intake must record pre-existing implementation rather than
   fabricate plan-before-code chronology.
2. Artifact reconciliation remains fail-closed: an initial attempt to record
   the ready Mission was rejected because `PRODUCT_REVIEW.md` had already been
   changed to `approved_with_conditions` while the state projection still said
   pending. State/events did not advance. Restoring only the Markdown status,
   then recording Mission, transitioning, and recording Product Review in
   sequence succeeded without control-plane repair or replay.
3. Canonical local verification can expand while remote CI silently retains an
   old test filter; parity must be reviewed as an explicit repository fact.
4. Operational Pilot evidence is useful but must not self-approve the Stage 1
   evaluation or displace product work.

## Recommended Next Sprint

First obtain Founder diff review, then a separate exact-file Promotion
Authorization Gate for this Slice 0 and Pilot 4 archive. After promotion, return
to product-first execution with a new founder decision for the narrow next
architecture/13 slice; do not restart Harness evaluation work unless separately
requested.

## Git Status

- Branch: `codex/phase-3c-schema-v5-slice-0-contracts`
- HEAD: `b781071fbf726cde69e93cb9cd98c74abdff0ba3`
- Upstream: none configured
- Staged files: none
- Commit/push/merge/deployment: none
- Working tree: intentionally contains the authorized unstaged diff and Pilot
  4 evidence for Founder review
