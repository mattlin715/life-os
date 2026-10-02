# Sprint Report

Status: completed

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-09-29T15:57:00Z
- Updated at: 2026-10-02T18:06:00Z

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, `failed`, or `cancelled`.

## Sprint ID

2026-09-29-android-m2a-direct-fresh-v5

## Mission

Create a bounded synthetic-only Android M2-A review candidate that initializes
directly into exact schema v5 with truthful origin evidence, verifies it on a
disposable emulator, packages the review evidence, and stops before Founder
manual acceptance or any publication/production action.

## Starting Commit

`51dff4998ca8696aeaf1527f058d785303fc0cd4` on clean `develop`, matching
`origin/develop` after live fetch.

## Ending Commit Or Working-Tree State

HEAD remains `51dff4998ca8696aeaf1527f058d785303fc0cd4` on
`codex/android-m2a-direct-fresh-v5`. Exact non-workflow working-tree digest is
`146c28adc2591a71eefb2b82fba97eefbd82559ce71d0c1a350ab3c9a676b919`.
All changes are unstaged and uncommitted.

## Final Status

completed

## Product Decision

The authorized temporary-package M2-A direct fresh-v5 design is implemented
and theory-aligned after one bounded durability correction. The Founder
completed the corrected manual checklist 10/10 and accepted the exact unstaged
candidate under `ANDROID-M2A-FOUNDER-REVIEW-002` Option A. No production,
promotion, or publication decision has been made.

## Engineering Summary

Added a separate direct-v5 initializer and receipt verifier, direct-aware
runtime/writer adapters, a fail-closed Android M2-A app-private publication
facade, a trilingual synthetic Experience UI, exact package safeguards,
build/inspect/native automation, and architecture/runbook documentation.
Fresh creation uses canonical compatibility plus v5 DDL in one transaction,
sets `user_version=5`, records zero migration receipts, and cannot be confused
with historical migration. Ordered parent-directory sync barriers now make
both live-name creation and successful pending-name retirement durable before
ready is claimed.

## Behavior Changed

The temporary Android package `com.lifeos.review.m2a` can create or reopen one
verified direct-origin disposable database and perform only synthetic
Experience create/list/get plus a non-content locale preference. Partial,
pending, malformed, newer, sidecar, receipt-invalid, and open-conflict states
are preserved and refused. Desktop and accepted M1 behavior remain separate.
The exact final disposable AVD profile also reopens ready after shutdown/restart
with both pending names absent.

## Files Changed

Added direct-init schema/Rust modules, the M2-A backend/capability/frontend,
M2-A build/inspect/native/contract scripts, architecture 22, and runbook 13.
Updated bounded schema/runtime adapters, Android generated/config surfaces,
frontend routing, exact package guards/tests, canonical verifier, Index,
Roadmap, and current workflow evidence. No Constitution or ADR file changed.

## Tests

PASS: direct-init Rust 2/2; Android M2-A backend Rust 8/8; M2-A contract 10/10;
M1 preservation contract 9/9; M2-A frontend 11/11; TypeScript; Founder package
contracts; build and APK inspection; exact final Android 36 x86_64 native suite.
Canonical verification passed 33/33 workflow tests, 15/15 Founder package
tests, 399/399 Vitest tests, 241/241 Rust library tests, integration/feature and
legacy-refusal suites, build, whitespace, UTF-8, secret, link, and Constitution
checks.

## Repository Verification

PASS using `powershell -NoProfile -ExecutionPolicy Bypass -File
.\scripts\verify.ps1`, recorded against HEAD and exact digest. The first run
identified one stale M1 identity assertion in the ordinary-package contract;
it was narrowly corrected and the full canonical path then passed.

## Manual Verification

The original candidate failed Founder Step 1 and its evidence is preserved as
`ANDROID-M2A-FOUNDER-STEP1-FAILURE-001`. The corrected exact candidate then
passed Founder manual review 10/10 and was explicitly accepted under
`ANDROID-M2A-FOUNDER-REVIEW-002` Option A. Runbook 13 defines the checklist and
separate automated failure fixtures; native automation remained distinct from
the Founder acceptance.

## Architecture Updates

Architecture 22 records the direct-origin model, separation from migration,
shared versus Android responsibilities, exclusive-create/copy/file-sync
publication, ordered parent-directory durability barriers, fail-closed mixed
states, write acknowledgement, evidence scope, and explicit limitations.

## ADR Updates

None. ADR-0012 remains Accepted and unchanged; this bounded slice implements
its direct fresh-v5 direction without activating the production identity.

## Documentation Synchronization

Index and Roadmap link the new Founder review candidate. Runbook 13 binds the
corrected exact 155,208,006-byte debug APK, SHA-256
`85d6911b34afc31b7e847fc34cd1c8ed05b63fe8084193f7aaf6e0a69e127ebb`,
native report SHA-256
`c15991f9b69dfe5400bb35f0425796af4279890a2a2ac03eaf86e5dbcb51387e`,
HEAD, build/inspect/native steps, manual checklist, preserved Step 1 failure
evidence, failure-fixture separation, and non-authority boundary.

## Data And Migration Impact

No canonical schema shape changed and no database was migrated. Only disposable
synthetic app-private data was created. Direct initialization produces a final
v5 database, direct receipt, enabled database contract, and zero historical
migration receipts.

## Provenance And Consent Impact

The new content-free direct receipt truthfully records origin and pinned schema
facts without user content. No history, provider transmission, consent event,
AI output, or real profile is selected or activated.

## Risks

Evidence covers one Android 36 x86_64 AOSP emulator only. It does not prove
other APIs/ABIs/OEMs, physical devices or power loss, graceful process shutdown,
deterministic raw same-UID SIGKILL, production signing/upgrades, multiprocess
access, backup/restore, full recovery, import/export, or real-data readiness.
Mixed publication states intentionally block rather than self-repair.

## Deferred Items

Any promotion/commit; production identity/signing; real data; broader device
matrix; recovery/backup/retention/delete/import/export; and all other M2
slices.

## Human Decisions

Resolved: `ANDROID-M2A-FOUNDER-REVIEW-002` Option A — corrected manual checklist
10/10 PASS and exact unstaged candidate accepted. The response explicitly
withholds staging, commit, push, merge, PR, archive/reset, release, real data,
`com.lifeos.app`, and another M2 slice.

## Review Cycles

One Founder-authorized revision cycle. The previous candidate failed manual
Step 1; its evidence was preserved, the bounded directory-durability correction
was implemented, and focused/build/inspect/native/canonical verification was
repeated before returning to the Founder gate.

## Workflow Lessons

Exact package-successor assertions must distinguish current generated identity
from immutable accepted predecessor evidence. Native Android app-private
storage cannot assume Windows-like hard-link availability, and synced file
contents do not alone prove durable directory-entry removal. Exclusive
creation, ordered file/directory sync, preserved sources, exact-AVD restart
evidence, and explicit mixed-state refusal form the bounded contract without
overstating atomicity or physical-power-loss proof.

## Recommended Next Sprint

None. The current M2-A review workflow is complete but remains unpublished and
unarchived. Any promotion, repository closeout, production identity, or
subsequent M2 work requires separate explicit authorization.

## Git Status

Branch `codex/android-m2a-direct-fresh-v5`; HEAD
`51dff4998ca8696aeaf1527f058d785303fc0cd4`; all candidate and workflow changes
unstaged; staged files none. No commit, push, merge, PR, archive/reset,
distribution, deployment, release, production activation, real-data access, or
other M2 work occurred.
