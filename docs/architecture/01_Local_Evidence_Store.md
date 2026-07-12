---
status: Draft
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/12
depends:
  - docs/architecture/00_MVP_Architecture.md
  - docs/product/00_MVP_User_Flow.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0006-mvp-tech-stack.md
referenced_by:
  - docs/architecture/07_Persisted_Context_Recovery_Vertical_Slice.md
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

`ExperienceEntry`, `EvidenceCandidate`, `ReflectionPrompt`, and `PatternNote` are persisted locally in SQLite. Artifact records remain source-scoped and reviewable; persistence does not make a candidate true.

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

Evidence and pattern notes should use candidate-oriented status values:

- `candidate`
- `confirmed`
- `rejected`

Only the user can confirm or reject candidate records.

Reflection prompts use a separate lightweight lifecycle:

- `suggested`
- `answered`
- `skipped`

This keeps questions distinct from evidence truth states.

This boundary protects the distinction between:

- What the user wrote.
- What the system proposed.
- What the user confirmed.

## Evidence Candidate Boundary

The first evidence candidate boundary exists in the UI.

It supports:

- Generating evidence candidates from one `ExperienceEntry` through the shared Harness.
- Showing candidate text, kind, and status.
- Letting the user edit candidate text before confirmation.
- Letting the user confirm a candidate.
- Letting the user reject a candidate.

Evidence candidates persist locally with their source entry and review state.

They are not exported.

They are not imported.

They do not create reflection prompts, pattern notes, identity labels, advice, or diagnosis.

OpenAI or Gemini receives only the validated task-specific Context Packet when configured; otherwise the local mock provides the same bounded fallback. Provider-side processing is distinct from local SQLite persistence.

See `docs/architecture/04_Evidence_Candidate_Boundary.md`.

## Reflection Prompt Boundary

The first reflection prompt boundary exists in the UI.

It supports:

- Generating reflection prompts from confirmed evidence candidates only.
- Showing open-ended questions under the source entry.
- Letting the user write an optional answer.
- Letting the user skip a prompt.
- Showing a per-entry reflection summary.

Reflection prompts, responses, and skipped state persist locally with their source entry.

They are not exported.

They are not imported.

They do not create pattern notes, growth notes, identity labels, advice, diagnosis, or conclusions.

The local mock remains a fallback. Real-provider calls use the same validated Context Packet and do not change the review boundary.

See `docs/architecture/05_Reflection_Prompt_Boundary.md`.

## Pattern Candidate Boundary

The first pattern candidate boundary exists in the UI.

It supports:

- Generating one pattern candidate from confirmed evidence.
- Optionally using answered reflection prompt responses as user-authored context.
- Showing the candidate as a hypothesis for review.
- Letting the user confirm a candidate.
- Letting the user reject a candidate.
- Showing a per-entry pattern summary.

Pattern candidates and review state persist locally with their source entry.

They are not exported.

They are not imported.

They do not create growth notes, identity labels, advice, diagnosis, MBTI/personality types, scores, or conclusions.

Only saved user-authored reflection responses may enter a Pattern Context Packet. An unsaved UI draft is not a durable artifact and cannot be sent to a provider.

See `docs/architecture/06_Pattern_Candidate_Boundary.md`.

## SQLite Boundary

SQLite is the accepted local persistence direction for the MVP.

Schema version 3 adds the source-scoped `persisted_artifacts` table. Payloads preserve domain records, source references, review state, and provenance. The v2-to-v3 migration runs on one SQLx connection inside a transaction and advances `user_version` only after success.

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

Initialization uses SQLite `PRAGMA user_version` migrations. Experience deletion explicitly removes all dependent artifact rows, preventing orphaned insight.

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

## Reuse Boundary

Rejected evidence and patterns are excluded from durable storage. Dependent reflections are removed when their source evidence is rejected. Providers receive only a validated current-Experience Context Packet; artifact export/import remains deliberately Experience-only.

## Open Questions

- Future consent-based retention of rejected artifacts for evaluation is a governance question; current behavior removes rejected artifacts rather than retaining them.
- What export/import versioning guarantees are needed before supporting more record types?
- When would a normalized artifact schema provide enough value to replace the current source-scoped payload table?
- What local encryption level is required before the first private daily-use prototype?
