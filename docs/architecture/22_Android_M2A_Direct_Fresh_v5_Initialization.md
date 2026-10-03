---
status: Founder review candidate
version: 1.0
owner: product-and-engineering
last_updated: 2026/10/03
depends:
  - docs/00_Constitution.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/21_Android_M1_Disposable_Persistence_Architecture.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
referenced_by:
  - docs/00_Index.md
  - docs/12_Roadmap.md
  - docs/dev/13_Android_M2A_Direct_Fresh_v5_Runbook.md
---

# 22 Android M2-A Direct Fresh-v5 Initialization

## Status and authority

`ANDROID-M2A-DISPOSABLE-IMPLEMENTATION-001` authorizes this narrow M2-A
synthetic review candidate. It implements and tests direct fresh-v5
initialization under temporary identity `com.lifeos.review.m2a`. It does not
activate the Founder-selected production identity `com.lifeos.app`, real data,
the rest of M2, providers, distribution, or release. Founder review of the
exact diff and manual UI remains required.

ADR-0012 remains Accepted and unchanged. This implementation supplies evidence
for its already-selected direct initialization direction; it creates no new
architecture authority.

## Old scaffold and direct path

M1 intentionally created an empty v4 candidate and invoked the canonical
v4-to-v5 migrator. That exercised migration behavior but was not truthful fresh
installation history. M2-A does not invoke `EMPTY_V4_BASE_SCHEMA`,
`migrate_disposable_v4`, or the migration receipt path.

The direct initializer instead creates one pending database, applies the
canonical final v5 compatibility projection plus `schema_v5.sql` inside one
transaction, activates the required database contract, sets `user_version=5`,
and verifies the committed result. Compatibility projection tables remain
because they are part of the final v5 contract, not because a v4 database ever
existed. A direct database contains zero historical migration receipts.

## Shared and Android-specific responsibilities

Shared, platform-independent responsibilities are:

- canonical final DDL and exact schema-v5 structural verification;
- fixed SQL, domain types, idempotent request semantics, and durable re-read;
- lifecycle eligibility rules for Evidence, Reflection, Pattern, and recovery
  writers;
- strict historical migration verification, unchanged for migrated databases.

Android M2-A owns:

- resolution of the app-private directory with no caller-selected path;
- fixed pending/live database and origin-receipt names;
- process serialization and state inventory before opening or initializing;
- explicit refusal of database `-wal`, `-shm`, and `-journal` sidecars;
- same-directory, exclusive-create-and-sync publication, ordered parent-directory
  durability barriers, and fail-closed partial-publication handling;
- temporary package identity, backup/transfer exclusions, lifecycle wiring,
  and native disposable evidence.

Desktop initialization, migration, backup, recovery, real-profile routing, and
feature activation do not use the direct Android facade and are unchanged.

## Origin receipt and schema invariants

The content-free initialization receipt records:

- `origin = direct_fresh_v5`;
- application ID `com.lifeos.review.m2a` at the Android boundary;
- schema version 5 and pinned final-schema/compatibility hashes;
- stable empty-source and initial-target manifests;
- a fixed receipt schema version.

It contains no Experience text or fabricated source version. The receipt is
not inserted into the historical migration receipt table. Reopen verifies the
external receipt, exact schema structure, database contract, direct origin, and
sidecar-free name inventory before allowing operations. Later content changes
do not rewrite initialization history.

Historical migration verification remains strict and separate. A direct
receipt cannot make a migrated database pass, and a migration receipt cannot
make a direct database pass.

## Creation, publication, and readiness states

1. **Absent:** neither pending nor live names exist; initialization may begin.
2. **Creating:** DDL and activation occur in one transaction on the pending
   database. Failure before commit leaves no authoritative database.
3. **Committed pending:** the pending database and content-free receipt are
   written and independently verified; neither is yet authoritative.
4. **Publishing:** each already-verified pending source is copied into a
   same-directory live file opened with exclusive create, then explicitly
   synced. Android app-private storage refused hard links in native evidence,
   so M2-A does not depend on them. The database live name is populated before
   the receipt live name; interruption preserves the pending sources and every
   partially or fully written live name. After both live files are synced, the
   app syncs their parent directory so both live-name creations are durable
   before retirement begins.
5. **Retiring pending names:** the app removes both pending names and syncs the
   same parent directory again. A failed retirement or directory sync blocks
   the current process; it never permits that run to claim ready.
6. **Ready:** both live names exist, no pending names or SQLite sidecars exist,
   the pending-name retirement crossed its directory durability barrier, and
   direct verification passes. Only this state allows create/list/get.

Any mixed live/pending state is ambiguous and blocks reopen. M2-A never repairs,
renames over, deletes, restores, migrates, or retries an ambiguous state. The
operator must preserve the fixture for review.

The process lock serializes initialization and operations. Exclusive creation
protects against accidental destination replacement; the pending copies make a
partial live copy detectable and preserve the verified source. Publication is
not misrepresented as one atomic two-file rename: authority exists only after
both live files are present, both live-name creations and pending-name removals
have crossed their ordered parent-directory sync barriers, both pending names
are absent, and full direct-v5 verification passes. These facts do not
establish a multi-process Android coordination protocol; production
multi-process use remains unsupported.

## Write acknowledgement

The UI exposes only synthetic Experience create/list/get. A write is
acknowledged only after the direct database has passed readiness checks, the
transaction committed, and the row was re-read through the verified direct-v5
runtime. A stable request ID makes exact retries idempotent; the same ID with
different content fails.

An interrupted transaction, failure before commit, or uncertain commit result
does not authorize optimistic success. Exact durable state determines whether a
retry reports `committed`, `alreadyCommitted`, or fails closed. This is tested
process/SQLite evidence, not a physical-power-loss guarantee.

## Implemented and tested boundary

The candidate covers direct bootstrap without legacy migration invocation,
canonical schema parity, truthful direct-origin receipt, valid direct-v5 reopen,
exact CJK persistence, locale restoration, duplicate requests, concurrent
initialization, transaction rollback, uncertain write commit, publication
interruption, malformed/newer/pending/sidecar preservation, and explicit open
failure. The final freshly prepared Founder profile is checked for absent
pending names, the exact disposable AVD is shut down and restarted without a
host/device-wide sync, and the app must reopen ready with both pending names
still absent. Package contracts check the temporary identity, no network
permission, backup exclusions, accepted icon assets, and M0/M1 evidence
preservation.

The native baseline is one dedicated Android 36 x86_64 AOSP emulator. The local
SDK has an Android 37 platform but no second API/ABI emulator system image, so a
second native configuration is not available without installing toolchains.
Build-only minimum-SDK declarations are not native evidence.

Not proven: physical device behavior, ARM, OEM variants, API 24–35 or API 37,
graceful process shutdown, deterministic raw same-UID SIGKILL, actual physical
power loss, production signing/upgrades, multi-process access, backup/restore,
complete migration/recovery, or portable continuity.

## Product boundary

The trilingual UI stores and reopens exact user-entered synthetic text. It adds
no inference, Evidence extraction, Pattern generation, diagnosis, advice,
identity finalization, historical context, provider traffic, or AI result. The
selected locale remains one app-private, non-content preference. This preserves
**We Build Mirrors, Not Oracles** and does not turn initialization evidence into
a user truth claim.

The debug package and all fixtures are disposable. Do not enter real Life OS
data, import a desktop profile, or infer readiness for production from this
bounded review candidate.
