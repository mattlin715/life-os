# 雙 Cloud 合作協定：共同基準，分開職責
2026-11-26 · 保護 Founder 認知負擔與產品一致性。

## 建議順序
1. 新開 Cloud「Life OS v15｜產品方向與審閱」。給 repo、發布後精確 handoff SHA、README 路徑與 REVIEWER prompt。
2. reviewer 實際走讀後，交回短版理解回述及可下載的 REVIEWER_RECEIPT。Founder 確認沒有理解偏差；不是重新核定所有設計。
3. 新開另一 Cloud「Life OS v15｜A-F1 限定工程」。給相同 repo/SHA、ENGINEER prompt，以及上述 receipt／Founder 確認。engineer 也須實際走讀，不能只信 reviewer。
4. 工程只寫其 Cloud 工作區。把精確結果包送 reviewer；reviewer 對實際來源與證據複審，輸出具體差異，不替 Founder 接受。
5. Founder 明確核定後才談保存／PR／整合／Android build 與下一段。不要讓 reviewer 通過自動觸發另一 session 執行。

## 角色
- 產品／審閱：保護動機、核心旅程、作用範圍，審差異及證據；可寫自己工作區的交接／審查報告，不改 repo 產品／治理或工程來源，不 commit/push。可討論替代方案，須標 proposal。日後若要寫規格另外授權。
- 工程：只實作當次授權切片；不能自行替換產品方向，也不負責核定自己成果。
- Founder：最終產品與權限決策者。兩個 session 的建議均不是 Founder 決定。

## 不假設共享
兩個 Cloud task 不一定共用未提交檔案、背景、附件或訊息。不要把另一 session 的 /workspace 路徑當可讀。
共享基準由 Git 精確 SHA 固定；動態成果以可讀附件／精確後續 commit 傳遞。尚未授權 Git 保存時，用自包含輸出包傳遞，由 Founder 上傳或提供可讀連結。
reviewer 讀不到 diff／log／測試程式時，標「僅報告宣稱、未獨立核對」，不可批准實作符合。
Cloud workspace／暫存預覽不是 Git 永久備份。關機前先確認兩個 session 已能讀到 repo/commit/HTML；不依賴 C:/D: 或本機 localhost。
手機預覽只有在該 Cloud 環境提供可用的受控 HTTP preview 時使用；存取與存活期限依實際環境，不承諾永久網址，不改公開分享權限。

## REVIEWER_RECEIPT 最小欄位
- handoff commit、base SHA、原型 hash、manifest 驗證結果。
- 已讀文件、W1–W11 實走項／未走項、證據檔。
- 三故事回述：支持／反例／未知如何不同；A/B/C 不是同一件事。
- 已確認體驗／示例／已核 A 限制／待決／本輪不做表。
- 原型與規格矛盾、程式依據缺口及真正需要 Founder 的決定。
- 本次允許／未允許；建議工程範圍，不把建議寫成授權。

## ENGINEERING_RESULT 最小欄位
- exact base/HEAD/tree、dirty diff digest、新檔 hashes、可重建材料。
- 範圍與功能逐項完成／未完成，原型 V-ID→source symbol→tests→result。
- 測試 command／platform／actual exit code、原始 failures、not_run、環境限制。
- v5 等價性、Tx E 故障注入、R1 恢復前後 bytes／拒絕狀態與無重放證據。
- 未核事項／設計不合／ADR、手動驗收需求、Git 及工作樹狀態。
- 無新的授權就停止，不由 PASS 推進下一階段。

## 日後防止再漂移
每次改動先列 3–5 行「不變／本次變／延期／需 Founder」。小文案調整不必每次重開完整設計，但改變資料保留、同意、作者、理解生命週期、主路徑或 scope 必須明列。不要用一長串 gate 問題取代實際可用性；只把真正不可自行決定的事交 Founder。
