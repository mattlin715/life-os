# Engineering Plan

Status: approved

- Sprint ID: 2026-08-09-phase-3c-slice4c6b-legacy-successor-parity
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `6f2c64c13a6085e73cc4f4c5dccf6d51a73e543d`
- Working-tree digest reviewed: `6918f007203286c6d37e4848622eb1430df6fd23`
- Created at: 2026-08-09T19:28:00+09:00
- Updated at: 2026-08-09T19:28:00+09:00

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement only Founder-authorized Slice 4C-6B: migrated pending Evidence correction; migrated suggested Reflection answer/skip; migrated answered Reflection response correction; migrated suggested Context Recovery answer/skip. All paths remain private, unregistered, and disposable-only.

## Existing Implementation Understanding

- The migration core produces exact `legacy-v4-raw` revision 1 baselines, exact source/artifact dependency edges, normalized known-or-`legacy_unknown` provenance, and byte-equal schema-v4 projections.
- Evidence already has strict artifact-specific legacy candidate/provenance validation used by Slice 4C-6A; `correct_pending` currently rejects non-canonical serialization.
- Reflection and Context Recovery canonical writers already implement successor, skip, guard, projection, rollback, read-only reconciliation, and ambiguous-COMMIT contracts; they lack strict action-time legacy validators and legacy branches.
- Reflection answered correction is currently reachable because `App.tsx` leaves the answered response textarea enabled and the save button active for a dirty non-skipped response.
- Context Recovery only permits answer/skip while `suggested`; answered/skipped turns expose no correction/deletion action.

## Affected Modules

- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_reflection_write.rs`
- `src-tauri/src/schema_v5_context_recovery_write.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- repository-required `.ai/workflow` artifacts/archive only

## Proposed Design

1. Correct architecture/13 Slice 4C-6A promotion facts first, using `git diff --shortstat a9bbf7cc... 5c8c8755...` (`14 files changed, 3188 insertions(+), 45 deletions(-)`).
2. Evidence: reuse `validate_legacy_review_candidate` before legacy correction; parse exact allowed fields; append the existing canonical user successor with predecessor revision 1, exact current source edge, retained original text/kind/editability/created time, pending/ineligible head, and candidate v4 projection. Preserve the raw predecessor and normalized content provenance.
3. Reflection: add an artifact-specific legacy validator proving revision-1 baseline metadata, exact raw/projection bytes and digest, allowed v4 shape/status, exact unique Evidence IDs, normalized prompt/response provenance representation, authorship, source, timestamps, and dependency equality.
4. For legacy suggested answer, append a canonical mixed successor with immutable question/prompt provenance, exact Experience/Evidence edges, `answers_prompt` to the raw baseline, and new user response provenance.
5. For legacy suggested skip, reuse the baseline revision, append only the skip review/head fact, and update the v4 projection status/time without rewriting retained content.
6. For legacy answered correction, append a canonical mixed successor referencing the raw baseline through `answers_prompt`, preserving prompt bytes/provenance and exact Evidence/Experience edges, with new user response provenance and existing dependent closure behavior.
7. Context Recovery: add the equivalent strict legacy validator for the supported v4 turn shape, prompt/response provenance, exact source edge, raw/projection equality, and historical exclusion.
8. For legacy suggested answer, append a canonical mixed successor with exact prompt lineage and new response provenance; for skip, retain the raw baseline and update review/head/projection only.
9. Do not create generic legacy mutation infrastructure; keep validators and branches in their existing artifact writers.

## Alternatives Considered

- Recanonicalize legacy payloads before action: rejected; destroys historical byte identity.
- Relax serialization checks without strict validation: rejected; admits malformed or contradictory history.
- Add generic cross-artifact legacy parser: rejected; obscures artifact-specific provenance and eligibility rules.
- Add missing runtime/UI actions: prohibited and unnecessary for current-action parity.

## Data Lifecycle Impact

Only explicit user actions append successors or review facts. Skip creates no response content. Old revision content remains exact. No real database is accessed.

## SQLite Or Migration Impact

No DDL or migration-core change. Tests inject exact supported schema-v4 artifact rows into disposable fixtures, then invoke the promoted migration path. Production `SCHEMA_VERSION` remains 4.

## Provenance Impact

Legacy prompt/content provenance remains unchanged, including `legacy_unknown`. New user-authored content receives deterministic exact user provenance at the injected action time. Prompt and response provenance remain separate.

## Historical Context Impact

Context Recovery remains categorically excluded. Reflection successors require exact current confirmed Evidence. Existing dependent Pattern invalidation and ADR-0009 Historical Question cascade behavior are reused only where correction requires them.

## Consent Impact

None. No consent state or policy changes.

## Provider Transmission Impact

None. No provider call or registered command.

## Import And Export Impact

None. Fixture insertion is test setup only. Export v2 remains unauthorized.

## Test Strategy

- Add exact-v4-to-v5 fixture builders for legacy pending Evidence, suggested/answered Reflection, and suggested Context Recovery.
- Prove each six-action happy path, exact predecessor bytes/digest/imported facts, deterministic canonical successor, authorship/provenance split, exact dependencies, v4 projection parity, reconfirmation requirement, and historical exclusion.
- Refuse stale source/artifact, stale/rejected Evidence, deleted/orphaned/cross-source/malformed/unknown-field/duplicate/conflicting state.
- Inject every meaningful new write boundary and compare exact logical pre-state.
- Repeat ambiguous-COMMIT exact pre/post/third-state classification with no retry/repair.
- Re-run canonical existing writers and Slice 4C-6A regressions.

## Repository Verification Strategy

Run focused Rust tests for affected modules, `cargo clippy --all-targets -- -D warnings`, then `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Record verification through the repository workflow.

## Manual UI Verification

Not applicable. This slice has no registered Tauri command, renderer/UI/startup path, app-data path, or real database. Founder diff review is required.

## Rollback Or Recovery Strategy

One `BEGIN IMMEDIATE` transaction and the existing guard wrap each action. Every pre-commit failure must reconcile to the exact pre-manifest. Ambiguous COMMIT is classified from read-only exact pre/post evidence; third state returns `recovery_required`. No automatic retry, replay, repair, restore, cleanup, or rebinding.

## Documentation Impact

Minimal factual architecture/13 update for 4C-6A promotion and final Slice 4C-6B disposable evidence. No Book Zero or ADR changes.

## ADR Impact

No new ADR or status change. This is a bounded implementation of accepted ADR-0007, ADR-0009, and ADR-0011.

## Risk Level

Medium. The slice spans three artifact writers and legacy parsing, but remains isolated to synthetic disposable fixtures with exact fail-closed validation and no runtime surface.

## Exact Anticipated Final Allowlist

Four product/document paths plus the 11 final workflow archive artifacts listed by Product Review. No migration, DDL, production SQLite, Tauri, TypeScript, provider, ContextPacket, Book Zero, or ADR path is allowed.

## Escalation Decision

No unresolved consequential decision. Stop at `human_decision_required` if any action requires a fifth product/document path, inferred provenance/dependency, new user action, relaxed historical exclusion, DDL, migration-core, runtime, or provider/consent change.
