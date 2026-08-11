# Engineering Report

Status: completed_with_follow_up

## Sprint ID

2026-08-11-historical-relevance-origin-disclosure-correction

## Implementation Summary

Added local-only exact relevance-origin disclosures while preserving aggregate governed reasons.

## Existing System Areas Inspected

Historical retrieval, candidate types, App candidate rendering, i18n, governed packet tests, and architecture/08/09.

## Files Added

src/app/HistoricalCandidateRelevance.tsx and its focused test.

## Files Modified

src/historicalContext/types.ts, retrieve.ts, retrieve.test.ts, src/app/App.tsx, i18n.ts, and i18n.test.ts.

## Files Deleted

none

## Behavior Changed

Explicit-open candidate cards now identify Experience, confirmed Evidence, and saved user Reflection matches and show bounded exact excerpts.

## Data Model Impact

none; visibleMatches is ephemeral candidate presentation data.

## Migration Impact

none; production schema remains v4.

## Provenance Impact

Aggregate governed relevance remains unchanged; local UI makes match origin inspectable.

## Historical Context Impact

Presentation only. Matching, score, ordering, caps, date filtering, eligibility, and panel lifecycle are unchanged.

## Consent Impact

none

## Provider Transmission Impact

none

## Tests Added

Exact origin grouping, ineligible exclusion, aggregate compatibility, three-language component rendering, and i18n parity.

## Tests Executed

Focused 45 tests plus canonical 40 files and 303 Vitest tests, 189 Rust tests, 12 backup/restore tests, and 8 schema-contract tests.

## Verification Results

passed after removing two workflow-only trailing blank lines found by the first canonical run.

## Manual Verification Required

Founder Step 9R on the same real local-history candidates.

## Documentation Updates

Workflow evidence only.

## ADR Impact

none

## Deviations From Plan

The UI rendering was isolated into a focused component and test; authorized behavior is unchanged.

## Known Limitations

This is lexical origin disclosure, not semantic relevance or Phase 4 interpretation.

## Remaining Risks

Founder must judge whether the additional bounded excerpts are legible and sufficiently explanatory.

## Git State

Unstaged working changes only; no commit, push, merge, PR, deployment, or release.

## Engineer Completion Status

completed_with_follow_up
