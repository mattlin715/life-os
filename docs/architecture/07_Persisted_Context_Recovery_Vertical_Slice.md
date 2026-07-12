---
status: Implemented
version: 1.2
owner: product-and-engineering
last_updated: 2026-07-12
depends:
  - docs/00_Constitution.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/11_MVP.md
referenced_by: []
---

# Persisted Context Recovery Vertical Slice (Implemented)

## Boundary

This source-scoped vertical slice persists reviewed context locally in SQLite schema version 3: evidence, reflection prompts and responses, pattern candidates, and optional Context Recovery turns. It does not retrieve other Experiences, create longitudinal memory, or make identity claims.

## Correctness rules

- A complete artifact bundle is replaced in one SQLite transaction; a failed insert rolls the delete back.
- Editing an Experience atomically clears every derived artifact and recovery turn. Deleting it clears the same dependents.
- Evidence or patterns marked rejected are removed from durable storage immediately. They are never hydrated, sent to an AI provider, retained as hidden evaluation data, exported, or imported.
- A skipped reflection/recovery question is a durable user choice, not rejection feedback and not extra context.
- Generated questions/artifacts and user-authored responses carry distinct provenance. Legacy hydrated prompt/response provenance is explicitly `legacy_unknown`, never inferred as user confirmation.
- Reflection textarea edits live in an explicit UI-draft state, keyed by exact source Experience ID and exact Reflection Prompt ID. Only an explicit Save writes trimmed `response`, `status`, user `responseProvenance`, and `updatedAt` together; only a successful durable commit clears that exact submitted draft. Failed or stale saves preserve it. Skip clears the draft and response provenance. A restart hydrates only the committed response.
- A dirty Reflection draft blocks Pattern generation with localized low-pressure guidance. It is neither included in a Context Packet nor silently autosaved.
- Pattern availability is one deterministic decision shared by rendering and execution: pending generation, confirmed Evidence, dirty Reflection drafts, then the Context Gate. Insufficient context is visible, rather than silently refused. An explicit Add Context request can create one optional suggested Context Recovery turn, including after a prior skip; it never duplicates an open suggestion or treats Reflection responses as event context.
- Before a generated result is applied, its complete snapshot is revalidated inside the serialized artifact-mutation queue against the current Experience revision, answered recovery turns, confirmed Evidence, answered user-authored Reflections, and current Harness/prompt versions. This closes same-process queue races; the SQLite expected-Experience revision check remains the final write-time protection and is not a cross-process guarantee.

## Context recovery

The deterministic Context Sufficiency Gate can ask one source-scoped, optional clarification question for sparse input. Answering is optional, skipping does not deepen inference, and no Experience save is blocked. The context packet is limited to the current Experience, answered recovery turns, confirmed evidence, answered reflection responses, locale, provider/model, Harness version, prompt version, and requested task. In-flight results are discarded when the Experience or snapshot dependencies change.

## Remaining work before Definition of Done

Provider requests now consume the same validated task-specific Context Packet. SQLite schema migration and atomic mutations run through a Rust SQLx seam on one connection; integration tests cover v2-to-v3 preservation, rejected exclusion, injected failure rollback, and unchanged `user_version`.

The document remains `in_progress` until founder manual desktop verification is completed across restart and multilingual flows.

## Deferred

Cross-experience retrieval, longitudinal summaries, revision history, artifact export/import, accounts/sync, identity engines, diagnosis, and retained rejected-artifact evaluation data remain out of scope.
