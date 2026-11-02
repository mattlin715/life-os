import type { ContextPacket } from "../ai/harness/contextPacket";
import { createGenerationSnapshot, isGenerationSnapshotCurrent } from "../ai/harness/generationSnapshot";
import type { ExperienceEntry, ReflectionPrompt } from "../types/domain";
import type { ArtifactMutation, ArtifactMutationOutcome } from "../shared/storage/artifactMutation";
import type { LocalEvidenceStore, PersistedArtifactBundle, SaveArtifactsOptions } from "../shared/storage/types";
import { reflectionWritePrecondition } from "../shared/storage/reflectionWritePrecondition";

export interface ReflectionGenerationOptions {
  flights: Set<string>;
  store: LocalEvidenceStore;
  buildPacket: (source: ExperienceEntry, durable: PersistedArtifactBundle) => ContextPacket;
  generate: (packet: ContextPacket) => Promise<ReflectionPrompt[]>;
  mutate: (entryId: string, mutation: ArtifactMutation, options: SaveArtifactsOptions) => Promise<ArtifactMutationOutcome>;
  onPending: (pending: boolean) => void;
  onError: (error: unknown) => void;
}
export interface ReflectionGenerationOutcome {
  status: "committed" | "busy" | "no_evidence" | "empty" | "stale_generation" | "failed";
  bundle?: PersistedArtifactBundle;
  addedPromptIds?: string[];
}

/** Product entry shared by App and focused tests, not a test-owned mutation copy. */
export async function generateReflectionForSource(entry: ExperienceEntry, options: ReflectionGenerationOptions): Promise<ReflectionGenerationOutcome> {
  // Synchronous, before the first await: render-time disabled alone cannot protect reentry.
  if (options.flights.has(entry.id)) return { status: "busy" };
  options.flights.add(entry.id);
  try {
    options.onPending(true);
    const source = await options.store.getExperience(entry.id);
    if (!source) return { status: "stale_generation" };
    const durable = await options.store.listArtifacts(entry.id);
    const packet = options.buildPacket(source, durable);
    const snapshot = createGenerationSnapshot(packet);
    const expectedReflectionSet = reflectionWritePrecondition(durable);
    if (!packet.confirmedEvidence.length) return { status: "no_evidence" };
    const prompts = await options.generate(packet);
    if (prompts.length === 0) return { status: "empty" };
    const latestSource = await options.store.getExperience(entry.id);
    const latest = await options.store.listArtifacts(entry.id);
    if (!isGenerationSnapshotCurrent(snapshot, latestSource, latest) || latestSource?.body !== source.body || expectedReflectionSet !== reflectionWritePrecondition(latest)) return { status: "stale_generation" };
    const append = (current: PersistedArtifactBundle): PersistedArtifactBundle => {
      const evidenceIds = new Set(current.evidence.filter(item => item.status === "confirmed").map(item => item.id));
      const ids = new Set([...current.evidence, ...current.reflections, ...current.patterns, ...current.recoveryTurns].map(item => item.id));
      for (const prompt of prompts) {
        if (!prompt.id || ids.has(prompt.id) || prompt.sourceEntryId !== source.id || prompt.status !== "suggested" || !prompt.question.trim() ||
          prompt.sourceEvidenceIds.length === 0 || prompt.sourceEvidenceIds.some(id => !evidenceIds.has(id))) throw new Error("reflection_generation_invalid_batch");
        ids.add(prompt.id);
      }
      return { ...current, reflections: [...current.reflections, ...prompts] };
    };
    append(latest); // Reject a malformed batch before attempting a write; recheck inside the queue too.
    const outcome = await options.mutate(entry.id, append, {
      generationSnapshot: snapshot, expectedReflectionSet,
    });
    if (outcome.status !== "committed") return { status: outcome.status };
    const bundle = outcome.bundle;
    if (!bundle || prompts.some(prompt => !bundle.reflections.some(saved => saved.id === prompt.id && saved.question === prompt.question))) {
      throw new Error("reflection_generation_commit_unconfirmed");
    }
    return { status: "committed", bundle, addedPromptIds: prompts.map(prompt => prompt.id) };
  } catch (error) {
    options.onError(error);
    return { status: "failed" };
  } finally { options.flights.delete(entry.id); options.onPending(false); }
}
