# Engineering Plan

Status: approved

- Sprint ID: 2026-08-09-phase-3c-remaining-lifecycle-parity-closure-audit
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `1428f610c161eb89b57d4c9d44da6fd99be7682b`
- Working-tree digest reviewed: repository workflow digest recorded at Product Review sequence 9
- Created at: 2026-08-08T16:36:53.589Z
- Updated at: 2026-08-08T16:36:53.589Z

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3C-SLICE4C5-001` as Option A. Implement only private, unregistered,
disposable exact-v5 Experience correction consequences and the exact verifier
support they require. Preserve all exclusions in the Founder response. Any
need for runtime integration, legacy-v4 baseline actions, generic lifecycle
infrastructure, or new policy returns to `human_decision_required`.

## Existing Implementation Understanding

- `schema_v5_experience_write.rs` already appends immutable Experience
  corrections, advances the source head, synchronizes the guarded v4
  Experience projection, cascades ADR-0009 Historical Questions, and provides
  deterministic rollback and conservative COMMIT classification.
- Its update path currently refuses every ordinary artifact through
  `ordinary_artifact_lifecycle_requires_later_slice`; parent deletion already
  performs the full source-scoped purge.
- Every ordinary current revision retains an exact
  `derived_from_experience` edge. Promoted Evidence, Reflection, Pattern, and
  Context Recovery verifiers already validate their canonical active and
  terminal states, but source-caused invalidation support is incomplete.
- Existing artifact-specific invalidation proves stale Evidence/Reflection
  dependencies. This slice must add the distinct stale source-revision proof,
  not weaken those verifiers into accepting arbitrary invalidated state.
- Historical Question deletion is already destructive by ADR-0009, but the
  Experience update path must prove exact schema-v4/normalized-v5 parity before
  invoking the cascade.

## Affected Modules

- `src-tauri/src/schema_v5_experience_write.rs`
- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_reflection_write.rs`
- `src-tauri/src/schema_v5_pattern_write.rs`
- `src-tauri/src/schema_v5_context_recovery_write.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- repository workflow artifacts and eventual sprint archive only

No Tauri registration, renderer, UI, startup, production schema constant,
migration DDL, provider, ContextPacket, consent, retention, backup/restore, or
Phase 4 file is in scope.

## Proposed Design

1. Replace the blanket ordinary-artifact refusal on exact-current Experience
   correction with an exact consequence planner executed inside the existing
   `BEGIN IMMEDIATE` transaction.
2. Before mutation, validate the source and all same-source ordinary heads with
   their promoted exact verifiers; reject cross-source inbound ordinary edges,
   malformed identities, duplicate dependency facts, cycles, contradictory
   projections, unsupported lifecycle states, and unexpected inbound state.
3. For each active Evidence, Reflection, Pattern, or Context Recovery head whose
   current revision has exactly one stale-target `derived_from_experience`
   dependency to the old current Experience revision:
   - retain revision content, provenance, review state, and dependency bytes;
   - set lifecycle to `invalidated`, eligibility to `ineligible`, and the exact
     source-supersession reason;
   - append one deterministic system lifecycle event linked to that exact
     dependency;
   - remove only its guarded schema-v4 projection.
4. Leave already invalidated, rejected/content-purged, and deleted artifacts
   byte-for-byte unchanged and require their existing exact verifier contract;
   never duplicate lifecycle events or resurrect projections.
5. Verify exact Historical Question schema-v4/normalized-v5 source coverage,
   then use the promoted ADR-0009 cascade. Context Recovery remains categorically
   excluded from historical dependencies.
6. Append and advance the Experience correction without rebinding any ordinary
   dependency, synchronize its v4 projection, run all exact verifiers and
   database checks, then remove the compatibility guard.
7. Extend only the four artifact verifiers so `invalidated` is accepted when a
   unique lifecycle event proves the current revision's exact stale
   `derived_from_experience` dependency. Preserve their existing artifact-
   caused invalidation paths and all other fail-closed rules.
8. Add deterministic failure points around consequence planning/application,
   Historical Question parity/cascade, projection synchronization, and
   reconciliation. Reuse the existing exact pre-state/post-state/third-state
   COMMIT outcome model.

## Alternatives Considered

- Delete ordinary artifacts on source edit: rejected because correction is not
  rejection/deletion and would erase user-owned or reviewable context.
- Rebind dependencies to the new source revision: rejected because it invents
  provenance and silently treats old output as based on new content.
- Add a generic dependent engine: rejected as speculative infrastructure and
  outside the artifact-specific accepted architecture.
- Combine legacy-v4 baseline current-action parity: explicitly deferred by the
  Founder; it remains a separate cutover blocker.

## Data Lifecycle Impact

Disposable exact-v5 fixtures only. Source correction becomes non-destructive
for ordinary artifact content while making it unavailable for current use.
Historical Questions retain the approved destructive stale-source cascade.
Parent deletion remains the existing complete destructive source-scoped purge.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, production `user_version`, or startup
change. All writes occur in one existing disposable `BEGIN IMMEDIATE`
transaction against synthetic exact-v5 databases.

## Provenance Impact

Ordinary artifact content/provenance and exact old source edges remain
unchanged. One deterministic lifecycle fact records source-caused invalidation
without recasting authorship or user review. No dependency rebinding occurs.

## Historical Context Impact

Affected Historical Questions must prove exact schema-v4/normalized-v5 parity
and are then removed through ADR-0009 cascade. No packet is rehydrated,
transmitted, or reused. Context Recovery remains ineligible for historical use.

## Consent Impact

No new consent or policy change. Existing successful-use consent/transmission
facts are deleted only with the already-authorized Historical Question cascade;
unrelated unsuccessful audit metadata remains unchanged.

## Provider Transmission Impact

None. No provider or ContextPacket code is modified and no call is made.

## Import And Export Impact

No import contract change and no export-v2 work.

## Test Strategy

- Build exact-v5 disposable fixtures through the promoted migration core.
- Cover every active state combination: pending/confirmed Evidence;
  suggested/answered/skipped Reflection; candidate/confirmed Pattern; and
  suggested/answered/skipped Context Recovery.
- Assert byte-exact retained content/provenance/review/dependency facts,
  invalidated/ineligible heads, exact single invalidation event, absent v4
  projections, and no rebinding.
- Cover mixtures plus already invalidated, rejected/content-purged, and deleted
  artifacts with no duplicate event or resurrection.
- Cover exact Historical Question parity/cascade, unrelated failed-audit
  preservation, and Context Recovery historical exclusion.
- Cover cross-source, malformed, stale, missing, duplicate, cyclic,
  contradictory, unsupported, and unexpected inbound refusal with exact
  unchanged manifests.
- Exhaustively regress parent deletion for all ordinary kinds and retained
  revision histories.
- Inject failures at every new meaningful boundary and assert logical rollback.
- Exercise committed, definite-noncommit, ambiguous exact pre-state, ambiguous
  exact post-state, and third-state `recovery_required` classification.
- Run focused Rust tests and Clippy before canonical verification.

## Repository Verification Strategy

Run focused Rust tests for the five affected modules, then
`cargo clippy --all-targets -- -D warnings`, then the canonical repository
command:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Confirm the Constitution is unchanged, production schema/startup maximum stay
4, no production/runtime registration exists, and only the allowlisted source,
architecture, and workflow files changed.

## Manual UI Verification

Not applicable. The module remains private, unregistered, and reachable only
from synthetic/disposable Rust tests. Founder review is diff plus automated
evidence; no desktop runtime path is authorized.

## Rollback Or Recovery Strategy

Every pre-commit error rolls back and is verified read-only against the exact
pre-manifest. Ambiguous COMMIT accepts only the exact post-manifest as success
or exact pre-manifest as definite non-commit; any third state returns
`recovery_required` without retry, replay, repair, cleanup, or candidate
selection. This is disposable evidence, not production restart proof.

## Documentation Impact

Synchronize only factual Slice 4C-5 evidence and remaining cutover fences in
architecture/13. Workflow artifacts truthfully record planning,
implementation, verification, Theory review, and terminal status. No new ADR or
Book Zero document.

## ADR Impact

No new ADR and no ADR status change. The implementation is bounded evidence for
accepted ADR-0009 and ADR-0011 decisions.

## Risk Level

High within the disposable contract because one source correction touches four
artifact kinds and Historical Question cascades atomically. Exposure remains
bounded because the module is private, unregistered, schema-neutral in
production, and exercised only on synthetic fixtures.

## Escalation Decision

Proceed with the exact Founder-authorized plan. Stop at
`human_decision_required` if implementation reveals a need to weaken verifier
contracts, add a new lifecycle policy, change schema/DDL, touch runtime paths,
or include legacy-v4 baseline action parity.
