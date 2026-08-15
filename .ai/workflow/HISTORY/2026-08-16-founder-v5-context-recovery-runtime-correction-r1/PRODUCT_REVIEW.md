# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-16-founder-v5-context-recovery-runtime-correction-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Working-tree digest reviewed: f6c186d646db0c5ffe1503bb3f4606f2d50fbf8571ae1cd3b965a327fc53067a
- Created at: 2026-08-16T12:35:00+09:00
- Updated at: 2026-08-16T12:35:00+09:00

## Mission Interpretation

Repair one proven Candidate-only runtime integration defect and make failure
visible at the same Context Recovery interaction. Do not broaden schema-v5,
Context Recovery, or product authority.

## Problem Statement

The persisted artifact kind is `recovery_turn`, but the real v5 runtime facade
queries `context_recovery` before saving an answer or skip. This fails before
the governed writer transaction. The general storage error is outside the
visible Context Recovery area, producing an apparently inert action.

## User Value

A Founder can save the event context they explicitly entered and immediately
understand any future failure without wondering whether the action was ignored.

## Relevant Primary Definitions

`docs/03_Principles.md`, `docs/06_Memory.md`, `docs/Reflection.md`,
`docs/09_AI.md`, and `docs/10_Privacy.md`: Context Recovery remains explicit,
current-task-scoped, user-owned, and historically ineligible.

## Relevant ADRs

ADR-0009 remains the historical consent/provenance authority. ADR-0011 remains
the append-only revision and lifecycle authority. Neither is changed.

## Current Implementation Context

Candidate R1 is implemented but unpromoted on the current working branch. The
parent sprint is archived as incomplete. The private writer consistently uses
`recovery_turn`; only the facade lookup differs. Read-only manual evidence proves
the failed answer created no partial revision and the ordinary profile remained
unchanged.

## In Scope

- Change the one runtime lookup discriminator to `recovery_turn`.
- Add a Rust regression that creates and answers a recovery turn through the
  real `save_artifacts` facade.
- Show a calm localized error within the active Context Recovery region when a
  save/skip mutation fails.
- Add focused UI and three-language tests.
- Synchronize factual Candidate documentation and build a new unsigned package.

## Out Of Scope

Schema/DDL or migration-policy changes; automatic retry; any profile mutation
by automation; ordinary profile or real-user data; new Context Recovery action;
provider, ContextPacket, consent, Phase 4, Android, Harness changes; Git
promotion, distribution, deployment, or release.

## Product Constraints

The successful behavior must remain exactly the existing explicit first answer
or skip. The error disclosure must not imply repair, retry, or data loss.

## Evidence And Provenance Constraints

The exact current Experience and immutable prompt lineage remain revalidated by
the promoted writer. No provenance field or artifact discriminator is renamed.

## Historical Context Constraints

Context Recovery remains categorically excluded from historical eligibility,
selection, consent, packets, and longitudinal memory.

## Consent Constraints

No consent behavior changes.

## AI-Role Constraints

No AI call or inference behavior changes. The fix is local persistence routing
and visible failure only.

## Privacy Constraints

All automated tests use disposable fixtures. The current Founder and ordinary
profiles must not be opened or mutated by automation.

## User-Agency Constraints

Only explicit Save or Skip may invoke the writer. Failure remains fail closed
and visible beside the action; no automatic replay occurs.

## Acceptance Criteria

1. Runtime lookup uses the persisted `recovery_turn` discriminator.
2. A disposable exact-v5 fixture creates a suggestion and saves its first answer
   through `save_artifacts`, producing exact projection/head/revision evidence.
3. Failure creates no optimistic durable UI claim and renders a localized alert
   inside the same Context Recovery region.
4. Success clears the local alert.
5. English, Traditional Chinese, and Japanese copy are equivalent.
6. Schema/DDL, production `SCHEMA_VERSION`, provider, ContextPacket, consent,
   Phase 4, and ordinary-profile behavior remain unchanged.
7. Focused tests, Clippy, canonical verification, and unsigned package build pass.

## Risks

A too-broad UI refactor could alter ordinary v4 behavior. Mitigation: extract
only the existing Context Recovery markup and keep state/session semantics local.

## Open Questions

none; Founder Decision 004 provides exact authority.

## Human Decision Required

false. Manual installation and affected Step 11D-3 remain Founder-owned after
implementation.

## Recommendation

Implement the minimal correction with exact facade and UI tests.

## Review Status

approved_with_conditions
