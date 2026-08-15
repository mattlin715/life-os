import type { FounderSchemaV5State } from "../shared/storage/sqlite/founderSchemaV5";
import type { UiCopy } from "./i18n";

interface Props {
  readonly copy: UiCopy;
  readonly state: FounderSchemaV5State;
  readonly pending: boolean;
  readonly cancelled: boolean;
  readonly error: string | null;
  readonly onAuthorize: () => void;
  readonly onCancel: () => void;
}

export function FounderSchemaV5MigrationPanel({ copy, state, pending, cancelled, error, onAuthorize, onCancel }: Props) {
  return (
    <section className="welcome-card founder-v5-migration" aria-label={copy.founderV5Title}>
      <div className="welcome-copy">
        <p className="soft-label">{copy.databaseLocalLabel}</p>
        <h1>{copy.founderV5Title}</h1>
        <p className="welcome-subtitle">{copy.founderV5Intro}</p>
        <ul>
          <li>{copy.founderV5LocalOnly}</li>
          <li>{copy.founderV5BackupDisclosure(state.backupRetentionDays)}</li>
          <li>{copy.founderV5ProviderBoundary}</li>
          <li>{copy.founderV5CancelBoundary}</li>
        </ul>
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
