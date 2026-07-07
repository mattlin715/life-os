import type { LocalEvidenceStore } from "../types";

// TODO: Implement this boundary with the Tauri SQL plugin after the local
// evidence store schema is intentionally defined. Do not add migrations here.
export function createSqliteLocalEvidenceStore(): LocalEvidenceStore {
  throw new Error("SQLite local evidence store is not implemented yet.");
}
