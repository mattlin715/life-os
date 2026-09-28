---
status: Founder accepted M1 evidence
version: 1.0
owner: product-and-engineering
last_updated: 2026/09/28
depends:
  - docs/00_Constitution.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/20_Android_Build_Feasibility_M0.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
referenced_by:
  - docs/00_Index.md
  - docs/12_Roadmap.md
  - docs/dev/12_Android_M1_Disposable_Persistence_Runbook.md
---

# 21 Android M1 Disposable Persistence Architecture

## Status and authority

This document is the Founder-accepted **disposable M1 evidence package** and
architecture decision record companion. `ANDROID-M1-FOUNDER-REVIEW-004` Option A
accepted the exact unstaged candidate, selected future stable identity
`com.lifeos.app`, and accepted ADR-0012. It is not production authorization or
M2 completion. The temporary Android identity, debug APK, synthetic database,
and test AVD must not receive real user data.

The prototype supplies evidence for Android architecture, storage authority,
and application-identity decisions. It does not authorize production storage,
migration, recovery, release, distribution, cloud synchronization, provider
traffic, or historical-context transmission.

## Recommended architecture

| Decision surface | Recommendation | Current evidence state |
| --- | --- | --- |
| Runtime | Keep Tauri + React with a Rust-owned Android persistence adapter | Prototype implemented and automated/native tested |
| Device authority | One app-private SQLite database is the authoritative local copy for that Android installation | Prototype implemented for synthetic data only |
| Initial schema | New Android installations initialize directly to the exact canonical schema-v5 contract | Prototype implemented; production bootstrap remains M2 |
| Shared logic | Reuse domain types, fixed SQL, schema contracts, transaction semantics, and exact-v5 verification where platform-independent | Prototype reuses the canonical v5 migration/runtime core |
| Android duties | Own sandbox path resolution, temporary publication, lifecycle integration, permissions, backup exclusions, and platform-specific fault evidence | Prototype implemented; production guarantees remain unapproved |
| Final identity | Use stable production ID `com.lifeos.app`; keep `com.lifeos.review.m1` disposable | Founder selected direction; activation remains a later governed phase |
| Backup/transfer | Continue excluding automatic Android backup and device transfer until a governed portable recovery design exists | Founder accepted direction and consequence; continuity design remains later work |

This keeps **We Build Mirrors, Not Oracles** intact: the M1 UI records and
reopens exact user-entered synthetic text. It performs no inference, diagnosis,
Evidence extraction, Pattern generation, identity finalization, or advice.

## Identity options

### Option A — `com.lifeos.app` (recommended)

Use the existing product identifier across supported platforms. This gives the
Android product one stable upgrade and sandbox identity and avoids encoding a
platform fork into the product name. Android and desktop installations remain
separate local authorities; a matching identifier does not imply shared files,
sync, provider consent, or data continuity.

The identity must be selected before real Android data or distribution. Android
treats application ID and signing lineage as installation, update, and private-
data ownership boundaries. Moving data from `com.lifeos.review.m1` is explicitly
out of scope; the review app is disposable.

### Option B — `com.lifeos.android`

This makes platform separation explicit but permanently fragments the product
identity without creating a meaningful privacy boundary. It is viable only if
independent store/distribution or lifecycle policy later requires it.

### Option C — retain `com.lifeos.review.m1`

Rejected for production. It is intentionally temporary, debug-signed, and
synthetic-only. It must never become an accidental stable identity.

## Storage ownership and initialization

The prototype resolves an app-private data directory from the verified runtime
identity and uses fixed names:

- live database: `android-m1-disposable-v5.db`;
- initialization candidate: `.android-m1-fresh-v5.pending.db`.

No caller supplies a path. The adapter serializes initialization and operations
inside the process. If neither path exists, it creates a minimal exact v4 base
inside the pending file, runs the canonical v4-to-v5 transaction, activates
schema-v5 lifecycle writes, verifies exact committed v5, and only then renames
the candidate to the live name. This is a prototype technique for exercising
the canonical schema; production M2 should provide a direct, versioned fresh-v5
bootstrap rather than describing this as a user-data migration.

If the live database already exists, the adapter accepts only an exact verified
v5 runtime. It never falls back to v4 or memory. A pending file, malformed
database, newer schema, ambiguous state, or open failure blocks the app and is
preserved. M1 does not delete, repair, restore, import, migrate, or guess.

The desktop profile and its recovery machinery are never inspected. Reusing
pure schema/domain logic does not copy desktop path, WAL/sidecar, backup,
quiescence, recovery, or real-profile guarantees onto Android.

The disposable Android package reuses the largest exact PNG payload embedded
in the existing PC desktop source `src-tauri/icons/icon.ico`. Its legacy,
round, density-specific, and Android 8+ adaptive launcher assets are generated
derivatives, not a separate brand design. Source and output SHA-256 contracts,
adaptive-resource references, and the manifest round-icon binding fail closed
if regeneration drifts back to the Android/Tauri placeholder artwork.
The adaptive foreground keeps the unchanged canonical image uniformly centered
at less than 60% of each 108-unit density canvas, with transparent padding on
all four sides. This keeps the pale ring inside Android's guaranteed adaptive
safe zone instead of allowing the launcher mask to clip its cardinal edges.

The selected UI locale is the only preference kept outside the M1 database. It
is stored by the Rust facade in the fixed app-private file
`android-m1-locale.pref` as one of `en`, `zh-TW`, or `ja`. This is a
non-content presentation preference: it contains no Experience,
Evidence, Reflection, Pattern, identifier, or provider data. Missing, invalid,
or inaccessible preference storage fails safely to English. Clearing the
temporary app's data also clears this preference.

## Supported contract

The Android facade exposes only:

1. `createExperience`;
2. `listExperiences`;
3. `getExperience`.

It does not expose update, delete, import/export, artifacts, historical context,
backup/restore, migration/recovery, AI, providers, credentials, or sync. This is
not a complete `LocalEvidenceStore` implementation or daily-reflection flow.

Each save receives a stable request ID. The backend validates the request ID
and body, commits through the exact schema-v5 writer, re-reads durable state,
and only then returns `committed`. Retrying the same ID and exact body returns
`alreadyCommitted`; reusing an ID for different content fails. The UI disables
double submission, keeps failed text editable, and preserves the same request
ID for a retry whose outcome may be uncertain.

`committed` proves the SQLite transaction completed and the row was observable
through the verified runtime in the tested process. It does not prove physical
storage survived power loss. A process terminated before commit produced no
row; termination after commit but before acknowledgement preserved the row.

## Backup and user-visible consequence

The APK keeps `allowBackup=false`, Android 12+ data-extraction exclusions, and
legacy full-backup exclusions. It declares no `android.permission.INTERNET`.
This minimizes ungoverned copying but has a direct consequence: uninstalling
the app, clearing its data, or replacing a device can permanently lose the
local database. The prototype offers no restore path.

Production activation therefore requires an explicit user-facing continuity
policy. Governed export/import, encrypted synchronization, recovery, and their
consent/provenance boundaries remain separate work; backup exclusion must not
be presented as equivalent to a complete user-ownership solution.

## Evidence and limits

| Matrix | Result |
| --- | --- |
| Android 36 / x86_64 / dedicated AOSP emulator | Native tested |
| Temporary ID `com.lifeos.review.m1` / debug signing | Packaged and inspected |
| Fresh exact v5; Chinese/Japanese exact text; duplicate request | Passed |
| Background/foreground and force-stop/relaunch | Passed |
| Locale selection across force-stop/relaunch | Passed with app-private non-content preference |
| Launcher/adaptive icon alignment with canonical PC icon | Safe-zone-corrected source/output contract, APK inspection, native matrix, and Founder visual confirmation passed |
| Founder manual UI review | Exact candidate accepted under `ANDROID-M1-FOUNDER-REVIEW-004` Option A; APK SHA-256 `2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975` |
| Force-stop before commit / after commit before acknowledgement | Passed with debug-only synchronization holds |
| Emulator-process termination and restart | Acknowledged row persisted |
| Interrupted schema transaction | Host Rust test passed and remained v4 |
| Concurrent initialization | Host Rust test passed |
| Malformed/newer/open-failure states | Refused and preserved |
| Runtime permission / backup / ABI inspection | Passed; no INTERNET; x86_64 only |
| Graceful process-level shutdown | Unexecuted; no portable Android contract claimed |
| Raw same-UID SIGKILL timing | Unexecuted because the attempted timing was not deterministic |
| Actual physical power loss | Unsupported; no physical device was used |
| API 24–35, API 37, ARM ABIs, OEM devices | Not native tested |
| Production backup/restore, migration/recovery, signing/upgrades | Unsupported and unauthorized |

The manifest declares minimum SDK 24, but only Android 36 x86_64 is native
evidence. Packaging a minimum does not prove that matrix.

## Later phases remain separate

- **M2:** complete governed Android `LocalEvidenceStore`, direct fresh-v5
  bootstrap, migrations, recovery, retention, deletion, export/import, failure
  UX, upgrade compatibility, and broader durability/device evidence.
- **M3:** provider credentials, network capability, per-generation consent,
  exact historical packets, and actual-use provenance.
- **M4:** production signing lineage, distribution, upgrades, device-transfer
  policy, optional encrypted sync, deployment, and release operations.

None of these is implied by M1 tests or by acceptance of this review candidate.
