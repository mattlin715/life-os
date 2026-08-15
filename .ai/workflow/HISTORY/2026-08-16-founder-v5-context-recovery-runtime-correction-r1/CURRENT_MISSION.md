# Current Mission

Status: ready

- Sprint ID: 2026-08-16-founder-v5-context-recovery-runtime-correction-r1
- Mission title: Correct isolated Founder v5 Context Recovery runtime routing and local error disclosure
- Origin: founder_resolution
- Base branch: codex/desktop-schema-v5-founder-dogfood-activation-r1
- Starting commit: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Background: Candidate R1 manual Step 11D-3 found a pre-transaction artifact-kind mismatch after the bounded parent sprint reached cycle 3 of 3.
- Problem: The runtime facade looks up Context Recovery using `context_recovery` while persisted authority uses `recovery_turn`; the resulting mutation error is not visible beside the action.
- Intended outcome: Correct only the exact discriminator and local three-language error visibility, verify through the real facade, build an unsigned isolated Candidate package, and return to the affected manual step.
- Initial scope: Runtime discriminator, focused Rust facade regression, focused UI/i18n error disclosure and tests, factual Candidate documentation, workflow evidence, verification, and unsigned package.
- Explicit non-scope: No schema or DDL changes, profile mutation by automation, ordinary profile, real user data, provider, ContextPacket, consent, Phase 4, Android, Harness expansion, distribution, deployment, release, staging, commit, push, merge, or PR.
- Relevant Book Zero definitions: `docs/03_Principles.md`, `docs/06_Memory.md`, `docs/Reflection.md`, `docs/09_AI.md`, `docs/10_Privacy.md`.
- Relevant ADRs: ADR-0009 and ADR-0011.
- Relevant architecture documents: `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`, `docs/architecture/17_Desktop_Schema_v5_Founder_Dogfood_Activation_Candidate_R1.md`.
- Relevant code areas: `src-tauri/src/schema_v5_runtime.rs`, `src-tauri/src/schema_v5_context_recovery_write.rs`, `src/app/App.tsx`, `src/app/i18n.ts`, and focused tests.
- Constraints: Follow AGENTS.md and `.ai/workflow/WORKFLOW.md`; preserve the closed disposable profile until the Founder performs the new packaged manual step.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-15T20:32:19.468Z
- Updated at: 2026-08-16T12:35:00+09:00
