import { afterEach, describe, expect, it, vi } from "vitest";
import fixture from "../../../src-tauri/tests/fixtures/schema_v5/reflection_generation_capture.json";
import { createContextPacket } from "../../ai/harness/contextPacket";
import { createGenerationSnapshot, isGenerationSnapshotCurrent } from "../../ai/harness/generationSnapshot";
import { extractEvidenceCandidates, generateLocalReflectionPrompts } from "../../ai/providers/placeholderProvider";
import type { EvidenceCandidate, ExperienceEntry, ReflectionPrompt } from "../../types/domain";
import { createArtifactMutationRunner } from "./artifactMutation";
import type { PersistedArtifactBundle } from "./types";

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
import { createFounderSchemaV5LocalEvidenceStore } from "./sqlite/founderSchemaV5LocalEvidenceStore";

afterEach(() => { vi.useRealTimers(); vi.unstubAllGlobals(); mocks.invoke.mockReset(); });

describe("synthetic Reflection generation capture (mock IPC, not SQLite proof)", () => {
  for (const { queued, refuseSecond } of [
    { queued: false, refuseSecond: false }, { queued: true, refuseSecond: false },
    { queued: true, refuseSecond: true },
  ]) {
    it(`captures ${queued ? "queued" : "sequential"} replacement; replayed refusal=${refuseSecond}`, async () => {
      vi.useFakeTimers();
      const source = fixture.source as ExperienceEntry;
      const ev = fixture.evidence as EvidenceCandidate;
      const prompts = fixture.prompts as ReflectionPrompt[];
      const packet = createContextPacket({ currentExperience: source, evidence: [ev],
        locale: "zh-TW", requestedTask: "reflection", provider: "mock" }).packet;
      vi.setSystemTime(source.createdAt);
      vi.stubGlobal("crypto", { randomUUID: () => ev.id });
      expect(extractEvidenceCandidates(packet)[0]).toEqual({ ...ev, status: "candidate", updatedAt: ev.createdAt });
      const snapshot = createGenerationSnapshot(packet);
      let durable: PersistedArtifactBundle = { evidence: [ev], reflections: [], patterns: [], recoveryTurns: [] };
      const trace: unknown[] = [];
      const writes: unknown[] = [];
      mocks.invoke.mockImplementation(async (command: string, args?: { bundle: PersistedArtifactBundle }) => {
        trace.push({ command, args: structuredClone(args), before: structuredClone(durable) });
        if (command === "list_founder_v5_artifacts") return structuredClone(durable);
        if (command === "get_founder_v5_experience") return source;
        if (command === "save_founder_v5_artifacts" && args) {
          writes.push(structuredClone(args));
          // This refusal is a transport stub, independently reproduced by the real Rust replay.
          if (refuseSecond && writes.length === 2) throw new Error("founder_runtime_mutation_not_single_action");
          // Capture-only transport: Rust replay separately tests actual acceptance/refusal.
          durable = structuredClone(args.bundle);
          return { status: "committed", bundle: structuredClone(durable) };
        }
        throw new Error(`Unexpected command: ${command}`);
      });
      const run = createArtifactMutationRunner({ store: createFounderSchemaV5LocalEvidenceStore(),
        onCommitted: (_, bundle) => trace.push({ callback: "onCommitted", bundle }),
        onError: (error) => trace.push({ callback: "onError", error: String(error) }) });
      const generate = (index: number) => {
        vi.setSystemTime(prompts[index].createdAt);
        vi.stubGlobal("crypto", { randomUUID: () => prompts[index].id });
        const generated = generateLocalReflectionPrompts(packet);
        expect(generated).toEqual([prompts[index]]);
        trace.push({ generation: index + 1, snapshot, before: structuredClone(durable), generated });
        // Same replacement mutation as App.tsx generateReflectionPrompts; not a React event replay.
        return run(source.id, current => ({ ...current, reflections: generated, patterns: [] }), { generationSnapshot: snapshot });
      };
      if (queued) await Promise.all([generate(0), generate(1)]);
      else { await generate(0); expect(isGenerationSnapshotCurrent(snapshot, source, durable)).toBe(true); await generate(1); }
      const expectedWrites = prompts.map(prompt => ({ entryId: source.id,
        bundle: { evidence: [ev], reflections: [prompt], patterns: [], recoveryTurns: [] },
        expectedExperienceUpdatedAt: source.updatedAt, occurredAt: prompt.updatedAt }));
      expect(writes).toEqual(expectedWrites);
      expect(mocks.invoke.mock.calls.map(([command]) => command)).toEqual(Array(2).fill([
        "list_founder_v5_artifacts", "get_founder_v5_experience", "list_founder_v5_artifacts", "save_founder_v5_artifacts",
      ]).flat().concat(refuseSecond ? ["list_founder_v5_artifacts"] : []));
      expect(durable.reflections).toEqual([prompts[refuseSecond ? 0 : 1]]);
      if (refuseSecond) {
        expect(trace.slice(-3)).toMatchObject([
          { callback: "onError" }, { command: "list_founder_v5_artifacts" },
          { callback: "onCommitted", bundle: { reflections: [prompts[0]] } },
        ]);
      }
      console.log("REFLECTION_FRONTEND_CAPTURE", JSON.stringify({ queued, refuseSecond, evidencePlane: "mock IPC capture only", trace }));
    });
  }
});
