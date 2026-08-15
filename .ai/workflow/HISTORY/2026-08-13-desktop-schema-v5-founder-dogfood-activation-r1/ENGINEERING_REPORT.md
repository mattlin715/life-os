# Engineering Report

Status: completed_with_follow_up

- Sprint ID: 2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Working-tree digest at corrective-cycle-2 verification: 6f9d1450ebde154ccbf41d1a1874586a3be162561e50360ea362161d75ecaa61
- Created at: 2026-08-13T22:05:00+09:00
- Updated at: 2026-08-15T01:30:00+09:00

## Implementation Summary

Implemented the isolated Windows Founder schema-v5 activation candidate behind the immutable `com.lifeos.founderdogfood` identity, Cargo feature `founder-schema-v5`, and frontend compile gate. The ordinary application remains schema v4. The candidate provides explicit migration, verified backup, durable restart classification, typed v5 storage routing, backup delete/restore controls, three-language disclosure, and an unsigned private package.

## Existing System Areas Inspected

`src-tauri/src/sqlite.rs`, filesystem safety and backup/restore modules, the promoted schema-v5 migration/writer modules, typed schema-v4 commands, storage adapters, startup UI, package scripts/configuration, architecture/13, architecture/15, architecture/16, ADR-0009, and ADR-0011.

## Files Added

Architecture/17; candidate build/package scripts and test; candidate Tauri config; `schema_v5_founder_activation.rs`; `schema_v5_runtime.rs`; Founder migration/backup panels and tests; Founder v5 TypeScript command/store adapters and tests; `src/vite-env.d.ts`.

## Files Modified

Current workflow artifacts; Index and architecture/13/15/16; Founder package runbook; package verifier and canonical verification script; Cargo manifests; filesystem safety; Tauri registration; promoted private v5 writers/migration visibility and batch command surfaces; App/i18n; local-store selection.

## Files Deleted

none

## Behavior Changed

Only the candidate build registers the schema-v5 startup and typed database commands. Missing isolated profiles create exact v5. Exact v4 profiles disclose migration and require one explicit authorization. Existing incompatible or ambiguous states fail closed. Verified v5 routes current product actions through typed Rust boundaries. The ordinary build retains existing schema-v4 startup and storage.

## Data Model Impact

The accepted fixed schema-v5 DDL and receipt contract are activated only in the isolated Founder profile. `SCHEMA_VERSION` and ordinary startup maximum remain 4.

## Migration Impact

Exact v4 only; v2, v3, malformed, newer, unsafe, active, sidecar-bearing, changed, or contradictory sources are refused. One exact-owned verified backup precedes one migration transaction. `user_version = 5` remains last. Restart classification is read-only and evidence-based.

## Provenance Impact

Promoted revision, authorship, lifecycle, dependency, migration-receipt, and ADR-0009 provenance contracts are preserved. Runtime projection drift fails closed.

## Historical Context Impact

Retrieval, ephemeral selection, eligibility, packet content, and generated Historical Question semantics are unchanged. The candidate adds only schema-v5 persistence routing for the already-authorized ADR-0009 actions.

## Consent Impact

No policy or UI semantic change. Existing per-generation/per-purpose consent and atomic consumption are preserved.

## Provider Transmission Impact

No provider, model, request, retention, or ContextPacket change.

## Tests Added

Package source-contract test; Founder activation fresh-v5, collision,
first-authorization exact-v4, and existing-verified-backup explicit-resume
tests; production-initialized-v4 schema-manifest migration regression; explicit
blocked-v5 verified-backup restore regression; typed runtime Experience/Evidence
tests; TypeScript command/store tests; three-language migration/backup panel
tests. Existing 191 ordinary-feature Rust
tests cover migration failure boundaries, restart classifications, lifecycle
consequences, ADR-0009 parity, and filesystem safety.

## Tests Executed

- `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`: passed.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --features founder-schema-v5 -- -D warnings`: passed.
- Focused Windows replacement and parent-durability tests: passed.
- Corrective cycle 2 focused activation tests: 6/6 passed.
- Corrective cycle 3 focused activation tests: 6/6 passed, including one
  production-initialized-v4 migration followed by a typed Experience create.
- `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-founder-schema-v5-candidate.ps1 -ReviewSuffix cycle1`: passed; corrected unsigned NSIS package and content-free manifest verified.
- `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-founder-schema-v5-candidate.ps1 -ReviewSuffix cycle2`: passed; unsigned NSIS package and content-free manifest verified without installation or launch.
- `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-founder-schema-v5-candidate.ps1 -ReviewSuffix cycle3`: passed; corrected unsigned NSIS package and content-free manifest verified without installation or launch.

## Verification Results

Passed after corrective cycle 3: workflow 17/17; Founder package 8/8;
candidate package 1/1; Vitest 45 files/323 tests; ordinary-feature Rust library
191/191; backup/restore integration 12/12; schema contract 8/8; Founder
activation 6/6; runtime focused 2/2; TypeScript typecheck; frontend build; Rust
check; feature check; Clippy with warnings denied; UTF-8, whitespace, secret,
Markdown-link, and Constitution checks. The full Founder-feature Rust test
binary contains 197 tests; the ordinary-feature canonical suite remains
191/191. The cycle-3 installer is
`.artifacts/desktop-schema-v5-founder-candidate-r1/9a226f7081aa-cycle3/Life-OS-Founder-Schema-v5-Candidate-R1-Review-cycle3_0.2.0_x86_64-pc-windows-msvc-setup.exe`
with SHA-256
`c9709e90c994ce11c27526f521ca0b6044e5bef1c3dc7513798bd73876ea3a20`.
It has not been installed or launched.

## Manual Verification Required

The isolated-profile package review remains in progress. Founder Steps
7B-R6A through 7B-R6G manually proved explicit restore of the exact verified
backup, explicit reauthorization, migration, v5 verification, backup
disclosure, and restart reconstruction. Step 11A then exposed a runtime-writer
manifest-predicate mismatch before `BEGIN IMMEDIATE`; normal close and
read-only inspection proved zero new Experience rows and no governed-state
drift. Corrective cycle 3 is canonically verified. A newly built cycle-3
package must repeat only the affected typed-write path before the remaining
manual matrix continues.

Cycle-3 manual installation then used the NSIS uninstall path with the explicit
`Delete the application data` checkbox selected. The Founder confirmed that
choice. Filesystem timestamps and read-only inspection prove the isolated
`com.lifeos.founderdogfood` root was deleted and recreated immediately after
installation; the prior two Experiences, four artifacts, owned-operation
state, and verified backup were therefore intentionally removed by that
destructive uninstall, not silently lost by migration or runtime code. The
ordinary profile hash remained unchanged. The recreated exact-v5 profile then
accepted one packaged typed Experience create without error; the closed
database contains exactly that row, valid foreign keys, and `integrity_check =
ok`. Because this write used fresh-v5 rather than the promoted-runtime-v4
manifest, it does not by itself prove the cycle-3 migrated-v4 correction.

The Founder then completed the reachable Evidence and Reflection paths on the
fresh-v5 disposable profile. Evidence candidate creation, pending correction,
exact confirmation, Reflection prompt creation, first saved response, and
saved-response correction all passed. Pattern remained correctly gated until
Context Recovery supplied additional event context. Context Recovery prompt
creation passed, but saving the first non-empty response produced no visible
local result and did not unlock Pattern.

After the Founder closed the Candidate normally, read-only inspection found
schema v5, one Experience, three projected artifacts, six normalized artifact
revisions, valid foreign keys, and `integrity_check = ok`. The Context Recovery
artifact remained exactly one `recovery_turn` suggested/pending revision with
no response content; no WAL, SHM, or rollback-journal sidecar remained. The
ordinary profile hash remained
`bafdddac5f32f4289c318318f701336eeb5a62fb2e297c3cc15c1ab29372df4f`.
This proves the failed action wrote no partial answer state.

## Documentation Updates

Added architecture/17 and synchronized factual navigation/boundaries in the Index, architecture/13/15/16, and Windows Founder package runbook.

## ADR Impact

No ADR status or decision changed. ADR-0009 and ADR-0011 remain controlling authority.

## Deviations From Plan

Windows directory handles cannot honestly satisfy `FlushFileBuffers` as a parent-directory durability primitive. The final replacement path therefore uses same-volume `MoveFileExW` with `MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH`, closed-file flush/revalidation, and conservative outcome classification. Architecture/17 records the bounded durability claim and residual power-loss risk.

Founder manual Step 7B then found a separate fail-closed path-identity defect.
The backup and live-v4 evidence were valid, but the migration request used the
non-canonical Tauri app-data path while the owned operation contained the
canonical Windows path. The migration core correctly refused the mismatch
before recording `Migrating`. Corrective cycle 1 reuses the exact
`operation.live` identity and adds focused first-authorization and
verified-backup explicit-resume regressions. No live Founder data was changed
by diagnosis or implementation.

The corrected cycle-1 package then reached commit but failed closed during
post-commit schema-object verification. Forensic read-only evidence proved the
fixed contract fixture and promoted runtime-v4 database contain the same eleven
legacy objects while preserving different `sqlite_master.sql` formatting. The
manifest contract now accepts exactly the existing fixture digest or one fixed
promoted-runtime-v4 digest and rejects every third representation. A new
production-initialized-v4 regression proves the full backup, migration, close,
reopen, and v5 verification path. Exact durable
`V5BlockedRestoreAvailable` evidence may now expose only a Founder-triggered
restore of its reverified backup; no retry, repair, candidate selection, or
automatic recovery was added.

After successful explicit restore and migration, the first packaged typed
Experience create failed closed with
`experience_write_schema_manifest_mismatch`. The runtime writer still compared
only the contract-fixture manifest while migration/startup already used the
exact two-manifest predicate. Corrective cycle 3 makes the Experience verifier
reuse that same closed predicate. Because the other typed writers reuse the
Experience verifier chain, no broader manifest or lifecycle policy was added.
A focused regression migrates a production-initialized v4 fixture and then
proves typed Experience create plus guarded projection reconciliation.

The continued fresh-v5 manual journey found a separate runtime-adapter defect.
`schema_v5_runtime.rs` requests the current Context Recovery revision using
artifact kind `context_recovery`, while the promoted writer, normalized head,
and schema-v4 projection all use `recovery_turn`. The lookup therefore fails
before the Context Recovery answer transaction starts. The application stores
the resulting error in the general mutation error surface rather than beside
the Context Recovery action, so the visible symptom is an apparently inert
Save button. No retry or corrective write was attempted.

## Known Limitations

Unsigned Founder-only Windows package; no malicious same-user protection; no durability claim beyond Windows write-through move plus post-operation evidence; no real-user evidence; no ordinary-profile v5; no Android; no export v2; no Phase 4.

## Remaining Risks

The Context Recovery runtime discriminator and local error disclosure must be
corrected before the remaining lifecycle journey can continue. The current
sprint has already consumed all three bounded revision cycles, so the workflow
forbids a fourth correction attempt. Old-v4 refusal, retention/delete,
keyboard, narrow-window, and three-language behavior still require Founder
manual evidence. Real-user migration and ordinary desktop enablement require
separate gates.

## Git State

Branch `codex/desktop-schema-v5-founder-dogfood-activation-r1`; HEAD `9a226f7081aabc071571f4a745e1343dbdb7d927`; unstaged implementation and workflow changes only; no staged files; no upstream; no commit, push, merge, PR, deployment, distribution, or release.

## Engineer Completion Status

completed_with_follow_up: corrective cycle 3 focused tests, Clippy, canonical
verification, and unsigned package build pass. The combined migration and
typed-Experience evidence was Founder-accepted, and the Evidence/Reflection
manual path passed. The Context Recovery answer path is now blocked by a
reproducible runtime artifact-kind mismatch. Because review cycle 3 is the
configured maximum, a separate Founder-authorized corrective sprint is needed
before manual review can resume.
