# Current Mission

Status: ready

- Sprint ID: 2026-07-18-pilot-2-interruption-recovery
- Mission title: Evaluate Stage 1 interruption and recovery through repository-only checkpoints
- Origin: founder_request
- Base branch: develop
- Starting commit: af2d8736a20cc1820dacc566e25c5accd6b41d59
- Background: Pilot 1 proved the basic handoff and exposed active-state test-fixture leakage; Pilot 2 must evaluate recovery without hidden chat context.
- Problem: Stage 1 documents fail-closed recovery, but no real restarted-sprint evidence yet proves repository-only resume or torn-state detection.
- Intended outcome: Produce a repository-grounded audit, a durable mission, and an approved-with-conditions Product Review for a later separately authorized recovery exercise.
- Initial scope: Pilot 2A audit and product boundary only; current workflow artifacts may change.
- Explicit non-scope: No implementation, Engineering Plan, product/runtime/schema/provider changes, architecture/13, Stage 2 or Stage 3, commit, push, merge, deployment, or live control-plane corruption.
- Relevant Book Zero definitions: No Book Zero definition changes; `docs/00_Constitution.md` remains higher authority and the Engineering Harness must not alter the moral relationship between AI and user.
- Relevant ADRs: `docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md` (Accepted; unchanged).
- Relevant architecture documents: `docs/architecture/14_AI_Orchestration_Evolution.md`, `.ai/workflow/WORKFLOW.md`, `.ai/workflow/WORKFLOW_EVALUATION.md`, and `docs/dev/08_Engineering_Harness.md`.
- Relevant code areas: `scripts/ai-workflow.mjs`, `scripts/ai-workflow.node-test.mjs`, `.ai/workflow/WORKFLOW_CONTRACT.json`, `WORKFLOW_STATE.json`, and `EVENTS.jsonl`.
- Constraints: Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`; use capacity-resilient checkpoints; treat repository state as authority; never auto-repair a mismatch; preserve the untracked architecture/13 file; stop after Product Review without Engineering Planning or implementation.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-17T17:08:00.996Z
