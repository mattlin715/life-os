import type { ReflectionPrompt } from "../types/domain";
import { answerReflectionPrompt } from "../ai/harness/reflectionResponse";

/**
 * Reflection text is intentionally held outside persisted artifacts until the
 * user explicitly saves it. The nested keys are exact source and prompt IDs:
 * neither ID is encoded into a delimiter-based composite key.
 */
export type ReflectionDrafts = Record<string, Record<string, string>>;

/**
 * A mutable, UI-local guard used only to reject duplicate activation before
 * React has time to render the durable result. Entries and prompts remain
 * separate map keys, so caller-controlled IDs are never delimiter-encoded.
 */
export type ReflectionSaveFlights = Map<string, Set<string>>;

export function beginReflectionSave(
  flights: ReflectionSaveFlights,
  entryId: string,
  promptId: string,
): boolean {
  const prompts = flights.get(entryId);
  if (prompts?.has(promptId)) return false;
  if (prompts) {
    prompts.add(promptId);
  } else {
    flights.set(entryId, new Set([promptId]));
  }
  return true;
}

export function endReflectionSave(
  flights: ReflectionSaveFlights,
  entryId: string,
  promptId: string,
): void {
  const prompts = flights.get(entryId);
  if (!prompts) return;
  prompts.delete(promptId);
  if (prompts.size === 0) flights.delete(entryId);
}

export function isReflectionSaveInFlight(
  flights: ReflectionSaveFlights,
  entryId: string,
  promptId: string,
): boolean {
  return flights.get(entryId)?.has(promptId) ?? false;
}

/**
 * A queued duplicate may observe the first save as already durable. Returning
 * that exact prompt prevents an unchanged answer from becoming a fabricated
 * append-only correction while preserving genuine later edits.
 */
export function answerReflectionPromptIfChanged(
  prompt: ReflectionPrompt,
  submittedDraft: string,
  updatedAt?: string,
): ReflectionPrompt {
  const normalized = submittedDraft.trim();
  if (prompt.status === "answered" && prompt.response === normalized) {
    return prompt;
  }
  return answerReflectionPrompt(prompt, submittedDraft, updatedAt);
}

export function canSaveReflectionDraft(response: string): boolean {
  return Boolean(response.trim());
}

function hasOwn(object: Record<string, string>, key: string): boolean {
  return Object.prototype.hasOwnProperty.call(object, key);
}

export function draftValue(
  drafts: ReflectionDrafts,
  entryId: string,
  prompt: ReflectionPrompt,
): string {
  const sourceDrafts = drafts[entryId];
  return sourceDrafts && hasOwn(sourceDrafts, prompt.id)
    ? sourceDrafts[prompt.id]
    : prompt.response ?? "";
}

export function isReflectionDraftDirty(
  drafts: ReflectionDrafts,
  entryId: string,
  prompt: ReflectionPrompt,
): boolean {
  const sourceDrafts = drafts[entryId];
  return Boolean(
    sourceDrafts &&
      hasOwn(sourceDrafts, prompt.id) &&
      sourceDrafts[prompt.id] !== (prompt.response ?? ""),
  );
}

export function entryHasDirtyReflectionDraft(
  drafts: ReflectionDrafts,
  entryId: string,
  prompts: ReflectionPrompt[],
): boolean {
  return prompts.some((prompt) => isReflectionDraftDirty(drafts, entryId, prompt));
}

export function setReflectionDraft(
  drafts: ReflectionDrafts,
  entryId: string,
  prompt: ReflectionPrompt,
  response: string,
): ReflectionDrafts {
  if (response === (prompt.response ?? "")) {
    return removeReflectionDraft(drafts, entryId, prompt.id);
  }

  return {
    ...drafts,
    [entryId]: { ...drafts[entryId], [prompt.id]: response },
  };
}

/** Remove one exact source/prompt draft without touching neighboring sources. */
export function removeReflectionDraft(
  drafts: ReflectionDrafts,
  entryId: string,
  promptId: string,
): ReflectionDrafts {
  const sourceDrafts = drafts[entryId];
  if (!sourceDrafts || !hasOwn(sourceDrafts, promptId)) return drafts;

  const { [promptId]: _removed, ...remainingPrompts } = sourceDrafts;
  if (Object.keys(remainingPrompts).length === 0) {
    const { [entryId]: _removedEntry, ...remainingEntries } = drafts;
    return remainingEntries;
  }

  return { ...drafts, [entryId]: remainingPrompts };
}

/** Remove all UI drafts owned by one exact source Experience. */
export function removeReflectionDraftsForEntry(
  drafts: ReflectionDrafts,
  entryId: string,
): ReflectionDrafts {
  if (!Object.prototype.hasOwnProperty.call(drafts, entryId)) return drafts;
  const { [entryId]: _removed, ...remainingEntries } = drafts;
  return remainingEntries;
}

/**
 * A save clears only the exact value that was submitted, and only after its
 * durable mutation commits. A newer edit made while the save was in flight is
 * intentionally retained.
 */
export function clearReflectionDraftAfterSuccessfulSave(
  drafts: ReflectionDrafts,
  entryId: string,
  promptId: string,
  submittedDraft: string,
  committed: boolean,
): ReflectionDrafts {
  const sourceDrafts = drafts[entryId];
  if (!committed || !sourceDrafts || sourceDrafts[promptId] !== submittedDraft) {
    return drafts;
  }
  return removeReflectionDraft(drafts, entryId, promptId);
}

/** Drop drafts invalidated by a durable bundle replacement for one exact source. */
export function reconcileReflectionDrafts(
  drafts: ReflectionDrafts,
  entryId: string,
  prompts: ReflectionPrompt[],
): ReflectionDrafts {
  const sourceDrafts = drafts[entryId];
  if (!sourceDrafts) return drafts;

  const byId = new Map(prompts.map((prompt) => [prompt.id, prompt]));
  const remainingPrompts = Object.fromEntries(
    Object.entries(sourceDrafts).filter(([promptId, value]) => {
      const prompt = byId.get(promptId);
      return Boolean(
        prompt && prompt.status !== "skipped" && value !== (prompt.response ?? ""),
      );
    }),
  );

  if (Object.keys(remainingPrompts).length === Object.keys(sourceDrafts).length) {
    return drafts;
  }
  if (Object.keys(remainingPrompts).length === 0) {
    return removeReflectionDraftsForEntry(drafts, entryId);
  }
  return { ...drafts, [entryId]: remainingPrompts };
}
