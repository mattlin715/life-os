# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-09-phase-3c-remaining-lifecycle-parity-closure-audit
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: `1428f610c161eb89b57d4c9d44da6fd99be7682b`
- Working-tree digest implemented: `a0cfca85bc861a806cd3d10ef4be00760cc41986699bf0da2adf361a0e897060`
- Created at: 2026-08-09T00:00:00.000Z
- Updated at: 2026-08-09T00:00:00.000Z

## Implementation Summary

Extended the existing private, unregistered, path/connection-injected
Experience schema-v5 writer so an exact-current Experience correction applies
complete same-source ordinary artifact consequences in its existing disposable
transaction. Active Evidence, Reflection, Pattern, and Context Recovery heads
retain exact immutable content/provenance/review/source-dependency facts,
become invalidated/ineligible, and lose only their guarded schema-v4
projections. Historical Questions first prove schema-v4/normalized-v5 parity
and then follow the promoted ADR-0009 cascade.

## Existing System Areas Inspected

The promoted Experience, Evidence, Reflection, Pattern, Context Recovery,
Historical Question, migration, schema contract, lifecycle, projection,
operation-manifest, and COMMIT classification code; schema-v5 DDL and exact-v4
fixtures; ADR-0007/0009/0011; architecture/09/10/12/13; and the current
schema-v4 Experience mutation behavior.

## Files Added

None.

## Files Modified

- `src-tauri/src/schema_v5_experience_write.rs`
- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_reflection_write.rs`
- `src-tauri/src/schema_v5_pattern_write.rs`
- `src-tauri/src/schema_v5_context_recovery_write.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- Exact current sprint workflow artifacts under `.ai/workflow/`

## Files Deleted

None.

## Behavior Changed

No production or user-visible behavior changed. The private disposable
Experience correction path no longer refuses a valid source merely because
ordinary artifacts exist. It validates their exact state, advances the source,
adds one deterministic invalidation fact per active artifact, removes its v4
projection, cascades exact affected Historical Questions, and reconciles the
complete database before commit. Already invalidated artifacts remain
unchanged across later source corrections. Parent deletion retains its complete
source-scoped purge behavior.

## Data Model Impact

No DDL or schema contract changed. Existing tables, relationship types,
lifecycle states, and guarded projections are used. Production remains schema
v4.

## Migration Impact

None in production. Tests create exact-v5 disposable fixtures through the
promoted migration core. No production `SCHEMA_VERSION`, startup maximum,
`user_version`, fresh-v5 initialization, or real database path changed.

## Provenance Impact

Ordinary artifact content, authorship, provenance, review state, current
revision, and the old exact `derived_from_experience` edge remain byte-exact.
One deterministic system lifecycle event references that exact dependency and
records `source_experience_revision_superseded`. No authorship rewrite,
confirmation, rejection, recalculation, source reselection, or dependency
rebinding occurs.

## Historical Context Impact

The affected Historical Question identity set must match across schema-v4
authority and normalized schema-v5 dependencies. Exact current-Experience and
packet-item source/artifact revision representations are checked before the
existing ADR-0009 cascade removes the generated artifact, packet snapshot,
actual-use provenance, lifecycle link, dependencies, and tied successful
consent/transmission facts. Unrelated unsuccessful audit metadata is not
touched. Context Recovery remains categorically excluded from historical use.

## Consent Impact

No new consent and no policy change. Only consent/transmission records already
tied to a stale generated artifact follow the approved cascade.

## Provider Transmission Impact

None. No provider, ContextPacket, transport, source eligibility, or runtime
path changed.

## Tests Added

Focused Experience regressions now cover the full authorized active-state
matrix; byte-exact retained content/provenance/review/dependency facts; exact
single invalidation events; no rebinding; repeated source correction without
duplicate events; Historical Question representation mismatch refusal;
ADR-0009 cascade; rollback after ordinary consequences; exhaustive parent
deletion; and ambiguous-COMMIT exact pre/post classification. The artifact
verifier extensions also run through every existing module regression.

## Tests Executed

- `cargo test --lib experience_write::tests -- --nocapture`: passed, 21/21.
- `cargo test --lib -- --nocapture`: passed, 162/162.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `git diff --check`: passed.
- Canonical `scripts/verify.ps1`: passed with 17 workflow tests, 26 Vitest
  files / 204 tests, 162 Rust library tests, 12 backup/restore integration
  tests, 8 schema-contract integration tests, TypeScript typecheck, frontend
  production build, Rust check, repository hygiene checks, and no Constitution
  diff.

## Verification Results

Focused tests, the full Rust library suite, Clippy, diff hygiene, and canonical
repository verification pass on the reported product diff.

## Manual Verification Required

No desktop runtime path exists for this private unregistered disposable-only
boundary. Founder diff review is required; desktop runtime manual verification
is not proposed.

## Documentation Updates

Architecture/13 version 4.1 records the Founder-authorized Slice 4C-5
working-tree evidence, exact consequence contract, test evidence, and all
retained production/later-slice fences.

## ADR Impact

No ADR status or decision changed. ADR-0007 provenance, ADR-0009 stale-source
cascade, and ADR-0011 append-only/no-rebinding/ordinary-invalidation rules are
implemented without expansion.

## Deviations From Plan

None. During local formatting, an unrelated formatting-only change to the
Historical Question writer was detected and restored before reporting; it is
not part of the final diff.

## Known Limitations

This is disposable transaction evidence only. It does not complete migrated
legacy-v4 baseline current-action parity, production restart recovery,
real-user safety, or production schema-v5 readiness. It does not add standalone
Context Recovery correction/deletion or suggested/skipped prompt deletion.

## Remaining Risks

Production cutover remains blocked by migrated legacy current-action parity,
runtime/migration activation decisions, real-user backup/restore and recovery
evidence, and separate Founder review. Any new legal artifact relationship must
receive artifact-specific lifecycle policy rather than inferred generic
cascade behavior.

## Git State

Branch `codex/phase-3c-remaining-lifecycle-parity-closure-audit` at
`1428f610c161eb89b57d4c9d44da6fd99be7682b`; authorized unstaged changes only;
no upstream; no stage, commit, push, merge, PR, deployment, or release.

## Engineer Completion Status

completed_with_follow_up
