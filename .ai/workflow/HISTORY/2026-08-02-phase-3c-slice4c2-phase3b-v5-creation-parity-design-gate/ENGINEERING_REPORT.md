# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-02-phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: `666518eb53ba1cebf64ce87e4add1d2467c8dcbb`
- Working-tree digest implemented: `4c46ac5b17b2315aac8c5059f605be765ddfcab85263f7fbbc5e1015357767de`
- Created at: 2026-08-01T21:50:00.000Z
- Updated at: 2026-08-02T00:15:00.000Z

## Implementation Summary

Implemented the Founder-authorized private, unregistered, disposable-only
Historical Question schema-v5 creation-parity writer. It preserves exact
ADR-0009 v4 authority while atomically adding one linked normalized v5
representation and exact-revision dependencies.

## Existing System Areas Inspected

`src/historicalContext/governedPacket.ts`; `src-tauri/src/sqlite.rs`;
`src-tauri/schema/schema_v5.sql`; migration/backfill helpers; promoted
Experience, Evidence, Reflection, Pattern, Context Recovery, and confirmed-
Evidence lifecycle modules; ADR-0009/ADR-0011; architecture/09/10/12/13.

## Files Added

- `src-tauri/src/schema_v5_historical_question_write.rs`

## Files Modified

- `src-tauri/src/schema_v5_migration.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- Exact sprint workflow artifacts under `.ai/workflow/`

## Files Deleted

None.

## Behavior Changed

No production or user-visible behavior changed. A private test-only Rust path
can now prove exact Phase 3B v4/v5 creation parity, exact duplicate idempotency,
conflicting duplicate refusal, no-question/failure non-creation, complete
rollback, and conservative ambiguous-COMMIT classification on disposable v5
fixtures.

## Data Model Impact

No schema change. Disposable transactions populate existing v4 Historical
Question tables and existing v5 artifact/revision/content/provenance/dependency/
lifecycle-link tables under the promoted compatibility guard.

## Migration Impact

None in production. Tests produce exact-v5 disposable fixtures with the
promoted migration core. Production schema/startup maximum and user databases
remain v4.

## Provenance Impact

Exact packet bytes/digest stay authoritative in ADR-0009 storage. Successful
creation adds one deterministic normalized AI generation provenance fingerprint
and role link containing exact provider/model/purpose, consent/transmission,
packet identity, contract versions, generated time, and source-artifact set.
No-question and failed transport add none.

## Historical Context Impact

No retrieval or eligibility change. The writer accepts only bounded packet
Experience, confirmed Evidence, and answered user-authored Reflection sources,
validates unique IDs before set equality, reuses the promoted verifier chain,
and separately proves selected Reflection-to-confirmed-Evidence dependency
closure.

## Consent Impact

None. Only existing consumed exact consent plus successful exact transmission
are accepted. Scope, destination, purpose, packet, and source mismatch fail
closed. No consent creation or reuse is introduced.

## Provider Transmission Impact

None. No provider or ContextPacket file changed and no call is possible from
the private module.

## Tests Added

Eight focused Rust tests cover successful parity and exact packet bytes;
idempotent/conflicting duplicates; no-question and unsuccessful audit behavior;
consent/source/Reflection dependency drift; malformed/duplicate/unsupported/
source-cap/citation/output-contract refusals; thirteen meaningful rollback boundaries;
ambiguous COMMIT exact pre/post/third-state classification; and the existing
guarded ADR-0009 cascade reaching the new representation.

## Tests Executed

- `cargo test ... schema_v5_migration::historical_question_write::tests -- --nocapture`: 8/8 passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- Canonical repository verification: passed with 17 workflow tests, 26 Vitest
  files / 204 tests, 145 Rust library tests, 12 backup/restore integration
  tests, and 8 schema-contract tests.

## Verification Results

Focused implementation checks, Clippy with warnings denied, TypeScript
typecheck, frontend build, Rust check, repository hygiene, and canonical
verification passed. Theory Alignment Review remains before archive.

## Manual Verification Required

No desktop runtime check applies because the module is private, unregistered,
disposable-only, and has no Tauri/UI/startup/app-data caller. Founder diff review
is required after workflow archive.

## Documentation Updates

Architecture/13 version 3.5 records Slice 4C-1 promotion facts and the bounded
Slice 4C-2 authority, mapping, transaction, evaluation, and production fences.

## ADR Impact

No ADR or status change. ADR-0009 and ADR-0011 meanings are preserved.

## Deviations From Plan

Theory Review Cycle 0 removed a parallel Rust prohibited-word taxonomy and
instead requires the exact existing `historical-question-output-v1` and
`phase-3b-safety-v1` packet contracts. The authoritative Product Harness remains
the only semantic Phase 4 evaluator. The implementation also found that the
global promoted Reflection verifier is
necessary but not sufficient for a packet-selected legacy Reflection after its
exact `uses_evidence` edge is removed. The bounded writer therefore reuses the
full verifier and adds a packet-specific exact dependency-set check; this is not
a new Product Harness policy.

## Known Limitations

Disposable evidence does not prove production restart recovery, real-user
safety, fresh-v5 initialization, migration cutover, runtime behavior, or
operating-system durability. No automatic recovery action exists.

## Remaining Risks

Production v5 still requires all remaining write parity, cutover/recovery gates,
real-user backup/migration evidence, explicit disclosure, manual review, and
separate Founder authorization. Generic unreferenced provenance-record cleanup
remains governed by the existing authorized lifecycle/orphan-cleanup policy;
this slice creates no new cleanup path.

## Git State

Branch `codex/phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate`, HEAD
`666518eb53ba1cebf64ce87e4add1d2467c8dcbb`, no upstream, unstaged authorized
working changes only, no staged files, commit, push, merge, PR, or deployment.

## Engineer Completion Status

`completed_with_follow_up`
