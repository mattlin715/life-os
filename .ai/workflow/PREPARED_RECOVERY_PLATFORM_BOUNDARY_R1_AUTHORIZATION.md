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

## Validation resource decision — 2026/11/03

Decision PREPARED-RECOVERY-PLATFORM-BOUNDARY-R1-VALIDATION-RESOURCE-001 explicitly resolved by the next direct Founder chat message:

> 授權在 D: 使用全新驗證輸出目錄，保留現有證據，以不變的 canonical verifier 重驗相同 source commit

Use fresh D:/Lin/Validation/prepared-recovery-platform-boundary-r1-d-20261103-01 with an exact detached638626a0e0dfbe9fa8c6e108158be26f78e4f4e6 clean clone. No old target/dist/node_modules/ignored assets are copied; installed toolchain/registry reuse only. Fresh Cargo output, TEMP/TMP, dependency install/store and logs are on D. The canonical verifier and source remain unchanged. Preserve all failed C outputs and all worktrees. This authorization is for revalidation, not cleanup, product changes, merge or profile/app/036 operations. Source-specific evidence must not be mislabeled as an exact newer workflow-commit PASS.

## Prepublication and Ubuntu follow-through — 2026/11/03

Evidence source: direct Founder request titled "Life OS — Recovery R1：完成發布前核對與 Ubuntu CI" in the current chat. Exact scope quote:

> 授權：整理既有驗證紀錄、完成範圍內審閱與必要驗證、精確本地提交、正常 push recovery 分支、建立 stacked draft PR，並查核其 Ubuntu CI。

Preserve638626a and230c45e; no amend/rebase/squash. Freeze one final publication HEAD after factual workflow records and bounded alignment review. Existing D PASS applies only to638626a. If formal gate requires fresh final-HEAD canonical, use one adequately provisioned isolated D source/output/TEMP with unchanged verifier and locked dependencies, without old target/dist/ignored asset transplantation. Save exact-HEAD validation and actual Ubuntu results as new ignored evidence; do not add PASS-only commits to cause an endless verification cycle. Later local workflow records must explicitly distinguish the frozen tested/published HEAD and uncommitted post-verification metadata.

Normal recovery-branch push only after local gates; icon remote/base must remain0b3987d6b34557939a8c557146d95a2893346a12. Reuse matching PR if present, otherwise stacked draft base codex/ci-tauri-icon-input-r1, head codex/prepared-recovery-platform-boundary-r1; attach it. Inspect existing pull_request Check run at its actual synthetic merge SHA, all jobs/steps, maximum15 minutes, no blind rerun or background schedule. No PR1 changes, ready/merge/develop push, release/deployment/candidate/app/profile/cleanup/036 operation, Git settings/hook/signing bypass, or new product development is authorized.
