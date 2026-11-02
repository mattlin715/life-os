# Windows Founder Dogfooding Package R1

## Reflection projection-order follow-up — 2026/10/31

The generation-r1-001 candidate remains preserved, unaccepted, and is not replaced
in place. The ordering follow-up must first reproduce the submitted reverse-ID,
same-time multi-question payload through real SQLite; then validate/review the
single runtime projection boundary before canonical verification and any new
ReviewSuffix package. Evidence and independent source bindings belong under
`.artifacts/reflection-projection-ordering-r1/run-001/`. Manifest gitSha remains
the base commit, never a substitute for all uncommitted source hashes.

Canonical Reflection display order follows stored created_at/id with SQLite
BINARY tie-breaking, not generation order or parsed chronological instants.
New questions may interleave with old questions when their sort keys interleave.
Identify Q1/Q2 by their selected record/ID where observable, not by fixed list
position or unique wording. Adding questions must retain the old draft and saved
answer; drafts need not survive shutdown. An empty/malformed/stale/uncertain
result is not success and must not be retried during the same manual acceptance.

No installer or app execution is authorized by this follow-up. After independent
Founder diff/package acceptance and new environment/install consent, run the
existing synthetic journey with explicit AI_PROVIDER=mock at every launch.
Keep the stopped profile untouched and all historical causes UNKNOWN. Remote IPC
still exists; package inspection is not zero-network or manual-lifecycle proof.

## Reflection additive generation follow-up — 2026/10/31

Use the new cumulative schema-v5 candidate only after separate Founder diff and
package review. The previously accepted provenance-only installer remains
unchanged and separately bound. Build receipts and exact source hashes live in
`.artifacts/reflection-generation-correction-r1/run-001/`; no commit is implied
by a manifest whose gitSha is still the base HEAD. Neither host PASS nor this
runbook authorizes installation, launch, cleanup or use of the stopped profile.

In a future separately authorized NEW synthetic environment: save a source and
confirm Evidence; generate Q1; retain an unsaved Q1 draft while explicitly adding
Q2; verify the draft and Q1 ID are retained. Save answer A, normally close/reopen,
correct to B, normally close/reopen, then delete the source and normally
close/reopen to confirm its related content disappears. Also add a question to
an already answered Q1 and confirm its answer/provenance remain. A provider may
return multiple questions: the complete batch is appended once or refused,
never truncated or split. Rapid same-source clicks must not start a second
in-flight generation; another explicit request is allowed only after completion.

Preserve the existing identity, gates, current-user install and process-scoped
`AI_PROVIDER=mock` requirements. New package review, environment/install/launch,
manual observations and Founder acceptance are independent gates. An uncertain
save, unexpected state or normal-close failure stops the journey without retry,
DB repair, force-kill, profile reuse or036 continuation. The original session's
historical trigger remains UNKNOWN; host reproduction cannot fill that gap.

## Desktop Reflection provenance follow-up — 2026/10/29

This independent host/build follow-up uses the existing schema-v5 candidate,
not this runbook's historical schema-v4 baseline. See
[architecture/17](../architecture/17_Desktop_Schema_v5_Founder_Dogfood_Activation_Candidate_R1.md).
First-save and response-correction host regressions reproduced a post-commit
failure: frontend user provenance omitted four nullable fields while the writer
stored explicit nulls. The bounded runtime fix validates the complete response
operation before mutation and canonicalizes only those four absent/null fields.
The writer and full postcondition remain unchanged. Failed synthetic fixtures
and actual outputs are retained in the ignored follow-up evidence directory.

Only host verification and a new unsigned local candidate package/inspection are
authorized here. No installation, application launch, OS-account creation,
runtime database, real/historical-profile access, manual review or Android/native
operation has been performed or authorized by this slice. Package creation does
not grant any of those permissions or accept the new diff.

After separate Founder authorization:

1. Provide a **new dedicated disposable Windows account/profile**. Do not reuse,
   overwrite, clear or probe an existing candidate/review profile. A new worktree
   or changed `APPDATA` does not isolate Tauri or its dependencies.
2. Bind the exact installer/manifest SHA-256, candidate identity/title and
   current-user installation to that account. Verify the new exact target root
   and ownership before launch; collisions or ambiguous paths stop review.
3. Preserve `com.lifeos.founderdogfood`, the `founder-schema-v5` binary feature,
   `VITE_LIFE_OS_FOUNDER_SCHEMA_V5=1`, Tauri-derived root and fixed `life-os.db`.
   Build with the existing candidate script and a new `-ReviewSuffix`; never
   replace old output. No new DB path or identity override is part of this fix.
4. For **every** initial launch and reopen, use the exact installed executable
   under the disposable account with process-scoped `AI_PROVIDER=mock`. Do not
   assume a Start Menu launch inherits the earlier process environment. Do not
   use the ordinary desktop launcher, a dev server, or provider failure as the
   way to choose mock. Confirm the UI reports mock before creating content.
5. Use only a unique synthetic source: review Evidence, request a question,
   save A, close normally, reopen the same controlled way and see A; correct to
   B, close/reopen and see B; delete that source, then reopen and confirm source
   and dependent content are absent. Question and source lineage must remain
   unchanged during response correction. These steps are **not_run** here.
6. Migration/recovery disclosure, ambiguous save, unexpected existing data or
   close/reopen failure stops the review. Preserve that new synthetic profile
   and exact package evidence; do not retry, repair, clean sidecars or force-kill.
   Uninstallation and profile cleanup require their own explicit authority.

`AI_PROVIDER=mock` currently takes the existing unsupported-provider branch and
directly selects mock. The explanation is unchanged; remote generation IPC is
still in the binary. This is not removal of all network capability. Host SQLite
reopen tests are not packaged UI acceptance, Android lifecycle, power-loss,
OEM/ARM64/physical-device durability, or secure erasure proof. The original036
STOP and ordinary-profile migration/recovery gates remain independent.

## Status and boundary

This Book One runbook covers one Windows-only, unsigned, local, non-distributed
Founder review package for the current schema-v4 Life OS desktop product. It is
not a Private Alpha, deployment, release, signed build, or schema-v5 activation.

The package changes no React product behavior, Rust runtime behavior, SQLite
schema, provider, ContextPacket, consent policy, or persistence contract.
Production `SCHEMA_VERSION` and the supported startup maximum remain 4.

The separately authorized schema-v5 Founder activation candidate reuses the
same isolated identifier only for a controlled migration/restart review. Its
implemented, Founder-reviewed, and promoted boundary and separate build command are recorded in
`docs/architecture/17_Desktop_Schema_v5_Founder_Dogfood_Activation_Candidate_R1.md`.
This runbook's schema-v4 package remains the baseline and is not rewritten as a
schema-v5 package.

Candidate build (does not install or launch):

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-founder-schema-v5-candidate.ps1
```

## Identity and data-profile separation

| Surface | Ordinary development app | Founder dogfooding package |
| --- | --- | --- |
| Product name | `Life OS` | `Life OS Founder Dogfood` |
| Tauri identifier | `com.lifeos.app` | `com.lifeos.founderdogfood` |
| Window title | `Life OS` | `Life OS — Founder Dogfood (Private)` |
| Bundle | normal configured targets | NSIS, current user only |

The SQL plugin resolves the relative `sqlite:life-os.db` beneath Tauri's
application config directory. Tauri derives that directory from the bundle
identifier. Therefore the Founder package resolves a different directory from
the ordinary development app without accepting or constructing a database path
in renderer or package configuration.

On Windows, the expected profile roots are under `%APPDATA%`:

- ordinary development: `%APPDATA%\com.lifeos.app`
- Founder dogfooding: `%APPDATA%\com.lifeos.founderdogfood`

The build and verification scripts do not open either directory or database.
The Founder must perform installation and data-retention observations in a
disposable Windows account or profile.

## Build

From the repository root:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-founder-dogfood-package.ps1
```

The script:

1. fails unless the package/source contract and exact changed-path allowlist pass;
2. builds the bundled frontend, then supplies the exact Founder override through Tauri's `TAURI_CONFIG` merge while compiling only the final application binary with Tauri's production `custom-protocol` feature;
3. applies the Windows GUI subsystem and Rust-compatible CRT entry-point flags only to that final binary, restores the caller's environment, and verifies both embedded Founder markers and PE subsystem 2;
4. bundles one unsigned current-user NSIS installer with the same override and revalidates the binary;
5. copies it under ignored `.artifacts/windows-founder-dogfood-r1/<git-sha-prefix>/`;
6. writes and revalidates `manifest.json` from the closed installer bytes;
7. does not install, start, stop, uninstall, distribute, deploy, or release it.

An existing review output directory fails closed. Remove that exact ignored
directory explicitly before intentionally rebuilding.

The build command is deterministic in inputs, selection, output naming, and
validation. This document does not claim byte-identical NSIS output across
machines or toolchain versions. The manifest SHA-256 identifies the exact bytes
that are reviewed.

Founder manual Step 3 invalidated an earlier binary that lacked explicit
`TAURI_CONFIG`: it opened as `Life OS` and attempted the development `localhost`
URL. That evidence is preserved in the workflow archive. The current build
contract requires the override during final-binary compilation because bundling
cannot replace an executable's already embedded Tauri configuration.

## Content-free manifest

`manifest.json` has exactly six fields:

- `applicationVersion`
- `gitSha`
- `buildTarget`
- `artifactFilename`
- `artifactSize`
- `sha256`

It contains no user path, database metadata, Life OS content, secret, runtime
state, timestamp, credential, or machine identity. The Git SHA identifies the
repository baseline; an uncommitted review package is not thereby promoted.

## Automated verification

Focused contract tests run inside the canonical verification path:

```powershell
node --test .\scripts\founder-dogfood-package.node-test.mjs
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

The tests cover identity collision, base-config drift, schema drift, Windows
GUI/console/malformed PE classification, forbidden
override surface, exact manifest keys, filename path leakage, extra metadata,
size mismatch, checksum mismatch, ignored output, and the actual repository
allowlist. They do not replace the Founder install/start/restart/uninstall
review.

## Founder manual review

Keep the installer local and use a disposable Windows account or profile.
Review one step at a time:

1. Confirm the installer says `Life OS Founder Dogfood` and does not replace the development app.
2. Install for the disposable current user and acknowledge the unsigned local-build warning without distributing the file.
3. Launch from the installed shortcut with no repository dev server or console.
4. Complete one local-mock Experience → Evidence → Reflection → completion journey; Pattern and history stay optional.
5. Close normally and restart. Confirm saved schema-v4 records reconstruct and session-only panels reopen closed.
6. Confirm Experience-only export is honestly labelled and no Upgrade, migration, backup, restore, updater, telemetry, or release control exists.
7. Check English, Traditional Chinese, and Japanese in a narrow window and by keyboard.
8. Uninstall. Record whether `%APPDATA%\com.lifeos.founderdogfood` remains, and confirm `%APPDATA%\com.lifeos.app` is untouched. Do not delete either directory as part of this review unless separately authorized.

Manual success is evidence for bounded Founder dogfooding only. It does not
authorize promotion, distribution, deployment, release, schema-v5 activation,
Phase 4, or Android work.
