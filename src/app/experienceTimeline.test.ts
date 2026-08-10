import { describe, expect, it } from "vitest";
import type { ExperienceEntry } from "../types/domain";
import {
  emptyExperienceTimelineFilters,
  filterExperienceTimeline,
  hasExperienceTimelineFilters,
} from "./experienceTimeline";

const entries: ExperienceEntry[] = [
  {
    id: "newest",
    body: "Quiet progress after the meeting",
    createdAt: "2026-08-10T03:00:00.000Z",
    updatedAt: "2026-08-10T03:00:00.000Z",
    userEditable: true,
  },
  {
    id: "older",
    body: "會議後感到安心",
    createdAt: "2026-08-09T03:00:00.000Z",
    updatedAt: "2026-08-09T03:00:00.000Z",
    userEditable: true,
  },
  {
    id: "malformed",
    body: "quiet imported record",
    createdAt: "not-a-date",
    updatedAt: "not-a-date",
    userEditable: true,
  },
];

const utcEnvironment = {
  timeZone: "Etc/UTC",
  localStartOfDay: ({ year, month, day }: { year: number; month: number; day: number }) =>
    Date.UTC(year, month - 1, day),
};

describe("experience timeline filtering", () => {
  it("matches normalized local body text without changing stable order", () => {
    const result = filterExperienceTimeline(entries, {
      ...emptyExperienceTimelineFilters,
      keyword: "QUIET",
    });
    expect(result.status).toBe("valid");
    expect(result.entries.map((entry) => entry.id)).toEqual(["newest", "malformed"]);

    const wide = filterExperienceTimeline(entries, {
      ...emptyExperienceTimelineFilters,
      keyword: "會議",
    });
    expect(wide.entries.map((entry) => entry.id)).toEqual(["older"]);
    expect(entries.map((entry) => entry.id)).toEqual(["newest", "older", "malformed"]);
  });

  it("uses inclusive saved-date boundaries and excludes malformed timestamps", () => {
    const result = filterExperienceTimeline(
      entries,
      { keyword: "", startDate: "2026-08-09", endDate: "2026-08-10" },
      utcEnvironment,
    );
    expect(result.status).toBe("valid");
    expect(result.entries.map((entry) => entry.id)).toEqual(["newest", "older"]);
    if (result.status === "valid") expect(result.rangeTimeZone).toBe("Etc/UTC");
  });

  it("combines keyword and saved-date constraints deterministically", () => {
    const result = filterExperienceTimeline(
      entries,
      { keyword: "quiet", startDate: "2026-08-10", endDate: "2026-08-10" },
      utcEnvironment,
    );
    expect(result.entries.map((entry) => entry.id)).toEqual(["newest"]);
  });

  it("fails closed for incomplete, inverted, and unresolvable ranges", () => {
    expect(
      filterExperienceTimeline(entries, {
        keyword: "",
        startDate: "2026-08-10",
        endDate: "",
      }),
    ).toMatchObject({ status: "blocked", reason: "missing_dates", entries: [] });
    expect(
      filterExperienceTimeline(
        entries,
        { keyword: "", startDate: "2026-08-10", endDate: "2026-08-09" },
        utcEnvironment,
      ),
    ).toMatchObject({ status: "blocked", reason: "inverted_range", entries: [] });
    expect(
      filterExperienceTimeline(
        entries,
        { keyword: "", startDate: "2026-08-10", endDate: "2026-08-10" },
        { ...utcEnvironment, timeZone: null },
      ),
    ).toMatchObject({ status: "blocked", reason: "timezone_unavailable", entries: [] });
  });

  it("clearing filters restores the complete bounded loaded list", () => {
    expect(hasExperienceTimelineFilters(emptyExperienceTimelineFilters)).toBe(false);
    expect(
      filterExperienceTimeline(entries, emptyExperienceTimelineFilters).entries,
    ).toEqual(entries);
  });
});
