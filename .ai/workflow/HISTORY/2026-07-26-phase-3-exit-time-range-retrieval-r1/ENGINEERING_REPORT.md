# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-07-26-phase-3-exit-time-range-retrieval-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 5714b3eeeeb4c9612e445dbd78e4f606df41fa27
- Working-tree digest implemented: dbd0bd7d4df934aa52824d25caf2af0de017c965f45e41ba31607edf5b399f9d
- Created at: 2026-07-26T14:44:41Z
- Updated at: 2026-07-26T14:48:30Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the Founder-authorized R1 production vertical slice: an explicit
per-panel, session-only source-Experience saved-date control that fails closed
until a valid range is applied, captures the device timezone, filters before
the unchanged lexical ranking/caps, shows exact inclusive range metadata, and
invalidates affected selection/preflight state on every control change.

## Existing System Areas Inspected

- `src/historicalContext/date.ts`, retrieval, panel, selection, governed packet,
  and their tests.
- Historical candidate, selection, preflight, deletion, and rendering paths in
  `src/app/App.tsx`.
- `src/app/i18n.ts`, styles, storage interfaces, provider/packet boundaries, and
  architecture/08, 09, 11, 12, and 13.

## Files Added

- `src/historicalContext/savedDateRange.ts`
- `src/historicalContext/savedDateRange.test.ts`
- `src/app/HistoricalSavedDateRangeFilter.tsx`
- `src/app/HistoricalSavedDateRangeFilter.test.tsx`

## Files Modified

- `src/historicalContext/types.ts`
- `src/historicalContext/retrieve.ts`
- `src/historicalContext/retrieve.test.ts`
- `src/historicalContext/panelRetrieval.ts`
- `src/historicalContext/panelRetrieval.test.ts`
- `src/app/App.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `src/styles.css`
- `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `docs/11_MVP.md`
- `docs/12_Roadmap.md`
- Repository-mediated current workflow artifacts under `.ai/workflow/`.

## Files Deleted

none.

## Behavior Changed

- Each explicitly opened historical panel has an inactive-by-default saved-date
  filter.
- Enabling it blocks retrieval until both dates are explicitly applied.
- Valid applied ranges use source `createdAt`, inclusive local calendar dates,
  captured device IANA timezone, and next-day exclusive instants.
- Invalid or incomplete controls show localized fail-closed feedback and call
  no retrieval.
- Applied ranges filter before lexical matching, ranking, and limiting.
- Candidate cards show the exact inclusive range/timezone beside lexical terms.
- Every control change clears that current Experience's ephemeral selection and
  closes its preflight.
- Closing a panel still performs no retrieval; its applied range remains only
  for the app session and is re-evaluated on reopen.

## Data Model Impact

Ephemeral TypeScript/React control state only. No durable domain, storage,
import, export, or database record changed.

## Migration Impact

None. Production `SCHEMA_VERSION` and SQLite `user_version` remain 4.

## Provenance Impact

None. Range metadata is not Evidence, provenance, a packet field, or a durable
artifact.

## Historical Context Impact

Adds only an explicit deterministic narrowing constraint. Existing eligibility,
lexical terms, scoring, stable ranking, candidate caps, exact-ID selection, and
Phase 3B generated-question boundaries remain intact.

## Consent Impact

No policy or persistence change. Range controls never imply consent and instead
invalidate an affected open preflight so the existing exact-content consent
path must begin again.

## Provider Transmission Impact

None. No provider, `ContextPacket`, governed historical packet, prompt,
evaluator, transport, or persistence file changed.

## Tests Added

- 8 strict saved-date/calendar/timezone/control/preflight helper tests.
- 4 UI static-render tests across three locales and fail-closed visibility.
- 4 retrieval cases covering inactive parity, pre-ranking filtering, inclusive
  boundaries/zero results, and malformed source timestamps.
- 3 explicit-panel cases covering blocked no-call, applied routing, and inert
  closed-panel range.
- 1 i18n semantic-parity case.

## Tests Executed

- `pnpm typecheck` — passed.
- Focused Vitest command covering range, retrieval, panels, selection, i18n,
  and UI — 6 files / 61 tests passed.
- Canonical `scripts/verify.ps1` — passed: workflow 17/17; Vitest 24
  files/183 tests; Rust library 70/70; backup/restore 12/12; schema contract
  8/8; typecheck, frontend build, Rust check, and repository hygiene passed.

## Verification Results

- Focused implementation tests: passed.
- TypeScript typecheck: passed.
- `git diff --check`: passed.
- Forbidden boundary diff inspection: no `src/ai`, storage, Rust/SQLite,
  Constitution, provider, or ContextPacket changes.
- Canonical repository verification: passed with exit code 0.
- Founder manual UI verification: pending by explicit design.

## Manual Verification Required

Yes, Founder-owned after diff review: inactive parity; same-day/multi-day
inclusive filtering; visible timezone; invalid/inverted behavior; selection and
preflight invalidation; no-match result; closed/reopen behavior; and English,
Traditional Chinese, and Japanese parity.

## Documentation Updates

- architecture/08 version 0.8 records the exact R1 implemented boundary and
  still-pending Founder/manual/promotion states.
- MVP and Roadmap factually identify the saved-date working-tree slice without
  claiming promotion or closing other structured-retrieval gaps.
- architecture/13 version 2.2 corrects the already-promoted Slice 3B facts.

## ADR Impact

No new or changed ADR decision. ADR-0009 separation and ADR-0010 Phase 3 exit
boundary are applied, not redefined.

## Deviations From Plan

The UI was extracted into a small component so the authorized UI surface has
direct static-render tests without adding a test dependency. No authority or
behavior deviation occurred.

## Known Limitations

- `createdAt` is saved time, not event time; the UI labels it accordingly.
- The captured timezone applies to the last explicit Apply action and remains
  session-only.
- Static UI tests verify controls/copy/alert structure; Founder runtime review
  remains necessary for interaction and layout.
- R1 does not implement emotion, relationship, or value-conflict retrieval.

## Remaining Risks

- Platform/browser date controls and timezone presentation require Founder
  desktop review.
- A device timezone change affects only a future Apply; the currently applied
  exact instants retain the displayed captured timezone.
- Manual verification must confirm the preflight visibly closes in the full
  desktop flow.

## Git State

- Branch: `codex/phase-3-exit-time-range-retrieval-r1`
- HEAD: `5714b3eeeeb4c9612e445dbd78e4f606df41fa27`
- Working tree: unstaged authorized product/document/workflow changes.
- Staged files: none.
- Upstream: not configured for this new local branch.
- Commit, push, merge, PR, deployment: not performed.

## Engineer Completion Status

completed_with_follow_up
