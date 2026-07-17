# Life OS Sprint Workflow

## Purpose

This is the repository-mediated lifecycle for one bounded multi-role sprint. It
controls engineering work; it does not define product theory or store runtime
Life OS data.

`WORKFLOW_CONTRACT.json` is the machine-readable vocabulary and transition
contract. `EVENTS.jsonl` is the append-only current-sprint transition journal.
Use `node scripts/ai-workflow.mjs` for state transitions; do not hand-edit the
JSON state or journal to bypass validation.
The current implementation and future event-driven gate are documented in
[`architecture/14`](../../docs/architecture/14_AI_Orchestration_Evolution.md).

Write the substantive Markdown review first, including its active Sprint ID and
completed required sections. Then use `record-artifact` to reconcile its Status
into state and the event journal before requesting the next transition. Every
mutating command requires the currently observed event sequence so stale work
fails closed.

## State Vocabulary

`WORKFLOW_STATE.json.status` must be one of:

- `idle`
- `intake`
- `product_review`
- `awaiting_product_clarification`
- `engineering_planning`
- `implementation`
- `validation`
- `theory_alignment_review`
- `revision_required`
- `human_decision_required`
- `completed`
- `failed`
- `cancelled`

`current_phase` uses the same phase names, except an idle state may keep
`intake` as the next phase. Artifact statuses are defined by their templates and
must not be substituted for workflow statuses.

## Allowed Transitions

| From | Allowed next state |
| --- | --- |
| `idle` | `intake` |
| `intake` | `product_review`, `human_decision_required`, `cancelled`, `failed` |
| `product_review` | `engineering_planning`, `awaiting_product_clarification`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `awaiting_product_clarification` | `product_review`, `human_decision_required`, `cancelled` |
| `engineering_planning` | `implementation`, `product_review`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `implementation` | `validation`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `validation` | `theory_alignment_review`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `theory_alignment_review` | `completed`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `revision_required` | `product_review`, `engineering_planning`, `implementation`, `human_decision_required`, `failed`, `cancelled` |
| `human_decision_required` | the recorded blocked phase, `cancelled`, or `failed`, after an explicit founder response |
| `completed`, `failed`, `cancelled` | none; `archive` creates the terminal archive and resets a new idle projection |

No transition may bypass the evidence needed by the destination phase. Intake
cannot transition directly to implementation or completion.

## Phase 0: Intake

1. Inspect branch, HEAD, upstream, status, untracked files, recent history, and
   diff.
2. Confirm no unfinished sprint would be overwritten.
3. Create a unique sprint ID and complete `CURRENT_MISSION.md`.
4. Change its Status from `draft` to `ready` only after every `[required]`
   field is resolved, then record it with `workflow:record-artifact`.
5. Record actual repository coordinates in `WORKFLOW_STATE.json`.
6. Route relevant primary definitions, ADRs, architecture, product, development,
   code, and tests.
7. Complete a Product Review for every sprint using this multi-role workflow.
   A routine mechanical change that does not benefit from role-separated review
   may use the ordinary Engineering Harness instead of opening a multi-role
   sprint; once this workflow is started, Product Review is not skipped.

## Phase 1: Product Review

Apply the Chief Product Theorist role and complete `PRODUCT_REVIEW.md`.

- `approved` -> Engineering Planning.
- `approved_with_conditions` -> Engineering Planning with every condition copied
  into the plan.
- `revision_required` -> revise the mission or review.
- `human_decision_required` -> stop and populate `DECISION_REQUIRED.md`.
- `rejected` -> cancel the sprint unless the founder supplies a new mission.

## Phase 2: Engineering Planning

Apply the Senior Product Engineer role and complete `ENGINEERING_PLAN.md` before
editing. The plan must cite the approved product boundary, affected modules,
alternatives, data and lifecycle effects, tests, verification, recovery,
documentation, and escalation result. If implementation would change product
meaning, return to Product Review; if it needs founder authority, stop.

## Phase 3: Implementation

Implement the smallest approved change. Preserve unrelated work, avoid
speculative scaffolding, add focused tests, synchronize factual documentation,
and complete `ENGINEERING_REPORT.md`. Do not merge, push, deploy, or take an
unapproved destructive action.

## Phase 4: Validation

The orchestrator independently inspects the actual diff, test files, command
output, architecture synchronization, ADR implications, migrations, runtime
claims, and git state. Reports must be corrected if they disagree with
repository reality. Run the canonical verification path. Record manual checks as
passed only when actually performed.

## Phase 5: Theory Alignment Review

Apply the Chief Product Theorist role to the actual diff and completed
Engineering Report. Complete `THEORY_ALIGNMENT_REVIEW.md` against acceptance
criteria, Constitution, primary definitions, ADRs, user agency, privacy,
psychological safety, provenance, consent, lifecycle, and scope.

A `rejected` Theory Alignment Review routes to `failed` with evidence, or to
`cancelled` when the founder cancels. It is not silently converted into an
approval or an unbounded revision.

## Phase 6: Bounded Revision

For `revision_required`:

1. increment `review_cycle` exactly once;
2. require a concrete transition reason so the append-only event journal records
   the failed criterion, evidence, owner, and required correction;
3. append the cycle to `THEORY_ALIGNMENT_REVIEW.md` only when the correction
   originated in theory alignment; earlier product or plan corrections remain
   visible in the event journal and final Sprint Report;
4. route to the smallest responsible phase;
5. validate again and repeat theory review.

The maximum is three cycles. A fourth correction attempt is forbidden. If the
remaining issue is consequential or ambiguous, stop at
`human_decision_required`; otherwise mark `failed` with reproducible evidence.

## Phase 7: Completion Or Stop

Complete `SPRINT_REPORT.md`, reconcile the JSON state, and report one terminal
result. `completed_with_follow_up` is a report-level result represented by JSON
status `completed` plus documented deferred items. Completion never implies
commit, push, pull request, merge, deployment, or release.

Run `pnpm workflow:archive` to promote terminal artifacts under
`HISTORY/<sprint-id>/` before resetting the current files to their idle
templates. The archive must exclude secrets and runtime user data. Clear
`DECISION_REQUIRED.md` only after its decision is resolved or the terminal state
records why it remains unresolved.

## Human Decision Package

Every escalation includes observed evidence, affected high-authority sources,
available options, benefits, risks, reversibility, data/privacy impact,
orchestrator recommendation, safe default, blocked files or phases, and the
exact founder response needed. Silence is never approval.

The state records `blocked_phase`, `resume_phase`, active decision IDs, and the
decision-resolution evidence reference. `DECISION_REQUIRED.md` preserves the
exact founder response, selected option, authorized scope, response time, and
resume phase before work continues. Role approval never changes an ADR or
architecture decision state and never grants implementation authority that a
governing document explicitly withheld.

## Control-Plane Persistence

Current `.ai/workflow/` artifacts are tracked repository control-plane metadata,
not product data. The working-tree digest excludes current workflow artifacts to
avoid self-referential hashes, while including mission code, documentation,
roles, templates, contracts, and unrelated untracked files. Intermediate
workflow artifacts may be committed only when the task authorizes a commit.
They never authorize merge, push, deployment, or protected-branch actions.

The event journal uses monotonic sequences, previous-event hashes, idempotency
keys, repository HEAD, branch, and working-tree digest. Terminal archives remain
required before reset and are created through a temporary-directory promotion
plus validated reset. Cross-machine leases remain outside the current kernel
until separately implemented and verified.

## Recovery After Interruption

Treat the repository as authoritative: compare the state file and artifacts with
the actual branch, HEAD, diff, and check output. Resume from the last phase whose
evidence can be independently verified. Record discrepancies rather than
guessing. Never reset, discard, or overwrite unrelated changes to force a clean
state.
