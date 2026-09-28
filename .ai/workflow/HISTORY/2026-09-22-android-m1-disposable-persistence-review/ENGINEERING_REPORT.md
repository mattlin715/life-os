# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-09-22-android-m1-disposable-persistence-review
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df
- Working-tree digest implemented: 469747c01d7b3f74e1d6c75203785a06b538f79f2237f501d49d933500fd6269
- Created at: 2026-09-22T09:06:00Z
- Updated at: 2026-09-28T09:12:00Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the isolated `com.lifeos.review.m1` Tauri/React Android review app,
an app-private exact-schema-v5 Rust facade, a trilingual synthetic Experience
save/list/get journey, deterministic build/inspection contracts, dedicated AVD
native fault evidence, and an Android architecture/ADR package. Desktop routing
is unchanged. The Founder accepted the exact unstaged candidate, selected
`com.lifeos.app`, and accepted ADR-0012 under
`ANDROID-M1-FOUNDER-REVIEW-004` Option A; production activation remains later.

Founder-authorized revision cycle 1 now also remembers the selected locale in
the fixed app-private non-content file `android-m1-locale.pref`, restores it
across process restart, and aligns Traditional Chinese/Japanese copy with the
desktop language for saved moments, local storage, observable clues, pattern
hypotheses, and reflection without adding any desktop workflow or production
capability.

Founder-authorized revision cycle 2 reuses the largest exact PNG embedded in
the canonical PC desktop `src-tauri/icons/icon.ico`, regenerates only the
required Android launcher/adaptive icon assets, binds their exact source/output
digests and resource references, and adds the manifest round-icon binding. No
brand redesign or product behavior was introduced.

Founder-authorized revision cycle 3 scales only the unchanged canonical image
inside the five adaptive foreground canvases to less than 60%, leaving exact
centered transparent padding. Legacy and round assets remain byte-identical.
The contract now decodes each foreground, verifies its dimensions and alpha
bounds, and proves the xxxhdpi inner pixels exactly equal the canonical 256px
ICO payload.

## Existing System Areas Inspected

`docs/00_Constitution.md`, Book Zero privacy/MVP/Roadmap sources, Accepted ADRs,
`docs/architecture/01_Local_Evidence_Store.md`, architecture 13 and 17-20,
`src/shared/storage/`, `src-tauri/src/schema_v5_*`, Android generated project,
M0 scripts/runbook/evidence, and `.ai/workflow/`.

## Files Added

`docs/architecture/21_Android_M1_Disposable_Persistence_Architecture.md`;
`docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md`;
`docs/dev/12_Android_M1_Disposable_Persistence_Runbook.md`;
`scripts/android-m1.ps1`; `scripts/android-m1-native-review.ps1`;
`scripts/android-m1-cdp-probe.mjs`; `scripts/android-m1-contract.node-test.mjs`;
`src-tauri/capabilities/android-m1.json`; `src-tauri/src/android_m1.rs`;
`src-tauri/src/schema_v5_fresh_base.rs`; and `src/android-m1/`.

## Files Modified

`.ai/workflow/CURRENT_MISSION.md`, `PRODUCT_REVIEW.md`, `ENGINEERING_PLAN.md`,
workflow state/journal, `docs/00_Index.md`, `docs/12_Roadmap.md`, `package.json`,
`scripts/verify.ps1`, `src/main.tsx`, `vite.config.ts`, `src-tauri/Cargo.toml`,
`src-tauri/src/lib.rs`, `src-tauri/src/schema_v5_founder_activation.rs`,
`src-tauri/tauri.android.conf.json`, the bounded generated Android identity/
activity/string/icon files, and generated Tauri capability schemas.

## Files Deleted

none.

## Behavior Changed

Android M1 build mode now loads only the bounded review UI and Rust commands.
The app creates an exact-v5 database only in its verified app-private directory,
saves synthetic Experiences with an idempotent request identity, lists/reopens
exact text, and fails closed on pending, malformed, newer, ambiguous, or open-
failure states. No success is shown before a committed receipt. The APK remains
offline, debug-signed, x86_64-only, and excluded from backup/transfer.

Locale selection is now a separate presentation preference, not Experience or
schema-v5 content. Only `en`, `zh-TW`, or `ja` is accepted; missing, invalid, or
unreadable state falls back to English. App-data clear/uninstall removes it.

## Data Model Impact

No canonical schema change. The prototype reuses the exact existing schema-v5
contract and only writes synthetic Experience source/revision/event rows through
the canonical runtime writer. Its temporary database is separate from all
desktop and Founder profiles.

## Migration Impact

No real-profile or production migration. Fresh prototype initialization uses a
fixed empty v4 base in a pending app-private file solely to execute and verify
the canonical v4-to-v5 transaction, then publishes the exact-v5 candidate.
Pending or existing non-exact-v5 state is preserved and refused. Direct
production fresh-v5 bootstrap, upgrades, and recovery remain M2.

## Provenance Impact

The canonical schema-v5 source, revision, event, and guard-token contracts are
retained. The M1 UI does not create AI artifacts or represent its text as
Evidence, Reflection, Pattern, or identity truth.

## Historical Context Impact

none; no historical-context command, packet, UI, or data path is linked.

## Consent Impact

none; no provider or historical-context consent behavior is exposed or changed.

## Provider Transmission Impact

none; no provider runtime or `INTERNET` permission is present in the APK.

## Tests Added

Nine Node contract tests; eleven Android M1 frontend tests; six Rust M1 tests;
CDP journey/locale/fault probes; and a bounded Android 36 x86_64 native script.

## Tests Executed

- `node --test scripts/android-m1-contract.node-test.mjs`: PASS 9/9, including
  canonical desktop icon source, generated output, adaptive-resource/manifest
  bindings, exact transparent padding, centering, and canonical inner pixels.
- `pnpm typecheck`: PASS.
- `pnpm test:run`: PASS 50 files / 388 tests.
- local-toolchain `cargo test --manifest-path src-tauri/Cargo.toml android_m1::tests`: PASS 6/6.
- `pnpm android:m1:build`: PASS with bounded Windows jniLibs copy fallback.
- `pnpm android:m1:inspect`: PASS for icon-corrected APK hash
  `2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975`,
  with canonical desktop icon SHA-256
  `62d764181ef9137aec875f345345daef4bd398fa828f80cdb35bded563ad2ade`.
- `pnpm android:m1:native`: PASS every claimed native scenario; report at
  `.artifacts/android-m1/native-review/native-result.md`.

## Verification Results

Focused automation, build, APK inspection, and dedicated-emulator evidence pass.
Native evidence includes fresh/reopen/exact CJK/duplicate, background,
force-stop, before-versus-after-commit, app-private `user_version=5`, emulator
process termination/restart, malformed/newer preservation, open failure, and a
fresh final review profile. Revision-cycle native evidence additionally proves
the selected Traditional Chinese locale survives force-stop/relaunch through
the app-private non-content preference. Revision-cycle-3 focused checks, build
inspection, and the complete native matrix pass against the safe-zone-corrected
APK. Canonical repository verification also passed against non-workflow digest
`469747c01d7b3f74e1d6c75203785a06b538f79f2237f501d49d933500fd6269`.
The Founder then visually accepted the safe-zone-corrected icon and exact
disposable candidate under `ANDROID-M1-FOUNDER-REVIEW-004` Option A.

## Manual Verification Required

Completed by the Founder for exact APK SHA-256
`2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975`.
The complete pale ring is visible on all four sides inside Android's mask and
the launcher icon matches the existing PC desktop Life OS icon. Functional
review was also completed 10/10. This accepts disposable M1 evidence only.

## Documentation Updates

Added architecture 21, ADR-0012, dev runbook 12, and synchronized the Book Zero
index and Roadmap with Founder-accepted M1 evidence and M2-M4 fences.

## ADR Impact

ADR-0012 changed from `Proposed` to `Accepted` only through the explicit Founder
decision `ANDROID-M1-FOUNDER-REVIEW-004` Option A. No other ADR changed.

## Deviations From Plan

Raw same-UID SIGKILL was attempted but its timing was not deterministic enough
to claim, so it is reported unexecuted. A synchronized debug-only hold plus
force-stop demonstrates before-commit and after-commit-before-ack boundaries.
No portable process-level graceful-shutdown contract or physical power-loss
test is claimed.

The first revision-cycle native attempt showed that WebView `localStorage` did
not restore reliably after immediate force-stop in this disposable runtime. It
was not accepted as evidence. The implementation was replaced with the bounded
Rust-owned app-private locale file and the complete native matrix then passed.

## Known Limitations

Only Android 36 x86_64 AOSP emulator is native tested. API 24 minimum packaging,
other APIs/ABIs/OEM devices, physical power loss, production signing/upgrades,
backup/restore, migration/recovery, update/delete/import/export, providers, and
sync are unsupported or unexecuted.

## Remaining Risks

The selected direction still excludes backup and transfer, so uninstall/data
clear/device loss can destroy local data until a separately governed continuity
design exists. M1 evidence is insufficient for production activation; M2
persistence, migration, recovery, lifecycle, and broader evidence remain.

## Git State

Branch `codex/android-m1-disposable-persistence-review`; HEAD remains
`04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df`; all candidate changes are unstaged;
no commit, merge, push, PR, archive/reset, distribution, release, or M2 work.

## Engineer Completion Status

`completed_with_follow_up`: exact disposable implementation/evidence and the
Founder decision are complete. Production activation, M2, continuity, signing,
distribution, and release remain separately governed follow-up work.

## Authorized Integration Closeout

`ANDROID-M1-INTEGRATION-CLOSEOUT-001` was recorded through the supported
terminal-follow-up path. The exact 58-path candidate plus bounded closeout delta
was committed as `d0e38640a53361c281d65b4befb6208b33f84346`. Closeout added only
the exact M1 archive prefix and neighboring-prefix regression, factual
authorization/manifest text, and removal of one trailing space from
`scripts/android-m1-native-review.ps1`; no command or runtime behavior changed.

Canonical verification passed after that commit at non-workflow digest
`26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e`.
Official archive/reset-to-idle, feature publication, non-fast-forward `develop`
integration, merged verification, and final publication remain orchestration
steps rather than product changes.
