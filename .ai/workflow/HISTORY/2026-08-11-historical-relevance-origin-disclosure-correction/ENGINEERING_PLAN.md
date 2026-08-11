# Engineering Plan

Status: approved

- Sprint ID: 2026-08-11-historical-relevance-origin-disclosure-correction
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: d9d39a548e8d6bd0e901029ef94230c486111a5de2411df26ad31beb08d9fcfe
- Created at: 2026-08-11
- Updated at: 2026-08-11

## Approved Product Boundary

Implement presentation provenance only. Do not change matching, score, caps, eligibility, aggregate reasons, or governed transport.

## Existing Implementation Understanding

retrieve.ts already computes body, Evidence, and Reflection term arrays before flattening. App.tsx displays flattened terms beside only sourceExcerpt.

## Affected Modules

src/historicalContext/types.ts, retrieve.ts, retrieve.test.ts, src/app/App.tsx, i18n.ts, i18n.test.ts, and workflow evidence.

## Proposed Design

Add optional local visibleMatches containing kind, exact artifact ID where applicable, terms, and bounded excerpt. Render localized rows in the explicit-open card. Keep reasons unchanged.

## Alternatives Considered

Copy-only clarification is not inspectable. Experience-only retrieval would change approved matching and ranking.

## Data Lifecycle Impact

none

## SQLite Or Migration Impact

none

## Provenance Impact

No governed provenance change; UI origin is derived ephemerally.

## Historical Context Impact

Presentation only; algorithm and eligibility unchanged.

## Consent Impact

none

## Provider Transmission Impact

none

## Import And Export Impact

none

## Test Strategy

Focused retrieval grouping, excerpt bounds, aggregate compatibility, ineligible exclusion, i18n parity, typecheck, then canonical verification.

## Repository Verification Strategy

scripts/verify.ps1, workflow validation, diff check, prohibited-path audit, and no staged files.

## Manual UI Verification

Step 9R verifies each term is tied to Experience, confirmed Evidence, or saved Reflection with a matching excerpt.

## Rollback Or Recovery Strategy

Revert the optional field and rendering; no stored state is involved.

## Documentation Impact

Workflow evidence only before Founder acceptance.

## ADR Impact

none

## Risk Level

low to moderate

## Escalation Decision

Any need to alter aggregate reasons, ranking, eligibility, packet assembly, persistence, or provider behavior returns to Founder.
