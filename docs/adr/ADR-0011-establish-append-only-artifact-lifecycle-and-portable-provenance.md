---
status: Accepted
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
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
  - docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md
  - docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md
referenced_by:
  - docs/00_Index.md
---

# ADR-0011: Establish Append-Only Artifact Lifecycle And Portable Provenance

## Status

Accepted by the founder on 2026/07/16 with all sixteen decisions in `architecture/12` recorded as `1B, 2B, 3B, 4B, 5B, 6B, 7B, 8B, 9A, 10B, 11A, 12A, 13B, 14B, 15B, 16B`.

Acceptance establishes the Phase 3C lifecycle, retention, deletion, export, rollback, and verification design policy only. It does not authorize schema v5 migration, retention cleanup, production implementation, UI activation, import implementation, provider transmission beyond existing ADR-0009 authority, or Phase 4 work. All Phase 3 exit gaps remain blocking until implementation and verification are complete.

## Context

ADR-0007 requires durable reviewed artifacts to preserve provenance, user review, revision history, correction, deletion, and export distinctions. Schema v4 preserves only the current source-scoped artifact payload, while Phase 3B separately preserves exact successful historical packets and actual-use provenance.

The Phase 3 exit audit records blocking gaps in revision/rejection history, post-review correction/deletion, complete artifact/provenance export, and user-facing provenance/dependency inspection. Mutable payloads and timestamps cannot represent these guarantees. Retaining rejected or deleted content merely for convenience would conflict with privacy and user control.

The decision must preserve three existing boundaries:

1. confirmation records the user's judgment, not objective truth;
2. AI hypotheses, user content, review decisions, and provenance remain distinct;
3. ADR-0009 remains authoritative for Historical Question packets, consent, actual-use provenance, and deletion.

## Decision

Life OS will use the founder-approved additive normalized Phase 3 lifecycle design direction with:

- stable source and artifact identities;
- append-only revision metadata and separate purgeable content rows;
- explicit append-only review and lifecycle events;
- exact revision-bound dependencies;
- deterministic current-state and context-eligibility projections;
- content-free tombstones for approved deletion and purge cases;
- a canonical versioned complete export independent of the SQLite layout;
- non-destructive feature disable rather than automatic down migration.

Rejected content is purged immediately while preserving only a content-free rejection event, digest, and minimal lifecycle metadata until parent deletion. Superseded content remains visible and context-ineligible until artifact deletion unless the user purges that revision. Correction creates a new revision that requires new confirmation. Ordinary dependent hypotheses remain visibly invalidated and ineligible. Artifact tombstones remain content-free and expire with the parent Experience. The exact schema migration remains unauthorized and requires a separate founder checkpoint.

Schema v4 Historical Question records remain governed by ADR-0009. Phase 3C must not retain them under Phase 4's hypothesis policy, broaden their task, or treat their packet snapshots as general revision storage.

## Consequences

- Correction becomes an explicit new revision rather than an in-place overwrite.
- Confirmation, rejection, correction, supersession, invalidation, deletion, and content purge become inspectable facts.
- Prior content can be purged without erasing the fact that a revision or deletion existed.
- Dependency invalidation can be transactional and understandable.
- Export can distinguish user content, AI hypothesis, review decision, current state, superseded state, and purged state.
- Migration complexity increases and must include honest v4 baseline backfill, reconciliation, restart, failure rollback, and minimum-version checks.
- A binary that predates schema v5 cannot be treated as a safe rollback target after v5 writes.
- Documentation approval still does not close any Phase 3 exit gap; implementation and founder verification remain required.

## Alternatives

### Extend schema v4 JSON payloads

Rejected because whole-history payload rewrites, weak relational constraints, and difficult purge/dependency queries make provenance fragile.

### Use a generic event-sourced database

Rejected for the smallest Phase 3 foundation because generic replay increases complexity and obscures artifact-specific constitutional distinctions.

### Keep only current state

Rejected because it leaves ADR-0007 and the audited Phase 3 exit gaps unresolved.

### Retain all rejected and deleted content

Rejected because audit convenience is not sufficient purpose for hidden retention of personal interpretation.

## Founder Acceptance Record

The founder accepted this ADR and the complete sixteen-decision package on 2026/07/16. `architecture/12` is `Founder-approved`.

Migration implementation was explicitly not authorized. Schema v5 migration, cleanup, production code, UI changes, import implementation, staging, commit, push, merge, and Phase 4 implementation remain unauthorized until separately approved.
