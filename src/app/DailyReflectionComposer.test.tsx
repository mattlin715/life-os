import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { DailyReflectionComposer } from "./DailyReflectionComposer";
import { uiText } from "./i18n";

describe("DailyReflectionComposer", () => {
  it.each(["en", "zh-TW", "ja"] as const)(
    "keeps Experience entry as the explicit primary action in %s",
    (locale) => {
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <DailyReflectionComposer
          copy={copy}
          body=""
          saving={false}
          providerDetail={copy.aiFallbackDetail}
          onBodyChange={vi.fn()}
          onSave={vi.fn()}
        />,
      );
      expect(html).toContain(copy.hero);
      expect(html).toContain(copy.experienceAria);
      expect(html).toContain(copy.saveMoment);
      expect(html).toContain('id="daily-reflection-composer"');
      expect(html).not.toContain(copy.databaseReadinessOpen);
    },
  );
});
