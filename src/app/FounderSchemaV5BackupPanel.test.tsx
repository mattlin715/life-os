import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { FounderSchemaV5BackupPanel } from "./FounderSchemaV5BackupPanel";
import { uiText } from "./i18n";

const state = {
  state: "ready", reason: null, detectedSchemaVersion: 5, supportedSchemaVersion: 5,
  initializationRequired: false, migrationAvailable: false, backupAvailable: true,
  restoreAvailable: false,
  backupRelativePath: "life-os-test.operation/backup.db", backupRetentionDays: 30,
  backupCreatedAt: "2026-08-13T00:00:00.000Z", backupExpiresAt: "2026-09-12T00:00:00.000Z",
} as const;

describe("Founder schema-v5 backup disclosure", () => {
  it.each(["en", "zh-TW", "ja"] as const)("discloses exact-owned backup and explicit actions in %s", (language) => {
    const html = renderToStaticMarkup(<FounderSchemaV5BackupPanel copy={uiText[language]} state={state} pending={false} error={false} onDelete={() => {}} onRestore={() => {}} />);
    expect(html).toContain("life-os-test.operation/backup.db");
    expect(html).toContain("2026-09-12");
    expect((html.match(/<button/g) ?? [])).toHaveLength(2);
  });

  it.each(["en", "zh-TW", "ja"] as const)("offers only explicit restore for exact blocked recovery in %s", (language) => {
    const recovery = { ...state, state: "blocked", reason: "post_commit_schema_manifest_mismatch", restoreAvailable: true } as const;
    const html = renderToStaticMarkup(<FounderSchemaV5BackupPanel copy={uiText[language]} state={recovery} pending={false} error={false} onDelete={() => {}} onRestore={() => {}} />);
    expect(html).toContain(uiText[language].founderV5RecoveryRestoreAvailable);
    expect(html).toContain(uiText[language].founderV5RestoreBackup);
    expect(html).not.toContain(uiText[language].founderV5DeleteBackup);
    expect((html.match(/<button/g) ?? [])).toHaveLength(1);
  });
});
