---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/10
depends:
  - docs/architecture/00_MVP_Architecture.md
  - docs/architecture/04_Evidence_Candidate_Boundary.md
  - docs/architecture/05_Reflection_Prompt_Boundary.md
  - docs/product/00_MVP_User_Flow.md
referenced_by: []
---

# 06 Pattern Candidate Boundary

## Purpose

This document defines the first MVP boundary for pattern candidates.

The boundary connects:

Confirmed Evidence + Reflection Context  
-> Pattern Candidate  
-> User Review

It does not define the user.

It exists to prove that Life OS can surface a tentative pattern for review while preserving the principle:

> We Build Mirrors, Not Oracles.

## Why Pattern Candidates Are Hypotheses

A pattern candidate is a hypothesis because it is produced from partial context.

The system only sees selected entries, confirmed evidence, and optional reflection responses. It does not know the whole person, the whole history, or the right interpretation.

Therefore, pattern language must remain uncertain:

- "One possible pattern to review is..."
- "This may suggest..."
- "You may want to examine whether..."

The product must not say:

- "You are..."
- "Your personality is..."
- "This proves..."
- "You should..."

Only the user can decide whether a candidate is meaningful.

## MVP Scope

The MVP pattern candidate boundary supports only:

- Generating one mock pattern candidate for one entry.
- Using confirmed evidence as the required source.
- Optionally using answered reflection responses as user-authored context.
- Showing the candidate text under the entry reflection section.
- Letting the user confirm or reject the candidate.
- Showing a simple per-entry pattern summary.

Pattern candidates are session-only in this sprint.

They disappear after app restart.

## Required Inputs

Pattern candidate generation requires:

- One `ExperienceEntry`.
- At least one `EvidenceCandidate` with status `confirmed`.

Pattern candidate generation may also use:

- `ReflectionPrompt` records with status `answered`.
- The user-authored `response` text from answered prompts.

Skipped prompts and unanswered suggested prompts are not treated as reflection context.

Rejected evidence is not used.

## Candidate Lifecycle

Pattern candidates use the candidate lifecycle:

- `candidate`
- `confirmed`
- `rejected`

Initial mock output is always `candidate`.

The system does not auto-confirm pattern candidates.

Confirmed means "the user chose to keep this candidate for review in the current session." It does not mean the candidate is an objective truth.

Rejected means "the user does not want to keep this candidate in the current session."

## User Review Requirement

The UI must show pattern candidates as reviewable hypotheses.

The user must be able to:

- See the candidate text.
- See that it is a hypothesis, not a conclusion.
- Confirm the candidate.
- Reject the candidate.

The candidate must not silently create a growth note, identity label, or persistent memory.

## What Is Not Included

This sprint does not include:

- Real AI provider integration.
- Pattern candidate persistence.
- A SQLite pattern table.
- Export/import of pattern candidates.
- Cross-entry pattern search.
- Dashboard, charts, or scores.
- Personality analysis.
- MBTI or personality types.
- Diagnosis.
- Advice.
- Identity labels.
- Growth notes.
- Automatic conclusions.

## Privacy / Agency Guardrails

Pattern candidates must preserve user agency:

- They must be visibly tentative.
- They must be generated from user-confirmed evidence.
- They must remain inspectable.
- They must require user review.
- They must not be persisted without a separate schema and consent decision.
- They must not be exported/imported in the current boundary.
- They must not infer identity, mental health status, or personality type.

The user owns the interpretation.

The system only offers a mirror.

## Open Questions

- Should future pattern notes be persisted only after explicit confirmation?
- Should rejected pattern candidates be stored for audit, discarded, or stored only with consent?
- How should cross-entry pattern candidates show supporting evidence without becoming a dashboard?
- How should pattern candidates connect to Awareness and Growth without turning into advice?
- What export/import semantics are needed if pattern persistence is added later?
