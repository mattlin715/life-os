---
status: First/append R1 source slice; new Android review pending
version: "0.10"
owner: LIN MENGLUNG
last_updated: 2026/11/07
depends:
  - docs/architecture/24_Android_M2C_Offline_Synthetic_Daily_Reflection.md
referenced_by:
  - docs/00_Index.md
---

# Android M2-C review runbook



## Independent Windows ARM64 pilot candidate (synthetic, unsigned)

The Founder authorized applying the independent no-symlink packaging proof to
one Life OS synthetic candidate. Build from the verified first/append source,
using the opt-in Rust `android-pilot` feature and Vite `android-pilot` mode.
The source overlay selects `com.lifeos.review.pilot`; both the runtime guard
and the fresh-v5 DB/receipt filenames bind to that scope. Historical M2-C and
production IDs remain separate. The pilot has no debug hooks, provider transport,
real-data or migration authority.

The standalone JDK/SDK/NDK/Gradle/Rust toolchain may be reused; app source,
Android project, dependency extraction, build target, and candidate outputs are
separate. Direct Cargo ARM64 compilation hands an ordinary, hash-verified .so
to Gradle; the old emulator runner is not part of this path. Candidate evidence
is static only: exact packaged library, JNI exports, ARM64, ELF/ZIP alignment,
merged manifest, permissions and backup rules. No signature, installation,
device operation, or physical-device result follows. M2-D/036 remains stopped.

### Pilot 3001 startup correction

The Founder installed 3000 on Zenfone 10; its WebView showed
`ERR_UNKNOWN_URL_SCHEME` before rendering Life OS. The effective Cargo
overlay used a Windows drive path for `frontendDist`, which Tauri parsed as
an external URL. The APK JSON alone did not expose this mismatch.

For 3001, build Vite in pilot mode into the ignored repository-root
`dist/` and keep the Cargo overlay at `frontendDist: "../dist"`, relative
to the Tauri config. Do not turn on a dev server or network permissions.
Inspect the actual APK library for the frontend asset keys and exact
embedded HTML/JS/CSS content and reject the old host launch URL. Use 3000
as a failing control. Keep `com.lifeos.review.pilot` and bump only
`versionCode` to 3001; signing and in-place phone update remain separate.
No successful Android startup or data retention is implied by these checks.

### Pilot 3001 follow-up — save handoff and content ownership

On Zenfone 10 the Founder reached the UI and saved a moment in 3001, but
had to scroll past the list to find Reflection. The question and original
also looked too similar. This follow-up changes the UI only: successful
create and exact ID/body readback reveal and focus the saved moment once,
without navigating on an unverified outcome. **Start reflection** is visible
at the top of the open moment and jumps to the existing review controls;
neither action generates a candidate or a question. Labels and structural
borders separate the user's original, local demo questions (no AI call),
and the user's own response. Unverified origin is not labelled as a known
demo or user answer. Storage, requests, provenance and ADR-0012 stay unchanged.

Source/host scope only. After separately authorized packaging and update,
verify on Zenfone 10 with synthetic data: from a long list with the keyboard
open, save a new moment; check that the correct heading and Start reflection
appear without manual scrolling. Tap the shortcut and verify it does not
generate anything. Explicitly reveal the demo, then answer it: distinguish
origin labels in both light and dark themes, including after reopening.
No Android runtime result is claimed for this follow-up.

### Pilot 3002 feedback — simplify the just-saved moment

The Founder reported seeing **Start reflection** on Zenfone 10, but did not
know to choose it: editing help and the edit, delete, close and revision
controls competed with it. The long-list and keyboard-open conditions were
not confirmed by that report.

After exact create/readback, the new moment's card now presents its saved
heading and original text, followed by one prominent **Start reflection**
action. Edit, delete, close and version history are under a closed **More
actions** disclosure. Editing help appears in the editor. The shortcut still
only navigates to the unchanged Reflection section; saving and navigating
do not generate an artifact. Ordinary reopening is not treated as a new
save. Delete still requires explicit confirmation.

Source/host scope only. After separate packaging and installation approval,
save a synthetic moment on the phone and check whether it is clear where to
tap next without reading any help. Check that **More actions** starts closed
and that the edit hint appears only after entering the editor.

### Pilot 3003 feedback — one next action for saved and reopened moments

The Founder recognized **Start reflection** on Zenfone 10, but a short
moment was blocked by an explanation below the fold. Opening an existing
moment from the list also brought the secondary controls back into view.

Both successful creation and an exact list read bring the moment into view
with the same primary action and a closed **More actions** disclosure. The
same local context check used by Reflection chooses that action: when the
saved original needs context, the card explains why and offers **Add context
to this moment** instead of a link to a blocked step. It opens the existing
editor with the original text. Cancel leaves the saved content untouched;
the next action is recalculated from the verified saved revision, not the
draft. There is no change to the gate, artifact generation or storage.

Source/host scope only. After separate packaging and update approval, try a
short and a contextual synthetic moment, both after saving and when reopened
from the list. Verify the reason and next action are together without a swipe,
the disclosure starts closed, and **Start reflection** appears after an explicit
save with enough context. No runtime acceptance is claimed for this change.

### Pilot 3004 feedback — open the clue review

On Zenfone 10 with 3004, **Start reflection** looked like a no-op when the
target was already on screen. The extra disclosure and duplicated original
made it hard to tell what to do next or which text the actions affected.

- **Start reflection** expands the clue review in the same card, without
  creating or accepting a clue. **Create a demo clue from my original** is
  the first action for a moment with no active clue.
- The pending demo clue shows its exact text and explains that the actions
  affect only this clue, not the original. **Use this clue for reflection**
  is primary. Editing or rejecting targets the clue only. Its visible state
  replaces the generic success message; failures and uncertain results
  remain visible.
- Once a clue is used, **Show a demo question** is the primary next action
  when there are no active questions. Collapsing the review itself does not
  discard session drafts. No gate, storage, or provider changes.

Source/host scope only; non-Pilot M2-C keeps its existing presentation.
Zenfone acceptance for this follow-up is pending a separately approved build:
save or reopen a contextual synthetic moment, tap **Start reflection** while
the card is already visible, and check that the next action and the target
of each clue action are clear without expanding additional details.

### Pilot 3005 feedback — reference the original once

Founder installed 3005 on Zenfone 10 and found the clue card clearer, but
the repeated text made **Edit this clue** look like editing the original.
For the Pilot follow-up, the card refers to the original above when its
content is the exact local quote (or exactly the original body). This also
applies after adoption. **Reflect on this original** adopts the existing
clue; **Use only an excerpt** opens an empty field and saves through the
existing pending clue correction, still requiring review before adoption.
Distinct clue text remains available in full. No changes to source editing,
storage, provenance, provider behavior, or non-Pilot M2-C presentation.
Device verification of this follow-up is pending a separately approved build.

### Pilot 3006 feedback — keep the active clue visible

Founder reported that the heading **Demo clue** and a collapsed adopted view
made it unclear whether Reflection would use the initial quote or their edit.
Keep the current clue in the same card before and after adoption, using
**Reflection clue you edited** when the saved text differs from the original
candidate. **Use this text** adopts that exact revision. On success change
the card status to **Adopted**, leave the text visible, and offer the existing
demo question action. When the clue is the full original, show only a stable
reference to the original above. The optional details retain provenance and
explain that adoption is not independent verification. No changes to the
original Experience, storage, request handlers, or non-Pilot presentation.
Host evidence only; device confirmation needs a separately approved build.

### Pilot 3007 feedback — one notice after editing the original

The Founder reported that editing an original after answering several questions
showed one identical source-change warning per invalidated artifact. In Pilot
Reflection, replace these warnings with one **Original updated** notice.
Count only invalidated Reflection questions; if only clues were invalidated,
say so without showing a question count. Do not show old question/answer
text or count active questions or rejected clues. The existing next action
remains available when eligible; nothing is regenerated automatically.
The older notice is informational when the user has already started new work.
No change to invalidation, retention, providers, or non-Pilot presentation.
Host evidence only; device confirmation needs a separately approved build.

### M3 Model Pilot R1 — independent connected variant (source/mock only)

The connected build must pair the explicit Vite mode and Rust feature
`android-pilot-online`, the online Tauri overlay, and the online-only Android
manifest overlay. For the independent Gradle route, apply the overlay only in
that build's **release** source set with namespace/application ID
`com.lifeos.review.pilot.model`. Keep the launcher activity in that namespace
and the no-backup/extraction resources; use a distinct visible app label.
Check the final merged manifest for INTERNET only in the connected build,
HTTPS/no cleartext, non-debuggable, the matching app ID and exclusions.
The existing offline build recipe does not apply the release overlay.

With a separately approved build and live device test, enter the key only on
the phone. Check the exact destination and fictional clue, cancel once, then
explicitly send and receive one Gemini question. Save the question and answer
locally; reopen and verify both labels and that saving caused no network call.
Also verify Skip, key clear/restart, Back/settings and draft retention, late
reply rejection, and unchanged offline Pilot data. Android keyboard and OS
clipboard behavior are device-specific; the app does not manage the clipboard.

## First/append R1 — source/host scope only

Do not run the historical emulator/package procedures for this slice. The
selected first physical target is the Founder-reported Zenfone 10 on Android 15.
Pixel 7 is deferred. Host tests do not establish device, OEM, or ARM64 behavior.
No package/identity/signing/install or real-data authority was granted.

After separate approval and candidate binding, use only synthetic data. Verify:
first question → type a draft → explicit append and inspect the original draft
→ save an answer → normal relaunch/readback → edit then cancel → edit then save
→ relaunch and confirm the updated answer with both authorship sources → cancel
source deletion → confirm it → relaunch and verify no related content returns.
Observe keyboard and visible controls, system Back/gesture, background/lock and
resume. Stop on unexpected state; do not clear an old install or assume a
failed UI acknowledgment means no commit. Normal relaunch is not crash or
power-loss evidence; the local mock does not test AI question quality.

## Independent R1 Founder review — no new Android run

Daily Reflection Saved-Response Correction R1 is a separate host-only candidate
from commit 5646a718ac0fe6251d85e7e2da70f3b2e11faae5 on branch
codex/daily-reflection-response-correction-r1. It leaves the original M2-D 036
STOP, workflow, budgets and evidence untouched. Historical APK/manual/native
results below are not evidence or authority for R1. Do not execute those native
instructions for this slice.

Consolidated diff review path (synthetic data only):

1. Inspect the reopened original Experience, unchanged question and separately
   labelled saved answer. Inspect optional prompt/response provenance.
2. Choose **Edit my response / 修改回答 / 自分の回答を編集**. The editor must
   contain the saved answer, labelled unsaved; the saved answer stays visible.
3. Change it and Cancel. No request/revision should be created; saved text stays.
4. Edit again. Blank and unchanged text cannot be saved. Save a changed answer
   once; busy controls must not create a duplicate or claim early success.
5. Reread the saved state, then make and save a second correction. The artifact
   identity, original source, Evidence, question and prompt origin must remain;
   response revisions and user authorship must be preserved.
6. Review failure/stale/uncertain test cases: draft remains visible/copyable,
   same-request reconciliation only, read-only current-state inspection, explicit
   discard wording rather than quiet draft loss.
7. Check English, Traditional Chinese and Japanese labels for the same meaning.

At the original host handoff this checklist was not_run. Later, Founder A
accepted bounded interactions using actual components and a synthetic in-memory
browser preview, not a persistence preview or Android PASS. B independently
accepted the exact diff/existing host evidence. Do not open an ordinary desktop
profile as a substitute.
Component hook/handler coverage does not prove DOM focus/layout. SQLite reopen
proof does not establish Android lifecycle, process death, physical power loss,
OEM/ARM behavior, multiprocess durability or secure physical erasure.

The independent current workflow Engineering Report records exact commands,
actual exits, preserved failures, hash deltas and final verification evidence.
The 2026-10-25 separate closeout request authorizes official R1 workflow
closure/archive and precise local commit only. Merge/PR/push/production
activation, Android/native and any next slice remain unauthorized.


## Independent R1 acceptance closeout — 2026-10-25

A: 「接受本次範圍的 browser UI 可用性」. B: 「接受精確 R1 diff 與既有 host 驗證證據」.
Accepted HEAD 5646a718ac0fe6251d85e7e2da70f3b2e11faae5, digest
aef20ce69e815c2e9946baaaf81d24227663f78eb877c60101404d9aef307613.
Host verification, agent browser smoke and Founder browser acceptance remain
separate; Android/native/physical review is not_run. A/B did not grant Git
authority; the separate closeout/local-commit request does.

Tracked decision, evidence hash index and terminal report are retained in
.ai/workflow/HISTORY/2026-10-18-daily-reflection-response-correction-r1/ after
official archive. Detailed ignored originals remain in .artifacts/response-correction-r1/.
Closing documents and exact archive-path guard synchronization change the
closeout digest, not accepted product source. No new ADR/policy/schema change.

Follow-ups: focus returns to body after cancel/success; preview server exited1
on Vite watcher EBUSY (not a clean stop). Historical last no-listener observation
is not current liveness proof. No server continuity/reload/persistence proof;
A/B did not accept the later-disclosed preview defect. No preview restart/port
query/repair here. Original036 STOP remains open. Local commit is not develop
integration, release, real-data readiness or production activation.

## Current Founder004 acceptance and authorized integration closeout

Founder `ANDROID-M2C-FOUNDER-REVIEW-004 Option A` accepted the exact synthetic
M2-C candidate after a new direct **6/6 PASS** checklist; no earlier manual PASS
was transferred. Accepted APK SHA-256:
`a4054fbf277d8ffe214121751248d15afc065dc6dcd42d7bf44c7f5bbc9002ff`.
Accepted non-workflow digest:
`024b91a34c787a57aa9a6d9220928392bd082108603ace3ff18b3f0207c7bdf8`.
Founder004 response, manual completion and acceptance closeout are frozen under
`.artifacts/android-m2c/founder-acceptance/20261004180223/`; direct six responses
and populated restart/final profile evidence are under
`.artifacts/android-m2c/founder-review/20261004172109/`. Native34/34, all original
assertions, prior failures and old9/9 remain preserved; old9/9 is historical only.

`ANDROID-M2C-INTEGRATION-CLOSEOUT-001` separately authorizes bounded factual
documentation, exact archive-prefix packaging compatibility, official
archive/reset, commits, normal feature/develop pushes and no-ff integration.
Publication/integration results must be read from the actual promotion manifest;
authority is not a claim that these operations already succeeded. Only factual
docs/package compatibility may change the accepted digest; product/APK/native
instrumentation/manual evidence remain unchanged. Historical sections below
retain their earlier pending/unstaged wording; they are not the current gate.
No production UX finalization, real data, `com.lifeos.app`, Provider/BYOK, product
network/sync, later slice or Phase4 is authorized. Physical/OEM/ARM/power-loss,
multiprocess and secure-erasure evidence gaps remain. No new ADR/status change.

## Historical Founder003 bounded rejection-UX follow-up

Direct authorities: `ANDROID-M2C-FOUNDER-REVIEW-003 Option C`, one-off
`ANDROID-M2C-FOUNDER-BOUNDED-FOLLOWUP-001 Option A`, and fixture-only
`ANDROID-M2C-REJECTION-UX-TEST-FAIL-001 Option A`. The two UI changes are a clear
post-rejection **建立新的本機示範候選** action and one concise rejection summary
with complete optional non-content history, synchronized EN/zh-TW/ja. No product
policy or stored/request behavior changes. Cycle3/max3/kernel remain unchanged.
Any new failure must preserve evidence and stop; no automatic extra correction.

Evidence root: `.artifacts/android-m2c/rejection-ux/20261004134814`. Failed83/1
focused log/test are retained; only the fixture ID was renamed and84/84 passed.
The old4232 APK and full46-path raw candidate baseline, Founder003 Step4
observation/profile and prior accepted/native/diagnostic failures remain intact.
The additive native safeguard is `scripts/android-m2c-rejection-cdp-probe.mjs`.
The original150 assertions/full31 groups are not replaced, weakened or skipped.

New full harness: `.artifacts/android-m2c/rejection-ux/20261004134814/native-owned.ps1`
with fixed new5598/CDP9238, AVD `lifeos_m2c_20261004134814_api36_x86_64` and temporary
`com.lifeos.review.m2c`. Its only base substitutions are root/ports/authority tag;
three additive rejection-presentation groups run on separately owned fresh
synthetic fixtures. No replay/clear/reinstall of old5588/5596 or failures.
Final exact empty profile receives complete emulator shutdown/restart before the
Founder gate. Build/inspect/native/canonical are independent; read their actual
receipts and the new final manifest before beginning manual review.

### New Founder004 checklist — all six items require fresh review

Do not transfer old9/9, or Founder003 Steps1–3. First rebind final exact APK,
non-workflow digest, allowlist and this owned profile. One step at a time; wait for
`Step N PASS` or `Step N FAIL`. Failure preserves screen/data/profile and stops.

1. **Original:** storage ready, temporary review identity and complete canonical
   icon; three languages. Save synthetic multiline original, directly edit the
   original before adopting anything, save and reopen exact text.
2. **Inspect:** separate local-demo candidate and visible scope note; only one
   primary original. Optional full text is the persisted candidate; provenance
   remains local_mock / mock / null.
3. **Correct:** candidate-not-original label and new-pending-revision help.
   Cancel without change, then save a distinct correction: original unchanged,
   corrected words visible and awaiting review.
4. **Adopt/reject/recreate:** adoption permits a question without changing the
   original. On another synthetic moment reject once, then explicitly **建立新的
   本機示範候選**; it is a new candidate awaiting review, not restored old content.
   Reject again: one primary summary, history count2 in a collapsed optional view.
   Expand both non-content metadata records, then collapse. No rejected text or
   question from rejected Evidence. Check corresponding EN/ja labels and hierarchy.
5. **Optional response:** question and user answer distinct. Unsaved draft is not
   durable/context. Save one response; elsewhere skip without a response. Stored
   content retains its creation language.
6. **Reopen:** restart exact disposable app; original, adopted candidate, saved
   response and locale retained. Rejection summary/history remain inspectable.
   Existing source edit invalidation/delete purge unchanged; scroll safe areas,
   no horizontal overflow, everyday labels in all three languages.

Founder diff and manual review are pending, not acceptance/promotion. No real
data, production identity, provider/BYOK, network/sync, archive/reset or later slice.

## Historical UX001 gate — historical acceptance does not transfer

`ANDROID-M2C-SOURCE-CANDIDATE-UX-001` permits presentation-only source-versus-
candidate correction and fresh verification. Prior Founder002 manual9/9, APK
`5b25c2f69c3069b91e7d943b1c41b0562217832a48c056175900dbc196ebff7c`, revision50
and exact46-file baseline are preserved under
`.artifacts/android-m2c/source-candidate-ux/baseline`. The revised candidate must
receive its own consolidated diff/manual review; revised manual status is not_run.

Preserve accepted AVD `lifeos_m2c_20261004072106_api36_x86_64` / emulator-5588 and
all prior failed fixtures. Current SYNC001 follow-up uses the byte-derived full harness
`.artifacts/android-m2c/source-candidate-ux/sync-20261004105142/native-owned.ps1`,
with repository-root location, fresh serial5596/ports5596-5597/CDP9236 and SYNC001
authorization tag as its only substitutions. Older5590 failed and diagnostic
5592/5594 fixtures are preserved and never reused. All ownership guards, existing31 native groups and full
final-profile shutdown/restart checks stay intact. Do **not** run the old fixed-
5588 harness against the accepted emulator. Recheck target absence before creation.

### Historical UX001 short manual checklist — no transfer to Founder004

Bind revised APK/hash/digest, exact owned AVD/serial/package and fresh synthetic
profile before Step1. Any unexpected failure: preserve evidence and stop, without
clearing/reinstalling/repairing. Automated native PASS is not manual acceptance.

1. **Original text:** ready storage, temporary identity/canonical icon and all3
   languages; save synthetic text, open **編輯原文** directly before adopting any
   candidate, save a source revision and reopen exact original. No reconfirmation
   of the source is required for source editing.
2. **Candidate inspection:** show **線索候選・本機示範**. Read the visible note:
   initially quoted original, no AI extraction/analysis, candidate actions do not
   modify original. Only one primary original; optional full candidate text equals
   persisted content. Optional provenance remains local_mock / mock / null.
3. **Candidate correction:** label says **候選內容（不是原文）** and help explains a
   new pending revision. Cancel leaves data unchanged; save a distinct correction,
   verify pending review and unchanged original, full corrected words visible.
4. **Adopt/reject:** **採用為反思線索** leaves original unchanged and permits a
   question, not independent factual verification. On another synthetic moment,
   **不採用** excludes rejected content; no question from rejected Evidence.
5. **Optional response:** question and optional answer clearly distinct; unsaved
   draft is not stored/context. Save a user-authored response; another moment can
   skip without saving a response. Stored content retains its creation language.
6. **Save/reopen:** restart this exact disposable app, verify exact original,
   adopted candidate and saved response plus locale; source edit still invalidates
   dependent content and explicit deletion retains existing purge policy. Check
   scroll safe-area/no horizontal overflow and everyday labels in all3 languages.

These six checks replace—not inherit—the historical9/9 for the revised APK.
No production identity/real data/provider/network, promotion/archive/reset or
later slice/Phase4 is authorized.

### Historical verification stop — no acceptance transfer

Revised debug APK SHA-256:
`4232e71d887ee2d30c9275a7d047af8d960784ba34548eae3015e2f0a919869d`.
Build/inspect, focused UI79 and Node contracts10 passed. Native run
`20261004093607`, separately owned `lifeos_m2c_20261004093607_api36_x86_64`
on5590, stopped with exit1 at existing `status-review` / `status-unconfirmed`.
Storage diagnostic was ready; this is not proof of the failed condition's cause.
In that historical run the complete reflection journey/final restart were not
reached, and canonical was not_run. This evidence remains a failed run, regardless
of later verification; see the current SYNC001 binding before beginning review.

`.artifacts/android-m2c/native-review/20261004093607/FAILED.txt`, failed-screen.png,
failure-current-preserved.db/receipt and ownership/AVD are preserved. No replay,
clear, repair or reinstall is authorized by this stop. Accepted old5588 and its
baseline are unchanged. Read-only analysis suggests the existing probe opens a
record and immediately clicks a debug control without awaiting asynchronous
readiness; that is a hypothesis, not a proven instrumentation-only cause.

## Authority and preparation

Initial slice: `ANDROID-M2C-SYNTHETIC-REFLECTION-001`; current presentation-only
follow-up: `ANDROID-M2C-SOURCE-CANDIDATE-UX-001`. No production `com.lifeos.app`, real
data/profile/phone, provider/history/Pattern/Growth/import/export/backup/sync,
promotion/archive/reset/release or later slice. Use synthetic text only. Exact
APK/working-tree digest and owned profile must be rebound before review; earlier
manual passes never transfer. Preserve any unexpected failure, do not repair it.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
pnpm android:m2c:build
pnpm android:m2c:inspect
# Current follow-up: exact fresh RunId and mapped owned target only (see above).
powershell -NoProfile -ExecutionPolicy Bypass -File .artifacts/android-m2c/source-candidate-ux/sync-20261004105142/native-owned.ps1 -RunId <fresh-RunId>
```

Tools/JDK/SDK environment stays process-local. The initial historical harness
used5588/CDP9228; do not operate its accepted profile. Current mapped full native
run uses a new dedicated Android36 x86_64 AVD, `emulator-5596`, CDP9236 and absent
`com.lifeos.review.m2c`, with ownership/APK evidence under
`.artifacts/android-m2c/native-review/<RunId>/`. Existing emulators are preserved
without commands. Serial/port/package/AVD collisions and physical/unsafe device
inventory block the run. Native exports only exact owned synthetic app-private
fixtures to evidence; this is not a product export/backup API.

## Evidence

- `.artifacts/android-m2c/authorization.txt`: exact Founder scope.
- Baseline/focused/canonical logs and result receipts: host evidence only.
- `.artifacts/android-m2c/review.apk` and `apk-inspection.txt`: debug x86_64 APK,
  temporary identity, signature, no INTERNET, backup/transfer excluded, icon.
- Native ownership, screenshots, DB/receipt snapshots, fault/request metadata and
  native result: exact disposable emulator evidence; preserve failures.
- Final manifest after the last change: exact path inventory, non-workflow
  digest, APK SHA256/build provenance and evidence hashes. It must not relabel
  this candidate as M2-A/M2-B acceptance.

## Historical nine-item Founder checklist — prior candidate only

Automation is not manual acceptance. Begin only after the consolidated diff,
final APK/digest and disposable profile are verified; respond `Step N PASS` or
`Step N FAIL`. Failure preserves evidence and stops review.

1. Confirm review identity/complete pale-ring icon, offline synthetic disclosure,
   empty ready profile and English/Traditional Chinese/Japanese controls.
2. Save a multiline synthetic Experience that includes a feeling and a quotation;
   reopen exact source. No candidate is generated automatically.
3. Explicitly show local demonstration; the main original stays visible once.
   An exact repeat says it uses the original above; expand the full-text view to
   inspect it. Open Source information -> Technical details (optional): verify
   `origin=local_mock`, `provider=mock`, `model=null`, then collapse the details.
   Correct pending words, cancel once, then save a
   correction; the changed text must be visible and still await fresh review.
   Explicitly accept this version; its full text remains expandable.
4. Request the neutral question; inspect mock prompt origin. Type a response but
   do not save; close/reopen or restart and confirm the unsaved response is gone.
5. Write and explicitly save your response. Reopen after process restart; exact
   response remains, with user response provenance distinct from mock prompt.
6. In a separate synthetic moment, reject pending words: current candidate text
   is removed and cannot produce a question. Create a new candidate by explicit
   action, accept it, request a question and skip; no user response is saved.
7. Edit the source of the answered journey; old Evidence/Reflection is visibly
   invalidated and ineligible. No silent regeneration, substitution or summary.
8. Cancel deletion and verify no change. Then explicitly delete that source;
   reopen after restart and verify no source/dependent active text remains.
9. Review three-language everyday wording, source/author disclosure, calm neutral
   saved feedback, locale restoration, safe-area scrolling and final scope limits.
   No disposable debug probes or raw technical JSON should appear in the ordinary
   layout. Verify optional source/technical information remains accessible.

Founder acceptance, if later explicitly granted, concerns this exact unstaged
candidate only and does not grant promotion or a later slice. Confirmed artifact
correction/deletion, response correction and superseded-history controls are not
part of this review build. Superseded content follows Accepted retention; parent
deletion purges dependent text. No physical secure-erasure guarantee.

## Preserved initial evidence and current correction gate

The initial Founder review passed Steps 1–2 and stopped at Step 3 on information
burden, not a storage failure. The direct request, live DOM/screenshot and exact
synthetic database/receipt are preserved under `founder-review/20261004063458`
and `ui-correction-1`. Review cycle 2 changes presentation only. Prior manual
passes never transfer; the corrected exact candidate requires all nine steps.
Corrected build/inspection passed for APK SHA-256
`5b25c2f69c3069b91e7d943b1c41b0562217832a48c056175900dbc196ebff7c`.
Native run `20261004072106` passed 31 groups on its new dedicated API36 x86_64
AVD, including trilingual presentation assertions and actual pending,
optional-origin, corrected-pending and answered screenshots. The final empty
DB/receipt survive exact emulator shutdown/restart, with no pending names; the
owned emulator is stopped after automation. Correction focused47/mobile
contracts38 and canonical verification passed; current final digest/stabilized
verification must be read from the current Sprint Report and final manifest.
These are engineering results, not Founder acceptance. Initial evidence below
is historical. A preflight refused retiring old TCP ports without creating or
changing any AVD; its refusal log/receipt remain preserved.

Debug build2/inspection2 passed for APK SHA-256
`805bb82c513e6db0627454d572a4a9b955d8ee556d3381f91d0d63cdf51edb41`.
Native run `20261003230216` passed 31 recorded groups on
`lifeos_m2c_20261003230216_api36_x86_64`, `emulator-5588`, API36 x86_64.
Its final empty ready DB/receipt survive full emulator shutdown/restart with no
pending files. The owned emulator is stopped after automation; rebind its exact
owner/APK/profile and reopen visibly before Founder review, not a prior M2-B AVD.

Focused host frontend/shared Harness57 and direct Rust8 checks passed. Final
canonical result and exact candidate digest must be read from the current Sprint
Report and ignored `final-manifest.json`; do not infer them from native success.
Historical native run1 failed in probe selection and remains preserved. The
first canonical attempt failed on an outdated active Android identity test;
that narrow test synchronization does not change product behavior.

Corrected Founder consolidated diff and nine-item manual UI review remain pending. Review
each step separately, preserve unexpected failures, and do not auto-accept.
No actual power-loss/crash or physical-device proof, production readiness, real
data activation, accepted ADR change or later-phase permission is implied.

## SYNC001 test-only follow-up

Founder directly authorized offered ANDROID-M2C-UX-NATIVE-SYNC-001 Option A with exact response「授權 test-only 同步修正與重新驗證」. Only native readiness guard, regression safeguard/evidence/docs; all150 original probe assertions unchanged. Wait for exact source ID/body/revision/authorship/lineage, idle journey and enabled controls with30000ms predicate timeout; no arbitrary delay/replacement request/auto-retry. Product/UI/store/Harness/provider/domain/Rust/schema/config/icon untouched. Rebuild/inspect/focused/full affected native/canonical on fresh owned5596/CDP9236 before new manual gate. Original failure cause unconfirmed, no retrospective proof claim.

Two preceding diagnostic runs completed only the English status-review checks,
then intentionally stopped: not full native PASS and not proof of the original
failure cause. Their logs,150 retained assertions, atomic click timeline and
profile snapshots remain preserved. Full native has passed. Canonical must bind the exact final digest before Founder
review; manual6/6 is not_run and prior9/9 remains historical only.

## Historical SYNC001 verification binding and handoff

Current verified native binding: run20261004105142, AVD lifeos_m2c_20261004105142_api36_x86_64, emulator-5596, CDP9236, temporary com.lifeos.review.m2c. Build/inspect PASS; APK4232e71d887ee2d30c9275a7d047af8d960784ba34548eae3015e2f0a919869d (byte-identical to the earlier UX APK). Focused UI79/contracts11 PASS; full native31/31 PASS, including all3 language source/candidate semantics and focus, exact unchanged source after correction/adoption, pending review, content/provenance disclosure, user response, rejection/skip/drafts/invalidation/duplicates/stale/rollback/reconciliation and restart. Final fresh/restarted DB and receipt byte-identical, no pending names, zero source/artifact revisions/migration receipts. Physical-device/power-loss limits unchanged. Original failed5590 and diagnostic5592/5594 preserved; old cause remains unconfirmed. Revised manual6/6 not_run; old9/9 historical only.

Canonical evidence and final non-workflow digest are bound in
`.artifacts/android-m2c/source-candidate-ux/sync-20261004105142/final-manifest.json`
and its canonical-result.json receipt. Founder diff/manual gate may open only
after that exact receipt passes; automation does not supply manual acceptance.
The disposable profile is retained, not replaced/reinstalled, for the six checks
in runbook15. Rebind exact serial/AVD/APK before Step1; other emulators are
inventory-only. Keep all46 paths unstaged and index empty; no promotion/archive,
production/real data/provider/network/sync or later slice/Phase4.

## Preserved rejection-UX native binding — before Founder004 acceptance

New rejection-UX build/inspect PASS, APK a4054fbf277d8ffe214121751248d15afc065dc6dcd42d7bf44c7f5bbc9002ff; focused84/84 and contracts12/12 PASS. Independently owned run20261004134814 / lifeos_m2c_20261004134814_api36_x86_64 / emulator-5598 / CDP9238 passed34/34 groups (all31 original plus3 additive locale groups). Final fresh/restarted DB/receipt byte-identical, no pending names, counts0|0|0. Native snapshots and screenshots cover3 distinct IDs,2 purged rejected heads,1 new pending candidate, one summary, all6 returned non-content metadata fields, unchanged original and disclosure-only backend invariance in EN/zh-TW/ja. Engineering evidence, not Founder visual acceptance. Canonical is independently required; its actual result and final digest are authoritative only in the final manifest. Fresh manual6/6 not_run.

Authoritative current receipts/final manifest: `.artifacts/android-m2c/rejection-ux/20261004134814/`. Prior Founder0033/6 before Step4 stop, accepted0029/9 and all failure/diagnostic evidence remain historical; no PASS transfer. All47 candidate paths stay unstaged; no acceptance/promotion or later slice. Physical-device/power-loss/OEM/ARM/multi-process/secure-erasure limits unchanged.


### Connected R2: original Founder device review checklist

After separately authorized build, signing, and installation, test on Zenfone 10
with fictional material only. Reopen one saved moment, then use **Ask me one
question** without opening the local demo. Cancel the preview once; then check
that it contains the exact original and, if selected, an exact optional excerpt.
Try an unrelated focus and confirm it cannot be used. A prior edited demo clue
must not be used alone or silently included.

Answer and save. Expect **See another angle**, not another mandatory question.
Check that the second preview contains only this original, Gemini question and
saved answer. Cancel it once, then explicitly confirm. Try an answer with too
little information: feedback should admit uncertainty rather than fabricate or
prescribe. Try leaving during a request and check that a late reply cannot
appear on another moment. Reopen to confirm the question and answer are still
local, and the angle is not. Changing or deleting the source should not silently
reattach the old pair. This checks ordinary reopening, not crash durability.
Do not use real journal data, copy the API key into evidence, or upload diary
contents, databases, or sensitive screenshots.

### 3101 Founder-reported result and R3 planning

On 2026-11-18 JST, the Founder reported the question/answer stayed after
reopening; only the session-only angle disappeared. The three additional
cancellation, phone lifecycle/draft/stale-response and disposable-source
deletion checks were explicitly reported OK. The earlier feedback was useful
(`有帶來不同的理解`). These are bounded direct Founder reports; the optional
focus and insufficient-context cases are not separately established.

Source-only follow-up removes the connected focus/demo controls and makes
question versus feedback explicit. It does not change the installed 3101
or the offline build. The next integrated source/package decision is routed
through `docs/product/01_Android_Model_Pilot_R3_Proposal.md`, not another
round of separate APKs for each small UI correction. No real-data, historical
summary, credentials retention, or feedback persistence is enabled by this
report alone.

### R3 credential source and remaining schema gate

The Founder authorized R3 sources and host tests without a package. The
connected source now offers opt-in encrypted remembered key, native restore,
session-only replacement and clear/forget. Before any later standalone build,
stage all three `android-pilot-online/java` files into the fresh Gradle project;
the native build guard rejects an incomplete or stale copy. This does not
authorize running it, changing a previous candidate or touching a device.

Host crypto/lifecycle checks use a synthetic injected backend; Java compilation
against Android APIs is not Android Keystore proof. Later phone review must
test opt-in restart and same-signer update continuity, switching to session-only,
forget, an unreadable credential and no plaintext fallback; do not ask the
Founder to export, photograph or paste keys. Check backup/transfer exclusions
statically first; device-transfer validation needs its own explicit scope.

Feedback save and supplements remain unimplemented pending the proposed
ADR-0013 exact model-turn schema/upgrade checkpoint. Do not show a saved
feedback state, add temporary JSON persistence or claim the full R3 journey
works. Keep the one integrated device review after those sources are complete,
not a package merely for the credential work.
