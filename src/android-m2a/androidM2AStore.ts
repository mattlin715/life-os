import { invoke } from "@tauri-apps/api/core";
import type { ExperienceEntry } from "../types/domain";
import type { CreateExperienceInput, LocalEvidenceStore } from "../shared/storage/types";

export type AndroidM2ASupportedOperation = keyof Pick<
  LocalEvidenceStore,
  "createExperience" | "listExperiences" | "getExperience"
>;

export const androidM2ASupportedOperations: readonly AndroidM2ASupportedOperation[] = [
  "createExperience",
  "listExperiences",
  "getExperience",
];

export type AndroidM2AStorageStatus = {
  state: "ready";
  reason: string | null;
  schemaVersion: 5;
  applicationId: "com.lifeos.review.m2a";
  databaseFilename: string;
  receiptFilename: string;
  initializationOrigin: "directFreshV5";
  syntheticOnly: true;
  supportedOperations: string[];
  unsupportedOperations: string[];
};

export type AndroidM2ADebugPhase = "beforeCommit" | "afterCommitBeforeAck";
export type AndroidM2ALocalePreferenceValue = "en" | "zh-TW" | "ja";

export interface AndroidM2ALocalePreference {
  read(): Promise<AndroidM2ALocalePreferenceValue | null>;
  write(locale: AndroidM2ALocalePreferenceValue): Promise<void>;
}

export type AndroidM2ASaveReceipt = {
  acknowledgement: "committed" | "alreadyCommitted";
  entry: ExperienceEntry;
};

export interface AndroidM2AExperienceStore {
  status(): Promise<AndroidM2AStorageStatus>;
  createExperience(
    input: CreateExperienceInput,
    requestId: string,
    debugPhase?: AndroidM2ADebugPhase,
  ): Promise<AndroidM2ASaveReceipt>;
  listExperiences(): Promise<ExperienceEntry[]>;
  getExperience(id: string): Promise<ExperienceEntry | null>;
}

function editable(entry: Omit<ExperienceEntry, "userEditable"> & { userEditable?: boolean }): ExperienceEntry {
  return { ...entry, userEditable: true };
}

export const androidM2AExperienceStore: AndroidM2AExperienceStore = {
  status: () => invoke<AndroidM2AStorageStatus>("m2a_storage_status"),
  async createExperience(input, requestId, debugPhase) {
    const receipt = await invoke<Omit<AndroidM2ASaveReceipt, "entry"> & { entry: Omit<ExperienceEntry, "userEditable"> }>(
      "m2a_create_experience",
      {
        requestId,
        body: input.body,
        debugPhase,
        debugHoldMs: debugPhase ? 30_000 : undefined,
      },
    );
    return { ...receipt, entry: editable(receipt.entry) };
  },
  async listExperiences() {
    return (await invoke<Array<Omit<ExperienceEntry, "userEditable">>>("m2a_list_experiences")).map(editable);
  },
  async getExperience(id) {
    const entry = await invoke<Omit<ExperienceEntry, "userEditable"> | null>("m2a_get_experience", { id });
    return entry ? editable(entry) : null;
  },
};

export const androidM2ALocalePreference: AndroidM2ALocalePreference = {
  read: () => invoke<AndroidM2ALocalePreferenceValue | null>("m2a_get_locale_preference"),
  write: (locale) => invoke<void>("m2a_set_locale_preference", { locale }),
};
