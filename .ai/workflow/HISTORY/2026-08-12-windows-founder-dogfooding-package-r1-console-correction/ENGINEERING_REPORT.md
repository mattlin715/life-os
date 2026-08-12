# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-console-correction
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest implemented: 21e70ef78e08482e9c0b9d9402ef32c98fcc4d8993afbc24019710b657987e1e
- Created at: 2026-08-12T21:45:00+09:00
- Updated at: 2026-08-12T21:45:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Corrected the package build to compile only the final binary with `/SUBSYSTEM:WINDOWS` and `/ENTRY:mainCRTStartup`, require PE subsystem 2 before and after Tauri bundling, and test synthetic GUI/CUI/malformed PE inputs. The failed CUI artifact was moved under an ignored `superseded-cui-*` directory and is not the review package.

## Existing System Areas Inspected

Package scripts/tests, Tauri split build/bundle CLI, release PE headers, and existing R1 documentation/workflow evidence.

## Files Added

none beyond the already-listed R1 files and this correction workflow archive.

## Files Modified

- `scripts/build-founder-dogfood-package.ps1`
- `scripts/founder-dogfood-package.mjs`
- `scripts/founder-dogfood-package.node-test.mjs`
- `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md`
- workflow artifacts

## Files Deleted

none

## Behavior Changed

The accepted review installer now embeds a Windows GUI-subsystem application rather than a console-subsystem application. No product/runtime source changed.

## Data Model Impact

none

## Migration Impact

none

## Provenance Impact

none

## Historical Context Impact

none

## Consent Impact

none

## Provider Transmission Impact

none

## Tests Added

Added one focused PE-classification test, bringing the package suite to 6 tests.

## Tests Executed

- Package suite: 6/6 passed.
- First one-flag build: failed with LNK2019, preserved as truthful corrective evidence.
- Two-flag final binary build: passed.
- Pre/post-bundle PE inspection: subsystem 2 passed.
- Corrected NSIS build and exact manifest verification: passed.
- Canonical verification: scheduled for Validation and not yet claimed.

## Verification Results

Focused correction and real package build passed after one bounded implementation correction. Canonical verification remains to run. Manual install remains not run.

## Manual Verification Required

Yes; Founder install/start/restart/uninstall review is still required.

## Documentation Updates

Runbook now records GUI subsystem and explicit CRT entry-point compilation plus byte-level validation.

## ADR Impact

none

## Deviations From Plan

Initial `/SUBSYSTEM:WINDOWS` alone failed because MSVC selected `WinMain`; the package command now explicitly retains Rust's `mainCRTStartup`. No protected source was changed.

## Known Limitations

The linker seam is Windows/MSVC-specific and intentionally confined to this Windows package. Founder launch verification remains necessary.

## Remaining Risks

Manual startup is needed to prove actual visible-console behavior, shortcut launch, and normal app operation.

## Git State

Branch `codex/windows-founder-dogfooding-package-r1`, HEAD `c7e767a`, no upstream, no staged files, package/workflow allowlist only.

## Engineer Completion Status

completed_with_follow_up
