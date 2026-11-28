# 正式 R3 處置原始證據索引

狀態：**正式處置證據已發布，待獨立審閱**。本目錄只發布既有處置資料；沒有重新執行 workflow，也沒有重新產生原始操作輸出。

## 本次 Git 交接範圍

- Repository：https://github.com/mattlin715/life-os
- 固定 parent：`0e8e464c2a6057ec557f315571f3e9e0c80f550f`。
- Branch：`codex/r3-formal-disposition-evidence-20261128`。
- 唯一新增範圍：`docs/validation/r3-formal-disposition-20261128/`。
- 使用獨立發布 checkout；原正式處置 checkout `/workspace/r3-governance-source` 的 HEAD、index 與檔案內容保持原狀。
- `.ai/workflow/`、工具、ADR、產品來源、CI 及依賴均與固定 parent 相同。發布 branch 的活動控制面仍是該 parent 的歷史版本；本輪正式處置結果請以本目錄原始證據核對，不能把發布 branch 當作已推進到 idle 的執行 checkout。
- 沒有 merge／release、接受 ADR 或啟動 A-F1；證據發布不構成結果接受或下一階段授權。

## 閱讀入口

1. [REPORT.md](REPORT.md)：原 `RESULT.md` **逐位元組複製**，保留當時 A／B／C、idle、未決事項及計數誤判說明。該報告的「沒有 Git 保存」描述的是原處置階段；本次新授權僅發布其既有證據。
2. [RESULT.json](RESULT.json)：原完整機讀結果；[FINAL_GIT.json](FINAL_GIT.json) 是原操作結束時的 checkout 狀態。
3. [COMMANDS.jsonl](COMMANDS.jsonl)：原九筆實際命令／核對紀錄，包括 exit 1；各項原始 `.json`、`.stdout`、`.stderr` 同名保留於本目錄。
4. [cancellation/VERIFICATION.json](cancellation/VERIFICATION.json)、[archive/VERIFICATION.json](archive/VERIFICATION.json)、[idle/VERIFICATION.json](idle/VERIFICATION.json)：原階段核對資料。
5. [COPY_MAP.json](COPY_MAP.json)：44 份直接複製檔案的原路徑、發布路徑、長度與 SHA-256；原 `/workspace` 路徑僅供來源追溯，不要求另一個 session 能讀取。
6. [FILES.md](FILES.md)：本候選 commit 的完整新增檔案清單；[PUBLICATION_CHECKS.json](PUBLICATION_CHECKS.json) 是本次發布的只讀完整性檢查，明確區別於原 workflow 證據。

## 完整原始結果包

- [原始結果包](raw-evidence/life-os-r3-formal-disposition-20261128.tar.gz)
- [原始外層 SHA-256](raw-evidence/life-os-r3-formal-disposition-20261128.tar.gz.sha256)
- 原包 SHA-256：`b9bc4c871e4ac59dbb0ec42382ccd621680b1a5fcc08b20cec96c6b9c4a7bf99`。

此壓縮檔直接複製先前交付檔，**沒有重打包**。其中 94 個 payload 及其原 SHA256SUMS 均保留；本次核對沒有缺檔或 hash 不符。內容包含原 before／cancelled／published archive／idle snapshots、兩份完整 operation receipts、原授權、完整 tracked-control-changes.patch 與所有實際命令輸出。

本目錄直接提供必要的可讀副本；全部原件在原始結果包。REPORT.md 內的其餘相對路徑，指解開原包後的 `r3-formal-disposition-20261128/` 根目錄。原 [ORIGINAL_SHA256SUMS](ORIGINAL_SHA256SUMS) 亦適用該解包根目錄；不要把它誤用成發布目錄的檔案清單。

[本目錄 SHA256SUMS](SHA256SUMS) 則涵蓋本次發布的檔案（不含清單自身）。兩份清單用途不同，沒有修改原清單來配合發布布局。

## 計數誤判與更正

- [原失敗 metadata](06-final-audit-first-attempt.json)、[原 stderr](06-final-audit-first-attempt.stderr)：一次只讀 audit 把原 HISTORY/README.md 算入 archive 目錄數，assertion exit **1**。
- [更正 metadata](07-final-audit.json)、[更正 stdout](07-final-audit.stdout)：確認 README.md 是原清單中 hash 未變的檔案，按目錄數核對後 exit **0**。
- 全部原始 bytes 直接保留；未重做 cancel／archive、未改工具，也沒有把失敗輸出換成新執行的輸出。

## 原結果與驗證限制

原紀錄：取消 exit 0、R3 cancelled／sequence 57；archive exit 0；活動控制面 idle／sequence 0／空事件檔／無新 mission。原 HISTORY 639 檔 hashes 未變；schema 仍未解，ADR-0013 仍 Proposed；必要 recovery 未發生。

原始證據的捕捉限制如實保留：

- 原本的單 writer `/proc` 程序／FD 原始列表未另存成檔，只有既有座標與觀測摘要；本次不補造歷史列表。
- inline 最終 audit 的完整 stdin 程式碼未另存成檔；其原始錯誤、失敗斷言及更正紀錄仍在。
- 原外部歷史 raw logs 是 `not_acquired`，目前仍缺，沒有重製。
- 原包 manifest 所列 94 個 payload **缺件為 0**；上述「未另存／未取得」不被改寫為已具備。
- Windows canonical 仍 blocked/not_run；跨平台斷電耐久性未驗。本次只進行 bytes／hash／Git 範圍核對，沒有重跑 workflow、測試或 reviewer 整輪審閱。

停止等待獨立審閱，不啟動 A-F1。
