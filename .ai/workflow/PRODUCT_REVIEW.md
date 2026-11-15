# Product Review

Status: approved_with_conditions

- Sprint ID: android-pilot-byo-gemini-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 869a25e217cb1297bd0569964b2b69b56fe9650d
- Working-tree digest reviewed: ac4029e060d2349c1433b1eddd81abf3e1b21b3d6747bd5ebc29f48f5592aff4
- Created at: 2026-11-15 JST
- Updated at: 2026-11-15 JST

## Mission Interpretation

Founder-only phone trial; one open question after explicitly adopting and previewing a synthetic clue.

## Problem Statement

Offline demo cannot establish the experience of receiving a model question; whole ContextPacket transmission is out of scope.

## User Value

Try the interaction in the existing phone Reflection flow, without a proxy or uploading a whole journal.

## Relevant Primary Definitions

Constitution, Reflection, Memory, AI, Privacy and Product Harness: AI offers a neutral question; the user owns interpretation and consent.

## Relevant ADRs

ADR-0005 provider-independent behavior; ADR-0007 provenance; ADR-0009 historical consent not in scope; ADR-0012 is non-authority for live traffic; direct Founder authorization is source/mock only.

## Current Implementation Context

Existing isolated synthetic Pilot is offline. Shared desktop provider transports whole packets with original, identity and prior answers; not suitable.

## In Scope

Settings, retained locale preference, memory-only key, fixed standalone Gemini transport, exact preview/confirm, unsaved trial question and response, mock host tests.

## Out Of Scope

Live call, build package, sign/install, Git promotion, other providers, custom model/endpoint, real or historical data, automation, persistence.

## Product Constraints

Keep default/offline paths intact; explicitly enable a separate connected build mode; synthetic-only.

## Evidence And Provenance Constraints

Only an adopted current clue can be selected; label returned question source and do not persist it.

## Historical Context Constraints

No history or IDs in provider payload; no retrieval.

## Consent Constraints

Destination and exact text visible before a single explicit confirmed send; no background sends.

## AI-Role Constraints

Ask one open question grounded in the clue without judgment, diagnosis, advice or inferred motives; fail visibly rather than substitute.

## Privacy Constraints

No bundled key or persistence/logs. Fixed Google HTTPS only, no redirects/retries. No sensitive, confidential, or personal material.

## User-Agency Constraints

May stay offline, cancel, clear key, answer or skip; Settings does not discard drafts.

## Acceptance Criteria

Mock strict scope, key lifetime, duplicates, late results, language and draft continuity; then stop before packaging.

## Risks

Secret exposure through telemetry or input suggestions, UI confusion, stale responses, accidental network build. Apply fail-closed feature and scope checks.

## Open Questions

None for source/mock. Android packaging/device/live provider separately gated.

## Human Decision Required

False for source/mock; live enablement and build remain separate Founder checkpoints.

## Recommendation

Implement the bounded opt-in vertical slice without changing canonical policy.

## Review Status

approved_with_conditions
