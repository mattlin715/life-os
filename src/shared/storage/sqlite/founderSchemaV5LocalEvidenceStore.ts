import { invoke } from "@tauri-apps/api/core";

import type { ISODateTime } from "../../../types/domain";
import { validateArtifactBundle } from "../artifactValidation";
import type { LocalEvidenceStore, PersistedArtifactBundle } from "../types";

const createId = () => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
const now = (): ISODateTime => new Date().toISOString();
const nextRevisionAt = (current: ISODateTime): ISODateTime => {
  const parsed = Date.parse(current);
  return new Date(Number.isNaN(parsed) ? Date.now() : Math.max(Date.now(), parsed + 1)).toISOString();
};

function mutationTimestamp(current: PersistedArtifactBundle, desired: PersistedArtifactBundle): string {
  for (const key of ["evidence", "reflections", "patterns", "recoveryTurns"] as const) {
    const before = new Map(current[key].map((item) => [item.id, item]));
    const after = new Map(desired[key].map((item) => [item.id, item]));
    const ids = new Set([...before.keys(), ...after.keys()]);
    for (const id of ids) {
      const left = before.get(id);
      const right = after.get(id);
      if (JSON.stringify(left) !== JSON.stringify(right)) return right?.updatedAt ?? now();
    }
  }
  return now();
}

export function createFounderSchemaV5LocalEvidenceStore(): LocalEvidenceStore {
  const getExperience = (id: string) => invoke<Awaited<ReturnType<LocalEvidenceStore["getExperience"]>>>("get_founder_v5_experience", { id });
  return {
    async createExperience(input) {
      const occurredAt = now();
      return invoke("create_founder_v5_experience", { id: createId(), body: input.body, occurredAt });
    },
    async importExperiences(entries) {
      const result = await invoke<{ imported: number; skipped: number }>("import_founder_v5_experiences", { entries });
      return { importedCount: result.imported, skippedCount: result.skipped };
    },
    async listExperiences() { return invoke("list_founder_v5_experiences"); },
    getExperience,
    async updateExperience(id, patch) {
      const current = await getExperience(id);
      if (!current) return null;
      const occurredAt = nextRevisionAt(current.updatedAt);
      return invoke("update_founder_v5_experience", {
        id,
        expectedUpdatedAt: current.updatedAt,
        body: patch.body ?? current.body,
        occurredAt,
      });
    },
    async deleteExperience(id) {
      const current = await getExperience(id);
      if (!current) return;
      const committed = await invoke<boolean>("delete_founder_v5_experience", { id, expectedUpdatedAt: current.updatedAt });
      if (!committed) throw new Error("stale_generation");
    },
    async listArtifacts(entryId) { return invoke("list_founder_v5_artifacts", { entryId }); },
    async saveArtifacts(entryId, bundle, options = {}) {
      const validated = validateArtifactBundle(entryId, bundle);
      const current = await this.listArtifacts(entryId);
      const result = await invoke<{ status: "committed" | "stale_generation"; bundle: PersistedArtifactBundle }>("save_founder_v5_artifacts", {
        entryId,
        bundle: validated.bundle,
        expectedExperienceUpdatedAt: options.expectedExperienceUpdatedAt,
        occurredAt: mutationTimestamp(current, validated.bundle),
      });
      return { ...result, validationIssues: validated.validationIssues };
    },
    async saveHistoricalConsent(event) { await invoke("save_founder_v5_historical_consent", { event }); },
    async saveHistoricalTransmission(event) { await invoke("save_founder_v5_historical_transmission", { event }); },
    async saveHistoricalQuestionArtifact(artifact) { return invoke("save_founder_v5_historical_question", { artifact }); },
    async listHistoricalQuestionArtifacts(currentExperienceId) { return invoke("list_founder_v5_historical_questions", { currentExperienceId }); },
    async deleteHistoricalQuestionArtifact(id) { await invoke("delete_founder_v5_historical_question", { id }); },
    async purgeExpiredHistoricalAuditRecords(timestamp) { await invoke("purge_founder_v5_expired_historical_audit", { timestamp }); },
  };
}
