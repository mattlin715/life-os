# Engineering Report

Status: completed

- Sprint ID: 2026-09-20-android-build-feasibility-m0
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 1575943094f24bd83c42088fbe4bb1a296083c50
- Original terminal working-tree digest: a259381da08b02dd384ba08b0accbe2498663428d68833851cb897cd3f7681c4
- Created at: 2026-09-20T13:16:00Z
- Updated at: 2026-09-21T07:08:00Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the bounded Android Build Feasibility M0 candidate under temporary
identity `com.lifeos.feasibility.m0`. Android dynamically selects a trilingual,
synthetic, session-only React shell and compiles a bare Tauri builder with no
desktop SQLite, migration/recovery, filesystem, dialog, provider, credential,
or historical-context runtime. A debug x86_64 APK was built and inspected.
Native review later found and corrected the missing Tauri mobile entry-point
export and Android 16 system-bar overlap. The corrected exact APK completed the
11-item disposable-emulator checklist and received explicit Founder acceptance.

## Existing System Areas Inspected

- `AI_CONTRIBUTOR_GUIDE.md`, Constitution, relevant Book Zero, accepted ADRs,
  and current desktop architecture.
- Frontend App/storage/provider startup and Tauri Cargo, builder, commands,
  plugins, capabilities, generated schemas, and package contract.
- Android generation/build, manifest merging, backup resources, APK
  signing/ABI, local toolchain, and emulator acceleration state.
- Canonical verification, Index, Roadmap, and workflow control plane.

## Files Added

- `src/android-m0/AndroidFeasibilityApp.tsx`, CSS, and focused test.
- `src-tauri/tauri.android.conf.json`, Android-only capability, and
  `src-tauri/src/desktop_runtime.rs`.
- Exact generated Android surface under `src-tauri/gen/android/`, plus generated
  Android/mobile schemas.
- `scripts/android-m0.ps1`, `scripts/android-m0-contract.node-test.mjs`.
- `docs/architecture/20_Android_Build_Feasibility_M0.md` and
  `docs/dev/11_Android_M0_Runbook.md`.

## Files Modified

- Frontend/build selection: `package.json`, `vite.config.ts`, `src/main.tsx`,
  `src/vite-env.d.ts`.
- Tauri isolation: `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`, desktop
  capability and generated capability schemas.
- Verification: `scripts/verify.ps1`; Founder-dogfood and ordinary package
  contract implementations/tests.
- Documentation: `docs/00_Index.md`, `docs/12_Roadmap.md`.
- Current workflow artifacts.

## Files Deleted

`src-tauri/gen/android/app/src/main/res/xml/file_paths.xml`, an unused generated
FileProvider resource. No product-data or desktop file was deleted.

## Behavior Changed

Android-mode builds now render only the bounded M0 shell. Sample text is
mirrored verbatim in component state and can be cleared. Android registers only
a bare Tauri builder and Android `core:default` capability. Ordinary desktop
builds keep the existing App, commands, plugins, and schema-v5 runtime.

## Data Model Impact

None. M0 creates no product database, record, artifact, backup, or cross-device
state.

## Migration Impact

None. Android exposes no schema initialization, v4 fallback, migration,
recovery, cleanup, or retained backup behavior. Desktop migration is unchanged.

## Provenance Impact

None. Sample text is visibly synthetic and session-only, not Evidence or a
reviewed artifact.

## Historical Context Impact

None. No retrieval, packet, consent, transmission, persistence, or
cross-experience interpretation exists.

## Consent Impact

No product-consent contract changed. The Founder handled the relevant Android
SDK license interactively; automation did not answer license prompts.

## Provider Transmission Impact

None at runtime. Android has no provider command/import and the APK has no
Internet permission. Build dependency downloads are documented separately.

## Tests Added

- Six Node contracts for exact generated files, identity/manifest,
  backup/transfer exclusions, native safe-area handling, Rust isolation, and
  frontend isolation.
- Four Vitest cases for trilingual boundaries, session-only mirror/clear,
  temporary identity, and no analysis/persistence claims.
- Ordinary package contract now asserts distinct desktop and temporary Android
  identifiers instead of requiring Android to be absent.
- The legacy Founder-package guard delegates only the two bounded Android M0
  trees to the Android exact-file contract and rejects neighbor prefixes.

## Tests Executed

- `node --test scripts/android-m0-contract.node-test.mjs`: 6/6 passed.
- Focused Android M0 Vitest: 4/4 passed.
- `pnpm run typecheck`, desktop `pnpm run build`, Android web build: passed.
- Android-target Rust compilation and desktop `cargo check`: passed.
- `pnpm android:m0:build`: passed through the exact no-elevation symlink
  fallback; forbidden bundle-token scan passed; Gradle assembled the APK.
- `pnpm android:m0:inspect`: passed identity, permission, backup, cleartext,
  ABI, signature, and digest checks.
- Founder-dogfood package tests: 11/11 passed; Founder schema-v5 candidate: 1/1
  passed; ordinary schema-v5 review package: 3/3 passed after its assertion was
  updated to follow the desktop runtime extraction.
- Fresh canonical repository verification passed with exit code 0. It includes
  workflow 33/33, Founder-package 11/11, Founder-v5 1/1, ordinary-package 3/3,
  Android contract 6/6, Vitest 377/377 across 49 files, TypeScript/build, Rust
  225/225 plus integration/compatibility/activation/runtime checks, whitespace,
  UTF-8, secret-file, link, and Constitution-diff checks.
- Workflow control-plane regression after the terminal-follow-up correction:
  33/33 passed in disposable repositories.
- After `ANDROID-M0-PROMOTION-001 Option A` was recorded, the complete canonical
  command was run again as the publication preflight and passed with exit code
  0; no Android APK rebuild or native-review repetition was performed.

## Verification Results

The original build-only terminal verification passed before the later native
corrections. The stale-digest failure was reproduced and recorded rather than
reported as a current pass. The accepted corrected build/package evidence is:

- fresh canonical command:
  `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`;
- exit code 0 at `2026-09-21T05:56:04Z`, current verified non-workflow digest
  `89dae6d3db296e49cdb850f2a1f952dc8b6d2b8d116acc3f15bd658ee3697f27`;
- captured output
  `.artifacts/android-m0/promotion-review/fresh-canonical-verification.txt`,
  55132 bytes, SHA-256
  `aa301c3dcf09a44ff0fca93fb9e30507322ad53fb8d30f21f758dada56dcbbd1`.

The first fresh attempt correctly failed at the legacy Founder-package changed-
path guard because a duplicate `.ai/README.md` documentation edit was outside
that guard. The edit was removed without changing product behavior, and the
entire canonical command was rerun from the beginning. The failed attempt is
retained as
`.artifacts/android-m0/promotion-review/fresh-canonical-verification-attempt-1-failed.txt`;
it is not presented as passing evidence.

The final pre-publication canonical output is retained at
`.artifacts/android-m0/promotion-execution/pre-publication-canonical-verification.txt`,
54688 bytes, SHA-256
`90a34bc8a7dce481c7457b789a49e323f2c6ad966ec0ff3965af7ca11e22db1e`.

- APK `src-tauri/gen/android/app/build/outputs/apk/x86_64/debug/app-x86_64-debug.apk`;
- 218677173 bytes; SHA-256
  `bd1a927f0f1347a11fccbe1d63a81e0851a907d6df981ed8b5f2c4db00291cad`;
- ID `com.lifeos.feasibility.m0`, version `0.3.0` / code `3000`;
- x86_64 `liblife_os_lib.so` only; Android debug signer; v2 verified;
- no dangerous/runtime network permission, backup and cleartext disabled, both
  backup/transfer exclusion resources packaged.

The native checklist passed 11/11 on disposable AVD
`lifeos_m0_api36_x86_64`, exact serial `emulator-5554`. Evidence hash is
`a147e80e890e7ca5f2dfc5eea8873d2a1ca8e47fca9e9ba00b05faf9d731ecbd`.
The Founder accepted it with `ANDROID-M0-NATIVE-ACCEPT-001 Option A`. The
machine `manual_ui` projection remains `not_run` because the existing workflow
has no formal interface for this field; acceptance is recorded in the exact
artifact/evidence path and is not hand-written into JSON.

## Manual Verification Required

Completed and Founder-accepted. The 11/11 result covers cold launch, five
disclosures, three languages, portrait/safe-area/long text, keyboard resize,
verbatim mirror/clear, Back/background/foreground, force-stop session loss,
runtime boundary checks, and clean native logs. It is not repeated during this
promotion-preparation follow-up.

## Documentation Updates

Added the platform-boundary record and runbook. Updated Index and Roadmap to
record the accepted native result while keeping M1-M4 unauthorized. Added the
bounded completed-state follow-up control path to the workflow documentation.

## ADR Impact

No new or modified ADR. M0 makes no durable production Android decision.

## Deviations From Plan

Gradle 8.14 could not run on Studio JBR 25, so verified project-local OpenJDK 21
is used only in the build process. Tauri's final Windows jniLibs symlink was
denied without Developer Mode; the script permits only that exact failure,
copies the compiled library, and packages without elevation. Existing dev doc
number 09 was occupied, so the runbook uses next-free number 11. Product scope
did not deviate.

## Known Limitations

- APK is debug-signed, x86_64-only, large/unoptimized, and temporary.
- Gradle emits SDK XML-version and Gradle 9 deprecation warnings but succeeds.
- Normal AndroidX Startup/ProfileInstaller merged infrastructure remains; it is
  not a Life OS provider or dangerous permission.

## Remaining Risks

M0 evidence proves only the reviewed disposable-emulator feasibility boundary.
Production identity, storage, provider, signing, distribution, upgrades, and
sync remain future governed decisions.

## Git State

- Branch `codex/android-build-feasibility-m0`; HEAD/base
  `1575943094f24bd83c42088fbe4bb1a296083c50`.
- Staged files: none. Product/scripts/generated Android/docs/workflow changes
  are unstaged; SDKs, caches, APK, AVD, report, and debug key remain ignored.
- At this report snapshot, no commit or push has yet occurred. The resolved
  promotion decision authorizes exactly one containing M0 commit and a normal
  fast-forward push of the current branch after the final allowlist/index
  preflight. PR, merge, archive/reset, distribution, deployment, and release
  remain forbidden.

## Engineer Completion Status

`completed`. Product implementation, corrected debug APK, inspection, native
11/11, Founder native acceptance, repeated fresh canonical verification, and
theory review are complete. `ANDROID-M0-PROMOTION-001 Option A` authorizes only
the exact-bound single-commit branch publication that follows this report
snapshot. It does not authorize M1, merge, PR, archive/reset, distribution,
deployment, or release.

## Terminal Follow-Up Control-Plane Correction

The prior workflow was terminal at revision 25 while its verified digest still
described the build-only candidate. A new bounded
`start-terminal-follow-up` command now:

- checks completed status, sprint ID, exact HEAD/branch/sequence, authorization
  reference/evidence, and the existing event chain;
- appends without rewriting the first 25 events;
- preserves the complete terminal coordinates plus prior verification and
  build-only Founder decision evidence;
- returns only to `validation` with verification `pending`;
- writes state/events atomically with rollback; and
- grants no manual pass, theory approval, promotion, archive, Git, or M1 action.

Disposable regression fixtures prove stale-terminal reproduction, accepted
continuation, coordinate/reference/chain rejection, event-prefix and Founder-
response preservation, two-write rollback, verification invalidation, and the
requirement for fresh canonical verification before theory review.

## Promotion Whitespace Correction

The first exact staging preflight exposed three previously invisible whitespace
errors in untracked generated Android sources: one final blank line in each of
`src-tauri/gen/android/build.gradle.kts` and
`src-tauri/gen/android/buildSrc/build.gradle.kts`, plus one whitespace-only
line in `BuildTask.kt`. Canonical verification had not seen them earlier because
the paths were untracked when its staged/unstaged diff checks ran.

`ANDROID-M0-PROMOTION-WHITESPACE-001 Option A` authorized only those three
non-semantic removals and another bounded terminal follow-up. No other product
or generated source changed. The corrected staged diff passes
`git diff --cached --check`. The complete canonical verifier then passed again
with exit code 0; output is retained at
`.artifacts/android-m0/promotion-execution/whitespace-corrected-canonical-verification.txt`,
50013 bytes, SHA-256
`9f698f94ce858b68e598f03e1b0052b869c614d0682a3b45d7db67cd560c8e2f`.
