# Engineering Plan

Status: approved

- Sprint ID: 2026-09-22-android-m1-disposable-persistence-review
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-22T01:32:00+09:00
- Updated at: 2026-09-28T09:10:00+09:00

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement only a disposable, synthetic Experience create/list/get prototype at `com.lifeos.review.m1`; preserve exact schema-v5 or fail closed; keep desktop behavior unchanged; do not expose update/delete/import/artifact/history operations; keep the APK offline and non-backupable; distinguish draft/commit/failure and use an idempotent request ID; prepare Android-native and automated evidence; leave production architecture, final identity, M2–M4, and Founder acceptance unresolved.

## Existing Implementation Understanding

M0 supplies a reviewed Tauri Android shell, generated API 36 project, safe-area activity, local toolchain script, backup exclusions, and package inspection. `createLocalEvidenceStore` deliberately has no Android production path. Desktop fresh-v5 initialization constructs the canonical empty v4 base, migrates with `schema_v5_migration`, activates lifecycle writes, verifies the receipt/contract/schema manifest, and routes typed writes through `schema_v5_runtime`. Its Windows profile/recovery wrapper is not portable authority.

## Affected Modules

- New `src/android-m1/` review UI, tests, CSS, and narrow `Pick<LocalEvidenceStore, ...>` adapter.
- `src/main.tsx`, `vite.config.ts`, `package.json` for a distinct M1 build mode/commands.
- `src-tauri/src/android_m1.rs`, shared fresh-v5 base constant, and minimal visibility/cfg changes needed to reuse the canonical schema-v5 migration/runtime core without exposing desktop recovery commands.
- `src-tauri/Cargo.toml` target dependency availability; no new third-party package family is introduced.
- `src-tauri/tauri.android.conf.json` and generated Android package/manifest/resources for the temporary identity.
- New bounded M1 build/inspect/native-review scripts and focused contract tests; M0 scripts/evidence stay unchanged.
- New architecture/runbook/Proposed ADR plus index/roadmap synchronization.

## Proposed Design

1. **Identity/isolation:** package and namespace `com.lifeos.review.m1`; app-private `app_data_dir/android-m1-disposable-v5.db`; exact identity check in every native command; no caller-supplied paths.
2. **Fresh exact v5:** extract the existing canonical empty-v4 base SQL to one shared Rust constant. On first open, create a uniquely named staging database, run the existing v4→v5 migration, activate lifecycle writes, verify exact committed v5, then publish. Any live/staging conflict or invalid/newer state is preserved and refused. No migration of pre-existing data and no auto repair/delete.
3. **Narrow façade:** Tauri commands initialize/status, create, list, and get only. The renderer adapter is typed as a three-operation subset of `LocalEvidenceStore`; desktop store selection is untouched.
4. **Commit/idempotency:** renderer generates one request ID per draft. Backend uses it as the Experience ID. Existing same-ID/same-body content is returned as `alreadyCommitted`; same-ID/different-body fails closed. Success is returned only after the promoted v5 writer commits and re-reads exact content. UI locks a pending request and does not clear the draft on failure.
5. **Test hooks:** debug/disposable-only delay modes before the write or after committed re-read enable exact emulator process-kill tests. Existing schema-v5 writer fault injection plus focused Rust tests proves rollback of an interrupted transaction. Hooks are unavailable outside Android M1 commands and are documented as non-production.
6. **Durability wording:** acknowledged means SQLite transaction commit completed and the row was re-read. It does not prove hardware flush, emulator crash, host crash, or actual power-loss survival. Those remain explicit gaps.
7. **UI:** trilingual calm form/list/detail surface, data-state markers for deterministic native review, progressive `details` technical section, synthetic/disposable labels, and exact text rendering.

## Alternatives Considered

- **Frontend `tauri-plugin-sql`:** rejected because it grants renderer-level SQL rather than a narrow domain façade.
- **New minimal “v5-like” schema:** rejected because it would be a different schema and violate the exact-v5/no-fallback condition.
- **Compile the desktop activation wrapper wholesale:** rejected because it would import Windows profile/recovery guarantees and identity policy.
- **IndexedDB or memory:** rejected because neither proves app-private SQLite schema-v5 persistence.
- **Full production Android `LocalEvidenceStore`:** deferred to M2; it would falsely expose unsupported lifecycle operations.

## Data Lifecycle Impact

Only explicit synthetic create is supported. Records persist inside the disposable app sandbox until app data is cleared/uninstalled; there is no UI delete, export, retention, backup, restore, or transfer. This consequence is visible in the technical disclosure and runbook. No real data may be entered.

## SQLite Or Migration Impact

The prototype creates a fresh canonical schema-v5 database only. It never opens v4 as writable, imports an existing database, or migrates a user/profile database. Exact receipt, contract, version, and schema-manifest verification runs before operations. Malformed/newer/ambiguous state blocks. Canonical desktop migration/runtime logic is reused, while Android publication/durability claims remain separate and Proposed.

## Provenance Impact

The Experience is user-authored synthetic text. Canonical v5 source revision/provenance/compatibility projection writes are reused. No AI or derived artifact is created. The request ID also serves as an idempotency identity, not as proof of user identity.

## Historical Context Impact

None: no desktop paths, historical APIs, existing databases, profiles, provider payloads, or M0 app data are accessed.

## Consent Impact

None for provider/history because there is no transmission. OS backup and device transfer are explicitly disabled; changing that consequence requires a later governed decision.

## Provider Transmission Impact

None. APK manifest has no `INTERNET` permission and the M1 web bundle must contain no provider/runtime desktop route.

## Import And Export Impact

Unsupported and absent from the UI/native command surface. M2 remains responsible for governed portability.

## Test Strategy

- Vitest: trilingual copy, draft vs committed states, no premature success, exact text reopen, retry/idempotency, honest failure, supported-operation surface.
- Rust focused tests: fresh exact-v5 create/reopen, CJK exactness, duplicate request behavior, injected in-transaction rollback, concurrent initialization serialization, malformed/newer/ambiguous refusal, open/write failure.
- Node contract tests: exact identity/config/generated namespace, no INTERNET, backup exclusions, safe area, separate M0 preservation, command allowlist, build mode isolation, docs/ADR status and phase fences.
- APK inspection: application ID, ABI, debug certificate, permissions, cleartext, backup/data-extraction rules, packaged web/native isolation, hashes.
- Dedicated API 36 x86_64 AVD: fresh install, create/list/get exact CJK, background/foreground, force-stop/relaunch, process kill before commit and after commit-before-ack, database survival, package/runtime permissions. Graceful shutdown, force-stop, process kill, emulator crash, and actual power loss are reported separately; unsupported/unexecuted cases remain gaps.
- Desktop regression: typecheck/unit/canonical verifier and unchanged desktop route/config behavior.

## Repository Verification Strategy

Run focused frontend, Node, Rust host tests; M1 build and inspect; exact-serial native script; then `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Record all commands and exit codes in ignored evidence and the Engineering Report. Inspect unstaged diff/whitespace/secrets and workflow consistency.

## Manual UI Verification

Owner: Founder. Provide a short install/open → enter synthetic CJK → save → open exact record → close/force-stop → relaunch → reopen checklist. Automated native execution prepares evidence but never marks Founder manual review passed.

## Rollback Or Recovery Strategy

Code rollback is deletion of the M1-only diff before promotion; no production state is activated. Disposable runtime recovery is intentionally unsupported: an invalid/ambiguous database is preserved and the app reports a blocked state. For further testing, uninstall/clear the dedicated disposable package only under explicit test setup; never auto-clear from the app. M2 must design production recovery.

## Documentation Impact

Add one M1 architecture evidence document, one M1 runbook, and one Proposed ADR; update Index/Roadmap links and status language. Preserve M0 and desktop documents. Every claim labels Proposed, prototype-implemented, automated-verified, native-tested, Founder-pending, and production-unauthorized separately.

## ADR Impact

Create Proposed ADR-0012 after confirmed numbering. It recommends, but does not accept, a stable production identity plus Android-owned app-private SQLite adapter reusing schema/domain write contracts. Founder decision is required to accept or revise it.

## Risk Level

High: storage authority, application identity, native process lifecycle, and schema reuse are consequential. Risk is bounded by the temporary identity, synthetic-only app-private data, exact fail-closed checks, no network/backup/import, no production routing, and a mandatory Founder gate.

## Escalation Decision

No early escalation is required because the Founder explicitly authorized a disposable prototype, Proposed documents, and bounded corrections. Stop and escalate if exact schema-v5 reuse cannot compile on Android, a new production authority is needed, the disposable database could touch another profile/package, or testing would require a physical device/real data/destructive action outside this sandbox.

## Founder-Authorized Revision Cycle 1

`ANDROID-M1-FOUNDER-REVIEW-001` Option C authorizes exactly two corrections to
the original review APK after its manual checklist passed 10/10:

1. keep only the selected locale code in a fixed app-private Rust-owned preference,
   restore it on restart, map Traditional Chinese to `zh-Hant` for the document
   language, and fail safely to English if the preference is absent, invalid,
   or inaccessible; and
2. align Traditional Chinese and Japanese wording with the desktop vocabulary
   for saved moments, local storage, observable clues, pattern hypotheses, and
   reflection while retaining every synthetic-only M1 limitation.

Focused tests cover the fixed key, allowed values, invalid/inaccessible
fallback, non-content scope, and terminology. The dedicated native run selects
Traditional Chinese, force-stops/relaunches, verifies the restored locale, and
then proceeds through the existing persistence matrix. Documentation must state
that app-data clearing removes the preference and that the earlier 10/10 manual
result does not accept the corrected hash. No ADR status, stable identity,
production authority, Git publication, or M2 scope is added.

## Founder-Authorized Revision Cycle 2

`ANDROID-M1-FOUNDER-REVIEW-002` Option C authorizes only launcher-icon
alignment after the locale/copy-corrected APK passed its manual checklist 10/10.
Use the largest exact PNG payload embedded in the tracked PC source
`src-tauri/icons/icon.ico` as the input to the existing Tauri icon generator;
do not redraw, reinterpret, or replace the Life OS brand. Synchronize only the
Android density-specific legacy, round, and adaptive resources plus the manifest
round-icon binding. Extend the exact M1 contract with source/output SHA-256
bindings and adaptive-resource checks so placeholder icon regression fails.
Rebuild, inspect, rerun the complete disposable native and canonical matrices,
then return to a fresh Founder icon-review gate. No product-content behavior,
storage schema, locale behavior, ADR status, stable identity, real-data
authority, Git publication, or M2 scope is changed.

## Founder-Authorized Revision Cycle 3

`ANDROID-M1-FOUNDER-REVIEW-003` Option C authorizes only the adaptive-icon
safe-zone correction. The first canonical derivative placed the pale ring too
close to the adaptive foreground edges, so Android's rounded launcher mask
clipped the ring at the top, bottom, left, and right. Preserve the exact PC
desktop icon pixels and dark adaptive background, but scale the canonical image
uniformly inward inside each density-specific adaptive foreground canvas with
transparent padding. Do not change legacy or round launcher assets, brand
geometry, color, text, product behavior, or any persistence surface.

Extend the exact icon contract to bind the padded foreground output hashes and
verify transparent edge padding plus a centered, uniformly scaled canonical
payload. Rebuild and inspect the APK, rerun the complete disposable native and
canonical matrices, then return to a fresh Founder icon-review gate. All Git,
release, real-data, ADR-acceptance, stable-identity, and M2 authority remains
withheld.
