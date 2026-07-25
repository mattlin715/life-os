# Decision Required

Status: resolved
- Sprint ID: 2026-07-26-phase-3c-post-slice2b2-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-25T17:36:12.6880273Z
- Updated at: 2026-07-25T17:36:12.6880273Z

## Decision ID

PHASE3C-SLICE2B3-001

## Sprint ID

2026-07-26-phase-3c-post-slice2b2-gate

## Decision Summary

Choose whether the next Phase 3C increment securely composes Slice 2A backup
creation with Slice 2B-2 operation ownership and adds a private Windows
replacement adapter, jumps directly to disposable schema-v5 migration
execution, or defers storage work for the independent structured-retrieval gap.

## Why Automation Stopped

No current authority permits Slice 2B-3. The promoted contracts also contain a
real mismatch: Slice 2A requires the backup destination to be absent, while
Slice 2B-2 pre-creates that file as an ownership reservation. The current
Windows durability adapter cannot establish parent-directory durability, the
replacement adapter has no concrete production implementation, and the
quiescence probe is only an injected interface. Successful prior tests and
promotion do not resolve these product/data-safety decisions.

## Relevant Constitution Clauses

- Preserve local-first user control and the user's authority over stored data.
- Do not silently overwrite, repair, roll back, downgrade, or delete user data.
- Preserve provenance, correction, deletion, visible uncertainty, and user
  agency.
- Preserve **We Build Mirrors, Not Oracles.**

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/06_Memory.md`: stored history remains user-owned information.
- `docs/10_Privacy.md`: local backup copies are sensitive duplicates and need
  bounded purpose, lifecycle, and deletion behavior.
- `docs/12_Roadmap.md`: five Phase 3 exit gaps remain blocking before Phase 4.
- `docs/appendix/Harness.md`: evidence must not be overstated as authority.

## Relevant ADRs

- ADR-0007: exact reviewed-artifact content, authorship, state, and provenance
  must survive.
- ADR-0009: exact historical consent, packet, transmission, dependency,
  deletion, and actual-use provenance must survive; consent is never inferred.
- ADR-0010: Phase 4 remains separately gated and unauthorized.
- ADR-0011: lifecycle/portable-provenance direction is Accepted, while
  production migration and destructive recovery require separate authority.

## Observed Repository Evidence

1. `develop` and `origin/develop` are at non-fast-forward merge
   `277c4b5b2031d5bf88dc2a33b03765c103c62629`; feature commit
   `a27faec856b2b1d628e87a1c8df1d13279668e14` is present.
2. Production `SCHEMA_VERSION` and SQLite `user_version` remain 4.
3. Workflow was valid and idle at intake. The clean promoted baseline passed
   canonical verification: 17 workflow tests, 22 Vitest files/163 tests,
   40 Rust library tests, 12 Slice 2A/2B-1 integration tests, 8 schema-contract
   tests, typecheck, frontend build, Rust check, hygiene, and Constitution
   checks.
4. Slice 2B-2 is private and unregistered. It has no Tauri, renderer, UI,
   startup, app-data, real-database, or product activation caller.
5. `create_verified_backup` refuses an existing destination and invokes
   `VACUUM INTO` only after the absence check.
6. `prepare_operation_with_id` creates backup, staging, and state paths with
   `create_new`; the resulting empty backup path is incompatible with the
   promoted Slice 2A repository contract.
7. Slice 2B-1 proves only fixture-local logical replacement and exact rollback.
   It is not a platform replacement implementation.
8. Slice 2B-2's `ReplacementAdapter` has only injected test behavior.
9. `SystemDurability` synchronizes file contents but reports Windows parent
   directory synchronization unsupported. The executor refuses before
   replacement when an adapter requires that guarantee.
10. `QuiescenceProbe` reflects a supplied state and rechecks it. No concrete
    process-wide coordinator currently stops every SQLite/persistence caller.

## Slice 2B-2 Factual Closeout

Architecture/13 version 1.6 now records that:

- Slice 2B-2 completed Theory Alignment Review and Founder diff review;
- feature commit `a27faec856b2b1d628e87a1c8df1d13279668e14`
  was promoted through non-fast-forward merge
  `277c4b5b2031d5bf88dc2a33b03765c103c62629`;
- canonical local verification passed before promotion and again on promoted
  `develop`;
- remote CI was not separately observed;
- desktop runtime smoke was not applicable because no runtime surface exists;
- promotion grants no product activation, real-data, schema-v5, or later-slice
  authority.

## Slice Compatibility Matrix

| Boundary | Slice 2A | Slice 2B-1 | Slice 2B-2 | Conclusion |
| --- | --- | --- | --- | --- |
| Backup creation | Absent destination, `VACUUM INTO`, exact verification | Uses existing verified backup | Pre-creates backup as owned empty file | Directly incompatible |
| Collision safety | Refuses existing destination | Refuses staging collision | `create_new` for backup/staging/state | Must preserve refusal while leaving backup absent |
| Ownership evidence | Test-local check | Test-local staging ownership | Operation ID and state-backed files | Needs an operation-directory/name claim |
| Replacement | None | Logical injected simulation | Trait/outcome model only | Windows implementation missing |
| Failure semantics | Best-effort cleanup of test output | Byte-identical live rollback in fixture | Known/unknown/recovery-required model | Can compose only with exact owned-output proof |
| Durability | SQLite/output and closed-file evidence | No OS claim | File sync; Windows parent sync unsupported | Windows durable commit unproved |
| Quiescence | Fixture behavior | Fixture behavior | Injected probe contract | Process-wide proof missing |
| Runtime | Disconnected | Disconnected | Disconnected | Safe common boundary to preserve |

## Available Options

### Option A — Slice 2B-3 bounded integration

Authorize only a private, unregistered integration in synthetic/disposable
app-like directories:

- create a high-entropy operation directory/state with create-new semantics;
- claim the exact backup child pathname in state while keeping it absent;
- preserve create-new staging/state ownership;
- immediately revalidate canonical identities, no reparse/alias, quiescence,
  sidecars, source identity, state ownership, and destination absence;
- invoke existing Slice 2A `VACUUM INTO` backup creation once;
- immediately validate the closed output's direct/single-link ownership,
  digest, source manifest, schema v4, foreign keys, integrity, and exact records;
- preserve ambiguous output; delete only positively proved exact-owned
  incomplete output;
- integrate, rather than duplicate, the promoted Slice 2A helper behavior;
- add a private `cfg(windows)` `ReplaceFileW` adapter with conservative
  committed/failed-unchanged/outcome-unknown mapping;
- keep `SystemDurability` Windows flows fail-closed before replacement when
  required metadata durability is unsupported;
- add deterministic cross-platform and Windows-only disposable tests;
- synchronize factual documentation, verify, review theory, archive/reset, and
  stop at Founder diff review.

### Option B — Jump to Slice 3A disposable schema-v5 migration

Begin executing the fixed schema-v5 DDL and reconciliation against disposable
databases before the backup ownership and Windows replacement seams compose.

### Option C — Defer storage and begin structured retrieval

Make only the factual closeout and start a separate Founder-gated product sprint
for explicit emotion, relationship, value-conflict, and user-selected time-range
retrieval. Do not add Slice 2B-3 or schema-v5 behavior.

## Benefits

### Option A

- Connects three promoted foundations instead of adding another isolated
  abstraction.
- Resolves the exact absent-destination/ownership mismatch without truncation,
  overwrite, or deleting an unknown file.
- Makes Windows API outcome semantics testable while refusing unsupported
  durability claims.
- Keeps changes reversible and synthetic-only.

### Option B

- Produces earlier executable schema-v5 evidence.
- Could expose DDL/reconciliation defects sooner.

### Option C

- Directly advances the one Phase 3 gap unrelated to architecture/13.
- Delivers more immediate reflective-product value than more migration
  infrastructure.
- Avoids adding Windows filesystem code now.

## Risks

### Shared data-loss and privacy risks

- A backup is a second copy of sensitive Life OS data.
- An incorrectly owned destination can overwrite or absorb an unrelated file.
- SQLite may leave an incomplete/corrupt `VACUUM INTO` output after
  interruption.
- Aliases, hard links, junctions, reparse points, antivirus, sync tools, and a
  second same-user process can change path semantics.
- Automatic cleanup can destroy evidence if ownership is inferred.
- Open connections, queued writes, or sidecars invalidate replacement safety.

### Option A

- The SQLite interface accepts a pathname rather than an already-owned handle.
  A high-entropy child in an exclusively created operation directory plus an
  application exclusive-operation lock bounds ordinary application races, but
  does not defeat a malicious same-user process with equivalent access.
- `ReplaceFileW` has documented failures that can remove or rename the replaced
  file. Conservative classification will intentionally produce
  `recovery_required`.
- `REPLACEFILE_WRITE_THROUGH` is unsupported; logical success is not crash
  durability.
- Current quiescence remains an interface contract, so production activation
  stays blocked.

### Option B

- Executes migration logic before the pre-migration backup can be safely and
  coherently owned or restored on Windows.
- Encourages false confidence from disposable migration success while the
  destructive recovery boundary remains disconnected.
- Higher rollback and future data-loss risk; it creates another isolated
  evidence layer.

### Option C

- Leaves four storage-related Phase 3 gaps and the migration safety sequence
  blocked.
- Structured retrieval can drift into sensitive or identity inference unless
  separately governed.
- Does not reconcile already-built backup/restore foundations.

## Windows Durability And Replacement Threat Model

1. `ReplaceFileW` is the appropriate private Windows API boundary because it
   replaces an existing file with a replacement file and preserves selected
   attributes/ACLs. It requires same-volume files.
2. Its `REPLACEFILE_WRITE_THROUGH` flag is documented as unsupported.
   Therefore API success proves only that the call reported logical
   replacement, not parent-directory or power-loss durability.
3. The existing staged-file `sync_all`/Windows flush can establish a closed
   candidate-content boundary before replacement, subject to OS/hardware
   honesty. It does not make the directory entry durable.
4. Return success maps to logical `Committed`, followed by state recording and
   exact live-file validation. It must not be labelled durable completion on
   Windows under the current `SystemDurability`.
5. `ERROR_UNABLE_TO_REMOVE_REPLACED` may become `FailedUnchanged` only when the
   live path and exact pre-call digest/identity reconcile. Otherwise it is
   `OutcomeUnknown`.
6. `ERROR_UNABLE_TO_MOVE_REPLACEMENT`,
   `ERROR_UNABLE_TO_MOVE_REPLACEMENT_2`, undocumented errors, missing paths,
   changed identities, or contradictory evidence are `OutcomeUnknown`.
7. On Windows, any higher-level contract requiring parent-directory durability
   refuses before calling the adapter. Slice 2B-3 tests adapter semantics but
   does not authorize a real durable replacement flow.

## Failure And Restart Matrix

| Point or evidence | Required result |
| --- | --- |
| Operation-directory/state create-new collision | Refuse; preserve pre-existing path; create no backup |
| Root, operation directory, state, or source identity changes | Refuse before SQLite; no cleanup of unknown paths |
| Active/unknown activity or WAL/SHM/journal sidecar | Refuse; no checkpoint, mutation, deletion, or backup |
| Backup pathname exists before `VACUUM INTO` | Refuse; never truncate, overwrite, or delete it |
| Collision injected after claim but before final preflight | Refuse; preserve collision evidence |
| SQLite open/create/VACUUM fails before ownership can be proved | Record failure; preserve uncertain output as `recovery_required` |
| Interrupted output is direct, single-link, and exactly operation-owned | Mark incomplete; cleanup may target only that exact owned file |
| Post-close reparse/link/identity/digest/manifest/schema/FK/integrity/record mismatch | `recovery_required`; preserve evidence; no automatic retry |
| Backup verifies exactly | Record backup-verified state; no runtime activation follows |
| Windows adapter returns success in direct adapter test | Logical `committed`; post-validate; never claim parent-directory/power-loss durability |
| Windows error 1175 plus exact unchanged live reconciliation | `failed_unchanged`; preserve backup/state |
| Windows error 1176 or 1177 | `outcome_unknown`; preserve all evidence |
| Other error or contradictory path/digest/identity | `outcome_unknown`; no retry, rollback, repair, or candidate selection |
| Restart at claimed-but-absent state | Report resumable evidence only; do not autonomously create |
| Restart with unverified backup output | `recovery_required`; preserve; no autonomous deletion |
| Restart with verified backup | Report verified evidence; no automatic migration/restore |
| State missing, corrupt, stale, or contradictory | `recovery_required`; no candidate choice |

## Reversibility

### Option A

High. The module remains private and unreachable from product runtime.
Disposable fixtures and operation roots can be removed. Production schema,
app-data, and user databases remain unchanged.

### Option B

Medium in disposable tests, but architecturally poor: it advances migration
execution before rollback/recovery integration and increases pressure to
overgeneralize test evidence.

### Option C

High for storage because no filesystem behavior changes. Retrieval work has its
own semantic, consent, and UI reversibility questions.

## Data And Privacy Impact

- Option A uses synthetic schema-v4 fixtures only. State and manifest metadata
  are content-free. No real path, Experience, artifact, packet, credential, or
  provider error is eligible.
- Option B also proposes disposable data, but starts executable schema
  transformation without a composed recovery foundation.
- Option C creates no backup copy, but future sensitive-category retrieval
  needs its own minimization and user-control gate.

## Effect On Five Phase 3 Exit Gaps

| Gap | Option A | Option B | Option C |
| --- | --- | --- | --- |
| Complete artifact/provenance export | Connects safety foundation; not closed | Migration execution foundation; not closed | No progress |
| Revision/rejection history | Connects safety foundation; not closed | Could exercise future v5 storage; not closed | No progress |
| Post-review correction/deletion | Connects safety foundation; not closed | Could exercise future v5 lifecycle; not closed | No progress |
| Emotion/relationship/value-conflict/time-range retrieval | No progress | No progress | Direct progress through a separate gate |
| User-facing provenance/dependency inspection | Connects safety foundation; not closed | No user-facing inspector; not closed | No progress |

No option authorizes Phase 4 or completes the Phase 3 Exit Audit.

## Orchestrator Recommendation

Select **Option A**. The bounded operation-directory/name-claim design preserves
Slice 2A destination nonexistence and Slice 2B-2 collision safety without
overwriting or truncating a path. It truthfully limits protection to the
application threat model, keeps Windows durability fail-closed, and connects
existing foundations. Option B is unsafe sequencing. Option C is valuable but
should follow this small integration closure or be explicitly chosen as a
priority change.

## Default Safe Action

Remain at `human_decision_required`. Do not create a feature branch or modify
Rust/tests beyond the already completed factual documentation closeout until
the Founder provides an exact resolution.

## Blocked Files Or Phases

- `src-tauri/src/filesystem_safety.rs`,
  `src-tauri/tests/schema_v5_backup.rs`, Cargo dependencies, and any new
  Slice 2B-3 test/module file.
- Any Slice 2B-3 feature branch.
- Real user data, app-data, production backup/restore/replacement, Tauri,
  renderer, UI, startup, retention, sidecar cleanup, schema v5,
  `user_version = 5`, migration, Slices 3-6, Phase 4, Harness expansion,
  Stage 2/3, Git promotion, PR, and deployment.

## Exact Founder Response Needed

To authorize the recommended bounded option, reply exactly:

> I resolve PHASE3C-SLICE2B3-001 by selecting Option A. I authorize Phase 3C Slice 2B-3 only: a private, unregistered backup ownership handoff and Windows replacement integration exercised exclusively against synthetic/disposable app-like directories and schema-v4 fixtures; a collision-resistant create-new operation directory and content-free state that claim an exact high-entropy backup child pathname while leaving that pathname absent for VACUUM INTO; preservation of create-new staging/state ownership; immediate pre-VACUUM revalidation of canonical root, operation-directory, state, source and destination identities, quiescence contract, sidecar absence, path absence, distinctness, same-volume, alias, hard-link, symlink/junction/reparse, and collision boundaries; one invocation of the integrated existing Slice 2A VACUUM INTO creation path; immediate post-close proof of a direct, single-link, exact-operation-owned regular backup followed by SHA-256, governed source-manifest, schema-v4, foreign-key, integrity, and exact-record verification; preservation as recovery_required whenever output ownership or validity is ambiguous; deletion only of positively proved exact-owned incomplete disposable output; integration of existing Slice 2A behavior rather than a parallel backup system; a private cfg(windows) ReplaceFileW adapter with conservative committed, failed-unchanged, and outcome-unknown classification; Windows execution remaining fail-closed before replacement whenever required parent-directory durability cannot be established; explicit restart-state evidence; deterministic cross-platform and Windows-only disposable tests; factual documentation synchronization; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that this does not prove protection against a malicious same-user process, process-wide production quiescence, Windows parent-directory or power-loss durability, production replacement safety, or real-user recovery. I do not authorize real user databases, real app-data paths, production backup/restore/replacement activation, Tauri, renderer, UI or startup integration, migration disclosure, retention, delete-now, scheduling, autonomous retry/replay/rollback/repair/candidate selection, SQLite sidecar cleanup, schema-v5 DDL, user_version 5, v4-to-v5 migration, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

To select another option, name `Option B` or `Option C` and state the exact
authorized scope. Silence never resolves the decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE2B3-001 by selecting Option A. I authorize Phase 3C Slice 2B-3 only: a private, unregistered backup ownership handoff and Windows replacement integration exercised exclusively against synthetic/disposable app-like directories and schema-v4 fixtures; a collision-resistant create-new operation directory and content-free state that claim an exact high-entropy backup child pathname while leaving that pathname absent for VACUUM INTO; preservation of create-new staging/state ownership; immediate pre-VACUUM revalidation of canonical root, operation-directory, state, source and destination identities, quiescence contract, sidecar absence, path absence, distinctness, same-volume, alias, hard-link, symlink/junction/reparse, and collision boundaries; one invocation of the integrated existing Slice 2A VACUUM INTO creation path; immediate post-close proof of a direct, single-link, exact-operation-owned regular backup followed by SHA-256, governed source-manifest, schema-v4, foreign-key, integrity, and exact-record verification; preservation as recovery_required whenever output ownership or validity is ambiguous; deletion only of positively proved exact-owned incomplete disposable output; integration of existing Slice 2A behavior rather than a parallel backup system; a private cfg(windows) ReplaceFileW adapter with conservative committed, failed-unchanged, and outcome-unknown classification; Windows execution remaining fail-closed before replacement whenever required parent-directory durability cannot be established; explicit restart-state evidence; deterministic cross-platform and Windows-only disposable tests; factual documentation synchronization; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that this does not prove protection against a malicious same-user process, process-wide production quiescence, Windows parent-directory or power-loss durability, production replacement safety, or real-user recovery. I do not authorize real user databases, real app-data paths, production backup/restore/replacement activation, Tauri, renderer, UI or startup integration, migration disclosure, retention, delete-now, scheduling, autonomous retry/replay/rollback/repair/candidate selection, SQLite sidecar cleanup, schema-v5 DDL, user_version 5, v4-to-v5 migration, Slices 3-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Phase 3C Slice 2B-3 only: private unregistered absent-path backup ownership handoff, integrated Slice 2A VACUUM INTO verification, private cfg(windows) ReplaceFileW outcome adapter, synthetic/disposable schema-v4 tests, factual docs, canonical verification, Theory Review, archive/reset, and stop at Founder diff review; all production activation, real data, schema v5, later slices, Phase 4, Harness expansion, Git promotion, PR, and deployment remain excluded.

## Decided At And Evidence Reference

- Decided at: 2026-07-25T17:56:19.729Z
- Evidence reference: .ai/workflow/DECISION_REQUIRED.md#PHASE3C-SLICE2B3-001

## Resume Phase

engineering_planning
