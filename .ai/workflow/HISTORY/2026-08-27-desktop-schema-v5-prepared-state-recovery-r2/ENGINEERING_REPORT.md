# Engineering Report

Status: completed

- Sprint ID: 2026-08-27-desktop-schema-v5-prepared-state-recovery-r2
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899
- Working-tree digest implemented: a84c6cf19ef306fda74e98e67c2d42c362163011e3ce8c6b7a5bc5b0c85f5d1f
- Created at: 2026-08-27T19:15:00+09:00
- Updated at: 2026-09-01T02:03:18+09:00

## Implementation Summary

Implemented two ordinary-desktop-only, explicit-open recovery classifications. The original path covers the exact schema-v4 Prepared operation with zero staging and proven empty WAL/matching zero-frame SHM. Founder-selected Option A-R3 adds only the exact schema-v5 `v5_ready` operation with unchanged verified v4 backup, valid prior prepared-recovery receipt/quarantine, zero-byte WAL, and matching zero-frame SHM created by historical schema-v4 refusal. Both paths bind typed preflight claims, exclude concurrent SQLite activity, revalidate immediately, quarantine only approved transient files, write strict content-free receipts, preserve database bytes, and require close/restart. Manual A-R3 recovery passed, but exposed that the completion control did not close the window because the Tauri close capability was absent and its rejected Promise was discarded. Correction cycle 4 grants only the main-window close permission, awaits the close request, and preserves the completed recovery result while showing a distinct close-only error if native close fails. All autonomous verification used synthetic/disposable roots; the real ordinary profile was not accessed.

## Existing System Areas Inspected

- `AI_CONTRIBUTOR_GUIDE.md`, Constitution and routed privacy/AI/memory/reflection definitions.
- `docs/architecture/01_Local_Evidence_Store.md`, architecture 13/15/18, ADR-0007, ADR-0009 and ADR-0011.
- `src-tauri/src/filesystem_safety.rs`, `schema_v5_founder_activation.rs`, `schema_v5_migration.rs`, `sqlite.rs`, and Tauri command registration.
- `src/shared/storage/sqlite/founderSchemaV5.ts`, startup/store adapters, migration/backup panels, application startup routing and i18n.
- R1 ordinary review runbook and current workflow/Harness state.

## Files Added

- `src-tauri/src/schema_v5_prepared_recovery.rs`
- `src/app/PreparedStateRecoveryPanel.tsx`
- `src/app/PreparedStateRecoveryPanel.test.tsx`
- `src/app/closeLifeOs.ts`
- `src/app/closeLifeOs.test.ts`
- `docs/architecture/19_Desktop_Schema_v5_Prepared_State_Recovery_and_Real_Profile_Migration_R2.md`

## Files Modified

- `src-tauri/Cargo.toml`
- `src-tauri/src/filesystem_safety.rs`
- `src-tauri/src/schema_v5_founder_activation.rs`
- `src-tauri/src/schema_v5_migration.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/capabilities/default.json`
- `src-tauri/gen/schemas/capabilities.json`
- `src/shared/storage/sqlite/founderSchemaV5.ts`
- `src/shared/storage/sqlite/founderSchemaV5.test.ts`
- `src/shared/storage/createLocalEvidenceStore.test.ts`
- `src/app/App.tsx`
- `src/app/FounderSchemaV5BackupPanel.test.tsx`
- `src/app/FounderSchemaV5MigrationPanel.tsx`
- `src/app/FounderSchemaV5MigrationPanel.test.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `src/styles.css`
- `docs/00_Index.md`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`
- `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
- `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
- `scripts/founder-dogfood-package.mjs`
- `scripts/ordinary-schema-v5-review-package.mjs`
- `scripts/ordinary-schema-v5-review-package.node-test.mjs`
- current sprint workflow artifacts under `.ai/workflow/`

## Files Deleted

none.

## Behavior Changed

For ordinary desktop schema-v5 builds, bounded startup refusals can open one neutral recovery entry point. Prepared-v4 eligibility still quarantines only state/staging/WAL/SHM. A-R3 eligibility requires exact schema v5, one internally consistent `v5_ready` operation, unchanged verified backup and prior recovery evidence, zero-byte WAL and matching zero-frame SHM. Its explicit action quarantines only WAL/SHM, writes a strict content-free receipt with moved/protected facts, and re-verifies database/operation/backup contracts unchanged. Unsupported states never expose an action. Both success paths return restart-required and never invoke migration or restore. The restart/close control now has the exact Tauri close capability, awaits native close, and reports a localized close-only failure without changing the completed recovery state or inviting a second recovery action.

## Data Model Impact

No product data model or personal artifact schema changed. Operational receipt schemas store only classification, operation identifier, app/database schema versions, UTC timestamp, and relative file names/sizes/SHA-256 digests. The prepared-v4 quarantine contains four transient files; the A-R3 quarantine contains only the exact WAL/SHM pair and its receipt records eight protected technical file facts. Neither contains a database copy or personal-row content.

## Migration Impact

No DDL, user-version sequence, backup, migration receipt, projection, restore or migration-policy behavior changed. A private immutable verifier is available only after the A-R3 classifier proves the exact empty sidecar pair; normal runtime verification still refuses every sidecar. Recovery does not authorize or start migration. Existing ordinary v4-to-v5 migration remains a separate invocation after restart.

## Provenance Impact

No Experience, Evidence, Reflection, Pattern, Context Recovery, historical-question, revision, dependency or authorship provenance changed. The operational recovery receipt is outside product artifact provenance and has its own strict classifier.

## Historical Context Impact

None. Retrieval, relevance disclosure, ephemeral selection, governed preflight, provider packet, actual-use provenance and cascade behavior remain unchanged.

## Consent Impact

Adds one explicit recovery action bound to the current content-free claim. Recovery consent is session-bounded and is neither persisted nor reused as migration consent. Migration still requires its existing separate explicit action.

## Provider Transmission Impact

None. No provider, model, API key, prompt, ContextPacket, consent-event or transport code changed.

## Tests Added

- Twelve Rust recovery tests: the original exact success, cancellation byte identity, malformed/ambiguous refusal, 30-case fail-closed matrix, TOCTOU claim change, strict receipt/deletion, injected boundaries and coordination-lock refusal, plus A-R3 exact success, write-free inspection/cancel, claim/contract drift refusal, protected-file preservation and partial-failure checks.
- Ten recovery-panel assertions covering prepared-v4 and A-R3 English, Traditional Chinese and Japanese disclosure, moved/protected facts, ineligible no-action behavior, restart-only consent separation, no migration/restore action, and single-column/long-value wrapping.
- Strict typed-adapter validation now requires both moved and preserved technical file arrays; migration/backup panel regressions remain covered.
- Three close-port tests prove awaited desktop close, propagated rejection and browser fallback; panel/i18n tests prove a close-only failure is distinct from recovery failure in English, Traditional Chinese and Japanese.
- Ordinary-package contract tests prove the source and generated capability files each contain the exact main-window close permission.

## Tests Executed

- `cargo test --manifest-path src-tauri/Cargo.toml --features desktop-schema-v5 schema_v5_prepared_recovery`: passed, 12/12.
- `cargo test --manifest-path src-tauri/Cargo.toml --features desktop-schema-v5`: passed, 216/216 plus backup 12/12 and schema contract 8/8.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --features desktop-schema-v5 -- -D warnings`: passed.
- `pnpm run typecheck`: passed.
- targeted recovery/adapter frontend tests: passed, 15/15; TypeScript passed.
- full Vitest after A-R3: passed, 362/362.
- `git diff --check`: passed; only Git line-ending notices were emitted.
- `node --test scripts/founder-dogfood-package.node-test.mjs`: passed, 8/8 after correction-cycle synchronization.
- correction-cycle-4 focused Vitest (`closeLifeOs`, prepared recovery panel and i18n): passed, 33/33.
- `node --test scripts/ordinary-schema-v5-review-package.node-test.mjs`: passed, 3/3.
- correction-cycle-4 `pnpm run typecheck`: passed.

## Verification Results

Earlier focused Rust, Clippy, TypeScript and frontend checks passed. The first canonical run failed because the historical Founder-package contract had not yet allowlisted the four new R2 successor paths; correction cycle 1 synchronized only that existing active-successor list. Manual Phase A correction cycle 2 then passed canonical with workflow contracts 17/17, package contracts 11/11, Vitest 357/357, typecheck/build, 212 default Rust tests, backup 12/12, schema contract 8/8, legacy-v4 compatibility/refusal, ordinary activation, Founder activation/runtime, whitespace, UTF-8, secret-file, Markdown-link and Constitution checks.

Option A-R3 verification passes: recovery module 12/12, full schema-v5-feature Rust 216/216 plus backup 12/12 and schema-contract 8/8, TypeScript, full Vitest 362/362, warnings-denied Clippy, `git diff --check`, workflow validation, and the complete repository-owned canonical command. The final exact diff audit also passes. One ignored unsigned successor package was built and verified without installation or launch.

After synchronizing Option A-R3, the complete canonical command passed on 2026-08-30: workflow 17/17, package contracts 11/11, Vitest 362/362, typecheck/build, Rust 216/216 plus backup 12/12 and schema contract 8/8, legacy-v4 compatibility/refusal, ordinary/Founder activation and runtime checks, whitespace, UTF-8, secret-file, Markdown-link, and Constitution checks. The A-R3 implementation adds `src-tauri/src/schema_v5_migration.rs` to the revised exact 33-path allowlist.

Correction cycle 4 canonical verification passed on 2026-08-31: workflow 17/17, Founder package 8/8, Founder candidate 1/1, ordinary package 3/3, Vitest 369/369, TypeScript/build, Rust 216/216 plus backup 12/12 and schema contract 8/8, legacy-v4 compatibility/refusal, ordinary/Founder activation/runtime, and repository-safety checks. One ignored unsigned successor package was built and its manifest verified: `Life-OS-Ordinary-Schema-v5-Review-R1-Review-prepared-recovery-r2-close-control-cycle4_0.3.0_x86_64-pc-windows-msvc-setup.exe`, size `5779951`, SHA-256 `DA357971D79262495789A20D4441B4F16BB805F8F4EDC41626E9D747D7ED6F1B`. It was installed and launched only in the disposable `LifeOSReviewR1` account; it was not distributed, deployed, or released.

After synchronizing the completed disposable manual evidence, the same complete canonical command passed again on 2026-09-01 against the final exact 40-path working tree: workflow 17/17, Founder package 8/8, Founder candidate 1/1, ordinary package 3/3, Vitest 369/369, TypeScript/build, Rust 216/216 plus backup 12/12 and schema contract 8/8, all legacy/ordinary/Founder compatibility and runtime checks, and every repository-safety check.

## Manual Verification

Founder-owned disposable Manual Phase A is complete. Correction cycle 4 proved installation preserved the completed profile, normal runtime reconstructed both records and the managed backup, and an independently reconstructed clone could exercise the corrected controls without repeating recovery against the completed source. **Preserve and close** closed natively with no mutation. One explicit clone recovery produced exactly one strict receipt and exact WAL/SHM quarantine while preserving database/operation/backup/staging/prior-recovery evidence. The corrected success button closed natively, restart reconstructed both records without reused consent, and normal close left no live sidecars. The completed source profile remained byte-identical during the clone exercise, was restored to the active disposable identity, and passed a final restart/close. The Founder accepted the exact 40-path diff and this completed disposable evidence as Option A on 2026-09-01. Real-profile Phase B, migration Phase C and promotion remain separate unauthorized gates.

## Documentation Updates

Created architecture 19 and synchronized architecture 13, 15, 18, the Book One index, and the ordinary Windows review runbook. Prior workflow archives were not rewritten.

## ADR Impact

No new ADR and no ADR status or policy change. ADR-0007 persistence/provenance and ADR-0009 historical consent boundaries remain unchanged; ADR-0011 sequencing remains enforced.

## Deviations From Plan

Correction cycle 1 added `scripts/founder-dogfood-package.mjs` to the exact allowlist solely to synchronize its pre-existing active-successor source list. Correction cycle 2 added only `src/styles.css` for disclosure containment. Founder-selected correction cycle 3 adds `src-tauri/src/schema_v5_migration.rs` to expose a narrowly named immutable verifier only after exact empty-sidecar proof, extends the existing recovery module/typed panel/i18n/tests/docs, and does not change DDL, migration, backup, restore, provider, consent or product-data behavior. Manual A-R3 exposed one capability/UI defect after the recovery itself completed correctly. Correction cycle 4 therefore adds only the exact Tauri main-window close capability, an awaitable close port, visible localized close-only failure handling, focused tests, and package-contract parity. It does not alter recovery classification, filesystem action, schema, migration, backup, restore, provider, consent or product-data behavior. The planned Windows `Win32_System_IO` feature remains the only dependency-surface adjustment.

## Known Limitations

Only the three exact classifiers are recoverable: prepared-v4 empty-sidecar,
A-R3 v5-ready empty-sidecar, and the historical-frontend post-commit manifest
predicate. A-R3 additionally requires internally consistent committed-v5,
`v5_ready`, verified-backup and prior-recovery contracts. Non-empty/framed/
ambiguous WAL, malformed SHM, journal, unexpected/changed operation or recovery
files, missing/changed backup or migration evidence, destination conflict,
links/reparse/hard links, changed claim, or activity mismatch remain
recovery-required with no cleanup control. A partial mutation failure preserves
quarantine evidence and does not retry or roll back. Quarantine deletion is
intentionally not provided.

## Remaining Risks

The correction-cycle-2 installer passed through Step A11-4 before Step A12 exposed the exact legacy-refusal sidecars. A-R3 recovery, strict receipt/quarantine evidence, correction-cycle-4 native close controls and restart reconstruction are now Founder-verified in the disposable account. The separately reconstructed clone and restored completed profile remain isolated review evidence. No automatic cleanup, ignore, checkpoint, repair or retry is acceptable. Real-profile evidence may differ from every synthetic predicate; real disposition and real migration remain separately unauthorized. Android remains blocked.

## Git State

Branch `codex/desktop-schema-v5-real-profile-recovery-r2` at HEAD `8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899`; working tree intentionally modified, no staged files, no feature-branch upstream, no commit/push/merge/PR. Current product/docs implementation plus workflow artifacts occupy the revised exact 40-path ceiling after bounded Manual Phase A correction cycle 4.

## Engineer Completion Status

completed — the exact 40-path diff and disposable Manual Phase A evidence were accepted by the Founder. No later authority was granted.

## R2B implementation addendum — 2026-09-02

Repository implementation now binds exact schema-v4 source representation,
accepts the exact historical derived schema-v5 manifest, and adds the
post-commit recovery classification/action described in the approved addendum.
The action preserves the verified backup, zero staging and prior recovery
receipt/quarantine; it changes only lifecycle writes and the bound operation
state, then writes a strict content-free receipt. Startup exposes recovery but
not restore for the exact state. UI copy and tests cover all three languages
and omit migration, restore and backup deletion.

Focused evidence currently passed:

- TypeScript typecheck;
- 22 focused frontend adapter/panel tests;
- exact unknown-source-representation refusal;
- historical frontend-v4 direct migration plus typed write; and
- 16 prepared-recovery Rust tests, including exact post-commit recovery,
  write-free cancel, startup reason/restore-unavailable, claim drift,
  preserved evidence, content-free receipt, restart verification, typed write,
  and a write-locked schema-drift refusal that preserves database bytes.

Canonical verification, final diff reconciliation and the single ignored
unsigned R2B package now pass. Canonical evidence includes workflow `17/17`,
package contracts `12/12`, Vitest `373/373`, feature Rust `222/222`, backup
`12/12`, schema contract `8/8`, legacy refusal `33/33`, ordinary activation
`10/10`, Founder activation `10/10`, runtime `4/4`, typecheck, frontend build,
Rust check, whitespace, UTF-8, secret-file, Markdown-link and Constitution
checks. Warnings-denied Clippy also passed separately.

The one ignored package is
`8173d4fe76ee-r2b-post-commit-manifest-lock` / `5805550` bytes / SHA-256
`30ec0a075f79728b6c87a2f9ba9c5ed3ec1d7188d85b96650fc23cd8715ade9b`.
Its strict six-field manifest was independently matched. No real profile was
accessed, installed build launched, or Git/promotion action performed during
this correction. Engineering stops at the disposable Founder manual-review
gate.

## R2B disposable Founder evidence — 2026-09-14

The BF1-BF8 disposable matrix and Founder acceptance are now recorded. BF8
passed with one application launch, one authorized synthetic Experience write,
zero provider transmissions, zero migration attempts and zero recovery
executions during the restart/write step. The exact Experience digest is
`dcc220682c7fdc3096faee9de5eb3da2255127dda0290901518bb40fe506a6ae`;
its revision is 1, predecessor is SQL NULL, authorship is user, and provider
metadata is SQL NULL.

The operation remains `06c19891cf629f9075f2e349780950c4` at `v5_ready` /
`lifecycle_writes_enabled`. The post-commit receipt, migration receipt, verified
schema-v4 backup, zero staging, prepared-recovery receipt/quarantine, ten
retained profiles and installed binary remained unchanged, with no live SQLite
sidecars or Life OS process after close. This synchronization cycle changes no
implementation behavior and creates no package.

Fresh 2026/09/14 focused verification passed: workflow contracts `17/17`,
ordinary review package contracts `3/3`, TypeScript typecheck and diff hygiene.
The canonical repository verifier then passed workflow `17/17`, package
contracts `8/8 + 1/1 + 3/3`, Vitest `373/373`, feature Rust `222/222`, backup
`12/12`, schema contract `8/8`, legacy refusal `33/33`, ordinary activation
`10/10`, Founder runtime `4/4`, typecheck, frontend build, Rust checks,
whitespace, UTF-8, secret-file, Markdown-link and Constitution checks.

The workflow event/state pair remains at the earlier resolved Manual-A
`human_decision_required` state. Its recorded verification predates R2B, its
resume phase is `theory_alignment_review`, and the current workflow kernel both
requires fresh verification to enter that phase and permits recording it only
from `validation`. No event/state file was hand-edited and no workflow-kernel
behavior was changed. The new promotion decision is therefore prepared in the
decision artifact but not activated or resolved in machine state.

## R2B workflow-control correction — 2026-09-14

The stale-verification deadlock was reproduced without changing the active
state: the resolved Manual-A gate could not enter `theory_alignment_review`
because repository verification was stale, while `record-verification`
correctly refused every phase except `validation` and the resolved gate could
resume only to `theory_alignment_review`.

The bounded correction adds the explicit CLI operation
`refresh-verification-for-resume`. It is accepted only from
`human_decision_required` when the current decision is resolved, its exact
non-empty evidence reference matches the command, its resume phase is
`theory_alignment_review`, the current event sequence matches, and the
canonical verifier passed with exit code zero. The operation appends one
`resume_verification_refreshed` event, captures HEAD, branch and the
non-workflow working-tree digest, atomically updates the state projection, and
leaves the status, active decision IDs, exact Founder response and resume phase
unchanged. Failed/skipped results, unresolved or differently routed decisions,
stale sequences, reference mismatch and later repository mutation all fail
closed.

Exact control-plane files added to the R2 diff are
`.ai/workflow/WORKFLOW.md`, `scripts/ai-workflow.mjs`, and
`scripts/ai-workflow.node-test.mjs`. Canonical verification initially exposed
one directly related compatibility gate: the historical Founder package source
allowlist rejected those three new paths. Only those three paths were added to
the existing allowlist in `scripts/founder-dogfood-package.mjs`; package
behavior and manifest shape remain unchanged. The other thirty original
non-workflow R2 paths remain byte-identical to the captured 40-path baseline.

Focused workflow tests pass `28/28`, including the fifteen required
deadlock/guard/freshness/atomicity/immutability/resume/new-decision behaviors.
The Founder package contract passes `8/8`, `git diff --check` passes, and the
full canonical verifier passes with workflow `28/28`, package contracts
`8/8 + 1/1 + 3/3`, Vitest `373/373`, Rust `222/222`, backup `12/12`, schema
contract `8/8`, legacy refusal `33/33`, activation/runtime checks and all
repository hygiene checks. No UI review, installer build, application launch,
profile access, product behavior, promotion or Git write occurred.

## R2B local-promotion factual closeout — 2026-09-18

- The reviewed R2B feature commit is f43c019bd2f5ad41cbe496f0bdb6010374ca17b7, with parent 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899.
- Local develop contains non-fast-forward merge 8918fe49993d3cc5e9905a33af64602ac3a511af and the bounded CRLF validator correction ea6be14363ec66f937b0de5d517d6640c628551c.
- The CRLF correction changes only scripts/ai-workflow.mjs and scripts/ai-workflow.node-test.mjs; workflow tests pass 29/29.
- Canonical verification passes from clean local develop at ea6be14363ec66f937b0de5d517d6640c628551c.
- origin/develop remains 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899 and local develop is ahead by exactly three commits. No push, PR, deployment, distribution or release has occurred.
- This closeout changes only workflow artifacts and the official generated archive. It adds no product, persistence, recovery, schema, UI, provider or consent behavior.
- No application launch, real-profile access or mutation, migration, recovery, retry, restore, repair, checkpoint, backup deletion, Phase 4 or Android action occurred.
