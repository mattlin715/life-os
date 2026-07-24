# Decision Required

Status: resolved
- Sprint ID: 2026-07-24-phase-3c-post-slice2a-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-24T00:55:00+09:00
- Updated at: 2026-07-24T00:55:00+09:00

## Decision ID

PHASE3C-SLICE2B1-001

## Sprint ID

2026-07-24-phase-3c-post-slice2a-gate

## Decision Summary

Choose whether the next Phase 3C increment is a fixture-only verified
restore/atomic-replacement simulation, a broader production backup/restore
integration, or no restore work yet.

## Why Automation Stopped

Architecture/13 still withholds Slice 2B, restoration, file replacement,
production backup, real-user-data work, and schema-v5 activation. Successful
Slice 2A tests and promotion do not grant that authority.

## Relevant Constitution Clauses

- Preserve local-first user control and the user's ability to correct and
  delete their own information.
- Do not silently perform destructive actions or make system capability the
  authority over the user.
- Preserve **We Build Mirrors, Not Oracles.**

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/10_Privacy.md`: local storage, minimization, legible retention, and
  deletion.
- `docs/03_Principles.md`: user agency and evidence before conclusion.
- `docs/06_Memory.md`: stored history remains user-controlled data, not
  system-owned memory.

## Relevant ADRs

- ADR-0007: exact reviewed-artifact provenance.
- ADR-0009: exact historical consent, transmission, packet, dependency,
  deletion, and actual-use chain.
- ADR-0011: accepted lifecycle direction with migration and destructive
  recovery separately gated.
- ADR-0010: Phase 4 remains outside this deterministic storage sprint.

## Available Options

### Option A — Phase 3C Slice 2B-1 fixture-only restore simulation

Authorize only integration-test-local Rust behavior over synthetic/disposable
schema-v4 fixtures. It may restore from a previously verified backup, require
exact digest and governed source-manifest verification, stage to an exact owned
temporary path, exercise a test-injected atomic replacement boundary, verify
schema/FK/integrity/exact records, inject bounded failures, prove failed restore
leaves the live fixture unchanged, clean only owned temporary files, synchronize
factual documentation, run canonical verification and Theory Alignment Review,
archive/reset, and stop at Founder diff review.

The replacement seam is explicitly a simulation. It must not claim to prove
production operating-system atomicity.

### Option B — Broader production backup/restore integration

Authorize production app-data paths, real backup/restore commands, user-facing
disclosure, real file replacement, restart behavior, permissions, retention,
and recovery integration.

### Option C — Defer restore

Make only the factual Slice 2A closeout. Do not implement restoration or
replacement behavior; schema-v5 migration remains blocked.

## Benefits

- **A:** tests the next dangerous ordering and rollback boundary without
  touching real data or creating a product promise.
- **B:** advances toward a usable recovery feature, if all unresolved product
  and platform policies were designed and approved.
- **C:** has the smallest immediate change and no new destructive primitive.

## Risks

- **A:** cannot prove production filesystem atomicity, crash durability,
  process locking, SQLite sidecar handling, or real permissions. A careless
  test cleanup path could still be destructive unless exact ownership is
  enforced.
- **B:** introduces immediate data-loss and privacy risk across backup
  location, duplicated personal data, retention, disclosure, replacement,
  restart, and multi-platform behavior. Current authority and evidence are
  insufficient.
- **C:** yields no new restoration evidence and leaves schema-v5 cutover
  blocked.

## Reversibility

- **A:** fully reversible repository test/document change; all files are
  disposable. Failures must leave the live fixture byte-identical and cleanup
  only the owned staging path.
- **B:** not safely reversible without a complete production backup,
  replacement, restart, and recovery contract. It may affect real data.
- **C:** no implementation to revert; the gate may be reopened later.

## Data And Privacy Impact

Option A uses synthetic data only. Its manifest remains content-free, while the
disposable backup itself contains only fixture content. No network, provider,
real app-data path, or user database is touched. Option B would duplicate and
replace real personal data and therefore requires a separate, substantially
broader privacy and user-control decision. Option C has no data impact.

## Orchestrator Recommendation

Select **Option A**. It provides useful failure evidence while preserving the
distinction between a fixture proof and production restore authority.

## Default Safe Action

Option C. Until an exact response is recorded, implement no restore or
replacement behavior.

## Blocked Files Or Phases

Blocked: changes to `src-tauri/tests/schema_v5_backup.rs` or any new restore
test module; any implementation or Engineering Plan for Slice 2B-1; all
production Rust/TypeScript/UI files; schema-v5 activation; Slices 3-6; Phase 4;
staging, commit, push, merge, PR, and deployment.

## Exact Founder Response Needed

To choose the recommendation, respond exactly:

`I resolve PHASE3C-SLICE2B1-001 by selecting Option A. I authorize Phase 3C Slice 2B-1 only: integration-test-local verified restore and test-injected atomic replacement simulation against synthetic/disposable schema-v4 fixtures; restoration only from a previously verified backup with exact expected database digest and governed source-manifest digest; owned temporary staging; immediate pre-replacement digest, source-manifest, schema-version, foreign-key, integrity, and exact-record revalidation; fail-closed digest mismatch, malformed/corrupt backup, schema mismatch, manifest mismatch, expected-record mismatch, destination conflict, interruption, permission, and injected replacement failures; proof that every failed restore leaves the disposable live fixture byte-identical; cleanup of exact owned temporary fixture files only; factual documentation synchronization; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. The replacement seam is simulation evidence only and must not claim production operating-system atomicity. I do not authorize production backup paths, backup or restore of real user data, production restore or file replacement, Tauri command registration, renderer/UI/startup/app-data integration, SQLite sidecar recovery, retention cleanup, delete-now, scheduling, schema v5, user_version 5, migration, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.`

To choose another option, name Option B or Option C and state the exact
authorized and excluded scope. Silence never resolves this decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE2B1-001 by selecting Option A. I authorize Phase 3C Slice 2B-1 only: integration-test-local verified restore and test-injected atomic replacement simulation against synthetic/disposable schema-v4 fixtures; restoration only from a previously verified backup with exact expected database digest and governed source-manifest digest; owned temporary staging; immediate pre-replacement digest, source-manifest, schema-version, foreign-key, integrity, and exact-record revalidation; fail-closed digest mismatch, malformed/corrupt backup, schema mismatch, manifest mismatch, expected-record mismatch, destination conflict, interruption, permission, and injected replacement failures; proof that every failed restore leaves the disposable live fixture byte-identical; cleanup of exact owned temporary fixture files only; factual documentation synchronization; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. The replacement seam is simulation evidence only and must not claim production operating-system atomicity. I do not authorize production backup paths, backup or restore of real user data, production restore or file replacement, Tauri command registration, renderer/UI/startup/app-data integration, SQLite sidecar recovery, retention cleanup, delete-now, scheduling, schema v5, user_version 5, migration, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Slice 2B-1 only: integration-test-local verified restore and test-injected atomic replacement simulation over synthetic/disposable schema-v4 fixtures, with exact digest/source-manifest/schema/FK/integrity/record revalidation, owned staging and cleanup, bounded fail-closed tests, factual docs, verification, theory review, archive/reset, and stop at Founder diff review; all production, real-data, schema-v5, later-slice, Phase 4, Harness-expansion, Git promotion, PR, and deployment scopes remain excluded.

## Decided At And Evidence Reference

- Decided at: 2026-07-23T16:22:48.893Z
- Evidence reference: .ai/workflow/DECISION_REQUIRED.md#PHASE3C-SLICE2B1-001

## Resume Phase

engineering_planning
