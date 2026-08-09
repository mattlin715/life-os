# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-09-phase-3c-slice4c6-legacy-v4-baseline-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: `a9bbf7cc687f0d8dcc4bda9e1d402bf309fac590`
- Working-tree digest implemented: `750b3e109086cc0841fd9c2d5a5089ba25962efd271893b0dbfbc6694ffdd943`
- Created at: 2026-08-09
- Updated at: 2026-08-09

## Implementation Summary

Implemented only Founder-authorized Slice 4C-6A. The existing private,
unregistered, path/connection-injected Evidence and Pattern writers now accept
exact migrated `legacy-v4-raw` pending/candidate baselines for explicit
confirmation or rejection in disposable exact-v5 fixtures. Strict
artifact-specific parsing and exact durable reconciliation preserve the raw
legacy predecessor and fail closed on unsupported or contradictory evidence.

## Existing System Areas Inspected

- `src-tauri/src/schema_v5_migration.rs`
- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_pattern_write.rs`
- `src-tauri/src/schema_v5_evidence_lifecycle.rs`
- `src-tauri/tests/fixtures/schema_v5/v4.sql`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- Accepted ADR-0009 and ADR-0011 boundaries already referenced by architecture/13

## Files Added

none

## Files Modified

- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_pattern_write.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- repository-required mutable workflow artifacts under `.ai/workflow/`

## Files Deleted

none

## Behavior Changed

- Exact migrated legacy pending Evidence can be confirmed or rejected through
  the existing private disposable writer.
- Exact migrated legacy candidate Pattern can be confirmed or rejected through
  the existing private disposable writer.
- Confirmation preserves revision content, digest, provenance, source and
  dependency edges, and imported review evidence; only the exact review/head
  state and guarded v4 status/action time change.
- Rejection purges content and the guarded v4 projection while retaining only
  authorized content-free review/lifecycle/tombstone facts.
- Pattern `legacy_unknown` provenance remains honest and is not injected into
  the schema-v4 projection.
- Pattern provenance source arrays are reconciled by unique exact set while the
  byte order of the raw legacy predecessor remains untouched.

## Data Model Impact

No DDL, schema object, table, column, trigger, index, or production data-model
change. Existing schema-v5 disposable representations are used unchanged.

## Migration Impact

No migration implementation or activation change. Fixtures are created only
through the promoted exact-v4 disposable migration core. Production
`SCHEMA_VERSION` and startup maximum remain 4.

## Provenance Impact

No provenance rewrite or relabeling. Exact known legacy provenance is
reconciled against normalized provenance. Missing legacy provenance remains
`legacy_unknown` with null provider/model/Harness/prompt/time fields.

## Historical Context Impact

No new historical retrieval, packet, transmission, or persistence behavior.
Existing exact legal ADR-0009 dependent consequences remain authoritative;
unsupported inbound relationships fail closed.

## Consent Impact

none

## Provider Transmission Impact

none

## Tests Added

Eight focused legacy-review tests across Evidence and Pattern cover:

- confirmation with known and `legacy_unknown` provenance;
- raw payload, digest, dependency, and imported-review preservation;
- rejection content/projection purge and content-free facts;
- unsupported unknown-field refusal;
- every applicable confirmation/rejection failure boundary and exact logical rollback;
- ambiguous-COMMIT exact pre-state, post-state, and third-state classification.

## Tests Executed

- `cargo test --manifest-path .\src-tauri\Cargo.toml --lib migrated_legacy -- --nocapture`: 8 passed.
- `cargo test --manifest-path .\src-tauri\Cargo.toml --lib schema_v5_migration::evidence_lifecycle::tests -- --nocapture`: 7 passed after the bounded verifier reconciliation.
- `cargo test --manifest-path .\src-tauri\Cargo.toml --lib`: 170 passed.
- `cargo clippy --manifest-path .\src-tauri\Cargo.toml --all-targets -- -D warnings`: passed.
- Canonical `scripts/verify.ps1`: pending workflow validation phase.

## Verification Results

- Focused legacy tests: passed.
- Complete Rust library suite: passed, 170 tests.
- Clippy all targets with warnings denied: passed.
- Canonical repository verification: pending next workflow phase.
- Desktop runtime verification: not applicable; no registered runtime path.

## Manual Verification Required

No desktop runtime/manual UI verification is recommended because the boundary
is private, unregistered, and disposable-only. Founder diff review remains
required before any promotion authorization.

## Documentation Updates

Architecture/13 version 4.3 records the exact 4C-6A implementation evidence,
limits, current unpromoted state, and continuing production-v4 fence.

## ADR Impact

No ADR status or decision changed. ADR-0009 and ADR-0011 behavior is preserved.

## Deviations From Plan

No authority or file-scope deviation. During full regression, the Pattern
read-only verifier was corrected to compare known legacy provenance source IDs
as an exact unique set while preserving the byte order of the raw predecessor;
canonical and invalidation regressions then passed.

## Known Limitations

- Slice 4C-6A does not complete migrated current-action parity.
- Slice 4C-6B remains required for legacy Evidence candidate correction,
  Reflection answer/skip/correction, and Context Recovery answer/skip.
- No production restart, real-user, runtime, or production schema-v5 evidence.
- No autonomous retry, replay, repair, rollback, cleanup, or candidate selection.

## Remaining Risks

Production migration/recovery must remain blocked until 4C-6B and every other
explicit exit gate are separately authorized, implemented, verified,
Founder-reviewed, and promoted. Disposable logical reconciliation does not by
itself prove real-user operational safety.

## Git State

- Branch: `codex/phase-3c-slice4c6-legacy-v4-baseline-current-action-design-gate`
- HEAD: `a9bbf7cc687f0d8dcc4bda9e1d402bf309fac590`
- Upstream: none
- Staged files: none
- Working tree: only the three authorized product/document paths and mutable
  workflow artifacts are modified.

## Engineer Completion Status

`completed_with_follow_up`: Slice 4C-6A implementation is complete; canonical
verification, terminal reviews, workflow archive/reset, Founder diff review,
and the separately gated Slice 4C-6B remain.
