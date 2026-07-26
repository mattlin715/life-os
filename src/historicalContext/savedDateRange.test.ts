import { describe, expect, it } from "vitest";
import {
  applyHistoricalSavedDateRange,
  buildHistoricalSavedDateRange,
  createHistoricalSavedDateRangeControl,
  getHistoricalSavedDateRangeControl,
  historicalSavedDateRangeConstraint,
  isTimestampInHistoricalSavedDateRange,
  parseCalendarDate,
  removeHistoricalSavedDateRangeControl,
  setHistoricalSavedDateRangeEnabled,
  setHistoricalSavedDateRangeEnd,
  setHistoricalSavedDateRangeStart,
  shouldCloseHistoricalPreflightForSavedDateRangeChange,
  type CalendarDate,
  type HistoricalSavedDateRangeControls,
  type HistoricalSavedDateRangeEnvironment,
} from "./savedDateRange";

function utcEnvironment(
  timeZone = "UTC",
  boundary?: (date: CalendarDate) => number | null,
): HistoricalSavedDateRangeEnvironment {
  return {
    timeZone,
    localStartOfDay:
      boundary ??
      ((date) => {
        const value = new Date(0);
        value.setUTCFullYear(date.year, date.month - 1, date.day);
        value.setUTCHours(0, 0, 0, 0);
        return value.getTime();
      }),
  };
}

describe("historical saved-date range", () => {
  it("accepts exact calendar dates and rejects impossible or host-locale values", () => {
    expect(parseCalendarDate("2024-02-29")).toEqual({ year: 2024, month: 2, day: 29 });
    for (const value of ["2023-02-29", "2026-13-01", "2026-01-00", "07/26/2026", " 2026-07-26"]) {
      expect(parseCalendarDate(value)).toBeNull();
    }
  });

  it("uses inclusive calendar dates represented by an exclusive next-day boundary", () => {
    const result = buildHistoricalSavedDateRange(
      "2026-07-10",
      "2026-07-10",
      utcEnvironment(),
    );
    expect(result.status).toBe("valid");
    if (result.status !== "valid") return;

    expect(isTimestampInHistoricalSavedDateRange("2026-07-10T00:00:00.000Z", result.range)).toBe(true);
    expect(isTimestampInHistoricalSavedDateRange("2026-07-10T23:59:59.999Z", result.range)).toBe(true);
    expect(isTimestampInHistoricalSavedDateRange("2026-07-11T00:00:00.000Z", result.range)).toBe(false);
  });

  it("captures timezone and supports injected 23-hour and 25-hour local days", () => {
    const boundaries = new Map([
      ["2026-03-29", 1000],
      ["2026-03-30", 1000 + 23 * 60 * 60 * 1000],
      ["2026-10-25", 2000],
      ["2026-10-26", 2000 + 25 * 60 * 60 * 1000],
    ]);
    const environment = utcEnvironment("Test/DST", (date) =>
      boundaries.get(
        `${date.year}-${String(date.month).padStart(2, "0")}-${String(date.day).padStart(2, "0")}`,
      ) ?? null,
    );

    const spring = buildHistoricalSavedDateRange("2026-03-29", "2026-03-29", environment);
    const autumn = buildHistoricalSavedDateRange("2026-10-25", "2026-10-25", environment);
    expect(spring.status === "valid" ? spring.range.endExclusiveMs - spring.range.startInclusiveMs : null).toBe(23 * 60 * 60 * 1000);
    expect(autumn.status === "valid" ? autumn.range.endExclusiveMs - autumn.range.startInclusiveMs : null).toBe(25 * 60 * 60 * 1000);
    expect(spring.status === "valid" ? spring.range.timeZone : null).toBe("Test/DST");
  });

  it("fails closed for missing, invalid, inverted, unavailable, or unresolved boundaries", () => {
    expect(buildHistoricalSavedDateRange("", "2026-07-10", utcEnvironment())).toEqual({
      status: "invalid",
      reason: "missing_dates",
    });
    expect(buildHistoricalSavedDateRange("2026-02-30", "2026-03-01", utcEnvironment())).toEqual({
      status: "invalid",
      reason: "invalid_date",
    });
    expect(buildHistoricalSavedDateRange("2026-07-11", "2026-07-10", utcEnvironment())).toEqual({
      status: "invalid",
      reason: "inverted_range",
    });
    expect(buildHistoricalSavedDateRange("2026-07-10", "2026-07-10", utcEnvironment(""))).toEqual({
      status: "invalid",
      reason: "timezone_unavailable",
    });
    expect(buildHistoricalSavedDateRange("2026-07-10", "2026-07-10", utcEnvironment("UTC", () => null))).toEqual({
      status: "invalid",
      reason: "timezone_unavailable",
    });
  });

  it("excludes malformed source timestamps rather than guessing", () => {
    const result = buildHistoricalSavedDateRange("2026-07-10", "2026-07-10", utcEnvironment());
    expect(result.status).toBe("valid");
    if (result.status !== "valid") return;
    expect(isTimestampInHistoricalSavedDateRange("July 10, 2026", result.range)).toBe(false);
    expect(isTimestampInHistoricalSavedDateRange("", result.range)).toBe(false);
  });

  it("keeps controls immutable and requires explicit Apply after every change", () => {
    let controls: HistoricalSavedDateRangeControls = new Map();
    const original = controls;
    controls = setHistoricalSavedDateRangeEnabled(controls, "current", true);
    controls = setHistoricalSavedDateRangeStart(controls, "current", "2026-07-10");
    controls = setHistoricalSavedDateRangeEnd(controls, "current", "2026-07-12");
    expect(original.size).toBe(0);
    expect(historicalSavedDateRangeConstraint(getHistoricalSavedDateRangeControl(controls, "current"))).toEqual({
      status: "blocked",
      reason: "not_applied",
    });

    controls = applyHistoricalSavedDateRange(controls, "current", utcEnvironment());
    expect(historicalSavedDateRangeConstraint(getHistoricalSavedDateRangeControl(controls, "current")).status).toBe("applied");

    controls = setHistoricalSavedDateRangeStart(controls, "current", "2026-07-11");
    expect(getHistoricalSavedDateRangeControl(controls, "current").appliedRange).toBeNull();
    expect(historicalSavedDateRangeConstraint(getHistoricalSavedDateRangeControl(controls, "current"))).toEqual({
      status: "blocked",
      reason: "not_applied",
    });
  });

  it("stores visible errors, disables without an implicit range, and removes exact controls", () => {
    let controls: HistoricalSavedDateRangeControls = new Map([
      ["other", createHistoricalSavedDateRangeControl()],
    ]);
    controls = setHistoricalSavedDateRangeEnabled(controls, "current", true);
    controls = applyHistoricalSavedDateRange(controls, "current", utcEnvironment());
    expect(getHistoricalSavedDateRangeControl(controls, "current").error).toBe("missing_dates");

    controls = setHistoricalSavedDateRangeEnabled(controls, "current", false);
    expect(historicalSavedDateRangeConstraint(getHistoricalSavedDateRangeControl(controls, "current"))).toEqual({
      status: "inactive",
    });

    const removed = removeHistoricalSavedDateRangeControl(controls, "current");
    expect(removed.has("current")).toBe(false);
    expect(removed.has("other")).toBe(true);
  });

  it("closes only the preflight owned by the current Experience whose range changed", () => {
    expect(shouldCloseHistoricalPreflightForSavedDateRangeChange("current", "current")).toBe(true);
    expect(shouldCloseHistoricalPreflightForSavedDateRangeChange("other", "current")).toBe(false);
    expect(shouldCloseHistoricalPreflightForSavedDateRangeChange(null, "current")).toBe(false);
  });
});
