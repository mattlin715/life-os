# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-27-desktop-schema-v5-prepared-state-recovery-r2
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899
- Working-tree digest reviewed: a84c6cf19ef306fda74e98e67c2d42c362163011e3ce8c6b7a5bc5b0c85f5d1f
- Created at: 2026-08-27T00:55:00+09:00
- Updated at: 2026-08-27T00:55:00+09:00

## Mission Interpretation

Add one product-visible escape hatch for the exact fail-closed R1 pre-write state, not a repair facility. The surface first explains and re-proves content-free facts. Only a separate explicit recovery action may disposition the exact transient evidence. Recovery returns the unchanged schema-v4 database to the existing migration-required state; it does not migrate, back up, restore, or reuse authorization.

### Founder-authorized A-R3 addendum

Manual Phase A later proved that a historical schema-v4 binary can visibly
refuse a valid migrated schema-v5 database while creating a zero-byte WAL and
matching zero-frame SHM. The live database, valid `v5_ready` operation and
verified backup remained byte-identical, and the earlier prepared-state
receipt/quarantine remained valid. Founder selection `Option A-R3` authorizes
one second product-visible escape hatch for only that exact disposable state.

This addendum preserves the original product decision: no automatic cleanup,
ignore-on-open, checkpoint, retry, repair, restore or migration. Opening is
read-only. The only mutation control may quarantine the exact WAL/SHM pair and
write one strict content-free receipt after claim-bound revalidation. The live
v5 database, current operation/staging/backup, prior receipt/quarantine and
personal records remain unchanged. Success closes/restarts into the existing
v5 runtime, not migration or restore. English, Traditional Chinese and
Japanese disclosure must distinguish files moved from protected files. The
real Founder profile, Phase B/C, promotion and every later platform/release
phase remain out of scope.

## Problem Statement

R1 correctly preserved uncertainty after its verifier created empty WAL/SHM sidecars between operation preparation and backup preflight. The live database remained byte-unchanged schema v4, while one exact-owned operation stayed `prepared`, staging stayed zero bytes, and no backup or migration receipt existed. The promoted app now refuses this evidence, but offers no narrowly governed recovery action. General cleanup would be unsafe; silent cleanup would break user agency and evidence truth.

## User Value

The Founder can understand why the local profile is blocked, preserve the evidence, or explicitly remove only what is proven transient without exposing personal content or risking database mutation. The separation between recovery and migration makes authority legible and reversible up to each explicit action.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human Before AI, Privacy Before Profit, local-first ownership, and truthful documentation require explicit authority and no silent recovery.
- `docs/02_Philosophy.md` and `docs/03_Principles.md`: Context Before Insight and Evidence before Conclusion require complete structural context before classifying transient evidence.
- `docs/06_Memory.md` and `docs/Reflection.md`: local memory and user-authored meaning must remain intact; recovery cannot reinterpret or inspect journal content.
- `docs/09_AI.md`: AI remains a mirror; no AI inference participates in recovery.
- `docs/10_Privacy.md`: only the minimum content-free file and lifecycle facts may be disclosed or retained.

## Relevant ADRs

- `ADR-0004`: local-first data ownership supports an on-device, explicit recovery boundary.
- `ADR-0007`: reviewed artifacts and provenance remain untouched because database bytes are never opened writable by recovery.
- `ADR-0009`: historical selection, consent, packet, transmission, and actual-use provenance are unaffected.
- `ADR-0011`: append-only revision/lifecycle authority and exact dependencies remain unchanged; the recovery receipt is operational content-free evidence, not a user artifact revision.

## Current Implementation Context

- Promoted and canonically verified: ordinary default `desktop-schema-v5`, exact-v4 explicit migration, verified backup, fixed v5 DDL, typed v5 routing, conservative restart classification, persistent-WAL immutable reads, and explicit writer close.
- Founder-observed but not resolved: one real-profile action stopped before backup/migration with schema v4 byte-identical to pre-attempt evidence, one exact-owned `prepared` operation, zero-byte staging, no backup, WAL/SHM present, and no rollback journal.
- Repository-recorded root cause: a former non-immutable verifier created the sidecars; the corrected verifier/runtime was proven only on disposable fixtures and promoted in R1.
- Not implemented: exact prepared-state recovery, recovery receipt, recovery UI, or any second real-profile migration.
- The existing `Prepared` state record does not itself contain a live digest because the R1 stop occurred before backup verification. Therefore the reviewed pre-attempt digest remains an external Founder evidence item: automation must accept an exact expected digest and prove equality/TOCTOU, while final real-profile eligibility remains a Founder manual gate rather than an autonomous claim.

## In Scope

- Exact ordinary-identity prepared-state classifier and mutation using path-injected synthetic/disposable roots.
- Content-free file metadata, digest, SQLite-header, WAL-frame, and operation-state evidence only; no SQL row reads.
- Explicit-open EN/zh-TW/ja recovery surface with Preserve and close, Show technical details, Prepare recovery, and Cancel controls.
- Exact process-local lock, quiescence proof, immediate revalidation, content-free durable receipt, bounded disposition of only exact supported transient evidence, and database byte-preservation verification.
- Recovery/migration authorization separation and restart to existing migration-required disclosure.
- Thirty-case automated matrix, documentation, ignored unsigned package, and Founder manual preparation.

## Out Of Scope

- Accessing or mutating the real profile during autonomous work; deriving a real baseline digest from personal data; selecting a cleanup candidate automatically; non-empty or unproven WAL handling; SQLite checkpoint/repair; arbitrary SQL; general cleanup; backup/restore changes; schema/DDL changes; provider or ContextPacket behavior; consent policy; Phase 4; Android; export/import expansion; telemetry; cloud features; Git promotion or release operations.

## Product Constraints

- Recovery is available only for ordinary `com.lifeos.app`, exact schema v4, one exact-owned `prepared` operation, exact known digest, zero staging, no backup/receipt/v5 mutation, proven-empty WAL plus matching SHM, no journal/activity/path ambiguity.
- Opening and cancelling are byte-identical and grant no authority.
- Any mismatch exposes refusal/preservation only, never a cleanup control.
- Recovery success may reveal the existing migration disclosure but cannot automatically open or execute it.
- A new migration remains one separately explicit action with fresh current evidence and new verified backup.

## Evidence And Provenance Constraints

The prepared-v4 classifier may hash whole closed files and inspect fixed file/header structures but does not open SQL. After exact empty-sidecar proof, A-R3 may query only immutable technical receipt/contract/schema/integrity invariants and must not retrieve, display, log or interpret personal content fields. Receipts contain only operation id, approved classification, relative filenames, sizes, SHA-256 digests, schema/app versions, timestamp and bounded protected-file facts. They contain no user root, row content, credentials, secrets, or provider data. Database and all user artifacts remain byte-identical during recovery.

## Historical Context Constraints

No historical retrieval, selection, relevance, consent, transmission, packet assembly, actual-use artifact, or Phase 4 behavior changes. Tests must use synthetic records only.

## Consent Constraints

Recovery authorization is per current preflight and is consumed only by the bounded recovery call. It cannot become migration consent. Migration keeps the existing separate explicit action and revalidation.

## AI-Role Constraints

No model, provider, prompt, inference, diagnosis, profile, or identity statement participates. The UI reports filesystem facts and uncertainty only.

## Privacy Constraints

No personal content field is retrieved, parsed, returned or logged. Absolute user-root paths are neither returned to the renderer nor stored in the receipt. Automation is structurally prevented from targeting the real app-data root and uses disposable path injection.

## User-Agency Constraints

The Founder can preserve and close, inspect technical details, cancel without change, or explicitly prepare recovery. Recovery and migration remain separate buttons, surfaces, state transitions, and invocations. Failure never retries.

## Acceptance Criteria

1. Only the exact eligible state exposes the explicit recovery controls; every listed mismatch fails closed without cleanup control.
2. Preflight and technical detail disclose all required content-free facts in EN/zh-TW/ja and disclose that rows are not read, cancel changes nothing, recovery is not migration, and later migration requires new consent.
3. Cancel/preserve paths are byte-identical.
4. Recovery revalidates under a process-local lock, refuses TOCTOU, writes one durable content-free receipt, and disposes only exact prepared/staging/empty-WAL/matching-SHM evidence.
5. Database bytes, identity, schema v4, and expected digest remain exact after success; no writable SQLite open or checkpoint occurs.
6. Restart reconstructs migration-required and creates no migration operation or backup.
7. A later explicit migration continues to use the existing R1 verified-backup/migration contract and no recovered consent.
8. The 30 required synthetic/disposable cases, focused tests, Clippy warnings-denied, and canonical verification pass.
9. Architecture 19 plus architecture 13/15/18, index, and runbook describe implemented truth and remaining gates.
10. One ignored unsigned package is built and verified but not installed or launched automatically; real profile and Android remain untouched.
11. A-R3 eligibility requires exact schema v5, one internally consistent committed migration/runtime contract, one exact `v5_ready` operation, unchanged verified v4 backup, valid earlier recovery receipt/quarantine, zero-byte WAL, matching zero-frame SHM, absent journal/conflict, and a claim over all moved/protected facts.
12. A-R3 success quarantines only WAL/SHM, writes one technical-only receipt, preserves and re-verifies the database/operation/backup/prior recovery evidence, requires close/restart, and never exposes migration or restore action.

## Risks

- Destructive misclassification could erase evidence or user state; mitigated by exact state schema, digest, file identity, empty-WAL structural proof, path/link checks, lock, quiescence, and immediate revalidation.
- The old `Prepared` record lacks a stored baseline digest; the build must not infer historical equality. Disposable tests provide a supplied expected digest, and real action must remain blocked until Founder manual evidence supplies and confirms the recorded digest.
- Crash during multi-file disposition can leave partial cleanup. The receipt and ordered, idempotence-refusing postcondition must report honest recovery-required state; no automatic continuation or retry is allowed.
- Windows process detection cannot prove arbitrary external SQLite clients are absent alone; exclusive file handles plus application process checks and closed-file revalidation are required, with unknown treated as refusal.

## Open Questions

none for repository/disposable implementation. Real-profile digest/evidence eligibility, disposition, and later migration remain the specified separate Founder manual gates.

## Human Decision Required

false for this implementation phase. Phase B and Phase C each require the exact separate Founder questions after disposable Phase A.

## Recommendation

Proceed with a dedicated exact-recovery module and renderer contract, reusing existing ownership/path primitives where semantically exact. Keep the supplied expected digest as a required recovery claim rather than embedding a personal database fingerprint in tracked source. Stop at Founder diff/manual review.

## Review Status

approved_with_conditions

## R2B product-review addendum — 2026-09-02

The correction remains within the approved local-first/user-agency boundary
only if it is an exact allowlist rather than semantic SQL normalization. Three
fixed schema-v4 representations and their three fixed derived schema-v5
manifests may be recognized; every other representation must fail before
migration. The recovery surface may appear only for the exact blocked
phase/outcome, committed receipt, historical derived manifest, disabled or
already-enabled lifecycle boundary, verified backup, zero staging, valid prior
prepared recovery evidence, absent sidecars and unchanged claim.

The explicit action may enable lifecycle writes, write one content-free
technical receipt and finalize only the bound operation state. It must expose
no migration, restore or backup-deletion action and must not perform DDL,
retry, restore, repair, checkpoint, schema decrement, backup deletion or
personal-content transformation. Cancel and Preserve and close remain
write-free. Product review remains `approved_with_conditions`; disposable
manual review and every real-profile action remain separate Founder gates.

## R2B repository product result — 2026-09-02

The exact allowlist, consent separation, no-content disclosure, absent
migration/restore/delete controls and same-transaction fail-closed activation
recheck are implemented and automation-verified. The single ignored package is
evidence preparation only, not product acceptance or distribution. Product
review remains `approved_with_conditions`: the next condition is a separately
authorized disposable Founder manual review.

## R2B disposable product acceptance — 2026-09-14

The Founder completed and accepted the disposable BF1-BF8 manual matrix. The
review directly covered the three-language disclosure, technical-detail
containment and focus, write-free Cancel path, exact completion action,
post-recovery restart, retained managed backup and one typed synthetic write.
The result preserves the product boundary: evidence is shown rather than
interpreted, recovery remains explicit, no personal content was reviewed, and
no provider transmission occurred.

The disposable manual condition is therefore satisfied. Product status remains
`approved_with_conditions` only because repository promotion and any real-profile
action are separate Founder decisions; neither is authorized here.
