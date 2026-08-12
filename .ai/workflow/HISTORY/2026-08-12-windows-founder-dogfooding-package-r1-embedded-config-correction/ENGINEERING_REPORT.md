# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-embedded-config-correction
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest implemented: 52f5e7dea8a660fd3ae94878d7d896d5df0d1e80ba4a3152d07c17f5759dcad3
- Created at: 2026-08-12T22:25:00+09:00
- Updated at: 2026-08-12T22:25:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Corrected the installed-package embedded configuration defect. The build now compiles only the final `life-os` binary with exact Founder `TAURI_CONFIG`, Tauri production `custom-protocol`, and Windows GUI/CRT linker flags; restores prior environment state; validates embedded Founder identifier/title and PE subsystem 2; bundles with the same override; revalidates; and emits a new exact manifest. No product/runtime source changed.

## Existing System Areas Inspected

Founder Step 3 live evidence, package scripts/tests, Tauri `TAURI_CONFIG` build/codegen behavior, Tauri `custom-protocol` production feature, PE headers, generated binary markers, and package runbook.

## Files Added

none beyond the existing R1 files and this correction workflow archive.

## Files Modified

- `scripts/build-founder-dogfood-package.ps1`
- `scripts/founder-dogfood-package.mjs`
- `scripts/founder-dogfood-package.node-test.mjs`
- `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md`
- workflow artifacts

## Files Deleted

none

## Behavior Changed

The next reviewed installer is compiled to show `Life OS — Founder Dogfood (Private)` and load embedded frontend assets without a dev server. Live confirmation remains pending until Founder-controlled uninstall/reinstall/relaunch.

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

Package suite now has 8 tests, including omission of `TAURI_CONFIG`, embedded identity/title markers, PE GUI/CUI/malformed classification, manifest failures, schema/identity drift, forbidden override surface, and real repository contract.

## Tests Executed

- Package tests: 8/8 passed.
- Global `RUSTFLAGS` attempt: failed truthfully because flags reached proc-macro DLLs; workflow entered revision cycle 1.
- Revised final-binary build with exact `TAURI_CONFIG` + `tauri/custom-protocol`: passed.
- Pre/post-bundle embedded identity/title and PE GUI verification: passed.
- New installer manifest: passed; SHA-256 `4fbc234d3bbcdc496ced2bd074d9949bc3bb047567339c3d9c61ca4daec08895`.
- Canonical verification: scheduled for Validation and not yet claimed.

## Verification Results

Focused revised build passed after one bounded workflow revision. Canonical verification remains to run. Manual retest remains not run.

## Manual Verification Required

Yes. Founder must close/uninstall the known-bad installed package, reinstall the corrected artifact, and repeat launch/restart/UI checks.

## Documentation Updates

Runbook records the invalidated localhost artifact, the exact `TAURI_CONFIG` and production custom-protocol requirement, environment restoration, and retained manual boundary.

## ADR Impact

none

## Deviations From Plan

One bounded revision: global `RUSTFLAGS` affected dependency DLL links. The final plan scopes linker flags only to the final binary and supplies the exact config through documented `TAURI_CONFIG`.

## Known Limitations

Static embedded markers and feature selection strongly cover the defect, but only reinstall/relaunch can prove Windows runtime behavior and data-profile separation end to end.

## Remaining Risks

The known-bad installed process/profile state must be handled manually. The automation does not close or uninstall it.

## Git State

Branch `codex/windows-founder-dogfooding-package-r1`; HEAD `c7e767a`; no upstream or staged files; exact R1/correction paths only. Installed package remains the earlier failed artifact until Founder replacement.

## Engineer Completion Status

completed_with_follow_up
