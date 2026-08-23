# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-23-ordinary-v5-real-profile-phase-b-blocked-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest reviewed: d527ba9cce3541f58bcb524201b7b2aea8d8d79fe37c90cdb9ca35ccaa7b5dd5
- Created at: 2026-08-22T22:48:16.501Z
- Updated at: 2026-08-22T22:48:16.501Z

## Mission Interpretation

Record the actual ordinary-profile Phase B outcome as a safe fail-closed pre-backup stop, not as a completed migration, and preserve the real profile plus exact-owned operation evidence until the Founder separately chooses whether a bounded code correction sprint should begin.

## Problem Statement

The one authorized real-profile migration action did not reach backup creation or schema-v5 commit. The main database remains exact pre-state schema v4, but the attempted verifier opened the persistent-WAL database through a non-immutable read-only SQLite connection. That read created `-wal` and `-shm`, causing the final sidecar check to stop the operation in durable `prepared` state. The current sprint has no authority to retry, repair, checkpoint, clean up, restore, or dispose of that real evidence.

## User Value

Failing closed protected the Founder's actual local history from an unverified migration. Truthful classification and preserved evidence protect trust and allow a later correction to be validated on disposable persistent-WAL fixtures before any new real-profile decision.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human Before AI, Privacy Before Profit, documentation truth, and evidence-based decisions require the blocked state to remain visible rather than being converted into success.
- `docs/03_Principles.md`: trust, evidence, and user agency require explicit uncertainty and no hidden recovery action.
- `docs/06_Memory.md`: locally retained reflection history remains user-owned evidence with provenance and must not be casually rewritten.
- `docs/10_Privacy.md`: personal local data belongs to the user; transparency, continuing consent, and user control govern any further database action.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: persisted reviewed artifacts and provenance must remain intact.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`: no historical provider-transmission or consent semantics change is involved or authorized.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`: append-only lifecycle/provenance authority must not be weakened by migration recovery shortcuts.

## Current Implementation Context

- Implemented and canonically verified before Phase B: ordinary schema-v5-capable build, explicit v4 migration disclosure, verified backup/migration/restore core, typed v5 routing, disposable Manual Phase A, and the Reflection double-submit correction.
- Founder-authorized for this real profile: one exact reviewed launch and one explicit migration action, with no retry on an ambiguous or blocked state.
- Actually observed in Phase B: exact reviewed executable; required three-language disclosure; one explicit action; generic stop disclosure; normal shutdown; main database equal to the recorded pre-action bytes; schema version 4; one `prepared` exact-owned operation; zero-byte staging file; no backup; WAL/SHM present; no rollback journal.
- Repository diagnosis: `ExactV4CandidateVerifier::inspect` in `src-tauri/src/schema_v5_migration.rs` calls `connect(path, true)` without `immutable(true)`. `create_owned_verified_backup` then detects the newly created sidecars at its final pre-VACUUM guard and refuses backup creation.
- Independent disposable reproduction: a persistent-WAL schema-v4 SQLite database with no initial sidecars creates WAL/SHM when opened normally read-only and retains them after close, while an immutable read-only open performs the same version/integrity reads without creating sidecars. The main database bytes remain unchanged in both cases.
- Not implemented or verified: a persistent-WAL-safe exact-v4 verifier regression; a corrected ordinary installer; disposition of the real prepared operation evidence; a second real-profile attempt; real-profile schema v5.

## In Scope

- Preserve and classify exact content-free Phase B evidence.
- Identify the smallest technically plausible correction boundary.
- Present a Founder decision before implementation or further real-profile action.

## Out Of Scope

- Any access to personal rows or raw retained content.
- Any real-profile write, deletion, checkpoint, retry, repair, restore, backup, migration, cleanup, or candidate selection.
- Any code correction before the new Founder gate.
- Provider, ContextPacket, consent, schema/DDL, lifecycle semantics, Phase 4, Android, distribution, deployment, release, or Git promotion.

## Product Constraints

- A safe refusal is not migration success.
- Diagnosis does not grant repair or retry authority.
- The real profile stays the authoritative pre-migration schema-v4 dataset.
- Exact-owned operation evidence and SQLite sidecars remain untouched until separately authorized.
- Any correction must first prove behavior on a synthetic persistent-WAL exact-v4 fixture.

## Evidence And Provenance Constraints

Only content-free metadata may enter repository workflow artifacts: state classes, version numbers, sidecar presence, operation phase, file presence/size, code paths, and equality results. No database content or raw database digest is recorded.

## Historical Context Constraints

No historical-context selection, packet construction, inference, or provider transmission occurs. Existing data and provenance remain untouched.

## Consent Constraints

The original single-attempt authorization is exhausted by the blocked attempt. It does not authorize cleanup, a corrected build, or a second real-profile migration.

## AI-Role Constraints

The engineering assistant may diagnose repository code and recommend a bounded option, but may not reinterpret a stopped migration as success or choose a recovery policy for the Founder.

## Privacy Constraints

Do not open or report personal content. Do not copy, move, hash into documentation, or mutate the real database, sidecars, operation directory, or staging evidence.

## User-Agency Constraints

The Founder must explicitly choose whether to start a separate disposable-only correction sprint. A later real-profile evidence disposition and retry require another explicit gate after corrected disposable evidence.

## Acceptance Criteria

1. The Phase B attempt is recorded as blocked before backup and before schema migration.
2. The exact pre-state equality, schema v4, prepared operation, no backup, and WAL/SHM evidence are preserved without personal content.
3. The persistent-WAL read-only sidecar mechanism is reproduced on a disposable database and tied to the actual verifier code.
4. No real-profile mutation, retry, cleanup, or repair occurs.
5. The next implementation scope, if selected, is disposable-only and cannot itself authorize a second real-profile attempt.
6. Workflow stops at `human_decision_required` with an exact Founder response.

## Risks

- Leaving the current profile in prepared evidence state means the schema-v5 review build will continue to fail closed; this is the safe default.
- Deleting sidecars or operation evidence without a governed disposition could erase forensic state or violate the one-attempt boundary.
- Applying only a code change without a persistent-WAL regression could repeat the defect.
- Treating an immutable read as universally sufficient without focused Rust verification could hide a malformed or uncheckpointed fixture; the correction sprint must test the exact safe precondition.

## Open Questions

Whether the Founder authorizes a separate bounded, disposable-only persistent-WAL verifier correction sprint. Real evidence disposition and a second real migration attempt are intentionally deferred to a later independent gate.

## Human Decision Required

Resolved by the Founder's exact response `Option A`. Decision ID:
`ORDINARY-V5-PHASEB-WAL-VERIFIER-CORRECTION-001`. The approved correction is
disposable-only; the actual ordinary profile and all prepared recovery evidence
remain untouched, and a later real-profile action requires another gate.

## Recommendation

Select Option A: authorize only a separate disposable correction sprint that makes exact-v4 verification sidecar-free, adds production-shaped persistent-WAL regression evidence, synchronizes factual documentation, runs focused/Clippy/canonical verification, builds a new ignored unsigned review installer, and stops before any real-profile evidence disposition or retry.

## Review Status

approved_with_conditions
