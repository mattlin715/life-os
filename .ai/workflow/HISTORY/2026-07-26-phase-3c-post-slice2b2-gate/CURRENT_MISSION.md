# Current Mission

Status: ready

- Sprint ID: 2026-07-26-phase-3c-post-slice2b2-gate
- Mission title: Close promoted Slice 2B-2 facts and Founder-gate backup ownership handoff and Windows replacement integration
- Origin: founder_request
- Base branch: develop
- Starting commit: 277c4b5b2031d5bf88dc2a33b03765c103c62629
- Background: Slice 2B-2 is promoted, but architecture/13 still describes it as an unpromoted working tree. Its create-new ownership primitive also pre-creates the backup path, while the promoted Slice 2A `VACUUM INTO` contract requires that destination not exist.
- Problem: Life OS needs a truthful integration design for handing an exclusively claimed but absent backup pathname to SQLite and a Windows replacement boundary with explicit known-failure versus ambiguous-outcome semantics. Existing quiescence evidence is a caller contract, not process-wide proof.
- Intended outcome: Correct only factual Slice 2B-2 promotion drift, audit Slice 2A/2B-1/2B-2 compatibility, and present a bounded Founder decision package for Slice 2B-3 without implementing it.
- Initial scope: Repository evidence review; factual architecture/13 closeout; backup ownership handoff and TOCTOU analysis; Windows `ReplaceFileW` and filesystem durability analysis; compatibility and failure matrices; Product Review; Founder Decision Package; transition to `human_decision_required`.
- Explicit non-scope: No feature branch, production/test implementation, schema v5, `user_version = 5`, real user data, production backup/restore activation, app-data/startup/Tauri/UI integration, retention, autonomous recovery, SQLite sidecar handling, later slices, Phase 4, Harness expansion, Git promotion, PR, or deployment.
- Relevant Book Zero definitions: `docs/00_Constitution.md`, `docs/03_Principles.md`, `docs/06_Memory.md`, `docs/09_AI.md`, `docs/10_Privacy.md`, `docs/12_Roadmap.md`, and `docs/appendix/Harness.md`.
- Relevant ADRs: ADR-0007, ADR-0009, ADR-0010, and ADR-0011.
- Relevant architecture documents: architecture/01, architecture/09, architecture/10, architecture/11, architecture/12, and architecture/13.
- Relevant code areas: `src-tauri/tests/schema_v5_backup.rs`, `src-tauri/src/filesystem_safety.rs`, `src-tauri/src/sqlite.rs`, `src-tauri/src/lib.rs`, and the archived Slice 2A/2B-1/2B-2 workflow evidence.
- Constraints: Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`; preserve schema v4 and local-first user authority; distinguish claimed path ownership from a created file; do not claim unsupported Windows durability or quiescence.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-25T17:30:07.049Z
