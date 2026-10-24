# Engineering Plan

Status: approved

- Sprint ID: 2026-10-18-daily-reflection-response-correction-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 5646a718ac0fe6251d85e7e2da70f3b2e11faae5
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-10-18
- Updated at: 2026-10-18

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

PRODUCT_REVIEW approved_with_conditions. Preserve draft on failure, strict current eligibility, no writer policy changes, same frozen request only; Founder review remains pending.

## Existing Implementation Understanding

ReflectionJourney imports domain prompt helper, shared packet/gate/placeholder, store request builder, journeyCopy. AndroidM2CApp mounts it keyed by source revision with lock/reopen callbacks. IPC is m2c_mutate_artifact -> android_m2c.rs with_m2c_storage -> android_reflection::mutate -> canonical reflection_write::execute_direct_fresh -> transaction/projection verification -> adapter reconciliation -> frontend get/artifacts. No new IPC required.

## Affected Modules

Write-set: ReflectionJourney.tsx and .test.tsx, androidM2CStore.ts, journeyCopy.ts, schema_v5_android_reflection.rs (adapter + tests), scripts/android-m2c-contract.node-test.mjs (precise existing safeguard extension), architecture24, runbook15, current workflow artifacts. No canonical writer source change planned.

## Proposed Design

Add distinct correctResponse operation in frozen identity tuple. Shape requires one exact Evidence ref and expected current artifact/source revisions. Route to CorrectResponse, not SaveResponse or Evidence CorrectPending. UI response editor uses stored row, retains original saved text separately, normalizes only at explicit save, rejects blank/unchanged, fences hashing/IPC, preserves frozen request and draft during failures. Read current state without dropping uncertain request/draft. Validate correction reread matches saved text/authorship/current eligible target before success. Block unrelated operations during response editing.

## Alternatives Considered

Reusing answer or correct is rejected: first means initial save, second means Evidence. Direct SQL/second store rejected. Generic mock platform/history editor rejected. Existing request/reconciliation and canonical writer suffice.

## Data Lifecycle Impact

Only canonical append-only response revisions, original prompt lineage and existing dependency invalidation. Cancel creates nothing. No retention/deletion rule change.

## SQLite Or Migration Impact

No schema/migration/SQL policy edits. Reuse canonical writer. Isolated tempfile fixtures only. Tools copied as independent bytes; no original checkout cache/output writes.

## Provenance Impact

Prompt provenance retained; new response revision user-authored, no candidate reconfirmation requirement.

## Historical Context Impact

None: reflectionPacket excludes drafts/responses/history as before.

## Consent Impact

No new consent semantics; explicit save is user authorship.

## Provider Transmission Impact

None: no remote provider, credential or product network use.

## Import And Export Impact

None: unsupported mobile operations remain unsupported.

## Test Strategy

Focused component interaction harness without new dependency; frozen identity cross-language contract; adapter SQLite first/repeated correction, unchanged/blank/stale/source/Evidence/unsupported refusal, exact revision/provenance/lineage/dependency state, rollback, duplicate/uncertain reconciliation, reopen. Run existing affected suites and canonical verifier, retain actual exits and failed outputs.

## Repository Verification Strategy

Inspect verifier and nested scripts first. Version here invokes host node/static contracts, Vitest, build, cargo tests/checks and integrity checks only. No Android runtime command is invoked by static contract tests. Independent tools/caches and output paths; no upgrades. Commands capture actual child exit and raw output. Final canonical command: powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1.

## Manual UI Verification

Founder owns final reopen -> edit -> cancel -> edit/save -> reload. No safe existing browser persistence preview observed; deliver diff/test materials rather than fabricate one.

## Rollback Or Recovery Strategy

Leave unstaged diff for review, never reset original. Preserve failures. At most five evidence-based host correction rounds; no blind retries, no weakened assertions. Stop only exactly owned new host children on timeout; no name-based kills.

## Documentation Impact

Append factual independent R1 sections to architecture24/runbook15, preserving historical acceptance and STOPs; update last_updated only. Index navigation/Book Zero remain unchanged.

## ADR Impact

No new ADR or status change: implementation of existing authorized canonical correction, no new policy or architecture.

## Risk Level

Medium: saved-answer lineage and uncertain outcomes require persistence and UI proofs; bounded by existing canonical writer.

## Escalation Decision

No implementation escalation now; final stop at human_decision_required for Founder consolidated diff/usability review. Native evidence stays unproven.
