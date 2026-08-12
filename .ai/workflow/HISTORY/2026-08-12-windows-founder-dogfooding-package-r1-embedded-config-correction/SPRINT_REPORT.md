# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Created at: 2026-08-12T23:04:30+09:00
- Updated at: 2026-08-12T23:04:30+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, `failed`, or `cancelled`.

## Sprint ID

2026-08-12-windows-founder-dogfooding-package-r1-embedded-config-correction

## Mission

Correct the installed Founder package's embedded Tauri identity and production
asset protocol without changing Life OS product runtime behavior.

## Starting Commit

c7e767a397995c1d96daebcafa08eb1e578e356e

## Ending Commit Or Working-Tree State

Unstaged Windows Founder Dogfooding Package R1 working tree at unchanged HEAD,
with one corrected ignored installer and content-free manifest.

## Final Status

completed_with_follow_up

## Product Decision

Step 3 manual evidence showed that the superseded package opened the ordinary
`Life OS` title and attempted to load localhost. That package therefore failed
the private identity and packaged-runtime contract. The bounded correction
compiles the final binary with the exact Founder override and Tauri production
custom protocol, while leaving uninstall, reinstall, launch, and acceptance to
the Founder.

## Engineering Summary

The package builder now passes the exact Founder override through
`TAURI_CONFIG`, enables `tauri/custom-protocol` for the final application
binary, applies Windows GUI linker arguments only to that binary, verifies the
embedded Founder identifier and private title before and after bundling, and
rejects a direct final-binary compile that omits the Founder configuration.
Superseded ignored artifacts were preserved as evidence rather than overwritten.

## Behavior Changed

The review installer now contains a GUI binary with packaged assets and the
Founder-only identity. No Life OS product behavior, storage behavior, AI flow,
or schema behavior changed.

## Files Changed

Only the already authorized R1 package override, build/verification scripts,
package tests, `.gitignore`, package runbook, factual architecture/16 status,
and repository workflow evidence changed. No production React, TypeScript,
Rust database, provider, or ContextPacket source changed.

## Tests

Eight package contract tests, 17 workflow tests, 41 Vitest files / 310 tests,
189 Rust library tests, 12 backup/restore integration tests, 8 schema contract
tests, Clippy with warnings denied, TypeScript typecheck, frontend production
build, Rust check, UTF-8, whitespace, secret, Markdown-link, and Constitution
checks passed.

## Repository Verification

`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`
passed after the final source change. The corrected binary reports PE subsystem
2 and embeds `com.lifeos.founderdogfood` plus `Life OS — Founder Dogfood
(Private)`.

## Manual Verification

Founder Step 1 installer identity passed and Step 2 installation passed for the
superseded package. Step 3 truthfully failed because that package showed the
ordinary title and localhost refusal. The corrected package has not yet been
installed or launched; manual UI status therefore remains `not_run` for this
correction sprint.

## Architecture Updates

No new architecture authority. Existing architecture/16 factual package status
remains bounded to Founder dogfooding and schema v4.

## ADR Updates

none

## Documentation Synchronization

The R1 runbook documents exact embedded-config compilation, custom-protocol,
GUI subsystem verification, marker verification, the superseded package
failure, and the Founder-owned reinstall gate.

## Data And Migration Impact

none; production `SCHEMA_VERSION` and supported maximum remain 4. No profile,
database, migration, backup, restore, or cleanup operation was added.

## Provenance And Consent Impact

none

## Risks

The corrected package still requires live Founder reinstall, launch, restart,
profile-isolation, uninstall, and development-profile preservation review.
Unsigned Windows installer warnings and platform-specific packaging remain
bounded dogfooding risks, not distribution readiness.

## Deferred Items

Founder manual package lifecycle review, Founder diff acceptance, separately
authorized promotion, distributable Private Alpha, production schema v5,
deployment, and release.

## Human Decisions

No new product-governance decision was required. Founder manual acceptance and
promotion remain explicit future gates.

## Review Cycles

One bounded workflow revision cycle. The initial global `RUSTFLAGS` attempt
failed because linker flags reached proc-macro DLLs; the implementation was
replanned to scope linker arguments to the final binary and then passed all
focused and canonical verification.

## Workflow Lessons

Installer metadata and successful bundling do not prove the executable's
embedded Tauri identity or production URL. The package contract must verify the
final binary itself, and broad build flags must not be used where they can alter
dependency artifact types.

## Recommended Next Sprint

Complete the bounded Founder manual lifecycle review. Only after separate
acceptance and promotion should desktop schema-v5 activation or packaged
recovery evidence be considered; Android remains parked.

## Git Status

Branch `codex/windows-founder-dogfooding-package-r1`; HEAD `c7e767a`; no staged
files, commit, push, merge, PR, distribution, deployment, or release.
