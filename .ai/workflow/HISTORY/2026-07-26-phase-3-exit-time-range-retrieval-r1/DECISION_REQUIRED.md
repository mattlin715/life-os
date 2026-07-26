# Decision Required

Status: resolved
- Sprint ID: 2026-07-26-phase-3-exit-time-range-retrieval-r1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-26T14:12:29Z
- Updated at: 2026-07-26T14:12:29Z

## Decision ID

`PHASE3-RETRIEVAL-R1-001`

## Sprint ID

`2026-07-26-phase-3-exit-time-range-retrieval-r1`

## Decision Summary

Decide whether to authorize one production UI/local-retrieval vertical slice
that adds an explicit user-controlled saved-date range to the existing
historical-context panel under the exact contract recorded in Product Review.

## Why Automation Stopped

This changes visible historical retrieval and therefore the user's control over
which personal records are surfaced. Book Zero permits a user-chosen time range
but does not decide the implementation semantics of record time, inclusive
boundaries, timezone, selection invalidation, or production UI activation.
Successful baseline tests and prior architecture acceptance do not grant that
authority.

## Relevant Constitution Clauses

- We Build Mirrors, Not Oracles.
- Human Before AI.
- Evidence Before Conclusion.
- Privacy Before Profit.
- User agency and control remain primary.

No Constitution edit is proposed.

## Relevant Primary Definitions

- `docs/06_Memory.md`: relevant continuity rather than maximum recall;
  user-chosen time-range retrieval must be selective and explainable.
- `docs/03_Principles.md`: context must remain bounded, optional, and
  agency-preserving.
- `docs/Reflection.md`: retrieval does not own cross-time meaning.
- `docs/09_AI.md`: AI remains a Context Steward.
- `docs/10_Privacy.md`: local retrieval is not provider-use consent.

## Relevant ADRs

- ADR-0009 keeps local retrieval, selection, consent, packet assembly,
  transmission, and provenance separate.
- ADR-0010 makes structured retrieval a Phase 3 exit dependency while Phase 4
  remains blocked.
- ADR-0011 does not authorize schema-v5 activation or migration for this work.

## Available Options

### Option A — Authorize the bounded production vertical slice

Authorize exactly this contract:

- one explicit per-panel, session-only “filter by saved date” control;
- inactive filter preserves current Phase 3A behavior;
- both start and end are required after explicit activation;
- the range applies only to source Experience `createdAt`, disclosed as saved
  date, never `updatedAt` or an inferred event date;
- both user dates are inclusive in the device IANA timezone captured at Apply,
  implemented as start-inclusive and next-day-start-exclusive instants;
- invalid/missing/inverted dates, unavailable timezone, and malformed source
  timestamps fail closed without unfiltered fallback;
- filter before existing lexical match, ranking, and unchanged candidate caps;
- show exact range, inclusive semantics, and timezone beside visible lexical
  relevance, as local UI metadata only;
- any range-control change closes the affected preflight and clears that
  Experience's ephemeral selections;
- closing stops retrieval but may retain the applied range for this app
  session; reopening retrieves fresh; opening never activates;
- English, Traditional Chinese, and Japanese parity;
- no persistence, provider/packet/consent/schema change or Phase 4 behavior.

### Option B — Design and evaluation contracts only

Keep the Product Review contract and add only test/evaluation scaffolding. Do
not activate production retrieval or UI.

### Option C — Defer structured retrieval

Do not implement R1. Separately evaluate Phase 3C Slice 4 current-state write
parity under a new Founder gate.

## Benefits

- **Option A:** delivers immediate user-visible control, reduces irrelevant
  local exposure, and closes part of a blocking Phase 3 exit gap without new
  AI authority or storage.
- **Option B:** further lowers implementation risk and can freeze contracts,
  but creates no user value and leaves structured retrieval unimplemented.
- **Option C:** may advance lifecycle infrastructure, but continues the recent
  infrastructure-first path and leaves the visible retrieval gap untouched.

## Risks

- **Option A:** users may confuse saved time with event time; timezone/DST and
  preflight invalidation need careful tests; UI state can become complex.
- **Option B:** contract scaffolding can become another non-product detour and
  does not satisfy the exit gap.
- **Option C:** further delays the product-facing Phase 3 exit and preserves the
  current inability to focus local history by period.

## Reversibility

- **Option A:** high. The feature is local, ephemeral, schema-neutral, and can
  be disabled without data migration; the inactive path preserves Phase 3A.
- **Option B:** high, but no production behavior exists to evaluate.
- **Option C:** high as a sequencing decision, but the gap remains.

## Data And Privacy Impact

Option A reads existing persisted `createdAt` values in memory, keeps range and
selection ephemeral, narrows before ranking/capping, and transmits or persists
nothing new. It does not change the historical packet, `ContextPacket`,
provider behavior, consent events, SQLite schema, or user data. Options B and C
have no runtime data effect.

## Orchestrator Recommendation

Option A. Use `createdAt` only and label it saved date. Capture the device IANA
timezone on Apply, normalize to exact instants, filter before lexical ranking,
and fail closed rather than falling back. This is the narrowest honest
production slice supported by current data.

## Default Safe Action

Do not edit production code. Retain the factual architecture/13 correction and
the proposed contract while the workflow remains blocked.

## Blocked Files Or Phases

Until resolution, do not edit `src/`, production tests, storage, schema,
providers, ContextPacket, Book Zero, Harness behavior, or later phases. Do not
stage, commit, push, merge, open a PR, or deploy.

## Exact Founder Response Needed

To authorize Option A, reply exactly:

```text
I resolve PHASE3-RETRIEVAL-R1-001 by selecting Option A. I authorize Phase 3 Exit Structured Retrieval R1 only: implement one explicit per-panel, session-only saved-date range control for the existing local historical-context panel; preserve current Phase 3A behavior when the filter is inactive; require explicit activation and both start and end dates; apply the range only to source Experience createdAt, disclosed as saved date and never as updatedAt or inferred event date; interpret both dates inclusively in the device IANA timezone captured at Apply using start-inclusive and next-day-start-exclusive instants; fail closed with visible English, Traditional Chinese, and Japanese messages for missing, invalid, inverted, or unresolvable ranges and exclude malformed source timestamps without falling back to unfiltered or whole-history retrieval; filter before the unchanged local-lexical-v1 matching, ranking, and candidate caps; show the exact range, inclusive semantics, and timezone beside lexical relevance as local UI metadata only; close the affected governed preflight and clear that current Experience's ephemeral selections whenever the range controls change; preserve closed-panel no-retrieval behavior while retaining only the last valid applied range for the current app session; add focused retrieval, boundary, timezone, lifecycle, selection, preflight, i18n, UI, stable-ranking, cap, zero-result, and regression tests; synchronize only factual Book One documentation; run canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff and manual UI review. I do not authorize an event-date field or inference, updatedAt filtering, persistence, schema changes, migration, provider or ContextPacket changes, consent-policy changes, whole-history loading, inferred emotion/relationship/value taxonomies, embeddings, vector search, memory graphs, summaries, Phase 4, Harness expansion, staging, commit, push, merge, PR, or deployment.
```

To choose Option B or C, state the selected option and its exact authorized
scope. Silence never resolves this decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3-RETRIEVAL-R1-001 by selecting Option A. I authorize Phase 3 Exit Structured Retrieval R1 only: implement one explicit per-panel, session-only saved-date range control for the existing local historical-context panel; preserve current Phase 3A behavior when the filter is inactive; require explicit activation and both start and end dates; apply the range only to source Experience createdAt, disclosed as saved date and never as updatedAt or inferred event date; interpret both dates inclusively in the device IANA timezone captured at Apply using start-inclusive and next-day-start-exclusive instants; fail closed with visible English, Traditional Chinese, and Japanese messages for missing, invalid, inverted, or unresolvable ranges and exclude malformed source timestamps without falling back to unfiltered or whole-history retrieval; filter before the unchanged local-lexical-v1 matching, ranking, and candidate caps; show the exact range, inclusive semantics, and timezone beside lexical relevance as local UI metadata only; close the affected governed preflight and clear that current Experience's ephemeral selections whenever the range controls change; preserve closed-panel no-retrieval behavior while retaining only the last valid applied range for the current app session; add focused retrieval, boundary, timezone, lifecycle, selection, preflight, i18n, UI, stable-ranking, cap, zero-result, and regression tests; synchronize only factual Book One documentation; run canonical verification, Theory Alignment Review, archive/reset, and stop at Founder diff and manual UI review. I do not authorize an event-date field or inference, updatedAt filtering, persistence, schema changes, migration, provider or ContextPacket changes, consent-policy changes, whole-history loading, inferred emotion/relationship/value taxonomies, embeddings, vector search, memory graphs, summaries, Phase 4, Harness expansion, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Phase 3 Exit Structured Retrieval R1 Option A only: explicit per-panel session-only createdAt saved-date range, inclusive captured-device-timezone semantics, fail-closed validation, pre-ranking lexical narrowing, visible local range reason, affected selection/preflight invalidation, three-locale parity, focused tests, factual Book One synchronization, verification/review/archive, and stop at Founder review. All stated non-scope remains excluded.

## Decided At And Evidence Reference

- Decided at: 2026-07-26T14:27:56.761Z
- Evidence reference: PHASE3-RETRIEVAL-R1-001

## Resume Phase

product_review
