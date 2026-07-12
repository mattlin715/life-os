import { describe, expect, it } from "vitest";
import { answerReflectionPrompt, skipReflectionPromptRecord } from "./reflectionResponse";
import { createContextPacket } from "./contextPacket";
import { evidence, experience, reflection } from "../../test/fixtures";
describe("Reflection response authorship", () => {
  it("separates AI prompt from user response and clears it on skip", () => { const answered = answerReflectionPrompt({ ...reflection(), response: undefined, status: "suggested" }, "My own words"); expect(answered.promptProvenance.origin).toBe("local_mock"); expect(answered.responseProvenance?.origin).toBe("user"); const skipped = skipReflectionPromptRecord(answered); expect(skipped.responseProvenance).toBeUndefined(); });
  it("updates the durable response and user provenance together", () => { const updatedAt = "2026-07-12T03:00:00.000Z"; const answered = answerReflectionPrompt(reflection(), "A revised response", updatedAt); expect(answered.response).toBe("A revised response"); expect(answered.updatedAt).toBe(updatedAt); expect(answered.responseProvenance).toMatchObject({ origin: "user", generatedAt: updatedAt }); });
  it("does not treat legacy answered response provenance as user-authored", () => { const legacy = { ...reflection(), responseProvenance: { ...reflection().promptProvenance, origin: "legacy_unknown" as const } }; const result = createContextPacket({ currentExperience: experience(), evidence: [evidence()], reflections: [legacy], locale: "en", requestedTask: "pattern", provider: "mock" }); expect(result.packet.answeredReflectionResponses).toHaveLength(0); expect(result.issues).toContainEqual({ code: "reflection_response_provenance_unknown", artifactId: legacy.id }); });
});
