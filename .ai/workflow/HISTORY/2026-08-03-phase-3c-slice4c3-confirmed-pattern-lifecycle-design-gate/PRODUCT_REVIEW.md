# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-03-phase-3c-slice4c3-confirmed-pattern-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `e385ed7f465376dae734afacc08f49ac5532b980`
- Working-tree digest reviewed: `3ce814928011130f730ea9c18dc1ef7581c28f3292d399df455520172ad202b0`
- Created at: 2026-08-02T16:15:00.000Z
- Updated at: 2026-08-02T16:30:00.000Z

## Mission Interpretation

Design, but do not implement, the smallest private disposable lifecycle
boundary that lets a user correct or delete one exact-current confirmed
single-Experience Pattern hypothesis. The design must preserve exact authorship,
provenance, source dependencies, compatibility projection, and fail-closed
consequences without inventing a future dependent framework.

## Problem Statement

The promoted Slice 4B-3 boundary can create, confirm, and reject a disposable
schema-v5 Pattern candidate. It deliberately refuses inbound-dependent state and
does not correct or delete a confirmed Pattern. ADR-0011 and architecture/12
require post-review Pattern correction/deletion, but implementation authority
has not been granted. Production schema and startup support remain v4.

## User Value

A user can eventually revise a hypothesis without rewriting its AI origin, and
can delete it without leaving reusable hidden text. This makes Life OS a mirror
whose hypotheses remain revisable and user-owned, not an oracle whose past
wording becomes permanent truth.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: **We Build Mirrors, Not Oracles**; user authority
  and refusal boundaries remain primary.
- `docs/05_Identity.md` and `docs/07_Awareness.md`: a Pattern is a revisable
  hypothesis, not Evidence, Identity, diagnosis, or final meaning.
- `docs/06_Memory.md`: durable context requires provenance, correction,
  deletion, and user control.
- `docs/Reflection.md`: meaning is co-created but user-owned; confirmation is
  useful-for-reflection, not objective truth.
- `docs/09_AI.md`, `docs/10_Privacy.md`, and `docs/appendix/Harness.md`:
  authorship, uncertainty, exact inputs, refusal, and local-first boundaries
  stay visible and testable.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md` requires
  durable reviewed artifacts to retain source, authorship, and provenance.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`
  excludes Pattern from historical input and permits no Pattern-based
  Historical Question dependency.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`
  accepts append-only correction, exact-revision dependencies, content purge,
  content-free tombstones, and no silent dependent rebinding. It does not by
  itself authorize this implementation or production schema v5.

## Current Implementation Context

**Implemented, verified, Founder-reviewed, and promoted:** the private
disposable Pattern create/confirm/reject boundary; confirmed-Evidence lifecycle
consequences; Phase 3B Historical Question creation parity; and the fixed
schema-v5 contract. Slice 4C-2 was promoted through feature commit
`72aebec594813f33759a2aa9ba77ccee54d7b017` and non-fast-forward merge commit
`e385ed7f465376dae734afacc08f49ac5532b980`.

**Founder-approved design:** architecture/12 and ADR-0011 require confirmed
single-Experience Pattern correction/deletion with immutable revisions,
reconfirmation, purge, and exact consequences.

**Proposed only:** Slice 4C-3 Option A below.

**Not authorized:** implementation, production schema/user_version 5,
real-user migration, runtime activation, Phase 4, and Git promotion. Production
`SCHEMA_VERSION` and startup maximum are 4.

### Pattern source dependency inventory

The promoted single-Experience Pattern has exactly these outgoing source edges:

| Source | Cardinality | Exact rule |
| --- | --- | --- |
| Experience revision | exactly 1 | `derived_from_experience`; exact current active source revision |
| Evidence revision | 1 or more | `uses_evidence`; exact current confirmed, eligible, same-source revisions |
| Reflection revision | 0 or more | `uses_reflection_response`; exact current answered, eligible, same-source revisions actually used; complete Reflection -> Evidence -> Experience verifier chain applies |

The immutable Pattern provenance `sourceArtifactIds` must equal the declared
Evidence-plus-Reflection artifact set after identifier well-formedness and
array uniqueness are checked. Another Pattern and Context Recovery are not
authorized Pattern sources. Source selection changes are not correction and
remain a future separately governed capability.

### Pattern inbound dependency audit

Repository authority confirms Pattern is presently a leaf hypothesis:

- `artifact_dependencies.relationship_type` contains no `uses_pattern` edge;
- no promoted writer creates a normalized edge whose source is a Pattern;
- Context Recovery depends on one Experience and, for an answer, its immutable
  prompt through `answers_prompt`; it may not depend on Pattern;
- ADR-0009 permits only Experience, confirmed Evidence, and eligible saved
  user-authored Reflection as historical inputs; Pattern is explicitly excluded;
- the current Context Packet accepts existing Pattern data only to detect
  orphaned source references; Pattern is not emitted as provider input or used
  to create a dependent artifact.

Therefore every normalized or historical inbound edge to a Pattern is
malformed or future, not current product behavior. Option A requires a
zero-inbound invariant. Such an edge fails closed; it does not grant authority
for a generic invalidation framework.

### Proposed correction state transition

`confirmed / active / eligible @ rN` becomes
`pending / active / ineligible @ rN+1` in one transaction:

1. require exact current Experience `sN` and Pattern `rN`;
2. prove full v4/v5 projection parity, exact outgoing dependency set, complete
   promoted verifier chain, unique durable IDs, and zero inbound edges;
3. require the caller to supply the complete source dependency set and prove it
   is exactly equal to `rN`; no edge is added, removed, inferred, or rebound;
4. append immutable user-authored `rN+1` with predecessor `rN`, new canonical
   content, and user content provenance; retain `rN` AI/local-mock provenance;
5. copy the validated outgoing dependency set to `rN+1` as exact revision
   references, then append `corrected` and `superseded` lifecycle facts;
6. set the head to `rN+1`, pending and ineligible; write no confirmation event;
7. atomically synchronize the guarded schema-v4 pending/candidate projection;
8. keep `rN` visible as superseded and context-ineligible under Decision 9A.

Explicit later reconfirmation must target `rN+1` exactly. Any source dependency
drift requires a new capability, not silent correction behavior. ADR-0011 is
consistent with exact-set preservation, but this implementation remains a
consequential withheld authority; Option A explicitly asks the Founder to
authorize it.

### Proposed deletion state transition

`confirmed|pending / active @ rN` becomes `deleted / ineligible / no current
revision` in one explicit user action:

1. require the exact current Pattern revision and prove parity, outgoing source
   dependencies, and zero inbound edges;
2. append explicit user `deleted` lifecycle evidence;
3. purge every retained Pattern revision content row, including superseded
   content, and append required `content_purged` facts;
4. retain only the existing immutable revision/provenance/dependency metadata
   authorized by ADR-0011; none is reusable Pattern text;
5. clear the current head, remove the guarded v4 projection, and create one
   minimal content-free artifact tombstone containing no text or raw content
   digest;
6. leave unrelated artifacts unchanged and permit no retrieval, generation,
   profiling, Harness learning, provider use, or dependency rebinding.

Deletion is distinct from rejection: rejection applies to a pending candidate
and preserves content-free rejection history; deletion is an explicit removal
of an existing artifact and purges all retained revisions.

### Transaction, reconciliation, and ambiguous-COMMIT model

One existing private Pattern module, operating only on exact-v5 disposable
fixtures, would use one `BEGIN IMMEDIATE` transaction. It would prove the fixed
contract/receipt, disabled activation, empty guard, exact current heads,
complete dependencies, v4/v5 parity, and zero inbound closure before mutation;
write one exact operation guard; perform correction or deletion; reconcile
heads, revisions, content/provenance, lifecycle facts, outgoing dependencies,
v4 projection, tombstone, `foreign_key_check`, `integrity_check`, and guard
emptiness; then commit.

Every meaningful revision, provenance, content, dependency, lifecycle, head,
projection, purge, tombstone, reconciliation, guard, and commit boundary needs
deterministic failure injection. A generic COMMIT error is classified only by
closing writable access and comparing a read-only exact pre-state or exact
post-state manifest. A third durable state returns `recovery_required`; there
is no retry, replay, rollback, repair, cleanup, rebinding, or candidate choice.

## In Scope

- Minimal factual architecture/13 correction for Slice 4C-2 promotion.
- Founder decision package `PHASE3C-SLICE4C3-001`.
- If separately authorized: correction/deletion only in the existing private,
  unregistered, path/connection-injected disposable Pattern module.
- Exact-set preservation, zero-inbound refusal, rollback/restart evidence,
  focused tests, factual documentation, canonical verification, Clippy, Theory
  Alignment Review, workflow archive/reset, and stop at Founder diff review.

### Exact prospective implementation allowlist

Only these product files may change after Option A authorization:

1. `src-tauri/src/schema_v5_pattern_write.rs`
2. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

The existing workflow may update its standard active artifacts under
`.ai/workflow/` and create the standard archive at
`.ai/workflow/HISTORY/2026-08-03-phase-3c-slice4c3-confirmed-pattern-lifecycle-design-gate/`.
No DDL, Cargo, production SQLite, Tauri registration, TypeScript, renderer, UI,
provider, ContextPacket, ADR, Book Zero, or Harness-contract file is allowed.
If implementation evidence requires any additional product path, stop for a
new Founder decision before editing it.

## Out Of Scope

Production schema/user_version 5; real user data/app-data; migration/fresh-v5
activation; Tauri/renderer/UI/startup; Pattern generation or source reselection;
Context Recovery or Historical Question dependence on Pattern; generic future
dependents; provider/ContextPacket/consent changes; export v2; retention;
production backup/restore/recovery; Phase 4; identity/sensitive inference;
Harness expansion; staging, commit, push, merge, PR, deployment, or release.

## Product Constraints

A corrected Pattern remains a hypothesis. User editing changes authorship only
for the successor revision and does not make it Evidence or Identity. Prior
authorship is immutable, confirmation never carries forward, and source sets do
not change through a correction.

## Evidence And Provenance Constraints

Original AI/local-mock content provenance stays on `rN`; user provenance stays
on `rN+1`. Arrays must be individually valid and unique before set equality.
The promoted Pattern, Reflection, Evidence, and Experience verifiers must be
reused rather than partially copied. Dependents are never rebound.

## Historical Context Constraints

Pattern remains categorically excluded from ADR-0009 eligibility. Any
`historical_artifact_dependencies` edge targeting Pattern is malformed and
causes fail-closed unchanged state. No historical artifact is cascaded merely
because the Pattern itself is changed, because no such legal relation exists.

## Consent Constraints

No consent or provider use occurs. Existing Phase 3B consent, packet,
transmission, retention, and deletion semantics are unchanged.

## AI-Role Constraints

There is no generation or semantic validation of arbitrary Pattern text in
this lifecycle slice. The system records a user's revised hypothesis without
endorsing it. Diagnosis, recurrence, contradiction, change-over-time claims,
sensitive inference, and identity finalization remain prohibited.

## Privacy Constraints

Deletion removes every retained Pattern content payload and prevents future
use. Only policy-authorized content-free metadata remains. The private module
cannot reach production or real user data.

## User-Agency Constraints

Correction and deletion require explicit user action and exact revision
expectations. Silence does nothing. Stale or contradictory state fails closed;
the system never chooses new sources or repairs dependencies for the user.

## Acceptance Criteria

| Case | Required result |
| --- | --- |
| Successful confirmed correction | Append user-authored successor; predecessor unchanged; head pending/ineligible; v4 candidate projection exact |
| Reconfirmation | Only exact corrected revision can be confirmed; predecessor confirmation does not carry |
| Prior content | Superseded AI/local-mock revision remains visible and context-ineligible until deletion |
| Stale or duplicate correction | Fail closed; no partial revision/event/dependency/projection write |
| Pending/rejected/deleted/invalidated correction | Refuse unchanged |
| Experience mismatch | Refuse exact source revision drift |
| Evidence/Reflection drift | Complete promoted verifier chain refuses stale or ineligible source |
| Malformed/duplicate/cross-source source set | Refuse before set equality or mutation |
| Source selection change | Refuse; no add/remove/rebind |
| Unsupported ordinary inbound edge | Refuse through zero-inbound invariant |
| Historical Question edge to Pattern | Refuse as malformed ADR-0009 state |
| Successful deletion | Explicit deletion fact; all Pattern content purged; head cleared; v4 projection removed |
| Tombstone | Minimal and content-free; no reusable text or raw content digest |
| Stale or duplicate deletion | Refuse unchanged |
| Every write boundary | Exact logical rollback before commit |
| Ambiguous COMMIT exact pre-state | Classify definite non-commit without replay |
| Ambiguous COMMIT exact post-state | Classify committed without rewriting |
| Ambiguous COMMIT third state | `recovery_required`; no autonomous action |
| Reconciliation | Exact v4/v5 parity, current-content rules, guard emptiness, foreign-key and integrity checks pass |
| Scope | Production schema/startup maximum stay 4; no provider, runtime, real-data, Phase 4, identity or sensitive inference behavior |

## Risks

- Copying source edges without explicit equality could silently change a
  hypothesis's basis. Option A requires caller-supplied exact-set equality.
- A speculative generic invalidation framework would create ontology and Phase
  4 authority not present in the product. Zero-inbound refusal avoids this.
- Deleting only current content would leave superseded text reusable. The
  transaction must purge all retained Pattern revision content.
- Dual v4/v5 state can drift if not reconciled in one transaction.
- Disposable tests can establish deterministic contract evidence but not
  production restart recovery, real-user safety, or production-v5 readiness.

## Open Questions

None for the authorized Slice 4C-3 boundary. Any additional product file,
inbound Pattern consumer, source reselection, or production/runtime activation
requires a new Founder decision.

## Human Decision Required

No. The Founder resolved `PHASE3C-SLICE4C3-001` as Option A. Promotion remains
a later separate Founder gate.

## Recommendation

Implement only authorized **Option A**. Repository authority proves Pattern has
exact outgoing Experience/Evidence/Reflection sources but no legal inbound
consumer. Preserve the explicitly supplied exact source set and fail closed on
every inbound edge. Do not implement a generic future-dependent framework.

## Review Status

`approved_with_conditions`
