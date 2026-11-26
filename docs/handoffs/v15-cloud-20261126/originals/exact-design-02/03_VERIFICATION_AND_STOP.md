# A｜驗收與相容性矩陣

**精確設計 02 · 2026-11-25 · 本輪僅文件檢查，以下產品驗證均未執行。**

## 1｜相容性與拒絕

| 測試輸入／情境 | 必須取得的結果 | 不得誤報的結果 |
| --- | --- | --- |
| A 的全新合成 sandbox／真空目錄 | 獨佔初始化，精確 15001、A contract／receipt／manifest、舊語意子集＋A 六表。重開不可重新初始化，已存資料依 02 讀回。 | 不只以 user_version 合格、不把 app 私有路徑當成正式資料授權。 |
| A 遇上 v4/v5、較新 schema、錯包 identity、未識別來源或錯誤 receipt | 拒絕並保全，測試前後 live/pending、receipt、journal、資料與路徑狀態。啟動拒絕不能先拿舊資料開 writer。 | 不遷移、import、重命名、清空、修復、create-on-error 或在新 DB 「成功打開」當替代。 |
| 只有 live、只有 receipt、初始化 pending 或 mixed publication | 阻擋保存；注入每個發佈中斷點，確定無 overwrite、自動續跑或 rollback journal 誤判。有效使用中的 hot journal 另驗。 | 發佈 receipt 與非正文 operation receipt 不共用恢復狀態。 |
| journal 與第二個程序 | 依 04 先做非 SQLite 檢查並重驗既有排他鎖；完整驗證在無側檔的乾淨路徑，或另核 R1 後。案例 R-01～R-07 均 not_run。 | 不把外部 receipt 當完整契約，不以 immutable、Rust Mutex 或鎖檔存在代替引擎鎖；未知不原地恢復。 |
| 原 v5 fixture、舊 verifier 與舊入口 | 既有獨立 v5 路徑同樣的精確 readiness、片刻、合成離線問答、重讀、修訂與刪除行為；不接受 A profile。依原 v5 fixture 輸出比對。 | 不宣稱已將舊資料或 3101 搬到 A、亦不更動舊 APK。 |
| A 中既有概念的等價性 | 同一合成故事走 A adapter 與基準 v5 runtime，比對既有可見狀態、作者／譜系、當時版本、失效／清理；A 增量另檢。 | 只在 A 建新資料不等於舊 writer 相容證明；不用放寬 schema manifest 或換假 DB。 |
| Capability／身份矩陣 | A 方可取得列名的 A commands；desktop、既有 Pilot、M2-D、production 不能調。驗實際合併配置與允許清單；沒有 provider/key、通用 SQL/路徑、備份/transfer、不預期的網路能力或新理解匯出。 | 不把建置 config 或編譯通過當作真實權限證據；真正包和裝置另驗。 |
| 每一平台獨立 | Windows host 適用項只證 native contract 與 repository checks；Android ARM64 候選打包後再核查配置，Pixel 7／Zenfone 10 分台、同一明確候選測常態關閉／重開。 | 不聲稱另一台／舊簽章延續、OS 電源故障、release、升級、真的 AI 或長期資料安全。 |

### 限定補正：SQLite 准入與副作用（全部 not_run）

對每個 case 固定快照：DB、journal、其他側檔、所有 receipt/pending、lockfile、名稱、大小／hash、路徑與 app/engine I/O；控制其他 writer。一次能區分「根本未進 SQLite」「只讀」「已允許 engine 嘗試」；不以成功／錯誤碼代替證據。限合成 fixtures。R1 欄是另核後才有的未來驗收，不是現行 fallback。

| 案例 | 前置與可分辨結果 | 狀態 |
| --- | --- | --- |
| R-01 合法中斷 | 真正已發佈合成 A 由 SQLite 自然留下可驗證的 hot journal，不偽造檔名代替。R0 阻擋、檔案 unchanged、未開 SQLite。若 R1 已核：只讓引擎回到正確前態／清 journal，無業務 replay，最終 S3 及 receipt graph 驗證才 Ready。正常無側檔的 commit-before-ACK 另驗 committed。 | not_run |
| R-02 外部錯 contract／manifest＋journal | 外部 receipt 宣告、檔頭或 identity 與 A 固定值不同。兩政策在 S0 拒絕，未開 SQLite、不新增鎖或側檔，保留所有原件。 | not_run |
| R-03 內部錯 contract／manifest＋journal | 外部閘可通過，錯誤在一致 DB 中才能查。R0 阻擋且零寫入；若 R1 已核且合成原件符合信任模型，engine 可先 rollback，再由 S3 拒絕；沒有 Ready、Tx P/E、receipt 結案。含 terminal blocked_after_recovery_attempt 及原件可能已變的證據。 | not_run |
| R-04 不明 journal | 來源／配對無法依信任模型限縮、格式明顯異常、額外側檔、WAL/SHM 或 super-journal 線索，無論 R0/R1 都不進 SQLite、不移動或清檔。只靠檔名或看似正確的 header 不算來源可信。 | not_run |
| R-05 第二程序／SQLite contention | 第一個程序持 A OS 排他鎖，第二個不准進 SQLite，根目錄無由它建立的檔；另測已獲准 SQLite open 的 VFS BUSY/LOCKED 走停止，不取 RESERVED 作規避、不以清鎖／重試處理。保留實際寫入或未嘗試證據。 | not_run |
| R-06 恢復後驗證失敗 | 若 R1 已核，engine 執行 rollback 但完整 FK、projection、receipt 或 A graph 驗證失敗，保留恢復後現況，不做 post-validation mutation 或自動修復。R0 此分支不可達；其阻擋不能當引擎／驗證通過。 | not_run |
| R-07 初始化 mixed／唯讀需要恢復 | mixed 即使有看似熱的 journal 也絕不 open；清潔分支遇到 SQLite 要求 rollback／側檔在過程出現，停止不自動切換到 rw。另用證據區分讀鎖／driver 無持久寫入與引擎嘗試。 | not_run |

## 2｜後續驗收用例與失敗判準

| ID | 實驗 | 成功條件 |
| --- | --- | --- |
| A-X01 | 本人建立理解、留空標題、保存、正常關閉重開、改一版。 | 原生讀回 1 個 head、2 個 revision／內容、精確參考、正確前任／作者；fallback 只是畫面；片刻↔理解都找得到；AI 呼叫為零。 |
| A-X02 | 開理解後新建合成片刻，確認連結但維持理解原樣；另一次再提交修訂。 | 新片刻＋link 原子成立，head/refs 不變；跳回理解當時版或明確去現行版。新 understanding revision 才加入 later source。取消、返回、過時的目標均無默默連結；可重新明確選擇單存片刻。 |
| A-X03 | 新內容 commit 前後注入中斷，分別測 u_create、u_revise、moment_create_linked、u_delete、moment_delete。 | Tx P 與 Tx E 分開觀察；after-effect-before-ACK 不重放，所有結果數及引用正確；mid-effect rollback 後沒有孤兒／部分清理；metadata 不含字句、雜湊、舊 request payload。 |
| A-X04 | 兩版編輯競爭、同 handle 雙按、同 handle 改文字、用 op_id 或舊 process handle 重送。 | CAS 拒絕舊前任，不能覆寫新版；雙按只一個 effect；異 payload／舊 handle／op_id 作寫入拒絕；重開只能讀狀態。 |
| A-X05 | 未完成／未知／已提交但被更新／已刪結果／刪除操作／已結清，逐一查回。 | in_progress、unknown、not_applied 與各 committed 分流完全符合 02。not_found 不判未存；未確認時不復活內容。測至少一次先留未結清 committed 再開新合法操作，能證 superseded/removed。 |
| A-X06 | 64 筆未結清邊界、有 pending／有 terminal；結案乃至被刪結果。 | 第 65 筆拒絕，不覆寫、不 TTL 掩蓋；同時最多一個 pending。已核對 terminal 結案才清，刪除時 scrub 同交易，不保被刪內容、識別或可回放 input。 |
| A-X07 | 有理解同時引用舊片刻／現行片刻；修改片刻；刪除一項、再刪唯一剩餘；再次編修理解。 | 原 refs 不重綁，只有受影響的變 removed；清除應刪的來源文字、副本，以及 A 的來源引用指標；既有核心墓碑不算可重新讀取的內容。即使全部 removed，仍可編修；新來源只接受有效精確 current revision。本人自寫摘錄仍可出現於當時理解且已提前告知，不許拿它宣告系統保留了來源副本。 |
| A-X08 | 在刪除預覽和確認間，理解來源／影響集合變化；另試取消、刪理解與舊 deep link。 | stale 需重新預覽；取消全無異動。刪除理解含歷史、links／引用和可識別結果欄位，片刻及原問答保持；返回、快取、readback、重開、既有片刻匯出不復現清掉的內容。 |
| A-X09 | 輸入、切頁、切背景、程序回收、正常重啟、明確放棄。 | 同程序且畫面仍在可接續；重啟只讀已存與尚可查的 metadata。UI 不謊稱草稿已存、無自動重送或模型。切 app 回來不被當成已完成真正重開測試。 |
| A-X10 | 內容限制、錯頁游標、不合來源、512 字節標題、UTF-8 邊界。 | 建議限制與表字典一致；無靜默截斷、錯頁、作者或角色混淆；不同 understanding 的 revision 不可串讀。拒絕前或回滾後 native graph 保持原狀。 |

### 限定補正：最小 writer 故障證明（全部 not_run）

| 案例 | 注入點及原子性證據 | 狀態 |
| --- | --- | --- |
| W-01 | 成功的 Create core 後、link 前強制失敗；canonical head、content/provenance、projection、link、guard 均不留新資料，Tx P 尚可查但沒有 committed。 | not_run |
| W-02 | A refs / link / receipt scrub 後，在 Delete core 清理中失敗；舊 source、相依、理解內容、原 refs/link 及舊結果識別全部 rollback。另測 stale preview / NotFound 不可被當 A-only 成功。 | not_run |
| W-03 | core／A graph 已寫及 receipt 已轉 committed，但還沒 COMMIT，注入失敗；證全部效果與 receipt transition 回滾。另在 durable COMMIT 後、ACK 前中斷，證合併結果及 receipt 一次讀回、不重送。若實際留下 journal，按 R0/R1，不能跨條件照讀。 | not_run |
| W-04 | 在 Migration 和 DirectFresh 基準重跑固定 verifier、前後態分類與既有 caller regressions，A/v5 交叉拒絕；檢查既有錯投影、不同 origin、錯 manifest、非空 guard 和未知 outcome 仍全 fail closed。詳細名稱與 callers 見 [05](05_WRITER_EXTRACTION_PLAN.md)。 | not_run |

### 證據分層及停止

1. **未來 scope 經治理及另行授權後才開工。** 先固定一個允許寫入的隔離位置、精確來源與驗收矩陣。從 exact DDL/readiness + 無 UI 交易證明著手，再接最小 UI；測得 Tx E 無法包含現有 writer 與新資料時停，不以分段補償降格。先做單一 fixture 的正常保存／刪唯一來源，再擴到故障矩陣。
2. **Host 才能聲稱的層**：僅合成 fixtures；生成 A schema DDL、object/table manifests、對應 digest 和 receipt 的精確來源綁定；針對 v5 邊界做回歸與故障注入，再在授權環境依 AGENTS 所列原生 Windows canonical verifier 執行。保留既有 missing repo-local pnpm / exit 1，不推論通過，不替換或放寬檢查。任何新失敗、未知、資料不一致或越權能力停止並如實留證。
3. **候選與實機分開**：另批 build／簽章、實際包與兩手機；確定 app ID、各 build 選項與 APK hash，核對實際 Android route／capability／生效設定後才做退出、背景與 OS 回收、正常重開。裝置 evidence 不外推到 B 模型或 C。在同一新合成候選已接 trigger 的層加故障測試，不在原紀錄上演練。
4. **暫不做**：裝新 app、進入原實機 profile、真實 AI、實際提供真實資料、受邀分享、資料匯出擴張、原 R3 decision／ADR-0013 修改、M2-D／036 操作、遷移、發佈。上述各項不是本輪授權。

## 3｜唯讀查核依據與已知阻力

以下是 Android Pilot 保存 worktree 路徑，相對路徑只作技術引用。檔名和程式邊界可讀，不等於目前遠端或 HEAD 綁定已重新證明。

| 基準來源 | 對設計的約束 |
| --- | --- |
| AGENTS.md、AI_CONTRIBUTOR_GUIDE.md、docs/00_Index.md、Architecture 01、ADR-0012 | 繼續單一 authoritative app-private storage，固定來源與版本、idempotent readback、失敗保全；不把 design scope 稱為已允許不同 schema、資料遷移或 Profile activation。 |
| src-tauri/src/android_m2a.rs:315–380、483–545；src-tauri/src/schema_v5_direct_init.rs:110–155 | 現有 v5 路徑用固定 profile path、receipt、exact schema、空遷移 receipt 和 scope。A 要獨立權限、版本和 manifest，保留原拒絕，不從 runtime 路徑抄一個「遇壞重建」流程。 |
| src-tauri/schema/schema_v5.sql:62–105、148–173；src-tauri/src/schema_v5_migration.rs 的 TARGET_SCHEMA_VERSION／TARGET_TABLE_MANIFESTS | 既有 source revision 有可引用的複合唯一鍵，但 v5 artifact vocabulary 是封閉的；A 型別要自己的表示與語意驗證。 |
| src-tauri/src/schema_v5_experience_write.rs:1341、1442–1489、1536；src-tauri/src/schema_v5_android_reflection.rs:134、204、663 | 原生 writer 有自帶 verify、projection、transaction。最大的實作風險是跨新舊邊界把操作切成兩次 commit；A 只複用經審的 transaction-local primitives，不呼叫會自己 commit 的舊 façade 再補寫。 |
| src-tauri/src/lib.rs:59–110、src-tauri/Cargo.toml 的 Android features、src/android-m2c/androidM2CStore.ts:5–50 | 既有 online 與 offline 已有身分分流與命令 gating；A 要用新的獨立契約，不把 provider/key command 或既有 config 漏過來。 |

## 4｜下一個核定點

**請審這個設計，不重開已同意的 A1–A4。** 特別看兩組本輪才出現的精確提案：

1. **候選 namespace／版本／manifest 與相容拒絕**：固定 A 專用 profile、暫定 user_version 15001、不得升級、獨立 exact verifier／capability。確認需如何依 ADR-0012 受治理審閱 A 的獨立合約（不修改既有 v5／production 決策）。
2. **A2-d 精確落點**：Tx P + Tx E、跨程序排他、最多 64 筆不含正文的未結清 receipts、明確結案刪除、無 TTL 靜默清除。確認內容刪除後 scrub 的範圍與阻擋規則。A3 的語意已同意，這裡審的是能否真的原子做到。

另有一個最小恢復取捨待核：[04 的 R0／R1](04_SQLITE_RECOVERY_ADMISSION.md)。R1 不是本輪預設或已批准；若不另核，仍不得對 journal-bearing 原件做 recovery-open。也須審 [05](05_WRITER_EXTRACTION_PLAN.md) 的函式級抽取與未證風險。只有精確設計獲明確核定、且另外取得 source／repo／Git（若需要）的相應權限，才交給本地 Windows 開發。命名碰撞、DDL hash、Android 鎖語意及 writer 組合的可行性在授權的前置上證明；不確定先停，不把空白說成已完成。
