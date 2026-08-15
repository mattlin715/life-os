# Engineering Plan

Status: approved

- Sprint ID: 2026-08-16-founder-v5-context-recovery-runtime-correction-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Working-tree digest reviewed: f6c186d646db0c5ffe1503bb3f4606f2d50fbf8571ae1cd3b965a327fc53067a
- Created at: 2026-08-16T12:40:00+09:00
- Updated at: 2026-08-16T12:40:00+09:00

## Approved Product Boundary

Product Review is approved with the condition that the change remains the exact
Candidate runtime discriminator, real-facade regression, and local three-language
failure disclosure. No action, lifecycle, schema, or authority expansion.

## Existing Implementation Understanding

`schema_v5_runtime::save_artifacts` diffs one artifact category and delegates to
promoted private writers. The recovery branch alone queries
`artifact_revision(..., "context_recovery")`; storage and writer contracts use
`recovery_turn`. App commit-first mutations restore durable state after failure,
but only a top-level `storageError` is visible.

## Affected Modules

Anticipated implementation allowlist:

- `src-tauri/src/schema_v5_runtime.rs`
- `src/app/App.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `src/app/ContextRecoveryPanel.tsx` (new)
- `src/app/ContextRecoveryPanel.test.tsx` (new)
- `scripts/founder-dogfood-package.mjs` (Cycle 1 allowlist synchronization)
- `docs/architecture/17_Desktop_Schema_v5_Founder_Dogfood_Activation_Candidate_R1.md`
- repository-required current workflow artifacts and terminal archive

Package artifacts remain ignored. Cycle 1 adds only the three authorized new
UI/test paths to the existing Candidate-successor package allowlist; package
identity, build, manifest, and verification behavior remain unchanged.

## Proposed Design

1. Replace the lookup discriminator with the existing persisted constant value
   `recovery_turn`.
2. Extend the runtime test fixture to create a suggested recovery turn and then
   save a user answer through `save_artifacts`; verify projection, head, exact
   two-revision lineage, and activated-v5 reconciliation.
3. Extract the existing Context Recovery region to a presentation component.
4. Track a session-only per-Experience recovery mutation failure flag. Save and
   Skip clear it before the explicit attempt, set it only on failed outcome, and
   clear it on successful commit.
5. Render localized `role="alert"` disclosure inside the region. Preserve the
   existing general diagnostic error and durable-state reconciliation.

## Alternatives Considered

Changing the persisted discriminator was rejected because `recovery_turn` is
already the governed authority. Adding automatic retry was rejected. Testing
only the private writer was rejected because it would miss the facade defect.
A full App refactor was rejected as unnecessary.

## Data Lifecycle Impact

No new data. Successful answers retain the existing append-only provenance and
prompt dependency. Failure writes nothing and automatic retry remains absent.

## SQLite Or Migration Impact

None. No DDL, manifest, receipt, migration, startup maximum, or
`SCHEMA_VERSION` change.

## Provenance Impact

None beyond proving the existing writer is reached with the exact current
revision.

## Historical Context Impact

None. Recovery turns remain excluded.

## Consent Impact

None.

## Provider Transmission Impact

None.

## Import And Export Impact

None.

## Test Strategy

- Rust: real facade create-suggestion then first-answer regression and exact
  durable assertions.
- UI: Context Recovery region shows localized failure alert and retains actions.
- i18n: equivalent calm copy in English, Traditional Chinese, and Japanese.
- Existing Vitest/Rust suites guard ordinary behavior.

## Repository Verification Strategy

Run focused Vitest, focused Founder-feature Rust tests, Clippy with warnings
denied, then `powershell -NoProfile -ExecutionPolicy Bypass -File
.\scripts\verify.ps1`.

## Manual UI Verification

Founder-owned: install only the new unsigned isolated Candidate package, reopen
the preserved disposable fresh-v5 profile, and repeat only Step 11D-3. No
automated launch or profile mutation.

## Rollback Or Recovery Strategy

Before promotion, discard only the corrective diff if rejected. Do not mutate
or restore any profile. No automatic retry/replay/repair is introduced.

## Documentation Impact

Factual architecture/17 note only: parent sprint incomplete, exact defect and
separate correction evidence. No new design document or ADR.

## ADR Impact

No new ADR and no ADR status change.

## Risk Level

low: one exact discriminator and bounded UI disclosure; mitigated by real-facade
and focused component tests plus canonical verification.

## Escalation Decision

No unresolved consequential decision. Proceed within Founder Decision 004 and
stop before manual review or Git promotion.
