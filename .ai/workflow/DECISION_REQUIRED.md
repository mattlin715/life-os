# Decision Required

Status: resolved
- Sprint ID: 2026-09-22-android-m1-disposable-persistence-review
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-09-27T18:26:00Z
- Updated at: 2026-09-27T18:26:00Z

## Decision ID

`ANDROID-M1-FOUNDER-REVIEW-004`

## Sprint ID

2026-09-22-android-m1-disposable-persistence-review

## Decision Summary

Review the exact Android M1 launcher icon after the third and final bounded
revision. The unchanged canonical PC icon is now uniformly centered below 60%
of every adaptive foreground canvas with transparent padding, so Android's
rounded mask should show the entire pale ring. Legacy and round launcher assets
and all product behavior remain unchanged.

## Why Automation Stopped

Exact contracts decode every foreground and verify dimensions, alpha bounds,
centering, output hashes, and canonical xxxhdpi inner pixels. Build, APK
inspection, the complete dedicated Android 36 x86_64 native matrix, and
canonical repository verification pass. Automation and captured rendering show
the full ring, but only the Founder can accept the visual result.

## Relevant Constitution Clauses

`docs/00_Constitution.md` keeps the Founder as final constitutional authority
and requires Life OS to build mirrors, not oracles. This visual correction
changes no constitutional text, product worldview, or behavior.

## Relevant Primary Definitions

`docs/10_Privacy.md`, `docs/11_MVP.md`, and `docs/12_Roadmap.md` continue to
govern local ownership, user agency, scope, and accurate review-state claims.
The icon contains no user content or inference.

## Relevant ADRs

Accepted ADR-0004, ADR-0006, ADR-0007, and ADR-0011 remain unchanged.
ADR-0012 remains **Proposed** until an explicit Founder decision is recorded.

## Available Options

### Option A — accept safe-zone-corrected candidate and recommended architecture

After visually confirming the complete ring, accept the exact unstaged Android
M1 candidate, select stable production identity `com.lifeos.app`, and accept the
Proposed ADR-0012 architecture direction. Authorize only bounded workflow and
document recording. Do not authorize staging, commit, push, release, real-data
activation, or M2 implementation.

### Option B — accept corrected prototype evidence but defer architecture

After visually confirming the complete ring, accept only the disposable M1
prototype evidence and UI/icon corrections. Keep ADR-0012 Proposed and leave
final identity, production storage authority, and backup/transfer policy
undecided.

### Option C — visual review fails; stop for a separate future scope

Do not accept the icon. Identify the exact remaining visual issue and stop this
sprint without another in-sprint correction. The configured third and final
bounded revision has been used; any further implementation requires a separately
authorized scope. Keep every later authority withheld.

### Option D — reject/cancel

Reject the candidate and cancel the sprint without staging, publication,
production activation, or later-phase work.

## Benefits

- **A:** accepts the corrected visual identity and closes the M1 architecture
  choice while preserving M2-M4 gates.
- **B:** preserves the corrected disposable prototype evidence without
  prematurely fixing production identity or continuity policy.
- **C:** reports a remaining visual failure without bypassing the bounded-review
  limit.
- **D:** prevents adoption of an unsuitable direction.

## Risks

- **A:** `com.lifeos.app` and signing lineage become the intended long-lived
  Android identity; backup/transfer exclusion still means uninstall, data clear,
  or device loss can destroy local data until later continuity work.
- **B:** production Android architecture remains open.
- **C:** the current sprint cannot make another correction; a separately scoped
  future task is required.
- **D:** retains no accepted M1 direction and requires a future replacement.

## Reversibility

- **A:** reversible before real data/distribution, but costly afterward.
- **B:** fully reversible; the temporary package remains disposable.
- **C:** leaves the exact unstaged candidate intact for later review.
- **D:** files remain unstaged and require separate authority to discard/reset.

## Data And Privacy Impact

The APK remains synthetic-only in `com.lifeos.review.m1`, has no INTERNET
permission, excludes backup/transfer, and accesses no desktop or real profile.
The icon correction adds no provider, credential, historical-context, sync,
production storage, or user-content path.

## Orchestrator Recommendation

Option A only if the complete pale ring is visible and the Founder also accepts
the stable identity and backup/transfer consequence. Otherwise choose B after
visual acceptance, or C if the visual check still fails. This is a
recommendation, not approval.

## Default Safe Action

Take no action: keep all changes unstaged, ADR-0012 Proposed, the temporary
identity synthetic-only, and M2/real data/publication blocked.

## Blocked Files Or Phases

Workflow completion, ADR-0012 acceptance, stable production identity, and all
M2+ work remain blocked. Git staging/commit/push, archive/reset, distribution,
deployment, release, and production storage remain explicitly unauthorized.

## Exact Founder Response Needed

Inspect the currently visible Android icon and confirm:

1. the same dark field, complete pale circular ring, and centered small `LO` as
   the existing PC desktop Life OS icon;
2. the ring is visible at the top, bottom, left, and right with comfortable
   space inside Android's rounded mask;
3. no colorful Tauri placeholder artwork is present.

Exact candidate:

- APK SHA-256: `2e0e41c8b03bff13c3bc4191de7bb972833ea62b8131d119727f158e716f1975`
- canonical PC icon SHA-256: `62d764181ef9137aec875f345345daef4bd398fa828f80cdb35bded563ad2ade`
- package: `com.lifeos.review.m1`
- AVD: `lifeos_m1_api36_x86_64`
- serial: `emulator-5582`

Then send exactly one of:

- `ANDROID-M1-FOUNDER-REVIEW-004 Option A. I visually confirm the complete pale ring is visible on all four sides inside the Android mask and the launcher icon matches the existing PC desktop Life OS icon. I accept the exact unstaged Android M1 review candidate, select com.lifeos.app as the stable production identity, and accept the Proposed ADR-0012 architecture direction. Authorize only recording this decision and completing the current workflow gate. Do not stage, commit, push, archive/reset, release, activate real data, or start M2.`
- `ANDROID-M1-FOUNDER-REVIEW-004 Option B. I visually confirm the complete pale ring is visible on all four sides inside the Android mask and the launcher icon matches the existing PC desktop Life OS icon. I accept only the corrected disposable Android M1 prototype evidence and defer the final identity and ADR-0012 architecture decision. Keep all changes unstaged and do not start M2.`
- `ANDROID-M1-FOUNDER-REVIEW-004 Option C. The visual review still fails because: <exact issue>. Stop the current sprint at this gate. Do not modify, stage, commit, push, archive/reset, release, activate real data, or start M2; any further correction requires a separately authorized scope.`
- `ANDROID-M1-FOUNDER-REVIEW-004 Option D. Reject and cancel this candidate. Do not discard or reset files without separate authorization.`

Silence is not acceptance.

## Resolution Status

resolved

## Exact Founder Response

ANDROID-M1-FOUNDER-REVIEW-004 Option A. I visually confirm the complete pale ring is visible on all four sides inside the Android mask and the launcher icon matches the existing PC desktop Life OS icon. I accept the exact unstaged Android M1 review candidate, select com.lifeos.app as the stable production identity, and accept the Proposed ADR-0012 architecture direction. Authorize only recording this decision and completing the current workflow gate. Do not stage, commit, push, archive/reset, release, activate real data, or start M2.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Record Founder manual acceptance of the exact unstaged Android M1 candidate; select com.lifeos.app as the stable production identity; accept ADR-0012 architecture direction; synchronize decision documentation and complete the current workflow only. No staging, commit, push, archive/reset, release, real-data activation, production deployment, or M2.

## Decided At And Evidence Reference

- Decided at: 2026-09-28T08:58:18.265Z
- Evidence reference: ANDROID-M1-FOUNDER-REVIEW-004

## Resume Phase

theory_alignment_review
