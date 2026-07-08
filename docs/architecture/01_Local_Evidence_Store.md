---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/08
depends:
  - docs/architecture/00_MVP_Architecture.md
  - docs/product/00_MVP_User_Flow.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
referenced_by: []
---

# 01 Local Evidence Store

## Purpose

This document defines the first local data boundary for Life OS MVP.

It does not define the final database schema.

It does not define the full memory system.

It defines the minimum boundary needed to let the product save user-authored experiences locally while preserving privacy, agency, and evidence discipline.

## What The Local Store Is Responsible For

The local store is responsible for preserving user-owned records used by the MVP core loop:

Experience  
-> Evidence  
-> Reflection  
-> Awareness  
-> Growth

For the first boundary, the local store must support:

- Creating an experience entry.
- Listing experience entries.
- Reading one experience entry.
- Updating a user-authored experience entry.
- Deleting a user-authored experience entry.

The store must treat user text as user-owned data.

The store must make deletion possible.

The store must remain inspectable enough for future export.

## What It Is Not Responsible For

The local store is not responsible for:

- Deciding what an experience means.
- Producing identity labels.
- Running AI analysis.
- Confirming evidence automatically.
- Diagnosing the user.
- Ranking, recommending, or nudging engagement.
- Defining the final Memory Model.
- Defining the final SQLite schema.

Storage preserves records.

It does not create truth.

## MVP Minimal Data Boundary

The first data boundary contains four domain records:

| Record | Purpose |
| --- | --- |
| `ExperienceEntry` | User-authored life experience text. |
| `EvidenceCandidate` | Possible evidence extracted from an experience. |
| `ReflectionPrompt` | A question proposed for reflection. |
| `PatternNote` | A possible pattern note across evidence or entries. |

`ExperienceEntry` is persisted in SQLite.

`EvidenceCandidate` is wired into the first UI review boundary as session-only data.

`ReflectionPrompt` and `PatternNote` remain minimal types so future work can connect the MVP loop without inventing new concepts.

## User Ownership Requirements

User-owned records must support:

- Inspect: the user can see what is stored.
- Edit: the user can correct their own record.
- Delete: the user can remove records.
- Export: the user can take experience entries out in portable formats.
- Import: the user can restore Life OS JSON experience exports.

The current boundary supports inspect, edit, delete, export, and import for `ExperienceEntry`.

## AI Output Remains Candidate

AI-generated records must never become product truth automatically.

Evidence, reflection prompts, and pattern notes should use candidate-oriented status values:

- `candidate`
- `confirmed`
- `rejected`

Only the user can confirm or reject candidate records.

This boundary protects the distinction between:

- What the user wrote.
- What the system proposed.
- What the user confirmed.

## Evidence Candidate Boundary

The first evidence candidate boundary exists in the UI.

It supports:

- Generating mock evidence candidates from one `ExperienceEntry`.
- Showing candidate text, kind, and status.
- Letting the user confirm a candidate.
- Letting the user reject a candidate.

Evidence candidates are currently session-only.

They are not persisted to SQLite.

They are not exported.

They are not imported.

They do not create reflection prompts, pattern notes, identity labels, advice, or diagnosis.

The mock provider is intentionally shallow.

It exists to validate the review boundary, not to prove analysis quality.

See `docs/architecture/04_Evidence_Candidate_Boundary.md`.

## SQLite Boundary

SQLite is the accepted local persistence direction for the MVP.

The first persistence spike stores only `ExperienceEntry`.

It does not persist evidence candidates, reflection prompts, or pattern notes.

The schema is intentionally minimal:

```sql
CREATE TABLE IF NOT EXISTS experience_entries (
  id TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

The SQLite implementation maps `content` to the domain field `body`.

The domain field `userEditable` is not persisted yet because it is currently an invariant of user-authored experience entries.

This spike does not create a migration framework.

It uses `CREATE TABLE IF NOT EXISTS` during store initialization.

SQLite is implemented through the Tauri SQL plugin.

In-memory storage remains available for frontend-only development outside the Tauri desktop runtime.

## Export Boundary

The first export boundary exists for `ExperienceEntry`.

It supports:

- JSON export for portable structured data.
- Markdown export for human-readable archives.

Desktop export uses the Tauri save dialog and filesystem plugin.

Frontend-only development may still use browser download fallback.

Export includes only user-authored experience entries and minimal timestamps.

It does not include evidence candidates, reflection prompts, pattern notes, identity labels, AI interpretation, provider metadata, or hidden analytics.

Import is intentionally deferred.

See `docs/architecture/02_Experience_Export_Boundary.md`.

## Import Boundary

The first import boundary exists for `ExperienceEntry`.

It supports only Life OS JSON exports from the export boundary.

Import does not support Markdown.

Import does not support arbitrary JSON.

Imported entries preserve their `id`, `content`, `createdAt`, and `updatedAt`.

If an imported entry id already exists, the imported duplicate is skipped and the existing entry is not overwritten.

Desktop import uses the Tauri open dialog and filesystem plugin.

Frontend-only development may use a browser file input fallback.

Import does not create evidence candidates, reflection prompts, pattern notes, identity labels, AI interpretation, provider metadata, or hidden analytics.

See `docs/architecture/03_Experience_Import_Boundary.md`.

## Deferred Persistence

The following records are intentionally not persisted yet:

- `EvidenceCandidate`
- `ReflectionPrompt`
- `PatternNote`

They remain candidate-oriented domain types.

Persisting them requires a separate schema decision because AI output must remain inspectable, editable, rejectable, and never automatically treated as truth.

## Open Questions

- Should rejected AI output be stored, discarded, or stored only with explicit user consent?
- How should deleted entries affect related evidence, reflection prompts, and pattern notes?
- What export/import versioning guarantees are needed before supporting more record types?
- How should schema versioning be introduced without over-engineering the MVP?
- What local encryption level is required before the first private daily-use prototype?
