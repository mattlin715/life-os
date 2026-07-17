---
status: Accepted
version: 0.2
owner: product-and-engineering
last_updated: 2026/07/12
depends:
  - docs/00_Constitution.md
  - docs/adr/ADR-0001-documentation-hierarchy.md
  - docs/adr/ADR-0002-single-source-of-truth.md
referenced_by:
  - docs/00_Index.md
  - docs/dev/08_Engineering_Harness.md
  - docs/architecture/14_AI_Orchestration_Evolution.md
---

# ADR-0008: Engineering Harness Governance Is Tool-Independent

## Status

Accepted by the human founder on 2026/07/12.

## Context

Life OS may be modified through Codex, Claude Code, or future coding tools. Their platform features differ, but product governance, authority order, verification expectations, and escalation boundaries must remain stable. Duplicating full policy in tool-specific instruction files would create drift and could let a platform implementation quietly become the source of truth.

## Decision

Treat this as a durable **repository architecture decision**, not a product or constitutional decision. Canonical engineering knowledge remains in vendor-neutral repository assets: `AI_CONTRIBUTOR_GUIDE.md`, `AGENTS.md`, `docs/dev/`, shared scripts, and CI. Tool-specific files are thin adapters that route to those sources and cannot override the authority hierarchy.

## Consequences

- A new coding tool receives a small adapter rather than a copied governance manual.
- Verification and operational procedures are shared and executable where practical.
- Product Harness governance remains separate from Engineering Harness governance.
- Platform hooks and permissions may add enforcement, but cannot define product policy.
- Future changes to this boundary require founder review; this acceptance does not authorize constitutional or sensitive-policy changes.

## Alternatives Considered

### Tool-specific governance files as independent sources

Rejected because duplicated rules drift and create competing authority.

### No repository-level engineering guidance

Rejected because every session would reconstruct operational boundaries from prompts and platform defaults.

### Constitutional amendment

Rejected because this changes repository execution architecture, not the moral relationship between Life OS AI and the user.
