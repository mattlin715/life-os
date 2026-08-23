import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { uiText } from "./i18n";
import { FounderSchemaV5MigrationPanel } from "./FounderSchemaV5MigrationPanel";

const state = { state: "migration_required", reason: null, detectedSchemaVersion: 4, supportedSchemaVersion: 5, initializationRequired: false, migrationAvailable: true, backupAvailable: false, restoreAvailable: false, backupRelativePath: null, backupRetentionDays: 30, backupCreatedAt: null, backupExpiresAt: null } as const;

describe("Founder schema-v5 migration disclosure", () => {
  it.each(["en", "zh-TW", "ja"] as const)("shows explicit migrate and cancel controls in %s", (language) => {
    const html = renderToStaticMarkup(<FounderSchemaV5MigrationPanel copy={uiText[language]} state={state} ordinary={false} pending={false} cancelled={false} error={null} onAuthorize={vi.fn()} onCancel={vi.fn()} />);
    expect(html).toContain(uiText[language].founderV5Authorize);
    expect(html).toContain(uiText[language].founderV5Cancel);
    expect(html).toContain(uiText[language].founderV5ProviderBoundary);
  });
  it("makes the cancelled state non-actionable", () => {
    const html = renderToStaticMarkup(<FounderSchemaV5MigrationPanel copy={uiText.en} state={state} ordinary={false} pending={false} cancelled error={null} onAuthorize={vi.fn()} onCancel={vi.fn()} />);
    expect(html).toContain(uiText.en.founderV5Cancelled);
    expect(html.match(/disabled=""/g)).toHaveLength(2);
  });
  it.each(["en", "zh-TW", "ja"] as const)("uses ordinary-profile disclosure in %s", (language) => {
    const copy = uiText[language];
    const html = renderToStaticMarkup(<FounderSchemaV5MigrationPanel copy={copy} state={state} ordinary pending={false} cancelled={false} error={null} onAuthorize={vi.fn()} onCancel={vi.fn()} />);
    expect(html).toContain(copy.ordinaryV5Title);
    expect(html).toContain(copy.ordinaryV5Intro);
    expect(html).toContain(copy.ordinaryV5Purpose);
    expect(html).toContain(copy.ordinaryV5BackupSensitivity);
    expect(html).toContain(copy.ordinaryV5OlderBinaryBoundary);
    expect(html).not.toContain(copy.founderV5Title);
  });
  it.each(["en", "zh-TW", "ja"] as const)("does not change Founder Candidate disclosure in %s", (language) => {
    const copy = uiText[language];
    const html = renderToStaticMarkup(<FounderSchemaV5MigrationPanel copy={copy} state={state} ordinary={false} pending={false} cancelled={false} error={null} onAuthorize={vi.fn()} onCancel={vi.fn()} />);
    expect(html).not.toContain(copy.ordinaryV5Purpose);
    expect(html).not.toContain(copy.ordinaryV5BackupSensitivity);
    expect(html).not.toContain(copy.ordinaryV5OlderBinaryBoundary);
  });
});
