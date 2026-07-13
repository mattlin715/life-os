import { describe, expect, it } from "vitest";
import {
  formatExperienceTimestamp,
  formatHistoricalSourceDate,
  isValidTimestamp,
  toSafeHtmlDateTime,
} from "./date";

const sourceTimestamp = "2026-07-13T23:30:00.000Z";

describe("strict persisted timestamp contract", () => {
  it.each([
    "2026-07-14T08:30:00Z",
    "2026-07-14T08:30:00.000Z",
    "2026-07-14T08:30:00+09:00",
    "2026-07-14T08:30:00.1+09:00",
    "2026-07-14T08:30:00.12+09:00",
    "2026-07-14T08:30:00.123+09:00",
    "2026-07-14T08:30:00.1234Z",
    "0001-01-01T00:00:00Z",
  ])("accepts a complete ISO/RFC3339 instant: %s", (timestamp) => {
    expect(isValidTimestamp(timestamp)).toBe(true);
  });

  it.each([
    "July 14, 2026",
    "07/14/2026",
    "0",
    "2026-07-14",
    "2026-07-14T08:30:00",
    " 2026-07-14T08:30:00Z",
    "2026-07-14T08:30:00Z ",
    "not-a-valid-timestamp",
    "0000-01-01T00:00:00Z",
    "2026-02-30T08:30:00Z",
    "2025-02-29T08:30:00Z",
    "2026-07-14T24:00:00Z",
    "2026-07-14T08:60:00Z",
    "2026-07-14T08:30:60Z",
    "2026-07-14T08:30:00+24:00",
    "2026-07-14T08:30:00+09:60",
    "",
    "  ",
  ])("rejects malformed, incomplete, or impossible source values: %s", (timestamp) => {
    expect(isValidTimestamp(timestamp)).toBe(false);
  });
});

describe("HTML datetime serialization", () => {
  it("normalizes a numeric-offset source without changing the stored source string", () => {
    const source = "2026-07-14T08:30:00+09:00";

    expect(toSafeHtmlDateTime(source)).toBe("2026-07-13T23:30:00.000Z");
    expect(toSafeHtmlDateTime(source)).not.toContain(source);
  });

  it("normalizes supported long fractional precision to milliseconds for HTML", () => {
    expect(toSafeHtmlDateTime("2026-01-01T00:00:00.1234Z")).toBe(
      "2026-01-01T00:00:00.123Z",
    );
  });

  it.each([
    "0001-01-01T00:00:00+14:00",
    "9999-12-31T23:59:59-14:00",
  ])("keeps a governed boundary source presentation-valid without an unsafe HTML year: %s", (source) => {
    expect(isValidTimestamp(source)).toBe(true);
    expect(
      formatExperienceTimestamp(source, "en", "Date unavailable", { timeZone: "UTC" }),
    ).not.toBe("Date unavailable");
    expect(formatHistoricalSourceDate(source, "en", undefined, { timeZone: "UTC" })).not.toBe(
      "Date unavailable",
    );
    expect(toSafeHtmlDateTime(source)).toBeNull();
  });

  it.each([
    "0000-01-01T00:00:00Z",
    "not-a-valid-timestamp",
    "2026-07-14",
  ])("does not provide HTML datetime for unsupported source: %s", (timestamp) => {
    expect(toSafeHtmlDateTime(timestamp)).toBeNull();
  });
});

describe("timestamp formatting", () => {
  it("uses an injected UTC timezone for deterministic English historical and Experience dates", () => {
    expect(formatHistoricalSourceDate(sourceTimestamp, "en", undefined, { timeZone: "UTC" })).toBe(
      "Jul 13, 2026",
    );
    expect(
      formatExperienceTimestamp(sourceTimestamp, "en", "Date unavailable", { timeZone: "UTC" }),
    ).toBe("Jul 13, 2026, 11:30 PM");
  });

  it("uses Asia/Tokyo rather than UTC for the same source instant", () => {
    expect(
      formatHistoricalSourceDate(sourceTimestamp, "en", undefined, { timeZone: "Asia/Tokyo" }),
    ).toBe("Jul 14, 2026");
    expect(
      formatExperienceTimestamp(sourceTimestamp, "en", "Date unavailable", {
        timeZone: "Asia/Tokyo",
      }),
    ).toBe("Jul 14, 2026, 8:30 AM");
  });

  it("uses explicit Traditional Chinese locale formatting independently from the host locale", () => {
    expect(
      formatExperienceTimestamp(sourceTimestamp, "zh-TW", "日期無法使用", {
        timeZone: "Asia/Tokyo",
      }),
    ).toBe("2026年7月14日 上午8:30");
    expect(
      formatHistoricalSourceDate(sourceTimestamp, "zh-TW", undefined, { timeZone: "Asia/Tokyo" }),
    ).toBe("2026年7月14日");
  });

  it("uses explicit Japanese locale formatting independently from the host locale", () => {
    expect(
      formatExperienceTimestamp(sourceTimestamp, "ja", "日付を表示できません", {
        timeZone: "Asia/Tokyo",
      }),
    ).toBe("2026/07/14 8:30");
    expect(
      formatHistoricalSourceDate(sourceTimestamp, "ja", undefined, { timeZone: "Asia/Tokyo" }),
    ).toBe("2026/07/14");
  });

  it("uses localized fallbacks rather than raw invalid source strings", () => {
    expect(formatHistoricalSourceDate("not-a-valid-timestamp", "en")).toBe("Date unavailable");
    expect(formatHistoricalSourceDate("", "zh-TW")).toBe("日期無法使用");
    expect(formatHistoricalSourceDate("  ", "ja")).toBe("日付を表示できません");
    expect(formatExperienceTimestamp("not-a-valid-timestamp", "en", "Date unavailable")).toBe(
      "Date unavailable",
    );
    expect(formatExperienceTimestamp("", "zh-TW", "日期無法使用")).toBe("日期無法使用");
    expect(formatExperienceTimestamp("  ", "ja", "日付を表示できません")).toBe(
      "日付を表示できません",
    );
  });
});
