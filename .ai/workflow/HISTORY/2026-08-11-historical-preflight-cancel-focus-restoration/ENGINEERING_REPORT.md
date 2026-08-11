# Engineering Report

Status: completed_with_follow_up

## Sprint ID

2026-08-11-historical-preflight-cancel-focus-restoration

## Implementation Summary

Cancel and adjust-source actions now close only their matching preflight and restore focus plus block-start viewport position to the same Experience's historical-context panel after React renders.

## Existing System Areas Inspected

App historical panel state, consent preflight callbacks, saved-date controls, and existing post-render navigation helpers.

## Files Added

none

## Files Modified

`src/app/App.tsx`, `src/app/journeyNavigation.ts`, `src/app/journeyNavigation.test.ts`, and `src/app/HistoricalConsentPreflight.test.tsx`.

## Files Deleted

none

## Behavior Changed

Cancel and adjust-source return the user to the exact local-history panel instead of leaving the viewport over a later Experience.

## Data Model Impact

none

## Migration Impact

none; production schema remains v4.

## Provenance Impact

none

## Historical Context Impact

Navigation only. Panel, candidates, selections, and saved-date state remain unchanged.

## Consent Impact

none; cancellation remains no consent and no transmission.

## Provider Transmission Impact

none

## Tests Added

Exact historical-panel target, post-render focus, block-start scrolling, reduced motion, missing-target safety, and three-language control-label parity.

## Tests Executed

Focused 3 files and 17 tests; canonical 40 files and 306 Vitest tests, 189 Rust tests, 12 backup/restore tests, and 8 schema-contract tests.

## Verification Results

passed

## Manual Verification Required

Founder Step 10C-R for both cancel and adjust-source paths.

## Documentation Updates

Workflow evidence only.

## ADR Impact

none

## Deviations From Plan

none

## Known Limitations

Manual viewport behavior still requires Founder confirmation in the real desktop window.

## Remaining Risks

Browser focus/scroll behavior can vary with window height; the exact narrow-window case is retained for manual review.

## Git State

Unstaged working changes only; no commit, push, merge, PR, deployment, or release.

## Engineer Completion Status

completed_with_follow_up
