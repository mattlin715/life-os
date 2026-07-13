---
status: Implemented
version: 0.6
owner: product-and-engineering
last_updated: 2026/07/14
depends:
  - docs/00_Constitution.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/10_Privacy.md
  - docs/architecture/00_MVP_Architecture.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
referenced_by:
  - docs/00_Index.md
  - docs/11_MVP.md
  - docs/architecture/00_MVP_Architecture.md
  - docs/product/00_MVP_User_Flow.md
---

# 08 Local Historical Context Selection Foundation (Implemented)

## Purpose

Phase 3A lets a user inspect a small local-only set of possible prior Experience sources. It prepares context assembly; it does not perform Cross-Experience Reflection, pattern generation, identity inference, or provider transmission.

## Candidate Model

Each ephemeral candidate contains its exact source Experience ID, ISO source timestamps, a bounded excerpt, visible shared-term reasons, eligible confirmed Evidence IDs, eligible user-authored answered Reflection IDs, and deterministic ranking metadata. Candidate timestamps remain source data. The UI formats a valid timestamp with the explicit active Life OS language (`en`, `zh-TW`, or `ja`) and the device/user timezone; locale selection does not change the represented instant. A valid source is parsed under the governed timestamp contract and separately serialized as normalized UTC machine-readable text for an HTML `dateTime` attribute only when that normalized value remains within the unsigned positive four-digit HTML year boundary; the raw source remains unchanged in persistence. A presentation-valid source that normalizes outside that HTML boundary uses a non-time element without an attribute. Invalid or empty imported timestamps do not crash the panel and render a localized calm fallback instead, without an HTML `dateTime` attribute. An authored-timezone field is deferred; it is not inferred or written during this phase.

Selection is a separate, UI-only `Map<string, ReadonlySet<string>>`: the outer key is the exact current Experience ID and the inner set holds exact source Experience IDs. Immutable updates create replacement maps and sets suitable for React state. This avoids delimiter-composed identities and object-prototype lookup, including for imported IDs such as `__proto__`, `constructor`, `prototype`, `toString`, or IDs containing `:`. The map is deliberately non-serializable by design; it is not persisted and no schema migration is needed.

Open-panel state is a separate UI-only `ReadonlySet<string>` of exact current Experience IDs. It is likewise immutable, prototype-safe, non-serializable, and has no delimiter-composed identity. Removing an Experience removes its exact panel state.

Raw Experience text, confirmed Evidence, user Reflection, and AI hypotheses remain distinct. Retrieval receives only persisted Experiences, eligible persisted artifact records, locale, and its bounded limit. Unsaved Reflection drafts are absent from the retrieval type and call boundary; they are not merged into committed Reflection records. Rejected Evidence, skipped/foreign/orphaned Reflection, and deleted Experiences are not eligible historical evidence.

## Local Retrieval Rule

`local-lexical-v1` runs only after the user explicitly opens that Experience's local-history panel. Initial hydration and closed panels perform no candidate retrieval; opening one panel retrieves only for that exact current Experience. English uses deterministic explicit-locale case normalization and meaningful terms. Traditional Chinese and Japanese normalize text with NFKC, split it into contiguous Unicode letter/number runs, and create visible adjacent-character terms only within each run. Punctuation, whitespace, newlines, emoji, and symbols cannot manufacture a shared term across a boundary. A source appears only when it has visible overlap. Results exclude the current Experience, are limited to at most three by default, and sort by score, source timestamp, then exact source ID. No relevant overlap produces no candidate.

This is a modest retrieval aid, not a claim that similarity creates meaning. The user can inspect each source, include or exclude it, clear the selection, or continue with no history.

## Ephemeral Selection Reconciliation

A pure reconciliation helper retains selections only for an explicitly open panel's current eligible candidate set. If a candidate disappears because its artifact is rejected, its source Experience changes, or bounded ranking removes it, that selection is removed. If it reappears later, it remains unselected until the user chooses it again. Unrelated current/source selections remain unchanged. Closing a panel preserves its valid ephemeral selection for the current app session but stops retrieval; reopening retrieves fresh local candidates and reconciles before the browser paints the panel. Experience deletion removes the Experience both as a current key and as a source from other current keys. Reconciliation creates no durable state.

## Provider Boundary

Historical candidates and selection state do not enter `ContextPacket`, the shared OpenAI/Gemini/mock transport payload, provider adapters, mock input, or persistence. Existing evidence, reflection, and pattern requests retain their single-Experience packet. Opening, retrieving, selecting, excluding, or clearing local sources makes no provider call. A later phase needs founder-approved consent and a new transmission design before any selected historical material can be sent to a provider.

## Deferred

Durable selection, consent records, historical packet transmission, embeddings, vector search, Cross-Experience Reflection, longitudinal summaries, and Phase 3B policy are deferred.
