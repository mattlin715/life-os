# Workflow Evaluation

Status: founder_evaluation_pending

Evaluate this repository-native operating model after three to five real,
bounded Codex App sprints. Do not infer readiness from the scaffolding alone.

## Current Evidence State

Four bounded real sprints are archived under `.ai/workflow/HISTORY/`: Pilot 1
through Pilot 3 exercised runbook correction, interruption recovery, and an
exact human-decision pause/resume; Pilot 4 used the same workflow for the real
Phase 3C Slice 0 product contract sprint. This is operational evidence, not a
formal reliability approval.

Formal Founder evaluation remains pending. Stage 1 has not been independently
reviewed, because one orchestrator thread applies all roles, and this evidence
does not establish that Stage 2 or Stage 3 is ready.

## Evaluation Matrix

| Area | Evidence to inspect | Pass condition |
| --- | --- | --- |
| Handoff completeness | Current and archived artifacts | Required fields are present and the next role does not rely on chat-only context. |
| Responsibility separation | Product Review, plan, report, theory review | Product meaning, implementation, validation, and final alignment are explicitly role-separated. This is not independent-review assurance because one orchestrator thread applies the roles. |
| Escalation quality | `DECISION_REQUIRED.md` cases | Consequential decisions stop; routine reversible choices do not create noise. |
| State correctness | `EVENTS.jsonl`, state revision, hashes, and timestamps | Every transition is allowed, hash-chained, recoverable, and agrees with repository reality. |
| Bounded revision | Event journal and originating review artifact | At most three cycles; each has new evidence and an explicit owner. |
| Interruption recovery | Restarted sprint | A new thread resumes from artifacts and git facts without hidden chat memory. |
| Verification reliability | Reports versus command output | No failed or skipped check is represented as passed. |
| Documentation drift | Links and authority citations | `AGENTS.md`, Engineering Harness, ADR-0008, roles, and workflow remain aligned. |
| Machine readability | JSON parse and fixed headings | State parses; vocabulary is stable; required headings can be located reliably. |
| Data safety | Workflow and history content | No secrets, journals, runtime databases, or provider payloads are stored. |

## Readiness Questions

Before considering event-driven orchestration, confirm that multiple completed
sprints show reliable state recovery, appropriate escalation, stable artifact
schemas, bounded retries, truthful verification, explicit founder decisions, and
no dependence on one platform's hidden state.

## Recommended First Pilot

Use a low-risk, documentation-only mission: audit one factual cross-link or
development command in `docs/dev/`, correct it if stale, and run canonical
verification. Exclude Constitution or Book Zero changes, historical-context
behavior, migrations, cloud features, provider contracts, and architectural
replacement. This exercises every handoff without risking product data.
