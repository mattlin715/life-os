import { describe, expect, it, vi } from "vitest";
import type { EvidenceCandidate, ExperienceEntry, ReflectionPrompt } from "../types/domain";
import { placeholderProvider } from "../ai/providers/placeholderProvider";
import { retrieveCandidatesForOpenHistoricalPanels } from "./panelRetrieval";
import { closeHistoricalContextPanel, openHistoricalContextPanel } from "./panelState";
import { clearHistoricalContextSelection, isHistoricalContextSelected, removeHistoricalContextSelectionsForExperience, toggleHistoricalContextSelection } from "./selection";
import { findHistoricalContextCandidates } from "./retrieve";
import { buildHistoricalSavedDateRange } from "./savedDateRange";

const timestamp = (day: number) => `2026-07-${String(day).padStart(2, "0")}T00:00:00.000Z`;
const entry = (id: string, body: string, day = 1): ExperienceEntry => ({ id, body, createdAt: timestamp(day), updatedAt: timestamp(day), userEditable: true });
const provenance = (entryId: string, origin: "user" | "ai" = "ai") => ({ origin, sourceEntryId: entryId, sourceArtifactIds: [], provider: origin === "user" ? undefined : "mock" as const, model: null, harnessVersion: "harness-v1", promptVersion: "v1", generatedAt: timestamp(1) });
const evidence = (id: string, entryId: string, text: string, status: EvidenceCandidate["status"] = "confirmed"): EvidenceCandidate => ({ id, sourceEntryId: entryId, text, kind: "observation", status, userEditable: true, provenance: provenance(entryId), createdAt: timestamp(1), updatedAt: timestamp(1) });
const reflection = (id: string, entryId: string, sourceEvidenceIds: string[], response: string, status: ReflectionPrompt["status"] = "answered", userAuthored = true): ReflectionPrompt => ({ id, sourceEntryId: entryId, sourceEvidenceIds, question: "What mattered?", response, status, promptProvenance: provenance(entryId), responseProvenance: userAuthored ? provenance(entryId, "user") : provenance(entryId), createdAt: timestamp(1), updatedAt: timestamp(1) });
const savedDateRange = (startDate: string, endDate: string) => {
  const result = buildHistoricalSavedDateRange(startDate, endDate, {
    timeZone: "UTC",
    localStartOfDay: ({ year, month, day }) => {
      const value = new Date(0);
      value.setUTCFullYear(year, month - 1, day);
      value.setUTCHours(0, 0, 0, 0);
      return value.getTime();
    },
  });
  if (result.status !== "valid") throw new Error(result.reason);
  return result.range;
};

describe("local historical context retrieval", () => {
  it("returns no history for one entry and always excludes the current Experience", () => {
    const current = entry("current", "The project deadline kept me awake.");
    expect(findHistoricalContextCandidates({ currentExperience: current, experiences: [current], artifactsByEntryId: {}, locale: "en" })).toEqual([]);
    const candidates = findHistoricalContextCandidates({ currentExperience: current, experiences: [current, entry("past", "The deadline was difficult.", 2)], artifactsByEntryId: {}, locale: "en" });
    expect(candidates.map((candidate) => candidate.sourceExperienceId)).toEqual(["past"]);
  });

  it("does not retrieve a deleted Experience merely because stale artifact data exists", () => {
    const current = entry("current", "The deadline kept me awake.");
    expect(findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current],
      artifactsByEntryId: { deleted: { evidence: [evidence("deleted-evidence", "deleted", "deadline")], reflections: [] } },
      locale: "en",
    })).toEqual([]);
  });

  it("uses bounded, deterministic ranking with visible English terms and stable ID ties", () => {
    const current = entry("current", "The project deadline kept me awake.");
    const candidates = findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, entry("b", "A deadline changed the project plan.", 2), entry("a", "The project deadline felt heavy.", 2), entry("extra", "An unrelated quiet afternoon.", 3)],
      artifactsByEntryId: {}, locale: "en", maxCandidates: 2,
    });
    expect(candidates).toHaveLength(2);
    expect(candidates.map((candidate) => candidate.sourceExperienceId)).toEqual(["a", "b"]);
    expect(candidates[0].reasons[0]).toMatchObject({ kind: "shared_visible_terms" });
    expect(candidates[0].reasons[0].terms).toContain("deadline");
    expect(candidates[0].ranking.algorithmVersion).toBe("local-lexical-v1");
  });

  it("preserves the exact Phase 3A path when no saved-date range is supplied", () => {
    const current = entry("current", "The project deadline mattered.", 15);
    const experiences = [
      current,
      entry("older", "The project deadline mattered before.", 2),
      entry("newer", "The project deadline mattered again.", 12),
    ];
    const input = { currentExperience: current, experiences, artifactsByEntryId: {}, locale: "en" as const };
    expect(findHistoricalContextCandidates(input)).toEqual(
      findHistoricalContextCandidates({ ...input, savedDateRange: undefined }),
    );
  });

  it("filters by source createdAt before unchanged lexical ranking and limiting", () => {
    const current = entry("current", "project deadline planning", 20);
    const candidates = findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [
        current,
        entry("outside-high", "project deadline planning project deadline planning", 2),
        entry("inside-b", "project deadline", 10),
        entry("inside-a", "project deadline", 10),
      ],
      artifactsByEntryId: {},
      locale: "en",
      maxCandidates: 2,
      savedDateRange: savedDateRange("2026-07-10", "2026-07-10"),
    });

    expect(candidates.map((candidate) => candidate.sourceExperienceId)).toEqual([
      "inside-a",
      "inside-b",
    ]);
    expect(candidates.every((candidate) => candidate.ranking.algorithmVersion === "local-lexical-v1")).toBe(true);
  });

  it("includes both saved-date boundaries and returns zero results outside them", () => {
    const current = entry("current", "project deadline", 20);
    const experiences = [
      current,
      { ...entry("start", "project deadline", 1), createdAt: "2026-07-10T00:00:00.000Z" },
      { ...entry("end", "project deadline", 1), createdAt: "2026-07-12T23:59:59.999Z" },
      { ...entry("after", "project deadline", 1), createdAt: "2026-07-13T00:00:00.000Z" },
    ];
    const candidates = findHistoricalContextCandidates({
      currentExperience: current,
      experiences,
      artifactsByEntryId: {},
      locale: "en",
      savedDateRange: savedDateRange("2026-07-10", "2026-07-12"),
    });
    expect(candidates.map((candidate) => candidate.sourceExperienceId).sort()).toEqual(["end", "start"]);
    expect(findHistoricalContextCandidates({
      currentExperience: current,
      experiences,
      artifactsByEntryId: {},
      locale: "en",
      savedDateRange: savedDateRange("2026-06-01", "2026-06-02"),
    })).toEqual([]);
  });

  it("excludes malformed source createdAt only when an active range must prove eligibility", () => {
    const current = entry("current", "project deadline", 20);
    const malformed = { ...entry("malformed", "project deadline", 2), createdAt: "July 2, 2026" };
    expect(findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, malformed],
      artifactsByEntryId: {},
      locale: "en",
    })).toHaveLength(1);
    expect(findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, malformed],
      artifactsByEntryId: {},
      locale: "en",
      savedDateRange: savedDateRange("2026-07-01", "2026-07-31"),
    })).toEqual([]);
  });

  it("normalizes English casing with the explicit Life OS English locale", () => {
    const current = entry("current", "ISTANBUL remains important.");
    const candidates = findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, entry("past", "istanbul was important too.", 2)],
      artifactsByEntryId: {},
      locale: "en",
    });

    expect(candidates).toHaveLength(1);
    expect(candidates[0].reasons[0].terms).toContain("istanbul");
  });

  it.each([
    ["zh-TW", "\u6df1\u547c", "\u6df1\uff0c\u547c"],
    ["zh-TW", "\u6df1\u547c", "\u6df1 \u547c"],
    ["zh-TW", "\u6df1\u547c", "\u6df1\n\u547c"],
    ["zh-TW", "\u6df1\u547c", "\u6df1\ud83d\ude42\u547c"],
    ["ja", "\u96c6\u4e2d", "\u96c6\u3002\u4e2d"],
    ["ja", "\u96c6\u4e2d", "\u96c6 \u4e2d"],
    ["ja", "\u96c6\u4e2d", "\u96c6\n\u4e2d"],
    ["ja", "\u96c6\u4e2d", "\u96c6\ud83d\ude42\u4e2d"],
  ] as const)("does not create a cross-boundary %s candidate", (locale, currentBody, sourceBody) => {
    const current = entry("current", currentBody);
    expect(findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, entry("past", sourceBody, 2)],
      artifactsByEntryId: {},
      locale,
    })).toEqual([]);
  });

  it.each([
    ["zh-TW", "\u6df1\u547c", "\u6df1\u547c\u653e\u9b06", "\u6df1\u547c"],
    ["ja", "\u96c6\u4e2d", "\u96c6\u4e2d\u3059\u308b", "\u96c6\u4e2d"],
  ] as const)("keeps adjacent %s terms visible and explainable", (locale, currentBody, sourceBody, sharedTerm) => {
    const current = entry("current", currentBody);
    const candidates = findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, entry("past", sourceBody, 2)],
      artifactsByEntryId: {},
      locale,
    });

    expect(candidates).toHaveLength(1);
    expect(candidates[0].reasons[0].terms).toContain(sharedTerm);
  });

  it("matches Traditional Chinese and Japanese only through visible local terms", () => {
    const chinese = entry("current-zh", "會議後我一直感到不安");
    const japanese = entry("current-ja", "会議の後も不安が残った");
    expect(findHistoricalContextCandidates({ currentExperience: chinese, experiences: [chinese, entry("past-zh", "昨天會議後也感到不安", 2)], artifactsByEntryId: {}, locale: "zh-TW" })).toHaveLength(1);
    expect(findHistoricalContextCandidates({ currentExperience: japanese, experiences: [japanese, entry("past-ja", "会議の後に不安だった", 2)], artifactsByEntryId: {}, locale: "ja" })).toHaveLength(1);
  });

  it("uses only confirmed Evidence and committed user-authored answered Reflection", () => {
    const current = entry("current", "The deadline made the conversation hard.");
    const past = entry("past", "A neutral note.", 2);
    const candidates = findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, past],
      artifactsByEntryId: {
        past: {
          evidence: [
            evidence("keep", "past", "A deadline created pressure."),
            evidence("reject", "past", "The conversation was hard.", "rejected"),
            evidence("foreign", "other", "deadline"),
          ],
          reflections: [
            reflection("answered", "past", ["keep"], "The deadline made conversation difficult."),
            reflection("skipped", "past", ["keep"], "conversation", "skipped"),
            reflection("orphan", "past", ["missing"], "deadline"),
            reflection("ai-answer", "past", ["keep"], "deadline", "answered", false),
          ],
        },
      }, locale: "en",
    });
    expect(candidates).toHaveLength(1);
    expect(candidates[0].confirmedEvidenceIds).toEqual(["keep"]);
    expect(candidates[0].answeredReflectionIds).toEqual(["answered"]);
  });

  it("does not manufacture candidates from rejected Evidence or skipped Reflection", () => {
    const current = entry("current", "The private word appears here.");
    const past = entry("past", "An unrelated moment.", 2);
    const candidates = findHistoricalContextCandidates({
      currentExperience: current,
      experiences: [current, past],
      artifactsByEntryId: { past: { evidence: [evidence("rejected", "past", "private word", "rejected")], reflections: [reflection("skipped", "past", ["rejected"], "private word", "skipped")] } },
      locale: "en",
    });
    expect(candidates).toEqual([]);
  });

  it("keeps exact IDs collision-safe and manages inclusion only in ephemeral UI state", () => {
    let selections = toggleHistoricalContextSelection(new Map(), "a", "a:b");
    selections = toggleHistoricalContextSelection(selections, "a:b", "a");
    expect(isHistoricalContextSelected(selections, "a", "a:b")).toBe(true);
    expect(isHistoricalContextSelected(selections, "a:b", "a")).toBe(true);
    selections = clearHistoricalContextSelection(selections, "a");
    expect(isHistoricalContextSelected(selections, "a", "a:b")).toBe(false);
    expect(isHistoricalContextSelected(selections, "a:b", "a")).toBe(true);
    selections = removeHistoricalContextSelectionsForExperience(selections, "a");
    expect(isHistoricalContextSelected(selections, "a:b", "a")).toBe(false);
  });

  it("opens, retrieves, selects, excludes, clears, and closes locally without a provider call", () => {
    const providerCall = vi.spyOn(placeholderProvider, "extractEvidence");
    const current = entry("current", "A project deadline changed today.");
    const past = entry("past", "A deadline changed the project.", 2);
    let panels = openHistoricalContextPanel(new Set(), current.id);
    retrieveCandidatesForOpenHistoricalPanels({
      openPanelExperienceIds: panels,
      experiences: [current, past],
      artifactsByEntryId: {},
      locale: "en",
    });
    let selections = toggleHistoricalContextSelection(new Map(), "current", "past");
    selections = toggleHistoricalContextSelection(selections, "current", "past");
    selections = toggleHistoricalContextSelection(selections, "current", "past");
    clearHistoricalContextSelection(selections, "current");
    panels = closeHistoricalContextPanel(panels, current.id);
    retrieveCandidatesForOpenHistoricalPanels({
      openPanelExperienceIds: panels,
      experiences: [current, past],
      artifactsByEntryId: {},
      locale: "en",
    });
    expect(providerCall).not.toHaveBeenCalled();
    providerCall.mockRestore();
  });
});
