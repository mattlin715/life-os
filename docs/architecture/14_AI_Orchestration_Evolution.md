---
status: Implemented
version: 0.3
owner: product-and-engineering
last_updated: 2026/07/19
depends:
  - AGENTS.md
  - AI_CONTRIBUTOR_GUIDE.md
  - docs/00_Constitution.md
  - docs/dev/08_Engineering_Harness.md
  - docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md
referenced_by:
  - .ai/README.md
  - .ai/workflow/WORKFLOW.md
---

# 14 AI Orchestration Evolution

## Purpose And Decision State

This document describes how Life OS engineering work can evolve from one
manually triggered Codex App thread to more automated orchestration without
making a vendor, chat session, or hidden agent state the source of governance.

The repository-native Stage 1 structural foundation under `.ai/` is implemented.
After five bounded real sprints, the Founder resolved `HL-001` as `HL-A` on
2026/07/19: Stage 1 is operationally adequate for manually triggered,
repository-mediated Life OS product sprints with Founder checkpoints and one
writable worker. This provides no independent-review assurance and authorizes
neither Stage 2, Stage 3, autonomous orchestration, nor deployment.
Its operational reliability is not yet established and remains gated by the
pilot evaluation below. The foundation coordinates engineering work only; it
does not change Life OS runtime behavior,
Product Harness behavior, provider transport, SQLite data, or user consent.
Stages 2 and 3 are future options and are not implemented or authorized.

No new ADR is required for Stage 1. It is an operational application of Accepted
ADR-0008: canonical engineering governance remains tool-independent and
repository-owned, while tool prompts remain thin adapters.

## Authority And Separation

`AGENTS.md` remains the repository execution entry point. The Engineering
Harness remains the tool-independent execution contract. `.ai/roles/` defines
bounded responsibilities; `.ai/workflow/` holds current sprint state and
handoffs; `.ai/templates/` stabilizes artifact structure; and `.ai/prompts/`
launches a compatible coding tool.

This engineering orchestration is separate from:

- Book Zero theory and the Constitution;
- the Product Harness governing AI-user interaction;
- Life OS runtime storage and personal Memory;
- provider-specific platform controls.

Repository artifacts are workflow truth, but remain below higher authority.
They must not contain secrets, personal journal content, runtime databases, or
provider payloads.

## Implemented Stage 1: Manual Codex App Trigger

One founder-triggered orchestrator thread sequentially applies three roles in
one writable worktree:

1. **Life OS Orchestrator** inspects repository state, maintains the workflow,
   validates evidence, and controls escalation.
2. **Chief Product Theorist** translates the mission into an approved product
   boundary and later reviews the actual diff for theory alignment.
3. **Senior Product Engineer** plans and implements the smallest verifiable
   authorized change.

The handoff path is:

```text
Mission
-> Product Review
-> Engineering Plan
-> Implementation
-> Validation
-> Theory Alignment Review
-> Complete / Revise / Escalate
```

`WORKFLOW_CONTRACT.json` defines the state machine and artifact vocabulary.
Fixed Markdown artifacts preserve human-readable handoffs. `EVENTS.jsonl`
records append-only, hash-chained transitions, while `WORKFLOW_STATE.json`
provides the current projection. `scripts/ai-workflow.mjs` performs guarded
start, artifact recording, transition, founder-decision resolution,
verification recording, status, terminal archive/reset, and validation
operations. The repository verifier and
CI run focused workflow
contract tests and validate the idle or active state.

The current repository HEAD and a deterministic working-tree digest bind review
and verification evidence to the inspected snapshot. Mutable current-sprint
workflow files are excluded from that digest to avoid self-reference; the
workflow contract, role definitions, templates, code, and product diff remain
included. Current-sprint integrity is instead governed by the event chain and
artifact gates. A maximum of three
review cycles prevents unbounded self-correction. Consequential decisions stop
for an exact, scope-bounded founder response before resuming. Completion does
not authorize commit, push, pull request, merge, deployment, or release.

This is sequential role switching, not a multi-writer agent swarm. The existing
one-writable-agent-per-worktree boundary remains unchanged.

Distributed locking and cross-machine leases are not part of Stage 1. Terminal
archive/reset uses a temporary-directory promotion and validated idle reset. On
interruption or state/event mismatch the safe
behavior is fail-closed reconciliation, not autonomous repair.

## Proposed Stage 2: Semi-Automated Event Trigger

Possible future triggers include a GitHub issue or label, a branch or pull
request event, GitHub Actions, or a Codex SDK invocation. Before authorization,
Stage 2 would require:

- an authenticated event-to-mission mapping;
- explicit repository and branch permissions;
- idempotent state updates and interruption recovery;
- observable logs without user data leakage;
- protected-branch and human-approval gates;
- a threat model for prompt injection and untrusted issue content;
- proof that repository artifacts remain portable and canonical.

No GitHub workflow, SDK integration, event listener, or automatic branch action
is added by Stage 1.

## Proposed Stage 3: Persistent Orchestration

Longer-term options could use Codex SDK, Agents SDK, a Codex MCP server, or a
Symphony-style coordinator. This stage would add significant complexity:

- durable leases, concurrency control, and exactly-one writable worker;
- permission scoping and secret isolation;
- event replay, state migration, and crash recovery;
- cost, rate-limit, audit, and observability controls;
- human decision routing and expiry behavior;
- vendor portability and a supported manual fallback;
- security review for autonomous tool use.

The five-sprint Stage 1 evaluation established the bounded manual operating
conclusion above. It did not justify persistent or autonomous orchestration.

## Evolution Gate

The completed five-sprint evaluation is recorded in
`.ai/workflow/WORKFLOW_EVALUATION.md`. Its accepted Stage 1 conclusion does not
enable event-driven work. Any future proposal must still show:

- complete repository-mediated handoffs;
- stable and machine-readable artifacts;
- correct state transitions and bounded retries;
- reliable recovery without hidden chat context;
- proportionate escalation;
- truthful verification and manual-review boundaries;
- continued compatibility with `AGENTS.md`, the Engineering Harness, and
  ADR-0008.

Moving to Stage 2 or 3 requires a separate proposal. A new ADR is warranted only
if that proposal introduces a durable new architecture decision, permission
model, vendor coupling, or autonomous repository authority not already governed
by ADR-0008.

Because the same orchestrator thread sequentially applies all three roles,
Stage 1 provides explicit responsibility separation and traceability, not
independent-review assurance. The pilot must evaluate that limitation honestly.

## Rollback

Stage 1 is non-destructive. Stop using the launch prompt and continue through
`AGENTS.md` and the Engineering Harness directly. Existing `.ai` artifacts can
remain as documentation or be reverted through an ordinary reviewed repository
change. No product data or schema rollback is involved.
