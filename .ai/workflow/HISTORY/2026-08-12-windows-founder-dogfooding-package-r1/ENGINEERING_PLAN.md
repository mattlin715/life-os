# Engineering Plan

Status: approved

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-12T12:05:00+09:00
- Updated at: 2026-08-12T12:05:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement only an unsigned, local, non-distributed Windows Founder package for the unchanged schema-v4 application. Preserve the normal configuration and all product/runtime semantics, enforce a distinct identity/profile, emit only ignored output and a content-free manifest, and stop before installation or promotion.

## Existing Implementation Understanding

- `src-tauri/tauri.conf.json` defines `Life OS` / `com.lifeos.app`, production frontend build commands, and normal desktop window dimensions.
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts` uses relative `sqlite:life-os.db`.
- Promoted `tauri-plugin-sql` maps relative SQLite URLs under Tauri `app_config_dir`; Tauri resolves that directory as the OS config directory joined with the configured bundle identifier. Therefore a distinct identifier provides the required profile boundary without a runtime code change.
- `scripts/verify.ps1` is the canonical repository verification path and currently has no package-contract test.
- Build output under `src-tauri/target` is already ignored; a separate review-artifact directory needs an explicit ignore rule.

## Affected Modules

Anticipated exact implementation allowlist before edits:

1. `.gitignore`
2. `scripts/build-founder-dogfood-package.ps1`
3. `scripts/founder-dogfood-package.mjs`
4. `scripts/founder-dogfood-package.node-test.mjs`
5. `scripts/verify.ps1`
6. `src-tauri/tauri.founder-dogfood.conf.json`
7. `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md`
8. `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md`
9. Repository-required current workflow artifacts and the terminal archive under `.ai/workflow/HISTORY/2026-08-12-windows-founder-dogfooding-package-r1/`.

No other path is authorized. Generated `.artifacts/windows-founder-dogfood-r1/` and Tauri `target` outputs are ignored and are not repository diffs.

## Proposed Design

1. Add a Tauri merge override that changes only product name, identifier, window title, and NSIS/current-user bundle target while repeating the normal window dimensions to avoid array-merge drift.
2. Add a Node package-contract module with pure source/manifest validators and a CLI. It validates the unchanged base identity, distinct Founder identity, schema 4, fixed relative database name, no forbidden override keys, exact changed-path boundary, ignored output, and exact six-field manifest.
3. Add a PowerShell build entry point. It loads the repository toolchain, validates source first, invokes `tauri build` with the override and `--bundles nsis --ci --no-sign`, requires exactly one fresh Founder installer, copies it to a deterministic ignored review directory, writes the manifest from closed bytes, and verifies it again.
4. Add Node focused tests over synthetic files/artifacts and run them from canonical verification.
5. Synchronize architecture/16 promotion facts and add a factual install/manual-review guide. Do not claim cross-machine byte reproducibility, signing, release readiness, uninstall cleanup, or manual acceptance.

## Alternatives Considered

- Reusing the normal identifier was rejected because it could resolve the ordinary development profile.
- Adding a runtime database-path switch was rejected because it would alter protected product/runtime surfaces and create configuration risk.
- Copying or mutating the ordinary dev database was rejected as unnecessary and unsafe.
- MSI plus NSIS was rejected for R1 because it creates multiple artifact/lifecycle paths; one current-user NSIS installer is the smallest bounded slice.
- Byte-reproducible installers across machines were not claimed because Windows/NSIS toolchain metadata can vary; the repository command and validation contract are deterministic, and the exact produced bytes are identified by SHA-256.

## Data Lifecycle Impact

The package creates a distinct Tauri config directory only when the Founder explicitly installs and launches it. The build itself accesses no app-data directory or database. Uninstall retention is observed manually; the implementation does not delete either dogfooding or development data.

## SQLite Or Migration Impact

None. Production `SCHEMA_VERSION` and `user_version` remain 4. No DDL, initialization, migration, backup, restore, or database command changes.

## Provenance Impact

No product provenance changes. The package manifest is content-free artifact-integrity metadata and is not Life OS user-data provenance.

## Historical Context Impact

None. Historical retrieval, selection, packet, and generated-artifact behavior are untouched.

## Consent Impact

None. Consent state, disclosure, and invalidation semantics are untouched.

## Provider Transmission Impact

None. No provider configuration or transport path changes.

## Import And Export Impact

None. Experience-only export/import behavior and wording remain unchanged.

## Test Strategy

- Unit-test valid source contract and valid manifest.
- Reject base identity drift, identifier collision, forbidden updater/signing/network/deployment keys, schema-version drift, database-path drift, malformed or extra manifest fields, filename path leakage, Git/target/version mismatch, size mismatch, checksum mismatch, and non-ignored output.
- Execute actual source validation in the repository from the focused test.
- Build the real unsigned NSIS artifact once, then run exact manifest verification.
- Compare all tracked and non-ignored untracked paths to this allowlist before completion.

## Repository Verification Strategy

Run focused Node package tests, the package source verifier, Clippy with warnings denied for the existing Rust target, and `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Confirm Constitution and protected product surfaces have no diff, `SCHEMA_VERSION = 4`, generated output is ignored, and workflow validates before archival.

## Manual UI Verification

Founder-owned and required after automated completion. Review one bounded step at a time: installer identity/separation; disposable-profile install; launch without dev server; local-mock journey; close/restart; Experience-only export and absence of forbidden controls; EN/ZH/JA narrow-window and keyboard behavior; uninstall retention observation and ordinary dev-profile integrity.

## Rollback Or Recovery Strategy

No runtime rollout occurs. Before manual acceptance, rollback is deletion of ignored build artifacts and reversion of the unstaged package-only diff. The distinct identifier prevents package launch from selecting the ordinary dev profile. The build script fails closed on ambiguous artifacts or contract mismatch and never installs or uninstalls anything.

## Documentation Impact

Update architecture/16 only for factual promotion/current-state reconciliation and add `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md` with exact build, verification, manual review, data-profile, retention, and non-distribution boundaries.

## ADR Impact

No new ADR. Architecture/16 already contains Founder-approved packaging direction and exact scope; this sprint implements that bounded Book One slice without changing governance policy.

## Risk Level

medium. Product semantics are untouched, but Windows packaging and application identity determine data-profile safety. Static/source checks plus a real package build reduce the risk; Founder install/uninstall observation remains necessary.

## Escalation Decision

No escalation. The Founder already authorized the exact implementation and review boundary. Any need to touch product runtime, schema, data paths, updater/signing, or distribution returns to `human_decision_required`.
