import { beforeEach, describe, expect, it, vi } from "vitest";
import capturedFixture from "../../../../src-tauri/tests/fixtures/schema_v5/desktop_reflection_answers.json";
import { answerReflectionPrompt } from "../../../ai/harness/reflectionResponse";
import type { ReflectionPrompt } from "../../../types/domain";
import type { PersistedArtifactBundle } from "../types";

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));

import { createFounderSchemaV5LocalEvidenceStore } from "./founderSchemaV5LocalEvidenceStore";

describe("Founder schema-v5 typed local store", () => {
  beforeEach(() => { mocks.invoke.mockReset(); });

  it("uses fixed typed commands and never supplies SQL or a database path", async () => {
    mocks.invoke.mockImplementation(async (command: string) => {
      if (command === "list_founder_v5_experiences") return [];
      if (command === "get_founder_v5_experience") return null;
      if (command === "list_founder_v5_artifacts") return { evidence: [], reflections: [], patterns: [], recoveryTurns: [] };
      return undefined;
    });
    const store = createFounderSchemaV5LocalEvidenceStore();
    await store.listExperiences();
    await store.getExperience("source");
    await store.listArtifacts("source");
    for (const [, args] of mocks.invoke.mock.calls) {
      expect(JSON.stringify(args ?? {})).not.toMatch(/SELECT|INSERT|UPDATE|DELETE|life-os\.db/i);
    }
    expect(mocks.invoke.mock.calls.map(([command]) => command)).toEqual([
      "list_founder_v5_experiences",
      "get_founder_v5_experience",
      "list_founder_v5_artifacts",
    ]);
  });

  it("pins real frontend first-save/correction payloads consumed by the Rust SQLite regression", async () => {
    const captured = capturedFixture as { suggested: ReflectionPrompt; answerA: ReflectionPrompt; answerB: ReflectionPrompt };
    const answerA = answerReflectionPrompt(captured.suggested, " Synthetic answer A ", captured.answerA.updatedAt);
    const answerB = answerReflectionPrompt(answerA, " Synthetic answer B ", captured.answerB.updatedAt);
    expect(answerA).toEqual(captured.answerA);
    expect(answerB).toEqual(captured.answerB);
    expect(Object.keys(answerA.responseProvenance!).sort()).toEqual([
      "generatedAt", "origin", "sourceArtifactIds", "sourceEntryId",
    ]);
    // Constructing/editing a UI draft is not an explicit save.
    expect(mocks.invoke).not.toHaveBeenCalled();

    const evidence: PersistedArtifactBundle["evidence"][number] = {
      id: "runtime-evidence", sourceEntryId: "fixture-v4-history", status: "confirmed",
      text: "A synthetic observation", originalText: "A synthetic observation",
      kind: "observation", userEditable: true,
      createdAt: "2026-08-13T12:00:00.000Z", updatedAt: "2026-08-13T12:00:01.000Z",
      provenance: { origin: "local_mock", sourceEntryId: "fixture-v4-history", sourceArtifactIds: [],
        provider: "mock", model: null, harnessVersion: "harness-v1", promptVersion: "v1",
        generatedAt: "2026-08-13T12:00:00.000Z" },
    };
    let current: PersistedArtifactBundle = { evidence: [evidence], reflections: [captured.suggested], patterns: [], recoveryTurns: [] };
    mocks.invoke.mockImplementation(async (command: string, args?: { bundle: PersistedArtifactBundle }) => {
      if (command === "list_founder_v5_artifacts") return current;
      if (command === "save_founder_v5_artifacts" && args) {
        current = structuredClone(args.bundle);
        Object.assign(current.reflections[0].responseProvenance!, {
          provider: null, model: null, harnessVersion: null, promptVersion: null,
        });
        return { status: "committed", bundle: current };
      }
      throw new Error(`Unexpected command ${command}`);
    });
    const store = createFounderSchemaV5LocalEvidenceStore();
    for (const answer of [answerA, answerB]) {
      const result = await store.saveArtifacts("fixture-v4-history", { ...current, reflections: [answer] });
      expect(result.status).toBe("committed");
      expect(result.bundle.reflections[0].response).toBe(answer.response);
      const call = mocks.invoke.mock.calls[mocks.invoke.mock.calls.length - 1];
      expect(call[0]).toBe("save_founder_v5_artifacts");
      expect(call[1]).toEqual({ entryId: "fixture-v4-history", bundle: { ...current, reflections: [answer] },
        expectedExperienceUpdatedAt: undefined, occurredAt: answer.updatedAt });
    }
  });
});
