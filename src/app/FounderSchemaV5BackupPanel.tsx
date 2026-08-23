import type { FounderSchemaV5State } from "../shared/storage/sqlite/founderSchemaV5";
import type { UiCopy } from "./i18n";

interface Props {
  readonly copy: UiCopy;
  readonly state: FounderSchemaV5State;
  readonly ordinary: boolean;
  readonly pending: boolean;
  readonly error: boolean;
  readonly onDelete: () => void;
  readonly onRestore: () => void;
}

export function FounderSchemaV5BackupPanel({ copy, state, ordinary, pending, error, onDelete, onRestore }: Props) {
  if (!state.backupAvailable) return null;
  const canDelete = state.state === "ready";
  const canRestore = state.state === "ready" || state.restoreAvailable;
  const title = ordinary ? copy.ordinaryV5BackupTitle : copy.founderV5BackupTitle;
  const purpose = ordinary ? copy.ordinaryV5BackupPurpose : copy.founderV5BackupPurpose;
  const restoreWarning = ordinary ? copy.ordinaryV5RestoreWarning : copy.founderV5RestoreWarning;
  return (
    <details className="secondary-tools founder-v5-backup">
      <summary>{title}</summary>
      <p>{purpose}</p>
      <dl>
        <dt>{copy.founderV5BackupLocation}</dt><dd>{state.backupRelativePath}</dd>
        <dt>{copy.founderV5BackupCreated}</dt><dd>{state.backupCreatedAt ?? "—"}</dd>
        <dt>{copy.founderV5BackupExpires}</dt><dd>{state.backupExpiresAt ?? "—"}</dd>
      </dl>
      <p className="summary-note">{copy.founderV5ProviderBoundary}</p>
      <p className="summary-note">{restoreWarning}</p>
      {state.restoreAvailable ? <p className="summary-note" role="status">{copy.founderV5RecoveryRestoreAvailable}</p> : null}
      {error ? <p className="inline-error" role="alert">{copy.founderV5BackupActionFailed}</p> : null}
      <div className="candidate-actions">
        {canDelete ? <button type="button" className="ghost-button" disabled={pending} onClick={onDelete}>{copy.founderV5DeleteBackup}</button> : null}
        {canRestore ? <button type="button" className="ghost-button" disabled={pending} onClick={onRestore}>{copy.founderV5RestoreBackup}</button> : null}
      </div>
    </details>
  );
}
