# Engineering Plan

Status: approved

- Sprint ID: 2026-08-11-daily-reflection-journey-summary-dedup
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: 5baa1cbfe6542b020a536ed178aa4eea5519de8148313075ab992aa73e821648
- Created at: 2026-08-11
- Updated at: 2026-08-11

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is approved. Change only the shared JourneyStage presentation condition and its focused test. Preserve all pre-existing audit work.

## Existing Implementation Understanding

`JourneyStage` renders `summary` unconditionally. `App.tsx` passes the stage-specific next-step summary and independently renders the same state-derived guidance inside open Evidence, Reflection, and Pattern cards.

## Affected Modules

- `src/app/DailyReflectionJourneyView.tsx`
- `src/app/DailyReflectionJourneyView.test.tsx`
- repository-required workflow artifacts and archive

## Proposed Design

Change the summary paragraph to render only when `open` is false. Keep component props and callers unchanged. Add table-driven tests proving collapsed visibility and open omission for all three stage summary keys in EN/zh-TW/ja while child content remains visible.

## Alternatives Considered

Changing every `App.tsx` caller was rejected because it duplicates a shared presentation rule and expands the diff. Rewriting copy was rejected because the copy is useful in both intended contexts.

## Data Lifecycle Impact

none

## SQLite Or Migration Impact

none

## Provenance Impact

none

## Historical Context Impact

none

## Consent Impact

none

## Provider Transmission Impact

none

## Import And Export Impact

none

## Test Strategy

Run the focused Vitest file, then canonical `scripts/verify.ps1`. Audit the diff and untracked paths against the exact allowlist.

## Repository Verification Strategy

Canonical verification, workflow validation, `git diff --check`, no Constitution/schema/provider/ContextPacket/consent/persistence diff, and no staged files.

## Manual UI Verification

Founder-owned Step 1R: reopen the same Evidence stage and confirm the sentence appears once; then collapse and confirm it remains as the summary. Continue to Reflection/Pattern during later walkthrough steps.

## Rollback Or Recovery Strategy

Revert the one conditional and test if the detailed open guidance is missing. No stored state or data recovery is involved.

## Documentation Impact

No Book One update is required; this is a bounded correction discovered during live review. Workflow evidence records it.

## ADR Impact

No ADR impact; presentation deduplication creates no durable policy.

## Risk Level

low; one shared render condition and focused tests, with no state or data path.

## Escalation Decision

No escalation expected. Any need to alter `App.tsx`, i18n values, state, storage, or provider behavior returns to the Founder.
