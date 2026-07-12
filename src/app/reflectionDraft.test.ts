import { describe, expect, it, vi } from "vitest";
import { createContextPacket } from "../ai/harness/contextPacket";
import { answerReflectionPrompt } from "../ai/harness/reflectionResponse";
import { createArtifactMutationRunner } from "../shared/storage/artifactMutation";
import { createInMemoryLocalEvidenceStore } from "../shared/storage/inMemoryLocalEvidenceStore";
import type { LocalEvidenceStore } from "../shared/storage/types";
import { evidence, experience, reflection } from "../test/fixtures";
import {
  canSaveReflectionDraft,
  clearReflectionDraftAfterSuccessfulSave,
  draftValue,
  entryHasDirtyReflectionDraft,
  isReflectionDraftDirty,
  reconcileReflectionDrafts,
  removeReflectionDraftsForEntry,
  type ReflectionDrafts,
} from "./reflectionDraft";

describe("reflection UI drafts", () => {
  it("keeps an unsaved draft outside the Context Packet", () => {
    const entry = experience();
    const prompt = reflection();
    const drafts: ReflectionDrafts = { [entry.id]: { [prompt.id]: "Unsaved private draft" } };

    expect(draftValue(drafts, entry.id, prompt)).toBe("Unsaved private draft");
    expect(isReflectionDraftDirty(drafts, entry.id, prompt)).toBe(true);
    expect(entryHasDirtyReflectionDraft(drafts, entry.id, [prompt])).toBe(true);

    const packet = createContextPacket({
      currentExperience: entry,
      evidence: [evidence()],
      reflections: [prompt],
      locale: "en",
      requestedTask: "pattern",
      provider: "mock",
    }).packet;
    expect(packet.answeredReflectionResponses[0]?.response).toBe(
      "The unfinished conversation.",
    );
    expect(packet.answeredReflectionResponses[0]?.response).not.toBe(
      "Unsaved private draft",
    );
  });

  it("normalizes whitespace only after a successful Save and clears the submitted draft", () => {
    const entry = experience();
    const prompt = reflection();
    const submittedDraft = "  I understand this more clearly now.  ";
    const drafts: ReflectionDrafts = { [entry.id]: { [prompt.id]: submittedDraft } };
    const committedPrompt = answerReflectionPrompt(prompt, submittedDraft, "2026-07-12T06:00:00.000Z");

    const afterSave = clearReflectionDraftAfterSuccessfulSave(
      drafts,
      entry.id,
      prompt.id,
      submittedDraft,
      true,
    );
    expect(committedPrompt.response).toBe("I understand this more clearly now.");
    expect(draftValue(afterSave, entry.id, committedPrompt)).toBe(
      "I understand this more clearly now.",
    );
    expect(isReflectionDraftDirty(afterSave, entry.id, committedPrompt)).toBe(false);
    expect(entryHasDirtyReflectionDraft(afterSave, entry.id, [committedPrompt])).toBe(false);
    expect(committedPrompt.responseProvenance?.origin).toBe("user");
  });

  it("preserves the draft when the durable Save fails", async () => {
    const base = createInMemoryLocalEvidenceStore();
    const entry = await base.createExperience({ body: "A durable source experience." });
    const prompt = reflection("prompt", entry.id);
    const submittedDraft = "Keep this draft";
    const drafts: ReflectionDrafts = { [entry.id]: { [prompt.id]: submittedDraft } };
    const failingStore: LocalEvidenceStore = {
      ...base,
      saveArtifacts: vi.fn(async () => {
        throw new Error("disk full");
      }),
    };
    const run = createArtifactMutationRunner({
      store: failingStore,
      onCommitted: () => undefined,
      onError: () => undefined,
    });
    const outcome = await run(entry.id, (current) => ({
      ...current,
      reflections: [answerReflectionPrompt(prompt, submittedDraft)],
    }));

    const afterFailedSave = clearReflectionDraftAfterSuccessfulSave(
      drafts,
      entry.id,
      prompt.id,
      submittedDraft,
      outcome.status === "committed",
    );
    expect(outcome.status).toBe("failed");
    expect(afterFailedSave).toBe(drafts);
    expect(draftValue(afterFailedSave, entry.id, prompt)).toBe(submittedDraft);
    expect(isReflectionDraftDirty(afterFailedSave, entry.id, prompt)).toBe(true);
  });

  it("does not permit an empty-after-trim response to be saved", () => {
    expect(canSaveReflectionDraft("   \n\t ")).toBe(false);
    expect(canSaveReflectionDraft(" an answer ")).toBe(true);
  });

  it("keeps a:b drafts when reconciling or deleting the exact source a", () => {
    const promptA = reflection("prompt", "a");
    const promptAB = reflection("prompt", "a:b");
    const drafts: ReflectionDrafts = {
      a: { prompt: "draft for a" },
      "a:b": { prompt: "draft for a:b" },
    };

    const reconciled = reconcileReflectionDrafts(drafts, "a", []);
    expect(reconciled).toEqual({ "a:b": { prompt: "draft for a:b" } });
    expect(removeReflectionDraftsForEntry(drafts, "a")).toEqual({
      "a:b": { prompt: "draft for a:b" },
    });
    expect(draftValue(reconciled, "a:b", promptAB)).toBe("draft for a:b");
    expect(promptA.sourceEntryId).toBe("a");
  });
});
