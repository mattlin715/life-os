import type { ExperienceEntry, ISODateTime } from "../../types/domain";
import { validateArtifactBundle } from "./artifactValidation";
import type { CreateExperienceInput, LocalEvidenceStore, PersistedArtifactBundle, UpdateExperiencePatch } from "./types";

const emptyBundle = (): PersistedArtifactBundle => ({ evidence: [], reflections: [], patterns: [], recoveryTurns: [] });
const copyBundle = (bundle: PersistedArtifactBundle): PersistedArtifactBundle => structuredClone(bundle);
const now = (): ISODateTime => new Date().toISOString();
const createId = (): string => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;

/** Test/deterministic implementation of the same source-scoped atomic contract as SQLite. */
export function createInMemoryLocalEvidenceStore(): LocalEvidenceStore {
  const entries = new Map<string, ExperienceEntry>();
  const artifacts = new Map<string, PersistedArtifactBundle>();

  return {
    async createExperience(input: CreateExperienceInput) {
      const timestamp = now();
      const entry: ExperienceEntry = { id: createId(), body: input.body, createdAt: timestamp, updatedAt: timestamp, userEditable: true };
      entries.set(entry.id, entry);
      return { ...entry };
    },
    async importExperiences(importedEntries) {
      let importedCount = 0; let skippedCount = 0;
      for (const entry of importedEntries) {
        if (entries.has(entry.id)) { skippedCount += 1; continue; }
        entries.set(entry.id, { ...entry, userEditable: true }); importedCount += 1;
      }
      return { importedCount, skippedCount };
    },
    async listExperiences() { return [...entries.values()].sort((a, b) => b.createdAt.localeCompare(a.createdAt)).map((entry) => ({ ...entry })); },
    async getExperience(id) { const entry = entries.get(id); return entry ? { ...entry } : null; },
    async updateExperience(id: string, patch: UpdateExperiencePatch) {
      const existing = entries.get(id); if (!existing) return null;
      const updated = { ...existing, body: patch.body ?? existing.body, updatedAt: now() };
      // The write boundary is one operation: a revised experience cannot hydrate old derived context.
      entries.set(id, updated); artifacts.delete(id);
      return { ...updated };
    },
    async deleteExperience(id) { entries.delete(id); artifacts.delete(id); },
    async listArtifacts(entryId) { return copyBundle(artifacts.get(entryId) ?? emptyBundle()); },
    async saveArtifacts(entryId, bundle, options = {}) {
      const entry = entries.get(entryId);
      if (!entry) throw new Error("Cannot save artifacts for a missing experience.");
      if (options.expectedExperienceUpdatedAt && entry.updatedAt !== options.expectedExperienceUpdatedAt) {
        return { status: "stale_generation" as const, bundle: copyBundle(artifacts.get(entryId) ?? emptyBundle()), validationIssues: [] };
      }
      const validated = validateArtifactBundle(entryId, bundle);
      artifacts.set(entryId, copyBundle(validated.bundle));
      return { status: "committed" as const, bundle: copyBundle(validated.bundle), validationIssues: validated.validationIssues };
    },
  };
}
