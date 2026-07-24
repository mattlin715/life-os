# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-24-phase-3c-post-slice2a-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 9e0ff74f3fcdbe64d060e40d800078d90d4d9dc6
- Working-tree digest reviewed: 03993f812b030fb68c6dc9959e5a2c23618421c7f2a35271f3a5039e2383ff38
- Created at: 2026-07-24T00:50:00+09:00
- Updated at: 2026-07-24T00:50:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Truthfully close already-promoted Slice 2A evidence, then stop before any new
restore or replacement behavior until the Founder chooses among a fixture-only
proof, broader production integration, or continued deferral.

## Problem Statement

Architecture/13 still described Slice 2A as a current working-tree change,
miscounted seven evidence levels as five, and omitted its promotion commits.
After that factual correction, the remaining product question is whether to
authorize the lowest-threat useful restoration proof without laundering
test-local backup evidence into production backup/restore authority.

## User Value

A bounded restore simulation can expose data-loss and rollback defects before
any real database path is touched. Keeping it fixture-only preserves local-first
control and avoids presenting a test harness as a safe user recovery feature.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: local data, explicit user control, humility, and
  preservation of user agency remain controlling.
- `docs/10_Privacy.md`: local storage does not remove the obligation to minimize
  duplicated personal data and make deletion and retention behavior legible.
- `docs/11_MVP.md` and `docs/12_Roadmap.md`: this work is a storage-safety
  foundation; it does not introduce a new user-facing product or Phase 4
  interpretation capability.

## Relevant ADRs

- `ADR-0007`: reviewed artifact provenance must survive exact storage copies
  without being elevated or re-authored.
- `ADR-0009`: consent, transmission, packet, dependency, deletion, and
  actual-use provenance rows must remain exact and must not be reconstructed.
- `ADR-0010`: Phase 4 remains a separately gated, user-owned hypothesis
  capability and is not implicated by deterministic file restoration.
- `ADR-0011`: the append-only lifecycle and portable provenance direction is
  Accepted, but schema-v5 migration and destructive recovery still require
  separate authority.

## Current Implementation Context

- **Founder-approved:** architecture/12 and architecture/13; ADR-0011 is
  Accepted.
- **Implemented, verified, and promoted:** Slice 0; Slice 1A; Slice 1B-1;
  Slice 1B-2; and Slice 2A's private integration-test-local schema-v4 backup
  creation/verification harness.
- **Slice 2A promotion evidence:** feature commit
  `a5ba00650f396470b3a7ac265701d8ce3d90d35e`, non-fast-forward merge
  `9e0ff74f3fcdbe64d060e40d800078d90d4d9dc6`, clean `develop`, schema and
  `user_version` still 4, and passing canonical local verification. Remote CI
  was not separately observed.
- **Proposed only:** fixture-only verified restore and atomic replacement
  simulation.
- **Unauthorized:** production backup/restore, real file replacement,
  app-data integration, retention cleanup, schema-v5 activation, and later
  slices.

## In Scope

1. Correct factual Slice 2A promotion drift in architecture/13.
2. Compare three next-step alternatives.
3. Propose Slice 2B-1 as synthetic/disposable test-only behavior:
   - accept only a previously verified schema-v4 backup plus exact expected
     database digest and governed source-manifest digest;
   - revalidate digest, schema, manifest, foreign keys, integrity, and exact
     expected fixture records;
   - stage the candidate only at an exact owned temporary path;
   - exercise a test-injected atomic replacement boundary against a disposable
     live fixture;
   - fail closed before replacement and prove the live fixture remains
     byte-identical for digest, corruption, destination, interruption,
     permission, and injected failures;
   - clean up only exact owned temporary fixture files.
4. Stop at Founder decision; implementation requires an exact resolution.

## Out Of Scope

Production backup paths, real user data, Tauri commands, renderer/UI/startup,
app-data directories, production restore or operating-system replacement
guarantees, SQLite sidecar recovery, backup retention, delete-now, scheduling,
schema-v5 DDL, `user_version = 5`, live migration, Slices 3-6, Phase 4,
provider/ContextPacket changes, Harness expansion, Stage 2/3, Git promotion,
PR, and deployment.

## Product Constraints

- A passing simulation must be labeled fixture evidence, not production
  recoverability evidence.
- The replacement seam must not claim to prove platform-level atomicity.
- Every database handle must be closed before digesting, staging, or invoking
  the replacement seam.
- No successful restore may weaken current schema-v4 provenance, deletion, or
  ADR-0009 relationships.
- Schema version remains exactly 4.

## Evidence And Provenance Constraints

The verified backup's SHA-256 and governed source-manifest digest are inputs,
not suggestions. Both are recomputed immediately before the replacement
boundary. Expected fixture rows must match exactly. A mismatch produces no
replacement and no invented repair, revision, review event, consent, or
provenance.

## Historical Context Constraints

Historical rows may exist only as synthetic fixture content whose exact storage
preservation is checked. No historical retrieval, packet assembly, provider
transport, Cross-Experience Reflection, recurrence analysis, summary, or
Pattern inference is performed.

## Consent Constraints

No user consent or provider-use consent is created or reused. Existing
ADR-0009 consent rows are opaque fixture records to preserve. A future
production restore action would need separately approved disclosure and user
control; this sprint cannot define or imply that production consent.

## AI-Role Constraints

No model call or AI interpretation occurs. The Harness may verify deterministic
storage behavior but may not authorize its own production use or infer that a
successful fixture test makes a real user database safe.

## Privacy Constraints

Only synthetic data and OS-temporary paths are eligible. The content-free
manifest may contain identifiers, digests, versions, and timestamps but no
Experience text, artifact payload, packet content, credentials, or provider
errors. Exact temporary-path ownership limits cleanup.

## User-Agency Constraints

There is no user-facing restore in Slice 2B-1. The simulation must not silently
replace, repair, downgrade, or migrate any real database. Production recovery
would require a later Founder gate and explicit user action/disclosure.

## Acceptance Criteria

If Option A is later authorized:

1. Changes stay in integration-test-local code and factual documentation.
2. Only disposable schema-v4 fixtures are read or replaced.
3. The backup digest, source-manifest, schema version, foreign keys, integrity,
   and exact expected records are revalidated before replacement.
4. A unique owned staging path is required and destination conflicts fail
   closed.
5. Success replaces only the disposable live fixture and preserves exact
   expected records and provenance relationships.
6. Digest mismatch, malformed/corrupt backup, schema mismatch, manifest
   mismatch, expected-record mismatch, destination conflict, injected
   interruption, injected permission failure, and replacement failure all
   produce no successful restore.
7. Every pre-replacement or simulated replacement failure leaves the live
   fixture byte-identical.
8. Cleanup can remove only the exact owned staging file and never the backup or
   live fixture.
9. Production source, Tauri registration, UI, app-data paths, schema version,
   retention, providers, ContextPacket, and Phase 4 remain unchanged.
10. Canonical verification and Theory Alignment Review pass before Founder
    diff review.

## Risks

- A simulated replacement seam cannot prove Windows/macOS/Linux filesystem
  atomicity, crash durability, antivirus/locking behavior, or SQLite WAL/SHM
  handling.
- Digest verification can become stale unless repeated immediately before the
  replacement boundary.
- Over-broad cleanup could delete a live or backup file; exact owned-path checks
  are mandatory.
- Production backups contain private data even when manifests are content-free;
  this option intentionally avoids real data and retention policy.
- Broader production integration now would couple unresolved disclosure,
  location, permissions, retention, restart, and recovery semantics and create
  materially higher data-loss risk.

## Open Questions

Resolved by the Founder on 2026/07/24 through
`PHASE3C-SLICE2B1-001`, Option A. Implementation must remain inside the exact
fixture-only boundary recorded in `DECISION_REQUIRED.md`.

## Human Decision Required

false. `PHASE3C-SLICE2B1-001` is resolved as Option A and recorded in the
repository workflow.

## Recommendation

Proceed with the Founder-authorized Option A only. The Engineering Plan must
copy every acceptance criterion and exclusion above, treat the replacement seam
as simulation evidence rather than production atomicity proof, and stop at
Founder diff review after verification and archive/reset.

## Review Status

approved_with_conditions.
