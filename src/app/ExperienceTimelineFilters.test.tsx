import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { ExperienceTimelineFilters } from "./ExperienceTimelineFilters";
import { emptyExperienceTimelineFilters } from "./experienceTimeline";
import { uiText } from "./i18n";

describe("ExperienceTimelineFilters", () => {
  it.each(["en", "zh-TW", "ja"] as const)(
    "renders local keyword, saved-date, clear, count, and empty-result controls in %s",
    (locale) => {
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <ExperienceTimelineFilters
          copy={copy}
          filters={{ ...emptyExperienceTimelineFilters, keyword: "missing" }}
          resultCount={0}
          totalCount={3}
          error={null}
          rangeTimeZone={null}
          onKeywordChange={vi.fn()}
          onStartDateChange={vi.fn()}
          onEndDateChange={vi.fn()}
          onClear={vi.fn()}
        />,
      );
      expect(html).toContain('type="search"');
      expect(html.match(/type="date"/gu)).toHaveLength(2);
      expect(html).toContain(copy.timelineFilterClear);
      expect(html).toContain(copy.timelineResultCount(0, 3));
      expect(html).toContain(copy.timelineNoResults);
      expect(html).toContain(copy.timelineSavedDateSemantics);
    },
  );

  it("renders invalid range feedback as fail-closed", () => {
    const html = renderToStaticMarkup(
      <ExperienceTimelineFilters
        copy={uiText.en}
        filters={{ keyword: "", startDate: "2026-08-10", endDate: "2026-08-09" }}
        resultCount={0}
        totalCount={3}
        error="inverted_range"
        rangeTimeZone={null}
        onKeywordChange={vi.fn()}
        onStartDateChange={vi.fn()}
        onEndDateChange={vi.fn()}
        onClear={vi.fn()}
      />,
    );
    expect(html).toContain('role="alert"');
    expect(html).toContain(uiText.en.timelineFilterInvertedDate);
  });
});
