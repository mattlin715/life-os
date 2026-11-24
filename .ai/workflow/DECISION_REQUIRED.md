# Decision Required

Status: open
- Sprint ID: android-pilot-byo-gemini-r1
- Artifact schema: 1.0
- Authoring role: orchestrator
- Updated at: 2026-11-18 JST

## Decision ID

ANDROID-M3-GEMINI-R3-TYPED-MODEL-SCHEMA

## Decision Summary

R3 user-flow items 1,5,6 were approved for sources/host tests. Item 1 is
implemented and host-tested; items 5 and 6 require the now-concrete schema
decision described in proposed ADR-0013. Do not mistake this for another
request to approve the same flow, or a completed R3 implementation.

## Concrete Blocker

Current v5 kinds, provenance/dependency roles and exact schema/version checks
cannot express independent feedback and supplement chains. Patterns require
Evidence; Reflection cannot be repurposed as arbitrary dialog. Neither a
fake artifact nor JSON sidecar is an acceptable shortcut.

## Proposed Decision

Approve or revise ADR-0013's isolated Model Pilot v6 design: retain same app
identity/store and old question/answer data; add normalized typed model-turn
revisions, content, provenance, dependencies, lifecycle and separate upgrade
receipt. Correcting or deleting sources/answers/turns invalidates dependents;
parent deletion purges content. Preserve v5 history, require exact pre/post
upgrade verification, and refuse unsafe automatic downgrade. Then complete
the exact DDL/upgrade engineering artifact before enabling migration code.
Offline/desktop remain v5.

## Default Safe Action

Leave installed 3101 and all user content unchanged; keep feedback ephemeral.
Do not build a credential-only APK, add storage bypasses, accept an ADR
automatically, execute a migration or ask for secrets.

## Available Options

A. Accept the proposed Model Pilot v6 design and proceed with its exact
schema/upgrade plan under the already approved source/host scope.

B. Revise the storage/upgrade boundary before implementing feedback save
and supplements. Existing credential/UI source work remains preserved.

## Out of Scope

APK/sign/install, actual API, real journals, cross-moment Pattern feature,
migration execution on a profile, destructive recovery, signing-secret
access, commit/push. Canonical local pnpm failure is not waived.

## Exact Founder Response Needed

Accept or revise the concrete typed model-turn/schema-v6 design in
`docs/adr/ADR-0013-proposed-model-pilot-feedback-and-supplement-lifecycle.md`.

## Resolution Status

open

## Exact Founder Response

Not recorded.

## Selected Option And Authorized Scope

Not recorded.

## Decided At And Evidence Reference

Not recorded.

## Resume Phase

engineering_planning
