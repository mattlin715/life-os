# Decision Required

Status: resolved
- Sprint ID: 2026-07-19-phase-3c-slice-1b2-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-18T20:32:56.4978112Z
- Updated at: 2026-07-18T20:32:56.4978112Z

## Decision ID

`PHASE3C-SLICE1B2-001`

## Sprint ID

`2026-07-19-phase-3c-slice-1b2-gate`

## Decision Summary

Decide whether to replace the six remaining renderer-authored schema-v4
mutation paths with typed Rust commands now, authorize only artifact-bundle
replacement, or defer all Slice 1B-2 implementation.

## Why Automation Stopped

Architecture/13 explicitly withheld Slice 1B-2. These paths govern reviewed
artifacts, exact historical consent, transmission outcomes, actual-use
provenance, deletion cascades, and audit expiry. Repository evidence can define
and recommend a boundary, but it cannot grant production implementation
authority.

## Relevant Constitution Clauses

- Human before AI: the Founder decides consequential authority.
- Evidence before Conclusion: passing tests and promoted Slice 1B-1 do not imply
  Slice 1B-2 approval.
- Privacy before Profit: historical governance and local data cannot be weakened
  for speed.
- Documentation is truth: promoted, authorized, and implemented states remain
  distinct.

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/06_Memory.md`: user-controlled, revisable, deletable memory with
  provenance.
- `docs/Reflection.md`: user-owned interpretation.
- `docs/09_AI.md`: AI remains a mirror, not an authority.
- `docs/10_Privacy.md`: local control, transparency, and consent.
- `docs/appendix/Harness.md`: provider-independent behavior and governed
  evaluation.

## Relevant ADRs

- ADR-0007: reviewed artifact persistence and provenance.
- ADR-0008: tool-independent Engineering Harness; unchanged.
- ADR-0009: exact consent, transport, stale refusal, actual-use provenance, and
  deletion guarantees.
- ADR-0011: lifecycle direction Accepted; schema-v5 production work remains
  separately gated.

## Available Options

### Option A - Full bounded Slice 1B-2 (recommended)

Implement typed Rust commands and typed TypeScript adapters for:

1. whole-bundle `saveArtifacts`;
2. `saveHistoricalConsent`;
3. `saveHistoricalTransmission` plus atomic consent consumption;
4. `saveHistoricalQuestionArtifact` plus exact dependency persistence and all
   existing persistence-time revalidation;
5. `deleteHistoricalQuestionArtifact`;
6. `purgeExpiredHistoricalAuditRecords`.

Remove generic renderer statement-array commands from the Tauri command surface
after all callers are gone. Keep schema v4 and all product behavior unchanged.

### Option B - Slice 1B-2A artifact bundle only

Implement only typed `saveArtifacts`, its adapter, stale/failure/cascade tests,
and factual documentation. Leave consent, transmission, Historical Question,
audit cleanup, and generic historical transaction paths unchanged for a later
Founder gate.

### Option C - Defer

Make only the factual Slice 1B-1 promotion correction. Do not modify production
code.

## Benefits

- **Option A:** closes the complete schema-v4 mutation trust boundary in one
  coherent sprint and removes generic renderer mutation SQL as an exposed
  bypass.
- **Option B:** lowers immediate implementation and review surface around
  sensitive ADR-0009 paths.
- **Option C:** introduces no new production regression risk.

## Risks

- **Option A:** largest regression surface; incorrect transaction ordering could
  weaken consent consumption, stale response rejection, or deletion cascades.
- **Option B:** leaves the most sensitive historical mutation paths and generic
  command surface split across renderer and Rust, requiring another gate.
- **Option C:** preserves the current renderer-authored mutation trust boundary
  and delays the prerequisite for safe schema-v5 guarded writes.

## Reversibility

All options preserve schema v4 and create no migration. Option A or B can be
reverted as application code before schema-v5 activation, but any rollback must
retain compatibility with records already validly written under current v4
semantics. No destructive downgrade or data rewrite is authorized.

## Data And Privacy Impact

Option A or B changes only which local layer owns existing SQL. It does not add
provider transmission, broaden historical eligibility, reuse consent, change
retention policy, or inspect a real user database. Tests must use synthetic or
disposable fixtures.

## Orchestrator Recommendation

Choose **Option A**. Treat the work as five capacity-resilient micro-blocks with
repository checkpoints, not five new Founder gates: artifact bundle;
consent/transmission; Historical Question create/delete; audit cleanup and
generic-command removal; final integrated verification. Stop at Founder diff
review before Git promotion.

## Default Safe Action

If the Founder does not provide an exact response, remain at
`human_decision_required` and do not implement Slice 1B-2.

## Blocked Files Or Phases

Blocked pending resolution: production edits to
`src/shared/storage/sqlite/sqliteLocalEvidenceStore.ts`, its adapter tests,
`src-tauri/src/sqlite.rs`, `src-tauri/src/lib.rs`, and any associated factual
implementation documentation. Schema v5, migration, later slices, Phase 4,
Harness expansion, Git promotion, PR, and deployment remain blocked regardless
of the selected Slice 1B-2 option.

## Exact Founder Response Needed

To authorize the recommendation, reply exactly or equivalently:

> I resolve PHASE3C-SLICE1B2-001 by selecting Option A. I authorize Phase 3C
> Slice 1B-2 only: typed Rust commands and typed TypeScript adapters for
> saveArtifacts, historical consent, historical transmission with atomic
> consent consumption, Historical Question persistence with all current
> ADR-0009 revalidation and exact dependencies, Historical Question deletion,
> and expired historical audit cleanup; removal of generic renderer mutation
> SQL and generic statement-array Tauri command exposure after callers are
> removed; preservation of schema v4 and current product, consent, retention,
> deletion, provenance, and provider behavior; synthetic/disposable tests;
> factual documentation; canonical verification; Theory Alignment Review;
> archive/reset; and stop at Founder diff review. I do not authorize schema v5,
> user_version 5, migration, real user-database testing, backup, restore,
> retention-policy changes, lifecycle UI, Slices 2-6, Phase 4, provider or
> ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit,
> push, merge, PR, or deployment.

For Option B or C, name the option and explicitly state the authorized scope.
Silence never resolves the decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE1B2-001 by selecting Option A. I authorize Phase 3C Slice 1B-2 only: typed Rust commands and typed TypeScript adapters for saveArtifacts, historical consent, historical transmission with atomic consent consumption, Historical Question persistence with all current ADR-0009 revalidation and exact dependencies, Historical Question deletion, and expired historical audit cleanup; removal of generic renderer mutation SQL and generic statement-array Tauri command exposure after callers are removed; preservation of schema v4 and current product, consent, retention, deletion, provenance, and provider behavior; synthetic/disposable tests; factual documentation; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I do not authorize schema v5, user_version 5, migration, real user-database testing, backup, restore, retention-policy changes, lifecycle UI, Slices 2-6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Slice 1B-2 only: typed Rust and typed TypeScript mutation boundaries for artifact save, historical consent, transmission and atomic consent consumption, Historical Question save/delete, expired audit cleanup, generic renderer mutation SQL command removal, schema-v4 behavior preservation, synthetic tests, factual docs, verification, theory review, archive/reset, and stop at Founder diff review; all explicitly excluded scopes remain unauthorized.

## Decided At And Evidence Reference

- Decided at: 2026-07-18T20:44:11.068Z
- Evidence reference: .ai/workflow/DECISION_REQUIRED.md#PHASE3C-SLICE1B2-001

## Resume Phase

product_review
