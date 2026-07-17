# Current Mission

Status: ready

- Sprint ID: 2026-07-18-pilot-1-dev-runbook
- Mission title: Audit and correct the development agent runbook command path
- Origin: founder_request
- Base branch: develop
- Starting commit: d68a24a2b5e91200b6bbbf43c0593f46d2b8879f
- Background: First bounded Stage 1 operational pilot after orchestration promotion.
- Problem: The agent runbook predates the canonical Engineering Harness verifier and the start:desktop helper; audit one command section against current repository evidence.
- Intended outcome: A factual documentation-only correction, or an explicit no-change result, completed through the full repository workflow.
- Initial scope: docs/dev/05 command-path audit plus workflow evidence only.
- Explicit non-scope: No Book Zero, runtime behavior, schema, provider, migration, architecture/13, Stage 2, or Stage 3 changes.
- Relevant Book Zero definitions: No primary definition changes; Constitution authority and `docs/00_Index.md` source-of-truth routing reviewed.
- Relevant ADRs: `docs/adr/ADR-0006-mvp-tech-stack.md`; `docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md`.
- Relevant architecture documents: `docs/architecture/00_MVP_Architecture.md`; `docs/architecture/14_AI_Orchestration_Evolution.md`; `docs/dev/08_Engineering_Harness.md`.
- Relevant code areas: `package.json`; `scripts/use-local-dev-env.ps1`; `scripts/start-life-os.ps1`; `scripts/verify.ps1`.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-17T15:14:55.177Z
