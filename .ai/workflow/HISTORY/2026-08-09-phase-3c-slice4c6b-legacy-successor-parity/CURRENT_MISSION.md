# Current Mission

Status: ready

- Sprint ID: 2026-08-09-phase-3c-slice4c6b-legacy-successor-parity
- Mission title: Phase 3C Slice 4C-6B Migrated Legacy Successor-Producing Current-Action Parity
- Origin: founder_request
- Base branch: develop
- Starting commit: 6f2c64c13a6085e73cc4f4c5dccf6d51a73e543d
- Background: Slice 4C-6A was implemented, canonically verified, Founder-accepted, and promoted through feature commit 5c8c875522abd8e43e790a359b06fbd8f6adb889 and non-fast-forward merge commit 6f2c64c13a6085e73cc4f4c5dccf6d51a73e543d. Production SCHEMA_VERSION and startup support remain 4. The promoted migration core still preserves schema-v4 artifacts as byte-exact legacy-v4-raw baselines.
- Problem: Currently reachable schema-v4 actions still fail closed on migrated legacy baselines when they create a successor or exact user-response fact: pending Evidence correction; suggested Reflection answer/skip; answered Reflection response correction; and suggested Context Recovery answer/skip. Production cutover must remain blocked until disposable evidence proves these actions without rewriting legacy history or inventing provenance.
- Intended outcome: Implement only Founder-authorized Slice 4C-6B in the existing private, unregistered, path/connection-injected writers; preserve immutable legacy predecessors; create canonical successors only for explicit content-changing user actions; prove exact dependency/provenance/projection behavior, rollback and ambiguous-COMMIT classification; synchronize architecture/13; complete Product and Theory Alignment Review; archive/reset; and stop at Founder diff review.
- Initial scope: Current-action reachability matrix; minimal factual Slice 4C-6A promotion correction; legacy pending Evidence correction; legacy suggested Reflection answer/skip; legacy answered Reflection correction because it is reachable in the current UI; legacy suggested Context Recovery answer/skip; focused synthetic/disposable tests migrated through the promoted exact-v4 migration core; Clippy and canonical verification.
- Explicit non-scope: No production schema/user_version 5, real user database or app-data access, migration/fresh-v5 activation, Tauri/renderer/UI/startup registration, provider/ContextPacket/consent changes, new product actions, generic lifecycle framework, backup/restore activation, automatic retry/replay/repair/rebinding, export v2, Phase 4, Constitution/ADR changes, staging, commit, push, merge, PR, deployment, or release.
- Relevant Book Zero definitions: docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md; docs/09_AI.md; docs/10_Privacy.md; docs/appendix/Harness.md.
- Relevant ADRs: docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md; docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md; docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md.
- Relevant architecture documents: docs/architecture/01_Local_Evidence_Store.md; docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md; docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md; docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md; docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md.
- Relevant code areas: src/app/App.tsx current v4 action handlers; src-tauri/src/schema_v5_migration.rs; src-tauri/src/schema_v5_evidence_write.rs; src-tauri/src/schema_v5_reflection_write.rs; src-tauri/src/schema_v5_context_recovery_write.rs; promoted fixture and writer tests; archived Slice 4C-6A workflow evidence.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-09T10:18:55.662Z
