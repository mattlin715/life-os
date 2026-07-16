---
status: Founder-approved
version: 0.2
owner: product-and-engineering
last_updated: 2026/07/16
depends:
  - docs/00_Constitution.md
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/appendix/Harness.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/02_Experience_Export_Boundary.md
  - docs/architecture/04_Evidence_Candidate_Boundary.md
  - docs/architecture/06_Pattern_Candidate_Boundary.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
  - docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md
referenced_by:
  - docs/00_Index.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
---

# 12 Phase 3C Revision, Lifecycle, Provenance, And Export Foundation

## Purpose And Authority

This document records the founder-approved smallest coherent Phase 3 exit foundation for revision, review lifecycle, dependency inspection, and complete local export.

It is a **Founder-approved design gate**. Founder approval was recorded on 2026/07/16 with all sixteen decisions below. It does not authorize a migration, production code, destructive cleanup, UI activation, retention job, import, Phase 4 work, staging, commit, push, or merge. Migration and production implementation require separate explicit founder authority.

It closes no Phase 3 exit gap by documentation alone. Under Founder Decision 13A in `architecture/11`, all five gaps remain blocking until implementation, automated verification, and founder-manual verification are complete. Explicit emotion, relationship, value-conflict, and time-range retrieval is not designed here and remains a separate blocking Phase 3 exit gap.

**We Build Mirrors, Not Oracles.** A confirmed artifact records a user decision. It does not become objective truth. More history increases responsibility, not AI authority.

## Current Repository Baseline

The design begins from SQLite schema v4:

- `experience_entries` stores only the current Experience content and timestamps;
- `persisted_artifacts` stores one current JSON payload for Evidence, Reflection, Pattern, and Context Recovery artifacts;
- rejected legacy candidates are removed rather than preserved as review history;
- `historical_consent_events`, `historical_transmission_events`, `historical_question_artifacts`, and `historical_artifact_dependencies` preserve the accepted Phase 3B actual-use chain;
- Phase 3B source mutation deletes dependent Historical Question content and its successful packet snapshot;
- JSON and Markdown export cover Experience records, not the complete artifact and provenance graph;
- there is no general append-only source/artifact revision history or user-facing dependency inspector.

Migration cannot reconstruct prior values or rejected content that schema v4 never retained. Any future migration must create an honest baseline revision labelled `legacy_v4_baseline`; it must not fabricate earlier revisions, decisions, authorship, or deletion history.

## Scope

### In scope

1. Append-only revision metadata for persisted Experiences and Phase 3 artifacts.
2. Explicit user confirmation, rejection, correction, supersession, and deletion events.
3. Post-review correction and deletion for confirmed Evidence and confirmed single-Experience Pattern hypotheses.
4. Current, superseded, invalidated, rejected, and deleted projections without flattening them into one ambiguous status.
5. Exact revision-bound dependencies and a user-facing provenance/dependency inspection contract.
6. A complete, versioned source/artifact/revision/review/provenance export.
7. Compatibility rules for schema v4 and the ADR-0009 deletion lifecycle.
8. Non-destructive feature-disable and rollback behavior.

### Out of scope

- Phase 4 Cross-Experience Hypothesis storage or provider transport;
- Phase 4 packet, consent, retention-job, or cleanup implementation;
- recurrence, contradiction, change-over-time interpretation, historical summary, Awareness, Growth, diagnosis, advice, sensitive inference, or identity claims;
- a generic ontology intended to pre-fit future Phase 4 artifacts;
- whole-history loading or background profiling;
- import of the proposed complete export;
- the separate structured-retrieval exit gap;
- any Constitution or Book Zero redefinition.

## Required Concept Separation

| Concept | Meaning | Must not be flattened into |
| --- | --- | --- |
| User content | Text deliberately authored or edited by the user, such as Experience content or a saved Reflection response. | AI output merely because the user viewed it. |
| AI hypothesis | AI-authored tentative content, including an Evidence candidate or Pattern candidate. | User-authored content, objective fact, or identity. |
| Review decision | An explicit user action about one exact revision: confirm, reject, or skip where applicable. | Content revision or silence. |
| Revision | Immutable metadata for one exact content state, with authorship, digest, and predecessor. | Review state or mutable `updated_at`. |
| Rejection | An explicit review event for one exact candidate revision. | Inactivity, panel closure, deletion, or a negative identity claim. |
| Tombstone | Minimal content-free evidence that content was deleted or purged and must not be resurrected. | Hidden retained content. |
| Dependency | A typed edge from one exact dependent revision to one exact source revision or source record. | A loose current-ID reference. |
| Packet snapshot | The exact bounded content authorized and actually used for a governed historical provider call. | General revision storage or an export shortcut. |
| Export representation | A portable projection of the local graph with explicit content-present or content-purged states. | The live database schema. |
| Current state | The one revision and lifecycle projection presently eligible for display or governed reuse. | Every prior revision. |
| Superseded state | A prior revision replaced by an explicit later revision. It remains non-current and ineligible for context. | Rejected, deleted, or silently overwritten content. |

## Artifact Taxonomy And Authorship

The foundation remains Phase 3-specific:

| Record | Content authority | Review meaning |
| --- | --- | --- |
| Experience | User content. | Save or correction creates a new source revision; it is not an AI review decision. |
| Evidence | AI hypothesis until explicit confirmation; a user correction is a new user-authored revision of that reviewed artifact. | Confirmation means useful as reviewed Evidence, not objective truth. Rejection excludes it. |
| Reflection prompt | AI-authored prompt with provenance. | The prompt is not user content. |
| Reflection response | Saved user content attached to one exact prompt revision and exact source dependencies. | Saving or correcting the response is authorship, not confirmation of the prompt as truth. |
| Pattern | AI hypothesis before and after confirmation. | Confirmation means useful for continued reflection, never Evidence or Identity. |
| Context Recovery turn | User or AI content according to message authorship. | It is supporting conversation, not Evidence by default. |
| Historical Reflection Question | Governed Phase 3B generated artifact with exact successful packet provenance. | Its persistence follows ADR-0009 and is not upgraded into a Pattern or Phase 4 artifact. |

## Lifecycle Model

### Independent dimensions

One overloaded `status` field is insufficient. The design keeps four independent dimensions:

1. **Content lineage:** current revision, superseded revision, or content purged.
2. **Review decision:** pending, confirmed, rejected, skipped, or not applicable.
3. **Lifecycle:** active, invalidated, deleted, or content-purged tombstone.
4. **Context eligibility:** eligible or ineligible, with a deterministic reason.

Eligibility is derived from artifact kind, current revision, review decision, lifecycle, dependency validity, and the governing task. It is never inferred from existence alone.

### Commands and resulting events

| Explicit command | Required atomic result |
| --- | --- |
| Confirm | Append `review.confirmed` for the exact current revision and update the current projection. |
| Reject | Append `review.rejected`; make the artifact ineligible; apply the approved rejected-content retention rule. |
| Correct | Append a new immutable revision whose predecessor is the prior current revision; append `revision.superseded`; preserve authorship distinction; invalidate exact-revision dependents. |
| Supersede | Mark the prior revision non-current only because a specific later revision committed successfully. |
| Delete | Purge artifact content according to policy, append `lifecycle.deleted`, create a content-free tombstone if approved, and propagate dependency effects in the same transaction. |
| Purge prior revision | Remove only the content-bearing row, preserve immutable revision metadata and digest, and append `content.purged`. |

Selection, opening an inspector, silence, timeout, navigation, or application close creates none of these review decisions.

### Correction rules

- A correction never updates content in place.
- The new revision records `authored_by = user`, a reason class, and the exact predecessor revision.
- Original AI authorship and provenance remain attached to the prior AI revision while its content is retained.
- Every corrected Evidence revision returns to pending review and requires new explicit confirmation before governed reuse, as approved in Decision 7B.
- A corrected Pattern remains a hypothesis. User editing does not convert it into Evidence or identity truth.
- Dependents bound to the old revision are never silently rebound to the new revision.

## Schema Alternatives — No Migration Created

### Alternative A: extend JSON payloads in schema v4

Store revision arrays, review events, dependencies, and tombstones inside `persisted_artifacts.payload`.

- Benefit: few tables and a small first patch.
- Risks: rewrites the whole history on each edit; weak foreign-key enforcement; difficult atomic purge; poor dependency queries; easy provenance drift; overloads a current-state table.
- Assessment: not recommended.

### Alternative B: additive normalized lifecycle schema v5

Add normalized Phase 3 lifecycle tables while keeping schema v4 tables readable during controlled cutover.

Proposed logical records:

| Logical record | Purpose |
| --- | --- |
| `source_heads` | Stable Experience identity and current revision pointer. |
| `source_revisions` | Append-only Experience revision metadata, sequence, digest, authorship, predecessor, and timestamp. |
| `source_revision_content` | Content-bearing Experience payload separable from immutable metadata for authorized purge. |
| `artifact_heads` | Stable artifact identity, Phase 3 kind, source Experience, current revision pointer, and current projections. |
| `artifact_revisions` | Append-only revision metadata, sequence, digest, authorship, provenance reference, and predecessor. |
| `artifact_revision_content` | Content-bearing payload separable from immutable revision metadata. |
| `artifact_review_events` | Append-only explicit confirmation, rejection, and skip decisions for exact revisions. |
| `artifact_lifecycle_events` | Append-only correction, supersession, invalidation, deletion, and purge facts. |
| `artifact_dependencies` | Typed exact-revision edges for source, Evidence, Reflection, Pattern, and Historical Question relationships. |
| `content_tombstones` | Minimal content-free deletion or purge metadata; never reusable context. |
| `provenance_records` | Deduplicated immutable authorship/provider/model/Harness/prompt/schema metadata where present. |

Content is separated from immutable metadata so an authorized purge can remove text without pretending that a deletion never happened. No `INSERT OR REPLACE` is permitted. Current pointers change only after all revision, event, dependency, and compatibility writes pass in one transaction.

- Benefit: enforceable lineage, exact dependency inspection, safe content purge, complete export, and a clean Phase 3 foundation.
- Risks: more tables, migration/backfill complexity, transactional cutover, and no safe automatic binary downgrade after v5 writes.
- Assessment: recommended, subject to explicit founder migration approval.

### Alternative C: append-only event log with computed state

Persist every command as a generic event and rebuild all current state by replay.

- Benefit: maximum historical fidelity and one conceptual write path.
- Risks: generic event semantics obscure the Phase 3 taxonomy; replay and migration are complex; constraints move into application code; export and purge become harder to prove.
- Assessment: not recommended for the smallest production foundation.

## Proposed v5 Invariants

Decision 3B approves the normalized v5 design direction. Exact migration implementation remains unauthorized; if separately approved, its design must prove:

1. A stable source or artifact has at most one current revision.
2. Revision sequence is unique and monotonic within its stable identity.
3. Revision metadata is never updated or replaced.
4. Content can be deleted only through the approved purge transaction, leaving explicit metadata state.
5. Every review event names an exact subject revision and explicit user action.
6. Every dependency names the exact source revision actually used when one exists.
7. A superseded, rejected, invalidated, deleted, or content-purged revision is context-ineligible.
8. A current pointer cannot reference missing or purged content.
9. Dependency invalidation, lifecycle events, tombstones, and current projection changes commit atomically.
10. Legacy schema v4 records receive one honest baseline revision; missing history remains declared missing.
11. Phase 3B consent, transmission, packet digest, provider/model, source revisions, and artifact dependencies remain transactionally consistent.
12. Foreign keys are enabled and no replacement write can trigger accidental provenance cascades.

## Schema v4 And ADR-0009 Compatibility

Phase 3C must not reinterpret the Phase 3B packet or broaden its output authority.

- Existing schema v4 Historical Question rows remain readable and exportable.
- Existing `persisted_artifacts` rows remain a compatibility projection during the approved cutover; normalized v5 records become authoritative only after migration verification.
- A v4 record receives one `legacy_v4_baseline` revision. No prior rejection or revision is invented.
- The Phase 3B historical packet remains separate from the single-Experience `ContextPacket` and from generic artifact revision content.
- Consent and transmission records remain governed by ADR-0009 retention.
- Correcting, rejecting, or deleting an included Phase 3B source invalidates the exact dependency.
- ADR-0009 remains authoritative: source deletion cascade-deletes dependent Historical Question generated content and its successful packet snapshot by default. Phase 3C must not replace this with Phase 4's retained-invalidated-hypothesis policy.
- No additional Phase 3B transport tombstone is introduced unless the founder separately changes ADR-0009.
- Local deletion still cannot undo provider receipt.

## Dependency And Invalidation Rules

Dependencies are directional and revision-bound:

`dependent artifact revision -> exact source Experience or artifact revision`

Proposed relationship types are limited to actual Phase 3 relationships, such as `derived_from_experience`, `uses_evidence`, `answers_prompt`, `uses_reflection_response`, and `historical_packet_item`. They are not Phase 4 support/counter-evidence roles.

On source correction, rejection, supersession, deletion, or eligibility loss:

1. close any open preflight using the affected revision;
2. never rewrite or substitute the dependency;
3. mark ordinary dependent Phase 3 hypotheses visibly invalid and ineligible, or delete them according to the approved artifact-class policy;
4. apply ADR-0009 cascade deletion to dependent Historical Questions;
5. require a new user action, packet, and consent for any regeneration.

## Retention And Deletion Alternatives

### Rejected candidate content

| Alternative | Rule | Assessment |
| --- | --- | --- |
| R-A | Retain exact rejected content indefinitely. | Strong audit, high privacy cost, and incompatible without a specific purpose and retention rule. |
| R-B | Purge rejected content immediately after the rejection transaction; retain only a content-free rejection event and digest until the parent Experience or artifact is deleted. | Recommended. Preserves explicit review history without creating a reusable archive. |
| R-C | Purge both content and rejection metadata immediately. | Privacy-maximal but does not satisfy the Phase 3 rejection-history gap. |

Rejected content, its digest, and its event are never eligible for retrieval, generation, evaluation learning, or profiling.

### Superseded revision content

| Alternative | Rule | Assessment |
| --- | --- | --- |
| S-A | Keep prior content visible locally and in complete export until the user deletes the artifact; offer explicit per-revision purge. | Recommended for meaningful revision history and correction transparency. |
| S-B | Purge prior content immediately and keep metadata/digest only. | Strong privacy, but weakens user inspection of what changed. |
| S-C | Retain prior content forever even after artifact deletion. | Rejected; deletion would be misleading. |

### Artifact deletion

| Alternative | Rule | Assessment |
| --- | --- | --- |
| D-A | Purge all artifact revision content; retain a minimal content-free tombstone until the parent Experience is deleted; invalidate or delete dependents by policy. | Recommended for understandable deletion and anti-resurrection. |
| D-B | Hard-delete all content and metadata immediately. | Strong erasure but weak audit and dependency explanation. |
| D-C | Soft-delete while retaining hidden content. | Rejected; it violates user expectations and increases hidden retention. |

Deleting the parent Experience purges its source revision content, all source-scoped artifact content, approved tombstones, and dependency copies. Phase 3B dependent content follows ADR-0009 cascade behavior. Any exception requires a separate explicit purpose, retention duration, and founder approval.

## Provenance And Dependency Inspection Contract

The user-facing inspector must answer, without database knowledge:

1. What is this item: user content, AI hypothesis, review decision, or generated question?
2. Who authored the current revision and who authored prior revisions?
3. What exact source records and revisions shaped it?
4. Which provider, model, Harness, prompt, and schema versions were used, if any?
5. What did the user explicitly confirm, reject, correct, or delete?
6. Which revision is current and which are superseded or purged?
7. Which other artifacts depend on this item?
8. Is it eligible for future context, and if not, why?
9. Does an exact packet snapshot exist, and what consent/transmission chain governs it?
10. What will happen to dependents before the user confirms correction or deletion?

The inspector must not imply that provenance proves truth. IDs and digests may be expandable technical details, but human-readable authorship, lifecycle, sources, and consequences must be primary. English, Traditional Chinese, and Japanese must express the same contract.

## Complete Export Proposal

### Canonical machine format

Introduce `life-os-export-v2` as a UTF-8 JSON bundle. It is an export representation, not the SQLite layout.

```json
{
  "format": "life-os-export",
  "formatVersion": 2,
  "exportedAt": "ISO-8601",
  "application": { "version": "...", "schemaVersion": 5 },
  "manifest": { "contentDigestAlgorithm": "sha256", "bundleDigest": "..." },
  "sources": [],
  "sourceRevisions": [],
  "artifacts": [],
  "artifactRevisions": [],
  "reviewEvents": [],
  "lifecycleEvents": [],
  "dependencies": [],
  "provenance": [],
  "historicalPackets": [],
  "consentReferences": [],
  "transmissionReferences": [],
  "tombstones": []
}
```

Every content-bearing revision declares `contentState: present`; a purged revision declares `contentState: purged`, its digest, purge reason class, and timestamp without the removed text. Current and historical views are explicit. Missing legacy history is labelled `historyState: unavailable_before_v5`, not silently omitted.

The export includes exact Phase 3B successful packet snapshots only while the corresponding generated artifact still exists and ADR-0009 permits retention. It excludes API keys, credential locations, provider authorization headers, raw provider error bodies, unsaved drafts, ephemeral selection, cancelled preflights, and data already purged.

### Human-readable companion

Markdown export may add a human-readable lifecycle and provenance report, but it is not the complete or restorable representation. It must clearly mark AI hypotheses, user content, confirmation, rejection, supersession, invalidation, and purged content.

### Versioning and integrity

- `formatVersion` changes only for incompatible representation changes.
- Every record has a stable ID and record-level digest where content exists.
- The manifest lists counts by record type, omitted-by-policy counts, and digest algorithm.
- Canonical digest input uses deterministic key ordering, UTF-8, normalized line endings, and locale-independent serialization.
- Export failure is atomic: write to a temporary local file, flush, validate, then rename; partial bundles are removed.
- Import remains unsupported until separately designed and founder-approved. Export must not imply restorability before that gate.

## Rollback And Feature Disable

The safe rollback is non-destructive:

1. close lifecycle mutation controls and export v2 generation;
2. keep existing data readable through a version-aware compatibility reader;
3. keep schema v5 tables and `user_version` intact;
4. do not automatically down-migrate, drop history, or rewrite v5 current state into an older binary;
5. preserve Phase 3B generation, deletion, and provenance behavior unless its own feature is disabled;
6. keep existing artifacts inspectable but make unsupported mutation paths fail closed with a calm explanation;
7. require a forward fix or founder-approved destructive recovery plan before any data removal.

Before migration, the implementation plan must define backup, integrity check, injected-failure rollback, v4-to-v5 count/digest reconciliation, restart recovery, and the minimum compatible application version. A feature flag is not a substitute for migration rollback proof.

## Evaluation Matrix

| Case | Required result |
| --- | --- |
| Confirm one exact Evidence revision | One explicit review event; no content rewrite; current revision unchanged. |
| Silence or panel close | No confirmation or rejection event. |
| Reject candidate Evidence or Pattern | Content is purged under Decision 8B; content-free rejection history remains; never context-eligible. |
| Correct confirmed Evidence | New user-authored revision; original AI authorship remains distinct; old revision superseded; dependent artifacts handled atomically. |
| Correct confirmed Pattern | New user-authored hypothesis revision; never becomes Evidence or Identity. |
| Concurrent correction | One transaction wins; stale command writes no partial revision, event, or dependency changes. |
| Delete artifact | Content purged according to policy; tombstone and dependent effects agree; no hidden reusable content. |
| Delete Experience | Source and source-scoped content purged; Phase 3B dependents cascade-delete under ADR-0009. |
| Purge prior revision | Metadata/digest remains; content is absent from inspector, export, packets, logs, and future context. |
| Dependency inspection | Exact source and dependent revisions, authorship, lifecycle, and eligibility reason are visible. |
| Legacy v4 backfill | One honest baseline revision per live record; no invented history; counts and digests reconcile. |
| Injected migration failure | Entire migration rolls back; schema version and v4 data remain unchanged. |
| Restart during cutover | Database opens in one valid state; no mixed authority or silent loss. |
| Phase 3B source mutation | Open preflight closes; in-flight result fails stale; persisted dependent question follows ADR-0009 deletion. |
| Complete JSON export | All retained sources, artifacts, revisions, events, dependencies, provenance, eligible packets, and tombstones are represented. |
| Export after purge | Purged content is absent; explicit purged marker remains; bundle digest validates. |
| Export cancellation/failure | No partial destination file and no mutation of local records. |
| Secret exclusion | No API key, auth header, credential path, or raw provider error body appears. |
| Multilingual parity | English, Traditional Chinese, and Japanese show equivalent lifecycle actions, warnings, and consequences. |
| Feature disable | Reads remain safe; unsupported writes fail closed; no schema decrement or data deletion. |
| Phase 4 separation | No Phase 4 artifact, packet, transmission, recurrence, contradiction, or cross-time interpretation is introduced. |

## Numbered Founder Decisions

All sixteen decisions were explicitly approved by the founder on 2026/07/16. The accepted package is `1B, 2B, 3B, 4B, 5B, 6B, 7B, 8B, 9A, 10B, 11A, 12A, 13B, 14B, 15B, 16B`. These answers approve the design policy only. Schema v5 migration, retention cleanup, production implementation, UI activation, import implementation, and Phase 4 work remain unauthorized.

### 1. Foundation scope

- **1A:** revision/lifecycle only for confirmed Evidence and Pattern.
- **1B:** one Phase 3 lifecycle foundation for Experiences, Evidence, Reflection, Pattern, Context Recovery, and Historical Questions, with artifact-specific rules.
- **Recommendation:** **1B**, because export and dependency inspection cannot be complete if related records remain outside the model.
- **Founder answer (2026/07/16):** **1B approved.** Experiences, Evidence, Reflection, Pattern, Context Recovery, and Historical Questions share one Phase 3 lifecycle foundation while retaining artifact-specific rules.

### 2. Experience revision history

- **2A:** keep only current Experience text and timestamp.
- **2B:** create append-only Experience revision metadata/content using the same purge boundary as artifacts.
- **Recommendation:** **2B**, because exact source revision inspection and correction cannot rely on mutable `updated_at` alone.
- **Founder answer (2026/07/16):** **2B approved.** Experience revisions are append-only and use the same content-purge boundary as artifact revisions.

### 3. Schema strategy

- **3A:** extend schema v4 JSON payloads.
- **3B:** additive normalized schema v5 with separate heads, revision metadata/content, events, dependencies, provenance, and tombstones.
- **3C:** generic event-sourced store.
- **Recommendation:** **3B**. This decision authorizes only a design direction, not migration implementation.
- **Founder answer (2026/07/16):** **3B approved as the schema design direction; migration implementation is not authorized.**

### 4. Current-state authority during cutover

- **4A:** immediately delete or ignore schema v4 rows after backfill.
- **4B:** normalized v5 becomes authoritative after verified cutover; v4 rows remain a transactionally maintained compatibility projection until a later approved removal.
- **Recommendation:** **4B**.
- **Founder answer (2026/07/16):** **4B approved.** Normalized v5 may become authoritative only after verified migration cutover; v4 remains a temporary compatibility projection. Migration implementation is not authorized.

### 5. Review semantics

- **5A:** mutable status on the current payload is sufficient.
- **5B:** confirmation, rejection, and skip are append-only explicit review events bound to exact revisions; silence creates no event.
- **Recommendation:** **5B**.
- **Founder answer (2026/07/16):** **5B approved.** Confirmation, rejection, and applicable skip actions are explicit append-only review events bound to exact revisions. Silence and interface interaction are not review decisions.

### 6. Correction semantics

- **6A:** edit the confirmed payload in place.
- **6B:** append a user-authored correction revision, supersede the prior revision, preserve authorship, and invalidate exact-revision dependents without rebinding.
- **Recommendation:** **6B**.
- **Founder answer (2026/07/16):** **6B approved.** Correction creates a user-authored revision, supersedes the prior revision, and invalidates dependents bound to the prior revision. In-place overwrite and automatic rebinding are prohibited.

### 7. Confirmation after correction

- **7A:** confirmation automatically carries to every corrected Evidence or Pattern revision.
- **7B:** correction creates a new current revision that requires an explicit confirm action before governed reuse.
- **7C:** carry confirmation for user typo-only edits based on an automatic classifier.
- **Recommendation:** **7B**. It is legible and does not let a classifier decide user intent.
- **Founder answer (2026/07/16):** **7B approved.** Confirmation applies only to one exact revision. A corrected revision requires new explicit confirmation before governed context eligibility.

### 8. Rejected-content retention

- **8A:** retain exact rejected content indefinitely.
- **8B:** purge rejected content immediately and retain only a content-free rejection event/digest until parent deletion.
- **8C:** purge content and all rejection metadata immediately.
- **Recommendation:** **8B**, for a real rejection history without a reusable rejected-content archive.
- **Founder answer (2026/07/16):** **8B approved.** Rejected content is purged immediately. Only a content-free rejection event, digest, and minimal lifecycle metadata remain until parent deletion, and none may enter retrieval, generation, profiling, or Harness learning.

### 9. Superseded-content retention

- **9A:** retain prior revision content visibly until artifact deletion, with an explicit per-revision purge control.
- **9B:** purge superseded content immediately and retain only metadata/digest.
- **9C:** retain it even after artifact deletion.
- **Recommendation:** **9A**; **9C** is prohibited.
- **Founder answer (2026/07/16):** **9A approved.** Superseded content remains visible but permanently context-ineligible until artifact deletion. The user may purge an individual prior revision, leaving only content-free metadata, digest, and an explicit marker.

### 10. Ordinary Phase 3 dependent artifacts

- **10A:** source mutation cascade-deletes every dependent artifact.
- **10B:** retain ordinary user-reviewed dependent hypotheses as visibly invalid and ineligible, while allowing explicit deletion; never rewrite them.
- **10C:** keep dependents current and silently rebind them.
- **Recommendation:** **10B** for ordinary Phase 3 artifacts; **10C** is prohibited.
- **Founder answer (2026/07/16):** **10B approved.** Ordinary Phase 3 dependent hypotheses remain visibly invalidated and context-ineligible after source invalidation. They are never rewritten, recalculated, or rebound automatically, and the user may delete them completely.

### 11. Phase 3B dependent Historical Questions

- **11A:** preserve ADR-0009: source deletion cascade-deletes dependent generated content and successful packet snapshot; correction/rejection makes it ineligible and applies the approved deletion transaction.
- **11B:** retain Historical Questions as invalidated text using the Phase 4 hypothesis policy.
- **Recommendation:** **11A**. **11B** would weaken or silently amend ADR-0009.
- **Founder answer (2026/07/16):** **11A approved.** Any included-source correction, rejection, deletion, or eligibility loss cascade-deletes dependent Historical Question generated content and its successful packet snapshot under ADR-0009. No retained-invalidated policy or additional transport tombstone is introduced.

### 12. Artifact deletion tombstone

- **12A:** retain a minimal content-free tombstone until the parent Experience is deleted.
- **12B:** hard-delete all artifact metadata immediately.
- **12C:** retain hidden artifact content behind a deleted flag.
- **Recommendation:** **12A**; **12C** is prohibited.
- **Founder answer (2026/07/16):** **12A approved.** Artifact deletion purges all content and retains only a minimal content-free tombstone until parent Experience deletion, solely for dependency explanation, anti-resurrection, and export. It is ineligible for retrieval, generation, profiling, and Harness learning.

### 13. Complete export

- **13A:** extend Experience-only export with current artifact payloads only.
- **13B:** add canonical `life-os-export-v2` JSON containing all retained current and historical records, decisions, dependencies, provenance, allowed packets, and tombstones; keep Markdown as a human companion.
- **Recommendation:** **13B**.
- **Founder answer (2026/07/16):** **13B approved.** Canonical `life-os-export-v2` JSON represents all retained sources, artifacts, revisions, review/lifecycle events, dependencies, provenance, permitted Phase 3B packets, and tombstones. Markdown is a human-readable companion. Import implementation is not authorized.

### 14. Exported deleted/purged state

- **14A:** omit every purged record without explanation.
- **14B:** export content-free tombstones and explicit `contentState: purged` markers while excluding removed content.
- **Recommendation:** **14B**, so absence is understandable without undoing deletion.
- **Founder answer (2026/07/16):** **14B approved.** Export uses content-free tombstones and explicit `contentState: purged` markers without retaining, reconstructing, or exporting deleted content.

### 15. Rollback authority

- **15A:** allow automatic down migration to schema v4.
- **15B:** non-destructive feature disable only; keep v5 readable, never decrement schema, and require a separate founder gate for destructive recovery.
- **Recommendation:** **15B**.
- **Founder answer (2026/07/16):** **15B approved.** Only non-destructive feature disable and forward fix are authorized. Automatic down migration, schema-version decrement, lifecycle-data deletion, and use of an older binary as safe rollback are prohibited. Destructive recovery requires a new founder approval.

### 16. Activation gate

- **16A:** automated tests are sufficient.
- **16B:** require migration/restart/rollback/export/lifecycle tests plus a fixed English, Traditional Chinese, and Japanese founder-manual matrix before activation.
- **Recommendation:** **16B**.
- **Founder answer (2026/07/16):** **16B approved.** Activation requires complete automated contract tests and a fixed English, Traditional Chinese, and Japanese founder-manual verification matrix. Phase 3C cannot be declared complete or activated before explicit founder acceptance.

## Staged Implementation Sequence — Not Authorized Yet

1. **Decision closure:** record all founder answers; accept or reject ADR-0011; name the exact migration and retention authority.
2. **Contract-only implementation:** add domain types, pure lifecycle decisions, export schema, and evaluation fixtures without changing storage.
3. **Migration proof:** implement schema v5 only after explicit approval; test backup, backfill, reconciliation, injected rollback, restart, and no invented history.
4. **Transactional repository:** implement append-only revisions, explicit review/lifecycle events, exact dependencies, purge, tombstone, and stale-write rejection.
5. **Evidence vertical slice:** add post-review inspect, correct, reconfirm, reject, and delete for confirmed Evidence.
6. **Pattern and Reflection parity:** add hypothesis-safe Pattern lifecycle and saved user Reflection revision behavior without identity promotion.
7. **Phase 3B compatibility:** bind v5 source/artifact revisions while preserving immutable packet, consent, transaction revalidation, and ADR-0009 cascade behavior.
8. **Provenance inspector:** expose authorship, current/superseded state, sources, dependents, eligibility, and consequences in three languages.
9. **Complete export v2:** produce and validate atomic JSON plus optional Markdown companion; keep import disabled.
10. **Founder verification and promotion:** run canonical verification, execute the fixed manual matrix, correct defects, obtain explicit promotion approval, and only then commit/merge.
11. **Separate Phase 3 retrieval sprint:** design and implement emotion, relationship, value-conflict, and time-range retrieval. Phase 4 remains blocked until this fifth gap also passes its own gate.

## Founder Acceptance Record

On 2026/07/16, the founder accepted ADR-0011, approved Decisions `1B, 2B, 3B, 4B, 5B, 6B, 7B, 8B, 9A, 10B, 11A, 12A, 13B, 14B, 15B, 16B`, and approved this architecture as `Founder-approved`.

This acceptance does not authorize schema v5 migration, retention cleanup, production implementation, UI activation, import implementation, provider transmission beyond existing ADR-0009 authority, staging, commit, push, merge, or Phase 4 work. All Phase 3 exit gaps remain blocking until their implementations, automated verification, and founder-manual verification are complete.
