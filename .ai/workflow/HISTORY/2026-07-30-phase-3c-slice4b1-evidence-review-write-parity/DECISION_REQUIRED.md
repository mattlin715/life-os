# Decision Required

Status: resolved
- Sprint ID: 2026-07-30-phase-3c-slice4b1-evidence-review-write-parity
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-30T04:15:00+09:00
- Updated at: 2026-07-30T04:15:00+09:00

## Decision ID

`PHASE3C-SLICE4B1-001`

## Sprint ID

`2026-07-30-phase-3c-slice4b1-evidence-review-write-parity`

## Decision Summary

Decide whether to authorize one private, unregistered,
path/connection-injected Rust boundary that proves Evidence candidate creation,
still-pending correction, explicit confirmation, and rejection against
synthetic/disposable exact-v5 fixtures. It would maintain normalized v5
authority and the guarded v4 current-state projection in one transaction, then
verify the outcome read-only. It would not activate schema v5 in production or
create a user-visible feature.

## Why Automation Stopped

This proposal defines append-only authorship, exact user review, rejected
content purge, dependency behavior, and compatibility projection semantics for
personal Evidence. ADR-0011 and architecture/13 approve the design direction
but require separate Founder authority for each implementation slice. Prior
Slice 4A promotion, tests, and this proposal are not implementation authority.

## Relevant Constitution Clauses

- `docs/00_Constitution.md`: “We Build Mirrors, Not Oracles.”
- Meaning is co-created but user-owned.
- Local-first control, provenance, correction, deletion, consent, and
  non-final AI hypotheses cannot be weakened for storage convenience.

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/03_Principles.md`: Evidence supports reflection but is not objective
  truth; the system distinguishes recorded experience, AI interpretation, and
  user judgment.
- `docs/06_Memory.md`: durable context requires provenance, correction,
  deletion, and user control.
- `docs/Reflection.md`: hypotheses remain revisable and non-authoritative.
- `docs/09_AI.md`: AI output is tentative and reviewable.
- `docs/10_Privacy.md`: rejected/private content cannot become silent retained
  profiling material.
- `docs/appendix/Harness.md`: artifact provenance and task boundaries remain
  provider-independent.

## Relevant ADRs

- ADR-0007 requires durable reviewed artifacts to retain source, authorship,
  provenance, review, revision, correction, and deletion distinctions.
- ADR-0009 makes only confirmed exact-revision Evidence eligible for bounded
  historical use and requires fail-closed source/dependency invalidation.
- ADR-0011 requires append-only revisions, exact review/lifecycle events,
  immediate rejected-content purge with content-free history, retained
  superseded content, reconfirmation after correction, exact dependencies, and
  v4 as a compatibility projection. Production activation remains separately
  gated.

## Available Options

### Option A — Private disposable Evidence candidate and review parity

Authorize only:

- exact-v5 fixtures produced through the promoted migration core;
- one private, unregistered Rust Evidence mutation boundary;
- AI/local-mock candidate creation;
- correction of an exact current still-pending candidate through a new
  user-authored revision;
- explicit confirmation or rejection of one exact current pending revision;
- synchronized normalized v5 authority and guarded v4 projection;
- exact source revision dependencies, deterministic fixture inputs, failure
  injection, read-only reconciliation, focused tests, factual Book One
  synchronization, canonical verification, Theory Alignment Review,
  archive/reset, and stop at Founder diff review.

#### Exact transaction contract

**Common entry gate and command shape**

1. The command accepts a caller-injected disposable path/connection, one typed
   Evidence operation, the exact expected current source revision, the exact
   expected artifact revision when applicable, canonical action time,
   deterministic fixture IDs/tokens, and an expected pre/post operation
   manifest. It accepts no SQL string or generic bundle replacement.
2. Before `BEGIN IMMEDIATE`, require exact schema v5, the fixed schema-object
   digest, one immutable committed migration receipt, the exact
   `database_contract` with v5 authoritative/v4 projection enabled and
   lifecycle/export product activation still disabled, an empty compatibility
   guard, Slice 4A source projection parity, current-content invariants,
   foreign keys, and integrity. V4, malformed, inconsistent, or newer input
   refuses read-only.
3. Inside `BEGIN IMMEDIATE`, re-read the source head and require it to be
   active with `current_revision_id = expected_source_revision_id`, present
   source content, and an exact matching v4 Experience row. Every Evidence
   revision dependency names this exact source revision. A stale or one-sided
   state writes nothing.
4. New artifact revision content uses `canonical-json-v1` over immutable
   Evidence content fields: stable artifact/source IDs, exact text,
   `originalText`, allowed Evidence kind, `userEditable`, and original creation
   time. Review state, current eligibility, revision action time, and
   provenance are normalized separately and are not rewritten into v5 content
   to simulate review.
5. The v4 `persisted_artifacts.payload` is an explicitly lossy current-state
   projection shaped as the existing `EvidenceCandidate`: current text/kind,
   `originalText`, current candidate/confirmed status, current timestamps, and
   the original generated-artifact provenance needed by the schema-v4 UI. V5
   revision authorship/provenance/review remain authoritative; lineage is never
   reconstructed from this projection.

**Rows written by candidate creation**

6. `artifact_revisions`: insert revision 1 with no predecessor,
   `revision_reason=created`, `serialization_version=canonical-json-v1`, exact
   content digest, and authorship exactly `ai` or `local_mock`.
7. `artifact_revision_content`: insert the exact canonical content bytes and
   byte length. `provenance_records` stores/deduplicates the complete generated
   provenance fingerprint; `artifact_revision_provenance` links it as
   `role=content`.
8. `artifact_dependencies`: insert exactly one
   `derived_from_experience` edge from the new Evidence revision to the exact
   current source Experience revision.
9. `artifact_lifecycle_events`: append one `created` event with actor `system`,
   the exact generated revision, and a bounded Evidence-candidate reason.
   `artifact_review_events` receives no row because generation is not review.
10. `artifact_heads`: insert one active head pointing to the revision with
    `review_state=pending`, `eligibility_state=ineligible`, and reason
    `pending_explicit_review`.
11. `persisted_artifacts`: insert one `artifact_kind=evidence` row whose
    payload has `status=candidate`. Existing artifact identity, source
    mismatch, unsupported kind, incomplete AI/local-mock provenance, or ID
    collision aborts without mutation.

**Rows written by still-pending user correction**

12. Re-read an active Evidence head whose exact current revision equals the
    expected artifact revision and whose review state is `pending`. The prior
    revision/content/provenance/dependency must be internally consistent.
13. `artifact_revisions`: append `revision_number + 1`,
    `predecessor_revision_id=old current`,
    `authorship=user`, `revision_reason=corrected`, and the new canonical
    content digest. `artifact_revision_content` stores the corrected content.
    A distinct canonical user provenance record is linked to the new revision
    as `role=content`.
14. The original AI/local-mock revision, content, digest,
    `artifact_revision_provenance`, and source dependency remain byte- and
    row-immutable. The corrected revision records `originalText` as the exact
    first generated text; this compatibility aid does not replace the retained
    prior revision.
15. `artifact_dependencies`: append a new `derived_from_experience` edge for
    the new revision to the exact current source revision. Old dependencies are
    not rebound or deleted.
16. `artifact_lifecycle_events`: append `corrected` with subject=new and
    related=old, plus `superseded` with subject=old and related=new. Both name
    the explicit user action and exact time.
17. `artifact_heads`: advance only to the new revision, keep lifecycle active,
    set review back to `pending`, and set eligibility to ineligible with
    `correction_requires_confirmation`. No review event is added and no prior
    confirmation can carry forward.
18. `persisted_artifacts`: update the one v4 Evidence projection to corrected
    text with `status=candidate` and the action timestamp. No other artifact
    projection changes.

**Rows written by explicit confirmation**

19. Require the exact current active Evidence revision and a pending head.
    Insert exactly one `artifact_review_events` row whose
    `subject_revision_id` is that expected current revision,
    `decision=confirmed`, `actor=user`,
    `event_origin=explicit_user_action`,
    and timestamp quality is `exact_action_time`.
20. Confirmation writes no `artifact_revisions`,
    `artifact_revision_content`, `artifact_revision_provenance`,
    `artifact_dependencies`, or lifecycle event. It cannot convert authorship,
    Evidence kind, content, or provenance into objective truth.
21. Update `artifact_heads` to `review_state=confirmed`,
    active/eligible with reason `explicitly_confirmed_current_revision`, then
    update only the matching v4 projection status/timestamp to `confirmed`.

**Rows written by explicit rejection and synchronous purge**

22. Require the exact current active pending Evidence revision. Insert one
    exact `artifact_review_events` row with `decision=rejected` and explicit
    user-action metadata. Rejection does not rewrite the content as
    “rejected.”
23. Clear `artifact_heads.current_revision_id`, set
    `review_state=rejected`, `lifecycle_state=content_purged`,
    and eligibility to ineligible with reason
    `explicitly_rejected_content_purged`.
24. Delete only the rejected revision's `artifact_revision_content`, append one
    `artifact_lifecycle_events` `content_purged` event, and insert one
    content-free `content_tombstones` row with the retained digest and
    `reason_code=rejected_content_purged`. Revision metadata, exact review
    event, generated provenance link, and exact source dependency remain
    immutable content-free evidence until parent deletion.
25. Delete the matching `persisted_artifacts` row. V4 therefore contains no
    rejected current Evidence, while v5 retains content-free rejection history;
    neither projection retains rejected text.

**Exact review and stale/conflict refusal**

26. A unique `(artifact_id, subject_revision_id)` review event plus the
    expected pending head makes one exact revision reviewable once. Duplicate
    same-decision review, conflicting confirm/reject, a head already confirmed
    or rejected, stale expected source/artifact revision, missing content,
    cross-source identity, non-Evidence kind, or malformed payload/provenance
    fails closed before any durable write.
27. Confirmation/rejection never targets “the artifact in general.” The named
    expected revision must still be the current pending revision inside the
    transaction. A corrected revision is a new pending subject and therefore
    needs its own later explicit confirmation.

**Dependent behavior**

28. Outgoing `derived_from_experience` edges are required and retained.
    Confirmation changes no dependency.
29. A valid Reflection, Pattern, or Historical Question cannot depend on a
    pending Evidence revision under the current eligibility contracts.
    Therefore any inbound ordinary or historical dependency found during the
    authorized pending correction/rejection commands is contradictory evidence
    and the complete command refuses without mutation.
30. ADR-0009/Decision 11A remains the approved cascade for a future separately
    authorized correction, rejection, deletion, or eligibility loss of
    legitimately included confirmed Evidence. Slice 4B-1 does not implement
    confirmed-Evidence mutation or Phase 3B v5 writes, so it does not exercise
    that cascade. Ordinary Reflection/Pattern invalidation requires Decision
    10B parity and likewise remains fail-closed for this slice.

**Guard, reconciliation, rollback, and restart**

31. After all expected-state checks, insert exactly one high-entropy-shaped
    guard token inside the transaction. Write immutable v5 facts, then head
    projection, then v4 projection. Reconcile exact expected authority,
    projection, source/artifact dependency, event, provenance, content/tombstone
    state; remove the exact token; require guard count zero; then check current
    content, foreign keys, and integrity before commit.
32. Inject failure after every meaningful guard, provenance, revision,
    content, dependency, lifecycle, review, head, tombstone, purge, projection,
    reconciliation, and guard-removal boundary. Every pre-commit failure must
    reopen with the exact logical pre-manifest and empty guard.
33. Reuse Slice 4A's conservative commit adapter and close writable
    connections before classification. A generic commit error is
    outcome-unknown. Read-only reopen classifies exact post-manifest as
    committed, exact pre-manifest as failed-unchanged, and anything else as
    `recovery_required`; no retry, replay, rollback, repair, cleanup, or
    candidate selection follows.
34. Read-only post-commit reconciliation proves exact v5/v4 projection
    semantics, immutable migration receipt, database contract, fixed schema
    digest, operation manifest, review/lifecycle/provenance/dependency facts,
    content/tombstone state, guard emptiness, current-content invariants,
    foreign keys, and integrity. Migration receipt manifests remain immutable
    cutover evidence rather than live-state manifests.
35. This slice creates no durable per-write receipt. Immediate close/reopen and
    injected ambiguity prove fixture-local classification only; process crash,
    production restart recovery, and real-user safety remain unproved.

**Reusable versus disposable**

36. Reusable private code may include typed Evidence inputs/outcomes,
    canonical content/provenance/digest/ID functions, expected-revision
    validation, transaction ordering, review/lifecycle rules, guard discipline,
    projection assembler, fail-closed validator, and read-only reconciler.
37. Disposable evidence includes caller-injected paths/connections, exact-v5
    synthetic fixtures, deterministic clocks/IDs/tokens, failure and commit
    adapters, expected manifests, and every execution. The module stays
    private/unregistered and unreachable from Tauri, renderer, startup, UI,
    app-data, real user databases, providers, and `ContextPacket`.

### Option B — Contracts, fixtures, and evaluation only

Freeze the typed inputs/outcomes, canonical Evidence representation, expected
manifests, and test cases, but execute no Evidence mutation.

### Option C — Defer Evidence parity

Make no Slice 4B-1 changes and separately evaluate structured retrieval R2.

## Benefits

- **A:** Exposes the smallest real transaction risks in authorship,
  exact-revision review, rejected-content purge, dependency checks, and v5/v4
  parity while remaining disposable and unreachable from production.
- **B:** Lowest mutation risk, but repeats contract-only work and cannot prove
  purge/rollback/projection ordering.
- **C:** Prioritizes another product-facing retrieval slice, but leaves the
  accepted Evidence lifecycle foundation without executable write evidence.

## Risks

- **A:** Review/purge ordering is privacy-sensitive; a defect could retain
  content or erase history. Strict single-transaction ordering, tombstone
  checks, and disposable-only execution are mandatory.
- **A:** V4 is lossy; treating its embedded provenance/status as authority
  would collapse revision authorship and review history.
- **A:** Dependent invalidation is intentionally incomplete. Any attempt to
  delete, retain, or rebind ordinary dependents would exceed this scope.
- **B:** Paper confidence may hide SQL constraint, guard, and rollback defects.
- **C:** Defers infrastructure needed for post-review correction/deletion and
  complete lifecycle inspection/export.

For every option, passing tests is not production authorization.

## Reversibility

- **A:** Code remains private/unregistered and executes only against disposable
  fixtures. It can be removed without touching user data; no schema or down
  migration is introduced.
- **B:** Contract/fixture changes are removable.
- **C:** No repository behavior changes.

## Data And Privacy Impact

Option A touches synthetic/disposable data only. Pending and superseded content
remain local for inspectable revision history. Rejected content is purged
synchronously; only content-free digest, provenance, dependency, review,
lifecycle, and tombstone facts remain until parent deletion. No provider call,
consent, telemetry, clipboard, export, backup, retention scheduler, or real
user path is added.

## Orchestrator Recommendation

**Option A**, exactly as bounded above. It provides executable evidence for
the first ordinary artifact lifecycle without activating v5, expanding AI
authority, or guessing dependent behavior.

## Default Safe Action

Remain at `human_decision_required`. Make no implementation changes.

## Blocked Files Or Phases

Engineering Plan, Rust Evidence module/tests, implementation-status
documentation, verification, Theory Review, archive, and all Git/deployment
actions remain blocked pending the exact Founder response.

## Exact Founder Response Needed

To authorize Option A, reply exactly:

> I resolve PHASE3C-SLICE4B1-001 by selecting Option A. I authorize Phase 3C
> Slice 4B-1 only: exact-v5 disposable fixtures produced through the promoted
> migration core; one private, unregistered, path/connection-injected Rust
> Evidence mutation boundary implementing the exact Founder decision-package
> contract for AI/local-mock candidate creation, still-pending user correction,
> exact-current-revision confirmation, and exact-current-revision rejection;
> immutable per-revision AI, local_mock, and user authorship/provenance;
> corrected and superseded lifecycle events; explicit exact-revision review
> events; synchronous rejected-content purge with content-free tombstone and
> v4 projection omission; exact current Experience revision dependencies;
> fail-closed refusal for stale, duplicate, conflicting, malformed, unsupported,
> or any inbound-dependent state outside this slice; synchronized v5 authority
> and guarded v4 projection; deterministic injected fixture clocks,
> identifiers, guard tokens, failure points, commit outcomes, and manifests;
> read-only post-transaction reconciliation; focused synthetic/disposable
> tests; factual Book One documentation; canonical verification; Theory
> Alignment Review; archive/reset; and stop at Founder diff review. I accept
> that this does not implement confirmed-Evidence correction/deletion,
> ordinary dependent invalidation, Phase 3B v5 cascade writes, production
> restart recovery, real-user safety, or production schema-v5 readiness. I do
> not authorize production SCHEMA_VERSION 5, production user_version 5, real
> user databases or app-data, startup/Tauri/renderer/UI activation,
> whole-bundle replacement, Reflection, Pattern, Context Recovery, Historical
> Question or Phase 3B write parity, retention, export v2, backup/restore,
> automatic retry/replay/rollback/repair/cleanup/candidate selection, provider
> or ContextPacket changes, Phase 4, Harness expansion, staging, commit, push,
> merge, PR, deployment, or release.

The Founder may instead select Option B or C and state its exact authorized
scope. Silence never resolves this decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4B1-001 by selecting Option A. I authorize Phase 3C Slice 4B-1 only: exact-v5 disposable fixtures produced through the promoted migration core; one private, unregistered, path/connection-injected Rust Evidence mutation boundary implementing the exact Founder decision-package contract for AI/local-mock candidate creation, still-pending user correction, exact-current-revision confirmation, and exact-current-revision rejection; immutable per-revision AI, local_mock, and user authorship/provenance; corrected and superseded lifecycle events; explicit exact-revision review events; synchronous rejected-content purge with content-free tombstone and v4 projection omission; exact current Experience revision dependencies; fail-closed refusal for stale, duplicate, conflicting, malformed, unsupported, or any inbound-dependent state outside this slice; synchronized v5 authority and guarded v4 projection; deterministic injected fixture clocks, identifiers, guard tokens, failure points, commit outcomes, and manifests; read-only post-transaction reconciliation; focused synthetic/disposable tests; factual Book One documentation; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that this does not implement confirmed-Evidence correction/deletion, ordinary dependent invalidation, Phase 3B v5 cascade writes, production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, real user databases or app-data, startup/Tauri/renderer/UI activation, whole-bundle replacement, Reflection, Pattern, Context Recovery, Historical Question or Phase 3B write parity, retention, export v2, backup/restore, automatic retry/replay/rollback/repair/cleanup/candidate selection, provider or ContextPacket changes, Phase 4, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Slice 4B-1 Option A exactly as stated by the Founder: private unregistered disposable exact-v5 Evidence candidate, pending correction, exact-revision confirm/reject parity with immutable authorship/provenance, synchronous rejection purge/tombstone, exact source dependency, guarded v4 projection, fail-closed unauthorized dependents, deterministic tests, verification, Theory Review, archive/reset, and Founder diff review only; all listed production, runtime, later-slice, Git, deployment and release actions remain unauthorized.

## Decided At And Evidence Reference

- Decided at: 2026-07-29T18:46:33.858Z
- Evidence reference: PHASE3C-SLICE4B1-001 founder response in current Codex task on 2026-07-30

## Resume Phase

product_review
