# Engineering Plan

Status: approved

- Sprint ID: 2026-08-23-ordinary-v5-reflection-double-submit-correction-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest reviewed: 8d529230b27ef2447418f3c4fc06931d840a2511c4789b6c5153b1a8f9c85d94
- Created at: 2026-08-22T20:20:00.000Z
- Updated at: 2026-08-22T20:20:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions`. Correct only duplicate Reflection Save Answer activation in the unpromoted ordinary schema-v5 review candidate. Do not change Rust writer refusal, schema, migration, provider, consent, or any other product action.

## Existing Implementation Understanding

`App.tsx` has no synchronous per-prompt in-flight guard. Its button remains actionable until React receives durable state. A queued second invocation then builds an unchanged answered-to-answered correction from current durable state; the Rust writer correctly rejects that as `reflection_response_unchanged`. Existing draft helpers already separate session draft from durable response.

## Affected Modules

- `src/app/App.tsx`
- `src/app/reflectionDraft.ts`
- `src/app/reflectionDraft.test.ts`
- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `scripts/founder-dogfood-package.mjs` (validation-discovered exact allowlist synchronization)
- repository-required workflow archive artifacts

The script path was added only after canonical validation proved that its exact
successor allowlist had drifted behind the already archived disclosure cycle
and the two bounded Reflection helper paths. No generic path or new behavior was
authorized.

## Proposed Design

1. Add collision-safe per-entry/per-prompt begin/end in-flight helpers.
2. Add a pure answer mutation helper that returns an already-equal durable answered prompt unchanged and delegates genuine new or changed responses to the existing append-only response helper.
3. Add a synchronous ref guard plus rendered pending set in `App.tsx` so the exact Save Answer control disables immediately and releases in `finally`.
4. Preserve successful draft reconciliation and preserve the draft on actual failure.
5. Add focused duplicate, equality, neighboring-prompt, release, and genuine-correction tests.
6. Record the manual observation factually, verify, review, archive/reset, rebuild one ignored installer, and resume only Step 8D-2R.

## Alternatives Considered

- Time-based debounce was rejected because it is nondeterministic and can suppress a legitimate later edit.
- Weakening the Rust unchanged-response refusal was rejected because that writer invariant correctly prevents invented successor revisions.
- Treating all unchanged errors as success was rejected because it could hide unrelated stale or contradictory evidence.

## Data Lifecycle Impact

No durable shape change. One explicit save creates one response revision; duplicate activation creates none. Genuine later edits remain append-only corrections.

## SQLite Or Migration Impact

None. No Rust, SQL, DDL, schema version, migration, backup, or restore path changes.

## Provenance Impact

Positive preservation only: duplicate UI activation cannot manufacture a second user-response provenance event. Existing prompt and user authorship remain unchanged.

## Historical Context Impact

None. Historical retrieval, selection, packet assembly, and provenance are untouched.

## Consent Impact

None. Reflection response saving is unrelated to historical provider consent.

## Provider Transmission Impact

None. No provider or ContextPacket path changes.

## Import And Export Impact

None.

## Test Strategy

- Focused Vitest for `reflectionDraft.test.ts`.
- Prove a second begin for the same entry/prompt is refused synchronously.
- Prove another prompt remains independent and release permits a later operation.
- Prove an equal durable answered response returns the same object without new timestamps.
- Prove a changed answered response still produces a new user-authored successor representation.
- Run TypeScript typecheck and Clippy with warnings denied.

## Repository Verification Strategy

Run `git diff --check`, compare tracked/non-ignored untracked paths with the anticipated allowlist, confirm no Rust/schema/provider/consent diff from this correction, and run `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`.

## Manual UI Verification

Founder-owned. Install the newly hashed ignored review package in the same disposable account, reopen the existing v5 profile, change the already-saved response, click Save Answer twice rapidly, and verify one correction, immediate disabled state, no error, and clean restart reconstruction.

## Rollback Or Recovery Strategy

The correction is UI-only and reversible by removing its exact diff before promotion. Do not retry or mutate the disposable profile automatically. Stop if the same defect persists.

## Documentation Impact

Append one factual architecture/18 note describing the Step 8D-2 observation, bounded correction, and manual retest boundary.

## ADR Impact

No new ADR. ADR-0011 already requires append-only genuine corrections and supports refusing duplicate lifecycle facts.

## Risk Level

Medium: the code change is small, but incorrect concurrency handling could suppress a real user edit. Exact per-prompt scoping and `finally` release bound the risk.

## Escalation Decision

No escalation. The Founder confirmed duplicate activation and the active goal authorizes bounded evidence-based correction cycles. Any required Rust or policy change would stop for Founder review.
