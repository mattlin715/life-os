# Engineering Plan

Status: approved

- Sprint ID: 2026-08-03-phase-3c-slice4c3-confirmed-pattern-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `e385ed7f465376dae734afacc08f49ac5532b980`
- Working-tree digest reviewed: `3ce814928011130f730ea9c18dc1ef7581c28f3292d399df455520172ad202b0`
- Created at: 2026-08-02T16:32:00.000Z
- Updated at: 2026-08-02T16:32:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE4C3-001` as Option A. Extend only the existing private,
unregistered, path/connection-injected disposable Pattern writer with
exact-current confirmed correction and explicit deletion. Preserve an explicitly
supplied exact source set, require zero inbound edges, synchronize guarded v4/v5
state, prove rollback and conservative COMMIT classification, and stop at
Founder diff review. All production, runtime, real-data, Phase 4, Harness, Git
promotion, deployment, and release authority remains withheld.

## Existing Implementation Understanding

- `schema_v5_pattern_write.rs` already owns candidate creation, exact pending
  confirmation/rejection, deterministic context/failure points, guarded v4
  projection, complete Experience/Evidence/Reflection verification, exact
  dependency reconciliation, read-only manifests, and ambiguous-COMMIT
  classification.
- `PatternWriteCommand` has no correction/deletion variants. Current review
  commands refuse inbound edges and accept only pending active heads.
- `artifact_revisions` supports predecessor links; lifecycle events support
  corrected, superseded, deleted, and content_purged facts; heads support
  pending/confirmed and active/deleted states; tombstones are guarded.
- DDL has no `uses_pattern` relationship. Context Recovery and ADR-0009 cannot
  legally depend on Pattern.
- Evidence lifecycle is a comparison model for purge and lifecycle facts, but
  its dependent invalidation/cascade must not be copied because Pattern has no
  legal inbound consumer.

## Affected Modules

- Extend `src-tauri/src/schema_v5_pattern_write.rs` only for product code and
  co-located tests.
- Update `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
  only with factual implementation/verification evidence.
- Update standard `.ai/workflow/` artifacts through the repository workflow.

No DDL, Cargo, migration registration, Tauri, TypeScript, renderer, UI,
provider, ContextPacket, ADR, Book Zero, or Harness-contract path may change.

## Proposed Design

1. Add private `CorrectConfirmed` and `DeleteExact` command variants with exact
   current source/Pattern revisions and caller-supplied Evidence/Reflection
   revision arrays; validate identifier uniqueness before set equality.
2. Reuse the existing exact current Pattern and complete dependency/verifier
   functions. Require current confirmed/active/eligible state for correction;
   deletion accepts an exact active current Pattern state allowed by the
   authorized deletion contract and refuses deleted/invalidated/rejected state.
3. Verify caller source sets exactly equal the predecessor dependencies and
   immutable provenance source IDs; refuse any change or rebinding.
4. Require both normalized and historical inbound counts to be zero before any
   lifecycle mutation. Any edge is malformed/future and aborts unchanged.
5. Correction in one existing `BEGIN IMMEDIATE` boundary:
   - insert/reuse exact user provenance;
   - append canonical user-authored revision with predecessor;
   - copy the exact validated Experience/Evidence/Reflection revision edges;
   - append corrected and superseded lifecycle facts;
   - update the head to pending/active/ineligible without a review event;
   - write the guarded v4 candidate projection while retaining predecessor
     content/provenance.
6. Deletion in one transaction:
   - append explicit user deletion fact;
   - append purge facts and delete content rows for every retained Pattern
     revision, including superseded revisions;
   - preserve only authorized immutable metadata/provenance/dependencies;
   - clear current revision, set deleted/ineligible, delete v4 projection, and
     create one minimal content-free artifact tombstone with no content digest.
7. Extend exact Pattern verification for corrected/superseded and deleted
   states without weakening create/confirm/reject invariants.
8. Extend operation manifests/failure points so exact pre/post state covers all
   revision, provenance, content, dependency, lifecycle, head, projection,
   tombstone, guard, FK, and integrity facts.
9. Keep the existing single-attempt COMMIT adapter. Close writable access and
   classify exact pre-state, exact post-state, or third-state
   `recovery_required`; never retry or repair.

## Alternatives Considered

- Generic dependent invalidation was rejected because no legal Pattern inbound
  relationship exists and it would create speculative Phase 4 ontology.
- Correction-only was rejected because it leaves the accepted deletion/privacy
  gap open.
- A separate lifecycle module was rejected: extending the promoted Pattern
  writer reuses its exact validators and stays within the authorized two-file
  product allowlist.
- Source reselection was rejected because it changes the hypothesis basis and
  needs separate product authority.

## Data Lifecycle Impact

Synthetic/disposable data only. Correction retains prior content/provenance as
superseded and context-ineligible. Deletion purges all retained Pattern content
and leaves only policy-authorized metadata plus a content-free tombstone.

## SQLite Or Migration Impact

No schema, DDL, migration, `SCHEMA_VERSION`, startup maximum, or production
database change. Exact-v5 disposable fixtures come from the promoted migration
core and mutations use existing guards.

## Provenance Impact

The corrected successor has user content provenance; every predecessor keeps
its original AI/local-mock provenance. Exact source dependencies are copied only
after explicit caller equality. No provenance or dependency is rebound.

## Historical Context Impact

Pattern remains ineligible for ADR-0009. Any historical dependency targeting a
Pattern aborts as malformed. No Historical Question is created, retained,
cascaded, or interpreted by this slice.

## Consent Impact

None. No selection, preflight, consent, transmission, or actual-use provenance
behavior changes.

## Provider Transmission Impact

None. No provider call or ContextPacket change; the writer remains private and
unregistered.

## Import And Export Impact

None. Export v2 and import behavior remain unchanged.

## Test Strategy

- Successful correction for both AI and local-mock predecessors; exact user
  successor, unchanged predecessor provenance/content, pending/ineligible head,
  preserved source edges, exact v4 candidate projection, and later exact
  reconfirmation.
- Refuse stale/duplicate correction; pending/rejected/deleted/invalidated head;
  source Experience drift; Evidence/Reflection drift; malformed, duplicate,
  cross-source, missing, extra, or reordered-equivalent dependency arrays.
- Refuse every normalized or historical inbound edge without mutation.
- Successful deletion from confirmed and permitted active exact state; all
  retained current/superseded content removed; explicit deletion/purge facts;
  cleared head, removed v4 projection, content-free tombstone; unrelated
  artifacts unchanged.
- Refuse stale/duplicate deletion.
- Inject failure after each new provenance, revision, content, dependency,
  lifecycle, head, projection, purge, tombstone, reconciliation, and guard
  boundary; assert exact logical pre-state.
- Cover exact ambiguous-COMMIT pre/post/third-state outcomes with no retry.
- Verify v4/v5 parity, unique arrays before equality, guard emptiness,
  `foreign_key_check`, `integrity_check`, production schema v4, and no Phase 4.

## Repository Verification Strategy

Run focused Pattern tests, then
`cargo clippy --all-targets --all-features -- -D warnings`, then canonical
`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`.
Confirm no Constitution, DDL, production SQLite constant, Tauri, provider,
ContextPacket, UI, renderer, or Harness diff.

## Manual UI Verification

Not applicable. The authorized module remains private, unregistered,
disposable-only and has no UI/runtime route. Founder review is source diff and
automated evidence only.

## Rollback Or Recovery Strategy

Every pre-commit failure rolls back and read-only reconciliation must equal the
exact pre-manifest. A generic COMMIT error is outcome-unknown unless exact post
or pre state is proven. A third state is `recovery_required`; no retry, replay,
rollback, repair, cleanup, source selection, or rebinding occurs.

## Documentation Impact

Update architecture/13 version/factual Slice 4C-3 evidence only after focused
and canonical verification. No new design document or Book Zero rewrite.

## ADR Impact

No new ADR or status change. The implementation realizes bounded ADR-0011
correction/deletion policy while preserving ADR-0009 Pattern exclusion.

## Risk Level

Medium-high inside disposable fixtures because deletion is destructive and
correction must preserve exact lineage across v4/v5. Production risk is zero
because no runtime caller or production schema change is authorized.

## Escalation Decision

Proceed within Option A and the exact two-product-file allowlist. Stop at
`human_decision_required` if any legal inbound Pattern consumer, required DDL
change, source-reselection need, or additional product file is discovered.
