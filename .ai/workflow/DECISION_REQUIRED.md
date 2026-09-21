# Decision Required

Status: resolved
- Sprint ID: 2026-09-20-android-build-feasibility-m0
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-09-21T06:03:00Z
- Updated at: 2026-09-21T06:03:00Z

## Decision ID

ANDROID-M0-PROMOTION-001

## Sprint ID

2026-09-20-android-build-feasibility-m0

## Decision Summary

Decide whether the exact, fully reviewed Android M0 candidate may later be
promoted from an unstaged working-tree candidate into one bounded Git
publication action. This decision is not resolved by either prior acceptance.

## Why Automation Stopped

The corrected APK is build-inspected, passed the disposable-emulator native
checklist 11/11, received explicit Founder native acceptance, and the complete
candidate passed fresh canonical repository verification. Promotion changes
repository/publication state and therefore requires a new explicit Founder
decision. This review package performs no Git or release action.

## Evidence Binding

- Branch: `codex/android-build-feasibility-m0`
- HEAD: `1575943094f24bd83c42088fbe4bb1a296083c50`
- Fresh verified non-workflow digest:
  `89dae6d3db296e49cdb850f2a1f952dc8b6d2b8d116acc3f15bd658ee3697f27`
- APK: `src-tauri/gen/android/app/build/outputs/apk/x86_64/debug/app-x86_64-debug.apk`
- APK size: `218677173` bytes
- APK SHA-256:
  `bd1a927f0f1347a11fccbe1d63a81e0851a907d6df981ed8b5f2c4db00291cad`
- Native evidence:
  `.artifacts/android-m0/native-review/2026-09-21-native-checklist-safe-area-result.md`
- Native evidence SHA-256:
  `a147e80e890e7ca5f2dfc5eea8873d2a1ca8e47fca9e9ba00b05faf9d731ecbd`
- Founder native acceptance: `ANDROID-M0-NATIVE-ACCEPT-001 Option A`
- Fresh canonical output:
  `.artifacts/android-m0/promotion-review/fresh-canonical-verification.txt`
- Final changed-path allowlist will be captured at
  `.artifacts/android-m0/promotion-review/final-changed-path-allowlist.json`.

## Prior Decisions Preserved

`ANDROID-M0-FOUNDER-REVIEW-001 Option A` accepted only the earlier build-
verified candidate while native review was pending. The exact original Founder
response remains preserved in the terminal snapshot and append-only journal.
`ANDROID-M0-NATIVE-ACCEPT-001 Option A` accepts the corrected APK's native
checklist 11/11. Neither response authorizes staging, commit, push, merge,
release, distribution, archive/reset, or M1.

## Relevant Constitution Clauses

Human before AI, Privacy before Profit, We Build Mirrors Not Oracles, and
Documentation Is Truth require explicit authority and evidence-bound scope for
a repository-state transition.

## Relevant Primary Definitions

`docs/10_Privacy.md` and the evidence/agency boundaries remain satisfied: M0
creates no Evidence, Memory, Reflection, Pattern, identity conclusion, provider
transmission, or persistent product data.

## Relevant ADRs

ADR-0004/0005/0006 preserve the local-first Tauri/React and provider boundaries.
ADR-0007/0009/0011 remain inactive on Android M0. ADR-0008 requires this
repository-native, explicit Founder gate. No ADR status changes.

## Available Options

- **Option A - authorize a later bounded Git promotion (recommended):** permit
  a subsequent task to reverify the exact bound candidate, stage only the final
  allowlist, create one M0 commit on the current branch, and push that branch by
  normal fast-forward. This option does not authorize merge, PR creation,
  archive/reset, distribution, deployment, release, production identity, or M1.
- **Option B - request exact corrections:** identify the required source,
  control-plane, documentation, or evidence change; keep all current work
  unstaged and return only to the smallest responsible phase.
- **Option C - defer promotion:** preserve the exact unstaged candidate and
  open gate without any repository/publication action.

## Benefits

- Option A turns the fully evidenced M0 result into reviewable branch history
  while keeping product release and M1 separate.
- Option B permits a bounded correction without silently widening authority.
- Option C preserves the accepted evidence and current candidate unchanged.

## Risks

- Option A changes durable Git and remote branch state; an exact preflight must
  reject drift, staging outside the allowlist, secrets, or non-fast-forward
  publication.
- Option B can invalidate current hashes and requires fresh verification.
- Option C leaves the candidate dependent on the local unstaged working tree.

## Reversibility

The present review is fully reversible because nothing is staged or committed.
A later commit/push is durable history and must not be performed without Option
A plus a fresh bound preflight. No release or user data migration is involved.

## Data And Privacy Impact

No real Life OS profile, SQLite database, provider credential, historical
context, physical device, cloud backup, or device transfer was accessed. The
APK has no Internet permission and remains a debug-only temporary identity.

## Orchestrator Recommendation

Choose Option A only if the integrated diff and evidence package are accepted.
The later promotion task must fail closed on any candidate drift and must stop
after the single branch push. M1 remains a separate Founder-authorized sprint.

## Default Safe Action

Option C: keep the candidate unstaged, unpublished, and unreleased.

## Blocked Files Or Phases

Git staging, commit, push, merge, PR creation, archive/reset, distribution,
deployment, release, production Android identity/storage, and M1 remain blocked
until separately and explicitly authorized. This task stops at this open gate.

## Exact Founder Response Needed

To authorize only the later bounded Git promotion, reply exactly:

`ANDROID-M0-PROMOTION-001 Option A. 我授權後續任務在重新核對 exact candidate 與 final allowlist 後，僅 stage 該 allowlist、建立一個 Android M0 commit，並以 normal fast-forward push 目前 branch。不要 merge、建立 PR、archive/reset、distribution、deployment、release 或開始 M1。`

Or identify exact corrections for Option B, or reply
`ANDROID-M0-PROMOTION-001 Option C. 暫緩 promotion，保持所有 diff unstaged。`

## Resolution Status

resolved

## Exact Founder Response

ANDROID-M0-PROMOTION-001 Option A. 我授權後續任務在重新核對 exact candidate 與 final allowlist 後，僅 stage 該 allowlist、建立一個 Android M0 commit，並以 normal fast-forward push 目前 branch。不要 merge、建立 PR、archive/reset、distribution、deployment、release 或開始 M1。

## Selected Option And Authorized Scope

- Selected option: Option A - authorize bounded Git promotion
- Authorized scope: After rechecking the exact candidate and final allowlist, stage only that allowlist, create one Android M0 commit, and normal fast-forward push the current branch. Do not merge, create a PR, archive/reset, distribute, deploy, release, or start M1.

## Decided At And Evidence Reference

- Decided at: 2026-09-21T06:23:28.586Z
- Evidence reference: chat:2026-09-21-android-m0-promotion-option-a

## Resume Phase

theory_alignment_review
