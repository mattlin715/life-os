# Current Mission

Status: ready

- Sprint ID: 2026-08-01-phase-3c-slice4b4-context-recovery-write-parity-design-gate
- Mission title: Phase 3C Slice 4B-4 Context Recovery Write Parity Design Gate
- Origin: founder_request
- Base branch: develop
- Starting commit: b8205b12a4ac33ef23d20c84d54a2115fdecb830
- Background: Slice 4B-3 private disposable Pattern write parity was Founder-reviewed, canonically verified, and promoted through feature commit `bd1fa7482e8cc52a59518eabca5d4d8c5f541c35` and non-fast-forward merge `b8205b12a4ac33ef23d20c84d54a2115fdecb830`. Production schema and startup support remain v4. Architecture/13 still contains pre-promotion Slice 4B-3 wording, while Context Recovery is the next unimplemented artifact-specific schema-v5 write-parity boundary.
- Problem: The repository has a persisted schema-v4 Context Recovery flow and a fixed schema-v5 lifecycle contract, but no Founder-approved exact v5 typed mutation contract for creating a clarification prompt, saving the first user response, or recording an explicit skip. Reflection semantics cannot be copied silently because Context Recovery is current-Experience supporting context rather than reflective interpretation.
- Intended outcome: Correct only factual Slice 4B-3 promotion drift; establish the repository-grounded Product Review and Local-First Data Architecture package for `PHASE3C-SLICE4B4-001`; compare bounded alternatives; and stop at validated `human_decision_required` before implementation.
- Initial scope: Repository reading, factual architecture/13 synchronization, current-behavior audit, exact private/disposable Slice 4B-4 contract proposal, alternatives, risks, reversibility, automated evaluation matrix, Harness lessons, and Founder decision package.
- Explicit non-scope: No Slice 4B-4 implementation before exact Founder authorization; no production schema/user_version 5, migration/fresh-v5 activation, real-user/app-data access, runtime/UI/Tauri/startup integration, provider/ContextPacket/consent changes, historical transmission, Phase 3B v5 persistence, Phase 4, Harness expansion, Git promotion, deployment, or release.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/02_Philosophy.md` (Context Recovery); `docs/03_Principles.md` (Context Before Insight); `docs/06_Memory.md`; `docs/Reflection.md`; `docs/09_AI.md`; `docs/10_Privacy.md`.
- Relevant ADRs: ADR-0007 reviewed-artifact provenance; ADR-0009 historical-context consent and eligibility; ADR-0010 Cross-Experience/Phase 4 boundary; ADR-0011 append-only lifecycle and purgeable content.
- Relevant architecture documents: `docs/architecture/07_Persisted_Context_Recovery_Vertical_Slice.md`; architecture/11; architecture/12; architecture/13; the fixed schema-v5 DDL and promoted Slice 4A/4B writers.
- Relevant code areas: `src/types/domain.ts`; `src/ai/harness/{contextSufficiency,gateDecision,contextPacket,recoveryTurn,generationSnapshot}.ts`; `src/app/App.tsx`; artifact validation/persistence; `src-tauri/schema/schema_v5.sql`; and `src-tauri/src/schema_v5_{migration,experience_write,evidence_write,reflection_write,pattern_write}.rs`.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-01T11:06:31.322Z
