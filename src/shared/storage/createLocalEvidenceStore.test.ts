import { beforeEach, describe, expect, it, vi } from "vitest";

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

const state = (name: "missing" | "migration_required" | "ready" | "blocked", version: number | null, reason: string | null = null) => ({
  state: name,
  reason,
  detectedSchemaVersion: version,
  supportedSchemaVersion: 5,
  initializationRequired: name === "missing",
  migrationAvailable: name === "migration_required",
  backupAvailable: false,
  restoreAvailable: false,
  backupRelativePath: null,
  backupRetentionDays: 30,
  backupCreatedAt: null,
  backupExpiresAt: null,
});

describe("ordinary desktop schema-v5 startup", () => {
  beforeEach(() => {
    mocks.invoke.mockReset();
    mocks.initialize.mockReset();
    mocks.createSqliteStore.mockReset();
  });

  it("shows exact v4 as migration-required without constructing a writable store", async () => {
    mocks.invoke.mockResolvedValue(state("migration_required", 4));
    const runtime = createLocalEvidenceStoreRuntime();
    const blockedOperation = runtime.store.listExperiences().catch((error: unknown) => error);
    await expect(runtime.startup).resolves.toMatchObject({ state: "blocked", reason: "migration_required", detectedSchemaVersion: 4, supportedSchemaVersion: 5 });
    await expect(blockedOperation).resolves.toEqual(expect.objectContaining({ message: "local_database_startup_blocked" }));
    expect(mocks.invoke).toHaveBeenCalledTimes(1);
    expect(mocks.initialize).not.toHaveBeenCalled();
  });

  it("initializes a missing disposable profile directly through the exact-v5 command", async () => {
    mocks.invoke
      .mockResolvedValueOnce(state("missing", null))
      .mockResolvedValueOnce(state("ready", 5));
    const runtime = createLocalEvidenceStoreRuntime();
    await expect(runtime.startup).resolves.toMatchObject({ state: "ready", detectedSchemaVersion: 5, supportedSchemaVersion: 5 });
    expect(mocks.invoke.mock.calls.map(([command]) => command)).toEqual([
      "inspect_founder_schema_v5_startup",
      "initialize_founder_schema_v5_database",
    ]);
  });

  it("stabilizes v2 or v3 only to v4 before separate migration disclosure", async () => {
    mocks.invoke
      .mockResolvedValueOnce(state("blocked", 3, "older_schema_unsupported"))
      .mockResolvedValueOnce(undefined)
      .mockResolvedValueOnce(state("migration_required", 4));
    const runtime = createLocalEvidenceStoreRuntime();
    await expect(runtime.startup).resolves.toMatchObject({ state: "blocked", reason: "migration_required", detectedSchemaVersion: 4 });
    expect(mocks.invoke.mock.calls.map(([command]) => command)).toEqual([
      "inspect_founder_schema_v5_startup",
      "initialize_sqlite_database",
      "inspect_founder_schema_v5_startup",
    ]);
  });

  it("blocks malformed or recovery-required state and exposes no store", async () => {
    mocks.invoke.mockResolvedValue(state("blocked", 5, "recovery_required"));
    const runtime = createLocalEvidenceStoreRuntime();
    const blockedOperation = runtime.store.createExperience({ body: "must not be written" }).catch((error: unknown) => error);
    await expect(runtime.startup).resolves.toMatchObject({ state: "blocked", reason: "founder_v5_blocked" });
    await expect(blockedOperation).resolves.toEqual(expect.objectContaining({ message: "local_database_startup_blocked" }));
    expect(mocks.initialize).not.toHaveBeenCalled();
    expect(mocks.createSqliteStore).not.toHaveBeenCalled();
  });
});
