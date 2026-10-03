# Current Mission

Status: ready

- Sprint ID: 2026-10-03-android-m2b-synthetic-experience-lifecycle
- Mission title: Android M2-B Synthetic Experience Edit/Delete Lifecycle
- Origin: founder_request
- Base branch: develop
- Starting commit: aebef185346effda779a6a10850882a2dc19e701
- Background: Clean synchronized develop aebef185346effda779a6a10850882a2dc19e701; validated idle; M2-A acceptance/archive preserved; fresh full canonical baseline PASS (.artifacts/android-m2b/baseline-verification.txt).
- Problem: Android synthetic Experiences can be saved and read but not corrected or explicitly deleted.
- Intended outcome: Exact synthetic create/read/edit/delete journey and disposable native evidence, stopping at consolidated Founder diff and manual UI review.
- Initial scope: ANDROID-M2B-SYNTHETIC-LIFECYCLE-001 authorizes only synthetic edit/delete with the existing canonical v5 lifecycle, dedicated review identity/fixtures/APK, bounded corrections, architecture/runbook/tests.
- Explicit non-scope: No new retention/deletion policy, real data/profile, physical devices, production identity, migration/recovery, provider/network/credentials, sync/import/export/backup, stage/commit/push/merge/archive/reset/PR/release, later M2 or Phase 4.
- Relevant Book Zero definitions: docs/00_Constitution.md; docs/00_Index.md; docs/06_Memory.md; docs/10_Privacy.md; docs/11_MVP.md.
- Relevant ADRs: Accepted ADR-0011 lifecycle policy; Accepted ADR-0012 Android app-private direct-v5 architecture; ADR-0007 provenance and ADR-0009 historical consequences remain unchanged.
- Relevant architecture documents: architecture/01, architecture/12 and promoted Experience writer parity in architecture/13; architecture/22; dev/13.
- Relevant code areas: src-tauri/src/android_m2a.rs; schema_v5_runtime.rs; schema_v5_experience_write.rs; src/android-m2a; Android configuration, packaging/native scripts and contracts.
- Constraints: One writer, sequential repository roles, no Harness expansion. Map every behavior to Accepted policy and existing writer. Stop on policy disagreement. Preserve accepted apps and unknown state. Founder acceptance is not automated.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-10-03T06:58:42.814Z
