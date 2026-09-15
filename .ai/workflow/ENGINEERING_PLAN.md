# Engineering Plan

Status: approved

- Sprint ID: 2026-08-27-desktop-schema-v5-prepared-state-recovery-r2
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899
- Working-tree digest reviewed: a84c6cf19ef306fda74e98e67c2d42c362163011e3ce8c6b7a5bc5b0c85f5d1f
- Created at: 2026-08-27T01:10:00+09:00
- Updated at: 2026-08-27T01:10:00+09:00

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement only exact ordinary-identity recovery against path-injected synthetic/disposable roots. The original predicate covers the pre-backup prepared-v4 state. Founder-selected Option A-R3 adds only the exact post-migration `v5_ready` state with a valid owned operation, unchanged verified backup and prior recovery evidence, zero-byte WAL, and matching zero-frame SHM created by historical schema-v4 refusal. A supplied preflight database digest binds each mutation and is displayed for external Founder comparison; it is not a claim that the tracked repository knows a personal database fingerprint. Recovery and migration are separate invocations and user actions. No real-profile access, general repair, SQLite checkpoint, schema/DDL, provider, ContextPacket, consent, Phase 4, Android, or Git promotion is allowed.

## Existing Implementation Understanding

- `schema_v5_founder_activation::classify` refuses any WAL/SHM/journal before reading schema or operation state and maps a v4 `Prepared` operation to `migration_prepared` only when sidecars are absent.
- `filesystem_safety::prepare_operation` creates one direct-owned operation directory, zero-byte `staging.db`, and schema-3 `state.json`; at `Prepared`, all backup, migration, and outcome fields remain absent.
- The R1 state stopped after that preparation and before `create_owned_verified_backup`; no backup digest or live-before digest was recorded in `state.json`.
- R1 later corrected source reads to immutable mode and writer close, but deliberately retained sidecar refusal and no cleanup action.
- Existing migration authorization can resume only from migration-required/verified evidence and already creates a new backup plus fresh receipt. It must remain unchanged except that post-recovery startup can reach it again.
- Existing frontend renders migration or generic blocked startup; no prepared recovery contract exists.

## Affected Modules

Exact anticipated allowlist (40 paths after bounded Manual Phase A close-control correction cycle 4):

1. `.ai/workflow/CURRENT_MISSION.md`
2. `.ai/workflow/PRODUCT_REVIEW.md`
3. `.ai/workflow/ENGINEERING_PLAN.md`
4. `.ai/workflow/ENGINEERING_REPORT.md`
5. `.ai/workflow/THEORY_ALIGNMENT_REVIEW.md`
6. `.ai/workflow/DECISION_REQUIRED.md`
7. `.ai/workflow/SPRINT_REPORT.md`
8. `.ai/workflow/EVENTS.jsonl`
9. `.ai/workflow/WORKFLOW_STATE.json`
10. `src-tauri/Cargo.toml`
11. `src-tauri/capabilities/default.json`
12. `src-tauri/gen/schemas/capabilities.json`
13. `src-tauri/src/filesystem_safety.rs`
14. `src-tauri/src/schema_v5_founder_activation.rs`
15. `src-tauri/src/schema_v5_migration.rs`
16. `src-tauri/src/schema_v5_prepared_recovery.rs`
17. `src-tauri/src/lib.rs`
18. `src/shared/storage/sqlite/founderSchemaV5.ts`
19. `src/shared/storage/sqlite/founderSchemaV5.test.ts`
20. `src/app/App.tsx`
21. `src/app/closeLifeOs.ts`
22. `src/app/closeLifeOs.test.ts`
23. `src/app/FounderSchemaV5BackupPanel.test.tsx`
24. `src/app/PreparedStateRecoveryPanel.tsx`
25. `src/app/PreparedStateRecoveryPanel.test.tsx`
26. `src/app/FounderSchemaV5MigrationPanel.tsx`
27. `src/app/FounderSchemaV5MigrationPanel.test.tsx`
28. `src/app/i18n.ts`
29. `src/app/i18n.test.ts`
30. `src/styles.css`
31. `src/shared/storage/createLocalEvidenceStore.test.ts`
32. `docs/architecture/19_Desktop_Schema_v5_Prepared_State_Recovery_and_Real_Profile_Migration_R2.md`
33. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
34. `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`
35. `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
36. `docs/00_Index.md`
37. `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
38. `scripts/founder-dogfood-package.mjs`
39. `scripts/ordinary-schema-v5-review-package.mjs`
40. `scripts/ordinary-schema-v5-review-package.node-test.mjs`

The ordinary review builder still accepts a unique R2 suffix and emits the exact six-field manifest. Correction cycle 4 extends only the package source contract: source and generated capability evidence must both grant the existing `main` window exactly one `core:window:allow-close` command. The Founder-package active-successor allowlist includes the seven close-correction paths. Manual Phase A correction cycle 3 still adds `schema_v5_migration.rs` only to expose a private immutable verifier after the exact empty-sidecar proof; normal runtime verification continues to refuse every sidecar.

## Proposed Design

1. Add `filesystem_safety` exact-prepared inspection that validates canonical/direct ownership, no links/reparse/hard links, exact file allowlist, exact schema-3 `Prepared` state with every later-phase field absent, absent backup, and zero-byte staging. It returns only content-free relative names, sizes, and digests.
2. Add a Windows-focused `schema_v5_prepared_recovery` module with path-injected core functions and ordinary-identity Tauri adapters.
3. Preflight hashes the closed database, reads only its fixed SQLite header (`SQLite format 3`, schema v4, persistent-WAL header bytes), proves exactly one operation, proves WAL zero frames and matching initialized SHM with zero maximum frame/backfill, rejects journal/unexpected evidence, and returns an opaque claim digest over the exact content-free facts. It executes no SQL.
4. Mutation acquires the shared process-local database-operation lock, requires the renderer claim/digest, opens the database read-only with Windows read sharing only and acquires an immediate exclusive lock over SQLite's complete 512-byte Win32 coordination range beginning at `PENDING_BYTE` so active SQLite locks and writer/delete-capable handles cannot coexist, re-runs the complete classifier, and refuses any TOCTOU difference. `Win32_System_IO` is added only for the typed `OVERLAPPED` lock structure.
5. On success, create-new one exact sibling quarantine directory, move only the exact WAL, SHM, zero staging, and exact prepared state into it using write-through moves, remove the now-empty `.operation` directory, write/sync one canonical content-free receipt, then re-hash/re-read the locked database header. The database is never renamed, checkpointed, or opened writable.
6. Any failure returns recovery-required and does not retry. Partial quarantine remains preserved and cannot match the initial classifier.
7. A valid retained content-free receipt is summarized in the ordinary migration-required state. A separately explicit delete control may remove only that exact receipt; quarantined evidence and all database or migration state remain untouched.
8. Frontend adds a generic blocked-state entry point. Opening calls read-only preflight. Eligible results show required disclosures and controls; ineligible results omit Prepare recovery. Prepare recovery sends the exact claim once. Success stays in the recovery panel and asks the user to close/restart; it does not open migration. Preserve and close uses the app window close; Cancel returns to refusal unchanged.
9. Startup after explicit close/restart reaches the existing ordinary migration panel. Any later migration remains the unchanged separate authorization call and creates a new operation/backup.
10. The A-R3 classifier first proves the exact zero-byte WAL/matching zero-frame SHM, then uses an immutable read-only connection to verify the one committed schema-v5 receipt/runtime contract. This narrowly scoped verifier is callable only after sidecar proof; every normal runtime verifier continues to reject sidecars.
11. A-R3 claim binding includes the live v5 database identity/digest, the exact WAL/SHM to move, and eight protected file facts covering current operation state/staging/backup plus the earlier prepared-state receipt and four-file quarantine. Any change before execution refuses without a move.
12. Explicit A-R3 execution moves only WAL/SHM into a new exact-owned quarantine, writes one strict content-free receipt containing moved and protected file facts, re-verifies exact v5/committed receipt/runtime state/`v5_ready` operation/verified v4 backup, and requires close/restart. It never removes or rewrites the current operation or earlier recovery evidence.
13. Correction cycle 4 grants the existing `main` renderer only `core:window:allow-close`, awaits that promise, and distinguishes close-command failure from recovery failure. A rejected close preserves the already-completed recovery result, reports the technical reason in all three languages, directs the user to the native window close control, and never offers or repeats recovery.

## Alternatives Considered

- Delete evidence directly: rejected because multi-file failure would erase forensic context and make receipt durability harder.
- Automatic cleanup on startup: rejected; violates explicit authority and makes recovery indistinguishable from migration retry.
- SQLite checkpoint: rejected; would mutate database/WAL state and generalize into repair.
- Embed the real database hash in source: rejected as an unnecessary personal-data fingerprint. Display and bind the current digest; final real action requires external Founder comparison.
- Store recovery consent and reuse it for migration: rejected; two separate actions are constitutional/user-agency requirements.

## Data Lifecycle Impact

Adds exact quarantines containing only proven technical transient bytes plus separate content-free receipts. The prepared-v4 path holds four files; the post-migration A-R3 path holds only WAL/SHM and records digests for eight protected technical files that remain in place. Neither contains a database copy or interpreted row content. Personal-data retention policy and user artifact lifecycle do not change.

## SQLite Or Migration Impact

No DDL, schema version, SQL transaction, migration algorithm, backup, restore, or migration-receipt contract changes. Prepared-v4 recovery reads fixed database/WAL/SHM structures only. A-R3 additionally opens the already-proven zero-frame schema-v5 main database immutable/read-only to verify technical migration/runtime tables and manifests; it performs no write or checkpoint and never retrieves, displays, logs, or interprets personal content fields. Existing migration policy remains unchanged.

## Provenance Impact

None to product artifacts, revisions, dependencies, authorship, or migration receipts. The recovery receipt is operational filesystem evidence with a separate classification.

## Historical Context Impact

None. ADR-0009 retrieval, selection, preflight, consent, packet, transmission, actual-use provenance, and cascades remain byte/behavior unchanged.

## Consent Impact

One session claim binds read-only recovery preflight to one explicit recovery invocation. It is never persisted as migration consent. The existing migration button remains a new action.

## Provider Transmission Impact

None. No provider call, model, key, prompt, or ContextPacket path is touched.

## Import And Export Impact

None. Export v1 and partial import remain unchanged; recovery receipt is not added to personal export.

## Test Strategy

- Build synthetic v4 app-like roots only; assert test roots are not the ordinary app-data path.
- Cover all 30 requested matrix rows with named Rust/frontend/package assertions: exact success; digest/schema/operation/staging/backup/receipt/DDL/file/WAL/SHM/journal/activity/TOCTOU/path failures; every mutation boundary; cancel; content-free receipt; database bytes; restart; consent separation; second migration/ambiguity/old-v4/persistent-WAL regressions; no real path/Android/content leak.
- Use a real SQLite-generated persistent-WAL sidecar fixture where deterministic, plus fixed malformed synthetic WAL/SHM variants.
- Build a fully migrated synthetic schema-v5 fixture with a valid `v5_ready` operation, verified v4 backup and exact prior prepared-recovery receipt/quarantine; cover exact A-R3 success, read-only cancellation, claim/contract drift, protected-file byte preservation, strict two-file receipt/quarantine, and every meaningful partial-failure boundary.
- Render A-R3-specific English, Traditional Chinese and Japanese disclosure separately from prepared-v4 copy; assert moved/protected facts, no migration/restore action, restart-only success, and long-value containment.
- Focused commands: frontend panel/i18n/storage tests; Rust prepared-recovery and activation tests; `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings`.

## Repository Verification Strategy

Run `git diff --check`, focused tests, warnings-denied Clippy, then `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Reconcile actual changed paths with this allowlist and workflow state. Re-run canonical after any correction.

## Manual UI Verification

Owner: Founder. Build one ignored unsigned ordinary-identity package using a unique R2 suffix. Do not install or launch automatically. Present Manual Phase A one bounded step at a time; only after it passes ask the exact Phase B disposition question. Phase C remains a second exact question after Phase B success.

## Rollback Or Recovery Strategy

Before any real action, rollback is repository-only. Disposable recovery failures preserve the database and all remaining/quarantined evidence and are never retried automatically. A partial mutation cannot re-enter the exact eligible predicate. No automated repair path is provided.

## Documentation Impact

Create architecture 19 and synchronize architecture 13/15/18, index navigation, and dev runbook 10. Preserve all prior workflow archives verbatim.

## ADR Impact

No new ADR and no ADR status change. This is a bounded implementation of existing local-first, user-agency, provenance, and migration recovery authority.

## Risk Level

high — the action intentionally dispositions filesystem evidence beside a personal database. Exact predicates, content-free classification, quarantine-before-delete, exclusive locking, supplied digest binding, TOCTOU revalidation, and Founder-only real gates are mandatory mitigations.

## Escalation Decision

Proceed. Escalate immediately if exact real evidence later differs, WAL has frames/cannot be proven empty, personal SQL rows would need reading, a writable/checkpoint/repair action is needed, or any higher-authority/product boundary changes.

## R2B bounded correction plan — 2026-09-02

1. Add exact source-v4 manifests for fixture, promoted Rust runtime and the
   historical frontend/Rust hybrid; bind `ExactV4CandidateVerifier` to that
   allowlist so unknown schema-object representations fail pre-commit.
2. Add the exact third derived schema-v5 manifest to runtime/post-commit
   verification without generic SQL normalization.
3. Extend ordinary prepared recovery with classification
   `exact_historical_frontend_v4_post_commit_manifest_v1`, claim-bound
   inspection, exact operation/receipt/backup/prior-evidence checks, no-sidecar
   requirement, guarded lifecycle activation, create-new content-free receipt,
   and `v5_ready` operation finalization.
4. Route startup to this recovery with backup visible but restore unavailable;
   add English, Traditional Chinese and Japanese disclosure with no
   migration/restore/delete controls.
5. Add synthetic historical direct-migration/typed-write, unknown-source
   refusal, exact blocked recovery, cancellation, claim drift, preserved-file,
   content-free receipt, restart and UI tests.
6. Synchronize architecture 13/15/18/19, runbook 10 and workflow artifacts;
   run focused checks, warnings-denied Clippy and canonical verification.
7. Build exactly one ignored unsigned package with a unique R2B suffix. Do not
   install or launch it. Stop at disposable Founder manual review.

## R2B execution status — 2026-09-02

Repository steps 1–7 are complete. The lifecycle transition now obtains
`BEGIN IMMEDIATE` before repeating the complete exact committed-v5 verifier on
the same connection; a schema-drift regression proves rollback occurs before
mutation. Focused checks, warnings-denied Clippy and canonical verification
pass. The one ignored package and six-field manifest are verified. No manual,
real-profile, Git/promotion, release, Phase 4 or Android step is opened by this
status.

## R2B Founder-evidence synchronization plan — 2026-09-14

1. Hash-bind the five Founder-supplied BF8 artifacts and record their exact
   content-free result.
2. Synchronize only the existing R2/R2B architecture, runbook and workflow
   artifacts; make no implementation or package change.
3. Run focused workflow/package checks, diff hygiene and the canonical
   repository verifier.
4. Reconcile the exact diff and Git state without staging or committing.
5. Prepare one explicit local promotion decision, but do not resolve or execute
   it. Keep real-profile disposition and Android M0 parked.

No application launch, installation, profile action, new package, promotion,
Git publication, release, Phase 4 or Android action is part of this plan.
