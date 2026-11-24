---
status: Historical R3 source/host checkpoint retained; product direction realigned; feedback schema decision pending
version: "0.3"
owner: LIN MENGLUNG
last_updated: 2026/11/21
depends:
  - docs/00_Constitution.md
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/architecture/24_Android_M2C_Offline_Synthetic_Daily_Reflection.md
  - docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
referenced_by:
  - docs/architecture/24_Android_M2C_Offline_Synthetic_Daily_Reflection.md
  - docs/dev/15_Android_M2C_Offline_Synthetic_Daily_Reflection_Runbook.md
---

# Android Model Pilot R3：讓一次反思可以留存、補充與回看

## 2026/11/21 產品方向更新 — 先讀核心體驗契約

Founder 已同意 [Life OS 核心體驗契約草案 v0.1](02_Life_OS_Core_Experience_Contract.md)，
作為下一輪體驗設計與低擬真原型的共同基準：從保存 AI 回饋，轉向支持使用者形成並修正自己的理解。
本文件下方保留既有 R3 歷史授權、來源進度與限制；「下一版唯一主路徑」是當時提案，
不再單獨作為後續實作依據。先用虛構資料原型走讀三個情境，再核對政策與資料模型。
不得據舊第 5／6 點自動續做回饋／補充持久化；ADR-0013 仍 Proposed，既有 workflow gate 不變。
本次只有文件同步，不回溯撤銷既有成果，不授權開發、打包、模型傳送、實機操作或 commit/push。

## 目的與目前授權

承接 Founder 對 3101 的七點回饋。這是 Book One 的產品提案，不改寫 Book Zero 或已接受的 ADR。
目標不是增加聊天量，而是讓使用者能修正模型的理解、保留有用的回饋，並隨時停止。
將第 1–6 點整合為一次實機驗證，不為每個小改動各發一包。

- 已明確要求、可先完成來源修改：移除聚焦段落輸入、移除連網版示範動作、分開提問與回饋標示。
- Founder 已核定第 1、5、6 點一起做來源與主機測試，先不建包。
- 金鑰保存來源與 host checks 已實作；實際 Keystore／更新延續仍待後續手機驗證。
- 回饋／補充需要 canonical 資料結構變更；具體阻擋與新設計見 ADR-0013 草案，不以假 Evidence 或旁路保存繞過。
- 第 7 點只記錄供下次討論；不是跨片刻讀取、模型傳輸或背景分析授權。
- 本輪不建包、簽章、安裝、呼叫 API、讀取憑證秘密或 commit/push。

## 3101 的 Founder 實機回報

裝置為 Zenfone 10 / Android 15，Founder 回報更新完成且舊片刻仍在。
以下是對該次手機操作的直接回報，不是自動化證據或完整平台驗收：

| 檢查 | Founder 回報 |
| --- | --- |
| 不經示範線索直接問一題 | 能 |
| 真實 Gemini 提問、儲存問題與回答 | 能 |
| 另一個角度是否有帶來理解 | 「有帶來不同的理解」 |
| 重開後問題與回答 | 仍在；消失的只有暫存回饋 |
| 取消預覽、手機切換與草稿／過期結果、刪除測試片刻後重開 | 「這三點都測試了OK」 |

未另取得具體證據的項目仍未知，例如資訊不足時的回饋品質、其他 OEM、崩潰／斷電、
跨版本持久化的新設計、真實個人資料安全與受邀試用。不得把這份回報擴充成它們的 PASS。
3101 已簽章檔案 SHA-256 為
`BF3F5DC440289C74D5006539FBE0EE6163645065FEC8C211D8137C0CDFF0F9F0`。
手機安裝完成為 Founder 回報，未獨立讀取手機上的 APK、資料庫或個人內容。

## 下一版唯一主路徑

**歷史 R3 路徑；後續體驗方向改以本文件頂部所連結的核心體驗契約為準，非續作授權。**

記下片刻 → 問我一題 → 儲存我的回應 → 看看另一個角度 → 儲存這次回饋
→ 有需要才「補充資訊」→ 預覽並確認 → 更新的回饋 → 可再次明確儲存。

不要求為了完成流程辨識或接受 Pattern；沒有看見新角度也可以結束。

### 1. 可選「在這部手機記住金鑰」

建議由使用者明確勾選，預設仍只用本次；不得因為貼上金鑰就默默改為落盤。
保存時，以 Android Keystore 管理的加密金鑰加密 API 金鑰，密文放在 app-private、
排除備份的位置。Keystore 保護的是加密金鑰，不能把 API 金鑰直接當作可匯出的普通偏好。
原生層處理讀回／解密，畫面只顯示已設定，不自動填回完整秘密。

- 不在 APK、SQLite、一般偏好、日誌、匯出或明文檔案保存 API 金鑰。
- 提供清除／忘記；撤銷後不再自動讀回。僅移除 app 內金鑰，不等同向 Google 撤銷憑證。
- 同 identity／簽章正常更新需驗證可延續；解除安裝、清除資料、換機或加密金鑰失效需重填。
- 解密或儲存失敗明確顯示，不能轉存明文或宣稱已記住；失效時不刪除日記資料。
- 不承諾防止已解鎖或遭入侵裝置上的所有秘密濫用，也不從機型推定硬體保護層級。
- 具體 Kotlin/native adapter、原子寫入、異常處理及備份／device-transfer 測試在工程實作前確認。

來源實作以原生 Tauri bridge 呼叫 Java，使用 `AndroidKeyStore` AES-GCM 與 app-private
`noBackupFilesDir` 的 `AtomicFile`。勾選保存後才落密文；預設仍只用本次。清除／改為本次使用先
原子寫入停用標記，再移除包裝金鑰，避免移除失敗後重開又恢復舊金鑰。所有例外對畫面只回傳固定錯誤碼，
不轉送可能含秘密的原始例外。bridge 不提供 renderer 的金鑰讀回權限。
host 只驗 injected backend 的加密／生命週期及 Android API 編譯，未使用實際 Android Keystore。
依官方 [Android Keystore](https://developer.android.com/privacy-and-security/keystore)
及 [Auto Backup](https://developer.android.com/identity/data/autobackup) 文件；尚未實機驗證安全保存或更新延續。

### 2–4. 去掉分岔，分清作者與內容

- 移除「選用：聚焦其中一段」輸入和相關預覽欄位；以完整已儲存原文為唯一當次來源。
- 連網畫面不再顯示示範線索、採用／修正線索、本機示範展開或「再加一個示範問題」。
- 純離線版本維持可獨立建包。刪除的是連網版的介面動作，不是刪除既有 SQLite 紀錄。
- 真正已儲存的舊 Gemini 問答仍可讀回；不讓範例生成按鈕隨它重新出現。
- 問題標「Gemini 提問」，模型解讀標「Gemini 回饋」，使用者的回答／補充標「由你撰寫」。
- 尚未實作回饋保存前，仍如實說明回饋只是本次查看，不能先顯示「已儲存」。

### 5. 明確儲存「另一個角度」

新增「儲存這次回饋」，只在使用者明確操作後保存；儲存不呼叫 Gemini。
依 ADR-0007，不把所有模型回覆自動寫入長期記憶。

- 回饋是模型的可修正觀察／假設，不是已證實 Evidence，也不因儲存而變成客觀事實。
- 保存觀察、引用、可能解讀、不確定性，以及 provider/model/prompt 版本、生成時間、
  明確儲存時間、原文／問題／回答的精確版本關係；使用者補充與 AI 原文不得混寫。
- 原文或回答修改後，舊回饋不再當作對新版的有效解讀；畫面說明來源變更，不默默改寫舊解讀。
- 可以刪除回饋；刪除片刻須清除相關內容。新紀錄的刪除／失效規則需明列並測試。
- 舊版 3101 問答必須保留；已關閉程序而消失的暫存回饋無法事後補救。
- 先核對現有 canonical artifact 能否表達獨立回饋。精確 schema、升級交易、回滾／舊版拒絕策略
  仍須完成工程方案與 Founder 檢查；不直接塞進回答欄、假 Evidence 或額外 JSON sidecar。

### 6. 同片刻內「補充資訊」，而不是反覆問同一題

回饋下提供「補充資訊」。使用者可指出「哪裡不準確、還有什麼情境」，而不是被迫認同模型。
補充是獨立的使用者內容，不修改最初片刻、不覆寫已儲存回答或 AI 回饋。

- 「儲存補充」是本機動作；「用補充再看看」是另一個需明確同意的模型請求。
- 預覽顯示同片刻的完整原文、已存問答、本輪選用的回饋及已儲存補充，經確認後才送。
  不附帶其他片刻、未存草稿、未選取的早期輪次、識別資訊或整份 ContextPacket。
- 每輪綁定上述精確版本與上一輪關係，限制筆數／大小；超出時讓人選擇或明確拒絕，不能暗中截斷或摘要。
- 新回饋需說明補充帶來的修正，保留替代解釋和不確定性。必要時可提出一個具體澄清問題，
  不為了延長互動固定要求下一輪，也不能從單一片刻宣稱長期反覆模式或給出人格定論。
- 每輪分開明確儲存。重入、取消、換片刻、來源修正、儲存結果不確定和遲到結果沿用 fail-closed 邊界。
- 已存本片刻的補充用於下一輪，屬新增 provider-input 範圍；需本提案整體核定後才實作。

## 第 7 點：下次再談的跨片刻長期觀察摘要

Founder 想選定一個時間範圍，跨多個片刻整理可能的 Pattern 與長期觀察摘要。
這個需求已記錄，但不混入本次同片刻 R3，也不因為有時間篩選器就自動分析或傳送整段歷史。

此議題已有入口：`Reflection.md`、`architecture/11`、ADR-0010。下次從已接受的
設計與尚未完成的實作前提出發，不另造互相衝突的 Pattern 定義。
需討論具體範圍選取、支持／反證來源、替代解釋、資訊不足、使用者認同／反駁／刪除、
更正來源後摘要失效及每次傳輸同意。摘要必須是可修正的假設，而不是人格診斷或身份結論。
ADR-0010 的歷史傳輸與 Phase 3 入口限制仍適用；這份記錄不解鎖它們。

## 一次核定、一輪實機驗證

建議整體核定第 1、5、6 點的上述邊界後，完成必要工程方案與來源實作／host tests，
再另行建一個包，一次驗證「記住金鑰 → 問答 → 保存回饋 → 補充 → 修正理解 → 重開／刪除」。
不需要先為第 2–4 點發一個只改文案的包。

Host 覆蓋：三語介面、預覽與取消、只送選定同片刻內容、金鑰不明文落盤與清除／失效、
精確來源及各輪 provenance、保存零網路、原子性與不確定提交、舊版資料延續、重開與連帶刪除。
手機另驗 Keystore／加密保存及正常更新重開、鍵盤、返回／背景、過期回覆與回饋實際價值。
不使用日記／DB／秘密截圖作為回報材料；不把合成資料驗證當作長期取代實用價值評估。

**目前完成第 1–4 點的來源工作；未完成第 5、6 點。** 不拆出金鑰專用 APK。
schema v5 的 artifact kinds、provenance roles、dependency types 與 exact manifest 都是封閉集合，
無法正確表達獨立回饋及補充鏈。`docs/adr/ADR-0013-proposed-model-pilot-feedback-and-supplement-lifecycle.md`
提出 Model Pilot-only v6、分開的 model-turn records、舊問答延續與無自動降版的方案，**尚未接受**。
停止在這個具體資料結構／升級設計核定點，不改 accepted ADR，不新增 table，不用 sidecar 或假 Evidence。
