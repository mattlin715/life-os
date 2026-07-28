import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import type { ExperienceEntry } from "../types/domain";
import {
  assembleHistoricalContextPacket,
  type HistoricalQuestionArtifact,
} from "../historicalContext/governedPacket";
import { inspectHistoricalQuestionProvenance } from "../historicalContext/provenanceInspector";
import type { HistoricalContextCandidate } from "../historicalContext/types";
import {
  HistoricalProvenanceInspector,
  HistoricalProvenanceInspectorPanel,
} from "./HistoricalProvenanceInspector";
import { uiText } from "./i18n";

const at = (day: number) => `2026-07-${String(day).padStart(2, "0")}T00:00:00.000Z`;

async function fixture(): Promise<HistoricalQuestionArtifact> {
  const current: ExperienceEntry = { id: "current", body: "Current exact content.", createdAt: at(14), updatedAt: at(14), userEditable: true };
  const source: ExperienceEntry = { id: "source", body: "Historical exact content.", createdAt: at(10), updatedAt: at(10), userEditable: true };
  const candidate: HistoricalContextCandidate = {
    sourceExperienceId: source.id,
    sourceCreatedAt: source.createdAt,
    sourceUpdatedAt: source.updatedAt,
    sourceExcerpt: source.body,
    reasons: [{ kind: "shared_visible_terms", terms: ["exact"] }],
    confirmedEvidenceIds: [],
    answeredReflectionIds: [],
    ranking: { algorithmVersion: "local-lexical-v1", score: 5, matchedTermCount: 1 },
  };
  const packet = await assembleHistoricalContextPacket({
    currentExperience: current,
    selectedCandidates: [candidate],
    artifactsByEntryId: { source: { experience: source, evidence: [], reflections: [] } },
    explicitlyIncludedArtifactIds: new Set(),
    locale: "en",
    provider: "gemini",
    model: "model",
    retentionDisclosure: "disclosed",
  }, new Date(at(14)));
  return {
    id: "artifact",
    currentExperienceId: current.id,
    questions: [{ id: "question", text: "What do you notice?", sourceExperienceIds: [source.id] }],
    packet,
    consentId: packet.consent.reference,
    transmissionId: "transmission",
    generatedAt: "2026-07-14T00:01:00.000Z",
  };
}

describe("HistoricalProvenanceInspector", () => {
  it("is collapsed by default and does not render packet metadata or exact content", async () => {
    const artifact = await fixture();
    const html = renderToStaticMarkup(<HistoricalProvenanceInspector artifact={artifact} copy={uiText.en} />);
    expect(html).toContain("<details");
    expect(html).not.toContain("<details open");
    expect(html).toContain(uiText.en.historicalProvenanceOpen);
    expect(html).not.toContain(artifact.packet.packetDigest);
    expect(html).not.toContain(artifact.packet.currentExperience.content);
  });

  it.each(["en", "zh-TW", "ja"] as const)(
    "shows the local read-only boundary and four distinct lifecycle stages in %s",
    async (locale) => {
      const validation = await inspectHistoricalQuestionProvenance(await fixture());
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <HistoricalProvenanceInspectorPanel
          validation={validation}
          loading={false}
          revealContent={false}
          copy={copy}
          onRevealContent={vi.fn()}
        />,
      );
      expect(html).toContain(copy.historicalProvenanceLocalOnly);
      expect(html).toContain(copy.historicalProvenanceSelected);
      expect(html).toContain(copy.historicalProvenanceConsented);
      expect(html).toContain(copy.historicalProvenanceTransmitted);
      expect(html).toContain(copy.historicalProvenancePersisted);
      expect(html).toContain("local-lexical-v1");
      expect(html).not.toContain("Current exact content.");
      expect(html).not.toContain("Historical exact content.");
    },
  );

  it("reveals exact outgoing content only behind the second explicit control", async () => {
    const validation = await inspectHistoricalQuestionProvenance(await fixture());
    const hidden = renderToStaticMarkup(
      <HistoricalProvenanceInspectorPanel validation={validation} loading={false} revealContent={false} copy={uiText.en} onRevealContent={vi.fn()} />,
    );
    const revealed = renderToStaticMarkup(
      <HistoricalProvenanceInspectorPanel validation={validation} loading={false} revealContent copy={uiText.en} onRevealContent={vi.fn()} />,
    );
    expect(hidden).not.toContain("Current exact content.");
    expect(hidden).not.toContain("Historical exact content.");
    expect(revealed).toContain("Current exact content.");
    expect(revealed).toContain("Historical exact content.");
    expect(revealed).toContain(uiText.en.historicalProvenanceExactContentWarning);
  });

  it.each(["en", "zh-TW", "ja"] as const)(
    "fails closed without partial provenance disclosure in %s",
    (locale) => {
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <HistoricalProvenanceInspectorPanel
          validation={{ status: "invalid", reason: "contradictory" }}
          loading={false}
          revealContent={false}
          copy={copy}
          onRevealContent={vi.fn()}
        />,
      );
      expect(html).toContain('role="alert"');
      expect(html).toContain(copy.historicalProvenanceInvalid);
      expect(html).not.toContain(copy.historicalProvenanceSelected);
      expect(html).not.toContain(copy.historicalProvenanceRevealContent);
    },
  );
});
