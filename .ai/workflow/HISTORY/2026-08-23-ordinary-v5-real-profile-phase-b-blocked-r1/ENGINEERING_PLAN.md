# Engineering Plan

Status: approved

- Sprint ID: 2026-08-23-ordinary-v5-real-profile-phase-b-blocked-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest reviewed: d527ba9cce3541f58bcb524201b7b2aea8d8d79fe37c90cdb9ca35ccaa7b5dd5
- Created at: 2026-08-22T22:56:00.000Z
- Updated at: 2026-08-23T14:10:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder selected Option
A for `ORDINARY-V5-PHASEB-WAL-VERIFIER-CORRECTION-001`. Implement only a
disposable persistent-WAL correction. The real `com.lifeos.app` profile,
prepared operation, staging file, sidecars, and personal data remain untouched.
This sprint stops at Founder diff and disposable manual review; it cannot clean
up or retry the real migration.

## Existing Implementation Understanding

- `ExactV4CandidateVerifier::inspect` and `source_manifest_for_path` currently
  use a normal read-only SQLx connection. On a fully checkpointed database whose
  SQLite header retains WAL mode, this connection creates empty `-wal` and
  `-shm` sidecars.
- `create_owned_verified_backup` correctly rechecks sidecars immediately before
  `VACUUM INTO`, so it refuses after verifier-created sidecars.
- `SystemVacuumInto` also opens the source as a normal writable SQLite
  connection. On the same production-shaped database, that open would create
  source sidecars and make post-backup evidence ambiguous.
- SQLite permits `VACUUM INTO` from a read-only immutable source while writing
  only the exact-owned destination. Synthetic reproduction confirmed this
  remains sidecar-free.
- Existing activation tests create DELETE-journal fixtures and therefore did
  not cover a closed, fully checkpointed persistent-WAL source.

## Affected Modules

Exact anticipated implementation/document allowlist:

1. `src-tauri/src/schema_v5_migration.rs`
2. `src-tauri/src/filesystem_safety.rs`
3. `src-tauri/src/schema_v5_founder_activation.rs`
4. `src-tauri/src/schema_v5_runtime.rs`
5. `src-tauri/src/schema_v5_experience_write.rs`
6. `src-tauri/src/schema_v5_evidence_write.rs`
7. `src-tauri/src/schema_v5_evidence_lifecycle.rs`
8. `src-tauri/src/schema_v5_reflection_write.rs`
9. `src-tauri/src/schema_v5_pattern_write.rs`
10. `src-tauri/src/schema_v5_context_recovery_write.rs`
11. `src-tauri/src/schema_v5_historical_question_write.rs`
12. `scripts/founder-dogfood-package.mjs`
13. `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
14. `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
15. repository-required current `.ai/workflow/` artifacts and event/state files

Items 4-12 were added factually after the first disposable package proved that
the source-verifier correction alone was incomplete: stable activated-runtime
reads also required the immutable sidecar-prechecked boundary, typed writers
required explicit close before durable verification, and the existing Founder
package contract required synchronization with the already-registered lifecycle
module. These are revision-cycle-2 corrections within the approved boundary,
not new product or migration scope.

The ignored unsigned installer and manifest are build outputs, not staged
repository files.

## Proposed Design

1. Add a dedicated immutable read-only SQLite connection helper for exact
   source/durable inspection; do not globally change writable migration opens.
2. Use it in `ExactV4CandidateVerifier` and `source_manifest_for_path`.
3. Explicitly close verifier connections on both success and validation-error
   paths; a close failure remains fail-closed/recovery-required.
4. Open the `SystemVacuumInto` source as read-only and immutable. `VACUUM INTO`
   may create only the exact-owned absent destination inside the owned operation
   directory; source bytes and source sidecars remain unchanged.
5. Add a production-shaped fixture that sets persistent WAL mode, checkpoints,
   closes, proves header bytes `2/2`, and proves no initial sidecars.
6. Exercise the real activation/backup/migration boundary and prove durable v5,
   verified v4 backup, and no source sidecars.
7. Add explicit fail-closed cases for malformed source and for an uncheckpointed
   WAL source with sidecars; prove no operation/backup/migration is created.

## Alternatives Considered

- Delete or checkpoint sidecars: rejected because it mutates evidence and is
  explicitly unauthorized.
- Convert the real database from WAL to DELETE journal: rejected as an
  unneeded source mutation and policy change.
- Weaken the final sidecar guard: rejected because it would conceal active or
  uncheckpointed SQLite state.
- Make only the source/backup verifier immutable: initially selected, but the
  first packaged migration proved it incomplete because the normal activated
  runtime could create its own empty WAL/SHM pair. Revision cycle 2 therefore
  uses the same immutable, sidecar-prechecked read boundary for all stable
  schema-v5 runtime reads.
- Copy the main file directly: rejected because it would not provide the
  verified logical backup contract of `VACUUM INTO`.

## Data Lifecycle Impact

No product lifecycle semantics change. The correction changes only how the
already-authorized backup source is opened. Disposable fixtures are deleted by
test teardown. The real profile is untouched.

## SQLite Or Migration Impact

No DDL, schema version, migration transformation, receipt, manifest, restore,
retention, or classification policy changes. A fully checkpointed persistent-WAL
exact-v4 source can now be inspected and backed up without the application
creating source sidecars. Existing sidecars still fail closed.

## Provenance Impact

None. Exact record manifests, digests, artifact provenance, revision lineage,
and receipt bindings are unchanged and reverified by existing tests.

## Historical Context Impact

None. No historical selection, dependency, eligibility, packet, or cascade
behavior changes.

## Consent Impact

None. The migration still requires the existing explicit one-time action. This
correction creates no new authority for the current real profile.

## Provider Transmission Impact

None. No provider call, ContextPacket, key, model, or retention behavior is
changed.

## Import And Export Impact

None.

## Test Strategy

- Focused Rust tests for immutable exact-v4 verifier source inspection and
  explicit close behavior.
- Focused `SystemVacuumInto` persistent-WAL test proving destination creation,
  logical equality, and no source sidecars.
- Activation test through `authorize_founder_schema_v5_migration_at` using a
  closed, fully checkpointed persistent-WAL exact-v4 fixture.
- Malformed and uncheckpointed/sidecar source tests proving fail-closed state,
  no backup, and no migration.
- Run all affected Rust module tests.
- Run the real typed facade on a migrated persistent-WAL fixture and prove
  Experience, Evidence, and Context Recovery reads/writes leave no sidecars;
  rerun all Reflection, Pattern, Historical Question, lifecycle, rollback, and
  ambiguous-commit suites after explicit writer-close changes.
- Run Clippy with warnings denied for the Tauri crate.
- Run canonical `scripts/verify.ps1`.

## Repository Verification Strategy

Verify workflow state, focused tests, full canonical verification, exact diff,
tracked/non-ignored untracked allowlist, ordinary and legacy schema constants,
no registered extra runtime surface, and no staged files. Build the ordinary
review installer only through the existing ignored package script and record
its hash/version/identifier without adding it to Git.

## Manual UI Verification

Founder-owned and disposable-only. Preserve the first failed disposable v5
profile without retry, repair, restore, checkpoint, or cleanup. Install the new
unsigned review package in the disposable `LifeOSReviewR1` account, clone the
untouched disposable exact-v4 source into a new active profile, recreate its
persistent-WAL header with no sidecars, perform one explicit migration, verify
backup, typed runtime write, normal close, and restart, then stop. No action
against the real profile is permitted.

## Rollback Or Recovery Strategy

Before promotion, repository changes are reversible by ordinary diff review;
no Git reset is needed. Disposable fixtures/installers may be recreated. The
real profile and its preserved evidence are outside this correction and remain
unchanged. A later real evidence-disposition/retry proposal requires a separate
Founder gate.

## Documentation Impact

Update architecture/18 and dev/10 with the truthful Phase B fail-closed outcome,
root cause, correction boundary, automated evidence, and the continuing real
profile prohibition. Do not claim real-profile schema-v5 success.

## ADR Impact

No new ADR and no ADR status change. The correction implements the existing
fail-closed, verified-backup, local-first, provenance-preserving authority.

## Risk Level

Medium. The code change is narrow, but it guards production migration safety.
Production-shaped tests and a new disposable package review are mandatory.

## Escalation Decision

No additional decision is needed for implementation. Founder Option A resolves
the disposable correction boundary. Any current real-profile action remains
blocked and must not be inferred from this plan.
