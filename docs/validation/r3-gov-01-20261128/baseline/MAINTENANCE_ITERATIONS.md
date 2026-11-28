# 驗證與修正紀錄

- Baseline: 38/38 workflow tests PASS、真實 validate PASS；canonical powershell 不存在，actual launch exit 127，blocked。
- Iteration 1: 原始輸出 iteration-1.log，actual exit 1，84 tests / 78 pass / 6 fail。
  - 五項 before-write 中斷 hook 未觸發：對已等於 idle 範本的檔案省略寫入；測試預期每個 reset 階段皆能中斷。把階段 hook 移到 known-image 判斷前，另保留真正 replacement rename 前中斷點；並不增加多餘實際寫入。
  - receipt-conflict 在 reset 後才拒絕，雖保留了復原副本，卻不符合衝突時零寫入。把 receipt 目的地衝突／目錄檢查移到任何發布或 reset 前的全量預檢。
  - 同步加入操作種類前後狀態限定與目錄 symlink 拒絕；不改既有 verifier、CI 或依賴。
- Iteration 2: actual exit 0，84/84 PASS。複查後補足真正 replacement rename 前的 SIGKILL／exception、歷史 PASS 不繼承、錯誤 idle 範本、dangling symlink、內部 image hash 損壞、初始 preparing 衝突、重複 archive 名稱。相應縮限：證據鍵精確集合、idle 不繼承權限、symlink 不視為不存在、archive 附加證據對齊 checkpoint。這些是授權內恢復協定的邊界檢查，沒有擴張產品權限。
- Iteration 3: actual exit 0，95/95 PASS。最後逐項核對「未知檔案」要求，除 recovery/staged/published archive 外，將未知活動控制面項目也納入取消／封存／恢復的全量預檢；新增零寫入保全測例。中斷觀察 metadata 放到快照旁邊，避免把觀察檔混入待恢復控制面。進行最終重跑以核對此邊界修正。
- Iteration 4: actual exit 1，95 tests / 93 pass / 2 fail。新增未知控制面檔案預檢把兩份既存、非 reset 的參考文件也當未知，造成舊 completed/scoped-manual archive 回歸失敗。保留原始 iteration-4.log。修正為明列既存 WORKFLOW_EVALUATION.md 與 PREPARED_RECOVERY_PLATFORM_BOUNDARY_R1_AUTHORIZATION.md 為只讀參考項目；仍拒絕新增未知檔名，不刪文件、不放寬未知操作或 archive hashes。這兩份參考文件不被本操作重設，亦不授予本輪任何舊授權。
- Final: actual exit 0，95/95 PASS，0 skipped。最終源檔 hashes 綁定 final-workflow.json；65 次故障觀察（59 SIGKILL／6 exceptions）。新舊 58 archives verdict 全同且全有效，652 protected hashes 全同；兩份交付中斷快照的官方 recovery 在新暫存副本中各 exit 0。
