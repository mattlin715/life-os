import { invoke } from "@tauri-apps/api/core";

export const isFounderSchemaV5Candidate =
  import.meta.env.VITE_LIFE_OS_FOUNDER_SCHEMA_V5 === "1";

export const isOrdinaryDesktopSchemaV5 =
  !isFounderSchemaV5Candidate
  && import.meta.env.VITE_LIFE_OS_DESKTOP_SCHEMA_V5 !== "0";

export const isDesktopSchemaV5Active =
  isFounderSchemaV5Candidate || isOrdinaryDesktopSchemaV5;

export type FounderSchemaV5StateName = "missing" | "migration_required" | "ready" | "blocked";

export interface FounderSchemaV5State {
  readonly state: FounderSchemaV5StateName;
  readonly reason: string | null;
  readonly detectedSchemaVersion: number | null;
  readonly supportedSchemaVersion: 5;
  readonly initializationRequired: boolean;
  readonly migrationAvailable: boolean;
  readonly backupAvailable: boolean;
  readonly restoreAvailable: boolean;
  readonly backupRelativePath: string | null;
  readonly backupRetentionDays: 30;
  readonly backupCreatedAt: string | null;
  readonly backupExpiresAt: string | null;
}

type InvokeCommand = (command: string) => Promise<unknown>;
const keys = new Set(["state", "reason", "detectedSchemaVersion", "supportedSchemaVersion", "initializationRequired", "migrationAvailable", "backupAvailable", "restoreAvailable", "backupRelativePath", "backupRetentionDays", "backupCreatedAt", "backupExpiresAt"]);

export function validateFounderSchemaV5State(value: unknown): FounderSchemaV5State {
  if (!value || typeof value !== "object") throw new Error("founder_schema_v5_result_invalid");
  const state = value as Record<string, unknown>;
  if (Object.keys(state).some((key) => !keys.has(key))
    || !["missing", "migration_required", "ready", "blocked"].includes(String(state.state))
    || !(state.reason === null || typeof state.reason === "string")
    || !(state.detectedSchemaVersion === null || Number.isSafeInteger(state.detectedSchemaVersion))
    || state.supportedSchemaVersion !== 5
    || typeof state.initializationRequired !== "boolean"
    || typeof state.migrationAvailable !== "boolean"
    || typeof state.backupAvailable !== "boolean"
    || typeof state.restoreAvailable !== "boolean"
    || (state.restoreAvailable && (state.state !== "blocked" || !state.backupAvailable || state.detectedSchemaVersion !== 5))
    || !(state.backupRelativePath === null || typeof state.backupRelativePath === "string")
    || state.backupRetentionDays !== 30
    || !(state.backupCreatedAt === null || typeof state.backupCreatedAt === "string")
    || !(state.backupExpiresAt === null || typeof state.backupExpiresAt === "string")) throw new Error("founder_schema_v5_result_invalid");
  return state as unknown as FounderSchemaV5State;
}

async function command(
  name: string,
  invokeCommand: InvokeCommand = (commandName) => invoke<unknown>(commandName),
): Promise<FounderSchemaV5State> {
  return validateFounderSchemaV5State(await invokeCommand(name));
}

export const inspectFounderSchemaV5Startup = (invokeCommand?: InvokeCommand) => command("inspect_founder_schema_v5_startup", invokeCommand);
export const initializeFounderSchemaV5Database = (invokeCommand?: InvokeCommand) => command("initialize_founder_schema_v5_database", invokeCommand);
export const authorizeFounderSchemaV5Migration = (invokeCommand?: InvokeCommand) => command("authorize_founder_schema_v5_migration", invokeCommand);
export const inspectFounderSchemaV5Backup = (invokeCommand?: InvokeCommand) => command("inspect_founder_schema_v5_backup", invokeCommand);
export const deleteFounderSchemaV5Backup = (invokeCommand?: InvokeCommand) => command("delete_founder_schema_v5_backup", invokeCommand);
export const restoreFounderSchemaV4Backup = (invokeCommand?: InvokeCommand) => command("restore_founder_schema_v4_backup", invokeCommand);
