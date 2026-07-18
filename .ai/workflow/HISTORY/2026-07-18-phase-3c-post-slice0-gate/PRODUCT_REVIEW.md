# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-18-phase-3c-post-slice0-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 7921affde544a5852aa58782a1acb0a4f189520e
- Working-tree digest reviewed: 3f952218bcf689dadc1ea3fbf45c4e0032ce4f585f48c452c8f7cdb396cf765d
- Created at: 2026-07-18T11:38:35.628Z
- Updated at: 2026-07-18T11:38:35.628Z

## Mission Interpretation

Close Slice 0 truthfully, then ask whether the smallest production foundation may proceed. The product need is database-startup safety before any schema-v5 work, not broader lifecycle capability and not more Engineering Harness development.

## Problem Statement

Slice 0 is promoted and reproducible, but current production startup opens a writable path and executes `CREATE TABLE IF NOT EXISTS experience_entries` before reading `PRAGMA user_version`. It accepts every `user_version >= 4`, so a v4 binary does not refuse a newer database. The renderer constructs the SQLite store during `App` render, and the first refresh reaches cleanup and timeline reads without an explicit compatibility state. Architecture/13 defines this as a cutover prerequisite, but its Slice 1 currently combines startup safety with the much larger typed-mutation parity rewrite. Neither has production authority.

## User Value

Slice 1A would reduce the risk that an older Life OS binary writes to a database created by a newer version. A calm, explicit local compatibility state preserves psychological safety: the application should refuse unsafe writes rather than appearing to start normally and silently damaging provenance. Separating Slice 1A from Slice 1B restores product momentum without authorizing the larger persistence rewrite.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI, Privacy before Profit, and source-of-truth hierarchy require fail-closed behavior rather than hidden data risk.
- `docs/03_Principles.md`: evidence, visible uncertainty, trust, privacy, and user agency apply to local persistence behavior.
- `docs/06_Memory.md`: Memory must remain revisable, consented, provenance-preserving, and user-controlled.
- `docs/Reflection.md`: storage safety cannot turn historical interpretation into automated meaning.
- `docs/09_AI.md`: no AI authority or provider capability is involved in this local startup gate.
- `docs/10_Privacy.md`: transparent local data behavior is part of psychological safety.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: durable artifacts require understandable provenance and lifecycle safety.
- `docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md`: governs this sprint process only; it does not authorize product behavior.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`: existing schema-v4 Historical Question guarantees must remain unchanged.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`: accepts the design direction while explicitly requiring separate implementation authority.

## Current Implementation Context

- **Implemented and promoted:** Slice 0 fixed contract, v2/v3/v4 synthetic fixtures, candidate DDL byte alignment and schema digest, deterministic canonicalization/IDs, and eight test-only Rust integration cases. Feature commit `431c3e7e81b2dcefca873ad3ec1d73680c96d60c` is present through non-fast-forward merge `7921affde544a5852aa58782a1acb0a4f189520e`.
- **Implemented production baseline:** `SCHEMA_VERSION = 4`; production migration writes `PRAGMA user_version = 4`; current v2/v3-to-v4 migrations and schema-v4 storage behavior exist.
- **Observed gap:** `migrate_connection` performs writable DDL before reading the version and returns success for every version at least 4. `App` has no explicit startup compatibility gate.
- **Founder-approved design, not production-authorized:** architecture/13's exact schema-v5 contract and implementation sequence.
- **Proposed by this review:** split the existing Slice 1 plan into bounded Slice 1A startup safety and deferred Slice 1B typed v4 mutation parity.
- **Deferred:** production schema-v5 DDL, migration/cutover, backup/restore, lifecycle writes/UI, export v2, and Phase 4.

## In Scope

Before a Founder decision: factual synchronization in architecture/13, `.ai/workflow/WORKFLOW_EVALUATION.md`, and `docs/dev/08_Engineering_Harness.md`; this review; and the decision package.

If and only if Option A is explicitly authorized later, Slice 1A may:

1. add a Rust-owned pre-initialization inspection that determines database presence and `user_version` before any writable DDL;
2. refuse `user_version > 4`, because 4 is the production binary's supported maximum in this slice;
3. expose an explicit local startup/database compatibility state to the application;
4. prevent store construction, cleanup, timeline reads, and mutations while blocked;
5. preserve current schema-v4 behavior and existing v2/v3-to-v4 compatibility after the safe inspection gate;
6. synchronize `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, and generated lock metadata to application version `0.2.0` if required by the approved plan;
7. use only synthetic or disposable database fixtures for automated and manual verification.

## Out Of Scope

Slice 1B typed Rust mutation commands; schema-v5 DDL or `user_version = 5`; v5 backfill, compatibility triggers, migration receipt, backup, restore, cleanup, lifecycle UI/writes, import/export v2, provider or ContextPacket changes, Phase 4, any Engineering Harness feature, Stage 2/3, staging, commit, push, merge, PR, or deployment.

## Product Constraints

The startup gate must be calm and local. It must not claim data corruption when only incompatibility is known. It must not auto-repair, downgrade, decrement a schema version, select a backup, or run candidate v5 SQL. Browser/in-memory development must remain available without pretending it inspected a SQLite file.

## Evidence And Provenance Constraints

Existing v4 records and Phase 3B provenance must remain byte- and behavior-compatible. Version inspection is compatibility evidence, not permission to reinterpret records. All tests that alter a database must use temporary fixtures; the founder's live database must not be used during automated verification.

## Historical Context Constraints

No historical retrieval, selection, packet, question, dependency, retention, or Phase 3B behavior may change. No Phase 4 conclusion may be generated.

## Consent Constraints

None are newly introduced. ADR-0009 per-generation historical-use consent remains unchanged. Database compatibility inspection is a local startup safety check, not consent to migrate or transmit data.

## AI-Role Constraints

No AI inference, provider call, prompt, output evaluator, identity hypothesis, diagnosis, sensitive inference, or meaning generation is involved.

## Privacy Constraints

Inspection must read only local file/version metadata needed for compatibility and must not log personal content. A refusal state must not expose database content. No backup or duplicate database may be created.

## User-Agency Constraints

Unsafe databases fail closed with a clear state. No automatic upgrade, repair, deletion, downgrade, or hidden retry is allowed. The user keeps the original database unchanged and can exit or use a compatible application.

## Acceptance Criteria

For authorized Slice 1A only:

1. An existing v4 database is inspected before writable initialization and opens with current records unchanged.
2. A disposable database with `user_version = 5` or greater reaches a deterministic blocked state before schema DDL, cleanup, reads, or writes.
3. Blocked startup exposes a typed, local compatibility reason and does not construct the production SQLite store.
4. Fresh/disposable and v2/v3 compatibility paths preserve the existing schema-v4 outcome; no v5 object or `user_version = 5` appears.
5. Failure injection proves inspection/open errors fail closed without partial mutation.
6. Application version declarations are mutually consistent if changed.
7. Existing SQLite tests, Slice 0 contract tests, Vitest, build, typecheck, Rust check, workflow checks, and Constitution check pass.
8. No production user database is opened by automated tests or modified during founder review.
9. English, Traditional Chinese, and Japanese convey equivalent blocked-state meaning if user-facing copy is required.
10. The diff contains no Slice 1B, later-slice, Phase 4, provider, ContextPacket, or Harness expansion.

Founder manual checks after implementation should use disposable app-data/database copies to confirm: normal v4 startup and record preservation; a newer-version refusal with no writes; restart consistency; fresh/v3-to-v4 compatibility without any v5 activation; and equivalent EN/zh-TW/JA refusal meaning if UI copy is present.

## Risks

- Refactoring initialization ordering can regress fresh install or existing v2/v3-to-v4 compatibility.
- Opening SQLite for inspection can itself create or mutate a file unless connection flags and missing-file handling are explicit.
- Updating the application version to `0.2.0` could be mistaken for schema-v5 readiness; documentation and the state contract must say supported schema remains 4.
- A generic storage error would be technically fail-closed but psychologically unclear; the state should distinguish newer-schema incompatibility from ordinary I/O failure.
- Implementing typed mutation parity at the same time would multiply the write surface and weaken reviewability.
- An older already-distributed binary cannot be retrofitted; Slice 1A protects this binary and later versions, while database-level v5 write guards remain a later separately authorized concern.

## Open Questions

none — `PHASE3C-SLICE1A-001` was resolved by the Founder with Option A. Slice 1B remains deferred.

## Human Decision Required

false — `PHASE3C-SLICE1A-001` was resolved by exact Founder response recorded in `DECISION_REQUIRED.md`.

## Recommendation

Proceed with the Founder-authorized Option A only. The Engineering Plan must copy every scope fence and keep Slice 1B deferred until Slice 1A is implemented, verified, manually reviewed where applicable, and separately promoted.

## Review Status

approved_with_conditions
