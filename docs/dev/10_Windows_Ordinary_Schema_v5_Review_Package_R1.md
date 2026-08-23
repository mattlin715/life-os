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
