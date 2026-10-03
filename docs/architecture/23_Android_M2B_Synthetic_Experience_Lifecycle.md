---
status: Founder-accepted synthetic implementation
version: 0.4
owner: product-and-engineering
last_updated: 2026/10/04
depends:
  - docs/00_Constitution.md
  - docs/06_Memory.md
  - docs/10_Privacy.md
  - docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md
  - docs/architecture/22_Android_M2A_Direct_Fresh_v5_Initialization.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
referenced_by:
  - docs/00_Index.md
  - docs/dev/14_Android_M2B_Synthetic_Experience_Lifecycle_Runbook.md
---

# 23 Android M2-B Synthetic Experience Lifecycle

## Authority and isolation

`ANDROID-M2B-SYNTHETIC-LIFECYCLE-001` authorizes only synthetic implementation,
debug packaging, fresh disposable native fixtures and bounded corrections to
Founder consolidated diff + manual UI review. No new retention/deletion policy,
Accepted ADR change, real data, production or later slice.
**We Build Mirrors, Not Oracles.** Records stay user-authored text, not AI
Evidence, diagnosis, or identity claims.

Separate identity `com.lifeos.review.m2b` owns `android-m2b-disposable-v5.db`,
`android-m2b-direct-fresh-v5.receipt.json` and matching fixed pending names.
It reuses M2-A initialization/publication/readiness under a fixed identity-bound
profile: direct schema 5, zero migration receipts, exact schema/origin receipt,
serialized operations and ambiguous-state preservation. Desktop routes and
canonical DDL/hashes are unchanged. Accepted M0/M1/M2-A apps and archive/failure
evidence are not replaced, cleared or uninstalled.

## Exact approved policy mapping

| Behavior | Approved source | Existing canonical implementation |
| --- | --- | --- |
| Stable identity; immutable user-authored correction and exact predecessor | ADR-0011; architecture/12 correction rules | Experience Update / insert_source_revision / link_user_provenance |
| Expected current revision; later timestamp; atomic head/content/projection | architecture/12 v5 invariants and promoted parity in architecture/13 | BEGIN IMMEDIATE, guarded projection and full transaction reconciliation |
| Identical text creates an explicit new revision | Existing canonical Update contract (no same-text shortcut) | Unchanged Update branch, direct-origin tests |
| Correction retains prior text, never rebinds dependencies | architecture/12 supersession/dependency rules | Immutable source_revision_content and exact predecessor |
| Explicit parent deletion purges source/scoped content and tombstones | architecture/12 parent deletion; ADR-0009 historical consequences | Unchanged Delete branch clears pointer, purges source content, artifacts, tombstones and projection |
| Content-free lineage and anti-resurrection | ADR-0011 metadata and existing deletion | Deleted source_heads; immutable source_revisions/provenance; active list/get exclude deleted source |
| Commit plus verified reread and exact request reconciliation | ADR-0012 decisions 6/7 | Existing writer pre/post manifest classification plus typed direct lifecycle bridge |

No artifacts/history can be created through this slice. Any unexpected artifact
or dependency inventory fails closed at the facade, preserving state. This is
the authorized refusal option, not a new cascade/orphan policy.

## Frozen request identity

One immutable descriptor binds operation, source ID, expected revision, one
canonical timestamp and optional replacement body. Its ID is SHA-256 over a
versioned JSON tuple of those metadata fields and the body digest (empty for
delete). Rust recomputes the identity. Retry never advances timestamp or
substitutes a newer expected revision. No new ledger/table/retention policy.

Lost update acknowledgements reconcile against exact immutable source revision,
predecessor, timestamp, digest and user authorship. If superseded/deleted later,
`committedNotCurrent` returns no prior text. A delete matches the deleted head
timestamp and final source revision. Other stale work is refused. Acknowledgement
contains only request/source identity and status; UI rereads active record/list
before success. Reopening current state is read-only and does not pretend to
confirm an uncertain operation. Process serialization and expected-revision
transactions are not claims of arbitrary multi-process access safety.

## UX and deletion truth

Keep accepted icon/safe-area layout, three languages and app-private non-content
locale preference. Only add Edit/Save changes/Cancel and confirmed Delete/Cancel.
Founder `ANDROID-M2B-FOUNDER-REVIEW-001 Option C` authorized only a trilingual
copy/information-layer correction after the original manual checklist 7/7 PASS:
separate per-moment version headings from build-scope headings; keep deletion
confirmation focused on removal of current and previously saved text. Existing
metadata-retention and physical-erasure caveats remain in optional disclosure.
This changes no storage, revision-retention, deletion or consent policy. The
corrected exact APK requires a new Founder gate; prior manual PASS is historical.
Founder `ANDROID-M2B-FOUNDER-REVIEW-002 Option C`, after that corrected manual
7/7 PASS, authorizes status presentation only. Nonempty new-draft warnings belong
inside the new-moment card; changed-edit warnings belong inside the exact-text
editor. Empty or merely opened saved records show no unsaved warning. Verified
create/edit/delete feedback is separate above the input card and names its
operation; saving, unconfirmed, conflict and failure feedback remains visible
with existing controls. No timers hide abnormal outcomes. Existing lifecycle
handlers, requests and storage policy remain unchanged; another exact Founder
review is required rather than transferring either historical manual PASS.
Unsaved/cancel/navigation actions invoke no lifecycle writes. Old detail/list
text is hidden during uncertain mutation. Deletion clears detail/editor/list,
source draft and pending-create UI caches. No arbitrary backend payload is logged
or rendered. Optional disclosure, not a technical dashboard.

Prior text remains locally in canonical revision content until parent deletion;
this slice adds no history browser, per-revision purge, export or AI/context use.
Deletion is logical content purge, **not secure physical erasure**. Existing
content-free IDs, revision numbers, digests, authorship, lineage, times and
provenance remain. SQLite pages/journals/device media may have physical residues;
this experiment cannot claim secure erasure or power-loss durability.

## Verification and non-authority

Host tests, emulator checks and fault injections are separate. Native baseline:
exact-owned Android 36 x86_64 AOSP AVD. Debug-only before/after-commit holds and
canonical rollback-after-projection are bounded; force-stop and injected unknown
commit outcomes are not physical power loss. Preserve owned failure snapshots;
never clean unknown devices/profiles/states. Final evidence is recorded in
runbook 14 and the completed workflow. Founder accepted the corrected exact
candidate under `ANDROID-M2B-FOUNDER-REVIEW-003 Option A`, following the current
manual checklist 7/7 PASS. Accepted APK SHA-256:
`8be61db3f500514f01055846273460307580e3ed9246b078d2c3a50cc21eca43`;
accepted non-workflow digest:
`00bf3293f5aa7095cf19d6189a9233ed6120ac19003daf0ba3f448077200a021`.
Earlier correction handoffs and pending-review statements above describe their
historical candidate snapshots; manual acceptance was not transferred.

`ANDROID-M2B-INTEGRATION-CLOSEOUT-001` separately authorizes exact-scope
repository promotion, factual closeout, official workflow follow-up/archive/
reset, the exact M2-B archive-prefix compatibility adjustment, normal non-force
feature/develop pushes and a non-fast-forward develop merge. It grants no new
product behavior, APK rebuild/native run or manual review repetition.
No PR/release/distribution/deployment, real data/`com.lifeos.app`, provider/
network/credentials, sync/import/export/backup, migration/recovery, later M2 or
Phase 4 is authorized. Actual publication/integration results are recorded only
after verification; acceptance and integration do not establish production or
physical-device readiness.
