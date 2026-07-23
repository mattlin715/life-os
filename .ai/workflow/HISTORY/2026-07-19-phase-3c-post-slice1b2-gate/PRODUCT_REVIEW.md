# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-19-phase-3c-post-slice1b2-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 3c84d4660d425a65f1173f3f8501a76ba2b3262a
- Working-tree digest reviewed: 6652abf7cfe46328d71423884191651decb4e4132ae580b6a336e231c83589ab
- Created at: 2026-07-19T12:47:34.5296865Z
- Updated at: 2026-07-19T12:47:34.5296865Z

## Mission Interpretation

Close the factual Slice 1B-2 promotion drift, then decide whether Life OS may
take the smallest safe engineering step toward architecture/13 Slice 2. The
next step must produce backup-safety evidence without touching a real user
database, creating a user backup, enabling restore, or implying schema-v5
migration authority.

## Problem Statement

Slice 1B-2 completed the schema-v4 typed mutation boundary and was promoted by
feature commit `93fcc2bae358da21b857153757dbd143024598bd` and merge commit
`3c84d4660d425a65f1173f3f8501a76ba2b3262a`. Architecture/13 defines a verified
backup as a mandatory precondition for any later v5 migration, but its Founder
answers explicitly withhold backup, restore, cleanup, and migration
implementation. Earlier verification cannot be treated as implicit authority.

## User Value

A verified backup boundary reduces the risk that a future lifecycle migration
could make private local history unrecoverable. The immediate slice is not a
visible feature; its value is proof that the backup primitive fails closed
before Life OS ever exposes an Upgrade action or touches a user database.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI, Privacy before Profit, and user
  control require conservative handling of sensitive local copies.
- `docs/03_Principles.md`: local-first control and explicit user agency remain
  higher authority than migration convenience.
- `docs/10_Privacy.md`: a backup is another sensitive local copy, not harmless
  operational metadata.

No primary definition requires amendment.

## Relevant ADRs

- ADR-0007: reviewed artifact content and provenance must remain durable and
  distinguishable.
- ADR-0009: Historical Question packet/provenance/deletion semantics must survive
  any later migration exactly; this sprint does not change them.
- ADR-0011: the lifecycle design is Accepted, but schema-v5 migration,
  production implementation, cleanup, UI, and import remain separately gated.

No ADR status change is proposed.

## Current Implementation Context

- Implemented and promoted: schema-v5 fixed contract fixtures (Slice 0), newer
  schema startup refusal (Slice 1A), typed Experience mutations (Slice 1B-1),
  and typed artifact/historical mutations (Slice 1B-2).
- Current production schema: v4; `SCHEMA_VERSION = 4` and startup still
  initializes at `user_version = 4`.
- Founder-approved design only: exact v5 DDL, backup/restore policy, retention,
  migration, lifecycle schema, inspector, and export v2.
- Not implemented: `VACUUM INTO` backup creation, backup manifest, restore,
  backup cleanup, v5 cutover, lifecycle writes, or export v2.
- Factual synchronization in this sprint records the promoted Slice 1B-2 facts
  in architecture/13 without changing authority.

## In Scope

Founder decision package for **Slice 2A: disposable-fixture backup creation and
verification only**. If authorized later, the implementation boundary would be:

- Rust-owned, path-injected backup functions exercised only with synthetic
  temporary schema-v4 databases;
- SQLite `VACUUM INTO` with a destination that must not already exist;
- close/reopen verification of `user_version = 4`, database SHA-256,
  source-manifest digest, `foreign_key_check`, and `integrity_check`;
- exact content-free manifest fields already fixed by architecture/13;
- synthetic failure injection for destination conflict, vacuum failure,
  malformed database, version mismatch, digest/manifest corruption,
  foreign-key failure, and integrity failure;
- tests proving the source fixture remains unchanged and failures leave no
  verified-backup claim;
- factual documentation, canonical verification, Theory Alignment Review,
  archive/reset, and Founder diff review.

## Out Of Scope

- production Tauri command registration or renderer adapter;
- resolving or writing `<app_data_dir>/backups/schema-v5/`;
- opening, copying, restoring, or mutating a real user database;
- startup integration, Upgrade/Retry/Cancel/Restore UI or disclosure strings;
- restore implementation, file replacement, restart recovery, retention cleanup,
  delete-now, or 30-day scheduling;
- schema-v5 DDL, `user_version = 5`, migration, backfill, or receipts;
- Slices 2B and 3-6, Phase 4, provider/ContextPacket behavior, Harness changes,
  Git promotion, PR, or deployment.

## Product Constraints

The test primitive must remain unusable from production startup and UI. It may
prove a future mechanism; it may not claim a user backup exists or that v5 is
ready. Schema v4 remains the sole production authority.

## Evidence And Provenance Constraints

The manifest is content-free. It may contain only relative filename, backup ID,
database digest, source-manifest digest, schema/application versions, timestamps,
expiry timestamp, and verification result. It must contain no Experience,
artifact, packet, credential, provider receipt body, or raw provider error.

## Historical Context Constraints

The fixture must cover the current schema-v4 ADR-0009 tables sufficiently to
prove the backup preserves their bytes and relational integrity. It must not
reinterpret packets or create Phase 4 history.

## Consent Constraints

No historical consent or provider transmission occurs. Existing consent and
transmission fixture rows are preservation evidence only.

## AI-Role Constraints

No AI inference or generation is involved. The work cannot create a migration
recommendation to the user or treat technical readiness as permission.

## Privacy Constraints

Only generated disposable fixtures may contain backup content. Test artifacts
must stay under OS temporary directories and be removed by fixture teardown.
No real app-data path or credential file may be inspected.

## User-Agency Constraints

Production backup and restore remain impossible and invisible. A later user
flow must still require explicit Upgrade and explicit Restore actions; this
slice cannot pre-authorize either.

## Acceptance Criteria

If Option A is explicitly authorized:

1. no production command, startup path, UI, app-data path, or real database is
   connected to the primitive;
2. a disposable valid v4 fixture produces one verified backup through
   `VACUUM INTO` and a content-free manifest;
3. an existing destination fails without overwrite;
4. the closed backup digest, source-manifest digest, schema version, FK check,
   and integrity check are all required before `verified` can be true;
5. each malformed/corrupted/injected-failure case fails closed and leaves no
   verified claim;
6. source fixture bytes and logical records remain unchanged;
7. no schema-v5 DDL or `user_version = 5` exists outside existing test-only
   Slice 0 fixtures;
8. canonical verification and Theory Alignment Review pass;
9. workflow archives and stops at Founder diff review without Git promotion.

## Risks

- A test-only primitive could drift from later production integration; shared
  Rust logic and explicit path injection reduce, but do not eliminate, this risk.
- `VACUUM INTO` on fixtures does not prove live-database disk-space, lock, or OS
  permission behavior.
- Backup files are sensitive duplicates even when local; production integration
  therefore remains a separate consent/disclosure gate.
- Splitting Slice 2 delays restore proof, but keeps this authorization small and
  prevents accidental file-replacement authority.

## Open Questions

1. May Slice 2 be split so Slice 2A proves backup creation/verification before
   separately authorizing restore?
2. May the Rust primitive exist in production source if it is not registered as
   a Tauri command and is reachable only from disposable Rust tests?
3. Must every backup failure remove an incomplete destination immediately, or
   preserve it under a clearly unverified temporary name for diagnostics?

## Human Decision Required

true; decision ID `PHASE3C-SLICE2A-001`.

## Recommendation

Authorize Option A: Slice 2A disposable-fixture backup creation and verification
only. Delete incomplete destinations best-effort and never preserve them as
diagnostic content. Defer restore, production path integration, disclosure/UI,
retention cleanup, and migration to separately reviewed slices.

## Review Status

approved_with_conditions
