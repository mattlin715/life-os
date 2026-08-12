# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest implemented: 5bc6ab9e3a057d55088c09da6da03b7b9661e88d99ba229238accc1a249693ec
- Created at: 2026-08-12T12:25:00+09:00
- Updated at: 2026-08-12T12:25:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented one package-only Windows Founder dogfooding path for the unchanged schema-v4 application. The additive Tauri override has a distinct product name, bundle identifier, window title, and current-user NSIS target. A fail-closed build script validates the source boundary, builds without signing, selects one fresh installer, copies it to an ignored review directory, writes an exact six-field content-free manifest from closed bytes, and verifies it. No installer was launched or installed.

## Existing System Areas Inspected

- Normal `src-tauri/tauri.conf.json` and Tauri configuration schema.
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts` relative database URL.
- Installed `tauri-plugin-sql` path mapping and Tauri desktop `app_config_dir` identifier mapping.
- `src-tauri/src/sqlite.rs` production schema constant.
- `scripts/verify.ps1`, local toolchain loader, `.gitignore`, architecture/16, and development runbooks.

## Files Added

- `src-tauri/tauri.founder-dogfood.conf.json`
- `scripts/build-founder-dogfood-package.ps1`
- `scripts/founder-dogfood-package.mjs`
- `scripts/founder-dogfood-package.node-test.mjs`
- `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md`

## Files Modified

- `.gitignore`
- `scripts/verify.ps1`
- `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md`
- repository workflow artifacts

## Files Deleted

none

## Behavior Changed

The repository can now produce a local unsigned Founder-only NSIS installer through one explicit command. The installed application will present the private Founder product/window identity and resolve the relative database under identifier `com.lifeos.founderdogfood`, separate from normal `com.lifeos.app`. Normal development and product behavior are unchanged.

## Data Model Impact

No data-model change. The package retains schema v4 and the existing relative `sqlite:life-os.db`; only the Tauri application identity/profile changes for this package flavor.

## Migration Impact

None. No DDL, `user_version`, startup maximum, or migration activation change.

## Provenance Impact

No Life OS provenance change. The generated build manifest is content-free artifact-integrity metadata, not product provenance.

## Historical Context Impact

None.

## Consent Impact

None.

## Provider Transmission Impact

None.

## Tests Added

Five Node tests cover valid source separation, collision/schema/forbidden-surface refusal, exact manifest creation/validation, path/extra-field/size/digest refusal, and the real repository allowlist/source contract.

## Tests Executed

- `node --test scripts/founder-dogfood-package.node-test.mjs`: 5/5 passed.
- `node scripts/founder-dogfood-package.mjs verify-source --root .`: passed; version 0.2.0, dev identifier `com.lifeos.app`, Founder identifier `com.lifeos.founderdogfood`, schema 4.
- `powershell ... build-founder-dogfood-package.ps1`: passed; production frontend and Windows release compilation succeeded; one unsigned NSIS bundle was produced and verified.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`: passed.
- Canonical verification: scheduled for the Validation phase and not yet claimed.

## Verification Results

Passed focused source, manifest, actual Windows bundle, and Clippy checks. Canonical verification is not yet recorded. Manual installation, startup, restart, multilingual/narrow/keyboard review, uninstall, and data-retention observation are intentionally not run by the engineer and remain Founder-owned.

## Manual Verification Required

Yes. The Founder must review the exact installer one bounded step at a time. No manual step is currently claimed passed.

## Documentation Updates

Architecture/16 now records audit promotion facts and the distinct current R1 implementation state. The new development runbook documents identity/profile separation, build and verification commands, exact manifest fields, manual review, retention observation, and all non-distribution boundaries.

## ADR Impact

No ADR change. Existing accepted ADR decisions are preserved.

## Deviations From Plan

none

## Known Limitations

- Unsigned Windows warnings are expected.
- Cross-machine byte-identical NSIS output is not claimed.
- Manifest Git SHA identifies the baseline commit while the package configuration remains an unstaged review diff.
- Identifier separation is statically and build verified; live profile behavior and uninstall retention still require Founder observation.

## Remaining Risks

Manual installation could reveal Windows-specific shortcut, coexistence, WebView, or uninstall behavior not covered by static/package tests. The package must remain local and undistributed. A manual pass still does not authorize promotion, release, schema v5, Phase 4, or Android work.

## Git State

Branch `codex/windows-founder-dogfooding-package-r1`; HEAD `c7e767a397995c1d96daebcafa08eb1e578e356e`; no upstream configured; no staged files. Only the exact package/documentation/workflow allowlist is changed. Generated `.artifacts/` and `src-tauri/target/` output is ignored.

## Engineer Completion Status

completed_with_follow_up
