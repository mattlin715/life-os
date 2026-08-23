# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-23-ordinary-v5-reflection-double-submit-correction-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest reviewed: 8d529230b27ef2447418f3c4fc06931d840a2511c4789b6c5153b1a8f9c85d94
- Created at: 2026-08-22T20:11:21.113Z
- Updated at: 2026-08-22T20:18:00.000Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Treat the observed double activation as a bounded interaction defect in the already-authorized ordinary schema-v5 runtime review. Preserve the successful first durable save and prevent the duplicate activation from becoming a misleading error or extra revision.

## Problem Statement

The UI allowed a second Save Answer activation before the first request's committed state disabled the control. The queued mutation then observed an answered Reflection and correctly refused an unchanged correction as `reflection_response_unchanged`. The product surfaced that expected writer refusal as if the original save had failed.

## User Value

The user receives a truthful single-save experience: rapid duplicate activation neither creates extra history nor reports a false storage failure. Genuine later edits remain explicit append-only corrections.

## Relevant Primary Definitions

- `docs/03_Principles.md`: user agency and visible, revisable artifacts.
- `docs/Reflection.md`: saved user responses remain user-authored and distinct from AI prompts.
- `docs/06_Memory.md`: durable state must remain exact and provenance-aware.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: preserve explicit reviewed persistence and provenance.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`: genuine response correction appends; duplicate activation must not invent a correction.

## Current Implementation Context

The ordinary schema-v5 candidate is implemented and automated-verified but remains unpromoted. Disposable migration and restart steps 7B through 8D-1 passed. At Step 8D-2 the first response was visibly durable, while the second rapid activation produced `reflection_response_unchanged`. Rust correctly enforces unchanged-response refusal; the missing boundary is UI in-flight/idempotency handling.

## In Scope

- Add a synchronous per-Experience/per-prompt in-flight guard.
- Disable the exact Save Answer control while its mutation is pending.
- Resolve an already-equal durable response as a no-op without creating a successor revision.
- Add focused regressions for duplicate activation, equal durable state, and genuine changed-response correction.
- Factually record the manual observation in architecture/18.
- Rebuild one ignored review installer after focused and canonical verification.

## Out Of Scope

Rust writer behavior; schema/DDL; migration, backup or restore; provider and ContextPacket behavior; consent; real-profile access; Phase 4; Android; Git promotion; distribution or release.

## Product Constraints

One explicit user save remains the only authority for the response. Duplicate UI activation cannot manufacture an additional review/lifecycle fact.

## Evidence And Provenance Constraints

None. The correction neither retrieves nor transmits historical context.

## Historical Context Constraints

None. No consent or provider boundary is touched.

## Consent Constraints

None. Reflection response saving does not create or reuse historical-context consent.

## AI-Role Constraints

AI prompt authorship and user response authorship remain separate. The correction does not change generated content.

## Privacy Constraints

Unsaved drafts remain session-only. No profile is read by implementation or automated tests; only synthetic/in-memory fixtures are used.

## User-Agency Constraints

The first explicit save persists once. A rapid repeat is ignored, while a later changed response remains a deliberate user correction.

## Acceptance Criteria

1. Two rapid invocations for one prompt result in one durable save call.
2. The Save Answer button is disabled while that exact prompt is saving.
3. If queued durable state already contains the normalized submitted response, no successor revision is requested and no error is shown.
4. A different later response still invokes the normal append-only correction path.
5. Existing draft preservation on failure and successful draft clearing remain intact.
6. Focused tests, Clippy where applicable, and canonical verification pass.
7. Manual review resumes only at Step 8D-2 with the existing disposable schema-v5 profile.

## Risks

- Over-broad debouncing could suppress a genuine later edit; bound the guard to one exact entry/prompt only and release it in `finally`.
- Treating equality as idempotent must compare normalized durable response only and must not hide other dependency/provenance mismatches.
- The existing disposable profile already contains the successful answer; retest must intentionally edit it to a different value rather than replaying the same first-save path.

## Open Questions

None. The Founder confirmed the button was pressed twice, and the active goal explicitly authorizes bounded evidence-based correction cycles.

## Human Decision Required

false; none.

## Recommendation

Proceed with the narrow UI concurrency/idempotency correction and no Rust or policy change.

## Review Status

approved_with_conditions
