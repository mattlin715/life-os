import { describe, expect, it } from "vitest";
import type { ExperienceEntry } from "../types/domain";
import {
  assembleHistoricalContextPacket,
  type HistoricalQuestionArtifact,
} from "./governedPacket";
import { inspectHistoricalQuestionProvenance } from "./provenanceInspector";
import type { HistoricalContextCandidate } from "./types";

const at = (day: number) => `2026-07-${String(day).padStart(2, "0")}T00:00:00.000Z`;
const experience = (id: string, body: string, day: number): ExperienceEntry => ({
  id,
  body,
  createdAt: at(day),
  updatedAt: at(day),
  userEditable: true,
});

async function artifactFixture(): Promise<HistoricalQuestionArtifact> {
  const current = experience("current", "A current project conversation stayed with me.", 14);
  const source = experience("source", "An earlier project conversation felt unfinished.", 10);
  const candidate: HistoricalContextCandidate = {
    sourceExperienceId: source.id,
    sourceCreatedAt: source.createdAt,
    sourceUpdatedAt: source.updatedAt,
    sourceExcerpt: source.body,
    reasons: [{ kind: "shared_visible_terms", terms: ["project", "conversation"] }],
    confirmedEvidenceIds: [],
    answeredReflectionIds: [],
    ranking: { algorithmVersion: "local-lexical-v1", score: 10, matchedTermCount: 2 },
  };
  const packet = await assembleHistoricalContextPacket({
    currentExperience: current,
    selectedCandidates: [candidate],
    artifactsByEntryId: { source: { experience: source, evidence: [], reflections: [] } },
    explicitlyIncludedArtifactIds: new Set(),
    locale: "en",
    provider: "openai",
    model: "model",
    retentionDisclosure: "Provider retention remains governed by the disclosed account policy.",
  }, new Date(at(14)));
  return {
    id: "historical-question",
    currentExperienceId: current.id,
    questions: [{
      id: "question",
      text: "What feels important when you read these moments?",
      sourceExperienceIds: [current.id, source.id],
    }],
    packet,
    consentId: packet.consent.reference,
    transmissionId: "transmission",
    generatedAt: "2026-07-14T00:01:00.000Z",
  };
}

describe("Historical Question actual-use provenance inspector", () => {
  it("builds the four-stage read-only view from the artifact and exact packet snapshot", async () => {
    const artifact = await artifactFixture();
    const result = await inspectHistoricalQuestionProvenance(artifact);
    expect(result.status).toBe("valid");
    if (result.status !== "valid") return;

    expect(result.viewModel).toMatchObject({
      artifact: { id: artifact.id, currentExperienceId: "current" },
      packet: {
        id: artifact.packet.packetId,
        digest: artifact.packet.packetDigest,
        consentId: artifact.consentId,
        transmissionId: artifact.transmissionId,
        provider: "openai",
        model: "model",
        purpose: "invite_user_comparison_without_cross_time_conclusions",
      },
      questions: [{ id: "question", sourceExperienceIds: ["current", "source"] }],
      includedItems: [{ snapshotReference: `source@${at(10)}` }],
    });
    expect(result.viewModel.dependencies).toEqual(expect.arrayContaining([
      { kind: "included_item_to_source", fromId: "source", sourceExperienceId: "source" },
      { kind: "question_citation_to_source", fromId: "question", sourceExperienceId: "source" },
    ]));
  });

  it.each([
    null,
    {},
    { id: "artifact" },
    { id: "artifact", packet: [] },
  ])("fails closed for malformed or incomplete hydration: %j", async (value) => {
    expect((await inspectHistoricalQuestionProvenance(value)).status).toBe("invalid");
  });

  it("refuses unsupported packet and contract versions before rendering partial details", async () => {
    const artifact = await artifactFixture();
    const unsupported = structuredClone(artifact) as unknown as Record<string, any>;
    unsupported.packet.schemaVersion = "historical-packet-v2";
    expect(await inspectHistoricalQuestionProvenance(unsupported)).toEqual({
      status: "invalid",
      reason: "unsupported",
    });

    const unsupportedPrompt = structuredClone(artifact) as unknown as Record<string, any>;
    unsupportedPrompt.packet.versions.prompt = "future-prompt";
    expect((await inspectHistoricalQuestionProvenance(unsupportedPrompt)).status).toBe("invalid");
  });

  it("refuses a packet whose exact digest no longer matches", async () => {
    const artifact = await artifactFixture();
    const changed = structuredClone(artifact);
    changed.packet.currentExperience.content = "Changed after consent";
    expect(await inspectHistoricalQuestionProvenance(changed)).toEqual({
      status: "invalid",
      reason: "digest_mismatch",
    });
  });

  it("refuses mismatched consent, current Experience, and generated chronology", async () => {
    const artifact = await artifactFixture();
    expect((await inspectHistoricalQuestionProvenance({ ...artifact, consentId: "other" })).status).toBe("invalid");
    expect((await inspectHistoricalQuestionProvenance({ ...artifact, currentExperienceId: "other" })).status).toBe("invalid");
    expect((await inspectHistoricalQuestionProvenance({ ...artifact, generatedAt: at(1) })).status).toBe("invalid");
  });

  it("refuses duplicate or orphaned packet-represented dependencies", async () => {
    const artifact = await artifactFixture();
    const duplicate = structuredClone(artifact);
    duplicate.packet.includedItems.push(structuredClone(duplicate.packet.includedItems[0]));
    expect((await inspectHistoricalQuestionProvenance(duplicate)).status).toBe("invalid");

    const orphan = structuredClone(artifact);
    orphan.packet.includedItems.push({
      ...orphan.packet.includedItems[0],
      itemType: "evidence",
      sourceExperienceId: "missing-source",
      artifactId: "evidence",
      authorship: "user_confirmed_ai_candidate",
      reviewState: "confirmed",
    });
    expect((await inspectHistoricalQuestionProvenance(orphan)).status).toBe("invalid");
  });

  it("refuses foreign, duplicate, or current-only citations", async () => {
    const artifact = await artifactFixture();
    for (const sourceExperienceIds of [["foreign"], ["source", "source"], ["current"]]) {
      const changed = structuredClone(artifact);
      changed.questions[0].sourceExperienceIds = sourceExperienceIds;
      expect((await inspectHistoricalQuestionProvenance(changed)).status).toBe("invalid");
    }
  });

  it("shows only packet-represented content and does not invent Phase 4 interpretation", async () => {
    const artifact = await artifactFixture();
    const result = await inspectHistoricalQuestionProvenance(artifact);
    expect(result.status).toBe("valid");
    if (result.status !== "valid") return;
    expect(result.viewModel.outgoingContent.currentExperience.content).toBe(artifact.packet.currentExperience.content);
    expect(result.viewModel.outgoingContent.includedItems.map((item) => item.content))
      .toEqual(artifact.packet.includedItems.map((item) => item.content));
    expect(JSON.stringify(result.viewModel)).not.toContain("recurrence analysis");
    expect(JSON.stringify(result.viewModel)).not.toContain("identity hypothesis");
  });

  it("refuses a corrupted generated artifact that crosses the Phase 3B output boundary", async () => {
    const artifact = await artifactFixture();
    artifact.questions[0].text = "Why does this pattern keep recurring?";
    expect((await inspectHistoricalQuestionProvenance(artifact)).status).toBe("invalid");
  });
});
