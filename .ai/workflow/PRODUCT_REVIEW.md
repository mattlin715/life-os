# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-09-29-android-m2a-direct-fresh-v5
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 51dff4998ca8696aeaf1527f058d785303fc0cd4
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-29T09:12:00Z
- Updated at: 2026-09-29T09:12:00Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Implement only the direct-bootstrap portion of the Accepted Android direction. The result is synthetic, temporary, reviewable evidence—not a production Android store and not completion of M2.

## Problem Statement

M1 creates an empty v4 database and migrates it to v5. That path tests canonical migration machinery but falsely resembles legacy history when used as a fresh-install origin. A new Android installation needs one transaction that creates the canonical final objects directly, records truthful fresh-origin evidence, verifies the result, and publishes it without overwriting or repairing any existing state.

## User Value

This removes a misleading initialization model before real Android storage is considered. It gives the Founder concrete evidence that exact user-authored synthetic text can survive reopen/restart on a genuinely direct v5 database while preserving privacy, local ownership, and uncertainty boundaries.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: human authority, local ownership, privacy, and “We Build Mirrors, Not Oracles.”
- `docs/00_Index.md` routes Evidence to `docs/03_Principles.md` and Memory to `docs/06_Memory.md`: storage preserves user-owned evidence and continuity; it does not create truth.
- `docs/10_Privacy.md`: local-first handling and explicit boundaries remain higher authority than convenience.

## Relevant ADRs

- `docs/adr/ADR-0004-local-first-mvp.md`: local-first product direction.
- `docs/adr/ADR-0006-mvp-tech-stack.md`: Tauri + React + SQLite.
- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: durable artifacts require provenance; this slice creates no AI artifacts.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`: canonical v5 lifecycle remains unchanged.
- `docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md`: Accepted direct exact-v5 Android initialization direction; production activation remains explicitly withheld.

## Current Implementation Context

- Founder-approved direction: ADR-0012 is Accepted and chooses direct exact-v5 initialization for new Android installs.
- Implemented and Founder-accepted evidence: M1 uses temporary `com.lifeos.review.m1`, exact app-private persistence, trilingual UI, persistent locale preference, accepted launcher icon, no network permission, and excluded backup/transfer.
- Implemented but intentionally scaffolded: `src-tauri/src/android_m1.rs` creates `EMPTY_V4_BASE_SCHEMA`, invokes `migrate_disposable_v4`, activates lifecycle writes, and publishes a pending database.
- Canonical final DDL: `src-tauri/schema/schema_v5.sql` defines v5 authority, lifecycle, constraints, indexes, triggers, and guarded compatibility projections, but assumes the compatibility projection tables already exist.
- Current verifiers and experience writers require one historical 4-to-5 migration receipt. That strict migration path must remain unchanged; M2-A needs a separate direct-origin verifier rather than a weakened migration verifier or fabricated receipt.
- Baseline canonical verification passed on clean `develop` at the required commit before branch creation.

## In Scope

- Direct creation of all required final v5 objects, including compatibility projection tables, in one fresh transaction with no intermediate v4 state.
- Truthful app-private fresh-initialization receipt and exact direct-origin verification.
- Distinct pending, verified, published, ready, and write-eligible states; bounded publication and refusal of partial/ambiguous states.
- M1-equivalent create/list/get synthetic flow, locale behavior, icon assets, offline/backup exclusions, focused tests, APK inspection, and dedicated emulator evidence under `com.lifeos.review.m2a`.
- Narrow M2-A architecture/runbook and factual index/roadmap synchronization.

## Out Of Scope

Production identity `com.lifeos.app`; real data; complete `LocalEvidenceStore`; update/delete; migration/recovery/repair; import/export; provider, network, BYOK, sync; automatic backup or device transfer; production signing/distribution/release; physical devices; broad API/ABI/OEM readiness; other M2 slices or Phase 4.

## Product Constraints

Preserve the accepted simple trilingual Experience flow and copy. Do not add a database dashboard, inference, Evidence extraction, Reflection, Pattern, identity labels, advice, or unrelated UX. The technical details may identify direct initialization and temporary package truthfully.

## Evidence And Provenance Constraints

The initialization receipt must describe a direct fresh-v5 origin and must never claim a 4-to-5 migration. It may contain only content-free technical metadata and digests. User-authored synthetic Experience text remains distinct from initialization evidence.

## Historical Context Constraints

No historical context is assembled, accessed, imported, or transmitted. Canonical compatibility tables remain schema objects only and are empty at initialization.

## Consent Constraints

No provider or historical transmission occurs, so no consent event is created. Existing ADR-0009 consent behavior and desktop paths must remain unchanged.

## AI-Role Constraints

No AI operation exists in this slice. The UI and bundle must not expose provider, credential, inference, diagnosis, Pattern, identity-finalization, or oracle behavior.

## Privacy Constraints

Use only a package-owned disposable sandbox and synthetic fixtures. Keep `allowBackup=false`, transfer exclusions, cleartext disabled, and no INTERNET permission. Never inspect a desktop database, real profile, physical phone, or M0/M1 app data.

## User-Agency Constraints

The Founder remains responsible for manual UI acceptance. Automated tests may prove bounded mechanics but cannot mark experiential acceptance passed or authorize production use.

## Acceptance Criteria

1. Temporary package is exactly `com.lifeos.review.m2a`; M0/M1 packages are not uninstalled, cleared, or replaced.
2. Fresh bootstrap does not reference or invoke `EMPTY_V4_BASE_SCHEMA`, `migrate_disposable_v4`, or any invented v4 receipt.
3. Direct bootstrap creates the exact canonical final v5 tables, columns, constraints, indexes, triggers, compatibility projections, `user_version=5`, and enabled lifecycle contract in one transaction.
4. A truthful content-free direct-initialization receipt is verified before readiness; migration verifiers and desktop behavior remain strict and unchanged in outcome.
5. Existing exact direct-v5 reopens without initialization; malformed, newer, pending, sidecar, partial-publication, and ambiguous states are preserved and refused.
6. Creation, verification, publication, readiness, and first write acknowledgement are distinct; no existing live database is overwritten.
7. Focused tests cover DDL/commit/publication fault boundaries, concurrency/TOCTOU, duplicate save, uncertain commit, exact CJK text, locale restoration, backup/network isolation, and desktop regression isolation.
8. Android 36 x86_64 dedicated AVD evidence passes; any feasible already-installed second configuration is reported separately; physical-device and untested matrices remain explicit gaps.
9. Canonical `scripts/verify.ps1`, build, inspect, and native checks pass after the final change; final digest and APK hash are recorded.
10. Documentation is synchronized, ADR-0012 remains Accepted without authority change, all changes remain unstaged, and workflow stops at Founder diff/manual UI review.

## Risks

- A direct-origin exception could accidentally weaken desktop migration verification. Mitigation: separate origin-specific verifier/entry point and regression tests.
- Publishing a database and separate receipt cannot be a single filesystem rename. Mitigation: explicit two-object state machine; any partial state blocks and preserves evidence.
- SQLite main-file publication may ignore WAL/SHM/journal state. Mitigation: transactional close, explicit sidecar checks, exact tests, and no durability claim beyond observed evidence.
- Generated Android source and scripts could overwrite M1 evidence. Mitigation: new identity/package/AVD/artifact roots and contract checks; retain M1 archive and accepted hashes.
- Emulator success could be overgeneralized. Mitigation: report exact API/ABI/device/process coverage and unsupported physical power loss separately.

## Open Questions

None requiring product authority. The narrow technical choice of an app-private content-free direct-initialization receipt is reversible and within Accepted ADR-0012; if exact canonical parity proves impossible without changing Accepted schema policy, implementation must stop and produce a decision package.

## Human Decision Required

false; decision IDs: none. Founder manual diff/UI review remains the terminal gate, not an implementation blocker.

## Recommendation

Proceed with an origin-separated direct initializer and verifier. Keep migration receipts exclusively historical. Prefer an external app-private content-free technical receipt plus fail-closed partial-publication state over altering the Accepted canonical schema merely to store Android-origin metadata.

## Review Status

approved_with_conditions
