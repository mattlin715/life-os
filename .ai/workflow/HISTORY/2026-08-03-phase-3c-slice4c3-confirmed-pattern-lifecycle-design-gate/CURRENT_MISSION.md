# Current Mission

Status: ready

- Sprint ID: 2026-08-03-phase-3c-slice4c3-confirmed-pattern-lifecycle-design-gate
- Mission title: Design the smallest safe disposable confirmed Pattern correction/deletion lifecycle boundary and prepare Founder authorization.
- Origin: founder_request
- Base branch: develop
- Starting commit: e385ed7f465376dae734afacc08f49ac5532b980
- Background: Slice 4C-2 is promoted, but production schema and startup support remain v4. The promoted private Pattern writer covers disposable candidate creation, confirmation, and rejection only; confirmed Pattern correction and deletion remain absent.
- Problem: A confirmed single-Experience Pattern is a revisable hypothesis, yet the schema-v5 disposable evidence has no bounded lifecycle path for user correction, explicit deletion, exact dependency consequences, rollback, or ambiguous-COMMIT classification.
- Intended outcome: Correct Slice 4C-2 promotion facts, audit all authorized Pattern source and inbound dependencies, and present one exact Founder decision package for a private disposable Slice 4C-3 without implementing it.
- Initial scope: Product Review, dependency authority audit, correction/deletion state models, transaction and evaluation design, alternatives, risks, exact implementation allowlist, and Founder decision `PHASE3C-SLICE4C3-001`.
- Explicit non-scope: No implementation; no production schema/user_version 5; no real user data; no migration or runtime/Tauri/UI activation; no provider, ContextPacket, consent, ADR status, Phase 4, Harness expansion, Git promotion, PR, deployment, or release.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/02_Philosophy.md`; `docs/03_Principles.md`; `docs/05_Identity.md`; `docs/06_Memory.md`; `docs/Reflection.md`; `docs/07_Awareness.md`; `docs/09_AI.md`; `docs/10_Privacy.md`; `docs/appendix/Harness.md`.
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/06_Pattern_Candidate_Boundary.md`; `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`; `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`; `docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md`; `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Relevant code areas: `src/ai/harness/contextPacket.ts`; `src/shared/storage/artifactValidation.ts`; `src-tauri/schema/schema_v5.sql`; `src-tauri/src/schema_v5_pattern_write.rs`; `src-tauri/src/schema_v5_evidence_lifecycle.rs`; `src-tauri/src/schema_v5_reflection_write.rs`; `src-tauri/src/schema_v5_context_recovery_write.rs`; `src-tauri/src/schema_v5_historical_question_write.rs`.
- Constraints: Use the existing repository workflow; apply roles sequentially; distinguish sources from inbound dependents; preserve exact-revision authorship, provenance, correction, deletion, and no-rebinding rules; stop at `human_decision_required` before implementation.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-02T16:07:58.331Z
