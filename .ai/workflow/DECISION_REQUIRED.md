# Decision Required

Status: resolved
- Sprint ID: android-pilot-byo-gemini-r1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-11-15 JST
- Updated at: 2026-11-15 JST

## Decision ID

ANDROID-M3-GEMINI-LOCAL-SAVE-R1

## Sprint ID

android-pilot-byo-gemini-r1

## Decision Summary

The Founder replaced the session-only trial requirement with explicit local
save of the Gemini question and user response together. Save must not call Gemini.

## Why Automation Stopped

ADR-0012 reserves Android provider, credential and network authority for M3.
The current offline M2-C provenance allowlist refuses provider-generated rows.
The canonical writer separately creates a suggestion and saves an answer;
two separate commits could leave an unanswered remote question after failure.
The earlier implementation plan still authorizes a session-only trial, not a
new durable Android provider provenance boundary.

## Relevant Constitution Clauses

We Build Mirrors, Not Oracles; Human Before AI; Evidence Before Conclusion;
Privacy Before Profit.

## Relevant Primary Definitions

Reflection, Memory, Privacy, AI, and Product Harness. Explicit local save is
distinct from permission to transmit a clue.

## Relevant ADRs

ADR-0012 M3 authority boundary; ADR-0007 requires local provenance; ADR-0011
requires canonical append-only lifecycle. No ADR status change is proposed.

## Available Options

A. Authorize a bounded M3 connected synthetic Pilot with its own app-private
identity/store while retaining the offline Pilot. Use a single canonical,
verified transaction for explicit save with AI prompt provenance and user
response provenance.

B. Keep the same package/store and explicitly design verified compatibility of
both modes before admitting remote provenance. No implementation until the
offline reader behavior and rollback/update path are approved.

## Benefits

A preserves the current offline trust and data path. B could preserve access to
earlier moments in the same installation after compatibility is designed.

## Risks

A requires a separate installation and newly created synthetic trials. B can
make an older or offline build refuse to open the shared store after an AI save.
Both require atomic write/error reconciliation and exact revision binding.

## Reversibility

A leaves the offline app untouched. B requires an agreed compatibility plan;
never delete or reinterpret existing records as a downgrade recovery method.

## Data And Privacy Impact

Local save only after explicit action. The question and answer bind to the
exact source and adopted clue; prompt uses Gemini/provider/model provenance;
the answer is user-authored. Only the selected synthetic clue would ever be
sent to Gemini; saving sends nothing. No secret, draft, history or real data.

## Orchestrator Recommendation

A. Review an isolated M3 layout and canonical atomic save with mocked provider
only. Keep build, signing, install and actual outbound tests separately gated.

## Default Safe Action

Keep current offline Pilot and unfinished connected source intact; do not
enable a network package or relax the offline provenance allowlist.

## Blocked Files Or Phases

Connected Pilot identity and store scope; durable provider provenance;
paired local save and its canonical verification; no pack/sign/install or live calls.

## Exact Founder Response Needed

Approve A (standalone synthetic M3 identity/store and atomic save) or choose
the shared-store compatibility design B.

## Resolution Status

resolved

## Exact Founder Response

走A

## Selected Option And Authorized Scope

- Selected option: A
- Authorized scope: Isolated synthetic M3 identity and store, canonical paired local save, source and mock tests only; no package, signing, installation or outbound test.

## Decided At And Evidence Reference

- Decided at: 2026-11-14T17:35:01.140Z
- Evidence reference: direct-founder-chat:ANDROID-M3-GEMINI-LOCAL-SAVE-R1

## Resume Phase

engineering_planning
