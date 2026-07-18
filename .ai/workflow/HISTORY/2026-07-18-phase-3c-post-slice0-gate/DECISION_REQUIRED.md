# Decision Required

Status: resolved
- Sprint ID: 2026-07-18-phase-3c-post-slice0-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-18T11:38:35.628Z
- Updated at: 2026-07-18T11:38:35.628Z

## Decision ID

PHASE3C-SLICE1A-001

## Sprint ID

2026-07-18-phase-3c-post-slice0-gate

## Decision Summary

Decide whether the promoted Slice 0 evidence may lead to a bounded Slice 1A production startup-safety implementation, while Slice 1B and every schema-v5 migration capability remain deferred.

## Why Automation Stopped

Architecture/13 is Founder-approved but explicitly states that Slice 1 and production behavior require another Founder checkpoint. Passing Slice 0 tests and promotion prove only the fixed test contract. They do not authorize changes to startup behavior, application version declarations, or production code.

## Relevant Constitution Clauses

- `docs/00_Constitution.md`: Human before AI; Privacy before Profit; Evidence before Conclusion; Documentation Hierarchy; We Build Mirrors, Not Oracles.
- The Constitution is not proposed for modification.

## Relevant Primary Definitions

- `docs/03_Principles.md`: evidence, trust, privacy, user agency, and visible uncertainty.
- `docs/06_Memory.md`: user-controlled, revisable, provenance-preserving memory.
- `docs/10_Privacy.md`: transparent local data behavior as psychological safety.

## Relevant ADRs

- ADR-0007 requires durable provenance and understandable lifecycle safety.
- ADR-0009 keeps the existing schema-v4 Historical Question lifecycle unchanged.
- ADR-0011 accepts the Phase 3C design direction but withholds production implementation without a separate checkpoint.
- ADR-0008 governs the workflow only and supplies no product authority.

## Available Options

### Option A — Authorize Slice 1A only (recommended)

Authorize the smallest startup-safety vertical slice: inspect database presence/version before writable initialization, refuse versions greater than the current supported maximum of 4, expose a typed local compatibility state, block store construction/cleanup/reads/writes when incompatible, preserve current v4/fresh/v2/v3 compatibility behavior, synchronize application version declarations to `0.2.0` if required by architecture/13, and test only with synthetic/disposable databases. Explicitly defer Slice 1B and all v5 work.

### Option B — Defer all production implementation

Keep the current v4 production path unchanged. Retain this decision package and perform no Slice 1A or Slice 1B code changes.

### Option C — Authorize combined Slice 1A and Slice 1B

Implement startup safety and replace renderer-supplied generic SQL mutation paths with typed Rust commands in one sprint. This would broaden the write surface and requires a separate, more detailed mutation-contract review; it is not recommended by this package.

## Benefits

- **A:** Directly closes the newer-database acceptance hazard with the smallest reviewable change; keeps schema and data semantics at v4; creates a safe prerequisite for later work.
- **B:** Zero implementation risk now; no production files change.
- **C:** Reaches the eventual typed boundary sooner and reduces one future transition, but only by combining two materially different risk surfaces.

## Risks

- **A:** Startup ordering changes can regress fresh install or v2/v3-to-v4 compatibility; read-only inspection must not accidentally create/write a database; `0.2.0` must not imply v5 support.
- **B:** The current binary continues to accept `user_version >= 4` and can open a newer database without a dedicated compatibility state.
- **C:** Large diff, broad regression surface, harder rollback, and inadequate independent evidence for all existing Experience/artifact/Historical mutation semantics in one sprint.

## Reversibility

- **A:** Revert the startup gate and version declaration changes before any schema-v5 activation; no schema or user data rollback is required because the slice adds no v5 DDL or migration.
- **B:** Fully reversible by a later explicit authorization; no files or data change now.
- **C:** Source revert is possible while schema stays v4, but mutation-command rollout can create subtle behavior drift and demands a larger rollback and parity matrix.

## Data And Privacy Impact

Option A reads only local compatibility metadata needed to classify startup. It must not log or transmit personal content, create a backup, or run tests against the founder's live database. Existing v4 data, ADR-0009 consent/provenance, and provider behavior remain unchanged. Option B has no new data impact. Option C touches all current mutation paths and therefore has materially higher provenance and deletion risk.

## Orchestrator Recommendation

Select Option A. It separates the urgent safety prerequisite from the broader typed-mutation rewrite. After implementation, canonical verification and disposable-database manual checks, stop at Founder diff review; promotion remains a separate authorization.

## Default Safe Action

Option B. Without the exact Founder response below, remain at `human_decision_required` and make no production implementation changes.

## Blocked Files Or Phases

Blocked pending decision: `src-tauri/src/sqlite.rs`, `src-tauri/src/lib.rs`, startup-facing TypeScript/React storage files, application version declarations, tests for the new production boundary, and any factual implementation documentation. Engineering Planning and Implementation for Slice 1A are blocked. Slice 1B, Slices 2-6, Phase 4, schema-v5 activation, Harness expansion, staging, commit, push, merge, PR, and deployment remain blocked under every option except by later separate authority.

## Exact Founder Response Needed

To authorize the recommendation, reply exactly or equivalently:

> I resolve PHASE3C-SLICE1A-001 by selecting Option A. I authorize Phase 3C Slice 1A only: pre-writable database presence/version inspection, refusal of user_version greater than the current supported maximum 4, an explicit local startup/database compatibility state, blocked store construction/cleanup/reads/writes for incompatible databases, preservation of current schema-v4 and existing fresh/v2/v3 compatibility behavior, synchronized application version declarations to 0.2.0 if required by architecture/13, synthetic/disposable fixture tests, factual documentation synchronization, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I do not authorize Slice 1B, schema-v5 production DDL, user_version 5, live user-database migration or test mutation, backup, restore, cleanup, Slices 2-6, Phase 4, provider or ContextPacket changes, Engineering Harness expansion, staging, commit, push, merge, PR, or deployment.

To defer, reply: `I resolve PHASE3C-SLICE1A-001 by selecting Option B and authorize no production implementation.`

Option C requires a new detailed Founder package and will not be implemented from a bare `Option C` response.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE1A-001 by selecting Option A. I authorize Phase 3C Slice 1A only: pre-writable database presence/version inspection, refusal of user_version greater than the current supported maximum 4, an explicit local startup/database compatibility state, blocked store construction/cleanup/reads/writes for incompatible databases, preservation of current schema-v4 and existing fresh/v2/v3 compatibility behavior, synchronized application version declarations to 0.2.0 if required by architecture/13, synthetic/disposable fixture tests, factual documentation synchronization, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I do not authorize Slice 1B, schema-v5 production DDL, user_version 5, live user-database migration or test mutation, backup, restore, cleanup, Slices 2-6, Phase 4, provider or ContextPacket changes, Engineering Harness expansion, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Phase 3C Slice 1A only: pre-writable database presence/version inspection; refuse user_version > 4; explicit local startup/database compatibility state; block store construction, cleanup, reads, and writes when incompatible; preserve schema v4 and existing fresh/v2/v3 compatibility; synchronize application declarations to 0.2.0 if architecture/13 requires; synthetic/disposable fixture tests; factual docs; canonical verification; Theory Alignment Review; archive/reset; stop at Founder diff review. Excludes Slice 1B, schema v5 DDL/user_version 5, live user DB mutation, backup/restore/cleanup, Slices 2-6, Phase 4, provider/ContextPacket, Harness expansion, git promotion, PR, and deployment.

## Decided At And Evidence Reference

- Decided at: 2026-07-18T13:18:19.235Z
- Evidence reference: Founder response in current session for PHASE3C-SLICE1A-001

## Resume Phase

product_review
