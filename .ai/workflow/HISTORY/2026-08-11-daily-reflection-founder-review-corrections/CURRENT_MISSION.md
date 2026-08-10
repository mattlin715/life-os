# Current Mission

Status: ready

- Sprint ID: 2026-08-11-daily-reflection-founder-review-corrections
- Mission title: Daily Reflection Founder Review Corrections R1
- Origin: founder_request
- Base branch: develop
- Starting commit: f8fbf94812bac2e1359367362e9c1e0db1aeaef1
- Background: Founder manual review of the unpromoted Daily Reflection Core UX R1 found three usability and retrieval-quality defects. The existing R1 work and its archived sprint evidence predate this corrective sprint and must remain intact.
- Problem: Pattern gating does not explain why saved clues and Reflection answers do not replace missing event context; the optional local-history entry point is discoverable only after a long scroll; and adjacent-character CJK lexical reasons expose fragments and generic terms that create noise.
- Intended outcome: Make the missing-context explanation actionable, expose a calm local-history shortcut at the top of every open Experience, and replace fragmentary CJK overlap with bounded meaningful word segmentation and stop-term filtering, then return to Founder manual UI review.
- Initial scope: Reversible UI copy/layout, local-only deterministic retrieval refinement, focused EN/zh-TW/ja tests, factual Book One synchronization, canonical verification, and a running desktop review build.
- Explicit non-scope: No Constitution or Book Zero change; no schema, persistence, provider, ContextPacket structure, consent-policy, retention, Phase 4, Harness expansion, commit, push, merge, PR, deployment, or release.
- Relevant Book Zero definitions: docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md; docs/09_AI.md; docs/10_Privacy.md.
- Relevant ADRs: docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md; docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md.
- Relevant architecture documents: docs/architecture/06_Pattern_Candidate_Boundary.md; docs/architecture/08_Local_Historical_Context_Selection_Foundation.md; docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md.
- Relevant code areas: src/ai/harness/patternAvailability.ts; src/historicalContext/retrieve.ts; src/app/App.tsx; src/app/i18n.ts; src/styles.css and focused tests.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-10T16:53:20.369Z
