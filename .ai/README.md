# Life OS Multi-Role Sprint Orchestration

`.ai/` contains tool-independent collaboration artifacts under the authority of
`AGENTS.md` and the Engineering Harness. It is not a repository execution entry
point, a Product Harness, or a new source of product truth.

## Authority And Routing

1. Begin with [`AGENTS.md`](../AGENTS.md).
2. Follow the authority order and reading route there.
3. Use [`docs/dev/08_Engineering_Harness.md`](../docs/dev/08_Engineering_Harness.md)
   for the engineering execution contract.
4. Use the role specifications in [`roles/`](roles/) and the lifecycle in
   [`workflow/WORKFLOW.md`](workflow/WORKFLOW.md) for a substantive multi-role
   sprint.

Repository documents, code, git state, and verification output are the evidence
of record. Chat memory is only a convenience and must be revalidated.

## Contents

- `roles/`: bounded responsibilities, authority, and handoff contracts.
- `workflow/`: the current sprint state and review artifacts.
- `templates/`: stable, human-readable schemas used to reset current artifacts.
- `prompts/`: launch prompts for Codex App or another compatible coding tool.

## Safety Boundary

These files contain engineering workflow metadata only. Never place secrets,
credentials, personal journal content, runtime SQLite databases, private user
data, or full provider payloads in `.ai/`. Workflow artifacts do not authorize
constitutional changes, sensitive inference, historical provider transmission,
destructive migration, deployment, merge, or push.

Only one writable agent may operate in a worktree. The three roles are applied
sequentially by that agent; they are not permission to run concurrent writers.

## Start A Sprint

1. Confirm the previous sprint is terminal and archived.
2. Run `pnpm workflow:start -- --sprint-id <id> --mission-title <title>`.
3. Complete each required Markdown artifact, record its declared status with
   `pnpm workflow:record-artifact -- <options>`, then use
   `pnpm workflow:transition -- <options>`. Both require the observed
   `--expected-sequence <n>`.
4. Run `pnpm workflow:status` and `pnpm workflow:validate` at handoffs.
5. Record founder decisions with `pnpm workflow:resolve-decision -- <options>`;
   include `--expected-sequence <n>`. An approval in chat is not workflow
   evidence until its exact scope is recorded.
6. Stop at `human_decision_required` for consequential founder decisions.
7. After a terminal report, run `pnpm workflow:archive` to create the sprint
   archive and reset the current control plane to validated idle state.

`WORKFLOW_CONTRACT.json` is the machine-readable state contract and
`EVENTS.jsonl` is its append-only, hash-chained transition journal. Do not edit
state or event files by hand during a sprint. The canonical repository verifier
also runs the workflow tests and validator.

Stage 1 is a structurally implemented manual-trigger foundation. Its operational
reliability is deliberately `not_started` until three to five bounded pilot
sprints have produced recoverable evidence. See
[`architecture/14`](../docs/architecture/14_AI_Orchestration_Evolution.md).

The current files intentionally begin in an idle state. They are reusable
workflow control surfaces, not evidence that a sprint has been completed.
