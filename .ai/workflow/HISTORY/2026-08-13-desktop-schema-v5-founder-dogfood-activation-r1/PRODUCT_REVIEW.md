# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-13T12:00:00Z
- Updated at: 2026-08-13T12:00:00Z

## Mission Interpretation

Exercise the already accepted Phase 3C contracts through one isolated Windows Founder package, without turning private v5 evidence into ordinary-product authority. The candidate must make every database-changing step explicit, inspectable, recoverable only by user choice, and incapable of reaching `com.lifeos.app`.

## Problem Statement

Life OS has verified private migration and writer primitives, but no coherent packaged boundary proves that an isolated user can create or migrate a database, restart safely, continue the current product journey on v5, inspect the backup, or explicitly restore it. Infrastructure evidence alone is not dogfooding evidence.

## User Value

The Founder can evaluate the complete local-first cutover relationship: clear disclosure before change, a verified local backup, honest uncertainty, durable reconstruction after restart, continued daily-reflection use, and explicit control over retention, deletion, and restoration.

## Relevant Primary Definitions

- `docs/03_Principles.md`: evidence before conclusion, local-first control, visible uncertainty, and user agency.
- `docs/06_Memory.md`: memory remains user-owned, correctable, deletable, and provenance-bearing.
- `docs/Reflection.md`: reflection remains an invitation, not an authoritative conclusion.
- `docs/09_AI.md` and `docs/10_Privacy.md`: no new AI authority, provider transmission, or silent data movement.

## Relevant ADRs

- ADR-0007 preserves reviewed artifact provenance.
- ADR-0009 preserves exact historical selection/consent/transmission/actual-use separation and cascades.
- ADR-0011 governs append-only revisions, lifecycle facts, tombstones, and portable provenance.

## Current Implementation Context

- Implemented/promoted: ordinary schema-v4 runtime, isolated Founder package R1, exact v4-to-v5 private migration/restart evidence, filesystem safety primitives, and private writer parity for currently reachable actions.
- Founder-authorized in this sprint only: isolated Founder-package activation candidate and disposable evidence.
- Not authorized: ordinary profile v5, real-data migration, deployment/release, Phase 4, provider or consent changes, Android.

## In Scope

Compile-time Founder-only activation; startup classification; fresh exact-v5 initialization; explicit v4 migration with verified backup; restart/recovery classification; currently reachable typed v5 reads/writes; backup retention/delete-now and explicit restore; three-language disclosure; deterministic tests and packaged manual-review artifacts.

## Out Of Scope

Ordinary profile access, real personal data, new product actions, export v2, Phase 4, provider/ContextPacket/consent changes, Android, general rollout, distribution, deployment, release, or Git promotion.

## Product Constraints

Opening the app or disclosure is never migration authority. Cancel is a no-op. Exact valid v5 is authoritative only after durable verification. Every ambiguous state fails closed. The old v4 app must refuse v5 without writing.

## Evidence And Provenance Constraints

Reuse exact accepted DDL, migration receipt, source/target manifests, append-only writers, and ADR-0009 provenance. Do not invent history or rewrite legacy baseline bytes. Logs/manifests remain content-free.

## Historical Context Constraints

Existing retrieval and session-only selection remain unchanged. Context Recovery remains historically ineligible. Historical Question creation/deletion and exact dependency cascades must retain ADR-0009 semantics.

## Consent Constraints

No consent-policy change. Migration authorization is a separate one-operation local action and must not imply provider consent. Provider retention cannot be undone by local restore and must be disclosed.

## AI-Role Constraints

No new AI call, prompt, evaluator, or inference. The candidate preserves mirrors-not-oracles behavior and Phase 3 output fences.

## Privacy Constraints

Only the isolated Founder profile may be opened. Automated evidence uses disposable fixtures. Package manifests and logs contain no content, credentials, or personal paths. The ordinary profile must remain byte-untouched and uninspected by candidate code.

## User-Agency Constraints

Migration, backup deletion, and restore each require distinct explicit actions. No automatic retry, replay, repair, rollback, restore, cleanup, candidate selection, or silent alternate database.

## Acceptance Criteria

1. Compile/runtime identity prevents ordinary app selection of v5 commands.
2. Missing Founder database becomes verified exact v5; exact v4 remains unchanged until explicit authorization; cancel is a no-op.
3. Exact v4 migration creates and verifies one owned backup, writes user_version last, and reconstructs durable state after restart.
4. Older/newer/malformed, sidecar/activity/path/permission/identity/disk/ambiguous evidence fails closed without mutation.
5. Current Experience, Evidence, Reflection, Pattern, Context Recovery, ADR-0009, deletion, import, and cleanup actions route through typed v5 boundaries and reject stale state.
6. Backup retention/delete/restore controls are exact-owned, explicit, truthful, and never decrement schema.
7. English, Traditional Chinese, and Japanese disclosures are equivalent and accessible at narrow widths and keyboard focus.
8. Ordinary schema-v4 build/profile behavior, provider behavior, ContextPacket digests, consent policy, Phase 4 fences, and Experience-only export remain unchanged.
9. Focused tests, Clippy, canonical verification, Product/Theory review, packaged disposable evidence, and Founder manual review readiness complete before any promotion request.

## Risks

The slice touches migration, filesystem replacement, durable recovery, and all storage mutations. Highest risks are identity-boundary escape, partial multi-artifact writes, sidecar/connection ambiguity, false durability claims on Windows, v5/v4 projection drift, and accidental use of ordinary or personal data.

## Open Questions

none; the Founder authorization and accepted repository documents govern the bounded candidate. Any newly discovered moral, consent, destructive-data, or ordinary-profile requirement must stop at human decision.

## Human Decision Required

false; the Founder explicitly authorized this isolated candidate while withholding ordinary-profile and rollout authority.

## Recommendation

Proceed under the exact allowlist and fail-closed threat model. Prefer reusing promoted primitives over parallel systems. Stop after packaging at Founder diff/manual review.

## Review Status

approved_with_conditions
