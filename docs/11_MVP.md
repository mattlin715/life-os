---
status: Draft
version: 0.9
owner: LIN MENGLUNG
last_updated: 2026/08/11
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
  - docs/architecture/08_Local_Historical_Context_Selection_Foundation.md
  - docs/product/00_MVP_User_Flow.md
  - docs/adr/ADR-0001-documentation-hierarchy.md
  - docs/adr/ADR-0002-single-source-of-truth.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
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

Verified against the repository on 2026/08/10:

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
- Optional local historical candidate retrieval and source selection with visible lexical reasons, source preview, ephemeral include/exclude controls, and an explicit session-only saved-date range that narrows candidates before lexical ranking. Retrieval begins only when the user opens that Experience's panel; filtering and selection alone never send data or produce a Cross-Experience conclusion.
- Governed Historical Reflection Question generation for exact selected sources: exact-content preflight, per-generation/per-purpose consent, provider-independent bounded packet, OpenAI/Gemini destination disclosure, stale-work rejection, and actual-use provenance. The output is limited to neutral source-citing questions and may return no question.
- Promoted Provenance Inspector P1 adds a collapsed, explicit-open, local read-only actual-use inspector for one already-loaded Historical Question and exact packet snapshot. Exact outgoing content remains behind a second explicit reveal; invalid provenance fails closed rather than reloading current sources or showing partial evidence. The Founder accepted the diff and stepwise manual UI review on 2026/07/29; feature commit `824a2294f2090c541eff0530063fc0730c18cc63` was promoted to `develop` through non-fast-forward merge commit `cf7633780a1a0a72efcad7558e463ceb094468c4`. P1 is not deployed or released and does not replace the future schema-v5 provenance graph.
- Daily Reflection Core UX R1 was implemented on feature commit `9f889fe6880bbd0641c0308d60892adade418371`, passed Founder manual UI review, and was promoted to `develop` by non-fast-forward merge commit `c329edb80e0ab3bd5bcbcc4dedd33c9c3af6bf12`. It is not deployed or released. R1 restores the Experience composer as the first product action, gives the newest Experience a visible next-step summary, collapses older Experiences, adds session-only local keyword and saved-date timeline filters, moves database readiness under secondary data and diagnostics tools, and presents the governed historical preflight as a concise consent summary with exact transmission details behind an explicit disclosure. Founder-review corrections add an open-only local-history shortcut near the top of every Experience, explain why saved clues and Reflection do not invent missing event facts for the Pattern gate, and replace fragmentary CJK adjacent-character reasons with bounded concrete word overlap. The slice changes presentation and local retrieval only: schema v4, provider transport, ContextPacket structure, consent, retention, and actual-use provenance semantics remain unchanged.
- Daily Reflection Completion UX R2 was implemented on feature commit `a8dd26b0652fd284f6d18fd726a87577b79e9b32` and promoted to `develop` through non-fast-forward merge commit `76bc4addd954cd14a4ab82f3e4a2369efaab8820`. It derives a session-only journey from already loaded records, emphasizes one active Evidence or Reflection step, collapses completed stages behind explicit reopen controls, makes the next gentle step focusable without generating anything, and shows a calm completion review made only from the saved Experience, confirmed Evidence, user-authored Reflection responses, explicit skips, optional existing Pattern state, and optional existing Historical Reflection Questions. Pattern and history remain optional. R2 adds no generated summary, durable navigation state, schema change, provider action, ContextPacket change, consent change, or Phase 4 interpretation. Its archived sprint state records `manual_ui.status = not_run`; the repository therefore does not claim that R2 Founder manual acceptance was recorded. This evidence gap is not evidence that review failed. R2 is not deployed or released.

Important qualification: reviewed artifact records now survive restart locally; JSON/Markdown portability remains experience-only. Historical candidate retrieval, saved-date controls, and selection stay bounded and local; the range and selection remain ephemeral. Phase 3B permits historical provider transmission only through the separately governed one-generation Historical Reflection Question gate. Cross-Experience Reflection remains deferred.

## Current Product Gaps

- Context Recovery turns persist per Experience, but there is no general-purpose AI conversation history.
- A deterministic per-Experience Context Sufficiency Gate is implemented. It is heuristic and requires dogfooding evaluation; longitudinal and historical sufficiency remain deferred.
- There is no durable historical selection, blanket historical-use preference, Cross-Experience Reflection, or cross-experience analysis. The only historical provider task is the governed Phase 3B neutral-question slice.
- There are no longitudinal summaries.
- There is no memory graph.
- There is no identity hypothesis history.
- Schema v4 preserves existing source-scoped artifacts and adds historical consent, transmission, successful packet snapshot, actual-use provenance, and cross-source dependency records. Further normalization is deferred.
- The P1 inspector is schema-neutral and introduces no persistence API, query, retention change, provider behavior, or source rehydration. Its view is limited to the actual-use evidence already present in the loaded schema-v4 record.
- User-controlled per-call consent exists only for the bounded Phase 3B Historical Reflection Question task; no general longitudinal-memory authorization exists.
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
