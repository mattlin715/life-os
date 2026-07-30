# Engineering Plan

Status: approved

- Sprint ID: 2026-07-31-phase-3c-slice4b2-reflection-write-parity-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: bb7ef2a6b37f8b4c7fdeab6ee0063d469dd8f011
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-07-31T01:00:00+09:00
- Updated at: 2026-07-31T01:00:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Founder decision
`PHASE3C-SLICE4B2-001` Option A authorizes only a private, unregistered,
path/connection-injected Reflection write boundary on disposable exact-v5
fixtures for suggested-prompt creation, first saved response, append-only
response correction and explicit skip. Exact dependencies, split prompt/user
provenance, mixed combined authorship, guarded v4 projection, deterministic
failure evidence and read-only reconciliation are mandatory. All production,
runtime, schema activation, Historical Question, later-slice, Phase 4, Harness,
Git promotion, deployment and release exclusions remain binding.

## Existing Implementation Understanding

- `schema_v5_migration.rs` owns the fixed DDL, exact-v4-to-v5 disposable
  migration fixture path, canonical JSON/digests, deterministic identifiers and
  conservative commit outcome adapter.
- Nested `schema_v5_experience_write.rs` exposes only internal exact-v5 contract
  and operation-manifest verification primitives.
- Nested `schema_v5_evidence_write.rs` proves candidate, correction, confirm and
  rejection semantics against the same exact-v5 disposable contract.
- Migrated exact-v5 fixtures already contain exact current confirmed Evidence,
  answered/skipped Reflection baselines and guarded v4 projections. Tests may
  use those facts but the new boundary must create its own operation artifacts.
- Production `sqlite.rs` remains schema v4 and must not import or call this
  module.

## Affected Modules

- New: `src-tauri/src/schema_v5_reflection_write.rs`.
- Minimal private nesting declaration:
  `src-tauri/src/schema_v5_migration.rs`.
- Factual synchronization:
  `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Repository workflow artifacts under `.ai/workflow/`.

No Tauri command, renderer, UI, provider, ContextPacket, production schema,
startup, app-data, backup/restore or Historical Question module is affected.

## Proposed Design

1. Define private typed commands for create suggested prompt, save first user
   response, correct current response and explicitly skip a suggested prompt.
2. Reuse the promoted migration core and exact-v5 verification/manifest
   primitives; do not create DDL or a parallel fixture/mutation framework.
3. Validate exact current Experience revision plus a non-empty unique set of
   exact current confirmed, active, eligible, same-source Evidence revisions.
4. Persist the immutable initial prompt as AI or local-mock authorship with
   exact prompt-role provenance and exact Experience/Evidence dependencies.
5. Persist answered/corrected combined revisions as `mixed`, preserving the
   prompt bytes and original prompt provenance, adding exact user response
   provenance and an `answers_prompt` edge to the immutable initial revision.
6. Represent skip as an exact-revision review event and head/projection state;
   create neither a response revision nor response provenance.
7. Use one `BEGIN IMMEDIATE` transaction and one exact guard token; write v5
   authority/events/dependencies first, guarded v4 projection last, reconcile,
   remove the guard and commit through the injected outcome adapter.
8. Reopen read-only and classify only exact pre-state, exact post-state, or
   `recovery_required`; never retry, repair, cascade or rebind.

## Alternatives Considered

- Prompt/answer only: rejected by Founder Option A because it leaves explicit
  skip and correction parity immediately incomplete.
- Extend Evidence or Experience into a generic artifact writer: rejected
  because Reflection has distinct mixed-authorship and answers-prompt rules.
- Use production v4 whole-bundle persistence: rejected because it cannot prove
  immutable revision/provenance/dependency semantics and exceeds authority.

## Data Lifecycle Impact

Disposable data only. Prompt, answered and corrected content revisions are
append-only. Correction supersedes but does not erase prior response content.
Skip retains prompt content and adds one exact review fact. No background
invalidation, deletion cascade, retention or real-user lifecycle is added.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, startup maximum, or production
`user_version` change. Tests create exact-v5 files only through the promoted
private disposable migration core. All v5 and v4 projection mutations occur in
one test-local transaction.

## Provenance Impact

Prompt provenance remains immutable AI or local-mock. Saved/corrected response
provenance is exact user provenance. Combined content revisions are `mixed` but
never relabel the prompt as user-authored. `answers_prompt` targets the exact
initial prompt revision; provenance fingerprints deduplicate only identical
canonical facts.

## Historical Context Impact

No Phase 3B packet, selection, consent, transmission, Historical Question or
v5 cascade write occurs. Only a current answered revision with a non-empty user
response and current exact dependencies is marked locally eligible. Any
correction with an inbound ordinary or ADR-0009 dependent fails closed without
cascade or rebinding.

## Consent Impact

None. Local eligibility is not consent and no consent event is created,
consumed or reused.

## Provider Transmission Impact

None. Synthetic prompt provenance is input data; the module makes no network or
provider call and does not change provider/ContextPacket behavior.

## Import And Export Impact

None. No import, export v2, file output, retention or backup/restore behavior is
changed.

## Test Strategy

- AI and local-mock suggested prompt creation with immutable prompt provenance.
- Exact source and one-or-more confirmed same-source Evidence dependencies.
- First answer creates a mixed revision, user response provenance and exact
  `answers_prompt`; projection becomes answered and locally eligible.
- Correction appends a mixed revision and corrected/superseded lifecycle facts
  while retaining initial prompt linkage and prior content/provenance.
- Explicit skip records one review fact, no response revision/provenance, and a
  skipped/ineligible projection.
- Refusal of blank/malformed input, duplicate IDs/dependencies, stale/deleted/
  rejected/cross-source/ineligible dependency, stale prompt/head, conflicting
  state, answer/correction after skip, skip after answer and correction with
  any inbound dependency.
- Failure injection after each meaningful write/reconciliation boundary,
  logical rollback, empty guard, deterministic IDs/manifests, ambiguous commit
  pre/post classification, immutable receipt, foreign keys and integrity.
- Existing Slice 4A/4B-1, Phase 3B and full repository regressions.

## Repository Verification Strategy

Run focused Reflection Rust tests and:

`cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`

Then run canonical verification:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record exact results in the repository workflow during validation.

## Manual UI Verification

Not applicable. The module is private, unregistered, disposable-only and has no
Tauri, desktop, renderer, UI, startup, app-data or real-user surface.

## Rollback Or Recovery Strategy

Definite pre-commit failure rolls back the single transaction and must reopen as
its exact logical pre-manifest. Generic commit error is outcome-unknown until a
read-only reopen proves exact pre/post state; mixed evidence is
`recovery_required`. No automatic retry, replay, rollback, repair, cleanup,
restore, cascade or candidate selection is implemented. Removing the private
module changes no user database.

## Documentation Impact

Update architecture/13 only with factual Slice 4B-1 promotion closeout and
Slice 4B-2 disposable implementation/verification evidence plus preserved
production and later-slice fences. Do not modify Book Zero, the Constitution,
ADR status or archived workflow history.

## ADR Impact

No new ADR and no ADR status change. The implementation is bounded evidence
under accepted ADR-0007, ADR-0009, ADR-0011 and Founder-approved
architecture/13.

## Risk Level

High for lifecycle/provenance correctness, bounded to synthetic disposable
data. Mixed authorship, exact prompt linkage, eligibility and correction
inbound-dependency refusal can conflict if ordering is wrong; deterministic
rollback and read-only reconciliation tests are required.

## Escalation Decision

No additional decision is required. Implement only the exact Founder-approved
Option A contract. Stop at `human_decision_required` if correct implementation
requires DDL changes, production/runtime activation, dependent cascades,
confirmed-Evidence mutation, Historical Question writes or another excluded
scope.
