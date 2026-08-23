# Decision Required

Status: resolved
- Sprint ID: 2026-08-23-ordinary-v5-disclosure-manual-correction-r1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-23T04:42:00+09:00
- Updated at: 2026-08-23T04:42:00+09:00

## Decision ID

ORDINARY-V5-DISCLOSURE-CORRECTION-001

## Sprint ID

2026-08-23-ordinary-v5-disclosure-manual-correction-r1

## Decision Summary

Decide whether to authorize one additional, separate, disclosure-only
correction cycle after Founder Manual Phase A exposed three missing facts in the
ordinary schema-v5 pre-migration panel.

## Why Automation Stopped

The parent implementation already consumed its bounded three revision cycles.
The Founder did not activate migration. Step 6A-D1 then proved the disposable
schema-v4 database hash remained unchanged, no operation directory or SQLite
sidecar existed, and no Life OS process was running. Continuing requires new
Founder authority rather than silently exceeding the review limit.

## Relevant Constitution Clauses

- The human founder remains the final constitutional authority.
- Local-first control and informed, revisable user agency must be preserved.
- **We Build Mirrors, Not Oracles.** Schema v5 must not be framed as increasing
  AI truth or authority.

## Relevant Primary Definitions

- docs/06_Memory.md: durable memory remains user-owned and locally governed.
- docs/10_Privacy.md: a local backup is a separate copy of personal data and
  provider retention remains a distinct boundary.
- docs/03_Principles.md: visible consequences support informed control.

## Relevant ADRs

- ADR-0007: reviewed artifact provenance remains preserved.
- ADR-0009: local backup or restore cannot undo provider receipt or retention.
- ADR-0011: append-only lifecycle, revision, and deletion semantics remain
  unchanged.

No ADR status or decision changes are proposed.

## Available Options

### Option A - Authorize one bounded disclosure-only correction

Add ordinary-only English, Traditional Chinese, and Japanese statements that:

1. explain the bounded purpose of schema v5: preserving append-only revisions,
   lifecycle facts, provenance, and exact dependency integrity, without adding
   AI truth or authority;
2. explain that the verified local backup contains the same sensitive personal
   data and should be protected like the live database;
3. explain that older schema-v4 Life OS builds will refuse writes after the
   migration and that Life OS will not downgrade the database.

Render the statements only in the ordinary migration_required panel. Preserve
the Founder Candidate copy and all existing buttons and behavior. Add focused
panel/i18n tests, synchronize architecture/18 factually, run canonical
verification, rebuild one ignored unsigned ordinary review installer, and
resume only Manual Phase A Step 6A.

Anticipated implementation allowlist:

- src/app/FounderSchemaV5MigrationPanel.tsx
- src/app/FounderSchemaV5MigrationPanel.test.tsx
- src/app/i18n.ts
- src/app/i18n.test.ts
- docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md
- repository-required workflow artifacts
- ignored rebuilt installer and manifest only

### Option B - Accept the incomplete disclosure and continue

Continue Manual Phase A without adding the missing purpose, backup-sensitivity,
or older-binary consequence disclosures.

### Option C - Stop the ordinary activation goal

Cancel the ordinary schema-v5 activation candidate and leave the disposable
database at exact schema v4.

## Benefits

- **A:** Restores informed migration authorization without reopening schema,
  migration, backup, restore, or runtime implementation.
- **B:** Avoids another correction cycle and proceeds faster.
- **C:** Avoids all additional activation risk and work.

## Risks

- **A:** Adds one bounded cycle and must keep the copy calm and understandable.
- **B:** Contradicts the accepted disclosure contract and asks the Founder to
  authorize a whole-database migration without material consequences visible.
- **C:** Stops the current product-readiness objective and leaves ordinary
  schema-v5 activation unreviewed.

## Reversibility

- **A:** Copy-only changes are directly reversible before promotion; migration
  behavior and database state remain untouched.
- **B:** The missing disclosure remains until another explicit correction.
- **C:** A future Founder gate can restart the work from the preserved evidence.

## Data And Privacy Impact

Option A does not access or mutate a database. It makes the privacy impact of
the already-authorized verified backup visible before any migration action.
Options B and C also perform no data mutation at this decision gate.

## Orchestrator Recommendation

Select **Option A**. It is the smallest action consistent with informed local
control and does not reopen the migration core.

## Default Safe Action

Keep Life OS closed and keep the disposable database at exact schema v4. Do not
press the migration action or continue the manual matrix until this decision is
resolved.

## Blocked Files Or Phases

All implementation, installer rebuild, and Manual Phase A activity after Step
6A are blocked. Rust, schema, DDL, migration policy, backup/restore behavior,
real profiles, providers, ContextPacket, consent, Phase 4, Android, stage,
commit, push, merge, PR, deployment, distribution, and release remain outside
this decision.

## Exact Founder Response Needed

I resolve ORDINARY-V5-DISCLOSURE-CORRECTION-001 by selecting Option A. I authorize one separate bounded ordinary schema-v5 disclosure-only correction cycle: add English, Traditional Chinese, and Japanese pre-migration statements explaining the bounded schema-v5 purpose, the equal sensitivity of the verified local backup, and older schema-v4 write refusal after migration; render them only for the ordinary migration-required panel; preserve Founder Candidate copy and all migration, backup, restore, schema, provider, ContextPacket, consent, Phase 4, Android, and runtime behavior; add focused panel/i18n tests; synchronize architecture/18 factually; run focused and canonical verification; rebuild one ignored unsigned ordinary review installer; and resume only at Manual Phase A Step 6A. I do not authorize any Rust, schema, DDL, migration-policy, backup/restore, real-profile, provider, Phase 4, Android, staging, commit, push, merge, PR, deployment, distribution, or release change.

Silence does not resolve this decision.

## Resolution Status

resolved

## Exact Founder Response

I resolve ORDINARY-V5-DISCLOSURE-CORRECTION-001 by selecting Option A. I authorize one separate bounded ordinary schema-v5 disclosure-only correction cycle: add English, Traditional Chinese, and Japanese pre-migration statements explaining the bounded schema-v5 purpose, the equal sensitivity of the verified local backup, and older schema-v4 write refusal after migration; render them only for the ordinary migration-required panel; preserve Founder Candidate copy and all migration, backup, restore, schema, provider, ContextPacket, consent, Phase 4, Android, and runtime behavior; add focused panel/i18n tests; synchronize architecture/18 factually; run focused and canonical verification; rebuild one ignored unsigned ordinary review installer; and resume only at Manual Phase A Step 6A. I do not authorize any Rust, schema, DDL, migration-policy, backup/restore, real-profile, provider, Phase 4, Android, staging, commit, push, merge, PR, deployment, distribution, or release change.

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Disclosure-only UI and i18n correction in the ordinary migration-required panel, focused tests, factual architecture/18 sync, verification, one ignored unsigned review installer rebuild, and resume only at Manual Phase A Step 6A; no Rust, schema, migration, backup/restore, runtime-policy, real-profile, provider, Phase 4, Android, Git promotion, deployment, distribution, or release.

## Decided At And Evidence Reference

- Decided at: 2026-08-22T19:29:00.733Z
- Evidence reference: ORDINARY-V5-DISCLOSURE-CORRECTION-001

## Resume Phase

product_review
