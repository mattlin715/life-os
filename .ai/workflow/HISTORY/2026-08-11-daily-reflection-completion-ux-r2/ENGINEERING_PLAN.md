# Engineering Plan

Status: approved

- Sprint ID: 2026-08-11-daily-reflection-completion-ux-r2
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: c329edb80e0ab3bd5bcbcc4dedd33c9c3af6bf12
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-10T21:29:00.000Z
- Updated at: 2026-08-10T21:29:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement only deterministic session UI over already loaded v4 records: one current core step, explicit reopen, record-only completion, optional Pattern/history, focus navigation, three-language copy, tests, and factual Book One updates. Do not alter storage, provider, consent, provenance, schema, or AI task contracts.

## Existing Implementation Understanding

`App.tsx` owns loaded Experience/artifact state, ephemeral Reflection drafts, generation handlers, local historical panel state, and rendering. `dailyReflectionFlow.ts` derives only five coarse next actions. Reviewed Evidence, Reflection, Pattern, Recovery, and Historical Question records are already loaded per Experience. Rejected Evidence/Pattern content is deliberately removed from durable v4 artifacts; any same-session “set aside” acknowledgement must remain content-free and ephemeral rather than inventing persistence.

## Affected Modules

Anticipated product/document allowlist before implementation:

- `src/app/App.tsx`
- `src/app/DailyReflectionComposer.tsx`
- `src/app/DailyReflectionComposer.test.tsx`
- `src/app/dailyReflectionFlow.ts`
- `src/app/dailyReflectionFlow.test.ts`
- `src/app/dailyReflectionJourney.ts` (new)
- `src/app/dailyReflectionJourney.test.ts` (new)
- `src/app/DailyReflectionJourneyView.tsx` (new)
- `src/app/DailyReflectionJourneyView.test.tsx` (new)
- `src/app/journeyNavigation.ts` (new)
- `src/app/journeyNavigation.test.ts` (new)
- `src/app/contextRecoveryNavigation.ts`
- `src/app/contextRecoveryNavigation.test.ts`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `src/styles.css`
- `docs/11_MVP.md`
- `docs/product/00_MVP_User_Flow.md`
- repository-required `.ai/workflow/` active and archived sprint artifacts

Any additional product path requires a documented plan correction before editing.

## Proposed Design

1. Replace the coarse resolver with a pure `resolveDailyReflectionJourney` model that accepts counts and booleans only, returns detailed Evidence/Reflection/Pattern states, one active core stage, next action, completion facts, and optional history availability.
2. Add focused presentational components: a controlled `JourneyStage` disclosure and a `ReflectionCompletionReview`. They receive copy and existing record-derived values; they do not fetch, persist, or generate.
3. Add session-only per-entry stage override and content-free Pattern-set-aside acknowledgement in `App`. Only one stage body is displayed at once. Core-complete entries default to the completion surface with all stages collapsed.
4. Add explicit navigation helpers that set disclosure state and focus stable DOM targets. Use `matchMedia('(prefers-reduced-motion: reduce)')` to choose `auto` rather than `smooth` scrolling.
5. Guide focus only after successful explicit actions. Opening/navigation itself calls no generation handler.
6. Add a stable composer target for “record another Experience.”
7. Keep provider failures and stale-generation messages authoritative; the journey derives only committed state and therefore never silently advances.

## Alternatives Considered

- Keep all cards expanded and add stronger copy: rejected because it does not reduce competing visual emphasis.
- Persist current stage: rejected because the state is derivable and persistence is explicitly out of scope.
- Generate an AI completion summary: rejected as unnecessary authority expansion and oracle risk.
- Rewrite `App.tsx`: rejected; focused components/helpers minimize regression risk.

## Data Lifecycle Impact

None. Existing explicit save/review handlers remain the only durable actions. Stage overrides, completion disclosure, and set-aside acknowledgement are session-only.

## SQLite Or Migration Impact

None. No Rust, SQL, schema, `SCHEMA_VERSION`, `user_version`, startup, backup, or restore path is in the allowlist.

## Provenance Impact

None. The completion surface labels original Experience and user Reflection separately from AI/local-mock Pattern hypotheses and never rewrites provenance.

## Historical Context Impact

Only an optional navigation entry to the existing local panel and display of the count/list of already persisted Historical Reflection Questions. No retrieval, selection, or packet behavior changes.

## Consent Impact

None. Navigation does not select, preflight, consent, or transmit. Existing ADR-0009 controls remain unchanged.

## Provider Transmission Impact

None. Generation handlers are unchanged except for post-success focus guidance. New components/helpers have no provider dependencies.

## Import And Export Impact

None. No import/export format or behavior changes.

## Test Strategy

- Pure table-driven resolver tests for all required journey states, reload reconstruction, optional Pattern, rejected/session acknowledgement, and no Phase 4 fields.
- Static component tests in all three locales for stage semantics, record-only completion, authorship distinction, optional controls, and no generated synthesis.
- Navigation unit tests for keyboard-focus targets, reduced-motion behavior, missing elements, and no action callbacks.
- Existing R1, historical consent, provider, storage, and i18n suites remain regression coverage.
- Focused Vitest during implementation, then canonical repository verification.

## Repository Verification Strategy

Run `git diff --check`, focused Vitest, TypeScript typecheck/build as needed, Clippy only if Rust changes unexpectedly (none planned), then `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Audit tracked and non-ignored untracked paths against the allowlist; confirm Constitution and schema/provider/ContextPacket paths unchanged and index empty.

## Manual UI Verification

Founder-owned after implementation. Prepare a 17-step sequence covering creation, sparse Context Recovery, Evidence review, Reflection save/skip, completion, authorship, reopen, optional Pattern/history, second Experience, restart reconstruction, locale parity, narrow view, keyboard, and reduced motion. Start the desktop app when technically possible; do not claim acceptance.

## Rollback Or Recovery Strategy

The slice is presentation-only. Reverting the new components/helpers and their `App` integration returns R1 behavior without data rollback. Existing saved records remain unchanged.

## Documentation Impact

Correct R1 promotion drift before product edits, then add a factual unpromoted R2 description to `docs/11_MVP.md` and `docs/product/00_MVP_User_Flow.md` after implementation evidence exists.

## ADR Impact

No new ADR and no status change. The slice operates within accepted ADR-0007, ADR-0009, and ADR-0011 boundaries.

## Risk Level

medium: the product behavior is reversible and schema-neutral, but `App.tsx` integration spans several existing generation/review transitions. Pure resolution, focused components, and regression tests bound the risk.

## Escalation Decision

No Founder escalation is required. The objective explicitly authorizes the product boundary, and remaining choices are reversible implementation details.
