# R3-GOV-01：取消恢復映像關聯校驗

2026-11-28（Asia/Taipei）。**修正候選待獨立複審**。

Founder 本輪明確授權既有治理修補範圍的修改、測試、交付證據及 Git stage／commit／push 至專用候選 branch。此授權不包括 develop／main 修改或 merge、release、ADR 接受、真實 R3 workflow mutation 或 A-F1 啟動。

## 精確來源與候選保存

- Repository：https://github.com/mattlin715/life-os
- 固定起始／候選 commit 的直接 parent：`a9433c4e3a979110e5d159d60e2bfea6a536a1f2`。
- 基準 tree：`d2eeca50e8dbcab3db1b4c8c11e2fa47b132d1e9`。
- 專用 branch：`codex/r3-gov-01-governance-candidate`；沒有改用最新 branch tip。
- 保留既有 `/workspace/r3-governance-source` 六份未提交治理候選，核對前輪 95/95 來源 hashes 完全一致後修正；開始時 index 為空。原 `/workspace/life-os` checkout 未改。
- 同一候選 commit 保存六份實際治理來源／文件／範本及本目錄的必要證據；沒有重複提交整份 source 壓縮包。完整 commit SHA 由 commit／push 後回覆提供，不在自身內容虛構自我 SHA。

## 問題與修正

原恢復入口分別驗證 before、after 可形成合法 workflow，但沒有要求兩者是同一次取消的前後映像。重新匹配 operation hash 只證明紀錄 bytes 完整，不能證明內部關聯正確。因此 before 的歷史驗證可與 after 的取消 checkpoint 不同；after 也可能包含合法但已改寫原始 bytes 的事件前綴。

`validateCancellationImages` 現在於任何控制面寫入前要求：

1. `after.WORKFLOW_STATE.sprint_disposition.checkpoint` 與**完整 before state** deep strict equal，涵蓋歷史驗證及所有其餘欄位，不只 sprint／sequence。
2. after EVENTS 的開頭與 before EVENTS **逐位元組相同**，保留原始 JSON 格式、CRLF 及其他已合法存在的歷史 bytes。
3. 只追加一筆 `sprint_cancelled` JSONL event；若舊檔無結尾 LF，僅補一個界線換行。核對 from／to、sprint、sequence、previous event hash，並保留原 validator 對完整 hash chain、授權及 state 投影的檢查。

內部矛盾即 exit 1；不寫 current files，不刪／改 pending 或復原材料，不產生成功 receipt。重新計算並提供匹配的 operation SHA-256 仍不能通過。使用 Node 內建 `node:util` 的 deep equality，沒有新增依賴。

本輪新增來源差異只在 `scripts/ai-workflow-governance.mjs`、原 `scripts/ai-workflow.node-test.mjs` 及 `.ai/workflow/WORKFLOW.md`；`R3-GOV-01-helper.diff` 是相對前輪候選的零 context helper 差異，供聚焦審閱。候選 commit 同時保存原先六份獲授權治理修補，包括官方 cancel-sprint／recover-governance、獨立處置範本及導覽文件。

沒有修改真實 state、events、原決策／報告、HISTORY、WORKFLOW_CONTRACT.json、ADR、產品來源、UI、CI 或 verifier。沒有改 `resolve-decision`／一般 `createTransition` 的決策恢復限制。

## 測試與 actual 結果

平台：Linux x86_64，Node v24.22.3，Git 2.56.1。每項命令、actual exit、來源 hashes 見相應 JSON；log 以無損 JSON envelope 保存原始 bytes（包括空白與換行），可用 `read-log.py` 還原且核對 SHA-256，沒有裁掉失敗或修改 verifier。

| 驗證 | actual 結果 |
| --- | --- |
| 前輪完整候選基線：原 workflow 測試入口 | 95/95 PASS，exit 0；六份既有候選 hashes 保留於 baseline/。 |
| 修正前兩個必要負向案例：`node --test --test-name-pattern=R3-GOV-01 scripts/ai-workflow.node-test.mjs` | **0/2，exit 1（預期 red）**；兩次恢復子程序錯誤地各回傳 exit 0，故測試失敗，完整原始輸出保留。 |
| 修正後 R3-GOV-01 聚焦回歸，同命令 | 4/4 PASS，exit 0；必要兩案、額外空行拒絕、CRLF／unterminated 正常恢復。 |
| 最終 `node --test scripts/ai-workflow.node-test.mjs` | **99/99 PASS，exit 0，0 skipped**；原 95 項全部保留。來源 hashes 綁定 workflow-full.json。 |
| 58 份舊 archive 的固定基準／候選 validator 比對 | 58/58 有效、verdict 全同，exit 0；見 legacy-archives.json。 |
| 真實控制面只讀 `node scripts/ai-workflow.mjs validate` | exit 0；沒有執行任何 mutation。 |
| 主工具與 helper `node --check`、handoff integrity、`git diff --check` | 各 exit 0，見 checks.json 與對應輸出。 |
| Windows canonical `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1` | **blocked/not_run**：PowerShell 不存在，shell launcher exit 127；verifier 沒啟動，內部 exit code=null。Linux 結果不當 Windows PASS。 |

最終完整回歸記錄 64 次實際 SIGKILL、6 次 injected exceptions，以及三次使用重新匹配 operation hash 的矛盾拒絕（fault-observations.json）。SIGKILL 的 Node `status=null`／`signal=SIGKILL` 是 signal 終止，沒有正常數值 exit；不是 PASS exit code。

### 必要負向案例如何排除假陽性

- **歷史驗證不一致**：在全新合成 fixture 建立 pending cancellation，將 before 的歷史 repository verification 改成另一份合法 failed 紀錄；current state 與 before 一致，after checkpoint 保留原紀錄。before／after 各自先用真正 validator 通過，重算 image 與 operation hashes，再要求官方恢復拒絕。
- **合法鏈、不同原始前綴**：只在 after 的第一筆歷史 JSON 前加合法空白。event objects、hash chain、checkpoint 均仍合法，before／after 的 validator 都通過，但原始前綴 bytes 不同；重算 hashes 後仍須由新的前綴關聯檢查拒絕。
- 兩案及額外空行案都比對恢復前後**整棵 fixture 控制面**的檔案與目錄清單／hashes，涵蓋 pending record 與所有復原材料；並確認 start 仍被 pending gate 阻擋。沒有靠舊 hash mismatch、非法 workflow 或缺授權來造成假拒絕。

## 真實 R3 不變與相容性

`protected-before.json`／`protected-after.json` 的 **652 個原控制面／archive／ADR 檔案 hashes 完全一致**。R3 仍為 `android-pilot-byo-gemini-r1`、revision 56、human_decision_required、decision_resolution=null；原 failed 驗證與 manual_ui not_run 保留，ADR-0013 仍 Proposed。

舊 archive 不被補造新處置證據；58 份固定基準相容性通過。原正常取消、各 event/state 中斷、archive copy／publish／reset、恢復失敗／重入、completed 封存及新 mission 不繼承 PASS 的測試全保留並通過。新正常案例另證 CRLF／無結尾 LF 的前綴保留。

## 證據與重跑

從 repository 根目錄：

```text
node --test scripts/ai-workflow.node-test.mjs
node --test --test-name-pattern=R3-GOV-01 scripts/ai-workflow.node-test.mjs
node docs/validation/r3-gov-01-20261128/check-archives.mjs
python3 docs/validation/r3-gov-01-20261128/verify-evidence.py
python3 docs/validation/r3-gov-01-20261128/read-log.py docs/validation/r3-gov-01-20261128/regression-before-fix.log.json
```

workflow 測試只在新暫存合成 fixture 執行 mutation／建立測試 Git commits／終止子程序；archive 比對只讀原件並在暫存檢查副本驗證。不要對真實 R3 直接執行 cancel-sprint、archive、recover-governance、resolve-decision 或 start。

- SOURCE_HASHES.json：六份候選實際檔案 hashes。
- SHA256SUMS：六份來源及本目錄所有 payload 的逐檔 hashes，不含清單自身。
- regression-before-fix.*／regression-after-fix.*／workflow-full.*：red／green／完整回歸。
- baseline/：原 38 項基線、前輪 95 項候選、先前各次失敗與修正理由，明確標為歷史，未覆寫成新結果。
- checks.json、legacy-archives.json、protected-comparison.json：其他適用檢查與保全證據。

## 未驗項與停止點

尚待獨立複審／Founder 接受。Windows canonical、Windows／Mac／Android filesystem 行為及跨平台斷電耐久性 not_run；只證 Linux process interruption。SIGKILL／symlink 測例的 Windows 可執行性也待該平台核對。

仍維持 single writer；不新增跨機 lease、通用備份或 force/discard。初始 incomplete preparing、未知 bytes／檔案或衝突仍保全並拒絕，需獨立審閱。外部原始歷史 logs 未取得，不宣稱已封存。工具正常恢復成功僅在合成測試發生；Git push 成功不觸發任何 workflow action。

產品 UI／Rust／模型／真實資料／APK／裝置／M2-D／036 不在本次；沒有實作 A-F1、接受 ADR 或正式取消／封存 R3。到「**修正候選待獨立複審**」停止。
