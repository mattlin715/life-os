# Engineering Plan

Status: approved

- Sprint ID: 2026-08-01-phase-3c-slice4b3-pattern-write-parity-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: c2f518a409c4308682302f505da275b621f16708
- Working-tree digest reviewed: 154c7df11570746d907af04a0ad44656be201c0ba8528ebc0add5c84a190b70c
- Created at: 2026-08-01T02:20:00+09:00
- Updated at: 2026-08-01T02:20:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Founder decision
`PHASE3C-SLICE4B3-001` Option A authorizes only a private, unregistered,
path/connection-injected single-Experience Pattern write boundary on disposable
exact-v5 fixtures for AI/local-mock candidate creation, exact confirmation and
exact rejection. Exact current Experience, confirmed Evidence and actually used
answered Reflection dependencies, immutable generation provenance, complete
Reflection verifier reuse, rejected-content purge, guarded v4 projection,
deterministic failure evidence and read-only reconciliation are mandatory. All
production, runtime, schema activation, later-slice, Phase 4, Harness, Git
promotion, deployment and release exclusions remain binding.

## Existing Implementation Understanding

- `schema_v5_migration.rs` owns the fixed DDL, disposable exact-v4-to-v5
  fixture path, canonical JSON/digests, deterministic identifiers and
  conservative commit-outcome adapter.
- `schema_v5_experience_write.rs` exposes the private complete Experience
  contract verifier; `schema_v5_evidence_write.rs` reuses it for Evidence.
- `schema_v5_reflection_write.rs` reuses the complete Evidence verifier and
  adds prompt/response provenance, exact dependency and projection checks.
- The current schema-v4 Pattern projection stores source Evidence and optional
  Reflection IDs with immutable generation provenance and candidate/confirmed
  status. Rejection removes that projection.
- Production `sqlite.rs` remains schema v4 and must not import or call this
  module.

## Affected Modules

- New: `src-tauri/src/schema_v5_pattern_write.rs`.
- Minimal private nesting declaration:
  `src-tauri/src/schema_v5_migration.rs`.
- Minimal private verifier visibility change:
  `src-tauri/src/schema_v5_reflection_write.rs`.
- Factual synchronization:
  `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Repository workflow artifacts under `.ai/workflow/`.

No Tauri command, renderer, UI, provider, ContextPacket, production schema,
startup, app-data, backup/restore, Historical Question or Context Recovery
module is affected.

## Proposed Design

1. Define private typed create, confirm and reject commands with deterministic
   clocks, identifiers, guard tokens, failure points and commit outcomes.
2. Reuse the promoted migration fixture and verification primitives; do not add
   DDL, a generic writer, or parallel fixture machinery.
3. Validate one exact current Experience revision, at least one exact current
   confirmed eligible same-source Evidence revision, and zero or more exact
   current answered eligible same-source Reflection revisions actually used.
4. Call the complete Reflection verifier first, thereby reusing its Evidence
   and Experience verifier chain, then validate the Pattern-selected exact sets.
5. Require immutable AI/local-mock provenance whose exact source-artifact IDs
   equal the declared Evidence plus Reflection IDs. Reject unsupported or
   Context Recovery IDs rather than dropping or translating them.
6. Create one canonical Pattern revision with `derived_from_experience`,
   `uses_evidence` and optional `uses_reflection_response` edges; write pending
   review/lifecycle state and guarded v4 candidate projection atomically.
7. Confirm only one exact current pending revision. Append explicit user review
   evidence, mark it useful for reflection, and update only review/eligibility
   and projection status; content and generation provenance remain byte-stable.
8. Reject only one exact current pending revision. Append explicit review and
   purge lifecycle facts, remove projection/content, clear the head and retain
   a content-free tombstone containing the prior digest in one transaction.
9. Refuse inbound dependents and every stale, deleted, rejected, cross-source,
   orphaned, malformed, duplicate, conflicting or unsupported state without
   rebinding, cascade, retry or repair.
10. Reopen read-only and classify only exact pre-state, exact post-state, or
    `recovery_required` after ambiguous commit evidence.

## Alternatives Considered

- Contracts/tests without the private writer: rejected because Founder chose
  Option A and the promoted sequence now has sufficient dependency verifiers.
- Treat Pattern as Evidence or reuse the Evidence writer: rejected because a
  Pattern remains a tentative hypothesis with different confirmation meaning.
- Add Context Recovery edges or silently omit recovery provenance: rejected
  because fixed DDL and Founder authority do not authorize that dependency.
- Create a generic artifact writer: rejected as unnecessary abstraction and a
  larger audit surface.

## Data Lifecycle Impact

Disposable data only. Candidate content is append-only while pending or
confirmed. Confirmation preserves content/provenance. Rejection synchronously
purges the content and v4 projection while retaining only content-free review,
lifecycle, revision and tombstone facts. No real-user retention, background
invalidation, confirmed correction/deletion or cascade is added.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, startup maximum, fresh-v5 path, or
production `user_version` change. Tests create exact-v5 files only through the
promoted disposable migration core. All v5 authority and guarded v4 projection
changes occur in one test-local `BEGIN IMMEDIATE` transaction.

## Provenance Impact

Candidate authorship remains exactly AI or local mock with provider, model,
Harness, prompt, generated-time, source Experience and exact source-artifact
facts. Confirmation does not relabel or rewrite provenance. The declared
Evidence/Reflection set must exactly equal provenance source IDs; duplicates,
omissions, extras and unsupported Context Recovery fail closed.

## Historical Context Impact

None. No retrieval, packet assembly, Historical Question, consent,
transmission, Phase 3B v5 write or whole-history behavior is introduced.
Patterns remain excluded from the governed historical packet.

## Consent Impact

None. Local Pattern review is not provider-use consent. No consent event is
created, consumed, persisted or reused.

## Provider Transmission Impact

None. Synthetic provenance is fixture input only. The module makes no provider
call and does not change provider, model, prompt, safety or ContextPacket code.

## Import And Export Impact

None. No import, export v2, file output, retention or production backup/restore
behavior changes.

## Test Strategy

- AI and local-mock candidate creation with immutable exact provenance.
- One/multiple Evidence and zero/one/multiple answered Reflection dependencies.
- Exact provenance-source equality; duplicate/omitted/extra/Context Recovery
  source IDs fail closed.
- Complete Reflection -> Evidence -> Experience verifier-chain reuse.
- Confirmation changes only review, eligibility and v4 status while preserving
  content and provenance bytes/digests.
- Rejection purges content/projection and retains content-free review,
  lifecycle, revision and tombstone evidence.
- Refusal of blank/malformed inputs, stale/deleted/rejected/cross-source/
  orphaned/ineligible dependencies, stale head, conflicting state and inbound
  dependents.
- Failure injection after each meaningful write/reconciliation boundary;
  logical rollback, guard emptiness, deterministic IDs/manifests and ambiguous
  commit exact-pre/exact-post classification.
- Schema-v4 production boundary and existing Experience/Evidence/Reflection,
  Phase 3B and repository regressions.

## Repository Verification Strategy

Run focused Pattern Rust tests and:

`cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`

Then run canonical verification:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record exact results in the repository workflow during validation.

## Manual UI Verification

Not applicable. The module is private, unregistered, disposable-only and has no
Tauri, desktop, renderer, UI, startup, app-data or real-user surface.

## Rollback Or Recovery Strategy

Definite pre-commit failure rolls back the single transaction and must reopen as
the exact logical pre-manifest. Generic commit error is outcome-unknown until a
read-only reopen proves exact pre/post state; mixed evidence is
`recovery_required`. No automatic retry, replay, rollback, repair, cleanup,
cascade, rebinding or candidate selection is implemented. Removing the private
module changes no user database.

## Documentation Impact

Update architecture/13 only with factual Slice 4B-2 promotion closeout and
Slice 4B-3 disposable implementation/verification evidence while preserving
production and later-slice fences. Do not modify Book Zero, the Constitution,
ADR status or archived workflow history.

## ADR Impact

No new ADR and no ADR status change. The implementation is bounded evidence
under accepted ADR-0007, ADR-0009, ADR-0011 and Founder-approved
architecture/13.

## Risk Level

High for lifecycle and provenance correctness, bounded to synthetic disposable
data. Exact dependency equality, verifier-chain reuse, confirmation immutability
and synchronous rejection purge can conflict if transaction ordering is wrong;
deterministic rollback and read-only reconciliation are required.

## Escalation Decision

No additional decision is required. Implement only the exact Founder-approved
Option A contract. Stop at `human_decision_required` if correct implementation
requires DDL changes, production/runtime activation, Context Recovery edges,
dependent cascades, confirmed correction/deletion, Historical Question writes
or another excluded scope.
