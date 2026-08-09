# Product Review

Status: approved

- Sprint ID: 2026-08-10-phase-3c-database-readiness-inspector-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: bed87283f9141144a1c11500457363d5a8081aed
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-09T20:30:00Z
- Updated at: 2026-08-09T20:30:00Z

## Mission Interpretation

Implement the already Founder-authorized architecture/15 Option A: a user-opened, session-only and read-only view that reports bounded readiness facts about the production database path without changing that path or offering a write-capable action.

## Problem Statement

Life OS can refuse incompatible databases during startup and has extensive private disposable schema-v5 evidence, but the user has no calm, explicit way to inspect the local production-path readiness boundary. Infrastructure evidence must not be mistaken for production activation.

## User Value

The user can ask Life OS what it can safely observe about its local database, understand why a future operation would remain blocked, and close the disclosure without initiating migration, backup, repair, or any other state change.

## Relevant Primary Definitions

- `docs/03_Principles.md`: local-first control, evidence before conclusion, visible uncertainty, and user agency.
- `docs/06_Memory.md`: stored memory remains user-owned and must not be silently expanded or transmitted.
- `docs/09_AI.md`: the inspector is deterministic local software, not an AI interpretation or oracle.
- `docs/10_Privacy.md`: inspect only minimum local metadata after explicit user action.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`: existing persistence provenance remains unchanged.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`: no retrieval, consent, packet, or provider behavior changes.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`: future lifecycle authority does not authorize schema-v5 runtime activation.

## Current Implementation Context

- Implemented and promoted: schema-v4 production storage, startup compatibility refusal, and private/disposable Phase 3C evidence.
- Founder-approved and promoted: architecture/15 Option A and its exact implementation allowlist at merge `bed87283`.
- Not implemented: Database Readiness Inspector R1.
- Not authorized: production schema-v5 activation, migration, backup/restore, v5 routing, or real-user migration testing.

## In Scope

- One explicit-open readiness panel with a separate explicit `Check now` action.
- One bounded Tauri inspection command and typed TypeScript adapter.
- Read-only database/path/version/sidecar/quiescence and owned-operation metadata only.
- Calm English, Traditional Chinese, and Japanese disclosure.
- Synthetic/disposable destructive and malformed test cases.
- Only the architecture/15 maximum allowlist and repository-required workflow artifacts.

## Out Of Scope

All database creation, initialization, mutation, migration, backup, restore, checkpoint, cleanup, repair, candidate selection, retry, UI persistence, schema-v5 routing, lifecycle UI, export v2, provider behavior, Phase 4, deployment, and release.

## Product Constraints

The panel is closed by default. Opening it is not a check. Every check is explicit and fresh. It exposes no Upgrade, Backup, Restore, Delete, Repair, or Retry Migration control. Normal schema-v4 startup behavior is unchanged.

## Evidence And Provenance Constraints

The inspector reports bounded structural metadata only. It creates no provenance record, receipt, audit event, manifest, or operation state and must not expose raw database content, paths, SQL errors, secrets, or provider data.

## Historical Context Constraints

No historical retrieval, selection, packet assembly, transmission, or generated-artifact inspection occurs.

## Consent Constraints

No consent is requested, inferred, reused, or persisted. The explicit check authorizes only this local read-only observation.

## AI-Role Constraints

No AI inference is involved. Readiness classifications are deterministic bounded software outcomes and must not imply that migration is safe or authorized.

## Privacy Constraints

Return only classification, schema version when safely readable, sidecar presence, quiescence uncertainty, bounded operation-evidence status, supported maximum, and check time. Do not return local paths or raw errors.

## User-Agency Constraints

The user chooses when to open, when to check, and when to close. Closing changes nothing. Session state is not persisted. A blocked or uncertain result offers guidance only, never an automatic action.

## Acceptance Criteria

1. Opening the panel causes no command invocation; checking requires an explicit button action.
2. The command creates or changes no database, directory, sidecar, operation evidence, or preference.
3. Missing, older-supported, exact-v4, newer-unsupported, malformed, unreadable/path-unsafe, and recovery-required evidence produce bounded classifications.
4. Sidecars, non-proven quiescence, and owned-operation ambiguity are disclosed without cleanup or candidate selection.
5. No local path, raw SQL/filesystem error, content, Upgrade, Backup, Restore, Delete, or retry-migration control appears.
6. Repeated checks are fresh; disclosure state is session-only and closes without mutation.
7. English, Traditional Chinese, and Japanese meanings are equivalent.
8. Normal schema-v4 startup/product behavior and `SCHEMA_VERSION = 4` remain unchanged.
9. Focused tests, Clippy with warnings denied, canonical verification, Theory Alignment Review, and Founder manual UI review are completed before promotion.

## Risks

- SQLite or filesystem APIs can accidentally create files unless every open path is explicitly read-only and create-disabled.
- Sidecar or operation evidence can be mistaken for safety; ambiguity must classify as recovery required.
- Raw backend errors or local paths could leak through the command boundary.
- UI copy could imply migration readiness or provide a prohibited action.

## Open Questions

none; architecture/15 and the exact Founder Option A resolution govern this implementation.

## Human Decision Required

false; decision `PHASE3C-PRODUCTION-READINESS-R1-001` was already resolved as Option A and promoted.

## Recommendation

Proceed with the smallest allowlisted R1 implementation, verify exclusively with synthetic/disposable destructive fixtures, then stop for Founder diff and manual UI review.

## Review Status

approved
