---
status: Draft
version: 0.5
owner: product-and-engineering
last_updated: 2026/07/18
depends:
  - AI_CONTRIBUTOR_GUIDE.md
  - docs/00_Constitution.md
  - docs/00_Index.md
  - docs/appendix/Harness.md
  - docs/dev/05_Development_Agent_Runbook.md
  - docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md
referenced_by:
  - AGENTS.md
  - .ai/README.md
  - docs/architecture/14_AI_Orchestration_Evolution.md
---

# 08 Engineering Harness

## Purpose

The Engineering Harness is the thin, executable outer layer that helps contributors change Life OS consistently across coding tools. It protects governed product intent without becoming a second Product Harness or a substitute Constitution.

## Three Harnesses

| Layer | Governs | Canonical location |
| --- | --- | --- |
| Product Harness | How Life OS AI relates to users, context, evidence, provenance, and uncertainty | `docs/appendix/Harness.md`, `src/ai/harness/`, provider and persistence boundaries |
| Engineering Harness | How contributors discover authority, make changes, verify them, and escalate decisions | `AGENTS.md`, this document, `AI_CONTRIBUTOR_GUIDE.md`, `scripts/` |
| Platform Harness | Tool-specific permissions, hooks, skills, memory, and sandbox behavior | Codex, Claude Code, or another platform |

Platform behavior may assist execution. It cannot become the source of Life OS governance.

## Canonical Vendor-Neutral Assets

- `AI_CONTRIBUTOR_GUIDE.md` contains the detailed AI contributor governance.
- `AGENTS.md` is the concise repository entry point and routing contract.
- `docs/dev/` contains durable procedures and operational context.
- `.ai/` contains tool-independent role, workflow, template, and sprint-handoff
  artifacts under this Harness; it is not an authority entry point.
- `scripts/verify.ps1` is the canonical local deterministic verification path.
- `.github/workflows/check.yml` enforces the supported CI subset.
- `CLAUDE.md` is a thin routing adapter only; it does not duplicate policy.

One canonical definition should flow to thin platform adapters and shared scripts. Do not copy substantive rules into every tool directory.

## Contributor Workflow

1. Inspect the current branch, worktree, diff, and untracked files before editing.
2. Follow the task-specific route in `AGENTS.md` and identify governing sources.
3. Preserve existing work and state the feature boundary before implementation.
4. Keep Product Harness behavior, provider transport, and platform behavior separate.
5. Run `scripts/verify.ps1` after deterministic changes; report any local-only limitation.
6. Leave UI and experiential verification to the founder unless explicitly asked to automate it.

## Deterministic Verification

`scripts/verify.ps1` loads the repository-local development environment and runs tests, typecheck, frontend build, SQLite Rust tests, Rust check, staged and unstaged diff whitespace validation, UTF-8/replacement-character validation, tracked/staged/non-ignored-untracked secret-like file checks, local Markdown-link checks, and Constitution-diff reporting. It is read-only except for normal compiler/build output and returns non-zero when a deterministic check fails.

CI mirrors its supported checks: dependency installation, workflow contract
tests and validation, Vitest, typecheck, frontend build, SQLite Rust tests, and
Rust check. It does not receive private journals, local SQLite data, or an LLM
Judge.

## Human Review Boundaries

Only the founder may approve a constitutional change, Book Zero primary
definition, Product Harness behavior policy, ADR acceptance/status change,
implementation authority explicitly withheld by a governing document,
sensitive-inference policy, longitudinal-memory consent, destructive migration
policy, historical provider transmission, a new worldview, or
identity-finalization behavior. Contributors must provide observed evidence,
affected sources, alternatives, risks, a recommendation, and explicit founder
questions instead of making that change.

## Learning And Drift

Corrections can become documented regression cases, procedures, or proposed decisions after human review. Engineering Harness learning never permits an agent to rewrite governance, approve its own behavior, or silently optimize Product Harness prompts. Prefer synthetic or explicitly consented fixtures; never add private founder journal data to CI.

When documents or code disagree, follow the higher authority and record the factual gap. Use an ADR for durable repository or product architecture decisions, not for every routine convention.

## Multi-Agent Boundary

Use one writable agent per worktree. Parallel work requires isolated branches/worktrees; shared-directory writes are unsafe. File ownership may reduce conflict but does not replace isolation. Integrate only after deterministic verification. This foundation does not introduce agent swarms, repository locks, or autonomous merge approval.

The repository-native three-role workflow in [`.ai/`](../../.ai/README.md) is
compatible with this boundary because one writable agent applies the roles
sequentially. The roles create explicit product, implementation, validation, and
theory-review handoffs; they do not create concurrent writers or independent
sources of authority. State and reports must be reconciled with actual git facts
after interruption. `scripts/ai-workflow.mjs` enforces the state contract,
bounded revision count, structured founder-decision pause/resume, artifact
gates, and a hash-chained event journal. `scripts/verify.ps1` and CI exercise
that control plane before the existing repository checks.

## Versioning And Rollback

Version governed documents and prompts where their own boundaries require it. Keep verification scripts deterministic and reviewable. Revert an Engineering Harness policy through a documented repository change or a founder-approved ADR decision; do not silently change tool adapters to bypass it.

## Current Scope And Deferrals

Implemented here: vendor-neutral routing, a Claude adapter, shared verification,
CI alignment, review checkpoints, repository-native sequential role
orchestration, a machine-validated sprint contract, explicit sprint state and
handoffs, append-only transition events, a bounded revision loop, structured
human-decision resume evidence, and a future evaluation boundary. This is a
structural foundation. Four bounded real sprints are archived as operational
evidence, but formal Founder evaluation remains pending. Because one
orchestrator thread applies every role, those sprints are not independent-review
assurance and do not establish readiness for Stage 2 or Stage 3. The durable
repository-architecture decision is
recorded in [`ADR-0008`](../adr/ADR-0008-engineering-harness-governance-is-tool-independent.md),
and the evolution boundary is described in
[`architecture/14`](../architecture/14_AI_Orchestration_Evolution.md).

Deferred: cross-machine leases, tool-specific skill adapters, local hooks, an
automated Evaluation Harness, LLM judging,
automatic correction capture, file locking, concurrent agents, event-driven
triggers, SDK integrations, and autonomous merge approval.
`AGENTS.md`, `.ai/`, `CLAUDE.md`, shared documentation, and scripts are
sufficient for this manual foundation.
