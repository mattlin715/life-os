import type { FounderSchemaV5State } from "../shared/storage/sqlite/founderSchemaV5";
import type { UiCopy } from "./i18n";

interface Props {
  readonly copy: UiCopy;
  readonly state: FounderSchemaV5State;
  readonly ordinary: boolean;
  readonly pending: boolean;
  readonly cancelled: boolean;
  readonly error: string | null;
  readonly onAuthorize: () => void;
  readonly onCancel: () => void;
  readonly recoveryReceiptPending?: boolean;
  readonly onDeleteRecoveryReceipt?: () => void;
}

export function FounderSchemaV5MigrationPanel({ copy, state, ordinary, pending, cancelled, error, onAuthorize, onCancel, recoveryReceiptPending = false, onDeleteRecoveryReceipt }: Props) {
  const title = ordinary ? copy.ordinaryV5Title : copy.founderV5Title;
  const intro = ordinary ? copy.ordinaryV5Intro : copy.founderV5Intro;
  const localOnly = ordinary ? copy.ordinaryV5LocalOnly : copy.founderV5LocalOnly;
  return (
    <section className="welcome-card founder-v5-migration" aria-label={title}>
      <div className="welcome-copy">
        <p className="soft-label">{copy.databaseLocalLabel}</p>
        <h1>{title}</h1>
        <p className="welcome-subtitle">{intro}</p>
        <ul>
          <li>{localOnly}</li>
          {ordinary ? <li>{copy.ordinaryV5Purpose}</li> : null}
          <li>{copy.founderV5BackupDisclosure(state.backupRetentionDays)}</li>
          {ordinary ? <li>{copy.ordinaryV5BackupSensitivity}</li> : null}
          <li>{copy.founderV5ProviderBoundary}</li>
          {ordinary ? <li>{copy.ordinaryV5OlderBinaryBoundary}</li> : null}
          <li>{copy.founderV5CancelBoundary}</li>
        </ul>
        {ordinary && state.preparedRecoveryReceiptAvailable ? (
          <details className="secondary-tools prepared-recovery-receipt">
            <summary>{copy.preparedRecoveryReceiptTitle}</summary>
            <p>{copy.preparedRecoveryReceiptPurpose}</p>
            <dl>
              <dt>{copy.founderV5BackupLocation}</dt>
              <dd>{state.preparedRecoveryReceiptRelativePath}</dd>
              <dt>{copy.preparedRecoveryOperation}</dt>
              <dd>{state.preparedRecoveryReceiptOperationId}</dd>
              <dt>{copy.founderV5BackupCreated}</dt>
              <dd>{state.preparedRecoveryReceiptRecoveredAt}</dd>
            </dl>
            <button
              type="button"
              className="ghost-button"
              disabled={recoveryReceiptPending || pending}
              onClick={onDeleteRecoveryReceipt}
            >
              {copy.preparedRecoveryReceiptDelete}
            </button>
          </details>
        ) : null}
        {cancelled ? <p className="summary-note" role="status">{copy.founderV5Cancelled}</p> : null}
        {error ? <p className="inline-error" role="alert">{copy.founderV5Failed}</p> : null}
        <div className="candidate-actions">
          <button type="button" className="primary-button" disabled={pending || cancelled} onClick={onAuthorize}>
            {pending ? copy.founderV5Migrating : copy.founderV5Authorize}
          </button>
          <button type="button" className="ghost-button" disabled={pending || cancelled} onClick={onCancel}>
            {copy.founderV5Cancel}
          </button>
        </div>
      </div>
    </section>
  );
}
