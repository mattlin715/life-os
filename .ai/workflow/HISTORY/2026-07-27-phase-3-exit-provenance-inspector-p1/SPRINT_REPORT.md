# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Sprint ID: 2026-07-27-phase-3-exit-provenance-inspector-p1
- Created at: 2026-07-27T16:54:00.0000000Z
- Updated at: 2026-07-27T16:54:00.0000000Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, `failed`, or `cancelled`.

## Sprint ID

`2026-07-27-phase-3-exit-provenance-inspector-p1`

## Mission

Deliver one Founder-gated, product-facing, local read-only actual-use inspector
for an already-persisted Phase 3B Historical Question while preserving all
schema, consent, provider, deletion, and Phase 4 fences.

## Starting Commit

`db43b8f47815b0c6ddb1bd8daa9a9503c11a2146`

## Ending Commit Or Working-Tree State

Same uncommitted HEAD with product working-tree digest
`e6013577614138f2b77a4d20d8a26f0358a58053075c0ab038275bed131aca32`.

## Final Status

`completed_with_follow_up`: implemented, canonically verified, and theory
approved; Founder diff and manual UI review remain pending.

## Product Decision

Founder resolved `PHASE3-PROVENANCE-INSPECTOR-P1-001` with Option A and exact
bounded scope. P1 is explicitly partial and does not authorize schema-v5 graph
inspection or any new storage/provider behavior.

## Engineering Summary

Added a strict asynchronous runtime validator/view model and a local React
inspector. The outer surface is collapsed by default and validates only after
explicit open. It separates selection, consent, successful-transmission
reference, and persisted artifact, then keeps exact packet content behind a
second explicit reveal.

## Behavior Changed

Users can inspect exact existing actual-use evidence for one saved Historical
Question. Invalid or contradictory evidence shows a calm three-locale
fail-closed message. No new data is loaded, sent, copied, or persisted.

## Files Changed

Product:

- `src/historicalContext/provenanceInspector.ts`
- `src/historicalContext/provenanceInspector.test.ts`
- `src/app/HistoricalProvenanceInspector.tsx`
- `src/app/HistoricalProvenanceInspector.test.tsx`
- `src/app/App.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `src/styles.css`

Factual documentation:

- `docs/11_MVP.md`
- `docs/12_Roadmap.md`
- `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`
- `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

Repository workflow artifacts were updated through the native workflow.

## Tests

Added 20 focused validator/component cases plus one localization case. Final
canonical evidence: 17 workflow tests; 26 Vitest files / 204 tests; 70 Rust
library tests; 12 backup/restore tests; 8 schema-contract tests; TypeScript
typecheck; frontend build; Rust check; UTF-8, whitespace, secret, Markdown-link,
and Constitution checks all passed.

## Repository Verification

Passed:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

The first full run detected only a workflow-document CRLF whitespace issue after
all implementation checks passed. That document was normalized, and two later
canonical runs passed completely. The recorded verification is bound to the
final product digest above.

## Manual Verification

Pending Founder review. No desktop/manual result is claimed.

## Architecture Updates

Architecture 10 records the P1 schema-v4 read-only boundary. Architectures 08
and 13 contain only factual post-promotion corrections from sprint intake and
preserve every existing authority fence.

## ADR Updates

None. ADR-0009 remains Accepted and unchanged.

## Documentation Synchronization

MVP and Roadmap now distinguish the canonically verified working-tree P1 state
from Founder-reviewed, promoted, deployed, and released states.

## Data And Migration Impact

None. No storage API/query or schema object changed. Production
`SCHEMA_VERSION` and `user_version` remain 4.

## Provenance And Consent Impact

Existing actual-use evidence is inspectable but not mutated. Selection, consent,
transmission reference, and artifact persistence are visibly distinct. Opening
or revealing does not create or imply consent.

## Risks

Manual UI review remains. Strict validation can intentionally hide malformed
legacy provenance rather than show partial evidence. P1 does not expose the
future complete schema-v5 dependency/lifecycle graph.

## Deferred Items

Founder review/promotion, schema-v5 provenance/dependency graph inspection,
artifact correction/revision lifecycle, export v2, schema-v5 activation,
Phase 4, deployment, and release.

## Human Decisions

- `PHASE3-PROVENANCE-INSPECTOR-P1-001`: Option A, resolved with exact scope.
- Next: Founder diff and stepwise manual UI review; promotion requires a
  separate authorization.

## Review Cycles

Cycle 0 approved with the planned Founder manual follow-up. No revision cycle.

## Workflow Lessons

No Harness defect blocked product work. Runtime validation is necessary at the
presentation boundary because schema-v4 hydration is compile-time cast rather
than runtime validation. The existing workflow needed no expansion.

## Recommended Next Sprint

After P1 Founder review and promotion, evaluate the smallest remaining Phase 3
exit capability that improves user correction/export agency. Keep schema-v5
activation and Phase 4 separate.

## Git Status

Branch `codex/phase-3-exit-provenance-inspector-p1`, HEAD
`db43b8f47815b0c6ddb1bd8daa9a9503c11a2146`, no upstream, unstaged working
changes, no staged files, commit, push, merge, PR, deployment, or release.
