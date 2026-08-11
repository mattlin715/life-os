# Current Mission

Status: ready

- Sprint ID: 2026-08-11-daily-reflection-journey-summary-dedup
- Mission title: Daily Reflection Journey Summary Dedup Correction
- Origin: founder_request
- Base branch: develop
- Starting commit: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Background: During the Phase 3 Product Exit Founder walkthrough, the Founder observed that an open Evidence stage repeats the same next-step sentence in its heading summary and inside the actionable card. The existing audit documentation and archive are pre-existing unstaged work and must be preserved.
- Problem: `JourneyStage` always renders its summary, while each open Evidence, Reflection, and Pattern body already renders the detailed next-step guidance. Open stages therefore duplicate localized guidance and increase cognitive load.
- Intended outcome: Show the localized stage summary only while the stage is collapsed; keep the detailed next-step guidance inside an open stage; verify Evidence, Reflection, Pattern, and English/Traditional Chinese/Japanese behavior; return to Step 1R Founder review.
- Initial scope: `src/app/DailyReflectionJourneyView.tsx`, its focused test, and repository-required workflow evidence only.
- Explicit non-scope: No flow, AI, schema, provider, ContextPacket, consent, persistence, retrieval, lifecycle, Phase 4, audit-document, commit, push, or merge change.
- Relevant Book Zero definitions: `docs/03_Principles.md` and `docs/Reflection.md`; reduce friction while preserving Reflection before Answer and user agency.
- Relevant ADRs: ADR-0007 and ADR-0009 remain unchanged; the correction has no persistence or historical-consent effect.
- Relevant architecture documents: Proposed `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md` supplies the current Founder-review context but is not modified by this correction.
- Relevant code areas: `src/app/DailyReflectionJourneyView.tsx`, `src/app/DailyReflectionJourneyView.test.tsx`, and existing localized summary values in `src/app/i18n.ts`.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-11T08:08:01.363Z
