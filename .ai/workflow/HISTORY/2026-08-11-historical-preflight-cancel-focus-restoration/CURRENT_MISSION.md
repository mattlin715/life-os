# Current Mission

Status: ready

- Sprint ID: 2026-08-11-historical-preflight-cancel-focus-restoration
- Mission title: Historical Preflight Cancel Focus Restoration
- Origin: founder_request
- Base branch: develop
- Starting commit: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Background: Founder manual Step 10C proved cancellation does not transmit, but removing the tall preflight leaves the viewport on a later Experience instead of restoring orientation.
- Problem: Cancel and adjust-source actions close disclosure without returning focus and scroll to the same Experience's local-history panel.
- Intended outcome: Close only the preflight, preserve all local panel state, and restore keyboard focus and viewport to the exact historical-context panel top.
- Initial scope: App callback wiring, reusable deterministic navigation helper, focused lifecycle and three-language control tests, canonical verification, and Step 10C-R.
- Explicit non-scope: Retrieval, persistence, selection semantics, date-range semantics, consent, provider, ContextPacket, schema, Phase 4, whole-history behavior, commit, push, or merge.
- Relevant Book Zero definitions: docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md.
- Relevant ADRs: docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md.
- Relevant architecture documents: docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md; docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md.
- Relevant code areas: src/app/App.tsx; src/app/journeyNavigation.ts; src/app/HistoricalConsentPreflight.tsx and focused tests.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md; preserve every Founder scope fence.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-11T09:53:04.999Z
