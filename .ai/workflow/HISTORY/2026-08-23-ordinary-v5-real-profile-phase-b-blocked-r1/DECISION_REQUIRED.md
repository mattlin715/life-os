# Decision Required

Status: resolved
- Sprint ID: 2026-08-23-ordinary-v5-real-profile-phase-b-blocked-r1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-22T22:48:16.501Z
- Updated at: 2026-08-22T22:48:16.501Z

## Decision ID

`ORDINARY-V5-PHASEB-WAL-VERIFIER-CORRECTION-001`

## Sprint ID

`2026-08-23-ordinary-v5-real-profile-phase-b-blocked-r1`

## Decision Summary

Choose whether to authorize a new disposable-only correction sprint for the exact-v4 verifier's persistent-WAL sidecar behavior. This decision does not authorize any action on the current real ordinary profile.

## Why Automation Stopped

The one authorized Phase B migration action stopped before backup creation. Read-only evidence proves the main database still equals the pre-action schema-v4 state, but one exact-owned `prepared` operation, a zero-byte staging file, and WAL/SHM sidecars remain. The current verifier's non-immutable read-only connection reproducibly creates those sidecars on a persistent-WAL database, and the final backup preflight correctly refuses to continue. The original Phase B authority explicitly forbids retry or repair after a blocked state.

## Relevant Constitution Clauses

- `docs/00_Constitution.md` — Human Before AI.
- `docs/00_Constitution.md` — Privacy Before Profit.
- `docs/00_Constitution.md` — Memory Is Unreliable. Documentation Is Truth.
- `docs/00_Constitution.md` — Decision Test and Engineering Culture.

## Relevant Primary Definitions

- `docs/03_Principles.md` — Trust Principles and Context Principles.
- `docs/06_Memory.md` — Memory as evidence, consent, revision, and provenance.
- `docs/10_Privacy.md` — Data belongs to the user, transparency, and continuing consent.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`

## Available Options

### Option A — Authorize a separate disposable verifier correction sprint

Authorize only a new bounded sprint to make exact-v4 source inspection sidecar-free for a fully checkpointed persistent-WAL database; explicitly close the read-only verifier connection; add a production-shaped exact-v4 persistent-WAL regression through the real backup/migration command boundary; test fail-closed malformed/uncheckpointed states; synchronize architecture/18 and dev/10 factually; run focused Rust tests, Clippy with warnings denied, and canonical verification; build one ignored unsigned ordinary review installer; and stop at Founder diff/manual disposable review. The real profile, prepared operation, staging file, WAL/SHM, and all personal data remain untouched. Any evidence disposition or second real-profile attempt remains a separate later Founder gate.

### Option B — Close Ordinary Activation R1 as blocked

Do not implement a correction in this goal. Leave the real profile and all durable evidence untouched, record ordinary schema-v5 activation as not completed, and require a future independently proposed sprint before any further work.

## Benefits

- Option A: addresses the reproducible production-shape defect without risking the real profile and creates evidence needed for a later informed decision.
- Option B: minimizes immediate engineering activity and preserves the safest possible operational state.

## Risks

- Option A: an immutable verifier change still requires exact Rust and packaged disposable proof; it must not be treated as automatic authority to clean up or retry the real profile.
- Option B: the ordinary profile remains schema v4 and the current schema-v5 review build remains blocked by preserved prepared evidence.

## Reversibility

- Option A: repository-only and disposable-package work is reversible before any later promotion; the real profile is unchanged.
- Option B: no new mutation occurs; a future Founder gate may reconsider the correction.

## Data And Privacy Impact

Neither option authorizes reading personal rows. Neither option authorizes changing or deleting the real database, sidecars, operation directory, staging evidence, or any backup. Option A uses synthetic/disposable fixtures only.

## Orchestrator Recommendation

Option A. It preserves the real profile while converting the newly discovered production-shape mismatch into a bounded, testable correction. This recommendation is not approval.

## Default Safe Action

Option B behavior: do nothing to the real profile or current durable evidence, do not launch the schema-v5 review build, and do not retry migration.

## Blocked Files Or Phases

Engineering Planning, implementation, packaging, any real-profile evidence disposition, and any second real-profile migration attempt are blocked. The current ordinary profile must remain untouched.

## Exact Founder Response Needed

For Option A:

`I resolve ORDINARY-V5-PHASEB-WAL-VERIFIER-CORRECTION-001 by selecting Option A. I authorize only a separate disposable persistent-WAL exact-v4 verifier correction sprint: sidecar-free immutable source inspection with explicit close; production-shaped exact-v4 regression through the real backup/migration command boundary; malformed and uncheckpointed states failing closed; factual architecture/18 and dev/10 synchronization; focused tests, Clippy, canonical verification, one ignored unsigned ordinary review installer, and stop at Founder diff/manual disposable review. The current real com.lifeos.app profile, prepared operation, staging file, WAL/SHM, and personal data must remain untouched. This does not authorize cleanup, checkpoint, repair, restore, backup creation, retry, a second real-profile migration attempt, staging, commit, push, merge, PR, deployment, distribution, release, provider/ContextPacket/consent changes, Phase 4, or Android.`

For Option B:

`I resolve ORDINARY-V5-PHASEB-WAL-VERIFIER-CORRECTION-001 by selecting Option B. Close Ordinary Activation R1 as blocked, preserve the current real profile and all durable evidence unchanged, and do not implement, retry, repair, clean up, restore, migrate, stage, commit, push, merge, deploy, distribute, or release.`

## Resolution Status

resolved

## Exact Founder Response

Option A

## Selected Option And Authorized Scope

- Selected option: Option A
- Authorized scope: Separate disposable persistent-WAL exact-v4 verifier correction sprint only: sidecar-free immutable source inspection with explicit close; production-shaped exact-v4 regression through the real backup/migration command boundary; malformed and uncheckpointed states fail closed; factual architecture/18 and dev/10 synchronization; focused tests, Clippy, canonical verification, one ignored unsigned ordinary review installer, and stop at Founder diff/manual disposable review. The current real com.lifeos.app profile, prepared operation, staging file, WAL/SHM, and personal data remain untouched. No cleanup, checkpoint, repair, restore, backup creation, retry, second real-profile migration attempt, staging, commit, push, merge, PR, deployment, distribution, release, provider/ContextPacket/consent changes, Phase 4, or Android.

## Decided At And Evidence Reference

- Decided at: 2026-08-22T22:56:41.658Z
- Evidence reference: .ai/workflow/DECISION_REQUIRED.md#ORDINARY-V5-PHASEB-WAL-VERIFIER-CORRECTION-001

## Resume Phase

engineering_planning
