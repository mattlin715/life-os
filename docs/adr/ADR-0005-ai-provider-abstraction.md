---
status: Accepted
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/06
depends:
  - docs/03_Principles.md
  - docs/09_AI.md
  - IMPLEMENTATION_GUIDE.md
referenced_by:
  - docs/architecture/00_MVP_Architecture.md
  - docs/product/00_MVP_User_Flow.md
---

# ADR-0005: Adopt AI Provider Abstraction

## Status

Accepted

## Context

Life OS should not depend on one AI vendor.

Users may use OpenAI, Anthropic, Gemini, OpenRouter, local models, or future providers.

The product must preserve user agency, cost control, and long-term flexibility.

Book Zero defines AI as a mirror, pattern observer, and reflection partner. AI behavior must remain governed by Book Zero, not by provider defaults.

## Decision

MVP will define a simple AI Provider abstraction.

The first implementation may support only one provider, but code must not hard-code product logic into a single vendor.

## Consequences

- BYOK becomes possible.
- Local model support remains possible.
- Provider switching is easier.
- AI behavior remains governed by Book Zero, not provider defaults.
- Avoid over-engineering provider abstraction in MVP.
- Provider abstraction should be simple and testable.
