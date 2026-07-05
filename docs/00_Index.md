---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/06
depends:
  - docs/00_Constitution.md
referenced_by: []
---

# 00 Index

## Purpose

This document is the knowledge navigation layer for Book Zero.

Book Zero is no longer a set of isolated Markdown files. It is an interconnected knowledge system with hierarchy, source of truth rules, theory documents, and ADRs.

The purpose of this Index is to help any AI Agent quickly build a mental model of Book Zero and locate the primary definition, primary application, and related documents for each concept.

This is not a README.

This is not a product introduction.

This is not a replacement for the Constitution.

It is a navigation document.

## Documentation Hierarchy

Book Zero follows the Documentation Hierarchy defined in `docs/00_Constitution.md`.

Summary:

- Level 0: Constitution
- Level 1: Vision, Philosophy
- Level 2: Principles, Problem, Identity
- Level 3: Memory, Reflection, Awareness, Growth, AI, Privacy
- Level 4: MVP, Roadmap, Implementation planning
- Level 5: Code

Rule of use:

- Read higher-level documents before lower-level documents.
- Lower-level documents must not override higher-level documents.
- Code is implementation, not the source of product philosophy.
- When in conflict, follow the higher-level document.

## Concept Index

| Concept | Primary Definition | Primary Application | Related Documents |
| --- | --- | --- | --- |
| Mirror | `docs/00_Constitution.md` | `docs/03_Principles.md` | `README.md`, `docs/04_Problem.md`, `docs/Reflection.md`, `docs/09_AI.md` |
| Identity | `docs/05_Identity.md` | `docs/06_Memory.md` | `docs/Reflection.md`, `docs/adr/ADR-0003-identity-is-emergent.md` |
| Evidence | `docs/03_Principles.md` | `docs/06_Memory.md` | `docs/05_Identity.md`, `docs/Reflection.md`, `docs/07_Awareness.md` |
| Reflection | `docs/Reflection.md` | `docs/03_Principles.md` | `docs/05_Identity.md`, `docs/06_Memory.md`, `docs/07_Awareness.md`, `docs/08_Growth.md` |
| Pattern | `docs/05_Identity.md` | `docs/07_Awareness.md` | `docs/04_Problem.md`, `docs/06_Memory.md`, `docs/Reflection.md` |
| Meaning | `docs/Reflection.md` | `docs/05_Identity.md` | `docs/06_Memory.md`, `docs/09_AI.md` |
| Growth | `docs/08_Growth.md` | `docs/03_Principles.md` | `docs/06_Memory.md`, `docs/Reflection.md`, `docs/07_Awareness.md` |
| Awareness | `docs/07_Awareness.md` | `docs/08_Growth.md` | `docs/04_Problem.md`, `docs/06_Memory.md`, `docs/Reflection.md` |
| Privacy | `docs/10_Privacy.md` | `docs/06_Memory.md` | `docs/00_Constitution.md`, `docs/03_Principles.md`, `docs/09_AI.md` |
| Interaction | `docs/03_Principles.md` | `docs/09_AI.md` | `docs/00_Constitution.md`, `docs/Reflection.md` |
| Memory | `docs/06_Memory.md` | `docs/Reflection.md` | `docs/05_Identity.md`, `docs/07_Awareness.md`, `docs/adr/ADR-0003-identity-is-emergent.md` |
| Problem | `docs/04_Problem.md` | `docs/05_Identity.md` | `README.md`, `docs/06_Memory.md`, `docs/07_Awareness.md` |
| Documentation Hierarchy | `docs/00_Constitution.md` | `docs/adr/ADR-0001-documentation-hierarchy.md` | `README.md`, `docs/00_Index.md` |
| Source of Truth | `docs/00_Constitution.md` | `docs/adr/ADR-0002-single-source-of-truth.md` | `docs/00_Index.md` |

## Theory Index

| Theory | Primary Definition |
| --- | --- |
| We Build Mirrors, Not Oracles. | `docs/00_Constitution.md` |
| Human before AI. | `docs/00_Constitution.md` |
| Reflection before Answer. | `docs/00_Constitution.md` |
| Evidence before Conclusion. | `docs/00_Constitution.md` |
| Growth before Engagement. | `docs/00_Constitution.md` |
| Privacy before Profit. | `docs/00_Constitution.md` |
| Every interaction shapes the user's relationship with themselves. | `docs/03_Principles.md` |
| Every interaction teaches something. The question is: What are we teaching? | `docs/03_Principles.md` |
| Mirror before Advice. | `docs/03_Principles.md` |
| Identity is Dynamic. | `docs/05_Identity.md` |
| Identity is not a destination. Identity is an ongoing process of becoming. | `docs/05_Identity.md` |
| Identity is revealed through patterns, not declarations. | `docs/05_Identity.md` |
| Identity contains contradictions. | `docs/05_Identity.md` |
| Personality is an observation, not an identity. | `docs/05_Identity.md` |
| Identity is Emergent. | `docs/adr/ADR-0003-identity-is-emergent.md` |
| Life OS understands people through evidence, not declarations. | `docs/05_Identity.md` |
| Life OS does not collect memories. Life OS collects evidence. | `docs/06_Memory.md` |
| A good memory system does not preserve the past. It preserves continuity of identity. | `docs/06_Memory.md` |
| Evidence can be collected. Meaning must be discovered. | `docs/Reflection.md` |
| Evidence is accumulated. Meaning is discovered. | `docs/06_Memory.md` |
| Reflection is not output. Reflection is a reorganization of understanding. | `docs/Reflection.md` |
| Awareness is the ability to recognize previously invisible patterns. | `docs/07_Awareness.md` |
| Growth is integrating new understanding into future actions. | `docs/08_Growth.md` |
| AI should become less necessary as users become more self-aware. | `docs/09_AI.md` |
| Privacy is psychological safety required for honest self-reflection. | `docs/10_Privacy.md` |

## Reading Paths

### Understand Product

1. `README.md`
2. `docs/00_Constitution.md`
3. `docs/04_Problem.md`
4. `docs/05_Identity.md`
5. `docs/06_Memory.md`
6. `docs/Reflection.md`
7. `docs/07_Awareness.md`
8. `docs/08_Growth.md`

### Design AI Behavior

1. `docs/00_Constitution.md`
2. `docs/03_Principles.md`
3. `docs/05_Identity.md`
4. `docs/06_Memory.md`
5. `docs/Reflection.md`
6. `docs/07_Awareness.md`
7. `docs/09_AI.md`
8. `docs/10_Privacy.md`

### Modify Memory

1. `docs/00_Constitution.md`
2. `docs/03_Principles.md`
3. `docs/05_Identity.md`
4. `docs/06_Memory.md`
5. `docs/Reflection.md`
6. `docs/10_Privacy.md`
7. `docs/adr/ADR-0003-identity-is-emergent.md`

### Modify Identity

1. `docs/00_Constitution.md`
2. `docs/04_Problem.md`
3. `docs/03_Principles.md`
4. `docs/05_Identity.md`
5. `docs/06_Memory.md`
6. `docs/Reflection.md`
7. `docs/adr/ADR-0003-identity-is-emergent.md`

### Build MVP

1. `README.md`
2. `docs/00_Constitution.md`
3. `docs/01_Vision.md`
4. `docs/02_Philosophy.md`
5. `docs/03_Principles.md`
6. `docs/11_MVP.md`
7. `docs/12_Roadmap.md`

### Create Prompt

1. `docs/00_Constitution.md`
2. `docs/03_Principles.md`
3. `docs/05_Identity.md`
4. `docs/06_Memory.md`
5. `docs/Reflection.md`
6. `docs/09_AI.md`
7. `docs/10_Privacy.md`
8. `docs/appendix/Prompt Design.md`

### Architecture Design

1. `docs/00_Constitution.md`
2. `docs/03_Principles.md`
3. `docs/05_Identity.md`
4. `docs/06_Memory.md`
5. `docs/Reflection.md`
6. `docs/07_Awareness.md`
7. `docs/08_Growth.md`
8. `docs/09_AI.md`
9. `docs/10_Privacy.md`
10. `docs/adr/`

## ADR Index

| ADR | Status | Summary |
| --- | --- | --- |
| `ADR-0001-documentation-hierarchy.md` | Accepted | Adopt Documentation Hierarchy as the governance model for documents, decisions, and code. |
| `ADR-0002-single-source-of-truth.md` | Accepted | Each important product concept has one primary definition location; other documents may cite, apply, or extend it. |
| `ADR-0003-identity-is-emergent.md` | Accepted | Identity is not stored data; it emerges from evidence, patterns, reflection, and time. |

## Future Theory

The following theories are not yet complete in Book Zero. This list is for navigation only and does not define them.

- Digital Self Model
- Evidence Engine
- Reflection Engine
- Awareness Engine
- Growth Engine

## AI Agent Instructions

For first-time repository reading:

1. Do not modify code first.
2. Read `README.md`.
3. Read `docs/00_Constitution.md`.
4. Read `docs/04_Problem.md`.
5. Read `docs/05_Identity.md`.
6. Read `docs/06_Memory.md`.
7. Read `docs/Reflection.md`.
8. Read `docs/07_Awareness.md`.
9. Read `docs/08_Growth.md`.
10. Read `docs/03_Principles.md`.
11. Read the relevant architecture or implementation document only after the conceptual chain is clear.
12. Check ADRs before making long-term technical or conceptual decisions.

For any modification:

- Identify the concept being changed.
- Find its Primary Definition in this Index.
- Modify the Primary Definition first.
- Update primary applications and related documents only as references, applications, or consequences.
- If a decision changes hierarchy, source of truth, identity, memory, reflection, privacy, or AI behavior, consider whether an ADR is required.

Do not treat this Index as the source of product theory.

Use it to find the source.

## Canonical Definitions

| Concept | Canonical Definition |
| --- | --- |
| Identity | Identity is a living process through which a person continuously becomes themselves over time. |
| Reflection | Reflection is the process where evidence and identity meet again, reorganizing understanding. |
| Evidence | Evidence is recorded experience that can support self-understanding, pattern recognition, and reflection. |
| Awareness | Awareness is the ability to recognize patterns that were previously invisible. |
| Growth | Growth is the continuous process of integrating new understanding into future actions. |
| Meaning | Meaning is the user's discovered interpretation of evidence, identity, and lived experience. |
| Mirror | A mirror helps the user see themselves more clearly without claiming authority over them. |
| Pattern | A pattern is a repeated relationship across evidence, emotion, decision, behavior, or self-description. |
| Agency | Agency is the user's capacity to interpret, choose, and act from their own understanding. |
| Trust | Trust is the psychological safety that allows honest self-reflection and responsible use of memory. |
