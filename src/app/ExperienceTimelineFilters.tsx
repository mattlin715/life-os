import type { HistoricalSavedDateRangeError } from "../historicalContext/savedDateRange";
import type { UiCopy } from "./i18n";
import {
  hasExperienceTimelineFilters,
  type ExperienceTimelineFilterState,
} from "./experienceTimeline";

interface ExperienceTimelineFiltersProps {
  readonly copy: UiCopy;
  readonly filters: ExperienceTimelineFilterState;
  readonly resultCount: number;
  readonly totalCount: number;
  readonly error: HistoricalSavedDateRangeError | null;
  readonly rangeTimeZone: string | null;
  readonly onKeywordChange: (value: string) => void;
  readonly onStartDateChange: (value: string) => void;
  readonly onEndDateChange: (value: string) => void;
  readonly onClear: () => void;
}

function errorMessage(
  error: HistoricalSavedDateRangeError,
  copy: UiCopy,
): string {
  switch (error) {
    case "missing_dates": return copy.timelineFilterMissingDates;
    case "invalid_date": return copy.timelineFilterInvalidDate;
    case "inverted_range": return copy.timelineFilterInvertedDate;
    case "timezone_unavailable": return copy.timelineFilterTimezoneUnavailable;
  }
}

export function ExperienceTimelineFilters({
  copy,
  filters,
  resultCount,
  totalCount,
  error,
  rangeTimeZone,
  onKeywordChange,
  onStartDateChange,
  onEndDateChange,
  onClear,
}: ExperienceTimelineFiltersProps) {
  return (
    <section className="timeline-filters" aria-labelledby="timeline-filter-heading">
      <div className="timeline-filter-heading">
        <div>
          <p className="soft-label">{copy.timelineFilterKicker}</p>
          <h2 id="timeline-filter-heading">{copy.timelineFilterTitle}</h2>
        </div>
        <button
          type="button"
          className="ghost-button compact"
          disabled={!hasExperienceTimelineFilters(filters)}
          onClick={onClear}
        >
          {copy.timelineFilterClear}
        </button>
      </div>
      <div className="timeline-filter-controls">
        <label>
          <span>{copy.timelineKeywordLabel}</span>
          <input
            type="search"
            value={filters.keyword}
            placeholder={copy.timelineKeywordPlaceholder}
            onChange={(event) => onKeywordChange(event.target.value)}
          />
        </label>
        <label>
          <span>{copy.timelineSavedFrom}</span>
          <input
            type="date"
            value={filters.startDate}
            onChange={(event) => onStartDateChange(event.target.value)}
          />
        </label>
        <label>
          <span>{copy.timelineSavedThrough}</span>
          <input
            type="date"
            value={filters.endDate}
            onChange={(event) => onEndDateChange(event.target.value)}
          />
        </label>
      </div>
      <p className="timeline-filter-note">
        {rangeTimeZone
          ? copy.timelineSavedDateSemanticsWithZone(rangeTimeZone)
          : copy.timelineSavedDateSemantics}
      </p>
      {error ? <p className="timeline-filter-error" role="alert">{errorMessage(error, copy)}</p> : null}
      <output className="timeline-filter-count" aria-live="polite">
        {copy.timelineResultCount(resultCount, totalCount)}
      </output>
      {!error && resultCount === 0 ? (
        <p className="timeline-filter-empty">{copy.timelineNoResults}</p>
      ) : null}
    </section>
  );
}
