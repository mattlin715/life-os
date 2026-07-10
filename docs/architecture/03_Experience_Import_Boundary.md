---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/08
depends:
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/02_Experience_Export_Boundary.md
  - docs/product/00_MVP_User_Flow.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
referenced_by: []
---

# 03 Experience Import Boundary

## Purpose

This document defines the first import boundary for `ExperienceEntry`.

It exists to complete the first practical data portability loop:

Export user-authored experiences.

Import those same experiences back into Life OS.

The boundary is intentionally narrow.

It does not import meaning, identity, evidence, reflection, patterns, or AI output.

## Why Import Matters For Data Portability

Export proves the user can take their data out.

Import proves the user is not punished for doing so.

For a local-first product, data portability must work both ways at the smallest useful level.

The user should be able to archive, move, reset, or restore their own experience entries without depending on cloud sync or an account system.

## MVP Scope

The MVP import boundary supports only `ExperienceEntry` records exported by Life OS Sprint 9.

Import writes only:

- `id`
- `content`
- `createdAt`
- `updatedAt`

The store maps `content` back to the domain field `body`.

The imported entry remains user-editable.

## Supported Input Format

The only supported input is Life OS JSON export:

```json
{
  "exportedAt": "...",
  "app": "Life OS",
  "version": "0.1",
  "entries": [
    {
      "id": "...",
      "content": "...",
      "createdAt": "...",
      "updatedAt": "..."
    }
  ]
}
```

Markdown import is not supported.

Arbitrary JSON import is not supported.

## Validation Rules

Import must verify:

- `app` equals `Life OS`.
- `version` exists.
- `entries` is an array.
- Each entry has string `id`, `content`, `createdAt`, and `updatedAt`.

Unknown fields are ignored.

Invalid files are rejected before writing anything to the store.

## Duplicate Handling

The MVP uses a conservative duplicate strategy.

If an imported entry id already exists:

- Do not overwrite the existing entry.
- Skip the imported duplicate.
- Report the skipped count to the user.

If the imported entry id does not exist:

- Import the entry.
- Preserve the imported timestamps.

This protects existing user edits from accidental overwrite.

## Privacy Considerations

Import reads local files selected by the user.

Import must not send file content to any external provider.

Import must not interpret content.

Import must not create identity labels, pattern notes, evidence candidates, reflection prompts, or advice.

The import boundary should restore records, not explain them.

## What Import Is Not

Import is not sync.

Import is not merge conflict resolution.

Import is not a full backup system.

Import is not Markdown parsing.

Import is not permission to ingest arbitrary life data.

Import is not a way to introduce AI-generated conclusions into the local store.

## Open Questions

- Should future imports support encrypted archives?
- Should import require preview before writing once records become more complex?
- Should future imports support confirmed evidence only after evidence persistence exists?
- Should a future backup system use a separate archive format and ADR?
