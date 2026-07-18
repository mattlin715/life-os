# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-18-pilot-4-schema-v5-slice-0
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: b781071fbf726cde69e93cb9cd98c74abdff0ba3
- Working-tree digest reviewed: 527acca8dc06433f4206f5435337fed0afd48ded23fd3c65956081c3ad304849
- Created at: 2026-07-18T10:26:47.7445382Z
- Updated at: 2026-07-18T10:26:47.7445382Z

## Mission Interpretation

This is a recovery audit of already-existing, explicitly authorized Phase 3C
Slice 0 test work. The workflow must preserve that chronology: implementation
predated intake, and the repository-mediated phases now review, correct, and
validate it rather than pretending to have authorized it retroactively.

## Problem Statement

The existing diff freezes the Founder-approved schema-v5 candidate as test-only
contracts and fixtures, but workflow state remained idle and remote CI still
filters Rust tests to `sqlite::tests`. Without recovery evidence and CI parity,
the Slice 0 proof is incomplete even though local verification passes.

## User Value

Slice 0 does not create a visible feature. It reduces the risk that later local
database lifecycle work corrupts, invents, or silently loses user history by
freezing the candidate contract before any production database can be touched.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI and the source-of-truth hierarchy
  prevent a test contract from granting itself production authority.
- `docs/03_Principles.md`: evidence precedes conclusion; passing tests are
  evidence for a bounded contract, not proof of production migration safety.
- `docs/06_Memory.md`: provenance, correction, deletion, and user control must
  remain distinguishable and reversible where policy permits.
- `docs/Reflection.md`: AI-generated artifacts remain revisable hypotheses and
  are not identity truth.
- `docs/appendix/Harness.md`: the Product Harness and Engineering Harness remain
  separate; Pilot 4 evidence cannot self-approve Stage 1 reliability.

No primary definition changes.

## Relevant ADRs

- `ADR-0007` requires durable reviewed artifacts with provenance, correction,
  deletion, and export distinctions.
- `ADR-0009` remains authoritative for Historical Question consent,
  transmission, actual-use provenance, and deletion cascades.
- `ADR-0011` is Accepted and approves the append-only lifecycle policy while
  explicitly withholding production migration authority.
- `ADR-0008` governs the repository-native workflow only; Pilot 4 does not
  change its Accepted decision or authorize Stage 2/3.

## Current Implementation Context

- architecture/13 is Founder-approved and promoted at merge commit `b781071`.
- Its Decision 16B authorizes fixed contracts, synthetic fixtures, test-only
  DDL execution, invariant tests, and factual synchronization only.
- Before workflow intake, the working tree already contained modifications to
  `scripts/verify.ps1`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and six
  untracked files under `src-tauri/tests/`.
- The candidate DDL is byte-aligned with architecture/13 and has digest
  `396c06634ab871f892be36280468726cda1777b6bdf18039ea165d6b54e126dc`.
- The full schema-object manifest digest is
  `bc92f25e12f8a6829c152d4b51fcf741872d94724afaccdb2236d399f5b8abe8`.
- Local canonical verification passed before intake with 150 Vitest tests,
  eight existing Rust SQLite tests, and six Slice 0 integration tests.
- `.github/workflows/check.yml` still runs only `cargo test ... sqlite::tests`,
  so remote CI does not yet execute the six Slice 0 tests.
- Production `src-tauri/src/sqlite.rs` still declares `SCHEMA_VERSION = 4` and
  has no diff.

## In Scope

1. Preserve and independently review the existing Slice 0 diff.
2. Change remote CI to run the same complete Rust test suite as `verify.ps1`.
3. Revalidate architecture/13 byte alignment, v2/v3/v4 fixtures,
   canonicalization, deterministic IDs, immutability, guards, deletion cascade,
   `foreign_key_check`, and `integrity_check`.
4. Add only regression coverage demonstrably missing from the approved Slice 0
   invariant boundary.
5. Synchronize architecture/13 factual validation evidence without changing
   its Founder-approved decisions or production authority.
6. Complete Pilot 4 workflow artifacts, verification, theory review,
   Sprint Report, event chain, archive/reset, and Founder diff review.

## Out Of Scope

- Production DDL, migration functions, user-database access, schema activation,
  startup, backup, restore, retention, application-version activation, or old
  binary handling.
- React, TypeScript product storage, UI, import/export activation, providers,
  ContextPacket, historical transmission, Slice 1+, or Phase 4.
- Constitution, Book Zero, ADR status, or Founder-decision changes.
- New Orchestration Harness features, Stage 1 Exit Audit, Stage 2, Stage 3,
  autonomous repair, replay, commit, push, merge, or deployment.

## Product Constraints

The fixed DDL is a candidate contract executed only against in-memory synthetic
fixtures. Passing Slice 0 is a prerequisite for a future Founder checkpoint; it
must not be described as a production migration, completed schema-v5 cutover,
or permission to mutate user data.

## Evidence And Provenance Constraints

Every frozen object name/body, digest, canonicalization vector, deterministic
ID vector, and trigger error code must be reproducible from repository files.
Legacy fixtures may identify uncertain provenance as `legacy_unknown`; they may
not manufacture prior revisions, rejection history, or exact action times.

## Historical Context Constraints

Only synthetic Historical Question fixtures are used. ADR-0009 deletion
cascade compatibility may be tested locally; no historical Context Packet is
assembled from real user data and nothing is transmitted.

## Consent Constraints

No product-consent behavior changes. Existing synthetic consent rows exist only
to verify referential and deletion behavior. Founder workflow approval is not
provider-use consent.

## AI-Role Constraints

The AI may audit and test fixed contracts. It may not infer identity, diagnose,
perform Phase 4 reflection, or treat database structure as authority over user
meaning.

## Privacy Constraints

Fixtures must remain synthetic and contain no journals, runtime database,
credentials, provider payloads, or raw provider errors. No test may open the
production application database.

## User-Agency Constraints

The founder retains the next authority gate. A passing test suite, completed
workflow, or Pilot archive does not authorize promotion or production work.

## Approved Conditions

1. Record explicitly that Slice 0 implementation existed before workflow
   intake; do not fabricate a plan-before-code chronology.
2. Preserve every pre-intake file unless a bounded correction is supported by
   architecture/13 or CI parity evidence.
3. Synchronize `.github/workflows/check.yml` to execute all Rust tests.
4. Inspect the architecture/13 automated matrix for missing Slice 0-only
   invariants; add tests only when no existing test already proves the case.
5. Factual documentation must distinguish working-diff evidence, promoted
   implementation, and production authority.
6. No prohibited production or later-slice file may change.
7. `WORKFLOW_EVALUATION.md` remains unchanged; formal Stage 1 evaluation is a
   separate sprint.
8. Stop after archive/reset at Founder diff review with no staged files.

## Acceptance Criteria

1. Git evidence preserves the initial branch, HEAD, unstaged diff, and untracked
   files; no pre-existing Slice 0 work is discarded or rewritten without cause.
2. Candidate DDL exactly matches architecture/13 and all 17 tables, 12 indexes,
   55 triggers, names, bodies, and digests are fixed.
3. Synthetic v2/v3/v4 fixtures remain deterministic; v2/v3 cannot directly
   apply the v5 candidate and v4 can, without changing production code.
4. Tests cover canonicalization, deterministic IDs, current-content
   requirements, immutable records, all eighteen compatibility guards,
   transaction guard cleanup, ADR-0009 cascade, foreign keys, and integrity.
5. CI and canonical local verification both run the six Slice 0 integration
   tests and all checks pass.
6. Production `SCHEMA_VERSION` and production `user_version` remain v4; no
   production database path is opened or changed.
7. Constitution, UI, startup, backup, restore, import, provider, ContextPacket,
   Slice 1+, and Phase 4 have no diff.
8. architecture/13 factual evidence is synchronized without changing its
   Founder-approved decision package or granting production authority.
9. Workflow validation, artifact reconciliation, terminal archive/reset, and
   event-chain evidence pass.
10. No stage, commit, push, merge, deployment, or Stage 1 reliability approval
    occurs.

## Risks

- The pre-intake chronology cannot demonstrate that Product Review prevented
  the original implementation; this sprint can only audit and truthfully record
  recovery.
- SQLite object-body snapshots can drift across engine versions; fixed source
  DDL and repository CI provide the bounded reproducibility evidence, but
  production migration remains unproven.
- Synthetic fixtures cannot prove backup, restart, large-database performance,
  or real user-data cutover safety.
- Expanding tests into Slice 1+ would silently broaden authority; the automated
  matrix must be filtered through the Slice 0 boundary.
- Updating CI from filtered to complete Rust tests may expose unrelated future
  integration failures; this is desirable parity, not permission to weaken
  tests.

## Open Questions

None inside the authorized recovery boundary.

## Human Decision Required

No. The founder explicitly authorized this recovery audit and corrective scope.
Promotion and every later slice remain separate future decisions.

## Recommendation

Proceed to Engineering Planning under the eight conditions. Treat the existing
implementation as input evidence, make the minimal CI and missing-regression
corrections, validate, archive/reset, and stop at Founder diff review.

## Review Status

approved_with_conditions
