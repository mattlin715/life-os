# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-18T21:20:00+09:00
- Updated at: 2026-08-18T21:20:00+09:00

## Sprint ID

2026-08-18-desktop-schema-v5-ordinary-production-activation-r1

## Mission

Implement the bounded ordinary Windows desktop schema-v5 activation candidate,
verify it only with synthetic/disposable evidence, build an unsigned disposable
review package, and stop before any ordinary-profile or real-user operation.

## Starting Commit

`44ec6d56d645829488aa73d0b92bcf72b19487f4` on clean `develop`, matching the
local `origin/develop` reference.

## Ending Commit Or Working-Tree State

HEAD remains `44ec6d56d645829488aa73d0b92bcf72b19487f4` on
`codex/desktop-schema-v5-ordinary-production-activation-r1`. The implementation,
tests, documentation, and workflow evidence are unstaged. Before archival, 49
tracked or non-ignored untracked paths matched the Engineering Plan allowlist.
No commit, push, merge, PR, installation, deployment, distribution, or release
occurred.

## Final Status

completed_with_follow_up

## Product Decision

The Founder-authorized goal permits an ordinary-identity, disposable-review
implementation and automated evidence only. Manual Phase A remains a separate
Founder action. Any one-time migration of the real Founder-owned ordinary
profile remains Phase B and requires a new exact authorization after Phase A.

## Engineering Summary

Application version declarations are synchronized at `0.3.0`. One shared Rust
activation core now serves the ordinary and Founder identities without
duplicating DDL or typed writers. A schema-v5-capable ordinary build initializes
a missing database as exact v5, discloses exact-v4 migration and requires an
explicit action, stabilizes v2/v3 only through the existing typed v4 path before
reinspection, routes valid v5 through typed stores, and blocks newer, malformed,
sidecar-bearing, ambiguous, or path-unsafe states. The legacy no-default build
continues to support v4 only and refuses v5 writes.

## Behavior Changed

Only a schema-v5-capable ordinary desktop build gains explicit local migration,
backup, restore, delete-now, blocked-state, and feature-aware readiness
disclosures. Opening the application or a disclosure does not authorize a
migration. Existing provider, consent, ContextPacket, inference, and Phase 4
behavior is unchanged.

## Files Changed

Before workflow archival, the exact 49-path allowlist covered current workflow
artifacts; CI and verification; version and package contracts; Rust activation,
migration, readiness, and typed Experience validation; TypeScript storage and
UI adapters; three-language disclosure and tests; architecture 13/15/16/17/18;
development runbooks 09/10; and Tauri ordinary-review configuration. No Android,
real-profile, generated installer, manifest, build-output, or credential path is
tracked.

## Tests

- Workflow: 17 passed.
- Package contracts: Founder dogfood 8, Founder Candidate 1, ordinary review 2.
- Vitest: 46 files / 344 tests passed.
- Rust library: 200 tests passed.
- Backup/restore integration: 12 passed.
- Schema-v5 contract: 8 passed.
- Legacy no-default refusal: 33 passed.
- Ordinary activation: 6 passed.
- Founder activation: 6 passed.
- Founder runtime: 4 passed.
- Clippy all targets/all features with warnings denied: passed.
- TypeScript typecheck, frontend build, Rust check, UTF-8, whitespace, secret,
  Markdown-link, and Constitution checks: passed.

## Repository Verification

Canonical `scripts/verify.ps1` passed at working-tree digest
`7fda57e5385e31835eed2dac4f54ce2c8e91653861dd0e66243438eef2fdc60b`.

## Manual Verification

Not run. The next gate is Founder Manual Phase A in a disposable Windows
account, VM, or Sandbox. No pass is inferred from automation or package creation.

## Architecture Updates

Architecture/18 defines the bounded ordinary activation implementation,
authority fences, threat model, recovery semantics, and manual matrix.
Architecture/13, 15, 16, and 17 are synchronized factually without changing
constitutional or ADR authority.

## ADR Updates

None. ADR-0007, ADR-0009, and ADR-0011 remain unchanged and authoritative.

## Documentation Synchronization

Index and Book One architecture records now distinguish the promoted Founder
Candidate from this unpromoted ordinary candidate. Development runbook 10
records the disposable-only Windows review procedure and exact Phase A/Phase B
separation.

## Data And Migration Impact

Automation used synthetic/disposable databases only. No installer was installed
or launched, no real `%APPDATA%\\com.lifeos.app` path was accessed, and no user
database was initialized, migrated, backed up, restored, repaired, or deleted.
The legacy source constant `SCHEMA_VERSION = 4` remains the v4 compatibility
boundary; schema-v5 activation is feature-gated and does not change ordinary
user data without a later explicit action.

## Provenance And Consent Impact

Promoted migration receipts, exact dependency/provenance checks, and typed
artifact writers are reused. Receipt validation accepts promoted `0.2.0`
Candidate receipts through current `0.3.0` while new receipts record `0.3.0`.
Provider, ContextPacket, historical eligibility, retention policy, and consent
semantics are unchanged.

## Risks

Automated disposable evidence does not establish real ordinary-profile safety.
Unsigned packaging, Windows installation, explicit migration UI, restart,
backup disclosure, restore/delete controls, and three-language behavior still
require Phase A manual observation. A real Founder-profile migration remains a
separate higher-risk Phase B decision.

## Deferred Items

Founder Manual Phase A, Founder diff acceptance, and any promotion. Phase B
real-profile migration, Private Alpha distribution, Android, Phase 4, provider
or consent changes, deployment, and release remain unauthorized.

## Human Decisions

No unresolved implementation-policy decision remains. Manual Phase A is the
next explicit Founder gate. Phase B must not be asked or inferred until Phase A
passes.

## Review Cycles

Three of three bounded cycles were used: migration-receipt version compatibility,
shared Experience receipt verification, and feature-aware readiness truth plus
package-contract reconciliation. Final Theory Alignment Review is
`approved_with_follow_up`.

## Workflow Lessons

Application-version bumps require compatibility with promoted immutable receipt
versions rather than exact current-version equality. Runtime writers and
read-only diagnostics must share the same schema/receipt truth. Release package
checks must combine source/bundle identity with binary-observable behavior
because optimization may remove literal identifiers.

## Recommended Next Sprint

None automatically. Perform Founder Manual Phase A one bounded step at a time.
Only after Phase A passes may the Founder consider an exact Phase B authorization
for one migration of the Founder-owned ordinary profile.

## Git Status

Branch `codex/desktop-schema-v5-ordinary-production-activation-r1`; HEAD
`44ec6d56d645829488aa73d0b92bcf72b19487f4`; no staged files; no upstream;
implementation and workflow evidence remain unpromoted. Ignored installer and
manifest output remain outside Git.
