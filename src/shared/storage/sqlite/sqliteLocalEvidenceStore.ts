import { invoke } from "@tauri-apps/api/core";
import Database from "@tauri-apps/plugin-sql";
import type { ArtifactProvenance, ContextRecoveryTurn, EvidenceCandidate, ExperienceEntry, ISODateTime, PatternNote, ReflectionPrompt } from "../../../types/domain";
import { validateArtifactBundle } from "../artifactValidation";
import type { CreateExperienceInput, LocalEvidenceStore, PersistedArtifactBundle, UpdateExperiencePatch } from "../types";
import type { HistoricalConsentEvent, HistoricalQuestionArtifact, HistoricalTransmissionEvent } from "../../../historicalContext/governedPacket";
const DATABASE_PATH = "sqlite:life-os.db";
type ArtifactKind = "evidence" | "reflection" | "pattern" | "recovery_turn";
interface ExperienceEntryRow { id: string; content: string; created_at: string; updated_at: string; }
interface ArtifactRow { payload: string; }
interface HistoricalArtifactRow { payload: string; packet_snapshot: string; }
interface ExperienceRecordInput { id: string; content: string; createdAt: string; updatedAt: string; }
interface ExperienceMutationResult { status: "committed" | "stale_generation" | "not_found"; }
const now = (): ISODateTime => new Date().toISOString();
const nextRevisionAt = (current: ISODateTime): ISODateTime => {
  const currentTime = Date.parse(current);
  const nextTime = Number.isNaN(currentTime) ? Date.now() : Math.max(Date.now(), currentTime + 1);
  return new Date(nextTime).toISOString();
};
const createId = () => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
const mapExperience = (row: ExperienceEntryRow): ExperienceEntry => ({ id: row.id, body: row.content, createdAt: row.created_at, updatedAt: row.updated_at, userEditable: true });
const toExperienceRecord = (entry: ExperienceEntry): ExperienceRecordInput => ({ id: entry.id, content: entry.body, createdAt: entry.createdAt, updatedAt: entry.updatedAt });
const legacyProvenance = (entryId: string): ArtifactProvenance => ({ origin: "legacy_unknown", sourceEntryId: entryId, sourceArtifactIds: [], provider: "legacy_unknown", model: null, harnessVersion: null, promptVersion: null, generatedAt: null });
export async function initializeSqliteDatabaseConnection() { await invoke("initialize_sqlite_database"); return Database.load(DATABASE_PATH); }
function hydrate<T extends Record<string, unknown>>(record: T, entryId: string, kind: ArtifactKind): T {
  if (kind === "recovery_turn") return (record.promptProvenance ? record : { ...record, promptProvenance: record.provenance ?? legacyProvenance(entryId) }) as T;
  if (kind === "reflection") {
    const prompt = record.promptProvenance ? record : { ...record, promptProvenance: record.provenance ?? legacyProvenance(entryId) };
    return (prompt.status === "answered" && !prompt.responseProvenance ? { ...prompt, responseProvenance: legacyProvenance(entryId) } : prompt) as T;
  }
  return (record.provenance ? record : { ...record, provenance: legacyProvenance(entryId) }) as T;
}
export function createSqliteLocalEvidenceStore(dbPromise: Promise<Database> = initializeSqliteDatabaseConnection()): LocalEvidenceStore {
  const getExperience = async (id: string) => { const rows = await (await dbPromise).select<ExperienceEntryRow[]>("SELECT id, content, created_at, updated_at FROM experience_entries WHERE id = $1 LIMIT 1", [id]); return rows[0] ? mapExperience(rows[0]) : null; };
  const listByKind = async <T extends Record<string, unknown>>(entryId: string, kind: ArtifactKind): Promise<T[]> => (await (await dbPromise).select<ArtifactRow[]>("SELECT payload FROM persisted_artifacts WHERE source_entry_id = $1 AND artifact_kind = $2 ORDER BY created_at ASC", [entryId, kind])).map(({ payload }) => hydrate(JSON.parse(payload), entryId, kind) as T);
  return {
    async createExperience(input: CreateExperienceInput) { const createdAt = now(); const entry: ExperienceEntry = { id: createId(), body: input.body, createdAt, updatedAt: createdAt, userEditable: true }; await invoke("create_sqlite_experience", { experience: toExperienceRecord(entry) }); return entry; },
    async importExperiences(entries) { return invoke<{ importedCount: number; skippedCount: number }>("import_sqlite_experiences", { experiences: entries.map((entry) => toExperienceRecord({ ...entry, userEditable: true })) }); },
    async listExperiences() { return (await (await dbPromise).select<ExperienceEntryRow[]>("SELECT id, content, created_at, updated_at FROM experience_entries ORDER BY created_at DESC")).map(mapExperience); },
    getExperience,
    async updateExperience(id: string, patch: UpdateExperiencePatch) { const existing = await getExperience(id); if (!existing) return null; const updated = { ...existing, body: patch.body ?? existing.body, updatedAt: nextRevisionAt(existing.updatedAt) }; const result = await invoke<ExperienceMutationResult>("update_sqlite_experience", { id, content: updated.body, expectedUpdatedAt: existing.updatedAt, updatedAt: updated.updatedAt }); if (result.status === "stale_generation") throw new Error("stale_generation"); if (result.status === "not_found") return null; return updated; },
    async deleteExperience(id: string) { await invoke<ExperienceMutationResult>("delete_sqlite_experience", { id }); },
    async listArtifacts(entryId: string) { const [evidence, reflections, patterns, recoveryTurns] = await Promise.all([listByKind<EvidenceCandidate & Record<string, unknown>>(entryId, "evidence"), listByKind<ReflectionPrompt & Record<string, unknown>>(entryId, "reflection"), listByKind<PatternNote & Record<string, unknown>>(entryId, "pattern"), listByKind<ContextRecoveryTurn & Record<string, unknown>>(entryId, "recovery_turn")]); return { evidence, reflections, patterns, recoveryTurns }; },
    async saveArtifacts(entryId, bundle, options = {}) {
      if (!(await getExperience(entryId))) throw new Error("Cannot save artifacts for a missing experience.");
      const validated = validateArtifactBundle(entryId, bundle);
      const result = await invoke<ExperienceMutationResult>("save_sqlite_artifacts", { entryId, bundle: validated.bundle, expectedExperienceUpdatedAt: options.expectedExperienceUpdatedAt });
      if (result.status === "not_found") throw new Error("Cannot save artifacts for a missing experience.");
      if (result.status === "stale_generation") return { status: "stale_generation" as const, bundle: await this.listArtifacts(entryId), validationIssues: validated.validationIssues };
      return { status: "committed" as const, bundle: validated.bundle, validationIssues: validated.validationIssues };
    },
    async saveHistoricalConsent(event: HistoricalConsentEvent) {
      await invoke("save_sqlite_historical_consent", { event });
    },
    async saveHistoricalTransmission(event: HistoricalTransmissionEvent) {
      await invoke("save_sqlite_historical_transmission", { event });
    },
    async saveHistoricalQuestionArtifact(artifact: HistoricalQuestionArtifact) {
      return invoke<{ status: "committed" | "stale_generation" }>("save_sqlite_historical_question_artifact", { artifact });
    },
    async listHistoricalQuestionArtifacts(currentExperienceId: string) {
      const rows = await (await dbPromise).select<HistoricalArtifactRow[]>("SELECT payload, packet_snapshot FROM historical_question_artifacts WHERE current_experience_id = $1 ORDER BY created_at DESC", [currentExperienceId]);
      return rows.map((row) => ({ ...JSON.parse(row.payload), packet: JSON.parse(row.packet_snapshot) }) as HistoricalQuestionArtifact);
    },
    async deleteHistoricalQuestionArtifact(id: string) { await invoke("delete_sqlite_historical_question_artifact", { id }); },
    async purgeExpiredHistoricalAuditRecords(timestamp: string) {
      // Initialization/migration must finish before the first startup cleanup.
      // Unlike normal artifact mutations, this method is called by the initial
      // timeline refresh and otherwise races the Rust schema migration.
      await dbPromise;
      await invoke("purge_sqlite_expired_historical_audit_records", { timestamp });
    },
  };
}
