# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-console-correction
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest reviewed: current package working tree
- Created at: 2026-08-12T21:30:00+09:00
- Updated at: 2026-08-12T21:30:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Correct a concrete package reliability defect discovered after the original workflow archive. This is not a new product feature or Harness expansion.

## Problem Statement

The embedded release executable reports PE subsystem 3 (`windows_cui`), so the first built artifact could show a console and does not satisfy the authorized package acceptance criterion.

## User Value

The Founder gets an installed desktop app without a visible console while ordinary development and product runtime source remain unchanged.

## Relevant Primary Definitions

The Constitution and local-first/privacy definitions remain unchanged; the correction concerns only packaging truth and psychological clarity.

## Relevant ADRs

ADR-0007, ADR-0009, and ADR-0011 are unaffected.

## Current Implementation Context

The package scripts and first installer are implemented and automated-verified, but PE inspection disproved one acceptance claim after archive. No Founder manual acceptance or promotion occurred.

## In Scope

Compile only the final Rust binary with `/SUBSYSTEM:WINDOWS`, validate PE subsystem 2 before bundling, add synthetic PE tests, rebuild and verify the ignored package.

## Out Of Scope

Any change to `src-tauri/src`, product behavior, schema, provider, consent, deployment, distribution, release, or normal dev configuration.

## Product Constraints

Do not hide a console by changing ordinary runtime source. Fail closed before bundling if the PE header is malformed or not GUI subsystem 2.

## Evidence And Provenance Constraints

No product evidence/provenance impact.

## Historical Context Constraints

No historical context impact.

## Consent Constraints

No consent impact.

## AI-Role Constraints

No AI-role change.

## Privacy Constraints

No data/profile change; preserve the already validated distinct identifier.

## User-Agency Constraints

Founder retains install and manual acceptance control.

## Acceptance Criteria

1. Synthetic PE GUI binary passes and CUI/malformed binaries fail.
2. Package build compiles only the final binary with Windows subsystem linker flag.
3. Built release executable reports subsystem 2 before bundling.
4. New installer manifest verifies and replaces only the ignored review artifact.
5. Canonical verification passes and product runtime source has no diff.

## Risks

Cargo/Tauri split build and bundle commands could drift; the build must fail closed. Linker behavior must be inspected from bytes rather than inferred from flags.

## Open Questions

none

## Human Decision Required

false; this is a bounded correction within the already authorized package-script surface.

## Recommendation

Proceed, with no product source changes and no manual/release claims.

## Review Status

approved_with_conditions
