---
status: Draft
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/12
depends:
  - docs/architecture/00_MVP_Architecture.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/product/00_MVP_User_Flow.md
  - docs/03_Principles.md
  - docs/06_Memory.md
referenced_by:
  - docs/architecture/07_Persisted_Context_Recovery_Vertical_Slice.md
---

# 04 Evidence Candidate Boundary

## Purpose

This document defines the first `EvidenceCandidate` boundary for Life OS MVP.

It exists to connect:

Experience  
-> Evidence Candidate  
-> User Review

This sprint does not try to build intelligent analysis.

It builds the product boundary that prevents proposed evidence from becoming truth without the user.

## Why Evidence Candidates Are Not Truth

Evidence candidates are suggestions.

They may be useful.

They may be wrong.

They may be incomplete.

They may reflect only a surface reading of the entry.

Life OS must preserve the distinction between:

- What the user wrote.
- What the system proposed.
- What the user confirmed.

The candidate state exists because evidence should not become truth by default.

## MVP Scope

The MVP evidence candidate boundary supports:

- Generating candidates from one `ExperienceEntry` through the shared Harness.
- Showing candidates under the source entry.
- Letting the user edit candidate text before confirmation.
- Letting the user confirm a candidate.
- Letting the user reject a candidate.
- Showing session-level and per-entry review summaries.
- Persisting candidates locally with their source Experience and review state.

The local mock remains a deterministic fallback for review-boundary development. When configured, OpenAI or Gemini receives only the validated evidence Context Packet; provider transport does not change review behavior.

The optional `originalText` field may preserve the mock output before user editing.

Its purpose is only to distinguish the system-proposed text from the user-edited text.

It is not a full audit log.

The session review summary exists only to support user review.

It is not analytics.

It is not progress scoring.

It does not imply confirmed evidence is final truth.

Candidate state persists locally; it remains a review state, not product truth.

## Candidate Lifecycle

The first lifecycle is:

1. `candidate`: proposed by an OpenAI, Gemini, or local mock provider under the shared Harness contract.
2. `candidate` with edited text: revised by the user, but still not truth.
3. `confirmed`: manually accepted by the user for this session.
4. `rejected`: manually rejected by the user.

No candidate may skip user review.

No candidate may become confirmed automatically.

Edited candidate text still remains a candidate until the user confirms it.

Confirmation means the user accepts the edited candidate as meaningful for this session.

It does not mean the candidate has become final truth.

## User Review Requirement

Every evidence candidate must remain reviewable.

The user must be able to see:

- Source entry.
- Candidate text.
- Candidate kind.
- Candidate status.

The user must be able to edit candidate text before confirmation.

Editing candidate text is part of user agency.

The user must be able to confirm or reject without the system producing a conclusion.

For the current MVP boundary, confirmed or rejected candidates cannot be edited again.

If a user needs to change a confirmed or rejected candidate, a future reset or reopen flow should be designed explicitly.

## What Is Not Included

This sprint does not include:

- Evidence export or import.
- Editing candidate kind.
- Analytics dashboard.
- Progress scoring.
- Reflection prompts.
- Pattern notes.
- Identity labels.
- Advice.
- Diagnosis.
- Personality inference.
- Complex evidence ontology.

## Privacy / Agency Guardrails

Evidence candidate generation uses the environment-configured OpenAI or Gemini provider when available, otherwise the local mock fallback. The provider receives only the validated task-specific Context Packet.

When a real provider is selected, the current Experience and bounded, task-specific Context Packet may be sent to that provider. Local SQLite persistence remains separate from provider-side processing. Provider/model/Harness/prompt provenance remains visible on generated artifacts.

Evidence candidates are not exported.

Evidence candidates are not imported.

Evidence candidates do not create identity labels.

Evidence candidates do not create pattern notes.

Evidence candidates do not produce advice.

The user remains the reviewer.

## Open Questions

- What explicit consent and governance would be required before retaining rejected output for evaluation?
- When would artifact portability become valuable enough to version?
- Should confirmed evidence be included in a future export format?
