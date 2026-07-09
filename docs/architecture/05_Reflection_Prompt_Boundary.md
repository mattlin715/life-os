---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/09
depends:
  - docs/architecture/00_MVP_Architecture.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/04_Evidence_Candidate_Boundary.md
  - docs/product/00_MVP_User_Flow.md
  - docs/Reflection.md
  - docs/03_Principles.md
  - docs/06_Memory.md
referenced_by: []
---

# 05 Reflection Prompt Boundary

## Purpose

This document defines the first `ReflectionPrompt` boundary for Life OS MVP.

It connects:

Confirmed Evidence  
-> Reflection Prompt  
-> User Response Optional

This boundary does not make the system wiser.

It makes the product boundary visible: reflection starts from user-confirmed evidence, and meaning remains with the user.

## Why Reflection Prompts Are Not Answers

Reflection prompts are questions.

They are not conclusions.

They are not advice.

They are not diagnosis.

They are not identity labels.

A prompt may help the user look again at confirmed evidence, but it must not decide what the evidence means.

The system may propose a question.

The user may answer, skip, ignore, or disagree.

## MVP Scope

The MVP reflection prompt boundary supports:

- Generating mock reflection prompts from confirmed evidence only.
- Creating one open-ended question per confirmed evidence item.
- Showing prompts under the source entry's evidence section.
- Letting the user write an optional answer.
- Letting the user skip a prompt.
- Showing a per-entry reflection summary.
- Keeping prompts and answers session-only for this sprint.

The mock flow exists only to validate the product boundary.

It does not represent final AI quality.

## Prompt Lifecycle

The first lifecycle is:

1. `suggested`: proposed from confirmed evidence by the mock provider.
2. `answered`: the user saved an optional response.
3. `skipped`: the user skipped the prompt.

The current state transition rule is intentionally small:

- `suggested` can become `answered`.
- `suggested` can become `skipped`.
- `answered` can update and re-save its response.
- `answered` cannot become `skipped`.
- `skipped` cannot become `answered` in this sprint.

A future reopen flow may let the user revisit skipped prompts.

That flow is deferred because skipped prompt persistence is not defined yet.

No prompt may create a conclusion automatically.

No prompt may create a pattern note automatically.

No prompt may create a growth note automatically.

No prompt may become persisted memory in this sprint.

## User Response Optional

The user response is optional.

Answering a prompt is user-authored reflection.

Skipping a prompt is also valid.

The UI must not frame skipping as failure.

The UI must not require a response before the user can continue using the entry.

## What Is Not Included

This sprint does not include:

- Real AI provider calls.
- SQLite persistence for reflection prompts.
- Reflection prompt export or import.
- Pattern schema.
- Pattern note generation.
- Growth note generation.
- Scoring.
- Diagnosis.
- MBTI or personality inference.
- Identity labels.
- Advice.
- Automatic conclusions.
- Automated coaching language.

## Privacy / Agency Guardrails

Reflection prompt generation runs locally in a mock provider.

No entry content is sent to an external provider.

Prompts are generated only from user-confirmed evidence, not raw unreviewed candidates.

Prompts and prompt answers are not saved to SQLite.

Prompts and prompt answers are not exported.

Prompts and prompt answers are not imported.

The user remains the meaning-maker.

The system remains a mirror.

## Open Questions

- When should reflection prompts become persistent, if ever?
- Should skipped prompts be stored, discarded, or stored only with explicit consent?
- Should future prompts preserve their exact source evidence text for audit?
- Should prompt answers become part of a future memory model only after explicit user confirmation?
- How should future reflection output connect to pattern notes without turning questions into conclusions?
