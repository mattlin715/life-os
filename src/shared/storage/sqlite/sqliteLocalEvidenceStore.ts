import Database from "@tauri-apps/plugin-sql";

import type { ExperienceEntry, ISODateTime } from "../../../types/domain";
import type {
  CreateExperienceInput,
  LocalEvidenceStore,
  UpdateExperiencePatch,
} from "../types";

const DATABASE_PATH = "sqlite:life-os.db";

interface ExperienceEntryRow {
  id: string;
  content: string;
  created_at: string;
  updated_at: string;
}

function now(): ISODateTime {
  return new Date().toISOString();
}

function createId(): string {
  return globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
}

function mapExperienceEntry(row: ExperienceEntryRow): ExperienceEntry {
  return {
    id: row.id,
    body: row.content,
    createdAt: row.created_at,
    updatedAt: row.updated_at,
    userEditable: true,
  };
}

async function initializeDatabase(): Promise<Database> {
  const db = await Database.load(DATABASE_PATH);

  await db.execute(`
    CREATE TABLE IF NOT EXISTS experience_entries (
      id TEXT PRIMARY KEY NOT NULL,
      content TEXT NOT NULL,
      created_at TEXT NOT NULL,
      updated_at TEXT NOT NULL
    )
  `);

  return db;
}

export function createSqliteLocalEvidenceStore(): LocalEvidenceStore {
  const db = initializeDatabase();

  async function getExperience(id: string): Promise<ExperienceEntry | null> {
    const rows = await (
      await db
    ).select<ExperienceEntryRow[]>(
      `SELECT id, content, created_at, updated_at
       FROM experience_entries
       WHERE id = $1
       LIMIT 1`,
      [id],
    );

    return rows[0] ? mapExperienceEntry(rows[0]) : null;
  }

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

      await (
        await db
      ).execute(
        `INSERT INTO experience_entries (id, content, created_at, updated_at)
         VALUES ($1, $2, $3, $4)`,
        [entry.id, entry.body, entry.createdAt, entry.updatedAt],
      );

      return entry;
    },

    async importExperiences(importedEntries) {
      let importedCount = 0;
      let skippedCount = 0;
      const database = await db;

      for (const importedEntry of importedEntries) {
        const existing = await getExperience(importedEntry.id);

        if (existing) {
          skippedCount += 1;
          continue;
        }

        await database.execute(
          `INSERT INTO experience_entries (id, content, created_at, updated_at)
           VALUES ($1, $2, $3, $4)`,
          [
            importedEntry.id,
            importedEntry.body,
            importedEntry.createdAt,
            importedEntry.updatedAt,
          ],
        );
        importedCount += 1;
      }

      return { importedCount, skippedCount };
    },

    async listExperiences() {
      const rows = await (
        await db
      ).select<ExperienceEntryRow[]>(
        `SELECT id, content, created_at, updated_at
         FROM experience_entries
         ORDER BY created_at DESC`,
      );

      return rows.map(mapExperienceEntry);
    },

    getExperience,

    async updateExperience(id: string, patch: UpdateExperiencePatch) {
      const existing = await getExperience(id);

      if (!existing) {
        return null;
      }

      const updated: ExperienceEntry = {
        ...existing,
        body: patch.body ?? existing.body,
        updatedAt: now(),
      };

      await (
        await db
      ).execute(
        `UPDATE experience_entries
         SET content = $1, updated_at = $2
         WHERE id = $3`,
        [updated.body, updated.updatedAt, id],
      );

      return updated;
    },

    async deleteExperience(id: string) {
      await (await db).execute("DELETE FROM experience_entries WHERE id = $1", [
        id,
      ]);
    },
  };
}
