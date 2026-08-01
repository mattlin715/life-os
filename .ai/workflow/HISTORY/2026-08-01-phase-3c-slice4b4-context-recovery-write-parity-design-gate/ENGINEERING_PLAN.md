# Engineering Plan

Status: approved

- Sprint ID: 2026-08-01-phase-3c-slice4b4-context-recovery-write-parity-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: b8205b12a4ac33ef23d20c84d54a2115fdecb830
- Working-tree digest reviewed: cb3e9eff12468fea8f182dcd8e95cecbb3fa020cfaaa8894b9b92738bdbe2805
- Created at: 2026-08-02T00:20:00+09:00
- Updated at: 2026-08-02T00:20:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Founder decision
`PHASE3C-SLICE4B4-001` Option A authorizes only a private, unregistered,
path/connection-injected Context Recovery write boundary on disposable exact-v5
fixtures for exact AI/local-mock suggestion creation, one first explicit
non-empty user answer, and explicit skip of an unanswered suggestion. Complete
Experience verifier reuse, immutable prompt provenance, separate user response
provenance, exact dependencies, one-open-turn enforcement, current-task-only
answered eligibility, categorical historical exclusion, guarded v4 projection,
deterministic failure evidence and read-only reconciliation are mandatory. All
production, runtime, correction/deletion, later-slice, Phase 3B/4, Harness, Git
promotion, deployment and release exclusions remain binding.

## Existing Implementation Understanding

- `schema_v5_migration.rs` owns the fixed DDL, disposable exact-v4-to-v5
  fixture path, canonical JSON/digests, deterministic identifiers and
  conservative commit-outcome adapter.
- `schema_v5_experience_write.rs` exposes the complete private Experience
  contract verifier and operation-manifest helper reused by later private
  writers.
- Evidence, Reflection and Pattern writers demonstrate one-transaction v5
  authority plus guarded-v4 projection, deterministic failure injection and
  read-only pre/post outcome classification without production registration.
- Production schema-v4 Context Recovery persists source-scoped turns with
  distinct prompt and response provenance. Only answered non-empty turns are
  used for the current Experience task; they are not historical packet inputs.
- Production currently creates only deterministic local-mock prompts. The
  domain can represent AI provenance, but this slice does not invoke a model.

## Affected Modules

- New: `src-tauri/src/schema_v5_context_recovery_write.rs`.
- Minimal private nesting declaration: `src-tauri/src/schema_v5_migration.rs`.
- Factual synchronization:
  `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Repository workflow artifacts under `.ai/workflow/`.

No Tauri command, renderer, UI, provider, ContextPacket, production schema,
startup, app-data, backup/restore or Phase 3B persistence module is affected.

## Proposed Design

1. Define private typed create-suggestion, save-first-response and skip commands
   with deterministic clocks, identifiers, guard tokens, failure points and
   commit outcomes.
2. Reuse exact-v5 fixtures and the complete Experience verifier from the
   promoted migration/write foundation; add no DDL or parallel fixture system.
3. Require one exact current active Experience revision and validate all
   durable identifiers as well-formed and unique before comparing sets.
4. Create one immutable AI/local-mock prompt revision with exact prompt
   provenance, `derived_from_experience`, pending review and ineligible state.
5. Enforce at most one open suggested turn per source while allowing a later
   explicit suggestion after an answered or skipped terminal turn.
6. Save only the first explicit non-empty user response by appending an
   answered mixed-authorship revision. Preserve the original prompt and prompt
   provenance, add separate user response provenance and an exact
   `answers_prompt` dependency to the immutable initial revision.
7. Mark answered content `not_applicable` for artifact review and eligible only
   for the current-Experience task; never mark it historically eligible.
8. Skip only an unanswered exact current suggested turn by recording explicit
   user review state without creating response provenance or a replacement
   content revision.
9. Keep v5 authority and the exact guarded-v4 `recovery_turn` projection in one
   transaction; refuse stale, deleted, invalidated, malformed, blank,
   duplicate, conflicting, cross-source, unsupported and unexpected inbound
   dependency states without rebinding or cascade.
10. Reopen read-only and classify only exact pre-state, exact post-state, or
    `recovery_required` after ambiguous commit evidence.

## Alternatives Considered

- Contracts/tests without the private writer: rejected because Founder chose
  Option A and the promoted Experience verifier supports this bounded parity.
- Reuse Reflection as Context Recovery: rejected because recovery is a
  current-task clarification turn, not durable longitudinal reflection.
- Add a generic artifact writer: rejected because it expands the audit surface
  and could blur eligibility and lifecycle semantics.
- Permit response correction/deletion or background invalidation: rejected as
  explicitly outside this slice.

## Data Lifecycle Impact

Disposable data only. Suggested prompt content/provenance are immutable. A first
answer appends exact user response provenance and becomes current-task-only.
Explicit skip records a terminal user choice while retaining the unanswered
prompt and no response provenance. No real-user retention, correction,
standalone deletion, dependent invalidation, cascade or durable longitudinal
memory behavior is added.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, startup maximum, fresh-v5 path or
production `user_version` change. Tests create exact-v5 files only through the
promoted disposable migration core. Each authorized mutation uses one
test-local `BEGIN IMMEDIATE` transaction for v5 authority and guarded-v4
projection.

## Provenance Impact

Prompt authorship remains exactly AI or local mock with exact provider, model,
Harness, prompt, generated-time and source Experience facts. The answer has
separate exact user provenance and does not relabel or confirm the prompt.
Malformed/duplicate durable provenance IDs and unsupported source artifacts fail
closed before equality checks.

## Historical Context Impact

No historical eligibility. Context Recovery remains categorically excluded
from Phase 3B candidate eligibility, consent, packet assembly, transmission and
durable longitudinal memory. Tests must prove no historical dependency or
packet/provenance row is created.

## Consent Impact

None. Answering or skipping a local recovery prompt is not provider-use consent.
No consent event is created, consumed, persisted or reused.

## Provider Transmission Impact

None. AI/local-mock provenance is injected fixture evidence only. The private
module makes no provider call and does not change provider, model, prompt,
safety or ContextPacket behavior.

## Import And Export Impact

None. No import, export v2, file output, retention or production backup/restore
behavior changes.

## Test Strategy

- AI and local-mock suggestion creation with immutable exact provenance.
- At-most-one open suggestion per source and later opportunity after answered
  or skipped terminal turns.
- First non-empty response, immutable prompt lineage, separate user provenance,
  exact `answers_prompt`, mixed authorship and current-task-only eligibility.
- Explicit skip without a replacement revision or response provenance; silence
  remains no state transition.
- Complete Experience verifier reuse and exact current-active source refusal.
- Categorical historical/consent/transmission exclusion.
- Refusal of blank/malformed inputs, stale/deleted/invalidated/cross-source
  source or head, duplicate/conflicting state, unsupported provenance/locale,
  malformed or duplicate durable ID arrays and unexpected inbound dependents.
- Failure injection after meaningful write/reconciliation boundaries; logical
  rollback, guard emptiness, deterministic identifiers/manifests and ambiguous
  commit exact-pre/exact-post classification.
- Exact guarded-v4 projection and repository regressions, including existing
  Experience/Evidence/Reflection/Pattern and Phase 3B behavior.

## Repository Verification Strategy

Run focused Context Recovery Rust tests and:

`cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`

Then run canonical verification:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record exact results in the repository workflow during validation.

## Manual UI Verification

Not applicable. The module is private, unregistered, disposable-only and has no
Tauri, desktop, renderer, UI, startup, app-data or real-user surface.

## Rollback Or Recovery Strategy

Definite pre-commit failure rolls back the transaction and must reopen as the
exact logical pre-manifest. Generic commit error is outcome-unknown until a
read-only reopen proves exact pre/post state; mixed evidence is
`recovery_required`. No automatic retry, replay, rollback, repair, cleanup,
rebinding, cascade or candidate selection is implemented. Removing the private
module changes no user database.

## Documentation Impact

Update architecture/13 only with factual Slice 4B-3 promotion closeout and
Slice 4B-4 disposable implementation/verification evidence while preserving
production and later-slice fences. Do not modify Book Zero, the Constitution,
ADR status or archived workflow history.

## ADR Impact

No new ADR and no ADR status change. The implementation is bounded evidence
under accepted ADR-0007, ADR-0009, ADR-0011 and Founder-approved
architecture/13.

## Risk Level

High for lifecycle, provenance and historical-exclusion correctness, bounded to
synthetic disposable data. Prompt immutability, one-open-turn enforcement,
answers-prompt lineage and projection synchronization can conflict if
transaction ordering is wrong; deterministic rollback and read-only
reconciliation are required.

## Escalation Decision

No additional decision is required. Implement only the exact Founder-approved
Option A contract. Stop at `human_decision_required` if correct implementation
requires DDL changes, production/runtime activation, response correction or
deletion, dependent cascades, Phase 3B writes or another excluded scope.
