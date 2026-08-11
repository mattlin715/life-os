# Current Mission

Status: ready

- Sprint ID: 2026-08-12-historical-timeline-collapsed-focus-visibility
- Mission title: Historical Timeline Collapsed Focus Visibility
- Origin: founder_request
- Base branch: develop
- Starting commit: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Background: Founder narrow-window keyboard review found that repeated collapsed older-Experience summaries do not visibly reveal which row currently has Tab focus.
- Problem: The global external focus outline can be clipped by the disclosure card's `overflow: clip`, making keyboard location ambiguous before Enter.
- Intended outcome: Give the exact focused Experience summary a clear inset focus-visible treatment in collapsed and expanded states.
- Initial scope: CSS presentation, a test-only focus-style contract, three-language label assertions, canonical verification, and Step 13B-R.
- Explicit non-scope: Timeline order/open state, content, persistence, AI, provider, ContextPacket, consent, schema, Phase 4, commit, push, or merge.
- Relevant Book Zero definitions: docs/03_Principles.md and docs/Reflection.md.
- Relevant ADRs: none changed.
- Relevant architecture documents: docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md.
- Relevant code areas: src/styles.css; src/app/i18n.ts; focused test only.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md; preserve all Founder fences.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-12T00:00:00+09:00
