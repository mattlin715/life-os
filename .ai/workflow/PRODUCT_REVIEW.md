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

## R2 Revision Review

The earlier sections describe R1. The Founder reported that the 3100 path
made a demo clue mandatory, let an edited clue replace the original, and
offered no meaningful next step after answering. The documented R2 plan
supersedes that interaction for the separate connected Pilot only.
Explicit Founder start: **同意開始**.

The current saved original becomes the grounding source; focus may only
be a verbatim excerpt. A verified local save keeps model and user
authorship distinct and does not send anything. A separate affirmative
consent can request one neutral reflection grounded in that exact original
and saved pair, with uncertainty and no diagnosis, prescription, identity
claim, longitudinal inference, or persistence of feedback. User can stop
after saving. Same-moment only, fictional content only; no broad packet,
secret persistence, connected fallback, desktop or offline change.
Package/device/live provider remain separate Founder decisions.

## 3101 Founder feedback and R3 boundary (2026-11-18 JST)

The Founder reports successful Zenfone 10 update with old moments retained,
direct model question, verified local answer, and a useful second angle.
The Founder also explicitly reports the three cancellation, mobile
lifecycle/draft/stale-reply, and disposable source-deletion checks OK.
These are bounded direct reports, not independent telemetry, crash proof,
a completed canonical check, or invited-user acceptance.

Explicit source-only UI requests: remove optional focus; remove connected
local-demo and repeated demo-question controls; distinguish Gemini question
and feedback. Preserve offline mode and previously saved records. Do not
replace removal with an identical secondary generation path.

Integrate remembered credentials, explicitly reviewed feedback persistence
and same-moment user supplements into one R3 proposal for Founder approval.
Do not implement broader credential, retention or provider-input authority
under the earlier session-only R2 permission. Time-range cross-moment
Pattern reflection is recorded for later discussion under ADR-0010,
not implemented, activated, scheduled, or granted historical consent.
