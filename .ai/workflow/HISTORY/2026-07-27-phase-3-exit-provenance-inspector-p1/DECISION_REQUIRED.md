# Decision Required

Status: resolved
- Sprint ID: 2026-07-27-phase-3-exit-provenance-inspector-p1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-26T16:20:08.464Z
- Updated at: 2026-07-26T16:20:08.464Z

## Decision ID

`PHASE3-PROVENANCE-INSPECTOR-P1-001`

## Sprint ID

`2026-07-27-phase-3-exit-provenance-inspector-p1`

## Decision Summary

Decide whether Life OS may implement one production UI, local, read-only,
schema-neutral actual-use inspector for already persisted Phase 3B Historical
Question artifacts.

The proposed Option A contract is exact:

1. The inspector is collapsed by default and opens only through explicit user
   action on one Historical Question artifact.
2. It consumes only the HistoricalQuestionArtifact and exact packet snapshot
   already loaded from local persistence.
3. It may display generated artifact ID/time; questions and exact cited source
   Experience IDs; packet ID, schema, versions, digest, destination, and
   bounded purpose; consent and transmission IDs; exact included Experience
   and artifact IDs/types; authorship, review state, revisions, relevance
   reasons; source snapshot references; and dependency relationships
   represented by packet items.
4. Exact outgoing content remains separately collapsed and needs a second
   explicit local reveal.
5. Reveal stays local, creates no call/consent/audit, copies nothing
   automatically, persists no UI state, and does not modify packet bytes.
6. Selected, consented, successful-transmission reference, and persisted
   generated artifact are visibly distinct.
7. Missing, malformed, unsupported, or contradictory packet data fails closed
   with a calm localized message. Nothing is invented and current source
   content is never used as fallback.
8. Deleted content is never rehydrated from another table or current
   Experience. Existing cascade/deletion behavior remains authoritative.
9. Deleting the Historical Question removes its inspector through the existing
   lifecycle.
10. English, Traditional Chinese, and Japanese expose equivalent terminology.
11. No persistence API, query, schema, migration, retention, provider,
    ContextPacket, consent-policy, or Phase 4 change is allowed.
12. P1 is partial and does not replace the future schema-v5 graph inspector.

## Why Automation Stopped

The repository already authorizes persistence and deletion of the successful
actual-use chain, but it does not yet authorize this new production disclosure
surface. Opening and revealing sensitive historical packet content requires an
explicit Founder product decision even though it remains local and read-only.
This prompt and successful baseline verification are not approval.

## Relevant Constitution Clauses

- **We Build Mirrors, Not Oracles:** show lineage without declaring meaning.
- **Human before AI:** the user controls open, reveal, and delete.
- **Evidence before Conclusion:** distinguish persisted evidence of process from
  the truth of generated text.
- **Privacy before Profit:** exact content is minimized and hidden by default.
- **Documentation Hierarchy:** Book One and code cannot silently broaden the
  accepted consent or inference relationship.

No constitutional edit is proposed.

## Relevant Primary Definitions

- `docs/03_Principles.md`: traceable evidence and revisable conclusions.
- `docs/06_Memory.md`: selective, provenance-aware, correctable, deletable
  longitudinal memory.
- `docs/Reflection.md`: user-owned meaning; reflection before answer.
- `docs/09_AI.md`: bounded, uncertain, non-authoritative AI role.
- `docs/10_Privacy.md`: local-first control, ongoing consent, deletion, and
  anti-resurrection.

## Relevant ADRs

- ADR-0007: reviewed AI artifacts require provenance and visible distinction
  from user evidence.
- ADR-0009: selection, consent, transmission, and actual use are distinct; the
  exact successful packet shares the generated artifact lifecycle.
- ADR-0010: Phase 4 interpretation remains separately governed and disabled.
- ADR-0011: a future complete provenance graph is approved as a design
  direction, but P1 remains a partial schema-v4 view.

## Available Options

### Option A — Production read-only P1 inspector

Authorize the exact twelve-part bounded contract above, focused tests, factual
Book One synchronization, canonical verification, Theory Alignment Review,
workflow archive/reset, and stop at Founder diff/manual UI review.

### Option B — Contracts and evaluation only

Authorize pure validation/view-model contracts and synthetic fixtures only.
Do not connect the inspector to production UI.

### Option C — Defer P1

Make no inspector changes. Separately evaluate structured retrieval R2 for
explicit emotion, relationship, and value-conflict controls.

## Benefits

- **A:** Gives immediate user-visible provenance and deletion comprehension
  using already-retained data; no new storage or provider surface.
- **B:** Reduces implementation risk and proves malformed-data semantics before
  UI activation.
- **C:** Keeps focus on broader structured retrieval and avoids adding a dense
  UI now.

## Risks

- **A:** Technical detail may overwhelm users or make provenance look like
  proof of truth; exact content reveal increases shoulder-surfing exposure.
- **B:** Produces no user value yet and prolongs the current inspectability gap.
- **C:** Leaves a persisted actual-use chain that users still cannot understand
  and delays a relatively reversible improvement.

## Reversibility

- **A:** High. Remove or feature-disable the read-only UI; retained schema-v4
  data and deletion behavior remain unchanged.
- **B:** High. Contracts and fixtures can remain unused or be removed before
  activation.
- **C:** Complete. No implementation occurs.

No option authorizes deletion-policy or schema rollback.

## Data And Privacy Impact

Option A reads only the artifact and packet snapshot already hydrated for the
current Experience. It performs no new query, write, retention, transmission,
consent, audit, or clipboard action. Exact outgoing content is hidden by
default. It may reveal only the retained packet bytes after explicit local
action. Deleted content is never reconstructed. Existing ADR-0009 cascade
deletion removes the artifact, packet, dependencies, and inspector.

Option B uses synthetic data only. Option C changes no data behavior.

## Orchestrator Recommendation

Option A. It converts an existing enforceable actual-use chain into
understandable user-facing provenance without increasing AI authority or data
retention. The implementation must use one pure fail-closed validator/view
model and must phrase transmission as a locally verified successful reference,
not a provider-side guarantee.

## Default Safe Action

Do not implement the inspector. Preserve only the factual documentation
corrections and remain blocked at this decision.

## Blocked Files Or Phases

Engineering Planning, production source, UI, i18n, tests, storage adapters,
schema, provider code, and all later phases are blocked until an exact Founder
resolution is recorded with `scripts/ai-workflow.mjs`.

## Exact Founder Response Needed

To authorize Option A, respond exactly:

```text
I resolve PHASE3-PROVENANCE-INSPECTOR-P1-001 by selecting Option A. I authorize Phase 3 Exit Provenance Inspector P1 only: implement one collapsed-by-default, explicit-open, local, read-only, schema-neutral actual-use inspector for one already persisted Phase 3B Historical Question artifact; consume only the already-loaded HistoricalQuestionArtifact and exact packet snapshot; display the generated artifact identity and time, questions and exact citations, packet identity/digest/schema/versions, provider/model, bounded purpose, consent and transmission references, included Experience and artifact IDs/types/revisions, authorship/review state, relevance reasons, source snapshot references, and packet-represented dependencies; distinguish selected, consented, successful-transmission reference, and persisted generated artifact; keep exact outgoing content separately collapsed behind a second explicit local reveal; create no provider call, consent, audit event, clipboard copy, UI-state persistence, packet mutation, source rehydration, or storage mutation; fail closed with calm English, Traditional Chinese, and Japanese messages for malformed, unsupported, incomplete, or contradictory provenance; preserve existing ADR-0009 source invalidation and Historical Question deletion cascades; add focused validator, view-model, UI, lifecycle, malformed-data, deletion, privacy, i18n, R1 regression, Phase 3B regression, and no-Phase-4 tests; synchronize factual Book One documentation; run canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff and manual UI review. I accept that P1 is partial and does not complete or replace the future schema-v5 provenance/dependency graph inspector. I do not authorize a new persistence API, SQL query, schema table, migration, retention change, provider or ContextPacket change, consent-policy change, deleted-content resurrection, whole-history loading, Phase 4 interpretation, sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.
```

For Option B or C, state the selected option and exact authorized scope with the
same decision ID.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3-PROVENANCE-INSPECTOR-P1-001 by selecting Option A. I authorize Phase 3 Exit Provenance Inspector P1 only: implement one collapsed-by-default, explicit-open, local, read-only, schema-neutral actual-use inspector for one already persisted Phase 3B Historical Question artifact; consume only the already-loaded HistoricalQuestionArtifact and exact packet snapshot; display the generated artifact identity and time, questions and exact citations, packet identity/digest/schema/versions, provider/model, bounded purpose, consent and transmission references, included Experience and artifact IDs/types/revisions, authorship/review state, relevance reasons, source snapshot references, and packet-represented dependencies; distinguish selected, consented, successful-transmission reference, and persisted generated artifact; keep exact outgoing content separately collapsed behind a second explicit local reveal; create no provider call, consent, audit event, clipboard copy, UI-state persistence, packet mutation, source rehydration, or storage mutation; fail closed with calm English, Traditional Chinese, and Japanese messages for malformed, unsupported, incomplete, or contradictory provenance; preserve existing ADR-0009 source invalidation and Historical Question deletion cascades; add focused validator, view-model, UI, lifecycle, malformed-data, deletion, privacy, i18n, R1 regression, Phase 3B regression, and no-Phase-4 tests; synchronize factual Book One documentation; run canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff and manual UI review. I accept that P1 is partial and does not complete or replace the future schema-v5 provenance/dependency graph inspector. I do not authorize a new persistence API, SQL query, schema table, migration, retention change, provider or ContextPacket change, consent-policy change, deleted-content resurrection, whole-history loading, Phase 4 interpretation, sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Phase 3 Exit Provenance Inspector P1 Option A only: one collapsed-by-default explicit-open local read-only schema-v4 Historical Question actual-use inspector over the already-loaded artifact and exact packet snapshot, separate exact-content reveal, fail-closed three-locale validation, focused regressions, factual Book One synchronization, canonical verification, Theory Alignment Review, archive/reset, and stop at Founder review; every stated non-scope remains excluded.

## Decided At And Evidence Reference

- Decided at: 2026-07-27T16:30:41.383Z
- Evidence reference: PHASE3-PROVENANCE-INSPECTOR-P1-001

## Resume Phase

product_review
