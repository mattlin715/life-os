# Current Mission

Status: ready

- Sprint ID: 2026-08-23-ordinary-v5-reflection-double-submit-correction-r1
- Mission title: Correct duplicate Reflection answer submission exposed by Founder Manual Phase A
- Origin: founder_request
- Base branch: develop
- Starting commit: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Background: Founder Manual Phase A Step 8D-2 showed that a rapid second click on Save Answer can run after the first schema-v5 Reflection response has committed. The durable writer correctly refuses the second unchanged correction, but the UI surfaces `reflection_response_unchanged` even though the first save succeeded.
- Problem: The Reflection save action lacks a synchronous per-prompt in-flight guard and durable-current idempotency check, so duplicate user activation can be misreported as a storage failure.
- Intended outcome: One explicit save produces at most one durable Reflection revision, duplicate activation while it is in flight is ignored, an already-equal durable answer is treated as an idempotent no-op, and genuine changed answers retain append-only correction behavior.
- Initial scope: App Reflection save control, a small pure save-intent helper and focused tests, factual architecture/18 synchronization, canonical verification, one rebuilt ignored unsigned ordinary review installer, and resumption only at Manual Phase A Step 8D-2.
- Explicit non-scope: No Rust, schema, DDL, migration, backup/restore, provider, ContextPacket, consent, Phase 4, Android, real-profile access, staging, commit, push, merge, distribution, deployment, or release.
- Relevant Book Zero definitions: docs/03_Principles.md; docs/Reflection.md; docs/06_Memory.md.
- Relevant ADRs: docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md; docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md.
- Relevant architecture documents: docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md; docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md.
- Relevant code areas: src/app/App.tsx; src/app/reflectionDraft.ts; src/app/reflectionDraft.test.ts.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: product_review
- Created at: 2026-08-22T20:11:21.113Z
