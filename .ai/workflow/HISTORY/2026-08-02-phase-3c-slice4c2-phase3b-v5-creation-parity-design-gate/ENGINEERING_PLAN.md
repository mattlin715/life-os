# Engineering Plan

Status: approved

- Sprint ID: 2026-08-02-phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: `666518eb53ba1cebf64ce87e4add1d2467c8dcbb`
- Working-tree digest reviewed: `4e961904bd049b2f7f54ceaf663fe3be7cc298f7bc912cb880949bb5a2060e1c`
- Created at: 2026-08-01T21:20:00.000Z
- Updated at: 2026-08-01T21:20:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions` after exact Founder resolution of
`PHASE3C-SLICE4C2-001` Option A. Implement only a private, unregistered,
path/connection-injected, disposable-fixture Historical Question creation-parity
writer. Preserve production schema/startup maximum 4 and every explicit
non-scope in the resolution.

## Existing Implementation Understanding

- `src-tauri/src/sqlite.rs` owns the production schema-v4 typed Historical
  Question transaction and remains unchanged.
- `src-tauri/schema/schema_v5.sql` freezes the promoted v5 DDL and remains
  unchanged.
- `schema_v5_migration.rs` creates exact migrated disposable-v5 fixtures and
  already privately hosts the promoted write modules.
- `schema_v5_experience_write.rs`, `schema_v5_evidence_write.rs`, and
  `schema_v5_reflection_write.rs` expose authoritative private exact verifiers.
- `schema_v5_evidence_lifecycle.rs` owns the promoted v4/v5 Historical Question
  cascade consequences and must be exercised rather than duplicated.
- Existing ADR-0009 v4 rows remain authoritative for consent, transmission,
  packet bytes/digest, and compatibility behavior.

## Affected Modules

- New: `src-tauri/src/schema_v5_historical_question_write.rs`
- Private module declaration only: `src-tauri/src/schema_v5_migration.rs`
- Factual evidence: `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- Repository workflow artifacts for this sprint.

No other product, schema, Cargo, TypeScript, UI, provider, or Harness file is
authorized.

## Proposed Design

Implement one request/outcome/error model plus deterministic clock, identifier,
failure-point, and commit adapters local to the new module. Construct exact-v5
fixtures through the promoted migration core in tests. The writer opens only an
injected disposable path, begins one `BEGIN IMMEDIATE`, activates the existing
compatibility guard, validates exact consent/transmission/packet and unique
source sets, reuses promoted Experience/Evidence/Reflection verification,
creates v4 and v5 representations, reconciles all governed bytes/IDs/edges,
clears the guard, and commits. Reopen read-only for exact post-state
classification. Exact duplicates are idempotent only on complete equality;
conflicts fail closed. A valid no-question request returns unchanged before a
transaction.

## Alternatives Considered

- V5-only creation rejected because current runtime and ADR-0009 authority are
  schema-v4.
- Contract-only work rejected because it cannot prove atomic rollback/cascade.
- Production activation rejected by authority and safety boundaries.
- Parallel validators/cascade logic rejected; reuse promoted verifier and
  lifecycle paths.

## Data Lifecycle Impact

Disposable fixtures only. Successful creation gains one linked v4/v5 generated
artifact. Source correction/deletion exercises the existing Slice 4C-1 cascade.
No-question/provider-failure cases create no artifact or actual-use provenance.
Unsuccessful 30-day audit retention is unchanged.

## SQLite Or Migration Impact

No DDL, migration, `SCHEMA_VERSION`, `user_version`, startup, or real-database
change. Tests operate on exact-v5 disposable databases produced through the
promoted migration core. The new writer uses one guarded transaction.

## Provenance Impact

The v4 packet bytes/digest remain authoritative. The normalized revision points
to exact consent/transmission/packet identity and uses one deterministic
provenance fingerprint for exact AI generation provenance. Actual-use
provenance is created only with the persisted successful artifact.

## Historical Context Impact

No retrieval or eligibility expansion. Only packet-declared Experience,
confirmed Evidence, and eligible answered user-authored Reflection revisions
are allowed. Exact current revisions and complete dependency chains are
revalidated.

## Consent Impact

No policy change or new consent write path. The writer accepts only exact
already-consumed consent and matching successful transmission evidence. Any
scope, destination, packet, source, or expiry mismatch fails closed.

## Provider Transmission Impact

None. No provider code or calls. A synthetic successful-transmission record is
fixture input; provider failure is modeled as an unchanged outcome.

## Import And Export Impact

None. Export v2 and lifecycle writes remain disabled outside the private test
boundary.

## Test Strategy

Focused Rust tests will cover successful exact parity; exact duplicate and
conflicting duplicate; selection/cancel/failure/no-question unchanged behavior;
provider/model/purpose/digest/consent mismatch; stale Experience, rejected
Evidence, Reflection dependency drift, malformed/duplicate/orphaned arrays;
at-least-one historical citation; whole-history/unsupported kind and Phase 4
output refusal; every meaningful write/reconciliation failure boundary;
ambiguous COMMIT exact pre/post/unknown classification; v4/v5 parity; and
post-creation Slice 4C-1 direct/transitive deletion with no successful orphan.

## Repository Verification Strategy

Run focused module tests, `cargo clippy --all-targets --all-features -- -D
warnings`, then canonical
`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`.
Record canonical verification only during workflow validation.

## Manual UI Verification

Not applicable. The module is private, unregistered, disposable-only, and has
no Tauri, renderer, UI, startup, app-data, or real-user caller. Founder review
is diff/evidence review after archive.

## Rollback Or Recovery Strategy

All pre-commit failures roll back the complete logical transaction. A generic
COMMIT error is classified only after closing writable access and reopening
read-only: exact pre-state, exact post-state, or `recovery_required`. No retry,
replay, rollback-after-unknown, repair, cleanup, or candidate selection.

## Documentation Impact

Update architecture/13 only with factual Slice 4C-1 promotion and, after
verification, bounded Slice 4C-2 disposable evidence. No Book Zero rewrite.

## ADR Impact

No new ADR or ADR status change. This implements a bounded test-only portion of
accepted ADR-0009/ADR-0011 and Founder-approved architecture/13.

## Risk Level

Medium. Consent-bound dual representation and cascades are consequential, but
risk is bounded by disposable fixtures, no runtime registration, exact
transaction/reconciliation, promoted verifier reuse, and deterministic failure
injection.

## Escalation Decision

No additional Founder decision is required inside the exact Option A boundary.
Escalate and stop if implementation requires any file outside the allowlist,
policy reinterpretation, schema DDL change, runtime integration, source
eligibility change, or retention/deletion change.
