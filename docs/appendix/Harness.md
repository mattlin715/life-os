---
status: Draft
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/11
depends:
  - docs/00_Constitution.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
referenced_by:
  - README.md
  - AI_CONTRIBUTOR_GUIDE.md
  - IMPLEMENTATION_GUIDE.md
  - docs/00_Index.md
  - docs/12_Roadmap.md
  - docs/architecture/00_MVP_Architecture.md
---

# Harness Learning Architecture

## Purpose

This document is the Primary Definition of the Life OS Harness Learning Architecture.

The Harness is the governed system that constrains AI behavior, assembles context, preserves user-reviewed evidence, and improves interaction quality from evaluated feedback—without allowing a model to rewrite product philosophy.

It is not merely a prompt collection.

## Why Harness Exists

Models can change.

Providers can change.

Life OS's character, ethics, uncertainty discipline, privacy boundaries, and relationship with the user must not drift with them.

The Harness makes Book Zero operational across model calls and development cycles. It separates provider capability from product behavior and turns dogfooding into traceable, reversible learning.

## Harness Components

### 1. Constitutional Layer

The highest behavioral constraints:

- `Life_OS_Manifesto.md`
- `docs/00_Constitution.md`
- `docs/03_Principles.md`
- `docs/09_AI.md`
- `docs/10_Privacy.md`

This layer defines what AI is for, what it must never become, and which principles lower layers cannot override.

### 2. Context Assembly Layer

The Context Assembly Layer selects and labels the minimum relevant material for a task:

- Current Experience.
- Clarifying Conversation.
- Relevant Historical Experiences.
- Confirmed Evidence.
- User-authored Reflections.
- Confirmed, rejected, or revised Pattern Hypotheses.
- Language and locale.
- Explicit user preferences.

Selection must be relevance-based, consent-aware, and provenance-preserving. More context is not automatically better context.

### 3. Behavior Protocol Layer

Every provider must obey:

- Context Before Insight.
- Evidence before Conclusion.
- Reflection before Answer.
- Mirror before Advice.
- Calibrated uncertainty language.
- No diagnosis.
- No identity finalization.
- The user owns final interpretation.

These are product protocols, not provider preferences.

### 4. Memory Governance Layer

Memory governance covers:

- Raw Experience.
- Confirmed Evidence.
- Reflection Response.
- Pattern Hypothesis.
- User confirmation, rejection, or revision.
- Provenance.
- Consent.
- Edit, delete, and export.
- Retention boundaries.

AI inference and user-confirmed evidence must remain distinct. A derived artifact must never lose its source relationships.

### 5. Evaluation Layer

Evaluation asks:

- Did the response use enough context for its level of inference?
- Did it cite relevant evidence?
- Did it distinguish observation from inference?
- Did it preserve uncertainty?
- Did the user feel understood rather than judged?
- Did it improve Reflection?
- Did it become too generic, too confident, or too verbose?

Evaluation results are evidence for human review. They are not autonomous permission to modify behavior.

### 6. Learning Layer

The Learning Layer captures:

- Dogfooding feedback.
- User corrections.
- Rejected candidates.
- Prompt versioning.
- Evaluation cases.
- Regression tests.

Harness Learning does not mean model training on user data, autonomous prompt mutation, or silent personalization.

All behavior changes must be traceable, versioned, evaluated, reversible, and accepted through human review.

## Context Packet

Each model call may receive a task-specific Context Packet containing:

- system behavior and applicable constitutional constraints;
- current experience;
- relevant conversation context;
- confirmed evidence;
- selected historical evidence;
- user corrections and prior review outcomes;
- locale and response language;
- output schema;
- safety and sensitive-inference boundaries.

Each item should carry its type, source, and consent scope where applicable.

The packet must not include all available history by default. Historical material must be selected because it is relevant to the requested reflection, and the response must disclose its use.

## Context Sufficiency Gate

Before generating an insight, the Harness should assess whether the available context supports the intended inference.

Possible outcomes:

- **Sufficient for observation**: reflect what is directly present.
- **Clarification useful**: invite one or a few high-value questions.
- **Sufficient for tentative hypothesis**: proceed with explicit uncertainty.
- **Insufficient for cross-experience inference**: say so and revisit later.

The user may skip clarification. Skipping lowers the permitted inference depth; it does not make the user fail the flow.

## Output Contract

Insight-oriented output should contain:

1. **Observation** — what appears directly in the available material.
2. **Supporting Evidence** — the records or excerpts that support it.
3. **Hypothesis** — a possible interpretation, clearly labeled.
4. **Uncertainty** — what is missing or why confidence is limited.
5. **Alternative Explanation** — included when useful or when inference risk is material.
6. **Reflective Question** — an optional invitation that returns interpretation to the user.

An output may state that no meaningful pattern was found. The contract must not force depth.

## AI Hypothesis Protocol

Cross-time analysis follows:

Observation
→ Evidence
→ Possible Pattern
→ Alternative Explanation
→ Confidence
→ User Confirmation

Confirmation means the user accepts the hypothesis as useful for continued reflection. It does not convert the hypothesis into objective truth.

## Feedback Capture

For a reviewable artifact, the Harness may record:

- user edited;
- confirmed;
- rejected;
- ignored;
- regenerated;
- provider;
- model;
- prompt version.

Absence of a button click is not rejection. “Ignored” describes an observable interaction state, not user intent.

Feedback records require provenance and retention rules. Sensitive content must not be copied merely to make evaluation convenient.

## Evaluation Dataset

The governed evaluation dataset should include cases for:

- sparse input;
- emotionally intense input;
- ordinary trivial entry;
- contradictory entries;
- repeated relationship issue;
- insufficient historical evidence;
- sensitive inference temptation;
- model overconfidence;
- multilingual consistency.

Cases should test both output quality and refusal to manufacture insight. Synthetic or explicitly consented data should be preferred for regression fixtures.

## Prompt Versioning

Every behavior prompt, output schema, context-selection rule, and safety protocol must have a version.

A versioned change should record:

- what changed;
- why it changed;
- evaluation cases affected;
- human reviewer;
- rollback target.

Provider-specific adapters may format requests differently. They may not redefine the behavior contract.

## Learning Governance

Harness improvement follows:

Dogfooding
→ Captured Feedback
→ Evaluation Case
→ Proposed Change
→ Human Review
→ Versioned Update
→ Regression Check

The model does not rewrite its own prompt.

The model does not silently learn product philosophy from user data.

The model may propose a change. A human decides whether the change becomes part of the Harness.

## Provider Independence

OpenAI, Google AI, local models, and future providers must receive the same governed intent and produce the same conceptual output contract.

Provider capability may change latency, cost, or formatting. It must not change:

- who owns meaning;
- whether uncertainty is visible;
- whether evidence is cited;
- whether sparse context triggers clarification;
- whether diagnosis and identity finalization are prohibited.

## Current Implementation Status

The current code has a simple `AIProvider` interface and similar safety instructions in OpenAI and Gemini adapters. It does not yet implement the Harness described here.

Missing implementation includes:

- one shared versioned behavior protocol;
- Context Packet assembly;
- context sufficiency gating;
- historical evidence selection;
- persisted feedback/provenance;
- evaluation datasets and regression execution;
- prompt/schema version registry.

This document defines the target architecture. `docs/11_MVP.md` and `docs/12_Roadmap.md` define the delivery boundary.

## Related Decisions

- `docs/adr/ADR-0005-ai-provider-abstraction.md` records that provider adapters implement one provider-independent behavior contract.
- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md` records the durable persistence and provenance boundary for reviewed artifacts.
