import { describe, expect, it, vi } from "vitest";
import { authorizeFounderSchemaV5Migration, inspectFounderSchemaV5Startup, isDesktopSchemaV5Active, isOrdinaryDesktopSchemaV5, validateFounderSchemaV5State } from "./founderSchemaV5";

const ready = { state: "ready", reason: null, detectedSchemaVersion: 5, supportedSchemaVersion: 5, initializationRequired: false, migrationAvailable: false, backupAvailable: true, restoreAvailable: false, backupRelativePath: "life-os-test.operation/backup.db", backupRetentionDays: 30, backupCreatedAt: "2026-08-13T00:00:00.000Z", backupExpiresAt: "2026-09-12T00:00:00.000Z" } as const;

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
});
