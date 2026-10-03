---
status: Founder review candidate
version: 1.0
owner: product-and-engineering
last_updated: 2026/10/03
depends:
  - docs/architecture/22_Android_M2A_Direct_Fresh_v5_Initialization.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
  - docs/dev/08_Engineering_Harness.md
referenced_by:
  - docs/00_Index.md
  - docs/12_Roadmap.md
---

# 13 Android M2-A Direct Fresh-v5 Runbook

## Purpose and safety boundary

Build, inspect, and review only the synthetic M2-A candidate. Use no physical
phone, real profile, desktop database, credentials, provider traffic, global
SDK/JDK mutation, production signing, distribution, or release. The package
`com.lifeos.review.m2a`, database, receipt, and AVD are disposable. Do not clear,
replace, or uninstall the accepted M0/M1 review packages or AVD data.

## Deterministic host checks

From the repository root in PowerShell:

```powershell
node --test scripts/android-m2a-contract.node-test.mjs
pnpm vitest run src/android-m2a/AndroidM2AApp.test.tsx
pnpm typecheck
powershell -NoProfile -ExecutionPolicy Bypass -Command ". .\scripts\use-local-dev-env.ps1; cargo test --manifest-path src-tauri/Cargo.toml schema_v5_migration::direct_init::tests --no-fail-fast; cargo test --manifest-path src-tauri/Cargo.toml android_m2a::tests --no-fail-fast"
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

The canonical verifier also reruns M1 contracts and desktop Rust suites. Host
checks do not replace packaged-artifact, native, or Founder manual review.

## Build and inspect

```powershell
pnpm android:m2a:build
pnpm android:m2a:inspect
```

The child process uses repository-local JDK, SDK/NDK, AVD, and Gradle homes and
does not modify global `JAVA_HOME` or `PATH`. Only the already-understood Windows
post-Rust Tauri jniLibs symlink failure may use the bounded copy-and-Gradle
fallback; every other build failure stops.

Expected ignored evidence:

- APK: `src-tauri/gen/android/app/build/outputs/apk/x86_64/debug/app-x86_64-debug.apk`;
- inspection: `.artifacts/android-m2a/apk-inspection.txt`;
- native report: `.artifacts/android-m2a/native-review/native-result.md`;
- exported direct receipt and database under the same native-review directory.

The corrected Founder review candidate produced on 2026/10/03 JST is
155,208,006 bytes with APK SHA-256
`85d6911b34afc31b7e847fc34cd1c8ed05b63fe8084193f7aaf6e0a69e127ebb`,
from source HEAD `51dff4998ca8696aeaf1527f058d785303fc0cd4`. The native report generated
at `2026-10-02T16:21:41.9419132Z` binds its results to that same APK hash and
has SHA-256
`c15991f9b69dfe5400bb35f0425796af4279890a2a2ac03eaf86e5dbcb51387e`.
A rebuild creates a different candidate and requires new inspection and native
evidence.

Inspection must show package `com.lifeos.review.m2a`, debug signing, x86_64-only
native library, `allowBackup=false`, Android 12+ and legacy backup exclusions,
cleartext disabled, no `android.permission.INTERNET`, and the accepted launcher
icon assets.

## Dedicated native evidence

```powershell
pnpm android:m2a:native
```

The script refuses to start if any device is connected. It uses only dedicated
AVD `lifeos_m2a_api36_x86_64`, exact serial `emulator-5584`, emulator port 5584,
and debug CDP port 9224. It wipes that dedicated M2-A AVD, confirms the exact
M2-A package is absent, and never uninstalls or clears M0/M1 packages.

The run covers fresh launch, direct receipt and zero migration receipts,
save/list/get, exact Chinese/Japanese text, duplicate suppression, locale
restoration, background/foreground, force-stop/relaunch, synchronized
before-commit and after-commit-before-acknowledgement outcomes, emulator-process
restart, interrupted initialization, partial publication, pending state,
SQLite sidecars, malformed/newer state, open failure, and a final fresh Founder
profile. For that final profile, it verifies both pending names are absent,
shuts down and restarts the exact AVD without issuing `adb shell sync`, then
requires ready storage and both pending names still absent. Every destructive
fixture operation is restricted to this script's dedicated disposable M2-A
sandbox.

Do not broaden the result. Force-stop differs from graceful shutdown,
emulator-process failure, raw Linux signal timing, and physical power loss. No
physical device or second API/ABI target is tested.

## Founder manual UI checklist

Use synthetic text only. Record the final APK SHA-256, identity, AVD, serial,
source HEAD, and working-tree digest from the generated evidence package.

1. Launch the freshly reset M2-A app. Confirm the disposable/synthetic,
   offline, no-AI, and unsupported-operation disclosures, safe areas, and the
   accepted Life OS launcher icon.
2. Switch English, Traditional Chinese, and Japanese. Leave a non-English
   locale selected, force-stop/relaunch, and confirm it is restored.
3. Enter a unique synthetic Experience containing Chinese and Japanese plus
   meaningful whitespace. Confirm it is unsaved and absent from the list.
4. Tap Save once. Confirm the draft cannot be double-submitted and success is
   not displayed before the durable acknowledgement.
5. Confirm one saved item appears. Open it and compare exact text, whitespace,
   and line breaks.
6. Go Home, return, and repeat the exact-text check.
7. Force-stop `com.lifeos.review.m2a`, relaunch, reopen the item, and confirm it
   remains exact and singular.
8. Enter another draft without saving, force-stop/relaunch, and confirm it is
   not represented as committed.
9. Expand technical details. Confirm package `com.lifeos.review.m2a`, schema 5,
   origin `direct_fresh_v5`, synthetic-only storage, and create/list/get only.
10. Confirm there is no AI output, provider/network/credential behavior,
    desktop import, backup/restore, update/delete, production, or release claim.

Founder manual acceptance remains `not_run` until the Founder explicitly
records the result for the exact candidate. Automated native evidence is not a
substitute.

## Separate failure-state review fixtures

The failed Founder Step 1 evidence is preserved separately at
`.artifacts/android-m2a/founder-review-step1-failure-001/` as
`ANDROID-M2A-FOUNDER-STEP1-FAILURE-001`. It contains the byte-identical live
and pending database/receipt pairs plus the blocked UI, inventory, hashes, and
logcat. The corrected native run must not overwrite or delete this evidence.

Failure-state evidence must never reuse the final manual-review profile. The
native automation creates isolated malformed, newer-version, pending,
partial-publication, sidecar, interrupted-transaction, and open-failure
fixtures inside the dedicated M2-A AVD. Review the report and exported copies;
do not repair or delete a failed fixture merely to continue. After evidence is
captured, the script prepares a separately wiped, fresh Founder-review profile.

If reproducing one case manually, first clone or recreate the dedicated M2-A
AVD under a new explicitly disposable name, bind every `adb` call to its exact
serial, and keep it separate from M0/M1 and the final Founder profile. Stop on
any unknown device, package, path, or mixed state.

## Promotion closeout authorization

`ANDROID-M2A-PROMOTION-CLOSEOUT-001` authorizes only the bounded terminal
follow-up needed to correct the stale deferred-review wording, record this
authorization, rerun workflow and canonical verification, promote the exact
verified M2-A allowlist on the current feature branch, and archive/reset the
completed workflow in a separate closeout commit. Both feature-branch pushes
must be normal and non-force.

This authorization does not change product behavior and does not authorize a
merge into `develop`, a pull request, `com.lifeos.app`, real data, another M2
slice, distribution, deployment, or release.

## Cleanup and non-authority

Stopping the emulator is safe. Removing the dedicated M2-A AVD or temporary
package is optional disposable cleanup only after review evidence is preserved.
Never clean an unknown database or any M0/M1/desktop profile.

Passing this runbook does not authorize staging, commit, push, merge, workflow
archive/reset, `com.lifeos.app`, real data, another M2 slice, Phase 4,
distribution, deployment, or release.
