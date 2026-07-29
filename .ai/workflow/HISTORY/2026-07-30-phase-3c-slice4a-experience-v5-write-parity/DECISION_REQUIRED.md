# Decision Required

Status: resolved
- Sprint ID: 2026-07-30-phase-3c-slice4a-experience-v5-write-parity
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-30T01:15:00+09:00
- Updated at: 2026-07-30T01:15:00+09:00

## Decision ID

`PHASE3C-SLICE4A-001`

## Sprint ID

`2026-07-30-phase-3c-slice4a-experience-v5-write-parity`

## Decision Summary

Decide whether to authorize one private, unregistered, path/connection-injected
Rust boundary that proves Experience create, exact-revision correction, parent
delete, and duplicate-skipping import against synthetic/disposable exact-v5
fixtures. It would update normalized v5 authority and the guarded v4
compatibility projection in one transaction, then verify the result read-only.
It would not activate schema v5 in production or create a user-visible feature.

## Why Automation Stopped

The proposed command writes append-only personal-history records, defines
deletion behavior, and applies ADR-0009 source invalidation. ADR-0011 and
architecture/13 establish the design but explicitly require a later Founder
gate for each implementation slice. Tests, prior migration promotion, and this
proposal are not implementation authority.

## Relevant Constitution Clauses

- `docs/00_Constitution.md`: “We Build Mirrors, Not Oracles.”
- User content and meaning remain user-owned.
- Local-first control, transparency, provenance, correction, deletion, and
  consent cannot be weakened for implementation convenience.

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/03_Principles.md`
- `docs/06_Memory.md`
- `docs/09_AI.md`
- `docs/10_Privacy.md`
- `docs/appendix/Harness.md`

Together they require user-authored Experience content to remain distinct from
AI output, locally controlled, provenance-bearing, correctable, deletable, and
excluded from silent inference.

## Relevant ADRs

- ADR-0007: reviewed durable records require provenance and lifecycle clarity.
- ADR-0009: source correction/deletion invalidates exact historical use and
  cascade-deletes dependent Historical Questions and their successful packet
  evidence; it never silently rebinds them.
- ADR-0011: normalized v5, append-only Experience revisions, separate
  purgeable content, exact dependencies, content-free deletion evidence, and
  non-destructive feature disable are accepted policy. Production activation
  remains separately gated.

## Available Options

### Option A — Private disposable v5 Experience write parity

Authorize only:

- exact-v5 fixtures produced from the promoted migration core;
- one private, unregistered, path/connection-injected Rust mutation boundary;
- Experience create, exact-current-revision update, parent delete, and atomic
  duplicate-skipping v4-format import;
- synchronized v5 authority and v4 projection;
- deterministic clocks and identifiers injected for tests;
- bounded failure injection, read-only reconciliation, factual Book One
  documentation, canonical verification, Theory Alignment Review,
  archive/reset, and stop at Founder diff review.

#### Exact transaction contract

**Common entry gate and result states**

1. Open only a caller-supplied disposable path/connection. Before
   `BEGIN IMMEDIATE`, require `user_version = 5`, the fixed schema-object
   digest, exactly one immutable migration receipt, the exact
   `database_contract` (`authoritative_schema=5`,
   `compatibility_projection=enabled`, lifecycle/export still disabled), an
   empty guard, current-content invariants, foreign-key integrity, database
   integrity, and exact source-head/v4 projection parity.
2. Refuse malformed/unreadable files, v4, v2/v3, existing inconsistent v5,
   version greater than 5, missing/extra receipt or contract rows, altered
   schema objects, nonempty guard, or projection drift. Refusal is read-only.
3. Outcomes are explicit: `committed`, `stale_revision`, `not_found`,
   `duplicate_skipped` counts for import, or blocked
   `recovery_required`. No retry, repair, restore, or fallback occurs.

**Identifiers, clocks, and optimistic revision**

4. The optimistic value is the exact
   `source_heads.current_revision_id`, not v4 `updated_at`. Update and delete
   re-read and match that ID inside `BEGIN IMMEDIATE`; mismatch writes nothing.
5. Revision number is `1` for a new/baseline source and otherwise the
   predecessor's checked `revision_number + 1`. The predecessor must be the
   same source's active current revision and must still have content.
6. Content digest is lowercase SHA-256 over exact UTF-8 bytes without Unicode
   normalization. Source revision IDs reuse the promoted domain-separated
   `v5sr_ + SHA-256(source_id, operation/import updated_at, content_digest)`
   contract; a collision with non-identical facts aborts. User provenance uses
   the exact canonical `life-os/provenance-v1` fingerprint and may deduplicate
   only on exact equality.
7. A fixture supplies canonical UTC timestamps and a deterministic sequence of
   unique high-entropy-shaped guard tokens. The reusable seam requires an
   unpredictable token source; deterministic tokens are test evidence only.
   Create/update/delete timestamps must be canonical and strictly advance where
   a head advances.

**Guard and ordering**

8. After all expected-state checks and inside one `BEGIN IMMEDIATE`, insert
   exactly one guard token. It authorizes only that connection's v5/v4
   projection work; it is not a security boundary.
9. Write v5 authority first, then the v4 projection:
   revision metadata -> revision content -> head/current pointer -> exact
   provenance link -> required dependency invalidation/cascade -> v4
   `experience_entries` projection. A create inserts the head only after
   revision content exists.
10. Reconcile the expected v5 current state and v4 row, verify authorized
    Historical Question effects, delete the exact token, require guard count
    zero, then run current-content, foreign-key, and integrity checks before
    commit. Any v5, cascade, projection, reconciliation, token, or check failure
    rolls back the whole transaction.

**Create**

11. Create adds one `source_revisions` row (`revision_number=1`,
    `authorship=user`, `revision_reason=created`,
    `serialization_version=utf8-text-v1`, no predecessor), one exact
    `source_revision_content` row, one exact user provenance record/link, one
    active `source_heads` row, and one matching v4 `experience_entries` row.
    It creates no artifact, review/lifecycle event, dependency, or tombstone.
    Existing active or deleted source identity is a conflict and writes
    nothing.

**Update/correction**

12. Update appends one immutable user-authored `corrected` source revision with
    the exact prior current revision as predecessor, retains all prior revision
    metadata/content/provenance, and advances only the source head and matching
    v4 row. Existing exact dependencies are never rewritten to the new
    revision.
13. Affected Historical Questions for which the source is current or included
    are deleted under ADR-0009 inside the same guarded transaction. The existing
    v4 provenance trigger deletes consent/transmission records and the promoted
    v5 bridge removes the linked historical lifecycle projection.
14. If any ordinary non-historical artifact head or exact-revision dependent
    would require Decision 10B invalidation/reconfirmation, update fails closed.
    Slice 4A must not simulate that lifecycle by deleting or rebinding it;
    ordinary artifact parity remains Slice 5 work.

**Parent Experience delete**

15. Delete also requires the exact current source revision. It first applies
    the ADR-0009 Historical Question cascade, then removes source-scoped child
    artifact heads/contents/events/dependencies/tombstones only as the already
    approved consequence of deleting their parent Experience. Any external
    ordinary dependent requiring retained-invalidated behavior blocks the
    operation.
16. Delete clears the source current pointer, marks the source head `deleted`
    with the injected action time, purges all source revision content, removes
    the v4 Experience row, and leaves only content-free source revision
    metadata/digests/provenance plus the deleted source head. It creates no new
    `content_tombstones` row: artifact tombstones expire with parent deletion,
    and a separately retained source-delete tombstone was not approved.
    Deleted content cannot be retrieved or reconstructed.

**Atomic import**

17. Import is one ordered batch transaction. For each ID, an exact consistent
    active source/v4 pair or an already deleted source head is skipped; a
    one-sided or mismatched identity is integrity failure. For repeated IDs in
    the input, the first occurrence is authoritative for the batch and later
    occurrences are skipped deterministically.
18. Each new v4-format entry becomes exactly one honest
    `legacy_v4_baseline` revision (`revision_number=1`, `authorship=user`,
    `serialization_version=legacy-v4-raw`) using its exact current body and
    preserved timestamps. It does not invent created/corrected events or prior
    revisions. Any non-duplicate error rolls back every new row in the batch.

**Read-only reconciliation, rollback, and restart**

19. Before commit, compute a versioned operation manifest over exact affected
    v5 source authority, v4 projection, and authorized ADR-0009 rows. Do not
    rewrite or compare mutable current state to the immutable migration receipt
    manifests; those receipts describe cutover time.
20. After commit, close the writable connection and reopen read-only. Require
    the expected operation manifest, source-head/current-content/v4 parity,
    exact row counts/digests, immutable receipt and schema contract, fixed
    schema-object digest, empty guard, foreign keys, and integrity.
21. A generic commit error is outcome-unknown. Close the connection and
    classify read-only: exact pre-manifest is `failed_unchanged`; exact
    post-manifest is `committed`; anything else is `recovery_required`. No
    autonomous action follows. Injected failure tests prove guard absence and
    exact logical rollback after reopen.
22. This slice has no durable per-write operation receipt. Immediate
    close/reopen and injected ambiguous-commit evidence are fixture-local; a
    real process crash after commit and production restart recovery remain
    unproved and unauthorized.

**Reusable versus fixture-only**

23. Reusable production-quality code may include the private typed command
    inputs/outcomes, canonical digest/ID functions, transaction ordering,
    fail-closed validators, guard discipline, and read-only reconciler.
24. Fixture-only evidence includes the path caller, migrated synthetic
    databases, deterministic clock/token providers, failure/commit adapters,
    expected in-memory manifests, and all executions. The module remains
    unregistered and unreachable from Tauri, renderer, startup, UI, app-data,
    and real user databases.

### Option B — Contracts and fixtures only

Freeze typed inputs, outcome types, canonical manifests, and test fixtures, but
do not execute any Experience mutation.

### Option C — Defer Slice 4A

Make no Slice 4A changes and separately evaluate structured retrieval R2.

## Benefits

- **A:** Proves the smallest authority/projection transaction, correction
  lineage, stale refusal, and ADR-0009 cascade before any runtime activation.
- **B:** Lowest mutation risk, but repeats contract-only work already supported
  by promoted schema/migration evidence and leaves the central write risk
  untested.
- **C:** Prioritizes visible retrieval progress, but leaves the Phase 3C
  current-state write foundation blocked.

## Risks

- **A:** Most complex option; deletion and dependency boundaries can
  accidentally overreach. The exact fail-closed ordinary-artifact fence and
  disposable-only execution are required.
- **B:** Paper confidence may hide transaction-order and rollback defects.
- **C:** Defers infrastructure needed for append-only correction/deletion and
  complete provenance/export.

For every option, passing tests is not production authorization.

## Reversibility

- **A:** Code remains private/unregistered and executes only on disposable
  fixtures. It can be removed without touching user data. No down migration is
  created.
- **B:** Pure contract/fixture changes are fully removable.
- **C:** No repository behavior changes.

## Data And Privacy Impact

Option A touches only synthetic/disposable data. Its contract retains
superseded user content locally but purges all source content on parent
deletion, does not retain rejected/deleted hidden content, and preserves
ADR-0009 deletion. It adds no network, analytics, provider, clipboard,
telemetry, export, backup, or real-user path.

## Orchestrator Recommendation

**Option A**, exactly as bounded above. It is the smallest executable evidence
that reveals transaction, projection, stale-write, and cascade defects while
keeping production and ordinary artifact lifecycle activation blocked.

## Default Safe Action

Remain at `human_decision_required`. Make no implementation changes.

## Blocked Files Or Phases

Engineering Plan, Rust mutation code/tests, architecture/13 Slice 4A factual
implementation status, verification, Theory Review, archive, and every Git or
deployment action remain blocked pending the exact Founder response.

## Exact Founder Response Needed

To authorize Option A, reply exactly:

> I resolve PHASE3C-SLICE4A-001 by selecting Option A. I authorize Phase 3C
> Slice 4A only: exact-v5 disposable fixtures produced from the promoted
> migration core; one private, unregistered, path/connection-injected Rust
> Experience mutation boundary implementing the exact transaction contract in
> the Founder decision package for create, exact-current-revision update,
> parent delete, and atomic duplicate-skipping v4-format import; synchronized
> v5 authority and guarded v4 projection; deterministic injected fixture
> clocks, identifiers, guard tokens, failure points, commit outcomes, and
> expected manifests; ADR-0009 Historical Question cascade preservation;
> fail-closed refusal where ordinary artifact lifecycle parity is required;
> read-only post-transaction reconciliation; focused synthetic/disposable
> tests; factual Book One documentation; canonical verification; Theory
> Alignment Review; archive/reset; and stop at Founder diff review. I accept
> that this does not prove production restart recovery, real-user safety, or
> production schema-v5 readiness. I do not authorize production
> SCHEMA_VERSION 5, production user_version 5, real user databases or app-data,
> startup/Tauri/renderer/UI activation, ordinary artifact/Reflection/Pattern or
> Phase 3B write parity, lifecycle UI, export v2, retention, backup/restore,
> automatic retry/recovery/repair/cleanup, provider or ContextPacket changes,
> Phase 4, Harness expansion, staging, commit, push, merge, PR, deployment, or
> release.

The Founder may instead select Option B or C and state its exact authorized
scope. Silence never resolves this decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4A-001 by selecting Option A. I authorize Phase 3C Slice 4A only: exact-v5 disposable fixtures produced from the promoted migration core; one private, unregistered, path/connection-injected Rust Experience mutation boundary implementing the exact transaction contract in the Founder decision package for create, exact-current-revision update, parent delete, and atomic duplicate-skipping v4-format import; synchronized v5 authority and guarded v4 projection; deterministic injected fixture clocks, identifiers, guard tokens, failure points, commit outcomes, and expected manifests; ADR-0009 Historical Question cascade preservation; fail-closed refusal where ordinary artifact lifecycle parity is required; read-only post-transaction reconciliation; focused synthetic/disposable tests; factual Book One documentation; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that this does not prove production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, real user databases or app-data, startup/Tauri/renderer/UI activation, ordinary artifact/Reflection/Pattern or Phase 3B write parity, lifecycle UI, export v2, retention, backup/restore, automatic retry/recovery/repair/cleanup, provider or ContextPacket changes, Phase 4, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Phase 3C Slice 4A only: private unregistered path/connection-injected Rust Experience create, exact-current-revision update, parent delete, and atomic duplicate-skipping v4-format import against promoted-migration exact-v5 disposable fixtures; synchronized v5 authority and guarded v4 projection; deterministic injected fixture clock/IDs/tokens/failures/commit outcomes/manifests; ADR-0009 cascade; ordinary-artifact lifecycle fail-closed; read-only reconciliation; focused tests; factual Book One docs; canonical verification; Theory Alignment Review; archive/reset; stop at Founder diff review. All production activation, real data, runtime integration, later lifecycle/write slices, Phase 4, Harness expansion, Git promotion, deployment and release remain unauthorized.

## Decided At And Evidence Reference

- Decided at: 2026-07-29T16:18:49.690Z
- Evidence reference: PHASE3C-SLICE4A-001 founder response in current Codex task on 2026-07-30

## Resume Phase

product_review
