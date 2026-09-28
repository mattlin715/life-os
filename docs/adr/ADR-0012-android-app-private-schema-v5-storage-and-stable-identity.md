---
status: Accepted
version: 1.0
owner: product-and-engineering
last_updated: 2026/09/28
depends:
  - docs/00_Constitution.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/21_Android_M1_Disposable_Persistence_Architecture.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
referenced_by:
  - docs/00_Index.md
---

# ADR-0012: Use App-Private Schema-v5 Storage and a Stable Android Identity

## Status

Accepted by the Founder under `ANDROID-M1-FOUNDER-REVIEW-004` Option A on
2026/09/28. Acceptance selects the architecture direction and stable production
identity; it does not activate production storage, real data, M2, distribution,
or release.

## Context

Android M0 proved that the Tauri/React shell can be built and reviewed without
creating product data. M1 must decide the intended mobile runtime, local storage
authority, fresh-schema direction, and stable application identity without
silently beginning the larger M2 persistence, migration, and recovery program.

On Android, the application ID and signing lineage govern installation,
upgrades, and ownership of app-private files. Automatic Android backup and
device transfer could copy sensitive local data outside an explicit Life OS
continuity design. Desktop-specific paths, backup operations, sidecar rules,
and recovery state machines do not automatically apply to Android.

## Decision

Android production direction will be:

1. retain the Tauri + React application with a Rust-owned storage boundary;
2. use stable production application ID `com.lifeos.app` rather than the
   temporary `com.lifeos.review.m1` review identity;
3. treat one SQLite database in the Android app-private sandbox as the local
   authoritative copy for that installation;
4. initialize new Android installations directly to the exact canonical
   schema-v5 contract, with no v4 or memory fallback;
5. reuse platform-independent domain, schema, fixed-SQL, transaction,
   provenance, and verification logic while implementing Android-specific path,
   publication, lifecycle, permission, and fault semantics separately;
6. acknowledge a write only after its transaction has committed and the result
   can be read through the verified schema-v5 runtime, with idempotent request
   identity for uncertain retries;
7. fail closed and preserve malformed, newer, incomplete, or ambiguous state;
   repair, restore, deletion, and migration require separately governed M2
   authority;
8. keep automatic backup and device transfer excluded until a governed,
   user-visible continuity design is accepted.

The matching desktop and Android identifier names one product; it does not make
their databases shared, designate desktop or Android as a cross-device master,
or authorize sync. Each installation remains independently local until a later
governed synchronization decision.

## Consequences

- Real Android data must never be written under the disposable review ID.
- Choosing another production ID later would break the ordinary update/sandbox
  lineage and require an explicit continuity plan.
- App-private storage prevents casual external file access but is not encryption
  and does not replace device security or a future threat-model review.
- With backup/transfer excluded and no approved export/sync recovery, clearing
  app data, uninstalling, or losing the device can permanently lose data. The
  product must disclose this before production use.
- M2 must implement the complete supported store contract, direct bootstrap,
  version upgrades, recovery, retention, deletion, export/import, and a broader
  device/API/ABI durability matrix.
- M3 remains the only place to authorize providers, credentials, network use,
  or historical-context transmission.
- M4 remains responsible for signing, distribution, upgrade operations,
  optional sync, deployment, and release.

## Alternatives

### Use `com.lifeos.android`

Viable if platform-specific store or lifecycle constraints require a distinct
identity, but it fragments product identity without itself improving privacy or
storage isolation.

### Keep `com.lifeos.review.m1`

Rejected for production because it is temporary, debug-signed, synthetic-only,
and intentionally disposable.

### Use externally selectable files or a shared desktop database

Rejected. Caller-selected paths and desktop-profile access weaken the Android
sandbox boundary and import desktop recovery assumptions without evidence.

### Enable Android Auto Backup now

Rejected until retention, encryption, restore compatibility, device-transfer
semantics, and user disclosure are governed. Convenience does not substitute
for explicit ownership and recovery design.

### Start at schema v4 and migrate immediately

Rejected for a new installation. It creates unnecessary downgrade and migration
states. The M1 prototype's v4-to-v5 candidate is test scaffolding to exercise
the current canonical schema, not the recommended production bootstrap.

## Non-authority

This Accepted ADR selects architecture direction only. It does not accept the
current debug APK for production, activate real-data persistence, authorize M2
implementation, permit migration or recovery, allow provider traffic, or
approve signing/distribution/release. Founder manual acceptance of the exact
disposable review candidate is recorded separately under the same decision.
