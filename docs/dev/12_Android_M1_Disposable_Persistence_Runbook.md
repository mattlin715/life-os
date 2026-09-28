---
status: Founder accepted M1 evidence
version: 1.0
owner: product-and-engineering
last_updated: 2026/09/28
depends:
  - docs/architecture/21_Android_M1_Disposable_Persistence_Architecture.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
  - docs/dev/08_Engineering_Harness.md
referenced_by:
  - docs/00_Index.md
  - docs/12_Roadmap.md
---

# 12 Android M1 Disposable Persistence Runbook

## Purpose and safety boundary

Build, inspect, and review the synthetic-only Android M1 persistence candidate.
Use no physical phone, real Life OS profile, credentials, provider traffic,
desktop database, global SDK/JDK changes, production signing, distribution, or
release. The package `com.lifeos.review.m1` and its data are disposable.

## Build and inspect

From the repository root in PowerShell:

```powershell
pnpm android:m1:build
pnpm android:m1:inspect
```

The build script uses the repository-local Microsoft JDK 21, Android SDK/NDK,
AVD home, and Gradle home only in its child process. It does not modify global
`JAVA_HOME` or `PATH`. On Windows without symbolic-link permission, only the
known post-Rust Tauri jniLibs link failure may use the bounded copy-and-Gradle
fallback; all other failures stop.

Expected ignored evidence:

- APK: `src-tauri/gen/android/app/build/outputs/apk/x86_64/debug/app-x86_64-debug.apk`
- inspection: `.artifacts/android-m1/apk-inspection.txt`
- native report: `.artifacts/android-m1/native-review/native-result.md`

Inspection must show the temporary ID, debug certificate, x86_64-only native
library, `allowBackup=false`, both backup/transfer exclusion resources,
cleartext disabled, and no `android.permission.INTERNET`.

The launcher icon contract binds the Android density, round, and adaptive assets
to the largest exact PNG embedded in the existing PC source
`src-tauri/icons/icon.ico`. The Android assets are generated derivatives only;
do not redraw the logo or substitute Android/Tauri placeholder artwork.
Adaptive foregrounds center the unchanged canonical image at less than 60% of
the canvas with transparent padding; the exact contract rejects missing or
off-center padding so Android masks cannot clip the pale ring at four edges.

## Deterministic checks

```powershell
node --test scripts/android-m1-contract.node-test.mjs
pnpm vitest run src/android-m1/AndroidM1App.test.tsx
pnpm typecheck
powershell -NoProfile -ExecutionPolicy Bypass -Command ". .\scripts\use-local-dev-env.ps1; cargo test --manifest-path src-tauri/Cargo.toml android_m1::tests"
pnpm android:m1:inspect
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

These checks do not replace native or Founder UI review.

## Dedicated native evidence

```powershell
pnpm android:m1:native
```

The script refuses to begin if any device is connected, creates or selects only
the exact AVD `lifeos_m1_api36_x86_64`, and binds all `adb` actions to exact
serial `emulator-5582`. It uninstalls/installs the temporary package and clears
that package's data; it never selects an arbitrary target.

The automated native run covers the save/list/get journey, exact CJK text,
double submission, background/foreground, force-stop/relaunch, synchronized
termination before commit and after commit-before-acknowledgement, direct
app-private schema inspection, emulator-process termination/restart,
malformed/newer database refusal, open failure, and a final freshly cleared
Founder-review profile. Controlled commit holds exist only in debug builds.
It also changes the locale to Traditional Chinese, force-stops and relaunches
the app, and confirms the app-private non-content preference is restored before
returning the automated journey to English. The preference stores only a fixed
locale code in `android-m1-locale.pref`; it does not store Experience or other
product content.

Do not reinterpret the result: graceful application-process shutdown and raw
same-UID SIGKILL are unexecuted; actual physical power loss is unsupported.
No physical device, ARM ABI, OEM device, Android version other than API 36,
production backup/restore, migration/recovery, or release upgrade is proven.

## Founder manual UI checklist

Use only synthetic text. Before beginning, record the APK SHA-256, package,
AVD name, and serial from the evidence reports.

1. Launch the freshly cleared review app and confirm the synthetic/disposable,
   offline, no-AI, and unsupported-operation disclosures are visible. From the
   Android launcher, also confirm the app icon matches the PC desktop Life OS
   icon (dark field, pale ring, centered `LO`) rather than the Android/Tauri
   placeholder. The complete pale ring must remain visible at the top, bottom,
   left, and right inside Android's rounded mask.
2. Switch English, Traditional Chinese, and Japanese; confirm the terminology
   feels consistent with the desktop Life OS while the narrower synthetic-only
   boundary remains explicit, and confirm the screen respects the
   status/navigation bars. Leave a non-English language selected, force-stop and
   relaunch, and confirm that language is restored.
3. Enter a unique synthetic Experience containing Chinese and Japanese. Confirm
   it is visibly **unsaved** and absent from the committed list.
4. Tap Save once. While saving, confirm the draft cannot be submitted again and
   no success is displayed before the committed receipt.
5. Confirm the saved item appears once. Open it and compare the exact text,
   whitespace, and line breaks with the input.
6. Use Android Home, return to the app, and repeat the exact-text check.
7. Force-stop `com.lifeos.review.m1`, relaunch, open the saved Experience, and
   confirm it remains exact and appears only once.
8. Enter another draft without saving, force-stop/relaunch, and confirm it is
   not represented as committed data.
9. Expand technical details and confirm the displayed identity is
   `com.lifeos.review.m1`, schema is 5, storage is synthetic-only, and only
   create/list/get are claimed.
10. Confirm there is no AI result, provider/credential/network behavior,
    desktop import, backup/restore, update/delete, or release claim.

The Founder completed the functional checklist 10/10, then explicitly reviewed
the locale/copy and launcher-icon corrections through the bounded review cycles.
The exact safe-zone-corrected candidate was accepted under
`ANDROID-M1-FOUNDER-REVIEW-004` Option A on 2026/09/28. Accepted APK SHA-256:
`2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975`.
This manual acceptance does not authorize real data, production installation,
distribution, release, or M2. Do not infer any of those later authorities.

## Integration closeout authorization

`ANDROID-M1-INTEGRATION-CLOSEOUT-001` authorizes factual closeout
documentation, supported workflow terminal follow-up and archive/reset-to-idle,
exact archive-prefix compatibility, staging and committing the accepted M1
candidate and bounded closeout changes, a non-fast-forward merge into
`develop`, and normal non-force publication of the feature branch and
`develop`.

This repository authorization does not change product/runtime behavior. It does
not activate `com.lifeos.app`, production storage, real data, M2, distribution,
deployment, or release. It also forbids APK rebuild or launch, emulator/device
operation, and desktop or Android real-profile/database/sidecar access.

## Current candidate provenance

The 2026-09-22 native candidate was built from branch
`codex/android-m1-disposable-persistence-review` at unchanged base HEAD
`04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df`. The reviewed package is
`com.lifeos.review.m1`, version `0.3.0` (`versionCode 3000`), Android 36 x86_64,
and Android debug-signed. Final APK hash and inspection results are regenerated
by `pnpm android:m1:inspect`; ignored artifacts are evidence, not release assets.

## Cleanup

The final native script state is a freshly cleared temporary package on the
dedicated disposable AVD, with the emulator stopped. Removing that ignored AVD
or package is disposable cleanup. Never delete or modify a desktop/Founder
profile as part of this runbook.
