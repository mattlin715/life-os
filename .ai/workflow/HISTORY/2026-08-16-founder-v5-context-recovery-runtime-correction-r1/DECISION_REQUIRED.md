# Decision Required

Status: resolved
- Sprint ID: 2026-08-16-founder-v5-context-recovery-runtime-correction-r1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-16T15:05:00+09:00
- Updated at: 2026-08-16T15:05:00+09:00

## Decision ID

FOUNDER-V5-CONTEXT-RECOVERY-R1-MANUAL-001

## Sprint ID

2026-08-16-founder-v5-context-recovery-runtime-correction-r1

## Decision Summary

Choose whether to begin the Founder-owned installation and affected Step 11D-3
manual retest using only the newly built unsigned isolated Candidate package.

## Why Automation Stopped

Installing the package and exercising the preserved disposable Founder profile
are consequential local actions reserved for the Founder. Automation built and
verified the package but did not install, launch, retry, or mutate any profile.

## Relevant Constitution Clauses

Local-first control, user ownership, visible uncertainty, and explicit consent
to consequential local actions remain controlling. The Constitution is
unchanged.

## Relevant Primary Definitions

Context Recovery is current-task-scoped clarification and remains excluded from
historical context and durable longitudinal memory.

## Relevant ADRs

ADR-0009 historical consent/provenance and ADR-0011 append-only lifecycle and
provenance remain unchanged.

## Available Options

- Option A: begin the bounded manual retest now. Install only the exact new
  unsigned package, then follow one step at a time through affected Step 11D-3.
- Option B: defer the manual retest. Preserve the package and disposable profile
  unchanged; make no further product or profile action.

## Benefits

- Option A obtains the missing packaged runtime evidence for the exact Context
  Recovery answer path and localized failure disclosure.
- Option B avoids any current local profile mutation and keeps the sprint safely
  blocked for later review.

## Risks

- Option A changes the installed isolated Founder Candidate application and, on
  the later explicit Save action, may append one response revision to the
  disposable Founder profile. It does not touch the ordinary profile.
- Option B leaves Candidate R1 incomplete and provides no packaged proof of the
  correction.

## Reversibility

The installer can be uninstalled, but the manual test's explicit successful
Save is a durable append-only change to the disposable Founder profile. No
rollback, restore, or deletion is authorized by this decision.

## Data And Privacy Impact

Only `%APPDATA%\\com.lifeos.founderdogfood` may be used after explicit Founder
steps. `%APPDATA%\\com.lifeos.app` and all real-user or ordinary-profile data
remain excluded. No provider call or historical transmission is added.

## Orchestrator Recommendation

Option A, using the exact package and one bounded instruction at a time.

## Default Safe Action

Option B. Do not install or launch anything until the Founder explicitly says
`Manual now`.

## Blocked Files Or Phases

Founder manual Step 11D-3, terminal Sprint Report, archive/reset, and Founder
diff review remain blocked. Git stage, commit, push, merge, PR, distribution,
deployment, and release remain unauthorized.

## Exact Founder Response Needed

For Option A, reply exactly: `Manual now`.

For Option B, reply exactly: `Defer manual review`.

## Resolution Status

resolved

## Exact Founder Response

Manual now

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Founder authorizes only the bounded manual installation and affected Step 11D-3 retest using the exact new unsigned isolated Candidate package, one instruction at a time. No automatic profile mutation, ordinary-profile access, schema or policy expansion, Git promotion, distribution, deployment, or release.

## Decided At And Evidence Reference

- Decided at: 2026-08-15T20:58:23.623Z
- Evidence reference: FOUNDER-V5-CONTEXT-RECOVERY-R1-MANUAL-001

## Resume Phase

theory_alignment_review
