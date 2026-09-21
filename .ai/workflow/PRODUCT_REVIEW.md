# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-09-20-android-build-feasibility-m0
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 1575943094f24bd83c42088fbe4bb1a296083c50
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-20T08:00:18.8180919Z
- Updated at: 2026-09-20T08:00:18.8180919Z

## Mission Interpretation

M0 is a platform-feasibility review candidate, not an Android product activation. It may prove that the existing Tauri/React shell can package and cold-launch through an isolated Android application identity while presenting a truthful, trilingual, offline disclosure. It must not inherit the ordinary desktop storage, migration/recovery, provider, credential, or historical-context runtime merely because those paths are current defaults.

## Problem Statement

The repository is desktop-ready but has no Android build surface. The ordinary Cargo default enables desktop schema v5, the Rust builder registers SQLite, recovery, filesystem, and provider commands, and the frontend eagerly starts storage and provider discovery when `App` mounts. Building that combination unchanged would make a feasibility APK capable of entering desktop/v4 database or provider paths.

## User Value

The Founder receives concrete evidence about native Android build and launch feasibility without risking personal data, creating a future production identity commitment, or conflating M0 with Android Personal AI R0.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI, Privacy before Profit, and Mirrors Not Oracles require truthful limitations and no implied AI authority.
- `docs/02_Philosophy.md`, `docs/03_Principles.md`, `docs/09_AI.md`: a synthetic preview may reflect user-entered text but must not claim interpretation, inference, or real AI.
- `docs/06_Memory.md`, `docs/Reflection.md`, `docs/10_Privacy.md`: M0 must not silently persist, transmit, profile, or connect to desktop evidence.
- `docs/appendix/Harness.md`: provider capability is not product authority; M0 must expose no provider transport.

## Relevant ADRs

- ADR-0004 keeps the MVP local-first and forbids treating cloud/device transfer as implied continuity.
- ADR-0005 requires one governed provider boundary; M0 uses no provider.
- ADR-0006 keeps Tauri + React as the existing foundation while explicitly recording that mobile was deferred; this Founder-authorized feasibility slice tests the foundation without redefining the MVP.
- ADR-0007, ADR-0009, and ADR-0011 require provenance, consent, and lifecycle boundaries; M0 creates no durable product artifact and sends no historical context.
- ADR-0008 requires repository-native sequential workflow evidence and does not permit a new Harness kernel.

## Current Implementation Context

- Implemented and verified: clean `develop` equals `origin/develop` at `1575943`; the archived R3 evidence records exact real-profile recovery, lifecycle writes enabled, Founder Steps 1-16 accepted, restart reconstruction, a new typed write, retained verified v4 backup, and the legacy Pattern provenance correction.
- Implemented desktop behavior: `com.lifeos.app`, default Cargo feature `desktop-schema-v5`, desktop SQLite/recovery/provider commands, and frontend startup through `createLocalEvidenceStoreRuntime()` plus `getAiRuntimeStatus()`.
- Not implemented: `src-tauri/gen/android`, Android application configuration, Android-specific backend isolation, an M0 screen, APK build scripts, Android manifest backup rules, or emulator evidence.
- Toolchain fact: project-local Node/pnpm/Rust exist, but no JDK, Android SDK/NDK/build-tools/platform-tools/emulator/Gradle command or Rust Android targets are currently available; firmware virtualization is enabled, while Windows hypervisor feature state cannot be read without elevation.

## In Scope

- Isolated `com.lifeos.feasibility.m0` debug build configuration.
- Generated Tauri Android project and minimal Android manifest/resources.
- Compile/runtime separation that exposes no desktop database, migration/recovery, cleanup, provider HTTP, historical transmission, or credential-discovery command on Android.
- A packaged-asset M0 shell with English, Traditional Chinese, and Japanese disclosure plus a clearly synthetic, session-only in-memory preview.
- Minimal Android capabilities, explicit backup/transfer exclusion, narrow-screen/safe-area/keyboard behavior, build/test scripts, review documentation, and disposable-emulator smoke when safely available.

## Out Of Scope

Android R0; app-local schema v5; schema-v4 fallback; desktop migration or recovery changes; BYOK; real AI; historical context; credentials; real personal data; production signing; stable application identity; upgrade/data inheritance promises; distribution; release; physical-device operation; M1-M4; Phase 4; Git publication.

## Product Constraints

- The M0 screen must make its five boundaries visible without opening technical details: feasibility build, no real AI, no persistent product data, no desktop connection, and preview loss after close/restart.
- Desktop `com.lifeos.app` and its runtime remain unchanged.
- The M0 identifier is explicitly temporary and does not promise future installation/data continuity.
- Browser preview cannot count as Android native verification.

## Evidence And Provenance Constraints

No Evidence, Reflection, Pattern, Context Recovery, or historical artifact is created. Synthetic preview input remains component memory only, is labeled synthetic, and is never presented as confirmed evidence or durable state.

## Historical Context Constraints

No historical selection, packet, consent, provider transmission, actual-use provenance, or cross-experience analysis is available in M0.

## Consent Constraints

No provider or historical consent is requested because the corresponding capabilities do not exist. Android SDK license acceptance and real-device installation remain external human actions and are not inferred from this implementation authorization.

## AI-Role Constraints

M0 performs no AI inference. Copy must not imply analysis, diagnosis, advice, pattern recognition, or provider availability. The synthetic preview is only an immediate mirror of user-entered sample text.

## Privacy Constraints

No desktop profile access; no SQLite product store; no credential discovery; no runtime network capability; no sensitive permissions; no cloud backup or device-transfer eligibility; no APK, local SDK path, local.properties, keystore, or signing secret in Git.

## User-Agency Constraints

The language switch and preview are explicit user actions. The preview is disposable and clearable. No background operation, silent persistence, or automatic interpretation is permitted.

## Acceptance Criteria

1. Android builds under `com.lifeos.feasibility.m0` while desktop remains `com.lifeos.app`.
2. Android Rust compiles a minimal command/plugin surface with no SQLite, migration/recovery, cleanup, provider HTTP, historical transport, or credential-discovery registration; focused tests prove the surface.
3. Android frontend renders only the M0 disclosure/preview path and does not instantiate desktop storage or provider startup; focused tests prove this.
4. English, Traditional Chinese, and Japanese show all five M0 boundaries plus temporary-ID/debug-signing/no-continuity disclosure.
5. Manifest requests no camera, microphone, location, contacts, or broad-storage permissions and excludes app data from cloud backup and device transfer for the actual target SDK rules.
6. A debug APK is reproducibly packaged with recorded path, SHA-256, version, ABI set, application ID, manifest facts, and debug-signing verification; build-time network is distinguished from runtime capability.
7. A specifically identified disposable emulator is the only eligible install target. If none exists or hypervisor/tool licenses require external action, native runtime checks remain explicitly pending rather than simulated.
8. Cold launch, language switching, narrow/safe-area/keyboard, Back, background/foreground, restart, and absence of database/provider activation are recorded only if actually run on Android.
9. Canonical desktop verification passes and the Constitution, Book Zero, desktop schema/migration/recovery/provenance behavior, real profile, and desktop identity remain unchanged.
10. Minimal platform-boundary/build-runbook and Index/Roadmap navigation are synchronized; M1-M4 remain deferred and separately gated.

## Risks

- Generated Gradle defaults could accidentally enable backup, permissions, or the desktop feature surface.
- Tauri mobile generation may require versions or licenses not present locally.
- A build can succeed while emulator acceleration is unavailable.
- UI-only hiding would not isolate native commands.
- Existing desktop package-contract tests intentionally reject an Android generated surface and require a narrow, regression-tested compatibility update rather than a wildcard.

## Open Questions

None at the product boundary. Exact SDK/NDK/AGP versions follow the current checked-in Tauri CLI template and official compatibility guidance, not an architectural commitment. External Android license acceptance or Windows hypervisor enablement may still be required before build/runtime verification.

## Human Decision Required

false. The Founder prompt explicitly authorizes this bounded M0 implementation. It does not authorize automatic license acceptance, privilege elevation, physical-device use, publication, release, or M1.

## Recommendation

Proceed with the smallest separate Android M0 shell and compile-time backend/frontend isolation. Fail closed at any missing external license or hypervisor prerequisite and preserve a build-verified/manual-pending state rather than weakening the boundary.

## Review Status

approved_with_conditions
