# Engineering Report

Status: completed

- Sprint ID: 2026-09-29-android-m2a-direct-fresh-v5
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 51dff4998ca8696aeaf1527f058d785303fc0cd4
- Working-tree digest implemented: 146c28adc2591a71eefb2b82fba97eefbd82559ce71d0c1a350ab3c9a676b919
- Created at: 2026-09-29T15:22:49Z
- Updated at: 2026-10-02T16:24:00Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented a synthetic-only Android M2-A direct fresh-v5 review candidate under temporary identity `com.lifeos.review.m2a`. Fresh initialization creates the exact final schema from canonical compatibility and v5 DDL in one transaction, records a separate content-free `direct_fresh_v5` receipt, inserts zero historical migration receipts, and uses a direct-origin verifier/runtime without weakening historical migration. The Android facade publishes verified pending database and receipt sources through same-directory exclusive-create/copy/file-sync steps, syncs the parent directory after both live-name creations, retires both pending names, and syncs the parent directory again before enabling writes.

The Founder Step 1 failure showed byte-identical live and pending pairs after emulator shutdown/restart. Evidence `ANDROID-M2A-FOUNDER-STEP1-FAILURE-001` remains preserved. The correction also extends the native harness to verify absent pending names before and after shutting down and restarting the exact final-profile AVD without a masking `adb shell sync`. The accepted M1 trilingual create/list/get flow, locale preference, launcher icon, safe-area handling, backup/transfer exclusions, and offline boundary remain preserved in a distinct M2-A frontend/package/AVD. Desktop routes and M1 source/evidence remain present.

## Existing System Areas Inspected

- `src-tauri/schema/schema_v5.sql` and schema-v5 migration, verification, runtime, lifecycle, backup, recovery, and writer modules.
- `src-tauri/src/android_m1.rs`, `src/android-m1/`, generated Android package/config/capabilities, M1 scripts/contracts, and immutable M1 archive/commit evidence.
- Accepted ADR-0012, architecture 01/21, runbook 12, Roadmap, Index, Constitution, Privacy, persistence ADRs, and Engineering Harness.
- Local Android inventory: Android 36 default x86_64 system image; Android 37 platform without an installed API 37/alternate-ABI emulator image.

## Files Added

- `src-tauri/schema/schema_v5_compatibility.sql`
- `src-tauri/src/schema_v5_direct_init.rs`
- `src-tauri/src/android_m2a.rs`
- `src-tauri/capabilities/android-m2a.json`
- `src/android-m2a/AndroidM2AApp.tsx`
- `src/android-m2a/AndroidM2AApp.test.tsx`
- `src/android-m2a/androidM2AStore.ts`
- `src/android-m2a/android-m2a.css`
- `scripts/android-m2a.ps1`
- `scripts/android-m2a-native-review.ps1`
- `scripts/android-m2a-cdp-probe.mjs`
- `scripts/android-m2a-contract.node-test.mjs`
- `docs/architecture/22_Android_M2A_Direct_Fresh_v5_Initialization.md`
- `docs/dev/13_Android_M2A_Direct_Fresh_v5_Runbook.md`

## Files Modified

- Current sprint workflow artifacts under `.ai/workflow/`.
- `docs/00_Index.md`, `docs/12_Roadmap.md`.
- `package.json`, `vite.config.ts`, `src/main.tsx`.
- `scripts/verify.ps1`, Founder package guard/tests, and M1 successor-preservation contract.
- Android config/capability schema and exact generated Gradle/activity/string surface.
- `src-tauri/src/lib.rs`, `schema_v5_migration.rs`, direct-aware runtime/Experience writer, and Evidence/Reflection/Pattern/context-recovery writer verification adapters.

## Files Deleted

none.

## Behavior Changed

- M2-A fresh install no longer invokes `EMPTY_V4_BASE_SCHEMA`, `migrate_disposable_v4`, or any migration receipt path.
- Direct schema creation and its receipt are verified before publication; valid direct-v5 databases reopen without reinitialization.
- Live names are never overwritten. Exclusive create, copy, and sync preserve verified pending sources; any partial live/pending combination, SQLite sidecar, malformed/newer database, invalid receipt, or open conflict blocks and remains untouched.
- Both live-name creation and successful pending-name retirement now cross ordered app-private parent-directory durability barriers. A retirement whose final barrier fails cannot claim ready in that process.
- M2-A exposes only status, locale preference, and synthetic Experience create/list/get commands.
- Android automation is separately bound to M2-A identity, AVD, serial, ports, database, receipt, and artifact paths.

## Data Model Impact

No canonical schema shape change. Compatibility projection DDL was extracted verbatim into a reusable final-v5 SQL include and remains required by the canonical contract. Direct initialization sets `user_version=5`, inserts the enabled database contract, and inserts no migration receipt. Synthetic Experience rows use existing canonical tables and lifecycle rules.

## Migration Impact

Historical v4-to-v5 migration logic and strict receipt verification remain unchanged. Direct origin has separate types and verifier entry points. Desktop initialization, migration, production activation, backup, recovery, and real-profile routing are unchanged. No user or review database is migrated.

## Provenance Impact

Adds a content-free initialization receipt with `direct_fresh_v5` origin, application ID, schema version, pinned schema/compatibility hashes, deterministic empty source/initial target manifests, receipt schema version, and timestamp. It contains no Experience content and is never represented as historical migration evidence.

## Historical Context Impact

None. No history is selected, assembled, imported, or transmitted. Compatibility tables are empty schema objects at fresh creation.

## Consent Impact

None. No provider or historical transmission is activated, so no consent event is created or bypassed.

## Provider Transmission Impact

None. The package declares no INTERNET permission and contracts reject provider/desktop runtime surfaces.

## Tests Added

- Direct initializer success, strict origin separation, transactional failure boundaries, and uncertain commit verification.
- M2-A backend direct origin/CJK/reopen, transaction interruption, concurrent initialization, exclusive no-replace publication, partial publication, malformed/newer/receipt/sidecar/open failure, and locale preference tests.
- M2-A frontend 11-test flow/locale/copy/error/idempotency suite.
- Exact M2-A package, generated-surface, canonical icon, backup/network, direct-path, and M0/M1 preservation contract.
- Dedicated Android 36 x86_64 native journey and failure-fixture automation.
- Exact final-profile native restart regression: no pending names before shutdown, exact AVD restart without a harness-wide sync, ready reopen, and no pending names after restart.

## Tests Executed

- `cargo test --manifest-path src-tauri/Cargo.toml schema_v5_migration::direct_init::tests --no-fail-fast` — PASS, 2/2.
- `cargo test --manifest-path src-tauri/Cargo.toml android_m2a::tests --no-fail-fast` — PASS, 8/8 after the durability correction.
- `node --test scripts/android-m2a-contract.node-test.mjs` — PASS, 10/10 after adding the ordered-barrier and exact-AVD-restart safeguards.
- `node --test scripts/android-m1-contract.node-test.mjs` — PASS, 9/9.
- `node --test scripts/founder-dogfood-package.node-test.mjs` — PASS within combined 33-test run.
- `pnpm vitest run src/android-m2a/AndroidM2AApp.test.tsx` — PASS, 11/11.
- `pnpm typecheck` — PASS.
- `pnpm android:m2a:build` — PASS after the correction via bounded Windows no-symlink Gradle fallback after Tauri frontend/codegen/Rust success.
- `pnpm android:m2a:inspect` — PASS for corrected APK `85d6911b34afc31b7e847fc34cd1c8ed05b63fe8084193f7aaf6e0a69e127ebb`, temporary identity, debug signing, Android 36/x86_64, backup exclusions, cleartext disabled, and no unexpected permission.
- `pnpm android:m2a:native` — PASS for every claimed Android 36 x86_64 result, including the final-profile exact-AVD shutdown/restart and absent-pending-name regression; report SHA-256 `c15991f9b69dfe5400bb35f0425796af4279890a2a2ac03eaf86e5dbcb51387e`.

## Verification Results

- Baseline canonical verifier on clean required `develop`/`origin/develop`: PASS before implementation.
- Focused host/package/frontend/backend checks: PASS.
- Packaged-artifact inspection: PASS. Exact corrected review APK is 155,208,006 bytes with SHA-256 `85d6911b34afc31b7e847fc34cd1c8ed05b63fe8084193f7aaf6e0a69e127ebb`.
- Native Android 36 x86_64: PASS for direct receipt/zero migration receipts, exact CJK, locale restart, idempotency, background, force-stop, commit-boundary outcomes, emulator-process restart, malformed/newer/pending/partial/WAL/open failure preservation, and fresh final Founder profile.
- Final canonical repository verifier: PASS after the durability correction at non-workflow digest `146c28adc2591a71eefb2b82fba97eefbd82559ce71d0c1a350ab3c9a676b919`; 33/33 workflow tests, 15/15 Founder package tests, 9/9 M1 contracts, 10/10 M2-A contracts, 399/399 Vitest tests, 241/241 Rust library tests, integration/feature/refusal suites, typecheck, build, whitespace, UTF-8, secrets, links, and Constitution checks passed.
- Second native configuration: skipped; no additional installed emulator system image/ABI is available without installation.
- Physical device/power loss: unsupported and not run.

## Manual Verification Required

The original Founder manual UI review Step 1 is recorded as failed for the preserved pre-correction candidate. The corrected candidate now returns to the Founder gate at Step 1; the Founder must review the exact corrected diff/APK and perform the complete fresh launch, locale, synthetic save, exact reopen, Home/return, force-stop/relaunch, unsaved-draft, technical-details, and non-capability checklist in runbook 13. Automated native checks do not convert that failure into manual acceptance.

## Documentation Updates

Added architecture 22 and runbook 13; synchronized Index and Roadmap. They distinguish M1 scaffold from M2-A direct origin, shared versus Android duties, exclusive publication/readiness invariants, actual tested scope, and production/device gaps.

## ADR Impact

ADR-0012 remains Accepted and unchanged. M2-A implements only its already-accepted direct fresh-v5 direction. No Proposed ADR was needed and no new production authority was created.

## Deviations From Plan

Android native storage returned permission denied for same-directory hard-link creation. The correction uses exclusive live-file creation (`create_new`), byte copy, and explicit sync while retaining the verified pending source. It prevents overwrite and makes partial publication fail closed; documentation and safeguards do not claim a single atomic two-file rename. Windows adb transport required one bounded reconnect only when the daemon reported that the command was not delivered; device-side or ambiguous outcomes are never retried.

The first final canonical run exposed one stale ordinary-package regression test
that still expected the current Android config to be M1. Its assertion was
narrowly updated to the M2-A identity and title; the separate M1 contract still
checks the accepted M1 commit and current-successor boundary.

Founder Step 1 then exposed a durability gap not caught by the earlier native
suite: file contents had been synced, but successful pending-name removal had
not been followed by a parent-directory sync. The bounded correction adds a
live-name directory barrier before retirement and a deletion directory barrier
before ready. The native harness initially had one PowerShell parse error from
an underscore-separated numeric literal; it was corrected to `10000` before any
AVD mutation and the full native suite then passed.

## Known Limitations

- One Android 36 x86_64 AOSP emulator only; no API 24–35, API 37, ARM, OEM, or physical-device evidence.
- Force-stop, emulator-process failure, and transaction interruption do not prove graceful process shutdown, deterministic raw same-UID SIGKILL, or physical power-loss durability.
- No production identity/signing/upgrades, multi-process coordination, backup/restore, complete migration/recovery, retention/deletion, import/export, sync, provider, or real-data support.
- Debug APK and all data remain disposable synthetic evidence.

## Remaining Risks

Founder visual/manual acceptance of the corrected candidate is pending from Step 1. Device/filesystem diversity may expose behavior absent on the one emulator. The exclusive-copy publication protocol intentionally blocks rather than repairs a crash-created mixed state; production recovery remains a later governed M2 decision.

## Git State

Branch `codex/android-m2a-direct-fresh-v5`; HEAD `51dff4998ca8696aeaf1527f058d785303fc0cd4`; corrected non-workflow digest `146c28adc2591a71eefb2b82fba97eefbd82559ce71d0c1a350ab3c9a676b919`; all changes are unstaged; staged files: none. No commit, push, merge, PR, archive/reset, or upstream publication occurred. `origin/develop` matched the required base after live fetch at sprint start.

## Engineer Completion Status

completed
