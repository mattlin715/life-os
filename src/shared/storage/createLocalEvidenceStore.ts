import { invoke, isTauri } from "@tauri-apps/api/core";

import { createInMemoryLocalEvidenceStore } from "./inMemoryLocalEvidenceStore";
import {
  createSqliteLocalEvidenceStore,
  initializeSqliteDatabaseConnection,
} from "./sqlite/sqliteLocalEvidenceStore";
import {
  initializeFounderSchemaV5Database,
  inspectFounderSchemaV5Startup,
  isDesktopSchemaV5Active,
  isOrdinaryDesktopSchemaV5,
  type FounderSchemaV5State,
} from "./sqlite/founderSchemaV5";
import { createFounderSchemaV5LocalEvidenceStore } from "./sqlite/founderSchemaV5LocalEvidenceStore";
import type { LocalEvidenceStore } from "./types";

const SUPPORTED_SCHEMA_VERSION = isDesktopSchemaV5Active ? 5 : 4;

export type DatabaseStartupState =
  | {
      state: "ready";
      reason: null;
      detectedSchemaVersion: number | null;
      supportedSchemaVersion: number;
      initializationRequired: boolean;
      storage: "sqlite" | "memory";
      founderSchemaV5?: FounderSchemaV5State;
    }
  | {
      state: "blocked";
      reason: "newer_schema" | "inspection_failed" | "initialization_failed" | "migration_required" | "founder_v5_blocked";
      detectedSchemaVersion: number | null;
      supportedSchemaVersion: number;
      initializationRequired: false;
      storage: "sqlite";
      founderSchemaV5?: FounderSchemaV5State;
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
    if (isDesktopSchemaV5Active) {
      try {
        let founder = await inspectFounderSchemaV5Startup();
        if (
          isOrdinaryDesktopSchemaV5
          && founder.state === "blocked"
          && founder.reason === "older_schema_unsupported"
          && founder.detectedSchemaVersion !== null
          && founder.detectedSchemaVersion >= 0
          && founder.detectedSchemaVersion <= 3
        ) {
          await invoke("initialize_sqlite_database");
          founder = await inspectFounderSchemaV5Startup();
        }
        if (founder.state === "missing") founder = await initializeFounderSchemaV5Database();
        if (founder.state === "ready") {
          return {
            state: { state: "ready", reason: null, detectedSchemaVersion: 5, supportedSchemaVersion: 5, initializationRequired: false, storage: "sqlite", founderSchemaV5: founder },
            store: createFounderSchemaV5LocalEvidenceStore(),
          };
        }
        return {
          state: {
            state: "blocked",
            reason: founder.state === "migration_required" ? "migration_required" : "founder_v5_blocked",
            detectedSchemaVersion: founder.detectedSchemaVersion,
            supportedSchemaVersion: 5,
            initializationRequired: false,
            storage: "sqlite",
            founderSchemaV5: founder,
          },
          store: null,
        };
      } catch {
        return { state: { ...blockedState("inspection_failed"), supportedSchemaVersion: 5 }, store: null };
      }
    }
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
  // Startup can intentionally remain blocked while the UI renders migration
  // or recovery disclosure before any store method is called. Attach a
  // rejection observer immediately so that bounded fail-closed startup is not
  // reported as an unhandled promise; deferred methods still receive the same
  // rejection when invoked.
  void storePromise.catch(() => undefined);

  return {
    store: createDeferredStore(storePromise),
    startup: startupAndStore.then(({ state }) => state),
  };
}

export function createLocalEvidenceStore(): LocalEvidenceStore {
  return createLocalEvidenceStoreRuntime().store;
}
