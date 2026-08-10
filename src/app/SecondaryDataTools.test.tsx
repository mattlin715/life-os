import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { uiText } from "./i18n";
import { SecondaryDataTools } from "./SecondaryDataTools";

describe("SecondaryDataTools", () => {
  it.each(["en", "zh-TW", "ja"] as const)("keeps database diagnostics secondary in %s", (locale) => {
    const html = renderToStaticMarkup(
      <SecondaryDataTools
        copy={uiText[locale]}
        hasEntries
        readinessOpen={false}
        readinessResult={null}
        readinessChecking={false}
        onExportJson={vi.fn()}
        onExportMarkdown={vi.fn()}
        onImportJson={vi.fn()}
        onOpenReadiness={vi.fn()}
        onCheckReadiness={vi.fn()}
        onCloseReadiness={vi.fn()}
      />,
    );
    expect(html).toContain("<details");
    expect(html).toContain(uiText[locale].secondaryToolsSummary);
    expect(html).toContain(uiText[locale].databaseReadinessOpen);
    expect(html).not.toMatch(/>Upgrade<|>Restore<|>Repair<|>Delete</);
  });
});
