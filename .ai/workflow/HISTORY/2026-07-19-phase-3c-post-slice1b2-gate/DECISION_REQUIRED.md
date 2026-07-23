# Decision Required

Status: resolved
- Sprint ID: 2026-07-19-phase-3c-post-slice1b2-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-19T12:47:34.5296865Z
- Updated at: 2026-07-19T12:47:34.5296865Z

## Decision ID

PHASE3C-SLICE2A-001

## Sprint ID

2026-07-19-phase-3c-post-slice1b2-gate

## Decision Summary

Choose whether to authorize the smallest test-only pre-v5 backup safety slice.
The proposed Slice 2A would implement and test verified `VACUUM INTO` backup
creation against disposable schema-v4 fixtures only. It would not create a
backup for a user, restore a database, connect to startup/UI, or migrate to v5.

## Why Automation Stopped

Architecture/13 explicitly states that backup and restore implementation remain
unauthorized. A backup is a sensitive duplicate of local user history, and
successful Slice 1B-2 promotion does not grant authority to create, retain, or
restore one. The Founder must choose the exact boundary before Engineering Plan
or implementation.

## Relevant Constitution Clauses

- Human before AI: engineering convenience cannot silently decide recovery or
  replacement of user history.
- Privacy before Profit: a backup is sensitive user data, even when local.
- Documentation is truth: promoted evidence and Founder-approved design must be
  distinguished from production implementation authority.

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/03_Principles.md`: local-first control and user agency.
- `docs/10_Privacy.md`: bounded local storage, deletion, and disclosure.
- `docs/06_Memory.md`: user history remains user-owned and correctable.

## Relevant ADRs

- ADR-0007: preserve reviewed-artifact provenance and deletion distinctions.
- ADR-0009: preserve Historical Question consent, actual-use provenance, and
  cascade behavior byte-for-byte.
- ADR-0011: Accepted lifecycle design; migration, cleanup, UI activation, and
  production implementation remain separately gated.

## Available Options

### Option A — Slice 2A fixture-only backup creation and verification

Authorize only path-injected Rust logic and disposable tests for `VACUUM INTO`,
content-free manifest construction, database/source-manifest digests,
`user_version = 4`, foreign-key/integrity verification, and fail-closed cleanup
of incomplete test destinations. Do not register a Tauri command or connect a
real app-data/user-database path. Defer restore and every production behavior.

### Option B — Full disposable-fixture Slice 2 backup and restore harness

Authorize Option A plus explicit fixture restore, atomic replacement simulation,
restart/failure projections, and backup retention/deletion decisions. Still no
real user database or UI.

### Option C — Defer Slice 2

Make no backup/restore implementation. Preserve only the factual Slice 1B-2
promotion correction and stop Phase 3C storage work.

## Benefits

- **A:** smallest independently verifiable safety primitive; creates no user
  backup and grants no restore/file-replacement authority.
- **B:** closes more of architecture/13 Slice 2 before migration planning and
  reduces later integration uncertainty.
- **C:** avoids adding dormant migration-support code and preserves the current
  stable schema-v4 product exactly.

## Risks

- **A:** restore remains unproved; later production integration can expose
  locking, disk-space, and permission behavior not covered by fixtures.
- **B:** larger authority surface combines backup, destructive replacement,
  recovery state, and retention concerns in one sprint; higher review burden.
- **C:** Phase 3C lifecycle migration remains blocked and no new recovery-safety
  evidence is produced.

## Reversibility

- **A:** highly reversible before promotion; no production path or user file is
  touched. Removing the unregistered primitive/tests leaves schema v4 intact.
- **B:** code remains fixture-only but models destructive replacement behavior;
  more contracts and test surfaces become dependencies.
- **C:** no implementation change; factual promotion synchronization can be
  reviewed or reverted independently before Git promotion.

## Data And Privacy Impact

Option A and B may use only repository-defined synthetic content in OS temporary
directories. They may not inspect `%APPDATA%`, the Life OS database, credentials,
or provider data. Test teardown must remove temporary backup content. Option C
creates no data copy.

## Orchestrator Recommendation

Select **Option A**. It is the lowest-threat product-engineering step and fits a
capacity-resilient micro-slice. Incomplete destinations should be removed
best-effort and never retained for diagnostics. Require a separate Founder gate
for restore (Slice 2B), production path/UI integration, cleanup, and migration.

## Default Safe Action

If the Founder does not answer exactly, remain at `human_decision_required` and
do not create backup code, Engineering Plan, tests, commands, UI, or migration.

## Blocked Files Or Phases

Blocked pending this decision:

- backup implementation in `src-tauri/src/`;
- new backup/manifest Rust tests or fixtures;
- Engineering Plan and implementation phase;
- restore, startup, renderer, UI, cleanup, schema v5, Slices 2B-6, and Phase 4.

## Exact Founder Response Needed

For the recommended option, reply exactly:

`I resolve PHASE3C-SLICE2A-001 by selecting Option A. I authorize Phase 3C Slice 2A only: path-injected Rust backup creation and verification against synthetic/disposable schema-v4 fixtures; SQLite VACUUM INTO with destination-nonexistence enforcement; content-free manifest construction; closed-backup SHA-256, source-manifest, schema-version, foreign-key, and integrity verification; fail-closed malformed/corrupt/version-mismatch/destination-conflict/injected-failure tests; best-effort deletion of incomplete test destinations; proof that source fixtures remain unchanged; factual documentation; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. The primitive must not be registered as a Tauri command or connected to production startup, renderer, UI, app-data paths, or a real user database. I do not authorize restore, file replacement, retention cleanup, delete-now, 30-day scheduling, schema v5, user_version 5, migration, backup of real user data, Slice 2B, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.`

Option B or C must be selected explicitly with equally exact authorized scope.
Silence, “continue,” or prior architecture approval does not resolve this gate.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE2A-001 by selecting Option A. I authorize Phase 3C Slice 2A only: path-injected Rust backup creation and verification against synthetic/disposable schema-v4 fixtures; SQLite VACUUM INTO with destination-nonexistence enforcement; content-free manifest construction; closed-backup SHA-256, source-manifest, schema-version, foreign-key, and integrity verification; fail-closed malformed/corrupt/version-mismatch/destination-conflict/injected-failure tests; best-effort deletion of incomplete test destinations; proof that source fixtures remain unchanged; factual documentation; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. The primitive must not be registered as a Tauri command or connected to production startup, renderer, UI, app-data paths, or a real user database. I do not authorize restore, file replacement, retention cleanup, delete-now, 30-day scheduling, schema v5, user_version 5, migration, backup of real user data, Slice 2B, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Slice 2A only: path-injected Rust VACUUM INTO backup creation and verification against disposable schema-v4 fixtures, content-free manifest, digest/source-manifest/schema/FK/integrity checks, fail-closed cleanup and synthetic regressions, factual docs, verification, theory review, archive/reset, and stop at Founder diff review; all production, restore, cleanup, migration, later-slice, provider, Harness, Git promotion, PR, and deployment scopes remain excluded.

## Decided At And Evidence Reference

- Decided at: 2026-07-19T13:26:16.984Z
- Evidence reference: .ai/workflow/DECISION_REQUIRED.md#PHASE3C-SLICE2A-001

## Resume Phase

engineering_planning
