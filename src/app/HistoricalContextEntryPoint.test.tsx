import { Children, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { HistoricalContextEntryPoint } from "./HistoricalContextEntryPoint";
import { uiText } from "./i18n";

describe("HistoricalContextEntryPoint", () => {
  it.each(["en", "zh-TW", "ja"] as const)(
    "renders an explicit, localized, collapsed-panel shortcut in %s",
    (language) => {
      const html = renderToStaticMarkup(
        <HistoricalContextEntryPoint
          copy={uiText[language]}
          controlsId="history-current"
          isOpen={false}
          onOpen={() => undefined}
        />,
      );

      expect(html).toContain(uiText[language].historicalContextShortcutTitle);
      expect(html).toContain(uiText[language].historicalContextShortcutBody);
      expect(html).toContain(uiText[language].historicalContextShortcutOpen);
      expect(html).toContain('aria-controls="history-current"');
      expect(html).toContain('aria-expanded="false"');
    },
  );

  it("does nothing until the user activates the shortcut", () => {
    const onOpen = vi.fn();
    const element = HistoricalContextEntryPoint({
      copy: uiText.en,
      controlsId: "history-current",
      isOpen: false,
      onOpen,
    });

    expect(onOpen).not.toHaveBeenCalled();
    const button = Children.toArray(element.props.children)[1] as ReactElement<{
      onClick: () => void;
    }>;
    button.props.onClick();
    expect(onOpen).toHaveBeenCalledTimes(1);
  });

  it("labels an already-open panel as a view action rather than a close action", () => {
    const html = renderToStaticMarkup(
      <HistoricalContextEntryPoint
        copy={uiText["zh-TW"]}
        controlsId="history-current"
        isOpen
        onOpen={() => undefined}
      />,
    );

    expect(html).toContain(uiText["zh-TW"].historicalContextShortcutView);
    expect(html).toContain('aria-expanded="true"');
  });
});
