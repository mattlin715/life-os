import { describe, expect, it } from "vitest";
import {
  closeHistoricalContextPanel,
  isHistoricalContextPanelOpen,
  openHistoricalContextPanel,
  removeHistoricalContextPanelForExperience,
  toggleHistoricalContextPanel,
} from "./panelState";

describe("historical context panel state", () => {
  it("keeps delimiter-containing exact IDs independent", () => {
    let openPanels = openHistoricalContextPanel(new Set(), "a");
    openPanels = openHistoricalContextPanel(openPanels, "a:b");

    const afterClosingA = closeHistoricalContextPanel(openPanels, "a");
    expect(isHistoricalContextPanelOpen(afterClosingA, "a")).toBe(false);
    expect(isHistoricalContextPanelOpen(afterClosingA, "a:b")).toBe(true);
  });

  it.each(["__proto__", "constructor", "prototype", "toString"])(
    "opens and closes the prototype-like exact ID %s normally",
    (id) => {
      const opened = openHistoricalContextPanel(new Set(), id);
      expect(isHistoricalContextPanelOpen(opened, id)).toBe(true);

      const closed = closeHistoricalContextPanel(opened, id);
      expect(isHistoricalContextPanelOpen(closed, id)).toBe(false);
    },
  );

  it("toggles only the requested exact Experience ID", () => {
    let openPanels = openHistoricalContextPanel(new Set(), "a");
    openPanels = openHistoricalContextPanel(openPanels, "a:b");

    const toggled = toggleHistoricalContextPanel(openPanels, "a:b");
    expect(isHistoricalContextPanelOpen(toggled, "a")).toBe(true);
    expect(isHistoricalContextPanelOpen(toggled, "a:b")).toBe(false);
  });

  it("removes open-panel state when the exact Experience is deleted", () => {
    let openPanels = openHistoricalContextPanel(new Set(), "deleted");
    openPanels = openHistoricalContextPanel(openPanels, "remaining");

    const cleaned = removeHistoricalContextPanelForExperience(openPanels, "deleted");
    expect(isHistoricalContextPanelOpen(cleaned, "deleted")).toBe(false);
    expect(isHistoricalContextPanelOpen(cleaned, "remaining")).toBe(true);
  });

  it("uses immutable updates and preserves the reference for no-op changes", () => {
    const empty = new Set<string>();
    const opened = openHistoricalContextPanel(empty, "experience");
    expect(opened).not.toBe(empty);
    expect(openHistoricalContextPanel(opened, "experience")).toBe(opened);

    const closed = closeHistoricalContextPanel(opened, "experience");
    expect(closed).not.toBe(opened);
    expect(closeHistoricalContextPanel(closed, "experience")).toBe(closed);
  });
});
