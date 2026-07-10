---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/10
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
referenced_by: []
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
??Evidence  
??Reflection  
??Awareness  
??Growth

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

### Current Mock Boundary

The current MVP implementation supports the mock boundary:

Confirmed evidence  
-> Reflection prompt  
-> User response optional  
-> Pattern candidate hypothesis  
-> User review

The user can generate reflection prompts only after confirming at least one evidence candidate.

The prompt is a question, not an answer.

The user can answer or skip.

The user can generate a pattern candidate after confirming at least one evidence candidate.

Answered reflection responses may be used as optional context, but skipped and unanswered prompts are not treated as conclusions.

Pattern candidates are hypotheses for review, not identity labels or conclusions.

Evidence candidates, reflection prompts, reflection responses, and pattern candidates are session-only in this sprint.

They are not persisted, exported, imported, or used to generate growth notes.

## 3. Flow 1: Pattern Review

After multiple entries:

1. User opens timeline.
2. System surfaces repeated pattern candidates.
3. User reviews evidence.
4. User reflects on whether pattern is meaningful.
5. User may write growth note or future action.

Pattern review should feel like a mirror, not a diagnosis.

The system should show what evidence supports the pattern candidate.

The current MVP only supports per-entry, session-only pattern candidates.

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

The system may keep a local audit trail only if the user understands and consents.

### User wants to delete entry

The user can delete the entry and associated evidence, reflection, and pattern notes.

Deletion should be clear and understandable.

In the current MVP, associated evidence, reflection, and pattern candidates are session-only and disappear with the entry or after app restart.

### AI provider unavailable

The user can still save the raw experience locally.

The system should not block journaling or local review because AI is unavailable.

### User wants export

The user can export entries and structured evidence in a portable format.

Export supports ownership and trust.

## 7. MVP Success Criteria

The MVP succeeds if the user feels:

- I saw something I had not seen before.
- I can inspect the evidence.
- I do not feel judged.
- I remain in control.
- I want to try this again.

The MVP does not succeed because the UI is polished.

It succeeds if the core loop proves useful.
