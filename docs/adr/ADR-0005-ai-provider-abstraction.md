---
status: Accepted
version: 0.2
owner: LIN MENGLUNG
last_updated: 2026/07/11
depends:
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/09_AI.md
  - docs/appendix/Harness.md
  - IMPLEMENTATION_GUIDE.md
referenced_by:
  - docs/00_Index.md
  - docs/appendix/Harness.md
  - docs/architecture/00_MVP_Architecture.md
  - docs/product/00_MVP_User_Flow.md
---

# ADR-0005: Adopt Provider-Independent AI Behavior

## Status

Accepted

## Context

Life OS should not depend on one AI vendor.

Users may use OpenAI, Google AI, local models, or future providers. Provider capabilities, defaults, and safety behavior will differ and change.

A simple interface prevents vendor lock-in, but interface abstraction alone is insufficient. If each adapter independently defines prompts, uncertainty, context use, and output semantics, product behavior will drift between models.

Book Zero defines AI as a mirror, Context Steward, conversational elicitor, pattern observer, and reflection partner. These behaviors must remain governed by Life OS rather than provider defaults.

## Decision

Life OS will maintain a simple AI Provider abstraction and one provider-independent Harness behavior contract.

Provider adapters may handle transport, authentication, provider-specific request format, and response parsing.

They may not redefine:

- Context Before Insight;
- Evidence before Conclusion;
- Reflection before Answer;
- uncertainty and provenance requirements;
- diagnosis and identity-finalization prohibitions;
- the user's ownership of final interpretation;
- the conceptual output contract.

Harness prompts, schemas, context-selection rules, and safety behavior must be versioned, evaluated, reversible, and changed through human review.

## Consequences

- BYOK and provider switching remain possible.
- Local model support remains possible.
- The same evaluation cases must run across supported providers.
- A provider-specific adapter may not become the source of product philosophy.
- Prompt duplication across adapters should be replaced by shared versioned behavior and schema definitions.
- Provider/model/prompt version must be recorded for persisted generated artifacts.
- A provider that cannot satisfy the Harness contract must fall back, reduce capability, or remain unsupported.
- The abstraction should remain minimal; provider independence does not justify an abstraction-first engine rewrite.

## Rejected Alternatives

### Let each provider use its recommended default behavior

Rejected because Life OS ethics and interaction boundaries would drift with vendors.

### Encode all product behavior separately in every adapter

Rejected because duplicated prompts cannot provide a reliable, versioned behavioral source of truth.
