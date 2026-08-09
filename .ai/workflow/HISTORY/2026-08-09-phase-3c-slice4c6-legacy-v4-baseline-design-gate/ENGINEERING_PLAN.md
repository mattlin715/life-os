# Engineering Plan

Status: approved

- Sprint ID: 2026-08-09-phase-3c-slice4c6-legacy-v4-baseline-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `a9bbf7cc687f0d8dcc4bda9e1d402bf309fac590`
- Working-tree digest reviewed: `10678a4018ce9cb77707b2016928149c2c8f86aa5ac9ca033ec942bf88bbcf17`
- Created at: 2026-08-08T20:32:00.000Z
- Updated at: 2026-08-08T20:32:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions`. Founder decision
`PHASE3C-SLICE4C6-001` selected Option B and authorizes Slice 4C-6A only:
private, unregistered, disposable legacy-v4-raw Evidence and Pattern candidate
confirmation/rejection. Slice 4C-6B and production v5 remain unauthorized.

## Existing Implementation Understanding

- Both writers already execute one `BEGIN IMMEDIATE` transaction, maintain a
  guarded v4 projection, inject deterministic failures, reconcile read-only
  manifests, and conservatively classify ambiguous COMMIT outcomes.
- Evidence confirm/reject has no serialization fence, but its migrated behavior
  lacks focused proof and artifact-specific strict legacy validation.
- Pattern confirm/reject explicitly refuses every non-canonical revision with
  `pattern_legacy_review_requires_later_slice`.
- Full database verifiers preserve raw legacy content/digests and projection
  equality, but action-time validation must additionally prove the exact legal
  schema-v4 object shape, IDs, provenance representation, and dependencies.
- The promoted migration core already creates honest legacy baselines and must
  remain unchanged.

## Affected Modules

- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_pattern_write.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- repository-required `.ai/workflow` artifacts/archive only

## Proposed Design

1. Add artifact-specific legacy review validators inside each existing writer;
   do not create a generic legacy mutation framework.
2. Validators accept only `legacy_v4_baseline` revision 1 with no predecessor,
   `legacy-v4-raw`, exact current source/artifact identity, candidate status,
   known field set and legal field types, exact timestamps, exact projection,
   content digest, and governed known-or-legacy-unknown provenance.
3. Evidence validates its exact source identity, kind, editability, optional
   original/provenance representation, and zero legal inbound dependencies.
4. Pattern validates exact unique Evidence/Reflection source sets, complete
   promoted dependency-verifier chains, provenance source-set equality, and
   zero legal inbound dependencies.
5. Confirmation preserves the legacy revision/content/digest/provenance and
   changes only exact review/head eligibility plus guarded v4 current state.
6. Rejection uses the existing accepted purge/tombstone/projection path and
   leaves no content-bearing artifact revision.
7. Preserve canonical behavior by branching only when serialization is
   `legacy-v4-raw`; unsupported serialization remains fail closed.
8. Add exact-v4 synthetic rows before invoking the promoted migration core, so
   every focused legacy test proves migration-produced v5 state rather than
   manually constructing normalized tables.

## Alternatives Considered

- Remove only the Pattern serialization fence: rejected because permissive raw
  JSON and unproved Evidence behavior would remain in the cutover path.
- Modify the migration to canonicalize old rows: prohibited; destroys byte
  identity and fabricates normalized history.
- Build one shared generic legacy parser: rejected; artifact-specific rules and
  future evolution would be obscured.
- Implement Slice 4C-6B simultaneously: outside Founder authority.

## Data Lifecycle Impact

Confirmation adds one exact user review event and changes current review/head
state without changing retained content. Rejection explicitly purges artifact
content and removes the guarded v4 projection while retaining only accepted
content-free facts. All operations use disposable fixtures only.

## SQLite Or Migration Impact

No DDL, migration code, production `SCHEMA_VERSION`, startup maximum, or real
database changes. Tests create schema-v4 fixture rows and pass them through the
promoted private migration core.

## Provenance Impact

Known provenance must reconcile exactly with the legacy payload and normalized
record. A migration-produced `legacy_unknown` record remains unknown. No user,
provider, model, authorship, or timestamp fact is inferred or upgraded.

## Historical Context Impact

No historical packet or provider behavior changes. Evidence rejection applies
only already-authorized exact ADR-0009 cascade behavior where a legal dependent
exists. Pattern has zero legal historical inbound relationships and fails
closed if one is present.

## Consent Impact

None. No consent is created, consumed, reused, changed, or transmitted.

## Provider Transmission Impact

None. The writers remain private and unregistered; no provider call exists.

## Import And Export Impact

None. Schema-v4 fixture insertion is test setup, not product import. Export v2
remains unauthorized.

## Test Strategy

- Migrated legacy Evidence confirm preserves raw bytes, digest, provenance,
  dependency and migration receipt; adds exact review time and eligibility.
- Migrated legacy Evidence reject purges content/projection and retains bounded
  content-free facts; exact ADR-0009 behavior is regression-tested if present.
- Migrated legacy Pattern confirm preserves raw content/provenance/dependencies
  and remains a useful-for-reflection hypothesis.
- Migrated legacy Pattern reject purges content/projection.
- Known and `legacy_unknown` provenance cases remain exact.
- Malformed JSON, unknown field, unsupported shape, duplicate IDs, orphaned or
  cross-source dependency, stale revision, projection/digest mismatch, and
  unexpected inbound relationships fail closed without mutation.
- Inject every review/purge boundary and prove exact logical rollback.
- Ambiguous COMMIT exact pre/post/third-state behavior is repeated for legacy
  operations.
- Canonical review tests remain unchanged and passing.

## Repository Verification Strategy

Run focused Rust tests for both modules, `cargo clippy --all-targets -- -D
warnings`, then `powershell -NoProfile -ExecutionPolicy Bypass -File
.\scripts\verify.ps1`. Record the final canonical evidence only after the
working-tree digest is stable.

## Manual UI Verification

Not applicable for this slice. No Tauri registration, renderer, UI, startup,
app-data, or production caller is authorized. Founder diff review follows
automated verification.

## Rollback Or Recovery Strategy

Every pre-commit error rolls back and is reconciled to the exact pre-manifest.
Ambiguous COMMIT accepts only exact pre-state or exact post-state; any third
durable state returns `recovery_required`. No retry, replay, repair, rebinding,
cleanup, or candidate selection is added.

## Documentation Impact

Update only architecture/13 with factual Slice 4C-6A implementation evidence,
remaining 4C-6B blocker, test counts, and preserved production-v4 boundary.

## ADR Impact

No new ADR and no ADR status change. This implements the exact bounded
Founder-authorized application of ADR-0007, ADR-0009, and ADR-0011.

## Risk Level

Medium. Confirmation is non-destructive, but rejection purges content and
legacy parsing mistakes could either admit malformed history or block valid
current actions. Risk is bounded by private disposable paths, strict
artifact-specific validation, exact rollback, and no runtime registration.

## Escalation Decision

No additional Founder decision is required for this plan. Stop immediately if
implementation requires any fourth product/document path, DDL/migration
change, production caller, relaxed provenance, inferred dependency, or Slice
4C-6B behavior.
