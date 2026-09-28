import { invoke } from "@tauri-apps/api/core";
import type { ExperienceEntry } from "../types/domain";
import type { CreateExperienceInput, LocalEvidenceStore } from "../shared/storage/types";

export type AndroidM1SupportedOperation = keyof Pick<
  LocalEvidenceStore,
  "createExperience" | "listExperiences" | "getExperience"
>;

export const androidM1SupportedOperations: readonly AndroidM1SupportedOperation[] = [
  "createExperience",
  "listExperiences",
  "getExperience",
];

export type AndroidM1StorageStatus = {
  state: "ready";
  reason: string | null;
  schemaVersion: 5;
  applicationId: "com.lifeos.review.m1";
  databaseFilename: string;
  syntheticOnly: true;
  supportedOperations: string[];
  unsupportedOperations: string[];
};

export type AndroidM1DebugPhase = "beforeCommit" | "afterCommitBeforeAck";
export type AndroidM1LocalePreferenceValue = "en" | "zh-TW" | "ja";

export interface AndroidM1LocalePreference {
  read(): Promise<AndroidM1LocalePreferenceValue | null>;
  write(locale: AndroidM1LocalePreferenceValue): Promise<void>;
}

export type AndroidM1SaveReceipt = {
  acknowledgement: "committed" | "alreadyCommitted";
  entry: ExperienceEntry;
};

export interface AndroidM1ExperienceStore {
  status(): Promise<AndroidM1StorageStatus>;
  createExperience(
    input: CreateExperienceInput,
    requestId: string,
    debugPhase?: AndroidM1DebugPhase,
  ): Promise<AndroidM1SaveReceipt>;
  listExperiences(): Promise<ExperienceEntry[]>;
  getExperience(id: string): Promise<ExperienceEntry | null>;
}

function editable(entry: Omit<ExperienceEntry, "userEditable"> & { userEditable?: boolean }): ExperienceEntry {
  return { ...entry, userEditable: true };
}

export const androidM1ExperienceStore: AndroidM1ExperienceStore = {
  status: () => invoke<AndroidM1StorageStatus>("m1_storage_status"),
  async createExperience(input, requestId, debugPhase) {
    const receipt = await invoke<Omit<AndroidM1SaveReceipt, "entry"> & { entry: Omit<ExperienceEntry, "userEditable"> }>(
      "m1_create_experience",
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
    return (await invoke<Array<Omit<ExperienceEntry, "userEditable">>>("m1_list_experiences")).map(editable);
  },
  async getExperience(id) {
    const entry = await invoke<Omit<ExperienceEntry, "userEditable"> | null>("m1_get_experience", { id });
    return entry ? editable(entry) : null;
  },
};

export const androidM1LocalePreference: AndroidM1LocalePreference = {
  read: () => invoke<AndroidM1LocalePreferenceValue | null>("m1_get_locale_preference"),
  write: (locale) => invoke<void>("m1_set_locale_preference", { locale }),
};
