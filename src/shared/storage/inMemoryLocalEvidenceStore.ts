import type { ExperienceEntry, ISODateTime } from "../../types/domain";
import type {
  CreateExperienceInput,
  LocalEvidenceStore,
  UpdateExperiencePatch,
} from "./types";

function now(): ISODateTime {
  return new Date().toISOString();
}

function createId(): string {
  return globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
}

export function createInMemoryLocalEvidenceStore(): LocalEvidenceStore {
  const entries = new Map<string, ExperienceEntry>();

  return {
    async createExperience(input: CreateExperienceInput) {
      const timestamp = now();
      const entry: ExperienceEntry = {
        id: createId(),
        body: input.body,
        createdAt: timestamp,
        updatedAt: timestamp,
        userEditable: true,
      };

      entries.set(entry.id, entry);
      return entry;
    },

    async listExperiences() {
      return Array.from(entries.values()).sort((a, b) =>
        b.createdAt.localeCompare(a.createdAt),
      );
    },

    async getExperience(id: string) {
      return entries.get(id) ?? null;
    },

    async updateExperience(id: string, patch: UpdateExperiencePatch) {
      const existing = entries.get(id);

      if (!existing) {
        return null;
      }

      const updated: ExperienceEntry = {
        ...existing,
        ...patch,
        updatedAt: now(),
      };

      entries.set(id, updated);
      return updated;
    },

    async deleteExperience(id: string) {
      entries.delete(id);
    },
  };
}
