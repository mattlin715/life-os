import { describe, expect, it, vi } from "vitest";
import { authorizeFounderSchemaV5Migration, executePreparedStateRecovery, inspectFounderSchemaV5Startup, inspectPreparedStateRecovery, isDesktopSchemaV5Active, isOrdinaryDesktopSchemaV5, validateFounderSchemaV5State, validatePreparedRecoveryInspection } from "./founderSchemaV5";

const ready = { state: "ready", reason: null, detectedSchemaVersion: 5, supportedSchemaVersion: 5, initializationRequired: false, migrationAvailable: false, backupAvailable: true, restoreAvailable: false, backupRelativePath: "life-os-test.operation/backup.db", backupRetentionDays: 30, backupCreatedAt: "2026-08-13T00:00:00.000Z", backupExpiresAt: "2026-09-12T00:00:00.000Z", preparedRecoveryReceiptAvailable: false, preparedRecoveryReceiptRelativePath: null, preparedRecoveryReceiptOperationId: null, preparedRecoveryReceiptRecoveredAt: null } as const;

describe("Founder schema-v5 adapter", () => {
  it("selects ordinary schema-v5 in the default desktop frontend build", () => {
    expect(isOrdinaryDesktopSchemaV5).toBe(true);
    expect(isDesktopSchemaV5Active).toBe(true);
  });
  it("uses fixed commands without renderer paths or flags", async () => {
    const invoke = vi.fn(async () => ready);
    await inspectFounderSchemaV5Startup(invoke);
    await authorizeFounderSchemaV5Migration(invoke);
    expect(invoke.mock.calls).toEqual([["inspect_founder_schema_v5_startup"], ["authorize_founder_schema_v5_migration"]]);
  });
  it("fails closed on malformed or extra backend fields", () => {
    expect(() => validateFounderSchemaV5State({ ...ready, localPath: "secret" })).toThrow("founder_schema_v5_result_invalid");
    expect(() => validateFounderSchemaV5State({ ...ready, supportedSchemaVersion: 4 })).toThrow();
    expect(() => validateFounderSchemaV5State({ ...ready, state: "blocked", backupAvailable: false, restoreAvailable: true })).toThrow();
  });
  it("binds recovery execution to the inspected claim and current database digest", async () => {
    const inspection = { eligible: true, reason: null, classification: "exact_prepared_v4_empty_wal_v1", operationId: "op", database: { relativePath: "life-os.db", size: 1, sha256: "A".repeat(64) }, databaseIdentity: "windows-volume-1-file-2", evidenceFiles: [], preservedFiles: [], claimDigest: "B".repeat(64) } as const;
    const result = { classification: inspection.classification, operationId: "op", receiptRelativePath: "prepared-recovery-op.receipt.json", databaseSha256: "A".repeat(64), restartRequired: true } as const;
    const invoke = vi.fn().mockResolvedValueOnce(inspection).mockResolvedValueOnce(result);
    expect(await inspectPreparedStateRecovery(invoke)).toEqual(inspection);
    expect(await executePreparedStateRecovery(inspection, invoke)).toEqual(result);
    expect(invoke.mock.calls[1]).toEqual(["execute_prepared_state_recovery", {
      expectedClaimDigest: "B".repeat(64), expectedDatabaseSha256: "A".repeat(64),
    }]);
  });
  it("requires strict moved and preserved file facts for recovery inspection", () => {
    const valid = { eligible: true, reason: null, classification: "exact_v5_ready_legacy_empty_sidecar_v1", operationId: "op", database: { relativePath: "life-os.db", size: 1, sha256: "A".repeat(64) }, databaseIdentity: "windows-volume-1-file-2", evidenceFiles: [], preservedFiles: [{ relativePath: "life-os-op.operation/backup.db", size: 1, sha256: "C".repeat(64) }], claimDigest: "B".repeat(64) } as const;
    expect(validatePreparedRecoveryInspection(valid)).toEqual(valid);
    expect(() => validatePreparedRecoveryInspection({ ...valid, preservedFiles: undefined })).toThrow("prepared_recovery_result_invalid");
    expect(() => validatePreparedRecoveryInspection({ ...valid, preservedFiles: [{ ...valid.preservedFiles[0], absolutePath: "secret" }] })).toThrow("prepared_recovery_result_invalid");
  });
});
