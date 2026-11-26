# A｜函式級 writer 抽取計畫

**Exact Design 02 · 2026-11-25 · 設計待審。** 以下為保存基準原始碼的唯讀追查。函式簽名形狀、抽取順序與驗證均未實作，案例全部 not_run。

## 1｜現有呼叫鏈與不能跨過的邊界

| Caller（src-tauri/src，行號依保存工作樹） | 路徑與受影響程度 |
| --- | --- |
| schema_v5_runtime.rs:17、1035–1122、1197–1290 | create_experience、update_experience、delete_experience、import_experiences 走 execute_disposable 的 Migration 契約；create_experience_direct_fresh、mutate_experience_direct_fresh 走 execute_direct_fresh。create façade 在提交後再從 path 讀回；不是 connection-local helper。 |
| schema_v5_founder_activation.rs:896、920、944 附近 | Desktop Founder schema-v5 activation 經 migration-origin runtime；固定 activation/readiness/IPC 邊界不改。受共用 primitives 的語意回歸影響，但不是 A 入口。 |
| android_m1.rs:306；android_m2a.rs:537、612；android_m2b.rs:113、151；android_m2c.rs:148、217–260 | M1 使用 migration-origin create；M2-A create，M2-B 自己的 with_m2b_storage/create，以及 M2-B/M2-C native mutation，使用各自隔離的 direct-fresh 控制。M2-C/離線 Pilot/connected Pilot 共用 M2-C 閘及對應 build 分流。不改其身份、側檔保全、使用者回覆或 reconcile。 |
| schema_v5_android_reflection.rs:134–155、204、237、663 及下游 | verify_scope 是 M2-C artifact/依賴白名單，不是 integrity/投影替代。離線問答及連網保存走自己的 writer／snapshot，非本次 Experience 抽取的直接 caller，但須回歸來源更正／刪除造成的原有失效與讀回。A 的既有離線問答若需另一組 primitives，須單獨證明；不能把其 v5 writer 直接用到 15001。 |
| schema_v5_experience_write.rs:1626–1654、1523–1624 | execute_disposable / execute_direct_fresh 都自行開 connection、驗證、開始交易、commit／rollback、關閉、reopen reconcile。傳它 A path 或回來再補 link 都不合格。 |

## 2｜沿同一 connection 可抽取的 primitives

| 現有函式／區段（schema_v5_experience_write.rs） | 擬抽取形狀與不可丟失的行為 |
| --- | --- |
| validate_identifier/content/timestamp/input :111–178；current_source :375–398 | 純驗證及同 connection 的目前精確來源讀取。維持 ID／字元／時間原驗證、不碰來源其他版本、不引入 A 對理解文字的限制。A caller 檢其新欄位，但不代替 v5 validator。 |
| insert_source_revision、link_user_provenance、create_source :761–902 | 固定 SQL，生成既有 revision ID、history/content、user provenance、head、experience_entries projection、碰撞檢查與原有故障點。共用 create 原子片段；不複製一份變體。可傳入私有已驗交易與既有 context，不能取得 path、建新 connection 或任意 SQL。 |
| collect_ordinary_dependents、append_source_invalidation_event、invalidate_ordinary_dependents（:491、587、635 起）；collect_historical_questions_for_source、delete_historical_questions（:904、1052 起） | 同 connection 的依賴檢查與效果。保留歷史兩個表示的 parity、跨來源拒絕、原事件／tombstone 行為以及 0009 清理。A 不開啟歷史功能；仍驗 core 相關表／依賴在其允許範圍（預期空亦須驗），不可假設一定空而省略既有安全檢查。 |
| apply_command :1107–1439 的 Create/Update/Delete 三個分支 | 首選把分支拆為三個 transaction-local typed effects（擬名 apply_create_core / apply_update_core / apply_delete_core），原 v5 dispatcher 改為等價委派。Update 保留 stale/notfound、時間嚴格前進、head CAS、作者／前任、失效及 projection；Delete 保留所有 legacy/normalized 相依清理、source tombstone 與原生正文／projection 清除。Import 分支與批次授權留原 v5 caller，A 不曝露。 |
| insert_guard / remove_guard :715–759；verify_source_projection :272–344，及 current_content_checks / integrity_checks | 非交易持有者：guard insert/remove SQL 與 projection 檢查可抽取，前提是 A DDL／trigger 形狀精確相容。由各 caller 驗 guard 在交易前、寫完移除後為空；不能關掉 trigger、以假 token 或現存開關跳過。需先核清各 checker 是否包含 v5 名單／origin 硬綁；只共享確實同構的檢查，不使用整包 verifier 當抽取物。 |

**私有交易能力，而非跳過開關。** 提案形狀是兩個私有入口（v5 與 A），各自建只能在已驗 transaction 內取得的 token／借用；typed effects 只對此內部能力與固定輸入開放，不能被 IPC 拿到，也不能用 skip_verify／relaxed flag 製造它。在 Rust 發現借用、guard 或 SQL 繫結無法守住這條界線時停；不為了可編譯先放寬。現況 Create/Update/Delete 可見效果以傳入的 connection 執行，**這是可抽取的線索，不是已證明能安全用在 15001**。

## 3｜必須原樣留在各 v5 caller 的檢查

- execute_with_adapter（:1523–1624）保留 origin 專屬 connect → verify/pre-manifest → BEGIN IMMEDIATE → prepare → commit/rollback → 關閉、唯讀精確前／後態判別。prepare_write（:1462–1492）仍在 transaction 內完整前驗，一次 guard 範圍、既有 failure injection、投影/content/integrity 後驗、清 guard、完整驗證與 post-manifest。函式命名或重排不能移掉任何一關。
- verify_exact_v5_with_origin（:346–372）固定 user_version=5、schema object manifest、verify_receipt_and_contract（:215–271，Migration vs DirectFresh 不能混用）、guard 空、content、projection、integrity；verify_experience_database（:1442–1460）仍跑 origin 對應的 Pattern、ContextRecovery（及其下游 Evidence／Reflection）檢查。operation_manifest 資料集合、deterministic IDs、兩種 receipt 的語意、failure codes、status、舊 read-back/ack 不能因共用而變。
- v5 的直接前置仍走 android_m2a 的 refuse_sidecars／verify_ready、direct-init receipt/structure verifier 與各 adapter 的 scope；desktop/migration 的既有 activation/readiness 仍生效。不得讓 A gate 接受 v5 檔、放寬 v5 以接受 15001、改現有 immutable 或鎖策略來使 A 通過。
- 下游的 verify_exact_pattern_v5_direct 會接到 Reflection 驗證；verify_exact_context_recovery_v5_direct 會直接呼叫 direct_init::verify_direct_fresh_connection（見 schema_v5_pattern_write.rs:1590–1602、schema_v5_context_recovery_write.rs:2129–2140）。A 要設自己的完整 exact verifier，把 core artifact projection／FK／scope 等同等檢查以 connection-local 形狀保留；不能把原 v5 verifier 對 15001 的拒絕當成可忽略錯誤。若一個 checker 無法乾淨分離，該相容路徑未證，不可從 A gate 靜默刪除。

## 4｜A 一筆效果交易：順序與 postcondition

1. **A 自己准入**：先通過 04 所選政策的 S3；握同一 native 排他 owner，以 A 版本、receipt、scope、能力和精確全部 manifest 重驗。Tx P 依 02 只登記不含正文的 pending receipt；確認後才允許同一 owner 開 Tx E。Tx P 不寫任何片刻、理解、關聯、舊守衛或舊資料。
2. **Tx E 同一 connection**：BEGIN IMMEDIATE；重驗 A 完整契約、空 guard、pending/op、來源／目標版次、刪除預覽集合及必要 graph。適用 core mutation 時設定原相容的 FK/guard（與既有 trigger 對等），驗證之後才可取得私有 core handle，不用任何 path-based writer 或 readback façade。
3. **固定組合**：
   - moment_create_linked：驗目標 → 呼叫 Create core → 寫 link → 驗理解 head／refs 未改、原生片刻與投影已正確。link 任一失敗回滾 Create，不另試單存片刻。
   - moment_correct：使用 Update core 全部原失效／provenance；A 精確 refs 保持原指向，不能先更新 UI 或動原引用。
   - moment_delete：在同 transaction 核對影響集合，轉 A refs 成非內容 removed、刪導覽、刪舊結果識別；執行 Delete core 全部既有 purge。若 core 回 StaleRevision/NotFound 或失敗，先前 A-side 改動也回滾。刪除理解只用 A 固定 SQL，不能拿 core 刪片刻當捷徑。
4. **提交前雙層後驗**：驗 core projection/content/integrity、原作者及失效／刪除效果；驗 A 的精確 source revisions/links、刪除 scrub、期望理解內容和狀態、清 guard。把同一 pending 改成 committed＋最小結果，完整驗證 post-state 並記當次預期投影／比對資訊於記憶體；receipt transition 負責嚴格一筆 CAS。這之後只有一次 SQLite COMMIT。任何 A 或 core postcondition 不符就 rollback，Tx P 仍可查。只在能證實未提交時才另標 not_applied。
5. **核對／未知，不用第二次業務寫入**：正常時以真正 A gate 讀回結果、relations／purge 和 receipt 一致才回「已存」。ACK 遺失後只查已驗的現況與 receipt，不能以 A 的未持久內容 digest 推導或重製原 request。若看到已提交效果／已被替換／已清除了結果，依 02 分流；若仍不明或 SQLite 留側檔，依 04 卡住，不報成功、不自動重送。不要拿「read-only」名稱、第二 connection 或舊 v5 operation_manifest 來瞞過 recovery admission。

## 5｜後續的最小證據與停止條件

- **先做 v5 等價性，再啟用 A**：抽取點前後，Migration 與 DirectFresh 都必須通過既有測例並比對 bytes/old manifests/status。至少保留 writer 內 create_writes_one_v5_revision_and_matching_v4_projection、stale_update_rolls_back_without_revision_or_projection_change、update_invalidates_every_supported_active_ordinary_state_and_cascades_history、historical_representation_mismatch_fails_closed_before_source_mutation、parent_delete_exhaustively_purges_every_ordinary_state_fixture、import_failure_rolls_back_the_entire_new_batch、update_and_delete_cascade_failures_roll_back_exactly、all_create_failure_points_roll_back_authority_projection_and_guard、ambiguous_commit_classifies_exact_post_or_pre_state_without_retry、direct_update_delete_uncertain_commit_classifies_exact_pre_and_post_without_retry、malformed_v4_inconsistent_v5_and_newer_database_fail_closed。保留既有 canonical runner 未通過／未重跑狀態。
- **按 caller 回歸**：desktop activation + schema-v5 Reflection 讀回／來源 delete、M1 create、M2-A direct-fresh 發佈拒絕、M2-B 與 M2-C 的直接寫入／reconcile／scope。另驗 schema_v5_android_reflection.rs 的 source_edit_invalidates_and_delete_atomically_purges_dependents 與 batch_create_append_reopen_and_parent_delete；connected Pilot 的受影響範圍只證 core 語意，不呼叫真實模型。M2-D 與 036 不操作。
- **新的最小 failure points**：Create core 後、link 前；Delete A refs/link scrub 後、core purge 中；core/A graph 和 receipt 已更新、commit 前；commit 已 durable、readback/ACK 前；錯版／影響集合漂移時已有 Tx P；最後用原 v5 gate 開 A fixture、A gate 開 v5 fixture。見 [03 驗收](03_VERIFICATION_AND_STOP.md)；每個比較都包含 guard、projections、author/revisions、A links/refs/receipt。不得用盲目成功、第二個 commit 或 mock DB 算通過。
- **未證明而必須停的點**：15001 的完整 DDL/trigger 與 v5 core SQL 可共用性；可否將重疊 verifier 正確切成等價 core checks 而不漏 Pattern/Reflection/Evidence/ContextRecovery 各語意；A 離線問答 caller 是否另需抽取／相容；Rust 封裝確保不能從非 gated connection 配出 token；實際 Android 排他／SQLx open／SQLite locking 與 crash bytes 行為。任何不成立都須停回精確設計，不造旁路。此文件不發試作、SQL、測試或實作權限。
