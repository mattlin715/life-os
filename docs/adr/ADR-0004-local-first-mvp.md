---
status: Accepted
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/06
depends:
  - docs/00_Constitution.md
  - docs/10_Privacy.md
  - IMPLEMENTATION_GUIDE.md
referenced_by:
  - docs/architecture/00_MVP_Architecture.md
  - docs/product/00_MVP_User_Flow.md
---

# ADR-0004: Adopt Local-First MVP

## Status

Accepted

## Context

Life OS handles sensitive inner-life data: memories, emotions, identity, reflection, evidence, patterns, and growth.

The MVP must validate the core loop without introducing cloud trust, account systems, sync complexity, or privacy risk.

Book Zero defines privacy as the psychological safety required for honest self-reflection. A first version that depends on hosted storage, account systems, or cloud sync would increase trust requirements before the product has validated its core loop.

## Decision

Life OS MVP will be local-first.

The first version stores user data locally by default.

Cloud sync, accounts, multi-device sync, and hosted storage are out of scope.

## Consequences

- User data remains user-owned.
- Privacy risk is reduced.
- MVP can validate the core loop faster.
- Export/import should be considered early.
- Cloud architecture is deferred.
- Future sync requires new ADR.
- Local-first does not mean local-only forever.
