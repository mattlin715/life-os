---
status: Implemented
version: 0.6
owner: product-and-engineering
last_updated: 2026/09/14
depends:
  - docs/00_Constitution.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md
  - docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md
  - docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md
referenced_by:
  - docs/00_Index.md
  - docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md
---

# 19 Desktop Schema-v5 Prepared-State Recovery and Real-Profile Migration R2

## Status and authority

R2 implements three explicit, ordinary-identity recovery paths: the exact
pre-backup prepared state retained by R1, and the exact post-migration empty
WAL/zero-frame-SHM state created when a historical schema-v4 binary refuses a
valid schema-v5 database, plus the exact historical-frontend schema
representation that an older verifier rejected only after a valid schema-v5
COMMIT. The Founder separately completed real-profile Phase B and authorized
one Phase C attempt; that attempt created and verified one schema-v4 backup,
committed schema v5, then stopped fail closed with
`post_commit_schema_manifest_mismatch` before lifecycle writes were enabled.
The R2B correction itself is repository-only and uses synthetic roots; it does
not reopen or inspect the real `com.lifeos.app` profile. This document and the
implementation do not authorize real-profile recovery, retry, restore,
repair, checkpoint, schema decrement, backup deletion, migration, promotion,
distribution, deployment, release, Android, or Phase 4.

The product remains a mirror, not an oracle. Recovery changes only proven
technical transient evidence. Prepared-v4 preflight reads no SQL rows; A-R3
uses bounded immutable technical-contract and integrity queries but does not
retrieve, display, or interpret personal content fields.

## Exact R1 state

Architecture/18 records one failed real-profile attempt that stopped before
backup creation or migration. Repository evidence says the database remained
exact schema v4 and byte-identical to its pre-attempt value; one exact-owned
operation remained `prepared`; staging was zero bytes; no backup existed; and
the verifier-created WAL/SHM pair remained. R2 does not embed the personal
database fingerprint in source or tests. The explicit UI shows the current
closed-file identity and SHA-256 so the Founder can compare them with the
separately retained R1 evidence, and execution cryptographically binds itself
to that inspected claim.

## Supported predicate

Recovery is offered only when every condition below is re-proved:

1. compiled identity is ordinary desktop `com.lifeos.app` with
   `desktop-schema-v5`;
2. the database is a direct, single-link, non-reparse file with exact SQLite
   header `user_version = 4`;
3. its closed-file SHA-256 and platform file identity are part of the displayed
   and bound claim;
4. exactly one exact-owned `life-os-*.operation` exists;
5. operation state schema and identity are exact, phase is `prepared`, and all
   backup, migration, receipt, outcome, and v5 fields are absent;
6. the operation directory contains only `state.json` and zero-byte
   `staging.db`; no backup exists;
7. WAL and SHM are direct, single-link files; WAL is zero bytes or an exact
   header-only WAL with no frames; SHM has matching initialized headers,
   `mxFrame = 0`, `nPage = 0`, and `nBackfill = 0`;
8. no rollback journal, second operation, unexpected operation file, path
   alias, reparse point, traversal, hard link, or destination conflict exists;
9. a Windows read-share-only database handle excludes writer or delete-capable
   database handles, while an immediate exclusive lock over SQLite's exact
   512-byte coordination range at `PENDING_BYTE` refuses active SQLite
   shared, reserved, pending, or exclusive locks during the mutation window;
10. immediate reinspection produces the same claim digest and supplied
    database digest.

Any mismatch is ineligible or recovery-required. No cleanup control appears
for unsupported states. This is not a general SQLite repair tool.

### Exact post-migration legacy-refusal predicate

The second recovery control is offered only when every condition below is
re-proved together:

1. the direct, single-link, non-reparse live database has exact
   `user_version = 5` and its closed-file digest and platform identity are
   bound into the displayed claim;
2. exactly one exact-owned operation is present, with exact state schema,
   phase `v5_ready`, outcome `lifecycle_writes_enabled`, and migration,
   source-manifest, target-manifest, backup, and application-version evidence
   that agrees with the one committed migration receipt and exact schema-v5
   runtime contract;
3. that operation contains exactly `state.json`, zero-byte `staging.db`, and
   `backup.db`; the backup remains exact schema v4 and independently passes
   its stored SHA-256, manifest, foreign-key, integrity, and logical-record
   contract;
4. exactly one earlier prepared-state recovery receipt and its exact four-file
   quarantine remain valid and byte-identical;
5. no earlier v5-sidecar recovery receipt/quarantine or rollback journal
   exists;
6. the live WAL is a direct zero-byte file and the live SHM is a direct
   32768-byte file with matching index copies, initialized version 3007000,
   `mxFrame = 0`, page count `0`, and backfill `0`;
7. the execution window excludes concurrent SQLite activity and immediate
   reinspection produces the same claim over the database identity/digest,
   the exact two files to move, and all eight protected operation, backup,
   receipt, and quarantine files.

The main database is opened only through an immutable read-only connection
after the sidecar pair has passed that exact empty proof. This exception is
private to the classifier and does not weaken the normal rule that all other
schema-v5 runtime and verifier opens refuse any SQLite sidecar.

### Exact historical-frontend post-commit predicate

The R2B recovery control is a third, non-general predicate. It is offered only
when all of the following are re-proved together:

1. the live direct database is exact schema v5, has no WAL, SHM or rollback
   journal, and its file identity and closed-file digest are bound into the
   displayed claim;
2. exactly one owned operation exists and contains only `state.json`,
   zero-byte `staging.db`, and `backup.db`;
3. operation state is exactly `v5_blocked_restore_available`, outcome is
   exactly `post_commit_schema_manifest_mismatch`, and its migration ID,
   source manifest, target manifest, backup ID and one committed database
   receipt agree;
4. the verified backup still passes its stored digest, schema-v4 logical
   manifest, foreign-key and integrity evidence, including the exact fixed
   historical frontend-created schema representation;
5. the live database has the exact 95-object schema-v5 manifest derived from
   that historical representation, the fixed database contract is otherwise
   exact, source and target logical manifests agree, current-content checks
   pass, integrity passes, and lifecycle writes are either still `disabled`
   or already `enabled` after an interrupted explicit recovery;
6. the one earlier prepared-state recovery receipt and its exact four-file
   quarantine remain valid; the backup, zero staging, prior receipt and prior
   quarantine are bound as seven byte-preserved facts;
7. there is no second operation, unexpected operation file, receipt conflict,
   sidecar, path ambiguity or changed claim.

The source preflight is also strengthened. It now accepts only three fixed
schema-v4 object manifests: the contract fixture, the promoted Rust runtime,
and the historical frontend/Rust hybrid. Those exact sources produce three
fixed full schema-v5 manifests. An unknown additional object or changed stored
SQL representation is refused before backup/migration execution rather than
being discovered after COMMIT. This is an allowlist, not generic SQL
normalization or semantic equivalence inference.

## Explicit-open surface and consent separation

The blocked ordinary startup screen offers **Review bounded local recovery**
only for the narrow startup reasons that can correspond to an implemented
exact recovery predicate. Opening it performs bounded closed-file inspection
only. English,
Traditional Chinese, and Japanese copy explains database-byte preservation,
the exact quarantine set, the content-free receipt, cancellation, and the fact
that recovery is not migration consent.

Controls remain distinct:

- **Preserve and close** closes without mutation;
- **Show technical details** reveals only classification, operation identifier,
  relative paths, sizes, file identity, and SHA-256 values;
- **Prepare recovery** quarantines the exact four pre-backup transient files;
- **Quarantine the exact sidecars** quarantines only the proven WAL and SHM
  for the post-migration legacy-refusal classification;
- **Complete exact recovery** is available only for the historical-frontend
  post-commit predicate; it enables the already-committed schema-v5 lifecycle
  contract, finalizes only the bound operation state, and writes one
  content-free receipt;
- **Cancel** closes the surface without filesystem changes.

Successful recovery requires close and restart. The post-migration path
returns to the existing schema-v5 runtime; the pre-backup path returns to the
separate ordinary migration disclosure. Neither action opens migration,
creates a new operation or backup, reuses consent, or invokes a provider.
Exact v4 still requires a new button press as a separate per-operation
authorization.

## Mutation and durable evidence

The process-local migration lock serializes recovery with every schema-v5
operation. The pre-backup and empty-sidecar paths never open the database
writable and issue no SQL or checkpoint. The pre-backup path moves exactly four files: prepared
state, zero-byte staging, proven-empty WAL, and matching zero-frame SHM, then
removes the empty operation directory. The post-migration path moves exactly
two files: the proven zero-byte WAL and matching zero-frame SHM. It preserves
the live schema-v5 database, valid `v5_ready` operation, zero-byte staging,
verified schema-v4 backup, prior prepared-state receipt/quarantine, and all
personal records. Unexpected files are never selected.

The historical-frontend post-commit path performs no DDL, migration, restore,
replacement, checkpoint, schema decrement, backup deletion, or personal-row
transformation. It opens the writable connection, obtains `BEGIN IMMEDIATE`,
and then repeats the complete committed receipt, exact historical schema,
logical-manifest, current-content, contract and integrity verification inside
that same transaction. Any mismatch rolls back before the compatibility guard
or contract can change. Only after this write-locked recheck does the bounded
transaction use the existing compatibility guard to change
`database_contract.lifecycle_writes` from `disabled` to `enabled`. It then
writes a create-new content-free receipt and changes only the bound operation
state from `v5_blocked_restore_available` /
`post_commit_schema_manifest_mismatch` to `v5_ready` /
`lifecycle_writes_enabled`. If activation completed but later receipt/state
durability failed, restart remains fail closed and the same classifier can
distinguish the already-enabled contract; it never repeats activation
automatically. The explicit action must be selected again after a fresh claim
to finish only the missing durable step.

A create-new content-free receipt records only:

- receipt schema and approved classification;
- operation identifier;
- application and database schema versions;
- UTC action timestamp;
- relative filenames, byte sizes, and SHA-256 digests.

For the post-migration path the receipt additionally records the same bounded
facts for all protected files, so byte preservation remains independently
auditable. It contains no user root, Experience, Evidence, Reflection, Pattern, Context
Recovery, historical context, provider content, credential, or secret. The
prepared-v4 receipt is displayed after restart and has a separate explicit
delete control; deleting it does not delete quarantined evidence. The
post-migration receipt remains bounded technical evidence in the owned root
and is not presented as migration, backup, or personal content. Personal-data
retention policy is unchanged.

For the historical-frontend post-commit path, the receipt additionally records
only the fixed classification, migration/source/target digests, post-action
database file fact, pre-action operation-state file fact, seven preserved file
facts, and `lifecycle_writes = enabled`. It contains no SQLite row or personal
content field. The verified schema-v4 backup, zero staging, earlier receipt and
quarantine remain byte-identical.

Pre-backup success is reported only after the live database re-hashes to the
supplied digest, reclassifies as schema v4, all SQLite sidecars are absent, and
no operation candidate remains. Post-migration success additionally re-verifies
exact schema v5, its committed receipt/runtime contract, `v5_ready` state,
migration identity, and verified v4 backup after only the sidecars have moved.
Partial move or durability failure stops without
rollback, repair, retry, migration, or success claim and preserves the
quarantine as technical evidence.

## Unsupported states

Pre-backup recovery is refused for non-v4 or malformed databases, changed database
identity/digest, WAL frames, malformed WAL/SHM, non-matching SHM, rollback
journal, active writer/delete handle, non-zero staging, backup or migration
evidence, multiple/malformed operations, v5 state, missing/unexpected files,
links/reparse/alias/traversal, changed preflight claim, receipt/quarantine
conflict, or any unproved state. No automatic retry, replay, checkpoint,
cleanup, repair, restore, rollback, schema decrement, or candidate selection is
available. Post-migration sidecar recovery is refused for any non-v5 database,
non-`v5_ready` or contradictory operation, missing/changed backup or migration
contract, missing/changed earlier recovery evidence, nonempty or malformed
WAL/SHM, rollback journal, destination conflict, protected-file drift, active
SQLite coordination lock, or any unproved state. It never generalizes to a
sidecar ignore rule.

Historical-frontend post-commit recovery is refused for an unknown source or
derived schema manifest, any different operation phase/outcome, missing or
contradictory migration fields, failed logical/content/integrity checks,
changed or unverified backup, missing/changed prior prepared recovery
evidence, non-zero staging, unexpected operation/root files, any SQLite
sidecar, receipt conflict, file/claim drift, or any contract state other than
the exact disabled/enabled lifecycle boundary. The existing restore path is
not shown for this classification and is not invoked by it.

## Threat model and automated matrix

| Threat | Response |
| --- | --- |
| Renderer forges paths, SQL, identity, or classification | fixed commands resolve the compiled ordinary app-data path and strictly validate typed responses |
| Personal fingerprint leaks into repository/package | no real digest is embedded; package manifest remains technical-only |
| State changes after review | execution reopens the database under write exclusion and the SQLite coordination-range lock, then recomputes the complete bound claim |
| Database changes between immutable review and lifecycle activation | activation obtains `BEGIN IMMEDIATE` first, repeats the full exact committed-v5 verification on that same connection, and rolls back before mutation on any mismatch |
| WAL contains user state | anything beyond zero-frame proof is refused without checkpoint or removal |
| Alias/link/reparse swaps a selected file | canonical direct-file, single-link, exact-parent, exact-file-set, and post-move digest checks fail closed |
| Recovery consent becomes migration consent | recovery returns restart-required; migration remains the existing separate explicit action |
| Historical stored SQL is treated as arbitrary equivalence | only three fixed source and three fixed derived manifests are accepted; unknown representations fail before migration |
| Post-commit completion is mistaken for retry or restore | UI and backend bind the exact committed receipt/blocked outcome, expose no migration/restore/delete action, and change only lifecycle contract plus operation state |
| Receipt becomes a new personal-data store | strict content-free schema and explicit receipt-only deletion |
| Renderer close control is denied or fails | the main-window capability grants only the explicit close command; the renderer awaits it and reports a localized close-only failure without retracting completed recovery or repeating the action |

The focused Rust suite includes an asserted 30-case prepared-v4 synthetic matrix plus
success, byte-preserving cancellation, TOCTOU, content-free receipt, explicit
receipt deletion, and exact database preservation cases. The post-migration
suite separately proves the exact classifier, write-free inspection/cancel
behavior, protected-file preservation, claim/contract drift refusal,
partial-failure behavior, two-file quarantine, strict receipt shape, and
post-action schema/operation/backup verification. The repository's
existing migration, ambiguous-outcome, old-v4 refusal, typed persistent-WAL
runtime/restart, frontend, i18n, and package suites remain canonical coverage
for the separate second-migration sequence. Automation never resolves or opens
the real profile.

The packaged-window contract separately asserts that the `main` capability
contains exactly one `core:window:allow-close` permission and that generated
capability evidence matches source. A focused frontend port test proves the
desktop close promise is awaited, rejection remains observable, and the
non-Tauri browser fallback stays separate. Panel and i18n tests prove a close
failure is reported independently from recovery failure in all three
languages.

R2B focused automation adds a synthetic historical frontend-created v4
fixture, proves direct migration and a typed schema-v5 write, proves an
unexpected source object fails pre-commit, reconstructs the exact blocked
post-commit state, verifies write-free inspection/cancel, startup routing with
restore unavailable, claim drift refusal, seven protected-file digests,
content-free receipt, lifecycle activation, `v5_ready` finalization, restart
verification, and typed writes. A separate regression changes the schema after
review and proves the write-locked activation recheck refuses it without
changing database bytes or writing a recovery receipt. Frontend tests cover English, Traditional
Chinese and Japanese disclosure and assert that migration, restore and backup
deletion controls are absent from this recovery panel.

## Founder manual matrix

Manual Phase A uses only a disposable Windows account and reproduces the exact
structural state. It verifies hash, cancel-byte identity, exact quarantine,
database and logical-record preservation, restart to migration-required, a new
explicit migration, backup/exact-v5/restart/typed persistence, old-v4 refusal,
and English/Traditional Chinese/Japanese, narrow-window, keyboard, and wrapped
long technical-value behavior inside the recovery disclosure frame.

The Step A12 historical-binary check displayed the expected schema-v5 refusal
and left the live database, valid `v5_ready` operation state, and verified
schema-v4 backup byte-identical. On normal close, however, that installed
historical legacy-v4 executable created a zero-byte WAL plus a matching
32768-byte SHM whose two index copies agreed with `mxFrame`, page count, and
backfill all zero. No rollback journal appeared. This is not evidence of a row,
schema, operation-state, or backup write, but it is a filesystem mutation and
the current v5 startup correctly refuses the resulting sidecar-present state.
The R2 prepared-v4 predicate did not authorize disposition of this
post-migration state. Founder selection of Option A-R3 authorized one bounded
disposable-only correction cycle. The implemented successor is explicit-open
and exact-bound: it may quarantine only the proven empty WAL and matching
zero-frame SHM after revalidating the unchanged database identity/digest,
exact `v5_ready` operation, verified backup, retained prior recovery
receipt/quarantine, file ownership, and TOCTOU claim. Automatic deletion,
ignore-on-open, checkpoint, retry, repair, restore, or migration remains
forbidden. Disposable Manual Phase A reviewed this successor completely before
the Founder accepted the exact 40-path diff on 2026/09/01.

Correction-cycle-3 manual evidence proved the explicit two-file quarantine,
one exact technical-only receipt, unchanged schema-v5 database and protected
operation/backup/prior-recovery evidence, and absent live sidecars after
native close. It also exposed one renderer capability defect: the success
button called Tauri's window close command, but `core:window:default` does not
grant that command and the rejected promise was discarded. The recovery had
already completed correctly; the button itself appeared inert. The bounded
successor grants only `core:window:allow-close`, awaits the command, and shows
a localized close-only error if a future denial occurs. It does not change the
classifier, recovery mutation, receipt, migration, restore, provider, consent,
schema, DDL, or personal-data behavior.

Correction-cycle-4 disposable evidence passed in `LifeOSReviewR1`. The
hash-verified package installation changed no profile file. Normal runtime
reconstructed both disposable records and the managed migration backup. A
separately reconstructed clone then proved that **Preserve and close** closed
the RunAs window without filesystem mutation, while one explicit sidecar action
created exactly one strict content-free receipt and one exact two-file
quarantine. The corrected success control closed the window itself; restart
returned directly to the existing schema-v5 runtime with both records and no
live WAL, SHM, or rollback journal. The completed source profile remained
byte-identical while the clone was exercised, was then restored to the active
ordinary identity, and passed one final restart. Ordinary startup changed only
the live SQLite page digest through the already documented guarded runtime
transactions; operation state, verified backup, staging, both recovery
receipts/quarantines, and all sidecar-absence boundaries remained exact.

This completes disposable Manual Phase A. The Founder separately accepted the
exact repository diff on 2026/09/01; that acceptance does not authorize a
real-profile action or promote the implementation.

After separate Founder authorizations, real-profile Phase B completed the
exact prepared-state recovery and preserved schema-v4 database bytes. One
separately authorized Phase C attempt then created and verified one new
schema-v4 backup, committed schema v5, and stopped without claiming success at
`post_commit_schema_manifest_mismatch`; lifecycle writes remained disabled and
the backup/operation evidence remained retained. Read-only diagnosis identified
one schema-object difference only: the historical frontend multiline
`experience_entries` CREATE TABLE representation. Replaying the unchanged
schema-v5 DDL against the exact historical source representation reproduced
the same 95-object result. No personal row or content was inspected or
reported.

Founder selection of R2B Option B authorizes only the repository correction,
focused/canonical verification, factual documentation/workflow evidence and
one ignored unsigned disposable package. It does not authorize installation,
launch, or any further access to the real `com.lifeos.app` profile. The real
blocked state therefore remains preserved until a future separate Founder
decision after disposable manual review.

Any real post-commit recovery action is a new separate Founder decision. It is
not authorized by the earlier Phase B recovery, the consumed Phase C migration
attempt, diagnosis, implementation, automation, packaging, or this document.

### R2B disposable Founder manual completion — 2026/09/14

The Founder accepted the complete disposable-only R2B manual evidence through
BF8. The bounded matrix established all of the following without accessing the
real profile:

1. exact historical-frontend schema-v4 fixture construction and package
   installation preservation;
2. one intentionally blocked post-commit migration with exact schema-v5
   manifest, verified v4 backup, zero staging and lifecycle writes disabled;
3. prior prepared-recovery evidence completion, three-language disclosure,
   technical-detail containment and keyboard review, followed by a write-free
   Cancel action;
4. a byte-identical blocked-fixture clone and one explicit **Complete exact
   recovery** action that changed only the operational lifecycle contract,
   finalized the bound operation and wrote one content-free receipt;
5. restart into the normal schema-v5 runtime with the managed backup visible;
   and
6. one persisted synthetic schema-v5 Experience with append-only revision 1,
   SQL NULL predecessor, user authorship, no provider metadata and no provider
   transmission.

The final BF8 evidence is hash-bound as follows:

- runner:
  `3c2213fa433af2ed2643f205c69e53c2209b888485c92506ee6f7e852cec6963`;
- before launch:
  `edd4194692e135c9419bfb6be533c8da62270128ee9b29f3051ca73295720151`;
- after close raw:
  `c3cf827f0df68a77e44465376f09a50d1587cc74b20082986b392c3a66204c59`;
- verification:
  `b0cfa0d830beec9557648bbf233f927e904a606e837890f6358c2b4a5dca547c`;
- result:
  `d7ab885c5a2c1d88e994c10c69ce4f6fb7df31ab9e52294a17467027f42a7e99`.

BF8 finished with operation `06c19891cf629f9075f2e349780950c4`
at `v5_ready` / `lifecycle_writes_enabled`; the strict post-commit receipt,
committed migration receipt, verified schema-v4 backup, zero-byte staging and
prepared-recovery receipt/quarantine remained unchanged; all ten retained
profiles and the installed binary were byte-identical; no live SQLite sidecar
or Life OS process remained. The authorized synthetic Experience content digest
is `dcc220682c7fdc3096faee9de5eb3da2255127dda0290901518bb40fe506a6ae`.

This acceptance closes only the disposable R2B Founder manual-review gate. It
does not disposition the real blocked profile and does not authorize any real
recovery or mutation. Repository promotion is a new unresolved decision.

## Android sequencing gate

Android Build Feasibility M0 remains blocked until real-profile recovery is
safely resolved, real migration/restart succeeds or the Founder revises the
fence, R2 is promoted by clean non-fast-forward merge, `develop` equals
`origin/develop`, and canonical verification passes from that clean promoted
state. No Android source, build surface, signing behavior, storage policy, or
provider transport is added here.
