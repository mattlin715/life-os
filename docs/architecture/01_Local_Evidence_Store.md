---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/07
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

Only `ExperienceEntry` is wired into the first UI boundary.

The other records exist as minimal types so future work can connect the MVP loop without inventing new concepts.

## User Ownership Requirements

User-owned records must support:

- Inspect: the user can see what is stored.
- Edit: the user can correct their own record.
- Delete: the user can remove records.
- Export: the system must leave room for portable export.

The first sprint only implements inspect and delete for session entries.

Edit and export remain requirements for the local store boundary, not completed features.

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

## SQLite Boundary

SQLite is the accepted local persistence direction for the MVP.

The first implementation does not create migrations or a complete schema.

The code should expose a storage interface first.

SQLite should later become one implementation of that interface through the Tauri SQL plugin.

## Open Questions

- What is the smallest SQLite schema that preserves inspect, edit, delete, and export?
- Should rejected AI output be stored, discarded, or stored only with explicit user consent?
- How should deleted entries affect related evidence, reflection prompts, and pattern notes?
- Should export be Markdown-first, JSON-first, or both?
- How should schema versioning be introduced without over-engineering the MVP?
- What local encryption level is required before the first private daily-use prototype?
