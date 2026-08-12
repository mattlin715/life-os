# Engineering Plan

Status: approved

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-embedded-config-correction
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest reviewed: 812942ca4eb118d909c6cced8d1aff75c5773ceea4b41ee09da0b8e91a0da31c
- Created at: 2026-08-12T22:08:00+09:00
- Updated at: 2026-08-12T22:08:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is approved with conditions. Correct only the package build/contract/test/runbook path, embed the Founder config through Tauri build, preserve GUI subsystem 2, restore environment state, and require manual reinstall/retest.

## Existing Implementation Understanding

Founder Step 3 showed `Life OS`, localhost refusal, and installed process path under the Founder installer directory. The prior script used direct Cargo compilation followed by `tauri bundle`; Tauri configuration is embedded by build-time context generation, so bundling alone cannot replace the executable's default embedded config.

## Affected Modules

`scripts/build-founder-dogfood-package.ps1`, `scripts/founder-dogfood-package.mjs`, `scripts/founder-dogfood-package.node-test.mjs`, `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md`, and workflow artifacts. Existing R1 files remain in the overall allowlist; no protected product path is added.

## Proposed Design

Build the frontend; load the exact Founder override JSON into a temporary `TAURI_CONFIG`; compile only `--bin life-os` with `cargo rustc` and the two GUI/CRT linker arguments; restore prior `TAURI_CONFIG` in `finally`; verify PE subsystem 2 and embedded Founder identity/title markers; bundle with the same override; verify PE again; add source-contract checks that direct compilation cannot occur without exact `TAURI_CONFIG`; rebuild the exact ignored review artifact and manifest.

## Alternatives Considered

Changing `src-tauri/src/main.rs` is unnecessary and outside scope. Starting a dev server would mask the defect and is forbidden. Global `RUSTFLAGS` is rejected because it reaches proc-macro DLL dependencies and caused LNK2019. Direct final-binary compilation is acceptable only with exact `TAURI_CONFIG`, which Tauri build/codegen documents as the config-merge input.

## Data Lifecycle Impact

No product data impact. The currently installed failed package remains untouched until Founder-controlled close/uninstall.

## SQLite Or Migration Impact

No schema or migration impact.

## Provenance Impact

No provenance impact.

## Historical Context Impact

No historical-context impact.

## Consent Impact

No consent impact.

## Provider Transmission Impact

No provider impact.

## Import And Export Impact

No import/export impact.

## Test Strategy

Extend source validation for exact `TAURI_CONFIG`, binary-only compile, environment restoration, and same-override bundle; add embedded marker validation; rebuild real package; inspect PE and manifest; rerun canonical verification.

## Repository Verification Strategy

Focused tests, actual package build, PE validation, exact manifest verification, Clippy, canonical verify, protected-surface diff, changed-path allowlist, workflow archive/idle.

## Manual UI Verification

Founder-owned. Close the failed window, uninstall the failed package, reinstall the corrected package, and repeat launch checks one step at a time.

## Rollback Or Recovery Strategy

The earlier ignored artifact remains under `superseded-cui-*`; the current incorrect artifact may be moved to a separately labelled ignored superseded directory before rebuilding. No automatic installed-state action occurs.

## Documentation Impact

Update runbook with combined Tauri build rationale and record that manual Step 3 caught and invalidated the split-build artifact.

## ADR Impact

No ADR change.

## Risk Level

medium: packaging configuration determines both UI identity and frontend source. The fix is package-only but requires real reinstall evidence.

## Escalation Decision

No Founder escalation; exact corrective work is bounded by already authorized R1 package surfaces.
