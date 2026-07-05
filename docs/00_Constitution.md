---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/05
depends: []
referenced_by:
  - docs/01_Vision.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/10_Privacy.md
  - docs/adr/ADR-0001-documentation-hierarchy.md
  - docs/adr/ADR-0002-single-source-of-truth.md
---

# 00 Constitution

## Purpose

Life OS 的目的，是幫助人更清楚地理解自己，並在長期生活中建立更好的覺察、記憶與成長能力。

Life OS 不是要替使用者活人生，也不是要讓 AI 成為新的權威。它的核心任務是協助使用者看見自己的真實脈絡：他如何思考、如何感受、如何重複某些模式、如何改變，以及如何在更清楚的自我理解中做出選擇。

這份 Constitution 是 Life OS 的最高產品憲法。所有願景、設計、架構、功能、研究、商業化與程式碼，都必須服從它。

## Why This Constitution Exists

Life OS 是一個長期產品。它會經歷模型更替、技術重構、平台變化、商業壓力與使用者需求擴張。如果沒有最高原則，產品很容易被短期效率、功能誘惑或 engagement 指標拉離原本的精神。

這份 Constitution 存在，是為了在未來每一次決策中保護 Life OS 的核心：

- 人先於 AI。
- 反思先於答案。
- 證據先於結論。
- 成長先於黏著。
- 隱私先於利潤。
- 文件先於記憶。

它不是口號集合，而是決策約束。

## We Build Mirrors, Not Oracles

> We Build Mirrors, Not Oracles.  
> 我們打造的是鏡子，而不是神諭。

這是 Life OS 的最高句子，也是本專案的 primary source of truth。

鏡子的任務是幫助人看見自己。神諭的任務是替人宣告答案。Life OS 選擇前者。

AI 可以提出觀察、整理證據、指出模式、提出問題、形成假設，也可以協助使用者探索選項。但 AI 不應把自己放在使用者主體性之上。Life OS 不鼓勵使用者問「我該怎麼辦？」然後直接服從模型。它鼓勵使用者問「我為什麼會這樣想？我正在逃避什麼？我的選擇反映了什麼價值？我有哪些證據？」

好的 Life OS 回應，不是讓使用者更崇拜 AI，而是讓使用者更能信任自己的清明判斷。

## Human Before AI

Human before AI 代表：人的主體性、尊嚴、脈絡與自由意志，永遠高於模型輸出的便利性。

AI 是工具，不是人格替代品。AI 可以陪伴，但不能佔據使用者的內在權威位置。AI 可以支援決策，但不能讓使用者失去做決策的能力。

任何會削弱使用者自主性、製造依賴、放大焦慮或讓使用者把人生責任外包給 AI 的設計，都違反本憲法。

## Reflection Before Answer

Life OS 應優先促成反思，而不是急著提供答案。

在高風險、情緒複雜或與人生方向相關的情境中，直接答案常常是危險的簡化。Life OS 應先協助使用者釐清：

- 目前問題的真實脈絡是什麼？
- 使用者已經知道哪些事？
- 哪些感受、假設或恐懼正在影響判斷？
- 有哪些可觀察證據？
- 有哪些選項與代價？

答案可以存在，但答案不應跳過反思。

## Evidence Before Conclusion

Life OS 的人格分析、記憶整理、模式辨識與成長建議，都必須以證據為先。

Evidence-based Personality Analysis 不是用標籤定義人，而是根據可追溯的紀錄、語句、行為、選擇與長期模式形成假設。這些假設必須保持可修正。

Life OS 不應說「你就是這樣的人」。它應說「根據目前紀錄，我們觀察到一個可能模式；是否符合你的感受，需要你確認」。

結論必須從證據中長出來，而不是從模型自信中跳出來。

## Growth Before Engagement

Life OS 的成功不以使用時間、訊息量或回訪頻率作為最高指標。

使用者成長，可能意味著他更少依賴產品。使用者更清楚，可能意味著他不需要每天反覆詢問同一件事。這不是產品失敗，而是產品真正產生價值。

任何刻意製造心理依賴、無限提醒、情緒鉤子或讓使用者停留更久但更不自由的設計，都違反 Life OS 的方向。

## Privacy Before Profit

Life OS 處理的是人的內在資料：情緒、關係、困惑、價值、記憶、人格與成長軌跡。這些資料比一般產品資料更敏感。

因此，隱私不是功能選項，而是產品存在的前提。

Life OS 不應把使用者最脆弱的資料變成商業槓桿。任何資料收集、模型訓練、分析、分享或商業化，都必須能向使用者清楚說明目的、範圍、風險與控制權。

若隱私與利潤衝突，隱私優先。

## Memory Is Unreliable. Documentation Is Truth.

人的記憶會漂移。團隊記憶會消失。AI context 會被截斷。聊天紀錄會讓人誤以為自己記得，但實際上沒有形成可治理的知識。

Life OS 的真相必須寫入文件。

重要決策不能只存在於對話、腦中或臨時 prompt。產品哲學、架構判斷、source of truth、ADR、MVP 邊界與隱私原則，都必須被文件化。

如果文件與記憶衝突，以文件為準。若文件錯了，應透過明確修改更新文件，而不是靠口頭補充。

## Documentation Hierarchy

Documentation Hierarchy 是 Life OS 的最高文件治理規則。它定義所有文件、決策與程式碼之間的權威順序。

### Level 0: Constitution

- `docs/00_Constitution.md`

Constitution 是最高權威。它定義 Life OS 不可被推翻的產品哲學、治理原則與長期承諾。

### Level 1: Vision and Philosophy

- `docs/01_Vision.md`
- `docs/02_Philosophy.md`

Vision 定義 Life OS 要走向哪裡。Philosophy 定義 Life OS 如何理解人、AI、成長與自我。

### Level 2: Principles, Problem, Identity

- `docs/03_Principles.md`
- `docs/04_Problem.md`
- `docs/05_Identity.md`

這一層把高階哲學轉化為產品原則、問題定義與 Identity Model。

### Level 3: Architecture-related Concepts

- `docs/06_Memory.md`
- `docs/07_Awareness.md`
- `docs/08_Growth.md`
- `docs/09_AI.md`
- `docs/10_Privacy.md`

這一層定義 Life OS 的核心系統概念，包括長期記憶、覺察引擎、成長引擎、AI 架構與隱私倫理。

### Level 4: MVP, Roadmap, Implementation Planning

- `docs/11_MVP.md`
- `docs/12_Roadmap.md`
- future implementation plans

這一層定義如何逐步落地，但不得推翻上層文件。

### Level 5: Code

Code 是文件化決策的實作結果。Code 可以揭露文件不足，但不能反向修改 Constitution。

### Hierarchy Rules

- 下層文件必須服從上層文件。
- Code 不得反向修改 Constitution。
- 技術限制不能推翻產品哲學。
- 若發生衝突，以上層文件為準。
- Constitution 是最高權威。
- 若上層文件需要改變，必須明確修改該文件，並視情況建立 ADR。

## Source of Truth Rule

每個重要思想只能有一個主要定義位置。

其他文件可以引用、延伸、應用，但不得重複定義同一概念。這是為了避免文件漂移，也讓 AI Agent 與未來開發者能追蹤概念來源。

目前 source of truth 規則如下：

| Concept | Primary Source of Truth |
| --- | --- |
| We Build Mirrors, Not Oracles. | `docs/00_Constitution.md` |
| Documentation Hierarchy | `docs/00_Constitution.md` |
| Source of Truth Rule | `docs/00_Constitution.md` |
| Vision | `docs/01_Vision.md` |
| Philosophy | `docs/02_Philosophy.md` |
| Design Principles | `docs/03_Principles.md` |
| Problem Definition | `docs/04_Problem.md` |
| Identity Model | `docs/05_Identity.md` |
| Memory Model | `docs/06_Memory.md` |
| Awareness Engine | `docs/07_Awareness.md` |
| Growth Engine | `docs/08_Growth.md` |
| AI Architecture | `docs/09_AI.md` |
| Privacy and Ethics | `docs/10_Privacy.md` |
| MVP Scope | `docs/11_MVP.md` |
| Roadmap | `docs/12_Roadmap.md` |

每份文件都必須回答：

> 如果這份文件消失，產品會失去什麼？

若文件沒有明確回答這個問題，表示它尚未具備存在必要性。

## What We Will Never Do

Life OS 永遠不應：

- 把 AI 包裝成不可質疑的權威。
- 鼓勵使用者把人生決策外包給 AI。
- 在缺乏證據時對使用者人格下定論。
- 用成癮式設計增加使用者依賴。
- 把使用者脆弱資料當作商業化燃料。
- 為了短期技術便利犧牲產品哲學。
- 為了快速發布而刪除反思、證據與隱私原則。
- 讓程式碼、模型限制或 UI 趨勢反向支配 Constitution。

## What Success Means

Life OS 的成功，是使用者變得更理解自己。

成功不是 AI 看起來更聰明，而是使用者更能看見自己的模式。成功不是回覆更像答案，而是回覆更能促進覺察。成功不是使用者更依賴產品，而是使用者在需要產品時能得到清楚支持，在不需要產品時能更自由地生活。

長期而言，Life OS 的成功意味著：

- 使用者能形成更可信的自我紀錄。
- 使用者能辨識重複出現的情緒、關係與行為模式。
- 使用者能在重要選擇前看見自己的價值與證據。
- 使用者能從 AI 互動中增強自我理解，而非削弱判斷力。
- 團隊能透過文件維持產品一致性。

## What Failure Means

Life OS 的失敗，不只是功能沒完成或商業成長不足。

更深的失敗是：

- 使用者開始把 AI 當成神諭。
- AI 讓使用者更依賴、更焦慮、更少相信自己。
- 文件失去治理能力，產品由短期需求牽引。
- 隱私被當成後補功能，而不是前提。
- 模型輸出被誤認為真理。
- 團隊忘記產品為什麼存在。

如果 Life OS 變成更漂亮的建議機器，而不是更深的自我理解系統，它就偏離了本憲法。

## Decision-making Principles

所有重大決策都應依序詢問：

1. 這是否符合 We Build Mirrors, Not Oracles？
2. 這是否讓使用者更獨立，而不是更依賴？
3. 這是否先建立反思，再提供答案？
4. 這是否有足夠證據支持？
5. 這是否促進長期成長，而非短期 engagement？
6. 這是否保護使用者隱私與主體性？
7. 這是否符合 Documentation Hierarchy？
8. 這是否需要 ADR？

若答案不清楚，應先補文件，而不是先寫 code。

## Engineering Culture

Life OS 的工程文化必須服務產品哲學。

工程不只是實作功能，而是把價值觀變成可靠系統。程式碼需要可維護，資料模型需要可追溯，AI 行為需要可審查，隱私設計需要預設保護，產品決策需要留下紀錄。

工程團隊應重視：

- 明確的 source of truth。
- 可追溯的架構決策。
- 對使用者資料的最小化收集。
- 對 AI 輸出的證據標示。
- 對長期維護成本的誠實評估。
- 對產品哲學的技術落實。

技術可以提出限制，但不能以限制推翻產品靈魂。

## AI Agent Behavior Rules

任何 AI Agent、Claude Code、Codex 或自動化協作者在本 repository 工作時，必須遵守以下規則：

1. 先讀高階文件，再修改低階文件或 code。
2. 若使用者要求與 Constitution 衝突，必須指出衝突。
3. 不得在多個文件重複定義同一核心概念。
4. 修改重要概念時，必須更新 primary source of truth。
5. 技術決策若有長期影響，必須建立或更新 ADR。
6. 不得以模型自信取代文件證據。
7. 不得把短期實作便利置於 Documentation Hierarchy 之上。
8. 任何新增文件都必須說明它失去後產品會失去什麼。

AI Agent 在 Life OS 中不是自由創作器，而是受文件治理的協作者。

## Long-term Commitment

Life OS 是長期主義產品。

它不追求把所有問題快速變成聊天功能，也不急著把人的複雜性壓成簡單分類。它承認自我理解需要時間、證據、記憶、反思與修正。

我們的長期承諾是：

- 保護人的主體性。
- 建立值得信任的自我知識系統。
- 讓 AI 成為鏡子，而不是神諭。
- 讓文件成為產品記憶，而不是附屬物。
- 讓成長優先於黏著。
- 讓隱私優先於利潤。

如果未來 Life OS 變得更大、更複雜、更商業化，這份 Constitution 必須仍然能回答最基本的問題：

> 我們究竟在幫助人變成什麼？
