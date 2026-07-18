import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  invoke: vi.fn(),
  load: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/plugin-sql", () => ({ default: { load: mocks.load } }));

import { createSqliteLocalEvidenceStore } from "./sqliteLocalEvidenceStore";

const createDatabase = () => ({
  select: vi.fn(),
  execute: vi.fn(),
});

describe("sqliteLocalEvidenceStore typed Experience mutations", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("creates an Experience through a typed Rust command without renderer SQL", async () => {
    const db = createDatabase();
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

    const created = await store.createExperience({ body: "A synthetic moment" });

    expect(mocks.invoke).toHaveBeenCalledWith("create_sqlite_experience", {
      experience: {
        id: created.id,
        content: "A synthetic moment",
        createdAt: created.createdAt,
        updatedAt: created.updatedAt,
      },
    });
    expect(db.execute).not.toHaveBeenCalled();
  });

  it("imports an exact batch through one typed atomic command", async () => {
    const db = createDatabase();
    mocks.invoke.mockResolvedValue({ importedCount: 1, skippedCount: 1 });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

    const result = await store.importExperiences([
      { id: "one", body: "First", createdAt: "created-1", updatedAt: "updated-1" },
      { id: "two", body: "Second", createdAt: "created-2", updatedAt: "updated-2" },
    ]);

    expect(result).toEqual({ importedCount: 1, skippedCount: 1 });
    expect(mocks.invoke).toHaveBeenCalledTimes(1);
    expect(mocks.invoke).toHaveBeenCalledWith("import_sqlite_experiences", {
      experiences: [
        { id: "one", content: "First", createdAt: "created-1", updatedAt: "updated-1" },
        { id: "two", content: "Second", createdAt: "created-2", updatedAt: "updated-2" },
      ],
    });
    expect(db.execute).not.toHaveBeenCalled();
    expect(db.select).not.toHaveBeenCalled();
  });

  it("updates through a typed command with the durable expected revision", async () => {
    const db = createDatabase();
    db.select.mockResolvedValue([
      { id: "entry", content: "Before", created_at: "created", updated_at: "revision-1" },
    ]);
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

    const updated = await store.updateExperience("entry", { body: "After" });

    expect(updated).toMatchObject({ id: "entry", body: "After", createdAt: "created" });
    expect(mocks.invoke).toHaveBeenCalledWith("update_sqlite_experience", {
      id: "entry",
      content: "After",
      expectedUpdatedAt: "revision-1",
      updatedAt: updated?.updatedAt,
    });
    expect(db.execute).not.toHaveBeenCalled();
  });

  it("rejects a stale typed update instead of pretending it committed", async () => {
    const db = createDatabase();
    db.select.mockResolvedValue([
      { id: "entry", content: "Before", created_at: "created", updated_at: "revision-1" },
    ]);
    mocks.invoke.mockResolvedValue({ status: "stale_generation" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

    await expect(store.updateExperience("entry", { body: "Stale edit" })).rejects.toThrow(
      "stale_generation",
    );
    expect(db.execute).not.toHaveBeenCalled();
  });

  it("advances the durable revision even when an update occurs in the same clock tick", async () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-07-19T00:00:00.000Z"));
    try {
      const db = createDatabase();
      db.select.mockResolvedValue([
        {
          id: "entry",
          content: "Before",
          created_at: "2026-07-18T00:00:00.000Z",
          updated_at: "2026-07-19T00:00:00.000Z",
        },
      ]);
      mocks.invoke.mockResolvedValue({ status: "committed" });
      const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

      const updated = await store.updateExperience("entry", { body: "After" });

      expect(updated?.updatedAt).toBe("2026-07-19T00:00:00.001Z");
      expect(mocks.invoke).toHaveBeenCalledWith("update_sqlite_experience", {
        id: "entry",
        content: "After",
        expectedUpdatedAt: "2026-07-19T00:00:00.000Z",
        updatedAt: "2026-07-19T00:00:00.001Z",
      });
    } finally {
      vi.useRealTimers();
    }
  });

  it("deletes through the typed Rust cascade boundary without renderer SQL", async () => {
    const db = createDatabase();
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

    await store.deleteExperience("entry");

    expect(mocks.invoke).toHaveBeenCalledWith("delete_sqlite_experience", { id: "entry" });
    expect(db.execute).not.toHaveBeenCalled();
    expect(db.select).not.toHaveBeenCalled();
  });
});
