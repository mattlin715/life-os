---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/05
depends:
  - docs/00_Constitution.md
referenced_by:
  - docs/00_Constitution.md
---

# Life OS

> We Build Mirrors, Not Oracles.  
> 我們打造的是鏡子，而不是神諭。

Life OS 是一個人生作業系統。它結合 AI 心理陪伴、長期記憶、覺察引擎、成長引擎、Evidence-based Personality Analysis 與 Digital Self Model，目標不是替人做決定，而是幫助人更清楚地理解自己。

這個 repository 是 Life OS 的 Product Knowledge Base。它不是單純的文件夾，而是產品哲學、設計原則、架構判斷與未來實作的共同地基。

## Life OS 是什麼

Life OS 是一個幫助使用者建立自我理解、長期記憶與成長迴路的 AI 系統。

它把 AI 視為一面鏡子：協助使用者看見自己的模式、矛盾、價值、偏好、盲點與成長方向。它不把 AI 包裝成權威，也不鼓勵使用者把人生判斷外包給模型。

Life OS 的核心任務是讓使用者變得更獨立，而不是更依賴。

## Life OS 不是什麼

Life OS 不是普通 AI Chat App。聊天只是介面，不是產品本質。

Life OS 不是單純 AI Journal。記錄只是原料，不是最終價值。

Life OS 不是神諭系統。它不以「給答案」作為成功指標，也不把模型輸出視為真理。

Life OS 不是情緒操控或 engagement machine。它不以延長停留時間、製造依賴、刺激回訪作為核心增長策略。

## 為什麼世界需要 Life OS

人們正在把越來越多思考交給 AI，但人的自我理解並不會因為得到更多答案而自然變好。沒有結構化記憶、證據、反思與長期脈絡，AI 很容易變成更快的建議機器，而不是更好的理解工具。

Life OS 回應的是一個更深的問題：在 AI 變得越來越會回答之後，人要如何保有自己的判斷、主體性與成長能力？

我們相信，好的 AI 產品應該讓人更接近自己，而不是更遠離自己。

## 專案目前狀態

目前 Life OS 處於產品地基階段。這個階段的重點不是快速堆功能，而是建立能支撐未來五年決策的文件系統。

目前 repository 已建立：

- Product Constitution
- Vision / Philosophy / Principles 文件骨架
- Identity、Memory、Awareness、Growth、AI、Privacy 等核心概念文件
- MVP 與 Roadmap 文件骨架
- ADR 文件目錄
- Appendix 與 Research 區域

## 文件導覽

- [Constitution](docs/00_Constitution.md)：最高產品憲法，定義不可違背的原則。
- [Vision](docs/01_Vision.md)：產品願景的主要定義位置。
- [Philosophy](docs/02_Philosophy.md)：產品哲學的主要定義位置。
- [Principles](docs/03_Principles.md)：設計原則的主要定義位置。
- [Problem](docs/04_Problem.md)：說明為什麼世界需要 Life OS。
- [Identity](docs/05_Identity.md)：Identity Model 的主要定義位置。
- [Memory](docs/06_Memory.md)：Memory Model 的主要定義位置。
- [Awareness](docs/07_Awareness.md)：覺察引擎的主要定義位置。
- [Growth](docs/08_Growth.md)：成長引擎的主要定義位置。
- [AI](docs/09_AI.md)：AI 架構與行為模型的主要定義位置。
- [Privacy](docs/10_Privacy.md)：隱私與倫理的主要定義位置。
- [MVP](docs/11_MVP.md)：第一版產品範圍。
- [Roadmap](docs/12_Roadmap.md)：長期演進方向。
- [ADR](docs/adr/)：架構與治理決策紀錄。

## 如何閱讀這個 Repository

請先讀高階文件，再讀低階文件。Life OS 的文件不是平面筆記，而是有治理順序的知識系統。

如果某個技術選擇、功能設計或 UI 決策與高階文件衝突，以上層文件為準。Code 是實作，不是產品哲學的來源。

每份文件都應回答一個問題：

> 如果這份文件消失，產品會失去什麼？

如果答案不清楚，文件就需要被重寫、合併或降級。

## Documentation Hierarchy

Life OS 採用 Documentation Hierarchy 作為最高文件治理規則：

- Level 0: Constitution
- Level 1: Vision, Philosophy
- Level 2: Principles, Problem, Identity
- Level 3: Memory, Awareness, Growth, AI, Privacy
- Level 4: MVP, Roadmap, Implementation planning
- Level 5: Code

下層文件必須服從上層文件。技術限制不能推翻產品哲學。程式碼不得反向修改 Constitution。

完整規則見 [docs/00_Constitution.md](docs/00_Constitution.md)。

## 給 AI Agent / Claude Code / Codex 的閱讀順序

任何 AI Agent 在進行文件修改、產品設計或程式實作前，必須先依序閱讀：

1. README.md
2. docs/00_Constitution.md
3. docs/01_Vision.md
4. docs/02_Philosophy.md
5. docs/03_Principles.md
6. docs/11_MVP.md

AI Agent 不應只根據單一 issue 或 prompt 工作。若使用者要求與 Constitution 或高階文件衝突，AI Agent 必須指出衝突，並請求明確決策，而不是默默執行。
