# R3 正式處置結果待獨立審閱

2026-11-28（Asia/Taipei）。已依 Founder 本次明確接受工具、Linux 限制及 A／B／C 的核定，完成一次正式取消及一次正式 archive／idle reset。**目前為 idle，沒有啟動 A-F1 或任何新 mission。**

## 來源與授權

- Repository：https://github.com/mattlin715/life-os
- 執行 checkout：`/workspace/r3-governance-source`。
- 工具 HEAD：`0e8e464c2a6057ec557f315571f3e9e0c80f550f`；branch：`codex/r3-gov-01-governance-candidate`。執行前後均相同。
- 獨立審閱參照：`154cd61ab04ca194b3bd8675dc7eef8e9bffadf4` 的 `docs/reviews/r3-gov-01-20261128.md`。沒有切換／合併該 commit，也沒有重啟整輪審閱。
- 真正操作依據是本次 Founder 明確 A／B／C 核定，不是 reviewer PASS。原文（保留可見 Markdown quote markers）在 `FOUNDER_APPROVAL.md`；實際獨立處置 JSON 在 `founder-disposition.json`，同一份 repo 路徑為 `docs/validation/r3-disposition-20261128/founder-disposition.json`。
- 處置 scope 僅 `cancel-current-sprint-only`；schema 未解、ADR 狀態不變、無產品權限三項 constraints 保留。B／C 另由同份實際 Founder 回覆授權，沒有把 archive 當 schema resolution。
- 獨立處置證據 SHA-256：`2a94f7f01dab950b88a6f535ba8bf7476ab10de27e043e3b687d27edc82ca7ed`。記錄原文與 hash 不等於密碼學認證人類身分；來源是本 chat 的實際 Founder 核定。

## 執行前核對

HEAD／branch、六份來源 hashes、652 份原控制面／HISTORY／ADR hashes 均與既定候選相符。index 空且工作樹乾淨；R3 為 `android-pilot-byo-gemini-r1`、human_decision_required、sequence 56、decision_resolution=null。ADR-0013 為 Proposed。

末筆原事件為 `android-pilot-byo-gemini-r1:0056`，event_hash：`0c61f25a776dffcaf14b590f60423c9a742552efe7d17edc52672fc77516e88f`。state SHA-256 為 `fe3410992f6b0683824740df2a82ef02887a0c49b0f604e7d776bdd451ff52d8`；EVENTS SHA-256 為 `9b896cdff50c03e7c923e86b8852f46dcae5f4c05e15883ffd0f3f904ffd28cb`。

沒有 pending／preparing、receipt 目錄、同名目標 archive 或 Git lock。本機前檢及 B 前檢未觀察到其他候選 cwd 程序或開啟的寫入 descriptor；執行者是本次受派單一 session，未主張跨機 lease／全域排他保證。測試故障 hook 環境變數未設。完整來源／index／平台座標及原控制面逐檔副本在 `before/`。

## A｜取消結果

官方命令只執行一次：

```text
node scripts/ai-workflow.mjs cancel-sprint --root /workspace/r3-governance-source --sprint-id android-pilot-byo-gemini-r1 --expected-sequence 56 --evidence docs/validation/r3-disposition-20261128/founder-disposition.json --evidence-sha256 2a94f7f01dab950b88a6f535ba8bf7476ab10de27e043e3b687d27edc82ca7ed --founder-response <同份實際核定原文> --scope cancel-current-sprint-only
```

`01-cancel.json`／COMMANDS.jsonl 保存**實際完整 argv**（不是上述原文占位表示）、cwd、UTC 時間、actual return code、stdout／stderr hashes；原始輸出亦分檔保留。

- actual exit **0**；status=`cancelled`，sequence **57**，事件數 **57**。
- 原 56 筆事件共 **51,221 bytes** 的原始前綴逐位元組保留，只追加一筆合法 `sprint_cancelled` 事件。
- 新末筆：`android-pilot-byo-gemini-r1:0057`，hash：`a2831bb260b31faf745ecd7470ca1f4bf676bdabd2f3c1252721bbbb35f1bb45`。
- disposition checkpoint 與完整取消前 state 等價；新 event／state 的 disposition、授權 raw bytes／hash／path 完全一致。
- 原決策、七份報告／artifact、active decision IDs、blocked／resume／blocking reason、last_completed_phase 及完整 verification 均保留；原 652 份保護檔案除核定 event/state pair 外，其餘 **650** 份不變。
- `02-validate-cancelled` exit **0**；無 pending／preparing。這是 workflow 結構驗證，沒有把產品驗證登記為 PASS。

取消後完整 state／EVENTS、receipt 與核對結果在 `cancellation/`。

## B｜封存／idle reset 結果

A 的結果、sequence 57、原始事件前綴、完整 checkpoint、來源 hashes、index、單 writer 與目標無衝突再次核對後，官方命令只執行一次：

```text
node scripts/ai-workflow.mjs archive --root /workspace/r3-governance-source
```

`03-archive` actual exit **0**。發布目標：

```text
/workspace/r3-governance-source/.ai/workflow/HISTORY/android-pilot-byo-gemini-r1/
```

- manifest schema `2.0`，terminal_status=`cancelled`，sequence 57 的 state／末筆事件座標正確。
- **13 個 payload 的逐檔 hashes** 全部核對；連 manifest 共 **14 檔**，沒有額外未知檔案。
- 核心 10 檔與 archive 前控制面 bytes 完全相同；原決策仍 open、完整 57 筆 EVENTS bytes 相同。
- `CANCELLATION_CHECKPOINT.json` 等價於完整取消前 state；`SPRINT_DISPOSITION.json` 與取消投影相符；`SPRINT_DISPOSITION_EVIDENCE.json` 等同本次實際證據原始 bytes。
- manifest SHA-256：`55433e098b4aea04518eb48403a12919e8e8ba3fc5e5c32864e27c8d41cab34b`；manifest 自身由 archive operation record 綁定，沒有自我雜湊。
- 原有 **58 個歷史 archive 目錄**及原 HISTORY 共 **639 個檔案（含 README.md）** hashes 不變；新增本次目錄後為 59 個 archive 目錄。核定 reset 範圍之外的原保護檔案 **643** 份全部不變。
- 外部原始歷史 logs 未讀取／複製，manifest 如實記為 `not_acquired`。原驗證參照被保留，不宣稱已取得那些 logs。

新 archive 的完整交付副本在 `archive/published/android-pilot-byo-gemini-r1/`；原 archive 前核心控制面 bytes、receipt 與驗證結果也在 `archive/`。

## Idle 核對

七份 current Markdown、WORKFLOW_STATE 與 EVENTS 均與精確候選的 idle 範本／空事件檔一致：

- status=`idle`，sprint_id=null，sequence=0，事件 bytes=0。
- last_event_id／last_event_hash=null；active_decision_ids=[]；decision_resolution=null。
- repository_verify=`not_run`，manual_ui=`not_run`；沒有繼承 R3 的驗證狀態或產品權限。
- 沒有 sprint_disposition／terminal_follow_up／新 mission；沒有呼叫 start。
- state SHA-256：`c68e62a717965287468b418b45ccdad46590d2b9d1c21ad41e5ed82c3b818107`。
- 空 EVENTS SHA-256：`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`。
- `04-validate-idle` 與 `05-status-idle` 各 exit **0**；無 pending／preparing。

idle 副本及核對結果在 `idle/`。這是本次 workflow 處置終點，不是 A-F1 intake。

## C｜必要恢復及持久紀錄

兩次正式命令正常完成，沒有程序中斷，**未另行呼叫 recover-governance CLI**。官方 A／B 自身的正常持久寫入協定已完成；不把內部協定函式誤稱為另一次中斷恢復操作。

成功 receipts 均保留，未刪唯一復原材料；各 side 的 base64 原始 bytes、內部 hashes 與 before／after／published archive 已核對：

| 操作 | receipt ID | operation.json SHA-256 |
| --- | --- | --- |
| A cancel | `760892ec-e514-4b95-8d33-8d66cbb19806` | `6519fd96fe9569f73806d14fab7c8cae7e6ac28fa684078999ed1a2b8b019cc7` |
| B archive | `85f42815-c703-4a00-af8a-5c00d0607945` | `e9993f21c28c8ddb748d710faa33a89523f18fd237d39d473be5d6628e109ab5` |

原件位於 `.ai/workflow/.governance-receipts/<id>/`；結果包另保存完整副本。沒有 force、手改 state/events、清除未知檔案或故障注入。

## 未決事項及限制

1. `ANDROID-M3-GEMINI-R3-TYPED-MODEL-SCHEMA` **仍未解**；原 open 決策、取消 checkpoint 與 cancelled state 都保存在 archive。idle 的 current 決策範本為 not_required，不能據此聲稱原 schema 已解。
2. ADR-0013 **仍 Proposed，未接受**。R3 items 5／6 未完成；取消不代表功能完成、PASS、產品方向否決或能力授權。
3. 原 repository verification **failed／exit 1**、manual UI **not_run** 都原樣保存；本次 validator exit 0 沒有覆蓋它們。
4. 執行平台 Linux x86_64、Node v24.22.3、Git 2.56.1。Windows canonical 仍 blocked/not_run；本輪沒有啟動該 verifier。跨平台斷電耐久性未驗，沒有擴張保證。
5. 工具及來源未修改；未重跑已完成的整輪審閱／合成測試。未操作模型、金鑰、私人資料、APK、裝置或 M2-D／036。

## Git 狀態及證據讀法

HEAD／branch 不變；index bytes 前後 SHA-256 均為 `d2ceef7b81c0ba078a450407125cfd575afa01e71e73c4de2bf4bf9ae69e08c7`，無 staged 差異。實際工作樹有 **8 份核定控制面 tracked 變更**、**17 份核定 archive／receipts／Founder 證據新檔**，完整清單見 FINAL_GIT.json。沒有 stage／commit／push、merge 或 release。原 `/workspace/life-os` checkout 未動。

因此 GitHub 候選 commit 仍保存處置前控制面；本次正式結果保留於上述執行工作樹及本結果包。候選內 verify-evidence.py 的 652 份清單是處置前基線；正式 reset 後應使用本包分階段檢查及 643 份 immutable 清單，沒有改寫舊 verifier 或基線來追求 PASS。

COMMANDS.jsonl 及逐項 JSON／原始 stdout／stderr 保留實際結果。交付最後只讀 audit 曾錯把既存 HISTORY/README.md 算作 archive 目錄，導致一次 assertion exit 1；確認該 README 是原清單中 hash 未變的檔案後，改以目錄數核對並 exit 0。原錯誤保留於 `06-final-audit-first-attempt.*`；沒有重做取消或封存，沒有改工具。最終核對為 `07-final-audit.*`。

`RESULT.json` 為完整機讀結果；`before/`、`cancellation/`、`archive/`、`idle/` 保存各階段資料。SHA256SUMS 對本包所有 payload（不含自身）列出逐檔 hashes。可在解包目錄執行 `sha256sum -c SHA256SUMS` 作只讀完整性核對；不應對這些證據副本執行 mutation 或另啟動 mission。

停止於 **「R3 正式處置結果待獨立審閱」**。沒有啟動 A-F1。
