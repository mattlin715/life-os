---
status: Draft
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/11
depends:
  - docs/00_Constitution.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/09_AI.md
referenced_by:
  - docs/11_MVP.md
  - docs/adr/ADR-0002-single-source-of-truth.md
  - docs/appendix/Harness.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
---

# 10 Privacy

## Why Privacy Matters

Privacy matters because Life OS asks users to face what is difficult to face.

It deals with memory, emotion, contradiction, identity, relationships, fear, values, and growth.

Privacy in Life OS is not primarily a legal topic.

It is not primarily GDPR.

It is not primarily compliance.

Life OS defines Privacy as:

> The psychological safety required for honest self-reflection.

隱私，不是法律需求。

而是讓人願意誠實面對自己的心理安全感。

Without privacy, users will protect themselves from the system.

And if users must protect themselves from the system, honest Reflection becomes impossible.

## Privacy Enables Reflection

Reflection requires honesty.

Honesty requires safety.

If the user fears that their data will be used against them, sold, exposed, judged, or interpreted without consent, they will not fully reveal what needs to be reflected upon.

They may censor themselves.

They may avoid the most important memories.

They may describe themselves in safer but less truthful ways.

Without honesty, there is no Reflection.

Without Reflection, there is no Growth.

Privacy is therefore not outside the product philosophy.

Privacy is one of the conditions that makes the philosophy possible.

## Privacy Creates Trust

Trust is not created by saying the product is safe.

Trust is created when the product behaves in a way that deserves trust.

Life OS must be clear about what is remembered, why it is remembered, how it is used, and how the user can revise or remove it.

Trust grows when the user can understand the system's behavior.

Trust breaks when the system hides its use of personal data.

Because Life OS handles inner life, trust must be treated as foundational.

## Trust Enables Growth

Growth requires returning to difficult evidence.

A user may need to look at repeated patterns, contradictions, relational pain, avoidance, fear, shame, or decisions they are not proud of.

This cannot happen if the user feels watched by a system that may exploit them.

If the user fears data will be used, they cannot be fully honest.

Without honesty, there is no Reflection.

Without Reflection, there is no Growth.

Privacy protects the conditions under which Growth can happen.

## Data Belongs To The User

Data belongs to the user.

Life OS may help organize, preserve, and reflect evidence, but the evidence is not owned by the system.

The user's memories, self-descriptions, patterns, contradictions, and reflections must not become product assets detached from user agency.

Data should serve the user's self-understanding.

It should not be collected merely because it could be useful someday.

It should not be retained without purpose.

It should not be transformed into leverage over the user.

## Transparency Matters

Transparency matters because hidden data behavior destroys the possibility of trust.

Users should be able to understand:

- What is being remembered.
- Why it is being remembered.
- How it may be used.
- What conclusions or hypotheses are based on it.
- How uncertain those conclusions are.
- How to revise or remove the underlying record.

Transparency is not a technical detail.

It is part of the mirror.

If Life OS reflects something back to the user, the user should be able to see what evidence shaped that reflection.

## Consent Matters

Consent matters because memory without consent becomes surveillance.

A system that remembers without permission may become more powerful, but it becomes less worthy of trust.

Life OS should treat consent as ongoing, not one-time.

The user should be able to change their mind.

The user should be able to correct the record.

The user should be able to remove what no longer belongs in the system.

Consent preserves agency.

And agency is required for Growth.

## The Role Of AI

AI must handle privacy with humility.

AI should not infer beyond what is supported by evidence.

AI should not expose sensitive patterns unnecessarily.

AI should not use private context in ways the user cannot understand.

AI should not pretend that access to more data gives it moral authority.

AI can help users reflect on private evidence, but it must do so with restraint, transparency, and respect for consent.

The more intimate the data, the more careful the AI must become.

## Life OS Position

Life OS treats Privacy as the psychological safety required for honest self-reflection.

Privacy is not a compliance layer added after the product is built.

Privacy is part of the product's moral structure.

If users cannot trust Life OS with their inner life, Life OS should not ask for access to it.

The product must earn the right to remember.

And it must keep earning that right over time.

## Longitudinal Memory Raises The Trust Requirement

The stronger longitudinal memory becomes, the greater the privacy and psychological-safety requirement.

Continuity can improve Reflection. It can also make misuse, accidental exposure, or hidden profiling more harmful. Local-first storage reduces some risk but does not remove the need for consent, transparency, retention limits, and user control.

## Context Use Consent

Users should be able to understand:

- which historical records were referenced;
- why those records were relevant;
- how to exclude a memory from a reflection;
- how to delete an analysis and its sources;
- whether a specific conversation may use longitudinal memory.

Consent to store an Experience is not automatically consent to use it in every future model call.

## Sensitive Inference

More data does not grant unlimited permission to infer:

- mental health diagnosis;
- sexuality;
- religion;
- political identity;
- immutable personality;
- moral character.

Such inference is prohibited unless the user explicitly requests it and the product boundary permits it. Even then, output must remain a revisable, evidence-based hypothesis with visible uncertainty—not a diagnosis, fixed identity, or moral judgment.
