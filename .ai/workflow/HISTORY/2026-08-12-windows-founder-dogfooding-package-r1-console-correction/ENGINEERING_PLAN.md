# Engineering Plan

Status: approved

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-console-correction
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest reviewed: current package working tree
- Created at: 2026-08-12T21:32:00+09:00
- Updated at: 2026-08-12T21:32:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Approved with conditions: correct only package scripts/tests/docs, prove PE GUI subsystem 2, and keep all product/runtime source unchanged.

## Existing Implementation Understanding

The first `src-tauri/target/release/life-os.exe` parsed as PE32+ subsystem 3. The source lacks a release `windows_subsystem` attribute, but that protected source must not be changed. Tauri supports separate `build` and `bundle` commands, so the final package binary can be compiled with a package-local linker argument and verified before bundling.

## Affected Modules

Only `scripts/build-founder-dogfood-package.ps1`, `scripts/founder-dogfood-package.mjs`, `scripts/founder-dogfood-package.node-test.mjs`, `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md`, and required workflow evidence. Existing R1 diff remains unchanged otherwise.

## Proposed Design

Run the frontend build, then `cargo rustc --release --bin life-os -- -C link-arg=/SUBSYSTEM:WINDOWS`, parse the resulting PE header and require subsystem 2, then invoke `tauri bundle` with the existing override. Add a pure PE parser and synthetic GUI/CUI/malformed tests. Explicitly remove only the prior ignored R1 review directory before the authorized rebuild.

## Alternatives Considered

Changing `src-tauri/src/main.rs` was rejected because it is a protected ordinary runtime source. Post-hoc unverified binary patching was rejected. Ignoring the console was rejected because it violates the exact package contract.

## Data Lifecycle Impact

No data lifecycle impact.

## SQLite Or Migration Impact

No SQLite or migration impact.

## Provenance Impact

No provenance impact.

## Historical Context Impact

No historical context impact.

## Consent Impact

No consent impact.

## Provider Transmission Impact

No provider impact.

## Import And Export Impact

No import/export impact.

## Test Strategy

Synthetic PE32+ fixtures for subsystem 2 and 3 plus malformed/truncated cases; focused package suite; real rebuilt binary inspection; manifest verification; canonical verification.

## Repository Verification Strategy

Run focused tests, real rebuild, Clippy, canonical verify, allowlist/protected-surface diff, and workflow validation/archive.

## Manual UI Verification

Founder remains owner; manual review starts only after the corrected package is ready.

## Rollback Or Recovery Strategy

The old artifact is ignored and may be removed by exact path for rebuild. No installed state exists. Failure before the final move leaves no accepted review artifact.

## Documentation Impact

Update the runbook to document automated PE subsystem proof and the split build/bundle path.

## ADR Impact

No ADR change.

## Risk Level

medium: Windows linker/package behavior is safety-relevant, but correction is isolated to the build path and directly byte-verified.

## Escalation Decision

No escalation; correction stays within the package-specific build/test surfaces already authorized.
