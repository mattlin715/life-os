---
status: Founder-approved
version: 0.4
owner: product-and-engineering
last_updated: 2026/08/13
depends:
  - docs/00_Constitution.md
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/appendix/Harness.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
  - docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md
  - docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
referenced_by:
  - docs/00_Index.md
---

# 15 Phase 3C Production Activation Readiness Gate

## Purpose And Decision State

This Book One document is a **Founder-approved** production-activation
authorization gate. It audits what Life OS can prove after the promoted private
Slice 2 through Slice 4C-6B work and authorizes one smallest next implementation
slice only within the exact Option A boundary below.

It does not activate schema v5. The subsequently promoted Founder decision
authorizes only the exact read-only Inspector R1 runtime command and UI below;
it does not authorize a migration, backup, restore, retention job, write-capable
control, real-user migration test, deployment, or release.

Passing disposable tests is evidence about a contract. It is not evidence that
the production app can safely execute that contract against a user's database.

## Repository Baseline

- Audited branch point: `c28f5872f321ef0ad2f54f7952fc76f3c5e0be61`.
- This audit gate and its exact Founder Option A resolution were promoted through
  feature commit `7ce7cd4542a37f7163677725af12710762b2ccac` and
  non-fast-forward merge commit
  `bed87283f9141144a1c11500457363d5a8081aed`, whose parents are
  `c28f5872f321ef0ad2f54f7952fc76f3c5e0be61` and
  `7ce7cd4542a37f7163677725af12710762b2ccac`. The promoted scope was 14
  files with 1704 insertions and 17 deletions, measured with
  `git diff-tree --no-commit-id --shortstat -r c28f5872f321ef0ad2f54f7952fc76f3c5e0be61 7ce7cd4542a37f7163677725af12710762b2ccac`.
- Slice 4C-6B feature commit:
  `6d7b51d3d5ae3c27028951c3228f19a204bb62e1`.
- Slice 4C-6B non-fast-forward merge parents:
  `6f2c64c13a6085e73cc4f4c5dccf6d51a73e543d` and
  `6d7b51d3d5ae3c27028951c3228f19a204bb62e1`.
- Promotion scope: 15 files, 3055 insertions, 180 deletions, measured with
  `git diff-tree --no-commit-id --shortstat -r 6f2c64c13a6085e73cc4f4c5dccf6d51a73e543d 6d7b51d3d5ae3c27028951c3228f19a204bb62e1`.
- Production `SCHEMA_VERSION` and startup maximum are 4.
- `develop` and the local `origin/develop` ref were equal at audit intake with
  ahead/behind `0/0`; local `develop` had no upstream tracking branch.
- Canonical intake verification passed: 17 workflow tests, 26 Vitest files / 204
  tests, 182 Rust library tests, 12 backup/restore integration tests, and 8
  schema-contract integration tests, plus typecheck, build, Rust check, hygiene,
  and no Constitution diff.

## Evidence Vocabulary

- **Production v4:** reachable behavior used by the current desktop product.
- **Private verified:** promoted code exercised against synthetic/disposable
  fixtures only.
- **Compiled unreachable:** built into the Rust crate but not registered or
  called by startup, Tauri, renderer, UI, or app-data production paths.
- **Missing integration:** contract evidence exists, but the production caller,
  routing, disclosure, or lifecycle is absent.
- **Missing authority:** the Founder has not authorized the production action.
- **Missing verification:** required automated or manual evidence does not yet
  exist at the production boundary.
- **Deferred:** deliberately outside the smallest safe next slice.

Severity describes the consequence of activating schema v5 before the blocker
is closed. It does not describe the current schema-v4 product as unsafe.

## Production Readiness Matrix

| # | Capability | Implemented location and present evidence | Reachability and authority | Remaining blocker and missing verification | Premature activation severity | Deferred boundary |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Database startup inspection | `src-tauri/src/sqlite.rs` and `src/shared/storage/createLocalEvidenceStore.ts` perform production read-only presence/version inspection and block versions above 4. Private restart inspection exists in `filesystem_safety.rs` and `schema_v5_migration.rs`. | Production v4 inspection is authorized and reachable. Migration-state inspection is compiled unreachable and unauthorized for production. | No production routing for the nine migration/restart states; no pre-store v5 receipt/contract verification. | High | Option A may add read-only disclosure only; execution remains later. |
| 2 | Fresh-install initialization | Production initializes a missing database to v4. Fixed v5 DDL and exact contract tests create disposable v5 fixtures. | Production v4 reachable; fresh v5 private/test only and unauthorized. | No production fresh-v5 initializer, contract receipt, read routing, feature flags, or desktop manual evidence. | Critical | Fresh-v5 activation is a later independent gate. |
| 3 | v2/v3-to-v4 stabilization | `sqlite.rs::migrate_connection` production-migrates supported older schemas to v4 transactionally. | Production authorized and reachable. | No authorized combined v2/v3-to-v5 path; architecture/13 requires stabilization to v4 before a separately disclosed v5 upgrade. | High | Combined silent migration stays prohibited. |
| 4 | Explicit v4-to-v5 upgrade disclosure | Architecture/13 contains approved three-language disclosure requirements. Current UI discloses only checking/newer-schema/inspection/initialization failure. | Design Founder-approved; no production UI or runtime action. | No explicit-open readiness/disclosure surface, no separation between checking and migration consent, and no manual three-language evidence. | Critical | Recommended next slice covers disclosure without upgrade action. |
| 5 | Verified production backup creation | Slice 2A/2B-3 verify `VACUUM INTO`, exact ownership, digest, source manifest, schema, FK, integrity, and exact records on disposable fixtures. | Compiled private/unregistered; production and real-user backup unauthorized. | No app-data caller, sensitive-copy disclosure, real disk/permission evidence, production quiescence, or verified backup lifecycle. | Critical | Creation remains disabled in Option A. |
| 6 | App-data path ownership | Production resolves `app_data_dir()/life-os.db`. `filesystem_safety.rs` verifies canonical owned roots, distinct paths, links/reparse points, volume, and create-new ownership on injected paths. | Each half exists separately; integrated production operation ownership is absent and unauthorized. | No exact production operation-root creation, app-data identity handoff, or manual Windows/macOS evidence. | Critical | Option A may inspect metadata only; it creates no operation root. |
| 7 | WAL/SHM and connection quiescence | Private filesystem tests refuse WAL, SHM, journal, active, or unknown activity without cleanup. Current renderer uses plugin-SQL reads while Rust commands open separate connections. | Private verified; no process-wide production quiescence authority or caller. | Need a pre-store exclusive-operation boundary, proof all connections are closed, and platform desktop tests. Presence alone must never trigger checkpoint or deletion. | Critical | No quiescence claim in Option A. |
| 8 | Migration transaction execution | Slice 3A runs fixed DDL, backfill, reconciliation, receipts, guards, and `user_version = 5` last in one disposable `BEGIN IMMEDIATE` transaction with boundary injection. | Private verified and compiled unreachable; production DDL remains unauthorized. | No production caller, real-data performance envelope, preflight-to-transaction freshness contract, or release rollback plan. | Critical | Not in recommended slice. |
| 9 | Restart and ambiguous-COMMIT classification | Slice 3B records nine states and classifies exact durable v4/v5/blocked/unknown evidence read-only without retry. | Private verified; no production startup routing or persisted production operation. | No app-data operation ownership, UI mapping, manual interruption evidence, or production crash/power-loss proof. | Critical | Option A may only disclose pre-existing ambiguity if found; no action. |
| 10 | Blocked-state disclosure | Production has calm English/Traditional Chinese/Japanese newer-schema and inspection/init failure screens. | Production reachable for v4 startup failures. | No disclosure for backup, migrating, outcome unknown, v5 verifying, restore available, recovery required, retention failure, or disabled writes. | High | Recommended slice adds readiness-only states, not recovery controls. |
| 11 | Explicit restore | Slice 2B-1 verifies logical replacement simulation; Slice 2B-2/3 provide private ownership, replacement classification, and Windows adapter evidence. | Private/disposable only; no production restore or replacement authority. | No real app-data replacement, power-loss durability, sidecar recovery, explicit data-loss/provider-receipt UI, or cross-platform manual proof. | Critical | Separate future Founder gate. |
| 12 | Backup delete-now and 30-day retention | Architecture/13 Decision 10A approves the policy. Production 30-day cleanup exists only for unrelated unsuccessful ADR-0009 audit metadata. | Policy approved; backup retention implementation absent and unauthorized. | No backup registry, expiry scheduler, visible failure retry, exact-owned deletion, or manual evidence. | High privacy risk | Must ship with any production backup activation, not Option A. |
| 13 | v5 read routing | Guarded v4 projections and normalized v5 authority are verified in disposable fixtures. Current TypeScript reads schema-v4 tables through plugin SQL. | No production v5 reader or authority switch. | Need typed/read-only routing, contract-state gate, projection drift refusal, stale renderer invalidation, and v4/v5 parity tests at the runtime boundary. | Critical | Later activation slice. |
| 14 | v5 typed Experience writes | Slice 4A/4C-5 private writers cover create, import, correction, delete, source invalidation, ADR-0009 cascade, rollback, and ambiguity. | Private verified, unregistered, unauthorized. Production continues typed v4 commands. | No Tauri registration/routing, operation receipt/restart handling, or desktop/manual parity. | High | Later runtime-write gate. |
| 15 | Evidence/Reflection/Pattern/Context Recovery writes | Slices 4B, 4C, 4C-6A, and 4C-6B provide private canonical and migrated-current-action parity with exact provenance/dependencies. | Private verified, unregistered, unauthorized. | No v5 runtime commands/read models/UI reconciliation or production manual coverage. Some future UI-only lifecycle actions remain intentionally absent. | High | New lifecycle UI stays separate. |
| 16 | Phase 3B Historical Question parity | Slice 4C-2 privately verifies exact consumed consent, transmission, immutable packet, v4 authority, normalized v5 representation, dependencies, and ADR-0009 behavior. Production ADR-0009 remains schema v4. | v4 production reachable; v5 writer private and unauthorized. | No production v5 routing, restart reconciliation, or manual governed-generation/deletion after cutover. | High | Provider and ContextPacket behavior must not change. |
| 17 | Dependency invalidation and deletion | Private 4C slices verify Evidence/Reflection/Pattern/Experience consequences and exact Historical Question cascade. Production v4 uses current bundle/cascade semantics. | Both evidence layers exist, but v5 layer is unreachable. | Need production command routing, read-model state display, stale renderer handling, and exact real runtime regression. | High | Generic Phase 4 dependency infrastructure remains prohibited. |
| 18 | Old-binary write refusal | Candidate v5 DDL guard triggers and contract tests refuse guarded v4 projection writes without the exact guard. App versions are synchronized to 0.2.0. | Private contract evidence only. No production v5 database exists. | Need two-binary manual matrix showing older binary reads/refuses writes safely, release minimum-version enforcement, and no unsafe fallback. | Critical | Cannot be claimed by Option A. |
| 19 | Feature-disable behavior | ADR-0011/architecture13 approve non-destructive disable; private `database_contract` keeps lifecycle/export disabled during migration tests. | Design/private evidence only. | No production read-only/blocked routing, user disclosure, forward-fix recovery, or command-level refusal matrix. | High | Must precede runtime v5 writes. |
| 20 | Three-language UI disclosure | Current schema-v4 compatibility screens have English/Traditional Chinese/Japanese automated and prior Founder-manual evidence. Architecture/13 defines future upgrade/recovery copy. | Current refusal UI production reachable; migration disclosure absent. | Upgrade, backup, blocked migration, restore, expiry, and feature-disable states lack production UI and manual parity. | High | Recommended Option A covers readiness-only copy. |
| 21 | Desktop runtime manual verification | Founder manually verified schema-v5 refusal by the schema-v4 app and the three current compatibility messages. Private v5 writers have no desktop path. | Partial production v4 evidence only. | No actual production-path backup, migration, restart, v5 read/write, old-binary, restore, or retention manual run. | Critical | Option A has its own non-destructive manual matrix. |
| 22 | Release and rollback boundaries | No destructive down migration is authorized. Production build checks pass, but Windows/macOS release artifacts are not both verified. | No v5 release authority or deployed release. | Need signed/distributable build checks, minimum version, forward-fix plan, backup support window, and release rollback that never decrements schema. | Critical | Separate release gate after production verification. |
| 23 | Full v5 provenance inspection | Production Provenance Inspector P1 is a local read-only schema-neutral inspector for one already-loaded schema-v4 Historical Question packet. | P1 production implemented, Founder-manually accepted, and promoted; full v5 graph absent. | No normalized revision/review/lifecycle/tombstone/dependency graph inspector or cross-artifact navigation. | Medium for storage cutover; blocking for complete Phase 3 exit claim | Full inspector remains original Slice 6 work. |
| 24 | Export v2 | Current JSON/Markdown export is accurately Experience-only. Candidate `database_contract.export_v2` remains disabled. | Production v1 export reachable; v2 absent and unauthorized. | No canonical complete graph format, atomic export, digest/manifest, deleted/purged representation, three-language UI, or import policy. | Medium for storage cutover; blocking for complete Phase 3 exit claim | Original Slice 6 after storage lifecycle activation. |

## Original Slice 5 Reconciliation

The original architecture/13 sequence says Slice 5 will add Evidence first,
then Pattern and Reflection lifecycle parity. That wording predates the bounded
Slice 4B and 4C implementation series.

### Already proven privately

Promoted disposable evidence now covers:

- Evidence candidate creation, correction, confirmation, rejection, confirmed
  correction/deletion, exact dependent invalidation, purge, and ADR-0009
  cascade;
- Reflection suggestion, answer, correction, skip, answered deletion, Pattern
  invalidation, and ADR-0009 cascade;
- Pattern candidate creation, confirmation, rejection, confirmed correction,
  reconfirmation requirement, and deletion;
- Context Recovery suggestion, answer, and skip with categorical exclusion from
  historical context;
- Experience correction consequences across all current ordinary artifact
  kinds and exhaustive parent deletion;
- Historical Question normalized creation parity and deletion consequences;
- migrated `legacy_v4_baseline` Evidence/Pattern review actions and successor-
  producing Evidence/Reflection/Context Recovery actions currently reachable
  through the schema-v4 product.

### Still missing at production integration

- v5 read and typed-command routing;
- startup contract activation and exact blocked-state mapping;
- production operation ownership, quiescence, backup, migration, restart,
  restore, and retention handling;
- desktop runtime, old-binary, release, and real app-data evidence;
- lifecycle UI/read models for normalized history where current UI cannot show
  the full provenance graph.

### Genuinely unimplemented user-facing lifecycle actions

- explicit prior-revision content purge;
- standalone Context Recovery response correction/deletion;
- standalone deletion of suggested/skipped Reflection prompts;
- full revision/review/lifecycle/dependency/tombstone inspector;
- export v2.

The first three are not currently reachable product actions and may be safely
deferred until their user value is separately authorized. They must not be
invented merely to complete a list.

### Factual conclusion

“Slice 5 not started” is no longer accurate. Its private lifecycle contract
evidence is substantially implemented and promoted under the Slice 4B/4C
names. The original Slice 5 **production integration and Founder-manual gates**
have not started. This document does not rename or redefine accepted slices; it
proposes that architecture/13 receive this clarification only after Founder
acceptance.

## Original Slice 6 Reconciliation

Provenance Inspector P1 partially covers the inspector direction by showing one
already-loaded Historical Question's exact schema-v4 actual-use packet,
citations, provider/model, consent/transmission references, revisions, review
state, relevance, and packet-represented dependencies. It is collapsed,
explicit-open, local, read-only, and does not rehydrate sources.

P1 does not cover normalized schema-v5 Experience/artifact revision lineage,
review/lifecycle events, supersession, invalidation, content-free tombstones,
the complete dependency graph, or export v2. Those remain original Slice 6
work and are required before claiming the Phase 3 exit foundation complete.

## Threat Model

| Threat | Required fail-closed behavior |
| --- | --- |
| Startup migration race | Inspect before any writable initialization. A readiness result is not executable authority and cannot be reused after state changes. |
| Multiple SQLite connections | Unknown or active connection state is not quiescent. Never start backup, DDL, replacement, or restore. |
| WAL/SHM/journal sidecars | Detect and disclose; do not checkpoint, delete, move, truncate, or select another database. |
| Partially created backup | Preserve ambiguous output as `recovery_required`; delete only positively proven exact-owned incomplete output. |
| Disk-full or permission failure | Stop before migration. Do not retry silently or weaken durability. |
| Source mutation after manifest capture | Revalidate source identity and manifest immediately before the next write boundary; mismatch invalidates the operation. |
| Old binary opens v5 | Guarded writes fail closed; release policy must disclose the minimum compatible version. Never decrement schema. |
| Ambiguous COMMIT | Close writable connections and classify from exact read-only durable evidence. Generic SQL error means outcome unknown unless definite non-commit is proven. |
| Post-commit verification failure | Block with exact verified backup preserved. Do not auto-restore or promote to ready. |
| Automatic retry/replay temptation | No autonomous retry, replay, rollback, repair, restore, cleanup, or candidate selection. |
| Restore overwrites newer local data | Require a separate explicit action with backup time and data-loss-window disclosure and exact pre-replacement revalidation. |
| Provider receipts survive restore | Disclose that local restore cannot undo provider receipt or retention. |
| Backup retention failure | Keep the sensitive copy visible with exact path and retry/delete controls; never silently claim deletion. |
| UI claims success too early | Show success only after closed-file or reopened read-only durable verification; renderer state alone is not evidence. |
| v5 authority/v4 projection drift | Fail closed on any mismatch; never select whichever representation appears convenient. |
| Stale renderer state | Re-read authoritative state at every action boundary and reject stale commands; never rebind dependencies. |
| Current production fallback | Inspection, initialization, or schema mismatch blocks SQLite store construction. Never substitute in-memory storage for an existing unreadable local database. |
| Real-user data enters tests | Use only synthetic/disposable fixtures and app-like temp roots. Never copy the production app-data database into automated tests. |

## Next-Slice Alternatives

### Option A — Explicit read-only production readiness and disclosure

Add one explicit-open, session-only local database readiness panel and one
read-only production-path inspection command. The command runs only after the
user asks to check, never creates a missing database, never reads row content,
and never initializes, migrates, backs up, restores, checkpoints, deletes,
repairs, or writes operation state.

It may disclose only:

- whether the exact local database exists;
- detected `user_version` and the current supported maximum;
- exact-v4/older/newer/malformed/unreadable classification;
- sidecar presence and the fact that running-app quiescence is not proven;
- whether contradictory owned-operation evidence requires human recovery;
- that schema-v5 activation is unavailable in this slice.

The panel provides no Upgrade, Backup, Restore, Delete, Retry-migration, or
consent action. Closing it changes nothing. Normal schema-v4 behavior remains
unchanged.

**Benefit:** first production-path, three-language, manually reviewable evidence
without database mutation. **Risk:** users may mistake metadata readiness for
migration safety; copy must state the limit. **Reversibility:** remove the panel
and command; no schema or stored state exists.

### Option B — Fresh-install v5 without v4 migration

Would avoid existing-data mutation but immediately requires v5 readers,
writers, feature-disable semantics, packaging, and a split support matrix.
It is too broad for the next slice.

### Option C — Existing-v4 production migration activation

Would combine real backup, quiescence, DDL, restart, disclosure, routing,
restore, retention, and release risk. It is not independently reviewable and
is rejected for the next slice.

### Option D — Full lifecycle UI or export v2

Provides visible user control but does not close the production activation
boundary. It should follow verified storage/runtime routing, not precede it.

### Option E — Another private/disposable blocker

The repository already has deep private composition and lifecycle evidence.
Another private slice would be justified only by a newly demonstrated blocker.
No such blocker was found in this audit.

## Recommendation

Recommend **Option A**, decision ID
`PHASE3C-PRODUCTION-READINESS-R1-001`.

This is read-only and production-path reachable only after an explicit user
action. It is not migration-capable. It cannot mutate an existing database and
does not create a fresh v5 database. It establishes the disclosure and runtime
round-trip before any write-capable production gate.

## Exact Anticipated Option A Allowlist

Only the following product paths may change in an authorized implementation:

1. `src-tauri/src/filesystem_safety.rs`
2. `src-tauri/src/sqlite.rs`
3. `src-tauri/src/lib.rs`
4. `src/shared/storage/sqlite/databaseReadiness.ts` (new)
5. `src/shared/storage/sqlite/databaseReadiness.test.ts` (new)
6. `src/app/DatabaseReadinessPanel.tsx` (new)
7. `src/app/DatabaseReadinessPanel.test.tsx` (new)
8. `src/app/App.tsx`
9. `src/app/i18n.ts`
10. `src/app/i18n.test.ts`
11. `src/styles.css`
12. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
13. `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`
14. the repository-required current workflow artifacts and terminal archive for
    the uniquely named implementation sprint.

No DDL, shared v5 DDL file, migration writer, schema-v5 artifact writer,
provider, ContextPacket, consent, export, Tauri configuration, package version,
or Book Zero file is in the allowlist. Any need for another product path returns
to `human_decision_required`.

## Automated Acceptance Matrix For Option A

| Area | Required evidence |
| --- | --- |
| Default behavior | Initial render and normal startup make no readiness invoke; schema-v4 behavior and cleanup remain unchanged. |
| Explicit action | Only explicit open/check invokes the read-only command; repeated checks do not persist permission. |
| Missing database | Report missing without creating a file or directory. |
| Exact v4 | Report current v4 metadata and state that migration is unavailable; database bytes/mtime and governed rows remain unchanged. |
| Older schema | Distinguish older version; do not stabilize or migrate within the readiness command. |
| Newer schema | Fail closed and preserve the existing startup refusal contract. |
| Malformed/unreadable | Calm blocked state; no memory fallback, alternate database, repair, or raw SQLite error leakage. |
| Sidecars/activity | Report presence/unknown quiescence and leave WAL/SHM/journal byte-identical. |
| Owned evidence | Missing, malformed, contradictory, or multiple operation candidates report `recovery_required`; no selection or cleanup. |
| Path safety | Canonical app-data root and exact database identity; traversal, alias, link/reparse, and unexpected path fail closed. |
| No mutation surface | No Upgrade/Backup/Restore/Delete/Retry-migration button or invoke exists. No operation directory, receipt, manifest, backup, or state file is created. |
| Stale UI | Each displayed result includes inspection time/state and is never accepted as later migration authority. |
| i18n | English, Traditional Chinese, and Japanese have equivalent claims, warnings, and controls. |
| Regression | Existing startup, v4 CRUD, ADR-0009, R1 retrieval, P1 inspector, provider, and all private v5 tests remain green. |
| Scope | `SCHEMA_VERSION = 4`; no DDL/schema/provider/ContextPacket/consent/export/Phase 4 diff. |

## Founder Manual Verification Matrix For Option A

Use the desktop development app only after the implementation reaches Founder
review. Do not use a production migration candidate or enable any write action.

1. Confirm normal startup does not open or run the readiness check.
2. Explicitly open the local database readiness panel; confirm it says no
   backup or migration occurred and no upgrade is available.
3. Close and reopen the panel; confirm no consent/preference is persisted.
4. Restart the app; confirm panel state is closed and schema-v4 operation is
   unchanged.
5. Repeat the disclosure in English, Traditional Chinese, and Japanese.
6. Confirm there is no Upgrade, Backup, Restore, Delete, or retry-migration
   control.
7. For malformed/sidecar/operation-evidence cases, accept canonical automated
   disposable evidence unless the Founder deliberately supplies a disposable
   app profile. Never corrupt or copy the real user database for this check.

## Database Readiness Inspector R1 Implementation Evidence

The bounded R1 implementation is production-path reachable only after the user
opens the panel and separately chooses `Check now`. Opening, normal startup,
and normal schema-v4 product use do not invoke the command. The command returns
only a fixed content-free result covering database presence, bounded version
classification, sidecar presence, non-proven quiescence, owned-operation
evidence, the supported maximum, schema-v5 unavailability, and inspection time.

The Rust path validates the exact app-data database identity, rejects traversal,
links/reparse points, hard-link aliases, malformed or multiple operation
evidence, and uses create-disabled read-only immutable SQLite inspection.
Missing paths are not created. Sidecars and owned-operation evidence are not
checkpointed, selected, moved, deleted, repaired, or retried. The TypeScript
adapter rejects unknown or authority-expanding fields and suppresses raw backend
errors and paths. The session UI exposes only `Check now` and `Close` controls,
with equivalent English, Traditional Chinese, and Japanese disclosure.

R1 does not make the readiness result executable migration authority and does
not close any fresh-v5, existing-v4 migration, backup/restore, retention,
v5-routing, lifecycle UI, full provenance, export-v2, real-user, deployment, or
release gate. Production `SCHEMA_VERSION` and startup maximum remain 4.

Canonical repository verification passes with 17 workflow tests, 28 Vitest
files / 227 tests, 189 Rust library tests, 12 backup/restore integration tests,
and 8 schema-contract integration tests, plus TypeScript typecheck, production
frontend build, Rust check, repository hygiene, and no Constitution diff.
Focused R1 evidence includes 17 adapter tests, 5 panel tests, the three-language
i18n assertions, and 7 Rust readiness tests. Clippy also passes for all affected
Rust targets with warnings denied.

## Founder Decisions

The Founder must decide:

1. Whether to select Option A, B, C, D, or E.
2. If Option A, whether the exact read-only metadata and explicit-open contract
   above is accepted.
3. Whether architecture/13 may clarify that private Slice 5 lifecycle evidence
   is substantially implemented while production Slice 5 integration remains
   unstarted.
4. Whether the exact Option A allowlist and automated/manual matrices are
   accepted.
5. Whether no new ADR is required.

The recommended exact response is:

> I resolve PHASE3C-PRODUCTION-READINESS-R1-001 by selecting Option A. I
> authorize only the explicit-open, session-only, read-only production-path
> database readiness inspection and three-language disclosure defined in
> architecture/15, using the exact anticipated allowlist and acceptance
> matrices. The command must not create, initialize, migrate, back up, restore,
> checkpoint, clean up, repair, select, or mutate any database or operation
> evidence, and the UI must expose no write-capable control. I accept the
> factual Slice 5 clarification and that no new ADR is required. I do not
> authorize fresh-v5 initialization, existing-v4 migration, production backup,
> restore, retention, v5 read/write routing, lifecycle UI, full provenance
> inspection, export v2, real-user migration testing, provider or ContextPacket
> changes, Phase 4, Harness expansion, staging, commit, push, merge, PR,
> deployment, or release.

On 2026/08/10, the Founder resolved
`PHASE3C-PRODUCTION-READINESS-R1-001` as Option A and accepted:

- the explicit-open, session-only, read-only production-path database readiness
  inspection and three-language disclosure;
- the exact anticipated allowlist and automated/manual acceptance matrices;
- the factual Slice 5 clarification; and
- the conclusion that no new ADR is required.

This approval and document were subsequently promoted by the commits recorded in
the Repository Baseline. That promotion authorizes only Inspector R1 and does
not authorize fresh-v5 initialization, existing-v4 migration, production backup
or restore, retention, v5 read/write routing, lifecycle UI, full provenance
inspection, export v2, real-user migration testing, provider or ContextPacket
changes, Phase 4, Harness expansion, deployment, or release.
> 2026/08/13 factual note: architecture/17 implements the isolated Founder
> activation candidate authorized after this gate. This does not close the
> ordinary-profile, real-user, deployment, release, or Phase 4 gates.
