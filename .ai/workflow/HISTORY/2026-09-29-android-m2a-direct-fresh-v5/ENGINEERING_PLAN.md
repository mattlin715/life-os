# Engineering Plan

Status: approved

- Sprint ID: 2026-09-29-android-m2a-direct-fresh-v5
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 51dff4998ca8696aeaf1527f058d785303fc0cd4
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-29T09:18:00Z
- Updated at: 2026-10-02T16:12:00Z

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement only synthetic Android M2-A direct fresh-v5 bootstrap and the preserved M1-equivalent create/list/get review flow. Conditions: separate fresh-origin from historical migration evidence; preserve strict desktop/migration behavior; preserve partial states; use temporary identity/AVD/artifacts; keep backup/network exclusions; stop with all changes unstaged at Founder diff/manual UI review.

## Existing Implementation Understanding

- `src-tauri/src/android_m1.rs` owns the package-private path, in-process lock, locale preference, create/list/get commands, pending file, M1 scaffold, and debug commit holds.
- Its fresh path executes `EMPTY_V4_BASE_SCHEMA`, obtains a v4 manifest, calls `migrate_disposable_v4`, activates writes, verifies a committed migration receipt, then renames pending to live.
- `src-tauri/schema/schema_v5.sql` is the fixed canonical v5 DDL but intentionally assumes six v4/current-state compatibility projection objects and their indexes/triggers exist.
- `schema_v5_experience_write.rs` validates exact schema, one strict 4-to-5 migration receipt, lifecycle contract, projections, current content, and integrity around every transaction.
- `schema_v5_runtime.rs` list/get/create routes currently use the migration-origin verifier.
- Current generated Android project is the promoted M1 review surface with accepted icon/backup/safe-area resources. M1’s Git commits and workflow archive are immutable historical evidence.
- Locally installed native system-image coverage is only Android 36 default x86_64. Android platform 37 is installed but no API 37 emulator image/second ABI image is available, so only a bounded build/config inspection—not a second native configuration—is feasible without installation.

## Affected Modules

- Canonical schema/bootstrap: `src-tauri/schema/`, `src-tauri/src/schema_v5_migration.rs`, `schema_v5_experience_write.rs`, `schema_v5_runtime.rs`, new direct-init support, and removal of the Android dependency on `schema_v5_fresh_base.rs`.
- Android backend: new `src-tauri/src/android_m2a.rs`, `src-tauri/src/lib.rs`, configuration/package identity.
- Android frontend: new `src/android-m2a/`, `src/main.tsx`, `vite.config.ts`.
- Generated package: exact identity/activity/Gradle changes under `src-tauri/gen/android/`; icon, safe-area, backup, transfer, and permission resources remain byte/meaning identical.
- Automation: new `scripts/android-m2a*.{ps1,mjs}`, package scripts, contract/native checks, and canonical verifier wiring.
- Documentation/workflow: new architecture 22 and runbook 13; `docs/00_Index.md`, `docs/12_Roadmap.md`, current workflow artifacts.

## Proposed Design

1. Move the exact compatibility projection DDL from the Rust empty-v4 string into a canonical SQL include that contains no `PRAGMA user_version=4`. Treat it as the compatibility portion of the final v5 contract, not an initialization stage.
2. Add a direct initializer that creates a new pending SQLite file, begins one transaction, executes canonical compatibility DDL and canonical v5 DDL, inserts the guarded `database_contract` directly with lifecycle writes enabled, verifies empty canonical projections, sets `user_version=5`, commits, closes, and verifies exact schema. It never calls migration code and inserts zero migration receipts.
3. Add a content-free `DirectFreshV5Receipt` with a direct-origin format, temporary application ID, schema version, canonical DDL/schema-object digests, initialized timestamp, and deterministic receipt ID. Store it as a separate app-private JSON file so canonical schema and Accepted migration receipt meaning remain unchanged.
4. Keep the existing migration verifier exact. Add separate direct-origin schema verification and a direct-origin experience writer/runtime entry point. Existing runtime APIs continue to require the historical migration receipt; only the M2-A facade may select the direct route after verifying the app-private receipt.
5. Publish each verified pending source into a same-directory live file opened with exclusive create, copy all bytes, and sync it before proceeding: pending database -> live database -> pending receipt -> live receipt. Sync the app-private parent directory after both live names and their file contents are durable, then retire both pending names and sync that parent directory again before claiming ready. Android native evidence showed that app-private storage refuses hard-link creation, so the implementation does not depend on hard links. Any partial live/pending combination, directory-sync failure, or database `-wal`, `-shm`, or `-journal` blocks and is preserved. A state is write-eligible only when both live files exist, no pending or sidecars exist, both direct receipt and database verify, and the successful pending-name retirement has crossed the directory durability barrier.
6. Retain one process lock over readiness and each operation. Recheck exact state immediately before publication; exclusive destination creation provides no-overwrite behavior, while preserved pending sources make partial copies detectable and recoverable only through a later governed decision. Publication is not claimed as a single atomic two-file rename. No automatic retry, deletion, repair, restore, or cleanup occurs after an ambiguous failure.
7. Copy the accepted M1 frontend flow into a narrowly renamed M2-A entry, reuse its styling/icon/assets, update only identity/direct-bootstrap disclosures, and keep locale preference as bounded non-content app-private state.
8. Use `com.lifeos.review.m2a`, a new database/receipt namespace, artifact root, AVD `lifeos_m2a_api36_x86_64`, serial/port distinct from M1, and exact adb binding. Assert the M2-A package is absent before first install and never issue uninstall/clear for M0/M1 identities.

## Alternatives Considered

- Insert a fake 4-to-5 row in `schema_migration_receipts`: rejected as fabricated history.
- Change the canonical table to accept an initialization row: rejected for M2-A because it changes the Accepted canonical schema/migration contract when an Android-specific content-free receipt can preserve truthful origin.
- Let generic desktop runtime accept zero migration receipts: rejected because it weakens migration verification and broadens authority.
- Publish with `rename` after a precheck: rejected because rename may replace an existing destination on Android/Linux. Same-directory exclusive creation is no-overwrite; partial copy states retain their pending sources, remain explicit, and fail closed.
- Reuse or clear the M1 AVD/package: rejected by authorization and evidence-preservation constraints.

## Data Lifecycle Impact

Synthetic Experience create/list/get only. New records use canonical source revision, provenance, lifecycle, and v4 compatibility projection writes. The direct initialization receipt contains no Experience or user content. No update/delete/retention/export/import behavior is added.

## SQLite Or Migration Impact

Adds a direct creation path for a new empty exact-v5 database and separate origin verifier. Canonical final objects and schema manifests must match the accepted contract. Historical v2/v3/v4 initialization, migration, backup, recovery, and receipt validation remain behaviorally unchanged and are regression-tested.

## Provenance Impact

Direct initialization evidence is content-free and truthfully labeled fresh. Synthetic Experience creation continues to record user provenance through the existing canonical v5 writer. No AI provenance is created.

## Historical Context Impact

None. Compatibility/historical tables are created empty because they are part of the canonical final schema. No history is read, fabricated, assembled, or transmitted.

## Consent Impact

None. No consent event or provider transmission is allowed.

## Provider Transmission Impact

None. Bundle and APK contracts must retain no provider endpoint/credential surface and no Android INTERNET permission.

## Import And Export Impact

None. Both remain unsupported and unexposed.

## Test Strategy

- Rust direct-init tests: no migration invocation/receipt; exact schema parity; truthful receipt; valid reopen; DDL/precommit/postcommit faults; commit known/unknown classification; existing/pending/sidecar refusal; no overwrite; concurrent initialization; publication partial states; receipt tampering; malformed/newer database preservation.
- Rust experience tests: direct-origin create/list/get, duplicate request, exact CJK, first acknowledgement boundaries, and strict migration-origin regression.
- Frontend: preserved trilingual flow, copy/identity, locale restoration contract, loading/blocked/saving/duplicate behavior.
- Node contract: exact generated allowlist, temporary ID, activity/safe area/icon parity, backup/transfer exclusions, no permissions/network/provider/desktop paths, direct-only symbols, M1 evidence/archive preservation.
- APK: identity, signature, ABI, permissions, backup, cleartext, icon, native library, and digest.
- Native: Android 36 x86_64 dedicated AVD; fresh/direct receipt/schema inspection; CJK, duplicate, home/return, force-stop/relaunch, emulator termination, before/after commit holds; malformed/newer/pending/partial publication/sidecar/open failure preservation. After final fresh Founder-profile preparation, assert both pending names absent, terminate and restart the exact AVD without wiping or issuing a host/device-wide sync that could mask the product guarantee, then re-open the app and assert ready state plus absent pending names before preserving that profile for review.
- Regression: canonical `scripts/verify.ps1` and explicit desktop migration/recovery tests.

## Repository Verification Strategy

Run focused Node/Vitest/Rust checks during implementation, then M2-A build and APK inspection, the corrected native disposable suite including the final-profile AVD restart regression, and finally `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1` after the last tracked change. Preserve the Founder Step 1 failure evidence outside the regenerated native-review directory. Record final non-workflow digest, APK SHA-256, native report digest, and exact Git status. API 37 is build-tool/platform inventory only because no installed system image exists.

## Manual UI Verification

Owner: Founder. Final handoff will give concise steps for fresh launch, trilingual locale, save/list/open exact text, force-stop/restart reopen, unsaved draft, technical direct-origin details, icon/safe-area, and separate synthetic failure-state fixtures. Automated native evidence does not pass this gate.

## Rollback Or Recovery Strategy

No repository reset or runtime repair. Code changes remain unstaged and reviewable. Test fixtures use exact disposable temp directories/package/AVD only. The already-captured `ANDROID-M2A-FOUNDER-STEP1-FAILURE-001` evidence remains immutable while the disposable AVD may be wiped only by the bounded native regression. Runtime failures preserve live/pending database, receipt, and sidecars for inspection and block further writes. Historical M1 commits/archive/APK digest remain referenced and unchanged.

## Documentation Impact

Add `docs/architecture/22_Android_M2A_Direct_Fresh_v5_Initialization.md` and `docs/dev/13_Android_M2A_Direct_Fresh_v5_Runbook.md`; add factual navigation/status entries to Index/Roadmap. Update no Book Zero primary definition and do not rewrite M1 accepted evidence.

## ADR Impact

No new ADR. This is the bounded implementation evidence explicitly anticipated by Accepted ADR-0012. ADR-0012’s status, production identity choice, and non-authority remain unchanged.

## Risk Level

high, because origin verification, SQLite publication, directory-entry durability, and canonical schema parity are safety-sensitive. Risk is bounded by synthetic-only identity, exact no-overwrite paths, ordered file/directory sync barriers, separate verifiers, fail-closed partial states, host/native fault tests, an exact-AVD shutdown/restart regression, and Founder review before any promotion.

## Escalation Decision

No pre-implementation escalation is required. Stop at `human_decision_required` if canonical parity would require changing Accepted schema/receipt policy, if an existing M2-A install collides, if a test would touch M0/M1/real data, or if bounded verification cannot distinguish a commit/publication outcome safely.
