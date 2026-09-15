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
  readonly preparedRecoveryReceiptAvailable: boolean;
  readonly preparedRecoveryReceiptRelativePath: string | null;
  readonly preparedRecoveryReceiptOperationId: string | null;
  readonly preparedRecoveryReceiptRecoveredAt: string | null;
}

type InvokeCommand = (command: string, args?: Record<string, unknown>) => Promise<unknown>;
const keys = new Set(["state", "reason", "detectedSchemaVersion", "supportedSchemaVersion", "initializationRequired", "migrationAvailable", "backupAvailable", "restoreAvailable", "backupRelativePath", "backupRetentionDays", "backupCreatedAt", "backupExpiresAt", "preparedRecoveryReceiptAvailable", "preparedRecoveryReceiptRelativePath", "preparedRecoveryReceiptOperationId", "preparedRecoveryReceiptRecoveredAt"]);

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
    || !(state.backupExpiresAt === null || typeof state.backupExpiresAt === "string")
    || typeof state.preparedRecoveryReceiptAvailable !== "boolean"
    || !(state.preparedRecoveryReceiptRelativePath === null || typeof state.preparedRecoveryReceiptRelativePath === "string")
    || !(state.preparedRecoveryReceiptOperationId === null || typeof state.preparedRecoveryReceiptOperationId === "string")
    || !(state.preparedRecoveryReceiptRecoveredAt === null || typeof state.preparedRecoveryReceiptRecoveredAt === "string")
    || (state.preparedRecoveryReceiptAvailable !== (typeof state.preparedRecoveryReceiptRelativePath === "string"))
    || (state.preparedRecoveryReceiptAvailable !== (typeof state.preparedRecoveryReceiptOperationId === "string"))
    || (state.preparedRecoveryReceiptAvailable !== (typeof state.preparedRecoveryReceiptRecoveredAt === "string"))) throw new Error("founder_schema_v5_result_invalid");
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

export interface PreparedRecoveryFileFact {
  readonly relativePath: string;
  readonly size: number;
  readonly sha256: string;
}

export interface PreparedRecoveryInspection {
  readonly eligible: boolean;
  readonly reason: string | null;
  readonly classification: string | null;
  readonly operationId: string | null;
  readonly database: PreparedRecoveryFileFact | null;
  readonly databaseIdentity: string | null;
  readonly evidenceFiles: readonly PreparedRecoveryFileFact[];
  readonly preservedFiles: readonly PreparedRecoveryFileFact[];
  readonly claimDigest: string | null;
}

export interface PreparedRecoveryResult {
  readonly classification: string;
  readonly operationId: string;
  readonly receiptRelativePath: string;
  readonly databaseSha256: string;
  readonly restartRequired: true;
}

const digest = (value: unknown): value is string =>
  typeof value === "string" && /^[0-9a-f]{64}$/i.test(value);

function validateFileFact(value: unknown): value is PreparedRecoveryFileFact {
  if (!value || typeof value !== "object") return false;
  const fact = value as Record<string, unknown>;
  return Object.keys(fact).every((key) => ["relativePath", "size", "sha256"].includes(key))
    && typeof fact.relativePath === "string"
    && Number.isSafeInteger(fact.size) && Number(fact.size) >= 0
    && digest(fact.sha256);
}

export function validatePreparedRecoveryInspection(value: unknown): PreparedRecoveryInspection {
  if (!value || typeof value !== "object") throw new Error("prepared_recovery_result_invalid");
  const result = value as Record<string, unknown>;
  const allowed = ["eligible", "reason", "classification", "operationId", "database", "databaseIdentity", "evidenceFiles", "preservedFiles", "claimDigest"];
  if (Object.keys(result).some((key) => !allowed.includes(key))
    || typeof result.eligible !== "boolean"
    || !(result.reason === null || typeof result.reason === "string")
    || !(result.classification === null || typeof result.classification === "string")
    || !(result.operationId === null || typeof result.operationId === "string")
    || !(result.database === null || validateFileFact(result.database))
    || !(result.databaseIdentity === null || typeof result.databaseIdentity === "string")
    || !Array.isArray(result.evidenceFiles) || !result.evidenceFiles.every(validateFileFact)
    || !Array.isArray(result.preservedFiles) || !result.preservedFiles.every(validateFileFact)
    || !(result.claimDigest === null || digest(result.claimDigest))
    || (result.eligible && (!result.classification || !result.operationId || !result.database || !result.databaseIdentity || !result.claimDigest || result.reason !== null))
    || (!result.eligible && result.reason === null)) throw new Error("prepared_recovery_result_invalid");
  return result as unknown as PreparedRecoveryInspection;
}

export async function inspectPreparedStateRecovery(
  invokeCommand: InvokeCommand = (name, args) => invoke<unknown>(name, args),
): Promise<PreparedRecoveryInspection> {
  return validatePreparedRecoveryInspection(await invokeCommand("inspect_prepared_state_recovery"));
}

export async function executePreparedStateRecovery(
  inspection: PreparedRecoveryInspection,
  invokeCommand: InvokeCommand = (name, args) => invoke<unknown>(name, args),
): Promise<PreparedRecoveryResult> {
  if (!inspection.eligible || !inspection.claimDigest || !inspection.database) {
    throw new Error("prepared_recovery_not_authorizable");
  }
  const value = await invokeCommand("execute_prepared_state_recovery", {
    expectedClaimDigest: inspection.claimDigest,
    expectedDatabaseSha256: inspection.database.sha256,
  });
  if (!value || typeof value !== "object") throw new Error("prepared_recovery_result_invalid");
  const result = value as Record<string, unknown>;
  const allowed = ["classification", "operationId", "receiptRelativePath", "databaseSha256", "restartRequired"];
  if (Object.keys(result).some((key) => !allowed.includes(key))
    || typeof result.classification !== "string"
    || typeof result.operationId !== "string"
    || typeof result.receiptRelativePath !== "string"
    || !digest(result.databaseSha256)
    || result.restartRequired !== true) throw new Error("prepared_recovery_result_invalid");
  return result as unknown as PreparedRecoveryResult;
}

export async function deletePreparedRecoveryReceipt(
  relativePath: string,
  invokeCommand: InvokeCommand = (name, args) => invoke<unknown>(name, args),
): Promise<void> {
  await invokeCommand("delete_prepared_recovery_receipt", { relativePath });
}
