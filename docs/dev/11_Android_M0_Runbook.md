---
status: Implemented
version: 0.2
owner: product-and-engineering
last_updated: 2026/09/21
depends:
  - docs/architecture/20_Android_Build_Feasibility_M0.md
  - docs/dev/08_Engineering_Harness.md
referenced_by:
  - docs/00_Index.md
  - docs/12_Roadmap.md
---

# 11 Android M0 Runbook

## Purpose

Reproduce and review the isolated Android Build Feasibility M0 candidate. This
runbook does not authorize a physical device, Windows-feature or hypervisor
changes, administrator elevation, production signing, distribution, release,
or access to any real Life OS profile.

## Local toolchain snapshot

The following repository-local/ignored toolchain was inventoried on 2026-09-20:

| Component | Version or path |
| --- | --- |
| Android Studio | Quail 4 / 2026.1.4 Patch 1 |
| Studio JBR | 25.0.3 |
| Gradle build JDK | Microsoft OpenJDK 21.0.12.1, project-local |
| Android command-line tools | 22.0 |
| Platforms | Android 36 and 37.0 |
| Build-Tools | 35.0.0 and 36.0.0 |
| Platform-Tools | 37.0.1 |
| Emulator | 37.1.11 |
| NDK | 30.0.16248370 |
| Rust | 1.96.1 with all four Android targets |
| Disposable AVD | `lifeos_m0_api36_x86_64`, Android 36 AOSP x86_64 |

The relevant base Android SDK license was accepted interactively by the
Founder through Android Studio. Unrelated Google TV, XR, DBT, preview, GDK, and
MIPS licenses were declined. Do not automate license answers.

## Inventory, build, and inspect

Run from the repository root in PowerShell:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\android-m0.ps1 -Action Inventory
pnpm android:m0:build
pnpm android:m0:inspect
```

The script sets `JAVA_HOME`, `ANDROID_HOME`, `ANDROID_SDK_ROOT`, `NDK_HOME`,
`ANDROID_AVD_HOME`, Gradle home, and `PATH` only in its process. It does not
modify user or machine environment variables.

`android:m0:build` runs the exact source contract, the official Tauri Android
build path, Android-mode TypeScript/Vite bundling, an explicit forbidden-token
scan, Android Rust compilation, and Gradle debug packaging. On Windows without
Developer Mode, only Tauri's exact symbolic-link denial may use the documented
copy-and-package fallback. Every other failure stops the build.

Expected ignored outputs:

- APK: `src-tauri/gen/android/app/build/outputs/apk/x86_64/debug/app-x86_64-debug.apk`
- inspection: `.artifacts/android-m0/apk-inspection.txt`
- native build output, Gradle caches, SDKs, AVD, and debug keystore under ignored paths.

Review the inspection report for the temporary ID, x86_64-only library,
debug signer, SHA-256, merged permissions, `allowBackup=false`, cleartext=false,
and both packaged backup/transfer rule resources.

## Deterministic repository verification

```powershell
node --test scripts/android-m0-contract.node-test.mjs
pnpm vitest run src/android-m0/AndroidFeasibilityApp.test.tsx
pnpm run typecheck
pnpm run build
pnpm run build:android:m0:web
pnpm android:m0:inspect
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

These checks do not replace native UI review.

## Emulator prerequisite gate

At the initial build-candidate stop, `emulator-check accel` exited `6` because
the Windows hypervisor was unavailable. The Founder later completed the host
prerequisite interactively and restarted Windows. The accepted review verified
WHPX as usable and bound every `adb` action to exact disposable-emulator serial
`emulator-5554`; no physical device was used.

For any future reproduction, first re-check acceleration and device inventory.
Refuse to proceed if the selected serial does not begin with `emulator-`, if
more than one target is ambiguous, or if any physical-device serial would be
used. Host feature, license, and administrator prompts remain manual gates.

## Founder native UI checklist

Record the APK SHA-256 and exact emulator serial before review. Then verify:

1. cold launch reaches the M0 screen without a crash;
2. all five boundary disclosures are visible and truthful;
3. English, Traditional Chinese, and Japanese switch correctly;
4. the temporary identifier/debug-signing/no-continuity notice is visible;
5. narrow portrait layout, display cutouts/safe areas, and long text do not clip;
6. opening the keyboard keeps the focused textarea usable and the view resizes;
7. sample text is mirrored verbatim without analysis, and **Clear preview** clears it;
8. Android Back, background, and foreground behave normally;
9. force-stop/restart loses the session-only sample text;
10. no desktop profile, SQLite database, provider, credential, network request,
    historical context, backup, or device-transfer behavior appears;
11. native logs contain no crash, database startup, provider call, or secret.

## Accepted native-review result

The disposable-emulator checklist was completed on 2026-09-21 against APK
SHA-256 `bd1a927f0f1347a11fccbe1d63a81e0851a907d6df981ed8b5f2c4db00291cad`
on exact serial `emulator-5554`, with no physical device. All 11 observations
passed after correcting the mobile entry point and applying system-bar plus
display-cutout insets outside the scrolling WebView.

The Founder accepted the 11/11 result through
`ANDROID-M0-NATIVE-ACCEPT-001 Option A`. This acceptance applies only to M0 and
does not authorize staging, commit, push, release, or M1.

## Build-time versus runtime network

SDK, Maven/Gradle, Rust crate, and npm dependency acquisition are build-time
network activity. The inspected APK has no `android.permission.INTERNET`, no
provider command, and no provider frontend import. M0 therefore does not claim
or expose product runtime network behavior.

## Rollback and cleanup

No product migration or recovery exists. Repository changes remain normal Git
diffs; generated build outputs, toolchains, AVD state, local properties, and
debug signing material remain ignored. Do not reset unrelated work. Deleting a
disposable AVD or ignored toolchain is an operator cleanup choice, not a product
data operation.

## Official references

Accessed 2026-09-20 and 2026-09-21:

- Android Studio and SDK: <https://developer.android.com/studio>
- Android backup controls: <https://developer.android.com/identity/data/autobackup>
- Tauri Android prerequisites: <https://v2.tauri.app/start/prerequisites/>
- Microsoft OpenJDK downloads: <https://learn.microsoft.com/en-us/java/openjdk/download>
- Android edge-to-edge views: <https://developer.android.com/develop/ui/views/layout/edge-to-edge>
- Android 16 behavior changes: <https://developer.android.com/about/versions/16/behavior-changes-16>
