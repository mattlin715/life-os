# Sprint Report

Status: completed_with_follow_up

## Sprint ID

2026-08-11-historical-relevance-origin-disclosure-correction

## Mission

Make explicit-open local historical relevance inspectable without changing retrieval or governance.

## Starting Commit

76bc4addd954cd14a4ab82f3e4a2369efaab8820

## Ending Commit Or Working-Tree State

Same HEAD with unstaged product, test, audit, and workflow changes.

## Final Status

completed_with_follow_up

## Product Decision

Preserve local-lexical-v2 and governed aggregate relevance; disclose exact local match origin and bounded excerpt.

## Engineering Summary

Added optional ephemeral visibleMatches and a localized candidate relevance component.

## Behavior Changed

Candidate cards now show where each shared term actually appeared.

## Files Changed

Historical retrieval types/implementation/test, App, new focused component/test, i18n/test, and workflow evidence.

## Tests

Focused 45 passed; canonical 40 files and 303 Vitest, 189 Rust, 12 backup/restore, and 8 schema-contract tests passed.

## Repository Verification

passed

## Manual Verification

Founder Step 9R pending.

## Architecture Updates

none

## ADR Updates

none

## Documentation Synchronization

Workflow evidence only before Founder acceptance.

## Data And Migration Impact

none; SCHEMA_VERSION remains 4.

## Provenance And Consent Impact

No packet, digest, consent, selection, provider, or persistence change.

## Risks

Lexical overlap may still be low-value; origin disclosure makes that limitation inspectable rather than hiding it.

## Deferred Items

Semantic retrieval, sensitive taxonomy, Phase 4, and any ranking-policy change.

## Human Decisions

Founder already authorized the exact bounded correction; Step 9R remains.

## Review Cycles

Cycle 0 corrected workflow-only whitespace.

## Workflow Lessons

Real dogfooding exposed an explainability defect that aggregate automated relevance tests did not reveal.

## Recommended Next Sprint

Resume the existing Phase 3 Product Exit walkthrough after Step 9R.

## Git Status

No staged files, commit, push, merge, PR, deployment, or release.
