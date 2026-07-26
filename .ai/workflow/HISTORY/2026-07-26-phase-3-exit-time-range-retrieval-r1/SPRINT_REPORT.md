# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-26T14:51:00Z
- Updated at: 2026-07-26T14:51:00Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, `failed`, or `cancelled`.

## Sprint ID

2026-07-26-phase-3-exit-time-range-retrieval-r1

## Mission

Deliver one Founder-gated, user-visible Phase 3 exit capability: explicit,
local, session-only historical retrieval narrowing by source Experience saved
date, while preserving lexical relevance, consent separation, schema v4, and
the Phase 4 fence.

## Starting Commit

`5714b3eeeeb4c9612e445dbd78e4f606df41fa27`

## Ending Commit Or Working-Tree State

No new commit. Authorized unstaged working-tree digest:
`dbd0bd7d4df934aa52824d25caf2af0de017c965f45e41ba31607edf5b399f9d`.

## Final Status

completed_with_follow_up. Implementation, canonical verification, exact-scope
diff validation, and Theory Alignment Review passed. Founder diff and manual UI
review remain pending before any promotion authorization.

## Product Decision

`PHASE3-RETRIEVAL-R1-001` Option A was explicitly resolved. The exact saved-date
contract is implemented; all event-date inference, persistence, schema,
provider/packet, taxonomy, Phase 4, Harness, Git, PR, and deployment exclusions
remain in force.

## Engineering Summary

Added strict calendar/range helpers with injectable device-boundary conversion,
immutable per-Experience control state, pre-ranking `createdAt` filtering,
explicit-panel blocked/applied routing, a bounded React control, localized
messages, selection/preflight invalidation, UI/unit regressions, and factual
Book One synchronization.

## Behavior Changed

- Inactive historical panels retain Phase 3A behavior.
- The user may enable and Apply both inclusive saved dates.
- Incomplete/invalid/inverted/timezone-unavailable input retrieves nothing and
  never falls back.
- Valid ranges narrow before lexical matching/ranking/caps.
- Exact range/timezone is visible beside lexical relevance.
- Range changes clear affected selection and close affected preflight.
- State remains local and session-only.

## Files Changed

24 authorized paths:

- 4 new product/test files.
- 9 modified product source/test/style files.
- 4 factual Book One architecture/MVP/Roadmap files.
- 7 repository-mediated current workflow artifacts.

No unrelated or forbidden path changed.

## Tests

- Focused: 6 files / 61 tests passed.
- Full Vitest: 24 files / 183 tests passed.
- Workflow: 17/17 passed.
- Rust library: 70/70 passed.
- Backup/restore integration: 12/12 passed.
- Schema contract: 8/8 passed.

## Repository Verification

Canonical command passed with exit code 0:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Typecheck, production frontend build, Rust check, UTF-8, whitespace, secret,
Markdown-link, workflow, and Constitution checks passed.

## Manual Verification

Pending Founder ownership. Required matrix: inactive parity; same/multi-day
inclusive behavior; visible timezone; invalid/inverted fail-closed state;
selection/preflight invalidation; zero results; close/reopen; and three-locale
desktop behavior. No automated evidence is presented as a manual pass.

## Architecture Updates

- architecture/08 version 0.8 records R1's exact local retrieval semantics and
  pending Founder/manual/promotion status.
- architecture/13 version 2.2 records Slice 3B's prior promotion facts.

## ADR Updates

None. No ADR status or decision changed.

## Documentation Synchronization

`docs/11_MVP.md` version 0.7 and `docs/12_Roadmap.md` version 0.6 factually
record the working-tree R1 capability without claiming promotion or completion
of the remaining structured-retrieval categories.

## Data And Migration Impact

No durable data, SQLite, Rust, schema, `user_version`, migration, backup,
restore, import, export, or retention change. Production schema remains v4.

## Provenance And Consent Impact

No provenance or consent policy change. Range metadata is local UI state only.
Selection remains non-consent; affected preflight invalidation requires the
existing exact-content consent path to restart.

## Risks

- Founder desktop review must confirm date-control usability and full preflight
  closure.
- Saved date may differ from event time, so all copy remains explicit.
- Future device timezone changes affect only a future Apply.
- Emotion/relationship/value structured retrieval remains unresolved.

## Deferred Items

- Event-date representation or inference.
- Durable range preferences.
- Emotion, relationship, or value-conflict retrieval.
- Provider/packet/consent changes.
- Schema-v5 activation and later Phase 3C slices.
- Phase 4, Harness expansion, Stage 2/3, PR, and deployment.

## Human Decisions

- Resolved: `PHASE3-RETRIEVAL-R1-001` Option A.
- Pending: Founder diff acceptance, Founder manual UI results, and a separate
  promotion authorization. None is inferred.

## Review Cycles

Cycle 0 passed. No revision cycle was required.

## Workflow Lessons

Product-first progress can use the existing Harness without expanding it.
Separating a strict saved-date domain helper, an explicit-panel constraint, and
a small UI component made fail-closed behavior testable without changing
storage or provider contracts.

## Recommended Next Sprint

First complete Founder manual review and a separately authorized R1 promotion.
After clean promotion, re-audit whether the time-range portion of structured
retrieval is closed. The next bounded product-first candidate is a read-only
provenance/dependency inspector; do not infer authority for it from this sprint.

## Git Status

- Branch: `codex/phase-3-exit-time-range-retrieval-r1`
- HEAD: `5714b3eeeeb4c9612e445dbd78e4f606df41fa27`
- Changes: unstaged only.
- Staged files: none.
- Upstream: none configured.
- Commit/push/merge/PR/deployment: not performed.
