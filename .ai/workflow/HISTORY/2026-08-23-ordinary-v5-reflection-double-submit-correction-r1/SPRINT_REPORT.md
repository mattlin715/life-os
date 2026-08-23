# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-22T20:32:00.000Z
- Updated at: 2026-08-22T20:32:00.000Z

## Sprint ID

2026-08-23-ordinary-v5-reflection-double-submit-correction-r1

## Mission

Correct the duplicate Reflection answer submission defect observed during the disposable ordinary schema-v5 Founder review, without changing schema, migration, provider, consent, or lifecycle authority.

## Starting Commit

44ec6d56d645829488aa73d0b92bcf72b19487f4 on `codex/desktop-schema-v5-ordinary-production-activation-r1`.

## Ending Commit Or Working-Tree State

The repository remains at the starting commit with an unstaged working tree. This bounded correction adds no commit and does not stage, push, merge, deploy, distribute, or release anything.

## Final Status

completed_with_follow_up

Implementation, automated verification, and Theory Alignment Review are complete. Founder manual Step 8D-2R remains required against a newly built ignored unsigned disposable review installer.

## Product Decision

The first explicit Save Answer remains the only durable intent. While that save is in flight, another activation must be refused in the UI. If the current normalized draft already equals the durable answered response, the operation is an exact UI no-op rather than an attempted correction.

## Engineering Summary

- Added a per-Experience/per-prompt synchronous save-flight guard.
- Disabled the exact Save Answer control and exposed `aria-busy` during persistence.
- Added an idempotent already-durable answer path while preserving append-only genuine corrections.
- Added focused collision, release, idempotency, and genuine-correction tests.
- Synchronized architecture/18 with the truthful manual defect and correction evidence.
- Synchronized the existing Founder package contract with the two exact new correction paths and the two truthful workflow archive prefixes; no generic allowlist was added.

## Behavior Changed

A rapid second Save Answer activation for the same prompt is ignored while the first request is in flight. After durable state is reconciled, resubmitting identical normalized text performs no mutation. Editing to different text still invokes the existing append-only correction behavior.

## Files Changed

Correction implementation and factual synchronization are bounded to:

- `src/app/App.tsx`
- `src/app/reflectionDraft.ts`
- `src/app/reflectionDraft.test.ts`
- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `scripts/founder-dogfood-package.mjs`
- repository-required workflow artifacts under `.ai/workflow/`

Parent ordinary-activation working-tree changes remain preserved and unstaged.

## Tests

- Focused Reflection draft tests: 1 file / 7 tests passed.
- Founder package contract tests: 8 / 8 passed.
- Clippy: `cargo clippy --all-targets -- -D warnings` passed.
- Canonical Vitest: 46 files / 349 tests passed.
- Rust library: 200 tests passed.
- Backup/restore integration: 12 tests passed.
- Schema contract: 8 tests passed.
- Legacy refusal: 33 tests passed.
- Ordinary activation: 6 tests passed.
- Founder activation: 6 tests passed.
- Founder runtime: 4 tests passed.

## Repository Verification

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1` passed after one truthful contract correction. The first run found that `scripts/founder-dogfood-package.mjs` did not yet recognize the already archived disclosure correction and the two exact Reflection draft files. The package allowlist was updated only for those named successor paths, then the full canonical verification passed, including workflow, package, TypeScript, frontend build, Rust check, UTF-8, whitespace, secrets, Markdown links, and Constitution checks.

## Manual Verification

The Founder observed that the first Save Answer succeeded but the button remained dark green and actionable; clicking it again produced `reflection_response_unchanged`. The disposable profile must not be mutated automatically. Step 8D-2R remains pending on a new ignored unsigned review package and the existing disposable schema-v5 profile.

## Architecture Updates

architecture/18 records the observed duplicate-submit defect, the bounded UI/idempotency correction, automated evidence, and the pending Founder retest without claiming manual acceptance.

## ADR Updates

None. ADR status and authority are unchanged.

## Documentation Synchronization

Only the factual ordinary activation review document was updated by this correction.

## Data And Migration Impact

None. No database, profile, schema, DDL, migration, backup, restore, or operation evidence was accessed or changed by automation. Production activation behavior remains the parent sprint's unpromoted working tree.

## Provenance And Consent Impact

No provider, ContextPacket, consent, transmission, or provenance policy changed. The correction prevents duplicate UI activation from attempting to create a false second user action while preserving genuine append-only user corrections.

## Risks

- Packaged Windows behavior still requires Founder observation.
- The retest begins from an already answered durable Reflection, so it verifies a genuine correction followed by a rapid duplicate activation rather than recreating the original first-answer state.
- Parent ordinary schema-v5 activation remains unpromoted and unauthorized for real profiles or distribution.

## Deferred Items

- Founder Step 8D-2R manual retest.
- Remaining disposable Manual Phase A matrix.
- Any real Founder-profile Phase B review, which requires a separate explicit authorization after Phase A and final diff review.

## Human Decisions

No new consequential policy decision was required. This was bounded correction cycle 2 under the existing ordinary activation goal and its three-cycle limit.

## Review Cycles

Theory Review Cycle 0 approved the correction with the manual retest as follow-up. No implementation revision cycle was required after theory review.

## Workflow Lessons

An append-only Rust refusal is correct but must not be surfaced as a storage failure when the UI itself permitted an accidental duplicate activation. UI in-flight state and durable-state reconciliation must both preserve one explicit intent as one durable effect.

## Recommended Next Sprint

Do not start a new sprint yet. Build the ignored unsigned disposable ordinary review installer, complete Step 8D-2R, and then continue the existing Manual Phase A matrix one bounded Founder step at a time.

## Git Status

Branch: `codex/desktop-schema-v5-ordinary-production-activation-r1`.

HEAD: `44ec6d56d645829488aa73d0b92bcf72b19487f4`.

No files are staged. No commit, push, merge, PR, deployment, distribution, or release occurred.
