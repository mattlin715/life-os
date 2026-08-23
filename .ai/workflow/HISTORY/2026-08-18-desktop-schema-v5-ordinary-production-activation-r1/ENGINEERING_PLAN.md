# Engineering Plan

Status: approved

- Sprint ID: 2026-08-18-desktop-schema-v5-ordinary-production-activation-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-18T11:05:00+09:00
- Updated at: 2026-08-18T20:28:00+09:00

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement ordinary desktop schema-v5 activation using the promoted Candidate core, keep migration explicit and fail closed, preserve all Product Harness behavior, access no real profile, produce only an ignored unsigned review package, and stop before installation. Real-profile Phase B, distribution, deployment, release, Phase 4, Android, provider, ContextPacket, and consent changes remain excluded.

## Existing Implementation Understanding

- `schema_v5_founder_activation.rs` is the current Tauri/app-data adapter around promoted filesystem, backup, migration, restore, and typed runtime modules. It is identity-gated to `com.lifeos.founderdogfood` and uses one process-local exclusive-operation lock.
- `schema_v5_migration.rs`, `filesystem_safety.rs`, and the artifact-specific writer modules contain the accepted database contracts; they must not be forked.
- `createLocalEvidenceStore.ts` selects either the schema-v4 plugin-SQL store or Founder v5 typed store. The v5 path never constructs plugin SQL.
- Existing exact v4 initialization/migration remains in `sqlite.rs`; it can stabilize v2/v3 to exact v4 without exposing v5 DDL.
- App migration/backup UI exists but is Founder-named. The same behavioral surface can be made profile-aware without changing migration semantics.
- Candidate packaging already proves unsigned Windows GUI, isolated identity, ignored artifacts, and content-free manifests. The ordinary review package needs a separate contract because it intentionally uses `com.lifeos.app`.

## Affected Modules

1. Rust desktop activation adapter and Tauri registration: `src-tauri/src/schema_v5_founder_activation.rs`, `src-tauri/src/lib.rs`, `src-tauri/src/sqlite.rs`.
2. Build/version policy: `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/tauri.conf.json`, `package.json`, `pnpm-lock.yaml`, Vite environment declarations, and one ordinary review override.
3. TypeScript startup/store routing: `src/shared/storage/createLocalEvidenceStore.ts`, schema-v5 adapter/store files and tests.
4. UI/i18n: `src/app/App.tsx`, migration/backup panels and tests, `src/app/i18n.ts`, `src/app/i18n.test.ts`.
5. Packaging/verification: new ordinary review build and contract scripts/tests, Candidate regression contracts, `scripts/verify.ps1`, and `.github/workflows/check.yml` if required for parity.
6. Factual Book One: index, architecture/13, /15, /16, /17, a new architecture/18 implementation boundary, and desktop review runbooks.
7. Repository workflow artifacts for this sprint only.

## Proposed Design

### Shared core and identity policy

Keep exactly one activation implementation. Extend the current adapter into a desktop identity-policy core that accepts only an immutable compiled identity pair: ordinary `com.lifeos.app` under the ordinary schema-v5 build feature, or isolated `com.lifeos.founderdogfood` under the Founder feature. Add ordinary command wrappers that delegate to the same internal functions; retain Founder wrappers for regression compatibility. No renderer-supplied path, SQL, schema, operation ID, or policy flag is accepted.

The 0.3.0 application must write its actual package version into new migration receipts while continuing to accept already-promoted exact 0.2.0 Candidate receipts whenever they satisfy the unchanged `database_contract.minimum_application_version = 0.2.0` and every other immutable receipt/manifest predicate. Receipt compatibility is a bounded verifier change, not a DDL or minimum-version change.

### Startup

- Default desktop 0.3.0 builds compile the ordinary v5 feature and frontend gate coherently.
- Founder package remains separately gated by its identity and feature/environment pair.
- Missing database -> shared exact fresh-v5 initializer.
- Existing exact v4 -> migration-required, no writable store.
- Existing v2/v3 ordinary database -> call the existing typed Rust v4 initializer once, close it, re-inspect, then display the separate v4-to-v5 migration-required state.
- Exact verified v5 -> typed v5 store.
- Newer, malformed, sidecar, contradictory, incomplete, or ambiguous evidence -> blocked and no store.

### Migration and recovery

Reuse exact-owned operation creation, backup, two-manifest predicate, `BEGIN IMMEDIATE` migration, `user_version=5` last, immutable receipt, lifecycle activation, durable classification, delete-now, and explicit restore. Preserve no-autonomous-action semantics. Add profile kind to the bounded state so UI can render truthful ordinary versus isolated disclosure without revealing a path.

### Runtime routing

Use one TypeScript schema-v5 store with a compile-time command namespace selected by the build; all operations remain typed Rust commands. Current product actions and ADR-0009 use the promoted runtime functions. Schema-v4 remains available only to the old/no-default-feature compatibility build and the explicit v2/v3 stabilization command.

### Readiness inspection

Keep Inspector R1 read-only and explicit-open, but make its reported supported maximum and exact-schema classification match the compiled desktop capability. Ordinary and Founder schema-v5 builds report maximum 5 and can classify exact v5; no-default-feature compatibility builds continue to report maximum 4. This correction adds no migration, retry, repair, cleanup, or other write authority.

### Review package

Add an ordinary-identity review override with unmistakable disposable-review product/window title. Its build script verifies source contracts, compiles the ordinary feature/frontend gate, bundles NSIS without signing, copies only the installer into an ignored fresh SHA directory, writes the six-field content-free manifest, and never installs or launches.

## Alternatives Considered

- Duplicate the Founder module for ordinary: rejected because DDL/state/backup/runtime drift would create two authorities.
- Change only the Tauri identifier in the Founder package: rejected because it would not prove ordinary default build/version/startup policy and would preserve misleading Founder copy.
- Silently migrate exact v4 during startup: rejected by explicit user agency and accepted cutover policy.
- Keep ordinary schema v4 and only ship another isolated Candidate: rejected because it does not meet the authorized ordinary activation objective.
- Add Android/mobile abstractions now: rejected as speculative and explicitly fenced.

## Data Lifecycle Impact

Ordinary disposable profiles may create exact v5, append immutable revisions/events, create one exact-owned pre-migration backup, retain its disclosed 30-day deadline, delete it explicitly, or explicitly restore it to exact v4. No automated work touches real profiles. Restore loses post-migration local changes and is separately disclosed. No destructive down migration is added.

## SQLite Or Migration Impact

Production-capable ordinary startup maximum becomes exact v5 through the new activation policy, while the legacy `sqlite.rs` contract remains exact v4 for old-binary refusal and v2/v3 stabilization. Fixed v5 DDL and every promoted migration/runtime writer remain unchanged unless a verified integration defect requires a bounded correction. No new schema objects or DDL policy are planned.

## Provenance Impact

No provenance meaning changes. Migration preserves honest `legacy_v4_baseline`; new v5 writes use the existing exact authorship/provider/model/Harness/prompt/source dependency contracts. Package/workflow evidence stays content-free.

## Historical Context Impact

No retrieval, eligibility, selection, packet, or generated-output change. The ordinary v5 route uses the promoted ADR-0009 parity writer and cascade logic.

## Consent Impact

No historical consent policy change. Local migration authorization is per operation and distinct from provider consent. Cancel must remain write-free.

## Provider Transmission Impact

None. Provider adapters, request contents, destination disclosure, `store:false`, fallback, and output evaluators remain unchanged.

## Import And Export Impact

Current Experience-only import/export behavior remains honestly labelled. Typed v5 Experience import is routed; export v2 and complete artifact import remain deferred.

## Test Strategy

- Rust: identity/build-policy refusal, missing/exact-v4/exact-v5/older/newer/malformed classification, v2/v3 stabilization, cancel/no-write, backup/migration/restart, sidecars/quiescence/path/disk/permission, ambiguous commit, retention/delete, restore, typed runtime routes, stale revisions, ADR-0009, and old-v4 refusal.
- TypeScript/Vitest: strict state validation, ordinary/Founder command namespace, startup selection, v2/v3 stabilization, blocked state, store routing, three-language copy, migration/cancel/backup/restore panels, and existing reflection regressions.
- Package: exact ordinary identity, version 0.3.0, review-only title, ordinary feature/frontend gate, GUI subsystem, content-free manifest, ignored output, no Android surface, and Founder Candidate regression.
- Use synthetic/disposable databases and roots only.

## Repository Verification Strategy

Run focused Vitest and Rust tests, `cargo clippy --all-targets --all-features -- -D warnings`, package contract tests, then `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1` with a long timeout. Compare actual tracked and non-ignored untracked paths with the anticipated allowlist. Verify no Constitution diff, no Android path, and no real-profile path in source logs/manifests.

## Manual UI Verification

Owner: Founder. After automation and package build, stop before installation and guide Manual Phase A one bounded step at a time in a disposable Windows account/VM/Sandbox. Phase B remains a separate explicit decision after Phase A passes.

## Rollback Or Recovery Strategy

Code rollback before promotion is ordinary file reversion only; no database action is taken automatically. Runtime failure preserves exact evidence and blocks. Migration backup restore is explicit and exact-owned. Release rollback must be forward-fix or read-only; no schema decrement. The review installer remains ignored and non-distributed.

## Documentation Impact

Record Candidate promotion truth, clarify ordinary versus Founder states, add architecture/18 for the implemented ordinary boundary and threat model, synchronize 0.3.0 and the review-package runbook, and leave historical workflow archives unchanged.

## ADR Impact

No new ADR. ADR-0007, ADR-0009, ADR-0011 and Founder-approved architecture/13 already govern the data model, consent/provenance, lifecycle, migration, backup, restore, and non-destructive rollback. Architecture/18 applies them to the ordinary desktop.

## Risk Level

High. The change makes schema-v5 database writes reachable under the ordinary identity and introduces explicit whole-database migration/restore controls. Risk is bounded by pre-writable inspection, exact identity, disposable automation, one shared core, explicit authorization, verified backup, durable evidence, and a mandatory one-step manual gate.

## Escalation Decision

No escalation is required before implementation. Stop if real data, destructive down migration, policy change, or a fourth revision cycle becomes necessary.

## Exact Anticipated File Allowlist

- `.ai/workflow/CURRENT_MISSION.md`
- `.ai/workflow/PRODUCT_REVIEW.md`
- `.ai/workflow/ENGINEERING_PLAN.md`
- `.ai/workflow/ENGINEERING_REPORT.md`
- `.ai/workflow/THEORY_ALIGNMENT_REVIEW.md`
- `.ai/workflow/SPRINT_REPORT.md`
- `.ai/workflow/WORKFLOW_STATE.json`
- `.ai/workflow/EVENTS.jsonl`
- `.ai/workflow/HISTORY/2026-08-18-desktop-schema-v5-ordinary-production-activation-r1/**`
- `docs/00_Index.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`
- `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md`
- `docs/architecture/17_Desktop_Schema_v5_Founder_Dogfood_Activation_Candidate_R1.md`
- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `docs/dev/09_Windows_Founder_Dogfooding_Package_R1.md`
- `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
- `package.json`
- `pnpm-lock.yaml`
- `src/vite-env.d.ts`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/tauri.conf.json`
- `src-tauri/tauri.ordinary-schema-v5-review.conf.json`
- `src-tauri/src/lib.rs`
- `src-tauri/src/sqlite.rs`
- `src-tauri/src/schema_v5_founder_activation.rs`
- `src-tauri/src/schema_v5_migration.rs`
- `src-tauri/src/schema_v5_experience_write.rs`
- `src/shared/storage/createLocalEvidenceStore.ts`
- `src/shared/storage/createLocalEvidenceStore.test.ts`
- `src/shared/storage/sqlite/founderSchemaV5.ts`
- `src/shared/storage/sqlite/founderSchemaV5.test.ts`
- `src/shared/storage/sqlite/founderSchemaV5LocalEvidenceStore.ts`
- `src/shared/storage/sqlite/founderSchemaV5LocalEvidenceStore.test.ts`
- `src/shared/storage/sqlite/databaseReadiness.ts`
- `src/shared/storage/sqlite/databaseReadiness.test.ts`
- `src/shared/storage/sqlite/README.md`
- `src/app/App.tsx`
- `src/app/DatabaseReadinessPanel.tsx`
- `src/app/DatabaseReadinessPanel.test.tsx`
- `src/app/FounderSchemaV5MigrationPanel.tsx`
- `src/app/FounderSchemaV5MigrationPanel.test.tsx`
- `src/app/FounderSchemaV5BackupPanel.tsx`
- `src/app/FounderSchemaV5BackupPanel.test.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `scripts/start-life-os.ps1`
- `scripts/build-founder-schema-v5-candidate.ps1`
- `scripts/founder-dogfood-package.mjs`
- `scripts/founder-schema-v5-candidate-package.mjs`
- `scripts/founder-schema-v5-candidate-package.node-test.mjs`
- `scripts/build-ordinary-schema-v5-review.ps1`
- `scripts/ordinary-schema-v5-review-package.mjs`
- `scripts/ordinary-schema-v5-review-package.node-test.mjs`
- `scripts/verify.ps1`
- `.github/workflows/check.yml`

If implementation evidence requires another file, return to Engineering Planning and update this allowlist before editing it.
