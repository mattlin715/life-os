# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-26-phase-3c-post-slice3a-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `7e0e5c44e03e58769f834243477c901cb01771ab`
- Working-tree digest reviewed:
  `61d38f19aae1e1a91cd8b924f84eac1ac53527832e86f2652eb3bbeae3835b7c`
- Created at: 2026-07-26T12:22:29Z
- Updated at: 2026-07-26T12:34:00Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Close the promoted Slice 3A facts without changing its authority, inspect the
actual commit/restart uncertainty in the private disposable migration core, and
ask the Founder whether to authorize one smallest complete private Slice 3B
orchestration proof. This review does not implement or activate migration.

## Problem Statement

Slice 3A proves deterministic pre-commit rollback and post-commit verification,
but it does not model an uncertain SQL `COMMIT` result. The current
`migrate_disposable_v4` path treats every `COMMIT` error as an ordinary
`migration_commit_failed`, calls a rollback whose result is ignored, and returns
`recovery_required = false`. A connection failure can occur after SQLite has
durably committed but before the caller receives success, so that behavior can
misreport durable v5 as reversible v4 failure.

The promoted filesystem operation record owns backup/replacement evidence, but
its current states do not include migration, receipt, manifests, or
commit-outcome ambiguity. Its state writer syncs a truncated-in-place file;
missing, malformed, or contradictory state correctly fails closed, but the
record alone cannot prove the live database outcome.

## User Value

A bounded Slice 3B would reduce the chance that a future migration path retries,
restores, or selects a database after an uncertain commit. It provides
repository-owned evidence that user data remains untouched unless durable
database facts prove a valid v5 result. It does not yet create a user-facing
upgrade.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Privacy Before Profit; Human Before AI; user data
  control; no silent destructive recovery.
- `docs/06_Memory.md`: durable memory must remain revisable and preserve exact
  provenance; missing history must not be invented.
- `docs/10_Privacy.md`: data belongs to the user; transparency, correction,
  deletion, and ongoing consent are required.
- `docs/appendix/Harness.md`: product behavior remains provider-independent and
  evidence-bound; this engineering sprint must not alter Product Harness
  authority.

## Relevant ADRs

- ADR-0007 requires reviewed artifacts, lifecycle, deletion, revision, and
  provenance to survive persistence changes.
- ADR-0009 requires exact packet, consent, transmission, dependency, and
  actual-use distinctions to remain byte- and revision-coherent.
- ADR-0010 keeps Phase 4 implementation blocked and does not authorize this
  migration.
- ADR-0011 accepts additive normalized lifecycle direction, honest v4
  baseline, non-destructive feature disable, and no automatic down migration;
  exact implementation remains Founder-gated.

## Current Implementation Context

- Slice 3A feature commit
  `3947b1862c177aabc95dc1d1796a4e1ccbc3ade2` was promoted through two-parent
  merge `7e0e5c44e03e58769f834243477c901cb01771ab`.
- Rename-aware Git reports 17 files; `--no-renames` reports the exact 18 paths
  because the unchanged DDL moved to `src-tauri/schema/schema_v5.sql`.
- Baseline canonical verification passed: workflow 17/17, Vitest 22 files/163
  tests, Rust library 59/59, backup/restore integration 12/12, and schema
  contract 8/8.
- Production `SCHEMA_VERSION` remains 4. The migration and filesystem modules
  are private, unregistered, and unreachable from startup, Tauri, renderer, UI,
  app-data, or a real user database.
- All current pre-commit injected failures occur before `COMMIT`; the current
  test seam does not inject definite/ambiguous COMMIT outcomes.
- `filesystem_safety.rs` already owns operation identity, backup identity,
  expected database digest, and read-only fail-closed restart inspection.
  Creating an unrelated second orchestration system would duplicate authority.

## In Scope

- Factual Slice 3A promotion closeout in architecture/13.
- Commit-outcome and restart-state audit.
- Alternatives A/B/C and five-gap impact.
- One proposed, private, disposable-only Slice 3B boundary.
- A single extension of the existing owned-operation state contract rather than
  an independent orchestrator.
- Explicit Founder decision package and stop at
  `human_decision_required`.

## Out Of Scope

Implementation; feature branch creation; real user data; production app-data;
production schema v5; startup/Tauri/renderer/UI activation; production
backup/restore/replacement; automatic retry/replay/rollback/repair/restore or
candidate selection; disclosure UI; retention; lifecycle writes; export v2;
Slice 4+; Phase 4; providers; ContextPacket; Harness expansion; Stage 2/3; Git
promotion; PR; deployment.

## Product Constraints

Production remains safely usable as schema v4. A state label must never be
treated as stronger evidence than the closed database, receipt, contract,
manifests, guard emptiness, current-content invariants, foreign keys, integrity,
and verified backup. An uncertain outcome blocks automatic action.

## Evidence And Provenance Constraints

The restart classifier must preserve exact source/target manifests, database
contract, immutable migration receipt, schema-object manifest, guarded current
content, ADR-0009 records, backup digest, and operation ownership. It must not
invent a receipt, backup, consent, review event, or successful commit.

## Historical Context Constraints

No provider context or Historical Reflection Question behavior changes.
ADR-0009 packet/provenance/dependency bytes remain migration evidence only and
are not reused as general memory.

## Consent Constraints

No migration consent or disclosure UI is introduced. Future production upgrade
and restore actions still require separate explicit user controls. Existing
historical provider consent is neither migration consent nor recovery consent.

## AI-Role Constraints

The AI may classify repository and disposable-database evidence; it may not
choose a live database, retry, restore, repair, infer missing history, or claim
production safety.

## Privacy Constraints

State and backup metadata remain content-free. No raw Experience, artifact,
packet, provider error body, or credential may enter operation-state files or
logs. Synthetic fixtures only.

## User-Agency Constraints

Verified backups are preserved. No restart path performs automatic migration,
cleanup, rollback, restore, retry, or candidate selection. Any future
production restore remains an explicit separately authorized user action.

## Acceptance Criteria

If Founder-authorized, Slice 3B must:

1. integrate the promoted owned-operation/verified-backup evidence with Slice
   3A instead of creating a parallel migration system;
2. add explicit `Prepared`, `BackupVerified`, `Migrating`,
   `CommitOutcomeUnknown`, `V5Verifying`, `V5Ready`,
   `V4ReadyWithBackup`, `V5BlockedRestoreAvailable`, and
   `RecoveryRequired` classifications;
3. treat generic `COMMIT` errors as outcome-unknown unless an injected adapter
   provides proof of definite non-commit;
4. close the writable connection and classify from read-only durable database
   and exact backup evidence before returning a state;
5. recognize a valid committed v5 after a crash before state recording;
6. prove exact v4 plus verified backup without silently retrying;
7. block inconsistent v5, missing receipt, nonempty guard, schema drift,
   altered/missing backup, malformed/contradictory state, and multiple
   candidates;
8. prove read-only inspection does not mutate any disposable file;
9. keep production `SCHEMA_VERSION = 4`, all modules private/unregistered, and
   every production activation fence closed;
10. pass focused failure/restart tests, Clippy, canonical verification, Theory
    Alignment Review, archive/reset, and Founder diff review.

## Risks

- Incorrectly trusting state metadata could select the wrong durable database.
- Retrying COMMIT after ambiguity can duplicate or contradict lifecycle facts.
- Automatic restore could destroy a valid committed v5 database.
- State-file truncation or crash can make orchestration metadata malformed;
  durable database evidence therefore remains authoritative and malformed state
  must block.
- Fixture proof cannot establish Windows power-loss durability, process-wide
  quiescence, or real-user recovery.
- The slice strengthens a migration foundation but does not close any of the
  five Phase 3 exit gaps by itself.

## Open Questions

None. The Founder resolved `PHASE3C-SLICE3B-001` as Option A with the exact
bounded scope recorded in `DECISION_REQUIRED.md`.

## Human Decision Required

No. `PHASE3C-SLICE3B-001` is resolved. Implementation remains subject to every
recorded exclusion and must stop at Founder diff review.

## Recommendation

Implement the Founder-authorized Option A. Extend the single existing
owned-operation state contract with
tightly bound migration evidence and a conservative injectable COMMIT adapter.
Treat state as context, not truth; classify the closed live database and
verified backup read-only. Defer production activation and the independent
structured-retrieval gap.

Conditions:

1. create no parallel orchestration authority;
2. generic SQL COMMIT error is outcome-unknown unless definite non-commit is
   explicitly proved;
3. closed read-only database and exact verified-backup evidence are
   authoritative;
4. preserve backups and perform no autonomous retry, rollback, restore, repair,
   cleanup, or candidate selection;
5. keep every production/runtime/later-slice exclusion closed;
6. complete focused tests, Clippy, canonical verification, Theory Alignment
   Review, archive/reset, and stop at Founder diff review.

## Review Status

approved_with_conditions
