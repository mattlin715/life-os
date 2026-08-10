# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-10-daily-reflection-core-ux-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: f8fbf94812bac2e1359367362e9c1e0db1aeaef1
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-10T08:29:00.000Z
- Updated at: 2026-08-10T08:29:00.000Z

## Mission Interpretation

Recompose the already governed schema-v4 MVP into a calm daily-reflection journey without adding model capability or data authority. Experience entry and the next user-owned reflective action become visually primary. Existing diagnostics, portability, history, consent, provenance, and Pattern behavior remain available through appropriate progressive disclosure.

## Problem Statement

Repository evidence shows three product-friction points. `src/app/App.tsx` exposes Database Readiness in the top-bar primary controls, renders optional historical context before Evidence and Reflection inside every Experience, and expands the exact Phase 3B technical preflight immediately. The timeline has no keyword or saved-date filter. `src/app/i18n.ts` leaves many ordinary Evidence, Reflection, Pattern, history, consent, and provenance labels in English inside Traditional Chinese and Japanese flows. These are hierarchy and comprehension problems, not missing inference capability.

## User Value

The user can begin with one lived moment, immediately see the next useful action, find an earlier saved Experience locally, and understand consent without first decoding schema, packet, digest, or migration terminology. Technical truth remains inspectable when explicitly requested. This reduces startup threat while preserving agency, provenance, and exact consent.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI; Reflection before Answer; Evidence before Conclusion; We Build Mirrors, Not Oracles.
- `docs/03_Principles.md`: context and evidence must precede higher inference.
- `docs/06_Memory.md`: remembered material remains user-owned, traceable, correctable, and deletable.
- `docs/Reflection.md`: questions invite user interpretation and do not own meaning.
- `docs/09_AI.md`: AI remains humble, provider-independent, and subordinate to user agency.
- `docs/10_Privacy.md`: transparency and ongoing consent are psychological-safety requirements.
- `docs/11_MVP.md` and `docs/product/00_MVP_User_Flow.md`: the first usable loop begins with one Experience and proceeds through Evidence and Reflection.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: presentation must not blur AI output and user review.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`: selected, disclosed, consented, transmitted, and persisted states remain separate; consent is exact and one-use.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`: this UI slice must not invent or alter lifecycle authority.

## Current Implementation Context

- Implemented and promoted: schema-v4 Experience, Evidence, Reflection, Pattern, local historical selection, governed Historical Question generation, P1 provenance inspector, and read-only Database Readiness Inspector.
- Implemented code: `App.tsx` currently renders readiness in primary navigation, a dense full timeline, local history before review cards, and a fully expanded preflight.
- Implemented local retrieval: `local-lexical-v1` and per-panel saved-date range; this sprint does not change it.
- Founder-authorized in this sprint: reversible Daily Reflection Core UX R1 presentation, local timeline filtering, terminology, and tests.
- Deferred and unauthorized: schema-v5 activation, real-user migration, Phase 4, new provider behavior, and any consent-policy change.

## In Scope

- Experience-first primary viewport and visible next-action guidance.
- Progressive disclosure for secondary diagnostics, portability, older/completed Experience detail, historical context, and technical preflight content.
- Session-only keyword and saved-date filtering over loaded persisted Experiences, with deterministic combination, reset, count, and empty state.
- Natural English, Traditional Chinese, and Japanese core-path terminology.
- Concise ADR-0009 preflight summary plus explicit exact-content/privacy details available before consent.
- Semantic, keyboard-operable, focus-visible, narrow-window-safe UI and behavior-level tests.
- Minimal factual updates to `docs/11_MVP.md` and `docs/product/00_MVP_User_Flow.md`.

## Out Of Scope

Schema, persistence, migration, backup/restore, provider, ContextPacket, packet construction, consent state, retention, provenance lifecycle, historical ranking, Phase 4, diagnosis, identity inference, telemetry, dependencies, Harness redesign, stage, commit, push, merge, deployment, or release.

## Product Constraints

The first viewport must prioritize Experience entry. The newest Experience must expose the next useful action. Technical surfaces must remain reachable but secondary. Timeline filters use `createdAt` as the saved date, never an inferred event date. Search state and disclosure state remain in React memory only.

## Evidence And Provenance Constraints

Evidence candidate status, edit/confirm/reject actions, Reflection drafts, Historical Question persistence, and P1 provenance validation remain unchanged. Native labels may simplify ordinary language; advanced exact identifiers remain verbatim and inspectable.

## Historical Context Constraints

Existing explicit-panel retrieval, lexical reasons, caps, saved-date semantics, ephemeral selection, selection invalidation, and closed-panel no-retrieval behavior remain unchanged. Timeline search is a separate local view filter and does not alter historical candidate retrieval.

## Consent Constraints

Opening history, selecting a source, opening preflight, opening detailed disclosure, silence, and cancellation never create consent. The concise preflight must disclose selected material, destination, purpose, source types/count, sensitivity, and one-generation/one-purpose scope. Exact IDs, revisions, content, relevance, retention, packet ID/digest, and include/exclude controls must be available before the existing send action.

## AI-Role Constraints

No new AI call or output type. No Cross-Experience conclusion, recurrence narrative, diagnosis, identity finalization, sensitive inference, advice, or Pattern authority is introduced. Existing provider fallback and historical no-fallback behavior remain unchanged.

## Privacy Constraints

Filters use only locally loaded persisted Experience metadata/text and trigger no provider or persistence call. Detailed transmission content is hidden by default but accessible by explicit action. No clipboard, analytics, preferences, or new durable UI state.

## User-Agency Constraints

Consequential actions use explicit text labels. Users can clear filters, adjust sources, inspect exact outgoing content, cancel without transmission, and retain existing correction/rejection/deletion controls. Progressive disclosure must not conceal consent scope or the source-adjustment path.

## Acceptance Criteria

1. Experience entry is the first dominant action and Database Readiness is absent from primary navigation.
2. Database Readiness remains explicitly available under a secondary diagnostics/data surface with identical read-only behavior.
3. The newest Experience displays a localized next useful action; older Experience detail is progressively disclosed.
4. Keyword filtering returns deterministic local body-text matches.
5. Timeline date filtering uses inclusive saved-date (`createdAt`) semantics in the device timezone and never claims event date.
6. Keyword and date filters combine before display; invalid/incomplete ranges fail closed with localized feedback.
7. Clear restores the complete loaded list, count updates visibly, and empty results are clear in all three locales.
8. Filter operations invoke no provider, storage mutation, or preference persistence path.
9. Traditional Chinese and Japanese ordinary core-path labels use native Experience/Evidence/Reflection/Pattern/history/consent/provenance terminology; explicit advanced identifiers may remain English.
10. Preflight defaults to a concise summary of send boundary, destination, purpose, included types/count, sensitivity, and exact one-use consent.
11. Exact packet/source/artifact IDs, revisions, outgoing text, reasons, privacy/retention, packet digest relationship, and include/exclude controls are accessible before consent through explicit disclosure.
12. Opening details creates no consent; cancel uses the existing no-call/no-record path; changing source/provider/model/language/purpose continues to close or invalidate preflight.
13. Existing immutable packet/digest, stale-work, provider no-fallback, actual-use provenance, R1 retrieval, and Phase 3B output-boundary tests remain passing.
14. Changed controls are semantic, keyboard operable, focus-visible, and usable at narrow widths.
15. Constitution, schema v4, storage/provider/ContextPacket/consent semantics, and Phase 4 boundary remain unchanged.

## Risks

- High: accidental consent detail omission or a send action made visually ambiguous. Mitigation: dedicated preflight component tests assert summary, detailed disclosure, source controls, and explicit actions.
- Medium: App re-composition could hide existing actions or reset state unexpectedly. Mitigation: preserve handlers/state and add static/behavior helper tests plus Founder manual review.
- Medium: date semantics could be confused with event date. Mitigation: reuse the promoted saved-date range contract and explicit locale copy.
- Low: translation changes may over-localize provider/contract identifiers. Mitigation: native ordinary labels with verbatim advanced identifiers and proper nouns.

## Open Questions

None. The Founder prompt explicitly authorizes the bounded meaning and delegates reversible composition, wording, component organization, and testing decisions.

## Human Decision Required

false; no new decision ID. Any discovered need to weaken ADR-0009, change Book Zero, activate schema v5, or expand provider transmission must stop separately.

## Recommendation

Proceed with the smallest componentized UX slice. Keep the existing data/model handlers intact, reuse the saved-date contract, isolate timeline filtering and preflight presentation into testable pure/components, and route diagnostics through a native `details` secondary surface.

## Review Status

approved_with_conditions
