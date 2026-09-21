---
status: Implemented
version: 0.1
owner: product-and-engineering
last_updated: 2026/09/21
depends:
  - docs/00_Constitution.md
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
referenced_by:
  - docs/00_Index.md
  - docs/12_Roadmap.md
  - docs/dev/11_Android_M0_Runbook.md
---

# 20 Android Build Feasibility M0

## Status and authority

This document records an implemented and Founder-reviewed **build-feasibility
candidate**, not an Android product activation. Automated build and artifact
inspection are complete. On 2026-09-21, the Founder accepted the disposable-
emulator native checklist 11/11 under `ANDROID-M0-NATIVE-ACCEPT-001`.

M0 does not authorize Android R0, product data, production signing, a stable
application identity, distribution, deployment, release, or M1-M4. It changes
neither the desktop identity `com.lifeos.app` nor the desktop schema-v5 runtime.

## Implemented boundary

| Surface | M0 behavior |
| --- | --- |
| Application identity | Temporary `com.lifeos.feasibility.m0` |
| Frontend | Separate trilingual feasibility shell selected at build time |
| User input | Synthetic textarea state held only in React memory |
| Rust runtime | Bare Android `tauri::Builder`; no desktop modules, invoke handlers, or plugins |
| Persistence | No SQLite plugin, database, migration, recovery, import/export, or durable artifact |
| AI and providers | No provider command, discovery, credential, request, inference, or historical context |
| Backup and transfer | `allowBackup=false`; legacy and Android 12+ rules exclude every supported app-data domain |
| Network | No `android.permission.INTERNET`; cleartext traffic disabled |
| Packaging | Debug APK, x86_64 only, Android debug signing |

The generated Android project is checked against an exact allowlist. The
Android frontend is dynamically imported, so its production bundle does not
load the ordinary desktop App, local evidence store, or provider runtime. The
desktop runtime remains under `cfg(not(target_os = "android"))`; Android-only
Cargo compilation does not link desktop SQLite, HTTP, filesystem, dialog, or
time dependencies.

## Truthful product surface

English, Traditional Chinese, and Japanese copy makes five facts visible:

1. this is a feasibility build, not Android R0 or a release;
2. there is no real AI, inference, diagnosis, or advice;
3. no product database or persistent Life OS data is created;
4. no desktop profile, history, provider, credential, or cloud connection exists;
5. preview text is session-only and may disappear after close or restart.

The preview echoes text without analysis. It is not Evidence, Reflection,
Memory, a Pattern, or an identity statement. This preserves **We Build Mirrors,
Not Oracles** without pretending that M0 supplies the product's governed mirror.

## Packaged manifest and privacy evidence

The source manifest declares no `<uses-permission>`. AndroidX manifest merging
adds only the app-namespaced signature permission
`com.lifeos.feasibility.m0.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`; it is not
a dangerous runtime permission. The merged manifest also contains AndroidX
startup/profile-installer infrastructure, not a Life OS provider.

The corrected packaged APK was inspected on 2026-09-21:

- application ID: `com.lifeos.feasibility.m0`;
- version: `0.3.0` (`versionCode 3000`);
- ABI/library: `/lib/x86_64/liblife_os_lib.so` only;
- signer: Android debug certificate; APK Signature Scheme v2 verified;
- SHA-256: `bd1a927f0f1347a11fccbe1d63a81e0851a907d6df981ed8b5f2c4db00291cad`;
- size: `218677173` bytes;
- backup disabled and both backup-rule resources packaged;
- no camera, microphone, location, contacts, storage, or network permission.

The APK and inspection report are ignored build evidence, not release assets.
Build dependency downloads used the network; this is distinct from the APK's
runtime capability.

## Build-system boundary

Android Studio's bundled JBR 25 is retained for Studio. Gradle 8.14 cannot run
on that Java class version, so the repository build script uses a verified,
project-local Microsoft OpenJDK 21 only for its child process. It does not alter
global `JAVA_HOME` or `PATH`.

Tauri's Windows Android build reaches frontend generation and Rust compilation,
then requires a jniLibs symbolic link. Developer Mode/elevation is outside M0
authority. The script accepts only that exact failure, copies the already-built
`liblife_os_lib.so` into ignored jniLibs, and lets the generated Gradle project
package the APK. Any earlier or different Tauri failure remains fail-closed.

## Native-runtime evidence

The disposable Android 36 x86_64 AVD `lifeos_m0_api36_x86_64` was launched as
the exact serial `emulator-5554`; no physical device was used. Native review
first exposed a missing mobile entry-point export, then a system-bar overlap.
Both were corrected with regression contracts. The final APK passed cold
launch, trilingual rendering, portrait safe areas, long-copy scrolling,
keyboard resize, verbatim mirror/clear, Back/background/foreground, force-stop
session loss, package-boundary inspection, and clean-log review.

The Founder accepted all 11 runbook observations on 2026-09-21. This closes
only the M0 native-review gap. It does not authorize Git publication, release,
physical-device testing, or M1-M4.

## ADR impact

No new ADR is needed for this bounded feasibility slice. It makes no durable
choice for production Android storage, identity, signing, provider transport,
distribution, synchronization, or lifecycle semantics. Any M1-M4 decision that
crosses those boundaries requires a new governed review and any applicable ADR.

## Official references

Accessed 2026-09-20 and 2026-09-21:

- Android Studio and SDK: <https://developer.android.com/studio>
- Android backup controls: <https://developer.android.com/identity/data/autobackup>
- Tauri Android prerequisites: <https://v2.tauri.app/start/prerequisites/>
- Microsoft OpenJDK downloads: <https://learn.microsoft.com/en-us/java/openjdk/download>
- Android edge-to-edge views: <https://developer.android.com/develop/ui/views/layout/edge-to-edge>
- Android 16 behavior changes: <https://developer.android.com/about/versions/16/behavior-changes-16>
