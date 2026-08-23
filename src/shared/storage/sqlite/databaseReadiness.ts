import { invoke } from "@tauri-apps/api/core";

import { isDesktopSchemaV5Active } from "./founderSchemaV5";

export type DatabaseReadinessClassification =
  | "missing"
  | "older_supported"
  | "exact_v4"
  | "exact_v5"
  | "newer_unsupported"
  | "malformed"
  | "unreadable"
  | "path_unsafe"
  | "recovery_required";

export interface DatabaseReadinessResult {
  readonly classification: DatabaseReadinessClassification;
  readonly databaseExists: boolean | null;
  readonly detectedSchemaVersion: number | null;
  readonly supportedSchemaVersion: number;
  readonly walPresent: boolean | null;
  readonly shmPresent: boolean | null;
  readonly rollbackJournalPresent: boolean | null;
  readonly quiescence: "not_proven";
  readonly operationEvidence: "none" | "present" | "unknown";
  readonly schemaV5Available: boolean;
  readonly inspectedAtUnixMs: number;
}

type InvokeCommand = <T>(command: string) => Promise<T>;

const classifications = new Set<DatabaseReadinessClassification>([
  "missing",
  "older_supported",
  "exact_v4",
  "exact_v5",
  "newer_unsupported",
  "malformed",
  "unreadable",
  "path_unsafe",
  "recovery_required",
]);
const resultKeys = new Set([
  "classification",
  "databaseExists",
  "detectedSchemaVersion",
  "supportedSchemaVersion",
  "walPresent",
  "shmPresent",
  "rollbackJournalPresent",
  "quiescence",
  "operationEvidence",
  "schemaV5Available",
  "inspectedAtUnixMs",
]);

function isNullableBoolean(value: unknown): value is boolean | null {
  return value === null || typeof value === "boolean";
}

function failClosed(): DatabaseReadinessResult {
  const supportedSchemaVersion = isDesktopSchemaV5Active ? 5 : 4;
  return {
    classification: "unreadable",
    databaseExists: null,
    detectedSchemaVersion: null,
    supportedSchemaVersion,
    walPresent: null,
    shmPresent: null,
    rollbackJournalPresent: null,
    quiescence: "not_proven",
    operationEvidence: "unknown",
    schemaV5Available: isDesktopSchemaV5Active,
    inspectedAtUnixMs: Date.now(),
  };
}

export function validateDatabaseReadinessResult(value: unknown): DatabaseReadinessResult {
  if (!value || typeof value !== "object") return failClosed();
  const result = value as Record<string, unknown>;
  if (
    Object.keys(result).some((key) => !resultKeys.has(key))
    || typeof result.classification !== "string"
    || !classifications.has(result.classification as DatabaseReadinessClassification)
    || !isNullableBoolean(result.databaseExists)
    || !(result.detectedSchemaVersion === null || Number.isSafeInteger(result.detectedSchemaVersion))
    || result.supportedSchemaVersion !== (isDesktopSchemaV5Active ? 5 : 4)
    || !isNullableBoolean(result.walPresent)
    || !isNullableBoolean(result.shmPresent)
    || !isNullableBoolean(result.rollbackJournalPresent)
    || result.quiescence !== "not_proven"
    || !["none", "present", "unknown"].includes(String(result.operationEvidence))
    || result.schemaV5Available !== isDesktopSchemaV5Active
    || !Number.isSafeInteger(result.inspectedAtUnixMs)
    || Number(result.inspectedAtUnixMs) < 0
  ) {
    return failClosed();
  }
  return result as unknown as DatabaseReadinessResult;
}

export async function inspectDatabaseReadiness(
  invokeCommand: InvokeCommand = invoke,
): Promise<DatabaseReadinessResult> {
  try {
    return validateDatabaseReadinessResult(
      await invokeCommand<unknown>("inspect_database_readiness"),
    );
  } catch {
    return failClosed();
  }
}
