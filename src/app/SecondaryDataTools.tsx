import type { DatabaseReadinessResult } from "../shared/storage/sqlite/databaseReadiness";
import { DatabaseReadinessPanel } from "./DatabaseReadinessPanel";
import type { UiCopy } from "./i18n";

interface SecondaryDataToolsProps {
  readonly copy: UiCopy;
  readonly hasEntries: boolean;
  readonly readinessOpen: boolean;
  readonly readinessResult: DatabaseReadinessResult | null;
  readonly readinessChecking: boolean;
  readonly onExportJson: () => void;
  readonly onExportMarkdown: () => void;
  readonly onImportJson: () => void;
  readonly onOpenReadiness: () => void;
  readonly onCheckReadiness: () => void;
  readonly onCloseReadiness: () => void;
}

export function SecondaryDataTools({
  copy,
  hasEntries,
  readinessOpen,
  readinessResult,
  readinessChecking,
  onExportJson,
  onExportMarkdown,
  onImportJson,
  onOpenReadiness,
  onCheckReadiness,
  onCloseReadiness,
}: SecondaryDataToolsProps) {
  return (
    <details className="secondary-tools">
      <summary>{copy.secondaryToolsSummary}</summary>
      <div className="secondary-tools-body">
        <p className="summary-note">{copy.secondaryToolsIntro}</p>
        <section aria-label={copy.importExportAria}>
          <h2>{copy.portabilityTitle}</h2>
          <div className="utility-row">
            <button type="button" className="ghost-button" disabled={!hasEntries} onClick={onExportJson}>
              {copy.exportJson}
            </button>
            <button type="button" className="ghost-button" disabled={!hasEntries} onClick={onExportMarkdown}>
              {copy.exportMarkdown}
            </button>
            <button type="button" className="ghost-button" onClick={onImportJson}>
              {copy.importJson}
            </button>
          </div>
        </section>
        <section className="secondary-diagnostics" aria-label={copy.diagnosticsTitle}>
          <h2>{copy.diagnosticsTitle}</h2>
          {!readinessOpen ? (
            <button type="button" className="ghost-button" onClick={onOpenReadiness}>
              {copy.databaseReadinessOpen}
            </button>
          ) : (
            <DatabaseReadinessPanel
              copy={copy}
              result={readinessResult}
              checking={readinessChecking}
              onCheck={onCheckReadiness}
              onClose={onCloseReadiness}
            />
          )}
        </section>
      </div>
    </details>
  );
}
