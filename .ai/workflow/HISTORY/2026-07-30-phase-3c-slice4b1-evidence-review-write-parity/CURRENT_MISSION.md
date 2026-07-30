# Current Mission

Status: ready

- Sprint ID: 2026-07-30-phase-3c-slice4b1-evidence-review-write-parity
- Mission title: Founder-gated disposable Evidence candidate and review write parity
- Origin: founder_request
- Base branch: develop
- Starting commit: c7fc0c7a61d3b4f44237a83bf8288d1a7d8ae4ca
- Background: Phase 3C through disposable Slice 4A is promoted. Slice 4A proves private exact-v5 Experience current-state write parity against synthetic/disposable fixtures, while production `SCHEMA_VERSION` and startup support remain 4.
- Problem: No post-cutover command yet proves that ordinary Evidence candidate creation, pending correction, explicit confirmation, and rejection keep normalized v5 authority, immutable authorship/provenance/review history, exact source dependencies, and the v4 compatibility projection atomic and consistent.
- Intended outcome: Correct factual Slice 4A promotion drift and prepare a repository-grounded Founder decision package for the smallest private, unregistered, disposable-fixture Evidence candidate/review parity proof. Stop at `human_decision_required`; do not implement without explicit Founder authority.
- Initial scope: Repository and doctrine review; exact proposed Evidence transaction contract; authorship, provenance, review, rejection purge, dependency, guard, rollback/restart and reconciliation rules; alternatives, risks, automated acceptance matrix; minimal factual Book One synchronization.
- Explicit non-scope: Production schema/user-version 5; real user or app-data databases; Tauri, renderer, UI, startup, whole-bundle artifact replacement, confirmed-Evidence post-review correction/deletion, Reflection, Pattern, Context Recovery, Historical Question or Phase 3B v5 writes, retention, export v2, production recovery, provider or ContextPacket changes, Phase 4, Harness changes, staging, commit, push, merge, PR, deployment, or release.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/03_Principles.md`; `docs/06_Memory.md`; `docs/Reflection.md`; `docs/09_AI.md`; `docs/10_Privacy.md`; `docs/appendix/Harness.md`. Preserve “We Build Mirrors, Not Oracles,” Evidence as a reviewable AI hypothesis rather than truth, explicit user review, local-first control, provenance, correction, deletion, and silence as no decision.
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/01_Local_Evidence_Store.md`; `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`; `docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md`; `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Relevant code areas: `src-tauri/schema/schema_v5.sql`; `src-tauri/tests/schema_v5_contract.rs`; `src-tauri/src/schema_v5_migration.rs`; `src-tauri/src/schema_v5_experience_write.rs`; `src-tauri/src/sqlite.rs`; `src/types/domain.ts`; `src/shared/storage/artifactValidation.ts`; `src/shared/storage/artifactMutation.ts`; current Evidence provider/App flows; `src/historicalContext/`.
- Constraints: Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`; use the repository workflow CLI for state; one writable checkout; no Harness expansion; no implementation before exact Founder approval; preserve all explicit authority fences.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-29T18:28:46.017Z
