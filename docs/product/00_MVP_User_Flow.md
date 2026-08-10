---
status: Draft
version: 0.9
owner: LIN MENGLUNG
last_updated: 2026/07/14
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
  - docs/architecture/08_Local_Historical_Context_Selection_Foundation.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/02_Philosophy.md
  - docs/11_MVP.md
  - docs/appendix/Harness.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
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

### Daily Reflection Core UX R1 (implemented, not yet promoted or released)

The ordinary screen now leads with recording an Experience. The newest Experience is expanded with one calm next-action summary; older Experiences use progressive disclosure so the Evidence and Reflection loop remains legible without removing any governed capability. Each open Experience places a calm optional local-history shortcut near the top; it opens and brings the existing governed panel into view but does not select a source or imply consent. Optional portability and database-readiness tools are grouped under a secondary Data and diagnostics disclosure. The readiness inspector remains explicit-open, session-only, read-only, and fail-closed.

The locally loaded Experience timeline can be narrowed by a session-only keyword and an optional saved-date range. Keyword and date constraints combine deterministically; dates mean the persisted `createdAt` saved date, not an inferred real-world event date. The range includes both selected days in the device timezone. Incomplete, invalid, inverted, or unresolvable ranges fail closed and show no entries rather than falling back to an unfiltered list. Clearing filters restores the complete bounded loaded list. These controls make no provider call and persist no preference or search history.

When the Pattern gate needs Context Recovery, the UI explicitly acknowledges that confirmed clues and saved Reflection remain valid while explaining that they do not invent missing facts about what happened. The action asks for the specific missing event context rather than presenting an unexplained disabled Pattern control. Local historical suggestions use concrete visible word overlap: Traditional Chinese and Japanese no longer display adjacent-character fragments or generic feeling-report scaffolding as relevance reasons. This is lexical retrieval only, not semantic classification or a cross-experience conclusion.

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

### Historical Context UX (Phase 3A local-only foundation)

The Experience card presents a valid timestamp with date and time, formatted in the active Life OS language and the device timezone. Historical Context source cards present only the valid source date in that same language and timezone. The user may explicitly open a local-history panel for one Experience. Retrieval begins only then; closed panels and initial hydration do not retrieve candidates. The panel shows a small, deterministic set of inspectable prior sources with that date-only presentation, an excerpt, and visible shared-term reasons. The user can include, exclude, clear, or ignore every candidate. Closing the panel preserves its valid ephemeral selection for the current app session. Reopening retrieves fresh local candidates and removes selections which are no longer eligible before the panel is shown. Invalid, empty, or unsupported imported timestamps cannot crash either presentation; the UI shows a localized calm fallback rather than `Invalid Date` or the malformed raw value, and does not render a `dateTime` attribute. A valid imported source remains raw persisted data, while its HTML `dateTime` attribute is separately normalized from the parsed instant. The stored source value remains unchanged.

Candidate selection is ephemeral and local. Selection, panel opening, and silence do not send historical content or create consent, and they do not create a cross-Experience conclusion.

### Governed Historical Question UX (Phase 3B)

After exact source selection, the user may open a separate preflight. Its default view gives a concise statement of what will be sent, the provider/model destination, bounded purpose, included-source counts, sensitive-content reminder, single-generation/purpose scope, and explicit send, cancel, and source-adjustment paths. Before consent, a separate disclosure reveals source/artifact IDs and revisions, authorship/review state, relevance reasons, exact outgoing content, provider retention boundary, packet schema and versions, and the immutable packet/digest relationship with include/exclude controls. Opening either disclosure is not consent. Sending grants consent only for that one immutable packet and the Historical Reflection Question purpose. Changes to source, artifact, task, destination, locale, contract version, or packet digest close or invalidate the preflight and require a new one.

The provider may return one to three neutral questions that cite at least one selected historical source, or an honest no-question result. Recurrence, contradiction, change-over-time, summary, Pattern, Awareness, Growth, advice, diagnosis, sensitive identity inference, and identity finalization are rejected and remain deferred to Phase 4 or later.

## 9. Persistence Qualification

The current implementation persists Experiences, Context Recovery turns, Evidence Candidates, Reflection Prompts/responses/skips, Pattern Candidates, and accepted governed Historical Reflection Question artifacts locally with source relationships and provenance. AI questions and user answers retain distinct authorship metadata. Unsaved Reflection drafts are UI-only, source-scoped by exact Experience and prompt IDs, and are discarded on restart. A failed or stale explicit Save retains its draft. Rejected Evidence/Patterns are excluded from durable storage. Full revision history is not implemented.

Until artifact export is implemented, export must be described accurately as Experience-only JSON/Markdown export.
