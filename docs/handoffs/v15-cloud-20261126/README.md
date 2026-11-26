# Life OS v15｜雙 Cloud 交接入口
2026-11-26（Asia/Taipei） · Book One 交接索引 · 非新憲法、非 ADR、非整體實作授權

## 接手第一件事
這不是「根據需求重新設計 App」。Founder 已經逐版操作並接受 v15 原型；請先看懂樣品，再工作。**原始互動 HTML 是體驗交接核心，但不是已具備的資料庫、AI 或 Android 能力證明。**

1. 由使用者提供的 repository **精確 handoff commit SHA** 取得此目錄及完整 repository；不可只選最新 branch。讀根目錄 AGENTS.md、AI_CONTRIBUTOR_GUIDE.md 及其任務閱讀路線。
2. 執行本目錄 [verify_handoff.py](verify_handoff.py)，核對完整 SHA256SUMS、原型、原包及展開檔。此程式只讀取／驗證交接，不執行產品或改 Git。
3. 讀 [產品意圖與防偏移](01_PRODUCT_INTENT.md)、[最新核定紀錄](02_DECISION_LEDGER.md)。
4. 開啟 [原始 v15 HTML](originals/design-review-02/life-os-experience-prototype-v15.html)，按 [實際走讀清單](03_PROTOTYPE_WALKTHROUGH.md) 操作。不能只讀 HTML 或看截圖就宣稱已走讀。
5. 讀原版 [產品規格](originals/design-review-02/01_PRODUCT_EXPERIENCE.md)、[Android 邏輯設計](originals/design-review-02/02_ANDROID_ENGINEERING.md)、[驗收與分期](originals/design-review-02/03_ACCEPTANCE_AND_DECISIONS.md)；工程再逐份讀 [Exact Design 02](originals/exact-design-02/00_AUTHORIZATION_AND_START_HERE.md)。
6. 依角色使用 [審閱 prompt](prompts/REVIEWER.md) 或 [工程 prompt](prompts/ENGINEER.md)，遵循 [本輪工程邊界](04_CLOUD_ENGINEERING_SCOPE.md) 與 [交接協定](05_TWO_SESSION_PROTOCOL.md)。

## 一句話產品承諾
使用者不必自行整理所有記錄，就能開始回看；能知道觀察的依據、反駁不適用的解釋，用自己的話留下理解，並讓後來的經驗改變這份理解。

## 原件與時間順序
- originals/design-review-02/：原型與已審體驗／邏輯規格，逐位元組保留。
- originals/exact-design-02/：A 精確設計，逐位元組保留。
- archives/：上列兩份原始 ZIP，逐位元組保留。
- 原件內「待核定」「R1 尚未核定」是**當時狀態**。最新核定只由 02_DECISION_LEDGER 明確列出的範圍覆蓋，不改原件、不把所有未決項一口氣接受。
- 本目錄不是 repo 產品規格的第二套定義；它是原件、決策與執行角色的定位入口。與上位治理有衝突時指出差異、停下，不自行調和或刪需求。
- **本次 Git 保存不啟動任何工程 prompt。** 需 Founder 在目標 session 送出適用指示。

## 精確來源
程式保存基準：388d9304988a46c548cc124fe820d61957f2c3ea
基準 tree：fa8408ed3e42e4362a8d21d5e20c93e2e336d062
repository：https://github.com/mattlin715/life-os
本交接提交只增加此目錄；除這個目錄外的 tree 應等同上述基準。請把收到的 handoff SHA 與基準分開記錄。commit 不能把自己的 SHA 寫進自身內容；精確 handoff SHA 由發布後的啟動訊息提供。

## 沒有包含，也不可假設存在
沒有產品新實作、APK、金鑰、實人日記、DB、舊 host logs 或本機私密附件。歷史來源行號必須在固定基準重查。原型操作不等於 Android 實機驗收；Linux host PASS 不等於 Windows canonical 或手機 PASS。
