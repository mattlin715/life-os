# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-11-daily-reflection-founder-review-corrections
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: f8fbf94812bac2e1359367362e9c1e0db1aeaef1
- Working-tree digest implemented: 64a45fde5216e770b1532112f87775dbae32d2d586703a0d68b77a23592a801c
- Created at: 2026-08-11T02:20:00+09:00
- Updated at: 2026-08-11T02:20:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Corrected three Founder-observed R1 defects without altering the Pattern gate or ADR-0009 consent policy: explicit event-context explanation, a discoverable open-only history shortcut, and `local-lexical-v2` meaningful CJK overlap with persisted-v1 provenance compatibility.

## Existing System Areas Inspected

Inspected `src/ai/harness/patternAvailability.ts`, `contextSufficiency.ts`, `src/app/App.tsx`, `i18n.ts`, historical panel/selection/retrieval/packet/provenance code and tests, architecture/08/09/11, ADR-0009, and the unpromoted R1 diff/archive.

## Files Added

- `src/app/HistoricalContextEntryPoint.tsx`
- `src/app/HistoricalContextEntryPoint.test.tsx`

## Files Modified

- `src/app/App.tsx`, `src/app/i18n.ts`, `src/app/i18n.test.ts`, `src/app/HistoricalProvenanceInspector.test.tsx`, `src/styles.css`
- `src/historicalContext/types.ts`, `retrieve.ts`, `retrieve.test.ts`, `governedPacket.ts`, `provenanceInspector.ts`, `provenanceInspector.test.ts`
- `docs/11_MVP.md`, `docs/product/00_MVP_User_Flow.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`, `docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md`
- repository workflow artifacts for this corrective sprint

## Files Deleted

None.

## Behavior Changed

- Insufficient Pattern context now acknowledges that confirmed clues and saved Reflection remain valid, then names the missing event facts and the exact action.
- Every open Experience shows a top shortcut that opens (never toggles closed) and scrolls/focuses the existing historical panel; no source is selected.
- CJK retrieval uses locale word segmentation and explicit generic-term filtering. Visible reasons no longer expose adjacent fragments or generic `感到`/`覺得` scaffolding.
- New governed packets disclose `local-lexical-v2`; exact persisted v1 packets remain read-only inspectable.

## Data Model Impact

None. Ephemeral UI and retrieval data only.

## Migration Impact

None. Production schema and `user_version` remain 4.

## Provenance Impact

No record rewrite. The validator accepts exact v1 and v2 retrieval metadata; new immutable packet digests include v2 normally.

## Historical Context Impact

Panel-open retrieval only. English behavior, saved-date filtering, ranking formula, candidate caps, exact IDs, eligibility, and no-whole-history behavior remain unchanged. CJK token quality is narrower and readable.

## Consent Impact

No policy or state change. Opening and selection remain non-consent; exact preflight and per-generation/per-purpose consent remain required.

## Provider Transmission Impact

No new provider call or adapter behavior. Packet shape and transport remain unchanged.

## Tests Added

- 5 entry-point component tests.
- 3 new retrieval-quality regressions plus updated algorithm-version assertions.
- Persisted v1/current v2 provenance compatibility regression.
- Expanded three-language explanation and shortcut-boundary assertions.

## Tests Executed

- Focused Vitest: 55/55 passed.
- Canonical `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`: passed after stopping the running review executable that had locked the Rust target.

## Verification Results

Pass: 17 workflow tests; 35 Vitest files / 258 tests; 189 Rust library tests; 12 backup/restore integration tests; 8 schema-contract tests; TypeScript; frontend build (83 modules); Rust check; whitespace, UTF-8, secret, Markdown links; no Constitution diff. Initial canonical attempt was blocked only by Windows executable locking and passed after the exact Life OS review process was stopped.

## Manual Verification Required

Founder manual UI review remains required. No manual result is claimed. A fresh desktop process must be launched and reviewed one bounded step at a time.

## Documentation Updates

Architecture/08 documents v2 extraction and v1 provenance compatibility; architecture/09/11 and MVP/User Flow are factually synchronized without new authority.

## ADR Impact

No new ADR and no acceptance-status change.

## Deviations From Plan

The plan anticipated a conservative fallback; implementation uses complete visible runs when `Intl.Segmenter` is unavailable. No scope deviation.

## Known Limitations

- Runtime word segmentation is pinned-runtime deterministic but can differ across future ICU upgrades; regression tests make such drift visible.
- Manual review must confirm scroll/focus behavior and real-data relevance reasons.
- The complete R1 working tree remains unpromoted.

## Remaining Risks

Founder should inspect whether meaningful CJK terms are sufficiently selective with real entries. No destructive manual check is needed.

## Git State

Branch `codex/daily-reflection-core-ux-r1`; HEAD `f8fbf94812bac2e1359367362e9c1e0db1aeaef1`; unstaged R1 plus corrective work and workflow evidence; index empty; feature branch has no upstream. No commit, push, merge, PR, deployment, or release.

## Engineer Completion Status

completed_with_follow_up
