# Current Mission

Status: ready

- Sprint ID: 2026-08-02-phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate
- Mission title: Design the smallest disposable Phase 3B Historical Question schema-v5 creation parity boundary and prepare Founder authorization.
- Origin: founder_request
- Base branch: develop
- Starting commit: 666518eb53ba1cebf64ce87e4add1d2467c8dcbb
- Background: Slice 4C-1 is promoted at merge commit `666518eb53ba1cebf64ce87e4add1d2467c8dcbb`; production schema and startup support remain v4. Phase 3B schema-v4 Historical Question creation is implemented, while the schema-v5 lifecycle projection currently exists only for migrated legacy rows and deletion cascades.
- Problem: A future schema-v5 cutover would lose current Phase 3B write parity because a newly generated Historical Question cannot yet be created atomically in both the authoritative ADR-0009 schema-v4 subsystem and the normalized schema-v5 lifecycle projection.
- Intended outcome: Correct Slice 4C-1 promotion facts and prepare a Founder decision package for the smallest private, unregistered, disposable-only Historical Question schema-v5 creation-parity boundary. Stop before implementation.
- Initial scope: Repository evidence review; exact v4-to-v5 mapping; one-transaction design; duplicate/idempotency, no-question, provider-failure, deletion, rollback, and ambiguity contracts; evaluation matrix; precise implementation allowlist; factual architecture correction; Founder decision gate.
- Explicit non-scope: Production `SCHEMA_VERSION` or `user_version` 5, real user data, migration/fresh-v5 activation, startup/Tauri/renderer/UI, provider calls, ContextPacket or consent-policy changes, Phase 4, Harness expansion, implementation, stage, commit, push, merge, PR, deployment, or release.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/06_Memory.md`; `docs/Reflection.md`; `docs/09_AI.md`; `docs/10_Privacy.md`; `docs/appendix/Harness.md`.
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`; `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`; `docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md`; `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Relevant code areas: `src/historicalContext/governedPacket.ts`; `src/shared/storage/`; `src-tauri/src/sqlite.rs`; `src-tauri/schema/schema_v5.sql`; `src-tauri/src/schema_v5_migration.rs`; promoted schema-v5 write and lifecycle modules.
- Constraints: Follow AGENTS.md and `.ai/workflow/WORKFLOW.md`; use repository-mediated transitions; preserve Phase 3B state separation and exact packet bytes; no implementation before explicit Founder resolution.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-01T20:48:31.990Z
