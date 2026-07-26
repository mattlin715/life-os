# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-26-phase-3-exit-time-range-retrieval-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 5714b3eeeeb4c9612e445dbd78e4f606df41fa27
- Working-tree digest reviewed: 3cdb01163249e3903e9e7c2137bcfd9af8d1d3ef5dc8e49aef35d8a1b0382e0f
- Created at: 2026-07-26T14:12:29Z
- Updated at: 2026-07-26T14:27:56Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Close one visible part of the accepted Phase 3 structured-retrieval gap without
turning retrieval into interpretation: let the user explicitly narrow the
existing local lexical candidates by an exact saved-date range. The sprint must
first correct confirmed Slice 3B post-promotion wording and then stop for
Founder authority before editing production code.

## Problem Statement

Phase 3A already retrieves a bounded, deterministic candidate set only after a
panel is opened, but the user cannot constrain that set by time. Book Zero
explicitly names a user-chosen time range as an explainable retrieval signal,
and ADR-0010 keeps structured retrieval among the five blocking Phase 3 exit
gaps. The smallest honest step is a filter over existing lexical retrieval, not
a new relevance engine or a claim about when an Experience's described event
occurred.

## User Value

- A user can intentionally focus on a known period without loading all history.
- Lexical relevance remains visible and authoritative for candidate ranking.
- The filter reduces irrelevant exposure while keeping inclusion/exclusion
  under direct user control.
- No AI interpretation, provider call, durable preference, or schema change is
  needed.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human Before AI, Evidence Before Conclusion,
  Privacy Before Profit, and We Build Mirrors, Not Oracles.
- `docs/03_Principles.md`: context must improve understanding without reducing
  agency; Context Recovery remains bounded and optional.
- `docs/06_Memory.md`: longitudinal memory seeks relevant continuity rather
  than maximum recall; historical retrieval may use a user-chosen time range
  and must remain explainable and excludable.
- `docs/Reflection.md`: cross-time meaning remains reflective and user-owned;
  retrieval alone must not manufacture a conclusion.
- `docs/09_AI.md`: AI is a Context Steward, not the authority.
- `docs/10_Privacy.md`: local storage and retrieval do not imply provider-use
  consent; context use stays selective and controlled.

## Relevant ADRs

- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`:
  local retrieval, selection, consent, packet assembly, transmission, and
  provenance remain separate. Selection is not consent.
- `docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md`:
  structured retrieval is a blocking Phase 3 exit gap; Phase 4 implementation
  remains unauthorized.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`:
  accepted lifecycle/schema direction does not authorize schema-v5 activation
  or migration for this slice.

## Current Implementation Context

- **Implemented, Founder-reviewed, verified, and promoted:** Phase 3A
  `local-lexical-v1` retrieves only for explicitly opened panels, filters to
  eligible persisted sources/artifacts, excludes the current Experience, sorts
  deterministically, and limits candidates to three by default and ten
  maximum.
- **Implemented and promoted:** ephemeral exact-ID open-panel and selection
  state. Closing a panel stops retrieval; opening or selecting does not call a
  provider.
- **Implemented and promoted:** Phase 3B exact-content preflight, one-generation
  consent, separate historical packet, transport/persistence revalidation, and
  provenance for the bounded Historical Reflection Question task.
- **Founder-approved design, not production-authorized:** Phase 4
  Cross-Experience Hypotheses and Phase 3C schema-v5 cutover.
- **Implemented, verified, Founder-reviewed, and promoted as disposable
  evidence only:** Slice 3B migration restart orchestration at feature commit
  `b96bc84eae0b1d019cce761564c2689d92482705` and merge commit
  `5714b3eeeeb4c9612e445dbd78e4f606df41fa27`.
- **Prepared factual correction, not yet promoted:** architecture/13 version
  2.2 replaces stale Slice 3B working-tree/pending-promotion wording while
  preserving production `SCHEMA_VERSION = 4` and every later authority fence.
- **Proposed only:** the time-range retrieval contract below. No production
  behavior has changed in this sprint.

## In Scope

If and only if the Founder selects Option A:

1. Add one explicit, per-panel, ephemeral “filter by saved date” control.
2. Require the user to activate the filter and provide both start and end
   calendar dates before Apply.
3. Apply the range to the source Experience `createdAt` instant only. The UI
   must call it **saved date**, not event date. `updatedAt` is excluded because
   edits must not silently move an Experience between periods; no event date is
   currently represented and none may be inferred.
4. Interpret both dates as inclusive calendar dates in the device IANA
   timezone captured when Apply is pressed. Normalize them to
   `[local start-of-start-day, local start-of-day-after-end)` instants.
   Eligibility is `createdAt >= startInstant && createdAt < endExclusive`.
   This preserves inclusive user semantics across daylight-saving day lengths.
5. If the timezone cannot be resolved, a date is absent/invalid, the start is
   after the end, or a source `createdAt` is malformed, fail closed. An invalid
   range shows a localized message and retrieves no candidates; a malformed
   source cannot enter an active range.
6. With the filter inactive, run current Phase 3A behavior byte-for-behavior:
   no temporal filtering or new implicit default.
7. With a valid active filter, filter eligible Experiences before lexical
   matching, ranking, and candidate limiting. Preserve `local-lexical-v1`
   scoring, stable tie-breaking, and existing caps inside the narrowed set.
8. Show the exact saved-date range, inclusive semantics, and captured timezone
   beside the existing visible lexical reason. This is local UI retrieval
   metadata only and does not alter `ContextPacket`, the governed historical
   packet, or provider payloads.
9. Any activation toggle, start-date change, end-date change, or Apply action
   immediately closes an existing governed preflight and clears all ephemeral
   source selections for that current Experience. No selection is silently
   retained across a changed retrieval boundary.
10. Closing a panel performs no retrieval and retains the last valid applied
    range only for the current app session. Reopening re-runs local retrieval
    under that exact ephemeral range. Opening never activates a range.
11. Provide equivalent English, Traditional Chinese, and Japanese controls,
    disclosures, empty/error states, and reason semantics.

## Out Of Scope

- An event-date field, date inference from Experience text, `updatedAt`
  filtering, emotions/relationships/values taxonomies, embeddings, vector
  search, memory graphs, summaries, recurrence, contradiction, or Phase 4.
- Persistent filter preferences, database/schema changes, migration, provider
  calls, consent-policy changes, historical-packet or `ContextPacket` changes.
- Whole-history background loading, changed candidate caps, new fallback
  retrieval, Book Zero/Constitution edits, Harness expansion, Git promotion,
  PR, or deployment.

## Product Constraints

- The range narrows lexical relevance; it never replaces relevance.
- Invalid input cannot degrade to unfiltered or whole-history retrieval.
- No range means current behavior, not an implied “all time” user choice.
- The control must describe stored record time honestly and must not imply the
  date of the life event described in the text.

## Evidence And Provenance Constraints

The filter consumes only the persisted source Experience `createdAt` value and
does not create evidence, revision history, provenance, or a durable artifact.
The existing candidate retains exact source ID/timestamps and visible lexical
terms. Range display is local UI metadata and cannot be represented as AI
evidence or user meaning.

## Historical Context Constraints

Retrieval remains explicit-panel, local, bounded, deterministic, explainable,
provider-independent, and limited by existing caps. Rejected, skipped,
foreign, orphaned, deleted, stale, unsaved, or otherwise ineligible material
remains excluded. No whole-history load or Cross-Experience conclusion is
introduced.

## Consent Constraints

Opening, activating, applying, changing, or clearing a range; viewing a
candidate; and selecting a source are not consent. Phase 3B still requires a
fresh exact-content preflight and one-generation/per-purpose consent. Changing
the retrieval range invalidates an open preflight for the affected Experience.

## AI-Role Constraints

No AI participates in range selection or temporal classification. Life OS does
not infer relationships, emotions, values, event dates, recurrence, change, or
identity. The provider boundary and Phase 3B output evaluator remain unchanged.

## Privacy Constraints

All filtering and range state remain in memory on the device. Narrowing occurs
before ranking/capping and can reduce visible history. No range, timezone,
candidate, or selection is persisted or transmitted by this slice.

## User-Agency Constraints

The user explicitly activates, chooses, applies, changes, or disables the
range. Every candidate remains separately includable/excludable. Invalid input
has a visible explanation and a safe empty result rather than silent fallback.
The user may continue with the current Experience and no history.

## Acceptance Criteria

### Automated

1. No-filter retrieval matches the existing results, order, reasons, and caps.
2. Valid ranges filter Experiences before lexical scoring and limiting.
3. Start boundary is included; the next local day after the end is excluded;
   same-day ranges work; injected DST/timezone cases are deterministic.
4. Missing, invalid, inverted, or unresolvable-timezone ranges return a typed
   fail-closed result and never invoke unfiltered retrieval.
5. Malformed source `createdAt` is excluded only when an active range requires
   temporal eligibility and never crashes rendering.
6. Zero-result, default cap, maximum cap, and stable-ranking cases pass.
7. Closed panels do not retrieve; reopened panels re-run under the retained
   ephemeral applied range; opening alone does not activate it.
8. Every range control change clears the affected exact-ID selection and closes
   the affected governed preflight without calling a provider.
9. English, Traditional Chinese, and Japanese keys and semantics remain
   equivalent, including saved-date wording, inclusive range/timezone
   disclosure, invalid state, and empty result.
10. Existing provider, packet, consent, persistence, schema-v4, historical
    evaluator, and Phase 3A tests remain unchanged and passing.
11. Canonical `scripts/verify.ps1` passes with no Constitution diff.

### Founder manual UI review

1. In each locale, opening the panel without enabling the filter shows current
   Phase 3A candidates unchanged.
2. Enabling the filter, entering a valid same-day and multi-day saved-date
   range, and applying it narrows only lexically relevant candidates and shows
   the exact inclusive range plus timezone.
3. Missing and inverted ranges show a calm visible error and no candidates;
   they never show the prior unfiltered list.
4. Changing either date after selecting a source clears that selection and
   closes an open exact-content preflight; no provider request or consent record
   is created.
5. A valid range with no lexical match shows the localized empty state and the
   user can continue without history.
6. Closing and reopening the panel makes no provider call, retains only the
   session-local applied range, and retrieves fresh candidates.

## Risks

- `createdAt` is a saved timestamp, not necessarily the event time. Mitigation:
  explicit saved-date wording and no inference.
- Device timezone changes can alter future applications of the same calendar
  text. Mitigation: capture and display the timezone at Apply; retain exact
  instants until the user applies again.
- Date and selection state inside `App.tsx` can become coupled to preflight
  state. Mitigation: pure range/lifecycle helpers and focused invalidation
  tests before UI wiring.
- A temporal filter could be mistaken for stronger relevance. Mitigation:
  retain lexical scoring/reasons and display time as a user constraint, not an
  inferred reason.
- Hidden fallback on invalid input could expose more history. Mitigation:
  typed fail-closed results and explicit empty/error UI.

## Open Questions

none. The Founder selected Option A and authorized only the exact temporal
contract and non-scope recorded in `PHASE3-RETRIEVAL-R1-001`.

## Human Decision Required

No unresolved decision. `PHASE3-RETRIEVAL-R1-001` was resolved as Option A on
2026-07-26. Implementation must preserve every authorized condition and stop at
Founder diff and manual UI review.

## Recommendation

Proceed with the Founder-authorized Option A contract only. It delivers the
smallest user-visible part of a blocking Phase 3 exit gap while remaining
local-only, reversible, schema-neutral, and subordinate to existing lexical
relevance and Phase 3B consent.

## Review Status

approved_with_conditions
