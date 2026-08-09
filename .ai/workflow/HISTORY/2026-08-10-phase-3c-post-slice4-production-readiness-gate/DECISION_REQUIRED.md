# Decision Required

Status: resolved
- Sprint ID: 2026-08-10-phase-3c-post-slice4-production-readiness-gate
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-09T19:04:00.000Z
- Updated at: 2026-08-09T19:04:00.000Z

## Decision ID

`PHASE3C-PRODUCTION-READINESS-R1-001`

## Sprint ID

`2026-08-10-phase-3c-post-slice4-production-readiness-gate`

## Decision Summary

Choose the next single bounded Phase 3C implementation after the promoted
private lifecycle parity work. The recommendation is the first production-
path, explicit-open, read-only local database readiness and three-language
disclosure slice. It exposes no write-capable control and does not activate v5.

## Why Automation Stopped

The repository proves extensive private/disposable contracts but not safe
production routing. Registering even a read-only production-path command and
adding user-facing database disclosure changes reachable product behavior.
Fresh-v5 or existing-v4 migration would be much more consequential. The Founder
must choose the next authority boundary; test success and prior promotions do
not decide it.

## Relevant Constitution Clauses

- We Build Mirrors, Not Oracles.
- Human Before AI.
- Evidence Before Conclusion.
- Privacy Before Profit.
- Documentation Hierarchy and human authority govern lower implementation.

## Relevant Primary Definitions

- `docs/03_Principles.md`
- `docs/06_Memory.md`
- `docs/Reflection.md`
- `docs/09_AI.md`
- `docs/10_Privacy.md`

These require proportional claims, provenance, reversibility, transparency,
psychological safety, and user control.

## Relevant ADRs

- ADR-0007: reviewed artifacts and provenance must survive persistence changes.
- ADR-0009: historical consent/transport/actual-use/deletion remain exact.
- ADR-0011: append-only lifecycle, exact dependencies, non-destructive disable,
  and no schema decrement remain binding.

No new ADR is recommended because architecture/13 and the accepted ADRs already
govern the proposed read-only boundary and all later migration decisions.

## Available Options

### Option A — Explicit read-only production readiness and disclosure

Authorize only architecture/15's explicit-open, session-only, read-only
production-path metadata inspection and English/Traditional Chinese/Japanese
disclosure. It may report database existence/version, path identity, sidecar or
unknown-quiescence limits, malformed/unreadable state, and contradictory owned
operation evidence. It must create no file/state and expose no Upgrade, Backup,
Restore, Delete, or retry-migration action.

### Option B — Fresh-install v5 without existing-v4 migration

Authorize fresh databases to initialize directly to v5 while existing v4
databases remain v4.

### Option C — Existing-v4 production migration activation

Authorize the complete real production backup, migration, restart, routing,
restore, retention, and disclosure path.

### Option D — Full lifecycle UI or export v2 next

Defer activation work and implement normalized lifecycle inspection/control or
complete export.

### Option E — Another private/disposable blocker

Keep all production paths unchanged and close a newly named private blocker.

## Benefits

- **A:** first production-path round-trip and three-language manual evidence
  without database mutation or migration authority.
- **B:** validates v5 runtime on new data without migrating existing data.
- **C:** reaches real cutover fastest.
- **D:** advances visible user ownership and portability.
- **E:** retains maximum isolation from real user data.

## Risks

- **A:** metadata may be mistaken for execution readiness; wording must say no
  migration is available and quiescence is unproved.
- **B:** creates a split v4/v5 product and requires broad reader/writer/release
  support in one slice.
- **C:** combines all critical data-loss and recovery risks and is not
  independently reviewable.
- **D:** builds UI/export over a storage path that production cannot yet use.
- **E:** repeats private evidence without crossing an identified blocker and
  delays product progress.

## Reversibility

- **A:** remove the panel and command; it stores nothing and changes no schema.
- **B:** not safely reversible through a schema decrement after v5 writes.
- **C:** not safely reversible except through explicit verified restore/forward
  fix; local restore cannot undo provider receipts.
- **D:** code can be feature-disabled but data/export semantics become support
  commitments.
- **E:** private code is reversible but may add low-value complexity.

## Data And Privacy Impact

Option A reads only minimal local metadata after an explicit user action. It
does not read row content, create a backup or operation record, persist UI
state, or transmit data. Automated tests use synthetic/disposable fixtures.
Options B and C introduce v5 data and require separate, much broader privacy
and recovery authority.

## Orchestrator Recommendation

Select **Option A** with the exact allowlist and acceptance matrices in
architecture/15. Accept the factual clarification that private Slice 5
lifecycle evidence is substantially implemented while original Slice 5
production integration/manual gates remain unstarted. Confirm that no new ADR
is required.

## Default Safe Action

Remain at `human_decision_required`. Do not implement any option, activate v5,
access real app-data, stage, commit, push, merge, deploy, or release.

## Blocked Files Or Phases

All Option A production paths listed in architecture/15 remain blocked. Fresh
v5, migration, backup, restore, retention, v5 routing, lifecycle UI, full
inspector, export v2, Phase 4, Git promotion, deployment, and release remain
separately blocked even if Option A is selected.

## Exact Founder Response Needed

To authorize the recommendation, reply exactly or equivalently:

```text
I resolve PHASE3C-PRODUCTION-READINESS-R1-001 by selecting Option A. I authorize only the explicit-open, session-only, read-only production-path database readiness inspection and three-language disclosure defined in architecture/15, using the exact anticipated allowlist and acceptance matrices. The command must not create, initialize, migrate, back up, restore, checkpoint, clean up, repair, select, or mutate any database or operation evidence, and the UI must expose no write-capable control. I accept the factual Slice 5 clarification and that no new ADR is required. I do not authorize fresh-v5 initialization, existing-v4 migration, production backup, restore, retention, v5 read/write routing, lifecycle UI, full provenance inspection, export v2, real-user migration testing, provider or ContextPacket changes, Phase 4, Harness expansion, staging, commit, push, merge, PR, deployment, or release.
```

The Founder may select B, C, D, or E and must state the exact scope. Silence is
not a decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve PHASE3C-PRODUCTION-READINESS-R1-001 by selecting Option A. I authorize only the explicit-open, session-only, read-only production-path database readiness inspection and three-language disclosure defined in architecture/15, using the exact anticipated allowlist and acceptance matrices. The command must not create, initialize, migrate, back up, restore, checkpoint, clean up, repair, select, or mutate any database or operation evidence, and the UI must expose no write-capable control. I accept the factual Slice 5 clarification and that no new ADR is required. I do not authorize fresh-v5 initialization, existing-v4 migration, production backup, restore, retention, v5 read/write routing, lifecycle UI, full provenance inspection, export v2, real-user migration testing, provider or ContextPacket changes, Phase 4, Harness expansion, staging, commit, push, merge, PR, deployment, or release.

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Option A only: explicit-open, session-only, read-only production-path database readiness inspection and English/Traditional Chinese/Japanese disclosure under architecture/15 exact allowlist and acceptance matrices; no database or operation-evidence mutation and no write-capable UI control. All fresh-v5, migration, backup, restore, retention, v5 routing, lifecycle UI, full inspector, export v2, real-user testing, provider/ContextPacket, Phase 4, Harness expansion, Git promotion, deployment, and release authority remain withheld.

## Decided At And Evidence Reference

- Decided at: 2026-08-09T19:26:57.183Z
- Evidence reference: Founder response in Codex task on 2026-08-10 for PHASE3C-PRODUCTION-READINESS-R1-001

## Resume Phase

theory_alignment_review
