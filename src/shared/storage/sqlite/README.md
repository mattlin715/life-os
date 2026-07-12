# SQLite Local Evidence Store

`life-os.db` is local-first. Schema version 3 contains `experience_entries` and the source-scoped `persisted_artifacts` payload table.

Migration and multi-statement mutations use the Rust SQLx command seam so each transaction owns one SQLite connection. `PRAGMA foreign_keys = ON` is enabled. Migration checks `sqlite_master` before copying each legacy table, propagates every other error, and advances `PRAGMA user_version` only after commit. Rust integration tests cover v2 preservation, rejected exclusion, injected rollback, unchanged version, and intact legacy rows.

Artifact replacement, Experience edit invalidation, and Experience deletion are atomic. Storage failures return to the React commit-first mutation runner, which restores durable state and surfaces `storageError`. Rejected Evidence and Pattern records are excluded, orphaned Reflection/Pattern records are filtered, and Experience JSON/Markdown export/import remains Experience-only.
