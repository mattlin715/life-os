# Engineering Plan

Status: approved

- Sprint ID: 2026-08-10-phase-3c-post-slice4-production-readiness-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `c28f5872f321ef0ad2f54f7952fc76f3c5e0be61`
- Working-tree digest reviewed: `26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e`
- Created at: 2026-08-09T18:55:00.000Z
- Updated at: 2026-08-09T18:55:00.000Z

## Approved Product Boundary

The Product Review is `approved_with_conditions`. This sprint may perform only
the repository audit, factual architecture/13 synchronization, one Proposed
architecture/15 gate, minimal Index navigation, workflow evidence, and
verification. It must not implement any recommended runtime slice and must
stop at `PHASE3C-PRODUCTION-READINESS-R1-001`.

## Existing Implementation Understanding

- Production schema is v4 in `src-tauri/src/sqlite.rs`; startup performs a
  read-only presence/version check, blocks newer schemas, then opens writable
  initialization and typed v4 commands.
- The renderer loads a plugin-SQL database for reads while Rust mutation
  commands open their own connections. A future migration needs an explicit
  pre-store quiescence boundary; current runtime operation is not that proof.
- `filesystem_safety.rs` and `schema_v5_migration.rs` are compiled but private
  and unreachable from the Tauri handler. Their promoted tests use only
  synthetic/disposable paths and fixtures.
- Private schema-v5 writers cover current canonical and migrated legacy action
  parity but have no production v5 reader, command routing, startup, or app-data
  caller.
- Production UI currently discloses only checking, newer-schema,
  inspection-failed, and initialization-failed startup states. It has no v5
  upgrade disclosure or controls.

## Affected Modules

Current sprint anticipated allowlist:

1. `.ai/workflow/CURRENT_MISSION.md`
2. `.ai/workflow/PRODUCT_REVIEW.md`
3. `.ai/workflow/ENGINEERING_PLAN.md`
4. `.ai/workflow/ENGINEERING_REPORT.md`
5. `.ai/workflow/THEORY_ALIGNMENT_REVIEW.md`
6. `.ai/workflow/DECISION_REQUIRED.md`
7. `.ai/workflow/SPRINT_REPORT.md`
8. `.ai/workflow/WORKFLOW_STATE.json`
9. `.ai/workflow/EVENTS.jsonl`
10. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
11. `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`
12. `docs/00_Index.md`

The workflow contract, templates, roles, scripts, production source, tests,
DDL, and ADRs are excluded.

## Proposed Design

1. Reconcile architecture/13 with the promoted Slice 4C-6B Git evidence.
2. Create architecture/15 as a Proposed Book One authorization gate containing
   the 24-row matrix, Slice 5/6 reconciliation, threat model, alternatives,
   Option A recommendation, exact future allowlist, and acceptance matrices.
3. Add only a navigation entry in the Index.
4. Independently compare all tracked and non-ignored untracked changes to this
   allowlist and verify no production diff.
5. Complete Engineering Report, canonical verification, Theory Alignment
   Review, and the exact Founder decision package.

## Alternatives Considered

- Modify architecture/13 only: rejected because the requested matrix and next
  authorization package need a clear Proposed gate without overloading the
  already Founder-approved migration plan.
- Add an ADR: rejected because no new irreversible policy is proposed.
- Implement Option A now: prohibited; the prompt explicitly requires a Founder
  gate first.
- Proceed directly to fresh-v5 or v4 migration: rejected as too broad and able
  to create or mutate production data.

## Data Lifecycle Impact

None. Documentation and workflow metadata only; no Life OS record is read,
written, copied, migrated, retained, or deleted.

## SQLite Or Migration Impact

None. `SCHEMA_VERSION`, `user_version`, DDL, fixtures, startup, connections,
backup, restore, and retention code remain unchanged.

## Provenance Impact

None to runtime provenance. The audit records evidence locations and preserves
the distinction between private fixture proof and production authority.

## Historical Context Impact

None. ADR-0009 behavior is audited but not changed.

## Consent Impact

None. Database readiness disclosure is explicitly separated from historical
provider consent and migration authorization.

## Provider Transmission Impact

None. No provider or ContextPacket file may change.

## Import And Export Impact

None. Experience-only export remains current; export v2 stays deferred.

## Test Strategy

- Inspect exact source locations and promoted test coverage for every matrix
  row.
- Run `git diff --check`, workflow validation, and the canonical verifier.
- Confirm no Constitution, production source, schema, DDL, provider, or
  ContextPacket diff.
- Validate architecture/15 local links through the canonical link check.

## Repository Verification Strategy

Run:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

Record the exact test counts and exit code with `scripts/ai-workflow.mjs`.

## Manual UI Verification

Not applicable to this documentation-only audit. The Founder reviews the diff
and decision package. The proposed future Option A includes a separate
three-language desktop manual matrix.

## Rollback Or Recovery Strategy

No runtime rollback exists because no runtime state changes. Before promotion,
the uncommitted documentation can be reviewed file by file; no file will be
discarded automatically. If verification fails, correct only the responsible
document within at most three bounded review cycles.

## Documentation Impact

- architecture/13: factual promotion record only.
- architecture/15: readiness gate, updated to Founder-approved only after an
  exact recorded Founder resolution.
- docs/00_Index.md: minimal navigation only.
- Book Zero and accepted ADRs: unchanged.

## ADR Impact

No new ADR. Existing ADR-0007, ADR-0009, ADR-0011, and Founder-approved
architecture/13 already govern all reviewed policy boundaries.

## Risk Level

Medium. The changes are documentation-only, but an inaccurate readiness claim
could authorize unsafe production work. Risk is controlled by exact repository
evidence, explicit state vocabulary, no implementation, and Founder review.

## Escalation Decision

Proceed with the bounded documentation audit and stop at
`human_decision_required` for `PHASE3C-PRODUCTION-READINESS-R1-001`. After the
exact Founder resolution, perform one factual-only revision cycle to record the
Founder-approved state and Slice 5 clarification, reverify, archive/reset, and
stop at Founder diff review. No product implementation or Git promotion
authority is inferred.

## Revision Cycle 1 Plan

- Record the exact Founder Option A resolution through the repository workflow.
- Change architecture/15 from Proposed to Founder-approved without changing its
  approved scope, allowlist, or acceptance matrices.
- Synchronize architecture/13's factual distinction between substantially
  implemented private Slice 5 contract evidence and unstarted production
  integration/manual gates.
- Run canonical verification again and complete Theory Alignment Review.
- Do not edit product code or start the authorized implementation slice.
