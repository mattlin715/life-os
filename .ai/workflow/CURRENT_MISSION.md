# Current Mission

Status: ready

- Sprint ID: 2026-09-29-android-m2a-direct-fresh-v5
- Mission title: Android M2-A Direct Fresh-v5 Initialization
- Origin: founder_request
- Base branch: develop
- Starting commit: 51dff4998ca8696aeaf1527f058d785303fc0cd4
- Background: Android M1 established a Founder-accepted synthetic-only app-private schema-v5 persistence candidate, but its fresh-install path deliberately creates an empty schema-v4 scaffold and invokes the v4-to-v5 migration. Accepted ADR-0012 selects direct exact-v5 initialization for new Android installations and leaves implementation to M2.
- Problem: The M1 scaffold fabricates migration-shaped origin evidence for a database that has no legacy history. M2-A must prove a truthful direct final-schema bootstrap, exact verification, safe publication, and bounded reopen/write behavior without activating production storage.
- Intended outcome: One unstaged synthetic-only Android M2-A review candidate under temporary identity `com.lifeos.review.m2a`, with direct fresh-v5 initialization, truthful initialization evidence, exact canonical schema parity, fail-closed publication/reopen behavior, automated/native evidence, and a consolidated Founder diff plus manual UI review gate.
- Initial scope: Refactor only the shared canonical final-schema definitions needed by direct bootstrap; implement Android-specific pending/live publication and initialization receipt handling; preserve the M1 trilingual create/save/list/reopen UX, locale preference, icon assets, backup/network exclusions, and create/list/get boundary; add focused host, package, APK, and dedicated-emulator checks; synchronize narrow architecture/runbook/index/roadmap facts.
- Explicit non-scope: No production identity activation; no real profile or desktop database access; no migration, repair, restore, update/delete, import/export, sync, provider/BYOK/network, physical device, signing/distribution/release, M2-B, another M2 slice, or Phase 4. No staging, commit, push, merge, PR, archive/reset, or worktree creation.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; Evidence and Memory routes in `docs/00_Index.md`; user ownership, privacy, provenance, and the rule “We Build Mirrors, Not Oracles.” This sprint stores only synthetic user-authored text and makes no inference.
- Relevant ADRs: Accepted `docs/adr/ADR-0004-local-first-mvp.md`, `ADR-0006-mvp-tech-stack.md`, `ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`, `ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`, and `ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md`. ADR-0012 remains Accepted and unchanged.
- Relevant architecture documents: `docs/architecture/01_Local_Evidence_Store.md`, `13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`, `19_Desktop_Schema_v5_Prepared_State_Recovery_and_Real_Profile_Migration_R2.md`, `21_Android_M1_Disposable_Persistence_Architecture.md`, `docs/dev/12_Android_M1_Disposable_Persistence_Runbook.md`, plus new narrow M2-A architecture/runbook documents using the next available numbers.
- Relevant code areas: `src-tauri/schema/schema_v5.sql`, `src-tauri/src/schema_v5_*`, `src-tauri/src/android_m1.rs` (to be replaced or evolved into M2-A), `src/android-m1/` (preserved behavior with bounded M2-A naming as required), `src/main.tsx`, `src-tauri/gen/android/`, `scripts/android-m1*` (preserved M1 evidence; new M2-A scripts are separate), package scripts, Android contract tests, and workflow artifacts.
- Constraints: Follow AGENTS.md and `.ai/workflow/WORKFLOW.md`; one sequential writer; direct initialization must never invoke `EMPTY_V4_BASE_SCHEMA` or `migrate_disposable_v4`; existing migration verification must remain strict; ambiguous outcomes and pending/database/WAL/SHM states are preserved and refused; no existing M0/M1 installation may be cleared or replaced; manual acceptance remains Founder-owned.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-09-29T09:08:54.307Z
