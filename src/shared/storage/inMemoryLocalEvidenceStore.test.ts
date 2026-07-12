import { describe, expect, it } from "vitest";
import { createInMemoryLocalEvidenceStore } from "./inMemoryLocalEvidenceStore";

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
});
