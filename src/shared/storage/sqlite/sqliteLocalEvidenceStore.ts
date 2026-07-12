import { invoke } from "@tauri-apps/api/core";
import Database from "@tauri-apps/plugin-sql";
import type { ArtifactProvenance, ContextRecoveryTurn, EvidenceCandidate, ExperienceEntry, ISODateTime, PatternNote, ReflectionPrompt } from "../../../types/domain";
import { validateArtifactBundle } from "../artifactValidation";
import type { CreateExperienceInput, LocalEvidenceStore, PersistedArtifactBundle, UpdateExperiencePatch } from "../types";
const DATABASE_PATH = "sqlite:life-os.db";
type ArtifactKind = "evidence" | "reflection" | "pattern" | "recovery_turn";
interface ExperienceEntryRow { id: string; content: string; created_at: string; updated_at: string; }
interface ArtifactRow { payload: string; }
interface SqlStatement { query: string; values: unknown[]; }
const now = (): ISODateTime => new Date().toISOString();
const createId = () => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
const mapExperience = (row: ExperienceEntryRow): ExperienceEntry => ({ id: row.id, body: row.content, createdAt: row.created_at, updatedAt: row.updated_at, userEditable: true });
const legacyProvenance = (entryId: string): ArtifactProvenance => ({ origin: "legacy_unknown", sourceEntryId: entryId, sourceArtifactIds: [], provider: "legacy_unknown", model: null, harnessVersion: null, promptVersion: null, generatedAt: null });
async function executeTransaction(statements: SqlStatement[], expectedExperienceUpdatedAt?: string) { return invoke<{ status: "committed" | "stale_generation" }>("execute_sqlite_transaction", { statements, expectedExperienceUpdatedAt }); }
async function initializeDatabase() { await invoke("initialize_sqlite_database"); return Database.load(DATABASE_PATH); }
function hydrate<T extends Record<string, unknown>>(record: T, entryId: string, kind: ArtifactKind): T {
  if (kind === "recovery_turn") return (record.promptProvenance ? record : { ...record, promptProvenance: record.provenance ?? legacyProvenance(entryId) }) as T;
  if (kind === "reflection") {
    const prompt = record.promptProvenance ? record : { ...record, promptProvenance: record.provenance ?? legacyProvenance(entryId) };
    return (prompt.status === "answered" && !prompt.responseProvenance ? { ...prompt, responseProvenance: legacyProvenance(entryId) } : prompt) as T;
  }
  return (record.provenance ? record : { ...record, provenance: legacyProvenance(entryId) }) as T;
}
export function createSqliteLocalEvidenceStore(): LocalEvidenceStore {
  const dbPromise = initializeDatabase();
  const getExperience = async (id: string) => { const rows = await (await dbPromise).select<ExperienceEntryRow[]>("SELECT id, content, created_at, updated_at FROM experience_entries WHERE id = $1 LIMIT 1", [id]); return rows[0] ? mapExperience(rows[0]) : null; };
  const listByKind = async <T extends Record<string, unknown>>(entryId: string, kind: ArtifactKind): Promise<T[]> => (await (await dbPromise).select<ArtifactRow[]>("SELECT payload FROM persisted_artifacts WHERE source_entry_id = $1 AND artifact_kind = $2 ORDER BY created_at ASC", [entryId, kind])).map(({ payload }) => hydrate(JSON.parse(payload), entryId, kind) as T);
  return {
    async createExperience(input: CreateExperienceInput) { const createdAt = now(); const entry: ExperienceEntry = { id: createId(), body: input.body, createdAt, updatedAt: createdAt, userEditable: true }; await (await dbPromise).execute("INSERT INTO experience_entries (id, content, created_at, updated_at) VALUES ($1,$2,$3,$4)", [entry.id, entry.body, entry.createdAt, entry.updatedAt]); return entry; },
    async importExperiences(entries) { let importedCount = 0, skippedCount = 0; const db = await dbPromise; for (const entry of entries) { if (await getExperience(entry.id)) { skippedCount++; continue; } await db.execute("INSERT INTO experience_entries (id, content, created_at, updated_at) VALUES ($1,$2,$3,$4)", [entry.id, entry.body, entry.createdAt, entry.updatedAt]); importedCount++; } return { importedCount, skippedCount }; },
    async listExperiences() { return (await (await dbPromise).select<ExperienceEntryRow[]>("SELECT id, content, created_at, updated_at FROM experience_entries ORDER BY created_at DESC")).map(mapExperience); },
    getExperience,
    async updateExperience(id: string, patch: UpdateExperiencePatch) { const existing = await getExperience(id); if (!existing) return null; const updated = { ...existing, body: patch.body ?? existing.body, updatedAt: now() }; await executeTransaction([{ query: "UPDATE experience_entries SET content = $1, updated_at = $2 WHERE id = $3", values: [updated.body, updated.updatedAt, id] }, { query: "DELETE FROM persisted_artifacts WHERE source_entry_id = $1", values: [id] }]); return updated; },
    async deleteExperience(id: string) { await executeTransaction([{ query: "DELETE FROM persisted_artifacts WHERE source_entry_id = $1", values: [id] }, { query: "DELETE FROM experience_entries WHERE id = $1", values: [id] }]); },
    async listArtifacts(entryId: string) { const [evidence, reflections, patterns, recoveryTurns] = await Promise.all([listByKind<EvidenceCandidate & Record<string, unknown>>(entryId, "evidence"), listByKind<ReflectionPrompt & Record<string, unknown>>(entryId, "reflection"), listByKind<PatternNote & Record<string, unknown>>(entryId, "pattern"), listByKind<ContextRecoveryTurn & Record<string, unknown>>(entryId, "recovery_turn")]); return { evidence, reflections, patterns, recoveryTurns }; },
    async saveArtifacts(entryId, bundle, options = {}) {
      if (!(await getExperience(entryId))) throw new Error("Cannot save artifacts for a missing experience.");
      const validated = validateArtifactBundle(entryId, bundle);
      const statements: SqlStatement[] = [{ query: "DELETE FROM persisted_artifacts WHERE source_entry_id = $1", values: [entryId] }];
      for (const [kind, records] of [["evidence", validated.bundle.evidence], ["reflection", validated.bundle.reflections], ["pattern", validated.bundle.patterns], ["recovery_turn", validated.bundle.recoveryTurns]] as const) for (const record of records) statements.push({ query: "INSERT INTO persisted_artifacts (id, source_entry_id, artifact_kind, payload, created_at, updated_at) VALUES ($1,$2,$3,$4,$5,$6)", values: [record.id, entryId, kind, JSON.stringify(record), record.createdAt, record.updatedAt] });
      const result = await executeTransaction(statements, options.expectedExperienceUpdatedAt);
      if (result.status === "stale_generation") return { status: "stale_generation" as const, bundle: await this.listArtifacts(entryId), validationIssues: validated.validationIssues };
      return { status: "committed" as const, bundle: validated.bundle, validationIssues: validated.validationIssues };
    },
  };
}
