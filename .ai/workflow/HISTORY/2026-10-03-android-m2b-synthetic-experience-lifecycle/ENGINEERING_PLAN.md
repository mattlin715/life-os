# Engineering Plan

Status: approved

- Sprint ID: 2026-10-03-android-m2b-synthetic-experience-lifecycle
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: aebef185346effda779a6a10850882a2dc19e701
- Working-tree digest reviewed: baseline unchanged outside workflow
- Created at: 2026-10-03
- Updated at: 2026-10-03

## Approved Product Boundary

PRODUCT_REVIEW approved_with_conditions. Copy all conditions: canonical same-content correction; immutable identity/predecessor/user authorship; revision/projection atomicity; approved logical deletion only; content-free metadata and no physical-erasure promise; optional disclosure; distinct synthetic sandbox; no artifact/AI/policy expansion; cancel no writes; uncertain identity reconciliation; exact Founder gate.

## Existing Implementation Understanding

android_m2a.rs owns serialized app-private publication/receipt readiness and direct runtime calls. schema_v5_experience_write.rs already supports direct fresh Update/Delete with expected revision, BEGIN IMMEDIATE, transaction reconciliation and approved consequences. Revision ID binds source ID, timestamp and content digest. No new DDL or mutable text overwrite is needed.

## Affected Modules

Add src/android-m2b, src-tauri/src/android_m2b.rs, scripts/android-m2b* and contracts/tests; additive typed direct lifecycle wrapper in schema_v5_runtime; parameterize only fixed Android storage profiles to reuse M2-A publication. Android config/activity/Gradle label, web entry/mode, verifier/package scripts, architecture/23, dev/14 and navigation. Preserve canonical icon assets, M0/M1/M2-A frontend/source guards and desktop routes.

## Proposed Design

Share the verified initializer under two fixed, identity-bound app-private profiles. M2-B commands expose create/list/get/current revision/update/delete only. Lifecycle requests carry a fixed timestamp and a SHA-256 identity bound to operation, source ID, expected revision, timestamp and body digest. Reconciliation queries exact immutable revision/predecessor/timestamp or deleted head; successful old update after later mutation returns no retained text. No additional content ledger/table or retention policy. Retry uses the same frozen descriptor; it never substitutes a newer expected revision. Reject any artifact/historical dependency in synthetic-only sandbox, preserving state. Invoke canonical direct writer unchanged, then verify/reread state before acknowledging. Debug-only bounded before/after-commit probes and writer failure injection support native uncertainty tests. UI saves explicit drafts, uses revision snapshots for edit/delete, clears caches after purge and preserves trilingual copy/locale/icon.

## Alternatives Considered

Reject duplicate SQL/deletion implementation (policy drift), schema changes/receipt tables (unnecessary new contract), raw-text request journal (privacy), new timestamp on retry (duplicate/destructive risk), desktop adapter activation (scope violation). Reuse fixed-profile Android initializer and exact canonical transaction writer.

## Data Lifecycle Impact

Update retains old content under existing policy, including same-content new revision. Delete purges source and scoped content and permitted tombstones through existing writer, preserves only existing source metadata/provenance. No new retention or cascade rule; unexpected dependency state refused.

## SQLite Or Migration Impact

No DDL/schema/hash changes; direct fresh schema 5, zero migration receipts; strict migrated runtime unchanged. No migration, recovery, repair or sidecar cleanup.

## Provenance Impact

Expose current revision snapshots, exact predecessor/user authorship and immutable request reconciliation. No Evidence confirmation or dependency rebinding.

## Historical Context Impact

None exposed; unexpected dependencies refused, not regenerated or transmitted.

## Consent Impact

Explicit Save and confirmed Delete; cancel/draft cause no write.

## Provider Transmission Impact

None. No providers/credentials/INTERNET permission.

## Import And Export Impact

None. No portable continuity or backup implementation.

## Test Strategy

Rust direct lifecycle and facade tests: stable revisions/CJK/multiline/same-text, stale/duplicates/two editors/races, rollback/uncertain reconciliation, dependency refusal, logical purge and no resurrection. React/adapter tests: cancel/no writes, pending/conflict outcomes, clearing editor/detail/list, dedupe and exact request retry, languages/preferences. Contracts: fixed identity/paths/mode, no forbidden capabilities, accepted assets, deterministic verification wiring. Native exact-owned Android36 x86_64 fixtures, UI and SQL metadata probes, APK permission/backup/ABI inspection, native force-stop/restart and fault classification. Preserve all failure fixtures; no unknown-state deletion.

## Repository Verification Strategy

Run powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1 after last source/docs change; Android contracts/build/inspect/native separately. Record source HEAD, exact non-workflow digest, APK and native hashes. Host, emulator, fault injections and untested physical behavior labelled separately.

## Manual UI Verification

Founder owns final consolidated diff and create/edit/restart/cancel-delete/delete/restart checklist. Not automatically accepted.

## Rollback Or Recovery Strategy

Preserve failure state and evidence. No reset/revert/repair; unknown dependency/pending state blocks. Only exact sprint-owned fixtures may be recreated after evidence preservation. No real profiles.

## Documentation Impact

Architecture 23 and runbook 14 with exact policy mappings/limitations; factual minimal M2-A review acceptance wording correction, no historical archive rewriting; index/roadmap navigation only as needed.

## ADR Impact

No new ADR or Accepted status changes. Existing ADR-0011/0012 direction only applied within new bounded synthetic authority.

## Risk Level

Medium: lifecycle deletion and uncertain outcomes; mitigated by canonical writer, exact request reconciliation, scope isolation, content-free errors and focused/native verification.

## Escalation Decision

No policy conflict identified; proceed to bounded implementation. Stop if exact policy/writer disagreement or essential new authority emerges. No Harness expansion.

## Founder status-presentation correction (review cycle 3)

ANDROID-M2B-FOUNDER-REVIEW-002 Option C explicitly authorizes UI only after prior exact manual7/7 PASS. Render unsaved indicators within nonempty new draft / genuinely changed edit context; suppress blank/opened-saved false warnings. Separate clearly scoped create/edit/delete result feedback from draft state, retain saving/uncertain/conflict/failure and retry controls. Preserve all handlers, requests, adapter, Rust writers, storage/retention/deletion, CSS/icons/config. Add exact trilingual presentation tests and owned native interaction assertions, rebuild/inspect, full native/canonical verification, then new Founder gate. Prior APK/digest/manual/profile/source preserved under .artifacts/android-m2b/copy-correction-002/pre-correction. No automatic timers hiding abnormal states or new lifecycle transitions. No staging/publication/reset/real data/production/later slice.
