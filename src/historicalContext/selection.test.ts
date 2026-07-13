import { describe, expect, it } from "vitest";
import {
  clearHistoricalContextSelection,
  isHistoricalContextSelected,
  reconcileHistoricalContextSelectionsAgainstEligibleCandidates,
  reconcileHistoricalContextSelections,
  removeHistoricalContextSelectionsForExperience,
  toggleHistoricalContextSelection,
} from "./selection";

describe("historical context selection", () => {
  it("keeps delimiter-containing and reversed exact IDs independent", () => {
    let selections = toggleHistoricalContextSelection(new Map(), "a", "a:b");
    selections = toggleHistoricalContextSelection(selections, "a:b", "a");

    expect(isHistoricalContextSelected(selections, "a", "a:b")).toBe(true);
    expect(isHistoricalContextSelected(selections, "a:b", "a")).toBe(true);

    const cleared = clearHistoricalContextSelection(selections, "a");
    expect(isHistoricalContextSelected(cleared, "a", "a:b")).toBe(false);
    expect(isHistoricalContextSelected(cleared, "a:b", "a")).toBe(true);
  });

  it.each(["__proto__", "constructor", "prototype", "toString"])(
    "supports the prototype-like exact ID %s in either key role",
    (id) => {
      let selections = toggleHistoricalContextSelection(new Map(), id, "ordinary-source");
      selections = toggleHistoricalContextSelection(selections, "ordinary-current", id);

      expect(isHistoricalContextSelected(selections, id, "ordinary-source")).toBe(true);
      expect(isHistoricalContextSelected(selections, "ordinary-current", id)).toBe(true);
    },
  );

  it("cleans an Experience from exact current and source roles without prefix collisions", () => {
    let selections = toggleHistoricalContextSelection(new Map(), "a", "a:b");
    selections = toggleHistoricalContextSelection(selections, "a:b", "a");
    selections = toggleHistoricalContextSelection(selections, "other", "a:b");

    const cleaned = removeHistoricalContextSelectionsForExperience(selections, "a");

    expect(isHistoricalContextSelected(cleaned, "a", "a:b")).toBe(false);
    expect(isHistoricalContextSelected(cleaned, "a:b", "a")).toBe(false);
    expect(isHistoricalContextSelected(cleaned, "other", "a:b")).toBe(true);
    expect(cleaned.has("a")).toBe(false);
  });

  it("reconciles vanished candidates for only their current Experience", () => {
    let selections = toggleHistoricalContextSelection(new Map(), "current", "rejected-evidence-source");
    selections = toggleHistoricalContextSelection(selections, "current", "still-eligible");
    selections = toggleHistoricalContextSelection(selections, "unrelated-current", "unrelated-source");

    const afterRejectedEvidence = reconcileHistoricalContextSelections(
      selections,
      "current",
      new Set(["still-eligible"]),
    );
    expect(isHistoricalContextSelected(afterRejectedEvidence, "current", "rejected-evidence-source")).toBe(false);
    expect(isHistoricalContextSelected(afterRejectedEvidence, "current", "still-eligible")).toBe(true);
    expect(isHistoricalContextSelected(afterRejectedEvidence, "unrelated-current", "unrelated-source")).toBe(true);

    const afterSourceEditDropsTopN = reconcileHistoricalContextSelections(
      afterRejectedEvidence,
      "current",
      new Set(),
    );
    expect(afterSourceEditDropsTopN.has("current")).toBe(false);

    const afterReappearance = reconcileHistoricalContextSelections(
      afterSourceEditDropsTopN,
      "current",
      new Set(["rejected-evidence-source", "still-eligible"]),
    );
    expect(afterReappearance).toBe(afterSourceEditDropsTopN);
    expect(isHistoricalContextSelected(afterReappearance, "current", "rejected-evidence-source")).toBe(false);
    expect(isHistoricalContextSelected(afterReappearance, "current", "still-eligible")).toBe(false);
  });

  it("returns its original reference when reconciliation finds no stale selection", () => {
    const selections = toggleHistoricalContextSelection(new Map(), "current", "source");
    expect(reconcileHistoricalContextSelections(selections, "current", new Set(["source"]))).toBe(selections);
    expect(reconcileHistoricalContextSelections(selections, "other", new Set())).toBe(selections);
  });

  it("reconciles all currents, including a deleted current Experience, without reselecting a returning source", () => {
    let selections = toggleHistoricalContextSelection(new Map(), "current", "source-edited-away");
    selections = toggleHistoricalContextSelection(selections, "current", "keep");
    selections = toggleHistoricalContextSelection(selections, "deleted-current", "source");
    selections = toggleHistoricalContextSelection(selections, "unrelated-current", "unrelated-source");

    const reconciled = reconcileHistoricalContextSelectionsAgainstEligibleCandidates(
      selections,
      new Map([
        ["current", new Set(["keep"])],
        ["unrelated-current", new Set(["unrelated-source"])],
      ]),
    );

    expect(isHistoricalContextSelected(reconciled, "current", "source-edited-away")).toBe(false);
    expect(isHistoricalContextSelected(reconciled, "current", "keep")).toBe(true);
    expect(reconciled.has("deleted-current")).toBe(false);
    expect(isHistoricalContextSelected(reconciled, "unrelated-current", "unrelated-source")).toBe(true);

    const sourceReturns = reconcileHistoricalContextSelectionsAgainstEligibleCandidates(
      reconciled,
      new Map([
        ["current", new Set(["source-edited-away", "keep"])],
        ["unrelated-current", new Set(["unrelated-source"])],
      ]),
    );
    expect(sourceReturns).toBe(reconciled);
    expect(isHistoricalContextSelected(sourceReturns, "current", "source-edited-away")).toBe(false);
  });
});
