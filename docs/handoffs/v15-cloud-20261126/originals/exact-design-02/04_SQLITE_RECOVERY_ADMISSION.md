# A｜SQLite 開啟、保全拒絕與恢復入口

**Exact Design 02 · 2026-11-25 · 限定補正。** 本文取代 01 對 hot journal「先驗完整契約、再恢復」的隱含順序。沒有核定任何原地恢復例外。以保全拒絕作為目前授權上限；所有產品案例 not_run。

## 1｜官方規則與保存基準的實際路徑

- [SQLite rollback-mode locking §4.1](https://sqlite.org/lockingv3.html#dealing_with_hot_journals)：資料可讀之前，pager 判斷 journal 是否 hot；涉及 journal 結構、super-journal 與 SQLite RESERVED lock，不是檔名或 app 鎖。若需要恢復，SQLite 自行取 SQLite 鎖、回寫／同步 DB，再處置 journal。應用層不能代作、不能先取 SQLite RESERVED lock 掩蓋 hot 狀態。[Atomic Commit §4、§9.5](https://sqlite.org/atomiccommit.html) 補充 DB 可能截回原大小，原檔配對／名稱必須保留。
- [SQLite URI 參數](https://sqlite.org/uri.html#recognized_query_parameters) 與 [SQLITE_READONLY_ROLLBACK](https://sqlite.org/rescode.html#readonly_rollback)：read-only 不能滿足需要寫入的 rollback，讀取可以因此失敗，不表示無恢復需求。immutable 會略過鎖與變更偵測；A 不借既有 immutable 路徑、不用 nolock。禁止以 SQLite 開啟、SQL pragma 或讀取 schema 替代 S0。
- [SQLite 開啟介面](https://sqlite.org/c3ref/open.html)：既有檔案的 READWRITE 不包含 CREATE；底層可能退為唯讀，須確認實際模式。不能假定第一條顯式 SELECT 才觸及 pager；driver connect／hook／隱式 pragma 也受許可邊界限制。
- [SQLite file format](https://sqlite.org/fileformat2.html)：固定檔頭可供排除或暫時辨識（magic、頁大小、讀寫格式、offset 60 的 user_version）；不是 catalog、consistency 或身份證明。SQLite 檔頭的 application_id 不是 Android application ID；可複製的檔頭或 receipt 不構成配對鑑別。

| Android 保存原始碼（src-tauri/src） | 唯讀核對事實 |
| --- | --- |
| android_m2a.rs:196–210、308–327、411–437、537–545 | ensure_ready_at → refuse_sidecars，拒 live／pending 的 -wal、-shm、-journal；verify_ready 先拒側檔與 mixed，再呼叫 verify_direct_fresh_v5，之後 runtime list。with_m2c_storage 另做身份、process-local OPERATION_LOCK、app-private path 及 reflection scope。這條 v5 Android 正常路徑不把 hot journal 交給 SQLite；Mutex 不證明跨程序排他。 |
| schema_v5_direct_init.rs:110–181、215–258 | verify_direct_fresh_v5 先校驗外部 receipt，再以 read_only(true)+immutable(true)、no-create 驗結構／object manifest。verify_direct_fresh_structure 自身無側檔分類；不能脫離上游借用。 |
| schema_v5_migration.rs:934–974；schema_v5_runtime.rs:219–244、1151–1159 | connect(path,true) 進 immutable helper，其一般路徑先查側檔；connect(path,false) 為 read_only(false)、create_if_missing(false)。A 不用這些 immutable probes 查看未恢復的原件。 |
| schema_v5_experience_write.rs:1495–1517、1523–1541、1576–1622 | writer 先 connect(rw、no-create)，才 verify_experience_database／pre_manifest，然後 BEGIN IMMEDIATE；寫後／結果未知另用唯讀路徑比較 manifest。不能用它作 A 零寫入 preflight；不把 v5 Android 上游側檔拒絕外推給每個低層 caller。 |

只讀程式及官方文件，未跑 SQLite、SQLx 或 Android。driver 的第一個實際 I/O、Android 的鎖與同步語意仍未動態證明。

## 2｜從檔案到 Ready：不可交換的順序

### S0 非 SQLite 檢查（拒絕優先）

先驗固定打包 identity、app sandbox、專屬目錄及固定檔名。做 bounded stat／安全唯讀檔案檢查：目錄與 leaf 不跨出信任根，不接受符號連結／重解析、異常硬連結／ownership／類型、缺失或替換中的檔案；檢查 live、正式 receipt、兩個 pending、主庫和 pending 的側檔及未知 storage artefacts。確實不存在才算 Vacant；I/O、權限或歧義不是不存在。限定長度讀 receipt 及固定 DB 檔頭，比編譯常數／候選版號與已核定發佈資料；可排除明顯不同的 identity、format、version、manifest 宣告或不受支援的檔案形狀。不讀 B-tree 判斷完整契約，不能證明 journal hot 或屬於主檔。即使合法中斷令固定檔頭無法通過，也先保全。

再取得既有 app-private OS 排他鎖（安全開既有固定 lockfile，不 create、truncate 或替換；只在真正 Vacant 的發佈流程可獨佔建立），並重查清單、檔案身分／路徑及 receipt/header。無鎖、發現非協作者、身份漂移或鎖不可證明時停。排他範圍從 admission 到所有連線關閉，由 native 管。鎖只序列化遵守它的 A 程序，不能防 root、同 UID 任意替換或非合作 writer；這些不在 R1 信任模型。SQLite VFS 鎖全程不改。

### S1 分類與准入，尚未 Ready

| 磁碟狀態 | 結果 |
| --- | --- |
| 真 Vacant、合成候選身份成立 | 正常首次發佈，不是恢復；只獨佔建立 A 固定新檔、同步與驗證。失敗留下 mixed 時保全。 |
| 正式 live＋receipt、無側檔／pending，外部條件通過 | 排他範圍內以真正 read-only、no-create、正常 VFS locking、無持久設定寫入的 connect 路徑做 S3。若同次檢查出現 journal 或 SQLITE_READONLY_ROLLBACK，停止、記錄分類，不在這次開啟自動升級連線。 |
| 初始化 mixed（任何已存在 pending、只剩 live 或 receipt 等） | blocked_publication_preserved；不對 live 或 pending 嘗試引擎恢復、不退休、不清理。 |
| 已知不相容或來源未知 | blocked_preserved；有 journal 也不開 SQLite。包含外部 receipt 宣告不符、錯 raw 版號、不合格檔案、WAL/SHM、額外／疑似 super-journal、明顯外來或損壞 journal。無法界定單 DB／正規 VFS 或信任條件也拒絕。 |
| 另一程序持有 A 排他鎖，或 SQLite locking 回 BUSY/LOCKED | storage_busy_preserved；不搶占、不改設定、不自動輪詢；任何已獲准引擎嘗試已開始後才出現的錯誤，按 S2 報可能有變動，不由錯誤碼推定零寫入。 |
| 外部閘通過，存在唯一與固定 live 名稱配對的 -journal | 只是 possible rollback，不以檔名、header 或「同 UID」稱合法 hot。**目前 R0：blocked_recovery_not_authorized，原樣保留。** R1 另核才可進 S2；外部有疑義仍拒絕。 |

### S2 僅限待核 R1：引擎交易恢復，不移動檔案

全部成立才可能准入：專屬合成 profile 由相同 A 合約初始發佈，信任模型中只有同一受控 Android sandbox/native writer 可修改這組原檔；無匯入／restore／遷移或外來原檔路徑；單一 SQLite DB、已定 DELETE/normal locking、預期 API/VFS、固定 live／receipt／既有鎖與唯一同名 journal、無 pending／WAL／SHM／super-journal 線索。已持跨程序鎖、重驗檔案身分，且能證明路徑不會由其他相容 writer 替換。原始 receipt/hash/檔頭只排除已知錯誤，不能證明內部 contract 或主檔與 journal 的配對。不能限縮可信範圍便不進；不以新增驗證 sidecar 或遷移解題。

若另核，唯一 recover-open 為原路徑**真正 read-write、no-create、正常 SQLite locking**。實作前須證明 driver open 沒有先改持久設定、attach、寫 DDL 或起業務交易。由 SQLite 決定是否及如何 hot rollback；不手動模擬、不刪／改名／截 journal、不搬離 DB 配對，不做 CHECKPOINT／VACUUM／REINDEX／repair。若 SQLite 報 I/O、busy、corrupt、readonly、恢復不明，或 journal 仍在／有非預期側檔，不進 S3；不自動重開或換模式重試。若引擎可能已改頁、同步、截短或處理 journal，而驗證未通過，標 **blocked_after_recovery_attempt**，保存當前狀態；不承諾原 bytes 未變，也不回造原始樣子。

### S3 完整驗證（乾淨連線，或獲准 S2 後）

同一次排他範圍內，在 recover handle 已正常關閉或已證明沒有恢復需要後，確認檔案仍屬同一 bundle、無異常側檔，以正常 locking 的 read-only 讀取。驗完整 user_version、DB singleton 與 receipt/profile/identity、actual object/table manifests／DDL 綁定、空 migration receipts、空 write guard、integrity/FK/content、舊 core provenance／projection／scope、A 六表圖及 links／receipts 形狀。空資料 manifest 只證明初始狀態，不能與有內容的 live 比 hash。全過才 Ready；合法引擎恢復≠A pending op 已解決。其後才依 02 查 committed/pending；只有讀回已證實、舊程序已不能再寫的 pending 才可用一筆非正文短交易結案。S3 失敗不寫業務或 pending，不對 UI 供資料；若 S2 已動過檔，明確通報已知變動／不明，保留恢復後狀態。

## 3｜每階段允許的檔案變動

| 階段 | 允許 | 禁止 |
| --- | --- | --- |
| S0／S1 拒絕或普通 preflight | metadata／bounded read；持有或釋放既有 OS 鎖，不改 lockfile bytes。 | DB、sidecars、receipt、pending、目錄項及 lockfile 的 app/engine 建立、寫、刪、改名與模式變更；既有未知狀態不補建鎖。 |
| Vacant 發佈 | A 固定且新建的 lock/pending/live/receipt 與初始化所需 SQLite 檔，受獨佔發佈及同步契約限制。 | 覆寫既有檔；不主動清進程中斷留下的 mixed state。 |
| S2 若另核 R1 | 只有 SQLite pager 認為必需的原 live DB、配對 rollback journal 回寫、同步、截短或引擎 journal 處置，及必要的目錄效果。 | 修復其他檔、備份/clone、任意建立新主庫或側檔類型、改持久 journal 模式、提交業務或 receipt、手工清除。實際違反即停，不可事後說「未改」。 |
| S3 與拒絕 | 正常 VFS 鎖下 read-only；fail closed 保留當前狀態。 | 自動 fallback 到可寫、repair 或 checkpoint；S2 已有變動不在此假裝撤回。 |
| Ready 且另有合格操作 | 02 定義的業務交易／非正文狀態及必要 SQLite 交易檔。 | 在 admission 或驗證前寫業務，把 recover-open 當 Tx E。 |

零寫入指被保護檔案的 bytes／長度、檔案集合、目錄項與 app/engine 持久寫入；不把 OS 一般唯讀存取的 atime 或瞬態鎖算成應用寫入。未在實際 VFS 證明前，不把此設計約束當成 runtime 證據。

## 4｜衝突與最小 Founder 取捨

**無法同時保證「所有內部不相容的 DB 在拒絕前零寫入」與「先在同一原件上讓 SQLite 恢復，以便之後讀到一致的內部契約」。** 外部 receipt 和檔頭可能都符合，journal 也能由 SQLite 處理，但實際 catalog 或 singleton 之錯誤只有在取得一致狀態後才可見。外部身分吻合只能限縮風險，不能免除衝突。

| 選項 | 可保證範圍／代價 |
| --- | --- |
| **R0 嚴格保全（現在的授權上限）** | journal 一律不原地開啟；錯誤或未知走零寫入，仍可驗正常關閉重開，或已提交無側檔的 ACK 查回。合法 hot 的原地復原會受阻，不能寫成通過。 |
| **R1 可信 A 合成 profile 的原地 engine rollback（待核）** | 所有**事前可拒絕**的情況仍保全；僅對可信 A 合成原檔，容許 S2 rollback 的最小必要寫入發生在終局驗證前，事後可能仍拒絕。不含 real profile、其他候選、業務 replay 或遷移。若實際開啟路徑未證受限，不進。 |
| R2 原件封存、對合成快照離線鑑別（另案） | 原件可保留，但要另外證明 DB+journal 成對的穩定快照，另批複本存放、清理與人工驗證。複本結果不等於原 profile 恢復、不可自動搬回；不是 A 的第二資料庫或本輪產品範圍。 |

需 Founder 明確選：**維持 R0**，還是**另准 R1** 這個「內部契約尚未完全可讀前，容許 SQLite 自身 rollback，之後仍可拒絕」的限 A 合成例外。本包不選、不生效。若原件一律不容許終局拒絕前的寫入，必須接受 R0 不做原地 hot recovery。具體 not_run 案例見 [03 驗收](03_VERIFICATION_AND_STOP.md)。
