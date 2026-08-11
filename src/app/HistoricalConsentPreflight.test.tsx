import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import type { HistoricalContextPacket } from "../historicalContext/governedPacket";
import { HistoricalConsentPreflight } from "./HistoricalConsentPreflight";
import { uiText } from "./i18n";

describe("HistoricalConsentPreflight", () => {
  it.each(["en", "zh-TW", "ja"] as const)("shows concise consent controls and explicit exact details in %s", (locale) => {
    const packet = {
      packetId: "packet-1",
      packetDigest: "digest-1",
      schemaVersion: "historical-packet-v1",
      currentExperience: { id: "current", revision: "revision-current", content: "Current moment" },
      purpose: "invite_user_comparison_without_cross_time_conclusions",
      destination: { provider: "openai", model: "model", retentionDisclosure: "No storage requested" },
      versions: { harness: "harness-v1", prompt: "prompt-v1", outputSchema: "output-v1", safetyContract: "safety-v1" },
      includedItems: [{
        itemType: "experience",
        sourceExperienceId: "source",
        artifactId: null,
        revision: "revision-source",
        authorship: "user",
        reviewState: "persisted",
        content: "Earlier moment",
        relevanceReason: "shared visible terms",
      }],
    } as unknown as HistoricalContextPacket;
    const html = renderToStaticMarkup(
      <HistoricalConsentPreflight
        copy={uiText[locale]}
        packet={packet}
        eligibleArtifacts={[]}
        pending={false}
        onToggleSource={vi.fn()}
        onToggleArtifact={vi.fn()}
        onSend={vi.fn()}
        onCancel={vi.fn()}
        onAdjustSources={vi.fn()}
      />,
    );
    expect(html).toContain(uiText[locale].historicalPreflightSummary);
    expect(html).toContain(uiText[locale].historicalViewDetails);
    expect(html).toContain(packet.packetId);
    expect(html).toContain(packet.packetDigest);
    expect(html).toContain(uiText[locale].historicalConsentAndSend);
    expect(html).toContain(uiText[locale].historicalAdjustSources);
    expect(html).toContain(uiText[locale].historicalCancel);
    expect(html).toContain("<details");
  });
  it("does not infer consent, transmission, or selection changes from rendering disclosure", () => {
    const onSend = vi.fn();
    const onCancel = vi.fn();
    const onToggleSource = vi.fn();
    const onToggleArtifact = vi.fn();
    const packet = {
      packetId: "packet-2",
      packetDigest: "digest-2",
      schemaVersion: "historical-packet-v1",
      currentExperience: { id: "current", revision: "revision-current", content: "Current moment" },
      purpose: "invite_user_comparison_without_cross_time_conclusions",
      destination: { provider: "gemini", model: "model", retentionDisclosure: "Disclosed boundary" },
      versions: { harness: "harness-v1", prompt: "prompt-v1", outputSchema: "output-v1", safetyContract: "safety-v1" },
      includedItems: [],
    } as unknown as HistoricalContextPacket;
    renderToStaticMarkup(
      <HistoricalConsentPreflight
        copy={uiText.en}
        packet={packet}
        eligibleArtifacts={[]}
        pending={false}
        onToggleSource={onToggleSource}
        onToggleArtifact={onToggleArtifact}
        onSend={onSend}
        onCancel={onCancel}
        onAdjustSources={vi.fn()}
      />,
    );
    expect(onSend).not.toHaveBeenCalled();
    expect(onCancel).not.toHaveBeenCalled();
    expect(onToggleSource).not.toHaveBeenCalled();
    expect(onToggleArtifact).not.toHaveBeenCalled();
  });
});
