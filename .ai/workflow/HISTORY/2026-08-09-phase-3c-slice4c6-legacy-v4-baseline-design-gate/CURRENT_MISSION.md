# Current Mission

Status: ready

- Sprint ID: 2026-08-09-phase-3c-slice4c6-legacy-v4-baseline-design-gate
- Mission title: Define migrated legacy-v4 baseline current-action parity and prepare the Slice 4C-6 Founder gate
- Origin: founder_request
- Base branch: develop
- Starting commit: a9bbf7cc687f0d8dcc4bda9e1d402bf309fac590
- Background: Phase 3C Slice 4C-5 was promoted through feature commit f62050a6b96814376bc35f4924b996537e259bc1 and non-fast-forward merge commit a9bbf7cc687f0d8dcc4bda9e1d402bf309fac590. Private disposable schema-v5 writers now cover canonical current-state and lifecycle behavior, while production SCHEMA_VERSION and startup support remain 4. Migration creates honest byte-exact legacy-v4 baseline revisions that require a separate current-action parity audit.
- Problem: A future verified v4-to-v5 cutover would expose migrated legacy-v4-raw artifacts to actions already reachable in the current UI. Some private writers explicitly refuse those baselines, some appear structurally compatible but lack focused proof, and absent UI actions must not become artificial blockers. The legal matrix must be explicit before implementation or production migration authority is considered.
- Intended outcome: Correct only factual Slice 4C-5 promotion drift; define the exact migrated-state/current-action matrix, parsing-without-rewriting contract, review and successor semantics, genuine cutover blockers, safely deferrable actions, minimum slice sequence, exact proposed file allowlist, lifecycle-parity exit condition, and Founder decision package PHASE3C-SLICE4C6-001; then stop fail closed at human_decision_required.
- Initial scope: Repository evidence review; current production UI/storage reachability audit; schema-v5 migration/backfill and all private writer audits; archived Slice 3A-through-4C-5 evidence reconciliation; minimal factual architecture/13 update; Product Review and Founder decision package only.
- Explicit non-scope: No implementation of legacy actions before Founder approval; no production schema-v5 activation, SCHEMA_VERSION/user_version change, real user data, startup/Tauri/renderer/UI modification, new product action, DDL change, baseline recanonicalization, automatic repair, provider/ContextPacket/consent/retention change, export v2, Phase 4, Harness expansion, staging, commit, push, merge, PR, deployment, or release.
- Relevant Book Zero definitions: docs/02_Philosophy.md; docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md; docs/09_AI.md; docs/10_Privacy.md; docs/appendix/Harness.md.
- Relevant ADRs: docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md; docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md; docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md.
- Relevant architecture documents: docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md; docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md; docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md; docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md.
- Relevant code areas: src/app/App.tsx; src/shared/storage/; src-tauri/src/sqlite.rs; src-tauri/src/schema_v5_migration.rs; src-tauri/src/schema_v5_*_write.rs; src-tauri/schema/schema_v5.sql; schema-v5 contract/integration tests; .ai/workflow/HISTORY Slice 3A through Slice 4C-5 archives.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-08T20:04:15.669Z
