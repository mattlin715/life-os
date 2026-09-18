# Sprint Report

Status: completed

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-27T19:39:00+09:00
- Updated at: 2026-09-01T02:03:18+09:00

## Sprint ID

2026-08-27-desktop-schema-v5-prepared-state-recovery-r2

## Mission

Implement and verify a narrowly bounded ordinary desktop schema-v5 prepared-state recovery path through Founder diff/manual-review preparation, without touching the real profile or authorizing real disposition, real migration, promotion, Android, deployment, distribution or release.

## Starting Commit

`8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899` on promoted clean `develop`, then branch `codex/desktop-schema-v5-real-profile-recovery-r2`.

## Ending Commit Or Working-Tree State

HEAD remains `8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899`. The branch has one intentional unstaged/uncommitted exact 40-path diff after Manual Phase A correction cycle 4, no staged files, no upstream and no commit/push/merge/PR. Ignored build outputs are outside the allowlist.

## Final Status

completed — disposable Manual Phase A is complete through correction cycle 4. The exact close controls, separate-clone recovery, strict receipt/quarantine, protected evidence, restart reconstruction, restored completed profile and final sidecar-free close all passed. On 2026-09-01 the Founder accepted the exact 40-path diff and completed disposable evidence as Option A. No real-profile or promotion authority is inferred.

## Product Decision

The Founder accepted the exact implementation diff and completed disposable review only. Recovery is available solely for the exact ordinary schema-v4 Prepared predicate or the A-R3 exact schema-v5 `v5_ready` legacy-refusal predicate. The latter preserves database/operation/backup/prior recovery evidence and quarantines only proven-empty WAL/SHM. Recovery, migration, restore and backup remain separate explicit actions. All unsupported/changed states fail closed without cleanup.

## Engineering Summary

Added strict prepared-operation and A-R3 post-migration inspection, content-free claim binding, Windows handle/share and SQLite coordination-range exclusion, immediate revalidation, write-through four-file or exact two-file quarantine, strict content-free receipts and protected-file proof. Added typed Tauri/renderer adapters, neutral blocked-startup entry point, classification-specific EN/zh-TW/ja UI, restart separation, tests and architecture/runbook synchronization. Correction cycle 4 grants only the main-window close command, awaits it, and shows a distinct localized close-only error without changing completed recovery state.

## Behavior Changed

The ordinary schema-v5-capable app can explicitly review and recover only the retained pre-backup Prepared/empty-sidecar state or exact A-R3 migrated legacy-refusal state. Opening is read-only. Cancel and Preserve/close mutate nothing. Prepared-v4 recovery moves four technical files and restart may show the separate migration disclosure. A-R3 moves only WAL/SHM, preserves current v5 operation/backup/prior recovery evidence, and restart returns only to v5 runtime. Neither opens SQLite writable, checkpoints, retries, migrates, restores or reuses consent. Close controls now await the authorized native close command and preserve the success result while reporting a close-only failure.

## Files Changed

Exact 40-path allowlist:

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
11. `src-tauri/src/filesystem_safety.rs`
12. `src-tauri/src/schema_v5_founder_activation.rs`
13. `src-tauri/src/schema_v5_migration.rs`
14. `src-tauri/src/schema_v5_prepared_recovery.rs`
15. `src-tauri/src/lib.rs`
16. `src/shared/storage/sqlite/founderSchemaV5.ts`
17. `src/shared/storage/sqlite/founderSchemaV5.test.ts`
18. `src/app/App.tsx`
19. `src/app/FounderSchemaV5BackupPanel.test.tsx`
20. `src/app/PreparedStateRecoveryPanel.tsx`
21. `src/app/PreparedStateRecoveryPanel.test.tsx`
22. `src/app/FounderSchemaV5MigrationPanel.tsx`
23. `src/app/FounderSchemaV5MigrationPanel.test.tsx`
24. `src/app/i18n.ts`
25. `src/styles.css`
26. `src/shared/storage/createLocalEvidenceStore.test.ts`
27. `docs/architecture/19_Desktop_Schema_v5_Prepared_State_Recovery_and_Real_Profile_Migration_R2.md`
28. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
29. `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`
30. `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
31. `docs/00_Index.md`
32. `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`
33. `scripts/founder-dogfood-package.mjs`
34. `scripts/ordinary-schema-v5-review-package.mjs`
35. `scripts/ordinary-schema-v5-review-package.node-test.mjs`
36. `src-tauri/capabilities/default.json`
37. `src-tauri/gen/schemas/capabilities.json`
38. `src/app/closeLifeOs.ts`
39. `src/app/closeLifeOs.test.ts`
40. `src/app/i18n.test.ts`

No other non-ignored path is changed.

## Founder Diff Review Summary

- Recovery core: one new Rust module plus bounded filesystem-safety, activation, migration-verifier and Tauri-command registration changes implement exact prepared-v4 and A-R3 post-migration inspection, claim binding, four-file or two-file quarantine, strict content-free receipts, protected-file proof, receipt deletion for the original path, and restart-required separation.
- Renderer/UI: strict typed adapters and startup routing expose one neutral bounded-recovery entry point; the classification-specific panel, three-language copy and scoped CSS distinguish moved/protected evidence, explicit actions, cancellation/preserve-close and wrapped long values without exposing migration or restore.
- Regression coverage: focused Rust, renderer, i18n, migration/backup and package-contract tests cover exact success, the original 30-case refusal matrix, A-R3 contract/TOCTOU drift, partial failures, protected bytes, strict receipts, consent separation and layout containment.
- Documentation/control plane: architecture 19, architecture 13/15/18, the Book One index, Windows runbook and active sprint artifacts describe the same bounded behavior and manual gates.
- Package contract: the historical active-successor allowlist now admits the exact correction paths, and ordinary-package checks require source/generated main-window close-capability parity; manifest behavior is unchanged.
- Explicit absences: no Constitution, ADR, schema-v5 DDL, product artifact model, provider, ContextPacket, consent policy, Phase 4, Android, credential, installer, manifest, profile data, deployment, distribution or release content is added.

The mechanically rechecked actual diff and this report's numbered allowlist both contain exactly 40 paths with zero missing and zero extra entries. No path is staged; no Android- or credential-like path is present.

## Tests

Current A-R3 focused:
- Rust recovery 12/12 passed.
- full desktop-schema-v5 Rust 216/216 plus backup 12/12 and schema contract 8/8 passed.
- TypeScript passed.
- targeted recovery/adapter frontend 15/15 and full Vitest 362/362 passed.
- `git diff --check` passed.
- warnings-denied Clippy, workflow/package contracts, and the complete canonical successor verification passed.
- correction-cycle-4 close-port/panel/i18n 33/33, ordinary package 3/3 and TypeScript passed.

Latest correction-cycle-4 canonical rerun:
- workflow contracts 17/17;
- package contracts 12/12 across Founder/Candidate/ordinary;
- Vitest 369/369;
- TypeScript and production frontend build;
- desktop-schema-v5 Rust 216/216;
- backup 12/12 and schema contract 8/8;
- legacy-v4 compatibility and 33 refusal/runtime tests;
- ordinary activation 9/9;
- Founder activation 9/9 and runtime 4/4;
- whitespace, UTF-8, secret-file, Markdown-link and Constitution checks;
all passed.

Correction cycle 2 canonical verification passed after its exact 32-path layout correction. That earlier ignored package was installed only in the disposable account and visually retested. Correction cycle 3 built and manually exercised the exact A-R3 package; recovery and strict postconditions passed, while its inert close control exposed cycle 4. Cycle 4 built one ignored unsigned successor after canonical verification: `Life-OS-Ordinary-Schema-v5-Review-R1-Review-prepared-recovery-r2-close-control-cycle4_0.3.0_x86_64-pc-windows-msvc-setup.exe`, size `5779951`, SHA-256 `DA357971D79262495789A20D4441B4F16BB805F8F4EDC41626E9D747D7ED6F1B`. Its exact six-field manifest passed. It was installed and launched only in `LifeOSReviewR1`; it was not distributed, deployed or released.

## Repository Verification

The canonical command passed after correction cycle 2, after synchronizing Manual Phase A through Step A11-4, for Option A-R3, for the final correction-cycle-4 40-path source state before building the successor package, and again on 2026-09-01 after final manual-evidence synchronization. The latest run passed workflow 17/17, package contracts 12/12, Vitest 369/369, typecheck/build, Rust 216/216 plus backup 12/12 and schema contract 8/8, legacy-v4 compatibility/refusal, ordinary/Founder activation/runtime, and repository-safety checks.

## Manual Verification

In progress. Owner: Founder. Manual Phase A Step A1 passed on 2026-08-28: the ignored review installer existed, its SHA-256 matched `AB514CBD7CF1C8F57EF95C5E6AE88F27C1D438AF96146102DA571BF376DBDD31`, the Windows account was the disposable `LifeOSReviewR1` account, and no `life-os` process was running. Step A3-D1 then performed a read-only inventory of four direct, non-link disposable profiles. It confirmed a sidecar-free schema-v4 source profile at `com.lifeos.app.restored-v4-phase-a`, SHA-256 `40A303527D45B99F11BA63569832DE5FBF24724BE2324546B6E189948F6DDFBD`, while the current schema-v5 profile and both earlier v5 evidence profiles remained present. The trailing operation-file display failed with a PowerShell parser error before execution and changed nothing. Step A3-R1 preserved the prior active disposable v5 profile byte-identically and activated a fresh copy of the untouched v4 source changed only at SQLite header offsets `18,19`. The resulting active fixture is schema v4, SHA-256 `1D9FF0B810C52C976100BB14C75AFF92C35505473393F99EE9355A6514A90EBD`, persistent-WAL header `2/2`, operation-free, and sidecar-free; both source and preserved v5 hashes remained unchanged. Step A3-R2 then installed the hash-verified historical pre-correction verifier build without launching Life OS or changing any fixture fact; its installer SHA-256 was `AD8B71361E3217E35CA6FC37042404CF3651E909D09245E6569FE08A29DE1767`. Step A3-R3 directly launched that historical build and observed the complete Traditional Chinese ordinary migration disclosure without invoking migration or cancellation. Step A3-R4 invoked the disposable migration action exactly once; the historical build then stopped fail closed and explicitly made no success claim. No retry or second action occurred. The first A3-R5 inspection correctly refused because Life OS remained running, but the interactive paste continued with read-only technical checks. Those checks showed the unchanged v4 database hash, one prepared operation with only state plus zero-byte staging, zero-byte WAL, 32768-byte SHM, and no backup/journal/receipt/quarantine. CamelCase property lookup against the snake_case state file caused false schema/ID/path mismatch messages. No mutation followed. At Step A3-R5-D1, the Founder acknowledged the missed close instruction, closed the disposable app normally, and verified zero remaining processes without forced termination. Step A3-R5-R1 then passed the corrected snake_case closed-file inspection: the database remained byte-identical; one exact schema-3 `prepared` operation contained only `state.json` and zero-byte `staging.db`; all later fields were null; persistent-WAL header bytes remained `2/2`; WAL was zero bytes; SHM was 32768 bytes; and backup, journal, recovery receipt, and quarantine remained absent. Step A4-R1 installed the hash-verified R2 package without launching Life OS and proved that installation preserved every fixture hash, length, operation, schema/header, and absence boundary. Step A4-R2 directly launched R2 and observed a fail-closed `sqlite_sidecar_present` startup with the explicit `檢視 prepared-state recovery` entry point. Step A4-R3 explicitly opened the read-only review and observed all bounded database, prior-operation, personal-row, exact-quarantine, content-free-receipt, consent-separation, and write-free-cancel disclosures together with an eligible `準備 recovery` control. Step A4-R4 exposed the correct classification, operation ID, database digest/identity, claim digest, and exact four evidence facts, but native visual review found that long technical tokens overflowed the background frame instead of wrapping. No recovery, cancellation, or close action occurred. Bounded correction cycle 2 now scopes only the recovery card/wrapping CSS, one static regression, allowlist synchronization, canonical verification, one ignored replacement package, and packaged Step A4-R4 retest. Real-profile Phase B and second-migration Phase C remain unauthorized.

Correction-cycle installation and `Step A4-R4-R2` then passed without changing the prepared fixture: the exact technical evidence remained visible and every long path/digest wrapped inside its background frame. Step A5 exercised cancellation and proved the schema-v4 database, operation state, staging, WAL and SHM remained byte-identical with no receipt or quarantine. Step A6 invoked Prepare recovery exactly once. Step A7 closed-file verification proved the database hash and schema remained unchanged, the operation and live sidecars were removed, exactly four transient evidence files entered exact-owned quarantine, and the strict schema-1 content-free receipt contained the expected classification, relative paths, sizes and digests without a root-path disclosure.

Step A8 restart showed the original, separately authorized schema-v4 migration disclosure plus the retained recovery-receipt disclosure, proving recovery consent was not reused. Step A9 then explicitly authorized one disposable migration. Step A10 closed-file verification proved exact schema v5, one verified schema-v4 backup, `v5_ready` / `lifecycle_writes_enabled`, complete backup/source/target manifest evidence, zero-byte staging, no live SQLite sidecars, and retained recovery receipt/quarantine. Steps A11-1 through A11-4 proved restart reconstruction of the original record, one new typed Experience write, a changed live-v5 digest with unchanged backup digest, normal-close sidecar absence, and a second restart that reconstructed both records. Step A12 launched the exact historical installed legacy-v4 executable. The refusal UI passed and the live database, valid operation state, and verified backup remained byte-identical, but normal close created a zero-byte WAL and matching 32768-byte zero-frame SHM. The Founder selected A-R3. Three-language/layout/focus/cancel passed; one exact action quarantined only WAL/SHM and strict closed-file verification proved every protected byte unchanged, exact receipt/quarantine, and no live sidecars. The completion control did not close because the Tauri close capability was absent; native X closed normally and recovery was not repeated. No real profile was accessed.

Correction-cycle-4 installation preserved all twelve profile files. Normal runtime reconstructed both disposable records and the managed backup. The already completed profile was preserved byte-identically while a separate clone reconstructed only the exact empty WAL/zero-frame SHM state. On that clone, the corrected **Preserve and close** control closed natively and changed no fixture byte. One explicit sidecar action created one strict content-free receipt and one exact two-file quarantine while every protected database/operation/backup/staging/prior-recovery fact remained exact. The corrected success control then closed natively. Restart reconstructed both records, reused no migration consent and left no live sidecars. The clone was retained as independent evidence; the completed profile was restored to the active disposable identity and passed a final restart/close with both records, the managed backup, protected evidence and sidecar-absence boundaries intact. Ordinary runtime changed only the live SQLite page digest through already documented guarded transactions. No real profile was accessed.

## Architecture Updates

Created architecture 19 with exact R1 state, eligibility predicate, unsupported states, quarantine/receipt semantics, consent separation, threat model, automated/manual matrices and Android sequencing. Synchronized architecture 13, 15 and 18.

## ADR Updates

None. No ADR text/status changed.

## Documentation Synchronization

Updated `docs/00_Index.md` and `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`. Prior workflow archives remain unchanged.

## Data And Migration Impact

No product schema, DDL, row, projection, migration receipt, backup, restore or user-version logic changed. Prepared-v4 recovery quarantines four proven transient files. A-R3 quarantines only WAL/SHM and records bounded moved/protected technical facts while re-verifying the unchanged v5 database, current operation and backup. No database copy enters quarantine. A later migration still requires a new existing authorization and backup.

## Provenance And Consent Impact

No product provenance, historical ContextPacket, provider, actual-use or consent policy changed. Recovery consent binds one claim and is never reused as migration consent.

## Risks

Partial recovery failure intentionally preserves evidence without rollback/retry. The completed A-R3 action must not be repeated. Any real-profile structural mismatch must stop. The unsigned successor installer must not be used outside the disposable review account. Real data and Android remain blocked.

## Deferred Items

The real Phase B disposition and single Phase C attempt are historical facts and
grant no further authority. Deferred items are disposable R2B Founder manual
review, Promotion Authorization Gate, Private Alpha distribution/deployment/
release, and Android Build Feasibility M0.

## Human Decisions

Decision `DESKTOP-V5-PREPARED-RECOVERY-R2-MANUAL-A-001` was resolved as Option A
by the exact Founder response on 2026-09-01. That acceptance closed only its
diff/manual-review gate. Phase B and one Phase C attempt were later separately
authorized and executed; neither authorizes retry, restore, repair, another
migration, promotion, release, Phase 4 or Android.

## Review Cycles

Four bounded evidence-driven corrections are recorded: cycle 1 synchronized the historical package allowlist, cycle 2 corrected native technical-value containment, Founder-selected cycle 3 added only the exact A-R3 two-sidecar recovery, and cycle 4 corrected only native close authority/error handling after recovery passed. All disposable manual corrections pass, and the Founder accepted the final exact diff without authorizing any later gate.

## Workflow Lessons

Package source contracts that inspect the live diff must list each bounded successor path or canonical verification will correctly fail before product tests. Such synchronization must be planned as an explicit changed path and must not alter package behavior.

## Recommended Next Sprint

The bounded repository-only R2B correction is the current authorized work. It
must stop at disposable Founder manual review without installation or launch.
Android remains ineligible until this desktop correction is manually accepted,
cleanly promoted under separate authority, and the sequencing fence is
satisfied.

## Git Status

Unpromoted, undeployed and unreleased. Branch `codex/desktop-schema-v5-real-profile-recovery-r2`; HEAD unchanged; exact 40-path unstaged working tree; no staged files, commit, upstream, push, merge or PR. Ignored unsigned installer/manifests remain outside repository content and have not been distributed, deployed or released.

## R2B follow-up status — 2026-09-02

The earlier completed report remains historical evidence for the accepted
cycle-4 diff/manual gate. A separately authorized repository-only R2B follow-up
is now in progress after the one real Phase C attempt stopped at the exact
historical-source manifest boundary. Core Rust/UI implementation and focused
tests pass, including a same-transaction write-lock revalidation that rejects
schema drift without mutation. Canonical verification, exact 40-path
reconciliation and the one ignored unsigned package now pass; disposable
Founder manual review remains. No
installation, launch, further real-profile access, promotion, Git write,
release, Phase 4 or Android action occurred.

The final repository package evidence is `5805550` bytes with SHA-256
`30ec0a075f79728b6c87a2f9ba9c5ed3ec1d7188d85b96650fc23cd8715ade9b`.
The correction cycle stops at the manual-review gate under the exact Option B
boundary.

## R2B Founder manual completion — 2026-09-14

The Founder accepted the complete disposable BF1-BF8 evidence. The final BF8
restart/write step passed with exact hash-bound runner, before-launch,
after-close-raw, verification and result evidence; normal schema-v5 startup,
managed-backup visibility and one persisted synthetic Experience were manually
confirmed. Technical evidence proves revision 1 with SQL NULL predecessor,
user provenance, no provider metadata, unchanged protected recovery/migration
evidence and retained profiles, and no live SQLite sidecars after close.

This closes only the disposable R2B Founder manual-review gate. The current
cycle synchronizes that evidence, revalidates the exact repository diff and
prepares—without resolving—one promotion decision. The real profile remains in
the preserved blocked post-commit state with lifecycle writes disabled. No
promotion, Git write, release, Phase 4 or Android action is authorized.

Focused and canonical repository verification passed after synchronization.
The exact promotion decision
`DESKTOP-V5-PREPARED-RECOVERY-R2B-PROMOTION-001` is prepared in
`DECISION_REQUIRED.md` but is neither active nor resolved in the event-sourced
workflow state. The control plane remains on the older resolved Manual-A gate
because its stale-verification/resume-phase constraints form a fail-closed
transition cycle. No direct state/event edit or workflow-kernel change was
made. Promotion therefore remains unavailable unless the Founder separately
authorizes a bounded workflow-control correction and then resolves the
promotion decision.

## R2B workflow-control reconciliation — 2026-09-14

The Founder-authorized workflow-control correction is complete. Starting state
matched the expected branch/HEAD/remote parity, exact 40-path unstaged diff,
zero staged paths, revision `111`, resolved Manual-A decision and stale
`theory_alignment_review` verification boundary. A read-only `git fetch origin`
confirmed `develop == origin/develop == 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899`
at `0/0`.

The official CLI now provides only the bounded
`refresh-verification-for-resume` operation described in the Engineering
Report and workflow contract documentation. Focused workflow tests pass
`28/28`. After one package-allowlist compatibility correction, the Founder
package suite passes `8/8` and the full canonical verifier passes. The corrected
CLI recorded fresh verification at revision `112` and resumed the exact old
decision to Theory Alignment Review at revision `113` without changing its ID,
Founder response, evidence reference or authorized scope.

Founder review separates three layers:

1. The original accepted R2/R2B product and manual-evidence diff remains the
   exact baseline 40 paths; thirty of its thirty-one non-workflow paths are
   byte-identical, and `scripts/founder-dogfood-package.mjs` changes only by the
   three directly required control-plane allowlist entries. No product behavior
   or BF1-BF8 evidence changed.
2. The new workflow correction adds exactly `.ai/workflow/WORKFLOW.md`,
   `scripts/ai-workflow.mjs`, and `scripts/ai-workflow.node-test.mjs` to the
   changed-path set. It adds no runtime/UI surface and weakens no Founder gate.
3. Mutable workflow artifacts record the old decision as resolved historical
   evidence and activate
   `DESKTOP-V5-PREPARED-RECOVERY-R2B-PROMOTION-001` as a new unresolved machine
   decision. Promotion itself is not performed.

The real profile remains preserved at committed schema v5 with
`post_commit_schema_manifest_mismatch`, lifecycle writes disabled and its
verified schema-v4 backup retained. Android M0 remains ineligible under the
documented sequencing fence. No stage, commit, branch switch, merge, push, PR,
deployment, distribution or release occurred.

## R2B local-promotion completion and remote-closeout boundary — 2026-09-18

- Starting base: 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899.
- Feature commit: f43c019bd2f5ad41cbe496f0bdb6010374ca17b7.
- Non-fast-forward merge: 8918fe49993d3cc5e9905a33af64602ac3a511af, with exactly the base and feature commits as its two parents.
- Corrective commit: ea6be14363ec66f937b0de5d517d6640c628551c, limited to LF/CRLF workflow-artifact comparison and its regression test.
- Clean local develop at ea6be14363ec66f937b0de5d517d6640c628551c is ahead of origin/develop by exactly three commits. origin/develop remains at the starting base.
- Canonical verification passes, including workflow 29/29, Vitest 373/373, Rust 222/222, backup 12/12, schema contract 8/8, legacy refusal 33/33, ordinary and Founder activation 10/10 each, and Founder runtime 4/4.
- No remote branch was updated. Publication remains blocked pending a separate exact Founder response.
- No real profile was accessed. The recorded post-commit profile classification remains historical evidence only and requires a new read-only access decision after remote synchronization.
- Android M0 remains blocked.

Sprint ID: 2026-08-27-desktop-schema-v5-prepared-state-recovery-r2
