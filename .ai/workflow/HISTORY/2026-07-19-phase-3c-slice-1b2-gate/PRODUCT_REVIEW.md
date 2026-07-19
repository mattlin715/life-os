# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-19-phase-3c-slice-1b2-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 1ef3aa0acd57756f7593e3fa792c321f0e164dcc
- Working-tree digest reviewed: cd038cd913ae41d697be7a9fd3cfee2556fd678ff437edbbb1cf5ac6e450270e
- Created at: 2026-07-18T20:32:56.4978112Z
- Updated at: 2026-07-18T20:32:56.4978112Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Close factual Slice 1B-1 promotion drift, inspect every remaining
renderer-authored SQLite mutation, and stop for an exact Founder decision before
any Slice 1B-2 production implementation.

## Problem Statement

Slice 1B-1 removed renderer-supplied SQL from Experience create, update, delete,
and import. Six other `LocalEvidenceStore` mutation methods still construct SQL
or execute mutation SQL in TypeScript. This leaves the schema-v4 artifact and
ADR-0009 historical persistence boundary split between renderer-owned SQL and
Rust-owned revalidation. Architecture/13 explicitly withholds Slice 1B-2, so a
passing baseline and prior architecture approval are not implementation
authority.

## User Value

A complete typed mutation boundary makes local data rules auditable in one
trusted Rust layer without changing what the user sees or migrating their
database. It reduces the chance that a renderer defect bypasses stale-work,
deletion, consent, transmission, or provenance invariants while preserving the
user's local-first control.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI, Evidence before Conclusion,
  Privacy before Profit, and documentation truth.
- `docs/06_Memory.md`: memory remains user-controlled, revisable, deletable,
  and provenance-preserving.
- `docs/Reflection.md`: meaning remains user-owned; persistence does not confer
  AI authority.
- `docs/09_AI.md`: AI is a mirror and context steward, not an oracle.
- `docs/10_Privacy.md`: local control, transparency, and explicit consent remain
  safety requirements.

## Relevant ADRs

- `ADR-0007`: reviewed AI artifacts require durable provenance and user control.
- `ADR-0008`: the repository Engineering Harness remains tool-independent; no
  Harness expansion is proposed.
- `ADR-0009`: selection, consent, transmission, successful persistence, actual
  use, stale refusal, and deletion remain distinct and fail closed.
- `ADR-0011`: append-only lifecycle direction is Accepted, but production
  schema-v5 migration remains separately gated.

## Current Implementation Context

Implemented and promoted:

- Slice 0 fixed contracts and test-only schema-v5 fixtures: merge `7921aff`.
- Slice 1A startup compatibility refusal: merge `6e9dd66`.
- Slice 1B-1 typed Experience mutations: feature `cc82658`, non-fast-forward
  merge `1ef3aa0`.
- Application version is `0.2.0`; production `SCHEMA_VERSION` and SQLite
  `user_version` remain 4.
- Canonical baseline passes 17 workflow tests, 158 Vitest tests, 21 Rust unit
  tests, 8 schema-v5 test-only contract tests, typecheck, build, Rust check, and
  repository hygiene checks.
- The Founder resolved `PHASE3C-SLICE1B2-001` as Option A and authorized the
  full bounded Slice 1B-2 scope recorded in `DECISION_REQUIRED.md`. This
  authorization does not extend to any excluded schema, migration, product,
  Harness, Git-promotion, or deployment scope.

Observed remaining renderer mutation paths in
`src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`:

1. `saveArtifacts` builds source-scoped DELETE and INSERT statements and sends
   them to generic `execute_sqlite_transaction`.
2. `saveHistoricalConsent` directly executes an INSERT/conditional conflict
   update through the SQL plugin.
3. `saveHistoricalTransmission` builds transmission INSERT/update plus consent
   consumption statements and sends them to the generic command.
4. `saveHistoricalQuestionArtifact` builds artifact and dependency INSERTs and
   sends them to `execute_sqlite_historical_transaction`; Rust revalidates the
   packet sources and actual-use provenance, but the renderer still chooses the
   SQL statements.
5. `deleteHistoricalQuestionArtifact` directly executes DELETE through the SQL
   plugin.
6. `purgeExpiredHistoricalAuditRecords` builds expiry DELETE statements and
   sends them to the generic command.

Read-only renderer queries are outside this mutation-parity slice. Schema-v4
triggers and foreign keys currently provide dependency cleanup; the typed Rust
commands must preserve rather than duplicate or weaken those rules.

## In Scope

- Factual architecture/13 synchronization for the promoted Slice 1B-1 state.
- One Founder decision covering the six remaining mutation methods.
- If Option A is authorized: typed Rust commands and typed TypeScript adapters
  for all six methods, synthetic/disposable tests, factual documentation,
  canonical verification, Theory Alignment Review, archive/reset, and Founder
  diff review.
- Capacity-resilient implementation micro-blocks inside one authority: artifact
  bundle; consent/transmission; Historical Question create/delete; audit cleanup
  and generic-command removal; final cross-path verification.

## Out Of Scope

- Any implementation before the exact Founder response.
- Renderer read-query conversion or a new storage SDK.
- Schema-v5 DDL, `user_version = 5`, migration, real user-database testing,
  backup, restore, or a retention-policy change.
- Lifecycle UI, export/import v2, Slices 2-6, Phase 4, provider or ContextPacket
  behavior, consent-policy changes, Harness expansion, Stage 2/3, Git promotion,
  PR, or deployment.

## Product Constraints

- Production schema and `user_version` stay at 4.
- Public `LocalEvidenceStore` behavior and in-memory parity remain unchanged.
- Renderer adapters send typed records and identifiers, never SQL statements,
  for every authorized mutation path.
- Rust owns SQL, transaction boundaries, invariant revalidation, and error
  mapping; no silent fallback to generic renderer SQL is allowed.
- Existing successful, stale, deletion-cascade, duplicate/idempotent, and audit
  expiry semantics remain observable and regression-tested.

## Evidence And Provenance Constraints

Artifact replacement must keep validation, source scoping, expected Experience
revision, and dependent Historical Question invalidation atomic. Historical
Question persistence must continue to bind exact source/artifact revisions,
consumed consent, successful transmission, packet digest, provider, and model.
No candidate is promoted to confirmed evidence by this work.

## Historical Context Constraints

No retrieval, packet assembly, preflight, provider call, output evaluation, or
Phase 4 conclusion changes. Historical persistence remains the already accepted
Phase 3B question-only capability. Source edit/delete and artifact rejection
must continue to invalidate dependents fail closed.

## Consent Constraints

Selection is not consent. Consent remains per generation and per purpose,
bound to the exact packet. Typed persistence may enforce the existing contract
but may not broaden eligibility, reuse consent, add blanket authorization, or
change provider disclosure.

## AI-Role Constraints

No AI inference, diagnosis, identity finalization, recurrence conclusion, or
Cross-Experience Reflection is introduced. This is a local persistence trust
boundary only.

## Privacy Constraints

Tests use synthetic or disposable databases only. No real user database,
journal content, credentials, or provider payload is inspected or mutated.
Audit cleanup behavior keeps the accepted 30-day unsuccessful-record policy;
this slice does not redefine retention.

## User-Agency Constraints

Explicit artifact review, historical consent, generated-artifact deletion, and
source deletion retain their current meaning. Failed or stale mutations must
leave durable state unchanged and may not auto-repair or silently retry through
another path.

## Acceptance Criteria

If Option A is authorized:

1. All six remaining mutation methods invoke named typed Rust commands.
2. No renderer-supplied mutation SQL, generic statement array, or direct plugin
   mutation execute remains in `sqliteLocalEvidenceStore.ts`.
3. Generic `execute_sqlite_transaction` and
   `execute_sqlite_historical_transaction` are removed from the Tauri command
   surface once no authorized caller needs them.
4. Artifact save revalidates the expected Experience revision before DELETE or
   INSERT; stale/missing/failure cases write nothing.
5. Artifact validation, rejected-item exclusion, whole-bundle replacement, and
   dependent Historical Question deletion remain unchanged.
6. Consent ID conflicts cannot rewrite a different packet or timing scope.
7. Transmission persistence and consent consumption are one transaction and
   preserve current outcome rules.
8. Historical Question persistence preserves every existing ADR-0009
   transport-time and persistence-time revalidation and exact dependency row.
9. Historical Question deletion preserves dependent snapshot/provenance
   cascades without deleting unrelated artifacts.
10. Audit cleanup deletes only expired unreferenced audit records under the
    existing policy and is blocked for incompatible databases.
11. Every typed command refuses `user_version > 4` without writing.
12. Failure injection proves rollback; `foreign_key_check` is empty and
    `integrity_check` is `ok` for synthetic fixtures.
13. TypeScript adapter tests prove typed payloads and absence of SQL fields;
    Rust tests prove durable semantics.
14. Canonical verification, Theory Alignment Review, archive/reset, and Founder
    diff review complete before any promotion.

## Risks

- Converting all six methods is broader than Slice 1B-1 and touches the most
  sensitive Phase 3B persistence paths.
- Reimplementing conflict or cascade behavior incorrectly could consume consent
  without a matching transmission, retain stale artifacts, or delete unrelated
  provenance.
- Leaving generic commands registered would preserve an unintended bypass even
  if current TypeScript callers disappear.
- A narrower sub-slice reduces immediate review surface but prolongs the split
  trust boundary and requires another Founder gate.
- Automated tests cannot prove every desktop runtime behavior; a synthetic
  desktop smoke may remain an optional Founder review item after implementation.

## Open Questions

Resolved: the Founder selected Option A. Engineering may plan and implement
only the six typed mutation paths and explicitly authorized supporting tests and
factual documentation.

## Human Decision Required

No. `PHASE3C-SLICE1B2-001` was resolved exactly and recorded by the workflow.

## Recommendation

Proceed with the Founder-authorized **Option A: full bounded Slice 1B-2**. The
six methods are one coherent schema-v4 mutation trust boundary. Implement them
through capacity-resilient micro-block checkpoints and focused tests, then stop
at Founder diff review; promotion remains separately gated.

## Review Status

`approved_with_conditions`
