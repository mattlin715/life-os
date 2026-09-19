# Product Review

Status: approved
- Sprint ID: 2026-09-18-desktop-schema-v5-real-profile-post-commit-recovery-r3
- Artifact schema: 1.0
- Authoring role: chief_product_theorist

## Mission Interpretation

Remote publication is closed. Real-Profile Access Gate 2, Recovery Gate 3, and the separately authorized local R2B installation correction were bounded, non-transferable authorities. They do not authorize Founder manual UI review, further application launches, synthetic writes, publication, release, Phase 4, or Android.

## Acceptance Criteria

- Install only the exact hash-bound R2B installer once with `/S`, without launching Life OS during installation.
- Prove the installed executable has the accepted R2B size and SHA-256 and that the disclosed profile technical files remain byte-identical during installation.
- Before recovery, recheck zero Life OS processes and revalidate the exact immutable claim for operation `3dcac99f4be99989f96644a887df87cb` and classification `exact_historical_frontend_v4_post_commit_manifest_v1`.
- Launch the ordinary desktop application only as much as required to execute the authorized recovery exactly once, with no retry.
- Preserve the schema-v4 backup, prior receipt, prior quarantine, and all unrelated technical files.
- Verify content-free postconditions only after normal application close and zero process count.
- Select or print no personal Experience, Evidence, Reflection, or Pattern content.
- Stop before Founder manual UI review.

All authorized installation and Recovery Gate 3 criteria passed in one attempt.

## Installation Correction Result

- Installer invocation count: `1`; argument: `/S`; exit code: `0`.
- Life OS process count was `0` before and after installation, and no Life OS process was observed during installation.
- Exact installer remained size `5805550`, SHA-256 `30ec0a075f79728b6c87a2f9ba9c5ed3ec1d7188d85b96650fc23cd8715ade9b`.
- Exact six-field manifest remained size `380`, SHA-256 `3da38106c8d40917de594f81be7bf449f57441f151a9c8635727bb675ee621f7`, bound to Git SHA `8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899`.
- Installed executable became size `26935296`, SHA-256 `164355874bb158ea411da09aa24821c5273f76d8c4f721d91c595f2dce991db6`.
- The official installed-binary package contract passed.
- All disclosed real-profile technical files were byte-identical before and after installation.

## Recovery Gate 3 Result

- Application launch count: `1`; review action count: `1`; exact recovery completion count: `1`; normal close count: `1`; application exit code: `0`.
- Immediate pre-mutation revalidation matched operation `3dcac99f4be99989f96644a887df87cb` and classification `exact_historical_frontend_v4_post_commit_manifest_v1`; lifecycle writes were `disabled`.
- Migration was not rerun. No restore, repair, retry, schema decrement, backup replacement, backup deletion, or unapproved sidecar action occurred.
- Live database remains schema v5 with 95 schema objects and manifest `2dc9f712464806e84ab05bf0bfde31761b1b8de5a9756c819a9de3a95f91019e`; integrity is `ok`; foreign-key violations are `0`.
- The live database changed only as required for lifecycle activation: final size `786432`, SHA-256 `c39a51b890ce045fc35c3a291d2de308205b354e9dc6a0f71fa73c56e6ecfcdb`.
- Operation phase is `v5_ready`; outcome is `lifecycle_writes_enabled`; lifecycle writes are `enabled`.
- Exact new content-free receipt: `post-commit-manifest-recovery-3dcac99f4be99989f96644a887df87cb.receipt.json`, size `2429`, SHA-256 `9f4136912f4b01609dea96b5f5bd18ef218baa1b384e2140979aa3d78d78ed0e`.
- The verified schema-v4 backup, staging evidence, prior recovery receipt, prior quarantine, and all unrelated technical files remained byte-identical.
- Live WAL, SHM, and rollback journal were absent after normal close; unexpected technical-file changes were `0`.
- All handles were closed, final Life OS process count was `0`, and no personal content was selected or printed.
- The installed R2B executable remained byte-identical during recovery.

## Founder Manual Review Progress

- Step 1: **passed by exact Founder report** — the ordinary UI opened without a migration or recovery loop.
- Step 2: **passed by exact Founder report** — previous saved Experiences are visible.
- Step 3: **passed by exact Founder report** — previous saved Reflection state is reconstructed.
- Step 4: **passed by exact Founder report** — lifecycle-write controls are enabled.
- Step 5: **passed by exact Founder report** — the retained verified schema-v4 backup is disclosed.
- Step 6: **failed by exact Founder report** — the single authorized save attempt reported “pattern_provenance_sources_mismatch”.
- Step 6 containment: **passed by exact Founder report** — Life OS closed normally; separate process checks confirmed count 0.
- Step 6 corrective retry: **passed by exact Founder report** — after the verified legacy-provenance correction and exact NSIS-payload installation, the single authorized synthetic Experience save completed successfully.
- Post-report process observation found the previously launched Life OS PID no longer running. Automation did not terminate it and does not infer how it closed. Step 7 therefore requires a separately authorized relaunch.
- Step 7: **passed by exact Founder report** — after one separately authorized relaunch, the synthetic Daily Reflection flow completed normally.
- Step 8: **passed by exact Founder report** — confirmed Evidence, saved Reflection responses, and the completed reflection status are all represented normally for the synthetic record.
- Step 9: **passed by exact Founder report** — the Life OS window closed normally through the single authorized normal product-close action.
- Step 10: **passed by bounded content-free observation** — the Life OS process count was exactly 0 after the normal Step 9 close.
- Step 11: **passed by exact Founder report** — the exact installed ordinary application reopened without a migration or recovery loop.
- Step 12: **passed by exact Founder report** — prior saved records and the exact synthetic test Experience remain present after normal close and relaunch.
- Step 13: **passed by exact Founder report** — no duplicate migration, retry, recovery receipt, or recovery operation occurred during the bounded UI-visible review.
- Step 14 close: **passed by exact Founder report** — the Life OS window closed normally through the single authorized upper-right `X` action.
- Step 14 structural inspection: **passed after explicit Founder classification** — process count was 0; live WAL, SHM, rollback journal, and root `.life-os-fresh-v5-*.db` candidates were absent. The only present disclosed path was the exact operation-owned size-0 `life-os-3dcac99f4be99989f96644a887df87cb.operation\staging.db`, last written `2026-09-01T14:36:55.8448595Z`; it is expected retained recovery evidence required by the repository contract, not unexpected live staging.
- Step 15: **passed by exact Founder report** — English, Traditional Chinese, and Japanese recovery/result wording is clear and consistent; the Founder reported only technical backup/recovery copy and activated no lifecycle or data control.
- Step 16: **passed by exact Founder report** — keyboard navigation, visible focus, narrow-window layout, and long technical values behave normally without activating any data or lifecycle control.
- Founder Manual Review Steps 1–16 are complete. The application remains open because Step 16 did not authorize close or restart; no post-review promotion or publication action has occurred.
- No personal Experience, Evidence, Reflection, or Pattern content is recorded in workflow evidence.

## Immutable Step 6 Failure Diagnosis

The authorized closed-file diagnosis completed through one actual SQLite mode=ro&immutable=1 connection. An initial Windows Python alias lookup failed before any SQLite handle opened; the available local Anaconda interpreter then performed the single database inspection.

- Life OS process count before and after: 0.
- WAL, SHM, and rollback journal count before and after: 0.
- Query-only mode: enabled.
- Database user_version: 5.
- Integrity: ok; foreign-key violations: 0.
- Database size before and after: 786432.
- Database file identity before and after: device 3524333429794690964, file ID 28991922602376085.
- Database SHA-256 before and after: df3a90730c5fa607212ef1b4886c4b7fe59d6c3c9339ddd9b9aa546817dcaa6b.
- Database byte identity before and after: exact.
- Handles closed: yes.
- Active source heads: 16; source revisions: 16; active Experience projections: 16.
- Exact prescribed synthetic test Experience match count: 0. The failed Step 6 attempt did not create the test record.
- Active patterns: 4; all 4 use legacy-v4-raw serialization.
- All 4 have the same bounded mismatch class: their declared Evidence/Reflection source set differs from both canonical provenance sourceArtifactIds and projected legacy provenance sourceArtifactIds.
- The normalized dependency relations themselves are structurally consistent: source dependency mismatches 0, Evidence relation mismatches 0, Reflection relation mismatches 0.
- No personal content was selected or printed.

## Root-Cause Classification

The schema-v5 migration preserved the legacy raw provenance sourceArtifactIds in canonical provenance. The current pattern verifier instead requires those historical source arrays to equal the complete declared Evidence/Reflection dependency set. For these four historical patterns, raw and canonical legacy provenance agree with each other but omit part of the dependency set, while normalized dependency rows correctly represent the declared sources. This exact historical compatibility shape was not covered by the promoted regression fixtures.

A profile repair is neither authorized nor recommended as the first correction. The narrowest local-first correction is repository-only: preserve immutable historical provenance bytes, continue requiring normalized dependency rows to match declared sources exactly, and relax only the legacy-v4-raw equality check so matching raw/canonical historical provenance may remain incomplete. Canonical-json-v1 and every other invariant remain strict.

## Legacy-v4-raw Compatibility Correction Result

The repository-only correction is complete. The separately authorized local corrective promotion also completed exactly once.

- Modified only `src-tauri/src/schema_v5_pattern_write.rs`; the verifier and its focused tests are colocated in that file.
- Historical raw and canonical legacy provenance bytes remain untouched. For `legacy-v4-raw`, their `sourceArtifactIds` may remain incomplete only when the raw and canonical arrays agree exactly.
- Every non-source provenance field still must agree exactly between raw and canonical provenance.
- Declared Evidence and Reflection sources still must match the normalized dependency rows exactly.
- `canonical-json-v1` validation remains strict and unchanged.
- Schema-v5 DDL, schema version, receipts, migration semantics, backup behavior, and real-profile data were not changed.
- Added exact positive coverage for matching incomplete historical provenance and neighboring fail-closed coverage for raw/canonical source drift, non-source provenance drift, and normalized dependency drift.
- Focused verification passed: all three new tests passed individually, and the complete pattern-write module passed `25/25` tests.
- Initial focused and full canonical verification passed before promotion.
- The clean index and exact one-file code diff were rechecked immediately before staging.
- Only `src-tauri/src/schema_v5_pattern_write.rs` was staged and committed as `4746aed9e7cc7a6a01ddc3a39eb0db2bd0fde3df` with message `fix(storage): preserve legacy pattern provenance`.
- The commit contains exactly one file with `201` insertions and `21` deletions. Workflow artifacts remained unstaged and uncommitted.
- Post-commit canonical `scripts/verify.ps1` passed with exit code `0`, including workflow checks, `373` frontend tests, the `225`-test primary Rust suite, backup/contract suites, legacy schema-v4 refusal tests, ordinary and Founder schema-v5 activation tests, Founder runtime tests, formatting, UTF-8, secret-file, link, and Constitution checks.
- No package was created; no installer was invoked; Life OS was not launched; the real profile was not accessed or mutated; Step 6 was not retried; and no push, PR, deployment, distribution, release, Phase 4, or Android action occurred.

## Human Decision Required

The correction now has a stable local commit identity, and the separately authorized package build completed exactly once.

## Corrective Package Result

- Source commit: `4746aed9e7cc7a6a01ddc3a39eb0db2bd0fde3df`.
- Canonical builder invocation count: `1`; suffix: `r3-legacy-pattern-provenance`; exit code: `0`.
- Ignored output directory: `.artifacts/desktop-schema-v5-ordinary-review-r1/4746aed9e7cc-r3-legacy-pattern-provenance/`.
- Installer: `Life-OS-Ordinary-Schema-v5-Review-R1-Review-r3-legacy-pattern-provenance_0.3.0_x86_64-pc-windows-msvc-setup.exe`.
- Installer size: `5808395`; SHA-256: `98a1dcedf085d2cd0219df74d38f69e74261195b7c91e31d447f0d1e1ded3794`.
- Manifest size: `379`; SHA-256: `e6abfd752f0bdbdff61bafc655b9a7f5e34fe3a5f9a90129f90fc6d26e7c4699`.
- Release binary size: `26934784`; SHA-256: `7362acb9c49bcf668204c99d2748705bc0a41b186da1cb121d903e2a2d8be21c`.
- The output directory contains exactly the installer and manifest. The ordinary binary contract and strict six-field manifest both passed an independent post-build verification.
- No installation or application launch occurred. The real profile was not accessed or mutated. Step 6 was not retried. No commit, push, PR, deployment, distribution, release, Phase 4, or Android action occurred during packaging.

A separate installation gate is required before replacing the currently installed R2B executable. Installation must remain distinct from application launch and Step 6 retry, and must prove the disclosed real-profile technical files remain byte-identical without reading personal content.

## Corrective Installation Attempt Result

The separately authorized silent installation was invoked exactly once and then stopped fail closed on the first post-install identity deviation.

- Preflight Life OS process count was `0`.
- The exact installer size/SHA-256 and strict six-field manifest were verified immediately before mutation.
- Installer invocation count: `1`; argument: `/S`; exit code: `0`.
- No `life-os` process was observed during installation; final process count is `0`.
- The installed executable has the expected size `26934784`, and the ordinary binary contract passes, but its observed SHA-256 is `b219e2a71ec2b1281783a6b83a07de83f79f751ee03754a0af1897ad6d9e96f5`, not the authorized expected SHA-256 `7362acb9c49bcf668204c99d2748705bc0a41b186da1cb121d903e2a2d8be21c` of the post-build release binary.
- No retry, uninstall, downgrade, repair, application launch, Step 6 retry, migration, recovery, restore, deletion, or publication action occurred.
- Current content-free inspection confirms zero live WAL/SHM/rollback-journal files and all nine available cryptographic anchors match exactly: the live database matches the immutable Step 6 diagnosis digest, the new recovery receipt matches its recorded digest, and all seven preserved-file facts embedded in that unchanged content-free receipt match current bytes.
- The tenth disclosed file, the current operation `state.json`, is size `1091`, SHA-256 `5c76c516e820941270001ff4f3cfea6170ada3eab0a12245923f017c67603c6c`, with last-write time equal to the earlier recovery event. The installation verifier hashed it before invoking the installer, but the command stopped on the binary mismatch before emitting or retaining the in-memory before/after comparison; therefore exact pre/post byte equality for this one file is not claimed from that run.
- No personal Experience, Evidence, Reflection, or Pattern content was selected or printed.

The installed-binary digest mismatch may reflect a difference between the post-build release executable and the actual NSIS payload, but that is only a hypothesis. A separate immutable diagnosis is required before any launch or retry.

## Immutable Installed-Binary Diagnosis Result

The separately authorized immutable diagnosis completed without installation, extraction to disk, application launch, profile access, or other target mutation.

- Life OS process count before and after: `0`.
- The exact installer and strict manifest reverified successfully.
- 7-Zip identified the package as NSIS 3 Unicode with one `life-os.exe` payload. The payload was streamed directly to memory; no file was extracted.
- Post-build release binary, in-memory NSIS payload, and installed binary all have size `26934784`.
- SHA-256 values: post-build release `7362acb9c49bcf668204c99d2748705bc0a41b186da1cb121d903e2a2d8be21c`; NSIS payload `b219e2a71ec2b1281783a6b83a07de83f79f751ee03754a0af1897ad6d9e96f5`; installed binary `b219e2a71ec2b1281783a6b83a07de83f79f751ee03754a0af1897ad6d9e96f5`.
- The installed binary is byte-for-byte identical to the exact in-memory NSIS payload.
- Release versus payload differs in exactly `3` contiguous bytes at file offsets `21389488` through `21389490`, entirely inside `.rdata`; no PE metadata field differs and all other sections/bytes are identical.
- Marker classification is exact: the release binary contains one `__TAURI_BUNDLE_TYPE_VAR_UNK` marker at offset `21389464`; the NSIS payload and installed binary contain one `__TAURI_BUNDLE_TYPE_VAR_NSS` marker at the same offset. This is the deterministic Tauri NSIS bundle-type patch, not an unexplained installed-file mutation.
- Both binaries report product/version `Life OS Schema v5 Review` / `0.3.0`, are unsigned as expected, and the installed ordinary binary contract passes.
- All handles were closed. No personal or SQLite/profile content was accessed.

The prior install gate used the restored post-build release hash as the expected installed identity. That expectation was incorrect for the NSIS payload. The actual installed binary is the exact deterministic package payload and requires no reinstall or repair. The separately authorized application launch and Step 6 corrective retry then passed by exact Founder report.

## Review Status

Approved. Founder Manual Review Steps 1–16 are complete. Stop before any post-review workflow closeout, archive, Git promotion, publication, application close/restart, Phase 4, or Android action.
