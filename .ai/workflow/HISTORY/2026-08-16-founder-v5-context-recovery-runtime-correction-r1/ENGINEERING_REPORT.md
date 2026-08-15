# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-16-founder-v5-context-recovery-runtime-correction-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Created at: 2026-08-16T13:05:00+09:00
- Updated at: 2026-08-16T13:05:00+09:00

## Implementation Summary

Corrected the isolated Founder-v5 runtime to resolve Context Recovery heads with
the governed persisted kind `recovery_turn`. Added exact validation and
canonical postcondition representation for user response provenance, a
real-facade create-then-answer regression, and a session-only three-language
error alert inside the visible Context Recovery region.

## Existing System Areas Inspected

The real `save_artifacts` facade, promoted Context Recovery writer/projection,
Founder Tauri command and TypeScript store adapter, commit-first mutation runner,
App Context Recovery controls, domain helper, i18n, architecture/17, and the
archived incomplete parent sprint.

## Files Added

- `src/app/ContextRecoveryPanel.tsx`
- `src/app/ContextRecoveryPanel.test.tsx`
- parent sprint archive under
  `.ai/workflow/HISTORY/2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1/`

## Files Modified

- `src-tauri/src/schema_v5_runtime.rs`
- `src/app/App.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `docs/architecture/17_Desktop_Schema_v5_Founder_Dogfood_Activation_Candidate_R1.md`
- `scripts/founder-dogfood-package.mjs` (Cycle 1 allowlist synchronization)
- current repository workflow artifacts

## Files Deleted

none

## Behavior Changed

A Founder-v5 Context Recovery Save or Skip can now find the exact current
`recovery_turn` revision. On failed Save/Skip, the active Context Recovery
region shows a calm localized alert and retains the durable state restored by
the existing commit-first runner. Success clears the alert. No action is invoked
automatically.

## Data Model Impact

None. Existing schema-v5 objects, heads, revisions, dependencies, projections,
and receipts are unchanged. Production `SCHEMA_VERSION` remains 4.

## Migration Impact

None.

## Provenance Impact

Incoming user response provenance must exactly name the current source, current
recovery artifact, `user` origin, and action timestamp; optional provider/model/
Harness/prompt fields may be absent or null but never non-null. The runtime
normalizes only absence to the writer's existing exact null representation
before strict postcondition comparison. Contradiction fails before the writer.

## Historical Context Impact

None. Context Recovery remains historically ineligible.

## Consent Impact

None.

## Provider Transmission Impact

None.

## Tests Added

- Real-facade disposable exact-v5 suggestion followed by first saved answer,
  exact head eligibility, two-revision lineage, projection, and full runtime
  reconciliation.
- Contradictory response provenance refusal with unchanged suggested state and
  one retained revision.
- Context Recovery component localized in-region alert, explicit action, and
  success/no-alert tests.
- Three-language persistence-failure copy assertions.

## Tests Executed

- Focused Founder-feature runtime tests: 4/4 passed.
- Focused Vitest: 2 files, 20 tests passed.
- TypeScript typecheck: passed.
- Clippy all targets with `founder-schema-v5` and warnings denied: passed.
- Canonical repository verification: passed. It included 17 workflow tests,
  8 schema-v4 Founder package tests, 1 Candidate package contract test,
  46 Vitest files / 329 tests, 193 Rust library tests, 12 backup/restore
  integration tests, 8 schema-contract tests, 6 Founder activation tests, and
  4 focused Founder runtime tests, plus TypeScript typecheck, frontend build,
  Rust check, UTF-8, whitespace, secret, Markdown-link, and Constitution checks.
- Unsigned isolated Candidate package build and manifest verification: passed.
  The installer is 5,681,429 bytes with SHA-256
  `1585ae219ee041d36dcca9827117ea1226c8444f96d422297ebfd81f1007a285`.

## Verification Results

Focused and canonical implementation evidence pass. The corrected unsigned
package source, binary contract, and content-free manifest were verified. The
package was not installed, launched, distributed, deployed, or released.

## Manual Verification Required

Founder must install only the newly built unsigned isolated Candidate package
and repeat affected Step 11D-3 on the preserved disposable profile. Automation
did not open or mutate the Founder or ordinary profile.

## Documentation Updates

Architecture/17 v0.6 records the incomplete parent closeout, exact defect,
separate Founder authority, correction, strict provenance boundary, and local
error disclosure.

## ADR Impact

None. ADR-0009 and ADR-0011 remain unchanged.

## Deviations From Plan

The first real-facade regression proved a second part of the same adapter defect:
after reaching the writer, strict postcondition comparison differed because the
frontend user provenance omits optional provider fields while the promoted
writer emits their fixed null representation. The bounded correction validates
all incoming user provenance and canonicalizes only absent optional fields. It
does not weaken equality or change the writer/schema contract.

Revision Cycle 1: the first canonical run passed workflow validation but the
existing schema-v4 Founder package contract rejected the newly authorized
Context Recovery component and focused tests as paths outside its known active
successor allowlist. The correction adds exactly those three paths; it changes
no package identity, build, manifest, schema, or product behavior.

## Known Limitations

Unsigned isolated Candidate only. No automatic retry, real-user evidence,
ordinary-profile v5, Android, distribution, deployment, or release.

## Remaining Risks

Founder Step 11D-3 retest remains. The preserved disposable profile has not
been exercised by this sprint.

## Git State

Branch `codex/desktop-schema-v5-founder-dogfood-activation-r1`; HEAD
`9a226f7081aabc071571f4a745e1343dbdb7d927`; unstaged work only; no stage,
commit, push, merge, PR, distribution, deployment, or release.

## Engineer Completion Status

completed_with_follow_up: bounded implementation, canonical verification, and
unsigned package build pass; Founder manual retest remains.
