# Engineering Plan

Status: approved

- Sprint ID: 2026-11-03-prepared-recovery-platform-boundary-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 0b3987d6b34557939a8c557146d95a2893346a12
- Working-tree digest reviewed: 5e1374579abd2292652cf8b997e8aebed69b9cd335a225fd96c2b6c75262301d
- Created at: 2026/11/03
- Updated at: 2026/11/03

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

PRODUCT_REVIEW approved, AC1 centralized Windows capability; AC2 public pre-path/I-O and private mutation refusal; AC3 Windows behavior preserved; AC4 pure classifier and blocked/restore routing preserved; AC5 original30 named destinations and complete synthetic-root snapshots, held real SQLite rollback read/write/WAL connections; AC6三語 unsupported UI/close; AC7 canonical Windows and actual Ubuntu four checks; AC8 independent commit and stacked draft only.

## Existing Implementation Understanding

Two quarantine executors acquire guard first; R2B transaction independently rechecks; public identity/path gates lack OS check. Keep pure classifiers and startup R2B blocked helper unchanged.

## Affected Modules

Only recovery module/tests, panel tests with minimal recovery-specific localization if needed, architecture18/19 and current workflow. No Windows sharing/lock/move, R2B SQL core, DDL, ordinary migration/resume/restore, Reflection, icon, dependencies/features, CI, Constitution, ADR status or old archive changes.

## Proposed Design

Single fixed reason and native capability gate; public inspect refused result; execute/delete refuse before paths; every private executor/delete guards first. Non-Windows exclusive-open rejects without File::open. Preserve read-only structural classifiers and summary. Minimal localized reason-specific display if required.

## Alternatives Considered

B needs separate portability authority/design; C hides a production protection gap. A already approved. No Unix lock.

## Data Lifecycle Impact

No schema, data retention, backup or provenance changes; Windows behavior retained, non-Windows recovery/delete unavailable.

## SQLite Or Migration Impact

No SQL/DDL core changes. Synthetic independent connections hold rollback read/write or WAL transactions during refusal snapshots.

## Provenance Impact

Exact dated Founder authority; no product provenance changes.

## Historical Context Impact

None: no context selection or content changed.

## Consent Impact

No migration consent reuse, automatic execution or retry.

## Provider Transmission Impact

None.

## Import And Export Impact

None.

## Test Strategy

First source-order/UI regressions RED on baseline. Native non-Windows complete root snapshots with valid claims, complex state, repeated calls, all private executors and delete. Preserve pure original1-24 everywhere and Windows25-30; explicitly split success/partial-failure mutation tests to Windows; keep direct SQL transaction recheck cross-platform. Source contract proves public gating pre-path and private gating first; no test override in production API.

## Repository Verification Strategy

Focused Vitest/Rust, bounded source review, unchanged canonical Windows verifier. Commit first if clean-source package guard requires; no target/dist/icon outputs copied. Push only after PASS, stacked draft base icon branch; actual Ubuntu tests/check/legacy and frontend, record merge SHA.

## Manual UI Verification

not_run, unauthorized. Frontend synthetic tests only; Founder owns later UI gate.

## Rollback Or Recovery Strategy

No recovery execution or cleanup. Preserve source trees/logs; no reset/amend/rebase/force.

## Documentation Impact

Dated decision and original30 mapping in architecture19, successor sync18; workflow formal authority/evidence.

## ADR Impact

No new ADR required for explicitly scoped capability application; ADR-0006 unchanged, no status change.

## Risk Level

Medium: safety boundary, reduced by pre-I/O refusal and preserved Windows/SQL core.

## Escalation Decision

Proceed; stop on new independent CI failure or scope conflict.
