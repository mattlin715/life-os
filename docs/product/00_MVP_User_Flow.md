---
status: Draft
version: 0.4
owner: LIN MENGLUNG
last_updated: 2026/07/12
depends:
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/07_Awareness.md
  - docs/08_Growth.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/architecture/00_MVP_Architecture.md
  - docs/architecture/04_Evidence_Candidate_Boundary.md
  - docs/architecture/05_Reflection_Prompt_Boundary.md
  - docs/architecture/06_Pattern_Candidate_Boundary.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/02_Philosophy.md
  - docs/11_MVP.md
  - docs/appendix/Harness.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
referenced_by:
  - docs/11_MVP.md
  - docs/12_Roadmap.md
---

# 00 MVP User Flow

## Purpose

This document defines the first usable Life OS flow.

It translates the Book Zero core loop into a minimal interaction that can be built and tested.

It does not define the full product.

## 1. User Goal

The user wants to reflect on a real life experience and understand themselves better.

The user does not need a complete life system yet.

The user needs one trustworthy loop:

Experience
-> Evidence
-> Reflection
-> Awareness
-> Growth

## 2. Flow 0: Single Experience Reflection

1. User writes one experience.
2. System extracts evidence candidates.
3. User reviews / edits / rejects evidence.
4. System generates reflection prompts.
5. User answers or skips.
6. System proposes possible pattern note.
7. User confirms / edits / rejects.
8. Entry is saved to local timeline.

The system should keep the user in control at every step.

AI output is never final.

### Current implemented boundary

The current per-Experience flow is:

1. Save an Experience locally.
2. Run the deterministic Context Sufficiency Gate.
3. For sparse input, invite one optional Context Recovery answer.
4. Generate direct Evidence observations from a validated task-specific Context Packet.
5. Confirm, edit, or reject Evidence.
6. Generate and optionally answer/skip Reflection prompts.
7. Permit a tentative Pattern only when context is sufficient for that inference depth.
8. Persist valid source-scoped artifacts and provenance locally.

Skipping clarification permits observation but does not increase sufficiency or unlock Pattern generation. Rejected Evidence/Patterns are not durable, and dependent Reflection records are removed. Export/import remains Experience-only.

## 3. Flow 1: Pattern Review

After multiple entries:

1. User opens timeline.
2. System surfaces repeated pattern candidates.
3. User reviews evidence.
4. User reflects on whether pattern is meaningful.
5. User may write growth note or future action.

Pattern review should feel like a mirror, not a diagnosis.

The system should show what evidence supports the pattern candidate.

The current MVP supports only per-entry, locally persisted pattern candidates.

Cross-entry pattern review remains deferred.

## 4. UX Principles

- Mirror before advice.
- Evidence before conclusion.
- User owns interpretation.
- AI output is always editable.
- No automatic identity label.
- No diagnosis.
- No pushy notification.

The product should make reflection easier without making AI feel like an authority.

## 5. Empty State

The first-time user should not face a dashboard full of empty concepts.

The first screen should invite one small action:

> Write one experience you want to understand.

The empty state should avoid promising transformation.

It should create a calm, low-pressure starting point.

Suggested empty state content:

- One writing field.
- One short prompt.
- No metrics.
- No personality labels.
- No complex navigation.

## 6. Failure States

### AI output feels wrong

The user can reject, edit, or regenerate.

The system should treat wrong AI output as normal, not as user error.

### User rejects evidence

Rejected evidence should not be used as confirmed evidence.

The current boundary removes rejected output from durable storage. No hidden evaluation trail is retained.

### User wants to delete entry

The user can delete the entry and associated evidence, reflection, and pattern notes.

Deletion should be clear and understandable.

In the current MVP, source-scoped artifacts survive restart and are removed atomically when the Experience is edited or deleted.

### AI provider unavailable

The user can still save the raw experience locally.

The system should not block journaling or local review because AI is unavailable.

### User wants export

The user can currently export Experience entries in JSON or Markdown.

Export supports ownership and trust.

Reviewed evidence, reflection, conversation, and pattern artifacts require a versioned portability design before they are added to export.

## 7. MVP Success Criteria

The MVP succeeds if the user feels:

- I saw something I had not seen before.
- I can inspect the evidence.
- I do not feel judged.
- I remain in control.
- I want to try this again.

The MVP does not succeed because the UI is polished.

It succeeds if the core loop proves useful.

## 8. Revised Flow: Context Before Insight

The next MVP flow is:

1. User writes an Experience.
2. System checks whether context is sufficient for the requested inference.
3. If useful, the system invites a small number of clarifying questions.
4. User answers, skips, or stops Context Recovery.
5. System generates Evidence Candidates proportionate to available context.
6. User reviews evidence.
7. System offers Reflection.
8. System may propose a Pattern Hypothesis with visible sources and uncertainty.
9. Reviewed artifacts persist locally with provenance. Reflection edits remain UI drafts until explicitly saved; Save stores normalized (trimmed) text and clears its exact draft only after the durable commit succeeds. Unsaved text is not provider context and blocks Pattern generation.
10. Cross-Experience Reflection remains deferred.

### Context Recovery UX

- Questions are invitations, not requirements.
- Ask only what may materially improve understanding.
- Do not repeat questions the user has already answered.
- Do not force emotional depth.
- If the user skips, reduce inference depth and continue safely.
- Permit an honest outcome of “insufficient context” or “no meaningful pattern found.”
- Pattern availability uses the same Context Gate decision in the button and the execution guard. When Pattern needs more event context, the UI states that plainly and offers one optional Add Context action; saved Reflection answers do not substitute for that missing context.

### Historical Context UX (deferred)

Cross-Experience retrieval is not implemented in this vertical slice.

## 9. Persistence Qualification

The current implementation persists Experiences, Context Recovery turns, Evidence Candidates, Reflection Prompts/responses/skips, and Pattern Candidates locally with source relationships and provenance. AI questions and user answers retain distinct authorship metadata. Unsaved Reflection drafts are UI-only, source-scoped by exact Experience and prompt IDs, and are discarded on restart. A failed or stale explicit Save retains its draft. Rejected Evidence/Patterns are excluded from durable storage. Full revision history is not implemented.

Until artifact export is implemented, export must be described accurately as Experience-only JSON/Markdown export.
