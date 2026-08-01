# Decision Required

Status: resolved
- Sprint ID: 2026-08-02-phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-01T20:57:02.587Z
- Updated at: 2026-08-01T20:57:02.587Z

## Decision ID

`PHASE3C-SLICE4C2-001`

## Sprint ID

`2026-08-02-phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate`

## Decision Summary

Choose whether and how to close the disposable schema-v5 creation-parity gap
for the already-existing Phase 3B Historical Reflection Question capability.
This decision does not activate schema v5 or change the product contract.

## Why Automation Stopped

Architecture/13 is Founder-approved design, but Slice 4C-1 promotion did not
authorize a new writer. Implementing even a private disposable writer requires
explicit scope authority because it touches consent-bound historical
provenance, deletion consequences, and future migration readiness.

## Relevant Constitution Clauses

`docs/00_Constitution.md` remains unchanged. The design preserves **We Build
Mirrors, Not Oracles**, user ownership of meaning, local-first control, visible
provenance, and refusal to finalize identity or make sensitive inferences.

## Relevant Primary Definitions

`docs/06_Memory.md`, `docs/Reflection.md`, `docs/09_AI.md`,
`docs/10_Privacy.md`, and `docs/appendix/Harness.md` require governed historical
use, exact sources, user-controlled consent, neutral reflection, provider
boundary clarity, and no hidden whole-history or profile construction.

## Relevant ADRs

- ADR-0007: retain authorship and provenance for persisted AI artifacts.
- ADR-0009: retain the exact per-generation/per-purpose consent, immutable
  packet, successful-use provenance, eligible source types, and deletion model.
- ADR-0011: normalized v5 lifecycle/provenance/dependency design is accepted,
  while production schema activation remains separately gated.

## Available Options

### A. Full disposable creation parity

Authorize one private, unregistered, path/connection-injected Rust boundary,
exercised only on synthetic/disposable exact-v5 fixtures produced through the
promoted migration core. It atomically creates the authoritative ADR-0009 v4
Historical Question records and guarded v5 normalized head/revision/purgeable
content/provenance/exact dependencies/lifecycle link; reuses promoted exact
Experience/Evidence/Reflection verifiers; retains packet bytes and digest by
reference; implements exact duplicate/idempotency, rollback, ambiguous-COMMIT,
read-only reconciliation, no-question/failure, and Slice 4C-1 deletion tests.

### B. V5-only normalized creation

Create the normalized v5 artifact without the v4 compatibility projection.

### C. Contracts and tests only

Add mapping contracts, fixtures, and reconciliation tests without any writer.

### D. Direct production activation

Activate production schema v5 before Phase 3B creation parity is closed.

## Benefits

- **A:** closes the exact future write-parity gap while preserving the current
  ADR-0009 subsystem and exercising deletion end-to-end.
- **B:** less dual-write code, but only by abandoning current runtime/storage
  compatibility.
- **C:** lowest implementation risk and improves documentation, but leaves the
  critical writer and transactional proof absent.
- **D:** no additional disposable stage, but offers no safe parity evidence.

## Risks

- **A:** dual-representation complexity, ambiguous commit classification, and
  accidental validator drift; mitigated by one transaction, exact parity,
  authoritative verifier reuse, full failure injection, and no runtime caller.
- **B:** breaks the schema-v4 bridge and cannot preserve current product/runtime
  behavior; deletion and provenance may diverge.
- **C:** postpones the gap and cannot prove rollback or cascade integration.
- **D:** contradicts current authority and risks real-user provenance loss or
  inconsistent deletion. It is rejected.

## Reversibility

- **A:** fully removable test/private-module evidence; no production caller,
  user database, or schema constant changes.
- **B:** code is removable, but its design abandons required compatibility and
  is not an acceptable migration step.
- **C:** documentation/tests are reversible but deliver no write parity.
- **D:** production activation could mutate real databases and is not safely
  reversible under current authority.

## Data And Privacy Impact

Option A uses synthetic/disposable data only and makes no provider call. It
does not change consent, provider retention, unsuccessful 30-day audit
retention, source eligibility, or deletion policy. Packet content is not copied
into generic revision content and is never reconstructed after consent.

## Orchestrator Recommendation

Select **Option A**. It is the smallest option that closes the real parity gap
and is safely bounded by promoted fixtures, shared DDL, compatibility guards,
authoritative verifier chains, and the existing Slice 4C-1 cascade. This is a
recommendation, not approval.

### Exact authorized implementation contract for Option A

- Exact-v5 disposable fixtures only; private and unregistered.
- Existing consumed consent, successful transmission, immutable packet and
  generated output are inputs; no selection, preflight, provider, or consent
  behavior is added.
- One `BEGIN IMMEDIATE` creates exact v4 and v5 representations and verifies
  them before commit.
- Exact packet bytes/digest, provider/model/purpose, consent/transmission IDs,
  generated content/citations, and exact source revisions must agree.
- Source types remain Experience, confirmed Evidence, and eligible saved
  user-authored Reflection only; promoted verifier chains must be reused.
- Historical Question authorship is AI, review is not-applicable, lifecycle is
  active, and eligibility is ineligible for reuse as historical source.
- Actual-use provenance exists only with the successful generated artifact.
- Exact duplicate requests are idempotent only after complete byte equality;
  any conflicting reuse fails closed.
- Cancel, provider failure, and no-question output create no artifact or
  actual-use provenance. Existing unsuccessful audit retention is unchanged.
- Existing Slice 4C-1 bridge performs later source invalidation/deletion for
  both representations; no parallel cascade, tombstone policy, or rebinding.
- Every meaningful boundary receives deterministic failure injection;
  ambiguous COMMIT is read-only classified or remains `recovery_required`.
- No production schema/user_version change, runtime registration, real data,
  Phase 4, or deployment authority.

### Precise implementation allowlist if Option A is selected

Product/domain files:

1. `src-tauri/src/schema_v5_historical_question_write.rs` (new)
2. `src-tauri/src/schema_v5_migration.rs` (private module declaration/test-fixture access only)
3. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md` (factual evidence synchronization only)

Repository-mediated workflow artifacts for this exact sprint may be updated by
the existing workflow and archived under:

4. `.ai/workflow/CURRENT_MISSION.md`
5. `.ai/workflow/DECISION_REQUIRED.md`
6. `.ai/workflow/ENGINEERING_PLAN.md`
7. `.ai/workflow/ENGINEERING_REPORT.md`
8. `.ai/workflow/EVENTS.jsonl`
9. `.ai/workflow/PRODUCT_REVIEW.md`
10. `.ai/workflow/SPRINT_REPORT.md`
11. `.ai/workflow/THEORY_ALIGNMENT_REVIEW.md`
12. `.ai/workflow/WORKFLOW_STATE.json`
13. `.ai/workflow/HISTORY/2026-08-02-phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate/`

No `sqlite.rs`, schema DDL, Cargo manifest, TypeScript, renderer, UI, provider,
ContextPacket, Harness contract, or other file is authorized. Before any future
promotion, the final concrete file list must be re-derived and Founder-reviewed;
this design decision is not promotion authority.

## Default Safe Action

Make no implementation changes. Retain the design package and workflow block.

## Blocked Files Or Phases

All implementation files and Engineering Planning are blocked pending this
decision. Production schema v5, real-user migration, runtime activation,
provider/ContextPacket/consent changes, retention changes, Phase 4, Harness
expansion, Git promotion, deployment, and release remain blocked regardless of
the selected bounded option.

## Exact Founder Response Needed

To authorize the recommendation, respond exactly:

`I resolve PHASE3C-SLICE4C2-001 by selecting Option A. I authorize Phase 3C Slice 4C-2 only: exact-v5 disposable fixtures produced through the promoted migration core; one private, unregistered, path/connection-injected Rust Historical Question creation-parity boundary that accepts only an already-consumed exact per-generation/per-purpose consent, an exact successful transmission, the unchanged immutable packet snapshot and digest, and a validated neutral source-citing generated output; one BEGIN IMMEDIATE transaction creating and reconciling the authoritative ADR-0009 schema-v4 Historical Question, packet, exact dependencies and successful actual-use references with the guarded schema-v5 historical_question head, immutable revision, purgeable content, provenance fingerprint, exact current-Experience and packet-item revision dependencies, and lifecycle link; exact preservation of provider/model/purpose/consent/transmission/packet identity; reuse of the promoted complete Experience, Evidence and Reflection verifier chains; exact unique-ID and source-set validation before set equality; at least one historical citation; exact-duplicate idempotency only after complete byte equality and conflicting duplicates fail closed; no artifact or actual-use provenance for cancellation, provider failure or valid no-question output while preserving the existing independent 30-day unsuccessful audit boundary; deterministic failure injection, logical rollback, conservative ambiguous-COMMIT and read-only restart reconciliation; exact v4/v5 parity and promoted Slice 4C-1 deletion/invalidation cascade verification; focused synthetic/disposable tests; factual Book One documentation; Clippy; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that this does not prove production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, provider calls, provider or ContextPacket behavior changes, consent or retention changes, new source eligibility, lifecycle UI, export v2, production backup/restore/recovery, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.`

Alternatively, state Option B, C, or D and its exact authorized scope. Silence
never resolves the decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4C2-001 by selecting Option A. I authorize Phase 3C Slice 4C-2 only: exact-v5 disposable fixtures produced through the promoted migration core; one private, unregistered, path/connection-injected Rust Historical Question creation-parity boundary that accepts only an already-consumed exact per-generation/per-purpose consent, an exact successful transmission, the unchanged immutable packet snapshot and digest, and a validated neutral source-citing generated output; one BEGIN IMMEDIATE transaction creating and reconciling the authoritative ADR-0009 schema-v4 Historical Question, packet, exact dependencies and successful actual-use references with the guarded schema-v5 historical_question head, immutable revision, purgeable content, provenance fingerprint, exact current-Experience and packet-item revision dependencies, and lifecycle link; exact preservation of provider/model/purpose/consent/transmission/packet identity; reuse of the promoted complete Experience, Evidence and Reflection verifier chains; exact unique-ID and source-set validation before set equality; at least one historical citation; exact-duplicate idempotency only after complete byte equality and conflicting duplicates fail closed; no artifact or actual-use provenance for cancellation, provider failure or valid no-question output while preserving the existing independent 30-day unsuccessful audit boundary; deterministic failure injection, logical rollback, conservative ambiguous-COMMIT and read-only restart reconciliation; exact v4/v5 parity and promoted Slice 4C-1 deletion/invalidation cascade verification; focused synthetic/disposable tests; factual Book One documentation; Clippy; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that this does not prove production restart recovery, real-user safety, or production schema-v5 readiness. I do not authorize production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, provider calls, provider or ContextPacket behavior changes, consent or retention changes, new source eligibility, lifecycle UI, export v2, production backup/restore/recovery, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Phase 3C Slice 4C-2 Option A exactly as recorded in the Founder response; private unregistered disposable-only Historical Question schema-v5 creation parity, tests, factual documentation, verification, theory review, archive/reset, and stop at Founder diff review.

## Decided At And Evidence Reference

- Decided at: 2026-08-01T21:16:34.032Z
- Evidence reference: PHASE3C-SLICE4C2-001

## Resume Phase

product_review
