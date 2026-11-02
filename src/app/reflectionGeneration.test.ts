import { describe, expect, it, vi } from "vitest";
import { generateReflectionForSource } from "./reflectionGeneration";
import { createContextPacket } from "../ai/harness/contextPacket";
import { createInMemoryLocalEvidenceStore } from "../shared/storage/inMemoryLocalEvidenceStore";
import { createArtifactMutationRunner } from "../shared/storage/artifactMutation";
import type { LocalEvidenceStore, PersistedArtifactBundle } from "../shared/storage/types";
import type { ReflectionPrompt } from "../types/domain";
import { evidence, reflection } from "../test/fixtures";
import { reconcileReflectionDrafts, setReflectionDraft, type ReflectionDrafts } from "./reflectionDraft";
import { reflectionWritePrecondition } from "../shared/storage/reflectionWritePrecondition";
import { uiText } from "./i18n";

const question = (id: string, source: string): ReflectionPrompt => {
  const { response: _r, responseProvenance: _p, ...prompt } = reflection(id, source, ["ev"]);
  return { ...prompt, status: "suggested" };
};
async function harness(old: "none" | "suggested" | "answered" = "none") {
  const base = createInMemoryLocalEvidenceStore();
  const entry = await base.createExperience({ body: "SYNTHETIC sufficiently detailed reflection generation case." });
  const q1 = old === "answered" ? reflection("q1", entry.id, ["ev"]) : question("q1", entry.id);
  let initial: PersistedArtifactBundle = { evidence: [evidence("ev", entry.id)], reflections: old === "none" ? [] : [q1], patterns: [], recoveryTurns: [] };
  await base.saveArtifacts(entry.id, initial);
  const writes = vi.fn(base.saveArtifacts.bind(base));
  const store: LocalEvidenceStore = { ...base, saveArtifacts: writes };
  const errors = vi.fn();
  let drafts: ReflectionDrafts = old === "none" ? {} : setReflectionDraft({}, entry.id, q1, "Unsaved draft, not provider context");
  const runner = createArtifactMutationRunner({ store, onError: errors, onCommitted: (_, bundle) => { drafts = reconcileReflectionDrafts(drafts, entry.id, bundle.reflections); } });
  const generate = vi.fn(async () => [question("q2", entry.id)]);
  const pending = vi.fn();
  const options = { flights: new Set<string>(), store,
    buildPacket: (source: typeof entry, bundle = initial) => createContextPacket({ currentExperience: source, evidence: bundle.evidence,
      reflections: bundle.reflections, recoveryTurns: bundle.recoveryTurns, locale: "en", requestedTask: "reflection", provider: "mock" }).packet,
    generate, mutate: async (id: string, mutation: Parameters<typeof runner>[1], opts: Parameters<typeof runner>[2]) => {
      return runner(id, mutation, opts);
    }, onPending: pending, onError: errors };
  return { entry, base, store, writes, options, generate, errors, drafts: () => drafts,
    initial, q1, run: () => generateReflectionForSource(entry, options) };
}
describe("actual product Reflection generation coordinator", () => {
  it("creates the first question", async () => { const h = await harness(); expect((await h.run()).status).toBe("committed"); expect((await h.store.listArtifacts(h.entry.id)).reflections.map(p => p.id)).toEqual(["q2"]); });
  it.each(["suggested", "answered"] as const)("preserves %s Q1 and draft, then appends Q2", async old => {
    const h = await harness(old); const drafts = h.drafts(); expect((await h.run()).status).toBe("committed");
    const saved = await h.store.listArtifacts(h.entry.id); expect(saved.reflections).toEqual([h.q1, question("q2", h.entry.id)]); expect(h.drafts()).toEqual(drafts);
  });
  it("blocks synchronous reentry before the next render", async () => {
    const h = await harness(); let release!: (value: ReflectionPrompt[]) => void;
    h.generate.mockImplementation(() => new Promise(resolve => { release = resolve; }));
    const a = h.run(); const b = h.run();
    await vi.waitFor(() => expect(h.generate).toHaveBeenCalled());
    expect(h.generate).toHaveBeenCalledTimes(1); expect((await b).status).toBe("busy");
    release([question("q2", h.entry.id)]); await a; expect(h.writes).toHaveBeenCalledTimes(1);
  });
  it("refuses a suggested-set change while generation is pending", async () => {
    const h = await harness("suggested"); h.generate.mockImplementation(async () => {
      await h.base.saveArtifacts(h.entry.id, { ...h.initial, reflections: [...h.initial.reflections, question("other", h.entry.id)] });
      return [question("q2", h.entry.id)];
    });
    expect((await h.run()).status).toBe("stale_generation"); expect(h.writes).not.toHaveBeenCalled();
  });
  it("does not claim success for an empty batch or replace an existing ID", async () => {
    const h = await harness("suggested"); h.generate.mockResolvedValueOnce([]);
    expect((await h.run()).status).toBe("empty"); h.generate.mockResolvedValueOnce([question("q1", h.entry.id)]);
    expect((await h.run()).status).toBe("failed"); expect(h.writes).not.toHaveBeenCalled();
  });
  it.each(["source", "deleted", "evidence", "answer"] as const)("discards late generation after %s changes", async changed => {
    const h = await harness("answered"); h.generate.mockImplementation(async () => {
      if (changed === "source") await h.base.updateExperience(h.entry.id, { body: "SYNTHETIC changed source" });
      else if (changed === "deleted") await h.base.deleteExperience(h.entry.id);
      else {
        const bundle = await h.base.listArtifacts(h.entry.id);
        if (changed === "evidence") bundle.evidence[0] = { ...bundle.evidence[0], text: "Changed evidence", updatedAt: "2099-01-01T00:00:00.000Z" };
        else bundle.reflections[0] = { ...bundle.reflections[0], response: "Changed answer", updatedAt: "2099-01-01T00:00:00.000Z" };
        await h.base.saveArtifacts(h.entry.id, bundle);
      }
      return [question("q2", h.entry.id)];
    });
    expect((await h.run()).status).toBe("stale_generation"); expect(h.writes).not.toHaveBeenCalled(); expect(h.options.flights.size).toBe(0);
  });
  it("rechecks the suggested set inside the actual write queue, without retry", async () => {
    const h = await harness("suggested"); const mutate = h.options.mutate;
    h.options.mutate = async (...args) => {
      await h.base.saveArtifacts(h.entry.id, { ...h.initial, reflections: [...h.initial.reflections, question("other", h.entry.id)] });
      return mutate(...args);
    };
    expect((await h.run()).status).toBe("stale_generation"); expect(h.writes).not.toHaveBeenCalled(); expect(h.generate).toHaveBeenCalledTimes(1);
  });
  it("uses latest durable contents rather than stale loaded state; preserves unrelated artifacts", async () => {
    const h = await harness("answered"); const pattern = { id: "pattern", sourceEntryId: h.entry.id, sourceEvidenceIds: ["ev"], text: "A hypothesis", status: "candidate" as const,
      provenance: h.q1.promptProvenance, createdAt: h.q1.createdAt, updatedAt: h.q1.updatedAt };
    const latest = { ...h.initial, reflections: [...h.initial.reflections, question("other", h.entry.id)], patterns: [pattern] };
    await h.base.saveArtifacts(h.entry.id, latest);
    expect((await h.run()).status).toBe("committed");
    expect((await h.store.listArtifacts(h.entry.id))).toEqual({ ...latest, reflections: [...latest.reflections, question("q2", h.entry.id)] });
  });
  it("commits a whole generated batch once, then allows a later explicit request", async () => {
    const h = await harness("suggested"); h.generate.mockResolvedValueOnce([question("q2", h.entry.id), question("q3", h.entry.id)]);
    const result = await h.run(); expect(result.status).toBe("committed"); expect(result.addedPromptIds).toEqual(["q2", "q3"]); expect(h.writes).toHaveBeenCalledTimes(1);
    h.generate.mockResolvedValueOnce([question("q4", h.entry.id)]); expect((await h.run()).status).toBe("committed");
    expect((await h.store.listArtifacts(h.entry.id)).reflections.map(p => p.id)).toEqual(["q1", "q2", "q3", "q4"]);
  });
  it.each(["duplicate", "evidence_collision", "foreign", "bad_dependency"] as const)("rejects the entire %s batch before writing", async kind => {
    const h = await harness("suggested"); const q2 = question("q2", h.entry.id);
    const bad = kind === "duplicate" ? q2 : kind === "evidence_collision" ? question("ev", h.entry.id) : kind === "foreign" ? question("foreign", "elsewhere") : { ...q2, id: "bad", sourceEvidenceIds: ["missing"] };
    h.generate.mockResolvedValueOnce([q2, bad]); expect((await h.run()).status).toBe("failed"); expect(h.writes).not.toHaveBeenCalled(); expect(h.errors).toHaveBeenCalledTimes(1);
  });
  it.each([false, true])("does not claim success or retry after uncertain commit=%s; rereads actual state", async committedThenError => {
    const h = await harness("suggested"); h.writes.mockImplementationOnce(async (...args) => {
      if (committedThenError) await h.base.saveArtifacts(...args);
      throw new Error("synthetic uncertain transport acknowledgement");
    });
    const draft = h.drafts(); expect((await h.run()).status).toBe("failed"); expect(h.errors).toHaveBeenCalledTimes(1);
    expect(h.writes).toHaveBeenCalledTimes(1); expect(h.generate).toHaveBeenCalledTimes(1); expect(h.drafts()).toEqual(draft);
    expect((await h.store.listArtifacts(h.entry.id)).reflections.length).toBe(committedThenError ? 2 : 1);
    expect(h.options.flights.size).toBe(0);
  });
  it("does not trust an unconfirmed commit response", async () => {
    const h = await harness(); h.options.mutate = async () => ({ status: "committed", bundle: h.initial, validationIssues: [] });
    expect((await h.run()).status).toBe("failed"); expect(h.errors).toHaveBeenCalled();
  });
  it("keeps write preconditions order-independent and excludes suggested questions from provider context", async () => {
    const h = await harness("suggested"); expect(reflectionWritePrecondition({ ...h.initial, reflections: [h.q1, question("q2", h.entry.id)] })).toBe(reflectionWritePrecondition({ ...h.initial, reflections: [question("q2", h.entry.id), h.q1] }));
    await h.run(); const packet = h.generate.mock.calls[0]; expect(packet).toBeDefined();
    // Actual generator callback sees no suggested question or UI draft in the ContextPacket.
    const built = h.options.buildPacket(h.entry, h.initial); expect(built.answeredReflectionResponses).toEqual([]);
    expect(JSON.stringify(built)).not.toContain("Unsaved draft"); expect(JSON.stringify(built)).not.toContain('"q1"');
    for (const copy of Object.values(uiText)) { expect(copy.addReflection).not.toBe(copy.generateReflection); expect(copy.reflectionNoNewQuestion).toBeTruthy(); }
  });
  it("does not let one source's flight block a different source", async () => {
    const a = await harness(); const b = await harness(); b.options.flights = a.options.flights;
    let release!: (value: ReflectionPrompt[]) => void; a.generate.mockImplementation(() => new Promise(resolve => { release = resolve; }));
    const first = a.run(); await vi.waitFor(() => expect(a.generate).toHaveBeenCalled());
    expect((await b.run()).status).toBe("committed"); release([question("q2", a.entry.id)]); expect((await first).status).toBe("committed");
  });
  it("releases its flight on generation failure and permits only a later explicit new request", async () => {
    const h = await harness(); h.generate.mockRejectedValueOnce(new Error("SYNTHETIC generation failed"));
    expect((await h.run()).status).toBe("failed"); expect(h.generate).toHaveBeenCalledTimes(1); expect(h.options.flights.size).toBe(0);
    expect((await h.run()).status).toBe("committed");
  });
  it("also detects Evidence content changes that share the original timestamp", async () => {
    const h = await harness("suggested"); h.generate.mockImplementation(async () => {
      await h.base.saveArtifacts(h.entry.id, { ...h.initial, evidence: [{ ...h.initial.evidence[0], text: "Changed during the same clock tick" }] });
      return [question("q2", h.entry.id)];
    }); expect((await h.run()).status).toBe("stale_generation"); expect(h.writes).not.toHaveBeenCalled();
  });
  it("rechecks ID collisions against the latest bundle inside the queue", async () => {
    const h = await harness(); const mutate = h.options.mutate;
    h.options.mutate = async (...args) => {
      await h.base.saveArtifacts(h.entry.id, { ...h.initial, patterns: [{ id: "q2", sourceEntryId: h.entry.id, sourceEvidenceIds: ["ev"], text: "Another artifact won this ID", status: "candidate", provenance: h.q1.promptProvenance, createdAt: h.q1.createdAt, updatedAt: h.q1.updatedAt }] });
      return mutate(...args);
    }; expect((await h.run()).status).toBe("failed"); expect(h.writes).not.toHaveBeenCalled();
    expect((await h.store.listArtifacts(h.entry.id)).patterns[0].id).toBe("q2");
  });
});
