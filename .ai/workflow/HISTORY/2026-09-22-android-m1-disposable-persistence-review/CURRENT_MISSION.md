# Current Mission

Status: ready

- Sprint ID: 2026-09-22-android-m1-disposable-persistence-review
- Mission title: Android M1 Architecture Decision and Disposable Persistence Prototype
- Origin: founder_request
- Base branch: develop
- Starting commit: 04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df
- Background: Android M0 proved that an isolated Tauri shell can build and pass the Founder native checklist. Roadmap M1 now needs evidence for architecture, storage authority, and application identity without converting the prototype into M2 production persistence.
- Problem: The repository has no Android app-private SQLite adapter or runnable Android persistence journey. Desktop schema-v5 and Windows recovery guarantees cannot be assumed on Android, and a decision package needs native evidence before Founder acceptance.
- Intended outcome: A trilingual, offline, synthetic-only Android review app using a disposable application identity can explicitly commit an Experience, list and reopen exact text after relaunch, while failing closed on incompatible storage. Produce a concise architecture recommendation and Proposed ADR, automated/native evidence, APK provenance, and a single consolidated Founder diff plus manual UI review gate.
- Initial scope: Investigate M1 architecture choices; implement an experimental app-private fresh-v5 Android adapter and only the create/list/get journey it honestly supports; add bounded fault injection and tests; build a debug APK; exercise a dedicated Android 36 x86_64 disposable AVD; synchronize architecture/runbook/roadmap navigation and Proposed ADR documentation.
- Explicit non-scope: No production architecture acceptance; no full M2 persistence, migration, recovery, retention, import/export, or deletion; no M3 provider, credential, historical-context, consent, AI, Evidence, Pattern, or cloud behavior; no M4 final identity, signing, distribution, synchronization, deployment, or release; no desktop/profile access; no real data; no physical device; no Engineering Harness expansion; no stage, commit, merge, push, PR, archive/reset, or later phase.
- Relevant Book Zero definitions: `docs/01_Vision.md`, `docs/02_Philosophy.md`, `docs/03_Principles.md`, `docs/04_Identity.md`, `docs/06_Memory.md`, `docs/07_Growth.md`, `docs/Reflection.md`, `docs/10_Privacy.md`, routed through `docs/00_Index.md`; preserve mirrors-not-oracles, user authorship, local-first ownership, progressive disclosure, and reversible evidence boundaries.
- Relevant ADRs: `docs/adr/ADR-0004-local-first-persistence-and-user-data-ownership.md`, `ADR-0006-tauri-react-sqlite-desktop-mvp.md`, `ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`, `ADR-0011-append-only-artifact-lifecycle-and-revision-history.md`; a new decision remains Proposed only.
- Relevant architecture documents: `docs/architecture/01_Local_Evidence_Store.md`, `docs/architecture/20_Android_Build_Feasibility_M0.md`, `docs/dev/11_Android_M0_Runbook.md`, `docs/12_Roadmap.md`, and desktop schema-v5 runtime/migration boundaries for contrast rather than copied guarantees.
- Relevant code areas: `src/shared/storage/`, `src/android-m0/`, `src/main.tsx`, `vite.config.ts`, `src-tauri/`, generated Android manifest/Gradle resources, `scripts/android-m0.ps1`, and focused Android contract/native test scripts.
- Constraints: Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`; one writer and sequential roles; app ID separate from M0 and desktop; app-private synthetic-only storage; exact schema-v5 or fail closed with no v4/memory fallback; no INTERNET permission; debug signing; backup/transfer exclusions; bind one exact emulator serial and never a physical phone; distinguish graceful close, force-stop, kill, emulator crash, and unexecuted power loss; automated checks never imply Founder acceptance.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-09-22T01:16:49.299Z
