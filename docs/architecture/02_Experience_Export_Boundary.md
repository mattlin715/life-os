---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/08
depends:
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/product/00_MVP_User_Flow.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
referenced_by: []
---

# 02 Experience Export Boundary

## Purpose

This document defines the first export boundary for `ExperienceEntry`.

It exists to make user ownership practical.

Local-first storage is not enough if the user cannot take their own records out of the system.

The MVP export boundary is intentionally small: export user-authored experiences in clear portable formats without adding interpretation, hidden metadata, or backup complexity.

## Why Export Matters For User Ownership

Life OS handles sensitive self-reflection data.

That data must not feel trapped inside the app.

Export supports:

- Trust: the user can inspect what the app holds.
- Agency: the user can move, archive, or delete their own records.
- Privacy: the user does not need cloud sync to preserve a copy.
- Continuity: the user can keep a readable record outside Life OS.

Export is part of the local-first promise.

If the user cannot leave with their data, the system is not fully user-owned.

## MVP Scope

The first export boundary supports only `ExperienceEntry`.

Each exported entry includes:

- `id`
- `content`
- `createdAt`
- `updatedAt`

The export does not include:

- Evidence candidates.
- Reflection prompts.
- Pattern notes.
- Identity labels.
- AI interpretation.
- Provider metadata.
- Hidden analytics.

This boundary exports what the user wrote and the minimal timestamps needed to preserve context.

## Supported Formats

The MVP supports two formats:

| Format | Purpose |
| --- | --- |
| JSON | Portable structured data for future tooling, backup, or migration. |
| Markdown | Human-readable archive that can be opened without Life OS. |

Both formats include:

- Export timestamp.
- App name.
- Export version.
- Experience entries.

The desktop implementation uses the Tauri save dialog and filesystem plugin.

Browser download fallback remains available only for frontend-only development outside the Tauri runtime.

The JSON export is the only format supported by the first import boundary.

Markdown remains export-only.

## What Export Is Not

Export is not arbitrary import.

Export is not sync.

Export is not a full backup system.

Export is not a migration framework.

Export is not a way to smuggle AI conclusions into the user's archive.

The boundary should stay boring and trustworthy.

## Privacy Considerations

Exports may contain sensitive personal writing.

The app should avoid sending exported content to any provider.

The app should avoid adding invisible metadata.

The app should avoid combining user-authored content with unconfirmed AI output.

The user is responsible for where they save or share the exported file, but Life OS is responsible for making the export honest and readable.

## Open Questions

- Should future exports support encrypted archives?
- Should export include confirmed evidence only after evidence persistence exists?
- Should import require a separate ADR before implementation?
- Should export versioning become part of a broader local data migration strategy?
