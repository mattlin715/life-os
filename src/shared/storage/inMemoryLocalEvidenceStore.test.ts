import { describe, expect, it } from "vitest";
import { createInMemoryLocalEvidenceStore } from "./inMemoryLocalEvidenceStore";
import { assembleHistoricalContextPacket, createHistoricalConsentEvent, type HistoricalQuestionArtifact } from "../../historicalContext/governedPacket";

const provenance = (id: string) => ({ origin: "local_mock" as const, sourceEntryId: id, sourceArtifactIds: [], provider: "mock" as const, model: null, harnessVersion: "harness-v1", promptVersion: "v1", generatedAt: new Date().toISOString() });
describe("LocalEvidenceStore artifact contract", () => {
  it("hydrates valid artifacts but never rejected artifacts", async () => {
    const store = createInMemoryLocalEvidenceStore(); const entry = await store.createExperience({ body: "A contextual experience with enough detail to review." }); const stamp = new Date().toISOString();
    await store.saveArtifacts(entry.id, { evidence: [{ id: "keep", sourceEntryId: entry.id, text: "kept", kind: "observation", status: "confirmed", userEditable: true, provenance: provenance(entry.id), createdAt: stamp, updatedAt: stamp }, { id: "drop", sourceEntryId: entry.id, text: "drop", kind: "other", status: "rejected", userEditable: true, provenance: provenance(entry.id), createdAt: stamp, updatedAt: stamp }], reflections: [], patterns: [], recoveryTurns: [] });
    expect((await store.listArtifacts(entry.id)).evidence.map((item) => item.id)).toEqual(["keep"]);
  });
  it("invalidates dependencies on edit and deletion", async () => {
    const store = createInMemoryLocalEvidenceStore(); const entry = await store.createExperience({ body: "Original body." }); const stamp = new Date().toISOString();
    await store.saveArtifacts(entry.id, { evidence: [{ id: "e", sourceEntryId: entry.id, text: "kept", kind: "observation", status: "confirmed", userEditable: true, provenance: provenance(entry.id), createdAt: stamp, updatedAt: stamp }], reflections: [], patterns: [], recoveryTurns: [] });
    await store.updateExperience(entry.id, { body: "Revised body." }); expect((await store.listArtifacts(entry.id)).evidence).toHaveLength(0);
    await store.deleteExperience(entry.id); expect(await store.getExperience(entry.id)).toBeNull(); expect((await store.listArtifacts(entry.id)).evidence).toHaveLength(0);
  });
  it("persists successful actual-use provenance only with current revisions and shares source deletion lifecycle", async () => {
    const store = createInMemoryLocalEvidenceStore();
    const current = await store.createExperience({ body: "Current project conversation." });
    const source = await store.createExperience({ body: "Earlier project conversation." });
    const candidate = { sourceExperienceId: source.id, sourceCreatedAt: source.createdAt, sourceUpdatedAt: source.updatedAt, sourceExcerpt: source.body, reasons: [{ kind: "shared_visible_terms" as const, terms: ["project"] }], confirmedEvidenceIds: [], answeredReflectionIds: [], ranking: { algorithmVersion: "local-lexical-v1" as const, score: 10, matchedTermCount: 1 } };
    const packet = await assembleHistoricalContextPacket({ currentExperience: current, selectedCandidates: [candidate], artifactsByEntryId: { [source.id]: { experience: source, evidence: [], reflections: [] } }, explicitlyIncludedArtifactIds: new Set(), locale: "en", provider: "openai", model: "model", retentionDisclosure: "disclosed" });
    const consent = createHistoricalConsentEvent(packet); await store.saveHistoricalConsent(consent);
    const transmission = { id: "transmission", consentId: consent.id, packetDigest: packet.packetDigest, provider: "openai" as const, model: "model", outcome: "sent" as const, createdAt: packet.assembledAt, expiresAt: consent.expiresAt }; await store.saveHistoricalTransmission(transmission);
    const artifact: HistoricalQuestionArtifact = { id: "historical", currentExperienceId: current.id, questions: [], packet, consentId: consent.id, transmissionId: transmission.id, generatedAt: packet.assembledAt };
    await expect(store.saveHistoricalQuestionArtifact({ ...artifact, id: "mismatched", packet: { ...packet, destination: { ...packet.destination, model: "different" } } })).resolves.toEqual({ status: "stale_generation" });
    await expect(store.saveHistoricalQuestionArtifact(artifact)).resolves.toEqual({ status: "committed" });
    expect(await store.listHistoricalQuestionArtifacts(current.id)).toHaveLength(1);
    await store.updateExperience(source.id, { body: "Changed source." });
    expect(await store.listHistoricalQuestionArtifacts(current.id)).toHaveLength(0);
    await expect(store.saveHistoricalQuestionArtifact(artifact)).resolves.toEqual({ status: "stale_generation" });
  });
});
