import type { ExperienceEntry } from "../types/domain";
import {
  buildHistoricalSavedDateRange,
  deviceSavedDateRangeEnvironment,
  isTimestampInHistoricalSavedDateRange,
  type HistoricalSavedDateRangeEnvironment,
  type HistoricalSavedDateRangeError,
} from "../historicalContext/savedDateRange";

export interface ExperienceTimelineFilterState {
  readonly keyword: string;
  readonly startDate: string;
  readonly endDate: string;
}

export type ExperienceTimelineFilterResult =
  | {
      readonly status: "valid";
      readonly entries: readonly ExperienceEntry[];
      readonly rangeTimeZone: string | null;
    }
  | {
      readonly status: "blocked";
      readonly entries: readonly [];
      readonly reason: HistoricalSavedDateRangeError;
    };

export const emptyExperienceTimelineFilters: ExperienceTimelineFilterState = {
  keyword: "",
  startDate: "",
  endDate: "",
};

export function hasExperienceTimelineFilters(
  filters: ExperienceTimelineFilterState,
): boolean {
  return Boolean(
    filters.keyword.trim() || filters.startDate || filters.endDate,
  );
}

function normalizeKeyword(value: string): string {
  return value.normalize("NFKC").toLocaleLowerCase("en-US").trim();
}

export function filterExperienceTimeline(
  entries: readonly ExperienceEntry[],
  filters: ExperienceTimelineFilterState,
  environment: HistoricalSavedDateRangeEnvironment =
    deviceSavedDateRangeEnvironment(),
): ExperienceTimelineFilterResult {
  const keyword = normalizeKeyword(filters.keyword);
  const hasStart = Boolean(filters.startDate);
  const hasEnd = Boolean(filters.endDate);

  if (hasStart !== hasEnd) {
    return { status: "blocked", entries: [], reason: "missing_dates" };
  }

  const rangeResult = hasStart
    ? buildHistoricalSavedDateRange(
        filters.startDate,
        filters.endDate,
        environment,
      )
    : null;
  if (rangeResult?.status === "invalid") {
    return { status: "blocked", entries: [], reason: rangeResult.reason };
  }
  const range = rangeResult?.status === "valid" ? rangeResult.range : null;

  return {
    status: "valid",
    entries: entries.filter((entry) => {
      const keywordMatches =
        !keyword || normalizeKeyword(entry.body).includes(keyword);
      const dateMatches =
        !range ||
        isTimestampInHistoricalSavedDateRange(entry.createdAt, range);
      return keywordMatches && dateMatches;
    }),
    rangeTimeZone: range?.timeZone ?? null,
  };
}
