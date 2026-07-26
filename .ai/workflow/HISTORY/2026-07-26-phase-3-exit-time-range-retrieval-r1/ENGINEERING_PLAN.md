# Engineering Plan

Status: approved

- Sprint ID: 2026-07-26-phase-3-exit-time-range-retrieval-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 5714b3eeeeb4c9612e445dbd78e4f606df41fa27
- Working-tree digest reviewed: 3cdb01163249e3903e9e7c2137bcfd9af8d1d3ef5dc8e49aef35d8a1b0382e0f
- Created at: 2026-07-26T14:29:00Z
- Updated at: 2026-07-26T14:29:00Z

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions` after the Founder resolved
`PHASE3-RETRIEVAL-R1-001` as Option A. Implement only one explicit per-panel,
session-only source-Experience `createdAt` saved-date range that narrows the
existing lexical candidate set. Preserve inactive Phase 3A behavior, schema v4,
provider/packet/consent behavior, and every stated non-scope. Stop at Founder
diff and manual UI review without Git promotion.

## Existing Implementation Understanding

- `src/historicalContext/retrieve.ts` owns deterministic `local-lexical-v1`
  matching, ranking, and caps. It currently considers all eligible Experiences
  except the current one.
- `src/historicalContext/panelRetrieval.ts` is the explicit-open-panel boundary;
  closed panels do not call retrieval.
- `src/historicalContext/date.ts` already strictly validates persisted
  RFC3339/ISO timestamps and separates source parsing from presentation.
- `src/historicalContext/panelState.ts` and `selection.ts` keep exact-ID,
  session-only state.
- `src/app/App.tsx` owns open panels, candidate reconciliation, ephemeral
  selections, and governed preflight invalidation.
- `src/app/i18n.ts` contains equivalent English, Traditional Chinese, and
  Japanese product copy.
- Phase 3B packet assembly consumes selected candidate IDs and lexical reasons.
  The range must remain outside that packet contract.

## Affected Modules

- New `src/historicalContext/savedDateRange.ts` and focused tests: strict date
  validation, device-timezone capture/conversion seam, applied range/control
  types, fail-closed results, and immutable per-Experience control updates.
- `src/historicalContext/types.ts`: optional valid applied saved-date range on
  retrieval input only; candidate and packet types remain unchanged.
- `src/historicalContext/retrieve.ts` and tests: pre-lexical range filtering and
  malformed-source exclusion under an active range.
- `src/historicalContext/panelRetrieval.ts` and tests: per-panel inactive,
  blocked, or applied constraint; blocked ranges skip the retriever.
- `src/historicalContext/selection.ts` tests: confirm exact current-Experience
  clearing used by range changes.
- `src/app/App.tsx`: per-panel session-only controls, explicit enable/date/Apply
  handlers, selection/preflight invalidation, applied constraint routing, and
  visible control/error/range reason.
- `src/app/i18n.ts` and i18n tests: equivalent three-locale saved-date language.
- `src/styles.css`: minimal layout for the bounded control and messages.
- `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`:
  factual implemented R1 behavior only after verification.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`:
  already-prepared Slice 3B promotion correction only.

## Proposed Design

1. Define `HistoricalSavedDateRange` with raw start/end `YYYY-MM-DD`, captured
   IANA timezone label, and numeric start-inclusive/end-exclusive instants.
2. Define a per-panel `HistoricalSavedDateRangeControl`:
   inactive; enabled/editing with no applied range; enabled/invalid with a typed
   localized reason key; or enabled/applied with the exact range.
3. Use a strict calendar-date parser. Production boundary conversion uses
   device-local midnight and captures
   `Intl.DateTimeFormat().resolvedOptions().timeZone`; the converter is
   injectable for deterministic timezone/DST tests. End is the next local
   midnight, not `23:59:59.999`.
4. Any enable/start/end change removes the applied range. App handlers also
   clear that exact current Experience's selection and close its preflight.
   Apply repeats the same invalidation before validating.
5. Map each open panel to `inactive`, `blocked`, or `applied`. `inactive` omits
   the retrieval range and preserves existing behavior. `blocked` writes an
   empty candidate result without calling the retriever. `applied` passes only
   the verified range.
6. In `findHistoricalContextCandidates`, exclude the current Experience, then
   apply the saved-date predicate, then execute the unchanged lexical mapping,
   sorting, and limiting. An invalid persisted `createdAt` is excluded only
   when a valid range is active.
7. Render the exact applied dates, inclusive wording, and captured timezone
   beside each candidate's lexical reason. Do not add the range to candidate
   reasons, packet assembly, digest, preflight content, or provider payload.
8. Closing a panel leaves its control in memory but the existing explicit-panel
   function performs no retrieval. Reopening recomputes under the last valid
   applied range.

## Alternatives Considered

- Filter after limiting: rejected because relevant in-range sources could be
  hidden by out-of-range top candidates.
- Filter on `updatedAt`: rejected because editing would silently change
  historical period membership.
- Infer event date: rejected because the data model has no explicit event date
  and invention would violate Evidence Before Conclusion.
- UTC date semantics: rejected because the authorized contract is the user's
  device calendar and captured timezone.
- Persist the range: rejected because selection/filter state is authorized as
  session-only.
- Add range to the historical packet: rejected because provider/packet changes
  are outside authority and local retrieval metadata is sufficient.

## Data Lifecycle Impact

Only ephemeral React/map state is added. Range controls disappear on app
restart. Experience deletion removes its control entry. No existing record,
artifact, selection, consent, or provenance row is changed.

## SQLite Or Migration Impact

None. No SQLite interface, SQL, Rust command, schema constant, `user_version`,
migration, backup, restore, or app-data behavior changes.

## Provenance Impact

None. The range is a local retrieval constraint, not an Evidence or durable
artifact. Existing exact source/provenance semantics remain unchanged.

## Historical Context Impact

One optional deterministic pre-ranking filter is added. It uses only
Experience `createdAt`, preserves existing lexical scoring and caps, and fails
closed for invalid active controls. Selection stays ephemeral.

## Consent Impact

No consent contract change. Every range control mutation closes the affected
preflight and clears its selection, forcing fresh source choice and the
existing Phase 3B preflight/consent path.

## Provider Transmission Impact

None. No `ContextPacket`, governed historical packet, provider adapter,
destination, prompt, evaluator, or transport payload changes.

## Import And Export Impact

No format change. Imported malformed `createdAt` remains displayable under the
existing fallback, but is ineligible while a saved-date range is active.
Ranges are neither imported nor exported.

## Test Strategy

- Strict valid/invalid calendar dates, same-day and inclusive boundaries.
- Injected UTC and DST-like 23/25-hour boundary converters plus unavailable
  timezone.
- Inactive, editing/invalid, and applied immutable control transitions.
- Range filtering before lexical ranking/caps, zero results, stable tie-breaks,
  malformed source timestamps, and unchanged inactive retrieval.
- Open/closed/reopen panel behavior and blocked-range no-call proof.
- Selection clearing and preflight invalidation decision helpers.
- Three-locale key/semantic parity and App control labels.
- Existing historical packet/provider/persistence tests as regressions.

## Repository Verification Strategy

Run focused Vitest files during implementation, then:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record the exact canonical exit code and counts through the workflow.

## Manual UI Verification

Owner: Founder after diff review. Verify inactive parity; same-day/multi-day
range; timezone/range disclosure; invalid/inverted fail-closed empty state;
selection and preflight invalidation; no-match behavior; closed/reopened panel;
and English/Traditional Chinese/Japanese parity. No provider call should occur
from range controls.

## Rollback Or Recovery Strategy

The feature is session-only and schema-neutral. A code rollback removes the
control without data repair. Runtime invalid input returns an empty local
candidate set and can be corrected or disabled by the user. No autonomous
repair or persisted cleanup exists.

## Documentation Impact

Update only architecture/08 after implementation/verification and retain the
minimal architecture/13 promotion correction. No Book Zero, Constitution,
Roadmap phase advancement, ADR status, or Harness document changes.

## ADR Impact

No new ADR. Book Zero already names a user-chosen time range, and ADR-0009
already separates local retrieval from consent/transmission. This slice applies
those accepted boundaries without changing them.

## Risk Level

Medium. Data/storage risk is low, but date/timezone semantics and preflight
invalidation are privacy-relevant. Pure helpers, injected boundary tests, and
fail-closed panel behavior bound the risk.

## Escalation Decision

No further escalation is required inside the exact Founder-authorized contract.
Stop and return to `human_decision_required` if implementation requires an
event-date field, persistence, packet/provider/schema changes, inferred
taxonomy, or any other excluded authority.
