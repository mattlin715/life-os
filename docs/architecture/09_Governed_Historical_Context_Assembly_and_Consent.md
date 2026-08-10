---
status: Implemented
version: 1.2
owner: product-and-engineering
last_updated: 2026/08/11
depends:
  - docs/00_Constitution.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/appendix/Harness.md
  - docs/architecture/00_MVP_Architecture.md
  - docs/architecture/08_Local_Historical_Context_Selection_Foundation.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
referenced_by:
  - docs/00_Index.md
  - docs/12_Roadmap.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
---

# 09 Governed Historical Context Assembly and Consent

## Purpose

This document records the founder-approved and implemented Phase 3B boundary for **Governed Historical Context Assembly and Consent**.

It governs the one bounded path by which exact user-selected historical material may enter a model call. It does not change the Constitution or authorize Phase 4 Cross-Experience Reflection.

More context increases responsibility, not AI authority.

## Decision-State Vocabulary

| State | Meaning in this document |
| --- | --- |
| Implemented | Verified current repository behavior on 2026/07/14. |
| Proposed | A design recommendation awaiting explicit founder approval. No Phase 3B item remains in this state. |
| Founder-approved | A decision the human founder has explicitly accepted. The ten Phase 3B decisions in this document entered this state on 2026/07/14. |
| Deferred | Outside Phase 3B or intentionally postponed until a later approved capability. |

These states must not be collapsed. The implemented packet contract does not make selection into consent, and implementation does not authorize any call that fails the per-generation gate.

## Observed Repository Evidence

### Implemented in Phase 3A

- The current `local-lexical-v2` retrieves at most a small bounded candidate set only after the user explicitly opens a local-history panel. Previously persisted `local-lexical-v1` actual-use records remain inspectable without rewriting.
- Each candidate exposes an exact source Experience ID, bounded excerpt, visible shared-term reason, source timestamps, and eligible artifact IDs.
- Source selection is an ephemeral exact-ID `Map<string, ReadonlySet<string>>`; it is not serialized or persisted.
- Reopening a panel retrieves fresh candidates and removes no-longer-eligible selections without reselecting a returning source.
- Rejected Evidence, skipped or orphaned Reflection, AI-authored Reflection responses, deleted Experiences, and unsaved drafts are ineligible.
- Historical candidates and selections are absent from `ContextPacket`, OpenAI/Gemini transport payloads, mock provider output, and SQLite.
- Opening, selecting, excluding, clearing, or closing history makes no provider call and produces no Cross-Experience conclusion.

### Implemented Phase 3B provider and persistence boundaries

- The existing single-Experience `ContextPacket` still supports `evidence`, `reflection`, and `pattern`; Phase 3B uses a separate immutable, provider-independent historical packet for `historical_reflection_questions`.
- OpenAI and Gemini share one historical transport contract, explicitly send `store: false`, disclose provider/model and retention-policy boundaries, and do not silently fall back.
- Additive SQLite schema version 4 stores separate consent events, transmission outcomes, successful packet snapshots, actual-use provenance, and cross-source dependencies.
- Source/artifact revisions, packet digest, destination, consent, transmission outcome, and eligibility are revalidated before transport and inside the serialized persistence transaction.
- Selection remains ephemeral. Successful packet snapshots share the generated artifact deletion lifecycle; unsuccessful minimal audit metadata expires after 30 days.
- Regression tests protect consent separation, packet integrity and limits, provider parity, stale work, provenance, migration rollback, multilingual disclosure semantics, sensitive-inference refusal, and the Phase 3B/Phase 4 output boundary.

### Source-of-truth conclusions

- `docs/10_Privacy.md` governs ongoing consent and states that storing an Experience is not consent to use it in every future model call.
- `docs/06_Memory.md` governs relevant continuity, provenance, correction, deletion, and explainable selective retrieval.
- `docs/Reflection.md` defines Cross-Experience Reflection as comparison capable of proposing repeated themes or contradictions when sources are visible.
- `docs/09_AI.md` requires AI to act as a Context Steward, avoid unrelated sensitive history, and preserve user-owned meaning.
- `docs/appendix/Harness.md` governs provider-independent context assembly and requires type, source, consent scope, and visible historical use.
- ADR-0005 requires provider parity or refusal. ADR-0007 requires provenance and understandable dependency behavior for durable AI artifacts.

No Book Zero rewrite was needed. ADR-0009 is Accepted, and the implementation is specified by this boundary plus [`architecture/10`](10_Historical_Question_Persistence_Vertical_Slice.md).

### Affected source-of-truth documents

| Source | Implemented effect |
| --- | --- |
| `docs/02_Philosophy.md` | Applies Context Before Insight; no definition change proposed. |
| `docs/03_Principles.md` | Applies evidence, reflection, context, trust, and agency principles; no definition change proposed. |
| `docs/05_Identity.md` | Preserves emergent identity and hypothesis limits; no definition change proposed. |
| `docs/06_Memory.md` | Applies selective continuity, consent, provenance, correction, and deletion; no definition change proposed. |
| `docs/Reflection.md` | Preserves the boundary between question generation and Cross-Experience Reflection; no definition change proposed. |
| `docs/09_AI.md` | Applies Context Steward and provider-independence behavior; no definition change proposed. |
| `docs/10_Privacy.md` | Applies ongoing context-use consent and sensitive-inference restraint; no definition change proposed. |
| `docs/11_MVP.md` | Records the bounded implemented historical-question slice while preserving Phase 4 exclusions. |
| `docs/12_Roadmap.md` | Records Phase 3B as implemented and founder-verified without advancing Phase 4. |
| `docs/appendix/Harness.md` | Records the implemented provider-independent consent, packet, evaluator, and provenance boundary. |

The Constitution is unaffected and must remain unchanged.

## Phase 3B Boundary

Phase 3B implements:

- explicit historical-use consent;
- preflight disclosure;
- exact bounded historical packet assembly;
- transport-time revalidation;
- provider destination and retention disclosure;
- actual-use provenance and deletion dependencies;
- regression cases for the consent and transport boundary.

Phase 3B must not generate:

- recurrence claims;
- contradiction analysis;
- change-over-time interpretation;
- historical summaries;
- Cross-Experience Pattern hypotheses;
- identity hypotheses or personality labels.

Those remain Phase 4 or later.

## Can Phase 3B Transmit History Without Becoming Phase 4?

### Implemented answer: yes, but only for one narrow task

The only implemented non-Phase-4 production purpose is **Historical Reflection Question generation**.

The model may receive the current Experience and exact consented historical excerpts solely to produce one to three neutral, source-citing questions that invite the user to compare the material themselves.

The output contract must:

- identify which exact sources each question refers to;
- use question form rather than an observation or conclusion;
- avoid asserting that anything repeats, contradicts, changes, causes, reveals, or defines the user;
- avoid summarizing the selected history;
- avoid Evidence, Pattern, Awareness, Growth, diagnosis, advice, and identity claims;
- allow an honest no-question result when a responsible question cannot be formed;
- remain editable, rejectable, deletable, and visibly AI-generated;
- never become eligible historical evidence merely because it was generated.

Example of permitted form:

> When you place the current Experience beside source `experience-123`, what feels similar or different to you?

Example of prohibited Phase 4 form:

> These Experiences show a recurring conflict-avoidance pattern.

This narrow task validates consent, packet assembly, provider parity, and provenance without delegating cross-time interpretation to the model. If this bounded contract cannot be honored, production historical transmission fails closed; it must not expand into Phase 4 behavior.

## Six Separate States

| State | Current behavior |
| --- | --- |
| 1. Local candidate retrieval | **Implemented.** Explicit-panel, bounded, local, explainable. |
| 2. User source selection | **Implemented.** Ephemeral exact-ID selection. Selection is not consent. |
| 3. Consent | **Implemented.** Explicit for one generation and one declared purpose after exact-content preflight. |
| 4. Context Packet assembly | **Implemented.** Immutable, bounded, provider-independent historical packet assembled for one consent event. |
| 5. Provider transmission | **Implemented and founder-verified for the bounded task.** Each call still fails closed unless its exact consent and revalidation checks pass. |
| 6. Generated-artifact provenance | **Implemented.** Persist actual-use provenance only for an authorized successful call, with minimal audit metadata for attempted transport. |

Opening a panel, selecting a source, leaving a source selected, silence, inactivity, or a prior consent event must never advance state 3.

## Implemented Consent Contract

### Granularity

Consent should be:

- explicit per generation;
- explicit per declared purpose;
- bound to one exact preflight packet digest;
- bound to exact source and artifact IDs and revisions;
- bound to provider, model, task, Harness version, packet schema version, and locale;
- consumed by one transport attempt;
- requested again after cancellation, failure, or retry.

There should be no blanket future authorization and no durable "always use my history" preference in Phase 3B.

Local storage consent is not provider-use consent. Prior selection is not provider-use consent. Consent for Historical Reflection Question generation is not consent for Phase 4 analysis.

### Invalidation

Any material change after disclosure invalidates consent, including:

- adding, removing, editing, deleting, rejecting, or revising a source or selected artifact;
- changing the current Experience or its revision;
- changing task, purpose, provider, model, locale, output contract, or safety contract;
- changing bounded outgoing content, packet digest, schema version, Harness version, or prompt version;
- loss of artifact eligibility or a stale/malformed source;
- application restart before transport, unless a later founder-approved design explicitly proves safe resumability.

An invalidated consent event cannot be silently refreshed. The system must show a new preflight and ask again.

## Implemented Preflight Disclosure

Immediately before consent, one screen should show:

- the exact current Experience and historical source IDs;
- every included artifact ID and type;
- the exact bounded outgoing text, not a vague category label;
- authorship and review state for every artifact;
- the visible relevance reason that led to the candidate;
- the provider and model destination;
- the exact requested task and prohibited outputs;
- a warning when selected content may be sensitive;
- known local persistence behavior and separately stated provider-retention uncertainty or policy;
- `Include`, `Exclude`, `Cancel`, and explicit `Send selected sources` controls.

The confirmation control must name the action. A generic `Continue` control is insufficient.

Removing an item must recompute the packet digest and invalidate any earlier consent. Cancel must make no provider call.

## Implemented Eligibility Boundary

| Content | Phase 3B rule |
| --- | --- |
| Persisted current Experience text | Eligible when disclosed. |
| Persisted selected historical Experience text | Eligible only as an exact selected, bounded source. |
| Confirmed Evidence | Eligible only when source-scoped, current, disclosed, and explicitly included. |
| Saved user-authored Reflection response | Eligible only when answered, provenance is user, dependencies remain confirmed, and it is explicitly included. |
| Unsaved draft | Excluded. |
| Rejected Evidence or Pattern | Excluded. |
| Skipped, foreign, orphaned, or AI-authored Reflection response | Excluded. |
| Deleted, stale, malformed, or revision-mismatched artifact | Excluded and consent-invalidating. |
| Unreviewed AI output | Excluded. |
| Pattern or identity hypothesis | Excluded from Phase 3B historical transmission. |
| Unrelated sensitive history | Excluded even if locally available. |
| Whole-history loading | Prohibited. |

Source-level selection does not silently select every artifact under that source. The preflight must expose and control the exact artifact-level content that would leave the device.

## Implemented Provider-Independent Historical Context Packet

This contract is implemented as a separate immutable TypeScript packet and a shared transport serializer; it does not modify the existing `ContextPacket`.

Each immutable packet contains:

- packet ID, digest, schema version, assembled-at timestamp, and expiry;
- current Experience ID, revision/snapshot reference, and bounded current content;
- requested task and purpose;
- locale and response language;
- provider and model destination;
- Harness, prompt, output-schema, and safety-contract versions;
- source Experience IDs and exact source revisions/snapshots;
- exact artifact IDs and artifact types;
- authorship and review state;
- bounded transmitted content for each included item;
- visible relevance reason and retrieval algorithm version;
- one-shot consent reference and consent scope; the consent timestamp exists only on the separately persisted event created by the explicit send action;
- explicit maximum source count and content-size limits.

The transport serializer must be shared across providers and should fail closed on unknown fields, unknown artifact types, revision mismatch, missing provenance, unsupported retention behavior, or packet size overflow.

The packet must not contain the candidate collection, unselected sources, hidden profile fields, rejected material, drafts, or the user's whole history.

## Implemented Persistence and Provenance

The lifecycle should remain separated:

| Record | Implemented rule |
| --- | --- |
| Ephemeral selection | Do not persist. |
| Consent event | Persist minimal local metadata: consent reference, exact scope, source/artifact IDs and revisions, destination, versions, packet digest, timestamp, and consumed/invalidated state. Do not treat it as a preference. |
| Packet snapshot before send | Keep immutable in memory only. |
| Transmission event | Persist minimal local outcome metadata for sent, failed, cancelled-before-send, cancelled-after-send, or refused. Do not copy provider error bodies containing user content. |
| Successful packet snapshot | Persist the exact bounded used snapshot only when an authorized call succeeds and its generated artifact is durably accepted; tie their deletion lifecycle together. |
| Generated artifact provenance | Persist consent reference, transmission reference, packet digest/schema, provider/model, Harness/prompt versions, exact source/artifact IDs/types/revisions, and generated timestamp. |

This design records consent without pretending consent proves use, and records actual use without turning a prior grant into blanket permission.

A successful provider response must not create a durable artifact until the same source, eligibility, consent, destination, version, and packet-digest checks pass again inside the serialized persistence boundary.

## Stale Work, Correction, and Deletion

### Revalidation immediately before transport

Re-read every current and historical source. Confirm exact IDs, revisions, artifact states, dependencies, provider/model, task, versions, consent reference, and packet digest. Any mismatch fails closed before network transport.

### Stale response rejection

Revalidate again before persistence. If any source was edited/deleted, Evidence rejected, Reflection changed, consent invalidated, destination changed, feature disabled, or Harness/schema version changed while the call was in flight, discard the generated result before persistence.

### Dependency behavior

- A pre-send edit/delete/rejection invalidates the packet and consent.
- A post-send source edit/delete/rejection immediately makes dependent generated artifacts ineligible for future context.
- Deleting a source should cascade-delete dependent generated content and its successful packet snapshot by default.
- A minimal content-free transport tombstone may remain only if the founder approves that audit need and retention period.
- Correcting a source must not silently rewrite or regenerate a dependent artifact. The user must choose a new generation and give new consent.

Provider receipt cannot be undone by local deletion. The preflight must say this plainly.

### Rollback and feature disable

Disabling the feature must close the transport gate while leaving Phase 3A local retrieval usable. Existing historical generated artifacts should remain inspectable but ineligible for reuse until the feature is re-enabled under a compatible approved contract. A rollback must not silently downgrade or drop consent/provenance records.

## Provider and Privacy Boundary

- BYOK changes credential ownership; it does not make a remote provider local and does not itself guarantee retention behavior.
- The preflight must disclose the real provider and model, not only a product label.
- Local packet/provenance persistence and provider-side retention are separate facts.
- Use `store: false` only where the provider supports and the implementation verifies it. Do not imply that another provider has an equivalent control.
- Provider policy, account configuration, regional processing, and retention claims must be verified at implementation time and shown accurately.
- A provider that cannot honor packet limits, output constraints, deletion expectations, destination disclosure, or the shared Harness contract must refuse historical transmission, reduce to local-only behavior, or remain unsupported.
- Provider fallback must not silently change the destination after consent. A fallback provider requires a new preflight and new consent.

## Sensitive Inference Boundary

Phase 3B must not authorize or produce:

- mental-health diagnosis;
- sexuality inference;
- religious identity inference;
- political identity inference;
- moral-character judgment;
- immutable personality labels;
- identity finalization;
- silent profiling.

This Phase 3B restriction applies even when a user asks for such an inference. A future product boundary would require separate founder review; it cannot be smuggled into historical packet consent.

## Evaluation and Regression Package

Use synthetic or explicitly consented fixtures. The same conceptual cases must run against every supported provider.

| Case | Required result |
| --- | --- |
| Selection without consent | No packet assembly for transport and no provider call. |
| Panel open or user silence | No implied consent and no provider call. |
| Consent then source edit/delete | Consent invalid; no send. |
| Consent then artifact rejection/orphaning | Consent invalid; excluded artifact; no send. |
| Task, purpose, provider, or model change | New preflight and consent required. |
| Packet content or version change | Digest mismatch; no send. |
| Malformed or stale source | Calm refusal; no raw malformed content sent. |
| Unsaved draft or unreviewed AI output | Excluded from preflight and packet. |
| Whole-history temptation | Enforced source/count/size limit; no send if exceeded. |
| Multilingual disclosure | English, Traditional Chinese, and Japanese communicate the same sources, destination, purpose, warning, and controls. |
| Sensitive-context temptation | Refuse prohibited inference without leaking unrelated sensitive history. |
| Provider parity | Equivalent packet, consent, output, refusal, and provenance semantics. |
| Provider fallback | No silent destination switch; require new consent. |
| Cancel before send | No network call; consent not consumed as actual use. |
| Failure after send | No generated artifact; record minimal sent/failed outcome without sensitive error content. |
| Source changes during call | Reject stale response before persistence. |
| Feature disabled during call | Reject response and prevent further transmission. |
| Generated output asserts recurrence/contradiction/change | Schema or evaluator rejects it; no persistence. |
| Generated output creates a Pattern, summary, diagnosis, or identity claim | Reject as outside Phase 3B. |
| No useful neutral question exists | Return no-question result; do not manufacture depth. |

Implementation regression coverage is routed through `src/historicalContext/governedPacket.test.ts` for consent, packet, limits, staleness, multilingual semantics, prohibited output, and no-question behavior; `src/ai/providers/sharedJsonProvider.test.ts` for provider parity and no fallback; `src/shared/storage/inMemoryLocalEvidenceStore.test.ts` for actual-use lifecycle; and `src-tauri/src/sqlite.rs` tests for additive migration rollback and serialized persistence revalidation.

## Schema Migration Decision

The founder approved and the implementation applied an additive, non-destructive schema v4 design on 2026/07/14. Schema v3 could not represent consent events, transmission outcomes, successful packet snapshots, or cross-source dependencies safely; the existing source-scoped artifact payloads were not overloaded.

Its schema v4 tables, serialized revision and actual-use-provenance checks, 30-day unsuccessful-record retention, dependency deletion lifecycle, and feature-disable strategy are specified in [`architecture/10`](10_Historical_Question_Persistence_Vertical_Slice.md). No destructive down migration is authorized.

## Alternatives and Risks

### Alternative A: no Phase 3B production transmission

Keep all historical content local until Phase 4 is separately approved.

- Benefit: lowest privacy and boundary risk.
- Cost: consent, packet, provider-parity, and provenance design cannot be validated in real use before Phase 4.

### Alternative B: bounded Historical Reflection Question generation (recommended)

Transmit only exact consented sources to generate neutral source-citing questions.

- Benefit: tests governed transport without authorizing cross-time conclusions.
- Cost: model wording may still imply a conclusion, so strict schema and evaluation rejection are required.

### Alternative C: full Cross-Experience Reflection in Phase 3B

Rejected for this phase because it collapses the roadmap boundary and authorizes recurrence, contradiction, or change interpretation before Phase 4 governance.

### Principal risks

- consent fatigue or habituated clicking;
- source preview that hides the true outbound content;
- provider fallback changing destination silently;
- content drift between preflight and send;
- packet snapshots duplicating sensitive content locally;
- deletion that cannot undo provider receipt;
- neutral questions that smuggle in Cross-Experience conclusions;
- provenance that records IDs but not the exact revisions actually used;
- a broad consent preference becoming silent profiling.

## Accepted Founder Package

The founder approved one coherent package, not isolated permissions:

1. Consent is explicit per generation and per purpose, bound to an exact packet digest and destination.
2. Phase 3B may transmit history only for Historical Reflection Question generation.
3. Only exact persisted Experience text, confirmed Evidence, and saved user-authored Reflection responses are eligible.
4. Source selection remains ephemeral and is never consent.
5. Minimal consent and transport metadata persist locally; an exact used packet snapshot persists only with a successful generated artifact.
6. Additive schema v4 is required for production transmission and is implemented; no destructive down migration is authorized.
7. Any source/artifact/task/provider/model/content/version change invalidates consent.
8. Phase 3B categorically refuses the sensitive inference classes listed above.
9. Phase 4 retains recurrence, contradiction, change-over-time, summaries, Pattern hypotheses, and all Cross-Experience conclusions.
10. Historical transmission is available only through the implemented governed gate; all other historical provider use remains disabled.

## Founder Decision Questions (Resolved)

1. Is historical-use consent per generation and per purpose?
2. May Phase 3B perform any production historical transmission?
3. If yes, is Historical Reflection Question generation the only approved non-Phase-4 task?
4. Are persisted Experience text, confirmed Evidence, and saved user-authored Reflection responses the only eligible artifact classes?
5. Must source selection remain ephemeral?
6. Should minimal consent/transport metadata persist, with the exact used packet snapshot tied only to a successful generated artifact?
7. Is a new schema migration justified only after production transmission is approved?
8. Do source/artifact/task/provider/model/content/version changes and restart-before-send invalidate consent?
9. Should Phase 3B categorically refuse diagnosis, sexuality/religious/political identity inference, moral-character judgment, immutable personality labels, identity finalization, and silent profiling?
10. Do recurrence, contradiction, change-over-time, historical summaries, Cross-Experience Pattern hypotheses, and identity interpretation remain deferred to Phase 4 or later?

## Founder Decision Record

On 2026/07/14, the founder explicitly approved ADR-0009 and all ten decisions above as one coherent package:

1. Consent is per generation and per purpose and binds one immutable packet.
2. Phase 3B may perform governed production historical transmission after implementation verification and founder manual review.
3. Historical Reflection Question generation is the only approved production historical task.
4. Eligibility is limited to persisted Experience text, confirmed Evidence, and saved user-authored Reflection responses.
5. Source selection remains ephemeral and never becomes consent or a preference.
6. Minimal local consent/transmission metadata persists; unsuccessful records expire after 30 days; a successful packet snapshot and actual-use provenance persist only with the accepted generated artifact and share its deletion lifecycle; no additional tombstone is retained.
7. An additive migration and non-destructive rollback/feature-disable design are approved. Any destructive rollback requires a new founder checkpoint.
8. Every listed source, artifact, task, purpose, destination, content, locale, contract, version, digest, eligibility, feature-state, or restart-before-send change invalidates consent. Revalidation is required before transport and before persistence.
9. The listed sensitive inference classes are categorically prohibited, including when requested by the user.
10. Recurrence, contradiction, change-over-time, historical summaries, Cross-Experience Pattern hypotheses, Awareness, Growth, causality, trends, advice, and identity interpretation remain deferred to Phase 4 or later.

Acceptance did not authorize silent activation. The repository verification path and founder manual verification succeeded on 2026/07/14; every historical transport still requires a fresh exact preflight and explicit consent.
