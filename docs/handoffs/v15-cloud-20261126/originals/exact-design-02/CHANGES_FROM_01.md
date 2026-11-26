# Exact Design 02：相對 01 的差異摘要

**2026-11-25 · 只補兩點及必要交叉引用；沒有加入產品範圍或改核定。**

| 檔案 | 相對 01 變更 |
| --- | --- |
| 00_AUTHORIZATION_AND_START_HERE.md | 版本、原包雜湊、補正入口；八項 Founder 核定照錄。 |
| 01_CANDIDATE_AND_SCHEMA.md | 將啟動順序修成「非 SQLite 排除 → 分流 → 准許時恢復 → 完整驗證」，固定檔名／身份／版本候選與六張表未改；指向函式級抽取。 |
| 02_TRANSACTIONS_AND_RECOVERY.md | 重開當次須通過 S3；嚴格保全不能當作 rollback 已完成，不允許讀回悄悄提權；Tx E 參照 05，其他交易和產品語意不改。 |
| 03_VERIFICATION_AND_STOP.md | 拆出預期的外部／內部錯 contract 或 manifest、hot journal、不明／mixed、第二程序、恢復後失敗，以及四個最小 writer 故障案例。全部 not_run。 |
| 04_SQLITE_RECOVERY_ADMISSION.md | 新增：依 SQLite 官方規則與 v5 真實開啟路徑，明列各階段允許的檔案變動、信任模型與 R0/R1 取捨。 |
| 05_WRITER_EXTRACTION_PLAN.md | 新增：具名函式與 callers、不可刪掉的 v5 verifier/readiness、A 同 Tx E、回歸／故障原子性及尚未證明的點。 |
| FILES.md／SHA256SUMS | 新包清單與完整性，不變更任何 repo manifest。 |

**不變更：**已核定 A1–A4 及 A2 各子項、隔離 synthetic 與不升級、理解與來源語意、匯出與草稿限制、B/C/D 界線、既有 v5 契約與 ADR-0013 Proposed、M2-D／036 STOP。

**停點：**保全拒絕是截至本輪的授權上限，不預先選擇原地恢復例外；待 Founder 審閱 R0／R1 及抽取方案。未跑 SQL、測試、程式試作、建包或裝置。Exact Design 01 ZIP SHA-256 50d4a5b422b712522e30bfb38e866b598894dfe445a3542e04597732f639e43a 原樣保留。
