# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-26-phase-3c-post-slice2b2-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 277c4b5b2031d5bf88dc2a33b03765c103c62629
- Working-tree digest reviewed: 971f75ff5752ecd8c270d6969c7c671b64a51c3f39b969ea5d9ca5b67e2a58a2
- Created at: 2026-07-25T17:36:12.6880273Z
- Updated at: 2026-07-25T18:02:00.0000000Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Truthfully close promoted Slice 2B-2 facts, then evaluate whether one bounded
integration slice can connect the promoted Slice 2A backup contract to the
Slice 2B-2 ownership and replacement contracts without weakening local-data
safety. Stop before implementation until the Founder chooses an option.

## Problem Statement

The promoted foundations do not yet compose:

- Slice 2A deliberately requires an absent backup destination before issuing
  `VACUUM INTO`.
- Slice 2B-2 creates the backup, staging, and state files with `create_new`
  during `prepare_operation`, so the backup path already exists.
- Slice 2B-2 has only an injected `ReplacementAdapter`; `TestReplacement` is
  not a production Windows implementation.
- `SystemDurability` reports parent-directory synchronization unsupported on
  Windows, and the existing execution path refuses a replacement that requires
  it before invoking the adapter.
- `QuiescenceProbe` accepts an injected activity value. It does not prove that
  every SQLite connection, renderer task, cleanup operation, or persistence
  caller is actually stopped.

Deleting, truncating, or overwriting the pre-created backup would weaken the
ownership boundary. Treating the interface as production quiescence or the test
adapter as Windows atomicity would overstate evidence.

## User Value

A bounded Slice 2B-3 can remove a real integration blocker before schema-v5
migration work. It would prove that an operation can exclusively claim a
high-entropy destination name while leaving that pathname absent for SQLite,
verify the closed backup immediately, and classify Windows replacement results
without touching real user data. This reduces future data-loss risk while
preserving an easy rollback boundary because no runtime entry point exists.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: the Founder and user remain final authorities;
  local data must not be silently overwritten, repaired, or reinterpreted.
- `docs/06_Memory.md`: stored history is user-owned information.
- `docs/10_Privacy.md`: local duplicates remain sensitive and require
  minimization, explicit lifecycle boundaries, and safe deletion behavior.
- `docs/12_Roadmap.md`: all five Phase 3 exit gaps remain blocking; storage
  foundations do not authorize Phase 4.
- `docs/appendix/Harness.md`: evidence, uncertainty, and user authority remain
  visible; deterministic storage code is not an oracle.

## Relevant ADRs

- `ADR-0007`: preserve reviewed-artifact content, authorship, review state, and
  provenance exactly.
- `ADR-0009`: preserve exact consent, packet, transmission, dependency,
  deletion, and actual-use provenance; never reconstruct consent.
- `ADR-0010`: Cross-Experience Reflection remains separately gated and
  user-owned; it is not part of this storage work.
- `ADR-0011`: append-only lifecycle and portable-provenance direction is
  Accepted, but production migration and destructive recovery remain
  separately authorized.

## Current Implementation Context

- **Founder-approved:** architecture/12 and architecture/13. ADR-0011 is
  Accepted.
- **Implemented, verified, and promoted:** Slice 0, Slice 1A, Slice 1B-1,
  Slice 1B-2, Slice 2A, Slice 2B-1, and bounded Slice 2B-2.
- **Slice 2B-2 promotion evidence:** feature commit
  `a27faec856b2b1d628e87a1c8df1d13279668e14`, non-fast-forward merge
  `277c4b5b2031d5bf88dc2a33b03765c103c62629`, clean synchronized `develop`,
  and passing canonical local verification before and after promotion. Remote
  CI was not separately observed. Desktop runtime smoke was not applicable
  because the module is private, unregistered, and unreachable from runtime.
- **Current production database:** `SCHEMA_VERSION` and SQLite `user_version`
  remain 4.
- **Proposed only:** Slice 2B-3 backup-name ownership handoff and a private
  Windows replacement adapter in synthetic/disposable app-like directories.
- **Unauthorized:** schema-v5 execution, real user data, production
  backup/restore, product activation, retention, later slices, and Phase 4.

## Compatibility Matrix

| Contract | Slice 2A | Slice 2B-1 | Slice 2B-2 | Integration result |
| --- | --- | --- | --- | --- |
| Backup destination | Must be absent; collision refuses; `VACUUM INTO` creates output | Consumes an already verified backup | `prepare_operation` pre-creates the backup file with `create_new` | **Incompatible:** Slice 2A cannot consume the currently reserved backup path without weakening one contract |
| Ownership | Test-local pathname check; failure cleanup is best-effort | Staging uses test-local owned creation | Backup, staging, and state are all create-new owned files | Needs a distinct state-backed claim for an absent backup path |
| Backup verification | Closed SHA-256, source manifest, schema, FK, integrity | Revalidates those plus exact expected records | Abstract `CandidateVerifier` accepts those evidence fields | Semantically compatible after one shared private integration seam |
| Restore staging | Not applicable | Test-local copy and logical replacement simulation | Owned staging plus exact cleanup rules | Compatible as evidence, but 2B-1 remains test-only |
| Replacement | None | Injected logical simulation only | Trait and outcome classes only | Missing concrete Windows adapter |
| Durability | SQLite output sync depends on SQLite/source synchronous behavior; no parent-directory claim | No OS durability claim | File sync; Unix parent sync; Windows parent sync unsupported | Windows durable commit remains unproved and must fail closed |
| Restart evidence | None | Test rollback only | Prepared/committed/recovery-required state classification | Compatible only after backup creation updates state truthfully |
| Quiescence | Fixture-local connection behavior | Fixture-local only | Injected probe/guard contract | No process-wide proof; production activation remains blocked |
| Runtime activation | None | None | None | Compatible: all remain disconnected |

## In Scope

1. Preserve the factual architecture/13 Slice 2B-2 promotion closeout.
2. Founder-gate one Slice 2B-3 that may:
   - replace pre-created backup-file ownership with an operation-state-backed,
     high-entropy claim to an absent pathname inside an exclusively created,
     canonical operation directory;
   - preserve create-new state/staging ownership and refuse any collision,
     alias, hard link, symlink, junction, or reparse-chain mismatch;
   - revalidate the directory identity, operation state, quiescence contract,
     sidecar absence, source identity, and backup-path absence immediately
     before `VACUUM INTO`;
   - have SQLite create the claimed destination exactly once and immediately
     close and validate the resulting regular file, link count, digest,
     governed source manifest, schema v4, foreign keys, integrity, and exact
     expected synthetic records;
   - preserve ambiguous or unprovably owned output instead of deleting it;
   - extract and integrate the existing Slice 2A backup logic into the private,
     unregistered filesystem module rather than create a parallel backup path;
   - implement a private Windows `ReplaceFileW` adapter with conservative
     result classification and deterministic injected tests;
   - keep Windows execution fail-closed before replacement when the required
     metadata durability guarantee cannot be established;
   - run only against synthetic/disposable app-like roots.
3. Stop at Founder decision before creating a feature branch or implementation.

## Out Of Scope

Real user databases; real app-data paths; production backup/restore or file
replacement activation; Tauri commands; renderer/UI/startup integration;
migration disclosure; retention, delete-now, or scheduling; autonomous retry,
replay, rollback, repair, candidate selection, or sidecar cleanup; schema-v5
DDL; `user_version = 5`; v4-to-v5 migration; Slices 3-6; Phase 4;
provider/ContextPacket changes; Harness expansion; Stage 2/3; staging, commit,
push, merge, PR, or deployment.

## Product Constraints

- A pathname claim must not be represented by a pre-created backup file.
- A unique operation directory and state record can govern an absent child
  name, but this proves only the application threat model. It does not protect
  against a malicious same-user process with equivalent filesystem access.
- The final absence and directory-identity check must be adjacent to SQLite
  invocation. Any observed collision fails closed; an injected collision must
  never be truncated or deleted.
- An output is owned only after the exact operation state, directory identity,
  expected path, and post-close file identity all reconcile. If ownership is
  ambiguous, preserve evidence and return `recovery_required`.
- `ReplaceFileW` success is a logical replacement result, not proof of
  power-loss durability. Windows parent-directory durability remains
  unsupported in the current contract.
- `ERROR_UNABLE_TO_REMOVE_REPLACED` may be classified
  `failed_unchanged` only with exact post-error live identity/digest
  reconciliation. `ERROR_UNABLE_TO_MOVE_REPLACEMENT`,
  `ERROR_UNABLE_TO_MOVE_REPLACEMENT_2`, and any undocumented or contradictory
  state are `outcome_unknown`.
- The existing injected quiescence guard remains contract evidence only. No
  product activation may infer process-wide quiescence from it.

## Evidence And Provenance Constraints

Every successful synthetic backup must bind the operation ID, canonical root
and source identities, expected absent destination, closed backup SHA-256,
governed source-manifest digest, schema version, foreign-key result, integrity
result, and exact-record snapshot. State is content-free. No failure may create
or modify Life OS evidence, reflection, consent, transmission, dependency, or
provenance records.

## Historical Context Constraints

Historical rows may occur only as opaque synthetic fixture records used to
prove exact preservation. No retrieval, Context Packet, provider call,
Cross-Experience Reflection, recurrence, contradiction, change-over-time
interpretation, historical summary, Pattern hypothesis, or identity inference
is performed.

## Consent Constraints

No consent is created, consumed, reused, or transmitted. Existing ADR-0009
fixture rows are preservation evidence only. A future real-user backup or
restore action requires a separate disclosure, lifecycle, and explicit-action
gate.

## AI-Role Constraints

No model call or AI interpretation occurs. The orchestrator may recommend but
cannot authorize Slice 2B-3, choose a database as the user's truth, or convert
an ambiguous filesystem outcome into automatic recovery.

## Privacy Constraints

Only synthetic databases in disposable app-like roots are eligible. Operation
state and manifests must remain content-free. No Experience text, artifact
content, packet content, credential, raw provider response, or real app-data
path enters the evidence. Unknown or ambiguous files are preserved rather than
deleted.

## User-Agency Constraints

The proposed slice has no user-facing action and cannot silently back up,
replace, restore, repair, downgrade, migrate, or delete a real database.
Ambiguity remains visible as `recovery_required` for a later explicitly
designed human-controlled flow.

## Acceptance Criteria

If the Founder authorizes Option A:

1. Slice 2A's absent-destination rule remains unchanged.
2. Operation ownership uses a create-new operation directory/state claim while
   the exact backup child path remains absent until SQLite creates it.
3. Collision, alias, hard-link, symlink/reparse, parent-identity change, path
   escape, cross-volume, and unknown ownership all fail closed without
   truncation, overwrite, or unknown-file deletion.
4. The final preflight rechecks quiescence, sidecars, source identity, root and
   operation-directory identity, state ownership, and backup absence.
5. `VACUUM INTO` is invoked once against the exact claimed absent path.
6. After SQLite closes the output, the backup is opened without following a
   reparse point and immediately reconciled as a direct, single-link,
   operation-owned regular file.
7. Digest, manifest, schema-v4, foreign-key, integrity, and exact-record
   verification completes before the operation becomes backup-verified.
8. Any incomplete output is deleted only when exact operation ownership is
   positively proved; otherwise it is preserved as `recovery_required`.
9. The existing test-local backup logic is integrated into the private module;
   no second backup implementation or runtime registration is added.
10. A `cfg(windows)` private adapter calls `ReplaceFileW`; success maps to
    logical `committed`, documented unchanged failure is reconciled before
    `failed_unchanged`, and destructive/partial/undocumented results map to
    `outcome_unknown`.
11. Windows tests exercise API result mapping and real disposable-file success
    where deterministic, plus injected 1175/1176/1177/unknown outcomes. Tests
    do not claim crash durability.
12. With `SystemDurability` on Windows, any flow requiring parent-directory
    durability refuses before replacement; no successful API call is relabelled
    durable.
13. Restart tests cover claimed-but-absent, partially created, verified backup,
    failed unchanged, committed, and ambiguous evidence without autonomous
    action.
14. Quiescence remains explicitly unproved beyond the injected contract.
15. Production schema and `user_version` remain 4; Tauri, renderer, UI,
    startup, app-data, retention, provider, and Phase 4 remain unchanged.
16. Canonical verification and Theory Alignment Review pass before Founder
    diff review.

## Risks

- `VACUUM INTO` takes a pathname, not an application-owned file handle. A
  high-entropy child in an exclusively created operation directory plus a
  process-wide operation lock can prevent application races, but cannot defend
  against a malicious process running as the same OS user. This residual threat
  must remain explicit.
- SQLite can leave an incomplete/corrupt output after interruption. Ownership
  must be proved before cleanup; otherwise preservation is safer.
- Windows `ReplaceFileW` preserves useful file metadata, but its documented
  `REPLACEFILE_WRITE_THROUGH` flag is unsupported. A successful call therefore
  does not establish parent-directory or power-loss durability.
- Some `ReplaceFileW` failures can remove or rename the replaced file. Broadly
  treating nonzero errors as unchanged would risk data loss.
- Antivirus, indexing, sync tools, and a second same-user process can interfere
  with creation or replacement.
- A future activated caller still needs a real process-wide exclusive-operation
  coordinator; the current probe is insufficient.
- This slice connects foundations but closes none of the five Phase 3 exit gaps
  by itself.

## Open Questions

Resolved by the Founder through `PHASE3C-SLICE2B3-001`, Option A. Engineering
must remain inside the exact private, unregistered, synthetic/disposable,
schema-v4-only boundary recorded in `DECISION_REQUIRED.md`.

## Human Decision Required

false. Decision `PHASE3C-SLICE2B3-001` is resolved exactly as Option A.

## Recommendation

Proceed only with the Founder-authorized Option A. The Engineering Plan must
copy the ownership, Windows durability, synthetic-data, no-runtime, and
no-promotion boundaries exactly and stop at Founder diff review after canonical
verification, Theory Alignment Review, and archive/reset.

## Review Status

approved_with_conditions.
