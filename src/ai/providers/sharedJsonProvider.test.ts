import { describe, expect, it, vi } from "vitest";
import { createContextPacket } from "../harness/contextPacket";
import { createJsonProvider } from "./sharedJsonProvider";
import { placeholderProvider } from "./placeholderProvider";
import { evidence, experience, recovery, reflection } from "../../test/fixtures";
describe("shared provider packet and provenance", () => {
  it("passes validated answered recovery and actual metadata", async () => {
    let captured: unknown; const request = vi.fn(async (_instructions: string, input: unknown) => { captured = input; return { candidates: [{ kind: "observation", text: "Direct observation" }] }; });
    const provider = createJsonProvider(request); const packet = createContextPacket({ currentExperience: experience(), recoveryTurns: [recovery("answered"), recovery("skipped")], locale: "en", requestedTask: "evidence", provider: "openai", model: "gpt-5-nano" }).packet;
    const [artifact] = await provider.extractEvidence(packet); expect((captured as { answeredClarificationTurns: unknown[] }).answeredClarificationTurns).toHaveLength(1); expect(artifact.provenance).toMatchObject({ provider: "openai", model: "gpt-5-nano", harnessVersion: packet.harnessVersion, promptVersion: packet.promptVersion });
  });
  it("keeps mock fallback metadata distinct", async () => { const packet = createContextPacket({ currentExperience: experience(), locale: "en", requestedTask: "evidence", provider: "mock", model: null }).packet; const [artifact] = await placeholderProvider.extractEvidence(packet); expect(artifact.provenance).toMatchObject({ provider: "mock", model: null, origin: "local_mock" }); });
  it("passes only the current Experience validated chain", async () => {
    let captured: unknown; const provider = createJsonProvider(async (_instructions, input) => { captured = input; return { text: "One tentative hypothesis." }; });
    const packet = createContextPacket({ currentExperience: experience(), evidence: [evidence(), evidence("foreign-e", "foreign")], reflections: [reflection(), reflection("foreign-r", "foreign", ["foreign-e"])], locale: "en", requestedTask: "pattern", provider: "gemini", model: "gemini-test" }).packet;
    await provider.suggestPatternNotes(packet); const input = captured as { confirmedEvidence: unknown[]; answeredReflectionResponses: unknown[] }; expect(input.confirmedEvidence).toHaveLength(1); expect(input.answeredReflectionResponses).toHaveLength(1);
  });
});
