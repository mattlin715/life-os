# Engineering Plan

Status: approved

- Sprint ID: 2026-07-26-phase-3c-post-slice2b2-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 277c4b5b2031d5bf88dc2a33b03765c103c62629
- Working-tree digest reviewed: 971f75ff5752ecd8c270d6969c7c671b64a51c3f39b969ea5d9ca5b67e2a58a2
- Created at: 2026-07-25T18:02:13.6265852Z
- Updated at: 2026-07-25T18:02:13.6265852Z

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Founder decision
`PHASE3C-SLICE2B3-001` authorizes only private, unregistered backup ownership
handoff and Windows replacement integration exercised against synthetic,
disposable app-like roots and schema-v4 fixtures. It must preserve the
absent-destination `VACUUM INTO` contract, create no runtime entry point, keep
Windows durable replacement fail-closed, and stop at Founder diff review after
canonical verification, Theory Alignment Review, and archive/reset.

## Existing Implementation Understanding

- Slice 2A's integration test computes the governed six-table source manifest,
  creates a backup once with `VACUUM INTO`, closes it, and validates SHA-256,
  schema v4, foreign keys, integrity, and source preservation.
- Slice 2B-1 proves only disposable logical replacement and exact-record
  revalidation.
- Slice 2B-2's private `filesystem_safety` module pre-creates backup, staging,
  and state files. This conflicts with Slice 2A's stricter absent-destination
  contract.
- The module already has canonical/reparse/link/volume checks, quiescence and
  sidecar refusal, content-free state, candidate evidence, durability and
  replacement traits, and restart classification.
- `SystemDurability` reports Windows parent-directory sync unsupported.
- There is no concrete Windows `ReplacementAdapter`.
- The module is declared privately in `src-tauri/src/lib.rs` and is not a
  Tauri command or runtime caller.

## Affected Modules

- `src-tauri/src/filesystem_safety.rs`: operation-directory ownership,
  absent-path backup creation orchestration, concrete private
  `SystemVacuumInto`, Windows `ReplaceFileW` adapter, restart semantics, and
  synthetic/disposable unit tests.
- `src-tauri/tests/schema_v5_backup.rs`: retain the promoted full Slice 2A/2B-1
  regression oracle; change only if necessary to demonstrate compatibility,
  never to weaken its absent-destination behavior.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`:
  factual Option A authority and bounded working-tree evidence.
- `.ai/workflow/`: plan, report, verification, theory review, sprint report,
  event chain, and archive/reset evidence.

No Cargo dependency is expected: `windows-sys` already enables
`Win32_Foundation` and `Win32_Storage_FileSystem`.

## Proposed Design

1. Replace per-file ownership at the application root with a create-new,
   high-entropy operation directory under the canonical disposable root.
2. Keep the exact backup child path absent. Create only staging and state with
   `create_new`; state records the claimed backup filename.
3. Preserve the canonical application root separately from the canonical
   operation directory. Require the live file to remain a direct single-link
   child of the former and backup/staging/state to be direct children of the
   latter.
4. Add explicit `BackupVerified` operation/restart state. `Prepared` requires
   backup absence; an unexpected backup in that phase is
   `recovery_required`.
5. Add a private async backup-creator boundary and concrete
   `SystemVacuumInto`. It opens the exact source without create-if-missing,
   executes one parameter-bound `VACUUM INTO`, and closes the source
   connection.
6. Add `create_owned_verified_backup`:
   - recheck guard, sidecars, canonical identities, state, distinct paths,
     same-volume evidence, and backup absence;
   - capture source bytes/digest and exact `CandidateVerifier` evidence;
   - invoke the creator once;
   - recheck guard/sidecars/source bytes;
   - open the output as a direct non-reparse single-link regular file;
   - sync the closed output and inspect it;
   - require schema 4, FK/integrity success, and exact source
     manifest/record-digest parity;
   - compute SHA-256, verify the returned exact contract, and record
     `BackupVerified`.
7. If creation reports failure and an exact direct single-link output is
   positively reconciled inside the operation directory, remove only that
   incomplete output. Any alias/reparse/multiple-link/state/identity ambiguity
   is preserved and recorded `recovery_required`.
8. Existing replacement preparation consumes only `BackupVerified` or exact
   legacy test state and continues to revalidate backup/staging evidence.
9. Add private `cfg(windows)` `WindowsReplacement` calling `ReplaceFileW` with
   no optional replacement backup:
   - nonzero return -> logical `Committed`;
   - `ERROR_UNABLE_TO_REMOVE_REPLACED` -> `FailedUnchanged`, followed by the
     executor's exact live-digest reconciliation;
   - 1176, 1177, and every other error -> `OutcomeUnknown`.
10. `WindowsReplacement::requires_parent_directory_sync` remains true.
    Therefore `SystemDurability` refuses before invocation on Windows; direct
    disposable adapter tests prove API and classification behavior without
    claiming durable product execution.
11. No autonomous resume, retry, replay, rollback, repair, candidate selection,
    sidecar cleanup, or unknown-file deletion is added.

## Alternatives Considered

- Pre-create an empty backup file: rejected because it violates the promoted
  Life OS Slice 2A absent-destination contract.
- Delete the reservation before `VACUUM INTO`: rejected because it creates an
  ownership gap and unsafe cleanup semantics.
- Use `MoveFileExW`: rejected for this slice because `ReplaceFileW` is the
  explicit existing-file replacement boundary and provides documented partial
  failure classes. Neither API is treated as proof of power-loss durability.
- Start schema-v5 execution: rejected by Founder decision and unsafe sequencing.
- Build a second backup implementation: rejected; orchestration will use one
  concrete `VACUUM INTO` path and preserve the promoted Slice 2A regression
  suite as the independent oracle.

## Data Lifecycle Impact

Synthetic disposable schema-v4 files only. The operation directory and state
are content-free except filenames, operation ID, digests, phases, and outcome
classes. No real user path or content is eligible. Exact-owned incomplete
backup cleanup is bounded to the current operation; ambiguous evidence is
preserved.

## SQLite Or Migration Impact

No migration or DDL. Production `SCHEMA_VERSION` and `user_version` remain 4.
The concrete backup primitive runs only from unit tests against OS-temporary
schema-v4 fixtures and has no production caller.

## Provenance Impact

No semantic change. Synthetic governed rows are compared through manifest and
exact-record digests. No provenance is created, reconstructed, revised, or
deleted.

## Historical Context Impact

None. Historical rows are opaque fixture records. No retrieval, selection,
Context Packet, provider call, Cross-Experience Reflection, Pattern, summary,
or identity inference occurs.

## Consent Impact

None. No consent is created, consumed, reused, or transmitted.

## Provider Transmission Impact

None. Provider and ContextPacket code must have no diff.

## Import And Export Impact

None. Import/export formats and behavior remain unchanged.

## Test Strategy

Focused Rust tests will cover:

1. operation directory/state/staging creation with backup absent;
2. operation-directory collision and backup-path collision preservation;
3. reparse, hard-link, path escape, identity change, sidecar, active/unknown
   activity, and cross-volume refusal;
4. one real `VACUUM INTO` invocation into the claimed absent path;
5. source byte preservation and output SHA-256/schema/FK/integrity/manifest/
   exact-record parity;
6. creator failure with exact-owned incomplete cleanup versus ambiguous-output
   preservation;
7. post-close digest, manifest, schema, FK, integrity, and record mismatch;
8. `Prepared`, `BackupVerified`, incomplete/contradictory, staged, committed,
   and recovery-required restart classifications;
9. Windows classifier mapping for success, 1175, 1176, 1177, and unknown;
10. actual Windows disposable `ReplaceFileW` success;
11. `SystemDurability` refusal before Windows replacement invocation;
12. no file outside the disposable owned root changes.

The promoted 12 Slice 2A/2B-1 tests and 8 schema-contract tests must continue
to pass unchanged in meaning.

## Repository Verification Strategy

Run Rust format, focused `cargo test filesystem_safety`, the existing
`schema_v5_backup` integration suite, and then:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

Confirm workflow, Vitest, all Rust suites, typecheck/build, UTF-8, whitespace,
secret, Markdown-link, and Constitution checks pass. Confirm no Tauri
registration, UI, startup, app-data, provider, ContextPacket, schema-version,
migration, or real-data diff.

## Manual UI Verification

Not applicable. No UI or runtime path is authorized or added. Founder diff
review remains mandatory.

## Rollback Or Recovery Strategy

All implementation remains unstaged on one feature branch. The private module
can be reverted without touching any user database. Tests remove only exact
owned temporary output; ambiguous state is preserved until the enclosing
temporary directory is disposed. No automatic recovery action exists.

## Documentation Impact

Update only architecture/13 factual authority/current evidence and the
repository workflow artifacts. Do not add a new ADR or design document.

## ADR Impact

No ADR change. ADR-0007, ADR-0009, ADR-0010, and ADR-0011 remain unchanged.

## Risk Level

Medium. Windows replacement and pathname ownership are subtle, but the module
is private, unregistered, path-injected, schema-v4-only, and invoked solely in
disposable tests. Unsupported durability and ambiguous ownership fail closed.

## Escalation Decision

No new decision is required. Any need for real app-data, user databases,
runtime/Tauri/UI/startup integration, process-wide production quiescence,
durable Windows activation, sidecar cleanup, retention, schema v5, migration,
later slices, or autonomous recovery must stop at a new Founder gate.
