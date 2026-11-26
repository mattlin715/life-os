# 下一輪建議任務：A-F1 限定 host 可行性實作
2026-11-26 · 只在 Founder 送出 ENGINEER prompt 後執行；本文件存在不是自動授權。

## 這輪要回答的工程問題
在不削弱既有 v5 的前提下，Exact Design 02 的新 A 理解、來源／導覽、非正文 receipt，能否由同一個受驗 native SQLite 交易保存與清理？限定 R1 能否按預檢／回滾／完整後驗誠實分流？

## 入口
先核對 handoff SHA／payload hashes、完成原型走讀；收到 Founder 已確認的 reviewer 理解回述，核對其同一基準。若沒有，先交走讀與範圍回述，不寫產品來源。
取來源保存基準與 handoff commit，不默默 rebase 到最新 develop。先讀實際治理與工作樹；不得覆寫未提交內容。預設 reuse 合適 Cloud checkout，若 task 自動有 branch 就記錄，不需要為名稱一致另造 worktree。讀取 Git 與必要取得指定 commit 可行；stage/commit/push/PR/merge 另授權。
原 workflow 是 R3 human_decision_required，不是 A mission。不得以本 prompt 關閉舊 gate 或改寫原事件；先核對官方可用的獨立任務入口，若必須另授權轉換才可進行，說明最小缺口並停。不得創造平行偽狀態遮掩 blocking condition。

## 允許的窄化範圍
1. 固定基準上先取得適用 host 回歸現況，保留既有失敗。列出實際影響檔與抽取點，與 Exact 05 對照。
2. 在 existing Rust module／私有函式邊界抽取 Create/Update/Delete transaction-local effects 及真正同構 checks；先證 v5 Migration / DirectFresh 等價，再接 A。不得用 skip verifier 開關或以第二份變體掩蓋抽取問題。
3. 建立不曝露給既有 app 的 A host 契約／獨立精確 DDL/verifier、最小 admission 與固定資料操作，足以驗 A 理解首版／修訂／讀回／刪除、精確來源及新片刻導覽。避免先做 UI 或完整 IPC／Android build 註冊；未完成端對端就是未完成，不宣稱候選已可用。
4. 依 Exact 02 實作／驗 Tx P、單一 Tx E、故障 rollback、提交後 ACK 遺失查回、重入／stale／刪除 scrub、64／pending／無 TTL 的限額、無 payload replay。只用全新合成 fixture。
5. 依 Exact 04 實作 host 上可驗的 S0/S1/S2/S3 與 R1，使用 SQLite 真正產生的 journal、真正跨程序鎖／崩潰點。Linux 證據只算 Linux，不替 Android OS lock / SQLx / VFS 背書；host 不足以證明時把 Android 入口維持不可用，報 gap。
6. 最小關鍵證據包含 Exact 03 的 R01–R07、W01–W04；合理排序先 W 等價與同 tx，再 R。不能重製同名假 journal 當恢復證據。其他 A host 驗收做能在本範圍執行的部分，逐案 not_run 不遺漏。
7. 可更新本輪必要工程計畫、測例／結果與待決的文件；不覆寫 immutable handoff 原件，不修改產品方向、原型、Constitution、Book Zero、ADR 狀態或舊驗證結果。若正式治理要求新 ADR，另交 Proposed 設計需求並停止被其 withholding 的實作，不冒用 ADR-0013。

## 明確不做
全 v15 UI／搜尋／主題、真實模型與金鑰、B/C、跨理解來源、AI artifacts 持久化、真實資料、匯出／升級／同步、APK/AAB/installer／簽署／裝置／emulator／M2-D／036、PR／merge／release。測試所需 Rust/frontend host 編譯不是 Android 建包授權。不新增依賴／改 CI／換工具鏈來繞過障礙；確有需要先報原因。

## Cloud 可做與不可冒稱
使用 repo 現有工具與 lockfiles；先列 Linux 可跑命令與環境。canonical 仍是 scripts/verify.ps1 指定路徑；若 Cloud 不能滿足 Windows／本地 pnpm 等前置，清楚記 blocked/not_run。可跑原有 Linux 適用的 tests/checks，不把它們改名 canonical PASS，不修改 verifier、刪測試或換 schema 換取綠燈。
不用等電腦開機才做所有 host 可行性工作；但不能為了全 Cloud 而把 Windows、Android 實機、AI 品質、Founder 人工驗收省掉。當任一必要前提不成立，停該路徑，報精確阻擋，不暗中降級。

## 交付
基準 handoff SHA + 程式 base SHA + final diff／new-file 清單與 hashes；逐項原型意圖→實作符號→測例／結果；完整錯誤與重跑理由、before/after failure 證據；platform 層級與 not_run；所有影響檔、ADR 判定、Git/index/status。
未 commit 的交付須有能在固定基準完整重建的 patch＋untracked 新檔與清單；不能只給 git diff 漏掉新檔。用合成測資，掃 secrets，不把私密 payload 放日誌。
停在「A-F1 工程候選待審」，不是 A 完成、Founder acceptance、可試用或 B 開始。
