# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-24T00:02:00+09:00
- Updated at: 2026-08-24T00:02:00+09:00

## Sprint ID

2026-08-23-ordinary-v5-real-profile-phase-b-blocked-r1

## Mission

Truthfully close the Founder-authorized ordinary schema-v5 activation review
after the single real-profile attempt failed closed, correct the discovered
persistent-WAL verifier and runtime connection-lifetime defects only on
disposable fixtures, and return an automated and Founder-manually reviewed diff
without retrying, repairing, or modifying the real profile.

## Starting Commit

`44ec6d56d645829488aa73d0b92bcf72b19487f4` on
`codex/desktop-schema-v5-ordinary-production-activation-r1`, with `develop` and
`origin/develop` at the same commit.

## Ending Commit Or Working-Tree State

HEAD remains `44ec6d56d645829488aa73d0b92bcf72b19487f4` on the feature branch. The
ordinary activation R1 implementation, documentation, package contracts, and
workflow evidence remain unstaged in the working tree. No commit, push, merge,
PR, deployment, distribution, or release occurred.

## Final Status

completed_with_follow_up

The authorized implementation, canonical verification, disposable Manual
Phase A, and successor persistent-WAL migration/runtime/restart review are
complete. The real Founder ordinary profile did not migrate and remains a
separate Founder-controlled recovery and retry gate.

## Product Decision

Ordinary desktop builds may be schema-v5 capable while retaining
`com.lifeos.app`, but an exact schema-v4 profile migrates only after the bounded
three-language disclosure, verified local backup, and explicit one-time user
authorization. Missing databases initialize at schema v5. Legacy no-default-
feature builds remain schema-v4-only and refuse schema v5. This decision does
not authorize distribution, deployment, release, Android, Phase 4, or a second
real-profile migration attempt.

## Engineering Summary

- Enabled `desktop-schema-v5` for the ordinary desktop default build while
  preserving the legacy no-default-feature schema-v4 boundary.
- Routed reachable Experience, Evidence, Reflection, Pattern, Context Recovery,
  and ADR-0009 Historical Question writes through the typed schema-v5 runtime.
- Preserved explicit migration, verified-backup, conservative restart,
  fail-closed recovery, and explicit restore contracts.
- Added ordinary-only English, Traditional Chinese, and Japanese migration
  purpose, backup-sensitivity, and older-v4 refusal disclosure.
- Corrected Reflection duplicate submission so unchanged saved text cannot be
  submitted again while a genuine edit remains append-only.
- Corrected persistent-WAL exact-v4 inspection and stable runtime reads to use
  immutable sidecar-prechecked connections, and explicitly closed every
  reachable typed writer before durable verification.

## Behavior Changed

The ordinary desktop production-shaped build now starts a missing database at
exact schema v5 and offers an explicit migration gate for exact schema v4. A
checkpointed, sidecar-free persistent-WAL v4 database can be inspected and
migrated without the verifier creating WAL/SHM sidecars. Pre-existing sidecars,
malformed state, ambiguous state, and incompatible schema still fail closed.
After activation, stable reads and typed writes close deterministically so a
normal close/restart does not create a false `sqlite_sidecar_present` recovery
state.

## Files Changed

The unpromoted diff spans the ordinary activation implementation and tests,
Tauri/Rust migration and typed-runtime modules, package and verification
scripts, ordinary review configuration, synchronized architecture/runbook
documents, and truthful workflow archives. The exact promotion allowlist must
be regenerated from the final cleanly archived working tree and approved by a
separate Founder Promotion Authorization Gate; nothing is staged here.

## Tests

- Vitest: 46 files, 349 tests passed.
- Rust default library suite: 204 tests passed.
- Schema-v5 backup suite: 12 passed.
- Schema-v5 contract suite: 8 passed.
- Legacy schema-v4 refusal suite: 33 passed.
- Ordinary activation suite: 9 passed.
- Founder activation suite: 9 passed.
- Migrated persistent-WAL typed runtime suite: 4 passed.
- Rust formatting, Clippy with warnings denied, TypeScript typecheck, frontend
  build, package-contract checks, diff check, UTF-8, secret, link, and
  Constitution checks passed.

## Repository Verification

Fresh canonical verification passed on HEAD
`44ec6d56d645829488aa73d0b92bcf72b19487f4` and final non-workflow digest
`61528d40bd3cde137c51aabc0bc3be2be80ad2af9bbab001d94db20925d024f0`:

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`

The exact verification result was recorded as exit code 0 before final theory
review and sprint closeout.

## Manual Verification

Founder disposable review passed on Windows user `LifeOSReviewR1`:

- Phase A covered cancellation without writes, exact-v4 migration, verified
  backup, restart reconstruction, older-v4 refusal, explicit restore, fresh-v5
  initialization, typed writes, English/Traditional Chinese/Japanese, narrow
  and keyboard review, and uninstall with application data retained.
- The successor persistent-WAL package migrated a fresh exact-v4 fixture once,
  reached `v5_ready / lifecycle_writes_enabled`, preserved its exact-v4 backup,
  left no sidecars after normal close, reconstructed after restart, accepted a
  new typed Experience, and reconstructed both records after the final restart.
- The final UI showed two records; the new `Persistent-WAL runtime review`
  Experience remained the current reflection. Final process count was zero.

The reviewed successor installer is an ignored unsigned disposable package,
version `0.3.0`, size 5,679,953 bytes, SHA-256
`96BB5C8CE77574572375F15359751CF38D7D4B04CB81D5D8A6F1F752BBA51EBF`.
It was not distributed, deployed, or released.

## Architecture Updates

`docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`
now records the ordinary activation boundary, Phase A evidence, the real
Phase B fail-closed outcome, both disposable persistent-WAL correction cycles,
and the remaining independent Founder gate. Architecture 13, 15, 16, and 17
are synchronized with the production-capable versus legacy-v4 distinction.

## ADR Updates

No ADR was added, accepted, or changed. ADR-0007, ADR-0009, and ADR-0011 remain
the governing provenance, historical-context, and append-only lifecycle
authorities.

## Documentation Synchronization

Book One index, schema-v5 activation/readiness architecture, Founder candidate
history, Windows package runbooks, package contract, engineering report, theory
review, and workflow evidence are synchronized. Book Zero and the Constitution
are unchanged.

## Data And Migration Impact

Disposable fixtures proved exact-v4 backup and migration to v5, typed writes,
restart persistence, older-v4 refusal, and explicit restore. The real Founder
ordinary database remains schema v4 and byte-identical to its recorded pre-
attempt state. Its exact-owned prepared operation, zero-byte staging file, and
WAL/SHM evidence remain untouched; no backup was created and no retry, repair,
checkpoint, cleanup, restore, or content inspection occurred.

## Provenance And Consent Impact

No provider, ContextPacket, historical-selection, consent, provenance,
dependency, or actual-use semantics changed. Typed schema-v5 writes preserve
the existing append-only provenance and lifecycle contracts.

## Risks

- The actual ordinary Founder profile remains intentionally fail-closed and
  cannot be treated as migrated.
- Its retained operation and sidecar evidence require an independent Founder
  disposition before any cleanup or retry.
- The reviewed package is unsigned and disposable-review-only; it is not a
  distributable or releasable artifact.
- Android remains blocked by the desktop promotion and real-profile evidence
  fences.

## Deferred Items

Real-profile evidence disposition, any second migration attempt, ordinary
schema-v5 promotion, Private Alpha distribution, deployment, release, Android,
Phase 4, and provider/ContextPacket/consent changes are deferred and separately
gated.

## Human Decisions

The Founder accepted disposable Manual Phase A, authorized exactly one real
Phase B migration action, then selected Option A after that action failed
closed. Option A authorized only disposable persistent-WAL correction and
review. The Founder subsequently completed the successor migration/runtime
manual matrix. No later Founder authorization permits real-profile retry or Git
promotion.

## Review Cycles

- Cycle 0: immutable exact-v4 verifier and persistent-WAL regression.
- Cycle 1: explicit migration-connection close after canonical verification
  exposed nondeterministic post-commit sidecar visibility.
- Cycle 2: immutable activated-runtime reads and explicit typed-writer close
  after the first packaged review surfaced an empty WAL/SHM pair.
- Cycle 3: factual manual-evidence/documentation synchronization and fresh
  canonical verification only; no product-code correction.

## Workflow Lessons

Read-only SQLite is not necessarily side-effect-free for a persistent-WAL
database. Safety classification must check sidecars before opening, use an
immutable connection for stable inspection, and explicitly close writable
connections before durable classification. A safe fail-closed result must
remain distinct from migration success and never imply retry authority.

## Recommended Next Sprint

First hold an independent Founder diff/manual acceptance and Promotion
Authorization Gate for the exact final allowlist. Separately, if desired, hold
a new Founder gate for the retained real-profile evidence disposition and a
possible second migration attempt. Neither action is authorized by this sprint.

## Git Status

- Branch: `codex/desktop-schema-v5-ordinary-production-activation-r1`
- HEAD: `44ec6d56d645829488aa73d0b92bcf72b19487f4`
- `develop` and `origin/develop`: same starting commit at the final pre-close
  audit
- Index: no staged files
- Upstream: none configured
- Working tree: uncommitted ordinary activation R1 implementation, tests,
  documentation, and workflow evidence only; exact final allowlist is reported
  after archive/reset verification
