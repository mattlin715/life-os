import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";

import type { DatabaseReadinessResult } from "../shared/storage/sqlite/databaseReadiness";
import { DatabaseReadinessPanel } from "./DatabaseReadinessPanel";
import { uiText } from "./i18n";

const result: DatabaseReadinessResult = {
  classification: "exact_v4",
  databaseExists: true,
  detectedSchemaVersion: 4,
  supportedSchemaVersion: 5,
  walPresent: false,
  shmPresent: false,
  rollbackJournalPresent: false,
  quiescence: "not_proven",
  operationEvidence: "none",
  schemaV5Available: true,
  inspectedAtUnixMs: Date.parse("2026-08-10T00:00:00.000Z"),
};

describe("DatabaseReadinessPanel", () => {
  it.each(["en", "zh-TW", "ja"] as const)(
    "renders equivalent explicit read-only disclosure in %s",
    (locale) => {
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <DatabaseReadinessPanel copy={copy} result={result} checking={false} onCheck={vi.fn()} onClose={vi.fn()} />,
      );
      expect(html).toContain(copy.databaseReadinessTitle);
      expect(html).toContain(copy.databaseReadinessNoAction);
      expect(html).toContain(copy.databaseReadinessExactV4);
      expect(html).toContain(copy.databaseReadinessQuiescenceUnknown);
      expect(html).toContain(copy.databaseReadinessOperationNone);
      expect(html).toContain(copy.databaseReadinessCheck);
    },
  );

  it("renders no result before an explicit check and exposes only check and close controls", () => {
    const html = renderToStaticMarkup(
      <DatabaseReadinessPanel copy={uiText.en} result={null} checking={false} onCheck={vi.fn()} onClose={vi.fn()} />,
    );
    expect(html).not.toContain(uiText.en.databaseReadinessExactV4);
    const labels = [...html.matchAll(/<button[^>]*>(.*?)<\/button>/gu)].map((match) => match[1]);
    expect(labels).toEqual([
      uiText.en.databaseReadinessClose,
      uiText.en.databaseReadinessCheck,
    ]);
    expect(labels.join(" ")).not.toMatch(/Upgrade|Backup|Restore|Delete|Retry migration/iu);
  });

  it.each(["en", "zh-TW", "ja"] as const)(
    "truthfully renders an exact-v5 read-only result in %s",
    (locale) => {
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <DatabaseReadinessPanel
          copy={copy}
          result={{ ...result, classification: "exact_v5", detectedSchemaVersion: 5 }}
          checking={false}
          onCheck={vi.fn()}
          onClose={vi.fn()}
        />,
      );
      expect(html).toContain(copy.databaseReadinessExactV5);
      expect(html).toContain(copy.databaseReadinessNoAction);
      const labels = [...html.matchAll(/<button[^>]*>(.*?)<\/button>/gu)].map((match) => match[1]);
      expect(labels).toEqual([copy.databaseReadinessClose, copy.databaseReadinessCheck]);
      expect(labels.join(" ")).not.toMatch(/Upgrade|Migrate|升級|遷移|移行/iu);
    },
  );

  it("keeps uncertain metadata visibly unknown and offers no recovery action", () => {
    const html = renderToStaticMarkup(
      <DatabaseReadinessPanel
        copy={uiText.en}
        result={{
          ...result,
          classification: "recovery_required",
          databaseExists: null,
          walPresent: null,
          shmPresent: null,
          rollbackJournalPresent: null,
          operationEvidence: "unknown",
        }}
        checking={false}
        onCheck={vi.fn()}
        onClose={vi.fn()}
      />,
    );
    expect(html).toContain(uiText.en.databaseReadinessRecoveryRequired);
    expect(html).toContain(uiText.en.databaseReadinessOperationUnknown);
    expect(html).not.toContain("C:\\");
  });
});
