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
interface SqlStatement { query: string; values: unknown[]; }
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
async function executeTransaction(statements: SqlStatement[], expectedExperienceUpdatedAt?: string) { return invoke<{ status: "committed" | "stale_generation" }>("execute_sqlite_transaction", { statements, expectedExperienceUpdatedAt }); }
async function executeHistoricalTransaction(statements: SqlStatement[], expectedRevisions: Array<{ id: string; updatedAt: string }>, expectedArtifacts: Array<{ id: string; sourceEntryId: string; updatedAt: string; artifactKind: "evidence" | "reflection" }>, expectedProvenance: { consentId: string; transmissionId: string; packetDigest: string; provider: string; model: string }) { return invoke<{ status: "committed" | "stale_generation" }>("execute_sqlite_historical_transaction", { statements, expectedRevisions, expectedArtifacts, expectedProvenance }); }
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
      const statements: SqlStatement[] = [{ query: "DELETE FROM historical_question_artifacts WHERE id IN (SELECT historical_artifact_id FROM historical_artifact_dependencies WHERE source_entry_id = $1)", values: [entryId] }, { query: "DELETE FROM persisted_artifacts WHERE source_entry_id = $1", values: [entryId] }];
      for (const [kind, records] of [["evidence", validated.bundle.evidence], ["reflection", validated.bundle.reflections], ["pattern", validated.bundle.patterns], ["recovery_turn", validated.bundle.recoveryTurns]] as const) for (const record of records) statements.push({ query: "INSERT INTO persisted_artifacts (id, source_entry_id, artifact_kind, payload, created_at, updated_at) VALUES ($1,$2,$3,$4,$5,$6)", values: [record.id, entryId, kind, JSON.stringify(record), record.createdAt, record.updatedAt] });
      const result = await executeTransaction(statements, options.expectedExperienceUpdatedAt);
      if (result.status === "stale_generation") return { status: "stale_generation" as const, bundle: await this.listArtifacts(entryId), validationIssues: validated.validationIssues };
      return { status: "committed" as const, bundle: validated.bundle, validationIssues: validated.validationIssues };
    },
    async saveHistoricalConsent(event: HistoricalConsentEvent) {
      await (await dbPromise).execute("INSERT INTO historical_consent_events (id, packet_digest, payload, state, created_at, expires_at) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT(id) DO UPDATE SET payload=excluded.payload, state=excluded.state WHERE historical_consent_events.packet_digest=excluded.packet_digest AND historical_consent_events.created_at=excluded.created_at AND historical_consent_events.expires_at=excluded.expires_at", [event.id, event.packetDigest, JSON.stringify(event), event.state, event.createdAt, event.expiresAt]);
    },
    async saveHistoricalTransmission(event: HistoricalTransmissionEvent) {
      await executeTransaction([
        { query: "INSERT INTO historical_transmission_events (id, consent_id, packet_digest, provider, model, outcome, created_at, expires_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT(id) DO UPDATE SET outcome=excluded.outcome, expires_at=excluded.expires_at WHERE historical_transmission_events.consent_id=excluded.consent_id AND historical_transmission_events.packet_digest=excluded.packet_digest AND historical_transmission_events.provider=excluded.provider AND historical_transmission_events.model=excluded.model AND historical_transmission_events.created_at=excluded.created_at", values: [event.id, event.consentId, event.packetDigest, event.provider, event.model, event.outcome, event.createdAt, event.expiresAt] },
        { query: "UPDATE historical_consent_events SET state = CASE WHEN $1 IN ('sent','failed','cancelled_after_send') THEN 'consumed' ELSE state END WHERE id = $2", values: [event.outcome, event.consentId] },
      ]);
    },
    async saveHistoricalQuestionArtifact(artifact: HistoricalQuestionArtifact) {
      const expectedRevisions = [{ id: artifact.packet.currentExperience.id, updatedAt: artifact.packet.currentExperience.revision }, ...artifact.packet.includedItems.filter((item) => item.itemType === "experience").map((item) => ({ id: item.sourceExperienceId, updatedAt: item.revision }))];
      const expectedArtifacts = artifact.packet.includedItems.filter((item) => item.artifactId).map((item) => ({ id: item.artifactId!, sourceEntryId: item.sourceExperienceId, updatedAt: item.revision, artifactKind: item.itemType === "evidence" ? "evidence" as const : "reflection" as const }));
      const statements: SqlStatement[] = [{ query: "INSERT INTO historical_question_artifacts (id, current_experience_id, packet_digest, payload, packet_snapshot, consent_id, transmission_id, created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)", values: [artifact.id, artifact.currentExperienceId, artifact.packet.packetDigest, JSON.stringify({ ...artifact, packet: undefined }), JSON.stringify(artifact.packet), artifact.consentId, artifact.transmissionId, artifact.generatedAt] }];
      for (const item of artifact.packet.includedItems) statements.push({ query: "INSERT INTO historical_artifact_dependencies (historical_artifact_id, source_entry_id, source_artifact_id, source_revision) VALUES ($1,$2,$3,$4)", values: [artifact.id, item.sourceExperienceId, item.artifactId, item.revision] });
      return executeHistoricalTransaction(statements, expectedRevisions, expectedArtifacts, { consentId: artifact.consentId, transmissionId: artifact.transmissionId, packetDigest: artifact.packet.packetDigest, provider: artifact.packet.destination.provider, model: artifact.packet.destination.model });
    },
    async listHistoricalQuestionArtifacts(currentExperienceId: string) {
      const rows = await (await dbPromise).select<HistoricalArtifactRow[]>("SELECT payload, packet_snapshot FROM historical_question_artifacts WHERE current_experience_id = $1 ORDER BY created_at DESC", [currentExperienceId]);
      return rows.map((row) => ({ ...JSON.parse(row.payload), packet: JSON.parse(row.packet_snapshot) }) as HistoricalQuestionArtifact);
    },
    async deleteHistoricalQuestionArtifact(id: string) { await (await dbPromise).execute("DELETE FROM historical_question_artifacts WHERE id = $1", [id]); },
    async purgeExpiredHistoricalAuditRecords(timestamp: string) {
      // Initialization/migration must finish before the first startup cleanup.
      // Unlike normal artifact mutations, this method is called by the initial
      // timeline refresh and otherwise races the Rust schema migration.
      await dbPromise;
      await executeTransaction([
        { query: "DELETE FROM historical_transmission_events WHERE expires_at <= $1 AND id NOT IN (SELECT transmission_id FROM historical_question_artifacts)", values: [timestamp] },
        { query: "DELETE FROM historical_consent_events WHERE expires_at <= $1 AND id NOT IN (SELECT consent_id FROM historical_question_artifacts)", values: [timestamp] },
      ]);
    },
  };
}
