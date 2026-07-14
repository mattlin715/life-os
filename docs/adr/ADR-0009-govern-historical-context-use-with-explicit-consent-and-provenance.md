---
status: Accepted
version: 1.1
owner: product-and-engineering
last_updated: 2026/07/14
depends:
  - docs/00_Constitution.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/appendix/Harness.md
  - docs/architecture/08_Local_Historical_Context_Selection_Foundation.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
referenced_by:
  - docs/00_Index.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
---

# ADR-0009: Govern Historical Context Use With Explicit Consent and Provenance

## Status

Accepted by the founder on 2026/07/14 after explicit approval of all ten decisions in the Acceptance Gate.

This ADR authorizes the bounded Phase 3B implementation described below. The implementation passed automated verification and founder manual review on 2026/07/14; every generation still remains fail-closed unless its exact preflight, consent, transport, and persistence checks succeed.

## Context

Phase 3A implements explicit-panel, bounded, local historical candidate retrieval and ephemeral exact-ID source selection. Historical candidates and selections do not enter `ContextPacket`, provider requests, or persistence, and selection creates no Cross-Experience conclusion.

Life OS now needs a durable decision before any selected historical material can leave the device. The decision must keep separate:

1. local retrieval;
2. user selection;
3. consent;
4. packet assembly;
5. provider transmission;
6. generated-artifact provenance.

Book Zero requires ongoing consent, selective and explainable context, provider-independent behavior, visible provenance, deletion and correction, uncertainty, and user-owned meaning. At decision time, SQLite schema version 3 and the single-Experience `ContextPacket` did not represent a governed historical-use lifecycle. The accepted implementation therefore added a separate historical packet and additive schema v4 rather than broadening `ContextPacket`.

Phase 4, not Phase 3B, owns recurrence, contradiction, change-over-time, historical summaries, Cross-Experience Pattern hypotheses, and cross-time interpretation.

## Decision

Life OS governs historical context use as follows.

### 1. Consent is exact and one-time

Historical provider use requires explicit consent for one generation and one declared purpose after a preflight shows the exact outgoing content, sources/artifacts, relevance reason, provider/model destination, task, sensitive-content warning, and include/exclude/cancel controls.

Selection, silence, opening a panel, local storage consent, or a previous consent event never implies current provider-use consent. There is no blanket future authorization in Phase 3B.

Consent binds to the exact packet digest, source/artifact revisions, task, purpose, provider, model, locale, Harness/prompt/schema versions, and output contract. Any material change invalidates it.

### 2. Phase 3B transmission is limited to a non-concluding task

The only Phase 3B production task is Historical Reflection Question generation: one to three neutral, source-citing questions that invite the user to compare exact selected material.

The model may not assert recurrence, contradiction, change, cause, summary, Pattern, Awareness, Growth, diagnosis, advice, identity, or final meaning.

### 3. Eligibility is narrow

Eligible content is limited to exact, persisted, disclosed, and explicitly included:

- Experience text;
- confirmed Evidence;
- saved user-authored answered Reflection responses with valid dependencies.

Unsaved drafts, rejected material, skipped/foreign/orphaned Reflection, AI-authored Reflection responses, deleted/stale artifacts, unreviewed AI output, Pattern or identity hypotheses, unrelated sensitive history, and whole-history loading are excluded.

### 4. Historical packets are provider-independent and fail closed

A versioned immutable packet records exact source Experience IDs, artifact IDs/types, authorship/review state, bounded transmitted content, relevance reason, source revisions/snapshots, consent reference/scope, provider/model destination, packet digest, and Harness/prompt/schema versions.

All providers receive the same conceptual contract. A provider that cannot honor the contract must refuse, reduce to local-only behavior, or remain unsupported. Provider fallback requires a new preflight and consent.

### 5. Selection, consent, transmission, and actual use remain distinct

- Source selection stays ephemeral.
- Minimal consent-event metadata persists locally and is not a preference.
- A packet snapshot remains in memory before send.
- Minimal transmission-outcome metadata records whether anything was sent without copying sensitive provider error bodies.
- The exact successful packet snapshot and actual-use provenance persist only with an authorized generated artifact and share its deletion lifecycle.

### 6. Staleness and deletion fail closed

Every source, artifact, revision, destination, task, version, consent reference, and packet digest is revalidated immediately before transport and again before persistence.

Pre-send changes invalidate consent. In-flight changes cause response rejection before persistence. A post-send source edit/delete/rejection makes dependent generated artifacts ineligible; source deletion should cascade-delete dependent generated content and the successful packet snapshot by default.

Local deletion cannot undo provider receipt and must be disclosed before consent.

### 7. Sensitive inference remains prohibited in Phase 3B

Phase 3B will not perform diagnosis; sexuality, religious, or political identity inference; moral-character judgment; immutable personality labeling; identity finalization; or silent profiling, even when requested by the user.

## Consequences

- The accepted implementation passed founder manual review on 2026/07/14; transmission remains gated per generation rather than globally pre-authorized.
- The existing single-Experience `ContextPacket` remains unchanged; governed historical use has a separate immutable packet contract.
- Additive schema v4 implements consent events, transmission outcomes, successful packet snapshots, and cross-source dependencies. No destructive down migration is authorized.
- Consent UI must disclose exact content and destination rather than relying on generic settings.
- Provider retention behavior must be verified and disclosed separately from local persistence. `store: false` may be claimed only where supported and verified.
- Cancellation and retry semantics become explicit: cancellation before send makes no call; failure or retry requires fresh consent.
- Provider parity evaluation must cover consent, packet contents, refusal, stale work, cancellation, provenance, and the prohibition on Phase 4 conclusions.
- Packet snapshots duplicate sensitive material locally and therefore require inspect, delete, retention, and dependency controls.

## Alternatives Considered

### Keep historical provider transmission disabled through Phase 3B

This was the safest fallback before approval and remains the operational fallback whenever the governed contract cannot be honored.

### Permit bounded Historical Reflection Question generation

Recommended because it tests governed consent and transport without authorizing Cross-Experience conclusions. It still requires strict output rejection because a model can imply a conclusion inside a question.

### Permit full Cross-Experience Reflection in Phase 3B

Rejected because it collapses the Phase 3B/Phase 4 boundary and would authorize a broader moral and inference relationship than this sprint may decide.

### Persist a blanket "use my history" preference

Rejected because it turns ongoing consent into background authorization and increases silent-profiling risk.

### Treat source selection as consent

Rejected because local organization and provider disclosure are materially different actions.

### Persist only source IDs, not revisions or packet content

Rejected because it cannot prove what exact material left the device or safely detect stale work.

## Founder Acceptance Gate (Satisfied)

The founder explicitly accepted all ten decisions on 2026/07/14:

1. per-generation and per-purpose consent;
2. whether any Phase 3B production transmission is allowed;
3. whether Historical Reflection Question generation is the sole allowed task;
4. eligible artifact classes;
5. ephemeral selection;
6. consent, transmission, packet-snapshot, and actual-use provenance retention;
7. whether a new migration is justified after approval;
8. consent invalidation rules;
9. the categorical Phase 3B sensitive-inference prohibition;
10. the capabilities deferred to Phase 4.

All items are accepted. Automated verification and founder manual verification passed on 2026/07/14. Phase 3A selection remains local and ephemeral; only the separately governed Phase 3B send action can authorize historical provider use.
