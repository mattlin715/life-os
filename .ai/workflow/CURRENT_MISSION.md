# Current Mission

Status: ready

- Sprint ID: android-pilot-byo-gemini-r1
- Mission title: Android Pilot settings and one Gemini question
- Origin: founder_request
- Base branch: develop
- Starting commit: 869a25e217cb1297bd0569964b2b69b56fe9650d
- Background: Synthetic offline Android Pilot; Founder requested same-screen live trial on Zenfone 10.
- Problem: Desktop provider sends more context than the trial permits; Android has no BYOK path.
- Intended outcome: One explicitly confirmed synthetic line of evidence yields one optional, unsaved reflection question.
- Initial scope: Source implementation and mock host verification; optional connected Pilot variant; session-only key.
- Explicit non-scope: No live call, build, signing, install, history, real data, desktop changes, sync, distribution or commit.
- Relevant Book Zero definitions: Constitution, Reflection, Privacy, Memory, AI, Harness.
- Relevant ADRs: ADR-0005, ADR-0007, ADR-0009, ADR-0012 (no status changes).
- Relevant architecture documents: Android M2-C architecture 24 and runbook 15, existing offline evidence remains unchanged.
- Relevant code areas: Android Pilot renderer, isolated native provider adapter and explicit manifest overlay.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-11-14T17:26:26.627Z
