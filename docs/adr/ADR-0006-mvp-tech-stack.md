---
status: Accepted
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/06
depends:
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/architecture/00_MVP_Architecture.md
referenced_by: []
---

# ADR-0006: Adopt Tauri + React + SQLite for MVP

## Status

Accepted

## Context

Life OS MVP must run on both Work Mac and Windows 11.

The MVP is local-first, privacy-sensitive, and should validate the core loop before cloud sync, mobile app, or complex architecture.

The repo currently has no `package.json` and `src/` is empty.

## Decision

Adopt Tauri + React + SQLite as the MVP tech stack.

## Consequences

- Desktop-first prototype supports Mac and Windows.
- SQLite supports local-first storage.
- Tauri is lighter than Electron.
- React enables fast MVP UI iteration.
- Local model integration remains possible later.
- Mobile app is deferred.
- Cloud sync is deferred.
- The first code sprint should only scaffold the app and module boundaries.

## Rejected Alternatives

- Electron + React + SQLite: viable but heavier.
- Web-only local prototype: fastest but weaker for desktop/local file/database integration.
- Flutter: strong cross-platform option but less aligned with current rapid desktop prototype path.
