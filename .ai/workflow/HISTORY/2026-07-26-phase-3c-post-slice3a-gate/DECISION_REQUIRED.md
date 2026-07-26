# Decision Required

Status: resolved
- Sprint ID: 2026-07-26-phase-3c-post-slice3a-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-26T12:22:29Z
- Updated at: 2026-07-26T12:22:29Z

## Decision ID

`PHASE3C-SLICE3B-001`

## Sprint ID

`2026-07-26-phase-3c-post-slice3a-gate`

## Decision Summary

Decide whether to authorize one private, unregistered, disposable-only Slice 3B
proof that integrates promoted verified-backup ownership with Slice 3A and
classifies definite, ambiguous, interrupted, and post-commit migration outcomes
from read-only durable evidence.

## Why Automation Stopped

The current migration core calls generic rollback after every SQL `COMMIT`
error, ignores rollback outcome, and returns an ordinary fail-closed error.
That cannot safely distinguish a known uncommitted transaction from a commit
that became durable before the connection reported failure. Changing this
boundary and adding durable restart states requires explicit Founder authority.

## Relevant Constitution Clauses

- Privacy Before Profit.
- Human Before AI.
- Evidence Before Conclusion.
- User data remains under user control.
- The AI must not take consequential destructive action without authority.

## Relevant Primary Definitions

- `docs/06_Memory.md`: provenance, correction, deletion, and no invented
  historical fact.
- `docs/10_Privacy.md`: transparency and ongoing user control.
- `docs/appendix/Harness.md`: evidence-bound behavior and explicit governance.

## Relevant ADRs

- ADR-0007: reviewed artifact and provenance preservation.
- ADR-0009: exact historical packet/consent/transmission/dependency semantics.
- ADR-0010: Phase 4 remains blocked and separate.
- ADR-0011: additive normalized lifecycle direction, honest baseline,
  non-destructive disable, and no automatic down migration.

## Available Options

### Option A — Authorize bounded private Slice 3B

Extend the existing owned-operation state contract, integrate its verified
schema-v4 backup evidence with Slice 3A, introduce an injectable COMMIT outcome
adapter, and implement read-only restart classification only against
synthetic/disposable exact-v4 databases.

The state model is:

- `Prepared`: exact owned operation exists; no verified backup.
- `BackupVerified`: exact v4 backup and source evidence match.
- `Migrating`: migration began after a final backup/source revalidation.
- `CommitOutcomeUnknown`: COMMIT result or state update is ambiguous.
- `V5Verifying`: durable user_version 5 detected; complete verification is
  required.
- `V5Ready`: receipt, contract, schema objects, manifests, guard emptiness,
  current content, foreign keys, and integrity all pass.
- `V4ReadyWithBackup`: exact logical v4 remains and the exact verified backup is
  available; no automatic retry.
- `V5BlockedRestoreAvailable`: v5 is present or partly evidenced, verification
  fails, and the exact verified backup is available; no automatic restore.
- `RecoveryRequired`: ownership, state, database, backup, or candidate evidence
  is missing, malformed, contradictory, multiple, or otherwise not safely
  classifiable.

Generic real SQL `COMMIT` errors default to outcome-unknown. Definite failure is
available only through explicit injected evidence. Every result closes the
writable connection and reopens read-only before classification.

### Option B — Authorize production migration activation

Connect backup and migration to startup/app-data/Tauri/UI and real user
databases.

### Option C — Defer migration and implement structured retrieval

Leave commit ambiguity unresolved and instead prepare the independent
emotion/relationship/value-conflict/time-range retrieval gap.

## Benefits

- **Option A:** closes a concrete safety hole without exposing migration,
  provides interruption/restart evidence, preserves one operation authority,
  and remains reversible because it is private and fixture-only.
- **Option B:** could move toward a user-visible v5 cutover, but would combine
  disclosure, quiescence, durability, restore, retention, and real-data risk.
- **Option C:** creates earlier user-visible retrieval value and directly
  addresses part of Phase 3 exit gap 4.

## Risks

- **Option A:** adds state complexity and still does not prove production
  durability or close any Phase 3 exit gap.
- **Option B:** can lose or select user data incorrectly; recovery, Windows
  durability, user approval, retention, and real-data verification are not
  proven.
- **Option C:** leaves a known migration ambiguity unresolved and delays
  revision/provenance foundations needed by gaps 1, 2, 3, and 5.

## Reversibility

- **Option A:** high. Private code and disposable fixtures only; production
  remains v4.
- **Option B:** low after a real v5 commit; automatic down migration is
  prohibited.
- **Option C:** high for product sequencing, but migration safety debt remains.

## Data And Privacy Impact

Option A uses synthetic/disposable databases and content-free operation state.
No user data, app-data, provider content, credentials, or production database
is accessed. Options B would expose real sensitive data and is not recommended.
Option C changes no storage data but may later require a separate Product
Harness retrieval gate.

## Orchestrator Recommendation

Option A.

Use one evolved owned-operation state record rather than a parallel
orchestrator. The record binds operation ID, exact live/backup paths, backup
digest/source manifest, expected v4 evidence, migration receipt/manifests when
present, and the current phase. Durable database and backup inspection remains
authoritative; state never authorizes an action.

Implement an injectable commit adapter with conservative outcomes:

- `Committed`;
- `DefinitelyNotCommitted`;
- `OutcomeUnknown`.

Production-like SQL errors map to `OutcomeUnknown` unless exact driver evidence
proves otherwise. After COMMIT or interruption, close the connection and
classify only from read-only durable evidence.

## Default Safe Action

Do not implement Slice 3B. Keep production schema v4 and retain this decision
package.

## Blocked Files Or Phases

Until resolution, do not modify implementation code, tests, production paths,
schema constants, startup, Tauri, renderer/UI, backup/restore activation,
lifecycle/export, Slice 4+, Phase 4, providers, ContextPacket, or Harness
behavior. Do not create a feature branch, stage, commit, push, merge, open a PR,
or deploy.

## Exact Founder Response Needed

To authorize Option A, reply exactly:

```text
I resolve PHASE3C-SLICE3B-001 by selecting Option A. I authorize Phase 3C Slice 3B only: create branch codex/phase-3c-slice3b-migration-restart-orchestration; implement one private, unregistered disposable migration orchestration by extending the existing owned-operation state contract rather than creating a parallel system; integrate the promoted exact schema-v4 verified-backup evidence with Slice 3A; add explicit Prepared, BackupVerified, Migrating, CommitOutcomeUnknown, V5Verifying, V5Ready, V4ReadyWithBackup, V5BlockedRestoreAvailable, and RecoveryRequired states; add an injectable COMMIT outcome adapter whose generic SQL error is OutcomeUnknown unless definite non-commit is proved; close writable connections and classify from read-only durable operation ownership, backup identity, live user_version, immutable receipt, database_contract, schema-object and source/target manifests, guard emptiness, current-content invariants, foreign_key_check, and integrity_check; preserve the exact verified backup; fail closed on missing, malformed, contradictory, altered, or multiple-candidate evidence; add deterministic interruption, definite-failure, ambiguous-v4, ambiguous-valid-v5, post-commit-blocked, rollback-failure, restart, and read-only-no-mutation tests against synthetic/disposable exact-v4 fixtures; synchronize factual documentation; run focused tests, Clippy, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I do not authorize real user databases or app-data paths, production SCHEMA_VERSION 5, startup/Tauri/renderer/UI activation, production backup/restore/replacement, automatic retry/replay/rollback/repair/restore/cleanup or candidate selection, migration disclosure UI, retention, lifecycle writes, export v2, Slice 4 or later, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.
```

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE3B-001 by selecting Option A. I authorize Phase 3C Slice 3B only: create branch codex/phase-3c-slice3b-migration-restart-orchestration; implement one private, unregistered disposable migration orchestration by extending the existing owned-operation state contract rather than creating a parallel system; integrate the promoted exact schema-v4 verified-backup evidence with Slice 3A; add explicit Prepared, BackupVerified, Migrating, CommitOutcomeUnknown, V5Verifying, V5Ready, V4ReadyWithBackup, V5BlockedRestoreAvailable, and RecoveryRequired states; add an injectable COMMIT outcome adapter whose generic SQL error is OutcomeUnknown unless definite non-commit is proved; close writable connections and classify from read-only durable operation ownership, backup identity, live user_version, immutable receipt, database_contract, schema-object and source/target manifests, guard emptiness, current-content invariants, foreign_key_check, and integrity_check; preserve the exact verified backup; fail closed on missing, malformed, contradictory, altered, or multiple-candidate evidence; add deterministic interruption, definite-failure, ambiguous-v4, ambiguous-valid-v5, post-commit-blocked, rollback-failure, restart, and read-only-no-mutation tests against synthetic/disposable exact-v4 fixtures; synchronize factual documentation; run focused tests, Clippy, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff review. I do not authorize real user databases or app-data paths, production SCHEMA_VERSION 5, startup/Tauri/renderer/UI activation, production backup/restore/replacement, automatic retry/replay/rollback/repair/restore/cleanup or candidate selection, migration disclosure UI, retention, lifecycle writes, export v2, Slice 4 or later, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Phase 3C Slice 3B only: private unregistered disposable migration orchestration; extend the existing owned-operation state contract; integrate verified schema-v4 backup evidence with Slice 3A; explicit migration/restart states; conservative injectable COMMIT outcome adapter; read-only durable classification; exact backup preservation; deterministic disposable failure/restart tests; factual documentation; focused/Clippy/canonical verification; Theory Alignment Review; archive/reset; stop at Founder diff review. All production activation, real-user data, later slices, Phase 4, Harness expansion, Git promotion, PR, and deployment remain excluded.

## Decided At And Evidence Reference

- Decided at: 2026-07-26T12:35:03.124Z
- Evidence reference: founder-message-2026-07-26-PHASE3C-SLICE3B-001

## Resume Phase

product_review
