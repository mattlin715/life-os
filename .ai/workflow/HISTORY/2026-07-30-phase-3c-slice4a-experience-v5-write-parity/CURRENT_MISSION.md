# Current Mission

Status: ready

- Sprint ID: 2026-07-30-phase-3c-slice4a-experience-v5-write-parity
- Mission title: Phase 3C Slice 4A Disposable v5 Experience Write Parity
- Origin: founder_request
- Base branch: develop
- Starting commit: cf7633780a1a0a72efcad7558e463ceb094468c4
- Background: Phase 3C Slices 0 through disposable Slice 3B are promoted. The production application remains schema v4. Provenance Inspector P1 was Founder-manually accepted and promoted by feature commit `824a2294f2090c541eff0530063fc0730c18cc63` and non-fast-forward merge `cf7633780a1a0a72efcad7558e463ceb094468c4`.
- Problem: The promoted disposable migration proves an exact v4-to-v5 cutover, but no post-cutover command yet proves that Experience create, correction, deletion, and import keep normalized v5 authority and the v4 compatibility projection atomic and consistent.
- Intended outcome: Correct factual P1 promotion drift and prepare a repository-grounded Founder decision package for the smallest private, unregistered, disposable-fixture Slice 4A Experience write-parity proof. Stop at `human_decision_required`; do not implement without explicit Founder authority.
- Initial scope: Repository and doctrine review; exact proposed Experience transaction contract; alternatives, risks, rollback/restart, reconciliation, automated acceptance matrix, and minimal factual Book One synchronization.
- Explicit non-scope: Production schema-v5 activation; production `SCHEMA_VERSION` or `user_version` changes; real user or app-data databases; Tauri, renderer, UI, startup, provider, ContextPacket, artifact/Reflection/Pattern/Phase 3B write parity; lifecycle UI; export v2; retention; Phase 4; Harness changes; staging, commit, push, merge, PR, deployment, or release.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/03_Principles.md`; `docs/06_Memory.md`; `docs/09_AI.md`; `docs/10_Privacy.md`; preserve user-owned Experience content, local-first control, provenance, correction/deletion, and “We Build Mirrors, Not Oracles.”
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/01_Local_Evidence_Store.md`; `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`; `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`; `docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md`; `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Relevant code areas: `src-tauri/schema/schema_v5.sql`; `src-tauri/tests/schema_v5_contract.rs`; `src-tauri/src/schema_v5_migration.rs`; `src-tauri/src/filesystem_safety.rs`; `src-tauri/src/sqlite.rs`; `src/shared/storage/`; current Experience CRUD/import tests.
- Constraints: Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`; use the repository workflow CLI for state; one writable checkout; no Harness expansion; no implementation before exact Founder approval.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-29T16:05:04.197Z
