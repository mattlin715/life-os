# Implementation Guide

## 1. Purpose

Book Zero 是產品理論。

但 Book Zero 不是永遠寫文件的理由。

Life OS 必須進入可驗證實作。文件存在，是為了讓實作更穩定，而不是取代實作。

這份文件回答：

> How do we turn Book Zero into working software without creating chaos?

它不是新的產品哲學。

它是從 Book Zero 到 MVP 的最小可執行橋樑。

## 2. Implementation Principle

> Build the smallest system that proves the theory.

建立最小系統，驗證理論是否成立。

不要一次實作完整 Life OS。

不要一次建立所有 engine。

不要過早設計完整 architecture。

不要把 Book Zero 的每個概念都立刻變成一個模組、一個服務或一個抽象框架。

第一版只需要驗證最核心循環是否真的對使用者有價值。

## 3. The Core Loop

Life OS MVP 的最小循環是：

Experience  
→ Evidence  
→ Reflection  
→ Awareness  
→ Growth

MVP 不需要完整 Digital Self。

MVP 不需要完整人格分析。

MVP 不需要完整長期 AI agent。

MVP 只需要證明一件事：

使用者輸入一段生活經驗後，系統能協助整理 evidence、提出 reflection prompt、讓使用者看見 pattern，並保存可回顧的成長脈絡。

如果這個循環不能成立，再大的架構也沒有意義。

## 4. MVP Implementation Boundary

第一版只做：

- Local-first personal app or desktop prototype
- Daily / event-based journal input
- Evidence extraction
- Reflection prompt generation
- Pattern note
- Simple memory timeline
- Manual review / edit / delete
- BYOK 或 local model friendly AI provider abstraction

第一版不做：

- 社群
- 推薦系統
- 自動診斷
- 自動人格定型
- 醫療或心理治療功能
- 複雜通知系統
- 商業化
- 多人協作
- 完整手機 App
- 完整 AI agent

這個 boundary 的目的，是保護 Life OS 不被 premature scope expansion 拉走。

## 5. Architecture Restraint

Architecture 必須服務 MVP。

Architecture 不應成為逃避驗證的地方。

禁止第一版就建立：

- Microservices
- 複雜 knowledge graph
- 過度抽象的 engine framework
- 太多 AI provider
- 雲端同步

建議第一版：

- Single app
- Local database
- Markdown / JSON export
- Simple provider interface
- Clear domain modules

第一版 architecture 的任務不是展示工程能力。

它的任務是讓 core loop 可以被測試、被修改、被丟棄或被保留。

## 6. Book Zero To Code Mapping

| Book Zero Concept | MVP Implementation |
| --- | --- |
| Mirror | AI response style that reflects evidence and asks questions without acting as an oracle |
| Evidence | Extracted structured notes from user-written experiences |
| Reflection | Generated reflective questions for the user to answer or ignore |
| Memory | Local evidence store and simple timeline |
| Identity | Not a profile; only emerging pattern notes that remain editable |
| Awareness | Pattern surfaced to the user as a hypothesis, not a conclusion |
| Growth | User-authored future action, reflection, or revised understanding |
| Privacy | Local-first, user-owned data with inspect/edit/delete |
| Agency | User can edit, delete, override, or reject AI output |

This table is a mapping, not a new theory.

If implementation pressure conflicts with Book Zero, Book Zero wins.

## 7. AI Contributor Implementation Rules

Before writing code, any AI contributor must:

1. Read `AI_CONTRIBUTOR_GUIDE.md`.
2. Read `docs/00_Index.md`.
3. Confirm which Book Zero concept the feature maps to.
4. Write the feature boundary.
5. Avoid adding undefined product theory.
6. Avoid breaking privacy, agency, or evidence principles for technical convenience.

Do not implement a feature just because it is technically easy.

Do not add abstractions because they sound future-proof.

Do not make AI more authoritative than Book Zero allows.

## 8. Definition Of Done For MVP Features

Every MVP feature must answer:

- Does this help the user understand themselves better?
- Does this preserve user agency?
- Does this use evidence instead of unsupported conclusion?
- Can the user inspect, edit, or delete the data?
- Does this avoid turning AI into an oracle?
- Is this the smallest useful version?

If any answer is unclear, the feature is not ready.

If any answer is no, the feature should be redesigned or deferred.

## 9. First Build Milestone

Milestone 0 goal:

A local prototype where the user can:

1. Write one experience.
2. Let AI extract evidence candidates.
3. Review/edit evidence.
4. Generate reflection questions.
5. Save the entry.
6. Later view patterns across entries.

This milestone is not about beautiful UI.

It is not about completeness.

It is not about proving the final product.

It is about validating whether the Life OS core loop creates real value:

Experience  
→ Evidence  
→ Reflection  
→ Awareness  
→ Growth

## 10. What We Must Not Build Yet

Do not build yet:

- Personality prediction
- MBTI inference
- Automated life advice
- Push notification habit loops
- Cloud account system
- Payment
- Social sharing
- Full mobile app
- Complex vector database
- Agent automation
- Dashboard vanity metrics

These may have future value.

But now they would distract from the only question that matters:

Can the core loop help a user understand themselves better?

## 11. Recommended Next Files

Recommended next implementation documents:

- `docs/architecture/00_MVP_Architecture.md`
- `docs/product/00_MVP_User_Flow.md`
- `docs/adr/ADR-0004-local-first-mvp.md`
- `docs/adr/ADR-0005-ai-provider-abstraction.md`

Do not create these until the team is ready to move from implementation guide to architecture planning.

These files should translate Book Zero into implementation constraints, not create new product theory.

## 12. Final Principle

> Theory must become practice, but practice must not betray theory.

理論必須走向實踐，但實踐不能背叛理論。
