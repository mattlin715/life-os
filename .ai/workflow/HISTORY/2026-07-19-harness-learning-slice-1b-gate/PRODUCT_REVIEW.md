# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-19-harness-learning-slice-1b-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 6e9dd6615cb7f99556d258080f6a45140fd55b68
- Working-tree digest reviewed: 1cf2402bb2f71de13212b9f8acc950921562af7893b893774626f7e402a9daff
- Created at: 2026-07-18T15:46:17.424Z
- Updated at: 2026-07-18T15:46:17.424Z

## Mission Interpretation

Close only factual Slice 1A and Engineering Harness learning drift, assess five
real archived sprints without self-approval, define a disposable Slice 1A
manual-review checklist, and stop for two separate Founder decisions before any
Slice 1B production implementation.

## Problem Statement

Slice 1A is promoted, but `architecture/13` still described it as unpromoted.
The Engineering Harness documents counted four rather than five real sprints.
The five archives now support a bounded Stage 1 operational conclusion, but only
the Founder may accept it. Separately, renderer-owned Experience mutations still
send SQL or directly execute SQL, which leaves the intended typed Rust mutation
boundary incomplete; architecture approval alone does not authorize that work.

## User Value

Factual documentation prevents false project-state decisions. A bounded typed
Experience mutation boundary would remove renderer control over SQL while
preserving local-first schema-v4 behavior, transactionality, deletion
semantics, and user-owned data. Separate decisions prevent Harness learning
from silently becoming product implementation authority.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI, Evidence before Conclusion,
  Privacy before Profit, and documentation truth.
- `docs/06_Memory.md`: memory remains user-controlled, revisable, and
  provenance-preserving.
- `docs/Reflection.md`: interpretation and meaning remain user-owned.
- `docs/09_AI.md`: AI is a mirror and context steward, not an oracle.
- `docs/10_Privacy.md`: local control, transparency, and ongoing consent protect
  psychological safety.
- `docs/appendix/Harness.md`: evidence supports evaluation but does not replace
  human review.

## Relevant ADRs

- `ADR-0007`: reviewed AI artifacts require provenance and user control.
- `ADR-0008`: Engineering Harness governance is tool-independent; its Accepted
  decision is unchanged.
- `ADR-0009`: historical consent, transmission, provenance, stale-work, and
  deletion guarantees must not regress.
- `ADR-0011`: append-only lifecycle direction is Accepted, while production
  migration remains separately gated.

## Current Implementation Context

Implemented and promoted:

- Slice 0: feature `431c3e7e81b2dcefca873ad3ec1d73680c96d60c`,
  non-fast-forward merge `7921affde544a5852aa58782a1acb0a4f189520e`.
- Slice 1A: feature `d4f86d72350c7068db76a6706ad7e5ff10ee67b5`,
  non-fast-forward merge `6e9dd6615cb7f99556d258080f6a45140fd55b68`.
- Application version is `0.2.0`; production `SCHEMA_VERSION` and SQLite
  `user_version` remain 4. Newer databases fail closed before writable store
  construction.

Observed Experience mutation boundary:

- `sqliteLocalEvidenceStore.ts` directly executes renderer-authored SQL for
  create and import and sends generic statement arrays to
  `execute_sqlite_transaction` for update and delete.
- The generic Rust transaction command executes arbitrary statements supplied
  by the renderer. Its optional Experience revision check is coupled to the
  first statement's first bound value.
- Update invalidates source artifacts and dependent Historical Questions;
  delete relies on schema-v4 foreign keys/triggers and explicit artifact
  deletion. Those semantics must remain atomic.
- Artifact, historical-consent, transmission, and provenance mutation paths are
  materially broader and remain a separate Slice 1B-2 boundary.

Founder-approved but not production-authorized:

- Schema-v5 production DDL, `user_version = 5`, migration, backup, restore,
  retention cleanup, lifecycle UI, export v2, and later architecture/13 slices.

## In Scope

- Factual promotion and five-sprint closeout corrections.
- Evidence-based proposal for `HL-001`.
- Disposable Slice 1A manual-review checklist.
- Alternatives and Founder gate for Phase 3C Slice 1B.
- If separately authorized later: only Slice 1B-1 typed Rust Experience
  create/update/delete/import commands and adapters, with schema v4 unchanged.

## Out Of Scope

- Engineering Harness expansion, Stage 2, Stage 3, autonomous repair, replay,
  concurrent agents, deployment, PR, or promotion.
- Slice 1B implementation before exact Founder authorization.
- Artifact/historical/consent/provenance typed mutation conversion (Slice
  1B-2), schema-v5 DDL or activation, migration, backup/restore/retention,
  later slices, Phase 4, provider/ContextPacket changes, or lifecycle UI.
- Real user-database mutation and browser/UI automation in this gate.

## Product Constraints

- Preserve schema v4 and all fresh/v2/v3/v4 startup compatibility.
- Preserve `LocalEvidenceStore` behavior across SQLite and in-memory adapters.
- Renderer may send typed data, never SQL, for any authorized Slice 1B-1 path.
- Update revision validation and all invalidation/deletion effects must occur in
  one Rust transaction and fail closed without partial mutation.
- Import must use a typed transaction boundary and preserve duplicate handling;
  any proposed all-or-nothing change must be explicit and tested.

## Evidence And Provenance Constraints

Experience update/delete must preserve artifact invalidation, historical
dependency deletion, consent/transmission cleanup, and actual-use provenance
semantics already enforced by schema-v4 triggers and ADR-0009. No AI artifact
may be reclassified as user evidence. Regression fixtures must be synthetic.

## Historical Context Constraints

No historical retrieval, packet, transport, or Cross-Experience behavior may
change. Experience mutations must continue to remove stale dependent Historical
Questions and their governed provenance exactly once and transactionally.

## Consent Constraints

No consent policy or provider transmission changes are authorized. Existing
generated-artifact and source deletion cascades must not strand or recreate
consent/transmission records.

## AI-Role Constraints

No AI inference or product response behavior changes. The Engineering Harness
may report evidence and recommend options; it cannot approve itself or infer a
Founder decision.

## Privacy Constraints

All manual and automated checks use synthetic or disposable databases. Do not
open or mutate the Founder's real Life OS database. No journal, provider payload,
secret, or sensitive-history fixture enters the repository.

## User-Agency Constraints

Experience edits and deletions remain explicit user actions. A stale edit must
not overwrite newer content or delete dependent artifacts. No automatic repair,
migration, background profiling, or hidden history use is introduced.

## Acceptance Criteria

Gate acceptance:

1. Slice 1A promotion facts and the five-sprint count are synchronized.
2. The ten Harness criteria cite archived evidence and keep the conclusion
   Founder-gated.
3. Manual verification is recorded as `not_run` unless the Founder actually
   performs the disposable checks.
4. `HL-001` and `PHASE3C-SLICE1B-001` remain separate decisions.

If Slice 1B-1 is later authorized:

5. Typed Rust commands own Experience create/update/delete/import SQL.
6. No renderer-supplied SQL reaches those four paths.
7. Schema and `user_version` remain v4; no user-database migration occurs.
8. Create/import preserve IDs, timestamps, duplicate behavior, and current
   public storage semantics.
9. Update revalidates the expected revision inside the transaction; mismatch
   fails closed with no content or dependent-record mutation.
10. Update/delete preserve schema-v4 artifact and historical-provenance cascade
    semantics; failure injection proves rollback.
11. Artifact and historical mutation conversion remains untouched for Slice
    1B-2.
12. Canonical verification, Theory Alignment Review, archive/reset, and Founder
    diff review complete before any Git promotion.

## Disposable Slice 1A Manual-Review Checklist

Status: `passed_with_procedure_correction`; owner: Founder; completed on
2026/07/19. No browser/UI automation was used.

Use a disposable database path only, preserving the real application database:

1. Back up nothing and do not point the app at the real Life OS data directory.
2. Launch against a missing disposable database and confirm calm startup reaches
   `ready` and creates only schema v4.
3. Launch against a disposable v4 fixture and confirm existing synthetic data
   remains readable and writable.
4. Launch against a disposable database with `user_version = 5` and confirm the
   English, Traditional Chinese, and Japanese compatibility disclosure is calm
   and explicit.
5. In the newer-database state, confirm timeline cleanup, reads, and writes do
   not run and no writable store is constructed.
6. Close the app and confirm the disposable v5 fixture's bytes or digest are
   unchanged.
7. Launch against a malformed disposable file and confirm fail-closed local
   disclosure without repair, downgrade, or mutation.
8. Record date, app version, fixture paths/digests, observed language, and result;
   delete only disposable fixtures when finished.

Observed results:

- A custom Tauri identifier isolated the verified runs from the real Life OS
  application-data directory.
- Missing-database startup created schema v4; a synthetic Experience survived a
  restart and remained editable; `integrity_check` returned `ok`.
- A marker-bearing `user_version = 5` fixture was refused. Traditional Chinese,
  English, and Japanese disclosures all stated detected 5, supported maximum 4,
  no change, and no automatic repair, downgrade, or write. Its SHA-256 was
  unchanged after launch, language switching, and close.
- A malformed fixture was refused without a fabricated version, repair, or
  mutation; its SHA-256 was unchanged.
- The Founder removed all six disposable Roaming/Local application-data paths,
  and no Life OS process remained.

Procedure correction and evidence limit:

- The first isolation attempt relied on overriding `APPDATA`, but Tauri's
  Windows resolver still opened the real `com.lifeos.app` database. The process
  was stopped immediately and all later cases used distinct Tauri identifiers.
- The real database was schema v4 with `integrity_check = ok`, and its hash did
  not change after the guard was captured. Because no hash existed before the
  first failed isolation attempt, pre-attempt logical equality cannot be proven.
  This is recorded as a manual-procedure correction, not hidden or represented
  as stronger evidence.

## Risks

- Full Slice 1B combines Experience, artifact, historical, consent, and
  provenance conversion, increasing regression and review surface.
- A narrow Slice 1B-1 leaves generic renderer SQL in deferred artifact and
  historical paths; documentation must state this honestly.
- Moving import into one Rust transaction can make failure atomic rather than
  partially applied; this is safer but must be explicitly tested and disclosed.
- Incorrect update ordering could delete dependent artifacts before discovering
  a stale revision. The revision check must precede every mutation inside the
  same transaction.
- Database triggers are part of current deletion semantics; duplicating them in
  Rust could create unintended cascades.
- The five-sprint Harness evidence has one sequential orchestrator, not an
  independent reviewer.

## Open Questions

1. `HL-001` is resolved as `HL-A`: the bounded Stage 1 conclusion is
   Founder-accepted, without independent-review assurance or Stage 2/3
   authority.
2. `PHASE3C-SLICE1B-001` is resolved as Option B: only Slice 1B-1 is
   authorized.
3. Manual review is resolved: completed with the documented procedure
   correction and evidence limit.

## Human Decision Required

No. `HL-001` and `PHASE3C-SLICE1B-001` were resolved exactly by the Founder;
the authorized implementation remains bounded by Option B.

## Recommendation

Implement the Founder-selected Option B only. Convert
Experience create/update/delete/import to typed Rust commands, preserve schema
v4 and current user behavior, and defer artifact/historical mutation conversion
to a separately reviewed Slice 1B-2. Stop again at Founder diff review.

## Review Status

approved_with_conditions
