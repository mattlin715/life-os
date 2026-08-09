import type { DatabaseReadinessResult } from "../shared/storage/sqlite/databaseReadiness";
import type { UiCopy } from "./i18n";

interface DatabaseReadinessPanelProps {
  readonly copy: UiCopy;
  readonly result: DatabaseReadinessResult | null;
  readonly checking: boolean;
  readonly onCheck: () => void;
  readonly onClose: () => void;
}

function boundedBoolean(
  value: boolean | null,
  copy: UiCopy,
): string {
  if (value === true) return copy.databaseReadinessYes;
  if (value === false) return copy.databaseReadinessNo;
  return copy.databaseReadinessUnknown;
}

function classificationMessage(result: DatabaseReadinessResult, copy: UiCopy): string {
  switch (result.classification) {
    case "missing": return copy.databaseReadinessMissing;
    case "older_supported": return copy.databaseReadinessOlder;
    case "exact_v4": return copy.databaseReadinessExactV4;
    case "newer_unsupported": return copy.databaseReadinessNewer;
    case "malformed": return copy.databaseReadinessMalformed;
    case "path_unsafe": return copy.databaseReadinessPathUnsafe;
    case "recovery_required": return copy.databaseReadinessRecoveryRequired;
    default: return copy.databaseReadinessUnreadable;
  }
}

function operationMessage(result: DatabaseReadinessResult, copy: UiCopy): string {
  if (result.operationEvidence === "none") return copy.databaseReadinessOperationNone;
  if (result.operationEvidence === "present") return copy.databaseReadinessOperationPresent;
  return copy.databaseReadinessOperationUnknown;
}

function Field({ label, value }: { readonly label: string; readonly value: string }) {
  return <div><dt>{label}</dt><dd>{value}</dd></div>;
}

export function DatabaseReadinessPanel({
  copy,
  result,
  checking,
  onCheck,
  onClose,
}: DatabaseReadinessPanelProps) {
  return (
    <section className="database-readiness-panel" aria-label={copy.databaseReadinessTitle}>
      <div className="database-readiness-heading">
        <div>
          <p className="soft-label">{copy.databaseLocalLabel}</p>
          <h2>{copy.databaseReadinessTitle}</h2>
        </div>
        <button type="button" className="ghost-button compact" onClick={onClose}>
          {copy.databaseReadinessClose}
        </button>
      </div>
      <p>{copy.databaseReadinessIntro}</p>
      <p className="summary-note">{copy.databaseReadinessNoAction}</p>
      <button
        type="button"
        className="primary-button"
        disabled={checking}
        onClick={onCheck}
      >
        {checking ? copy.databaseReadinessChecking : copy.databaseReadinessCheck}
      </button>
      {result ? (
        <div className="database-readiness-result" role="status" aria-live="polite">
          <h3>{copy.databaseReadinessClassification}</h3>
          <p>{classificationMessage(result, copy)}</p>
          <p className="summary-note">
            {copy.databaseReadinessCheckedAt(new Date(result.inspectedAtUnixMs).toISOString())}
          </p>
          <dl className="database-readiness-fields">
            <Field label={copy.databaseReadinessExists} value={boundedBoolean(result.databaseExists, copy)} />
            <Field
              label={copy.databaseReadinessDetectedVersion}
              value={result.detectedSchemaVersion?.toString() ?? copy.databaseReadinessUnknown}
            />
            <Field label={copy.databaseReadinessSupportedVersion} value={result.supportedSchemaVersion.toString()} />
            <Field label={copy.databaseReadinessWal} value={boundedBoolean(result.walPresent, copy)} />
            <Field label={copy.databaseReadinessShm} value={boundedBoolean(result.shmPresent, copy)} />
            <Field label={copy.databaseReadinessJournal} value={boundedBoolean(result.rollbackJournalPresent, copy)} />
            <Field label={copy.databaseReadinessQuiescence} value={copy.databaseReadinessQuiescenceUnknown} />
            <Field label={copy.databaseReadinessOperationEvidence} value={operationMessage(result, copy)} />
          </dl>
        </div>
      ) : null}
    </section>
  );
}
