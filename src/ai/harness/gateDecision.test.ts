import { describe, expect, it } from "vitest";
import { decideContextGate } from "./gateDecision";
import { recovery } from "../../test/fixtures";
describe("gate enforcement", () => {
  it("invites clarification for a sparse entry", () => expect(decideContextGate("很累", []).inviteClarification).toBe(true));
  it("allows observation but blocks pattern after skip", () => { const result = decideContextGate("很累", [recovery("skipped")]); expect(result.allowObservation).toBe(true); expect(result.allowPattern).toBe(false); expect(result.sufficiency).toBe("clarification_useful"); });
  it("allows a tentative hypothesis only after context improves", () => expect(decideContextGate("很累", [recovery("answered")]).allowPattern).toBe(true));
  it("is deterministic across hydration", () => { const turn = recovery("skipped"); expect(decideContextGate("疲れた", [turn])).toEqual(decideContextGate("疲れた", [structuredClone(turn)])); });
});
