# Current Mission

Status: ready

- Sprint ID: 2026-08-02-phase-3c-slice4c1-confirmed-evidence-lifecycle-design-gate
- Mission title: Phase 3C Slice 4C-1 Confirmed Evidence Lifecycle and Exact Dependent Invalidation Design Gate
- Origin: founder_request
- Base branch: develop
- Starting commit: adb269dc68d6ec3917819e6ea8c18d84cf89e902
- Background: Slice 4B-4 was promoted to develop as private disposable Context Recovery schema-v5 write evidence. Confirmed Evidence correction/deletion and exact dependent invalidation remain an explicitly unimplemented Phase 3C lifecycle gap.
- Problem: ADR-0011 Decisions 6B, 7B, 9A, 10B, 11A, and 12A require an exact transactionally coherent lifecycle. A partial confirmed-Evidence mutation could leave ordinary Reflection/Pattern dependents or ADR-0009 Historical Question actual-use records incorrectly current, retained, or rebound.
- Intended outcome: Correct Slice 4B-4 factual promotion drift, determine whether a complete disposable lifecycle slice is coherent without prior Phase 3B v5 creation parity, and stop at a Founder decision package before any implementation.
- Initial scope: Repository truth inspection; factual architecture/13 synchronization; exact Evidence -> Reflection -> Pattern and Evidence -> Historical Question dependency analysis; transaction, failure, reconciliation, privacy, rollback, and evaluation design; Founder options A-E.
- Explicit non-scope: No implementation; no production schema/user_version 5; no real data or app-data; no runtime/Tauri/UI/startup activation; no provider call or consent change; no Phase 4; no Harness expansion; no Git promotion.
- Relevant Book Zero definitions: We Build Mirrors, Not Oracles; Evidence before Conclusion; Reflection before Answer; Context Before Insight; Memory must be revisable, deletable, provenance-preserving, and user-controlled; AI hypotheses remain uncertain and user-owned.
- Relevant ADRs: ADR-0007, ADR-0009, ADR-0010, ADR-0011, especially ADR-0011 Decisions 6B, 7B, 9A, 10B, 11A, and 12A.
- Relevant architecture documents: architecture/09, architecture/10, architecture/11, architecture/12, and architecture/13.
- Relevant code areas: src-tauri/schema/schema_v5.sql; schema_v5_migration.rs; schema_v5_evidence_write.rs; schema_v5_reflection_write.rs; schema_v5_pattern_write.rs; schema_v5_context_recovery_write.rs; sqlite.rs; schema-v4 storage validation, historical packet persistence, and provenance inspector boundaries.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-01T17:33:52.063Z
