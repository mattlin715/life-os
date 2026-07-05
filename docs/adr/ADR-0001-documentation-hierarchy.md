---
status: Accepted
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/05
depends:
  - docs/00_Constitution.md
referenced_by:
  - docs/00_Constitution.md
---

# ADR-0001: Adopt Documentation Hierarchy as the Governance Model

## Status

Accepted

## Context

Life OS 是一個長期產品，會經歷模型、技術、UI、平台、商業模式的變化。若沒有明確文件階層，產品哲學會被短期技術決策稀釋。

Life OS 的核心不是單一功能，而是一套關於人、AI、記憶、覺察與成長的產品哲學。這些思想必須能在未來面對不同實作選擇時保持一致。

若所有文件與程式碼被視為同一層級，低階實作很容易反向決定高階原則。這會讓產品逐漸失去方向。

## Decision

採用 Documentation Hierarchy 作為文件治理與決策模型。

Life OS 的文件與實作依照以下層級維護：

- Level 0: Constitution
- Level 1: Vision, Philosophy
- Level 2: Principles, Problem, Identity
- Level 3: Architecture-related concepts, Memory, Awareness, Growth, AI, Privacy
- Level 4: MVP, Roadmap, Implementation planning
- Level 5: Code

下層必須服從上層。若發生衝突，以上層文件為準。Constitution 是最高權威。

## Consequences

- 所有文件依照階層維護。
- 下層不得推翻上層。
- 程式碼不得推翻產品憲法。
- AI Agent 必須先讀高階文件再進行實作。
- 技術決策需要透過 ADR 記錄。
- 當技術限制與產品哲學衝突時，必須先回到高階文件討論，而不是直接讓實作決定方向。
- 新文件需要清楚說明它在 hierarchy 中的位置。
