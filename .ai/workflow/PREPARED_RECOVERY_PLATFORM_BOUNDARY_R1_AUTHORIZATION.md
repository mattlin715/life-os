# Prepared Recovery platform boundary R1 — Founder authorization

- Date: 2026/11/03, Asia/Tokyo.
- Sprint ID: 2026-11-03-prepared-recovery-platform-boundary-r1.
- Evidence source: direct Founder messages in the current Codex chat; not inferred from code, CI, silence, or an agent recommendation.
- Decision ID: PREPARED-RECOVERY-PLATFORM-BOUNDARY-R1-001.
- Decision: A, explicitly approved before implementation.

## Exact preceding Founder decision

> 核定 A：將架構19三項explicit recovery及其receipt刪除暫限Windows，非Windows於任何mutation前明確拒絕；保留Mac一般MVP承諾，跨平台recovery另案設計與授權

## Implementation authorization

> 本輪授權：限定實作、synthetic 測試、文件同步、Windows host 驗證、本地 commit、修復分支 push、建立 stacked draft PR，以及查核對應 Ubuntu CI。

The subsequent Founder request titled "Life OS — Prepared Recovery 平台能力收斂 R1" explicitly authorizes bounded implementation, synthetic tests, documentation synchronization, Windows host verification, local commit, normal repair-branch push, a stacked draft PR, and inspection of its Ubuntu CI. The exact scope is the three architecture/19 recovery actions (prepared-v4, v5 sidecars, historical-frontend R2B lifecycle completion including interrupted explicit re-execution) and recovery receipt deletion. Unsupported non-Windows execution must refuse before profile path resolution or I/O; private mutation helpers must also refuse. Structural classification and blocked-state/restore boundaries remain intact.

Windows sharing, LockFileEx, moves, R2B transaction core, DDL, ordinary migration/resume/restore, Reflection, icon assets, dependencies/features, CI workflow, Constitution, Accepted ADR status and old workflow archives are excluded. Minimal localized recovery-only UI clarification is allowed if existing copy is insufficient.

No merge, direct develop push, force/amend/rebase, release/deployment, candidate or installer rebuild, manual app operation, real/stopped profile access, SDK/ADB/emulator action, cleanup or ANDROID-M2D-036 continuation is authorized. Stop with repair PR and CI results awaiting Founder review.

## Provenance and baseline

This is a new dated capability decision, not a statement that Windows-only recovery was always policy. ADR-0006's Mac/Windows MVP direction is unchanged. Cross-platform recovery needs separate design, review and authorization; no new ADR or ADR status change is authorized here.

Verified before creating the isolated worktree: icon source/remote head 0b3987d6b34557939a8c557146d95a2893346a12; develop and PR #1 base e8db8f0f8e59aed7b321a8792a139b53e7328609; PR #1 open/draft, base develop, head codex/ci-tauri-icon-input-r1. Only new branch codex/prepared-recovery-platform-boundary-r1 starts from that exact icon commit. The source path supplied without the dot separator was resolved to the existing C:/Users/a7152/.codex/worktrees/ci-tauri-icon-input-r1/LifeOperatingSystem.
