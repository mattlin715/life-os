# Engineering Report

Status: completed

- Sprint ID: 2026-08-02-phase-3c-slice4c1-confirmed-evidence-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: adb269dc68d6ec3917819e6ea8c18d84cf89e902
- Working-tree digest implemented: ba95d5702470c5179b364a878908fcefa0fbc9096f263cf4b4726fb8a7d628ba
- Created at: 2026-08-01T18:46:00.966Z
- Updated at: 2026-08-01T18:46:00.966Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the Founder-authorized Option B as one private, unregistered,
path/connection-injected disposable schema-v5 confirmed-Evidence lifecycle
boundary. Exact-current confirmed Evidence correction/deletion now coordinates
ordinary exact dependent invalidation, ADR-0009 migrated Historical Question
cascade deletion, guarded schema-v4 projection changes, failure injection,
exact reconciliation, and conservative COMMIT classification.

## Existing System Areas Inspected

- `src-tauri/src/schema_v5_migration.rs` and shared migration/manifest helpers.
- `src-tauri/src/schema_v5_evidence_write.rs`.
- `src-tauri/src/schema_v5_reflection_write.rs`.
- `src-tauri/src/schema_v5_pattern_write.rs`.
- `src-tauri/src/schema_v5_context_recovery_write.rs`.
- `src-tauri/schema/schema_v5.sql` lifecycle, dependency, tombstone, projection,
  and Historical Question bridge constraints.
- Schema-v4 Historical Question triggers and promoted migration fixtures/tests.
- ADR-0009, ADR-0011, architecture/12, and architecture/13.

## Files Added

- `src-tauri/src/schema_v5_evidence_lifecycle.rs`.

## Files Modified

- `src-tauri/src/schema_v5_migration.rs`: private child-module registration.
- `src-tauri/src/schema_v5_evidence_write.rs`: exact deleted-Evidence verifier.
- `src-tauri/src/schema_v5_reflection_write.rs`: retained invalidated Reflection
  verifier with exact obsolete-source dependency evidence.
- `src-tauri/src/schema_v5_pattern_write.rs`: retained invalidated Pattern
  verifier for direct/transitive exact obsolete-source dependencies.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`:
  Slice 4B-4 promotion correction and factual Slice 4C-1 working-tree evidence.
- Active `.ai/workflow/` mission, decision, review, plan, report, state, and
  event artifacts through the repository workflow.

## Files Deleted

none.

## Behavior Changed

- Private fixture-local correction appends a user revision, retains predecessor
  content/provenance, returns Evidence to pending/ineligible, and projects a
  candidate without confirmation carry-forward.
- Private fixture-local deletion purges all Evidence content, retains revision
  and provenance metadata plus one content-free artifact tombstone, and removes
  the projection.
- Current exact Reflection/Pattern dependents retain content but become
  invalidated/ineligible with one fact per exact invalidating edge; transitive
  Reflection-to-Pattern invalidation occurs in the same transaction.
- Affected migrated Historical Questions are deleted across exact schema-v4/v5
  representations through the existing ADR-0009 cascade.
- No runtime or user-visible production behavior changed.

## Data Model Impact

No DDL change. The implementation exercises already-approved schema-v5 heads,
revisions, content, provenance, dependencies, lifecycle events, tombstones,
links, and guarded schema-v4 projections only in disposable fixtures.

## Migration Impact

No migration or fresh-v5 initialization change. Exact-v5 fixtures are produced
through the promoted migration core. Production `SCHEMA_VERSION` and startup
maximum remain 4.

## Provenance Impact

Old AI/local-mock and new user provenance remain immutable and separate.
Invalidation events cite exact dependency IDs. Deletion tombstone contains no
content or content digest. Historical actual-use provenance is removed only by
the existing ADR-0009 trigger chain.

## Historical Context Impact

Only migrated Historical Questions with exact one-to-one v4/v5 representation
and an exact packet-item dependency on the old Evidence revision are deleted.
Missing, extra, or contradictory representation fails closed. No retrieval,
generation, transmission, or Phase 4 interpretation was added.

## Consent Impact

No new or reused consent. Successful artifact-linked consent/transmission rows
follow existing deletion triggers. Unrelated unsuccessful audit retention is
unchanged.

## Provider Transmission Impact

none. No provider code or caller was added.

## Tests Added

Seven focused disposable Rust tests cover:

1. no-dependent confirmed correction and pending reconfirmation;
2. mixed direct/transitive ordinary plus Historical Question consequences,
   including a question that selected the invalidated Reflection without
   directly selecting the Evidence;
3. explicit deletion content purge and content-free tombstone;
4. stale revision and incomplete one-to-one parity refusal;
5. every correction failure boundary rollback;
6. every deletion failure boundary rollback;
7. committed and unchanged outcome-unknown classification without retry.

Existing Pattern durable-ID and dependency-state regressions continue to cover
malformed/duplicate/cross-source/unsupported dependency inputs through the
reused complete verifier chain.

## Tests Executed

- `cargo test schema_v5_migration::evidence_lifecycle::tests -- --nocapture`:
  7/7 passed.
- `cargo test --all-targets --all-features`: 137 Rust library tests,
  12 backup/restore integration tests, and 8 schema-contract tests passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed after formatting.

## Verification Results

- Focused lifecycle tests: passed.
- Full Rust tests: passed.
- Clippy warnings denied: passed.
- Rust formatting: passed.
- Canonical repository verification: passed with 17 workflow tests, 26 Vitest
  files / 204 tests, 137 Rust library tests, 12 backup/restore tests, and 8
  schema-contract tests plus typecheck, build, Rust check, and hygiene checks.
- Manual desktop/UI verification: not applicable to an unregistered module.

## Manual Verification Required

No runtime manual verification applies. Founder diff review remains required
after canonical verification and Theory Alignment Review.

## Documentation Updates

Architecture/13 version 3.3 records Slice 4B-4 promotion truth and bounded
Slice 4C-1 implementation evidence while preserving every production and later
phase fence.

## ADR Impact

No ADR or status change. The transaction implements accepted ADR-0011 Decisions
6B/7B/9A/10B/11A/12A and preserves ADR-0009 deletion semantics.

## Deviations From Plan

Theory Review Cycle 0 found that the first Historical Question closure covered
only direct old-Evidence packet items. The bounded correction now evaluates
every newly invalidated ordinary revision as a possible Historical Question
packet source, compares schema-v4/v5 IDs per exact source before union, and
fails closed if that source is no longer exact-current at preflight. No generic
artifact framework was extracted.

## Known Limitations

- Private, unregistered, disposable-only; no real user or production proof.
- Accepts only exact migrated-v5 fixtures with one-to-one Historical Question
  parity.
- Phase 3B v5 runtime creation parity remains absent and blocks production v5.
- No UI, Tauri command, startup, automatic recovery, or confirmed Pattern
  lifecycle support.

## Remaining Risks

- Future production activation requires a separate Phase 3B v5 creation writer
  and cutover/restart evidence.
- Future v5 UI must expose retained invalidated ordinary artifacts; current v4
  compatibility projection intentionally omits them.
- Production safety and real-user migration remain entirely unproven by this
  fixture-local slice.

## Git State

- Branch: `codex/phase-3c-slice4c1-confirmed-evidence-lifecycle-design-gate`.
- HEAD: `adb269dc68d6ec3917819e6ea8c18d84cf89e902`.
- Working tree: unstaged authorized sprint changes, including one untracked new
  Rust module.
- Staged files: none.
- Commit/push/merge/PR/deployment: none.

## Engineer Completion Status

completed
