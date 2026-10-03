//! Direct creation and verification of a new exact schema-v5 database.
//!
//! This module is deliberately separate from the v4-to-v5 migration path.
//! It creates no legacy rows and no migration receipt.

use super::{
    canonical_file_bytes, current_content_checks, deterministic_id, integrity_checks,
    is_expected_schema_object_manifest, manifest, migration_error, schema_object_manifest,
    sha256_hex, split_fixed_ddl, user_version, CommitAttemptOutcome, CommitOutcomeAdapter,
    MigrationError, RollbackAttemptOutcome, SqlCommitOutcomeAdapter, CURRENT_APPLICATION_VERSION,
    EXPECTED_DDL_SHA256, SOURCE_TABLE_MANIFESTS, TARGET_SCHEMA_VERSION, TARGET_TABLE_MANIFESTS,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{raw_sql, Connection, SqliteConnection};
use std::path::Path;

const COMPATIBILITY_DDL: &str = include_str!("../schema/schema_v5_compatibility.sql");
const EXPECTED_COMPATIBILITY_DDL_SHA256: &str =
    "ef73f1dc24c594efafaeb06b0071a78001654c18784e8478f6a324db9d694ee9";
const EXPECTED_EMPTY_SOURCE_MANIFEST_SHA256: &str =
    "37c48bf35dacab3dbabd43c90c9b62d76643f5eb3adc60a51db4cc16c75783ae";
const EXPECTED_EMPTY_TARGET_MANIFEST_SHA256: &str =
    "5c58b7655955887086bb7fceaeb7927b1e5557efcf3e7330431b466b240217bb";
pub(crate) const DIRECT_RECEIPT_FORMAT: &str =
    "life-os/android-direct-fresh-v5-initialization-receipt-v1";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum DirectInitFailurePoint {
    #[default]
    None,
    AfterCompatibilityDdl,
    AfterFinalDdlStatement(usize),
    AfterDatabaseContract,
    AfterVersionMutation,
    PostCommitVerification,
}

#[derive(Clone, Debug)]
pub(crate) struct DirectInitRequest<'a> {
    pub(crate) path: &'a Path,
    pub(crate) application_id: &'a str,
    pub(crate) initialized_at: &'a str,
    pub(crate) failure_point: DirectInitFailurePoint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DirectFreshV5Receipt {
    pub(crate) format: String,
    pub(crate) receipt_id: String,
    pub(crate) origin: String,
    pub(crate) application_id: String,
    pub(crate) application_version: String,
    pub(crate) schema_version: i64,
    pub(crate) compatibility_ddl_sha256: String,
    pub(crate) final_ddl_sha256: String,
    pub(crate) schema_object_manifest_digest: String,
    pub(crate) source_manifest_digest: String,
    pub(crate) target_manifest_digest: String,
    pub(crate) initialized_at: String,
}

fn fail(code: impl Into<String>) -> MigrationError {
    MigrationError {
        code: code.into(),
        recovery_required: false,
    }
}

fn recovery(code: impl Into<String>) -> MigrationError {
    MigrationError {
        code: code.into(),
        recovery_required: true,
    }
}

fn inject(
    actual: DirectInitFailurePoint,
    expected: DirectInitFailurePoint,
) -> Result<(), MigrationError> {
    if actual == expected {
        Err(fail(format!("injected_direct_init_failure:{expected:?}")))
    } else {
        Ok(())
    }
}

fn compatibility_ddl_sha256() -> String {
    sha256_hex(&canonical_file_bytes(COMPATIBILITY_DDL))
}

fn receipt_id(application_id: &str, schema_manifest: &str, initialized_at: &str) -> String {
    deterministic_id(
        "v5fi_",
        DIRECT_RECEIPT_FORMAT,
        &[
            application_id,
            CURRENT_APPLICATION_VERSION,
            EXPECTED_COMPATIBILITY_DDL_SHA256,
            EXPECTED_DDL_SHA256,
            schema_manifest,
            EXPECTED_EMPTY_SOURCE_MANIFEST_SHA256,
            EXPECTED_EMPTY_TARGET_MANIFEST_SHA256,
            initialized_at,
        ],
    )
}

async fn verify_direct_contract_connection(
    connection: &mut SqliteConnection,
) -> Result<(String, String, String), MigrationError> {
    if user_version(connection).await? != TARGET_SCHEMA_VERSION {
        return Err(recovery("direct_v5_version_mismatch"));
    }
    let schema_manifest = schema_object_manifest(connection).await?;
    if !is_expected_schema_object_manifest(&schema_manifest) {
        return Err(recovery("direct_v5_schema_manifest_mismatch"));
    }
    let migration_receipt_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM schema_migration_receipts")
            .fetch_one(&mut *connection)
            .await
            .map_err(|error| migration_error("direct_v5_migration_receipt_unreadable", error))?;
    if migration_receipt_count != 0 {
        return Err(recovery("direct_v5_migration_receipt_forbidden"));
    }
    let contracts: Vec<(i64, String, String, String, String)> = sqlx::query_as(
        "SELECT authoritative_schema, minimum_application_version, \
         compatibility_projection, lifecycle_writes, export_v2 FROM database_contract",
    )
    .fetch_all(&mut *connection)
    .await
    .map_err(|error| migration_error("direct_v5_contract_unreadable", error))?;
    if contracts.len() != 1
        || contracts[0].0 != TARGET_SCHEMA_VERSION
        || contracts[0].1 != super::MINIMUM_APPLICATION_VERSION
        || contracts[0].2 != "enabled"
        || contracts[0].3 != "enabled"
        || contracts[0].4 != "disabled"
    {
        return Err(recovery("direct_v5_contract_mismatch"));
    }
    let guard_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v5_compatibility_write_guard")
        .fetch_one(&mut *connection)
        .await
        .map_err(|error| migration_error("direct_v5_guard_unreadable", error))?;
    if guard_count != 0 {
        return Err(recovery("direct_v5_guard_not_empty"));
    }
    current_content_checks(connection).await?;
    integrity_checks(connection).await?;
    let source_manifest = manifest(connection, &SOURCE_TABLE_MANIFESTS).await?;
    let target_manifest = manifest(connection, &TARGET_TABLE_MANIFESTS).await?;
    Ok((schema_manifest, source_manifest, target_manifest))
}

pub(super) async fn verify_direct_fresh_connection(
    connection: &mut SqliteConnection,
) -> Result<(), MigrationError> {
    verify_direct_contract_connection(connection)
        .await
        .map(|_| ())
}

pub(crate) async fn verify_direct_fresh_structure(path: &Path) -> Result<(), MigrationError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(false)
        .read_only(true)
        .immutable(true)
        .foreign_keys(true);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|error| migration_error("direct_v5_open_failed", error))?;
    let result = verify_direct_fresh_connection(&mut connection).await;
    connection
        .close()
        .await
        .map_err(|error| recovery(format!("direct_v5_close_failed:{error}")))?;
    result
}

async fn direct_receipt_from_connection(
    connection: &mut SqliteConnection,
    application_id: &str,
    initialized_at: &str,
) -> Result<DirectFreshV5Receipt, MigrationError> {
    let (schema_manifest, source_manifest, target_manifest) =
        verify_direct_contract_connection(connection).await?;
    let compatibility_digest = compatibility_ddl_sha256();
    if compatibility_digest != EXPECTED_COMPATIBILITY_DDL_SHA256
        || source_manifest != EXPECTED_EMPTY_SOURCE_MANIFEST_SHA256
        || target_manifest != EXPECTED_EMPTY_TARGET_MANIFEST_SHA256
    {
        return Err(recovery("direct_v5_empty_contract_digest_mismatch"));
    }
    let receipt_id = receipt_id(application_id, &schema_manifest, initialized_at);
    Ok(DirectFreshV5Receipt {
        format: DIRECT_RECEIPT_FORMAT.into(),
        receipt_id,
        origin: "direct_fresh_v5".into(),
        application_id: application_id.into(),
        application_version: CURRENT_APPLICATION_VERSION.into(),
        schema_version: TARGET_SCHEMA_VERSION,
        compatibility_ddl_sha256: compatibility_digest,
        final_ddl_sha256: EXPECTED_DDL_SHA256.into(),
        schema_object_manifest_digest: schema_manifest,
        source_manifest_digest: source_manifest,
        target_manifest_digest: target_manifest,
        initialized_at: initialized_at.into(),
    })
}

pub(crate) async fn verify_direct_fresh_v5(
    path: &Path,
    expected: &DirectFreshV5Receipt,
    application_id: &str,
) -> Result<(), MigrationError> {
    if expected.format != DIRECT_RECEIPT_FORMAT
        || expected.origin != "direct_fresh_v5"
        || expected.application_id != application_id
        || expected.application_version != CURRENT_APPLICATION_VERSION
        || expected.schema_version != TARGET_SCHEMA_VERSION
        || expected.compatibility_ddl_sha256 != compatibility_ddl_sha256()
        || expected.compatibility_ddl_sha256 != EXPECTED_COMPATIBILITY_DDL_SHA256
        || expected.final_ddl_sha256 != EXPECTED_DDL_SHA256
        || expected.source_manifest_digest != EXPECTED_EMPTY_SOURCE_MANIFEST_SHA256
        || expected.target_manifest_digest != EXPECTED_EMPTY_TARGET_MANIFEST_SHA256
        || expected.initialized_at.trim().is_empty()
        || expected.receipt_id
            != receipt_id(
                application_id,
                &expected.schema_object_manifest_digest,
                &expected.initialized_at,
            )
    {
        return Err(recovery("direct_v5_receipt_metadata_mismatch"));
    }
    verify_direct_fresh_structure(path).await?;
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(false)
        .read_only(true)
        .immutable(true)
        .foreign_keys(true);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|error| migration_error("direct_v5_open_failed", error))?;
    let schema_manifest = schema_object_manifest(&mut connection).await?;
    connection
        .close()
        .await
        .map_err(|error| recovery(format!("direct_v5_close_failed:{error}")))?;
    if schema_manifest != expected.schema_object_manifest_digest {
        return Err(recovery("direct_v5_receipt_schema_mismatch"));
    }
    Ok(())
}

async fn attempt_direct_fresh_v5<A: CommitOutcomeAdapter>(
    request: &DirectInitRequest<'_>,
    adapter: &A,
) -> Result<(DirectFreshV5Receipt, CommitAttemptOutcome), MigrationError> {
    if request.path.exists() {
        return Err(fail("direct_v5_destination_exists"));
    }
    let options = SqliteConnectOptions::new()
        .filename(request.path)
        .create_if_missing(true)
        .read_only(false)
        .foreign_keys(true);
    let mut connection = SqliteConnection::connect_with(&options)
        .await
        .map_err(|error| migration_error("direct_v5_create_failed", error))?;
    raw_sql("BEGIN IMMEDIATE")
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("direct_v5_begin_failed", error))?;

    let transaction_result = async {
        raw_sql(COMPATIBILITY_DDL)
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("direct_v5_compatibility_ddl_failed", error))?;
        inject(
            request.failure_point,
            DirectInitFailurePoint::AfterCompatibilityDdl,
        )?;

        let (before_guards, guards) = split_fixed_ddl()?;
        let mut index = 0;
        for statement in before_guards.into_iter().chain(guards) {
            raw_sql(statement)
                .execute(&mut connection)
                .await
                .map_err(|error| migration_error("direct_v5_final_ddl_failed", error))?;
            inject(
                request.failure_point,
                DirectInitFailurePoint::AfterFinalDdlStatement(index),
            )?;
            index += 1;
        }

        let guard_token = deterministic_id(
            "v5fg_",
            "life-os/direct-fresh-v5-contract-guard-v1",
            &[request.application_id, request.initialized_at],
        );
        sqlx::query("INSERT INTO v5_compatibility_write_guard (token, created_at) VALUES (?, ?)")
            .bind(&guard_token)
            .bind(request.initialized_at)
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("direct_v5_guard_insert_failed", error))?;
        sqlx::query(
            "INSERT INTO database_contract (singleton, authoritative_schema, \
             minimum_application_version, compatibility_projection, lifecycle_writes, \
             export_v2, updated_at) VALUES (1, 5, ?, 'enabled', 'enabled', 'disabled', ?)",
        )
        .bind(super::MINIMUM_APPLICATION_VERSION)
        .bind(request.initialized_at)
        .execute(&mut connection)
        .await
        .map_err(|error| migration_error("direct_v5_contract_insert_failed", error))?;
        sqlx::query("DELETE FROM v5_compatibility_write_guard WHERE token = ?")
            .bind(&guard_token)
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("direct_v5_guard_remove_failed", error))?;
        inject(
            request.failure_point,
            DirectInitFailurePoint::AfterDatabaseContract,
        )?;

        raw_sql("PRAGMA user_version = 5")
            .execute(&mut connection)
            .await
            .map_err(|error| migration_error("direct_v5_version_write_failed", error))?;
        inject(
            request.failure_point,
            DirectInitFailurePoint::AfterVersionMutation,
        )?;
        verify_direct_contract_connection(&mut connection).await?;
        Ok::<(), MigrationError>(())
    }
    .await;

    if let Err(error) = transaction_result {
        let rollback = adapter.rollback(&mut connection).await;
        connection
            .close()
            .await
            .map_err(|close_error| recovery(format!("direct_v5_close_failed:{close_error}")))?;
        return Err(match rollback {
            RollbackAttemptOutcome::RolledBack => error,
            RollbackAttemptOutcome::Failed { error_class } => recovery(format!(
                "direct_v5_precommit_rollback_unknown:{}:{error_class}",
                error.code
            )),
        });
    }

    let receipt = direct_receipt_from_connection(
        &mut connection,
        request.application_id,
        request.initialized_at,
    )
    .await?;
    let commit = adapter.commit(&mut connection).await;
    connection
        .close()
        .await
        .map_err(|error| recovery(format!("direct_v5_close_failed:{error}")))?;
    Ok((receipt, commit))
}

pub(crate) async fn initialize_direct_fresh_v5(
    request: DirectInitRequest<'_>,
) -> Result<DirectFreshV5Receipt, MigrationError> {
    let post_commit_failure =
        request.failure_point == DirectInitFailurePoint::PostCommitVerification;
    let path = request.path.to_path_buf();
    let application_id = request.application_id.to_string();
    let (receipt, outcome) = attempt_direct_fresh_v5(&request, &SqlCommitOutcomeAdapter).await?;
    match outcome {
        CommitAttemptOutcome::Committed => {
            if post_commit_failure {
                return Err(recovery(
                    "injected_direct_init_post_commit_verification_failure",
                ));
            }
            verify_direct_fresh_v5(&path, &receipt, &application_id).await?;
            Ok(receipt)
        }
        CommitAttemptOutcome::DefinitelyNotCommitted { error_class } => Err(fail(format!(
            "direct_v5_commit_definitely_not_committed:{error_class}"
        ))),
        CommitAttemptOutcome::OutcomeUnknown { error_class } => {
            if verify_direct_fresh_v5(&path, &receipt, &application_id)
                .await
                .is_ok()
            {
                Ok(receipt)
            } else {
                Err(recovery(format!(
                    "direct_v5_commit_outcome_unknown:{error_class}"
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    struct InjectedCommitAdapter {
        outcome: CommitAttemptOutcome,
        commit_first: bool,
    }

    impl CommitOutcomeAdapter for InjectedCommitAdapter {
        async fn commit(&self, connection: &mut SqliteConnection) -> CommitAttemptOutcome {
            if self.commit_first {
                raw_sql("COMMIT").execute(connection).await.unwrap();
            }
            self.outcome.clone()
        }

        async fn rollback(&self, connection: &mut SqliteConnection) -> RollbackAttemptOutcome {
            match raw_sql("ROLLBACK").execute(connection).await {
                Ok(_) => RollbackAttemptOutcome::RolledBack,
                Err(error) => RollbackAttemptOutcome::Failed {
                    error_class: format!("injected_rollback_failed:{error}"),
                },
            }
        }
    }

    fn request(path: &Path) -> DirectInitRequest<'_> {
        DirectInitRequest {
            path,
            application_id: "com.lifeos.review.m2a",
            initialized_at: "2026-09-29T00:00:00.000Z",
            failure_point: DirectInitFailurePoint::None,
        }
    }

    #[tokio::test]
    async fn commit_outcome_unknown_accepts_only_exact_durable_direct_v5() {
        let committed = TempDir::new().unwrap();
        let committed_path = committed.path().join("committed.db");
        let (receipt, outcome) = attempt_direct_fresh_v5(
            &request(&committed_path),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_after_commit".into(),
                },
                commit_first: true,
            },
        )
        .await
        .unwrap();
        assert!(matches!(
            outcome,
            CommitAttemptOutcome::OutcomeUnknown { .. }
        ));
        verify_direct_fresh_v5(&committed_path, &receipt, "com.lifeos.review.m2a")
            .await
            .unwrap();

        let unchanged = TempDir::new().unwrap();
        let unchanged_path = unchanged.path().join("unchanged.db");
        let (receipt, outcome) = attempt_direct_fresh_v5(
            &request(&unchanged_path),
            &InjectedCommitAdapter {
                outcome: CommitAttemptOutcome::OutcomeUnknown {
                    error_class: "injected_without_commit".into(),
                },
                commit_first: false,
            },
        )
        .await
        .unwrap();
        assert!(matches!(
            outcome,
            CommitAttemptOutcome::OutcomeUnknown { .. }
        ));
        assert!(
            verify_direct_fresh_v5(&unchanged_path, &receipt, "com.lifeos.review.m2a")
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn direct_origin_does_not_weaken_historical_migration_verifier() {
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("direct.db");
        let receipt = initialize_direct_fresh_v5(request(&path)).await.unwrap();
        verify_direct_fresh_v5(&path, &receipt, "com.lifeos.review.m2a")
            .await
            .unwrap();
        let migration_error = super::super::verify_any_committed_v5(&path)
            .await
            .unwrap_err();
        assert_eq!(migration_error.code, "durable_v5_receipt_count_mismatch");
    }
}
