# Current Mission

Status: ready

- Sprint ID: 2026-09-20-android-build-feasibility-m0
- Mission title: Android Build Feasibility M0 - Native Review Candidate
- Origin: founder_request
- Base branch: develop
- Starting commit: 1575943094f24bd83c42088fbe4bb1a296083c50
- Background: Desktop schema-v5 R3 recovery, lifecycle activation, Founder Manual Review Steps 1-16, workflow archive, local promotion, remote synchronization, and clean canonical verification are complete at `1575943094f24bd83c42088fbe4bb1a296083c50`; Android work was previously fenced behind those facts.
- Problem: The repository has no Android generated project or native review candidate, while the desktop-default schema-v5 feature, Tauri command registration, frontend storage startup, and provider discovery would be unsafe if reused unchanged on Android.
- Intended outcome: Produce a reproducible, isolated debug Android feasibility APK with application ID `com.lifeos.feasibility.m0`, a trilingual native-launch disclosure and synthetic in-memory preview, exact Android/desktop runtime isolation evidence, and a bounded Founder manual-review runbook.
- Initial scope: Official-document toolchain assessment; Tauri Android initialization; generated Android project; Android-only configuration and feature/target gating; minimal safe-area/narrow-screen/keyboard UI; backup/transfer exclusion; focused tests; APK packaging; manifest/signing/digest inspection; and disposable-emulator smoke only when a specifically identified safe emulator exists.
- Explicit non-scope: Android Personal AI R0; Android SQLite/schema-v5 activation; schema-v4 fallback; desktop migration/recovery changes; mobile BYOK; provider calls; real personal data; Phase 4; M1-M4; production signing, distribution, deployment, release, Git publication, or real-device interaction.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/02_Philosophy.md`; `docs/03_Principles.md`; `docs/06_Memory.md`; `docs/Reflection.md`; `docs/09_AI.md`; `docs/10_Privacy.md`; `docs/appendix/Harness.md`.
- Relevant ADRs: `docs/adr/ADR-0004-local-first-mvp.md`; `docs/adr/ADR-0005-ai-provider-abstraction.md`; `docs/adr/ADR-0006-mvp-tech-stack.md`; `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`; `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`; `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`; `docs/architecture/19_Desktop_Schema_v5_Prepared_State_Recovery_and_Real_Profile_Migration_R2.md`; R3 archive under `.ai/workflow/HISTORY/2026-09-18-desktop-schema-v5-real-profile-post-commit-recovery-r3/`.
- Relevant code areas: `src-tauri/Cargo.toml`; `src-tauri/tauri.conf.json`; `src-tauri/capabilities/`; `src-tauri/src/lib.rs`; `src/shared/storage/createLocalEvidenceStore.ts`; `src/shared/storage/sqlite/founderSchemaV5.ts`; `src/app/App.tsx`; `src/app/i18n.ts`; `src/styles.css`; Android generated/config/build surfaces and focused scripts/tests added by this sprint.
- Constraints: Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`; preserve desktop identity/runtime and Book Zero; use one writable checkout and sequential roles; do not access the desktop profile; do not accept Android licenses automatically, elevate privileges, alter global toolchain variables, operate a physical device, or stage/commit/push/archive this sprint.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-09-20T07:55:31.410Z

## Integration Closeout Authorization (2026-09-22)

`ANDROID-M0-INTEGRATION-CLOSEOUT-001` authorizes only factual
post-publication workflow/evidence reconciliation, the existing CLI terminal
follow-up and archive/reset-to-idle path, an exact archive-prefix compatibility
update where required by an existing package allowlist, narrow closeout
commits, a non-fast-forward merge into `develop`, and normal non-force pushes
of the existing feature branch and `develop`.

This closeout does not change the accepted M0 product behavior. It does not
authorize product implementation, Constitution or Book Zero changes, ADR
status changes, Product Harness changes, profile/database/sidecar access,
migration or recovery, device or emulator operation, distribution, release,
PR creation, or M1 implementation. The accepted APK and native 11/11 evidence
must remain byte-for-byte unchanged.
