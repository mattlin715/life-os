# Current Mission

Status: ready

- Sprint ID: 2026-08-04-phase-3c-slice4c4-answered-reflection-lifecycle-design-gate
- Mission title: Design the smallest safe answered Reflection correction/deletion lifecycle and exact dependent consequences, then obtain Founder authorization.
- Origin: founder_request
- Base branch: develop
- Starting commit: 7c09dd7d008773e157e7da38de2263661d91307a
- Background: Disposable Slice 4C-3 confirmed-Pattern lifecycle evidence was Founder-reviewed and promoted at merge commit 7c09dd7d008773e157e7da38de2263661d91307a, while production schema and startup support remain v4. The promoted Reflection writer supports answered-response correction only when there are no inbound dependents and does not support explicit answered-Reflection deletion.
- Problem: Correcting or deleting an answered Reflection without exact dependent handling could leave ordinary Pattern hypotheses eligible against a superseded or deleted response, or retain ADR-0009 Historical Questions and exact packet snapshots whose disclosed source is no longer current or has been deleted.
- Intended outcome: Correct Slice 4C-3 post-promotion wording factually, audit the complete current Reflection dependency graph, and present PHASE3C-SLICE4C4-001 with the smallest safe private disposable lifecycle options and an exact Founder authorization boundary; stop before implementation.
- Initial scope: Product Review, dependency inventory, lifecycle transition contract, evaluation matrix, alternatives, risks, exact prospective file allowlist, and Founder decision package for answered Reflection correction/deletion and their exact Pattern and ADR-0009 consequences.
- Explicit non-scope: No implementation; no production schema/user_version 5, migration/fresh-v5 activation, real user data or app-data, runtime/Tauri/renderer/UI/startup, new prompt generation, Evidence or historical eligibility changes, provider/ContextPacket/consent/retention changes, Pattern regeneration or rebinding, Phase 4, generic future-dependent infrastructure, backup/restore/recovery, export v2, Harness expansion, Git staging/commit/push/merge, PR, deployment, or release.
- Relevant Book Zero definitions: docs/00_Constitution.md; docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md; docs/09_AI.md; docs/10_Privacy.md; docs/appendix/Harness.md.
- Relevant ADRs: ADR-0007 reviewed-artifact provenance; ADR-0009 exact historical consent/provenance and source-change cascade; ADR-0011 append-only lifecycle, exact dependencies, correction, invalidation, deletion, and tombstones.
- Relevant architecture documents: docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md; docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md; docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md; docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md.
- Relevant code areas: src-tauri/src/schema_v5_reflection_write.rs; src-tauri/src/schema_v5_pattern_write.rs; src-tauri/src/schema_v5_evidence_lifecycle.rs; src-tauri/src/schema_v5_historical_question_write.rs; src-tauri/src/schema_v5_migration.rs; src-tauri/schema/schema_v5.sql; promoted disposable tests embedded in those modules.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-04T11:45:08.461Z
