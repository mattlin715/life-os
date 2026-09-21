# Engineering Plan

Status: approved

- Sprint ID: 2026-09-20-android-build-feasibility-m0
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 1575943094f24bd83c42088fbe4bb1a296083c50
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-20T08:00:18.8180919Z
- Updated at: 2026-09-20T09:18:00Z

## Approved Product Boundary

`PRODUCT_REVIEW.md` is `approved_with_conditions`: implement only an isolated Android feasibility/debug shell under `com.lifeos.feasibility.m0`, with no product persistence, desktop profile connection, provider/credential capability, historical context, or real data. Desktop `com.lifeos.app` and its schema-v5 runtime remain unchanged. Runtime evidence may be claimed only from a specifically identified disposable emulator.

## Existing Implementation Understanding

- The desktop build uses Tauri CLI `2.11.4`, Tauri Rust/JS major `2`, React `18.3`, project-local Node `24.18.0`, pnpm `11.10.0`, and Rust/Cargo `1.96.1`.
- `Cargo.toml` defaults to `desktop-schema-v5`; `lib.rs` registers provider HTTP commands, SQLite commands, migration/recovery commands, and dialog/fs/sql plugins.
- `App.tsx` mounts `createLocalEvidenceStoreRuntime()` and later calls `getAiRuntimeStatus()`, so rendering the desktop App on Android is not safe even if controls are hidden.
- `tauri.conf.json` uses desktop identity `com.lifeos.app`; `capabilities/default.json` grants desktop dialog/fs/sql/window capabilities.
- `src-tauri/gen/android` is absent and an existing package-contract test asserts its absence.
- The project-local toolchain now contains Android Studio Quail 4 / `2026.1.4 Patch 1`, bundled JBR `25.0.3`, command-line tools `22.0`, platform `android-37.0`, Build-Tools `36.0.0`, Platform-Tools `37.0.1`, Emulator `37.1.11`, NDK `30.0.16248370`, an Android 36 AOSP `x86_64` system image, and all four required Rust Android targets. The Android Studio and command-line-tools downloads matched their official SHA-256 values.
- Disposable AVD `lifeos_m0_api36_x86_64` exists under ignored evidence storage, but `emulator-check accel` exits `6`: firmware virtualization is enabled while a Windows hypervisor driver/feature is unavailable. Native emulator execution therefore remains Founder-manual pending and cannot be claimed in this sprint's automated evidence.

## Affected Modules

- Android configuration/generated surface: `src-tauri/tauri.android.conf.json`, `src-tauri/gen/android/`, Android manifest and backup XML.
- Backend boundary: `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`, focused source-contract tests.
- Frontend boundary/UI: `src/main.tsx`, a new Android M0 component/copy/style module, `src/vite-env.d.ts`, and focused Vitest coverage.
- Build/verification: bounded PowerShell/Node scripts for process-local Android environment, debug APK build, source/manifest/APK inspection, and optional serial-bound emulator smoke.
- Documentation/navigation: one M0 platform-boundary record, one build/manual runbook, `docs/00_Index.md`, `docs/12_Roadmap.md`, and current workflow evidence.
- Existing desktop package allowlist test: replace only the obsolete “no Android surface” assertion with an exact M0 surface contract; do not broaden the allowlist.

## Proposed Design

1. Install no global variables. Discover Android Studio/JBR and SDK paths, then set `JAVA_HOME`, `ANDROID_HOME`, `ANDROID_SDK_ROOT`, and `NDK_HOME` only inside repository build scripts.
2. Use the checked-in Tauri CLI to initialize Android after the official prerequisites and four Rust targets exist. Preserve generated structure except for bounded manifest/config/privacy changes.
3. Add `tauri.android.conf.json` with temporary ID `com.lifeos.feasibility.m0`, Android M0 title, packaged assets, and mobile build command. Keep base desktop config unchanged.
4. On `target_os = "android"`, compile a minimal Tauri builder with no dialog/fs/sql plugins and no invoke registrations. Keep all existing desktop modules/commands under non-Android cfg. This prevents the default desktop feature from producing an Android v4/v5 fallback.
5. Build a separate `AndroidFeasibilityApp` selected by a compile-time frontend flag. It imports no storage/provider modules, keeps preview text in React state only, and exposes trilingual boundary copy plus clear/reset behavior.
6. Add Android manifest backup rules for both pre-Android-12 and Android-12+ formats, set `allowBackup=false`, and explicitly exclude all supported app-data domains from cloud backup and device transfer. Add no sensitive permission.
7. Package debug APKs only. Inspect the merged manifest, APK application ID/version/ABI/signature, record SHA-256, and ensure outputs, local.properties, SDK paths, and debug keystore stay ignored.
8. If a dedicated AVD is available, enumerate devices first, refuse any non-emulator serial, bind every adb command with `-s <serial>`, and run install/cold-launch/language/layout/Back/background/foreground/restart/log checks. Otherwise report runtime/manual pending.

## Alternatives Considered

- Disable the desktop Cargo default feature globally: rejected because it changes ordinary desktop behavior and can silently fall back to schema v4.
- Hide desktop controls in CSS: rejected because native commands and startup effects remain reachable.
- Reuse the desktop App with an in-memory store: rejected because provider discovery and desktop UI semantics still load, and the boundary would be hard to prove.
- Create a second application/repository: rejected as unnecessary duplication and architecture drift.
- Use browser preview as runtime evidence: rejected because it does not prove Android packaging, WebView, manifest, lifecycle, or signing.

## Data Lifecycle Impact

Android M0 holds only synthetic preview text in component memory. Closing/restarting may discard it. No lifecycle event, artifact, export, import, cleanup, backup, or retained evaluation record is created.

## SQLite Or Migration Impact

No Android SQLite plugin, connection, database path, schema initialization, migration, recovery, retention, or v4 fallback. Existing desktop schema-v5 compile/runtime behavior remains the baseline regression target.

## Provenance Impact

None. M0 creates no AI/user-reviewed artifact. Synthetic preview is visibly not Evidence and has no persistence/provenance claim.

## Historical Context Impact

None. No retrieval, packet, consent, transmission, persistence, or analysis path is rendered or registered.

## Consent Impact

No product consent contract changes. External Android SDK licenses must be accepted directly by the Founder/operator; automation will not answer license prompts.

## Provider Transmission Impact

None at runtime. Android registers no provider command and the M0 frontend imports no provider adapter. Dependency download network during build is documented separately from APK runtime capability.

## Import And Export Impact

None. Android M0 exposes no import/export or file picker.

## Test Strategy

- Vitest: trilingual disclosure, temporary-ID/signing/no-continuity copy, session-only preview/clear, Android entry selection, and no desktop storage/provider import in the M0 module.
- Rust/source contract: Android target selects only the minimal builder; desktop command/plugin registration remains intact; default desktop feature does not become v4 fallback.
- Generated-project contract: exact identifier/config, minimal permissions, both backup-rule formats, no local.properties/debug keystore tracking, and narrow known generated paths.
- Build artifact: `aapt2`/`apkanalyzer` or equivalent manifest/ABI/version checks, `apksigner verify --print-certs`, and SHA-256.
- Optional emulator: serial-qualified install/start/force-stop/Back/background/foreground/restart, UI/log evidence, and no database/provider activation.

## Repository Verification Strategy

Run focused frontend/Rust/generated/manifest tests, TypeScript/build checks, Android debug build and artifact inspector, then the canonical `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Verify no Constitution/Book Zero/desktop identity/profile data/secret/signing artifact diff and no staged files.

## Manual UI Verification

Founder-owned final review: launch the APK on a disposable emulator or later explicitly authorized real device; check five disclosures, all three languages, safe areas, narrow layout, keyboard resize/focus, Back, background/foreground, restart loss, and absence of desktop/provider behavior. This sprint stops before the Founder performs the final review.

## Rollback Or Recovery Strategy

All repository changes remain unstaged on the feature branch and are reviewable. Android outputs and the disposable AVD are outside product data and need no migration. Do not reset or delete repository work automatically. Desktop remains available because base config, ID, and non-Android builder path are preserved.

## Documentation Impact

Add one factual M0 feasibility/platform-boundary document and one reproducible build/manual runbook with official-source URLs and access date `2026-09-20`; add minimal Index/Roadmap links and an R3 current-status pointer if needed. Do not rewrite Book Zero or the R3 archive.

## ADR Impact

No new ADR. M0 is an explicitly authorized, isolated feasibility implementation under accepted ADR-0004/0005/0006/0007/0008/0009/0011 and does not choose the production Android storage, identity, signing, provider, or distribution architecture.

## Risk Level

medium. The product surface is intentionally small, but build-system generation, Android manifest defaults, default Cargo features, and missing external toolchain/licenses can create false confidence or accidental capability leakage unless fail-closed tests remain exact.

## Escalation Decision

Proceed with the bounded implementation and build-verification path. The Founder/operator installed the official project-local toolchain, personally handled the relevant SDK license flow, and explicitly authorized completing all work that does not require Windows hypervisor setup. Do not launch an emulator, enable Windows features, request elevation, use a physical device, or claim native runtime evidence. Package, inspect, test, and document the debug APK, then stop with emulator UI/lifecycle checks clearly marked `manual pending`.
