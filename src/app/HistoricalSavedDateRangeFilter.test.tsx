import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { createHistoricalSavedDateRangeControl } from "../historicalContext/savedDateRange";
import { HistoricalSavedDateRangeFilter } from "./HistoricalSavedDateRangeFilter";
import { uiText } from "./i18n";

describe("HistoricalSavedDateRangeFilter", () => {
  it.each(["en", "zh-TW", "ja"] as const)(
    "renders explicit saved-date controls in %s without implying an event date",
    (locale) => {
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <HistoricalSavedDateRangeFilter
          control={{
            ...createHistoricalSavedDateRangeControl(),
            enabled: true,
            startDate: "2026-07-01",
            endDate: "2026-07-02",
          }}
          message={null}
          copy={copy}
          onEnabledChange={vi.fn()}
          onStartChange={vi.fn()}
          onEndChange={vi.fn()}
          onApply={vi.fn()}
        />,
      );

      expect(html).toContain(copy.historicalSavedDateFilterLabel);
      expect(html).toContain(copy.historicalSavedDateStartLabel);
      expect(html).toContain(copy.historicalSavedDateEndLabel);
      expect(html).toContain('type="date"');
      expect(html.toLocaleLowerCase()).not.toContain("event date");
    },
  );

  it("renders fail-closed feedback as an alert and hides date fields while inactive", () => {
    const inactive = renderToStaticMarkup(
      <HistoricalSavedDateRangeFilter
        control={createHistoricalSavedDateRangeControl()}
        message={null}
        copy={uiText.en}
        onEnabledChange={vi.fn()}
        onStartChange={vi.fn()}
        onEndChange={vi.fn()}
        onApply={vi.fn()}
      />,
    );
    expect(inactive).not.toContain('type="date"');

    const invalid = renderToStaticMarkup(
      <HistoricalSavedDateRangeFilter
        control={{ ...createHistoricalSavedDateRangeControl(), enabled: true }}
        message={uiText.en.historicalSavedDateInverted}
        copy={uiText.en}
        onEnabledChange={vi.fn()}
        onStartChange={vi.fn()}
        onEndChange={vi.fn()}
        onApply={vi.fn()}
      />,
    );
    expect(invalid).toContain('role="alert"');
    expect(invalid).toContain("No history was retrieved");
  });
});
