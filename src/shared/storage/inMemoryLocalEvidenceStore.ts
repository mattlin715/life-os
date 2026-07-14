import type { ExperienceEntry, ISODateTime } from "../../types/domain";
import { validateArtifactBundle } from "./artifactValidation";
import type { CreateExperienceInput, LocalEvidenceStore, PersistedArtifactBundle, UpdateExperiencePatch } from "./types";
import type { HistoricalConsentEvent, HistoricalQuestionArtifact, HistoricalTransmissionEvent } from "../../historicalContext/governedPacket";

const emptyBundle = (): PersistedArtifactBundle => ({ evidence: [], reflections: [], patterns: [], recoveryTurns: [] });
const copyBundle = (bundle: PersistedArtifactBundle): PersistedArtifactBundle => structuredClone(bundle);
const now = (): ISODateTime => new Date().toISOString();
const createId = (): string => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;

/** Test/deterministic implementation of the same source-scoped atomic contract as SQLite. */
export function createInMemoryLocalEvidenceStore(): LocalEvidenceStore {
  const entries = new Map<string, ExperienceEntry>();
  const artifacts = new Map<string, PersistedArtifactBundle>();
  const historicalConsents = new Map<string, HistoricalConsentEvent>();
  const historicalTransmissions = new Map<string, HistoricalTransmissionEvent>();
  const historicalQuestions = new Map<string, HistoricalQuestionArtifact>();
  const invalidateHistoricalDependents = (entryId: string) => {
    for (const [artifactId, artifact] of historicalQuestions) {
      if (artifact.currentExperienceId === entryId || artifact.packet.includedItems.some((item) => item.sourceExperienceId === entryId)) {
        historicalQuestions.delete(artifactId);
        historicalTransmissions.delete(artifact.transmissionId);
        historicalConsents.delete(artifact.consentId);
      }
    }
  };

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
      entries.set(id, updated); artifacts.delete(id); invalidateHistoricalDependents(id);
      return { ...updated };
    },
    async deleteExperience(id) { invalidateHistoricalDependents(id); entries.delete(id); artifacts.delete(id); },
    async listArtifacts(entryId) { return copyBundle(artifacts.get(entryId) ?? emptyBundle()); },
    async saveArtifacts(entryId, bundle, options = {}) {
      const entry = entries.get(entryId);
      if (!entry) throw new Error("Cannot save artifacts for a missing experience.");
      if (options.expectedExperienceUpdatedAt && entry.updatedAt !== options.expectedExperienceUpdatedAt) {
        return { status: "stale_generation" as const, bundle: copyBundle(artifacts.get(entryId) ?? emptyBundle()), validationIssues: [] };
      }
      const validated = validateArtifactBundle(entryId, bundle);
      artifacts.set(entryId, copyBundle(validated.bundle)); invalidateHistoricalDependents(entryId);
      return { status: "committed" as const, bundle: copyBundle(validated.bundle), validationIssues: validated.validationIssues };
    },
    async saveHistoricalConsent(event) { historicalConsents.set(event.id, structuredClone(event)); },
    async saveHistoricalTransmission(event) {
      if (!historicalConsents.has(event.consentId)) throw new Error("Cannot record transmission without consent.");
      historicalTransmissions.set(event.id, structuredClone(event));
      if (event.outcome === "sent" || event.outcome === "failed" || event.outcome === "cancelled_after_send") {
        const consent = historicalConsents.get(event.consentId)!;
        historicalConsents.set(consent.id, { ...consent, state: "consumed" });
      }
    },
    async saveHistoricalQuestionArtifact(artifact) {
      const revisions = [{ id: artifact.packet.currentExperience.id, revision: artifact.packet.currentExperience.revision }, ...artifact.packet.includedItems.filter((item) => item.itemType === "experience").map((item) => ({ id: item.sourceExperienceId, revision: item.revision }))];
      if (revisions.some((expected) => entries.get(expected.id)?.updatedAt !== expected.revision)) return { status: "stale_generation" as const };
      for (const item of artifact.packet.includedItems.filter((record) => record.artifactId)) {
        const bundle = artifacts.get(item.sourceExperienceId) ?? emptyBundle();
        const record = item.itemType === "evidence" ? bundle.evidence.find((candidate) => candidate.id === item.artifactId && candidate.status === "confirmed") : bundle.reflections.find((prompt) => prompt.id === item.artifactId && prompt.status === "answered" && prompt.responseProvenance?.origin === "user");
        if (!record || record.updatedAt !== item.revision) return { status: "stale_generation" as const };
      }
      const consent = historicalConsents.get(artifact.consentId);
      const transmission = historicalTransmissions.get(artifact.transmissionId);
      if (!consent || consent.state !== "consumed" || consent.packetDigest !== artifact.packet.packetDigest || !transmission || transmission.consentId !== consent.id || transmission.packetDigest !== artifact.packet.packetDigest || transmission.provider !== artifact.packet.destination.provider || transmission.model !== artifact.packet.destination.model || transmission.outcome !== "sent") return { status: "stale_generation" as const };
      historicalQuestions.set(artifact.id, structuredClone(artifact));
      return { status: "committed" as const };
    },
    async listHistoricalQuestionArtifacts(currentExperienceId) { return [...historicalQuestions.values()].filter((artifact) => artifact.currentExperienceId === currentExperienceId).map((artifact) => structuredClone(artifact)); },
    async deleteHistoricalQuestionArtifact(id) {
      const artifact = historicalQuestions.get(id);
      if (!artifact) return;
      historicalQuestions.delete(id); historicalTransmissions.delete(artifact.transmissionId); historicalConsents.delete(artifact.consentId);
    },
    async purgeExpiredHistoricalAuditRecords(timestamp) {
      for (const [id, event] of historicalTransmissions) if (event.expiresAt <= timestamp && ![...historicalQuestions.values()].some((artifact) => artifact.transmissionId === id)) historicalTransmissions.delete(id);
      for (const [id, event] of historicalConsents) if (event.expiresAt <= timestamp && ![...historicalQuestions.values()].some((artifact) => artifact.consentId === id)) historicalConsents.delete(id);
    },
  };
}
