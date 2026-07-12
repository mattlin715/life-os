import { describe, expect, it } from "vitest";
import { createContextPacket } from "./contextPacket";
import { evidence, experience, recovery, reflection } from "../../test/fixtures";
const make = (overrides: Partial<Parameters<typeof createContextPacket>[0]> = {}) => createContextPacket({ currentExperience: experience(), recoveryTurns: [recovery()], evidence: [evidence()], reflections: [reflection()], locale: "en", requestedTask: "pattern", provider: "openai", model: "gpt-test", ...overrides });
describe("Context Packet integrity", () => {
  it("includes answered recovery and excludes skipped and foreign turns", () => { const result = make({ recoveryTurns: [recovery("answered"), recovery("skipped"), recovery("answered", "foreign")] }); expect(result.packet.answeredClarificationTurns.map((x) => x.sourceEntryId)).toEqual(["entry"]); expect(result.issues).toContainEqual({ code: "foreign_recovery_turn", artifactId: "recovery-foreign" }); });
  it("keeps a valid confirmed evidence/reflection chain", () => { const result = make(); expect(result.packet.confirmedEvidence).toHaveLength(1); expect(result.packet.answeredReflectionResponses).toHaveLength(1); expect(result.issues).toEqual([]); });
  it("excludes reflections backed by rejected or missing evidence", () => { const result = make({ evidence: [evidence("evidence", "entry", "rejected")], reflections: [reflection()] }); expect(result.packet.answeredReflectionResponses).toHaveLength(0); expect(result.issues).toContainEqual({ code: "reflection_missing_confirmed_evidence", artifactId: "reflection" }); });
  it("excludes foreign-entry reflections", () => { const result = make({ reflections: [reflection("foreign-reflection", "foreign")] }); expect(result.packet.answeredReflectionResponses).toHaveLength(0); expect(result.issues).toContainEqual({ code: "foreign_reflection", artifactId: "foreign-reflection" }); });
  it("reports missing source evidence", () => { const result = make({ reflections: [reflection("missing", "entry", ["missing-evidence"])] }); expect(result.packet.answeredReflectionResponses).toHaveLength(0); expect(result.issues[0].code).toBe("reflection_missing_confirmed_evidence"); });
});
