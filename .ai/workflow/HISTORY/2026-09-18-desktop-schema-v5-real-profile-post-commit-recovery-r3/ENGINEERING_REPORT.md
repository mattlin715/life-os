# Engineering Report

Status: completed

- Sprint ID: 2026-09-18-desktop-schema-v5-real-profile-post-commit-recovery-r3
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 4746aed9e7cc7a6a01ddc3a39eb0db2bd0fde3df
- Working-tree digest implemented: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-19T21:51:15.3727707Z
- Updated at: 2026-09-19T21:51:15.3727707Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, or `failed`.

## Implementation Summary

Completed the exact real-profile R3 recovery and Founder manual review sequence under separate gates. The only product-code correction changed `legacy-v4-raw` Pattern provenance compatibility so incomplete historical `sourceArtifactIds` are accepted only when immutable raw and canonical provenance agree, while exact normalized dependency rows continue to cover every declared Evidence and Reflection source. The correction is committed locally as `4746aed9e7cc7a6a01ddc3a39eb0db2bd0fde3df` (`fix(storage): preserve legacy pattern provenance`). This report is a truthful closeout reconciliation and does not claim the workflow plan/report preceded the already completed gated work.

## Existing System Areas Inspected

- `src-tauri/src/schema_v5_pattern_write.rs` and colocated tests.
- `src-tauri/src/schema_v5_prepared_recovery.rs` recovery/retained-staging contract.
- `src/app/i18n.ts` three-language recovery and retained-backup wording.
- `scripts/verify.ps1` and `scripts/ai-workflow.mjs` verification/control-plane paths.
- Current `.ai/workflow` artifacts and exact Git state.

## Files Added

None in tracked product code. The ignored corrective package directory was created only under separate authority and is not part of the Git diff. The future workflow archive directory has not been created because archive execution remains separately gated.

## Files Modified

- Product commit: `src-tauri/src/schema_v5_pattern_write.rs` only (`201` insertions, `21` deletions in commit `4746aed`).
- Current unstaged closeout evidence: current `.ai/workflow` Markdown, JSON state, and event journal only.

## Files Deleted

None.

## Behavior Changed

`legacy-v4-raw` Patterns may retain incomplete historical provenance source arrays when raw and canonical arrays are identical. A compatibility write still fails closed if raw/canonical sources drift, any non-source provenance field drifts, or normalized dependencies fail to match every declared source. `canonical-json-v1` behavior is unchanged.

## Data Model Impact

None. No table, column, index, constraint, relationship type, projection schema, or serialized canonical schema changed.

## Migration Impact

No migration rerun or migration-code change. The exact post-commit recovery enabled already committed lifecycle writes without restore, retry, repair, checkpoint, schema decrement, or backup deletion.

## Provenance Impact

Historical raw and canonical provenance bytes remain immutable. Source-array incompleteness is tolerated only for `legacy-v4-raw` and only with raw/canonical equality. All non-source fields and normalized dependency coverage remain strict.

## Historical Context Impact

None. Local history, Phase 3B proposal/consent, provider packet construction, ranking, and relevance behavior are unchanged.

## Consent Impact

No product-consent semantics changed. Real-profile access, recovery, installation, launches, synthetic writes, close/restart actions, inspections, and every Founder manual step were separately authorized.

## Provider Transmission Impact

None. No provider transmission occurred or was authorized.

## Tests Added

Colocated Rust coverage in `schema_v5_pattern_write.rs` for:

- matching incomplete historical provenance accepted without rewriting bytes;
- raw/canonical source drift rejected;
- non-source provenance drift rejected;
- normalized dependency drift rejected.

## Tests Executed

- Three new focused tests: passed individually.
- Complete Pattern-write module: `25/25` passed.
- Pre-promotion focused and full canonical verification: passed.
- Post-commit `powershell -ExecutionPolicy Bypass -File scripts/verify.ps1`: exit `0`; workflow checks, `373` frontend tests, `225` primary Rust tests, backup/contract suites, schema-v4 refusal, ordinary/Founder schema-v5 activation, Founder runtime, formatting, UTF-8, secret, link, and Constitution checks passed.
- Exact package builder once with suffix `r3-legacy-pattern-provenance`: exit `0`; installer/manifest contract passed.
- A fresh closeout canonical verification is intentionally deferred to the next workflow validation phase and will be recorded separately.

## Verification Results

Passed: code-focused tests, full canonical verification before and after the corrective commit, package contract, deterministic NSIS payload diagnosis, bounded installation checks, exact Step 6 corrective save, and Founder Manual Review Steps 1–16.

Initially failed then corrected: Step 6 reported `pattern_provenance_sources_mismatch`. The first installed-hash expectation used the post-build release hash; immutable diagnosis proved the installed binary exactly matched the deterministic NSIS payload and differed from release only by the three-byte Tauri NSIS bundle marker.

Skipped under current authority: workflow archive execution, Git staging/commit for workflow closeout, push, PR, deployment, distribution, release, Phase 4, and Android.

## Manual Verification Required

Completed by the Founder. Exact reports passed Steps 1–16: startup/restart, old Experience/Reflection reconstruction, lifecycle-write controls, retained backup, synthetic Experience and Daily Reflection persistence, normal closes and zero-process boundaries, duplicate-lifecycle absence, expected retained zero-byte staging, three-language wording, and keyboard/focus/narrow-window/long-value layout.

## Documentation Updates

Current workflow evidence is being reconciled factually. No product, architecture, migration, or ADR document required a semantic update. The Harness archive remains prospective until a separate gate.

## ADR Impact

No new or modified ADR. The implementation preserves ADR-0004 local ownership, ADR-0007 exact provenance, ADR-0008 explicit Harness gates, and ADR-0009 historical-context consent separation.

## Deviations From Plan

The multi-role Engineering Plan and Report were reconciled after the gated implementation during explicitly authorized closeout preparation rather than preceding implementation. This timing deviation is recorded rather than hidden. Product/code scope itself remained exact: one product file and colocated tests.

## Known Limitations

The exact corrective commit and current workflow closeout are local only. `origin/develop` remains at `b0e68dfc2f743f6c3d7960d22057458a32ea4cb1` and does not contain `4746aed`. The application remained open after final Step 16 because close/restart was not authorized. Workflow archive and closeout commit remain separate decisions.

## Remaining Risks

Only governance/publication risks remain: archive correctness, exact workflow path set, local closeout commit, and later remote publication must be separately reviewed and authorized. No unresolved manual-review product defect is known.

## Git State

- Branch: `develop`.
- HEAD: `4746aed9e7cc7a6a01ddc3a39eb0db2bd0fde3df`.
- `origin/develop`: `b0e68dfc2f743f6c3d7960d22057458a32ea4cb1`; local `develop` is one corrective product commit ahead.
- Staged paths: none.
- Untracked paths: none at report creation.
- Working tree: current workflow artifacts only, unstaged; product code is clean at HEAD.

## Engineer Completion Status

`completed`. The bounded implementation and evidence reconciliation are complete. Proceed to independent canonical validation; do not archive, stage, commit, or publish without the separate gate.
