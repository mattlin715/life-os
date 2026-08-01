# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-02-phase-3c-slice4c1-confirmed-evidence-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: adb269dc68d6ec3917819e6ea8c18d84cf89e902
- Working-tree digest reviewed: 23a79e01afd55e2569eda5591a2d2c905ff20c9ca931986bf3fc9e9e54ac73c7
- Created at: 2026-08-01T17:46:10.475Z
- Updated at: 2026-08-01T18:16:24.868Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

This sprint is a design gate for the first complete confirmed-Evidence lifecycle
boundary. It must determine whether correction and deletion can preserve every
accepted ordinary-dependent and ADR-0009 consequence in one private,
unregistered, disposable-only transaction. It may synchronize factual promotion
evidence, but it must not implement or infer Founder authorization.

## Problem Statement

The promoted schema-v5 Evidence writer supports pending candidate correction,
confirmation, and rejection, but refuses inbound dependents and does not support
correction or deletion of confirmed Evidence. ADR-0011 already fixes the missing
semantics: correction creates a pending user revision without rebinding;
ordinary exact-revision dependents become visibly invalidated; Historical
Question generated content and its successful packet snapshot are deleted under
ADR-0009; deletion purges content and leaves only a content-free tombstone.

A writer that implements only the Evidence head change would be unsafe. It
could leave Reflection or Pattern revisions eligible against a superseded or
deleted Evidence revision, or leave Historical Question actual-used content
after its source was corrected or deleted.

## User Value

- The user can eventually correct an AI-originated hypothesis without the
  correction silently becoming confirmed Evidence.
- The user can delete confirmed Evidence without hidden text surviving in the
  lifecycle record.
- Previously generated ordinary artifacts remain honestly inspectable as
  invalidated rather than being rewritten to fit the new Evidence.
- Historical provider-use artifacts obey the stricter ADR-0009 deletion
  lifecycle, preserving control over actual-used content.
- Exact provenance explains which old revision invalidated which dependent.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: constitutional authority and
  **We Build Mirrors, Not Oracles**.
- `docs/03_Principles.md`: Evidence before Conclusion, Reflection before Answer,
  visible uncertainty, user control, and reversibility.
- `docs/06_Memory.md`: memory remains provenance-preserving, revisable,
  rejectable, and deletable; no silent identity accumulation.
- `docs/Reflection.md`: Reflection is an invitation to user interpretation, not
  an authoritative conclusion.
- `docs/09_AI.md`: AI output remains bounded, uncertain, and reviewable.
- `docs/10_Privacy.md`: local-first control, deletion, and explicit historical
  transmission boundaries.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`:
  reviewed artifacts require provenance and distinct AI/user authorship.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`:
  source correction, rejection, deletion, or eligibility loss invalidates the
  generated Historical Question actual-use chain; consent is not reusable.
- `docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md`:
  cross-time interpretation remains Founder-gated and outside this slice.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`:
  Decisions 6B, 7B, 9A, 10B, 11A, and 12A are accepted and are not reopened.

## Current Implementation Context

### Implemented and promoted

- Production remains schema v4 with typed mutation commands and ADR-0009
  Historical Question persistence/deletion behavior.
- The private schema-v5 migration core normalizes migrated schema-v4
  Historical Questions and records one-to-one lifecycle links.
- Private disposable schema-v5 writers now cover Experience, pending Evidence,
  Reflection, single-Experience Pattern, and Context Recovery bounded writes.
- Slice 4B-4 was promoted through feature commit
  `05d623cbe1361e1974517c91c996f7fb7189f83d` and non-fast-forward merge
  `adb269dc68d6ec3917819e6ea8c18d84cf89e902`.

### Founder-approved but not production-authorized

- Architecture/13 defines schema-v5 lifecycle, dependency, projection, and
  cutover direction. It does not authorize production schema v5 or runtime
  activation.

### Not implemented

- Confirmed-Evidence correction or deletion in schema v5.
- Ordinary dependent invalidation for Reflection and Pattern.
- Phase 3B schema-v5 runtime creation parity after migration.
- Production schema-v5 migration, startup, UI, Tauri, or real-user activation.

## Exact Dependency Graph

The authoritative normalized graph is exact-revision based:

1. Evidence current revision -> exact current source Experience revision via
   `derived_from_experience`.
2. Reflection current revision -> exact current Experience revision and one or
   more exact confirmed Evidence revisions via `uses_evidence`; answered
   Reflection also points to its immutable prompt via `answers_prompt`.
3. Pattern current revision -> exact current Experience revision, one or more
   exact confirmed Evidence revisions via `uses_evidence`, and zero or more
   exact answered Reflection revisions via `uses_reflection_response`.
4. Migrated Historical Question current revision -> exact current Experience
   and exact packet items through normalized historical dependency edges, while
   existing schema-v4 tables remain authoritative for packet snapshot,
   consent/transmission references, and actual-use deletion.

Correction or deletion of Evidence revision `E0` therefore has two distinct
graphs:

- Ordinary graph: `E0 -> Reflection`, `E0 -> Pattern`, and transitively
  `E0 -> Reflection -> Pattern`.
- Historical graph: `E0 -> Historical Question packet item -> generated
  artifact / successful packet snapshot / actual-use provenance`.

No edge may be rebound from `E0` to corrected revision `E1`.

## Local-First Transaction Analysis

### Required preconditions

- Exact schema-v5 disposable database produced through the promoted migration
  core.
- Exact current active source Experience revision passes the complete promoted
  Experience verifier.
- Exact current Evidence revision is active, confirmed, eligible, and passes
  the complete promoted Evidence verifier.
- Every inbound dependency ID is well-formed and unique before set equality.
- Every normalized Historical Question link has exactly one coherent schema-v4
  counterpart and vice versa; missing, extra, contradictory, or unsupported
  representation fails closed.

### One transaction boundary

One `BEGIN IMMEDIATE` transaction must:

1. Revalidate exact Experience, Evidence, ordinary dependents, Historical
   Question links, projection rows, packet/provenance rows, and pre-manifests.
2. Enumerate the complete direct and transitive exact dependency closure before
   mutation; reject cycles, cross-source edges, unsupported kinds, duplicate
   durable IDs, and partial prior invalidation.
3. For correction, append immutable user-authored revision `E1` with predecessor
   `E0`, retain `E0` content and AI/local-mock provenance, append corrected and
   superseded lifecycle facts, and set the Evidence head to pending/ineligible.
4. For deletion, append an explicit deletion lifecycle fact, purge all Evidence
   revision content, clear the current head revision, mark the head
   deleted/ineligible, and append one minimal content-free artifact tombstone.
5. For each exact ordinary edge from `E0`, append an edge-specific invalidation
   fact. If that dependent revision is its current active head, mark the head
   invalidated/ineligible and omit its schema-v4 projection. Retain its content,
   revision history, and provenance. Propagate exact Reflection invalidation to
   current Pattern revisions that use that Reflection. Transition each head at
   most once while preserving distinct invalidating edges.
6. Delete each affected Historical Question through one combined schema-v4 and
   normalized schema-v5 lifecycle operation: generated content, successful
   packet snapshot, actual-used content/provenance, normalized head/revisions,
   and the lifecycle link. Preserve only audit metadata already allowed by
   ADR-0009; do not create a transport tombstone or alter the 30-day
   unsuccessful-attempt rule.
7. Update guarded schema-v4 projections: corrected Evidence becomes candidate;
   deleted Evidence disappears; invalidated ordinary dependents disappear from
   compatibility projection. Schema v5 remains authority.
8. Reconcile exact post-manifests, lifecycle facts, dependencies, content
   absence/presence, guards, projections, packet/audit rows,
   `foreign_key_check`, and `integrity_check` before commit.
9. Classify a generic COMMIT error as outcome-unknown unless definite
   non-commit is proved. Close writable access and reconcile read-only without
   retry, replay, repair, rebind, cleanup, or candidate selection.

All pre-commit failures must restore one coherent pre-state across authority,
projection, lifecycle, Historical Question packet snapshots, and audit rows.

## Ordinary Versus ADR-0009 Dependent Behavior

| Dependent | Required behavior | Content retained? | Rebound? |
| --- | --- | --- | --- |
| Reflection using `E0` | Exact invalidation event; current head ineligible; v4 projection omitted | Yes | No |
| Pattern using `E0` directly | Exact invalidation event; current head ineligible; v4 projection omitted | Yes | No |
| Pattern using invalidated Reflection | Transitive exact invalidation in same transaction | Yes | No |
| Historical Question using `E0` | Cascade-delete generated content, successful packet snapshot, actual-used content/provenance, normalized v5 artifact/link | No, except permitted minimal audit metadata | No |

Ordinary dependents are retained because ADR-0011 Decision 10B requires visible
invalidation. Historical Questions are deleted because ADR-0009 and Decision
11A establish a stricter actual-use deletion policy. Treating both categories
the same would violate one of those authorities.

## Phase 3B Prerequisite Determination

Phase 3B schema-v5 runtime creation parity is **not a prerequisite for a
coherent disposable Slice 4C-1** if the slice accepts only exact migrated-v5
fixtures and requires complete one-to-one normalized/schema-v4 Historical
Question representation. This allows the combined lifecycle transaction to be
independently verified without inventing a parallel Phase 3B writer.

It **is a prerequisite for production schema-v5 activation**. After a future
cutover, new Historical Questions would otherwise lack the normalized graph
required by this lifecycle operation. Slice 4C-1 must therefore remain private,
unregistered, disposable-only and must not be presented as production-ready or
as closing Phase 3B v5 parity.

## In Scope

- Minimal factual architecture/13 synchronization for promoted Slice 4B-4.
- Repository-grounded design of a private disposable confirmed-Evidence
  correction/deletion transaction with complete consequences.
- Exact ordinary and Historical Question dependency closure, invariants,
  failure classification, reconciliation, and test matrix.
- Founder decision `PHASE3C-SLICE4C1-001`.

## Out Of Scope

- Implementation before Founder authorization.
- Production schema/user_version 5, migration, fresh-v5 initialization,
  app-data, real users, startup, Tauri, renderer, UI, or deployment.
- Phase 3B v5 creation parity, provider/ContextPacket/consent changes, new
  transmission, Phase 4, sensitive inference, retention jobs, export v2, and
  backup/restore activation.
- Reflection/Pattern content rewriting, automatic regeneration,
  reconfirmation, rebinding, cleanup, retry, replay, or repair.
- Generic artifact framework or Engineering Harness expansion.

## Product Constraints

- Semantic completeness takes priority over smallest line count.
- Correction is user editing of a hypothesis; it returns to pending and does
  not become confirmed Evidence without a later explicit confirmation.
- Deletion, rejection, supersession, and invalidation are distinct facts.
- Compatibility projections are not authority.

## Evidence And Provenance Constraints

- Preserve old AI/local-mock revision, content, and provenance after correction.
- New correction revision has exact user authorship and predecessor linkage.
- Retained ordinary dependents expose the exact invalidating dependency.
- Deleted Evidence retains no content, only minimal content-free metadata and
  tombstone until parent Experience deletion.
- No invented timestamps, authorship, review, or dependency facts.

## Historical Context Constraints

- Historical Questions using corrected/deleted Evidence follow ADR-0009
  cascade deletion, not ordinary invalidation retention.
- No whole-history loading, source rehydration, summary, cross-time conclusion,
  provider call, or Phase 4 behavior.

## Consent Constraints

- Existing consent is not reused or broadened.
- No new consent event is created.
- Deletion preserves only the permitted minimal audit boundary and does not
  create a new transport tombstone.

## AI-Role Constraints

- No AI call or generated replacement occurs.
- The corrected revision remains a user-authored pending hypothesis.
- No diagnosis, profiling, identity finalization, sensitive inference, or
  authoritative interpretation is added.

## Privacy Constraints

- All proposed evidence is local and disposable.
- Deletion purges Evidence and Historical Question actual-used content according
  to their distinct accepted policies.
- Superseded ordinary content remains local and context-ineligible only because
  ADR-0011 Decision 9A explicitly retains revision history until user purge or
  artifact deletion.

## User-Agency Constraints

- Correction and deletion require explicit user intent at their eventual
  product boundary; this slice does not activate that boundary.
- No automatic reconfirmation, rewrite, regeneration, recovery, or rebinding.
- Retained invalidated artifacts remain eligible for later explicit user
  deletion, not silent cleanup.

## Acceptance Criteria

If Option B is later authorized, focused disposable tests must cover:

1. Confirmed Evidence correction with no dependents creates `E1`, retains `E0`,
   returns the head to pending/ineligible, and projects candidate state.
2. Direct Reflection and direct Pattern dependents receive exact invalidation
   facts, remain stored, become ineligible, and are omitted from v4 projection.
3. `Evidence -> Reflection -> Pattern` invalidates transitively in one
   transaction without rewriting or rebinding.
4. Multiple direct/transitive edges produce unique edge facts while each head
   transitions once.
5. Correction with a successful Historical Question deletes the complete
   schema-v4/v5 actual-use chain while preserving only permitted audit metadata.
6. Deletion purges all Evidence content, retains minimal content-free metadata
   and tombstone, removes v4 projection, and applies every dependent consequence.
7. Correction requires explicit later reconfirmation; old dependents never
   rebind and old superseded content is context-ineligible.
8. Stale Evidence revision or changed/deleted source Experience fails closed.
9. Malformed, duplicate, orphaned, cross-source, cyclic, unsupported, missing,
   extra, or contradictory dependency/link state fails closed.
10. Consistent already-invalidated exact facts are idempotently recognized;
    partial or contradictory invalidation fails closed.
11. Mixed ordinary and Historical Question dependents reconcile in one
    transaction.
12. Injected failure after every lifecycle, dependency, content purge,
    projection, link, packet, provenance, and audit boundary restores the exact
    logical pre-state.
13. Ambiguous COMMIT yields an outcome-unknown/recovery-required classification
    derived by read-only reconciliation, with no retry or autonomous action.
14. Exact pre/post manifests, schema-v4 projection parity,
    `foreign_key_check`, and `integrity_check` pass.
15. No provider call, consent reuse, Phase 4 output, identity inference, or
    sensitive inference occurs.
16. Canonical verification and Clippy with warnings denied pass; production
    `SCHEMA_VERSION` and startup maximum remain 4.

## Risks

- **Graph incompleteness:** missing one direct or transitive edge could leave a
  stale dependent eligible. Mitigation: complete promoted verifier reuse,
  precomputed closure, unique durable IDs, fail-closed parity checks.
- **Mixed retention semantics:** retaining Historical Question content or
  deleting ordinary revision history would violate accepted policy. Mitigation:
  separate graph classes and explicit record-level assertions.
- **Projection drift:** removing invalidated rows from v4 projection may hide
  them from current schema-v4 UI. This is acceptable for private disposable
  evidence only; future v5 UI must expose invalidated history before production
  activation.
- **Phase 3B parity gap:** new post-cutover Historical Questions would lack v5
  graph parity. Mitigation: keep production activation blocked until a separate
  Founder-approved Phase 3B v5 write slice is implemented and verified.
- **Transaction size/complexity:** the complete boundary is larger than a
  correction-only writer. A partial writer is semantically unsafe; bounded
  helper extraction is allowed only for already-proven identical verifier or
  reconciliation logic, not a generic framework.
- **Ambiguous COMMIT:** retry could duplicate facts or delete more data.
  Mitigation: conservative outcome classification and read-only reconciliation.

## Alternatives

- **A — Correction only, refuse every dependent state.** Smallest safe code
  surface for no-dependent fixtures, but does not address deletion or the real
  lifecycle gap and delays validation of accepted dependent semantics.
- **B — Correction and deletion with complete ordinary and ADR-0009
  consequences.** Recommended. Larger, but it is the smallest semantically
  complete lifecycle slice and can be proven on migrated disposable fixtures.
- **C — Phase 3B v5 parity first.** Safest prerequisite ordering for production
  cutover, but unnecessary for disposable migrated-fixture proof and delays the
  user-control lifecycle.
- **D — Contracts, fixtures, and evaluation only.** Lowest implementation risk,
  but adds no executable lifecycle evidence beyond this design package.
- **E — Move to production cutover planning.** Rejected recommendation: known
  lifecycle and Phase 3B parity gaps make cutover planning premature.

## Reversibility

Option B remains private, unregistered, and disposable. It can be removed
without touching production schema-v4 data or runtime behavior. Within each
test transaction, any definite pre-commit failure rolls back. An ambiguous
commit is not automatically reversed; it is classified by read-only evidence
and stopped. Production activation remains a separate Founder gate.

## Open Questions

none. The Founder selected Option B in `PHASE3C-SLICE4C1-001`. Migrated
one-to-one Historical Question parity is a hard input precondition, and
production activation remains blocked on later Phase 3B v5 creation parity.

## Human Decision Required

No. `PHASE3C-SLICE4C1-001` was explicitly resolved as Option B. Authority is
limited to the bounded private disposable implementation recorded there.

## Recommendation

The Founder selected **Option B**. Implement one private, unregistered, disposable-only
confirmed-Evidence correction/deletion boundary with complete direct,
transitive, projection, and ADR-0009 consequences in one transaction. Do not
implement Phase 3B v5 creation parity inside this slice, but explicitly retain
it as a blocker for production schema-v5 activation.

## Review Status

approved_with_conditions
