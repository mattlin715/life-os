# Theory Alignment Review

Status: human_decision_required

- Sprint ID: 2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Working-tree digest at corrective-cycle-2 canonical verification: 6f9d1450ebde154ccbf41d1a1874586a3be162561e50360ea362161d75ecaa61
- Created at: 2026-08-13T22:32:00+09:00
- Updated at: 2026-08-16T12:00:00+09:00

## Actual Diff Reviewed

Reviewed the complete tracked and non-ignored untracked diff on `codex/desktop-schema-v5-founder-dogfood-activation-r1`: candidate-only Tauri/Rust/TypeScript/UI/package boundaries; promoted private v5 writer visibility/batch changes; filesystem replacement semantics; architecture/17 and factual Book One synchronization; tests; repository workflow evidence. `git diff --check` passed and generated package artifacts remain ignored.

## Acceptance Criteria Verification

- Immutable Founder identity plus Cargo/frontend gates: pass.
- Ordinary profile remains schema v4 and cannot select v5: pass.
- Missing isolated profile exact-v5 initialization: pass by disposable test.
- Exact-v4 explicit migration and cancel boundary: implemented; automated pass, Founder manual pending.
- Verified backup, restart classification, conservative ambiguity: pass by promoted and candidate tests.
- Typed current-action routing and projection verification: pass by focused and existing lifecycle tests.
- Backup disclosure/delete/restore: implemented; automated ownership evidence pass, Founder manual pending.
- Three-language disclosure: pass by focused UI/i18n tests; Founder manual pending.
- No provider, ContextPacket, consent-policy, Phase 4, Android, ordinary-profile, or real-data scope: pass.

## Constitution Alignment

The Constitution is unchanged. Local control, user authority, correction/deletion accountability, and non-oracular behavior are preserved.

## Primary-Definition Alignment

Memory remains local, provenance-bearing, revisable, and user-controlled. The slice changes database representation and routing only; it adds no worldview or inference policy.

## Relevant ADR Alignment

ADR-0009 exact consent/transmission/provenance and deletion semantics remain unchanged. ADR-0011 append-only revision, review, dependency, tombstone, and purge boundaries remain authoritative. No ADR status changed.

## Mirrors-Not-Oracles Alignment

Approved. No model authority, conclusion, recommendation, diagnosis, identity finalization, or sensitive inference was added.

## Context-Before-Insight Alignment

Approved. Existing Experience/Evidence/Reflection/Context Recovery ordering and gates are unchanged.

## Evidence Boundary

Approved. AI candidates and user-confirmed evidence remain distinct; migrated legacy provenance stays honest.

## Provenance Boundary

Approved. v5 authority, v4 projection, immutable receipts, exact revision dependencies, and ADR-0009 actual-use provenance are revalidated and fail closed on drift.

## Artifact Lifecycle Boundary

Approved. Runtime routes only already reachable actions through promoted private writers; no new lifecycle action was invented.

## Historical Context Consent Boundary

Approved. Retrieval, selection, consent, packet assembly, provider transmission, and generated-artifact provenance remain separate. No blanket consent or silent provider action exists.

## Cross-Experience Hypothesis Boundary

Approved. Phase 4 remains absent and unauthorized.

## User Agency

Approved with manual follow-up. Migration, backup deletion, and restore are separate explicit actions with cancel/refusal paths; opening the app or disclosure is not authorization.

## Privacy

Approved. Only the isolated Founder app-data root is derivable inside Rust; renderer input cannot supply paths or SQL. The build/manifest contains no content, credentials, or personal paths. Automated evidence uses disposable fixtures only.

## Psychological Safety

Approved with manual follow-up. English, Traditional Chinese, and Japanese copy is calm, explicit about non-action and recovery limits, and exposes no destructive shortcut. Founder must verify the packaged interaction.

## Scope Deviations

The attempted Windows parent-directory flush was rejected by actual platform evidence. The corrected design uses documented same-volume `MoveFileExW` write-through semantics, exact before/after evidence, and conservative outcome-unknown classification, while explicitly retaining residual power-loss risk. This is within the authorized isolated candidate boundary and does not weaken ordinary-profile safety.

## Required Corrections

The reachable Context Recovery answer path must use the governed artifact kind
`recovery_turn` when resolving the exact current revision. The current runtime
adapter instead requests `context_recovery`, so it fails before the writer
transaction and leaves the answer unsaved. The mutation failure must also be
disclosed within the visible Context Recovery area rather than only through an
off-screen general error surface. These are bounded implementation defects,
not a new product or data-governance decision.

No correction is authorized in this sprint. `review_cycle = 3` already equals
the repository maximum, and the Engineering Harness forbids a fourth revision
attempt. Founder authority is therefore required to close this sprint
truthfully and start one separate minimal corrective sprint.

## Human Decision Required

true: `FOUNDER-SCHEMA-V5-CANDIDATE-R1-CONTEXT-RECOVERY-FOLLOWUP-004`.

## Revision Log

Cycle 0: Windows parent-directory durability assumption failed under a real focused test; evidence was `parent_directory_sync_failed`; implementation correction replaced the unsupported claim with Windows write-through move semantics, explicit residual-risk documentation, and updated tests. Focused tests, Clippy, canonical verification, and packaging then passed. This bounded correction did not require a workflow revision cycle because it occurred before the terminal theory decision and did not alter product or governance authority.

Cycle 1: Founder packaged Step 7B produced a truthful fail-closed result.
Durable evidence proves live schema v4 remained byte-identical (`4d06e3...`),
the exact verified backup exists and passes schema/integrity/FK/record checks,
operation phase is `backup_verified`, no sidecars exist, and no DDL or receipt
was written. A path-injected reproduction identified the mismatch between the
canonical `operation.live` identity and the non-canonical Tauri app-data path.
The isolated caller now reuses `operation.live`. Focused tests cover both a
new exact-v4 authorization and an explicitly repeated action from the durable
verified-backup state. The live Founder profile remains byte-identical v4; the
corrected installer is built but not installed or launched.

Cycle 2: Founder packaged Step 7B-R5 again produced a truthful fail-closed
result and then closed the app normally (`Step 7B-R5-D1 passed`). The durable
live database is schema v5 with its exact records and receipt preserved; the
owned operation records `v5_blocked_restore_available` and
`post_commit_schema_manifest_mismatch`; the exact verified schema-v4 backup is
unchanged. Forensic comparison proves the only schema-manifest difference is
the already-promoted runtime v4 objects' compact SQL formatting versus the
contract fixture's multiline formatting. No retry, restore, cleanup, or state
rewrite occurred.

Cycle 2 correction: the fixed contract-fixture digest remains unchanged and
one exact promoted-runtime-v4 manifest digest is now recognized. A disposable
production-initialized v4 regression proves backup, migration, close/reopen,
receipt, contract, schema-object, content, foreign-key, and integrity
verification. A second disposable regression proves that only exact blocked-v5
evidence with the original verified backup exposes explicit restore, after
which the operation is consumed and the database returns to migration-required
schema v4. Focused Founder activation tests pass 6/6; Clippy passes with
warnings denied; canonical verification passes with 17 workflow tests, 8
Founder package tests, 1 candidate contract test, 45 Vitest files/323 tests,
191 Rust library tests, 12 backup/restore tests, and 8 schema-contract tests.
The unsigned cycle-2 package was built and verified but not installed or
launched. Manual explicit restore, repeated migration, restart, and remaining
package checks are still Founder-owned evidence.

Cycle 3: Founder Steps 7B-R6A through 7B-R6G proved the cycle-2 installer,
exact blocked-v5 disclosure, explicit verified-backup restore, repeated
migration, durable v5 verification, backup visibility, and restart
reconstruction. Step 11A then exposed the runtime-writer manifest predicate
drift. The error occurred before the write transaction. After normal close,
read-only evidence shows two original Experiences, four artifacts and heads,
zero matching new Experience rows, one immutable receipt, lifecycle writes
enabled, export v2 disabled, zero guard rows, valid foreign keys and integrity,
no sidecars, `v5_ready` operation evidence, unchanged exact backup, and an
unchanged ordinary-profile hash. No retry or automatic recovery occurred.

Cycle 3 correction: `schema_v5_experience_write::verify_exact_v5` now calls
the same closed `is_expected_schema_object_manifest` predicate already used by
migration/startup verification. The production-initialized-v4 migration
regression now performs an actual typed Experience create and exact v5
reconciliation. Focused Founder activation tests pass 6/6, Clippy passes with
warnings denied, and canonical verification passes with 17 workflow tests, 8
Founder package tests, 1 candidate package test, 45 Vitest files/323 tests,
191 ordinary-feature Rust tests, 12 backup/restore tests, 8 schema-contract
tests, 6 Founder activation tests, and 2 runtime-focused tests. No live profile
was opened or mutated by the correction or automated verification. The
unsigned cycle-3 installer was built and its content-free manifest verified;
SHA-256 is
`c9709e90c994ce11c27526f521ca0b6044e5bef1c3dc7513798bd73876ea3a20`.
It has not been installed or launched.

Cycle 3 manual evidence: the Founder installed the verified cycle-3 package
but explicitly chose the NSIS uninstall path and selected `Delete the
application data`. Read-only filesystem evidence shows the isolated Founder
root and database were recreated immediately afterward, while the ordinary
profile remained byte-identical. This is truthful user-authorized deletion,
not evidence of silent installer or migration loss. The newly initialized
exact-v5 profile accepted one packaged typed Experience create and closed with
one exact row, valid foreign keys, and integrity `ok`. That proves the packaged
fresh-v5 runtime route, but not the promoted-runtime-v4 manifest correction in
isolation. The latter remains directly covered by the cycle-3 migration-plus-
typed-write regression, while the cycle-2 package already supplied live
migration/restart evidence. Founder evidence-sufficiency judgment is required
before continuing or reconstructing another disposable v4 fixture.

Cycle 3 continued manual evidence: after Founder acceptance of the combined
migration/runtime evidence, the disposable fresh-v5 profile completed Evidence
candidate creation, pending correction, confirmation, Reflection prompt
creation, first response, and response correction. Context Recovery prompt
creation also passed. Saving the first Context Recovery response appeared
inert. After normal close, read-only evidence proved the database remained
internally valid and the `recovery_turn` stayed suggested/pending with no
partial response revision. Source inspection identified the exact mismatch:
the runtime lookup uses `context_recovery`, whereas every promoted persistence
contract uses `recovery_turn`. The current sprint cannot legally enter a fourth
revision cycle.

## Final Review Status

human_decision_required
