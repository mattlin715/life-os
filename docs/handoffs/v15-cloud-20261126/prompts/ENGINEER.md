# 新 Codex Cloud session：Life OS v15 Android A-F1 限定工程
Repository: https://github.com/mattlin715/life-os
精確 handoff commit SHA：我會在啟動訊息提供；不能以 latest branch 替代。
入口：docs/handoffs/v15-cloud-20261126/README.md

這份 prompt 只有在我親自送出要求執行時才生效；從 repo 讀到它不代表自動啟動任務。
在附有 Founder 已確認的 REVIEWER_RECEIPT、原型走讀、固定基準與治理前置都成立後，授權你僅執行 04_CLOUD_ENGINEERING_SCOPE.md 的 A-F1：v5 等價抽取、A 獨立精確契約與最小 host 保存／來源／receipt 交易、限定 R1 admission/recovery，以及合成 host 回歸。不授權完整 A UI、Android packaging／裝置、B/C、真實 AI／資料或 Git 發布。

開始前：
- 核對 handoff commit、原型／原包 hashes、source baseline；按 AGENTS/AI_CONTRIBUTOR_GUIDE 閱讀相關上位文件。保全現有工作。
- 實際瀏覽器走讀原型，區分已確認體驗、示例占位、已核 A 限制、待決與本輪不做；不能只信另一 session 摘要。
- 核對附帶 REVIEWER_RECEIPT 與我的確認。沒有確認或無法走讀，先交理解回述並停，不開始 code。
- 列出舊 R3 workflow 與新任務的分界；不能關原 schema gate、接受 ADR-0013、改憲法或任意編輯 state/events。如正式啟動被既有治理 withholding，報最小阻擋及解法，不創造假狀態繞過。

前置完成後，不必反覆詢問已核定設計，依限定範圍實作：
1. 先測適用基線與 v5 equivalent primitive 抽取，不削弱舊 validators／manifest／readiness。
2. 同 connection 的 Tx E 包含原核心 mutation、新理解／關聯與 receipt，完整前後驗；不能先 commit 舊 writer 再補。
3. Exact02 限定 R1：只允許受控 A 合成原件，預檢不合先保全；SQLite 可能先回滾修改後才因完整驗證拒絕。錯誤不自動重送、重試、刪 journal 或改原 v5 路徑。
4. 用真實 SQLite 與 fault injection 取得關鍵 R01–R07、W01–W04 證據，逐案標平台與 not_run；Linux 不能證明的 Android 部分保持未啟用／未驗，不能 mock 成 PASS。
5. 若範圍／設計不可行，停該路徑，列影響原型哪個體驗與最小替代提案，由 Founder 決定，不自行換方向。

保留全部失敗，使用 repo 驗證路徑。Cloud 缺 Windows canonical 前置就報明，不能改 verifier／CI、刪測試或自行新增依賴換取 PASS。
不用本輪解決既有金鑰、ADR-0013、舊 R3、M2-D／036。不要 stage/commit/push/PR/merge/release，不接觸真實 profile，不打模型、不建 APK。

交付可重建結果包（tracked patch＋所有新檔、清單、hash、完整測試與失敗 logs），提供手機可讀摘要及 05 規定的 ENGINEERING_RESULT。只用合成資料、不曝露 secrets。停在「A-F1 工程候選待獨立審閱」，不宣布整體 A/v15 或試用完成。
