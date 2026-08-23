# Product Review

Status: approved

- Sprint ID: 2026-08-23-ordinary-v5-disclosure-manual-correction-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest reviewed: 8b223206e00e2b8097f851adcf50b1a881fb40149b23886f32c5d9ddf0c8cf28
- Created at: 2026-08-23T04:35:00+09:00
- Updated at: 2026-08-23T04:35:00+09:00

## Mission Interpretation

Correct only the missing ordinary pre-migration disclosure facts found during
Founder Manual Phase A, then rebuild the ignored disposable installer and resume
the exact affected Step 6A. The correction is not migration authority.

## Problem Statement

The visible disclosure proves explicit authorization, local-only scope,
verified backup, 30-day retention, provider-retention limits, and write-free
cancellation. It does not disclose why schema v5 is needed, that the backup is
an equally sensitive local copy, or that the legacy schema-v4 build will refuse
writes after migration. These were explicit requirements of the authorized goal.

## User Value

Before a whole-database migration, the user can understand the purpose, local
privacy consequence, and compatibility consequence rather than consenting to a
technical version change without meaningful context.

## Relevant Primary Definitions

- `docs/06_Memory.md`: durable memory remains user-owned and locally governed.
- `docs/10_Privacy.md`: local copies and provider retention are distinct privacy
  boundaries.
- `docs/03_Principles.md`: visible consequences support user agency and informed
  control.

## Relevant ADRs

- ADR-0007: reviewed artifact provenance remains preserved.
- ADR-0009: local restore cannot undo provider receipt/retention.
- ADR-0011: append-only lifecycle and deletion semantics remain unchanged.

No ADR change is proposed.

## Current Implementation Context

The ordinary activation implementation is unpromoted but automated-verified.
Manual Phase A has passed package identity, disposable-account isolation,
corrected legacy-v4 startup, one Experience/Evidence/Reflection journey, exact
schema-v4 baseline, and v5 installer no-write installation. Step 6A exposed the
copy gap before any cancel or migration action. Step 6A-D1 proved the database
hash unchanged, schema v4, zero operation directories, zero sidecars, and zero
running Life OS processes.

## In Scope

- Add three ordinary-only disclosure facts in English, Traditional Chinese, and
  Japanese: bounded purpose, backup sensitivity, and older-binary write refusal.
- Render them only in the ordinary migration-required panel.
- Add focused panel and locale-parity tests.
- Synchronize architecture/18 with the truthful manual observation.
- Run focused tests, canonical verification, rebuild an ignored unsigned review
  installer, and resume only at Step 6A.

## Out Of Scope

Rust, schema, DDL, migration/recovery state machines, backup/restore behavior,
retention policy, provider, ContextPacket, consent, Phase 4, Android, real
profiles, distribution, deployment, release, stage, commit, push, merge, and PR.

## Product Constraints

Copy must be calm, bounded, non-coercive, and understandable without exposing a
full path or promising schema v5 solves product meaning. It must not imply that
opening the panel or reading disclosure authorizes migration.

## Evidence And Provenance Constraints

No evidence content or provenance changes. Schema v5 purpose is described only
as preserving revision/lifecycle/dependency/provenance structure, not as adding
truth or AI authority.

## Historical Context Constraints

No historical eligibility, retrieval, packet, transmission, or Phase 3B change.

## Consent Constraints

Migration authorization remains separate from provider consent. The correction
adds information before the existing explicit per-operation action only.

## AI-Role Constraints

No AI inference, prompt, provider, model, output, or authority change.

## Privacy Constraints

The backup-sensitivity statement must explain that the verified backup contains
the same local personal data and should be protected like the live database.

## User-Agency Constraints

Cancel remains write-free. The user sees old-binary refusal before migration and
retains separate explicit migration, backup deletion, and restore actions.

## Acceptance Criteria

1. Ordinary English, Traditional Chinese, and Japanese disclosure visibly state
   all three missing facts.
2. Founder Candidate disclosure and behavior remain unchanged.
3. Buttons and migration semantics remain unchanged.
4. Focused panel/i18n tests and canonical verification pass.
5. Rebuilt ignored installer has a new exact hash and passes package contracts.
6. Founder repeats Step 6A only; schema-v4 baseline remains unchanged before it.

## Risks

Copy can become overly technical or alarming. Bounded wording and locale tests
reduce this risk. Proceeding without correction would weaken informed migration
authorization. A fourth unapproved revision would violate the prior cycle limit.

## Open Questions

Resolved by Founder selection of Option A in
`ORDINARY-V5-DISCLOSURE-CORRECTION-001`.

## Human Decision Required

false. The exact Founder response is preserved in `DECISION_REQUIRED.md`.

## Recommendation

Authorize one separate bounded disclosure-only correction sprint. This is safer
than accepting incomplete disclosure and does not reopen the migration core.

## Review Status

approved
