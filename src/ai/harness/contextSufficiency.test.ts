import { describe, expect, it } from "vitest";
import { assessContextSufficiency } from "./contextSufficiency";
describe("Context Sufficiency Gate", () => {
  it("recognizes real multilingual sparse input", () => {
    expect(assessContextSufficiency("很累")).toBe("clarification_useful");
    expect(assessContextSufficiency("今天工作很煩，但我不知道為什麼")).toBe("clarification_useful");
    expect(assessContextSufficiency("疲れた")).toBe("clarification_useful");
    expect(assessContextSufficiency("今日は仕事で少し落ち着かなかった")).toBe("clarification_useful");
  });
  it("answered clarification improves sufficiency", () => expect(assessContextSufficiency("很累", ["今天的會議沒有結論，回家後我仍一直想到同事說的話。"])).toBe("sufficient_for_tentative_hypothesis"));
  it("an adequate entry avoids unnecessary clarification", () => expect(assessContextSufficiency("Today at work I felt tense after the meeting, and I wrote down what was said because it stayed with me afterwards.")).not.toBe("clarification_useful"));
});
