---
status: Proposed
version: "0.1"
owner: LIN MENGLUNG
last_updated: 2026/11/18
depends:
  - docs/00_Constitution.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/adr/ADR-0012-android-app-private-schema-v5-storage-and-stable-identity.md
  - docs/product/01_Android_Model_Pilot_R3_Proposal.md
referenced_by:
  - docs/product/01_Android_Model_Pilot_R3_Proposal.md
  - docs/architecture/24_Android_M2C_Offline_Synthetic_Daily_Reflection.md
---

# ADR-0013 (Proposed): Independent Model Feedback and User Supplements

## Status and authorization

**Proposed, not accepted. No migration or feedback/supplement persistence is implemented.**
The Founder approved R3 items 1, 5 and 6 together for source and host tests,
not packaging. The approved proposal specifically requires exact canonical
storage/upgrade review if the current schema cannot express the new records.
The credential source work does not depend on the journal schema.
This ADR isolates the now-concrete storage decision; it is not a request to
reapprove the entire UI, remote provider, or cross-moment Pattern feature.

## Verified blocker in the existing code

1. `schema_v5.sql` restricts `artifact_heads.artifact_kind` to Evidence,
   Reflection, Pattern, recovery turn and historical question. Provenance
   roles are only `content`, `prompt`, `response`; dependencies have a closed
   relationship vocabulary without feedback/supplement parent roles.
2. Canonical Pattern creation (`schema_v5_pattern_write.rs`) requires
   confirmed Evidence. The accepted R2 source-only Gemini question does not
   manufacture Evidence, and a neutral feedback is not necessarily a Pattern.
3. Reflection's question/response schema and mixed authorship mean that
   inserting a feedback into the answer, or treating a user supplement as a
   newly generated question, would falsify the content's semantic role.
4. The exact v5 verifier pins tables, DDL and schema manifests.
   `database_contract.authoritative_schema` has `CHECK(... = 5)` and old
   migration receipts truthfully describe v4→v5. Appending tables while still
   claiming an unchanged v5, suppressing verification, or rewriting those
   receipts is not compatible with that contract.

The implementation must not widen Pattern admission, fake Evidence, store
interpretations in an extra JSON file, add arbitrary payload keys, or change
Book Zero to work around these constraints.

## Proposed decision: isolated Model Pilot v6, one app and one journal

Keep `com.lifeos.review.pilot.model`, the current signing lineage and the
existing app-private journal filename. Add a precisely versioned **Model
Pilot schema v6** path only when `android-pilot-online` is selected.
Desktop and independently buildable offline paths remain exact v5 and refuse
unsupported databases rather than reinterpret them.

Use separate normalized model-turn records within the same canonical SQLite
database. Do not change the five v5 artifact kinds or overload their
compatibility projections. Do not make saving feedback a user confirmation
that its interpretation is objectively true.

### Proposed new tables and invariants

| Table | Fields and purpose |
| --- | --- |
| `pilot_turn_heads` | Stable `id`, `source_id` FK, `kind` (`feedback` or `supplement`), current revision FK, lifecycle (`active`, `invalidated`, `deleted`), timestamps. Deleted means null current revision and no retained content. |
| `pilot_turn_revisions` | Stable revision ID, turn FK, source ID, strictly increasing number, optional predecessor, authorship (`ai` for feedback, `user` for supplement), content digest, creation time. Immutable lineage; no in-place content edits. |
| `pilot_turn_content` | Revision FK; typed nullable columns: observation, interpretation, uncertainty, literal basis quote, optional clarification question, or user supplement text. A closed kind/shape check prevents AI text being stored as user content and vice versa. Content is purgeable, revision metadata is not a shadow copy. |
| `pilot_turn_provenance` | Turn revision FK and canonical provenance-record FK, with separate AI-content or user-content role. Does not misuse the existing artifact-revision FK to refer to a new model turn. |
| `pilot_turn_dependencies` | Exact source revision, root Reflection/answer revision, parent feedback revision and explicitly selected supplement revisions, each with an explicit role and FK. The transaction verifies same-source, current/eligible revisions, role/kind, acyclic ancestry and exact selection. |
| `pilot_turn_events` | Content-free explicit-save, supersession, invalidation, deletion and purge events, bound to exact revisions and action time. Saving is review-for-retention, not Evidence confirmation. |
| `pilot_upgrade_receipts` | Separate model v5→v6 receipt: versioned DDL, complete pre/post manifest digests, owned backup reference and action time. Existing v4→v5 receipts and the original direct-fresh-v5 initialization receipt remain unmodified. |

AI provenance must include provider, model, Harness and prompt versions,
locale, generated time and explicit-save time. Put it in the canonical
provenance relationship rather than another free-form interpretation blob.
The full DDL, FKs, triggers, canonical serialization and manifests must be
reviewed as the next engineering artifact before migration code is enabled.
No table needs or may contain a model API key.

### Source, user and model ownership

- Initial feedback depends on the exact original and saved question/answer.
- A saved supplement is user-authored, separately linked to the feedback it
  qualifies and that same root question/answer. It does not edit the moment.
- A further feedback is a new AI result with a parent link, not a rewrite of
  the earlier model's words. Explicit save gives it a stable record.
- A further user correction creates a new supplement revision. Prior
  interpretations remain distinguishable, superseded and context-ineligible.
- Only user-selected current saved material may be used for another request.
  Stored feedback is never automatically Evidence, Pattern, Identity,
  longitudinal memory or authority over the user.

### Local write and provider boundaries

Use a native process-local result ticket for saving exact model output. The
renderer cannot mint a retained model response by editing its JSON. Saving,
deleting or editing local supplemental text never invokes the provider.
An identical retry reconciles the same exact action; a different text or
revision is refused. An uncertain commit freezes that action for read-only
reconciliation and never silently resends to Gemini.

Every feedback request has its own destination/material preview and explicit
synthetic-only consent. It sends exactly the saved current original, root
question/answer, selected parent feedback and selected saved supplements.
Do not transmit local IDs, other moments, unselected early rounds or drafts.
Bound round count and total bytes; refuse overflow with a choice rather than
silently truncating or auto-summarizing. Native code checks all revisions
before transmission and after reply; the renderer drops late/stale results.

### Lifecycle: correction, deletion and restart

- Editing the source, root answer, retained parent feedback or a selected
  supplement invalidates the exact downstream closure in the same transaction.
  Never attach earlier interpretations to revised input silently.
- Invalidated content is visible as historical only and excluded from context.
  The UI explains the changed source once rather than warning per question.
- Deleting a feedback removes its text and content-bearing provenance;
  downstream supplements/results are invalidated and cannot be sent again
  unless the user makes a new explicit choice under a new valid parent.
- Deleting a supplement purges its content and invalidates all dependents.
- Deleting a moment purges **all** model-turn content and content-bearing
  copies for that source along with existing dependent content, transactionally.
  Minimal tombstones follow ADR-0011 and expire with parent deletion.
- API receipt outside the device cannot be undone by local deletion.
- Unaccepted feedback and unsaved drafts remain session-only. After process
  restart only explicitly saved turns rehydrate; no recovered-content claim.

### Upgrade and rollback

Before upgrade, verify the exact Model Pilot identity, owned direct-v5
receipt, active v5 schema and complete source/artifact manifests. Refuse
unknown/partial/newer states without creating or repairing a database.
Create a new exclusively owned app-private pre-upgrade backup with an exact
digest and explicit retention/disposal decision. Never modify the existing
snapshot to obtain a matching manifest. No backups go to cloud/transfer.

On the same controlled connection, perform the new normalized DDL and an
explicit `database_contract` rebuild with authoritative version 6, preserving
the other verified contract values; set `PRAGMA user_version=6`. Record the
new receipt without altering v5 receipt history. Verify the complete v6
schema plus unchanged preexisting records before acknowledging. The native
source/answer writers and invalidation/deletion paths need an explicit
Model-v6 contract adapter; do not simply skip their v5 checks.

An ordinary precommit failure rolls the whole action back. For an uncertain
commit, classify only exact v5-prestate or exact v6-poststate via independent
read-only verification; any third state is blocked and preserved. A 3101
binary is **not** a supported downgrade after v6 writes. No automatic
down-migration, source reset, uninstall, app-data clearing or restore occurs.
Restoring a pre-upgrade backup, if ever proposed, is a separate explicit
data-loss decision; it cannot preserve newly saved v6 turns.

### Required checks before one integrated APK

Host fixtures must cover old 3101 question/answer retention, complete schema
and receipt validation, every upgrade/write failure boundary, uncertain
commit reconciliation, duplicate/divergent requests, feedback/user authorship,
round-by-round exact previews, stale replies, saved-only restart readback,
transitive correction/deletion and purge of all content-bearing copies.
No privacy or data-lifecycle test may be weakened to get a passing result.
Founder then reviews one packaged update on Zenfone 10 after separate
packaging, signing and installation gates, using fictional data only.

## Alternatives rejected

- Saving in `localStorage`, a JSON sidecar or a second improvised journal DB:
  breaks canonical atomic dependency/lifecycle handling.
- Pretending feedback is Evidence, appending it to a user response, or using
  a recovery prompt as an arbitrary transcript: erases semantic ownership.
- Broadly allowing zero-Evidence Pattern records: changes a separate accepted
  boundary and wrongly implies every feedback is a Pattern.
- Persisting every model response automatically: conflicts with explicit
  reviewed retention under ADR-0007 and the approved R3 proposal.
- New package identity and start-over store: unnecessary loss of continuity.

## Explicit Founder decision needed

Accept or revise the isolated Model Pilot v6 design and review the resulting
exact DDL/upgrade artifact before migration implementation. This is separate
from the already approved R3 user flow. No accepted ADR status is changed
by writing this draft. Cross-moment Pattern summaries (ADR-0010), production
real-data use, package generation and Git publication remain out of scope.
