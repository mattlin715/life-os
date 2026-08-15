import type { FounderSchemaV5State } from "../shared/storage/sqlite/founderSchemaV5";
import type { UiCopy } from "./i18n";

interface Props {
  readonly copy: UiCopy;
  readonly state: FounderSchemaV5State;
  readonly pending: boolean;
  readonly error: boolean;
  readonly onDelete: () => void;
  readonly onRestore: () => void;
}

export function FounderSchemaV5BackupPanel({ copy, state, pending, error, onDelete, onRestore }: Props) {
  if (!state.backupAvailable) return null;
  const canDelete = state.state === "ready";
  const canRestore = state.state === "ready" || state.restoreAvailable;
  return (
    <details className="secondary-tools founder-v5-backup">
      <summary>{copy.founderV5BackupTitle}</summary>
      <p>{copy.founderV5BackupPurpose}</p>
      <dl>
        <dt>{copy.founderV5BackupLocation}</dt><dd>{state.backupRelativePath}</dd>
        <dt>{copy.founderV5BackupCreated}</dt><dd>{state.backupCreatedAt ?? "—"}</dd>
        <dt>{copy.founderV5BackupExpires}</dt><dd>{state.backupExpiresAt ?? "—"}</dd>
      </dl>
      <p className="summary-note">{copy.founderV5ProviderBoundary}</p>
      <p className="summary-note">{copy.founderV5RestoreWarning}</p>
      {state.restoreAvailable ? <p className="summary-note" role="status">{copy.founderV5RecoveryRestoreAvailable}</p> : null}
      {error ? <p className="inline-error" role="alert">{copy.founderV5BackupActionFailed}</p> : null}
      <div className="candidate-actions">
        {canDelete ? <button type="button" className="ghost-button" disabled={pending} onClick={onDelete}>{copy.founderV5DeleteBackup}</button> : null}
        {canRestore ? <button type="button" className="ghost-button" disabled={pending} onClick={onRestore}>{copy.founderV5RestoreBackup}</button> : null}
      </div>
    </details>
  );
}
