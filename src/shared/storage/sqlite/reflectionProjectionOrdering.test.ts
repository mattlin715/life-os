import { afterEach, expect, it, vi } from "vitest";
import capturedPayloads from "../../../../src-tauri/tests/fixtures/schema_v5/reflection_projection_ordering.json";
import { generateReflectionForSource } from "../../../app/reflectionGeneration";
import { createContextPacket } from "../../../ai/harness/contextPacket";
import { generateLocalReflectionPrompts } from "../../../ai/providers/placeholderProvider";
import { createArtifactMutationRunner } from "../artifactMutation";
import { reconcileReflectionDrafts, type ReflectionDrafts } from "../../../app/reflectionDraft";
import type { ExperienceEntry, EvidenceCandidate } from "../../../types/domain";
import type { PersistedArtifactBundle } from "../types";
const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
import { createFounderSchemaV5LocalEvidenceStore } from "./founderSchemaV5LocalEvidenceStore";

afterEach(() => { vi.useRealTimers(); vi.unstubAllGlobals(); mocks.invoke.mockReset(); });
it("captures reverse-ID same-time multi-question payloads from the real product entry and adapter (mock IPC only)", async () => {
  vi.useFakeTimers();
  const t0 = "2026-10-31T00:00:00.000Z", t1 = "2026-10-31T00:00:01.000Z", t2 = "2026-10-31T00:00:02.000Z";
  vi.setSystemTime(t2);
  const source: ExperienceEntry = { id: "ordering-source", body: "SYNTHETIC: inspect reflection projection ordering.", userEditable: true, createdAt: t0, updatedAt: t0 };
  const confirmedEvidence: EvidenceCandidate[] = [1, 2].map(n => ({ id: `ordering-evidence-${n}`, sourceEntryId: source.id,
    text: `SYNTHETIC evidence ${n}`, originalText: `SYNTHETIC evidence ${n}`, kind: "observation", status: "confirmed", userEditable: true,
    provenance: { origin: "local_mock", sourceEntryId: source.id, sourceArtifactIds: [], provider: "mock", model: null, harnessVersion: "harness-v1", promptVersion: "v1", generatedAt: t0 }, createdAt: t0, updatedAt: t1 }));
  vi.stubGlobal("crypto", { randomUUID: () => "ordering-question-M" });
  const q1 = generateLocalReflectionPrompts(createContextPacket({ currentExperience: source, evidence: confirmedEvidence.slice(0, 1), locale: "en", requestedTask: "reflection", provider: "mock" }).packet)[0];
  const captures: Record<string, unknown> = { source, confirmedEvidence };
  for (const name of ["first", "append"]) {
    const before: PersistedArtifactBundle = { evidence: confirmedEvidence, reflections: name === "append" ? [q1] : [], patterns: [], recoveryTurns: [] };
    let current = structuredClone(before);
    let request: unknown;
    let drafts: ReflectionDrafts = name === "append" ? { [source.id]: { [q1.id]: "Unsaved synthetic draft" } } : {};
    const originalDrafts = structuredClone(drafts);
    mocks.invoke.mockImplementation(async (command: string, args?: { bundle: PersistedArtifactBundle }) => {
      if (command === "get_founder_v5_experience") return source;
      if (command === "list_founder_v5_artifacts") return structuredClone(current);
      if (command === "save_founder_v5_artifacts" && args) { request = structuredClone(args); current = structuredClone(args.bundle); return { status: "committed", bundle: current }; }
      throw new Error(command);
    });
    const ids = ["ordering-question-Z", "ordering-question-A"];
    vi.stubGlobal("crypto", { randomUUID: () => ids.shift()! });
    const store = createFounderSchemaV5LocalEvidenceStore();
    const onError = vi.fn();
    const mutate = createArtifactMutationRunner({ store, onError,
      onCommitted: (_, bundle) => { drafts = reconcileReflectionDrafts(drafts, source.id, bundle.reflections); } });
    const result = await generateReflectionForSource(source, { flights: new Set(), store,
      buildPacket: (entry, durable) => createContextPacket({ currentExperience: entry, evidence: durable.evidence, reflections: durable.reflections, recoveryTurns: durable.recoveryTurns, locale: "en", requestedTask: "reflection", provider: "mock" }).packet,
      generate: async packet => generateLocalReflectionPrompts(packet), mutate, onError, onPending: vi.fn() });
    expect(result.status).toBe("committed"); // Transport stub only; real Rust replay independently checks this.
    expect(onError).not.toHaveBeenCalled(); expect(drafts).toEqual(originalDrafts);
    expect(current.reflections.map(p => p.id)).toEqual(name === "append" ? [q1.id, "ordering-question-Z", "ordering-question-A"] : ["ordering-question-Z", "ordering-question-A"]);
    expect(new Set(current.reflections.map(p => p.createdAt))).toEqual(new Set([t2]));
    captures[name] = { before, request };
  }
  expect(captures).toEqual(capturedPayloads);
  console.log("REFLECTION_ORDERING_PAYLOADS", JSON.stringify(captures));
});

it("retains the UI-only draft when native canonical order interleaves the old and new questions", () => {
  const before = capturedPayloads.append.before;
  const sourceId = capturedPayloads.source.id;
  const q1 = before.reflections[0];
  const drafts = { [sourceId]: { [q1.id]: "Unsaved synthetic draft" } };
  const canonical = [...capturedPayloads.append.request.bundle.reflections].reverse();
  // Reconciliation keys by exact ID, never position or question text.
  expect(reconcileReflectionDrafts(drafts, sourceId, canonical as Parameters<typeof reconcileReflectionDrafts>[2])).toEqual(drafts);
});
