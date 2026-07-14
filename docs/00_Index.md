---
status: Draft
version: 0.6
owner: LIN MENGLUNG
last_updated: 2026/07/14
depends:
  - docs/00_Constitution.md
referenced_by:
  - README.md
  - AI_CONTRIBUTOR_GUIDE.md
  - IMPLEMENTATION_GUIDE.md
---

# 00 Index

## Purpose

This document is the knowledge navigation layer for Book Zero.

It helps a contributor locate the Primary Definition, Primary Application, and Related Documents for each governed concept. It is not a product introduction and does not replace the Constitution or the primary theory documents.

## Documentation Hierarchy

The hierarchy is defined in `docs/00_Constitution.md`:

- Level 0: Constitution.
- Level 1: Vision and Philosophy.
- Level 2: Principles, Problem, and Identity.
- Level 3: Memory, Reflection, Awareness, Growth, AI, and Privacy.
- Level 4: MVP, Roadmap, and implementation planning.
- Level 5: Code.

Lower levels must not override higher levels. When theory and implementation differ, document the gap rather than treating code as philosophy.

## Concept Index

| Concept | Primary Definition | Primary Application | Related Documents |
| --- | --- | --- | --- |
| Mirror | `docs/00_Constitution.md` | `docs/03_Principles.md` | `Life_OS_Manifesto.md`, `README.md`, `docs/Reflection.md`, `docs/09_AI.md` |
| Vision | `docs/01_Vision.md` | `docs/12_Roadmap.md` | `docs/00_Constitution.md`, `docs/11_MVP.md` |
| Philosophy | `docs/02_Philosophy.md` | `docs/03_Principles.md` | `docs/01_Vision.md`, `docs/09_AI.md` |
| Context | `docs/02_Philosophy.md` | `docs/03_Principles.md` | `docs/06_Memory.md`, `docs/Reflection.md`, `docs/09_AI.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md` |
| Context Before Insight | `docs/02_Philosophy.md` | `docs/03_Principles.md` | `docs/Reflection.md`, `docs/09_AI.md`, `docs/appendix/Harness.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md` |
| Context Recovery | `docs/02_Philosophy.md` | `docs/product/00_MVP_User_Flow.md` | `docs/Reflection.md`, `docs/09_AI.md`, `docs/11_MVP.md`, `docs/12_Roadmap.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md` |
| Conversation Layer | `docs/09_AI.md` | `docs/product/00_MVP_User_Flow.md` | `docs/02_Philosophy.md`, `docs/appendix/Harness.md`, `docs/12_Roadmap.md` |
| Identity | `docs/05_Identity.md` | `docs/06_Memory.md` | `docs/Reflection.md`, `docs/adr/ADR-0003-identity-is-emergent.md` |
| Evidence | `docs/03_Principles.md` | `docs/06_Memory.md` | `docs/05_Identity.md`, `docs/Reflection.md`, `docs/07_Awareness.md` |
| Memory | `docs/06_Memory.md` | `docs/Reflection.md` | `docs/05_Identity.md`, `docs/10_Privacy.md`, `docs/adr/ADR-0003-identity-is-emergent.md` |
| Longitudinal Memory | `docs/06_Memory.md` | `docs/11_MVP.md` | `docs/10_Privacy.md`, `docs/12_Roadmap.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`, `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md` |
| Historical Context Consent | `docs/10_Privacy.md` | `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md` | `docs/06_Memory.md`, `docs/09_AI.md`, `docs/appendix/Harness.md`, `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md` |
| Reflection | `docs/Reflection.md` | `docs/03_Principles.md` | `docs/06_Memory.md`, `docs/07_Awareness.md`, `docs/08_Growth.md` |
| Cross-Experience Reflection | `docs/Reflection.md` | `docs/11_MVP.md` | `docs/06_Memory.md`, `docs/07_Awareness.md`, `docs/12_Roadmap.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md` |
| Pattern | `docs/05_Identity.md` | `docs/07_Awareness.md` | `docs/06_Memory.md`, `docs/Reflection.md` |
| Meaning | `docs/Reflection.md` | `docs/02_Philosophy.md` | `docs/05_Identity.md`, `docs/09_AI.md` |
| Awareness | `docs/07_Awareness.md` | `docs/08_Growth.md` | `docs/06_Memory.md`, `docs/Reflection.md` |
| Growth | `docs/08_Growth.md` | `docs/03_Principles.md` | `docs/07_Awareness.md`, `docs/Reflection.md` |
| Privacy | `docs/10_Privacy.md` | `docs/06_Memory.md` | `docs/00_Constitution.md`, `docs/09_AI.md`, `docs/appendix/Harness.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md` |
| AI Role | `docs/09_AI.md` | `docs/Reflection.md` | `docs/03_Principles.md`, `docs/10_Privacy.md` |
| AI Hypothesis Protocol | `docs/09_AI.md` | `docs/appendix/Harness.md` | `docs/06_Memory.md`, `docs/Reflection.md`, `docs/07_Awareness.md` |
| Harness | `docs/appendix/Harness.md` | `docs/architecture/00_MVP_Architecture.md` | `docs/09_AI.md`, `docs/11_MVP.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`, `docs/adr/ADR-0005-ai-provider-abstraction.md` |
| MVP Scope | `docs/11_MVP.md` | `docs/product/00_MVP_User_Flow.md` | `docs/architecture/00_MVP_Architecture.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/12_Roadmap.md` |
| Roadmap | `docs/12_Roadmap.md` | `docs/11_MVP.md` | `docs/01_Vision.md`, `docs/appendix/Harness.md` |
| Documentation Hierarchy | `docs/00_Constitution.md` | `docs/adr/ADR-0001-documentation-hierarchy.md` | `README.md`, `AI_CONTRIBUTOR_GUIDE.md` |
| Source of Truth | `docs/00_Constitution.md` | `docs/00_Index.md` | `docs/adr/ADR-0002-single-source-of-truth.md` |

## Reading Paths

### Understand Product Soul

1. `Life_OS_Manifesto.md`
2. `README.md`
3. `docs/00_Constitution.md`
4. `docs/01_Vision.md`
5. `docs/02_Philosophy.md`
6. `docs/03_Principles.md`

### Understand Current Product And Next Work

1. `docs/11_MVP.md`
2. `docs/12_Roadmap.md`
3. `docs/product/00_MVP_User_Flow.md`
4. `docs/architecture/00_MVP_Architecture.md`

### Design AI Or Harness Behavior

1. `docs/00_Constitution.md`
2. `docs/02_Philosophy.md`
3. `docs/03_Principles.md`
4. `docs/06_Memory.md`
5. `docs/Reflection.md`
6. `docs/09_AI.md`
7. `docs/10_Privacy.md`
8. `docs/appendix/Harness.md`
9. `docs/adr/ADR-0005-ai-provider-abstraction.md`
10. `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`

### Modify Longitudinal Memory

1. `docs/00_Constitution.md`
2. `docs/02_Philosophy.md`
3. `docs/03_Principles.md`
4. `docs/05_Identity.md`
5. `docs/06_Memory.md`
6. `docs/Reflection.md`
7. `docs/10_Privacy.md`
8. `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`
9. `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`
10. `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`
11. `docs/adr/ADR-0003-identity-is-emergent.md`
12. `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`
13. `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`

## ADR Index

| ADR | Status | Summary |
| --- | --- | --- |
| `ADR-0001-documentation-hierarchy.md` | Accepted | Documentation hierarchy governs documents, decisions, and code. |
| `ADR-0002-single-source-of-truth.md` | Accepted | Each important concept has one primary definition. |
| `ADR-0003-identity-is-emergent.md` | Accepted | Identity emerges from evidence, patterns, reflection, and time. |
| `ADR-0004-local-first-mvp.md` | Accepted | MVP stores user data locally by default. |
| `ADR-0005-ai-provider-abstraction.md` | Accepted | Provider adapters implement one provider-independent behavior contract. |
| `ADR-0006-mvp-tech-stack.md` | Accepted | MVP uses Tauri + React + SQLite. |
| `ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md` | Accepted | Reviewed AI artifacts persist locally with provenance and user-controlled lifecycle. |
| `ADR-0008-engineering-harness-governance-is-tool-independent.md` | Accepted | Repository engineering governance is canonical and tool-independent; adapters remain thin. |
| `ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md` | Accepted | Historical provider use requires per-generation consent, an exact bounded packet, actual-use provenance, and the Phase 3B/Phase 4 boundary. |

## Canonical Definitions

| Concept | Canonical Definition |
| --- | --- |
| Context Before Insight | Without sufficient context, there is no insight worth trusting. |
| Identity | Identity is a living process through which a person continuously becomes themselves over time. |
| Reflection | Reflection is the process where evidence and identity meet again, reorganizing understanding. |
| Evidence | Evidence is recorded experience that can support self-understanding, pattern recognition, and reflection. |
| Memory | Memory preserves evidence and reviewed continuity, not immutable judgment. |
| Awareness | Awareness is the ability to recognize patterns that were previously invisible. |
| Growth | Growth is the continuous process of integrating new understanding into future actions. |
| Meaning | Meaning is co-created through reflection but remains user-owned. |
| Harness | The governed system that constrains AI behavior, assembles context, preserves reviewed evidence, and improves through human-reviewed learning. |

## AI Contributor Rule

Do not treat this Index as product theory. Use it to find the source.

Modify the Primary Definition first. Apply the change in lower-level documents without creating a competing definition. If implementation differs from the governed definition, record the gap explicitly.
