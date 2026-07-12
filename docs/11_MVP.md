---
status: Draft
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/12
depends:
  - docs/01_Vision.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/04_Problem.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/07_Awareness.md
  - docs/08_Growth.md
  - docs/09_AI.md
  - docs/10_Privacy.md
referenced_by:
  - README.md
  - IMPLEMENTATION_GUIDE.md
  - docs/00_Index.md
  - docs/12_Roadmap.md
  - docs/appendix/Harness.md
  - docs/architecture/00_MVP_Architecture.md
  - docs/product/00_MVP_User_Flow.md
  - docs/adr/ADR-0001-documentation-hierarchy.md
  - docs/adr/ADR-0002-single-source-of-truth.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
---

# 11 MVP

## Purpose

This document is the Primary Definition of Life OS MVP Scope.

It distinguishes implemented behavior from the next scope required to validate the product theory.

## MVP Purpose

The MVP is not a complete Life OS.

It exists to test whether the core theory creates real value:

1. Will an ordinary user describe an experience?
2. Does guided conversation improve the context of that experience?
3. Can AI propose inspectable evidence candidates?
4. Does Reflection create genuine value rather than plausible-sounding depth?
5. Does relevant cross-experience context produce more substantial insight?
6. Does the user still feel ownership of interpretation?

The MVP should build the smallest system capable of answering these questions honestly.

## Current Implemented Scope

Verified against the repository on 2026/07/11:

- Tauri 2 + React 18 + SQLite desktop architecture.
- Desktop-first configuration intended for Windows and macOS.
- Local-first SQLite persistence for `ExperienceEntry` plus source-scoped evidence, reflection, and pattern review artifacts.
- Create, list, edit, and delete experience records.
- JSON and Markdown experience export.
- Life OS JSON experience import with duplicate-id skipping.
- English, Traditional Chinese, and Japanese UI copy.
- OpenAI provider through the Responses API.
- Google Gemini provider with `gemini-3.1-flash-lite` as the default configuration.
- Local mock fallback when real provider configuration or calls fail.
- Evidence Candidate generation, editing, confirmation, and rejection.
- Reflection Prompt generation from confirmed evidence, optional response, and skip flow.
- Pattern Candidate generation from one experience, confirmed evidence, and optional answered reflection context.
- Pattern Candidate confirmation and rejection.
- Provider and model runtime status in the desktop UI.
- Explicit Reflection UI drafts: only a saved response with user provenance is durable or eligible for a Pattern Context Packet; dirty drafts block Pattern generation.
- Stale generated results are discarded by snapshot checks both before and inside the serialized artifact mutation boundary, with the SQLite Experience revision check as final write-time protection.

Important qualification: reviewed artifact records now survive restart locally; JSON/Markdown portability remains experience-only, and no cross-experience retrieval occurs.

## Current Product Gaps

- Context Recovery turns persist per Experience, but there is no general-purpose AI conversation history.
- A deterministic per-Experience Context Sufficiency Gate is implemented. It is heuristic and requires dogfooding evaluation; longitudinal and historical sufficiency remain deferred.
- There is no cross-experience retrieval.
- There is no cross-experience analysis.
- There are no longitudinal summaries.
- There is no memory graph.
- There is no identity hypothesis history.
- Schema v3 preserves source relationships, review state, and provider/model/Harness/prompt provenance in source-scoped payloads; further normalization is deferred.
- There is no user-controlled per-call longitudinal memory consent.
- Shared Harness and prompt versions, Context Packet validation, sufficiency gating, and regression tests are implemented. A governed long-term evaluation dataset remains deferred.
- Tauri bundling is configured, but distributable Windows and macOS builds have not both been verified as release artifacts in this repository.

## Revised MVP Core Flow

Experience
→ Context Sufficiency Check
→ Context Recovery Conversation when needed
→ Evidence Candidates
→ User Review
→ Reflection
→ Pattern Hypothesis
→ Persisted Review Artifacts
→ Cross-Experience Reflection when enough evidence exists

Context Recovery is optional and bounded. Cross-experience reflection must show which historical evidence was used.

## Must Have Before Private Alpha

- Persist reviewed AI artifacts with provenance.
- Let users review past conversations and analyses.
- Recover context for sparse input through low-burden questions.
- Retrieve relevant historical experiences rather than loading all history.
- Support cross-experience analysis when enough evidence exists.
- Give users control over whether long-term memory is used.
- Provide clear correction, rejection, deletion, and source-removal behavior.
- Preserve behavioral and output consistency across English, Traditional Chinese, and Japanese.
- Apply one provider-independent Harness contract.
- Maintain basic evaluation cases and regression checks.
- Produce a distributable desktop build with verified installation and startup behavior.

## Explicitly Out Of Scope

- Mental health diagnosis.
- Autonomous therapy.
- Definitive personality labels.
- A social network.
- Engagement addiction mechanics.
- Cloud-first memory.
- Fully automated life advice.
- Invisible background profiling.
- Provider-controlled product philosophy.

## MVP Success Criteria

Success is not that an answer appears profound.

The MVP succeeds when:

- users feel understood without feeling defined;
- insights can be traced to evidence;
- the system asks before interpreting sparse input;
- relevant past context measurably improves insight quality;
- users can correct, reject, revise, and delete;
- users retain ownership of final meaning;
- users choose to continue using Life OS in real life;
- provider changes do not alter the product's ethical behavior.

## Scope Change Rule

New MVP capability must strengthen the revised core flow and remain subordinate to Book Zero.

If it increases model power without increasing provenance, consent, user control, or evaluability, it is not ready for MVP scope.
