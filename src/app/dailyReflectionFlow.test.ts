import { describe, expect, it } from "vitest";
import { nextDailyReflectionAction } from "./dailyReflectionFlow";

const base = {
  evidenceTotal: 0,
  evidencePending: 0,
  evidenceConfirmed: 0,
  reflectionTotal: 0,
  reflectionSuggested: 0,
  reflectionAnswered: 0,
};

describe("daily reflection next action", () => {
  it("moves through evidence and reflection without treating Pattern as required", () => {
    expect(nextDailyReflectionAction(base)).toBe("generate_evidence");
    expect(nextDailyReflectionAction({ ...base, evidenceTotal: 2, evidencePending: 2 })).toBe("review_evidence");
    expect(nextDailyReflectionAction({ ...base, evidenceTotal: 1, evidenceConfirmed: 1 })).toBe("generate_reflection");
    expect(nextDailyReflectionAction({ ...base, evidenceTotal: 1, evidenceConfirmed: 1, reflectionTotal: 1, reflectionSuggested: 1 })).toBe("answer_reflection");
    expect(nextDailyReflectionAction({ ...base, evidenceTotal: 1, evidenceConfirmed: 1, reflectionTotal: 1, reflectionAnswered: 1 })).toBe("reflection_complete");
  });
});
