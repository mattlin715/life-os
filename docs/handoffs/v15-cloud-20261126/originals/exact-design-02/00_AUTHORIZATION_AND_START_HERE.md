# Life OS v15 Android A｜精確設計 02

**2026-11-25（台北） · Book One 審閱交付 · 待 Founder 審閱精確設計。**
本輪在 repository 外記錄授權與設計，未寫 repo、未操作 Git、未執行 SQL／測試／建包／裝置／模型，沒有接受或更新 ADR。這是對 Exact Design 01 的兩處限定補正（SQLite 拒絕／恢復、writer 抽取），**不是實作或恢復授權**。

## 核定紀錄：綁定 Design Review 02

Founder 指示：「以 Design Review 02 為基準，逐項同意上述 A1、A2 主方向、A2-a～A2-d、A3、A4 推薦及限制。授權下一輪精確設計，不授權產品實作、ADR 接受、repo 寫入或 Git 操作。」

| 核定項 | 記錄 |
| --- | --- |
| A1 | 同意。理解為本人獨立、可修訂的內容，不依賴 AI 或問答；A／B 語意共用，不代表共用 DB。 |
| A2 主方向 | 同意。單一 app-private canonical SQLite、精確版本引用、不預設複製來源正文、原子提交。 |
| A2-a | 同意。fresh synthetic 隔離候選；不升級、無資料移轉、無真實 profile。 |
| A2-b | 同意。明確選擇後，保存後來片刻到理解的最小導覽關聯；不是證據，不改原理解版次。 |
| A2-c | 同意。A 工程候選不提供理解匯出；既有片刻匯出不包含理解。不算完整資料控制或 Phase 3 匯出結案。 |
| A2-d | 同意。原生最小非正文提交紀錄與重開查回；精確欄位、限額、到期／清理與並發策略由本輪提案，尚待審閱。 |
| A3 | 同意。刪片刻不刪本人理解；清系統來源內容／副本、標示缺失、不改綁，全部缺失仍可編修；刪理解保留片刻。本人寫入理解的摘錄可能留在當時修訂，必須揭露。 |
| A4 | 同意。A 僅程序內未提交草稿；程序終止不恢復正文。已提交靠 native 查回；B 前另核純文字草稿。 |

**既定界線不變：**A 只是工程前置。B 真實 AI＋合成單片刻試用、C Phase 4 跨時間、本人／受邀資料或發佈各走獨立入口。相應 B 驗收與資料控制完成後，可另行申請限單片刻的本人試用，不必等 C。ADR-0013 原仍 Proposed，原 R3 gate 與 M2-D／036 STOP 不處理。

## 如何審閱

| 檔案 | 重點 |
| --- | --- |
| [01 候選與資料契約](01_CANDIDATE_AND_SCHEMA.md) | 一個隔離 app，具名版本／路徑／capability、六個必要業務表及內容／關聯邊界；對既有 v5 的獨立路徑。 |
| [02 交易、恢復與生命週期](02_TRANSACTIONS_AND_RECOVERY.md) | 單一寫入邊界、兩段提交與 durable 結果、來源刪除、多來源與缺失後修訂，無舊 payload 重放。 |
| [03 驗收與相容矩陣](03_VERIFICATION_AND_STOP.md) | 可執行的未來驗收、預期拒絕／停止、唯讀查核依據、實作前真正剩下的審查。 |
| [04 SQLite 拒絕與恢復](04_SQLITE_RECOVERY_ADMISSION.md) | 先做不經 SQLite 的檢查；受信任 A 原件的引擎恢復若需先寫入，明列 Founder 取捨。 |
| [05 Writer 抽取](05_WRITER_EXTRACTION_PLAN.md) | 函式、callers、必留驗證與 A 的單交易收束。 |
| [相對 01 差異摘要](CHANGES_FROM_01.md) · [清單](FILES.md) | 限定改動與原件綁定。 |
| [SHA256SUMS](SHA256SUMS) | 逐檔雜湊，不含雜湊清單自身。 |

**本輪先審 04 的 R0／R1 是否需要另行核定，以及 05 的舊 verifier 不變與 Tx E 原子邊界。** 不重開 A1–A4。未核 R1 前，不把帶 journal 的檔案交給引擎恢復。

## 來源綁定與證據限制

- Exact Design 01 ZIP 原樣保留：SHA-256 50d4a5b422b712522e30bfb38e866b598894dfe445a3542e04597732f639e43a。
- Design Review 02 ZIP：SHA-256 dd71d48ec616c82017677a6a61db37144d0788c2eb724d7526552bb0ac8c9af9。授權表依其中 03 的推薦與限制，不把其他建議當批准。
- 原 v15 prototype SHA-256：91eb8d73442bffd109037f526344802876eff66f871e9641f8e190553547ac96；本包未改也未重包原型。
- Review 02 記載的保存 HEAD/tree：388d9304988a46c548cc124fe820d61957f2c3ea / fa8408ed3e42e4362a8d21d5e20c93e2e336d062。此次僅以檔案唯讀抽查 Android 保存 worktree，**沒有透過 Git 重新證明 HEAD、tree 或乾淨狀態**；原 M2-D checkout 不是設計或寫入落點。
- 精確提案中的名稱、數字、物理契約與測例是待審實作輸入，非已存在的功能。所有未來產品檢查目前均為 not_run。
