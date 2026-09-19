# Engineering Plan

Status: approved

- Sprint ID: 2026-09-18-desktop-schema-v5-real-profile-post-commit-recovery-r3
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 4746aed9e7cc7a6a01ddc3a39eb0db2bd0fde3df
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-19T21:50:13.2620137Z
- Updated at: 2026-09-19T21:50:13.2620137Z

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved`. The accepted boundary is the exact ordinary-desktop schema-v5 R3 sequence already governed by explicit Founder gates: immutable technical classification, one exact post-commit recovery, the minimum legacy-v4-raw pattern-provenance compatibility correction, exact package/install diagnosis, and Founder Manual Review Steps 1–16. This closeout reconciliation does not authorize new product behavior, real-profile access, application interaction, archive execution, Git promotion, publication, Phase 4, or Android.

This plan is written during explicitly authorized factual closeout reconciliation and does not claim it preceded the already completed gated implementation.

## Existing Implementation Understanding

`src-tauri/src/schema_v5_pattern_write.rs` verifies schema-v5 Pattern projections and gated compatibility writes. Historical `legacy-v4-raw` records may legitimately carry incomplete `sourceArtifactIds` in both immutable raw and canonical provenance, while normalized dependency rows remain the authoritative complete binding to every declared Evidence and Reflection source. `canonical-json-v1` provenance remains strict. The exact recovery path and content-free evidence are governed by `src-tauri/src/schema_v5_prepared_recovery.rs` and the promoted ordinary desktop package contract.

## Affected Modules

- Product implementation and focused Rust tests: `src-tauri/src/schema_v5_pattern_write.rs`.
- Current control-plane evidence only during closeout: `.ai/workflow/CURRENT_MISSION.md`, `PRODUCT_REVIEW.md`, `ENGINEERING_PLAN.md`, `ENGINEERING_REPORT.md`, `THEORY_ALIGNMENT_REVIEW.md`, `DECISION_REQUIRED.md`, `SPRINT_REPORT.md`, `WORKFLOW_STATE.json`, and `EVENTS.jsonl`.
- Prospective archive destination, not yet created: `.ai/workflow/HISTORY/2026-09-18-desktop-schema-v5-real-profile-post-commit-recovery-r3/`.

## Proposed Design

1. Preserve historical raw and canonical provenance bytes exactly.
2. For `legacy-v4-raw` only, require raw and canonical `sourceArtifactIds` to agree with each other while allowing the shared historical set to remain incomplete.
3. Independently require normalized dependency rows to match all declared Evidence and Reflection sources exactly.
4. Keep every non-source provenance field exact and keep `canonical-json-v1` unchanged and strict.
5. Add positive incomplete-provenance coverage and neighboring fail-closed source/dependency/non-source drift coverage.
6. Reconcile the factual repository, package, installation, recovery, diagnosis, and Founder Manual Review evidence without adding product changes.
7. Run focused workflow validation and the canonical `powershell -ExecutionPolicy Bypass -File scripts/verify.ps1` path, then stop before workflow archive execution, staging, or commit.

## Alternatives Considered

- Rewrite or canonicalize historical provenance: rejected because it would destroy immutable historical evidence.
- Trust incomplete provenance without normalized dependencies: rejected because it would weaken the exact source relationship.
- Relax `canonical-json-v1`: rejected as unnecessary and unsafe.
- Change schema-v5 DDL, schema version, migration, receipt, or backup semantics: rejected as outside the diagnosed failure and Founder authorization.
- Repair the real profile directly: rejected; the correction belongs in repository verification/write compatibility and was installed only through separately authorized exact packaging gates.

## Data Lifecycle Impact

The compatibility correction permits an already valid legacy Pattern lifecycle write only when immutable raw/canonical provenance agrees and normalized dependencies exactly cover declared sources. It does not create a new lifecycle stage. The one synthetic Experience and Reflection used in Founder review were created only under explicit manual gates; no additional lifecycle action is authorized during closeout.

## SQLite Or Migration Impact

No DDL, schema version, migration algorithm, retry behavior, receipt format, backup behavior, checkpoint, restore, schema decrement, or backup deletion changed. The correction changes only verification of an exact historical representation before an authorized compatibility write.

## Provenance Impact

Historical raw and canonical bytes remain preserved. All non-source fields remain exact. Raw and canonical source arrays must agree. Complete source truth remains bound through exact normalized dependency rows. `canonical-json-v1` remains strict.

## Historical Context Impact

None. The correction and manual review do not alter local-history selection, ranking, consent, provider packets, or Phase 3B behavior.

## Consent Impact

No consent boundary changes. Every application/profile/recovery/install/manual action remained separately authorized and non-transferable. The current closeout authority is repository-only.

## Provider Transmission Impact

None. No provider transmission was authorized or performed by the recovery, correction, manual review, or closeout preparation.

## Import And Export Impact

None. Import/export behavior and data formats are unchanged; Founder Step 16 explicitly avoided activating those controls.

## Test Strategy

- Focused Rust unit coverage for incomplete matching legacy provenance.
- Fail-closed coverage for raw/canonical source drift, non-source field drift, and normalized dependency drift.
- Existing pattern-write and exact schema-v5 verification suites.
- Workflow validation after every state/artifact reconciliation.
- Full canonical repository verification before terminal closeout preparation is reported.

## Repository Verification Strategy

Run `node scripts/ai-workflow.mjs validate` after reconciliations. Run `powershell -ExecutionPolicy Bypass -File scripts/verify.ps1` exactly once for this closeout preparation, record its actual exit code and repository coordinates through the workflow CLI, then revalidate state. Inspect the final unstaged path set, diff statistics, branch, HEAD, staged set, and remote comparison without staging or publication.

## Manual UI Verification

Owner: Founder. Steps 1–16 passed by exact Founder reports, including ordinary startup/restart, prior Experience and Reflection reconstruction, lifecycle writes, retained backup disclosure, synthetic Experience and Daily Reflection persistence, normal close/process-zero checks, duplicate-lifecycle absence, expected retained zero-byte staging classification, three-language technical wording, and keyboard/focus/narrow-layout behavior.

## Rollback Or Recovery Strategy

No automatic rollback, profile repair, restore, retry, deletion, or schema action is allowed. The verified schema-v4 backup and retained recovery evidence remain governed by explicit future gates. A future code rollback would require a separate repository decision and cannot rewrite real-profile history already accepted by Founder review.

## Documentation Impact

Only current workflow artifacts and the eventual Harness-generated archive need factual synchronization. Product, architecture, ADR, migration, and user documentation require no additional change because behavior stayed inside the already documented schema-v5 compatibility and recovery boundaries.

## ADR Impact

No new or modified ADR. ADR-0004, ADR-0007, ADR-0008, and ADR-0009 remain satisfied without reinterpretation.

## Risk Level

High for the overall sprint because it included exact real-profile recovery and manual profile mutation; those actions are complete and separately evidenced. Low for this closeout preparation because it is repository-only, workflow-only, unstaged, and uncommitted.

## Escalation Decision

No unresolved product or engineering ambiguity remains. Founder Manual Review Steps 1–16 are complete. Stop after verified closeout preparation for a separate workflow archive and local commit decision; do not infer archive, commit, push, PR, deployment, release, Phase 4, or Android authority.
