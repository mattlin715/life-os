# Product Review

Status: approved

- Sprint ID: 2026-08-11-daily-reflection-journey-summary-dedup
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: 5baa1cbfe6542b020a536ed178aa4eea5519de8148313075ab992aa73e821648
- Created at: 2026-08-11
- Updated at: 2026-08-11

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Treat the duplicated sentence as a presentation defect exposed during Founder review, not as intentional repetition. Preserve the collapsed summary as orientation and the open-body guidance as actionable detail.

## Problem Statement

The same localized next-step value is rendered by the stage heading and the open card. An open stage makes both visible, increasing reading burden without adding meaning.

## User Value

The user sees one concise instruction at a time: a summary while collapsed, detailed guidance while open.

## Relevant Primary Definitions

`docs/03_Principles.md` and `docs/Reflection.md`: protect cognitive flow, preserve user agency, and make reflective steps legible without inflating AI authority.

## Relevant ADRs

ADR-0007 and ADR-0009 are unaffected because this correction changes no artifact, provenance, history, consent, or transmission behavior.

## Current Implementation Context

R2 is promoted. `JourneyStage` currently renders `summary` unconditionally at `DailyReflectionJourneyView.tsx`; `App.tsx` also renders the same state-derived next-step value inside each open Evidence/Reflection/Pattern card. The Founder explicitly authorized the bounded correction during the Proposed architecture/16 walkthrough.

## In Scope

- Hide `JourneyStage` header summary when `open` is true.
- Preserve it when collapsed.
- Add focused Evidence/Reflection/Pattern and EN/zh-TW/ja regression tests.
- Run focused and canonical verification; return to Step 1R.

## Out Of Scope

Any copy rewrite, flow/state change, AI/schema/provider/ContextPacket/consent/persistence change, documentation expansion, promotion, or Git publication.

## Product Constraints

The detailed open-card guidance remains visible and authoritative for the current step. Collapsed stages retain orientation. All locales and all three stage kinds follow the same structural rule.

## Evidence And Provenance Constraints

none; no evidence or stored artifact changes.

## Historical Context Constraints

none; no history behavior changes.

## Consent Constraints

none; no consent behavior changes.

## AI-Role Constraints

No AI behavior changes.

## Privacy Constraints

No data access, persistence, telemetry, or transmission changes.

## User-Agency Constraints

The correction removes redundant text while preserving every explicit control and detailed instruction.

## Acceptance Criteria

1. A collapsed Evidence, Reflection, or Pattern stage displays its localized summary.
2. An open stage does not render its header summary but does render its child guidance.
3. The rule is verified for English, Traditional Chinese, and Japanese.
4. No i18n strings or product state logic change.
5. Focused tests and canonical verification pass.
6. Only the two authorized product/test files plus workflow evidence change.

## Risks

Hiding the summary could remove all guidance if an open card lacked its own next-step text. Existing `App.tsx` evidence confirms all three stage bodies supply detailed guidance; tests cover open child visibility.

## Open Questions

none

## Human Decision Required

false; the Founder already supplied the exact bounded authorization.

## Recommendation

Approve the smallest structural condition in `JourneyStage`; do not duplicate stage-specific branching or rewrite localized copy.

## Review Status

approved
