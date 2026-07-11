---
status: Accepted
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/11
depends:
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/adr/ADR-0003-identity-is-emergent.md
  - docs/adr/ADR-0004-local-first-mvp.md
referenced_by:
  - docs/00_Index.md
  - docs/11_MVP.md
  - docs/appendix/Harness.md
  - docs/architecture/00_MVP_Architecture.md
---

# ADR-0007: Persist Reviewed AI Artifacts With Provenance

## Status

Accepted

## Context

The current MVP persists user-authored experiences but keeps Evidence Candidates, Reflection Prompts and responses, and Pattern Candidates only in frontend session state.

Dogfooding showed that this breaks continuity after restart. Users cannot revisit prior analysis, compare changing interpretations, or build cross-experience reflection.

Persisting AI output without stronger boundaries would create a different risk: inference could become indistinguishable from user-confirmed evidence, lose its source, or silently accumulate into an identity judgment.

The repository therefore needs a durable decision before extending the SQLite model.

## Decision

Life OS will persist reviewed AI artifacts and user-authored reflection artifacts locally with explicit provenance.

Persisted artifacts must preserve:

- artifact type;
- source Experience and source artifact identifiers;
- whether content is user-authored or AI-generated;
- candidate, confirmed, rejected, skipped, or revised state as applicable;
- provider, model, prompt/schema version for generated content;
- creation and revision history;
- applicable consent and retention state.

Confirmation does not convert an AI hypothesis into objective fact. It records the user's review decision.

Deletion and correction must propagate through understandable relationships. Export must distinguish raw content, reviewed evidence, reflection, and hypothesis.

## Consequences

- The SQLite boundary must expand beyond `ExperienceEntry` through versioned migrations.
- Evidence, reflection, conversation, and pattern records need explicit lifecycle and provenance fields.
- The UI must allow review of prior artifacts after restart.
- Historical retrieval may use only eligible records under user-controlled memory settings.
- Rejected artifacts may be retained only under an explicit, documented purpose and consent/retention rule; they cannot silently influence user-facing hypotheses.
- Deleting a source must make dependent artifacts removable or visibly invalid rather than orphaned truth.
- Export/import versioning must evolve before reviewed artifacts become portable.
- Evaluation feedback must remain distinct from personal meaning and must not treat inactivity as rejection.

## Rejected Alternatives

### Keep AI artifacts session-only

Rejected because it prevents continuity and makes longitudinal reflection impossible.

### Persist only confirmed text without provenance

Rejected because it erases the distinction between user evidence and AI inference.

### Persist every model output automatically

Rejected because it increases privacy risk, noise, and silent profiling.
