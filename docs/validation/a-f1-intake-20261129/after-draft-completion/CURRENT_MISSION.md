# Current Mission

Status: draft

- Sprint ID: a-f1-host-feasibility-20261128
- Mission title: A-F1｜離線合成 host 交易與限定 R1 可行性
- Origin: founder_request
- Base branch: develop
- Starting commit: 0e8e464c2a6057ec557f315571f3e9e0c80f550f
- Background: R3 已獨立 cancelled 並封存，執行 checkout 已驗 idle；完整處置證據 commit 5df116fc8327c184bed057ba0ac54c45d7a5d751。本次 Founder 僅授權 A-F1 官方啟動及 draft intake 記錄。
- Problem: 驗證不削弱 v5 gates 的 A 獨立契約、來源／理解／關聯／receipt 同一 Tx E，以及限定 R1 准入與後驗；本次尚未實作。
- Intended outcome: Mission 目標沿用既定 A-F1 host 可行性；本次交付僅 intake、sequence 1、完整 draft mission 與原始啟動證據，等待產品審閱。
- Initial scope: 既定後續範圍：v5 Migration／DirectFresh 等價 primitives、A 15001 exact verifier／最小 host 操作、Tx P／單一 Tx E、非正文 receipts 及限定 R1 合成驗證。本次只執行官方 start 並補齊 draft mission，不進入下一 phase 或實作。
- Explicit non-scope: 本次不實作產品、不修改治理工具、不重做 R3 處置、不接受 ADR、不重開 A1–A4／Exact02／限定 R1、不執行 Git stage／commit／push；不做 UI、Android 建包／裝置、B/C、真實模型／金鑰／資料、遷移／匯出／同步或 M2-D／036。
- Relevant Book Zero definitions: docs/00_Constitution.md; docs/00_Index.md; docs/02_Philosophy.md; docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md; docs/09_AI.md; docs/10_Privacy.md. No definitions or behavior policies changed.
- Relevant ADRs: docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md; docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md; docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md. ADR-0013 remains Proposed and the R3 schema question remains unresolved; no ADR acceptance is authorized.
- Relevant architecture documents: docs/architecture/01_Local_Evidence_Store.md; docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md; docs/architecture/24_Android_M2C_Offline_Synthetic_Daily_Reflection.md; docs/handoffs/v15-cloud-20261126/02_DECISION_LEDGER.md and 04_CLOUD_ENGINEERING_SCOPE.md; that handoff originals/exact-design-02/01_CANDIDATE_AND_SCHEMA.md, 02_TRANSACTIONS_AND_RECOVERY.md, 03_VERIFICATION_AND_STOP.md, 04_SQLITE_RECOVERY_ADMISSION.md and 05_WRITER_EXTRACTION_PLAN.md.
- Relevant code areas: src-tauri/src/schema_v5_experience_write.rs; src-tauri/src/schema_v5_runtime.rs; src-tauri/src/schema_v5_direct_init.rs; associated origin/readiness/caller regressions. Any future A host adapter belongs to a later authorized engineering phase; no source file is changed by this intake.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-11-28T17:53:52.448Z

## Current authorization and stop

- Founder explicitly authorized this intake in the current chat on 2026-11-29 (Asia/Taipei). The mission ID intentionally retains the approved 20261128 suffix; it is not backdated execution evidence.
- Allowed now: one official start and completion of this draft mission only. This turn does not run record-artifact, transition, product review, engineering planning or implementation.
- Stop: A-F1 intake 已建立，待產品審閱.
- Operational checkout: /workspace/r3-governance-source; actual branch: codex/r3-gov-01-governance-candidate; exact source HEAD: 0e8e464c2a6057ec557f315571f3e9e0c80f550f.
- The inherited Base branch field is integration metadata, not a Git checkout/rebase action. Existing R3 disposition changes are preserved; no Git stage/commit/push is authorized.
- Current mission remains draft and sequence 1. Completed Markdown is not a ready registration, product approval or permission to implement.

## Product-review handoff and existing decisions

- A1–A4, Exact Design 02 and D3 limited R1 remain as already approved. Interpret historical wording in immutable originals through 02_DECISION_LEDGER.md; do not edit or reopen those decisions.
- Existing ENGINEER prerequisites remain: fixed handoff/prototype hashes, actual product walkthrough, and the Founder-confirmed product-understanding receipt. This turn does not claim those product checks were performed; the next product review reconciles their existing evidence.
- R3 governance/disposition review does not replace product-understanding evidence, A-F1 runtime evidence or an implementation grant.

## Proposed engineering acceptance, not executed in this intake

- Preserve v5 Migration/DirectFresh behavior and all exact manifest/readiness/origin gates; verify caller regressions and W-04.
- A uses its independent schema 15001 and exact verifier. Core mutation, A graph/links/delete scrub and receipt transition must share one Tx E; W-01 through W-03 cover rollback and durable-commit-before-ACK readback.
- Tx P records non-body pending metadata only. At most 64 unsettled receipts, at most one pending, no TTL cleanup or payload replay; verify CAS/stale/reentry/delete cleanup and unknown-preserved outcomes.
- R-01 through R-07 require real SQLite journals and cross-process locking. Limited R1 is only for eligible controlled A synthetic originals; SQLite may roll back before final S3 rejection, and those changes must be disclosed and preserved.
- Map remaining A-X cases to host-applicable evidence or explicit not_run. Preserve baseline failures, exact commands/exits, source hashes, tracked patch and every new file in later authorized engineering evidence.
- Product implementation and acceptance cases remain not_run in this intake. Linux workflow checks do not establish Windows canonical, Android/VFS, device-reopen or AI-quality results.

## R3 separation and references

- Complete disposition evidence: 5df116fc8327c184bed057ba0ac54c45d7a5d751; docs/validation/r3-formal-disposition-20261128/INDEX.md and REPORT.md.
- Preserved local archive: .ai/workflow/HISTORY/android-pilot-byo-gemini-r1/; cancelled state/event sequence 57 and both operation receipts remain unchanged.
- The evidence branch intentionally retains the pre-disposition active state at sequence 56. It is a reference branch, not this execution checkout. Environment changes never authorize re-cancellation, re-archival or hand-edited state/events.
- ANDROID-M3-GEMINI-R3-TYPED-MODEL-SCHEMA remains unresolved; ADR-0013 remains Proposed. This A-F1 mission inherits no R3 PASS or restricted capability authority.
