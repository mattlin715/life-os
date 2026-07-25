# Decision Required

Status: resolved
- Sprint ID: 2026-07-25-phase-3c-post-slice2b1-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-25T12:10:00+09:00
- Updated at: 2026-07-25T12:10:00+09:00

## Decision ID

PHASE3C-SLICE2B2-001

## Sprint ID

2026-07-25-phase-3c-post-slice2b1-gate

## Decision Summary

Choose whether the next Phase 3C increment creates a path-injected,
production-grade filesystem safety primitive exercised only in disposable
app-like directories, jumps to full production backup/restore integration, or
defers this storage path and begins the separate explicit structured-retrieval
Phase 3 gap.

## Why Automation Stopped

Architecture/13 authorized and promoted only fixture-local logical restore
simulation. It still withholds production filesystem code, app-data paths,
production backup/restore, real-user-data work, SQLite sidecar recovery,
schema-v5 activation, and later slices. Successful tests and promotion do not
grant the next authority.

## Relevant Constitution Clauses

- Preserve local-first user control and the user's authority over stored data.
- Do not silently repair, replace, delete, downgrade, or reinterpret user data.
- Preserve provenance, correction, deletion, uncertainty, and user agency.
- Preserve **We Build Mirrors, Not Oracles.**

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/06_Memory.md`: historical records remain user-owned information.
- `docs/10_Privacy.md`: local sensitive duplicates still require minimization,
  disclosure, retention limits, and deletion control.
- `docs/03_Principles.md`: Evidence before Conclusion and explicit user agency.
- `docs/12_Roadmap.md`: every Founder-approved Phase 3 exit gap remains
  blocking before Phase 4 production implementation.

## Relevant ADRs

- ADR-0007: preserve reviewed-artifact authorship, lifecycle, and provenance.
- ADR-0009: preserve exact packet/consent/transmission/dependency/deletion and
  actual-use provenance; no reconstruction.
- ADR-0010: Phase 4 is separately gated and remains unauthorized.
- ADR-0011: lifecycle and portable-provenance direction is Accepted, while
  migration and destructive recovery need explicit implementation authority.

## Observed Repository Evidence

1. `develop` and `origin/develop` are at non-fast-forward Slice 2B-1 merge
   `a476c38ba5c4a9b19a81fbb14973aacc4adf25bd`; feature commit
   `5b9d4613c6fa4bb86c8fdfd9009a8a7bbed710bd` is present.
2. Production `SCHEMA_VERSION` and SQLite `user_version` remain 4.
3. Slice 2B-1 lives only in `src-tauri/tests/schema_v5_backup.rs`.
4. Its replacement seam copies bytes into a disposable live fixture and
   restores fixture bytes on injected failure. It explicitly does not implement
   production operating-system atomic replacement.
5. It has no Tauri command, renderer/UI/startup/app-data path, real user
   database, retention behavior, or schema-v5 activation.
6. The promoted canonical baseline passed 17 workflow tests, 163 Vitest tests,
   27 Rust library tests, 12 backup/restore tests, 8 schema-contract tests,
   typecheck, frontend build, Rust check, and repository hygiene checks.
7. All five Phase 3 exit gaps remain blocking. This migration/storage plan
   supports four; explicit emotion, relationship, value-conflict, and
   time-range retrieval is the separate fifth gap.

## Available Options

### Option A — Slice 2B-2 disposable-path production filesystem primitive

Authorize production-quality, path-injected Rust filesystem-safety code, but
exercise it only against synthetic disposable app-like directories. It must:

- require an explicit caller-held quiescence guard and refuse active or unknown
  database activity;
- detect and fail closed on WAL, SHM, and rollback-journal sidecars without
  deleting or checkpointing them;
- validate one canonical owned root, distinct paths, no traversal, no
  symlink/reparse alias, same-volume replacement, and create-new collisions;
- use collision-resistant operation ownership for backup/staging/state files;
- require closed-file digest, source-manifest, schema, foreign-key, integrity,
  and exact-record validation immediately before replacement;
- flush staged data and report platform support/failure for parent-directory
  durability;
- inject a platform-aware replacement adapter whose result is exactly
  `committed`, `failed_unchanged`, or `outcome_unknown`;
- classify restart states without automatic replay, rollback, repair, sidecar
  deletion, or candidate selection;
- clean only exact files owned by the current operation;
- add deterministic synthetic/disposable tests, factual documentation,
  canonical verification, Theory Alignment Review, archive/reset, and stop at
  Founder diff review.

The production Rust module remains unregistered and unreachable from Tauri,
renderer, UI, startup, app-data, or a real database.

### Option B — Full production backup/restore and startup integration

Authorize real app-data paths, application quiescence, production
backup/restore commands, platform replacement, restart recovery, disclosure,
retention/delete-now, UI, and real user-database behavior.

### Option C — Defer migration filesystem work and begin structured retrieval

Make only the factual Slice 2B-1 closeout. Start a separate Founder-gated design
and implementation path for explicit emotion, relationship, value-conflict,
and user-selected time-range retrieval. Do not add filesystem code.

## Benefits

### Option A

- Materially advances from a test simulation to reusable production-grade
  filesystem contracts and platform boundaries.
- Exposes sidecar, quiescence, ownership, durability, and ambiguous-restart
  risks before real data is eligible.
- Preserves a narrow rollback boundary because no product runtime can invoke
  the primitive.
- Reduces uncertainty in the storage foundation supporting four Phase 3 gaps,
  although it closes none by itself.

### Option B

- Fastest path toward an actual migration safety flow and direct user-visible
  recovery capability.
- Could combine platform evidence and product integration in one increment.

### Option C

- Advances the one Phase 3 exit gap not addressed by architecture/13.
- Produces more direct reflective-product value than infrastructure work.
- Avoids adding production-capable filesystem code before activation design.

## Risks

### Shared filesystem and SQLite threat model

- open SQLite connections, queued writes, or a second process invalidate
  replacement assumptions;
- WAL/SHM/journal sidecars can contain authoritative or in-flight state;
- filesystem absence of sidecars does not prove database quiescence;
- TOCTOU can change paths or bytes after validation;
- aliases, traversal, symlinks, junctions, and reparse points can escape an
  intended root;
- cross-volume rename/copy is not atomic;
- Windows/macOS/Linux differ in replace, locking, antivirus interference,
  file/directory sync, and power-loss behavior;
- partial writes or interruption can leave staging or live identity ambiguous;
- successful replace followed by parent-sync or post-open validation failure
  cannot safely be treated as unchanged;
- backup digest, manifest, schema, or expected records may identify the wrong
  database;
- automatic rollback can overwrite legitimate newer state;
- cleanup can destroy live, backup, sidecar, or unrelated evidence if ownership
  is inferred rather than exact;
- backup files duplicate sensitive personal data.

### Option A

- Production-capable code could later be overgeneralized into activation
  authority; code and docs must state the disconnection.
- A caller-supplied guard is a contract, not a complete production quiescence
  implementation.
- CI on one host cannot prove cross-platform durability.
- Ambiguous outcomes will intentionally stop rather than provide automatic
  recovery.

### Option B

- Couples real data, platform semantics, startup, UI, privacy, retention, and
  destructive recovery before the primitive boundary is independently tested.
- A defect can cause irreversible data loss or expose sensitive backup copies.
- Requires multilingual manual verification and platform-specific evidence.
- Highest scope, rollback, and psychological-safety risk.

### Option C

- Leaves migration backup/restore safety blocked.
- Retrieval taxonomy and explanations can drift into semantic or identity
  inference if not separately governed.
- Storage gaps 1, 2, 3, and 5 remain blocked while attention moves elsewhere.

## Failure And Restart State Matrix

| Point or observed state | Required result |
| --- | --- |
| No quiescence guard, active connection/write, or unknown activity | Refuse before staging; live, backup, sidecars, and state unchanged. |
| WAL, SHM, or rollback journal exists | Refuse; do not checkpoint, delete, move, truncate, or infer safety. |
| Path alias, escape, symlink/reparse, pre-existing destination, or cross-volume pair | Refuse before content write. |
| Backup/digest/manifest/schema/FK/integrity/record mismatch | Refuse before staging or replacement; preserve evidence. |
| Staging create/write/flush/validation failure | Live remains byte-identical; remove only exact owned incomplete staging when outcome is known. |
| Parent-directory durability unsupported before replacement | Refuse any adapter contract that requires durable commit; report unsupported capability. |
| Crash before replacement | Live remains original; restart identifies owned staging and never auto-replays. |
| Adapter reports `failed_unchanged` | Verify live identity where possible; preserve backup/state; exact owned staging cleanup may be offered by the caller. |
| Adapter reports `outcome_unknown` or process stops during replacement | Return `recovery_required`; preserve live candidate, backup, staging, and state evidence; no retry or rollback. |
| Adapter reports committed but directory sync fails | `recovery_required`; do not claim durable success or auto-restore. |
| Post-replacement digest/manifest/schema/FK/integrity/record check fails | `recovery_required`; preserve all candidates and do not choose a truth automatically. |
| Replacement and durability verification succeed | Mark completed; cleanup only exact owned temporary files. |
| Cleanup fails after verified success | Mark completed-with-cleanup-required; never delete live, backup, manifest, sidecars, or unrelated files. |
| Restart metadata is missing, stale, corrupt, or contradictory | `recovery_required`; no automatic selection, deletion, or replay. |

## Reversibility

### Option A

High. The new module is unreachable from production entry points. It can be
feature-disabled or removed without changing a user database. Synthetic
temporary directories are disposable. No schema or app-data state changes.

### Option B

Low to medium. Real backup files, retention state, and replacements outlive a
process. A failed or ambiguous operation may require manual recovery and cannot
be safely undone automatically.

### Option C

High for this storage path because no filesystem behavior changes. Retrieval
work has its own future data, UI, and semantic rollback questions.

## Data And Privacy Impact

- Option A uses synthetic content only and writes only to explicit disposable
  test roots. Content-free manifests/state may hold digests, relative names,
  versions, timestamps, operation IDs, and outcomes, but no Experience,
  artifact, packet, credential, or provider-error content.
- Option B duplicates and may replace real sensitive Life OS data; it requires
  separately approved location, disclosure, visibility, expiry, delete-now,
  failure retry, and provider-receipt limitations.
- Option C adds no backup duplicate but may expose sensitive categories through
  retrieval unless separately minimized and governed.

## Effect On Phase 3 Exit Gaps

| Gap | Option A | Option B | Option C |
| --- | --- | --- | --- |
| Complete artifact/provenance export | Storage safety foundation only; not closed | May unblock future migration, still not closed | No progress |
| Revision/rejection history | Storage safety foundation only; not closed | May unblock v5 activation, still not closed | No progress |
| Post-review correction/deletion | Storage safety foundation only; not closed | May unblock lifecycle slices, still not closed | No progress |
| Explicit emotion/relationship/value-conflict/time-range retrieval | No progress | No progress | Direct progress; separate gate required |
| User-facing provenance/dependency inspection | Storage safety foundation only; not closed | May unblock later inspector, still not closed | No progress |

No option by itself authorizes Phase 4 or closes the Phase 3 exit audit.

## Orchestrator Recommendation

Select **Option A**. It is the smallest next increment that materially advances
production readiness beyond fixture simulation while preventing any real-user
or runtime activation. Option B is too broad and destructive for the current
evidence. Option C is a legitimate product-first alternative, but it abandons
the currently coherent migration-safety sequence before its filesystem risk is
bounded.

## Default Safe Action

Remain at `human_decision_required`. Do not create a feature branch or modify
production/test implementation until the Founder provides an exact response.

## Blocked Files Or Phases

- All production Rust/TypeScript/UI/configuration files.
- `src-tauri/tests/schema_v5_backup.rs` and any new Slice 2B-2 tests.
- Any Slice 2B-2 feature branch.
- Production backup/restore, Tauri registration, renderer/UI/startup/app-data,
  real user data, retention/delete-now/scheduling, sidecar cleanup, migration,
  schema v5, `user_version = 5`, Slices 3-6, Phase 4, Harness expansion,
  Stage 2/3, Git promotion, PR, and deployment.

## Exact Founder Response Needed

To approve the recommended bounded option, reply exactly:

> I resolve PHASE3C-SLICE2B2-001 by selecting Option A. I authorize Phase 3C Slice 2B-2 only: production-quality path-injected Rust filesystem safety primitives exercised exclusively against synthetic/disposable app-like directories; an explicit caller-held exclusive-operation/quiescence guard with active or unknown database activity failing closed; WAL, SHM, and rollback-journal detection and refusal without checkpointing, deletion, movement, or truncation; canonical owned-root and distinct-path validation; refusal of traversal, aliases, symlinks/reparse points, cross-volume replacement, and destination collisions; collision-resistant create-new operation ownership for backup, staging, and state files; closed-file exact digest, governed source-manifest, schema-version, foreign-key, integrity, and exact-record validation immediately before replacement and after reported commit; staged-file flush and honest platform-supported parent-directory durability reporting; a platform-aware injected replacement abstraction with committed, failed-unchanged, and outcome-unknown results; restart-state inspection with recovery_required for ambiguity and no autonomous retry, replay, rollback, repair, sidecar cleanup, or candidate selection; exact ownership-based cleanup; deterministic failure and restart tests; factual documentation synchronization; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. The production Rust module must remain unregistered and disconnected from Tauri commands, renderer, UI, startup, app-data paths, and any real user database. I do not authorize production backup or restore activation, backup or restore of real user data, real app-data testing, production file replacement invocation, user-facing disclosure or controls, retention, delete-now, scheduling, SQLite sidecar cleanup, schema v5, user_version 5, migration, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

To select another option, name `Option B` or `Option C` and state the exact
authorized scope. Silence never resolves the decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE2B2-001 by selecting Option A. I authorize Phase 3C Slice 2B-2 only: production-quality path-injected Rust filesystem safety primitives exercised exclusively against synthetic/disposable app-like directories; an explicit caller-held exclusive-operation/quiescence guard with active or unknown database activity failing closed; WAL, SHM, and rollback-journal detection and refusal without checkpointing, deletion, movement, or truncation; canonical owned-root and distinct-path validation; refusal of traversal, aliases, symlinks/reparse points, cross-volume replacement, and destination collisions; collision-resistant create-new operation ownership for backup, staging, and state files; closed-file exact digest, governed source-manifest, schema-version, foreign-key, integrity, and exact-record validation immediately before replacement and after reported commit; staged-file flush and honest platform-supported parent-directory durability reporting; a platform-aware injected replacement abstraction with committed, failed-unchanged, and outcome-unknown results; restart-state inspection with recovery_required for ambiguity and no autonomous retry, replay, rollback, repair, sidecar cleanup, or candidate selection; exact ownership-based cleanup; deterministic failure and restart tests; factual documentation synchronization; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. The production Rust module must remain unregistered and disconnected from Tauri commands, renderer, UI, startup, app-data paths, and any real user database. I do not authorize production backup or restore activation, backup or restore of real user data, real app-data testing, production file replacement invocation, user-facing disclosure or controls, retention, delete-now, scheduling, SQLite sidecar cleanup, schema v5, user_version 5, migration, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Slice 2B-2 only: production-quality path-injected Rust filesystem safety primitives exercised exclusively against synthetic/disposable app-like directories, with explicit quiescence guarding, sidecar refusal, canonical owned paths, exact validation, honest durability reporting, platform-aware replacement outcomes, restart-state inspection, exact-owned cleanup, deterministic tests, factual docs, verification, theory review, archive/reset, and stop at Founder diff review; no production activation, real data, schema v5, later slices, Phase 4, Harness expansion, Git promotion, PR, or deployment.

## Decided At And Evidence Reference

- Decided at: 2026-07-24T17:31:41.504Z
- Evidence reference: .ai/workflow/DECISION_REQUIRED.md#PHASE3C-SLICE2B2-001

## Resume Phase

engineering_planning
