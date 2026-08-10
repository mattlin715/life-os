# Engineering Plan

Status: approved

- Sprint ID: 2026-08-11-daily-reflection-founder-review-corrections
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: f8fbf94812bac2e1359367362e9c1e0db1aeaef1
- Working-tree digest reviewed: 390c3cd860c8d620c6e01c70955629fb9e7708d6369af54f87cd0efe802d7b91
- Created at: 2026-08-11T02:05:00+09:00
- Updated at: 2026-08-11T02:05:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Preserve the Pattern gate and ADR-0009 lifecycle; implement only explanatory UI, an open-only shortcut to the existing panel, and local lexical quality refinement. No inferred semantics or automatic historical action.

## Existing Implementation Understanding

`decidePatternAvailability` gates on Experience body plus saved Context Recovery, not Reflection answers. `App.tsx` renders the historical panel after the full Evidence/Reflection/Pattern stack. `retrieve.ts` creates every adjacent two-character term in CJK runs, causing fragments and generic overlap. Existing R1 files are unpromoted working changes and must remain intact.

## Affected Modules

- `src/app/App.tsx`, `src/app/i18n.ts`, `src/app/i18n.test.ts`, `src/styles.css`
- new `src/app/HistoricalContextEntryPoint.tsx` and test
- `src/historicalContext/retrieve.ts`, `retrieve.test.ts`, `types.ts`
- `src/historicalContext/governedPacket.ts`, `provenanceInspector.ts` and focused compatibility tests only as required to preserve old v1 provenance while producing v2 retrieval metadata
- `docs/11_MVP.md`, `docs/product/00_MVP_User_Flow.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`
- repository-required workflow artifacts

## Proposed Design

1. Add localized copy that explicitly separates valid saved Reflection from missing event-context facts and names what the user can add.
2. Render a compact entry point immediately after each Experience body. It is open-only; activation opens the existing local panel and calls `scrollIntoView` on that exact panel. Rendering and silence do nothing.
3. Introduce `local-lexical-v2`: English stays unchanged; zh-TW/ja use `Intl.Segmenter` word-like segments, explicit locale stop terms, minimum visible length, and a conservative no-cross-boundary fallback. Candidate ranking/caps remain unchanged.
4. New packets use v2 metadata; the read-only provenance inspector continues to accept already persisted v1 and new v2 records. No packet field or consent rule changes.

## Alternatives Considered

- Moving the full panel to the top would disrupt the established Evidence -> Reflection flow; use a discoverable shortcut and retain the governed panel in place.
- Curated adjacent bigrams were rejected because fragment boundaries remain visible and fragile.
- Embeddings or semantic retrieval are out of scope and would add opacity/provider or storage concerns.

## Data Lifecycle Impact

No persistence changes. Shortcut and panel state remain session-only. Retrieval candidates and reasons remain ephemeral. Existing generated artifacts retain their original algorithm version.

## SQLite Or Migration Impact

None. Production schema and `user_version` remain 4.

## Provenance Impact

No source or generated-artifact provenance is rewritten. The inspector must accept persisted v1 provenance and new v2 provenance without rehydration.

## Historical Context Impact

Local retrieval changes from CJK adjacent-character overlap to visible word-like overlap. It remains explicit-panel, bounded, deterministic for the pinned runtime, explainable, source-based, and provider-independent.

## Consent Impact

No policy change. Selection remains ephemeral and opening/scrolling/selection remains distinct from preflight and consent.

## Provider Transmission Impact

No new call or destination. If the user later authorizes a governed call, the unchanged packet field discloses `local-lexical-v2` and remains digest-bound.

## Import And Export Impact

None.

## Test Strategy

- Unit tests for zh-TW fragment/generic-term rejection and meaningful-term retention.
- Japanese word-like overlap and English regression tests.
- v1/v2 provenance-inspector compatibility and current packet metadata tests.
- Entry-point render, open-only callback, localized label, and no automatic action tests.
- Existing panel lifecycle, consent, R1, and i18n suites.

## Repository Verification Strategy

Run focused Vitest files, TypeScript typecheck, `git diff --check`, then `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`; independently audit forbidden paths and staged files.

## Manual UI Verification

Founder. Restart the desktop build and guide one bounded step at a time through the new Pattern explanation, top shortcut, local-only state, CJK reasons, selection/consent separation, and locale parity.

## Rollback Or Recovery Strategy

All changes are unstaged and reversible. The CJK fallback fails toward fewer candidates, never whole-history loading. If Segmenter is unavailable, conservative contiguous visible segments are used without cross-boundary fragments. No stored state needs rollback.

## Documentation Impact

Minimal factual synchronization in architecture/08 and the two already modified Book One product documents.

## ADR Impact

No new ADR. ADR-0009 decision is unchanged; architecture/08 receives a factual algorithm-version update.

## Risk Level

Medium: local retrieval ranking changes and compatibility with persisted provenance require explicit tests, but there is no persistence/provider/schema mutation.

## Escalation Decision

No Founder escalation is required for these reversible corrections within the unpromoted R1 review scope. Stop again at Founder manual review; do not promote.
