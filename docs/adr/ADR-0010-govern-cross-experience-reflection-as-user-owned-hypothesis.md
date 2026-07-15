---
status: Accepted
version: 0.2
owner: product-and-engineering
last_updated: 2026/07/16
depends:
  - docs/00_Constitution.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/07_Awareness.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/12_Roadmap.md
  - docs/appendix/Harness.md
  - docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md
  - docs/adr/ADR-0003-identity-is-emergent.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
referenced_by:
  - docs/00_Index.md
---

# ADR-0010: Govern Cross-Experience Reflection As A User-Owned Hypothesis

## Status

Accepted by the founder on 2026/07/16 with all sixteen decisions recorded in `architecture/11`. Acceptance establishes the Phase 4 design boundary only. It does not authorize Phase 4 production implementation, migration, retention cleanup, provider transmission, UI activation, or reuse of Phase 3B consent; all five Phase 3 exit gaps remain blocking.

## Context

Book Zero permits AI to compare selected evidence, observe possible patterns, surface contradiction, and support Reflection across time. It also requires uncertainty, provenance, ongoing consent, correction, deletion, and user-owned meaning.

Phase 3A implements bounded local retrieval and ephemeral selection. Phase 3B implements a separately governed Historical Reflection Question task. ADR-0009 explicitly excludes recurrence, contradiction, change-over-time interpretation, historical summary, Cross-Experience Pattern hypotheses, and identity meaning.

Phase 4 therefore requires a new decision. Model fluency can turn a small, biased personal sample into a persuasive narrative. Contradictions can become accusations. Differences over time can become fabricated stories of growth or decline. A prior consent event can be laundered into broader profiling. Durable output can become identity judgment if Evidence, hypothesis, and user meaning are flattened together.

The Phase 3 exit audit in `architecture/11` finds five open gaps: artifact/provenance export, revision and rejection history, post-review correction/deletion, structured retrieval, and provenance/dependency inspection. Founder Decision 13A makes all five gaps blocking; no deferral is approved. They must be completed and verified before Phase 4 production implementation.

## Decision

Life OS will govern Phase 4 output as a **user-owned Cross-Experience Hypothesis**, never as a conclusion or identity fact, subject to the accepted sixteen-decision package and the remaining implementation entry gates.

### Hypothesis contract

A candidate hypothesis must:

- use only exact manually selected, disclosed, eligible sources;
- distinguish observation, possible pattern or tension, counter-evidence, alternative explanation, and confidence basis;
- cite every supporting and counter-evidence source;
- remain tentative, editable, rejectable, deletable, and revisable;
- allow `insufficient context` and `no hypothesis`;
- require explicit user acceptance before durable persistence;
- preserve original AI authorship after user acceptance or editing;
- never automatically become Evidence, Awareness, Growth, Identity, diagnosis, advice, or final meaning.

### Consent contract

Phase 4 requires a new explicit consent for one generation, one comparison purpose, one immutable packet, and one disclosed provider/model destination. Phase 3B consent, selection, local storage consent, prior acceptance, silence, or inactivity are ineligible.

### Context and sufficiency contract

Retrieval remains local, selective, bounded, explainable, and user-controlled. The first slice uses manual exact selection. Recurrence or change review requires at least three independent Experiences across at least two user-selected time points. A two-source comparison may surface only a bounded tension, not recurrence. Counter-evidence and alternative explanations are mandatory.

### Provenance and lifecycle contract

Before acceptance, provider output remains session-only and is not a durable reusable artifact. Rejected candidate content is purged; silence is neither acceptance nor rejection. Any retained nonaccepted-activity audit is minimal metadata without source text, packet content, generated interpretation, prompt, user revision, or provider response.

At acceptance, a durable hypothesis must initially preserve the exact packet, consent, transmission, source/artifact revisions, support and counter-evidence roles, provider/model and Harness versions, original AI text, user revision, review state, dependencies, and invalidation history. Exact packet retention is not permission to keep deleted or corrected source text forever. Under accepted Decision 16B, source correction or deletion invalidates the hypothesis, purges content-bearing source copies from the packet/provenance, and retains only the user-accepted hypothesis text plus a non-content tombstone. This approved design policy does not authorize cleanup implementation.

Source or artifact edits, rejections, deletions, supersession, or eligibility loss invalidate dependent hypotheses. They must never be silently rewritten. Rejected or invalidated hypotheses never re-enter context. Deleting a Phase 4 hypothesis purges its interpretation, user revision, packet, and content-bearing provenance; only founder-approved minimal non-content audit metadata may remain. Provider receipt cannot be undone by local deletion and must be disclosed.

### Sensitive inference contract

Phase 4 will categorically prohibit diagnosis; sexuality, religious, or political identity inference; moral-character judgment; immutable personality labels; identity finalization; silent profiling; deterministic recurrence; deterministic contradiction resolution; and authoritative change-over-time narratives.

### Provider contract

All providers must implement one versioned conceptual contract or refuse. Provider fallback cannot change the consented destination. Whole-history loading and background profiling remain prohibited.

## Phase 3 Entry Dependency

Acceptance of this ADR does not authorize implementation. Founder Decision 13A already resolves the Phase 3 entry dependency: all five audited gaps block Phase 4 production implementation, and no Roadmap deferral is approved.

## Migration Position

Acceptance of this ADR does not authorize migration implementation. Decision 11B approves only the additive normalized schema direction; exact schema, migration, rollback, and destructive behavior require a separate founder checkpoint after the Phase 3 foundations are complete.

Founder Decision 11B approves a separate additive normalized schema as the durable-artifact design direction after the Phase 3 revision/export foundations. Schema v4 and the generic source-scoped artifact table must not be overloaded. Exact schema, migration transaction, rollback, and implementation remain unauthorized pending a separate founder checkpoint.

## Retention And Deletion Position

Decisions 15B and 16B approve the retention and deletion design policies without authorizing implementation. Architecture/11 separates ephemeral selection, consent states, transmission outcomes, session-only output, accepted/invalidated/deleted artifacts, content-bearing packets, and non-content tombstones.

The accepted design package is:

- cancelled and unconsumed activity creates no durable record;
- failed, stale, successful-but-unaccepted, and rejected content remains session-only with a 24-hour maximum;
- a minimal non-content tombstone for consumed attempts expires after 30 days;
- cleanup failures remain visible and retryable until cleanup succeeds;
- accepted hypotheses remain durable and preserve AI original versus user revision;
- source correction/deletion invalidates the accepted hypothesis, purges duplicated source/packet content, and retains only accepted hypothesis text plus non-content provenance;
- explicit hypothesis deletion purges all interpretation and content-bearing provenance;
- provider receipt is irreversible.

Decisions 15B and 16B resolve these periods and behaviors as design policy. Full old-source or packet retention is not approved and would require separate explicit revision-content retention consent. Retention jobs, cleanup transactions, retries, and production verification remain unauthorized pending their implementation checkpoint.

## Consequences

- Phase 4 becomes a stronger reflective mirror without gaining authority over meaning or identity.
- Selection, consent states, transmission outcomes, candidate review, accepted/invalidated/deleted artifacts, packet content, audit tombstones, and future reuse remain separate states.
- Output complexity and evaluation cost increase because counter-evidence, alternatives, citations, and lifecycle are mandatory.
- Exact packet snapshots add local privacy cost. They exist durably only for accepted artifacts and must be purged when included source content is corrected/deleted under the recommended policy.
- Bounded non-content tombstones improve operational audit but require expiry, visible cleanup failure, and idempotent retry.
- Provider parity or refusal becomes a release gate.
- Phase 3 gaps remain visible instead of being silently bypassed.
- Phase 3A and Phase 3B continue to operate under their own accepted boundaries.

## Alternatives Considered

### Questions only

Safest and already close to Phase 3B, but does not test whether bounded cross-experience hypotheses improve understanding.

### User-owned hypotheses

Recommended. It permits recurrence, tension, and source-bound change observations while requiring counter-evidence, alternatives, uncertainty, and user acceptance.

### Direct Cross-Experience conclusions

Rejected. Persuasive narratives would exceed evidence, weaken agency, and make AI an oracle.

### Persistent longitudinal consent

Rejected. It turns an intimate, purpose-specific action into background authorization.

### Whole-history retrieval

Rejected. It increases privacy risk, confirmation bias, irrelevant sensitive-context use, and silent profiling.

### Reuse schema v4 or flatten into generic artifacts

Rejected as the recommended durable direction because it blurs Phase 3B and Phase 4 consent, task, dependency, and artifact taxonomy.

## Founder Acceptance Record

The founder explicitly recorded all sixteen decisions in `architecture/11`: **1B, 2B, 3B, 4B, 5B, 6A, 7B, 8B, 9B, 10B, 11B, 12A, 13A, 14B, 15B, 16B**. The record includes the Phase 3 exit approach, output authority, sufficiency, eligibility, counter-evidence, consent, source selection, persistence, reuse, provenance, invalidation, migration direction, sensitive inference, verification gates, nonaccepted-activity retention, and source correction/deletion lifecycle.

All sixteen answers were explicitly recorded on 2026/07/15–2026/07/16. The founder accepted ADR-0010 as a whole on 2026/07/16 and simultaneously approved `architecture/11` as `Founder-approved`. The recorded schema, retention, and deletion directions remain design authority only and do not authorize implementation.
