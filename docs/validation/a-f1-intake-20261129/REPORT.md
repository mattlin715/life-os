# A-F1 intake 已建立，待產品審閱

2026-11-29（Asia/Taipei）。依 Founder 本次最小啟動授權完成一次官方 start，並補齊 draft mission。僅停在 intake；沒有進行產品審閱、工程計畫或實作。

## 固定來源與保全

- 執行 checkout：`/workspace/r3-governance-source`。
- 精確工具 HEAD：`0e8e464c2a6057ec557f315571f3e9e0c80f550f`。
- Branch：`codex/r3-gov-01-governance-candidate`，沒有切換／建立 Git branch。
- R3 正式處置證據：`5df116fc8327c184bed057ba0ac54c45d7a5d751`，`docs/validation/r3-formal-disposition-20261128/INDEX.md`。
- 既有正式 R3 處置變更保留。啟動前 HEAD／branch／index／工作樹／1163 份檔案內容指紋的共有欄位與原證據發布時摘要一致；來源六份 hashes 相符。
- 啟動前是 idle、sequence 0、空 EVENTS；mission ID 查重通過，無 pending／preparing。當下未觀察到其他候選 cwd 程序或開啟寫入 descriptor；只聲明本機觀察，沒有跨機 lease 保證。
- 證據 branch 的活動控制面仍是 R3 sequence 56；它沒有被當成執行 checkout。沒有重做取消、封存、恢復或手改 state/events。

## 實際啟動

Mission ID：`a-f1-host-feasibility-20261128`。日期後綴按 Founder 明確指定保留；實際本輪日期為 2026-11-29，未回填／偽造執行時間。

Mission title：`A-F1｜離線合成 host 交易與限定 R1 可行性`。

```text
node scripts/ai-workflow.mjs start --root /workspace/r3-governance-source --sprint-id a-f1-host-feasibility-20261128 --mission-title "A-F1｜離線合成 host 交易與限定 R1 可行性" --background <本輪實際背景> --problem <已核定問題> --outcome <本次 intake 交付> --scope <既有範圍及本次最小授權> --non-scope <明確非範圍>
```

`commands/01-official-start.json` 保存完整實際 argv（非上列說明占位）、cwd、UTC 時間、actual return code 及原始 stdout／stderr hashes。對應 `.stdout`／`.stderr` 原始檔也保留。

- 官方 start **只執行一次，exit 0**。
- 原狀態：idle、sprint_id=null、sequence 0、0 events。
- 新狀態：intake、sequence **1**、1 event、current_mission=`draft`。
- Event ID：`a-f1-host-feasibility-20261128:0001`。
- Event hash：`64b77b2e374391fab9dc2a66342eedadc96169f18e25c4458b25d25efe25f153`。
- 唯一事件為合法 `idle → intake` transition；previous_event_hash=null，event/state 對應。
- repository_verify 與 manual_ui 都仍 `not_run`；沒有繼承 R3 PASS、決策解決或產品權限。

## Draft mission 補齊

僅依本次授權編輯 CURRENT_MISSION.md，填齊四個路由欄位及必要說明：Book Zero、ADRs、架構／handoff、相關 code areas；並保留既定範圍、驗收要求、原型理解前置、R3 分界及停止點。

- 無 `[required]` placeholder；Status 保持 **draft**。
- 明列本輪只允許 start／draft 記錄，未執行產品審閱、工程計畫或實作。
- 沿用 A1–A4、Exact02、D3 限定 R1，沒有重開設計決策。產品理解與 walkthrough 前置由下一產品審閱依既有材料核對，本輪不宣稱已完成。
- 沒有呼叫 record-artifact 或 transition；沒有標記 ready。
- 補齊 Markdown 前後，WORKFLOW_STATE.json 與 EVENTS.jsonl **bytes 完全相同**，因此仍是 sequence 1／單一事件。

生成的 draft 在 `after-official-start/`，補齊後的版本在 `after-draft-completion/`；兩階段各保存 state、events 與 mission 原始 bytes。`DRAFT_COMPLETION.json` 記錄 hashes 及授權編輯範圍。

## 實際檢查結果

| 命令／檢查 | actual 結果 |
| --- | --- |
| 00-preflight-validate：官方 validate | exit 0，啟動前 idle 結構有效。 |
| 01-official-start：官方 start | exit 0；只執行一次。 |
| 02-validate-intake：官方 validate | exit 0。 |
| 03-status-intake：官方 status | exit 0；intake／sequence 1。 |
| 全部非忽略檔案 before／after 比較 | 同一 1163 檔集合，僅授權三檔 bytes 改變；其餘 1160 檔不變。 |
| R3 archive／兩份 receipts | 本次 R3 archive 14 檔與兩份 operation records 共 16 檔 hashes 不變；全部 HISTORY／receipts 655 檔亦不變。 |
| 工具、產品來源、ADRs | 未變；ADR-0013 仍 Proposed，原 R3 schema 問題仍未解。 |
| Git HEAD／branch／index | 未變；index hash 仍 `d2ceef7b81c0ba078a450407125cfd575afa01e71e73c4de2bf4bf9ae69e08c7`，無 staged diff。 |

本次沒有執行產品／SQLite／R1 實作測試，也沒有重跑先前整輪審閱。Windows canonical 及跨平台行為仍未驗；workflow validator exit 0 不是產品驗證 PASS。

## 本輪變更清單

相對啟動前，僅三個已存在檔案變更：

1. `.ai/workflow/CURRENT_MISSION.md`：官方建立 draft，隨後補齊授權範圍內記錄。
2. `.ai/workflow/WORKFLOW_STATE.json`：只由官方 start 寫成 intake／sequence 1。
3. `.ai/workflow/EVENTS.jsonl`：只由官方 start 追加唯一啟動事件。

逐檔 before／after hashes 在 CHANGES.json；完整清單在 before/FILE_HASHES.json 與 after-draft-completion/FILE_HASHES.json。既有 R3 的其他變更／新檔全部保留，本輪沒有在 repo 內新增檔案。

沒有 Git stage／commit／push、merge、release、ADR 接受、工具修改或產品實作。原證據發布 checkout 仍位於 `5df116fc8327c184bed057ba0ac54c45d7a5d751` 且乾淨；它沒有被修改成 intake。

## 前檢格式誤判及證據限制

一次官方 start **之前**的只讀 Python 前檢以「整個 dict 相等」比較摘要，因新 dict 多了 `index_diff` 欄位而 assertion exit 1；原／新共有欄位與內容指紋都相同，沒有實際來源漂移，當時尚未呼叫 start，也未寫入 repo。

已修正為逐項核對原摘要共有欄位，並另驗空 index，再執行第一次且唯一一次官方 start。這不是重跑失敗的 workflow 命令。診斷與 actual exit 見 before/PREFLIGHT_CHECK_CORRECTION.json。該次完整 tool error 保留於 chat；未另存 raw stdout／stderr 檔，本包明確標示此限制，不重製成原始輸出。其後四次官方 CLI 呼叫的原始輸出均有完整保存。

## 交付與停止

- FOUNDER_AUTHORIZATION.md：本次實際最小授權文字記錄。
- COMMANDS.jsonl／commands/：四次實際官方 CLI 命令、exit、時間、原始輸出及 hashes。
- before/、after-official-start/、after-draft-completion/：階段原始檔與清單。
- RESULT.json、CHANGES.json、R3_ARCHIVE_RECEIPT_HASHES.json：機讀核對結果。
- SHA256SUMS：本包 payload 逐檔 hashes（不含自身），可在解包根目錄執行 `sha256sum -c SHA256SUMS`，不會執行 workflow。

停止於 **「A-F1 intake 已建立，待產品審閱」**。產品審閱與工程計畫仍 pending，沒有進入 implementation。
