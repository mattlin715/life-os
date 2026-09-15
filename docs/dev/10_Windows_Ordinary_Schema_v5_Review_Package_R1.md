# Windows Ordinary Schema-v5 Review Package R1

## Boundary

This runbook builds one unsigned, ignored, non-distributed Windows installer
with the real ordinary identity `com.lifeos.app` and the unmistakable window
title `Life OS — Ordinary Schema v5 Review (Disposable Only)`. It is for a
disposable Windows account, VM, or Sandbox only.

The command does not install, launch, stop, uninstall, migrate, restore, or
open any profile. Never run the review installer while relying on a real
ordinary Life OS profile.

## Build

From the repository root:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-ordinary-schema-v5-review.ps1
```

The script:

1. validates version `0.3.0`, ordinary identity, review title, desktop feature,
   frontend gate, shared activation registration, and absence of Android;
2. builds the production custom-protocol Windows GUI binary;
3. verifies embedded identity/title/typed-command markers and PE GUI subsystem;
4. bundles one unsigned current-user NSIS installer;
5. copies only the installer to the ignored
   `.artifacts/desktop-schema-v5-ordinary-review-r1/<git-sha-prefix>/` path;
6. writes and verifies a six-field content-free manifest containing only
   version, Git SHA, target, artifact filename, size, and SHA-256;
7. does not install or launch the result.

An existing exact output directory fails closed. A deliberate rebuild requires
a new bounded suffix or explicit deletion of only that ignored review output.

## Manual Phase A

Use the exact installer and manifest reported by the build. In a disposable
Windows environment, first establish a small ordinary schema-v4 dataset, close
all Life OS processes, install the review build, cancel once, then explicitly
migrate. Review backup, restart, typed daily-reflection, Context Recovery,
deletion/invalidation, old-binary refusal, disposable restore, fresh-v5,
three-language, keyboard, narrow-window, uninstall, and retained-data behavior.

Do not infer permission to use `%APPDATA%\com.lifeos.app` on the Founder's real
Windows profile. After Phase A passes, that action remains behind a separate
exact Founder authorization. If authorized later, retain the verified backup
for 30 days and do not test real-data restore or delete-now.

### 2026/08/23 completion evidence

The Founder completed Manual Phase A in the disposable `LifeOSReviewR1`
account. The reviewed sequence covered corrected disclosure, write-free
cancel, explicit v4-to-v5 migration, verified backup, restart, typed daily
reflection, Context Recovery answer/skip, edit/delete consequences, old-v4
write refusal, explicit disposable restore, fresh-v5 initialization,
three-language and accessibility behavior, absent release surfaces, and
uninstall with application-data deletion left unselected.

Two ignored successor packages were required by bounded evidence-based
corrections: one ordinary-only disclosure correction and one Reflection
duplicate-submit correction. Their manifests remained content-free and their
hashes were checked before installation. No package artifact, manifest, user
profile, or database is repository content.

This completion is disposable-manually-verified only. It grants no permission
to inspect or migrate the Founder's real ordinary profile and does not imply
promotion, distribution, deployment, or release.

## Persistent-WAL correction review

The later separately authorized Phase B attempt stopped before backup or
migration because an exact-v4 source retained persistent WAL header bytes. A
normal read-only verifier open created WAL/SHM sidecars; the existing final
guard then failed closed. The real database remained byte-identical schema v4,
and its recovery evidence must not be cleaned up, checkpointed, retried, or
otherwise changed during this review.

The correction package is again unsigned, ignored, and disposable-only. It
must be reviewed in the `LifeOSReviewR1` account (or a newly isolated disposable
environment), never in the Founder's real Windows account. Before installing
it, establish an exact-v4 test profile, close Life OS, and verify that the main
database header records WAL mode while `-wal`, `-shm`, and `-journal` are all
absent. The bounded manual path then verifies:

1. opening the corrected review build shows the ordinary migration disclosure;
2. cancellation creates no operation evidence and changes no database bytes;
3. one explicit migration creates a verified schema-v4 backup and exact v5;
4. the source inspection itself does not create WAL/SHM sidecars; and
5. restart reconstructs the migrated disposable dataset.

The first persistent-WAL package reached exact v5 with a verified v4 backup,
then the normal runtime created an empty WAL/SHM pair and the next typed write
failed closed with `sqlite_sidecar_present`. After normal close, the disposable
database remained exact v5 with one owned operation and one verified v4 backup;
no retry, repair, restore, checkpoint, or cleanup was performed. Preserve that
failed disposable profile as forensic evidence.

The successor runtime correction keeps the same sidecar refusal and migration
policy. It makes stable runtime reads immutable and requires every bounded
schema-v5 writer to close its connection before durable verification. Review
the successor in a fresh clone of the untouched disposable exact-v4 fixture,
not by retrying the failed v5 profile. After explicit migration, verify all of
the following before restart:

1. no `sqlite_sidecar_present` storage error is shown;
2. the migrated record is reconstructed through the typed runtime;
3. normal application close leaves no WAL, SHM, or rollback-journal file;
4. one new typed Experience write succeeds and persists; and
5. a second normal close and restart also leave no sidecars.

If any sidecar is present before the explicit action, stop rather than
checkpointing, deleting, or repairing it. This review grants no authority to
touch the real profile or to retry its Phase B operation.

### 2026/08/23 successor runtime review evidence

The Founder completed the successor package review in `LifeOSReviewR1`. The
failed disposable v5 profile was preserved first and was not retried. The fresh
active fixture was exact schema v4 with persistent-WAL header bytes `2/2`, no
operation evidence, and no sidecars. Installing the hash-verified successor
package changed neither its bytes nor its schema.

One explicit UI migration reached `v5_ready` with
`lifecycle_writes_enabled`, retained one verified schema-v4 backup, and left a
zero-byte staging file. Normal close left no WAL, SHM, or rollback journal.
Restart reconstructed the original migrated record without
`sqlite_sidecar_present`, recovery-required, or manifest-mismatch disclosure.
One new typed Experience write then succeeded; its durable write changed only
the live v5 database, left the verified backup unchanged, and left no sidecars
after close. A final restart reconstructed both records and the application was
closed normally. This is disposable manual evidence only; the retained real
Phase B failure evidence was not accessed or changed.

## Evidence handling

Do not paste personal content, full paths, database metadata, credentials, or
machine identity into the manifest or sprint artifacts. Record only bounded
classifications, counts where necessary, package hash, and the Founder's
step-level pass/fail observations.

## R2 prepared-state recovery review

Use the R2 unsigned package only in a disposable Windows account. Never point
scripts or the package at the Founder profile during automated or disposable
review. Recreate exact schema v4, one exact `prepared` operation, zero-byte
staging, no backup, zero-byte/header-only WAL, matching zero-frame SHM, and no
rollback journal. Verify the installer SHA-256 before installation.

Review one bounded step at a time:

1. open the recovery disclosure and compare the displayed database file
   identity/SHA-256 with the disposable fixture record;
2. choose Cancel, close, and prove database, operation, staging, WAL, and SHM
   bytes unchanged;
3. reopen, choose Prepare recovery once, and verify only the four exact
   transient files moved to the exact-owned quarantine;
4. verify the database hash/schema/logical record set is unchanged and the
   receipt contains only classification, relative filenames, sizes, digests,
   versions, operation ID, and timestamp;
5. restart and verify the separate ordinary migration disclosure;
6. explicitly authorize one new migration, then verify the new backup, exact
   v5, typed write, normal close, and restart;
7. verify the legacy build refuses the v5 database without writing;
8. review English, Traditional Chinese, Japanese, narrow-window layout, and
   keyboard focus.

### R2 Manual Phase A correction cycle 2

The first packaged technical-details review displayed the exact eligible
classification, operation identifier, database digest/identity, claim digest,
and four evidence facts, but long unbroken paths and SHA-256 values overflowed
the disclosure background. The bounded successor makes only the recovery card
single-column and wraps long technical values within the frame. It adds no
recovery, migration, schema, data, provider, consent, or receipt behavior.

The ignored successor review installer is named
`Life-OS-Ordinary-Schema-v5-Review-R1-Review-prepared-recovery-r2-layout-cycle2_0.3.0_x86_64-pc-windows-msvc-setup.exe`
and has SHA-256
`1E1E53D9C2213E34E5365E5FC73914024FF0D314D922B3D6278266D914743464`.
Before installation, close the prior review application and prove the exact
prepared fixture remains byte-identical. Install without application-data
deletion, reopen the read-only review, expand technical details, and verify all
long values remain visibly contained before continuing the original cancel and
explicit-recovery matrix.

### R2 Manual Phase A evidence through Step A11-4

The Founder installed the correction-cycle successor only in the disposable
`LifeOSReviewR1` account and confirmed that the exact prepared fixture remained
unchanged. The packaged technical-detail retest passed: long paths, file
identity values and SHA-256 digests remained visibly contained inside the
recovery disclosure frame.

The subsequent disposable sequence proved write-free cancellation, one
explicit recovery, byte-identical schema-v4 database preservation, removal of
only the approved operation/sidecars, exact four-file quarantine, and one
strict content-free receipt without root-path disclosure. Restart showed the
ordinary migration disclosure separately and did not reuse recovery consent.

One separately authorized disposable migration then reached exact schema v5
with a verified schema-v4 backup and complete migration evidence. Normal close
left no WAL, SHM, or rollback journal. Restart reconstructed the original
record, one new typed Experience write persisted, the live v5 digest changed
while the backup digest remained unchanged, and a second restart reconstructed
both records. The historical legacy-v4 refusal check then produced the finding
below; final English, Traditional Chinese, Japanese, narrow-window, and
keyboard-focus review remains pending. This evidence grants no real-profile
authority.

### R2 Step A12 historical legacy finding

The exact installed historical legacy-v4 executable displayed the expected
schema-version refusal against the disposable exact-v5 profile. Closed-file
evidence proved the live database digest/schema/header, valid `v5_ready`
operation state, and verified schema-v4 backup digest remained unchanged. It
also proved that the historical process created a zero-byte WAL and a matching
32768-byte zero-frame SHM (`mxFrame = 0`, page count `0`, backfill `0`); no
rollback journal appeared.

Do not treat this as a complete write-free pass. The database and durable
migration evidence were unchanged, but the filesystem was mutated and the
current v5 build truthfully refuses the resulting sidecar-present state. Keep
the disposable evidence closed and untouched until using the exact successor
below. Do not manually delete, checkpoint, copy, move, repair, restore, retry,
or migrate it.

### R2 Manual Phase A correction cycle 3

The Founder selected Option A-R3 for one disposable-only successor. The new
path is not an ignore-on-open rule and is not general cleanup. It recognizes
only classification `exact_v5_ready_legacy_empty_sidecar_v1`: exact schema v5,
one valid `v5_ready` / `lifecycle_writes_enabled` operation, one unchanged
verified schema-v4 backup, one valid earlier prepared-state recovery
receipt/quarantine, zero-byte WAL, matching 32768-byte zero-frame SHM, no
rollback journal, and no conflicting destination.

Opening the review remains write-free. Only a separate explicit action may
move the exact WAL and SHM into a new exact-owned quarantine and create one
strict content-free receipt. The live v5 database, current operation state,
zero-byte staging, verified backup, earlier receipt/quarantine, and personal
records are claim-bound and must remain byte-identical. Cancel and Preserve
and close make no filesystem changes. Any mismatch stops without automatic
cleanup, checkpoint, retry, repair, restore, migration, or success claim.

The canonically verified ignored successor was built without automatic
installation or launch:

- installer: `Life-OS-Ordinary-Schema-v5-Review-R1-Review-prepared-recovery-r2-v5-sidecar-cycle3_0.3.0_x86_64-pc-windows-msvc-setup.exe`
- size: `5768727` bytes
- SHA-256: `97918E1B7EB7207684448D7D414AABEFDF229E14572B55C0E4C66EB34934D067`
- build target: `x86_64-pc-windows-msvc`
- source HEAD: `8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899`
- ignored package directory: `.artifacts/desktop-schema-v5-ordinary-review-r1/8173d4fe76ee-prepared-recovery-r2-v5-sidecar-cycle3/`

The adjacent six-field `manifest.json` passed the repository package verifier.
Resume one bounded step at a time only in `LifeOSReviewR1`:

1. with Life OS closed, verify the installer hash and re-prove the exact
   disposable A12 database, operation, backup, prior receipt/quarantine,
   zero-byte WAL, zero-frame SHM, and absent rollback journal;
2. install without application-data deletion and prove installation changed
   none of those bytes;
3. launch, confirm the ordinary fail-closed screen, explicitly open the
   bounded recovery review, and inspect its classification, two moved files,
   and protected-file facts;
4. review English, Traditional Chinese, and Japanese copy, long-value wrapping,
   narrow-window containment, and keyboard focus; then choose Cancel, close,
   and prove every fixture byte unchanged;
5. reopen, inspect again, invoke **Quarantine the exact sidecars** once, and
   close normally when instructed;
6. prove the live database, current operation, verified backup, earlier
   receipt/quarantine, and personal records are unchanged; prove only WAL/SHM
   moved; verify the new receipt's strict technical-only schema and the exact
   two-file quarantine;
7. restart and confirm the existing two disposable records are reconstructed,
   no recovery or migration consent is reused, and normal close leaves no live
   SQLite sidecars.

Stop immediately on any discrepancy and preserve all evidence. This cycle
does not authorize the real Founder profile, Phase B, Phase C, promotion,
staging, commit, push, merge, PR, deployment, distribution, release, Phase 4,
or Android.

### R2 Manual Phase A correction cycle 4

Correction cycle 3 completed the exact sidecar action before the close-control
finding. Closed-file evidence proved one strict content-free receipt, one
exact two-file quarantine, unchanged schema-v5 database, unchanged current
operation/backup/staging, unchanged earlier prepared-state receipt/quarantine,
and absent live WAL, SHM, and rollback journal. No recovery action may be
repeated.

The success button then appeared inert. Source and generated Tauri capability
evidence showed why: the renderer awaited no result and the main-window
capability did not include `core:window:allow-close`; `core:window:default`
does not grant that command. The Founder used the native window X once and the
process closed normally. This is a close-control defect, not a recovery,
receipt, database, migration, or consent failure.

The bounded successor:

1. grants only `core:window:allow-close` to the existing `main` window;
2. awaits the desktop close promise instead of discarding it;
3. reports a localized close-only error without retracting completed recovery
   or inviting a repeated action; and
4. adds focused capability-source/generated parity, close-port, panel, and
   three-language tests.

It changes no Rust, recovery classifier or mutation, schema, DDL, operation,
backup/restore, provider, ContextPacket, consent, Phase 4, Android, or runtime
storage behavior. Canonical verification passed with workflow 17/17, ordinary
package contracts 3/3, Vitest 369/369, TypeScript/build, Rust 216/216 plus
backup 12/12 and schema contract 8/8, and all compatibility/safety checks. The
new ignored unsigned installer is
`Life-OS-Ordinary-Schema-v5-Review-R1-Review-prepared-recovery-r2-close-control-cycle4_0.3.0_x86_64-pc-windows-msvc-setup.exe`,
size `5779951`, SHA-256
`DA357971D79262495789A20D4441B4F16BB805F8F4EDC41626E9D747D7ED6F1B`.
It was installed and launched only in `LifeOSReviewR1`; it was not distributed,
deployed, or released.

The correction-cycle-4 retest passed. Installation preserved all twelve
profile files. Normal startup reconstructed the two disposable records and
managed backup. To avoid repeating the completed recovery, the completed
profile was preserved byte-identically and one separate clone was reconstructed
with only the exact historical zero-byte WAL and matching zero-frame SHM. On
that clone, **Preserve and close** closed the window itself and changed no
fixture byte. One later explicit recovery moved exactly those two sidecars,
wrote one strict content-free receipt whose protected and quarantined facts all
matched, and left database, operation, backup, staging, earlier receipt and
earlier quarantine unchanged. The corrected success close control then closed
the RunAs window itself with process count zero.

Restart reconstructed both records with no reused migration consent and left
no live WAL, SHM, or rollback journal. The exercised clone was retained as
independent evidence. The original completed profile was restored to the active
ordinary identity, reverified, restarted once, and again reconstructed both
records and the same managed backup. Its final close left all protected
evidence exact and no live sidecars. Live SQLite page digests changed during
ordinary startup through already documented guarded runtime transactions; this
was not treated as protected-evidence drift.

### R2 Manual Phase A completion

Disposable Manual Phase A is complete through correction cycle 4. It proves
the exact prepared-v4 and post-migration legacy-empty-sidecar classifications,
write-free cancellation/preserve-close behavior, exact quarantines, strict
content-free receipts, recovery/migration consent separation, restart
reconstruction, close-control authority, three-language disclosure, narrow
layout, keyboard focus, typed persistence, old-v4 refusal, and preserved
verified backups. No real Founder profile was accessed or inspected.

The Founder accepted the exact 40-path repository diff and completed disposable
Manual Phase A evidence on 2026/09/01. That acceptance closed only the R2
manual/diff gate. Phase B, Phase C, promotion, staging, commit, push, merge,
PR, deployment, distribution, release, Phase 4, and Android remain
unauthorized.

Phase A grants no real-profile authority. Before Phase B ask exactly:

> “Do you authorize this exact reviewed build to disposition only the proven
> pre-write prepared operation, zero-byte staging file, and proven-empty WAL/SHM
> evidence in the real `com.lifeos.app` profile, while preserving the schema-v4
> database byte-identically?”

If explicitly authorized, perform content-free preflight, show the complete
eligibility result, require the UI action, execute once without retry, verify
the database bytes unchanged, and stop on any mismatch.

Phase B success grants no migration authority. Before Phase C ask exactly:

> “Do you authorize one new schema-v4-to-v5 migration of the recovered real
> `com.lifeos.app` profile using this exact reviewed build, with a newly verified
> schema-v4 backup retained for 30 days?”

If explicitly authorized, perform one action without automatic retry, verify
exact v5 and restart, let the Founder inspect existing records, complete one
bounded daily reflection, and retain the backup. Never test restore or
delete-now against real data.

## R2B historical-frontend post-commit correction review

The separately authorized real Phase B recovery completed with schema-v4
database bytes unchanged. The one separately authorized Phase C migration
attempt created and verified one schema-v4 backup, committed schema v5, then
stopped fail closed at `post_commit_schema_manifest_mismatch` before lifecycle
writes were enabled. Read-only diagnosis proved the migration result differed
from the accepted runtime-v4 result in only the stored multiline SQL bytes for
the historical frontend-created `experience_entries` table; the object set,
columns, constraints, source/target logical manifests and unchanged DDL replay
were otherwise exact. No personal row or content was inspected.

The R2B successor uses classification
`exact_historical_frontend_v4_post_commit_manifest_v1`. It:

1. binds the source migration preflight to three exact schema-v4 object
   manifests and rejects unknown representations before migration;
2. accepts the three corresponding exact full schema-v5 manifests without
   generic SQL normalization;
3. exposes recovery only for the exact blocked phase/outcome, committed
   receipt, historical derived manifest, disabled/already-enabled lifecycle
   boundary, verified backup, zero staging, prior prepared receipt/quarantine,
   absent sidecars and unchanged claim;
4. performs no DDL, migration, retry, restore, repair, checkpoint, replacement,
   schema decrement, backup deletion or personal-content transformation;
5. after one explicit action, obtains `BEGIN IMMEDIATE`, repeats the full exact
   committed-v5 verification on that same connection, then changes only
   lifecycle writes from disabled to enabled; any mismatch rolls back before
   mutation; it writes one strict content-free receipt, finalizes the operation
   as `v5_ready` / `lifecycle_writes_enabled`, and requires close/restart; and
6. shows no migration, restore or backup-deletion control in this recovery
   panel.

The ignored unsigned package must be built with a unique R2B suffix and its
six-field manifest recorded below. Building does not authorize installation or
launch. Disposable Founder review remains a later explicit gate.

### R2B repository package evidence — 2026/09/02

- ignored directory:
  `.artifacts/desktop-schema-v5-ordinary-review-r1/8173d4fe76ee-r2b-post-commit-manifest-lock/`
- installer:
  `Life-OS-Ordinary-Schema-v5-Review-R1-Review-r2b-post-commit-manifest-lock_0.3.0_x86_64-pc-windows-msvc-setup.exe`
- size: `5805550` bytes
- SHA-256:
  `30ec0a075f79728b6c87a2f9ba9c5ed3ec1d7188d85b96650fc23cd8715ade9b`
- Git SHA: `8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899`
- target: `x86_64-pc-windows-msvc`

The adjacent strict six-field `manifest.json` passed the repository verifier,
and a separate closed-file size/SHA-256 comparison matched it. This is the one
package created by the R2B correction cycle. It remains ignored, unsigned,
uninstalled, unlaunched, undistributed, undeployed and unreleased.

For that later review, use only a disposable Windows account and synthetic
content. First use the historical frontend initializer plus the earlier
cycle-4 verifier to construct the exact blocked post-commit fixture; never
copy or point at the real `com.lifeos.app` profile. With all applications
closed, record the disposable database/state/backup/prior-evidence file facts,
then proceed one bounded step at a time:

1. verify the new installer hash and prove installation changes none of the
   disposable fixture bytes;
2. launch, confirm the fail-closed screen, explicitly open the bounded recovery
   review, and inspect the exact classification, operation, database identity,
   claim, one controlled state fact and seven preserved facts;
3. review English, Traditional Chinese and Japanese disclosure, long-value
   wrapping, narrow-window containment and keyboard focus; confirm there is no
   migration, restore or backup-delete action;
4. choose Cancel, close, and prove every fixture byte unchanged;
5. restore/reconstruct a separate byte-identical disposable clone, reopen,
   invoke **Complete exact recovery** once, and close when instructed;
6. prove the verified backup, zero staging, earlier receipt/quarantine and
   synthetic personal records are unchanged; verify the strict content-free
   receipt, exact lifecycle-enabled database contract, and exact `v5_ready`
   operation state;
7. restart and confirm synthetic records reconstruct, one new typed write
   persists, the backup remains unchanged, and normal close leaves no WAL, SHM
   or rollback journal.

Stop on the first discrepancy. Do not retry or use the real profile. This
manual gate does not authorize real-profile recovery, restore, repair,
checkpoint, schema decrement, backup deletion, migration, promotion, staging,
commit, push, merge, PR, deployment, distribution, release, Phase 4 or
Android.

### R2B disposable Founder manual result — 2026/09/14

The complete BF1-BF8 disposable matrix passed and was Founder-accepted. The
review covered the bounded post-commit recovery disclosure and Cancel path,
technical details in English, Traditional Chinese and Japanese, an exact
separate recovery completion on a byte-identical clone, post-recovery restart,
managed-backup reconstruction, and one persisted synthetic schema-v5
Experience. The final BF8 evidence hashes are:

- runner: `3c2213fa433af2ed2643f205c69e53c2209b888485c92506ee6f7e852cec6963`;
- before-launch: `edd4194692e135c9419bfb6be533c8da62270128ee9b29f3051ca73295720151`;
- after-close-raw: `c3cf827f0df68a77e44465376f09a50d1587cc74b20082986b392c3a66204c59`;
- verification: `b0cfa0d830beec9557648bbf233f927e904a606e837890f6358c2b4a5dca547c`;
- result: `d7ab885c5a2c1d88e994c10c69ce4f6fb7df31ab9e52294a17467027f42a7e99`.

The final disposable state was exact schema v5 with operation `v5_ready`,
lifecycle writes enabled, one unchanged verified schema-v4 backup, unchanged
migration and recovery evidence, no live SQLite sidecars, and exactly one
authorized synthetic Experience. No provider transmission occurred. This
result closes only the disposable manual-review gate; the real blocked profile,
repository promotion and Android sequencing remain separate and unresolved.
