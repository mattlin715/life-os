import type {
  PreparedRecoveryInspection,
  PreparedRecoveryResult,
} from "../shared/storage/sqlite/founderSchemaV5";
import type { UiCopy } from "./i18n";

interface Props {
  readonly copy: UiCopy;
  readonly inspection: PreparedRecoveryInspection;
  readonly result: PreparedRecoveryResult | null;
  readonly pending: boolean;
  readonly error: string | null;
  readonly closeError: string | null;
  readonly onPrepare: () => void;
  readonly onCancel: () => void;
  readonly onClose: () => void;
}

export function PreparedStateRecoveryPanel({
  copy,
  inspection,
  result,
  pending,
  error,
  closeError,
  onPrepare,
  onCancel,
  onClose,
}: Props) {
  const isV5SidecarRecovery = inspection.classification === "exact_v5_ready_legacy_empty_sidecar_v1";
  const isPostCommitManifestRecovery = inspection.classification === "exact_historical_frontend_v4_post_commit_manifest_v1";
  const title = isPostCommitManifestRecovery
    ? copy.postCommitManifestRecoveryTitle
    : isV5SidecarRecovery
      ? copy.v5SidecarRecoveryTitle
      : copy.preparedRecoveryTitle;
  const boundaries = isPostCommitManifestRecovery
    ? [
        copy.postCommitManifestRecoveryDatabaseBoundary,
        copy.postCommitManifestRecoveryExactStateBoundary,
        copy.postCommitManifestRecoveryPersonalRowsBoundary,
        copy.postCommitManifestRecoveryMutationBoundary,
        copy.postCommitManifestRecoveryPreservedBoundary,
        copy.postCommitManifestRecoveryReceiptBoundary,
        copy.postCommitManifestRecoveryCancelBoundary,
      ]
    : isV5SidecarRecovery
    ? [
        copy.v5SidecarRecoveryDatabaseBoundary,
        copy.v5SidecarRecoveryOperationBoundary,
        copy.v5SidecarRecoveryPersonalRowsBoundary,
        copy.v5SidecarRecoveryEvidenceBoundary,
        copy.v5SidecarRecoveryReceiptBoundary,
        copy.v5SidecarRecoveryConsentBoundary,
        copy.v5SidecarRecoveryCancelBoundary,
      ]
    : [
        copy.preparedRecoveryDatabaseBoundary,
        copy.preparedRecoveryNoPriorMigrationBoundary,
        copy.preparedRecoveryPersonalRowsBoundary,
        copy.preparedRecoveryEvidenceBoundary,
        copy.preparedRecoveryReceiptBoundary,
        copy.preparedRecoveryConsentBoundary,
        copy.preparedRecoveryCancelBoundary,
      ];

  return (
    <section className="welcome-card prepared-recovery" aria-label={title}>
      <div className="welcome-copy">
        <p className="soft-label">{copy.databaseLocalLabel}</p>
        <h1>{title}</h1>
        <p className="welcome-subtitle">
          {isPostCommitManifestRecovery
            ? copy.postCommitManifestRecoveryIntro
            : isV5SidecarRecovery
              ? copy.v5SidecarRecoveryIntro
              : copy.preparedRecoveryIntro}
        </p>
        <ul>
          {boundaries.map((boundary) => <li key={boundary}>{boundary}</li>)}
        </ul>
        <details className="secondary-tools prepared-recovery-details">
          <summary>{copy.preparedRecoveryTechnicalDetails}</summary>
          <dl>
            <dt>{copy.preparedRecoveryClassification}</dt>
            <dd>{inspection.classification ?? "—"}</dd>
            <dt>{copy.preparedRecoveryOperation}</dt>
            <dd>{inspection.operationId ?? "—"}</dd>
            <dt>{copy.preparedRecoveryDatabaseDigest}</dt>
            <dd className="technical-value">{inspection.database?.sha256 ?? "—"}</dd>
            <dt>{copy.preparedRecoveryDatabaseIdentity}</dt>
            <dd className="technical-value">{inspection.databaseIdentity ?? "—"}</dd>
            <dt>{copy.preparedRecoveryClaimDigest}</dt>
            <dd className="technical-value">{inspection.claimDigest ?? "—"}</dd>
          </dl>
          {inspection.evidenceFiles.length > 0 ? (
            <>
              {isV5SidecarRecovery || isPostCommitManifestRecovery ? (
                <p className="summary-note">
                  <strong>
                    {isPostCommitManifestRecovery
                      ? copy.postCommitManifestRecoveryControlledFiles
                      : copy.v5SidecarRecoveryMovedFiles}
                  </strong>
                </p>
              ) : null}
              {inspection.evidenceFiles.map((file) => (
                <p className="summary-note technical-value" key={file.relativePath}>
                  {file.relativePath} · {file.size} B · {file.sha256}
                </p>
              ))}
            </>
          ) : null}
          {inspection.preservedFiles.length > 0 ? (
            <>
              <p className="summary-note"><strong>{copy.v5SidecarRecoveryPreservedFiles}</strong></p>
              {inspection.preservedFiles.map((file) => (
                <p className="summary-note technical-value" key={`preserved-${file.relativePath}`}>
                  {file.relativePath} · {file.size} B · {file.sha256}
                </p>
              ))}
            </>
          ) : null}
          {!inspection.eligible ? (
            <p className="inline-error" role="alert">
              {copy.preparedRecoveryIneligible(inspection.reason ?? "unknown")}
            </p>
          ) : null}
        </details>
        {result ? (
          <p className="summary-note" role="status">
            {isPostCommitManifestRecovery
              ? copy.postCommitManifestRecoveryComplete(result.receiptRelativePath)
              : isV5SidecarRecovery
              ? copy.v5SidecarRecoveryComplete(result.receiptRelativePath)
              : copy.preparedRecoveryComplete(result.receiptRelativePath)}
          </p>
        ) : null}
        {error ? <p className="inline-error" role="alert">{copy.preparedRecoveryFailed(error)}</p> : null}
        {closeError ? <p className="inline-error" role="alert">{copy.preparedRecoveryCloseFailed(closeError)}</p> : null}
        <div className="candidate-actions">
          {result ? (
            <button type="button" className="primary-button" onClick={onClose}>
              {copy.preparedRecoveryCloseRestart}
            </button>
          ) : (
            <>
              <button type="button" className="ghost-button" disabled={pending} onClick={onClose}>
                {copy.preparedRecoveryPreserveClose}
              </button>
              {inspection.eligible ? (
                <button type="button" className="primary-button" disabled={pending} onClick={onPrepare}>
                  {pending
                    ? (isPostCommitManifestRecovery
                        ? copy.postCommitManifestRecoveryPreparing
                        : isV5SidecarRecovery
                          ? copy.v5SidecarRecoveryPreparing
                          : copy.preparedRecoveryPreparing)
                    : (isPostCommitManifestRecovery
                        ? copy.postCommitManifestRecoveryPrepare
                        : isV5SidecarRecovery
                          ? copy.v5SidecarRecoveryPrepare
                          : copy.preparedRecoveryPrepare)}
                </button>
              ) : null}
              <button type="button" className="ghost-button" disabled={pending} onClick={onCancel}>
                {copy.preparedRecoveryCancel}
              </button>
            </>
          )}
        </div>
      </div>
    </section>
  );
}
