# Theory Alignment Review

Status: approved_with_follow_up

- Sprint ID: 2026-07-26-phase-3-exit-time-range-retrieval-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 5714b3eeeeb4c9612e445dbd78e4f606df41fa27
- Working-tree digest reviewed: dbd0bd7d4df934aa52824d25caf2af0de017c965f45e41ba31607edf5b399f9d
- Created at: 2026-07-26T14:49:30Z
- Updated at: 2026-07-26T14:49:30Z

Allowed final status: `approved`, `approved_with_follow_up`,
`revision_required`, `human_decision_required`, or `rejected`.

## Actual Diff Reviewed

Reviewed all 24 authorized changed paths from `git status --porcelain`,
including four new saved-date/UI test files, nine modified product source/test
files, four factual Book One documents, and repository-mediated workflow
artifacts. Independent exact-path validation passed. `git diff --check` passed.
No staged files exist. No diff exists under `src/ai`, `src/shared`,
`src-tauri`, `scripts`, `.github`, the Constitution, ADRs, or Product Harness
documents. Production `SCHEMA_VERSION` remains 4 and the algorithm identifier
remains `local-lexical-v1`.

## Acceptance Criteria Verification

1. **Inactive parity:** passed. Unit regression compares omitted/undefined
   range inputs; existing Phase 3A suite remains green.
2. **Filter before lexical ranking/caps:** passed. An out-of-range higher-score
   source cannot consume the in-range cap; stable exact-ID tie order remains.
3. **Inclusive/timezone boundaries:** passed. Start and end-day instants,
   same-day range, next-day exclusion, and injected 23/25-hour local days pass.
4. **Fail-closed invalid range:** passed. Missing, invalid, inverted,
   unavailable-timezone, and boundary-resolution cases return typed blocking
   results. Explicit-panel tests prove blocked input does not call retrieval.
5. **Malformed source timestamp:** passed. It is excluded under an active range
   and preserves the prior no-range behavior.
6. **Zero results/caps/ranking:** passed in focused and full suites.
7. **Closed/reopen lifecycle:** passed. Closed panels make no call; retained
   applied constraints are routed only after reopen.
8. **Selection/preflight invalidation:** passed by exact selection-clear
   regressions, pure exact-current preflight decision tests, and App handlers
   reused by enable/start/end/Apply.
9. **Three-locale/UI parity:** passed. English, Traditional Chinese, and
   Japanese copy semantics plus static control/alert rendering pass.
10. **Provider/packet/persistence/schema regressions:** passed. No relevant file
    diff exists and governed packet/provider/storage suites remain green.
11. **Canonical verification:** passed with workflow 17/17, Vitest 24
    files/183 tests, Rust 70/70, backup/restore 12/12, schema contract 8/8,
    typecheck, frontend build, Rust check, hygiene, and no Constitution diff.
12. **Founder manual UI review:** pending as the explicit follow-up gate.

## Constitution Alignment

Aligned. The change is explicit, optional, local, bounded, reversible, and
fail-closed. It changes no constitutional text or hierarchy.

## Primary-Definition Alignment

Aligned with `docs/06_Memory.md`: a user-chosen time range narrows relevant
continuity without maximum recall. It applies `docs/03_Principles.md`,
`Reflection.md`, `docs/09_AI.md`, and `docs/10_Privacy.md` without redefining
Context, Reflection, AI role, Memory, or Privacy.

## Relevant ADR Alignment

- ADR-0009: selection remains distinct from consent and provider use.
- ADR-0010: this implements a bounded part of the structured-retrieval Phase 3
  exit gap without entering Cross-Experience Reflection.
- ADR-0011: schema/lifecycle direction is untouched; schema v5 is not activated.

No ADR status or decision changed.

## Mirrors-Not-Oracles Alignment

Aligned. The device applies user-supplied dates mechanically. It does not infer
event dates, themes, identity, meaning, recurrence, change, or advice.

## Context-Before-Insight Alignment

Aligned. The range narrows context before the existing lexical relevance
calculation. It does not transform a date match into insight or stronger
semantic relevance.

## Evidence Boundary

Aligned. `createdAt` is treated only as a persisted saved instant. UI and docs
explicitly refuse to call it an event date. The range creates no Evidence.

## Provenance Boundary

Aligned. Exact source IDs/timestamps and existing lexical reasons remain. Range
metadata is visible but deliberately absent from packet/provenance persistence.

## Artifact Lifecycle Boundary

Aligned. No artifact creation, revision, review, rejection, deletion,
tombstone, export, or migration behavior changed.

## Historical Context Consent Boundary

Aligned. Range controls make no provider call and are not consent. Every range
mutation clears affected selection and closes its preflight, requiring the
existing exact-content selection/preflight/consent process.

## Cross-Experience Hypothesis Boundary

Aligned. No comparison conclusion, recurrence, contradiction, change-over-time,
summary, Pattern, identity hypothesis, or Phase 4 artifact is generated.

## User Agency

Aligned. The filter is inactive by default, requires enable/date/Apply actions,
shows errors rather than fallback, permits individual include/exclude, and can
be disabled without durable effects.

## Privacy

Aligned. Range state is session-only and local. Narrowing happens before
ranking/capping. No new content, timestamp, timezone, selection, or range leaves
the device or enters persistence.

## Psychological Safety

Aligned. Wording says saved date, provides calm visible fail-closed feedback,
does not imply that the user remembered an event on that date, and preserves
the option to continue without history.

## Scope Deviations

none. Extracting the UI control into a bounded component added direct UI tests
but no new product capability or dependency.

## Required Corrections

none.

## Human Decision Required

No new design decision. Founder diff review and Founder-owned manual UI
verification remain required before promotion; they do not authorize Git
actions by themselves.

## Revision Log

- Cycle 0: all criteria passed; no correction or revision transition required.

## Final Review Status

approved_with_follow_up
