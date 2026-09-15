# Decision Required

Status: resolved
- Sprint ID: 2026-08-27-desktop-schema-v5-prepared-state-recovery-r2
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-09-14T21:00:00+09:00
- Updated at: 2026-09-14T21:00:00+09:00

## Decision ID

DESKTOP-V5-PREPARED-RECOVERY-R2B-PROMOTION-001

## Sprint ID

2026-08-27-desktop-schema-v5-prepared-state-recovery-r2

## Decision Summary

The exact accepted R2/R2B product and disposable BF1-BF8 evidence are preserved.
A bounded Engineering Harness correction now allows fresh canonical verification
inside the already-resolved Manual-A gate without reopening it or weakening its
scope. The official CLI recorded that verification, resumed to
`theory_alignment_review`, and prepared this new promotion decision.

This decision asks only whether to authorize the separately bounded local Git
promotion described below. Promotion has not occurred.

## Why Automation Stopped

Founder diff review and explicit Promotion Authorization are consequential human
gates. Automated verification, prior product acceptance and the corrected
workflow cannot authorize staging, committing, branch switching or merging.
Silence is not approval.

## Relevant Constitution Clauses

The Constitution remains unchanged. Human Before AI, Evidence Before
Conclusion, Privacy Before Profit, documentation truth and “We Build Mirrors,
Not Oracles” require explicit human authority and accurate separation between
verified, accepted, promoted and released states.

## Relevant Primary Definitions

No Book Zero primary definition changes. The workflow correction governs
engineering evidence only; it does not interpret Identity, Memory, Reflection,
Awareness, Growth or personal Evidence.

## Relevant ADRs

ADR-0007, ADR-0009 and ADR-0011 remain unchanged. No ADR status or implementation
authority is modified by this decision.

## Available Options

- **Option A — authorize bounded local promotion.** Reverify the exact current
  changed-path set and clean index; stage only the reviewed paths; create one
  feature commit; switch only to a clean local `develop`; require the recorded
  ancestry and no unrelated changes; perform one non-fast-forward local merge;
  and rerun canonical verification. This option does not authorize push, PR,
  deployment, distribution, release, application launch, real-profile action,
  Phase 4 or Android.
- **Option B — preserve and stop.** Keep the exact uncommitted branch, ignored
  evidence and active promotion decision unchanged. Perform no stage, commit,
  branch switch, merge or publication action.

## Benefits

- Option A creates one reviewable local promotion boundary while keeping remote
  publication and all runtime/profile actions separate.
- Option B preserves the current proven state without any Git mutation.

## Risks

- Option A changes local Git history and could be misread as publication or
  runtime authority. The bounded procedure, clean-tree checks and explicit
  exclusions reduce but do not erase that coordination risk.
- Option B leaves the implementation unpromoted and Android ineligible, but
  preserves every current evidence boundary.

## Reversibility

Option B makes no change. Option A creates explicit local commits and a merge;
it does not push or rewrite remote history. No reset, rebase, force operation or
unreviewed-path staging is permitted.

## Data And Privacy Impact

Neither option authorizes access to the real Founder profile. Promotion changes
repository history only and performs no application launch, migration, recovery,
restore, provider transmission or personal-data operation.

## Orchestrator Recommendation

Recommend Option A only if the Founder accepts both the original R2/R2B diff
and the new workflow-control correction package. Otherwise select Option B.

## Default Safe Action

Option B. Preserve the current uncommitted branch and infer no authority from
silence.

## Blocked Files Or Phases

Local staging/commit/merge, remote push/PR, deployment/distribution/release,
real-profile recovery or other mutation, Phase 4 and Android remain blocked.

## Exact Founder Response Needed

For Option A:

> I select Option A. I authorize one bounded local R2B promotion cycle for the
> exact reviewed 43-path diff: reverify the branch, HEAD, clean index, exact
> path set and remote ancestry; stage only those 43 paths; create one feature
> commit; switch to a clean local develop branch; perform one non-fast-forward
> merge; run canonical verification; and report the resulting commits and Git
> state. I do not authorize push, PR, deployment, distribution, release,
> application launch, real-profile access or mutation, recovery, retry,
> restore, repair, checkpoint, schema decrement, backup deletion, migration,
> Phase 4, or Android.

For Option B:

> I select Option B. Preserve the exact reviewed 43-path diff and active
> promotion decision without staging, committing, switching branches, merging,
> pushing, opening a PR, deploying, distributing, releasing, launching the
> application, accessing or mutating the real profile, performing recovery or
> migration, beginning Phase 4, or starting Android work.

## Resolution Status

resolved

## Exact Founder Response

I select Option A. I authorize one bounded local R2B promotion cycle for the exact reviewed 43-path diff: reverify the branch, HEAD, clean index, exact path set and remote ancestry; stage only those 43 paths; create one feature commit; switch to a clean local develop branch; perform one non-fast-forward merge; run canonical verification; and report the resulting commits and Git state. I do not authorize push, PR, deployment, distribution, release, application launch, real-profile access or mutation, recovery, retry, restore, repair, checkpoint, schema decrement, backup deletion, migration, Phase 4, or Android.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: One bounded local R2B promotion cycle for the exact reviewed 43-path diff: reverify branch, HEAD, clean index, exact path set and remote ancestry; stage only those 43 paths; create one feature commit; switch to a clean local develop branch; perform one non-fast-forward merge; run canonical verification; and report the resulting commits and Git state. No push, PR, deployment, distribution, release, application launch, real-profile access or mutation, recovery, retry, restore, repair, checkpoint, schema decrement, backup deletion, migration, Phase 4, or Android is authorized.

## Decided At And Evidence Reference

- Decided at: 2026-09-15T17:22:01.051Z
- Evidence reference: Founder exact Option A response in current task on 2026-09-16

## Resume Phase

theory_alignment_review

## Historical Resolved Decision

- Decision ID: `DESKTOP-V5-PREPARED-RECOVERY-R2-MANUAL-A-001`
- Resolution status: `resolved`
- Selected option: `A`
- Decided at: `2026-08-31T17:01:21.708Z`
- Evidence reference: `Founder exact Option A response in current task on 2026-09-01`
- Authorized scope: Close only DESKTOP-V5-PREPARED-RECOVERY-R2-MANUAL-A-001 by accepting the exact 40-path repository diff and completed disposable Manual Phase A evidence. No real Founder profile access or mutation; no Phase B, Phase C, promotion, staging, commit, push, merge, PR, deployment, distribution, release, Phase 4, or Android authority.

Exact historical Founder response, preserved unchanged:

> I select Option A. I accept the exact 40-path Life OS Desktop Schema-v5 Prepared-State Recovery and Real-Profile Migration R2 repository diff and the completed disposable Manual Phase A evidence. This acceptance closes only DESKTOP-V5-PREPARED-RECOVERY-R2-MANUAL-A-001. It does not authorize access, inspection, recovery, backup, restore, migration, or mutation of the real Founder com.lifeos.app profile; Phase B, Phase C, promotion, staging, commit, push, merge, PR, deployment, distribution, release, Phase 4, and Android remain separately unauthorized.

That decision remains closed historical evidence. The new promotion decision is
independent and unresolved. The real profile remains preserved at committed
schema v5 with `post_commit_schema_manifest_mismatch`, lifecycle writes disabled
and its verified schema-v4 backup retained. Android M0 remains parked.