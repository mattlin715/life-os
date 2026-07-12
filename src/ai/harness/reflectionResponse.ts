import type { ReflectionPrompt } from "../../types/domain";
export function answerReflectionPrompt(prompt: ReflectionPrompt, response: string, updatedAt = new Date().toISOString()): ReflectionPrompt {
  return { ...prompt, response: response.trim(), status: "answered", responseProvenance: { origin: "user", sourceEntryId: prompt.sourceEntryId, sourceArtifactIds: [prompt.id], generatedAt: updatedAt }, updatedAt };
}
export function skipReflectionPromptRecord(prompt: ReflectionPrompt, updatedAt = new Date().toISOString()): ReflectionPrompt {
  return { ...prompt, response: undefined, responseProvenance: undefined, status: "skipped", updatedAt };
}
