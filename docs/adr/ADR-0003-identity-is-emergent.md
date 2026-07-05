---
status: Accepted
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/06
depends:
  - docs/03_Principles.md
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/Reflection.md
referenced_by:
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/Reflection.md
---

# ADR-0003: Identity Is Emergent

## Status

Accepted

## Context

目前 Book Zero 已完成 Problem、Identity、Memory、Principles。

Identity 不再被理解為固定人格。

Identity 也不是資料庫中的一個欄位。

Book Zero 已經建立幾個一致前提：

- Identity is dynamic.
- Identity is revealed through patterns, not declarations.
- Life OS does not collect memories. Life OS collects evidence.
- Reflection is the process where evidence and identity meet again.

因此，Life OS 需要明確決定：Identity 不是被直接儲存的資料，而是從 evidence、patterns、reflection 與 time 中浮現的結果。

## Decision

Life OS 將 Identity 定義為 emergent property。

Identity 不是 stored data。

Identity 是 evidence、patterns、reflection、time 共同形成的結果。

Memory 不直接儲存 Identity。

Memory 儲存 evidence。

Reflection 協助形成 Identity。

## Consequences

- Memory 不應把使用者固定成靜態身份欄位。
- Memory 應保存能支持理解的 evidence，而不是宣告 Identity。
- Identity 推論必須保持可修正。
- Reflection 是 Identity 形成過程中的必要環節。
- AI 可以整理 evidence、指出 patterns、提出 hypothesis，但不能宣告 Identity 的最終真相。
- 未來若建立資料模型，應避免把 Identity 設計成單一固定欄位。
- 未來若建立人格分析或 Digital Self Model，必須以 evidence、patterns、reflection、time 為基礎。
