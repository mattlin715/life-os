---
status: Implemented
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/12
depends:
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/04_Evidence_Candidate_Boundary.md
  - docs/Reflection.md
referenced_by:
  - docs/product/00_MVP_User_Flow.md
---

# 05 Reflection Prompt Boundary

## Purpose

Confirmed Evidence -> Reflection Prompt -> Optional User Response.

Prompts are questions, not answers, advice, diagnoses, or identity labels. The user may answer, skip, ignore, or disagree.

## Implemented boundary

- OpenAI, Gemini, and mock receive the same validated task-specific Context Packet.
- Only confirmed, current-Experience evidence may support a prompt.
- Suggested, answered, and skipped states persist locally in SQLite schema v3.
- A user response remains user-authored context. Skipping adds no context and is not treated as rejection feedback.
- Rejecting source evidence removes dependent reflection records atomically.
- Reflection artifacts remain outside Experience-only export/import.

## Guardrails

A prompt cannot automatically create a conclusion, Pattern, Growth note, diagnosis, or identity claim. Persistence preserves review continuity; it does not turn a prompt or response into product truth.

## Deferred

Cross-experience reflection, artifact portability, reopen/reset UX, and retained evaluation feedback require separate governed work.
