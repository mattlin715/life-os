---
status: Draft; Founder-agreed product direction; prototype baseline only
version: "0.1"
owner: LIN MENGLUNG
last_updated: 2026/11/21
depends:
  - docs/00_Constitution.md
  - docs/00_Index.md
  - docs/Reflection.md
  - docs/06_Memory.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
referenced_by:
  - docs/00_Index.md
  - docs/product/01_Android_Model_Pilot_R3_Proposal.md
  - docs/architecture/24_Android_M2C_Offline_Synthetic_Daily_Reflection.md
---

# Life OS 核心體驗契約草案 v0.1

> 2026/11/21 Founder 同意此草案，並要求先寫入 repo 作為跨 session 共同基準，再以可點擊低擬真手機原型驗證。這是 Book One 的體驗設計草案，不是新 Book Zero 定義、已接受 ADR、正式 MVP 範圍變更或產品實作授權。

## 1. 核心價值與驗收準則

**幫助你從生活中看見自己，也保有重新理解自己的自由。**

> 使用者不必自行整理所有記錄，就能開始回看；能知道觀察的依據、反駁不適用的解釋，用自己的話留下理解，並讓後來的經驗改變這份理解。

Life OS 支持使用者形成並修正自己的理解，不替人判定人格、揭示唯一根源或要求接受 AI 答案。不承諾記錄一個月就能找出潛意識原因或得到幸福；此切片也不取代完整 Life OS 的其他核心支柱。

## 2. 完整體驗路徑

**留下片刻 → 接受回看邀請／自行回看 → 確認預備範圍 → 核對觀察、依據與例外 → 探索 → 留下自己的理解 →〔可選〕小嘗試 → 以新經驗修正理解。**

- 每天可以只記錄並離開；儲存本身不呼叫模型。系統協助準備，不要求逐筆從零整理。
- 使用者選擇是否需要可能原因提示、文字草稿或具體建議；先容納自己的答案，不用提示把人導向固定成因。
- 沒有答案、不保存理解、暫不行動，都能自然結束。保存 AI 回饋有用途，但不是保存自己理解的前提或替代品。
- 「今天／回看／我的理解」是待原型驗證的入口建議，不是已定案的導覽或資料模型。

## 3. 不可違反的行為邊界

1. **分清作者與性質：** 使用者記錄、AI 推測、使用者目前理解不可混成客觀事實；AI 草稿經使用者確認也不能抹除 AI 來源。舊 AI 推測不累計成獨立新證據。
2. **解釋可反駁：** 顯示來源、反例與未知；未記錄不代表未發生，不同意不代表抗拒。更正必須真的改變探索方向。
3. **不強迫向內歸因：** 容納外部限制與實際風險，不推定童年成因、診斷或固定身份，也不只尋找痛苦，忽略支持自己的條件。
4. **主動準備不等於自動分析／傳送：** 提醒可選、可延後、可關閉，不累積欠帳；鎖屏不暴露敏感內容。內容分析與跨時間外傳另需明確同意，不做隱性心理側寫。
5. **理解屬於使用者：** AI 不自動改寫或替使用者接受；保留、修正、不再沿用與刪除按既有 provenance／lifecycle 規範設計，不承諾可永久保留已刪除來源內容。
6. **建議不是處方：** 只有使用者想繼續時，才提供考慮現實限制、可修改或不採用的小嘗試；不把未採用、未改善或未回來視為失敗。不以打卡、成長分數、羞愧或依賴推動使用。

## 4. 第一個試用切片的目標範圍與非目標

**目標範圍（待原型驗證，不表示目前可開發）：** 片刻與可選的單次探索；簡單定期邀請、時間範圍預選及送出確認；有來源、反例與未知的跨片刻觀察；自己的理解與可選小嘗試；新片刻帶來保留、補充、修改或暫不沿用。這個完整循環不能只交付到「保存 AI 回饋」。

**非目標：** 心理診斷、唯一潛意識根源、幸福保證；背景人格分析、複雜語意推薦、全面人生報告；任務排程、成長評分與連續打卡。此草案不授權同步、真實個資／受邀試用、模型傳送、schema／migration／retention 變更、打包、裝置操作或發布。

## 5. 三個情境的驗收條件

| 情境 | 必須成立的結果 |
| --- | --- |
| 期待被理解，但沒有表達 | 核對具體片刻，探索「那些沒有說出口的時候，你最擔心說出來之後會發生什麼？」；形成有適用範圍的暫時理解。新經驗能支持、細化或推翻它。 |
| 已表達，但仍被忽視 | 更正後撤回「沒有表達」的推測，轉向理解外在限制；不把責任再次推回使用者，不用「你還沒真正改變」維護原解釋。 |
| 資料不足或互相矛盾 | 不硬湊模式；允許澄清、保留未知、不保存理解或結束；有新資料才能重新評估，不能以沒有記錄推論問題已消失。 |

共同條件：看得懂依據、能有效反駁、能留下自己的話、能不帶負擔地離開。三組都加入後續片刻，驗證理解不是永久標籤。尚未執行原型走讀，不記為 PASS。

## 6. 與既有 R3／ADR 的關係及下一步

- 本契約是後續體驗設計與原型的共同基準；[既有 R3 提案](01_Android_Model_Pilot_R3_Proposal.md)保留來源進度、歷史授權與證據，不能單憑舊「唯一主路徑」繼續實作尚未完成的回饋／補充功能。保存 AI 回饋調整為支援能力，不是最終產品目標。
- [ADR-0010](../adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md)與[架構 11](../architecture/11_Cross_Experience_Reflection_Design_Gate.md)仍優先：既有首切片採人工精確選取；本草案的範圍預選與一鍵確認須在正式實作前核對或另行決策。不能把時間範圍當成整段歷史的外傳授權，也不取消既有 sufficiency／counter-evidence／entry gates。
- 「我的理解」是體驗名稱，未決定 canonical artifact 類型；AI 原文、AI 協助草稿、使用者自寫／修訂、來源依賴及刪除規則仍須分清。可選建議不代表可將 hypothesis 自動轉成 advice；其獨立行為邊界須另行對照 Book Zero／Product Harness。不得硬塞為 Evidence、Pattern 或旁路資料。
- [ADR-0013 草案](../adr/ADR-0013-proposed-model-pilot-feedback-and-supplement-lifecycle.md)仍為 Proposed，未接受、未駁回；不能反過來由其七表方案決定體驗。本次不修改任何 ADR 狀態、Book Zero 定義或 workflow gate。
- **下一步：** 使用虛構片刻、預寫分支、明確模擬保存／送出，做可點擊低擬真手機原型；三情境的「不符合／先停下／不足／修改理解」都必須可操作。不接模型、不用真實日記、不操作現有 App 或資料。原型工作仍須明確指定執行範圍；本次只落地文件。
- 不必先把契約寫到完美或接受新 ADR 才能做上述原型。原型走讀後修訂契約，再核對現有 R3／政策／資料模型並另行授權實作。體驗走讀不代替模型品質、工程可靠性、裝置驗收或長期效益驗證。
