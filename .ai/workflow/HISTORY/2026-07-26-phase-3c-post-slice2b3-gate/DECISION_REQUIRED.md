# Decision Required

Status: resolved
- Sprint ID: 2026-07-26-phase-3c-post-slice2b3-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-26T04:30:00+09:00
- Updated at: 2026-07-26T04:30:00+09:00

## Decision ID

`PHASE3C-SLICE3A-001`

## Sprint ID

`2026-07-26-phase-3c-post-slice2b3-gate`

## Decision Summary

Choose whether Life OS may implement a complete private schema-v4-to-v5
migration transaction exercised only against synthetic/disposable fixtures.
The recommended scope proves DDL, honest legacy backfill, reconciliation,
rollback, receipt, version-last commit, and post-commit blocked verification
without activating production migration.

## Why Automation Stopped

ADR-0011 and architecture/13 explicitly withhold migration implementation
authority until a separate Founder checkpoint. Passing Slice 0 and Slice 2
tests, the promoted filesystem primitives, this prompt, and repository silence
cannot authorize schema-v5 migration code.

## Relevant Constitution Clauses

- We Build Mirrors, Not Oracles.
- Human Before AI.
- Evidence Before Conclusion.
- Privacy Before Profit.
- Documentation Is Truth.

The migration must preserve exact known facts and visibly refuse uncertainty;
it may not invent revisions, review decisions, consent, provenance, or
historical meaning.

## Relevant Primary Definitions

- `docs/06_Memory.md`: evidence, provenance, correction, deletion, consent, and
  no silent identity accumulation.
- `docs/10_Privacy.md`: local ownership, transparency, and ongoing consent.
- `docs/12_Roadmap.md`: all five Phase 3 exit gaps remain blocking.
- `docs/appendix/Harness.md`: Product Harness/provider behavior is unchanged by
  storage migration.

## Relevant ADRs

- ADR-0007: reviewed artifacts require durable provenance and lifecycle.
- ADR-0009: Historical Question packet/consent/transmission/deletion remains
  authoritative and byte-preserved.
- ADR-0010: Phase 4 remains blocked by all five Phase 3 exit gaps.
- ADR-0011: normalized lifecycle direction is Accepted, but migration
  implementation needs this explicit Founder authorization.

## Available Options

### Option A — Authorize Slice 3A disposable migration core

Authorize only:

- branch `codex/phase-3c-slice3a-disposable-migration-core`;
- one private, unregistered, path/connection-injected Rust migration module;
- relocation of the unchanged executable DDL to one shared compile-time file
  used by both migration and contract tests, with continued byte alignment to
  architecture/13;
- ordered contract-verified DDL chunks, including projection guards installed
  last;
- synthetic/disposable exact-v4 fixtures only;
- exact legacy raw bytes, deterministic IDs, honest authorship/review/timestamp
  quality, and no reconstructed rejection history;
- one `BEGIN IMMEDIATE` DDL/backfill/reconciliation/receipt/guard/version
  transaction;
- `user_version = 5` only as the last mutation inside disposable tests;
- failure injection after every meaningful DDL, backfill, reconciliation,
  receipt, guard, and version boundary;
- exact governed-v4 rollback proof for every pre-commit failure;
- close/reopen read-only post-commit verification and blocked
  `recovery_required` result on inconsistency;
- malformed, v2, v3, v5, and greater-than-v5 refusal;
- lifecycle writes and export v2 disabled;
- focused tests, factual docs, canonical verification, Theory Alignment Review,
  archive/reset, and stop at Founder diff review.

### Option B — Authorize production startup/app-data migration

Connect schema-v5 migration to a real user database and application startup.
This would also require disclosure, cancellation, backup retention, recovery,
quiescence, Windows durability, and real-data evidence that do not exist.

### Option C — Defer migration and prioritize structured retrieval

Do not implement Slice 3A. Close this sprint and begin a separately governed
product design for explicit emotion, relationship, value-conflict, and
time-range retrieval.

### Option D — Defer all next implementation

Retain the factual Slice 2B-3 closeout and stop without opening another product
sprint.

## Benefits

- **A:** proves the complete lifecycle-storage cutover with no real-user-data
  exposure and no product activation.
- **B:** could make schema v5 available sooner.
- **C:** directly advances the one Phase 3 exit gap not addressed by the schema
  foundation.
- **D:** zero additional implementation risk.

## Risks

- **A:** disposable proof may be overstated as production readiness; malformed
  legacy shapes may reveal a contract gap; an incorrect DDL chunk loader could
  create a second schema authority unless byte reconstruction is tested.
- **B:** highest data-loss/privacy risk; production recovery, disclosure,
  quiescence, Windows durability, and rollback are unproved.
- **C:** four lifecycle/provenance exit gaps remain and migration uncertainty
  is deferred.
- **D:** all five Phase 3 exit gaps remain open.

## Reversibility

- **A:** highly reversible before activation; module is private and removable,
  and all pre-commit failures must reopen as exact governed v4.
- **B:** poor; a v5 user database cannot safely be written by older binaries and
  no automatic down migration exists.
- **C:** reversible as a local bounded retrieval design, but does not reduce
  migration risk.
- **D:** fully reversible; no new implementation.

## Data And Privacy Impact

- **A:** synthetic/disposable data only; no real path, user database, packet,
  journal, backup, or provider call.
- **B:** directly affects intimate real user data and is therefore not ready.
- **C:** no migration data impact, but future retrieval must still be local,
  selective, explainable, and consent-aware.
- **D:** none.

## Orchestrator Recommendation

Select **Option A** only. It is the smallest internally complete migration
evidence and preserves every production activation fence. Option B is not
recommended. Option C is a valid later sprint but does not replace the
lifecycle storage foundation.

## Default Safe Action

Do not implement migration code. Keep production schema v4 and remain at this
decision gate.

## Blocked Files Or Phases

Until resolution:

- no schema-v5 migration Rust module;
- no shared DDL relocation;
- no new migration fixtures/tests;
- no feature branch;
- no Engineering Plan or implementation phase;
- no production/startup/app-data/UI changes;
- no stage, commit, push, merge, PR, or deployment.

## Exact Founder Response Needed

To authorize the recommended bounded implementation, reply exactly:

```text
I resolve PHASE3C-SLICE3A-001 by selecting Option A. I authorize Phase 3C Slice 3A only: create branch codex/phase-3c-slice3a-disposable-migration-core; implement one private, unregistered, path/connection-injected Rust migration module exercised exclusively against synthetic/disposable exact schema-v4 fixtures; relocate the unchanged executable schema-v5 DDL to one shared compile-time file consumed by both the migration module and contract tests while preserving byte alignment with architecture/13 and the fixed object/digest contract; execute contract-verified ordered DDL with compatibility projection guards installed last; preserve exact legacy-v4-raw Experience and artifact bytes, deterministic IDs, honest authorship, legacy review state and uncertain timestamp quality, absent rejection history, ADR-0009 packet/provenance/dependency bytes, and no invented facts; perform one BEGIN IMMEDIATE DDL, backfill, reconciliation, database-contract, immutable-receipt, guard, foreign-key, integrity, current-content, manifest, and user-version transaction; set user_version 5 only as the last SQL mutation inside disposable test databases; inject failure after every meaningful DDL, backfill, reconciliation, receipt, guard, and version boundary; prove every pre-commit failure reopens with schema objects, user_version 4, governed row bytes/counts/digests, source manifest, and ADR-0009 records unchanged; close and reopen read-only for exact post-commit verification and return blocked recovery_required without autonomous action on inconsistency; refuse malformed, v2, v3, existing-v5, and greater-than-v5 inputs without mutation; keep lifecycle writes and export v2 disabled; add focused tests, factual documentation, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I accept that logical exact-v4 rollback does not claim a physically byte-identical SQLite file. I do not authorize production SCHEMA_VERSION 5, real user databases or app-data paths, startup/Tauri/renderer/UI activation, migration disclosure, fresh-install v5, combined v2/v3-to-v5 migration, production backup/restore/replacement, retention, lifecycle writes, export v2, Slices 4-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.
```

To choose another option, explicitly name `Option B`, `Option C`, or `Option D`
and state its authorized scope. Silence does not resolve the decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE3A-001 by selecting Option A. I authorize Phase 3C Slice 3A only: create branch codex/phase-3c-slice3a-disposable-migration-core; implement one private, unregistered, path/connection-injected Rust migration module exercised exclusively against synthetic/disposable exact schema-v4 fixtures; relocate the unchanged executable schema-v5 DDL to one shared compile-time file consumed by both the migration module and contract tests while preserving byte alignment with architecture/13 and the fixed object/digest contract; execute contract-verified ordered DDL with compatibility projection guards installed last; preserve exact legacy-v4-raw Experience and artifact bytes, deterministic IDs, honest authorship, legacy review state and uncertain timestamp quality, absent rejection history, ADR-0009 packet/provenance/dependency bytes, and no invented facts; perform one BEGIN IMMEDIATE DDL, backfill, reconciliation, database-contract, immutable-receipt, guard, foreign-key, integrity, current-content, manifest, and user-version transaction; set user_version 5 only as the last SQL mutation inside disposable test databases; inject failure after every meaningful DDL, backfill, reconciliation, receipt, guard, and version boundary; prove every pre-commit failure reopens with schema objects, user_version 4, governed row bytes/counts/digests, source manifest, and ADR-0009 records unchanged; close and reopen read-only for exact post-commit verification and return blocked recovery_required without autonomous action on inconsistency; refuse malformed, v2, v3, existing-v5, and greater-than-v5 inputs without mutation; keep lifecycle writes and export v2 disabled; add focused tests, factual documentation, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I accept that logical exact-v4 rollback does not claim a physically byte-identical SQLite file. I do not authorize production SCHEMA_VERSION 5, real user databases or app-data paths, startup/Tauri/renderer/UI activation, migration disclosure, fresh-install v5, combined v2/v3-to-v5 migration, production backup/restore/replacement, retention, lifecycle writes, export v2, Slices 4-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Phase 3C Slice 3A Option A only, exactly as stated in the Founder response; all listed exclusions remain binding.

## Decided At And Evidence Reference

- Decided at: 2026-07-25T19:53:27.318Z
- Evidence reference: Founder response in current Codex task

## Resume Phase

product_review
