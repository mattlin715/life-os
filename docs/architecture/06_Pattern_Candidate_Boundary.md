---
status: Implemented
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/12
depends:
  - docs/architecture/04_Evidence_Candidate_Boundary.md
  - docs/architecture/05_Reflection_Prompt_Boundary.md
referenced_by:
  - docs/product/00_MVP_User_Flow.md
---

# 06 Pattern Candidate Boundary

## Purpose

Confirmed Evidence + Valid Answered Reflection + Sufficient Context -> Tentative Pattern Candidate -> User Review.

A Pattern Candidate is a hypothesis, never an identity label or conclusion.

## Implemented boundary

- Pattern generation is blocked unless the Context Sufficiency Gate returns `sufficient_for_tentative_hypothesis`.
- Provider input contains only the current Experience, answered Context Recovery turns, confirmed evidence, and answered reflections whose source evidence is still confirmed.
- Candidates persist locally with source and provider/model/Harness/prompt provenance.
- Confirmed means useful for continued review, not objectively true.
- Rejected candidates are removed from durable storage and never hydrated or reused.
- Pattern artifacts are excluded from Experience-only export/import.

## Guardrails

The system must preserve uncertainty and may honestly report insufficient context. It must not manufacture recurrence from one event, diagnose, advise, score, or finalize identity.

## Deferred

Cross-experience retrieval, longitudinal Pattern review, Growth integration, and artifact portability remain out of scope.
