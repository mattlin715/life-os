# A｜候選與資料契約

**精確設計 02 · 2026-11-25 · 待審。** 以下是實作輸入，不是對 repo 或 ADR 的修改。A-u 不是通用 schema-v6，也不是 ADR-0013 七表設計。

## 1｜固定候選邊界

| 項目 | 建議鎖定值／行為 |
| --- | --- |
| 平台與身分 | Android ARM64、僅工程合成資料；獨立 application ID **com.lifeos.review.v15.a**，Cargo feature **android-v15-understanding-a**，與既有 Pilot/online 或 production activation feature 互斥。只建立新候選，不取代已安裝 Pilot，不用 com.lifeos.app；若精確 ID、feature 或版本已被使用，停止回審，不自行重用或改號。 |
| 命令與能力 | 新候選 capability **v15-understanding-a**，只對其本機主畫面曝光列名的解析命令與既有相容操作的 A 專用 adapter；無通用 SQL、路徑參數、匯入、provider、AI key、外連插件、背景資料同步或新 export 能力。保留本地種資料／走既有離線問答之適用操作，不把連網命令註冊進 A。審實際合併 Android manifest、capability 與 IPC，不只看原始設定檔。 |
| 唯一 DB | 只由 native app_data_dir() 解析；固定 **v15-a-synthetic.db**。建置用 **.v15-a-init.pending.db**，發佈憑據 **v15-a-fresh.receipt.json** 與 **.v15-a-init.pending.receipt.json**。既有 Android 契約要求的檔案類型／權限／符號連結拒絕保留。除 SQLite 自己的交易檔、初始化 receipt 和鎖定 metadata，不另開第二資料庫、內容或 recovery sidecar。 |
| 唯一寫入者 | native 在非 SQLite 預檢後、開啟 SQLite 前持有 app-private **.v15-a.lock** 對應的 OS 排他鎖，涵蓋所有連線及其關閉；lockfile 存在本身不是鎖。既有 profile 不補建鎖檔，見 04。取得後所有 A dispatch、readiness、關閉／reconcile 走同一 native gate。未經證明該 Android API 支援正確排他與關閉語意，不宣稱程序回收流程成立。 |
| 版本／contract | 候選暫定 SQLite **PRAGMA user_version = 15001**，合約 ID **life-os/android-v15-a/schema-1**，receipt format **life-os/android-v15-a/direct-fresh-1**。保留原有 v5 version=5／DDL／manifest 拒絕邊界；在 A 內沿用既有 v5 對象及 writer 語意，但用自己精確 manifest/runtime。**不得**把 15001 加入原 v5 verifier 的 accepted list。 |
| 候選限制 | 無 v4／v5 升級、無移植／同步、無備份／恢復途徑、無 Android Auto Backup 或 device transfer、無真實資料，無理解匯出或外送。既有片刻匯出如果在此候選原本可用，格式／資料範圍不變並清楚標僅片刻；不得為 A 新開一個匯出入口。 |

名稱／數字是本輪**精確提案**，不是宣告 repo 已保留或 Founder's ADR。實際 manifest 和 build receipt 須在未來授權的實作候選由實際 DDL、Tauri 設定、Android merge 結果和工具版本產生，不能將本文件雜湊用作 DDL 或 APK hash。

### 啟動、驗證與發佈

啟動的規範以 [04 SQLite 開啟與恢復入口](04_SQLITE_RECOVERY_ADMISSION.md) 為準：S0 不經 SQLite 預檢與取得既有鎖後重查 → S1 分類 → 無側檔走 S3；外部閘合格但帶 journal 仍保全（R0），只有另行核定的 R1 才許 S2 引擎恢復後進 S3。只有 S3 的完整一致性驗證通過才 Ready。初始化 mixed state 與 operation pending 永遠分開；不得以舊 v5 immutable 或 rw writer 路徑替代檢查。

正式 receipt 不含正文，欄位仍為 format、contract ID、application ID、profile ID、schema version、initialized_at、DDL digest、object manifest digest、空資料 manifest digest；DB contract 主鍵固定 singleton=1，不加另一個信任側檔。空資料 manifest 僅供初始發佈，使用後不拿它當 live DB digest。DELETE／normal locking、FULL synchronous、foreign_keys=ON 是新候選目標，不在拒絕、預檢或恢復階段以 pragma 改既有設定。

## 2｜資料表字典（規範性設計，非可執行 DDL）

保持 SQLite 一個真相來源。舊 Experience source head、revision、content 及既有 artifact 表、projections、writer invariants 以凍結的 v5 基準作為 A 的核心子集；A 合約另外有自己的單一 contract 記錄及下列六張 A 業務表。不得將理解放進既有 Evidence、Reflection 或 Pattern，不能假裝現有 A 專用 contract 只是 v5 memory adapter。

**A 的 contract singleton：**現有 v5 的 database_contract 有 authoritative_schema=5 的封閉 CHECK，不能原封不動搬來後填 15001，也不能同一 DB 掛一份聲稱仍由 v5 writer 支援的契約。A 使用自己的 database_contract 定義：singleton=1、authoritative_schema=15001、contract_id=life-os/android-v15-a/schema-1、application_id=com.lifeos.review.v15.a、profile_id（native 隨機 128-bit）、origin=direct_fresh、runtime_abi=1、initialized_at、compatibility_projection=enabled、lifecycle_writes=enabled、export_v2=disabled。全部必填，僅初始化設定；runtime 驗與編譯常數、receipt 和 layout 完全一致。既有 projection 表與交易內 guard 保留同一意圖；舊 v5 的 contract、schema object manifest 與檢查依然完全獨立，不經此 A verifier。A 的空 migration receipts 乃歷史防混用檢查，永不寫 migration row；如果核心 reuse 仍要求 authority=5，停在交易內抽取邊界，不雙寫兩個 authority。

**共用編碼**：新 head/revision/link/op ID 用 native 安全隨機 128-bit、前綴 u_/ur_/uf_/ua_ 加 32 位小寫十六進位；既有 source/revision IDs 不轉型、不重編號。時間為 native UTC ISO 8601 固定毫秒，revision 以 seq 定序而非時鐘，head last_saved_at 取 max(既有值, 這次提交時間) 防時鐘回跳。正文為有效 UTF-8、不含 NUL、至少一個非空白 Unicode 字元，最多 32,768 bytes；標題 trim 後為空即 NULL、否則至多 256 bytes；不對正文正規化或靜默截斷。顯示名稱 fallback 從目前正文開頭呈現，不建立另一內容欄。每理解版次 1–32 條來源，含缺失標記，排序以使用者提交的 ordinal。第一版至少一條有效來源；往後可全為已有缺失標記。A 不設空來源的新建入口。

| 表 | 欄位與強制約束 |
| --- | --- |
| **a_understanding_heads** | id PK；current_revision_id NN；created_at、last_saved_at NN。外鍵 (id,current_revision_id) → a_understanding_revisions(understanding_id,id)，可延後驗證但在 commit 必須成立；head 指最後 seq 的唯一版本。刪除 head 採完整實體邏輯刪除業務列，不留可導覽的理解 tombstone。 |
| **a_understanding_revisions** | id PK；understanding_id FK→heads ON DELETE CASCADE；seq INTEGER ≥1；predecessor_id NULL 僅首版；author = user；created_at。UNIQUE(understanding_id,id)、UNIQUE(understanding_id,seq)；複合前任 FK 保證同一 head。runtime 驗 seq=max+1，前任=原 head，無分支。內容／連結不可原地改寫；刪除 head 時整組清理。 |
| **a_understanding_revision_content** | revision_id PK/FK→revisions ON DELETE CASCADE；title NULL 或上述受限文字；body NN。所有活理解的 revision 具一份內容。此表不含 AI、原始上下文或來源快照。A 沒有逐版清理、匯出或自動保留副本，刪整份理解清所有版次；編輯現行版本不刪既有作者文字。 |
| **a_understanding_revision_sources** | understanding_revision_id FK→revisions ON DELETE CASCADE；ordinal INTEGER 1–32，連續；PK(revision_id,ordinal)（revision_id 指本欄）；role=start 或 later；state=bound 或 removed；source_id/source_revision_id 可空，bound 時兩者 NN，removed 時兩者必 NULL。bound 的複合 FK → source_revisions(source_id,id)，刪源由 native 先降為 removed 並清 ID，無 cascade 把理解刪掉。每版至多一個 start；同一版 bound source ID 不重複。removed 只保留所屬理解版次、ordinal、role 和狀態，不保存來源 ID、hash、標題、舊引文或快照。 |
| **a_understanding_followup_links** | id PK；moment_id NN UNIQUE FK→source_heads；understanding_id NN FK→heads ON DELETE CASCADE；target_revision_id NN；created_at NN。複合 FK→understanding_revisions(understanding_id,id)；moment_id 指向新建片刻的 stable ID；被刪片刻在同交易移邊（即使舊 source head 歷史標記仍存在）。指向理解當時版，顯示目前版是另一明確讀取；不成為來源證據。A 只從理解詳細頁「新增關聯片刻」使用，不可重綁既有片刻。 |
| **a_operation_receipts** | op_id PK；kind（見 02）；state=pending / committed / not_applied；prepared_at NN；settled_at nullable 僅 pending；result_kind/result_id/result_revision_id 可空、由 02 形狀規則限定；result_removed INTEGER 0/1。無外鍵使結果可被刪，但結果刪除必須同交易清可識別結果欄位。不得寫 renderer token、預期版本、來源集合、payload、輸入 digest、明文／錯誤回顯或任何供重放的資料。 |

**索引與完整性**：除 PK/UNIQUE，至少固定 heads(last_saved_at DESC,id DESC) 作 cursor；sources(source_id,state,understanding_revision_id) 作反查／移除；followup(understanding_id,created_at,id) 作理解端導覽；receipts(state,prepared_at,op_id) 作未結清查回。複合外鍵及索引名稱、型別、CHECK、觸發器／guard、空 manifest 為 A 獨立精確 DDL 的檢查對象。來源皆由 A native 固定讀取投影進出，不能把 SQLite FK 當成「現行版本與同意」的替代。寫後驗證資料圖符合欄位與語意，包含鎖定的理解 current head、有效來源、來源刪除與 receipt scrub；A 的舊語意 operation 必須經過同一受檢入口。

**複用方式**：見 [05 函式級 writer 抽取](05_WRITER_EXTRACTION_PLAN.md)。只抽已驗同 connection 的固定效果及同構檢查；v5 caller 所有 verifier/readiness 獨立保留，A 亦有完整、獨立的正向驗證。A 的舊 mutation、新 refs/links、receipt 與後驗必須在同一 Tx E。無法證明時停，不呼叫舊自行 commit 的 façade 再補關聯，不放寬 verifier。

**刻意不做**：搜尋／設定／提醒的額外 schema、匯出理解、外送資格、AI artifacts、跨理解來源、自動分析、舊資料升級、正式發布及 Phase 4。未來 B 只共用理解語意／保存契約；若需要新增候選、同 identity 升級或正式 AI provenance，分開審。
