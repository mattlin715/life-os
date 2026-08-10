# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-11-daily-reflection-completion-ux-r2
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: c329edb80e0ab3bd5bcbcc4dedd33c9c3af6bf12
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-10T21:26:00.000Z
- Updated at: 2026-08-10T21:26:00.000Z

## Mission Interpretation

R2 is a product-comprehension and completion slice over the promoted R1 flow. It must reduce cognitive load without hiding authority boundaries or inventing new AI behavior. The system should reflect the user's durable progress, point to one gentle next action, and let a completed reflection rest.

## Problem Statement

R1 restored the primary Experience flow and added truthful next-step copy, but every stage remains expanded. Users must scan multiple disabled or optional controls to infer what matters now, the summary cannot take them to the named step, and completion lacks a clear record-derived surface.

## User Value

The user sees what is active, what is complete, and what is optional without learning internal artifact terminology first. Reopening remains explicit. Completion respects stopping as a valid choice rather than pushing Pattern generation or historical context.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: human agency, reflection before answer, evidence before conclusion, and Mirrors not Oracles.
- `docs/03_Principles.md`: uncertainty and user-controlled interpretation remain visible.
- `docs/06_Memory.md`: durable state must remain provenance-preserving and user-controlled.
- `docs/Reflection.md`: Reflection is user-owned meaning-making; Pattern is revisable and not identity.
- `docs/09_AI.md` and `docs/10_Privacy.md`: no new provider authority, inference, silent profiling, or transmission.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: render current reviewed artifact state without weakening provenance.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`: optional history remains separate from selection, consent, and transmission.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`: no lifecycle or schema authority is changed by this presentation slice.

## Current Implementation Context

- Promoted R1 at merge `c329edb` implements Experience-first composition, entry summaries, local filters, a top history entry point, three-stage explanations, and local-lexical-v2.
- `src/app/dailyReflectionFlow.ts` currently returns only a coarse next action and does not resolve Context Recovery, dirty drafts, optional Pattern states, or historical-question availability.
- `src/app/App.tsx` renders all three review cards expanded and the summary text is not a navigation control.
- Book One still factually describes R1 as unpromoted; that is documentation drift, not product authority.
- Production schema and startup maximum remain v4.

## In Scope

- Pure deterministic per-Experience journey state from already loaded records and ephemeral draft/pending state.
- Progressive disclosure with one active stage, collapsed truthful summaries for completed stages, understandable future stages, and explicit reopen controls.
- Actionable summary/next-step navigation with keyboard focus and reduced-motion behavior.
- Calm core-completion surface derived only from Experience, confirmed Evidence, saved/skipped Reflection, optional existing Pattern review state, and optional persisted Historical Reflection Questions.
- Explicit choices to stop, reopen, inspect optional Pattern/history, or focus the composer.
- Three-language parity, tests, factual Book One updates, and verification.

## Out Of Scope

All schema, migration, provider, ContextPacket, consent, persistence-policy, inference-policy, Phase 4, identity, deployment, and Harness changes. Navigation may not call generation. Pattern and history may not become completion requirements.

## Product Constraints

- Exactly one primary active stage for incomplete core flow.
- Completion is reached by saved or explicitly skipped Reflection outcomes after confirmed Evidence; Pattern is optional.
- Context Recovery invitation is represented without treating its unsaved text as durable progress.
- A dirty Reflection draft blocks completion and remains the active work.
- Rejected Pattern is a valid optional terminal state, not failed core completion.

## Evidence And Provenance Constraints

Summaries may count or display only already loaded artifact states. They must not reinterpret candidate text or convert AI output into user Evidence. No provenance mutation occurs.

## Historical Context Constraints

History remains optional and local until the existing panel flow. Existing Historical Reflection Questions may be listed only as saved questions; no cross-time conclusion is produced.

## Consent Constraints

No consent is created or implied. Opening or navigating to history remains distinct from selecting sources, reviewing a packet, consenting, and sending.

## AI-Role Constraints

The resolver and completion surface are deterministic UI logic. They do not call a model, summarize content, infer recurrence, or decide meaning.

## Privacy Constraints

Stage disclosure state is session-only. No analytics, preferences, extra storage, clipboard, or background retrieval is added.

## User-Agency Constraints

The user can stop after core Reflection, reopen completed stages, skip a prompt, decline Pattern/history, and return to the composer. Navigation changes focus only.

## Acceptance Criteria

1. Resolver covers saved Experience, Context Recovery invited, missing/generated/reviewed Evidence, missing/suggested/skipped/answered/dirty Reflection, core completion, optional Pattern available/candidate/confirmed/rejected, and saved historical-question availability.
2. Incomplete flows expose one active stage; completed stages are collapsed with accurate summaries and explicit reopen.
3. Future stages explain prerequisites and cannot be mistaken for completed or required work.
4. The summary next-step control focuses the correct stage and never triggers generation.
5. Completion uses record facts only and treats Pattern/history as optional.
6. English, Traditional Chinese, and Japanese render equivalent meaning.
7. Existing provider fallback, stale-generation, persistence, historical consent, R1 filtering, and schema-v4 behavior remain unchanged.
8. Keyboard focus is visible; reduced-motion users receive no forced smooth scrolling.

## Risks

- Over-collapsing can hide reviewed content or make correction harder; explicit reopen controls and summaries mitigate this.
- State derivation can disagree with current controls; one pure resolver with exhaustive tests reduces drift.
- Programmatic focus can be disruptive; it occurs only after explicit navigation actions.
- Completion wording could imply judgment; copy must describe records, not evaluate the user.

## Open Questions

none. The Founder objective supplies the consequential product boundary; implementation details are reversible within it.

## Human Decision Required

false; no decision IDs.

## Recommendation

Proceed with one pure resolver, a small stage-disclosure component, App integration, existing-record completion surface, three-language copy, focused tests, and factual documentation correction. Preserve all authority fences.

## Review Status

approved_with_conditions
