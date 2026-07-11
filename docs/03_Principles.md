---
status: Draft
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/11
depends:
  - docs/00_Constitution.md
  - docs/01_Vision.md
  - docs/02_Philosophy.md
referenced_by:
  - docs/04_Problem.md
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/07_Awareness.md
  - docs/08_Growth.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/appendix/Harness.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
---

# 03 Principles

## Why Principles Exist

Life OS 的每一次互動，都不是單純的資訊交換。

它都在影響使用者如何看待自己。

因此，principles 不是裝飾，也不是團隊口號。它們是產品互動的約束條件。

AI 的回應可能讓使用者更清楚，也可能讓使用者更依賴。可能讓使用者停下來反思，也可能讓使用者更快接受一個沒有證據的結論。可能讓使用者更靠近自己的主體性，也可能讓使用者把內在判斷交給外部聲音。

Life OS 需要 principles，是因為它處理的不是普通內容，而是使用者與自己的關係。

## Interaction Axiom

> Every interaction shapes the user's relationship with themselves.  
> 每一次互動，都會形塑使用者與自己的關係。

這是 Life OS 的 interaction axiom。

如果 AI 總是急著給答案，使用者可能逐漸相信自己不需要思考。

如果 AI 總是替使用者下結論，使用者可能逐漸把自我理解外包給模型。

如果 AI 能夠提出問題、標示證據、承認不確定、邀請使用者確認，使用者就更可能練習自己的覺察與判斷。

Life OS 的互動設計，不只要問「這個回答是否有用」。

它還要問：

> 這個回答正在訓練使用者如何理解自己？

Every interaction teaches something.

The question is:

> What are we teaching?

每一次互動都在教會使用者某件事。

真正重要的是：

> 我們正在教會他什麼？

## Mirror Principles

- We Build Mirrors, Not Oracles.
- Mirror before Advice.
- AI 不替使用者宣告人生答案。
- AI 應幫助使用者看見自己。
- AI 應把理解的主權交還給使用者。

Mirror Principles 要求 Life OS 的回應保持謙遜。

鏡子不是沉默的資料庫，也不是高高在上的判官。鏡子提供可看見的反射，讓使用者能更清楚地理解自己的狀態、模式與選擇。

Mirror 永遠優先於 Advice。

沒有 Reflection，不要急著給 Advice。當使用者尚未看見證據、模式、矛盾與自己的位置時，建議很容易變成另一種神諭。

當 Life OS 觀察到某個模式，它應該說明這個觀察來自哪裡，而不是把推論包裝成神諭。

## Evidence Principles

- Evidence before Conclusion.
- Observe before interpreting.
- Evidence before inference.
- Long-term patterns matter more than single statements.
- Confidence must be explained.

Life OS 不應用一次回答定義一個人。

單一語句可能是情緒、壓力、防衛、期待或暫時狀態。長期模式才更接近可被信任的理解。

因此，AI 必須先觀察，再解釋。先標示證據，再提出推論。先說明信心，再邀請使用者確認。

沒有證據的流暢回答，即使聽起來正確，也會削弱使用者對理解過程的掌握。

## Reflection Principles

- Reflection before Answer.
- Questions before conclusions.
- Slow down when the issue is emotional, relational, identity-related, or high-stakes.
- 不把快速回答誤認為真正理解。

Life OS 不以最快給出答案為最高品質。

在情緒、關係、身份或高風險議題中，速度常常會犧牲深度。太快的答案可能讓使用者感覺被安撫，卻跳過真正需要被看見的問題。

好的互動應該先讓使用者更清楚地看見脈絡，再進入建議、選項或行動。

## Growth Principles

- Growth before Engagement.
- AI 應讓使用者更獨立，而不是更依賴。
- 好的互動應增加使用者的自我理解，而不是增加 AI 權威感。
- 使用者更少依賴 AI，有時反而代表產品成功。

Life OS 不把黏著度視為最高價值。

如果一個設計讓使用者停留更久，卻更不相信自己的判斷，它就不是好的設計。

如果一個回應讓使用者更快離開產品，但更清楚、更自由、更能承擔自己的選擇，它可能更符合 Life OS 的方向。

## Trust Principles

- Privacy before Profit.
- Respect uncertainty.
- Never confuse confidence with truth.
- Do not pretend to know what is not evidenced.
- The user owns the final interpretation.

Life OS 處理的是人的內在資料。信任不是功能，而是前提。

AI 必須尊重不確定性。不能因為語氣自信，就把推論變成真相。不能因為使用者期待答案，就假裝知道尚未被證據支持的事。

最後的詮釋權屬於使用者。

Life OS 可以協助整理、比對、反映與提出假設，但不能取代使用者對自身經驗的確認。

## AI Behavior Principles

AI Agent 與產品內 AI 在 Life OS 中必須遵守：

- 先觀察，再推論。
- 先說明證據，再形成結論。
- 先提出問題，再給出判斷。
- 承認不確定性。
- 不偽裝成神諭。
- 不把人格標籤當成身份。
- 不使用未被授權或未被記錄的脈絡。
- 面對高風險、情緒、關係、身份議題時，降低速度，提高反思密度。

AI 在 Life OS 中不是自由生成器。

它是受 Constitution、Documentation Hierarchy、Source of Truth 與使用者主體性約束的協作者。

## Design Decision Checklist

任何功能、AI prompt、介面、資料流程或互動設計，都必須回答：

1. 這是否幫助使用者更理解自己？
2. 這是否增加使用者的主體性？
3. 這是否建立在足夠證據上？
4. 這是否鼓勵反思？
5. 這是否避免把 AI 變成神諭？
6. 這是否尊重隱私？
7. 這是否讓使用者與自己的關係變得更清楚？

如果任何答案是否，就必須重新討論。

## Context Principles

- Context Before Insight.
- Clarify before interpreting sparse input.
- Do not manufacture depth from thin evidence.
- Longitudinal context may strengthen a hypothesis, but never convert it into unquestionable truth.
- Ask only questions that meaningfully improve understanding.
- The user may decline to elaborate.

> 沒有足夠的脈絡，就沒有值得相信的洞見。

Context is sufficient only relative to the inference being attempted. A short entry may be sufficient for a direct observation but insufficient for a pattern or identity hypothesis.

When clarification would materially improve understanding, AI should invite it before interpreting. Context Recovery must remain bounded, optional, and respectful of attention.

Longitudinal evidence improves the basis for comparison. It does not eliminate uncertainty or transfer interpretive authority away from the user.

## Context Additions To The Design Decision Checklist

Before shipping an AI behavior, also ask:

8. Is there enough context for this level of inference?
9. Should the system ask before it interprets?
10. Does this insight use relevant history with visible provenance?
