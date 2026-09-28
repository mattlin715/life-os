# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-09-22T09:44:00Z
- Updated at: 2026-09-28T09:12:00Z

## Sprint ID

2026-09-22-android-m1-disposable-persistence-review

## Mission

Prepare an Android M1 architecture decision package and isolated runnable
fresh-schema-v5 persistence prototype, verify it through automation and a
dedicated disposable emulator, and stop at one Founder diff/manual UI gate.

## Starting Commit

`04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df` from clean synchronized `develop`.

## Ending Commit Or Working-Tree State

The exact candidate and bounded pre-archive closeout delta were committed on
`codex/android-m1-disposable-persistence-review` as
`d0e38640a53361c281d65b4befb6208b33f84346`. The supported workflow terminal
follow-up is completing fresh verification and official archival.

## Final Status

`completed_with_follow_up`. `ANDROID-M1-FOUNDER-REVIEW-004` was resolved with
Option A; later production and M2 work remain separate.

## Product Decision

Accepted under `ANDROID-M1-FOUNDER-REVIEW-004` Option A: keep Tauri/React with
a Rust-owned Android adapter; use stable production ID `com.lifeos.app`; make one app-private
exact-schema-v5 SQLite database the local authority for each Android install;
share platform-independent domain/schema/transaction logic; keep Android path,
publication, lifecycle, permissions, and fault guarantees platform-specific;
acknowledge only after commit/re-read; preserve and fail closed; exclude
automatic backup/transfer until governed continuity exists.

## Engineering Summary

Implemented temporary package `com.lifeos.review.m1`, a trilingual synthetic
Experience create/list/get journey, exact fresh-v5 app-private initialization,
idempotent saves, honest error/state handling, bounded Rust commands, offline
x86_64 debug packaging, exact source/APK contracts, and disposable-emulator
fault evidence. Desktop routing and profiles remain untouched.

Founder-authorized revision cycle 1 adds a Rust-owned app-private locale-code
preference and rewrites Traditional Chinese/Japanese UI copy against desktop
terminology while preserving the same synthetic-only product boundary.

Founder-authorized revision cycle 2 reuses the canonical PC desktop icon source
for Android launcher/adaptive assets, adds exact source/output/resource guards,
and changes no product behavior or brand design.

Founder-authorized revision cycle 3 changes only adaptive foreground safe-zone
scale and transparent padding so Android masks do not clip the canonical ring.
Legacy/round assets and every non-icon behavior remain unchanged.

## Behavior Changed

Android M1 mode persists explicit synthetic saves locally and reopens exact
text after relaunch. It blocks on incompatible/ambiguous storage and exposes no
AI, provider, historical context, desktop import, repair, backup/restore,
update/delete, export/import, sync, or production behavior.
The last selected `en`, `zh-TW`, or `ja` locale is restored from a separate
non-content app-private file; invalid or unavailable preference state defaults
to English and never enters the schema-v5 content store.

## Files Changed

- Workflow: current mission, product review, engineering plan/report, theory
  review, decision package, sprint report, state, and event journal.
- Decision/docs: `docs/00_Index.md`, `docs/12_Roadmap.md`, architecture 21,
  Accepted ADR-0012, and dev runbook 12.
- Frontend: `src/android-m1/`, `src/main.tsx`, `vite.config.ts`, `package.json`.
- Rust/config: `src-tauri/src/android_m1.rs`, shared fresh base,
  `schema_v5_founder_activation.rs`, `lib.rs`, `Cargo.toml`, Android config,
  capability, generated identity/activity/string/capability schema files.
- Evidence scripts/contracts: Android M1 build, native, CDP, contract scripts,
  canonical verifier step, and bounded legacy package-contract synchronization.

## Tests

- Android M1 source contract: 9/9 pass, including exact canonical icon binding.
- Frontend/full Vitest: 50 files, 388 tests pass.
- Focused Android M1 Rust: 6/6 pass.
- Full canonical Rust suite and feature/refusal matrices: pass.
- APK build/inspection: pass for icon-corrected APK SHA-256
  `2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975`.
- Dedicated Android 36 x86_64 native run: every claimed scenario, including
  locale restoration after force-stop/relaunch, passes.
- Canonical `scripts/verify.ps1`: exit 0 on the exact candidate.

## Repository Verification

Passed at 2026-09-27T18:23:59Z, HEAD
`04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df`, non-workflow digest
`469747c01d7b3f74e1d6c75203785a06b538f79f2237f501d49d933500fd6269`.
Ignored transcript: `.artifacts/android-m1/canonical-verification.txt`.

## Manual Verification

The locale/copy-corrected APK completed the 10-step Founder checklist 10/10.
The first icon correction matched the brand but failed Founder review because
Android clipped the pale ring at all four cardinal edges. Manual review of the
safe-zone-corrected APK SHA-256
`2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975`
passed under `ANDROID-M1-FOUNDER-REVIEW-004` Option A: the complete pale ring is
visible on all four sides inside the Android mask and the launcher matches the
canonical PC desktop Life OS icon.

## Architecture Updates

Added architecture 21 with identity options, storage ownership, fresh-v5
boundary, shared-versus-platform responsibilities, acknowledgement semantics,
backup consequences, fail-closed design, evidence matrix, and M2-M4 fences.

## ADR Updates

ADR-0012 changed from `Proposed` to `Accepted` through the explicit Founder
decision. No other Accepted ADR changed or was reinterpreted.

## Documentation Synchronization

Index, Roadmap, architecture, ADR, and runbook consistently record the exact
unstaged M1 candidate and architecture direction as Founder-accepted while
keeping real data, production activation, M2, and release authority absent.

## Data And Migration Impact

Synthetic-only temporary app-private database. No desktop or real-profile
inspection/mutation and no schema change. The prototype uses an empty v4 test
base to exercise the exact canonical v5 transaction; direct production fresh-v5
bootstrap, upgrade migration, and recovery remain M2.

## Provenance And Consent Impact

Existing exact-v5 source/revision/event semantics are reused. No AI artifact,
provider, credential, consent, historical packet, actual-use provenance, or
network transmission is introduced.

## Risks

Only API 36 x86_64 emulator is native-tested. Physical power loss, raw same-UID
SIGKILL timing, graceful process-level shutdown, other APIs/ABIs/OEM devices,
production signing/upgrades, and continuity operations are unproven. Excluded
backup/transfer means uninstall/data clear/device loss can destroy local data.

## Deferred Items

All M2 production store, migration/recovery/lifecycle/export work; M3
provider/consent work; M4 signing,
distribution, upgrades, optional sync, deployment, and release.

## Human Decisions

Resolved: `ANDROID-M1-FOUNDER-REVIEW-001` Option C authorized only the locale
and copy correction plus fresh evidence; `ANDROID-M1-FOUNDER-REVIEW-002`
Option C authorized only canonical launcher-icon alignment and fresh evidence;
`ANDROID-M1-FOUNDER-REVIEW-003` Option C authorized only adaptive safe-zone
scale/padding plus fresh evidence; `ANDROID-M1-FOUNDER-REVIEW-004` Option A
accepted the exact final candidate, selected `com.lifeos.app`, and accepted
ADR-0012 without authorizing production or M2.

## Review Cycles

Three Founder-authorized theory revision cycles. Cycle 1 replaced a failed
WebView-local-storage attempt with the Rust-owned bounded locale preference and
passed fresh evidence. Cycle 2 replaced Android/Tauri placeholder launcher
assets with exact derivatives of the canonical PC icon; focused checks,
build/inspection, the complete native matrix, and canonical verification passed.
Cycle 3 added only exact centered transparent safe-zone padding after the ring
was visually clipped; focused checks, build/inspection, complete native, and
canonical verification passed. The Founder then gave final visual PASS and
accepted the exact candidate under review decision 004 Option A.

## Workflow Lessons

Legacy package guards that inspect the entire working tree must delegate a new
Android surface to its exact contract rather than freezing an obsolete active
identity. Native evidence must distinguish force-stop, app-process kill,
emulator-process termination, and physical power loss.

## Recommended Next Sprint

The current M1 workflow is complete. A separately authorized M2 planning sprint
may be proposed later but must not start implicitly from this acceptance.

## Git Status

The accepted candidate and bounded pre-archive closeout delta are committed as
`d0e38640a53361c281d65b4befb6208b33f84346`. Official archive/reset-to-idle,
feature publication, non-fast-forward `develop` merge, merged verification, and
final publication remain. No PR, distribution, deployment, release, real-data
activation, or M2 work occurred.

## Integration Closeout Authorization And Exact Promotion Manifest

Founder authorization `ANDROID-M1-INTEGRATION-CLOSEOUT-001` permits only factual acceptance/publication/closeout documentation, supported workflow terminal follow-up/archive/reset-to-idle, exact archive-prefix compatibility, staging/committing the accepted M1 candidate and bounded closeout changes, a non-fast-forward merge into `develop`, and normal non-force pushes of the feature branch and `develop`.

It does not authorize product/runtime behavior changes, APK rebuild or launch, emulator/device access, real-profile/database/sidecar access, production identity activation, real data, M2, distribution, deployment, or release.

### Category A — Founder-Accepted M1 Candidate

The following exact 58 paths are bound to accepted non-workflow digest `469747c01d7b3f74e1d6c75203785a06b538f79f2237f501d49d933500fd6269`, exact APK SHA-256 `2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975`, and `ANDROID-M1-FOUNDER-REVIEW-004` Option A:

- `.ai/workflow/CURRENT_MISSION.md`
- `.ai/workflow/DECISION_REQUIRED.md`
- `.ai/workflow/ENGINEERING_PLAN.md`
- `.ai/workflow/ENGINEERING_REPORT.md`
- `.ai/workflow/EVENTS.jsonl`
- `.ai/workflow/PRODUCT_REVIEW.md`
- `.ai/workflow/SPRINT_REPORT.md`
- `.ai/workflow/THEORY_ALIGNMENT_REVIEW.md`
- `.ai/workflow/WORKFLOW_STATE.json`
- `docs/00_Index.md`
- `docs/12_Roadmap.md`
- `package.json`
- `scripts/founder-dogfood-package.mjs`
- `scripts/founder-dogfood-package.node-test.mjs`
- `scripts/ordinary-schema-v5-review-package.node-test.mjs`
- `scripts/verify.ps1`
- `src-tauri/Cargo.toml`
- `src-tauri/gen/android/app/build.gradle.kts`
- `src-tauri/gen/android/app/src/main/AndroidManifest.xml`
- `src-tauri/gen/android/app/src/main/java/com/lifeos/feasibility/m0/MainActivity.kt`
- `src-tauri/gen/android/app/src/main/res/mipmap-hdpi/ic_launcher.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-hdpi/ic_launcher_foreground.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-hdpi/ic_launcher_round.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-mdpi/ic_launcher.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-mdpi/ic_launcher_foreground.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-mdpi/ic_launcher_round.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xhdpi/ic_launcher.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xhdpi/ic_launcher_foreground.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xhdpi/ic_launcher_round.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xxhdpi/ic_launcher.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xxhdpi/ic_launcher_foreground.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xxhdpi/ic_launcher_round.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher_foreground.png`
- `src-tauri/gen/android/app/src/main/res/mipmap-xxxhdpi/ic_launcher_round.png`
- `src-tauri/gen/android/app/src/main/res/values/strings.xml`
- `src-tauri/gen/schemas/capabilities.json`
- `src-tauri/src/lib.rs`
- `src-tauri/src/schema_v5_founder_activation.rs`
- `src-tauri/tauri.android.conf.json`
- `src/main.tsx`
- `vite.config.ts`
- `docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md`
- `docs/architecture/21_Android_M1_Disposable_Persistence_Architecture.md`
- `docs/dev/12_Android_M1_Disposable_Persistence_Runbook.md`
- `scripts/android-m1-cdp-probe.mjs`
- `scripts/android-m1-contract.node-test.mjs`
- `scripts/android-m1-native-review.ps1`
- `scripts/android-m1.ps1`
- `src-tauri/capabilities/android-m1.json`
- `src-tauri/gen/android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml`
- `src-tauri/gen/android/app/src/main/res/values/ic_launcher_background.xml`
- `src-tauri/src/android_m1.rs`
- `src-tauri/src/schema_v5_fresh_base.rs`
- `src/android-m1/AndroidM1App.test.tsx`
- `src/android-m1/AndroidM1App.tsx`
- `src/android-m1/android-m1.css`
- `src/android-m1/androidM1Store.ts`

### Category B — Authorized Closeout-Only Delta

- `.ai/workflow/SPRINT_REPORT.md`: record this authorization, exact manifest, publication evidence, and final refs.
- `.ai/workflow/ENGINEERING_REPORT.md`, `.ai/workflow/THEORY_ALIGNMENT_REVIEW.md`, `.ai/workflow/WORKFLOW_STATE.json`, and `.ai/workflow/EVENTS.jsonl`: supported terminal-follow-up and fresh closeout truth only.
- `docs/dev/12_Android_M1_Disposable_Persistence_Runbook.md`: record the bounded repository-integration authorization and unchanged product fences.
- `scripts/founder-dogfood-package.mjs`: allow only exact archive prefix `.ai/workflow/HISTORY/2026-09-22-android-m1-disposable-persistence-review/`.
- `scripts/founder-dogfood-package.node-test.mjs`: accept that exact prefix and reject neighboring prefixes.
- `scripts/android-m1-native-review.ps1`: remove the single trailing space at line 218 reported by `git diff --check`; no command or behavior changes.
- `.ai/workflow/HISTORY/2026-09-22-android-m1-disposable-persistence-review/`: official CLI-generated terminal archive; current workflow files reset from repository templates.

### Explicit Exclusions

APKs, SDK/JDK/toolchain files, databases, SQLite sidecars, credentials, debug signing keys, emulator/AVD state, private logs, and ignored artifacts are excluded from Git promotion. The available APK and ignored native/canonical evidence are hash-checked only and are not rebuilt, launched, or staged.
