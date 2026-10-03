---
status: Founder-accepted synthetic implementation
version: 0.4
owner: product-and-engineering
last_updated: 2026/10/04
depends:
  - docs/architecture/23_Android_M2B_Synthetic_Experience_Lifecycle.md
  - docs/dev/08_Engineering_Harness.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
referenced_by:
  - docs/00_Index.md
---

# 14 Android M2-B Synthetic Experience Lifecycle Runbook

## Safety and commands

Synthetic data only, temporary `com.lifeos.review.m2b`. Supports create/list/get,
exact-revision edit and confirmed delete; no real profiles/physical devices,
production/providers/credentials/INTERNET/artifacts/sync/import/export/backup/
migration/recovery/release. Direct fresh-v5 only; accepted apps/evidence preserved.

```powershell
node --test scripts/android-m2b-contract.node-test.mjs
pnpm android:m2b:build
pnpm android:m2b:inspect
pnpm android:m2b:native
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

Repository-local toolchain/per-process environment only. Official Tauri debug
x86_64 build with only the existing narrowly recognized Windows symlink fallback;
no elevation or Windows feature change. Verify/preserve accepted M2-A APK before
reusing ignored Gradle output. Separate artifact `.artifacts/android-m2b/review.apk`.
Inspect identity, debug signing, permissions/backup/transfer exclusions, ABI and
canonical accepted icon hash.

## Exact-owned native fixtures

Any initially connected device blocks the run. Each run creates an absent unique
`lifeos_m2b_<RunId>_api36_x86_64` AVD, checks configuration/name and absent M2-B
package, installs only this APK and binds `emulator-5586`, port 5586/CDP 9226.
Collision means stop, never overwrite/wipe. Ownership receipt binds exact
RunId/AVD/serial/package/APK. Accepted M0/M1/M2-A apps are never cleared/replaced.

Evidence: `.artifacts/android-m2b/native-review/<RunId>/`. Database fixture copies
contain only deliberately entered synthetic text. Acknowledgements/request
receipts/log output contain no raw text; probe failures suppress raw assertions.
Export fixtures before every owned reset/injected failure. Failed AVD/sandbox/
evidence remain; rerun uses another absent RunId, not unknown-state cleanup.

Checks: CJK/multiline create/edit/restart; immutable identity/revision/predecessor/
authorship; cancel edit/delete byte-identical database (no writes/events); confirmed
delete/cache clearing/reopen absence; same-content revision; duplicates/stale/two
editors/edit-delete races; rollback after projection; synchronized before/after
commit update/delete force-stop and exact request reconciliation; pending/newer/
malformed/dependency refusal preservation; canary logs. Final fresh profile must
survive emulator shutdown/restart without shell sync and without pending names.

## Evidence status and limitations

### Current Founder-authorized status correction

`ANDROID-M2B-FOUNDER-REVIEW-002 Option C` authorizes only trilingual status
presentation after the preceding exact manual7/7 PASS. Debug APK SHA-256:
`8be61db3f500514f01055846273460307580e3ed9246b078d2c3a50cc21eca43`.
Focused current frontend18 and M2-B contracts11, build and inspect PASS. Full
exact-owned native run `20261003185925` passes19 groups, including real UI
status interactions in all three languages and all original lifecycle/fault/
refusal/full-emulator-restart checks. Screenshots show visible unconfirmed
feedback and no false empty-draft warning. Canonical results and final digest are
recorded after the last documentation change in workflow/final manifest.
The current exact Founder manual checklist passed 7/7 and the candidate was
accepted under `ANDROID-M2B-FOUNDER-REVIEW-003 Option A`. Accepted non-workflow
digest: `00bf3293f5aa7095cf19d6189a9233ed6120ac19003daf0ba3f448077200a021`.
The bound manual record is `.artifacts/android-m2b/founder-manual-review.json`;
the exact decision is recorded in the workflow and preserved acceptance evidence.
Prior evidence is preserved;
backend/handlers/requests/storage/retention/deletion policy remains unchanged.

### First Founder-authorized copy correction (historical)

The original exact candidate completed Founder manual checklist 7/7 PASS, but
was not accepted. `ANDROID-M2B-FOUNDER-REVIEW-001 Option C` authorized only:
distinct per-moment/build disclosure labels and a plain deletion confirmation,
in English/Traditional Chinese/Japanese. Existing retained-metadata and physical
erasure caveats remain available in optional disclosure; storage/writers/policies
are unchanged. The original APK, manifest, manual results, workflow and manual
profile evidence are preserved under
`.artifacts/android-m2b/copy-correction-001/pre-correction/`; old AVD preserved.
Corrected candidate results are recorded in workflow and the exact final manifest
after execution; original 7/7 does not transfer to the new APK or imply acceptance.

Corrected debug APK SHA-256:
`c74b2cd962edcc5902bcd22c11772064f90be3f9a6b9ee9e821730dd9a0441f3`.
Focused frontend 12 and M2-B contract 10 checks, build and APK inspection PASS.
That candidate's native/canonical evidence lives under
`.artifacts/android-m2b/copy-correction-001/` and exact-owned native run directories;
workflow/final manifest distinguish passed checks from preserved failed attempts.
Backend, canonical writers, initializer, request adapter, CSS, icon and Android
configuration match the original candidate; no retention/deletion policy change.

### Original candidate evidence (historical)

Full canonical baseline PASS on clean synchronized develop
`aebef185346effda779a6a10850882a2dc19e701`. Focused host facade 4, frontend 9,
direct update/delete uncertain-commit classification and Android contracts
(M1 9, M2-A 10, M2-B 9) PASS. Debug build/inspection PASS; review APK SHA-256
`cf55afc9400173c6966a5afd13f4e350569fed787fb1f707d9618d43c04ae823`.

Native evidence is consolidated from run `20261003074305` lifecycle/fault/refusal
checks and its exact-owned supplemental-c final restart checks (15 groups; final
full emulator restart twice, UI ready and byte-identical DB/receipt, no pending
names). Original run/supplement timeouts remain failures, not retroactive exit0;
read-only diagnosis and screenshot observed ready storage/UI. Cold CDP document
reattachment corrected the instrumentation without clearing/reinstalling the
preserved final profile or replaying mutations. Evidence:
`.artifacts/android-m2b/native-review/20261003074305/native-consolidated.md`.
Optional bounded final-only command, collision-refusing and ownership-bound:
`powershell -NoProfile -ExecutionPolicy Bypass -File scripts/android-m2b-native-review.ps1 -RunId <owned-run> -ResumeFinal -SupplementId <a|b|c>`.
Never overwrite an existing supplement; this is not a repair/clear/retry command.

Final canonical result, exact non-workflow digest, file inventory and evidence
hashes are recorded after the last change in workflow and
`.artifacts/android-m2b/final-manifest.json` (outside the digest to avoid a
self-referential hash). Founder consolidated diff and manual review remain pending.

Physical devices, ARM/OEM/other APIs, actual power loss, arbitrary multi-process
access, physical erasure and full continuity remain untested. Host tests do not
prove Android guarantees; fault injection is not physical durability proof.

## Retention versus deletion

Explicit edits create new user-authored revisions even with identical text;
prior text remains locally until deletion. Parent delete purges logical source
content/projection/scoped content, while existing content-free source head,
revision IDs/digests/authorship/lineage/times/provenance remain. No secure physical
erasure promise for SQLite pages or device media. No new retention/cascade policy.

## Founder manual checklist (completed 7/7 PASS)

Status-presentation correction authority: `ANDROID-M2B-FOUNDER-REVIEW-002 Option C`.
The preceding corrected APK/manual7/7, source inventory and deleted synthetic
profile are preserved in `.artifacts/android-m2b/copy-correction-002/pre-correction`.
New exact APK/digest/check counts are recorded in workflow and final manifest
after verification. Empty/new/opened/edit/cancel/create/update/delete status
presentation is checked natively in en/zh-TW/ja, including visible conflict and
unconfirmed results. Focused tests also cover processing presentation. This adds
no storage, revision-retention, deletion, request or error-handling policy.

Bind exact final APK/hash/source HEAD/non-workflow digest/ownership receipt/AVD/
`emulator-5586`. One step at a time, explicit Founder PASS/FAIL; preserve and stop
on failure.

1. Launch exact fresh M2-B app. Confirm synthetic/offline/no-AI scope, accepted
   complete icon/safe areas and all three languages. Empty input must show no
   false unsaved draft warning.
2. Create unique synthetic CJK/multiline text; open exact text and ensure duplicate
   taps do not create another record. New nonempty draft warning stays in its
   input card; verified create feedback is separate and empty/opened saved text
   must not be labelled an unsaved draft.
3. Edit, Cancel (original unchanged), then edit/save new text. Optional disclosure
   explains retained revisions rather than deletion. Its per-moment heading must
   differ from the global build-scope heading in all three languages. Unchanged
   editor/cancelled edit has no false warning; changed editor warning belongs in
   that editor. Verified edit feedback identifies saved changes, not a new draft.
4. Force-stop/relaunch exact M2-B package. Reopen updated text and verify last
   selected non-English locale.
5. Delete, Cancel; saved text remains, no mutation. Confirmation describes current
   and previously saved text removal plainly, without metadata/physical-erasure
   jargon; those caveats remain in optional build disclosure.
6. Confirm Delete. Detail/editor/list no longer show record. Read logical purge/
   metadata/physical-erasure limits. Separate feedback identifies the deleted
   moment; empty input has no unsaved warning. Restart; record remains absent
   without a false draft warning.
7. No AI/artifact/provider/network/credential/import/export/backup/migration/
   recovery/production/release behavior.

## Non-authority

The consolidated Founder diff/manual gate was explicitly accepted; automated
checks did not accept the candidate. Prior pending-review wording in historical
sections describes their original snapshots, not the current gate.

Separate `ANDROID-M2B-INTEGRATION-CLOSEOUT-001` authorizes exact-scope staging/
commits, factual closeout, official terminal follow-up/archive/reset, only the
M2-B archive-prefix compatibility adjustment, normal non-force feature/develop
pushes and `--no-ff` integration. Closeout must not rebuild/launch the APK,
operate an emulator/device, access profiles or repeat accepted manual review.
Archive/reset precedes the first commit so published HEAD changes cannot stale
an active terminal workflow. Candidate and archive commits stay distinct;
verify merged develop before pushing it. Actual results and path/content planes
are recorded in `.artifacts/android-m2b/promotion-001/` after each operation.
No PR/distribution/deployment/release, new product behavior, real data,
production identity, later M2 slice or Phase 4 is authorized.
