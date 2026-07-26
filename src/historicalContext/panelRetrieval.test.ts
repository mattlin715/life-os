import { describe, expect, it, vi } from "vitest";
import type { ExperienceEntry } from "../types/domain";
import { closeHistoricalContextPanel, openHistoricalContextPanel } from "./panelState";
import {
  isHistoricalContextSelected,
  reconcileHistoricalContextSelections,
  toggleHistoricalContextSelection,
} from "./selection";
import {
  retrieveCandidatesForOpenHistoricalPanels,
  type HistoricalContextCandidateRetriever,
} from "./panelRetrieval";
import type { HistoricalContextCandidate } from "./types";
import type { HistoricalSavedDateRange } from "./savedDateRange";

const timestamp = "2026-07-13T00:00:00.000Z";
const entry = (id: string): ExperienceEntry => ({
  id,
  body: `Body for ${id}`,
  createdAt: timestamp,
  updatedAt: timestamp,
  userEditable: true,
});

function retrievalSpy(): HistoricalContextCandidateRetriever {
  return vi.fn(() => []);
}

function retrieve(
  openPanelExperienceIds: ReadonlySet<string>,
  experiences: ExperienceEntry[],
  retrieveCandidates = retrievalSpy(),
) {
  const result = retrieveCandidatesForOpenHistoricalPanels({
    openPanelExperienceIds,
    experiences,
    artifactsByEntryId: {},
    locale: "en",
    retrieveCandidates,
  });
  return { result, retrieveCandidates };
}

describe("explicit historical panel retrieval", () => {
  it("does not retrieve during hydration when zero panels are open", () => {
    const entries = [entry("first"), entry("second")];
    const { result, retrieveCandidates } = retrieve(new Set(), entries);

    expect(retrieveCandidates).not.toHaveBeenCalled();
    expect(result).toEqual(new Map());
  });

  it("fails closed without calling retrieval when an active range is not applied", () => {
    const current = entry("current");
    const source = entry("source");
    const retrieveCandidates = retrievalSpy();
    const result = retrieveCandidatesForOpenHistoricalPanels({
      openPanelExperienceIds: new Set([current.id]),
      experiences: [current, source],
      artifactsByEntryId: {},
      locale: "en",
      savedDateRangeConstraintsByCurrentExperienceId: new Map([
        [current.id, { status: "blocked", reason: "not_applied" }],
      ]),
      retrieveCandidates,
    });

    expect(retrieveCandidates).not.toHaveBeenCalled();
    expect(result.get(current.id)).toEqual([]);
  });

  it("passes an exact applied range only to its open panel", () => {
    const first = entry("first");
    const second = entry("second");
    const range: HistoricalSavedDateRange = {
      startDate: "2026-07-01",
      endDate: "2026-07-13",
      timeZone: "UTC",
      startInclusiveMs: 0,
      endExclusiveMs: 1,
    };
    const retrieveCandidates = retrievalSpy();
    retrieveCandidatesForOpenHistoricalPanels({
      openPanelExperienceIds: new Set([first.id]),
      experiences: [first, second],
      artifactsByEntryId: {},
      locale: "en",
      savedDateRangeConstraintsByCurrentExperienceId: new Map([
        [first.id, { status: "applied", range }],
        [second.id, { status: "blocked", reason: "not_applied" }],
      ]),
      retrieveCandidates,
    });

    expect(retrieveCandidates).toHaveBeenCalledTimes(1);
    expect(retrieveCandidates).toHaveBeenCalledWith(
      expect.objectContaining({ currentExperience: first, savedDateRange: range }),
    );
  });

  it("keeps a retained range inert while its panel is closed", () => {
    const current = entry("current");
    const range: HistoricalSavedDateRange = {
      startDate: "2026-07-01",
      endDate: "2026-07-13",
      timeZone: "UTC",
      startInclusiveMs: 0,
      endExclusiveMs: 1,
    };
    const retrieveCandidates = retrievalSpy();
    const result = retrieveCandidatesForOpenHistoricalPanels({
      openPanelExperienceIds: new Set(),
      experiences: [current],
      artifactsByEntryId: {},
      locale: "en",
      savedDateRangeConstraintsByCurrentExperienceId: new Map([
        [current.id, { status: "applied", range }],
      ]),
      retrieveCandidates,
    });

    expect(retrieveCandidates).not.toHaveBeenCalled();
    expect(result).toEqual(new Map());
  });

  it("retrieves only the one current Experience whose panel is open", () => {
    const first = entry("first");
    const second = entry("second");
    const { result, retrieveCandidates } = retrieve(new Set(["second"]), [first, second]);

    expect(retrieveCandidates).toHaveBeenCalledTimes(1);
    expect(retrieveCandidates).toHaveBeenCalledWith(expect.objectContaining({ currentExperience: second }));
    expect(result.has("first")).toBe(false);
    expect(result.has("second")).toBe(true);
  });

  it("retrieves once per exact open Experience when two panels are open", () => {
    const first = entry("first");
    const second = entry("second");
    const third = entry("third");
    const { result, retrieveCandidates } = retrieve(new Set(["first", "third"]), [first, second, third]);

    expect(retrieveCandidates).toHaveBeenCalledTimes(2);
    expect(retrieveCandidates).toHaveBeenNthCalledWith(1, expect.objectContaining({ currentExperience: first }));
    expect(retrieveCandidates).toHaveBeenNthCalledWith(2, expect.objectContaining({ currentExperience: third }));
    expect([...result.keys()]).toEqual(["first", "third"]);
  });

  it("stops retrieval for a panel once it is closed", () => {
    const first = entry("first");
    const second = entry("second");
    const open = retrieve(new Set(["first", "second"]), [first, second]);
    const closed = retrieve(new Set(["second"]), [first, second]);

    expect(open.retrieveCandidates).toHaveBeenCalledTimes(2);
    expect(closed.retrieveCandidates).toHaveBeenCalledTimes(1);
    expect(closed.retrieveCandidates).toHaveBeenCalledWith(expect.objectContaining({ currentExperience: second }));
    expect(closed.result.has("first")).toBe(false);
  });

  it("does not retrieve a closed panel when unrelated entries change", () => {
    const current = entry("open");
    const closed = entry("closed");
    const unrelatedChanged = { ...entry("unrelated"), body: "Updated independently" };
    const { result, retrieveCandidates } = retrieve(
      new Set(["open"]),
      [current, closed, unrelatedChanged],
    );

    expect(retrieveCandidates).toHaveBeenCalledTimes(1);
    expect(retrieveCandidates).toHaveBeenCalledWith(expect.objectContaining({ currentExperience: current }));
    expect(result.has("closed")).toBe(false);
    expect(result.has("unrelated")).toBe(false);
  });

  it("uses exact IDs for delimiter-containing and prototype-like panel IDs", () => {
    const plain = entry("a");
    const delimited = entry("a:b");
    const prototypeLike = entry("__proto__");
    const { result, retrieveCandidates } = retrieve(
      new Set(["a:b", "__proto__"]),
      [plain, delimited, prototypeLike],
    );

    expect(retrieveCandidates).toHaveBeenCalledTimes(2);
    expect([...result.keys()]).toEqual(["a:b", "__proto__"]);
  });

  it("reconciles a vanished source on reopen and never reselects it", () => {
    const current = entry("current");
    const source = entry("source");
    const sourceCandidate: HistoricalContextCandidate = {
      sourceExperienceId: source.id,
      sourceCreatedAt: source.createdAt,
      sourceUpdatedAt: source.updatedAt,
      sourceExcerpt: source.body,
      reasons: [{ kind: "shared_visible_terms", terms: ["body"] }],
      confirmedEvidenceIds: [],
      answeredReflectionIds: [],
      ranking: { algorithmVersion: "local-lexical-v1", score: 10, matchedTermCount: 1 },
    };
    let openPanels = openHistoricalContextPanel(new Set(), current.id);
    let selections = toggleHistoricalContextSelection(new Map(), current.id, source.id);

    openPanels = closeHistoricalContextPanel(openPanels, current.id);
    expect(isHistoricalContextSelected(selections, current.id, source.id)).toBe(true);

    openPanels = openHistoricalContextPanel(openPanels, current.id);
    const disappeared = retrieveCandidatesForOpenHistoricalPanels({
      openPanelExperienceIds: openPanels,
      experiences: [current, source],
      artifactsByEntryId: {},
      locale: "en",
      retrieveCandidates: () => [],
    });
    selections = reconcileHistoricalContextSelections(selections, current.id, new Set());
    expect(disappeared.get(current.id)).toEqual([]);
    expect(isHistoricalContextSelected(selections, current.id, source.id)).toBe(false);

    const reappeared = retrieveCandidatesForOpenHistoricalPanels({
      openPanelExperienceIds: openPanels,
      experiences: [current, source],
      artifactsByEntryId: {},
      locale: "en",
      retrieveCandidates: () => [sourceCandidate],
    });
    selections = reconcileHistoricalContextSelections(
      selections,
      current.id,
      new Set(reappeared.get(current.id)?.map((candidate) => candidate.sourceExperienceId)),
    );
    expect(isHistoricalContextSelected(selections, current.id, source.id)).toBe(false);
  });
});
