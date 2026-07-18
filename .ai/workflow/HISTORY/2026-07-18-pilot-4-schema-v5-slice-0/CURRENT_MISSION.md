# Current Mission

Status: ready

- Sprint ID: 2026-07-18-pilot-4-schema-v5-slice-0
- Mission title: Recover in-flight Phase 3C Slice 0 into repository-mediated validation
- Origin: explicit_founder_recovery_authorization
- Base branch: develop
- Starting commit: b781071fbf726cde69e93cb9cd98c74abdff0ba3
- Background: architecture/13 was Founder-approved and promoted before this sprint. The founder then authorized Slice 0 test-only implementation. Nine unstaged Slice 0 files or file groups already existed and canonical verification passed before workflow intake, while the repository workflow projection incorrectly remained idle. This sprint begins as a recovery audit of that existing work; it does not claim that intake or planning preceded implementation.
- Problem: The existing Slice 0 work is not yet represented by repository-mediated product review, planning, validation, theory review, CI parity, or archive evidence. Remote CI also still filters Rust tests to `sqlite::tests`, so it does not execute the new `src-tauri/tests/schema_v5_contract.rs` integration suite even though the local canonical verifier does.
- Intended outcome: Preserve and audit the in-flight Slice 0 diff, correct only missing Slice 0 CI/regression/factual-documentation coverage, complete all repository-native workflow phases truthfully, run canonical verification, archive/reset, and stop at Founder diff review without promotion.
- Initial scope: Existing test-only schema-v5 SQL and contract manifest; synthetic v2/v3/v4 fixtures; Rust integration tests; sha2 dev dependency; local/remote verification alignment; only missing Slice 0 regressions; factual architecture/13 synchronization; Pilot 4 workflow artifacts and event-chain evidence.
- Explicit non-scope: No production migration or DDL execution, user database mutation, `SCHEMA_VERSION` or production `user_version` change, startup gate, backup, restore, retention cleanup, UI, import, provider, ContextPacket, Slice 1 or later, Phase 4, Stage 2, Stage 3, deployment, stage, commit, push, merge, autonomous repair, recover command, or event replay. `WORKFLOW_EVALUATION.md` is unchanged except an optional factual Pilot 4 reference, which is not required by this mission.
- Relevant Book Zero definitions: `docs/00_Constitution.md` (Human before AI and documentation hierarchy), `docs/03_Principles.md` (evidence, uncertainty, user agency), `docs/06_Memory.md` (provenance, correction, deletion), `docs/Reflection.md` (reflection remains non-authoritative), and `docs/appendix/Harness.md` (human-reviewed learning cannot authorize itself). No primary definition changes.
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`, `ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`, `ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`, and Engineering Harness `ADR-0008`. ADR-0011 authorizes the lifecycle policy but production migration remains withheld.
- Relevant architecture documents: `docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md` and promoted Founder-approved `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`. Decision 16B authorizes Slice 0 test-only contracts and fixtures only.
- Relevant code areas: `.github/workflows/check.yml`, `scripts/verify.ps1`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tests/schema_v5_contract.rs`, `src-tauri/tests/fixtures/schema_v5/`, and factual architecture/13 evidence text. Production `src-tauri/src/sqlite.rs`, frontend, provider, storage, and ContextPacket code are review-only and must remain unchanged.
- Constraints: Preserve the pre-intake working tree exactly unless a Product Review condition authorizes a bounded corrective edit. Drive `.ai/workflow` only through `scripts/ai-workflow.mjs`; repository artifacts and Git facts are recovery authority. Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`.
- Current owner: orchestrator
- Current phase: intake_recovery_audit
- Created at: 2026-07-18T10:26:00.091Z
