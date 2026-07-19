import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  invoke: vi.fn(),
  load: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/plugin-sql", () => ({ default: { load: mocks.load } }));

import { createSqliteLocalEvidenceStore } from "./sqliteLocalEvidenceStore";
import type { HistoricalConsentEvent, HistoricalTransmissionEvent } from "../../../historicalContext/governedPacket";

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

describe("sqliteLocalEvidenceStore typed artifact and historical mutations", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("saves a validated artifact bundle through a typed Rust command", async () => {
    const db = createDatabase();
    db.select.mockResolvedValue([
      { id: "entry", content: "Body", created_at: "created", updated_at: "revision-1" },
    ]);
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));
    const bundle = { evidence: [], reflections: [], patterns: [], recoveryTurns: [] };

    await expect(store.saveArtifacts("entry", bundle, { expectedExperienceUpdatedAt: "revision-1" }))
      .resolves.toMatchObject({ status: "committed", bundle });

    expect(mocks.invoke).toHaveBeenCalledWith("save_sqlite_artifacts", {
      entryId: "entry",
      bundle,
      expectedExperienceUpdatedAt: "revision-1",
    });
    expect(db.execute).not.toHaveBeenCalled();
  });

  it("persists consent and transmission through named typed commands", async () => {
    const db = createDatabase();
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));
    const consent: HistoricalConsentEvent = {
      id: "consent", packetDigest: "digest", task: "historical_reflection_questions",
      purpose: "invite_user_comparison_without_cross_time_conclusions", provider: "openai", model: "model",
      sourceRevisions: [], state: "granted", createdAt: "created", expiresAt: "expires",
    };
    const transmission: HistoricalTransmissionEvent = {
      id: "transmission", consentId: consent.id, packetDigest: consent.packetDigest,
      provider: "openai", model: "model", outcome: "sent", createdAt: "created", expiresAt: "expires",
    };

    await store.saveHistoricalConsent(consent);
    await store.saveHistoricalTransmission(transmission);

    expect(mocks.invoke).toHaveBeenNthCalledWith(1, "save_sqlite_historical_consent", { event: consent });
    expect(mocks.invoke).toHaveBeenNthCalledWith(2, "save_sqlite_historical_transmission", { event: transmission });
    expect(db.execute).not.toHaveBeenCalled();
  });

  it("persists and deletes a Historical Question through typed commands", async () => {
    const db = createDatabase();
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));
    const artifact = {
      id: "question", currentExperienceId: "current", questions: [], consentId: "consent",
      transmissionId: "transmission", generatedAt: "generated",
      packet: {
        packetId: "packet", packetDigest: "digest", schemaVersion: "historical-packet-v1",
        assembledAt: "assembled", expiresAt: "expires",
        currentExperience: { id: "current", revision: "revision", content: "body" },
        task: "historical_reflection_questions",
        purpose: "invite_user_comparison_without_cross_time_conclusions",
        locale: "en", responseLanguage: "en",
        destination: { provider: "openai", model: "model", retentionDisclosure: "disclosure" },
        versions: { harness: "test", prompt: "historical-reflection-question-v1", outputSchema: "historical-question-output-v1", safetyContract: "phase-3b-safety-v1" },
        includedItems: [], consent: { reference: "consent", scope: "one_generation_one_purpose" },
        limits: { maxSources: 3, maxContentCharacters: 6000 },
      },
    };

    await store.saveHistoricalQuestionArtifact(artifact as never);
    await store.deleteHistoricalQuestionArtifact(artifact.id);

    expect(mocks.invoke).toHaveBeenNthCalledWith(1, "save_sqlite_historical_question_artifact", { artifact });
    expect(mocks.invoke).toHaveBeenNthCalledWith(2, "delete_sqlite_historical_question_artifact", { id: artifact.id });
    expect(db.execute).not.toHaveBeenCalled();
  });

  it("runs expired-audit cleanup through a timestamp-only typed command", async () => {
    const db = createDatabase();
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

    await store.purgeExpiredHistoricalAuditRecords("2026-07-19T00:00:00.000Z");

    expect(mocks.invoke).toHaveBeenCalledWith("purge_sqlite_expired_historical_audit_records", {
      timestamp: "2026-07-19T00:00:00.000Z",
    });
    expect(db.execute).not.toHaveBeenCalled();
  });

  it("never sends SQL-shaped fields through any remaining mutation adapter", async () => {
    const db = createDatabase();
    db.select.mockResolvedValue([
      { id: "entry", content: "Body", created_at: "created", updated_at: "revision-1" },
    ]);
    mocks.invoke.mockResolvedValue({ status: "committed" });
    const store = createSqliteLocalEvidenceStore(Promise.resolve(db as never));

    await store.saveArtifacts("entry", { evidence: [], reflections: [], patterns: [], recoveryTurns: [] });
    await store.deleteHistoricalQuestionArtifact("question");
    await store.purgeExpiredHistoricalAuditRecords("timestamp");

    for (const [, payload] of mocks.invoke.mock.calls) {
      const serialized = JSON.stringify(payload ?? {});
      expect(serialized).not.toMatch(/"(?:query|values|statements)"\s*:/);
    }
  });
});
