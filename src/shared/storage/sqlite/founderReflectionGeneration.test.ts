import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import payloads from "../../../../src-tauri/tests/fixtures/schema_v5/desktop_reflection_answers.json";
import { generateReflectionForSource } from "../../../app/reflectionGeneration";
import { createContextPacket } from "../../../ai/harness/contextPacket";
import { generateLocalReflectionPrompts } from "../../../ai/providers/placeholderProvider";
import { answerReflectionPrompt } from "../../../ai/harness/reflectionResponse";
import { createArtifactMutationRunner } from "../artifactMutation";
import type { PersistedArtifactBundle } from "../types";
import type { ExperienceEntry, ReflectionPrompt } from "../../../types/domain";
const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
import { createFounderSchemaV5LocalEvidenceStore } from "./founderSchemaV5LocalEvidenceStore";

beforeEach(() => { mocks.invoke.mockReset(); vi.useFakeTimers(); vi.setSystemTime(payloads.suggested.createdAt); });
afterEach(() => { vi.useRealTimers(); vi.unstubAllGlobals(); });
describe("product handler -> Founder adapter, mock IPC (Rust replay is separate)", () => {
  for (const conflictAtAdapter of [false, true]) {
    it(`captures actual generated additions and guards adapter-time conflict=${conflictAtAdapter}`, async () => {
      const at = "2026-08-13T12:00:00.000Z";
      const entry: ExperienceEntry = { id: "fixture-v4-history", body: "SYNTHETIC: a disposable reflection generation journey.", userEditable: true, createdAt: at, updatedAt: at };
      let current: PersistedArtifactBundle = { evidence: [{ id: "runtime-evidence", sourceEntryId: entry.id, text: "A synthetic observation", originalText: "A synthetic observation", kind: "observation", userEditable: true, status: "confirmed",
        provenance: { origin: "local_mock", sourceEntryId: entry.id, sourceArtifactIds: [], provider: "mock", model: null, harnessVersion: "harness-v1", promptVersion: "v1", generatedAt: at }, createdAt: at, updatedAt: "2026-08-13T12:00:01.000Z" }], reflections: [], patterns: [], recoveryTurns: [] };
      let reads = 0;
      const writes: unknown[] = [];
      mocks.invoke.mockImplementation(async (command: string, args?: { bundle: PersistedArtifactBundle }) => {
        if (command === "get_founder_v5_experience") return entry;
        if (command === "list_founder_v5_artifacts") {
          if (++reads === 4 && conflictAtAdapter) current.reflections.push({ ...payloads.suggested, id: "another-first-commit", status: "suggested" } as ReflectionPrompt);
          return structuredClone(current);
        }
        if (command === "save_founder_v5_artifacts" && args) { writes.push(structuredClone(args)); current = structuredClone(args.bundle); return { status: "committed", bundle: current }; }
        throw new Error(command);
      });
      const store = createFounderSchemaV5LocalEvidenceStore();
      const errors = vi.fn(); const committed = vi.fn();
      const mutate = createArtifactMutationRunner({ store, onError: errors, onCommitted: committed });
      const options = { flights: new Set<string>(), store,
        buildPacket: (source: typeof entry, durable: PersistedArtifactBundle) => createContextPacket({ currentExperience: source, evidence: durable.evidence, reflections: durable.reflections, locale: "en", requestedTask: "reflection", provider: "mock" }).packet,
        generate: async (packet: Parameters<typeof generateLocalReflectionPrompts>[0]) => generateLocalReflectionPrompts(packet),
        mutate, onPending: vi.fn(), onError: errors };
      vi.stubGlobal("crypto", { randomUUID: () => "runtime-reflection" });
      const first = await generateReflectionForSource(entry, options);
      if (conflictAtAdapter) {
        expect(first.status).toBe("stale_generation"); expect(writes).toEqual([]); expect(committed).not.toHaveBeenCalled();
      } else {
        expect(first.status).toBe("committed"); expect(current.reflections).toEqual([payloads.suggested]);
        vi.stubGlobal("crypto", { randomUUID: () => "runtime-reflection-2" });
        const second = await generateReflectionForSource(entry, options);
        expect(second.status).toBe("committed"); expect(writes).toHaveLength(2);
        expect(current.reflections).toEqual([payloads.suggested, { ...payloads.suggested, id: "runtime-reflection-2" }]);
        expect(answerReflectionPrompt(current.reflections[0], "Synthetic answer A", payloads.answerA.updatedAt)).toEqual(payloads.answerA);
        expect(answerReflectionPrompt(payloads.answerA as typeof current.reflections[number], "Synthetic answer B", payloads.answerB.updatedAt)).toEqual(payloads.answerB);
        console.log("FOUNDER_GENERATION_PAYLOADS", JSON.stringify(writes));
      }
      expect(errors).not.toHaveBeenCalled();
      for (const [, args] of mocks.invoke.mock.calls) expect(JSON.stringify(args ?? {})).not.toMatch(/expectedReflectionSet|life-os\.db|SELECT/);
    });
  }
});
