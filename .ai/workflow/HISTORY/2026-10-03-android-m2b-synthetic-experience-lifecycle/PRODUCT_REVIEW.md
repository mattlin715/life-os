# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-10-03-android-m2b-synthetic-experience-lifecycle
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: aebef185346effda779a6a10850882a2dc19e701
- Working-tree digest reviewed: clean baseline; workflow-only intake edits
- Created at: 2026-10-03
- Updated at: 2026-10-03

## Mission Interpretation

Complete only the synthetic Android Experience journey under ANDROID-M2B-SYNTHETIC-LIFECYCLE-001. This is application of existing lifecycle policy, not approval of new policy or production use.

## Problem Statement

Accepted M2-A exposes create/list/get only. Users cannot correct their own text or explicitly remove a saved synthetic Experience.

## User Value

Local authorship, correction and deletion controls without inference or technical-dashboard burden.

## Relevant Primary Definitions

Constitution: Mirrors, Not Oracles; Memory (docs/06_Memory.md): user-owned records, not inferred truth; Privacy (docs/10_Privacy.md): understandable inspect/edit/delete and no hidden reusable deleted content; MVP (docs/11_MVP.md): local Experience control. No primary definition changes.

## Relevant ADRs

ADR-0011 and architecture/12: immutable metadata/purgeable content, user-authored correction, exact predecessor, no dependency rebinding, approved parent deletion. ADR-0012: Rust-owned app-private direct fresh-v5, commit plus verified reread, idempotent request identity and fail-closed preservation. ADR-0007/0009 remain authoritative for dependencies; no artifacts/providers are exposed.

## Current Implementation Context

M2-A exact candidate was Founder accepted and promoted, archived, integrated at aebef185. Full baseline canonical verification passed this sprint. Canonical ExperienceWriteCommand Update creates a revision even for identical text, demands a later timestamp and expected current revision. Delete purges source content/projection and source-scoped content/tombstones while keeping content-free source head/revision/provenance metadata. The writer remains source of transaction semantics.

## In Scope

Separate com.lifeos.review.m2b, fresh synthetic fixtures, direct initializer reuse; edit/save/cancel; confirmed delete/cancel; exact current-revision checks, atomic writer reuse, safe request reconciliation, trilingual UI/locale/icon preservation; host/native/fault evidence and narrow docs.

## Out Of Scope

All non-scope fences in CURRENT_MISSION; no history export, per-revision purge UI, retention jobs, new deletion cascade, production or later M2 work.

## Product Constraints

Conditions: reuse canonical same-content revision behavior; preserve prior revisions on correction and purge all source text on delete; disclose logical purge versus physical erasure; optional provenance/limitations only; synthetic-only distinct sandbox. No current text silently promoted to Evidence.

## Evidence And Provenance Constraints

Identity preserved, new revision authored by user, exact predecessor and timestamp; current state/projection change in one canonical transaction. Deleted source metadata is content-free and never reusable content. No raw text in logs/errors/receipts.

## Historical Context Constraints

No historical UI/provider use. Unexpected dependency content is refused before mutation in this isolated slice; do not create/rebind/regenerate dependent artifacts.

## Consent Constraints

Explicit Save and confirmed Delete only. Draft/cancel/navigation/close cause no lifecycle mutation.

## AI-Role Constraints

No AI/artifact creation, conclusions, diagnostic advice or identity labels.

## Privacy Constraints

App-private synthetic storage, no network or backups; source text purged logically by canonical deletion, no SQLite/device secure-erasure claim. UI drafts/details/caches cleared after successful deletion.

## User-Agency Constraints

Cancel always non-mutating; no precommit success; block duplicate/stale submissions; uncertain outcome is reconciled using immutable exact request identity, never a new destructive request.

## Acceptance Criteria

1. Create/edit/restart returns exact CJK/multiline text and stable source identity with immutable predecessor/user revision metadata.
2. Cancel edit/delete causes no database/event changes; same-content edit follows canonical contract.
3. Duplicate requests reconcile without new revisions; stale/two-editor/edit-delete races refuse safely; rollback and uncertain commit classifications are honest.
4. Confirmed delete purges all logical source content, removes active list/get, clears visible caches, remains absent after restart; allowed metadata is documented/tested.
5. Malformed/newer/pending/dependency states fail closed and preserve bytes; no raw-content leakage or desktop regression.
6. Distinct inspected debug APK, bound owned Android36 x86_64 AVD, exact evidence/digest/hash, preserved accepted apps/icon/locale.
7. Founder consolidated diff + short manual UI checklist remains pending; no publication or next-slice action.

## Risks

Ambiguous acknowledgement, SQLite journal/physical storage residues, incomplete device matrix, unexpected dependencies and stale content caches. Host tests are not native durability proof.

## Open Questions

None requiring new policy identified in inspected approved sources/writer. If later found, stop with one consolidated Founder package.

## Human Decision Required

False before implementation under explicit bounded authorization. Final exact-candidate Founder acceptance remains required.

## Recommendation

Proceed only with all listed conditions and exact writer parity, then stop at Founder gate.

## Review Status

approved_with_conditions
