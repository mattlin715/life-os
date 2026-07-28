# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-07-27-phase-3-exit-provenance-inspector-p1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: db43b8f47815b0c6ddb1bd8daa9a9503c11a2146
- Working-tree digest implemented: e6013577614138f2b77a4d20d8a26f0358a58053075c0ab038275bed131aca32
- Created at: 2026-07-27T16:49:19.1794897Z
- Updated at: 2026-07-27T16:49:19.1794897Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Implemented the Founder-authorized P1 local actual-use inspector over one
already-loaded schema-v4 Historical Question and exact packet snapshot. The UI
is collapsed by default, validates on explicit open, distinguishes four
historical-use stages, and keeps exact outgoing content absent from rendering
until a second explicit reveal.

## Existing System Areas Inspected

- `src/historicalContext/governedPacket.ts`
- `src/historicalContext/types.ts`
- `src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`
- `src/shared/storage/inMemoryLocalEvidenceStore.ts`
- `src/app/App.tsx`
- `src/app/i18n.ts`
- `src/styles.css`
- schema-v4 Historical Question Rust persistence and cascade tests
- ADR-0009 and architectures 08, 09, 10, 12, and 13

## Files Added

- `src/historicalContext/provenanceInspector.ts`
- `src/historicalContext/provenanceInspector.test.ts`
- `src/app/HistoricalProvenanceInspector.tsx`
- `src/app/HistoricalProvenanceInspector.test.tsx`

## Files Modified

- `src/app/App.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `src/styles.css`
- `docs/11_MVP.md`
- `docs/12_Roadmap.md`
- `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`
- Required sprint workflow artifacts
- Pre-implementation factual corrections in architectures 08 and 13

## Files Deleted

None.

## Behavior Changed

An already-persisted Historical Question now offers an explicit local
provenance inspector. Valid records show artifact/question/citation, packet,
destination, purpose, consent, transmission, source/artifact snapshot, review,
relevance, and dependency metadata. Exact outgoing content requires a separate
reveal. Invalid records show one calm localized fail-closed message without
partial metadata.

## Data Model Impact

None. The existing `HistoricalQuestionArtifact` and embedded exact packet
snapshot remain unchanged.

## Migration Impact

None. Production `SCHEMA_VERSION` and SQLite `user_version` remain 4.

## Provenance Impact

Existing actual-use evidence becomes inspectable. Runtime validation verifies
supported versions, lifecycle consistency, citations, packet-represented
dependencies, limits, and packet digest before details render. No provenance is
created or mutated.

## Historical Context Impact

None to retrieval, selection, packet assembly, or transmission. Only one
already-loaded saved result is inspected. No source is rehydrated.

## Consent Impact

None. The existing consent reference is displayed; opening and revealing do not
request, consume, renew, persist, or imply consent.

## Provider Transmission Impact

None. No provider or ContextPacket file changed and the inspector has no
network-capable dependency.

## Tests Added

- 12 validator/view-model cases covering valid evidence, malformed and
  unsupported hydration, digest mismatch, lifecycle contradictions, duplicate
  and orphaned dependencies, citation failures, exact content, and the Phase 3B
  no-Phase-4 output boundary.
- 8 component rendering cases covering collapsed default, four stages, local
  disclosure, three locales, separate content reveal, and fail-closed output.
- 1 localization parity case.

## Tests Executed

- Focused Vitest: 3 files / 28 tests passed after the final no-Phase-4 case.
- Canonical `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`:
  17 workflow tests, 26 Vitest files / 204 tests, 70 Rust library tests,
  12 backup/restore tests, and 8 schema-contract tests passed.
- TypeScript typecheck, frontend production build, Rust check, UTF-8,
  whitespace, secret-file, Markdown-link, and Constitution checks passed.

## Verification Results

The first canonical run found one Engineering Plan CRLF/trailing-whitespace
artifact after all code, TypeScript, frontend, and Rust checks had passed. The
workflow document was normalized without changing implementation. The next
canonical run passed completely. A final canonical run is required after the
factual verification-count wording and workflow reports are synchronized.

## Manual Verification Required

Founder manual UI review remains required. It must verify collapsed default,
explicit open, four-stage distinction, exact citations and packet fields,
separate content reveal/hide, English/Traditional Chinese/Japanese parity,
R1 coexistence, and Historical Question deletion/unmount behavior. No manual
result is claimed.

## Documentation Updates

Architectures 08 and 13 and the Roadmap received pre-implementation factual
promotion corrections. Architecture 10, MVP, and Roadmap now state the bounded
P1 implemented/verified working-tree behavior and explicitly preserve pending
Founder review, promotion, deployment, and schema-v5 graph work.

## ADR Impact

No new ADR and no status change. P1 applies ADR-0009 without changing its
consent, transmission, retention, or deletion policy.

## Deviations From Plan

None in product scope. The implementation uses the existing Phase 3B output
validator in addition to structural/digest validation so a corrupted persisted
question cannot expose a Phase 4-crossing output.

## Known Limitations

P1 sees only the schema-v4 artifact and exact packet snapshot. It does not query
the separate dependency table, show later lifecycle history, repair malformed
records, or replace the schema-v5 provenance/dependency graph inspector.

## Remaining Risks

Founder manual UI review is pending. The validator intentionally fails closed
if old records do not satisfy the supported current Phase 3B contract; this
protects provenance truth but can hide details for a malformed record.

## Git State

Branch `codex/phase-3-exit-provenance-inspector-p1`, HEAD
`db43b8f47815b0c6ddb1bd8daa9a9503c11a2146`, no upstream, unstaged product,
documentation, and workflow changes only, with no staged files, commit, push,
merge, PR, deployment, or release.

## Engineer Completion Status

`completed_with_follow_up`: implementation and automated evidence are complete;
Founder diff and manual UI review remain required.
