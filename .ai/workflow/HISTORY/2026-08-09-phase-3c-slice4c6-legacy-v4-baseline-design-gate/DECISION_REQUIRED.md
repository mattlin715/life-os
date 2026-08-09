# Decision Required

Status: resolved
- Sprint ID: 2026-08-09-phase-3c-slice4c6-legacy-v4-baseline-design-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-08T20:13:52.826Z
- Updated at: 2026-08-08T20:13:52.826Z

## Decision ID

`PHASE3C-SLICE4C6-001`

## Sprint ID

`2026-08-09-phase-3c-slice4c6-legacy-v4-baseline-design-gate`

## Decision Summary

Choose the next bounded response to migrated `legacy-v4-raw` current-action
gaps. The recommendation is Option B: one private disposable Slice 4C-6A for
legacy Evidence and Pattern candidate confirmation/rejection, while retaining
complete current-action parity as a hard prerequisite to any production-v5
gate.

## Why Automation Stopped

Current UI actions would encounter explicit or unproved private-v5 legacy
review behavior after a future migration. Deciding that a preserved legacy
candidate may receive a new exact user review or be destructively purged changes
durable lifecycle policy and requires Founder authority. No implementation is
authorized by the architecture, passing tests, this prompt, or silence.

## Relevant Constitution Clauses

- We Build Mirrors, Not Oracles.
- Human Before AI.
- Evidence Before Conclusion.
- Privacy Before Profit.
- The Founder remains final constitutional authority; no Constitution edit is
  proposed.

## Relevant Primary Definitions

- `docs/02_Philosophy.md`
- `docs/03_Principles.md`
- `docs/06_Memory.md`
- `docs/Reflection.md`
- `docs/09_AI.md`
- `docs/10_Privacy.md`
- `docs/appendix/Harness.md`

They require exact evidence/provenance, user-owned meaning, review uncertainty,
correction/deletion, consent boundaries, and no silent profiling or AI
authority.

## Relevant ADRs

- ADR-0007: reviewed artifacts preserve provenance and exact user review state.
- ADR-0009: stale historical sources trigger exact cascade deletion; packet and
  consent facts are not broadened.
- ADR-0011: exact-revision review, append-only correction, no confirmation
  carry-forward or rebinding, immediate rejected-content purge, ordinary
  invalidation, and content-free deletion facts.

## Available Options

### Option A — All reachable legacy actions in one slice

Implement every current Evidence, Reflection, Pattern, and Context Recovery
legacy action together. This is broad and couples review-only, purge, successor,
provenance, and dependency behavior.

### Option B — Evidence and Pattern review actions first

Authorize only Slice 4C-6A: exact migrated candidate Evidence/Pattern
confirmation and rejection, strict parsing without predecessor rewrite, exact
new user review events, purge/projection rules, deterministic rollback and
reconciliation. Defer successor-producing actions to a final separately gated
4C-6B. **Recommended.**

### Option C — Successor-producing actions first

Implement Evidence edit and Reflection/Context Recovery response actions first,
leaving current Pattern confirm/reject blocked.

### Option D — Read-only migrated artifacts

Keep all legacy artifacts read-only and later add calm refusals, despite current
UI actions.

### Option E — Defer production v5 until the complete matrix is implemented

Perform no implementation now. Retain this as the mandatory cutover rule in
all cases; 4C-6A alone cannot unlock production.

## Benefits

- **A:** one named implementation phase, complete matrix if successful.
- **B:** smallest coherent review/purge boundary; removes explicit Pattern fence;
  proves Evidence behavior; makes risk inspectable before successor creation.
- **C:** advances user-authored successor paths first.
- **D:** simplest writer behavior.
- **E:** safest immediate no-change position and correct production gate.

## Risks

- **A:** excessive state, file, rollback, and review surface for one slice.
- **B:** does not finish cutover parity; 4C-6B remains mandatory.
- **C:** existing confirm/reject buttons still fail and more complex provenance
  changes are attempted first.
- **D:** silently removes existing capabilities at upgrade unless separately
  redesigned and disclosed.
- **E:** no product progress and leaves known blockers unresolved.

## Reversibility

- A/B/C are private disposable evidence only if exact scope is preserved; they
  do not touch production or real data. B is the smallest reversible step.
- D/E are reversible planning choices, but neither can authorize cutover.

## Data And Privacy Impact

Recommended Option B uses synthetic/disposable exact-v5 fixtures only. Confirm
preserves exact raw bytes and unknown provenance. Reject explicitly purges only
the chosen artifact content under ADR-0011 and retains bounded content-free
facts. No real data, provider, consent, packet, retention, migration, UI, or
Phase 4 behavior is touched.

## Orchestrator Recommendation

Select **Option B**. Authorize only the three-path product/document allowlist
and repository-required workflow artifacts. Require a separate 4C-6B Founder
gate and promotion before production migration/recovery can even be proposed.

## Default Safe Action

Remain at `human_decision_required`. Do not implement, stage, commit, push,
merge, activate schema v5, access real data, or infer authority.

## Blocked Files Or Phases

All implementation and engineering-planning transitions remain blocked. In
particular, `schema_v5_evidence_write.rs` and
`schema_v5_pattern_write.rs` cannot be edited until this decision is resolved.
Slice 4C-6B, production migration/recovery, runtime integration, DDL, UI,
providers, ContextPacket, consent, retention, export v2, Phase 4, Git promotion,
deployment, and release remain separately blocked.

## Exact Founder Response Needed

To authorize the recommendation, reply exactly or equivalently:

```text
I resolve PHASE3C-SLICE4C6-001 by selecting Option B. I authorize Phase 3C Slice 4C-6A only: exact-v5 disposable fixtures produced through the promoted exact-v4 migration core; extension only of the existing private, unregistered, path/connection-injected Rust Evidence and Pattern boundaries for exact legacy-v4-raw pending/candidate confirmation and rejection; strict artifact-specific parsing of the supported schema-v4 shape for validation without rewriting, recanonicalizing, replacing, or relabeling the byte-exact legacy predecessor; exact raw-byte, digest, projection, source, revision, unique-ID, dependency, provenance-representation, review-state and lifecycle revalidation; confirmation that changes only exact review/head eligibility, appends one explicit user decision at the actual injected action time, preserves imported review evidence and legacy_unknown provenance, makes confirmed Evidence eligible, and keeps confirmed Pattern a revisable useful-for-reflection hypothesis; rejection that synchronously purges all retained artifact content, removes the guarded v4 projection, records exact review/lifecycle facts, retains only ADR-0011-authorized content-free metadata, and applies only exact legal dependent consequences and ADR-0009 cascade behavior; malformed, unknown-field, unsupported-shape, stale, duplicate, missing, orphaned, cross-source, projection-mismatch, digest-mismatch, contradictory, or unsupported inbound evidence failing closed without repair or rebinding; deterministic failure injection, exact logical rollback, conservative ambiguous-COMMIT exact pre-state/post-state/third-state classification, and read-only reconciliation; changes limited to src-tauri/src/schema_v5_evidence_write.rs, src-tauri/src/schema_v5_pattern_write.rs, docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md, and repository-required workflow artifacts; focused synthetic/disposable tests; factual documentation; Clippy with warnings denied; canonical verification; Product and Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that Slice 4C-6A does not complete migrated current-action parity and that one separately authorized Slice 4C-6B remains required for legacy Evidence candidate correction, Reflection answer/skip/correction, and Context Recovery answer/skip before any production migration/recovery gate. I do not authorize Slice 4C-6B, production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, new product actions, DDL changes, generic legacy mutation infrastructure, standalone Context Recovery correction/deletion, provider or ContextPacket changes, consent or retention changes, export v2, production backup/restore/recovery, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.
```

The Founder may instead select A, C, D, or E and state the exact authorized
scope. Silence is not a decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-SLICE4C6-001 by selecting Option B. I authorize Phase 3C Slice 4C-6A only: exact-v5 disposable fixtures produced through the promoted exact-v4 migration core; extension only of the existing private, unregistered, path/connection-injected Rust Evidence and Pattern boundaries for exact legacy-v4-raw pending/candidate confirmation and rejection; strict artifact-specific parsing of the supported schema-v4 shape for validation without rewriting, recanonicalizing, replacing, or relabeling the byte-exact legacy predecessor; exact raw-byte, digest, projection, source, revision, unique-ID, dependency, provenance-representation, review-state and lifecycle revalidation; confirmation that changes only exact review/head eligibility, appends one explicit user decision at the actual injected action time, preserves imported review evidence and legacy_unknown provenance, makes confirmed Evidence eligible, and keeps confirmed Pattern a revisable useful-for-reflection hypothesis; rejection that synchronously purges all retained artifact content, removes the guarded v4 projection, records exact review/lifecycle facts, retains only ADR-0011-authorized content-free metadata, and applies only exact legal dependent consequences and ADR-0009 cascade behavior; malformed, unknown-field, unsupported-shape, stale, duplicate, missing, orphaned, cross-source, projection-mismatch, digest-mismatch, contradictory, or unsupported inbound evidence failing closed without repair or rebinding; deterministic failure injection, exact logical rollback, conservative ambiguous-COMMIT exact pre-state/post-state/third-state classification, and read-only reconciliation; changes limited to src-tauri/src/schema_v5_evidence_write.rs, src-tauri/src/schema_v5_pattern_write.rs, docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md, and repository-required workflow artifacts; focused synthetic/disposable tests; factual documentation; Clippy with warnings denied; canonical verification; Product and Theory Alignment Review; archive/reset; and stop at Founder diff review. I accept that Slice 4C-6A does not complete migrated current-action parity and that one separately authorized Slice 4C-6B remains required for legacy Evidence candidate correction, Reflection answer/skip/correction, and Context Recovery answer/skip before any production migration/recovery gate. I do not authorize Slice 4C-6B, production SCHEMA_VERSION 5, production user_version 5, migration or fresh-v5 initialization, real user databases or app-data, startup/Tauri/renderer/UI activation, new product actions, DDL changes, generic legacy mutation infrastructure, standalone Context Recovery correction/deletion, provider or ContextPacket changes, consent or retention changes, export v2, production backup/restore/recovery, automatic retry/replay/rollback/repair/cleanup/candidate selection, Phase 4, identity or sensitive inference, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: B
- Authorized scope: Phase 3C Slice 4C-6A only under the exact three-path product/document allowlist and all exclusions in the Founder response

## Decided At And Evidence Reference

- Decided at: 2026-08-08T20:54:04.238Z
- Evidence reference: founder-message-2026-08-09-PHASE3C-SLICE4C6-001

## Resume Phase

product_review
