import { describe, expect, it, vi } from "vitest";

import {
  inspectDatabaseReadiness,
  validateDatabaseReadinessResult,
} from "./databaseReadiness";

const exactV4 = {
  classification: "exact_v4",
  databaseExists: true,
  detectedSchemaVersion: 4,
  supportedSchemaVersion: 5,
  walPresent: false,
  shmPresent: false,
  rollbackJournalPresent: false,
  quiescence: "not_proven",
  operationEvidence: "none",
  schemaV5Available: true,
  inspectedAtUnixMs: 1_786_310_000_000,
} as const;

describe("database readiness adapter", () => {
  it("invokes only when explicitly called and uses the exact read-only command", async () => {
    const invokeCommand = vi.fn().mockResolvedValue(exactV4);
    expect(invokeCommand).not.toHaveBeenCalled();
    await expect(inspectDatabaseReadiness(invokeCommand)).resolves.toEqual(exactV4);
    expect(invokeCommand).toHaveBeenCalledOnce();
    expect(invokeCommand).toHaveBeenCalledWith("inspect_database_readiness");
  });

  it.each([
    "missing",
    "older_supported",
    "exact_v4",
    "exact_v5",
    "newer_unsupported",
    "malformed",
    "unreadable",
    "path_unsafe",
    "recovery_required",
  ] as const)("accepts the bounded %s classification", (classification) => {
    expect(validateDatabaseReadinessResult({ ...exactV4, classification }).classification)
      .toBe(classification);
  });

  it.each([
    null,
    {},
    { ...exactV4, classification: "ready_to_migrate" },
    { ...exactV4, supportedSchemaVersion: 4 },
    { ...exactV4, schemaV5Available: false },
    { ...exactV4, walPresent: "no" },
    { ...exactV4, localPath: "C:\\private\\life-os.db" },
  ])("fails malformed or authority-expanding payloads closed", (value) => {
    const result = validateDatabaseReadinessResult(value);
    expect(result.classification).toBe("unreadable");
    expect(result.schemaV5Available).toBe(true);
    expect(result.databaseExists).toBeNull();
  });

  it("suppresses raw command failures", async () => {
    const result = await inspectDatabaseReadiness(
      vi.fn().mockRejectedValue(new Error("C:\\Users\\private\\life-os.db SQL error")),
    );
    expect(result.classification).toBe("unreadable");
    expect(JSON.stringify(result)).not.toContain("private");
    expect(JSON.stringify(result)).not.toContain("SQL error");
  });
});
