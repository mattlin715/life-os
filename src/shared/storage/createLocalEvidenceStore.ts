import { invoke, isTauri } from "@tauri-apps/api/core";

import { createInMemoryLocalEvidenceStore } from "./inMemoryLocalEvidenceStore";
import {
  createSqliteLocalEvidenceStore,
  initializeSqliteDatabaseConnection,
} from "./sqlite/sqliteLocalEvidenceStore";
import type { LocalEvidenceStore } from "./types";

const SUPPORTED_SCHEMA_VERSION = 4;

export type DatabaseStartupState =
  | {
      state: "ready";
      reason: null;
      detectedSchemaVersion: number | null;
      supportedSchemaVersion: number;
      initializationRequired: boolean;
      storage: "sqlite" | "memory";
    }
  | {
      state: "blocked";
      reason: "newer_schema" | "inspection_failed" | "initialization_failed";
      detectedSchemaVersion: number | null;
      supportedSchemaVersion: number;
      initializationRequired: false;
      storage: "sqlite";
    };

interface RustDatabaseStartupState {
  state: "ready" | "blocked";
  reason: "newer_schema" | null;
  detectedSchemaVersion: number | null;
  supportedSchemaVersion: number;
  initializationRequired: boolean;
}

export interface LocalEvidenceStoreRuntime {
  store: LocalEvidenceStore;
  startup: Promise<DatabaseStartupState>;
}

function createDeferredStore(storePromise: Promise<LocalEvidenceStore>): LocalEvidenceStore {
  return {
    async createExperience(input) { return (await storePromise).createExperience(input); },
    async importExperiences(entries) { return (await storePromise).importExperiences(entries); },
    async listExperiences() { return (await storePromise).listExperiences(); },
    async getExperience(id) { return (await storePromise).getExperience(id); },
    async updateExperience(id, patch) { return (await storePromise).updateExperience(id, patch); },
    async deleteExperience(id) { return (await storePromise).deleteExperience(id); },
    async listArtifacts(entryId) { return (await storePromise).listArtifacts(entryId); },
    async saveArtifacts(entryId, bundle, options) { return (await storePromise).saveArtifacts(entryId, bundle, options); },
    async saveHistoricalConsent(event) { return (await storePromise).saveHistoricalConsent(event); },
    async saveHistoricalTransmission(event) { return (await storePromise).saveHistoricalTransmission(event); },
    async saveHistoricalQuestionArtifact(artifact) { return (await storePromise).saveHistoricalQuestionArtifact(artifact); },
    async listHistoricalQuestionArtifacts(currentExperienceId) { return (await storePromise).listHistoricalQuestionArtifacts(currentExperienceId); },
    async deleteHistoricalQuestionArtifact(id) { return (await storePromise).deleteHistoricalQuestionArtifact(id); },
    async purgeExpiredHistoricalAuditRecords(timestamp) { return (await storePromise).purgeExpiredHistoricalAuditRecords(timestamp); },
  };
}

function blockedState(
  reason: "inspection_failed" | "initialization_failed",
): DatabaseStartupState {
  return {
    state: "blocked",
    reason,
    detectedSchemaVersion: null,
    supportedSchemaVersion: SUPPORTED_SCHEMA_VERSION,
    initializationRequired: false,
    storage: "sqlite",
  };
}

export function createLocalEvidenceStoreRuntime(): LocalEvidenceStoreRuntime {
  if (!isTauri()) {
    const store = createInMemoryLocalEvidenceStore();
    return {
      store,
      startup: Promise.resolve({
        state: "ready",
        reason: null,
        detectedSchemaVersion: null,
        supportedSchemaVersion: SUPPORTED_SCHEMA_VERSION,
        initializationRequired: false,
        storage: "memory",
      }),
    };
  }

  const startupAndStore = (async (): Promise<{
    state: DatabaseStartupState;
    store: LocalEvidenceStore | null;
  }> => {
    let inspected: RustDatabaseStartupState;
    try {
      inspected = await invoke<RustDatabaseStartupState>("inspect_sqlite_database");
    } catch {
      return { state: blockedState("inspection_failed"), store: null };
    }

    if (inspected.state === "blocked") {
      return {
        state: {
          ...inspected,
          state: "blocked",
          reason: "newer_schema",
          initializationRequired: false,
          storage: "sqlite",
        },
        store: null,
      };
    }

    try {
      const database = await initializeSqliteDatabaseConnection();
      return {
        state: {
          ...inspected,
          state: "ready",
          reason: null,
          initializationRequired: false,
          storage: "sqlite",
        },
        store: createSqliteLocalEvidenceStore(Promise.resolve(database)),
      };
    } catch {
      return { state: blockedState("initialization_failed"), store: null };
    }
  })();

  const storePromise = startupAndStore.then(({ store }) => {
    if (!store) throw new Error("local_database_startup_blocked");
    return store;
  });

  return {
    store: createDeferredStore(storePromise),
    startup: startupAndStore.then(({ state }) => state),
  };
}

export function createLocalEvidenceStore(): LocalEvidenceStore {
  return createLocalEvidenceStoreRuntime().store;
}
