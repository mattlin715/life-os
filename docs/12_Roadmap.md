---
status: Draft
version: 0.8
owner: LIN MENGLUNG
last_updated: 2026/07/28
depends:
  - docs/01_Vision.md
  - docs/02_Philosophy.md
  - docs/06_Memory.md
  - docs/08_Growth.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/appendix/Harness.md
referenced_by:
  - README.md
  - docs/00_Index.md
---

# 12 Roadmap

## Purpose

This document is the Primary Definition of the Life OS Roadmap.

The roadmap uses capability milestones, not invented dates. A phase exits only when its criteria are evidenced.

## Phase 0 — Foundation

### Goal
Create a governed repository in which product philosophy can survive changes in people, models, and technology.

### Capabilities
- Manifesto and Book Zero hierarchy.
- Constitution, source-of-truth governance, and ADR process.
- Tauri scaffold, local-first storage direction, and ownership boundaries.

### Exit Criteria
- Primary concepts have governed source locations.
- Architecture and code can be checked against Book Zero.
- Long-term decisions are recorded in ADRs.

### Explicit Non-goals
- Complete product theory implementation.
- Production packaging or broad distribution.

### Book Zero Risks
- Treating documents as ceremony rather than executable constraints.
- Letting implementation silently redefine primary concepts.

## Phase 1 — Single Experience Mirror

### Goal
Validate a user-controlled reflection loop around one experience.

### Capabilities
- Experience create/edit/delete and local persistence.
- Evidence Candidate review.
- Reflection Prompt flow.
- Single-entry Pattern Candidate review.
- Multilingual UI and OpenAI / Google AI providers with mock fallback.

### Exit Criteria
- A user can complete the single-experience loop.
- AI output is visibly tentative and reviewable.
- Raw experience remains usable when providers fail.

### Explicit Non-goals
- Longitudinal understanding.
- Persistent AI artifacts.
- Identity conclusions.

### Book Zero Risks
- Mistaking a single-entry pattern candidate for a real recurring pattern.
- Rewarding fluent output more than evidence quality.

## Phase 2 — Context Recovery

### Goal
Help users turn sparse input into sufficient context without creating an interrogation.

### Capabilities
- Sparse-input and context-sufficiency detection.
- Guided, low-burden follow-up questions.
- Conversational completion with user-controlled stopping.
- Conversation persistence and review.

### Exit Criteria
- Sparse entries trigger clarification before high-confidence interpretation.
- Questions are bounded, skippable, and demonstrably improve usable context.
- Conversation provenance is preserved.

### Explicit Non-goals
- Infinite coaching conversation.
- Mandatory questionnaires.
- Deep interpretation when the user declines.

### Book Zero Risks
- Making users work for the model.
- Turning Context Recovery into pressure or surveillance.

## Phase 3 — Longitudinal Memory

### Goal
Preserve reviewed understanding so future Reflection can continue rather than restart.

### Capabilities
- Persist confirmed evidence, user reflections, and pattern hypotheses.
- Preserve confirmation, rejection, revision history, and provenance.
- Retrieve relevant past experiences by theme, emotion, relationship, value conflict, and user-selected time range.

### Phase 3A Foundation And Saved-Date Retrieval R1

The Phase 3A foundation implements a bounded set of prior Experience sources
with visible lexical reasons and user-controlled ephemeral selection. The
Founder-authorized R1 adds an explicit,
session-only source-Experience saved-date range that filters before the
unchanged lexical ranking and caps. It does not infer event dates, persist the
range, transmit historical content, infer a cross-experience pattern, or
establish consent; Phase 3B supplies the separate governed send gate. The
Founder manually accepted R1, and feature commit
`2e507728a136ee409c9aa5bb760bc12e57b0d6d1` was promoted through
non-fast-forward merge commit
`db43b8f47815b0c6ddb1bd8daa9a9503c11a2146`. R1 remains local,
session-only, schema-neutral, and not deployed as a release. Emotion,
relationship, and value-conflict retrieval remain unimplemented and require
separate governance rather than inferred taxonomies; the broader structured
retrieval exit gap is therefore not complete.

### Phase 3B Governed Historical Reflection Questions (Implemented and founder-verified)

Phase 3B implements explicit historical-use consent, exact preflight disclosure, bounded provider-independent packet assembly, transport-time and persistence-time revalidation, provider/privacy disclosure, and actual-use provenance before selected history enters a model call.

The approved non-Phase-4 task is limited to neutral, source-citing Historical Reflection Question generation. It may invite the user to compare exact selected sources, but it may not assert recurrence, contradiction, change, summary, Pattern, or identity meaning. The founder accepted [`architecture/09`](architecture/09_Governed_Historical_Context_Assembly_and_Consent.md) and [`ADR-0009`](adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md), then completed manual verification on 2026/07/14. Each transport remains fail-closed unless its exact preflight, consent, destination, source, and persistence checks succeed.

The Founder-authorized Provenance Inspector P1 is implemented and canonically
verified in the current uncommitted working tree. The Founder accepted its diff
and stepwise manual UI review on 2026/07/29; promotion and deployment remain
separate and unauthorized. It gives one already-loaded schema-v4 Historical Question a
collapsed, local, read-only actual-use view with a separate exact-content
reveal. It adds no query, persistence, provider call, consent action, source
rehydration, or Phase 4 interpretation. P1 is deliberately partial and does not
complete correction/export requirements or the future schema-v5
provenance/dependency graph inspector.

### Exit Criteria
- Every persisted artifact distinguishes user content from AI hypothesis.
- Users can inspect, correct, delete, and export artifacts and sources.
- Retrieval is selective, explainable, and consent-aware.

### Explicit Non-goals
- Loading all history into every model call.
- Silent identity profiles.
- Cloud-first memory.

### Book Zero Risks
- Converting continuity into monitoring.
- Losing provenance as records are summarized.

## Phase 4 — Cross-Experience Reflection

### Goal
Use relevant history to help users review recurrence, contradiction, and change across time.

### Capabilities
- Compare selected experiences.
- Recurring-theme review and historical summaries.
- Contradiction tracking and change-over-time reflection.
- Confidence levels and alternative explanations.

### Exit Criteria
- Cross-experience insights cite supporting records.
- Insufficient history produces an honest limitation, not invented depth.
- Users report that relevant history improves understanding without feeling invasive.

### Explicit Non-goals
- Automatic life narratives.
- Treating recurrence as causation.
- Final meaning generated by AI.

### Book Zero Risks
- Overfitting a small personal dataset.
- Using irrelevant sensitive history to simulate intimacy.

## Phase 5 — Identity Hypothesis

### Goal
Offer evidence-based perspectives on identity while keeping identity emergent and user-owned.

### Capabilities
- Long-term identity hypotheses with visible evidence.
- User confirmation, correction, rejection, and revision.
- Optional MBTI or Big Five lenses when explicitly requested.

### Exit Criteria
- No hypothesis becomes a final label.
- Confidence and alternative explanations are visible.
- Users can see how identity understanding changed over time.

### Explicit Non-goals
- Definitive personality classification.
- Hidden scoring or immutable traits.
- Diagnosis or moral-character judgment.

### Book Zero Risks
- Labels becoming identity prisons.
- Optional lenses becoming product truth.

## Phase 6 — Private Alpha

### Goal
Place the longitudinal mirror in sustained real-life use with strong safety and evaluation boundaries.

### Capabilities
- Verified desktop packaging and onboarding.
- Privacy and long-term-memory controls.
- Evaluation Harness, user testing, crash handling, and actionable errors.

### Exit Criteria
- Distributable builds install and launch reliably on supported desktop targets.
- Core safety, deletion, export, correction, and provider-failure cases pass.
- Dogfooding and alpha feedback enter a versioned human-review loop.

### Explicit Non-goals
- Public-scale growth.
- Engagement optimization.
- Cloud dependency.

### Book Zero Risks
- Shipping pressure weakening consent or provenance.
- Treating user silence as approval or rejection.

## Phase 7 — Sustainable Product

### Goal
Make Life OS durable without sacrificing user ownership or the mirror relationship.

### Capabilities
- Optional cloud sync and mobile exploration.
- Sustainable business model and provider strategy.
- Broader distribution with portable personal data.

### Exit Criteria
- Sustainability does not depend on selling inner-life data or addictive engagement.
- Sync and distribution preserve local-first control and Book Zero boundaries.
- Provider changes remain behaviorally governed by the Harness.

### Explicit Non-goals
- Privacy-for-profit tradeoffs.
- Attention capture as the primary business model.
- Centralized ownership of personal meaning.

### Book Zero Risks
- Commercial incentives turning the mirror into an oracle.
- Convenience eroding local-first ownership and psychological safety.

## Roadmap Governance

Phases describe dependency order, not a promise of calendar timing.

A later phase may be explored early, but it must not be declared complete before its exit criteria are evidenced. Dogfooding can change scope; it cannot silently override Book Zero.
