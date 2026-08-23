# Engineering Report

Status: completed

- Sprint ID: 2026-08-23-ordinary-v5-reflection-double-submit-correction-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest implemented: recorded by workflow event at completion
- Created at: 2026-08-22T20:24:00.000Z
- Updated at: 2026-08-22T20:24:00.000Z

## Implementation Summary

Added an exact per-Experience/per-prompt synchronous save guard, immediate rendered pending state, and durable-response equality handling. The Rust unchanged-response refusal remains untouched and genuine changed responses still use the existing append-only correction path.

## Existing System Areas Inspected

- `src/app/App.tsx`
- `src/app/reflectionDraft.ts`
- `src/shared/storage/artifactMutation.ts`
- `src/shared/storage/sqlite/founderSchemaV5LocalEvidenceStore.ts`
- `src-tauri/src/schema_v5_runtime.rs`
- `src-tauri/src/schema_v5_reflection_write.rs`

## Files Added

None outside repository-required workflow archive artifacts.

## Files Modified

- `src/app/App.tsx`
- `src/app/reflectionDraft.ts`
- `src/app/reflectionDraft.test.ts`
- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `scripts/founder-dogfood-package.mjs`

## Files Deleted

None.

## Behavior Changed

The first Save Answer activation synchronously marks the exact prompt in flight and disables its control. A second activation is ignored. If durable state already contains the normalized submitted answer, the queued mutation remains an exact no-op instead of requesting an unchanged correction. A genuinely changed response remains a normal correction.

## Data Model Impact

None.

## Migration Impact

None.

## Provenance Impact

Duplicate activation no longer risks presenting an invented correction attempt. Prompt and user-response provenance are unchanged.

## Historical Context Impact

None.

## Consent Impact

None.

## Provider Transmission Impact

None.

## Tests Added

Three focused assertions cover duplicate same-prompt refusal, delimiter-safe neighboring IDs and release, equal durable-answer idempotency, and genuine changed-answer correction.

## Tests Executed

- `pnpm exec vitest run src/app/reflectionDraft.test.ts`: passed, 1 file / 7 tests.
- `pnpm run typecheck`: passed.

## Verification Results

Focused tests, TypeScript typecheck, Clippy with warnings denied, the corrected Founder package contract, and canonical repository verification passed. Canonical totals include 17 workflow tests, 8 Founder package tests, 1 Candidate package test, 2 ordinary package tests, 46 Vitest files / 349 tests, 200 Rust library tests, 12 backup/restore integration tests, 8 schema-contract tests, 33 legacy-refusal tests, 6 ordinary activation tests, 6 Founder activation tests, and 4 Founder runtime tests.

## Manual Verification Required

Founder Step 8D-2R remains required against the same disposable schema-v5 profile using a newly hashed ignored review installer. No manual pass is inferred.

## Documentation Updates

Architecture/18 records the truthful first-save success, duplicate activation refusal, correction boundary, and pending retest.

## ADR Impact

No ADR change.

## Deviations From Plan

Canonical verification initially failed because the older Founder package source allowlist did not yet recognize the already archived disclosure correction or the two exact `reflectionDraft` paths. The allowlist was factually synchronized with those named paths and this sprint's future archive prefix; its rejection behavior and package authority were not broadened generically. The focused package suite and full canonical verification then passed.

## Known Limitations

The correction does not change or weaken Rust's unchanged-response refusal. It does not inspect or mutate any real profile.

## Remaining Risks

Packaged Windows event timing requires the bounded Step 8D-2R retest. Any executable lock must be resolved only by normal app closure.

## Git State

Branch `codex/desktop-schema-v5-ordinary-production-activation-r1`, HEAD `44ec6d56d645829488aa73d0b92bcf72b19487f4`, unstaged parent implementation plus this bounded correction, no staged files, no commit/push/merge.

## Engineer Completion Status

completed
