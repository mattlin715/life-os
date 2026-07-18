# Decision Required

Status: resolved
- Sprint ID: 2026-07-19-harness-learning-slice-1b-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-07-18T15:46:17.424Z
- Updated at: 2026-07-18T15:46:17.424Z

## Decision ID

`HL-001` and `PHASE3C-SLICE1B-001` are separate decisions.

## Sprint ID

`2026-07-19-harness-learning-slice-1b-gate`

## Decision Summary

1. Decide whether five archived sprints justify the bounded Stage 1 operational
   conclusion proposed in `WORKFLOW_EVALUATION.md`.
2. Separately decide whether to authorize full Slice 1B, only the Experience
   mutation subset Slice 1B-1, or no implementation.

The manual Slice 1A runtime checklist is a Founder review choice, not implied by
either decision.

## Why Automation Stopped

The Engineering Harness cannot approve its own operational adequacy. Accepted
architecture also does not authorize withheld production implementation. Both
decisions affect durable repository authority and require an exact Founder
response; silence is not consent.

## Relevant Constitution Clauses

- Human before AI: the Founder retains consequential authority.
- Evidence before Conclusion: five archives support but do not self-approve the
  Harness conclusion.
- Privacy before Profit: only synthetic/disposable data may be used.
- Documentation is part of the product: promotion and implementation status
  must remain factual.

No Constitution change is proposed.

## Relevant Primary Definitions

- `docs/06_Memory.md`: user-controlled, revisable, provenance-preserving memory.
- `docs/Reflection.md`: user-owned meaning.
- `docs/09_AI.md`: mirror rather than oracle.
- `docs/10_Privacy.md`: local control, transparency, and ongoing consent.
- `docs/appendix/Harness.md`: human-reviewed evaluation boundary.

## Relevant ADRs

- ADR-0007: reviewed artifacts and provenance.
- ADR-0008: tool-independent Engineering Harness governance; unchanged.
- ADR-0009: historical consent, provenance, stale-work, and deletion guarantees.
- ADR-0011: lifecycle direction Accepted; production migration remains gated.

## Available Options

### HL-001

- **HL-A — Accept bounded conclusion:** Record Stage 1 as operationally adequate
  for manually triggered, repository-mediated Life OS product sprints with
  Founder checkpoints and one writable worker. Explicitly retain no
  independent-review assurance and no Stage 2/3 authority.
- **HL-B — Keep evaluation pending:** Do not accept the conclusion; state the
  specific missing operational evidence or correction required.

### PHASE3C-SLICE1B-001

- **Option A — Full Slice 1B:** Convert Experience plus artifact, historical,
  consent, transmission, and provenance mutation paths to typed Rust commands.
- **Option B — Slice 1B-1 only (recommended):** Convert only Experience
  create/update/delete/import to typed Rust commands; preserve schema v4 and
  defer the broader mutation graph to Slice 1B-2.
- **Option C — Defer:** Make no Slice 1B production changes.

### Slice 1A Manual Review

- **Completed with procedure correction:** The Founder completed missing/v4,
  newer-v5, multilingual, and malformed disposable-fixture checks. Verified
  v5 and malformed hashes were unchanged, v4 persistence/editing worked, and
  all disposable application-data paths were removed. The Product Review
  records that the first `APPDATA`-override attempt opened the real database
  before a guard hash existed; later custom-identifier runs were isolated and
  the real database remained schema v4, integrity `ok`, and unchanged after the
  guard was captured.

## Benefits

- **HL-A:** Closes Stage 1 learning honestly and lets the Harness return to its
  intended role as a product-delivery mechanism.
- **HL-B:** Preserves a higher evidence threshold when a concrete gap remains.
- **Option A:** Completes the broad renderer-to-Rust trust-boundary conversion in
  one effort.
- **Option B:** Delivers the smallest product-first safety improvement with a
  reviewable rollback surface and isolates ADR-0009-sensitive paths.
- **Option C:** Avoids immediate implementation risk.

## Risks

- **HL-A:** May be overread as independent validation unless its caveats remain
  adjacent and explicit.
- **HL-B:** Can prolong process work unless the missing evidence is concrete and
  product-relevant.
- **Option A:** Large coupled change surface can regress consent, deletion,
  provenance, historical stale-work, or transaction behavior.
- **Option B:** Generic renderer SQL remains in deferred artifact/historical
  paths; this is partial trust-boundary progress, not completion.
- **Option C:** Existing renderer-owned Experience SQL and fragile generic
  revision-check coupling remain.

## Reversibility

- **HL-A/HL-B:** Documentation-only and reversible by a later evidence-backed
  Founder decision; neither changes product data.
- **Option A:** Code-revertable with schema v4 unchanged, but broad behavioral
  review and rollback evidence would be required.
- **Option B:** Narrowly code-revertable because it adds no schema or migration;
  existing v4 data remains authoritative.
- **Option C:** No repository behavior change.

## Data And Privacy Impact

HL-001 has no product-data impact. Option B keeps all data local and schema v4,
removes renderer SQL authorship only for Experience mutations, and must use
synthetic/disposable fixtures. It does not authorize opening or mutating a real
user database, provider transport, historical ContextPacket changes, or new
retention behavior.

## Orchestrator Recommendation

Select `HL-A` and `Option B`. The Slice 1A manual review is complete with the
procedure correction and evidence limit recorded in the Product Review.

## Default Safe Action

Remain at `human_decision_required`. Do not implement Slice 1B, change the
workflow evaluation status, or infer acceptance.

## Blocked Files Or Phases

Blocked pending exact resolution: production TypeScript/Rust storage mutation
files, implementation tests, implementation-aligned architecture changes,
Engineering Plan, implementation, verification/archive, and any Git promotion.
Always blocked by this gate: Harness expansion, Stage 2/3, schema-v5 activation,
migration, backup/restore/retention, Slices 1B-2 and 2-6, Phase 4, provider or
ContextPacket behavior, PR, and deployment.

## Exact Founder Response Needed

The manual review is complete. Please answer the two remaining decisions in
order.

1. **HL-001:** `I select HL-A` or `I select HL-B: <missing evidence>`.
2. **PHASE3C-SLICE1B-001:** `I select Option A`, `I select Option B`, or
   `I select Option C`.

To authorize the recommendation in one exact response:

> I acknowledge the completed Slice 1A manual review, including its isolation
> procedure correction and evidence limit. I resolve HL-001 by selecting HL-A
> and accept only the bounded Stage 1 conclusion recorded in the decision
> package. I separately resolve PHASE3C-SLICE1B-001 by selecting Option B and
> authorize Slice 1B-1
> only: typed Rust Experience create/update/delete/import commands and adapters,
> schema v4 preservation, synthetic/disposable tests, factual documentation,
> canonical verification, Theory Alignment Review, archive/reset, and stop at
> Founder diff review. I do not authorize Slice 1B-2, schema v5, migration,
> backup, restore, retention, later slices, Phase 4, Harness expansion, Stage 2
> or Stage 3, provider/ContextPacket changes, real user-database testing, Git
> promotion, PR, or deployment.

## Resolution Status

resolved

## Exact Founder Response

接受
I resolve HL-001 by selecting HL-A. I accept only the bounded Stage 1 conclusion recorded in the Founder decision package. This does not provide independent-review assurance and does not authorize Stage 2, Stage 3, autonomous orchestration, deployment, or any additional product implementation.

I resolve PHASE3C-SLICE1B-001 by selecting Option B. I authorize Phase 3C Slice 1B-1 only: typed Rust Experience create, update, delete, and atomic import commands; typed TypeScript adapters without renderer-supplied SQL for those paths; in-transaction expected-revision validation; stale-update fail-closed behavior; preservation of schema-v4 deletion, dependency, and ADR-0009 provenance semantics; duplicate-skipping atomic import with full rollback on failure; synthetic/disposable fixtures; factual documentation synchronization; canonical verification; Theory Alignment Review; archive/reset; and stop at Founder diff review. I do not authorize Slice 1B-2, schema v5, user_version 5, live migration, real user-database testing, backup, restore, retention, Slices 2–6, Phase 4, provider or ContextPacket changes, Harness expansion, Stage 2, Stage 3, staging, commit, push, merge, PR, or deployment.

## Selected Option And Authorized Scope

- Selected option: HL-A; PHASE3C-SLICE1B-001 Option B
- Authorized scope: HL-001: bounded Stage 1 conclusion accepted with no independent-review assurance and no Stage 2/3 authority. PHASE3C-SLICE1B-001 Option B: Slice 1B-1 typed Rust Experience create/update/delete/atomic import only, typed TypeScript adapters, schema v4 preservation, synthetic/disposable tests, factual docs, verification, theory review, archive/reset, and stop at Founder diff review; all listed exclusions remain.

## Decided At And Evidence Reference

- Decided at: 2026-07-18T19:25:40.767Z
- Evidence reference: .ai/workflow/DECISION_REQUIRED.md#HL-001-and-PHASE3C-SLICE1B-001

## Resume Phase

product_review
