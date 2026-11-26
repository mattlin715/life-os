# 最新核定與授權紀錄
整理日期 2026-11-26（Asia/Taipei）。本紀錄轉錄可見 Founder 指示，不是系統簽章或完整聊天匯出；不回寫舊文件的歷史狀態。

## D1｜已確認的產品體驗
Founder 已明示「目前設計已經符合期待」。核心樣品是原始 v15：
SHA-256 91eb8d73442bffd109037f526344802876eff66f871e9641f8e190553547ac96。
接受互動方向不代表原型的示例 AI、記憶體資料、字體、占位設定已正式落地；不授權原型中的全部行為一口氣投入 production。

## D2｜Design Review 02 的八項方向
Exact Design 02 原件記載的 Founder 核定原文：
> 以 Design Review 02 為基準，逐項同意上述 A1、A2 主方向、A2-a～A2-d、A3、A4 推薦及限制。授權下一輪精確設計，不授權產品實作、ADR 接受、repo 寫入或 Git 操作。

理解為本人獨立可修訂內容；單一 app-private canonical、精確版本、原子保存；fresh synthetic 不升級；明確後來片刻導覽關聯；A 暫不理解匯出；非正文提交查回；刪源保留理解但清系統副本；A 草稿僅程序內。完整範圍見原件，不以本摘要擴張。

## D3｜Exact Design 02 與限定 R1（本主對話直接核定）
Founder 原文：
> 核定 Exact Design 02 的 A 專用候選與資料契約、交易及提交紀錄規則、writer 抽取方向，並選擇限定 R1。接受符合條件的 A 合成候選可能在最終驗證前已被 SQLite 回滾修改，且後續仍可能拒絕使用。其餘保全與停止條件不變。這是精確設計核定，不授權產品實作、ADR 接受、repo／Git 操作、建包或裝置操作。

綁定 Exact Design 02 原 ZIP：
9e54a4b89f2e5fb8bd5b3d900241b80168cfac3e9b164460b9941806bf142cd1。
因此原件「R1 尚待核」在本範圍已被 D3 更新；其他待決沒有自動接受。
- schema 15001／獨立 A 身分與精確驗證，不修改 v5 的 accepted schema。
- Tx P 只登記非正文 pending；Tx E 是唯一業務效果 commit；來源、理解、關聯、receipt 同交易。
- receipt 至多 64 筆未結案、同時最多一筆 pending、無背景 TTL；未知不得清除或重放。
- 限定 R1：只在 04 的受控 A 合成原件及預檢條件成立時交 SQLite；可能先改原件後拒絕。未知／不合格狀態仍保全，不擴張成真實資料恢復。
- writer 抽取保留所有原 v5 gates；不能先舊 writer commit 再補關聯。
核定不代表任何新增測例已執行。

## D4｜交接與 Cloud 分工（本主對話直接要求）
Founder 要求新的 Codex Cloud 工程 session，及另一個新的產品方向／審閱 session，方便電腦關機後用手機繼續。
Founder 指定：
> 交接包應保留原始 HTML、對應體驗規格與最新核定紀錄；接手者須實際走讀原型，區分已確認體驗、示例／占位能力與尚未核定事項，再說明實作對應。若無法符合原型，先提出差異，不自行替換產品方向。

## D5｜本輪 repo／Git 保存授權（本主對話直接要求）
> 把這個交接包和html推到git上
> session只要取得指定 repository 與 精確 commit SHA，閱讀交接入口，實際走讀 V15 原型，再依核定範圍工作

本輪只增加交接原件、入口、核定紀錄、prompt 與檢查器；專用 topic branch 保存並 push，不 merge develop、不做產品實作、不接受 ADR、不變更舊 workflow。
D5 是 D3 之後新增的**交接保存**授權，不可用 D3 的舊禁 Git 文字誤判本次保存未授權，也不可把 D5 擴成工程自動 push／merge。

## 後續執行邊界
- 角色 prompt 是可審的派工範本；**只有 Founder 在新 session 明確送出該角色／階段 prompt，才啟動該範圍**，讀到檔案不自行派工。
- 本輪建議第一個工程任務是 A-F1 host 可行性切片，非完整 A UI、B 或 C；由 ENGINEER prompt 明定。
- 新 reviewer 首先實際走讀與回述，Founder 確認交接理解後再開工程；這個次序是防偏移工作安排，不是新的產品能力 gate。
- ADR-0013 仍 Proposed，原 R3 schema gate 未關；M2-D／036 STOP 不變。若既有治理要求額外 ADR／workflow 動作，指出精確阻擋與最小需要的核定，不冒名接受、手改事件或強行關閉。
- 真實模型、金鑰、實人資料、Android packaging/signing/install/device、release、跨端同步及遷移均不在本輪。
