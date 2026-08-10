# Current Mission

Status: ready

- Sprint ID: 2026-08-11-daily-reflection-completion-ux-r2
- Mission title: Daily Reflection Completion UX R2
- Origin: founder_request
- Base branch: develop
- Starting commit: c329edb80e0ab3bd5bcbcc4dedd33c9c3af6bf12
- Background: Daily Reflection Core UX R1 was Founder-reviewed and promoted, but the open Experience still presents Evidence, Reflection, and Pattern as three equally expanded work areas. The entry summary names a next step without reliably moving focus to it, and there is no calm record-derived completion surface.
- Problem: The current screen makes the user interpret workflow state across multiple controls. Completed work remains visually heavy, future work competes with the active step, and completion is only a sentence rather than an actionable resting place.
- Intended outcome: Add a deterministic local journey-state resolver and progressive stage presentation that shows one active step, collapses completed stages with truthful summaries, keeps future stages understandable, and offers a calm completion surface plus explicit optional Pattern/history choices without creating new inference.
- Initial scope: Reconcile factual R1 promotion status; implement pure journey-state resolution; progressive Evidence/Reflection/Pattern disclosure; actionable next-step focus; record-only completion view; English, Traditional Chinese, and Japanese parity; focused behavior and regression tests; factual Book One synchronization; canonical verification and Founder manual-review preparation.
- Explicit non-scope: No Constitution or Book Zero changes; no ADR status changes; no schema or migration changes; no provider, ContextPacket, consent, retention, or provenance-policy changes; no Phase 4, historical conclusion, identity inference, Harness expansion, commit, push, merge, PR, deployment, or release.
- Relevant Book Zero definitions: docs/00_Constitution.md; docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md; docs/09_AI.md; docs/10_Privacy.md.
- Relevant ADRs: docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md; docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md; docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md.
- Relevant architecture documents: docs/architecture/08_Local_Historical_Context_Selection_Foundation.md; docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md; docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md.
- Relevant code areas: src/app/App.tsx; src/app/dailyReflectionFlow.ts; src/app/i18n.ts; src/styles.css; existing Evidence, Reflection, Pattern, Context Recovery, and historical-context UI/tests.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md; use existing persisted records only for completion; Pattern remains optional; navigation must never generate; preserve all fail-closed provider and persistence behavior; use at most three bounded review cycles.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-10T21:22:57.821Z
- Updated at: 2026-08-10T21:26:00.000Z
