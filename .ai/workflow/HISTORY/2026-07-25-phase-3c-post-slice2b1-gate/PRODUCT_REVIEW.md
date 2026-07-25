# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-25-phase-3c-post-slice2b1-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: a476c38ba5c4a9b19a81fbb14973aacc4adf25bd
- Working-tree digest reviewed: c75187b937c4004a1326265784139b8ea0e9bbce4213c3d7c08aa27563db8087
- Created at: 2026-07-25T12:05:00+09:00
- Updated at: 2026-07-25T12:05:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Truthfully close the already-promoted Slice 2B-1 evidence, then stop before
creating any production filesystem primitive until the Founder chooses between
a bounded disposable-path primitive, full production integration, or shifting
to the separate structured-retrieval Phase 3 gap.

## Problem Statement

Slice 2B-1 proved only a test-local logical replacement simulation. It did not
prove operating-system replacement semantics, crash durability, SQLite
WAL/SHM/journal handling, exclusive database quiescence, restart-state
classification, or production path ownership. The next useful storage step
must add real reusable filesystem-safety code without being mistaken for an
activated backup/restore feature.

## User Value

A bounded Slice 2B-2 would reduce future migration data-loss risk by making
filesystem assumptions explicit and testable before Life OS can ever touch a
real user's database. Keeping it disconnected from app-data and runtime
activation preserves local-first control while materially advancing the
production storage foundation.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: the user remains final authority; local data and
  user agency cannot be overridden by silent repair or replacement.
- `docs/06_Memory.md`: stored history remains user-owned information rather
  than system-owned memory.
- `docs/10_Privacy.md`: local storage still requires minimization, legible
  retention, deletion control, and careful handling of sensitive duplicates.
- `docs/11_MVP.md` and `docs/12_Roadmap.md`: this is deterministic storage
  safety, not a new interpretation or Phase 4 capability.

## Relevant ADRs

- `ADR-0007`: reviewed artifacts and their provenance must survive exactly;
  copying or restoring does not change authorship or review state.
- `ADR-0009`: consent, packet, transmission, dependencies, deletion, and
  actual-use provenance must remain exact and must never be reconstructed.
- `ADR-0010`: Phase 4 remains a separately gated user-owned hypothesis
  capability and is not authorized here.
- `ADR-0011`: append-only lifecycle and portable provenance are Accepted, but
  migration, destructive recovery, and production activation remain separately
  gated.

## Current Implementation Context

- **Founder-approved:** architecture/12 and architecture/13; ADR-0011 is
  Accepted.
- **Implemented, verified, and promoted:** Slice 0, Slice 1A, Slice 1B-1,
  Slice 1B-2, Slice 2A, and Slice 2B-1.
- **Slice 2B-1 promotion evidence:** feature commit
  `5b9d4613c6fa4bb86c8fdfd9009a8a7bbed710bd`, non-fast-forward merge
  `a476c38ba5c4a9b19a81fbb14973aacc4adf25bd`, clean promoted `develop`,
  production schema and `user_version` still 4, and passing canonical local
  verification. Remote CI was not separately observed.
- **Implemented evidence limitation:** the private Rust integration test
  simulates logical replacement in OS-temporary fixtures and rolls fixture
  bytes back after injected failures. It is not production filesystem code and
  proves no real-user recovery guarantee.
- **Proposed only:** a path-injected production-grade Rust filesystem safety
  primitive exercised only against synthetic disposable app-like directories.
- **Unauthorized:** production backup/restore activation, app-data/startup/UI,
  real user data, retention, schema-v5 activation, and later slices.

## In Scope

1. Correct factual Slice 2B-1 promotion drift in architecture/13.
2. Compare three next-step alternatives.
3. Propose Slice 2B-2 as reusable production-grade Rust code that remains
   unregistered and disconnected from product runtime:
   - require an explicit caller-held exclusive-operation/quiescence guard;
     active or unknown connection/write state fails closed;
   - detect `-wal`, `-shm`, and rollback-journal sidecars and refuse without
     deleting or checkpointing them;
   - constrain live, backup, staging, and state files to one canonical owned
     disposable root; refuse aliases, traversal, symlinks/reparse points,
     collisions, cross-volume replacement, and pre-existing staging;
   - create collision-resistant owned files with create-new semantics;
   - revalidate exact backup digest, source manifest, schema version, foreign
     keys, integrity, and expected records after all database handles close and
     immediately before replacement;
   - sync staged file content and report parent-directory durability support;
   - use a platform-aware injected replacement abstraction with explicit
     `committed`, `failed_unchanged`, and `outcome_unknown` results;
   - classify restart states without automatic replay, rollback, repair, or
     candidate selection;
   - clean only paths carrying the exact current operation ownership;
   - exercise all behavior only in synthetic/disposable app-like directories.
4. Stop at Founder decision; no implementation or feature branch exists before
   an exact resolution.

## Out Of Scope

Tauri command registration, renderer/UI/startup/app-data integration, a real
user database, real-user backup/restore, production feature activation,
automatic repair/replay/rollback, WAL checkpointing or sidecar deletion,
retention/delete-now/scheduling, migration disclosure UI, schema-v5 DDL,
`user_version = 5`, Slices 3-6, Phase 4, provider/ContextPacket changes,
Harness expansion, Stage 2/3, staging, commit, push, merge, PR, and deployment.

## Product Constraints

- A quiescence guard is an explicit contract, not proof inferred from the
  absence of sidecar files. Unknown connection state fails closed.
- A replacement adapter may claim atomicity only for a specifically supported
  platform operation and same-volume path pair; generic rename and test
  rollback are not equivalent.
- `outcome_unknown` is a durable recovery-required state. The system must not
  automatically retry, choose a candidate, or delete evidence.
- File sync and parent-directory sync support/failure must be reported
  separately; unsupported durability cannot be presented as success.
- Schema and `user_version` remain exactly 4.

## Evidence And Provenance Constraints

The expected closed-backup SHA-256, governed source-manifest digest, schema
version, foreign-key result, integrity result, and exact expected synthetic
records are mandatory inputs. They are recomputed immediately before the
replacement boundary and after a reported commit. A mismatch creates no repair,
revision, review event, consent, transmission, dependency, or provenance.

## Historical Context Constraints

Historical rows may exist only as synthetic fixture content whose bytes and
relationships are preserved. No retrieval, Context Packet, provider transport,
Cross-Experience Reflection, recurrence, contradiction, change-over-time,
summary, or Pattern inference is performed.

## Consent Constraints

No historical-use consent is created, consumed, or reused. Existing ADR-0009
rows are opaque synthetic records. A future real-user restore action would need
separately approved disclosure and explicit user action; filesystem primitive
tests cannot grant or simulate that authorization.

## AI-Role Constraints

No model call or AI interpretation occurs. Deterministic filesystem evidence
cannot authorize production use, decide which database represents the user's
truth, or turn an ambiguous restart state into an automated recovery decision.

## Privacy Constraints

Only synthetic data is eligible. Test roots must be OS-temporary disposable
directories. Manifests and restart-state metadata remain content-free. No path,
digest, error, or log may expose Experience text, artifact content, packet
content, credentials, or raw provider errors. Backup copies remain sensitive
duplicates even when metadata is content-free.

## User-Agency Constraints

Slice 2B-2 has no user-facing action and cannot silently replace, repair,
downgrade, migrate, delete, or select any real database. Ambiguity must remain
visible as `recovery_required` for a later explicitly designed and authorized
human-controlled flow.

## Acceptance Criteria

If Option A is later authorized:

1. Production-quality Rust primitive code is path-injected, unregistered, and
   exercised only against synthetic disposable app-like directories.
2. A caller-held quiescence guard is mandatory; active or unknown database
   activity refuses before any write.
3. WAL, SHM, and rollback-journal sidecars refuse before staging or replacement
   and are never deleted, truncated, moved, or checkpointed.
4. All paths must be distinct, same-volume where required, canonical, under one
   exact owned root, non-symlink/non-reparse, and collision-free.
5. Backup, staging, and operation-state files use collision-resistant
   create-new ownership.
6. Closed backup and staging candidates pass exact digest, manifest, schema,
   foreign-key, integrity, and expected-record validation.
7. Staged bytes are flushed; platform-supported file and parent-directory
   durability is reported honestly and required where the adapter contract
   promises durable commit.
8. The injected replacement abstraction distinguishes committed,
   failed-unchanged, and unknown outcomes; no generic simulation is labelled
   production atomicity.
9. Restart inspection classifies untouched, staged, committed, cleanup-needed,
   and ambiguous states without autonomous retry, replay, rollback, repair, or
   candidate selection.
10. Every known pre-commit failure leaves the disposable live file
    byte-identical; every ambiguous/post-commit verification failure preserves
    all recovery evidence and returns `recovery_required`.
11. Cleanup removes only exact current-operation owned temporary files and
    never live, backup, sidecar, manifest, state evidence, or unrelated files.
12. Failure tests cover activity/sidecar refusal, path alias/traversal/symlink,
    collision, cross-volume/unsupported replacement, permission/locking, sync,
    interruption before/after replace, unknown outcome, post-replace
    verification, restart classification, and cleanup failure.
13. Production schema, Tauri registration, renderer/UI/startup/app-data,
    retention, provider/ContextPacket, and Phase 4 remain unchanged.
14. Canonical verification and Theory Alignment Review pass before Founder
    diff review.

## Risks

- SQLite process quiescence cannot be proven from filesystem sidecars alone;
  the future activated caller needs a real exclusive-operation mechanism.
- Windows, macOS, and Linux differ in replacement, locking, symlink/reparse,
  directory-sync, antivirus, and power-loss behavior. Unsupported guarantees
  must fail closed or remain explicitly unproved.
- TOCTOU exists between validation and replacement; paths and candidate bytes
  must be held/validated as close to the platform call as possible.
- A reported successful replacement followed by failed directory sync or
  failed post-open validation is ambiguous and must not trigger silent rollback.
- Restart metadata can itself be stale or damaged; unknown state preserves
  candidates and blocks automatic action.
- Backup copies duplicate sensitive information; production location,
  visibility, expiry, and deletion remain later user-facing gates.
- Option A adds production-capable code before product activation. Its value
  depends on strict disconnection and later platform/manual evidence.
- Option B couples too many unresolved high-risk surfaces at once.
- Option C advances a different Phase 3 blocker but leaves the migration
  recovery foundation stalled.

## Open Questions

Resolved by the Founder through `PHASE3C-SLICE2B2-001`, Option A. Engineering
must remain inside the exact disposable-path, unregistered, no-real-data
boundary recorded in `DECISION_REQUIRED.md`.

## Human Decision Required

false. `PHASE3C-SLICE2B2-001` is resolved as Option A and recorded in the
repository workflow.

## Recommendation

Proceed with the Founder-authorized Option A only. The Engineering Plan must
copy every acceptance criterion and exclusion, preserve the unregistered and
disposable-path boundaries, and stop at Founder diff review after verification
and archive/reset.

## Review Status

approved_with_conditions.
