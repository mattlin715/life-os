# Current Mission

Status: ready

- Sprint ID: 2026-08-10-phase-3c-database-readiness-inspector-r1
- Mission title: Phase 3C Database Readiness Inspector R1
- Origin: founder_request
- Base branch: develop
- Starting commit: bed87283f9141144a1c11500457363d5a8081aed
- Background: Founder-approved architecture/15 Option A is promoted at bed87283; production schema remains v4.
- Problem: Life OS has extensive private schema-v5 readiness evidence but no explicit-open bounded production-path readiness disclosure.
- Intended outcome: Implement a session-only read-only database readiness inspector with equivalent English, Traditional Chinese, and Japanese disclosure, then stop at Founder diff and manual review.
- Initial scope: Only architecture/15 Option A and its exact maximum allowlist.
- Explicit non-scope: No schema-v5 activation, migration, backup, restore, retention, v5 routing, lifecycle UI, full provenance inspector, export v2, real-user migration test, Phase 4, Harness expansion, Git promotion, deployment, or release.
- Relevant Book Zero definitions: `docs/03_Principles.md` (local-first control, evidence before conclusion), `docs/06_Memory.md` (user-owned and bounded memory), `docs/09_AI.md` (AI authority limits), and `docs/10_Privacy.md` (explicit local control and minimum disclosure).
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`, `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`, and `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`; none grants migration or write authority to this inspector.
- Relevant architecture documents: `docs/architecture/01_Local_Evidence_Store.md`, `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`, `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`, and the Founder-approved `docs/architecture/15_Phase_3C_Production_Readiness_Audit_and_Activation_Gates.md`.
- Relevant code areas: `src-tauri/src/filesystem_safety.rs`, `src-tauri/src/sqlite.rs`, `src-tauri/src/lib.rs`, `src/shared/storage/sqlite/`, `src/app/`, `src/app/i18n.ts`, and `src/styles.css`.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Anticipated file allowlist: the thirteen product/document paths authorized by architecture/15 Option A plus repository-required `.ai/workflow/` artifacts; no other tracked or non-ignored untracked path.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-09T20:27:20.941Z
