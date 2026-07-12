import { describe, expect, it } from "vitest";
import { recovery } from "../../test/fixtures";
import { decidePatternAvailability } from "./patternAvailability";

const founderReportedTraditionalChineseExperience =
  "今天和同事討論專案方向時，我聽見對方的回應後感到有些遲疑。會議結束後，這段對話一直留在心裡，也讓我反覆想著自己當時沒有說出口的部分。";

const decide = (overrides: Partial<Parameters<typeof decidePatternAvailability>[0]> = {}) =>
  decidePatternAvailability({
    body: founderReportedTraditionalChineseExperience,
    recoveryTurns: [],
    confirmedEvidenceCount: 1,
    hasDirtyReflectionDraft: false,
    isPatternPending: false,
    ...overrides,
  });

describe("Pattern availability", () => {
  it("blocks the founder-reported Traditional Chinese Experience when recovery is unsaved", () => {
    expect(decide().reason).toBe("insufficient_context");
    expect(decide().available).toBe(false);
  });

  it("allows the same Experience after saved Context Recovery", () => {
    expect(decide({ recoveryTurns: [recovery("answered")] }).reason).toBe("available");
  });

  it("allows a short Experience after saved Context Recovery", () => {
    expect(decide({ body: "今天很亂。", recoveryTurns: [recovery("answered")] }).available).toBe(true);
  });

  it("uses the required reason precedence", () => {
    expect(decide({ confirmedEvidenceCount: 0, hasDirtyReflectionDraft: true, isPatternPending: true }).reason).toBe("pending");
    expect(decide({ confirmedEvidenceCount: 0, hasDirtyReflectionDraft: true }).reason).toBe("missing_confirmed_evidence");
    expect(decide({ hasDirtyReflectionDraft: true }).reason).toBe("unsaved_reflection");
  });

  it("does not treat skipped Reflection as a Pattern blocker", () => {
    expect(decide({ body: founderReportedTraditionalChineseExperience.repeat(3) }).reason).toBe("available");
  });

  it("does not let a skipped Context Recovery improve sufficiency", () => {
    expect(decide({ recoveryTurns: [recovery("skipped")] }).reason).toBe("insufficient_context");
  });
});
