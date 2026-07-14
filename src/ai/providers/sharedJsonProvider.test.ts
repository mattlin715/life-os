import { describe, expect, it, vi } from "vitest";
import { createContextPacket } from "../harness/contextPacket";
import { createJsonProvider } from "./sharedJsonProvider";
import { placeholderProvider } from "./placeholderProvider";
import { evidence, experience, recovery, reflection } from "../../test/fixtures";
import { findHistoricalContextCandidates } from "../../historicalContext/retrieve";
import type { HistoricalContextPacket } from "../../historicalContext/governedPacket";
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
  it("keeps local historical candidates out of the OpenAI, Gemini, and mock provider contracts", async () => {
    const current = experience("current", "A deadline stayed with me.");
    const historicalSourceId = "source-id-unique-8e1b";
    const historicalSourceText = "A singular earlier deadline source text that must remain local.";
    const historical = experience(historicalSourceId, historicalSourceText);
    const candidates = findHistoricalContextCandidates({ currentExperience: current, experiences: [current, historical], artifactsByEntryId: {}, locale: "en" });
    expect(candidates).toHaveLength(1);

    const expectedTransportKeys = [
      "answeredClarificationTurns",
      "answeredReflectionResponses",
      "confirmedEvidence",
      "currentExperience",
      "harnessVersion",
      "locale",
      "model",
      "promptVersion",
      "provider",
      "requestedTask",
    ];
    const historicalTransportKeys = [
      "historicalContext",
      "historicalCandidates",
      "historicalSourceCollection",
      "historicalSourceExperiences",
      "historicalSources",
      "selectedHistoricalContext",
      "selectedHistory",
    ];
    const captureTransport = async (provider: "openai" | "gemini") => {
      let capturedPayload: unknown;
      const request = vi.fn(async (_instructions: string, input: unknown) => {
        capturedPayload = JSON.parse(JSON.stringify(input));
        return { candidates: [{ kind: "observation", text: "A direct observation." }] };
      });
      const packet = createContextPacket({ currentExperience: current, locale: "en", requestedTask: "evidence", provider, model: `${provider}-test` }).packet;
      await createJsonProvider(request).extractEvidence(packet);
      return { request, payload: capturedPayload as Record<string, unknown> };
    };

    for (const provider of ["openai", "gemini"] as const) {
      const { request, payload } = await captureTransport(provider);
      expect(request).toHaveBeenCalledTimes(1);
      expect(Object.keys(payload).sort()).toEqual(expectedTransportKeys);
      for (const key of historicalTransportKeys) expect(payload).not.toHaveProperty(key);
      const sharedJsonTransport = JSON.stringify(payload);
      expect(sharedJsonTransport).not.toContain(historicalSourceId);
      expect(sharedJsonTransport).not.toContain(historicalSourceText);
    }

    const mockPacket = createContextPacket({ currentExperience: current, locale: "en", requestedTask: "evidence", provider: "mock", model: null }).packet;
    const [mockArtifact] = await placeholderProvider.extractEvidence(mockPacket);
    const mockResult = JSON.stringify(mockArtifact);
    expect(mockResult).not.toContain(historicalSourceId);
    expect(mockResult).not.toContain(historicalSourceText);
    for (const key of historicalTransportKeys) expect(mockArtifact).not.toHaveProperty(key);
  });
  it("uses one provider-independent historical contract and never permits mock fallback", async () => {
    const base: Omit<HistoricalContextPacket, "destination"> = {
      packetId: "packet", packetDigest: "digest", schemaVersion: "historical-packet-v1", assembledAt: "2026-07-14T00:00:00.000Z", expiresAt: "2026-07-14T00:10:00.000Z",
      currentExperience: { id: "current", revision: "r-current", content: "Current exact text" }, task: "historical_reflection_questions", purpose: "invite_user_comparison_without_cross_time_conclusions", locale: "en", responseLanguage: "en",
      versions: { harness: "harness-v1", prompt: "historical-reflection-question-v1", outputSchema: "historical-question-output-v1", safetyContract: "phase-3b-safety-v1" },
      includedItems: [{ itemType: "experience", sourceExperienceId: "source", artifactId: null, revision: "r-source", authorship: "user", reviewState: "persisted", content: "Historical exact text", relevanceReason: "shared term", retrievalAlgorithmVersion: "local-lexical-v1" }],
      consent: { reference: "consent", scope: "one_generation_one_purpose" }, limits: { maxSources: 3, maxContentCharacters: 6000 },
    };
    const captures: unknown[] = [];
    for (const providerName of ["openai", "gemini"] as const) {
      const packet = { ...base, destination: { provider: providerName, model: "model", retentionDisclosure: "disclosed" } } as HistoricalContextPacket;
      const provider = createJsonProvider(async (_instructions, input) => { captures.push(input); return { questions: [{ text: "What feels important when you read both moments?", sourceExperienceIds: ["source"] }] }; });
      await expect(provider.generateHistoricalReflectionQuestions(packet)).resolves.toHaveLength(1);
    }
    const normalized = captures.map((capture) => ({ ...(capture as Record<string, unknown>), destination: { ...(capture as HistoricalContextPacket).destination, provider: "provider" } }));
    expect(normalized[0]).toEqual(normalized[1]);
    await expect(placeholderProvider.generateHistoricalReflectionQuestions({ ...base, destination: { provider: "openai", model: "model", retentionDisclosure: "disclosed" } } as HistoricalContextPacket)).rejects.toThrow("fallback is prohibited");
  });
});
