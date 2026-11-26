# A｜交易、恢復與生命週期

**精確設計 02 · 2026-11-25 · 待審。** 以下操作名、資料形狀與結果為契約，不是已註冊 IPC 或實測結果。

**先分清進入條件：**[04](04_SQLITE_RECOVERY_ADMISSION.md) 的 S3 通過才可查回或啟動 Tx P/E；status/list 不是以 read-only 名義執行引擎恢復的入口。R1 未核定前帶 journal 就保全，不判 not_applied。core primitives 與同 transaction postcondition 依 [05](05_WRITER_EXTRACTION_PLAN.md)。

## 1｜Native 操作面

| 操作／命令候選 | 輸入、輸出與邊界 |
| --- | --- |
| v15_a_storage_status | 不接受 profile/path。回 ready（contract ID、schema version、synthetic-only、capability）或 blocked_preserved（白名單原因）。啟動／發佈另按 01／04；故障文字不回傳 SQL 或內容。 |
| v15_a_understanding_list / get / by_moment | 目前先列表／雙向詳情；頁長 1–50（預設 10）、opaque cursor 綁同排序與範圍，依 last_saved_at DESC,id DESC。get 指定 understanding ID 與可選 revision ID；指定不屬於同 head 時拒絕。by_moment 用讀取一致性驗 current links、歷史精確來源／所屬理解，不把旁支當再次探索，不從已刪來源開錯頁。只回必要一次資料，不返回殘留已刪內容。 |
| v15_a_begin_edit | 本機當次操作，開理解／片刻編輯、或刪除影響預覽，回 native RAM-only 的 edit handle，綁 profile/process/目標版本／操作，一次提交；可讀當時版本但只可針對已重讀現行 head 提交。切畫面可保留；離開明確放棄、失效或已完成即作廢。只開頁不落地、不觸發其他流程。 |
| v15_a_submit | 只接受 active handle 下的一次指定操作與精確參數（下表），native 生成 op_id、執行兩段提交、從已確認狀態回讀。客戶端**不能**以 op_id 呼叫／重放 mutation。查不到、過期或異用途的 handle 一律拒絕；新一輪操作必須使用新 handle。 |
| v15_a_operation_status / list | 只查當前 app-private 的非正文提交狀態；list 可重開查最近未結案，包含 op_id、kind、時間與已核對結果 ID。沒有獨立查詢、搜尋、匯出 receipts 的產品入口。查不明時不可直接返回成功。 |
| v15_a_acknowledge_operation | 只對目前已核對的 terminal receipt，由 UI 明確告知後結案，刪除該 receipt；不能清 pending 或未知。丟掉已結案 ID 後再查應為 not_found，不能變成可重送。 |
| A 專用既有離線問答 | 保持適用的既有語意與操作，採 A exact adapter；不得讓舊 mutation endpoint 另開 connection 繞過 A gate。B 的 provider／Key／IPC 不在 A 註冊。 |

### 可提交的 kind

| Kind | 必要前置 / 使用者意圖 | 同一個效果交易要做的事 |
| --- | --- | --- |
| u_create | 有效 handle，title/body，1–32 條當前精確片刻版次；一條 start，其他 later；不接受 removed。 | 新 head／首版／內容／來源集合。提交返回 understanding_revision。 |
| u_revise | expected current understanding revision 與本人輸入。現行來源按序給 keep(舊 ordinal) 或 add(source ID, expected current revision, role)，不列即從**新版本**移除；至少一條。 | 建立新 revision、複製引用或非內容標記、加入新來源並以舊 head CAS 更新。keep 僅可指該 expected 版的已有邊；最多一個 start，新 source 在同版不能重複。允許全數 keep(removed)，不以來源已移除判斷理解無效；歷史版仍保留。 |
| u_delete | 最新理解版次、確認時仍有效的影響預覽。 | 清 head、所有 revisions/contents/refs、導覽關聯、相關結果欄位；片刻與其原問答維持。提交返回 deletion_ack。 |
| moment_create | 新合成片刻；沿用既有文字與 actor 邊界。 | 一次既有 canonical create 的 A 分支，不跟任何理解隱式相連。提交返回 source_revision。 |
| moment_create_linked | 從明確理解頁開始，確認新片刻及目標理解 id/expected revision；選「不要連結」即不使用此 kind。 | 同交易建立片刻／原生投影及最小 follow-up link。目標消失或版次更新即全無；可取消，另用新的明確操作單存片刻。理解 head、理解舊 refs 不變；若之後撰寫新理解版本再明確加入來源。 |
| moment_correct | 片刻 stable ID、expected current revision、明確更正文；沿用既有 lifecycle。 | 建立既有片刻 revision、原生投影並維持既有衍生失效規則；A 引用保留舊版，來源狀態讀取重新判斷，不能重綁。提交返回 source_revision。 |
| moment_delete | 最新片刻版次及原生影響預覽仍有效。 | 先把該 source 的 A refs（各理解、各版次）全部變 removed、清兩個指標，刪 follow-up link；同一交易執行既有 v5 語意的來源內容及既有相依處置，清 receipt 結果識別。理解 head/本人內容一字不動。提交返回 deletion_ack。 |

連結建立時只改導覽圖，不把「沿著理解記的新片刻」偷偷改成交叉時間分析。起點被刪時仍可維持原 start 標記；修訂可保留或移除舊標記，但不人工製造成「史上曾有另一片刻」。提交版需至少一個有效來源或既有缺失標記；在 A 不用改寫主來源來解決缺失。若舊引用從 valid 突然變 removed 或從 current 變 historical，提示來源已變，重新查看確認；已確認的缺失 tombstone 可以繼續提交。同一交易讀檢有效版本，不憑過去展示。

刪除預覽包含目前 head/revision、影響的 A ref/導覽關聯與既有資料類別／數量、保留的本人文字與可能的本人引用、適用匯出界線。native RAM token 綁該集合與範圍（不在 DB 或應用日誌保留 payload/digest）；提交前在效果交易以當時集合重驗，變更即 stale_preview，不接受舊確認。刪除理解不刪其他理解、片刻、問答；A 尚無小提醒或共享 AI 記錄，不能虛構已處理的內容類型。

## 2｜兩段流程，只有第二段是產品資料 commit

1. **使用者按下保存或刪除確認**：UI 防重複；handle 在 native 設為 submitting，凍結並綁該次 payload 和目標欄位。相同 handle 且同有效輸入的並發請求共享該次進度／結果，不能再寫一次；同 handle 不同資料拒絕。不把文字 digest／signature 保存到 operation table 或 crash log。發送中不可改內容；重開只查 receipt，不恢復 handle。
2. **Tx P 登記**：在排他 gate 下先確認 Ready、capability、limits、沒有未核對 pending。BEGIN IMMEDIATE，插入只含 op_id / kind / state=pending / prepared_at 的 receipt，commit。Tx P 未能確認，不啟動 Tx E；僅查已存在狀態、必要時要求使用者處理。此時沒有理解、片刻、導覽或來源改動。Native key 只在當次 active handle 綁定；op_id 是唯讀查回指標。
3. **Tx E 效果**：同一 owner 和 gate，BEGIN IMMEDIATE，重新驗精確版本、型別、完整輸入、來源狀態、receipts 尚 pending。執行 kind 全部資料改動及既有投影；驗 schema、FK、讀取投影、刪除後之缺失／清理、safety 等式；把相同 receipt 從 pending 原子改成 committed，寫 settled_at 與受限結果形狀，才一起 commit。沒有 ACK-before-commit；Tx E 是唯一業務資料 durable 邊界。
4. **讀回後才顯示已存**：在同 owner gate 由新驗證讀取 receipt＋既有結果，對照整份現行投影、版次、來源集合（刪除則對照不存在及清理結果）。若 commit 丟 ACK 或 readback 一時不明，進 status；不在新 request 路徑自動重送。精確失敗可保留這個程序內的草稿，改完用新 handle 再提交，不能重入舊 Tx E。
5. **not_applied 必須有反證**：Tx E 已確證 rollback，或新程序已獨佔且確認舊程序無寫入能力、04 S3 已通過，核對 receipt 仍 pending 且沒有在途 transaction。若需引擎恢復，必須另核 R1 且完成 S2→S3；R0 拒絕不能當作 rollback 已證實。以受控的本機短交易將 pending 轉 not_applied（只有 metadata）。如果 SQLite 開啟、鎖或持久化狀態不能證明，回 unknown，**保留**原狀態、阻止新的 A mutation；不得以按 X 或 TTL 清除未知並寫第二次。

### Durable 狀態與回讀

| DB 形狀 | UI / API 讀取之意義 |
| --- | --- |
| pending：無 settled_at、無結果，result_removed=0 | 同一 owner 尚處理時為 in_progress；無法確認時為 unknown。不可直接當未保存。唯讀 status 不執行業務資料 mutation。 |
| not_applied：有 settled_at、無結果，result_removed=0 | 確認沒有 Tx E 效果，已存紀錄未改；草稿仍在這個程序可再編。若程序已回收，明示文字沒有保存、需要重寫。 |
| committed + understanding_revision / source_revision：有 settled_at、result ID、result revision ID | 讀取目前與當時版本，區分 committed_current 或 committed_superseded。後者可以導航到現存的詳細頁，不覆寫它、不以舊 receipt 作存檔。 |
| committed + deletion_ack：有 settled_at，無目標 ID/revision ID | 回 committed_deleted，證明該刪除操作已完成；不保留被刪識別供復原。回讀若不自洽為 unknown，保留證據。 |
| committed + removed result：result_removed=1，result_kind、ID、revision ID 全 NULL | 被確認曾有提交，但原結果之後被刪且 ID 在刪除交易中抹去；回 committed_result_removed，不能復原正文／導航。只適用原來結果含 ID 的提交。 |
| 找不到 op_id | 回 not_found。可能根本沒進 Tx P 或已結案；**單憑找不到不能推定未保存**，不能重放。先由可用已存資料重新核對，保持未確認。 |

**保留與清理（A2-d 精確提案）**：只保未結案，最多 64 筆、同時最多一筆 pending。狀態結果限上述字段、不得由應用層備份、匯出、索引全文或用成傳輸歷史。已核對 terminal 可透過讀回畫面「完成」或明確確認提示即刪該筆；無背景 TTL，不將未確認紀錄用到期變成「未存」。達上限阻止新增 metadata 與新 mutation，要求處理已知 terminal，不能覆蓋或自動清 pending。刪 U 或片刻時，連結衍生的 create receipt 只指新片刻，不保留理解識別；所有與被刪結果 id 相同的結果欄位同交易清掉。刪除失敗不得提前 scrub；不能用 metadata 查出已清的正文。這是合成工程候選的容量／保留提案，不是正式隱私政策。

### 中斷／重開演練

| 中斷點 | 重開核對（重開當次須通過 04 S3，不沿用過往 Ready） |
| --- | --- |
| 明確提交之前／Tx P 確定未落地 | 無成功通知、沒有產品資料；單純 not_found 不足以證明發生了哪一個分支。程序內草稿不重開。 |
| Tx P 已 commit、Tx E 尚未開始 | S3 完整通過、獨佔且已排除舊 writer 或在途交易，核得 pending 才標 not_applied；沒有自動重放。 |
| Tx E 中間／commit 之前 | 若無側檔且 S3 已證實 pre-effect，pending 可按規則變 not_applied。帶 journal 時 R0 保全；另核 R1 才可由引擎 rollback 後驗證，無部分 effect 才結案。狀態不明時保留 pending。 |
| Tx E commit 已成功，讀回／ACK 之前被關閉 | 共存的產品結果與 committed receipt 由 native 查回；只讀回原結果，不再保存；若其後被修訂或刪除，照表分流。 |
| 任何 DB readiness、manifest 或合法 journal 核對未成立 | 依 04 分為 blocked_preserved／blocked_recovery_not_authorized／blocked_after_recovery_attempt 等原因；不把初始化 mixed 或 engine rollback 當業務 pending，不開第二 DB。 |

## 3｜可見狀態與生命週期

| 事件 | 舊版／來源連結 | 本人內容與 UI |
| --- | --- | --- |
| 片刻有後續更正 | bound ref 仍是當時的 source revision；舊版仍有規範內可讀內容時，「當時」與「目前」分開讀。無權讀舊版時標不可核對，而不是回傳目前正文充舊。 | 理解不自動改、來源不是 AI 推斷證據。 |
| 唯一來源刪除 | 清系統來源內容、A 的引用指標；留下非內容 removed 標記。既有核心 writer 按原契約保留的來源墓碑／revision metadata 不因此改寫，且不供 A 顯示、查回或恢復正文。 | 理解仍在且可改，來源區顯示「這個片刻已移除」；理解不是「無效／錯誤」。在提交保留已知缺失邊。不再有來源跳轉。 |
| 多來源之一刪除 | 只有受影響邊轉 removed，其餘原封不動；若原 start 消失，不能將 later 自動升格。 | 顯示「有些來源已無法核對」；本人理解維持可讀、可改、可刪。 |
| 本人曾在理解或舊版手寫／貼入引用 | 不能用全文替換、比對或刪掉本人創作來冒稱全抹除。 | 刪片刻前明示本人文字仍會留在理解／舊版本。可修改目前理解；若要清所有本人版本，在 A 請另行刪整份理解。 |
| 刪理解 | head/版次/內容/引用與導覽不存在。舊深連結回「已移除」不轉向其他理解。 | 不清原片刻、已存問答或其他理解；只更新必要提示與快取失效。 |
| A4 切頁、背景、放棄、程序回收 | 同程序且有效 handle 可續；明確關閉草稿／過期／已提交後回收 handle，來源 changed 顯示衝突。考慮 Android 程序回收與 webview 重新建立，不承諾仍可續。 | 無 durable 草稿；提交前提示「還沒保存，App 被關閉後需要重寫」。重開只支援 native receipts；保留的結果不是草稿。停留／放棄／重開零模型呼叫。 |

「清理」是已驗證的產品資料路徑與邏輯存取清理，不承諾物理抹除、Journal/OS 扇區的法證不可恢復、外部匯出的撤回或 provider 端刪除。前端依 mutation 後結果失效已載舊摘要／詳情，不以快取重新展示；debug 日誌、analytics、崩潰報告不得包含真實或合成 fixture 正文。
