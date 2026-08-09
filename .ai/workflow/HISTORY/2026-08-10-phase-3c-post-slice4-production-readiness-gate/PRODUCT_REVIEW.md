# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-10-phase-3c-post-slice4-production-readiness-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `c28f5872f321ef0ad2f54f7952fc76f3c5e0be61`
- Working-tree digest reviewed: `26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e`
- Created at: 2026-08-09T18:50:00.000Z
- Updated at: 2026-08-09T18:50:00.000Z

## Mission Interpretation

This sprint is a production-readiness audit and Founder gate. It may correct
the promoted Slice 4C-6B record and create a Proposed Book One gate, but it may
not activate schema v5 or implement the recommended next slice. Promoted
private fixture evidence must remain distinct from production reachability,
real-user safety, Founder authorization, manual acceptance, and release.

## Problem Statement

Life OS now has extensive private, unregistered, disposable evidence for
backup, replacement, migration, restart classification, normalized lifecycle
writes, dependency consequences, and migrated legacy actions. Production still
starts, reads, and writes schema v4. The private modules have no startup,
Tauri, renderer, UI, or real app-data caller. Without a precise audit, the
repository could mistake test depth for cutover readiness or continue building
private parity without crossing the smallest safe production boundary.

## User Value

Before any database upgrade, the user needs calm, exact disclosure of what will
happen, what will not happen, and why the database is or is not ready. The next
step should establish a real product-path disclosure and read-only inspection
boundary without silently migrating, backing up, replacing, or mutating the
user's database.

## Relevant Primary Definitions

- `docs/03_Principles.md`: evidence precedes conclusion; user agency and
  Context Before Insight constrain the transition from fixture evidence to a
  product claim.
- `docs/06_Memory.md`: durable personal memory requires provenance,
  correction, deletion, and consent; maximum retention is not the goal.
- `docs/Reflection.md`: technical migration cannot change the user-owned
  meaning or turn stored hypotheses into truth.
- `docs/09_AI.md`: more stored context increases responsibility, not AI
  authority.
- `docs/10_Privacy.md`: local-first operation still requires transparency,
  purpose limitation, and control over sensitive duplicates.

## Relevant ADRs

- `ADR-0007` requires reviewed artifacts and provenance to survive local
  persistence changes.
- `ADR-0009` keeps consent, transmission, packet snapshots, actual-use
  provenance, invalidation, and deletion exact across any schema transition.
- `ADR-0011` establishes append-only lifecycle, exact dependencies,
  content-free deletion facts, non-destructive feature disable, and no old-
  binary rollback after v5 writes.

No new ADR is needed: the audit introduces no new irreversible policy. The
existing accepted decisions and architecture/13 already govern disclosure,
backup, retention, restore, old-binary refusal, and non-destructive disable.

## Current Implementation Context

- Production `src-tauri/src/sqlite.rs` supports schema v4, inspects the database
  read-only before initialization, rejects versions greater than 4, stabilizes
  v2/v3 to v4, and exposes typed v4 commands.
- `src/shared/storage/createLocalEvidenceStore.ts` blocks store construction
  and all deferred operations when inspection or initialization fails.
- `src-tauri/src/filesystem_safety.rs` and
  `src-tauri/src/schema_v5_migration.rs` are compiled private modules with no
  registered Tauri command or production caller.
- Private Slice 2 through Slice 4C-6B tests cover the fixed DDL, backup and
  replacement contracts, restart classification, canonical writes, lifecycle
  consequences, ADR-0009 parity, and reachable migrated-legacy actions only on
  synthetic/disposable fixtures.
- Production has no v5 fresh initialization, migration disclosure, backup or
  restore activation, v5 read routing, v5 command routing, retention UI, full
  v5 provenance graph, or export v2.
- Provenance Inspector P1 is a production schema-v4, read-only, already-loaded
  Historical Question inspector. It is partial Slice 6 evidence, not the full
  normalized dependency inspector.

## In Scope

- Factual Slice 4C-6B promotion synchronization.
- A 24-area production-readiness matrix and threat model.
- Reconciliation of the original Slice 5 and Slice 6 wording.
- One Proposed architecture/15 gate, minimal Index navigation, exact next-
  slice allowlist, automated plan, manual plan, and Founder decision package.

## Out Of Scope

All runtime code, DDL, schema/user-version changes, real app-data access,
backup/restore/retention activation, Tauri registration, migration UI, v5
reader/writer activation, provider/ContextPacket/consent changes, export v2,
Phase 4, Harness changes, Git promotion, deployment, and release.

## Product Constraints

- A readiness result is evidence, not authorization and not a success claim.
- Opening a disclosure or running a read-only check must never imply migration
  consent.
- Any future migration action must be a separate explicit Founder-approved and
  user-controlled step.
- Current schema-v4 behavior must remain unchanged during this audit.

## Evidence And Provenance Constraints

The matrix must name implemented location, evidence level, reachability,
authority, blocker, and premature-activation severity for every row. Fixture
tests cannot be described as production or real-user evidence. No personal
content may enter workflow records or proposed tests.

## Historical Context Constraints

ADR-0009 semantics must survive any future cutover exactly. Provider receipts
cannot be undone by local restore. Historical Question deletion remains an
artifact-specific cascade rather than a generic lifecycle invalidation.

## Consent Constraints

Database-upgrade disclosure is not historical provider consent and must not
alter it. Existing consent, packet, transmission, and actual-use records remain
untouched in this sprint.

## AI-Role Constraints

No inference, summary, Pattern generation, identity conclusion, diagnosis, or
Phase 4 relationship is introduced. This is local data safety work.

## Privacy Constraints

- No real database or app-data path is accessed by the audit.
- A future read-only readiness check may inspect only the minimum local
  database/path metadata needed for its declared result.
- It must never create a backup, operation state, migration receipt, or other
  sensitive duplicate.
- Errors must not include row content or provider payloads.

## User-Agency Constraints

The future disclosure must be explicit-open, calm, three-language equivalent,
and include a clear close/continue-current-v4 path. It must expose no enabled
Upgrade or Restore action. Silence, opening, closing, or checking never grants
migration authority.

## Acceptance Criteria

For this audit sprint:

1. architecture/13 truthfully records Slice 4C-6B promotion and preserves all
   production fences.
2. architecture/15 contains the full matrix, Slice 5/6 reconciliation, threat
   model, alternatives, recommendation, allowlist, tests, manual checks, and
   exact Founder decision.
3. The recommendation is smaller than v5 activation and cannot mutate a real
   database.
4. No production code, schema, DDL, provider, ContextPacket, or Constitution
   diff exists.
5. Canonical verification passes and the workflow stops at
   `human_decision_required`.

For a separately authorized recommended implementation:

1. An explicit-open local database readiness panel invokes one read-only
   production-path inspection only after the user requests it.
2. The inspection never creates a missing database and never initializes,
   migrates, backs up, restores, checkpoints, cleans sidecars, or writes state.
3. Exact v4, older-than-v4, newer-than-v4, malformed, missing, sidecar-present,
   and inspection-failure states are distinguished without fallback.
4. The UI states that schema-v5 upgrade is unavailable in this slice, provides
   no Upgrade/Restore action, and preserves normal v4 operation.
5. English, Traditional Chinese, and Japanese copy have equal authority.
6. All state is session-only; no consent or readiness preference persists.

## Risks

- A read-only check could be mistaken for migration consent or readiness to
  execute; UI wording and disabled mutation surface must prevent that.
- Inspecting after renderer storage connections are open cannot prove future
  quiescence; the result must say that actual upgrade-time quiescence remains
  unproved.
- Duplicating filesystem rules in a new command could drift from the promoted
  private primitive; the future slice must reuse a narrow read-only seam.
- Proceeding directly to fresh-v5 or existing-v4 activation would mix DDL,
  routing, recovery, disclosure, and real-user risk in one review.

## Open Questions

- Founder decision `PHASE3C-PRODUCTION-READINESS-R1-001` must select the next
  implementation option.
- The recommended Option A does not decide production migration, backup,
  restore, retention, or v5 write activation; those remain later gates.

## Human Decision Required

true: `PHASE3C-PRODUCTION-READINESS-R1-001` after the audit, validation, and
Theory Alignment Review are complete.

## Recommendation

Recommend **Option A: explicit-open production-path read-only readiness and
disclosure only**. It is the first independently reviewable production-path
evidence, creates no migration authority, and leaves existing real-user data
unchanged. Defer fresh-v5 initialization, existing-v4 migration, lifecycle UI,
full inspector, and export v2.

Conditions: no runtime implementation in this sprint; no new ADR; preserve the
exact authority distinctions; and stop for the Founder decision.

## Review Status

approved_with_conditions
