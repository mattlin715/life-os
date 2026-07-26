# Current Mission

Status: ready

- Sprint ID: 2026-07-26-phase-3c-post-slice2b3-gate
- Mission title: Close promoted Slice 2B-3 facts and Founder-gate disposable schema-v5 migration core
- Origin: founder_request
- Base branch: develop
- Starting commit: e932ead6da3346d3783da22dc1d1295c31cdc979
- Background: Phase 3C Slice 2B-3 was promoted through feature commit 61518c7d92b88b9b7d40f2229a30ba9d8370b880 and two-parent non-fast-forward merge e932ead6da3346d3783da22dc1d1295c31cdc979. Production remains schema v4, while architecture/13 contains Founder-approved but separately gated schema-v5 migration and cutover design.
- Problem: Architecture/13 still describes Slice 2B-3 as current-working-tree and unpromoted evidence, and no Founder decision yet authorizes a complete private disposable-fixture migration core. Before implementation, the repository must prove that the fixed DDL has one shared code-consumable source, inventory exact v4 inputs, define honest deterministic backfill and reconciliation, and preserve rollback/recovery boundaries.
- Intended outcome: Minimal factual Slice 2B-3 closeout plus a repository-grounded Product Review and Founder decision package for the smallest internally complete Slice 3A disposable schema-v5 migration transaction, ending at human_decision_required with no migration implementation.
- Initial scope: Inspect doctrine, ADRs, architecture, schema-v5 contract/fixtures/tests, current schema-v4 storage behavior, and archived Slice 2B-3 evidence; update only architecture/13 promotion facts; define DDL source ownership, backfill/reconciliation, failure/restart tests, alternatives, risks, activation blockers, and the exact Founder response.
- Explicit non-scope: No migration module or DDL execution beyond existing tests; no feature branch before authorization; no real user database or app-data path; no production SCHEMA_VERSION 5, startup/Tauri/UI activation, backup/restore/replacement activation, lifecycle writes, export v2, retention, Phase 4, provider/ContextPacket, Harness expansion, Stage 2/3, stage, commit, push, merge, PR, or deployment.
- Relevant Book Zero definitions: docs/00_Constitution.md; docs/06_Memory.md; docs/10_Privacy.md; docs/12_Roadmap.md; docs/appendix/Harness.md.
- Relevant ADRs: ADR-0007 reviewed-artifact provenance; ADR-0009 Historical Question consent/provenance; ADR-0010 Phase 4 user-owned hypothesis and five blocking Phase 3 gaps; ADR-0011 append-only lifecycle and separately gated migration.
- Relevant architecture documents: architecture/01 local evidence store; architecture/11 Phase 3 exit audit; architecture/12 lifecycle/provenance/export foundation; architecture/13 schema-v5 migration/cutover plan.
- Relevant code areas: src-tauri/src/sqlite.rs; src-tauri/src/filesystem_safety.rs; src-tauri/tests/schema_v5_backup.rs; src-tauri/tests/schema_v5_contract.rs; src-tauri/tests/fixtures/schema_v5/.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md; preserve exact authority-state distinctions; use one sequential orchestrator; do not hand-edit workflow state/events; stop before implementation at the Founder decision.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-25T19:16:59.619Z
