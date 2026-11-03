import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
// @ts-expect-error Vitest runs in Node; frontend production code does not import this built-in.
import { readFileSync } from "node:fs";
import { uiText } from "./i18n";
import { PreparedStateRecoveryPanel } from "./PreparedStateRecoveryPanel";

const styles = readFileSync(new URL("../styles.css", import.meta.url), "utf8");
const recoverySource = readFileSync(new URL("../../src-tauri/src/schema_v5_prepared_recovery.rs", import.meta.url), "utf8");

describe("recovery platform boundary source contract", () => {
  it.each([
    "inspect_prepared_state_recovery", "execute_prepared_state_recovery", "delete_prepared_recovery_receipt",
  ])("guards public %s before profile path resolution or operation I/O", (name) => {
    const body = recoverySource.slice(recoverySource.indexOf(`pub fn ${name}(`)).split(/\n(?:pub |fn |#\[)/)[0];
    const gate = body.indexOf("require_recovery_platform()");
    expect(gate).toBeGreaterThan(-1);
    expect(gate).toBeLessThan(body.indexOf("paths(&app)"));
    const lock = body.indexOf("operation_lock()");
    if (lock >= 0) expect(gate).toBeLessThan(lock);
  });

  it.each([
    "execute_at_with_failure", "execute_v5_sidecar_at_with_failure", "execute_post_commit_manifest_at", "delete_receipt_at",
  ])("guards private %s before its first effect", (name) => {
    const body = recoverySource.slice(recoverySource.indexOf(`fn ${name}(`));
    expect(body.slice(body.indexOf("{") + 1).trimStart()).toMatch(/^require_recovery_platform\(\)\?;/);
  });

  it("does not equate non-Windows File::open with exclusion", () => {
    const fallback = recoverySource.match(/#\[cfg\(not\(windows\)\)\]\s*fn open_database_exclusive\([\s\S]*?\n}/)?.[0];
    expect(fallback).toContain("require_recovery_platform()");
    expect(fallback).not.toContain("File::open");
  });
});

const inspection = {
  eligible: true,
  reason: null,
  classification: "exact_prepared_v4_empty_wal_v1",
  operationId: "abc123",
  database: { relativePath: "life-os.db", size: 10, sha256: "A".repeat(64) },
  databaseIdentity: "windows-volume-1-file-2",
  evidenceFiles: [
    { relativePath: "life-os.db-wal", size: 0, sha256: "B".repeat(64) },
    { relativePath: "life-os.db-shm", size: 32768, sha256: "C".repeat(64) },
  ],
  preservedFiles: [],
  claimDigest: "D".repeat(64),
} as const;

const v5SidecarInspection = {
  ...inspection,
  classification: "exact_v5_ready_legacy_empty_sidecar_v1",
  operationId: "v5-ready-op",
  preservedFiles: [
    { relativePath: "life-os-v5-ready-op.operation/state.json", size: 1091, sha256: "E".repeat(64) },
    { relativePath: "life-os-v5-ready-op.operation/backup.db", size: 73728, sha256: "F".repeat(64) },
    { relativePath: "prepared-recovery-old.receipt.json", size: 1600, sha256: "1".repeat(64) },
  ],
  claimDigest: "2".repeat(64),
} as const;

const postCommitManifestInspection = {
  ...inspection,
  classification: "exact_historical_frontend_v4_post_commit_manifest_v1",
  operationId: "post-commit-op",
  evidenceFiles: [
    { relativePath: "life-os-post-commit-op.operation/state.json", size: 1091, sha256: "3".repeat(64) },
  ],
  preservedFiles: [
    { relativePath: "life-os-post-commit-op.operation/backup.db", size: 155648, sha256: "4".repeat(64) },
    { relativePath: "prepared-recovery-old.receipt.json", size: 1600, sha256: "5".repeat(64) },
  ],
  claimDigest: "6".repeat(64),
} as const;

describe("prepared-state recovery disclosure", () => {
  it.each(["en", "zh-TW", "ja"] as const)("explains recovery-only unsupported capability visibly in %s", (language) => {
    const copy = uiText[language];
    const refused = { eligible: false, reason: "prepared_recovery_platform_unsupported", classification: null, operationId: null, database: null, databaseIdentity: null, evidenceFiles: [], preservedFiles: [], claimDigest: null };
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={copy} inspection={refused} result={null} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );
    expect(html).toContain(copy.preparedRecoveryPlatformUnsupported);
    expect(html.indexOf(copy.preparedRecoveryPlatformUnsupported)).toBeLessThan(html.indexOf("<details"));
    expect(html).toContain(copy.preparedRecoveryCancel);
    expect(html).toContain(copy.preparedRecoveryPreserveClose);
    for (const action of [copy.preparedRecoveryPrepare, copy.v5SidecarRecoveryPrepare, copy.postCommitManifestRecoveryPrepare, copy.founderV5Authorize, copy.founderV5RestoreBackup]) {
      expect(html).not.toContain(`>${action}<`);
    }
  });
  it.each(["en", "zh-TW", "ja"] as const)("keeps recovery distinct from migration in %s", (language) => {
    const copy = uiText[language];
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={copy} inspection={inspection} result={null} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );
    expect(html).toContain(copy.preparedRecoveryTitle);
    expect(html).toContain(copy.preparedRecoveryDatabaseBoundary);
    expect(html).toContain(copy.preparedRecoveryConsentBoundary);
    expect(html).toContain(copy.preparedRecoveryPrepare);
    expect(html).toContain(copy.preparedRecoveryPreserveClose);
  });

  it("does not offer mutation for an ineligible state", () => {
    const refused = { ...inspection, eligible: false, reason: "wal_has_frames", classification: null, operationId: null, database: null, databaseIdentity: null, evidenceFiles: [], preservedFiles: [], claimDigest: null } as const;
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={uiText.en} inspection={refused} result={null} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );
    expect(html).toContain("wal_has_frames");
    expect(html).not.toContain(`>${uiText.en.preparedRecoveryPrepare}<`);
  });

  it("requires close and restart after success without offering migration", () => {
    const result = { classification: inspection.classification, operationId: inspection.operationId, receiptRelativePath: "prepared-recovery-abc123.receipt.json", databaseSha256: inspection.database.sha256, restartRequired: true as const };
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={uiText.en} inspection={inspection} result={result} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );
    expect(html).toContain(uiText.en.preparedRecoveryCloseRestart);
    expect(html).not.toContain(`>${uiText.en.founderV5Authorize}<`);
  });

  it("keeps long technical recovery facts inside the disclosure card", () => {
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={uiText.en} inspection={inspection} result={null} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );

    expect(html).toContain('class="secondary-tools prepared-recovery-details"');
    expect(styles).toMatch(
      /\.prepared-recovery\s*\{[^}]*grid-template-columns:\s*minmax\(0,\s*1fr\)/s,
    );
    expect(styles).toMatch(
      /\.prepared-recovery-details \.technical-value\s*\{[^}]*overflow-wrap:\s*anywhere[^}]*word-break:\s*break-word/s,
    );
  });

  it.each(["en", "zh-TW", "ja"] as const)("discloses exact schema-v5 sidecar recovery boundaries in %s", (language) => {
    const copy = uiText[language];
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={copy} inspection={v5SidecarInspection} result={null} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );

    expect(html).toContain(copy.v5SidecarRecoveryTitle);
    expect(html).toContain(copy.v5SidecarRecoveryDatabaseBoundary);
    expect(html).toContain(copy.v5SidecarRecoveryOperationBoundary);
    expect(html).toContain(copy.v5SidecarRecoveryConsentBoundary);
    expect(html).toContain(copy.v5SidecarRecoveryMovedFiles);
    expect(html).toContain(copy.v5SidecarRecoveryPreservedFiles);
    expect(html).toContain(copy.v5SidecarRecoveryPrepare);
    expect(html).not.toContain(copy.preparedRecoveryNoPriorMigrationBoundary);
    expect(html).not.toContain(`>${copy.founderV5Authorize}<`);
  });

  it("reports schema-v5 sidecar success without implying migration or restore", () => {
    const result = {
      classification: v5SidecarInspection.classification,
      operationId: v5SidecarInspection.operationId,
      receiptRelativePath: "v5-sidecar-recovery-v5-ready-op.receipt.json",
      databaseSha256: v5SidecarInspection.database.sha256,
      restartRequired: true as const,
    };
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={uiText.en} inspection={v5SidecarInspection} result={result} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );

    expect(html).toContain(uiText.en.v5SidecarRecoveryComplete(result.receiptRelativePath));
    expect(html).toContain(uiText.en.preparedRecoveryCloseRestart);
    expect(html).not.toContain(`>${uiText.en.founderV5Authorize}<`);
    expect(html).not.toContain(`>${uiText.en.founderV5RestoreBackup}<`);
  });

  it.each(["en", "zh-TW", "ja"] as const)("discloses exact post-commit recovery without offering migration or restore in %s", (language) => {
    const copy = uiText[language];
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={copy} inspection={postCommitManifestInspection} result={null} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );

    expect(html).toContain(copy.postCommitManifestRecoveryTitle);
    expect(html).toContain(copy.postCommitManifestRecoveryDatabaseBoundary);
    expect(html).toContain(copy.postCommitManifestRecoveryExactStateBoundary);
    expect(html).toContain(copy.postCommitManifestRecoveryMutationBoundary);
    expect(html).toContain(copy.postCommitManifestRecoveryPreservedBoundary);
    expect(html).toContain(copy.postCommitManifestRecoveryControlledFiles);
    expect(html).toContain(copy.postCommitManifestRecoveryPrepare);
    expect(html).not.toContain(`>${copy.founderV5Authorize}<`);
    expect(html).not.toContain(`>${copy.founderV5RestoreBackup}<`);
    expect(html).not.toContain(`>${copy.founderV5DeleteBackup}<`);
  });

  it("reports post-commit recovery success as recovery rather than migration", () => {
    const result = {
      classification: postCommitManifestInspection.classification,
      operationId: postCommitManifestInspection.operationId,
      receiptRelativePath: "post-commit-manifest-recovery-post-commit-op.receipt.json",
      databaseSha256: postCommitManifestInspection.database.sha256,
      restartRequired: true as const,
    };
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={uiText.en} inspection={postCommitManifestInspection} result={result} pending={false} error={null} closeError={null} onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );

    expect(html).toContain(uiText.en.postCommitManifestRecoveryComplete(result.receiptRelativePath));
    expect(html).toContain(uiText.en.preparedRecoveryCloseRestart);
    expect(html).not.toContain(`>${uiText.en.founderV5Authorize}<`);
    expect(html).not.toContain(`>${uiText.en.founderV5RestoreBackup}<`);
  });

  it.each(["en", "zh-TW", "ja"] as const)("reports a close-command failure without retracting completed recovery in %s", (language) => {
    const copy = uiText[language];
    const result = {
      classification: v5SidecarInspection.classification,
      operationId: v5SidecarInspection.operationId,
      receiptRelativePath: "v5-sidecar-recovery-v5-ready-op.receipt.json",
      databaseSha256: v5SidecarInspection.database.sha256,
      restartRequired: true as const,
    };
    const html = renderToStaticMarkup(
      <PreparedStateRecoveryPanel copy={copy} inspection={v5SidecarInspection} result={result} pending={false} error={null} closeError="window_close_refused" onPrepare={vi.fn()} onCancel={vi.fn()} onClose={vi.fn()} />,
    );

    expect(html).toContain(copy.v5SidecarRecoveryComplete(result.receiptRelativePath));
    expect(html).toContain(copy.preparedRecoveryCloseFailed("window_close_refused"));
    expect(html).not.toContain(copy.preparedRecoveryFailed("window_close_refused"));
  });
});
