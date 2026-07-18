# Current Mission

Status: ready

- Sprint ID: 2026-07-18-phase-3c-post-slice0-gate
- Mission title: Close promoted Slice 0 evidence and prepare the minimum Phase 3C production foundation gate
- Origin: founder_request
- Base branch: develop
- Starting commit: 7921affde544a5852aa58782a1acb0a4f189520e
- Background: Phase 3C Slice 0 fixed schema-v5 contracts, synthetic v2/v3/v4 fixtures, test-only DDL execution, digest checks, and eight invariant tests were promoted by feature commit 431c3e7e81b2dcefca873ad3ec1d73680c96d60c and non-fast-forward merge commit 7921affde544a5852aa58782a1acb0a4f189520e. Production remains schema v4.
- Problem: Promoted Slice 0 evidence is factually stale in architecture/13 and Engineering Harness evaluation documents, while Slice 1 combines a minimum startup-safety need with a broader typed-mutation rewrite that still lacks explicit production implementation authority.
- Intended outcome: Correct only factual post-promotion documentation drift, evaluate Slice 1A separately from Slice 1B, present a complete Founder decision package, and stop fail-closed at human_decision_required unless the Founder explicitly authorizes Slice 1A.
- Initial scope: Inspect promoted Slice 0 evidence and current startup/storage paths; update architecture/13, WORKFLOW_EVALUATION.md, and docs/dev/08 only where factual; prepare the Slice 1A/1B decision package; make no production implementation change before an explicit Founder decision.
- Explicit non-scope: Engineering Harness expansion; Stage 2 or Stage 3; schema-v5 DDL execution or user_version 5; user-database migration or mutation; backup, restore, cleanup, Slice 1B, Slices 2-6, Phase 4, lifecycle UI, import/export v2, provider or ContextPacket behavior; staging, commit, push, merge, PR, or deployment.
- Relevant Book Zero definitions: docs/00_Constitution.md (We Build Mirrors, Not Oracles; Human before AI; privacy and source-of-truth hierarchy); docs/03_Principles.md (Evidence before Conclusion; user agency); docs/06_Memory.md (revisable, consented, provenance-preserving memory); docs/Reflection.md (user-owned meaning); docs/09_AI.md (humble mirror); docs/10_Privacy.md (psychological safety and transparency).
- Relevant ADRs: ADR-0007 (reviewed artifacts with provenance); ADR-0008 (tool-independent Engineering Harness); ADR-0009 (Phase 3B historical-use lifecycle remains unchanged); ADR-0011 (append-only lifecycle design accepted but implementation remains separately gated).
- Relevant architecture documents: architecture/01, architecture/09, architecture/10, architecture/12, architecture/13, and architecture/14.
- Relevant code areas: Promoted src-tauri/tests/schema_v5_contract.rs and fixtures; current src-tauri/src/sqlite.rs and src-tauri/src/lib.rs; src/shared/storage/ including sqliteLocalEvidenceStore.ts and mutation paths; src/app/App.tsx startup flow; application version declarations. These are inspected only before Founder authority.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md; use the existing Harness as governance rather than product scope; preserve schema v4 and user data; do not infer approval from prior design approval, Slice 0 promotion, passing tests, or silence.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-18T11:27:02.885Z