import type { PersistedArtifactBundle, SaveArtifactsOptions } from "./types";
import type { LocalEvidenceStore } from "./types";
import { isGenerationSnapshotCompatible, isGenerationSnapshotCurrent } from "../../ai/harness/generationSnapshot";
export type ArtifactMutation = (current: PersistedArtifactBundle) => PersistedArtifactBundle;
export interface ArtifactMutationRunnerOptions { store: LocalEvidenceStore; onCommitted: (entryId: string, bundle: PersistedArtifactBundle) => void; onError: (error: unknown) => void; }
export interface ArtifactMutationOutcome { status: "committed" | "stale_generation" | "failed"; bundle?: PersistedArtifactBundle; validationIssues: string[]; }
export function createArtifactMutationRunner(options: ArtifactMutationRunnerOptions) {
  let queue: Promise<unknown> = Promise.resolve();
  return async (entryId: string, mutation: ArtifactMutation, saveOptions?: SaveArtifactsOptions): Promise<ArtifactMutationOutcome> => {
    const operation = queue.then(async () => {
      try {
        const durable = await options.store.listArtifacts(entryId);
        const snapshot = saveOptions?.generationSnapshot;
        if (snapshot) {
          const durableExperience = await options.store.getExperience(entryId);
          if (!isGenerationSnapshotCompatible(snapshot) || !isGenerationSnapshotCurrent(snapshot, durableExperience, durable)) {
            return { status: "stale_generation" as const, validationIssues: [] };
          }
        }
        const result = await options.store.saveArtifacts(entryId, mutation(durable), {
          ...saveOptions,
          expectedExperienceUpdatedAt: snapshot?.sourceExperienceUpdatedAt ?? saveOptions?.expectedExperienceUpdatedAt,
        });
        if (result.status === "stale_generation") return { status: "stale_generation" as const, validationIssues: result.validationIssues };
        options.onCommitted(entryId, result.bundle);
        return { status: "committed" as const, bundle: result.bundle, validationIssues: result.validationIssues };
      } catch (error) {
        options.onError(error);
        try { options.onCommitted(entryId, await options.store.listArtifacts(entryId)); } catch { /* original error remains authoritative */ }
        return { status: "failed" as const, validationIssues: [] };
      }
    });
    queue = operation.then(() => undefined, () => undefined);
    return operation;
  };
}
