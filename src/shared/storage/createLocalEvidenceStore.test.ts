import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  invoke: vi.fn(),
  initialize: vi.fn(),
  createSqliteStore: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: mocks.invoke,
  isTauri: () => true,
}));

vi.mock("./sqlite/sqliteLocalEvidenceStore", () => ({
  initializeSqliteDatabaseConnection: mocks.initialize,
  createSqliteLocalEvidenceStore: mocks.createSqliteStore,
}));

import { createLocalEvidenceStoreRuntime } from "./createLocalEvidenceStore";

describe("createLocalEvidenceStoreRuntime", () => {
  it("does not construct or expose SQLite operations when a newer schema is detected", async () => {
    mocks.invoke.mockResolvedValue({
      state: "blocked",
      reason: "newer_schema",
      detectedSchemaVersion: 5,
      supportedSchemaVersion: 4,
      initializationRequired: false,
    });

    const runtime = createLocalEvidenceStoreRuntime();
    const attemptedOperations = Promise.allSettled([
      runtime.store.listExperiences(),
      runtime.store.purgeExpiredHistoricalAuditRecords("2026-07-18T00:00:00.000Z"),
      runtime.store.createExperience({ body: "must not be written" }),
    ]);

    await expect(runtime.startup).resolves.toMatchObject({
      state: "blocked",
      reason: "newer_schema",
      detectedSchemaVersion: 5,
      supportedSchemaVersion: 4,
      storage: "sqlite",
    });
    await expect(attemptedOperations).resolves.toEqual([
      expect.objectContaining({ status: "rejected" }),
      expect.objectContaining({ status: "rejected" }),
      expect.objectContaining({ status: "rejected" }),
    ]);
    expect(mocks.initialize).not.toHaveBeenCalled();
    expect(mocks.createSqliteStore).not.toHaveBeenCalled();
  });
});
