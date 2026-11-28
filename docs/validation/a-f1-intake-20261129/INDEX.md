# A-F1 intake 證據索引

狀態：**A-F1 intake 證據已發布，待產品審閱**。發布不構成 intake 或產品審閱通過；沒有標記 mission ready 或推進 phase。

## 本次發布範圍

- Repository：https://github.com/mattlin715/life-os
- 固定 parent：`0e8e464c2a6057ec557f315571f3e9e0c80f550f`。
- Branch：`codex/a-f1-intake-evidence-20261129`。
- 唯一新增目錄：`docs/validation/a-f1-intake-20261129/`。
- 本次使用獨立發布 checkout，只複製既有檔案並新增索引／完整性清單；沒有重新執行 start 或任何 workflow 命令，也沒有重新產生原始操作輸出或另造大型結果包。
- 正式 `/workspace/r3-governance-source` 的 HEAD、branch、index、既有工作樹與檔案 bytes 保持原狀；仍是 intake／sequence 1／mission draft，產品審閱及工程計畫 pending。
- 本次 Git 發布依 Founder 新的明確證據發布授權。原報告／啟動授權中的「不新增 Git 寫入」描述原 intake 操作範圍，原文沒有因此被改寫。

## 必讀：證據 branch 不是目前執行控制面

本 branch 的 `.ai/workflow/` 與固定 parent 完全相同，活動 state 仍是處置前 R3 `android-pilot-byo-gemini-r1`、human_decision_required、sequence 56。**不能把它當成目前 A-F1 intake。**

真實操作結果透過本目錄 before／after 證據核對；不要將快照直接寫回 state/events，不要在證據 checkout 重跑 start、取消、封存或恢復來追上狀態。

R3 正式處置證據完整 SHA：[`5df116fc8327c184bed057ba0ac54c45d7a5d751`](https://github.com/mattlin715/life-os/blob/5df116fc8327c184bed057ba0ac54c45d7a5d751/docs/validation/r3-formal-disposition-20261128/INDEX.md)。本輪沒有改寫該歷史或重新處置 R3。

## 閱讀入口

| 需求 | 原始記錄／副本 |
| --- | --- |
| 實際 Founder 啟動授權 | [FOUNDER_AUTHORIZATION.md](FOUNDER_AUTHORIZATION.md)；為原 intake 時保存的實際授權文字，非新的 phase／產品實作授權，也非完整聊天匯出或人類身分簽章。 |
| 補齊後 draft mission | [after-draft-completion/CURRENT_MISSION.md](after-draft-completion/CURRENT_MISSION.md) |
| 操作結果報告 | [REPORT.md](REPORT.md)、[RESULT.json](RESULT.json)；兩者均原 bytes 複製。 |
| 啟動前 state／events／mission | [before/WORKFLOW_STATE.json](before/WORKFLOW_STATE.json)、[before/EVENTS.jsonl](before/EVENTS.jsonl)、[before/CURRENT_MISSION.md](before/CURRENT_MISSION.md) |
| 官方 start 後、補齊前 | [after-official-start/WORKFLOW_STATE.json](after-official-start/WORKFLOW_STATE.json)、[after-official-start/EVENTS.jsonl](after-official-start/EVENTS.jsonl)、[after-official-start/CURRENT_MISSION.md](after-official-start/CURRENT_MISSION.md)、[原核對](after-official-start/VERIFICATION.json) |
| 補齊後 state／events | [after-draft-completion/WORKFLOW_STATE.json](after-draft-completion/WORKFLOW_STATE.json)、[after-draft-completion/EVENTS.jsonl](after-draft-completion/EVENTS.jsonl)、[DRAFT_COMPLETION.json](DRAFT_COMPLETION.json) |
| 實際命令與 exits | [COMMANDS.jsonl](COMMANDS.jsonl)；commands/ 內保存完整 argv、時間、raw stdout／stderr 及 hashes。 |
| 原本變更與 hashes | [CHANGES.json](CHANGES.json)、[before/FILE_HASHES.json](before/FILE_HASHES.json)、[after-draft-completion/FILE_HASHES.json](after-draft-completion/FILE_HASHES.json) |
| R3 archive／receipts 不變核對 | [R3_ARCHIVE_RECEIPT_HASHES.json](R3_ARCHIVE_RECEIPT_HASHES.json)、RESULT.json；16 份 R3 專屬檔案及全部 655 份 HISTORY／receipts 的原核對結果保留。 |
| 本次複製與發布檢查 | [COPY_MAP.json](COPY_MAP.json)、[PUBLICATION_CHECKS.json](PUBLICATION_CHECKS.json)；原 `/workspace` 路徑僅供追溯，跨 session 不必存取那些路徑。 |
| 本 commit 完整新增清單 | [FILES.md](FILES.md) |

## 原命令及格式誤判

原始官方 CLI 紀錄共四次，均 exit 0：

1. [00-preflight-validate](commands/00-preflight-validate.json)：只讀 validate。
2. [01-official-start](commands/01-official-start.json)：唯一一次 start，從 idle／sequence 0 到 intake／sequence 1。
3. [02-validate-intake](commands/02-validate-intake.json)：只讀 validate。
4. [03-status-intake](commands/03-status-intake.json)：只讀 status。

每筆的同名 `.stdout`／`.stderr` 原檔都在 commands/；空輸出檔也完整保留。

另有一次 start **之前**的只讀 Python 格式比對誤判，actual exit 1；新摘要多出 index_diff 鍵，整個 dict 比較因此失敗，共有欄位與內容指紋相同。依原記錄修正為共有欄位逐項比較及另驗空 index，之後才執行第一次且唯一一次官方 start，沒有 workflow retry。

[before/PREFLIGHT_CHECK_CORRECTION.json](before/PREFLIGHT_CHECK_CORRECTION.json) 是原本就明確標示的**診斷摘要**，不是 raw error transcript。該次完整 tool error 未另存 raw stdout／stderr 檔；這項缺件如實保留，本次沒有重跑、補造或以新輸出取代它。

## 原件完整性與未驗限制

- 原清單所列 **35 個 payload 缺件為 0、hash 不符為 0**；加上原清單自身共 36 份檔案全部逐位元組複製。
- [ORIGINAL_SHA256SUMS](ORIGINAL_SHA256SUMS) 保留原 SHA256SUMS 的原始 bytes，核對原 35 個 payload。所有相對路徑仍可直接在本目錄找到。
- [SHA256SUMS](SHA256SUMS) 是本次發布清單，核對原件副本與本次索引等新增檔案（不含清單自身）。兩份清單沒有互相取代。
- 可在本目錄使用 `sha256sum -c ORIGINAL_SHA256SUMS` 與 `sha256sum -c SHA256SUMS` 作只讀 bytes 核對，不會執行 workflow。
- 完整格式誤判 raw tool response 未另存成檔，只有上述既有摘要及 chat 來源；沒有冒稱已補齊。
- 產品審閱、工程計畫、產品實作／測試、Windows canonical 及其他平台驗證原本未執行，仍未執行；本次不新增 PASS。
- mission 保持 draft，schema 問題仍未解、ADR-0013 仍 Proposed；沒有修改工具、develop／main、merge／release 或接受 ADR。

停止於 **「A-F1 intake 證據已發布，待產品審閱」**。
