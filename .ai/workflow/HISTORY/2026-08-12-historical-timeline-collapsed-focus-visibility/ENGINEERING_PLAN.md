# Engineering Plan

Status: approved

- Sprint ID: 2026-08-12-historical-timeline-collapsed-focus-visibility
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: 129c911cccd6564240279ec95430659b5667fd1719455c7c41381f29675dac68
- Created at: 2026-08-12T00:00:00+09:00
- Updated at: 2026-08-12T00:00:00+09:00

## Approved Product Boundary

Founder-authorized CSS focus visibility only; Product Review precedes implementation.

## Existing Implementation Understanding

Native summary focus works, but the card clips the global outside outline.

## Affected Modules

`src/styles.css`, one focused test file, and required workflow evidence.

## Proposed Design

Add an exact `.entry-disclosure > summary:focus-visible` rule using an inset box shadow and subtle background so the highlight remains inside the clipped card.

## Alternatives Considered

Removing overflow clipping would change card rendering; adding JavaScript focus state is unnecessary; changing labels would not distinguish identical repeated rows.

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

Raw CSS contract asserts the exact inset focus rule and absence of outline reliance; locale assertions cover current/older/open-detail labels in EN/zh-TW/ja.

## Repository Verification Strategy

Focused Vitest, TypeScript typecheck, then canonical `scripts/verify.ps1`.

## Manual UI Verification

Founder repeats narrow-window Tab navigation before pressing Enter.

## Rollback Or Recovery Strategy

Remove the bounded CSS rule and test; no data recovery.

## Documentation Impact

Workflow evidence only.

## ADR Impact

No new ADR.

## Risk Level

low; CSS-only presentation.

## Escalation Decision

No escalation required.
