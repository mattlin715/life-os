# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1
- Created at: 2026-08-12T21:25:00+09:00
- Updated at: 2026-08-12T21:25:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, `failed`, or `cancelled`.

## Sprint ID

2026-08-12-windows-founder-dogfooding-package-r1

## Mission

Build the bounded Windows Founder dogfooding package without changing schema-v4 product semantics.

## Starting Commit

c7e767a397995c1d96daebcafa08eb1e578e356e

## Ending Commit Or Working-Tree State

Unstaged package-only working tree on branch `codex/windows-founder-dogfooding-package-r1` at unchanged HEAD `c7e767a397995c1d96daebcafa08eb1e578e356e`; generated installer and manifest are ignored.

## Final Status

completed_with_follow_up

## Product Decision

Architecture/16's promoted audit authorizes one bounded Founder-only schema-v4 package. Implementation is complete and automated-verified. Founder manual install/start/restart/uninstall acceptance remains distinct; distribution, deployment, release, schema v5, Phase 4, and Android remain unauthorized.

## Engineering Summary

Added an additive Tauri Founder override, a fail-closed build/manifest contract, five focused tests, ignored artifact output, canonical-test registration, an operator runbook, and factual architecture reconciliation. Built one real unsigned current-user NSIS installer without installing or launching it.

## Behavior Changed

One new explicit repository command can build the existing product as `Life OS Founder Dogfood` with identifier `com.lifeos.founderdogfood`. The normal app remains `Life OS` / `com.lifeos.app`. No product feature or runtime behavior changed.

## Files Changed

Repository diff is limited to `.gitignore`; package override; three package scripts; `scripts/verify.ps1`; architecture/16; dev runbook; and this sprint's workflow evidence/archive. Exact final archive file count is recorded after repository-native archival.

## Tests

- Package focused tests: 5/5 passed.
- Workflow tests: 17/17 passed.
- Vitest: 41 files / 310 tests passed.
- Rust library: 189 passed.
- Backup/restore integration: 12 passed.
- Schema contract: 8 passed.
- Clippy all-targets with warnings denied: passed.
- Typecheck, frontend production build, Rust check, UTF-8, whitespace, secrets, Markdown links, and Constitution diff: passed.

## Repository Verification

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1` passed after the real package build. Workflow verification evidence records exit code 0 at HEAD `c7e767a` and working-tree digest `5bc6ab9e3a057d55088c09da6da03b7b9661e88d99ba229238accc1a249693ec`.

## Manual Verification

Not run. Founder owns the exact eight-step install/start/local-mock/restart/export/language/uninstall review. No manual pass is inferred from build or tests.

## Architecture Updates

Architecture/16 version 0.3 records the audit's feature/merge promotion, corrects stale unpromoted wording, distinguishes the current R1 implementation, and preserves every production/distribution fence.

## ADR Updates

No ADR updates.

## Documentation Synchronization

Added `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md` with profile separation, build/verification commands, manifest contract, honest reproducibility limits, manual matrix, retention observation, and non-distribution boundaries.

## Data And Migration Impact

No model, schema, migration, database, startup maximum, user data, app-data access, backup, restore, or retention implementation change. Production `SCHEMA_VERSION` remains 4.

## Provenance And Consent Impact

No Life OS provenance, historical context, consent, provider, or ContextPacket impact. The build manifest is content-free artifact-integrity metadata only.

## Risks

Unsigned warnings and machine/toolchain-specific installer bytes are expected. Live Windows coexistence, shortcut, restart, WebView, uninstall retention, narrow-window, keyboard, and language behavior still require Founder observation. The reviewed installer is local and must not be distributed.

## Deferred Items

Founder manual review; optional bounded corrections if evidence warrants; separate Founder diff acceptance and promotion authorization. Private Alpha, schema-v5 desktop activation, Phase 4, Android M0, deployment, and release are later independent gates.

## Human Decisions

No additional consequential decision was needed. The Founder had already authorized the exact package implementation; manual acceptance and promotion remain explicit future responses.

## Review Cycles

0. Product Review and Theory Alignment Review both approved the exact scope with manual follow-up; no corrective revision cycle was required.

## Workflow Lessons

Application identity is sufficient for profile separation because the existing relative SQLite URL is resolved under Tauri's identifier-derived app config directory. Package safety is strengthened by testing the exact override key surface and changed-path allowlist rather than adding runtime switches.

## Recommended Next Sprint

After Founder manual acceptance and an explicit promotion gate, separately evaluate bounded desktop schema-v5 production activation and packaged recovery evidence. Android Build Feasibility M0 remains parked until that desktop sequence is complete and promoted.

## Git Status

Branch `codex/windows-founder-dogfooding-package-r1`; HEAD `c7e767a397995c1d96daebcafa08eb1e578e356e`; no upstream; no staged files; exact package/documentation/workflow diff only; `.artifacts/` and `src-tauri/target/` ignored; no commit, push, merge, PR, deployment, distribution, or release.
