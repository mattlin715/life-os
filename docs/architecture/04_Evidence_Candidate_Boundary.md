---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/09
depends:
  - docs/architecture/00_MVP_Architecture.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/product/00_MVP_User_Flow.md
  - docs/03_Principles.md
  - docs/06_Memory.md
referenced_by: []
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

- Generating mock candidates from one `ExperienceEntry`.
- Showing candidates under the source entry.
- Letting the user edit candidate text before confirmation.
- Letting the user confirm a candidate.
- Letting the user reject a candidate.
- Showing session-level and per-entry review summaries.
- Keeping candidates session-only for this sprint.

The mock flow exists only to validate review behavior.

It does not represent final AI quality.

The optional `originalText` field may preserve the mock output before user editing.

Its purpose is only to distinguish the system-proposed text from the user-edited text.

It is not a full audit log.

The session review summary exists only to support user review.

It is not analytics.

It is not progress scoring.

It does not imply confirmed evidence is final truth.

It does not persist candidate state.

## Candidate Lifecycle

The first lifecycle is:

1. `candidate`: proposed by the mock provider.
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

- Real AI provider calls.
- SQLite persistence for evidence candidates.
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

Evidence candidate generation runs locally in a mock provider.

No entry content is sent to an external provider.

Evidence candidates are not exported.

Evidence candidates are not imported.

Evidence candidates do not create identity labels.

Evidence candidates do not create pattern notes.

Evidence candidates do not produce advice.

The user remains the reviewer.

## Open Questions

- When should evidence candidates become persistent?
- Should rejected candidates be stored for audit, discarded, or stored only with explicit consent?
- What is the minimal persistent evidence schema?
- Should confirmed evidence be included in a future export format?
- How should candidate editing work before reflection prompts are introduced?
