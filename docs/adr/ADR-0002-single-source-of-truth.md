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

# ADR-0002: Adopt Single Source of Truth for Product Knowledge

## Status

Accepted

## Context

Life OS 的核心思想會出現在多份文件中。如果重複定義，未來會產生不一致。

例如，We Build Mirrors, Not Oracles、Identity Model、Memory Model、AI Role Theory 與 Privacy Theory 都會在不同場景被引用。如果每份文件都重新定義同一概念，文件會逐漸漂移，AI Agent 也會因上下文不同而做出不一致修改。

Life OS 需要一個能讓人與 AI 都清楚追蹤概念來源的知識治理方式。

## Decision

每一個重要概念只能有一個主要定義文件。其他文件只能引用、應用或延伸，不得重新定義。

若某個概念需要改變，必須修改它的 primary source of truth，而不是在其他文件另寫一個變體。

## Consequences

- 減少文件漂移。
- 提高 AI Agent 閱讀一致性。
- 未來重構文件時能追蹤概念來源。
- 重要概念需要明確標示 source of truth。
- 文件引用可以增加，但主要定義不能分裂。
- 新增文件時必須確認它是在定義新概念，還是在引用既有概念。
