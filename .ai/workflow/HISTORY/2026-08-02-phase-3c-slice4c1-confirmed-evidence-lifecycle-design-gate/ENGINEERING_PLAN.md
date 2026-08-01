# Engineering Plan

Status: approved

- Sprint ID: 2026-08-02-phase-3c-slice4c1-confirmed-evidence-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: adb269dc68d6ec3917819e6ea8c18d84cf89e902
- Working-tree digest reviewed: 23a79e01afd55e2569eda5591a2d2c905ff20c9ca931986bf3fc9e9e54ac73c7
- Created at: 2026-08-01T18:16:24.868Z
- Updated at: 2026-08-01T18:16:24.868Z

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE4C1-001` as Option B. Implement only a private, unregistered,
path/connection-injected, disposable schema-v5 Evidence lifecycle boundary for
exact-current confirmed correction and explicit deletion. It must atomically
apply complete exact ordinary invalidation, ADR-0009 migrated Historical
Question cascade deletion, and guarded v4 projection changes. Production v5,
real data, runtime activation, Phase 3B v5 creation parity, providers, consent,
Phase 4, Harness expansion, Git promotion, deployment, and release remain
withheld.

## Existing Implementation Understanding

- `schema_v5_evidence_write.rs` supports candidate creation, pending correction,
  exact confirmation, and rejection; it currently refuses inbound dependents.
- Reflection and Pattern writers have complete active-state verifier chains but
  do not yet recognize the ADR-0011 `invalidated` lifecycle state.
- `schema_v5.sql` already defines invalidated/deleted heads, exact dependency
  events, content-free tombstones, guarded v4 projections, and a guarded bridge
  that deletes normalized Historical Question heads when the schema-v4
  Historical Question row is deleted.
- The promoted migration core creates one-to-one
  `historical_question_lifecycle_links` and exact normalized packet-item edges
  for existing schema-v4 Historical Questions.
- `operation_manifest`, integrity checks, guarded writes, and conservative
  COMMIT outcome classification are already proven patterns.

## Affected Modules

- Add `src-tauri/src/schema_v5_evidence_lifecycle.rs`.
- Register it privately from `src-tauri/src/schema_v5_migration.rs`; do not
  expose it from Tauri or `lib.rs` commands.
- Extend lifecycle-aware verification in
  `src-tauri/src/schema_v5_evidence_write.rs`,
  `src-tauri/src/schema_v5_reflection_write.rs`, and
  `src-tauri/src/schema_v5_pattern_write.rs` only as required to verify
  deleted Evidence and invalidated ordinary dependents.
- Synchronize factual evidence in architecture/13 after verification.
- Update active `.ai/workflow/` role artifacts through the workflow CLI.

## Proposed Design

1. Define exact correction/delete commands, deterministic context, failure
   points, committed outcome, and private execution seam.
2. Preflight the exact current confirmed Evidence and active source Experience.
3. Enumerate and validate the complete inbound closure:
   - direct Reflection/Pattern exact edges from the Evidence revision;
   - transitive Pattern edges from each affected Reflection revision;
   - schema-v4 Historical Question rows plus normalized v5 links/packet edges.
4. Validate durable IDs and uniqueness before set equality; reject unsupported,
   cross-source, cyclic, missing, extra, or contradictory state.
5. Within one `BEGIN IMMEDIATE` transaction and one compatibility guard:
   - correction appends a user revision, predecessor and source dependency,
     corrected/superseded facts, pending/ineligible head, candidate projection;
   - deletion appends deleted fact, purges all Evidence content, clears current
     head, leaves confirmed/deleted/ineligible minimal head plus content-free
     artifact tombstone, and removes projection;
   - current ordinary dependents are retained but invalidated/ineligible, gain
     dependency-specific invalidation facts, and lose v4 projection;
   - affected Historical Question schema-v4 rows are deleted only after exact
     v4/v5 parity validation, relying on existing guarded bridge and ADR-0009
     provenance trigger for the complete cascade.
6. Reuse the full Evidence -> Reflection -> Pattern verifier chain after adding
   exact invalidated/deleted-state verification; do not duplicate partial active
   status checks.
7. Reconcile exact post-manifest, content/tombstone/event/dependency/projection
   facts, foreign keys, and integrity before commit and again read-only.
8. Treat generic COMMIT errors as outcome-unknown; classify exact post/pre state
   without retry or autonomous action.

## Alternatives Considered

- Correction-only refusal of dependents was rejected because it does not prove
  Decisions 10B/11A or deletion.
- Phase 3B v5 parity first was deferred: not required for exact migrated
  disposable fixtures, but remains mandatory before production activation.
- Contract-only work was rejected after explicit Option B implementation
  authority.
- Generic artifact framework extraction was rejected as unnecessary Harness
  expansion. Only bounded verifier-state reuse is planned.

## Data Lifecycle Impact

Synthetic/disposable data only. Correction preserves old Evidence content and
provenance but makes it superseded/context-ineligible. Deletion purges all
Evidence content and retains only content-free minimal metadata/tombstone.
Ordinary dependents retain content as invalidated history; Historical Question
actual-use content follows ADR-0009 cascade deletion.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, startup maximum, or production database
change. Tests create exact-v5 disposable fixtures through the promoted
migration core. All mutation occurs under the existing compatibility guard.

## Provenance Impact

Correction adds exact user provenance without rewriting old AI/local-mock
provenance. Deletion retains no content in tombstone metadata. Invalidation
events cite exact dependency IDs. Historical actual-use provenance is deleted
only through the existing ADR-0009 cascade.

## Historical Context Impact

Only existing migrated Historical Questions that exactly used the old Evidence
revision are affected. Their schema-v4 and normalized-v5 representations must
be one-to-one and are deleted together. No retrieval, generation, provider
transmission, whole-history loading, or Phase 4 interpretation occurs.

## Consent Impact

No new consent or reuse. Successful consumed consent/transmission records tied
to a deleted generated artifact follow existing ADR-0009 triggers. The 30-day
unreferenced unsuccessful-attempt rule is unchanged.

## Provider Transmission Impact

none. The module is private and unregistered and performs no provider call.

## Import And Export Impact

none. Export v2 and import behavior are unchanged.

## Test Strategy

- Build exact-v5 fixtures through the promoted migration core, enriching only
  synthetic v4 input with complete Evidence/Reflection/Pattern records.
- Cover correction/deletion with no dependents, direct Reflection, direct
  Pattern, transitive Reflection -> Pattern, successful Historical Question,
  and mixed dependents.
- Assert pending reconfirmation, retained predecessor/provenance, no rebinding,
  invalidated retained content, projection removal, complete Historical
  Question/packet/provenance cascade, and deletion tombstone/content purge.
- Cover stale source/Evidence, malformed/duplicate/cross-source/unsupported
  dependencies, already-invalidated state, parity contradictions, and missing
  links.
- Inject failure after guard, correction/deletion, invalidation, projection,
  historical cascade, reconciliation, and guard removal; assert exact logical
  pre-state.
- Inject committed, definite non-commit, and outcome-unknown results; verify
  read-only exact manifests with no retry.
- Preserve all existing migration, writer, schema-contract, R1, provenance,
  provider, and Phase 4 regression tests.

## Repository Verification Strategy

Run focused Rust tests first, then `cargo clippy --all-targets --all-features --
-D warnings`, then canonical
`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`.
Confirm no Constitution, production `sqlite.rs` schema constant, provider,
ContextPacket, UI, Tauri command, or migration DDL diff.

## Manual UI Verification

Not applicable. The authorized module remains private, unregistered,
disposable-only and has no UI/runtime route. Founder review is source diff and
automated evidence only.

## Rollback Or Recovery Strategy

Definite pre-commit failures roll back and are read-only reconciled to the exact
pre-manifest. Generic COMMIT error is outcome-unknown unless exact post or pre
state is proved. No retry, replay, repair, rebind, cleanup, or candidate
selection occurs. Disposable fixture directories are test-owned.

## Documentation Impact

Update architecture/13 only with factual implemented/verified working-tree
evidence and retained authorization fences. No Book Zero rewrite or new design
document.

## ADR Impact

No new ADR and no status change. Implementation directly realizes accepted
ADR-0011 Decisions 6B/7B/9A/10B/11A/12A and preserves ADR-0009.

## Risk Level

High within the disposable test boundary because the transaction combines
destructive purge, transitive invalidation, and two persistence
representations. Production risk remains zero because the module is private,
unregistered, fixture-only, and production schema remains v4.

## Escalation Decision

Proceed within Founder-authorized Option B. Stop at `human_decision_required`
if repository evidence shows that exact one-to-one Historical Question parity
or complete ordinary dependency closure cannot be verified without Phase 3B v5
creation parity or a new policy decision.
