# Product Review

Status: approved

- Sprint ID: 2026-08-11-historical-preflight-cancel-focus-restoration
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: eb14081b89631eadc623672b42417eb38f5e788da3c2eefe134b08776ad976b5
- Created at: 2026-08-11T09:53:04.999Z
- Updated at: 2026-08-11T09:53:04.999Z

## Mission Interpretation

Correct spatial disorientation after an explicit non-transmission choice without changing historical-context state or governance.

## Problem Statement

The preflight is tall. Removing it while preserving the browser scroll offset makes later Experiences appear in its place, so the user loses the originating panel.

## User Value

Cancellation remains calm, reversible, and locationally predictable.

## Relevant Primary Definitions

`docs/03_Principles.md`, `docs/06_Memory.md`, and `docs/Reflection.md` require user agency, visible boundaries, and low-threat reflection flow.

## Relevant ADRs

ADR-0009 keeps selection, consent, transmission, and provenance distinct; this correction changes none of them.

## Current Implementation Context

`cancelHistoricalPreflight` only clears React preflight state. Existing navigation helpers already focus rendered targets after a frame with reduced-motion support.

## In Scope

Cancel and adjust-source focus restoration to the exact same Experience panel, plus deterministic tests and manual Step 10C-R.

## Out Of Scope

All retrieval, ranking, caps, storage, selection, date-range, consent, packet, provider, schema, Phase 4, and whole-history changes.

## Product Constraints

The panel remains open and its candidate, selection, and filter state remain unchanged.

## Evidence And Provenance Constraints

No durable evidence or provenance record is created or changed.

## Historical Context Constraints

No retrieval rerouting beyond existing React rendering; no candidate or packet contract change.

## Consent Constraints

Cancellation remains no consent and no transmission.

## AI-Role Constraints

No AI behavior.

## Privacy Constraints

Local UI navigation only.

## User-Agency Constraints

The user's cancel/adjust choice must be preserved exactly and the interface must return them to the place where they can revise it.

## Acceptance Criteria

1. Cancel and adjust close only the preflight.
2. After React renders, focus and viewport return to `historical-context-<entryId>` with block-start alignment.
3. Reduced-motion preference disables smooth scrolling.
4. Missing targets fail safely.
5. Three-language buttons retain equivalent callback behavior.
6. Canonical verification passes and Founder Step 10C-R remains manual.

## Risks

Low: an early focus attempt could race React rendering; frame scheduling and focused tests address it.

## Open Questions

none

## Human Decision Required

false; the Founder explicitly authorized this exact correction.

## Recommendation

Approve the bounded correction.

## Review Status

approved
