# Engineering Plan

Status: approved

- Sprint ID: 2026-08-11-historical-preflight-cancel-focus-restoration
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: eb14081b89631eadc623672b42417eb38f5e788da3c2eefe134b08776ad976b5
- Created at: 2026-08-11T09:53:04.999Z
- Updated at: 2026-08-11T09:53:04.999Z

## Approved Product Boundary

Founder-authorized local focus restoration only; Product Review will be approved before implementation.

## Existing Implementation Understanding

The preflight is rendered inside the still-open history panel. Clearing it changes document height but does not intentionally close the panel or alter filter/selection state.

## Affected Modules

`src/app/App.tsx`, `src/app/journeyNavigation.ts`, `src/app/journeyNavigation.test.ts`, and `src/app/HistoricalConsentPreflight.test.tsx`, plus repository-required workflow evidence.

## Proposed Design

Add an exact historical-panel navigation helper using one post-render frame, focus with `preventScroll`, block-start reveal, and reduced-motion support. Both cancel and adjust callbacks clear preflight then invoke it for the current Experience.

## Alternatives Considered

Browser scroll anchoring alone already failed. Keeping the preflight mounted would misrepresent cancellation. A generic global scroll reset would lose exact Experience context.

## Data Lifecycle Impact

none; state is preserved except the already-authorized ephemeral preflight close.

## SQLite Or Migration Impact

none

## Provenance Impact

none

## Historical Context Impact

Presentation navigation only; retrieval and packet relevance remain unchanged.

## Consent Impact

none; cancel remains no consent.

## Provider Transmission Impact

none

## Import And Export Impact

none

## Test Strategy

Test exact target ID, post-render scheduling, focus, block-start scroll, reduced motion, missing target, and EN/zh-TW/ja cancel/adjust callback parity.

## Repository Verification Strategy

Focused Vitest, TypeScript typecheck, then `scripts/verify.ps1`.

## Manual UI Verification

Founder Step 10C-R repeats selection, preflight, cancel and adjust-source return positioning.

## Rollback Or Recovery Strategy

Revert the helper wiring; no durable data requires recovery.

## Documentation Impact

Workflow evidence only; no source-of-truth policy changes.

## ADR Impact

No new ADR; ADR-0009 behavior is preserved.

## Risk Level

low; bounded session-only navigation.

## Escalation Decision

No escalation required within the explicit Founder authorization.
