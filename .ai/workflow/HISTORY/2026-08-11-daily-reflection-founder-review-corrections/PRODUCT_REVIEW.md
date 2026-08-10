# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-11-daily-reflection-founder-review-corrections
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: f8fbf94812bac2e1359367362e9c1e0db1aeaef1
- Working-tree digest reviewed: 390c3cd860c8d620c6e01c70955629fb9e7708d6369af54f87cd0efe802d7b91
- Created at: 2026-08-11T01:55:00+09:00
- Updated at: 2026-08-11T01:55:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Treat the Founder observations as acceptance feedback on the unpromoted R1 slice. Correct the smallest product surfaces without changing the moral or data relationship between AI and user.

## Problem Statement

The product currently hides an optional historical-reflection capability below a long artifact stack, gives a vague Context Recovery instruction after users already did valid Evidence and Reflection work, and explains CJK retrieval with raw adjacent-character fragments that are neither meaningful nor suitably selective.

## User Value

Users should understand what is missing and what action unlocks the tentative hypothesis, discover local historical reflection without scrolling, and see only concrete, readable overlap terms rather than lexical implementation noise.

## Relevant Primary Definitions

- `docs/03_Principles.md`: Context Before Insight and mirror-before-advice require an honest explanation rather than an unexplained disabled control.
- `docs/06_Memory.md`: history is governed evidence, not automatically authoritative memory.
- `docs/Reflection.md`: reflection supports meaning-making but does not silently invent missing event facts.
- `docs/09_AI.md` and `docs/10_Privacy.md`: historical use remains explicit, bounded, local until separately consented.

## Relevant ADRs

- ADR-0007 preserves review/provenance boundaries; unchanged.
- ADR-0009 keeps retrieval, selection, consent, transmission, and actual use distinct; the top shortcut must never imply consent.

## Current Implementation Context

Daily Reflection Core UX R1 is implemented and canonically verified on `codex/daily-reflection-core-ux-r1`, but remains unpromoted and not Founder-accepted. Historical retrieval is an existing promoted local-only capability. This sprint refines that unpromoted UI and the existing local lexical retrieval behavior; it does not authorize Phase 4 or historical transmission outside ADR-0009.

## In Scope

- Explicit explanation that saved Evidence/Reflection remain valid but missing event context still blocks a tentative Pattern.
- A top-of-Experience local-history shortcut that opens and moves focus/viewport to the existing panel.
- Meaningful CJK word segmentation, filtering of generic connector/feeling-report terms, and human-readable relevance reasons.
- Focused regression tests and factual Book One updates.

## Out Of Scope

No change to Pattern sufficiency policy, historical eligibility, candidate caps, saved-date behavior, selection persistence, provider transport, consent, ContextPacket structure, schema, retention, or Phase 4.

## Product Constraints

No artifact or user-authored response is rewritten. Retrieval reasons remain derived from exact visible local text and carry no inferred emotion, relationship, or value taxonomy.

## Evidence And Provenance Constraints

Panel-open retrieval remains explicit, bounded, local, deterministic, and source-citing. The shortcut opens only the existing panel. CJK matching may use only visible word-like segments and an explicit stop-term set; it may not infer synonyms or meaning.

## Historical Context Constraints

Panel-open retrieval remains explicit, bounded, local, deterministic, and source-citing. The shortcut opens only the existing panel. CJK matching may use only visible word-like segments and an explicit stop-term set; it may not infer synonyms or meaning.

## Consent Constraints

Selection remains ephemeral and is not consent. Preflight, exact-content disclosure, per-generation/per-purpose consent, and invalidation remain unchanged.

## AI-Role Constraints

AI remains a reflective aid. The UX may explain the deterministic gate, but must not claim that saved Reflection is worthless or that a Pattern is a conclusion.

## Privacy Constraints

No background whole-history retrieval, persistence, provider call, or new telemetry. Opening history is still a user action.

## User-Agency Constraints

The user can see and activate the local-history option near the top, can understand why Context Recovery is requested, and can decline both without penalty. No automatic inclusion or send occurs.

## Acceptance Criteria

1. The insufficient-context state explicitly acknowledges completed Evidence/Reflection and identifies the missing event-context dimensions.
2. Each open Experience shows a top shortcut; activation opens the existing local-history panel and brings it into view without selecting or sending anything.
3. Traditional Chinese examples no longer show fragments such as `對自`, `的失`, `望感`, or generic terms such as `感到` and `覺得`.
4. Meaningful terms such as `失望`, `女友`, `期待`, `滿足`, and `憤怒` remain eligible when visibly shared.
5. Japanese and English keep readable bounded behavior; ranking, caps, date filtering, and closed-panel behavior remain deterministic.
6. EN/zh-TW/ja copy and focused UI/retrieval tests pass; canonical verification passes.

## Risks

- `Intl.Segmenter` data can vary by runtime version; mitigate by using the repository-pinned desktop/runtime baseline, explicit stop terms, exact regression cases, and a deterministic conservative fallback.
- Over-filtering can hide a weakly related source; safer than presenting generic noise, and reversible without stored state.
- A shortcut that toggles closed could surprise users; make it open-only and scroll/focus the existing region.

## Open Questions

None. These are bounded reversible review corrections inside the already authorized R1 objective.

## Human Decision Required

false; none.

## Recommendation

Implement the bounded corrections and return to the Founder for a new stepwise manual UI review before any promotion package.

## Review Status

approved_with_conditions
