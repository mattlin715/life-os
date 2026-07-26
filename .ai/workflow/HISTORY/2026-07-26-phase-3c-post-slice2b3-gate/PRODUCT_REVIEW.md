# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-26-phase-3c-post-slice2b3-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: e932ead6da3346d3783da22dc1d1295c31cdc979
- Working-tree digest reviewed: 4cbc1d4c4b2fe7fe8df4830ece66105e894369d126f093b7e492d8966f0e9e39
- Created at: 2026-07-26T04:25:00+09:00
- Updated at: 2026-07-26T04:25:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Truthfully close the already-promoted Slice 2B-3 documentation, audit whether
the fixed schema-v5 contract can support one complete atomic migration against
disposable fixtures, and stop before implementation so the Founder can decide
whether to authorize that bounded migration core.

This is product storage work governed by the existing Engineering Harness. It
is not migration activation, a production recovery feature, or Harness
development.

## Problem Statement

Architecture/13 contains a fixed and reproducibly tested schema-v5 DDL
contract, deterministic ID/canonicalization rules, and a Founder-approved
migration design. Production remains schema v4, and no code performs the
required complete DDL/backfill/reconciliation transaction.

The next safe question is not whether Life OS should silently migrate a user
database. It is whether a private module can prove, with synthetic/disposable
fixtures, that one exact v4 state can become one internally valid v5 state
without inventing history and that every pre-commit failure returns to the
same governed v4 facts.

## User Value

A trustworthy append-only lifecycle foundation is required before users can
inspect revision/rejection history, correct reviewed artifacts without losing
lineage, export complete provenance, or inspect dependencies. A disposable
migration core reduces uncertainty around those future capabilities without
placing real user data at risk or implying that production migration is ready.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human Before AI, Evidence Before Conclusion,
  Privacy Before Profit, and We Build Mirrors, Not Oracles require truthful
  preservation and prohibit silent recovery or invented personal history.
- `docs/06_Memory.md`: Memory is an evidence system that must preserve
  provenance, consent, correction, deletion, selective retrieval, and no silent
  identity accumulation.
- `docs/10_Privacy.md`: personal data remains user-owned; storage and future use
  are distinct; opaque duplication or recovery would weaken trust.
- `docs/12_Roadmap.md`: Phase 3 must preserve review history and provenance;
  Phase 4 remains blocked until all five Phase 3 exit gaps are verified.
- `docs/appendix/Harness.md`: provider behavior and historical-context consent
  remain separate from storage migration.

## Relevant ADRs

- ADR-0007 requires reviewed durable artifacts, authorship, revision history,
  provenance, correction, deletion, and export distinctions.
- ADR-0009 remains authoritative for Historical Question packets, consent,
  transmission, actual-use provenance, and cascade deletion.
- ADR-0010 keeps all five Phase 3 exit gaps blocking and does not authorize
  Phase 4 implementation.
- ADR-0011 accepts the additive normalized lifecycle direction but explicitly
  withholds migration implementation until a separate Founder checkpoint.

## Current Implementation Context

### Promoted and verified

- `develop` and `origin/develop` are
  `e932ead6da3346d3783da22dc1d1295c31cdc979`.
- Slice 2B-3 feature commit
  `61518c7d92b88b9b7d40f2229a30ba9d8370b880` is the second parent of the
  non-fast-forward merge.
- Production `SCHEMA_VERSION` and SQLite `user_version` remain 4.
- All schema-v4 mutations are typed Rust commands.
- Slice 0 freezes the candidate DDL, object digest, deterministic-ID vectors,
  canonicalization rules, fixtures, and invariants.
- Slices 2A through 2B-3 provide only private disposable backup/restore/
  filesystem evidence. They remain unregistered and inactive.
- Canonical baseline passed: workflow 17/17, Vitest 22 files/163 tests, Rust
  library 49/49, backup/restore integration 12/12, schema contract 8/8,
  typecheck, build, Rust check, and hygiene.

### Founder-approved but not implemented

- Architecture/12 and ADR-0011 approve the normalized lifecycle direction.
- Architecture/13 approves the migration/cutover design and fixed candidate
  contract, while explicitly requiring another Founder authorization before
  migration implementation.

### Not authorized or implemented

- No migration module, production schema-v5 DDL, startup/app-data activation,
  real-user backup/restore, migration disclosure, lifecycle writes, export v2,
  retention, or Phase 4 behavior exists.

## Migration Readiness Audit

### A. Executable DDL source

Current facts:

- Architecture/13 contains the approved SQL fence.
- `src-tauri/tests/fixtures/schema_v5/schema_v5.sql` is the executable fixed
  fixture.
- `contract.json` freezes the DDL digest, object names/bodies, IDs,
  canonicalization, manifests, errors, and fixture digests.
- `schema_v5_contract.rs` byte-compares the architecture SQL fence with the
  fixture and validates the executable object manifest.

Recommendation:

- In an authorized Slice 3A, relocate the unchanged executable SQL once to
  `src-tauri/schema/schema_v5.sql`.
- Both the private migration module and contract tests must use `include_str!`
  on that same file.
- Architecture/13 remains the governing human-readable copy, with its existing
  byte-alignment test. `contract.json` remains a verification manifest, not
  another executable DDL source.
- Do not embed another Rust SQL string.

The fixed file must be consumed as ordered, immutable DDL chunks so failure can
be injected after each meaningful object. The existing SQL already separates
core objects from the v4 compatibility-guard section. A deterministic
fixed-contract loader must prove that its ordered chunks reconstruct the exact
canonical file and object manifest; it must not accept user-provided SQL or
generate table names.

### B. Exact schema-v4 inputs

1. `experience_entries`: ID, exact content bytes, created timestamp, updated
   timestamp.
2. `persisted_artifacts`: ID, source ID, kind, exact raw JSON bytes, created
   timestamp, updated timestamp. Current kinds are `evidence`, `reflection`,
   `pattern`, and `recovery_turn`.
3. `historical_consent_events`: exact packet digest, payload, state, creation
   and expiry.
4. `historical_transmission_events`: consent, packet digest, provider, model,
   outcome, creation and expiry.
5. `historical_question_artifacts`: current Experience, exact generated payload,
   exact packet snapshot, consent, transmission, digest, and creation time.
6. `historical_artifact_dependencies`: exact current/source IDs, optional
   artifact ID, and source-revision string.

Current source deletion cascades source-scoped artifacts and dependent
Historical Questions. Historical Question deletion cascades its successful
transmission and consent. Slice 3A must read those facts without changing their
v4 authority or ADR-0009 semantics.

### C. Honest backfill

- Experience content uses exact UTF-8 bytes.
- Existing artifact payloads remain exact `legacy-v4-raw` bytes; parsing is
  allowed only to validate known shape and derive explicit fields, never to
  normalize or reserialize the stored revision content.
- Every surviving record receives one deterministic revision-1
  `legacy_v4_baseline`; no earlier revision is invented.
- Deterministic IDs use the domain-separated contract and abort on a
  non-identical collision.
- Existing confirmed Evidence/Pattern or skipped Reflection/Recovery status may
  create only a `legacy_import` review event at `updated_at` with
  `record_updated_at_not_decision_time`.
- Candidate/suggested/answered records do not gain an invented confirmation or
  click.
- Rejected Evidence/Pattern should not survive in v4. If found, migration
  refuses with `unexpected_rejected_v4_payload`; absent rejection history is
  declared unrecoverable.
- Experience authorship is `user`. Artifact prompt/content/response authorship
  comes only from existing provenance; absent data remains `legacy_unknown`;
  combined prompt and user response may be `mixed`.
- Historical Question packets, consent, transmission, provider/model, packet
  digests, and dependency strings remain byte-for-byte unchanged.

Review-state projection must be deterministic and non-inventive:

- candidate Evidence/Pattern -> `pending`;
- confirmed Evidence/Pattern -> `confirmed` with legacy review event;
- suggested Reflection/Recovery -> `pending`;
- skipped Reflection/Recovery -> `skipped` with legacy review event;
- answered Reflection/Recovery -> `not_applicable` because the saved response
  is authorship, not confirmation of the prompt as truth;
- Historical Question -> `not_applicable`;
- a rejected surviving payload -> refusal, not a reconstructed rejection.

### D. Transaction and reconciliation

One injected path/connection against an exact disposable v4 fixture:

1. open with foreign keys enabled and require exactly `user_version = 4`;
2. capture source schema/object and governed row manifests;
3. `BEGIN IMMEDIATE`, re-read exact v4, and recapture the source manifest;
4. execute the exact fixed core DDL in ordered chunks;
5. insert one migration-scoped compatibility token;
6. backfill source revisions/content/heads, artifact revisions/content/heads,
   provenance, review/lifecycle facts, exact dependencies, and Historical
   Question lifecycle links in referential order;
7. calculate target counts and ordered digests inside the transaction;
8. insert `database_contract` with lifecycle writes and export v2 disabled;
9. insert the immutable migration receipt;
10. install the fixed v4 projection guards last, revalidate the unchanged v4
    projection, and delete the compatibility token;
11. require empty guard, exact reconciliation, current-content invariants,
    `foreign_key_check`, and `integrity_check`;
12. set `user_version = 5` as the last SQL mutation and commit;
13. close and reopen read-only;
14. require exact version, receipt, contract, schema-object digest, empty guard,
    manifests, current pointers, foreign keys, and integrity.

“Exact v4 rollback” means the same schema objects, `user_version = 4`, governed
row counts, field bytes, source manifest, and ADR-0009 records after reopening.
It does not claim a physically byte-identical SQLite file, because journal/page
layout may change during a transaction without changing governed data.

### E. Recovery

- Any natural or injected pre-commit failure must roll back every table,
  trigger, row, receipt, guard, and version change.
- A failure after commit cannot be represented as v4. It returns a blocked
  `committed_verification_failed`/`recovery_required` result and preserves the
  disposable v5 fixture for inspection.
- No automatic restore, retry, replay, repair, down migration, sidecar cleanup,
  or candidate selection.
- Windows replacement remains unavailable for normal activation because
  required parent-directory durability is unsupported.

## In Scope

Only if separately authorized:

- one private, unregistered, path/connection-injected Rust migration module;
- unchanged shared fixed DDL and contract alignment;
- synthetic/disposable exact-v4 fixtures;
- complete atomic DDL/backfill/reconciliation/receipt/version transaction;
- deterministic IDs, exact legacy bytes, honest review timestamp quality;
- exact Historical Question dependency/provenance preservation;
- failure injection after every meaningful DDL, backfill, reconciliation,
  receipt, guard, and version boundary;
- pre-commit exact-v4 rollback proof;
- post-commit close/reopen read-only verification and blocked failure result;
- malformed, v2, v3, v5, and greater-than-v5 refusal;
- focused tests, canonical verification, factual documentation, Theory Review,
  archive/reset, and stop at Founder diff review.

## Out Of Scope

- Production `SCHEMA_VERSION = 5`.
- Fresh-install v5 or v2/v3 stabilization combined with v5.
- Real user database or app-data paths.
- Startup, Tauri, renderer, UI, disclosure, or migration activation.
- Production backup/restore/replacement or retention.
- Runtime v5 reads/writes, compatibility projection mutations after migration,
  lifecycle commands, cleanup, inspector, export v2, or import.
- Phase 4, provider, ContextPacket, Product Harness, or consent changes.
- Engineering Harness expansion, Stage 2/3, Git promotion, PR, or deployment.

## Product Constraints

- The result must be one internally complete v5 transaction or unchanged v4;
  no successful partial v5 state is allowed.
- The fixed DDL and contract remain authoritative; the migration module cannot
  silently “fix” records or schema drift.
- Production behavior and supported schema maximum remain 4.
- The test module must remain unreachable from application startup and Tauri.

## Evidence And Provenance Constraints

- Preserve exact legacy payload bytes and explicit source timestamps.
- Do not invent rejection history, review clicks, authorship, provenance,
  consent, or dependency facts.
- Exact provenance objects may deduplicate only under the approved fingerprint.
- Missing provenance remains visibly `legacy_unknown`.
- Every reconciliation claim requires ordered count/digest evidence.

## Historical Context Constraints

ADR-0009 tables remain authoritative. Packet snapshots are not generic artifact
revision content. Historical Question dependencies must map exactly to baseline
source/artifact revisions or the migration aborts. No task, packet, eligibility,
provider, retention, or deletion rule changes.

## Consent Constraints

No consent event is created, reinterpreted, consumed, extended, or reused.
Existing bytes are preserved. Local migration authority would not authorize a
provider call or longitudinal-use consent.

## AI-Role Constraints

No AI output, inference, pattern, summary, identity claim, or Phase 4 behavior
is generated. Migration preserves stored facts; it does not decide meaning.

## Privacy Constraints

Only synthetic/disposable fixtures may be opened. No real user path, database,
backup, packet, or journal content enters tests or workflow artifacts.

## User-Agency Constraints

No silent production migration or automatic recovery. Production activation,
disclosure, explicit user initiation, cancellation, backup retention, and
restore remain separate future decisions.

## Acceptance Criteria

For an authorized Slice 3A:

1. production schema maximum remains 4 and no runtime caller exists;
2. the migration and tests consume one shared unchanged DDL file;
3. exact v4 is the only accepted source version;
4. every v4 table and artifact kind is covered;
5. legacy raw content bytes and ADR-0009 bytes are unchanged;
6. deterministic IDs and provenance fingerprints reproduce across independent
   disposable migrations;
7. review/authorship/timestamp quality is honest and rejection history is not
   invented;
8. target counts, digests, dependencies, current pointers, guards, receipt,
   foreign keys, and integrity reconcile before version change;
9. `user_version = 5` is the last mutation;
10. every injected pre-commit failure reopens as exact governed v4;
11. post-commit verification failure returns blocked without autonomous action;
12. malformed/v2/v3/v5/>v5 inputs refuse without mutation;
13. lifecycle writes and export v2 remain disabled;
14. focused and canonical verification pass;
15. Theory Alignment Review approves the exact diff;
16. sprint archives/resets and stops at Founder diff review.

## Alternatives

### A — Complete private disposable-fixture migration core

- Product value: proves the storage cutover needed for four Phase 3 foundation
  gaps without activating it.
- Data/privacy risk: bounded to synthetic fixtures; primary risk is false
  confidence or accidental later activation.
- Reversibility: code can be removed before activation; every pre-commit test
  rolls back to v4.
- Evidence: strongest next evidence for DDL/backfill/reconciliation.
- Phase 3 gaps: closes none by itself, but builds the storage prerequisite for
  revision/rejection history, post-review correction/deletion, complete export,
  and provenance/dependency inspection. Structured retrieval is unaffected.
- Remaining blockers: production recovery, disclosure, process-wide
  quiescence, Windows durability, runtime v5 write parity, lifecycle/UI/export,
  real-data verification, and Founder activation authority.

### B — Production startup/app-data migration activation

- Product value: makes v5 available to actual users sooner.
- Data/privacy risk: high; touches intimate real data without proven recovery,
  disclosure, quiescence, or Windows durability.
- Reversibility: unsafe; no automatic down migration and an older binary cannot
  safely write v5.
- Evidence: current fixture contracts are insufficient for real-user recovery.
- Phase 3 gaps: still would not supply lifecycle UI, export, inspector, or
  structured retrieval.
- Remaining blockers: all production activation and recovery items above.

### C — Defer migration and implement structured retrieval

- Product value: directly advances the separate emotion/relationship/
  value-conflict/time-range retrieval gap.
- Data/privacy risk: lower storage risk, but retrieval governance and
  explainability still need Founder design.
- Reversibility: high if kept local and bounded.
- Evidence: does not test lifecycle migration, rollback, or provenance export.
- Phase 3 gaps: can advance gap 4; gaps 1, 2, 3, and 5 remain.
- Remaining blockers: the full lifecycle/provenance foundation and all Phase 4
  gates.

## Risks

- A fixed DDL file contains both core objects and guards that architecture/13
  requires to be installed last. The loader must execute contract-verified
  ordered sections rather than one opaque batch.
- SQL statement parsing must not become a second editable schema. Ordered
  chunks must reconstruct the exact shared bytes and object manifest.
- Artifact payload shape/provenance drift can make an honest migration refuse.
  Refusal is safer than normalization by guess.
- Logical exact rollback must not be overstated as physical SQLite-file byte
  identity.
- Deterministic-ID collision, unresolved Historical dependencies, guard drift,
  or manifest mismatch must abort.
- Passing disposable tests could be misread as production readiness.

## Open Questions

- None. The Founder resolved `PHASE3C-SLICE3A-001` by selecting Option A.

## Human Decision Required

false — `PHASE3C-SLICE3A-001` was resolved by the exact Founder response
recorded in `DECISION_REQUIRED.md`.

## Founder Resolution

- Selected option: Option A.
- Authorized implementation: private, unregistered, path/connection-injected
  disposable-fixture migration core only.
- Binding conditions: production `SCHEMA_VERSION` remains 4; no real user
  database, app-data, startup, Tauri, renderer, UI, migration disclosure,
  production backup/restore/replacement, lifecycle writes, export v2, later
  slices, Phase 4, Harness expansion, Git promotion, PR, or deployment.
- Review conclusion: the authorized scope exactly matches the recommended
  bounded alternative and may proceed to Engineering Planning.

## Recommendation

Authorize Alternative A only. It is the smallest complete migration proof and
keeps real data, runtime activation, recovery claims, later slices, and Phase 4
closed. Do not authorize Alternative B. Alternative C remains a valid later
product sprint but does not replace the lifecycle storage foundation.

## Review Status

approved_with_conditions.
