# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-18-pilot-2-interruption-recovery
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: af2d8736a20cc1820dacc566e25c5accd6b41d59
- Working-tree digest reviewed: bbfef1ccad16efa4b8a4ae327e727f94e81cbe365ed28ebc8a8f64997390c840
- Created at: 2026-07-17T17:09:31Z
- Updated at: 2026-07-17T18:08:53Z

## Mission Interpretation

Use Pilot 2 to test whether Stage 1 can survive loss of chat context and detect
an interrupted control-plane write without guessing or silently repairing it.
Pilot 2A creates the durable repository checkpoint only. A later separately
authorized micro-block must resume from this checkpoint using repository facts.

## Problem Statement

Stage 1 says recovery is repository-authoritative and fail-closed, but current
evidence is mostly contract tests and a same-thread Pilot 1. There is no real
new-context resume record. `appendEventAndState` atomically writes each file but
writes `EVENTS.jsonl` before `WORKFLOW_STATE.json`; a hard interruption between
those writes can therefore leave a detectable event/state disagreement. The
current kernel has validation but intentionally has no autonomous reconciliation
or recovery command.

## User Value

The user is the repository contributor/founder, not a Life OS product user.
Reliable repository-only checkpoints reduce restart anxiety and prevent a model
capacity interruption from forcing an entire sprint to be reconstructed.

## Relevant Primary Definitions

- `docs/00_Constitution.md` remains higher authority and is unchanged.
- `docs/00_Index.md` confirms this is Engineering Harness work, not a new Book
  Zero definition.
- No Product Harness, Memory, Reflection, Evidence, Identity, or consent
  semantics are modified.

## Relevant ADRs

- `docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md`:
  repository artifacts and shared scripts remain canonical across tools.
  Its Accepted decision and status remain unchanged.

## Current Implementation Context

- Implemented: state and event writes use per-file atomic replacement.
- Implemented: event sequence, hashes, idempotency keys, state/event agreement,
  artifact status, role binding, and repository freshness are validated.
- Implemented: rejected commands restore prior state and event contents when
  the process remains alive long enough to run the rollback path.
- Implemented: expected-sequence guards reject stale commands.
- Implemented: terminal archive/reset uses temporary-directory promotion and a
  validated idle reset.
- Verified by tests: artifact-incomplete transitions do not advance state;
  malformed state/artifact combinations fail; stale verification fails;
  decision resume requires exact evidence; archive/reset returns to idle.
- Not operationally demonstrated: a fresh context resuming an actual sprint
  only from repository evidence.
- Not directly regression-tested: event-ahead-of-state and state-ahead-of-event
  torn projections produced as interruption fixtures.
- Not implemented by design: autonomous repair, distributed locks,
  cross-machine leases, event replay, or crash recovery coordination.
- Newly observed in review cycle 1: the terminal `completed` transition hit a
  Windows `EPERM` before replacing `EVENTS.jsonl`. Live events and state stayed
  consistent at revision 13, but `writeAtomic` left its 13,718-byte temporary
  file. The orphan changed the repository digest and correctly made verification
  stale. Its SHA-256 is
  `A57B4ED54C0CDAB7A1E589245B08A55577C8A7671501C2ADC37EA621CD14B0D5`.
- Newly confirmed gap: `writeAtomic` has no failure cleanup, and a failure while
  committing the second member of the event/state pair can leave a torn live
  projection. Existing detection is fail-closed, but write failure handling is
  not yet transactional at the pair boundary.

## In Scope

Pilot 2A:

- Audit the current interruption and recovery contract.
- Create a ready Current Mission and this Product Review.
- Leave a valid repository-native checkpoint at Product Review.

Potential Pilot 2B/C, only after separate authorization:

- Resume from the recorded branch, HEAD, diff, workflow state, event tail, and
  artifacts without relying on chat history.
- Use temporary copied fixtures to exercise event-ahead and state-ahead
  disagreement and prove validation fails closed.
- Add only the smallest test-harness or factual evaluation evidence justified
  by the Engineering Plan.

Review-cycle-1 corrective scope, authorized by the user's instruction to keep
the Harness operating autonomously through read-only subprocess audits:

- Preserve live revision 13 as authority and reject the orphan candidate event.
- After recording its path, size, hash, and content relationship, remove only
  the verified orphan temporary file.
- Modify `scripts/ai-workflow.mjs` only to clean up a temp file when its own
  atomic rename fails and to restore the original event/state pair when either
  pair write fails.
- Add deterministic fault-injection tests for first-write and second-write
  failure, proving original targets remain authoritative, no temp file remains,
  and the copied workflow validates.

## Out Of Scope

- Engineering Planning or implementation during Pilot 2A.
- Editing live `WORKFLOW_STATE.json` or `EVENTS.jsonl` by hand.
- Autonomous repair of unknown mismatches, event replay, a `recover` command,
  or destructive reconciliation. Deterministic rollback of the command's own
  incomplete write to its captured pre-command pair is not autonomous repair
  and is permitted only within the corrective scope above.
- Multi-writer concurrency, leases, Stage 2 triggers, Stage 3 orchestration,
  GitHub automation, SDK integration, or deployment.
- Life OS product/runtime/UI, Product Harness, providers, ContextPacket,
  SQLite, migration, schema, Book Zero, Constitution, or architecture/13.
- Commit, push, merge, or promotion.

## Product Constraints

- Repository evidence outranks chat memory.
- Recovery must resume only from the last independently verified phase.
- A mismatch must stop progress and report exact evidence; silence or a likely
  interpretation must never be treated as reconciliation.
- Capacity-resilient micro-blocks must end with branch, HEAD, workflow revision,
  changed paths, verification, and the next permitted action.

## Evidence And Provenance Constraints

The live event chain remains append-only and may be changed only through
`scripts/ai-workflow.mjs`. Corruption scenarios must exist only in temporary
fixtures. Results must distinguish detection from repair and contract tests
from an actual cross-context resume.

## Historical Context Constraints

None. No Life OS historical content is read, selected, assembled, or sent.

## Consent Constraints

None. No user content or provider operation is involved.

## AI-Role Constraints

The recovering agent may inspect and report repository facts. It may not infer
missing approval, invent a transition, reconstruct an event from chat, or grant
itself Engineering Planning, implementation, promotion, or deployment authority.

## Privacy Constraints

Workflow checkpoints must exclude secrets, credentials, personal journals,
runtime databases, provider payloads, and private user data. Temporary fixtures
must contain synthetic workflow metadata only.

## User-Agency Constraints

The founder controls whether Pilot 2 proceeds beyond this review and separately
controls any later promotion. A successful resume does not authorize Stage 2
or Stage 3.

## Acceptance Criteria

Pilot 2A:

1. Branch begins at promoted `develop` commit `af2d8736` and preserves the
   unrelated untracked architecture/13 file.
2. Workflow state, event chain, mission, and Product Review agree and validate.
3. This review records implemented, tested, unproven, and intentionally absent
   recovery behavior separately.
4. Workflow stops in `product_review`; no Engineering Plan or implementation is
   started.

Later Pilot 2B/C boundary:

5. A new context can recover branch, HEAD, active status, revision, event tail,
   approved review, and next permitted phase without a chat handoff.
6. Event-ahead-of-state and state-ahead-of-event temporary fixtures are both
   rejected with specific validation evidence.
7. The live control plane remains valid and is never used as a corruption
   fixture.
8. No autonomous repair or concurrency mechanism is introduced.
9. Targeted workflow tests and canonical verification pass before completion.
10. Final evidence states that one clean checkpoint recovery does not prove
    arbitrary crash recovery or Stage 2/3 readiness.
11. A failed atomic rename removes only the temp file created by that attempt,
    leaves the target unchanged, and rethrows the original failure.
12. Failure on either event/state pair write restores both captured originals;
    no candidate event is promoted and no orphan temp remains.
13. Fault injection is confined to disposable copied fixtures and never edits
    the live control plane.
14. The verified live orphan is removed only after its evidence is recorded;
    repository validation and canonical verification are rerun afterward.

## Risks

- A same-thread continuation could falsely claim cross-context recovery; the
  next block must begin by deriving facts from repository artifacts and record
  that evidence explicitly.
- Tests could prove only mismatch detection, not safe repair; wording must not
  collapse those claims.
- Deliberately corrupting the live control plane would endanger the sprint;
  all mismatch scenarios must use disposable copied fixtures.
- Expanding into a recovery command or distributed coordination would cross the
  Stage 1 boundary and require a separate design decision.
- Rollback itself may fail under persistent filesystem denial. The correction
  must make a best-effort restore, preserve the primary error, and leave
  validation fail-closed rather than claim guaranteed recovery.

## Open Questions

No founder-controlled product decision is open. The revised Engineering Plan
must define deterministic failure injection without exposing production-only
test hooks or broadening the public workflow API.

## Human Decision Required

False. This review applies the already accepted Engineering Harness boundary.
The founder's current authorization ends after Product Review; proceeding to
Engineering Planning requires a new explicit instruction but not a
constitutional decision package.

## Recommendation

Approve the bounded review-cycle-1 correction. Return through Engineering
Planning before changing the workflow kernel. Detection must remain the response
to unknown mismatch; rollback is limited to the command's own captured
pre-write pair.

## Revision Note

Cycle 1 was opened by real terminal-transition evidence, not a synthetic test.
The failed rename did not advance durable workflow state, but it exposed orphan
cleanup and pair-rollback gaps. This review expands scope only enough to make
the already intended fail-closed write contract reliable.

## Review Status

approved_with_conditions
