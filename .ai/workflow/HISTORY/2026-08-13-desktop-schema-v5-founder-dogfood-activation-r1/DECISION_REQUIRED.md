# Decision Required

Status: resolved
- Sprint ID: 2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-16T12:00:00+09:00
- Updated at: 2026-08-16T12:00:00+09:00

## Decision ID

FOUNDER-SCHEMA-V5-CANDIDATE-R1-CONTEXT-RECOVERY-FOLLOWUP-004

## Sprint ID

2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1

## Decision Summary

Whether to close the current Candidate R1 sprint truthfully at its bounded
three-cycle limit and authorize one separate minimal corrective sprint for the
fresh-v5 Context Recovery answer route.

## Why Automation Stopped

Founder manual review reached a currently reachable Context Recovery action.
Prompt creation passed, but saving the first non-empty response produced no
visible result. After normal close, read-only inspection proved no partial
answer was written: the schema-v5 database remains valid and the exact
`recovery_turn` remains suggested/pending with one revision.

Repository source inspection identified a deterministic adapter mismatch.
`schema_v5_runtime.rs` looks up this artifact using `context_recovery`; the
promoted writer, normalized head, and guarded schema-v4 projection use
`recovery_turn`. The failure occurs before the governed answer transaction.
The error is also not visible beside the Context Recovery controls.

The current workflow is already at review cycle 3 of 3. The repository Harness
forbids a fourth correction attempt, so this sprint cannot silently repair the
defect.

## Relevant Constitution Clauses

The Founder remains final constitutional authority. Local-first control,
visible failure, user ownership, correction/deletion, and We Build Mirrors,
Not Oracles remain unchanged.

## Relevant Primary Definitions

Context Recovery supplies current-task event context. It remains task-scoped,
historically ineligible, and distinct from durable longitudinal memory.

## Relevant ADRs

ADR-0009 and ADR-0011 remain controlling. No ADR status or decision changes.

## Available Options

- Option A: close the current Candidate R1 sprint as incomplete at its bounded
  review limit, archive/reset its truthful evidence, then start one separate
  repository workflow sprint named
  `2026-08-16-founder-v5-context-recovery-runtime-correction-r1`. Authorize only
  the exact runtime discriminator correction, a real-facade Rust regression,
  visible local mutation-error disclosure in the Context Recovery area with
  English/Traditional Chinese/Japanese parity, focused tests, Clippy,
  canonical verification, a new unsigned isolated Candidate package, and a
  repeat of the affected Step 11D-3 manual check. Stop again at Founder diff
  and manual review.
- Option B: stop Candidate R1 as incomplete and do not implement the
  correction.
- Option C: preserve the current sprint and disposable profile without further
  action while the Founder considers another bounded proposal.

## Exact Option A Boundary

The corrective sprint may change only the minimum runtime adapter, focused
Rust/runtime tests, focused UI/i18n error-disclosure code and tests, factual
Candidate documentation, package evidence, and repository-required workflow
artifacts. It may not change schema/DDL, migration policy, ordinary profile,
real user data, provider, ContextPacket, consent, Phase 4, Android, Harness
contracts, distribution, deployment, release, or Git promotion state. It may
not retry the failed action or mutate the current disposable profile until the
Founder explicitly performs the new packaged manual step.

## Benefits

Option A fixes a narrow reachable product defect, adds regression evidence at
the actual typed facade, and makes future failures visible at the point of
action. It preserves the bounded revision policy rather than hiding a fourth
cycle inside the current sprint.

## Risks

Option A creates another small sprint and unsigned package cycle. The current
fresh-v5 disposable profile remains manual evidence and must not be changed by
automation. Option B leaves Context Recovery answers unusable in the Candidate
and prevents completion of the remaining Pattern/manual matrix.

## Reversibility

Option A is code/test/package work only until the Founder explicitly installs
and exercises the new unsigned package. The current database remains untouched.
The correction can be discarded before promotion. Option B performs no work.

## Data And Privacy Impact

Current closed-file evidence is read-only: Founder database SHA-256
`aeec69625c36764876a15c862bfd41ecd1959f0dd14e1ccb12f250e3d9b9cff`,
schema v5, one Experience, three projected artifacts, six revisions, no
sidecars, valid foreign keys, and integrity `ok`. The ordinary profile SHA-256
remains
`bafdddac5f32f4289c318318f701336eeb5a62fb2e297c3cc15c1ab29372df4f`.
No provider or network action is involved.

## Orchestrator Recommendation

Option A.

## Default Safe Action

Do nothing. Do not launch, retry, edit the database, archive the sprint, or
start corrective implementation until the Founder answers.

## Blocked Files Or Phases

Candidate R1 completion, archive/reset, the Context Recovery answer retest,
remaining Pattern/manual checks, Founder diff review, and any promotion remain
blocked. Distribution, deployment, and release remain unauthorized.

## Exact Founder Response Needed

Reply with the exact Option A, B, or C resolution and bounded scope. No
authorization is inferred from Step 11D-3-D1, prior verification, or silence.

## Resolution Status

resolved

## Exact Founder Response

I resolve FOUNDER-SCHEMA-V5-CANDIDATE-R1-CONTEXT-RECOVERY-FOLLOWUP-004 by selecting Option A. I authorize closing the current Candidate R1 sprint truthfully as incomplete at its bounded three-cycle limit, archiving and resetting its evidence, and starting the separate sprint 2026-08-16-founder-v5-context-recovery-runtime-correction-r1. The new sprint may only correct the Context Recovery runtime artifact-kind discriminator, add a regression through the real typed facade, provide visible English／Traditional Chinese／Japanese mutation-error disclosure within the Context Recovery area, run focused tests、Clippy and canonical verification, build a new unsigned isolated Candidate package, and return to the affected Step 11D-3 manual review before Founder diff review. It must not retry or mutate the current disposable profile automatically. I do not authorize schema／DDL or migration-policy changes, ordinary-profile or real-user-data access, provider／ContextPacket／consent changes, Phase 4, Android, Harness expansion, distribution, deployment, release, staging, commit, push, merge, or PR.

## Selected Option And Authorized Scope

- Selected option: option_a
- Authorized scope: Close and archive the current incomplete cycle-limited sprint; then start one separate minimal Context Recovery runtime correction sprint limited to discriminator, real-facade regression, local three-language error disclosure, verification, unsigned package, and affected manual retest. No profile mutation by automation and no schema, ordinary-profile, provider, Phase 4, Android, Git promotion, distribution, deployment, or release authority.

## Decided At And Evidence Reference

- Decided at: 2026-08-15T20:31:29.104Z
- Evidence reference: FOUNDER-SCHEMA-V5-CANDIDATE-R1-CONTEXT-RECOVERY-FOLLOWUP-004

## Resume Phase

theory_alignment_review
