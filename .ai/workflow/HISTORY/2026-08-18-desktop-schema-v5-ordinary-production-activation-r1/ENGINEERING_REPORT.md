# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-18-desktop-schema-v5-ordinary-production-activation-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest implemented: 7fda57e5385e31835eed2dac4f54ce2c8e91653861dd0e66243438eef2fdc60b
- Created at: 2026-08-18T21:02:00+09:00
- Updated at: 2026-08-18T21:02:00+09:00

## Implementation Summary

Implemented the ordinary `com.lifeos.app` schema-v5-capable desktop path at
application version 0.3.0 by reusing the promoted Candidate activation,
filesystem, migration, backup/restore, typed runtime, artifact writers, and
ADR-0009 persistence core. Missing profiles initialize exact v5; exact v4 is
blocked behind explicit disclosed migration; v2/v3 stabilize under the legacy
v4 contract before that separate action; invalid or ambiguous evidence fails
closed. Built but did not install or launch one unsigned ignored ordinary-
identity disposable review package.

## Existing System Areas Inspected

`src-tauri/src/sqlite.rs`, `filesystem_safety.rs`, all `schema_v5_*` modules,
Tauri command registration and configs, TypeScript startup/store adapters,
App migration/backup/readiness UI, i18n, package scripts, CI/verification, and
architecture/13, /15, /16, /17 plus accepted ADR-0007, ADR-0009, ADR-0011.

## Files Added

- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
- `scripts/build-ordinary-schema-v5-review.ps1`
- `scripts/ordinary-schema-v5-review-package.mjs`
- `scripts/ordinary-schema-v5-review-package.node-test.mjs`
- `src-tauri/tauri.ordinary-schema-v5-review.conf.json`

## Files Modified

Workflow evidence; `.github/workflows/check.yml`; `docs/00_Index.md`;
architecture/13, /15, /16, /17; dev/09; version declarations; shared Tauri
activation/migration/legacy-startup code; TypeScript startup/readiness/store
adapters; App migration/backup/readiness UI and i18n; Founder and ordinary
package contracts/build scripts; `scripts/verify.ps1`.

## Files Deleted

none.

## Behavior Changed

- Default ordinary desktop builds are schema-v5 capable.
- Missing ordinary databases initialize exact v5 through the shared core.
- Exact v4 remains read-only/migration-required until explicit authorization.
- v2/v3 stabilize to exact v4 before separate migration disclosure.
- Exact verified v5 uses the promoted typed runtime routes.
- Inspector R1 remains read-only but reports maximum 5 and exact-v5 truth in
  v5 builds; no-default legacy builds remain maximum 4.
- Ordinary UI receives profile-accurate EN/zh-TW/ja migration, backup, restore,
  cancellation, blocked, retention, and provider-boundary copy.
- Founder Candidate builds explicitly exclude the ordinary default feature.

## Data Model Impact

No DDL or schema-object change. The accepted exact v5 DDL and guarded v4
projection remain single-source. Legacy `sqlite.rs::SCHEMA_VERSION` remains 4.

## Migration Impact

The previously private exact-v4-to-v5 migration is now reachable under the
ordinary identity only through explicit per-operation authorization. New
receipts record 0.3.0 while the verifier accepts exact promoted 0.2.0 receipts
through current, subject to every unchanged contract predicate.

## Provenance Impact

No semantic change. Existing exact authorship, immutable revisions, lifecycle
events, dependency edges, receipts, packet snapshots, and actual-use
provenance are reused.

## Historical Context Impact

No retrieval, selection, eligibility, packet, output, or Phase 4 change. The
promoted ADR-0009 typed v5 persistence and cascade behavior is routed unchanged.

## Consent Impact

No provider-consent change. Migration authorization remains a separate local
per-operation action; opening or cancelling disclosure is not authorization.

## Provider Transmission Impact

None.

## Tests Added

Ordinary package source/binary contracts; ordinary startup and v2/v3
stabilization; feature-aware readiness exact-v5/legacy-v4 behavior; identity,
version, copy, cancel, backup/restore, and Candidate regression assertions.

## Tests Executed

- Focused Vitest readiness/i18n: 42 passed.
- Focused readiness Rust tests: 7 all-features and 7 no-default passed.
- Founder/ordinary package contract tests: 3 passed.
- Clippy all targets/all features with warnings denied: passed.
- Canonical `scripts/verify.ps1`: passed before final package-validator
  correction and is rerun at the validation gate.
- Ordinary package build: passed without install or launch.

## Verification Results

Initial canonical run failed only because the old Founder package changed-path
contract did not yet recognize the active ordinary successor sprint. Its
bounded successor allowlist was synchronized and the next canonical run passed.
The ordinary binary contract was then corrected to verify the embedded review
title and typed command surface rather than requiring an identifier literal
that release optimization does not preserve; identity remains exact in source
and bundle configs. Final canonical verification is required at the validation
record for the final digest.

## Manual Verification Required

Yes. Founder-owned Phase A must use the exact ignored installer in a disposable
Windows account, VM, or Sandbox, one step at a time. Phase B real-profile access
is not authorized and requires a separate exact Founder response after Phase A.

## Documentation Updates

Candidate promotion facts were reconciled. Architecture/18 records ordinary
activation, state machine, shared core, threat model, review gate, and rollback.
Dev/10 records the ignored package and manual boundaries. Index and related
Book One documents were synchronized factually.

## ADR Impact

No ADR status or decision changed. ADR-0007, ADR-0009, and ADR-0011 are applied.

## Deviations From Plan

Inspector R1 required a feature-aware exact-v5 classification to avoid false
maximum-4/unavailable disclosure. The existing Founder package successor
allowlist required the current sprint paths. Release optimization does not
preserve the ordinary identifier literal, so binary validation uses title and
typed-command markers while source/bundle contracts prove identity. These were
bounded within the authorized package/readiness boundary and added no write
authority.

## Known Limitations

Disposable manual Phase A is not yet run. The Founder's real ordinary profile
has not been accessed or migrated. Export remains Experience-only. No Android,
Private Alpha distribution, updater, telemetry, deployment, or release exists.

## Remaining Risks

Real Windows process/quiescence, install ordering, v4 cancel/migrate/restart,
old-binary refusal, disposable restore, fresh-v5, uninstall retention, and
three-language/narrow-window/keyboard behavior still require Founder Phase A.
Real-profile Phase B remains a separate high-risk decision even after Phase A.

## Git State

Branch `codex/desktop-schema-v5-ordinary-production-activation-r1`; HEAD
`44ec6d56d645829488aa73d0b92bcf72b19487f4`; unstaged allowlisted changes;
no staged files; no upstream; no commit, push, merge, PR, deployment, or release.
Ignored package output is outside Git.

## Engineer Completion Status

completed_with_follow_up.
