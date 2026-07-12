import { describe, expect, it } from "vitest";
import { recovery } from "../../test/fixtures";
import { addSuggestedRecoveryTurn, answerRecoveryTurn, skipRecoveryTurnRecord } from "./recoveryTurn";
describe("Context Recovery authorship", () => {
  it("keeps AI/deterministic prompt authorship separate from user response", () => { const answered = answerRecoveryTurn(recovery("suggested"), "My answer", "2026-07-12T01:00:00.000Z"); expect(answered.promptProvenance.origin).toBe("local_mock"); expect(answered.responseProvenance?.origin).toBe("user"); });
  it("skip removes response authorship rather than fabricating feedback", () => { const skipped = skipRecoveryTurnRecord(recovery("answered")); expect(skipped.response).toBeUndefined(); expect(skipped.responseProvenance).toBeUndefined(); });
  it("adds one user-requested recovery opportunity without duplicating an open turn", () => {
    const suggested = recovery("suggested");
    const once = addSuggestedRecoveryTurn([], suggested);
    const twice = addSuggestedRecoveryTurn(once, recovery("suggested"));
    expect(once).toEqual([suggested]);
    expect(twice).toBe(once);
  });
});
