import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));

import { createFounderSchemaV5LocalEvidenceStore } from "./founderSchemaV5LocalEvidenceStore";

describe("Founder schema-v5 typed local store", () => {
  beforeEach(() => mocks.invoke.mockReset());

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
});
