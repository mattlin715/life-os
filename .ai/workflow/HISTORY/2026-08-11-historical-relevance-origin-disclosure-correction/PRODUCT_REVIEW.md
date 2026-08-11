# Product Review

Status: approved

- Sprint ID: 2026-08-11-historical-relevance-origin-disclosure-correction
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: d9d39a548e8d6bd0e901029ef94230c486111a5de2411df26ad31beb08d9fcfe
- Created at: 2026-08-11
- Updated at: 2026-08-11

## Mission Interpretation

Treat the mismatch as an explainability defect, not semantic inference. Preserve retrieval and governance; expose where matched words were found.

## Problem Statement

Candidate ranking aggregates matches from Experience text, confirmed Evidence, and eligible saved Reflection while the card displays only the Experience excerpt.

## User Value

Users can validate why a prior moment appeared before selecting it.

## Relevant Primary Definitions

docs/03_Principles.md, docs/06_Memory.md, and docs/Reflection.md require selective, inspectable, user-controlled context.

## Relevant ADRs

ADR-0009 keeps retrieval, selection, consent, packet assembly, and transmission separate. The governed aggregate relevance contract remains unchanged.

## Current Implementation Context

local-lexical-v2 computes body, Evidence, and Reflection matches separately, then flattens them into one reasons list. Exact matching artifact IDs already remain distinct.

## In Scope

Local-only per-origin terms and bounded excerpts; aggregate compatibility; EN/zh-TW/ja and eligibility tests; Step 9R.

## Out Of Scope

Persistence, eligibility, ranking, selection, consent, provider, packet, schema, Phase 4, whole-history, and promotion.

## Product Constraints

Use the exact existing term sets; do not create a second matcher or reorder candidates.

## Evidence And Provenance Constraints

Only confirmed Evidence and eligible saved user-authored Reflection may appear.

## Historical Context Constraints

Retrieval remains explicit-open, local, bounded, deterministic, session-only, and capped.

## Consent Constraints

Selection is not consent; neither state changes.

## AI-Role Constraints

No AI or semantic inference change.

## Privacy Constraints

Excerpts appear only in the explicit-open local panel and are not newly persisted or transmitted.

## User-Agency Constraints

The user can inspect support before include/exclude and ignore every candidate.

## Acceptance Criteria

1. Aggregate reasons, score, order, and caps remain unchanged.
2. Body, confirmed-Evidence, and saved-Reflection matches disclose exact origin.
3. Matching artifact excerpts are bounded and ineligible artifacts never appear.
4. Three-language labels are equivalent.
5. Governed packet tests and canonical verification pass.
6. No staged files or prohibited diffs.

## Risks

Additional text could increase density; show only exact matching records and bounded excerpts.

## Open Questions

none

## Human Decision Required

false; Founder authorization is explicit.

## Recommendation

Approve local-only origin disclosure while retaining aggregate relevance as the sole governed packet input.

## Review Status

approved
