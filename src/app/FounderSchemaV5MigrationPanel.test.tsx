import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { uiText } from "./i18n";
import { FounderSchemaV5MigrationPanel } from "./FounderSchemaV5MigrationPanel";

const state = { state: "migration_required", reason: null, detectedSchemaVersion: 4, supportedSchemaVersion: 5, initializationRequired: false, migrationAvailable: true, backupAvailable: false, restoreAvailable: false, backupRelativePath: null, backupRetentionDays: 30, backupCreatedAt: null, backupExpiresAt: null } as const;

describe("Founder schema-v5 migration disclosure", () => {
  it.each(["en", "zh-TW", "ja"] as const)("shows explicit migrate and cancel controls in %s", (language) => {
    const html = renderToStaticMarkup(<FounderSchemaV5MigrationPanel copy={uiText[language]} state={state} pending={false} cancelled={false} error={null} onAuthorize={vi.fn()} onCancel={vi.fn()} />);
    expect(html).toContain(uiText[language].founderV5Authorize);
    expect(html).toContain(uiText[language].founderV5Cancel);
    expect(html).toContain(uiText[language].founderV5ProviderBoundary);
  });
  it("makes the cancelled state non-actionable", () => {
    const html = renderToStaticMarkup(<FounderSchemaV5MigrationPanel copy={uiText.en} state={state} pending={false} cancelled error={null} onAuthorize={vi.fn()} onCancel={vi.fn()} />);
    expect(html).toContain(uiText.en.founderV5Cancelled);
    expect(html.match(/disabled=""/g)).toHaveLength(2);
  });
});
