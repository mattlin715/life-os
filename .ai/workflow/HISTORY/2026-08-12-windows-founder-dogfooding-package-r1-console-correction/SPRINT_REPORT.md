# Sprint Report

Status: completed_with_follow_up

- Artifact schema: 1.0
- Authoring role: orchestrator
- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-console-correction
- Created at: 2026-08-12T21:52:00+09:00
- Updated at: 2026-08-12T21:52:00+09:00

Allowed final status: `completed`, `completed_with_follow_up`,
`human_decision_required`, `failed`, or `cancelled`.

## Sprint ID

2026-08-12-windows-founder-dogfooding-package-r1-console-correction

## Mission

Correct the Founder package Windows subsystem without touching product runtime source.

## Starting Commit

c7e767a397995c1d96daebcafa08eb1e578e356e

## Ending Commit Or Working-Tree State

Unstaged package-only R1 working tree at unchanged HEAD with corrected ignored installer and manifest.

## Final Status

completed_with_follow_up

## Product Decision

The discovered CUI package contradicted the no-console requirement, so a bounded correction was necessary before Founder review. The correction stays within already authorized package scripts/tests/docs.

## Engineering Summary

Added PE subsystem parsing/validation, synthetic tests, and a split frontend/Rust GUI build/Tauri bundle path. Recorded the first linker failure and corrected it with the Rust-compatible CRT entry point.

## Behavior Changed

The review installer now contains a GUI-subsystem application. No product behavior changed.

## Files Changed

Only existing R1 package scripts/tests/runbook and correction workflow evidence changed.

## Tests

6 package tests, 17 workflow tests, 310 Vitest, 189 Rust, 12 backup/restore, 8 schema contract, Clippy, typecheck, build, Rust check, and hygiene passed.

## Repository Verification

Canonical `scripts/verify.ps1` passed after corrected real package build; PE subsystem 2 and manifest verified.

## Manual Verification

Not run; Founder owns the exact package lifecycle review.

## Architecture Updates

No architecture change beyond the original R1 factual synchronization.

## ADR Updates

none

## Documentation Synchronization

Runbook documents GUI subsystem, CRT entry point, and byte-level validation.

## Data And Migration Impact

none; schema remains 4.

## Provenance And Consent Impact

none

## Risks

Windows/MSVC linker specificity and live launch/uninstall behavior remain manual risks.

## Deferred Items

Founder manual review, diff acceptance, and separately authorized promotion.

## Human Decisions

none; correction required no new Founder policy decision.

## Review Cycles

0 workflow revision cycles; one implementation attempt failed at the linker and was corrected within the same bounded package surface with truthful evidence.

## Workflow Lessons

Successful packaging is insufficient evidence for GUI behavior; exact PE-header validation is now part of the contract.

## Recommended Next Sprint

After manual acceptance and promotion, separately gate desktop schema-v5 activation and packaged recovery evidence; Android remains parked.

## Git Status

Branch `codex/windows-founder-dogfooding-package-r1`; HEAD `c7e767a`; no staged files, commit, push, merge, PR, distribution, deployment, or release.
