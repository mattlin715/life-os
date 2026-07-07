import { isTauri } from "@tauri-apps/api/core";

import { createInMemoryLocalEvidenceStore } from "./inMemoryLocalEvidenceStore";
import { createSqliteLocalEvidenceStore } from "./sqlite/sqliteLocalEvidenceStore";
import type { LocalEvidenceStore } from "./types";

export function createLocalEvidenceStore(): LocalEvidenceStore {
  if (isTauri()) {
    return createSqliteLocalEvidenceStore();
  }

  return createInMemoryLocalEvidenceStore();
}
