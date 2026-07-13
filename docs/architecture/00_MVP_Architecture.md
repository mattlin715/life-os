---
status: Draft
version: 0.4
owner: LIN MENGLUNG
last_updated: 2026/07/14
depends:
  - docs/00_Constitution.md
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/07_Awareness.md
  - docs/08_Growth.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/adr/ADR-0004-local-first-mvp.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/02_Philosophy.md
  - docs/11_MVP.md
  - docs/appendix/Harness.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
referenced_by:
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/architecture/08_Local_Historical_Context_Selection_Foundation.md
---

# 00 MVP Architecture

## Purpose

This document defines the minimum architecture required to validate the Life OS core loop.

It does not define the final Life OS architecture.

It translates Book Zero into an MVP architecture that can be built, tested, revised, or discarded without creating unnecessary complexity.

The goal is not to build the whole system.

The goal is to validate whether the core loop creates value.

## 1. Architecture Principles

### Local-first

The MVP stores user data locally by default.

Cloud sync, accounts, hosted storage, and multi-device sync are out of scope.

### User-owned data

The user must be able to inspect, edit, delete, and export their data.

The product earns the right to remember.

### Evidence before conclusion

The system must preserve the distinction between evidence, pattern, hypothesis, and user meaning.

AI output must not become final truth.

### Editable AI output

All AI-generated evidence candidates, reflection prompts, and pattern notes must remain editable, rejectable, or removable by the user.

### Provider abstraction

AI behavior must be governed by Book Zero, not by any provider default.

The MVP should use a simple provider interface.

### No oracle behavior

The system must not answer life for the user.

It should reflect evidence and ask questions.

### Minimal architecture

Build only what is necessary to validate the MVP loop.

### No premature engines

Do not build full Evidence Engine, Reflection Engine, Awareness Engine, Growth Engine, or Digital Self Model yet.

## 2. Core Loop Architecture

The MVP architecture exists to support:

Experience  
→ Evidence  
→ Reflection  
→ Awareness  
→ Growth

This means:

- The user writes an experience.
- The system extracts evidence candidates.
- The user reviews and edits evidence.
- The system generates reflection prompts.
- The user answers or skips prompts.
- The system proposes possible pattern notes.
- The user confirms, edits, or rejects the pattern note.
- The entry is saved to a local memory timeline.

## 3. Suggested Modules

- Experience Input
- Evidence Extraction
- Reflection Prompt Generator
- Pattern Notes
- Local Memory Store
- AI Provider Interface
- Export / Import
- User Review Layer

## 4. What Each Module Does

### Experience Input

Captures a user-written daily or event-based experience.

It should support plain text first.

### Evidence Extraction

Transforms the user's experience into evidence candidates.

Evidence candidates may include events, emotions, decisions, tradeoffs, values in action, contradictions, and self-descriptions.

They are candidates, not facts.

### Reflection Prompt Generator

Generates reflective questions based on reviewed evidence.

It should not generate final answers.

### Pattern Notes

Stores user-confirmed or user-edited observations about repeated patterns.

Pattern notes must remain hypotheses.

### Local Memory Store

Stores entries, evidence, reflections, and pattern notes locally.

It should support inspect, edit, delete, and export.

### AI Provider Interface

Provides a simple boundary between Life OS behavior and AI provider calls.

The interface should support one provider first without hard-coding product logic to that provider.

### Export / Import

Allows the user to preserve ownership and portability.

Markdown and JSON export should be considered early.

### User Review Layer

Ensures AI output does not silently become product truth.

The user must review, edit, reject, or confirm important AI-generated artifacts.

## 5. What Not To Build Yet

Do not build yet:

- Full Digital Self Model
- Knowledge graph
- Cloud sync
- Agent automation
- Mobile app
- Personality prediction
- MBTI inference
- Complex dashboard

These may become useful later.

They are not required to validate the core loop.

## 6. Data Ownership

The MVP must make user ownership concrete.

The user should be able to:

- Inspect stored entries.
- Inspect extracted evidence.
- Edit AI-generated output.
- Delete entries, evidence, reflections, and pattern notes.
- Export data in portable formats.

Local-first reduces risk, but it does not remove the need for clear control.

## 7. AI Boundary

AI can propose:

- Evidence candidates.
- Reflection questions.
- Possible patterns.
- Alternative interpretations.

AI cannot decide:

- Final meaning.
- Identity labels.
- Diagnosis.
- Growth goals.
- Life advice.

The user confirms meaning.

AI supports the mirror.

It does not become the oracle.

## 8. Concept Taxonomy Deferred

Primary Definition / Primary Application is already handled in `docs/00_Index.md`.

Full Concept Taxonomy / Ontology is deferred until after MVP architecture stabilizes.

Personal Meaning vs Meaning Making should be handled in future ontology work.

The MVP should not invent a full ontology before the core loop is validated.

## 9. Architecture Open Questions

- What is the minimal persisted artifact and provenance model?
- How should SQLite schema migrations and rollback be versioned?
- How should Context Recovery turns be retained, revised, and deleted?
- What retrieval strategy is sufficient before a memory graph is justified?
- How should export/import be versioned?
- How should provider credentials be stored locally?
- Should AI calls be synchronous or queued?
- How should failed AI output be represented?
- What level of encryption is required for the first local prototype?

## 10. Harness-Aligned Architecture Direction

Dogfooding changes the next architecture boundary without changing the local-first foundation.

The target flow is:

Experience
→ Context Sufficiency Check
→ Context Recovery Conversation when needed
→ Evidence Candidates
→ User Review
→ Reflection
→ Pattern Hypothesis
→ Persisted Review Artifacts
→ Cross-Experience Reflection when enough evidence exists

### Required Boundaries

- **Context Sufficiency Gate** decides the maximum responsible inference depth.
- **Conversation Boundary** stores bounded clarification turns and user stopping decisions.
- **Artifact Repository** persists evidence, reflection, and pattern records with lifecycle and provenance.
- **Historical Retrieval Boundary** selects relevant records with explainable criteria and user control.
- **Harness Boundary** supplies one versioned behavior protocol and output contract to every provider adapter.
- **Evaluation Boundary** runs governed cases without training on private user content by default.

### Provenance Requirement

Every derived artifact must retain source identifiers, authorship, review state, and generation metadata. Persistence must not flatten user text, AI inference, and user confirmation into one undifferentiated record.

### Migration Requirement

The current database initializes a single table with `CREATE TABLE IF NOT EXISTS`. Persisting reviewed artifacts requires explicit schema versioning and migrations before private alpha.

### Current-Code Qualification

As of 2026/07/12, the code implements the single-Experience vertical slice: source-scoped Context Recovery turns and reviewed Evidence, Reflection, and Pattern artifacts persist locally with provenance in SQLite schema version 3. Provider calls use only the current Experience packet and its eligible source-scoped dependencies. Phase 3A adds an optional, ephemeral local lexical candidate selector with visible reasons and manual include/exclude; it is not in any Context Packet or provider request. Cross-Experience reflection and longitudinal-memory consent are not implemented. This section defines the next architecture direction; it does not claim those deferred capabilities are implemented.
