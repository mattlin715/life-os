# Engineering Plan

Status: approved

- Sprint ID: 2026-08-04-phase-3c-slice4c4-answered-reflection-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `7c09dd7d008773e157e7da38de2263661d91307a`
- Working-tree digest reviewed: `47528284534e0229716dd493462583c91f7460ffc873611c6742db72bd00c019`
- Created at: 2026-08-04T12:02:00.000Z
- Updated at: 2026-08-04T12:02:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE4C4-001` as Option A. Implement only private, unregistered,
path/connection-injected answered Reflection correction/deletion against
synthetic/disposable exact-v5 fixtures, with complete exact Pattern
invalidation and ADR-0009 Historical Question cascade. Only
`src-tauri/src/schema_v5_reflection_write.rs`, architecture/13, and standard
workflow artifacts may change. Every production/runtime/real-data/later-slice
boundary remains withheld.

## Existing Implementation Understanding

- `ReflectionWriteCommand::CorrectResponse` already appends a mixed successor,
  preserves immutable prompt lineage and exact Experience/Evidence edges, but
  refuses all inbound dependents.
- No answered Reflection deletion command exists and the Reflection verifier
  accepts only active or invalidated heads.
- Pattern uses `uses_reflection_response`; its promoted verifier retains
  invalidated content/provenance and requires absent v4 projection plus an
  exact invalidation event.
- The Evidence lifecycle demonstrates exact ordinary closure and the guarded
  ADR-0009 bridge: matched normalized/v4 dependencies, deletion of the v4
  Historical Question authority, and cascade reconciliation of linked v5
  records, consent, transmission, packet, and dependencies.
- Context Recovery has only Experience and self-prompt edges. Pattern has no
  legal inbound consumer.

## Affected Modules

1. `src-tauri/src/schema_v5_reflection_write.rs`
2. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
3. Standard `.ai/workflow/` current artifacts and terminal archive only

No other product file may change.

## Proposed Design

1. Add `DeleteAnswered` and bounded lifecycle failure points/outcome evidence.
2. Before mutation, validate the exact current answered Reflection, complete
   Experience/Evidence/prompt/provenance/projection contract, and scan inbound
   edges across all retained Reflection revisions.
3. Accept only self `answers_prompt`, same-source Pattern
   `uses_reflection_response`, and matched Historical Question
   `historical_packet_item` relations. Require active Patterns to target the
   exact current Reflection revision; accept older retained edges only when the
   Pattern is already invalidated/ineligible with its exact invalidation fact
   and no v4 projection. Refuse every other relation or incomplete state.
4. Require zero normalized/historical inbound consumers for every affected
   Pattern; do not create a transitive or generic dependent engine.
5. Reconcile each historical source across the normalized exact revision and
   schema-v4 `source_revision`, lifecycle link, generated artifact, consent,
   and transmission identity before mutation.
6. Correction reuses the existing append-only successor path, then invalidates
   exact active Pattern dependents, cascades exact Historical Questions, and
   writes the corrected Reflection projection. It never rebinds a dependent or
   adds confirmation.
7. Deletion applies the same closure, records explicit deletion/content-purge
   facts, clears the Reflection head, purges all retained revision content,
   removes the v4 projection, and creates one digest-free artifact tombstone.
8. Extend Reflection reconciliation for the deleted state and use the promoted
   complete Pattern verifier after lifecycle writes; verify historical bridge
   counts, current-content invariants, foreign keys, integrity, and guard
   emptiness.
9. Preserve existing `BEGIN IMMEDIATE`, deterministic guard/ID/clock/failure
   injection, operation-manifest, rollback, and ambiguous-COMMIT read-only
   classification machinery.

## Alternatives Considered

- Correction only: rejected because deletion needs the same closure and would
  create a second later implementation.
- Deletion only: rejected because dependent-aware correction remains blocked.
- Generic dependency engine: rejected because it invents future relationship
  authority and exceeds the allowlist.
- Modify DDL or production mutation paths: rejected; no schema/runtime authority.

## Data Lifecycle Impact

Synthetic fixtures only. Correction retains superseded Reflection content and
provenance under Decision 9A. Deletion purges every Reflection content row while
retaining authorized immutable content-free metadata and a digest-free
tombstone. Pattern content is retained but invalidated. Exact dependent
Historical Questions follow ADR-0009 cascade deletion. Unrelated unsuccessful
audit metadata remains.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, startup maximum, or production
`user_version` change. Disposable exact-v5 databases only. All changes occur in
one existing guarded `BEGIN IMMEDIATE` transaction.

## Provenance Impact

Prompt provenance remains exact and immutable. Correction adds only new user
response provenance. Pattern provenance is retained on invalidation.
Historical successful actual-use provenance is deleted only with its governed
Historical Question. No provenance is rewritten or rebound.

## Historical Context Impact

Old Reflection revisions become ineligible for new use. Existing exact
Historical Questions selecting the changed/deleted revision are deleted under
ADR-0009. A corrected response requires a later new selection, packet,
preflight, and consent before future historical use.

## Consent Impact

No consent creation, reuse, or policy change. Cascade removes only consent tied
to the deleted successful generated artifact according to the existing schema;
unrelated unsuccessful audit records remain.

## Provider Transmission Impact

None. No provider code or call is reachable from this private module.

## Import And Export Impact

None. No import, export-v2, or retention behavior changes.

## Test Strategy

Add focused disposable tests for: correction without dependents; one/multiple
Pattern dependents; Historical Question; combined closure; exact successor and
superseded state; retained invalidated Pattern and absent projection; no
rebinding; complete ADR-0009 cascade and unrelated audit retention; answered
deletion and all-content purge; digest-free tombstone; stale/duplicate/state
refusals; source/Evidence drift; malformed/duplicate/cross-source/cyclic and
unsupported inbound edges; incomplete Pattern/Historical parity; every new
failure boundary; ambiguous COMMIT pre/post/third state; deterministic IDs and
manifests; production schema boundary.

Focused commands:

- `cargo test schema_v5_migration::reflection_write::tests -- --nocapture`
- `cargo clippy --all-targets -- -D warnings`

## Repository Verification Strategy

Run `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`
after focused tests. Independently inspect the exact diff, allowlist, production
schema constant, Tauri registrations, workflow validity, Constitution diff,
staging area, and untracked files.

## Manual UI Verification

Not applicable. The module remains private, unregistered, disposable-only, and
has no desktop/UI/runtime path.

## Rollback Or Recovery Strategy

Inject failure at every meaningful lifecycle/cascade/purge boundary and prove
the exact logical pre-manifest after definite rollback. A generic COMMIT error
is outcome-unknown; close writable access and classify only exact read-only
pre/post manifests. Third state is `recovery_required`. No retry, replay,
rollback, repair, cleanup, source choice, or rebinding occurs autonomously.

## Documentation Impact

Extend architecture/13 with factual Slice 4C-4 implemented evidence only after
tests establish it. Preserve all production/runtime/later-slice fences. No Book
Zero, Constitution, ADR status, Harness, or new design document change.

## ADR Impact

No new ADR and no status change. This implements only bounded disposable
evidence under already accepted ADR-0009 and ADR-0011 consequences.

## Risk Level

Medium. The code remains production-inaccessible, but lifecycle correctness
spans exact revision graphs, dual v4/v5 representations, privacy deletion, and
ambiguous commit classification. Fail-closed closure validation and exhaustive
disposable tests contain the risk.

## Escalation Decision

Proceed to implementation. Stop immediately if any additional product file,
DDL change, new relationship, policy decision, runtime surface, or production
data access is required.
