# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-embedded-config-correction
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest reviewed: 812942ca4eb118d909c6cced8d1aff75c5773ceea4b41ee09da0b8e91a0da31c
- Created at: 2026-08-12T22:05:00+09:00
- Updated at: 2026-08-12T22:05:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Correct a concrete installed-package failure found by Founder manual review. This is a package implementation correction, not a new product feature or Harness expansion.

## Problem Statement

The installed package showed title `Life OS` and `ERR_CONNECTION_REFUSED` for localhost. The package had installer-level Founder identity but the embedded executable used the normal development-oriented Tauri configuration because direct `cargo rustc` did not receive the Founder config merge.

## User Value

The installed Founder package must start the bundled frontend, show the private Founder title, and require no development server or console while remaining isolated from the ordinary development profile.

## Relevant Primary Definitions

Constitution, local-first control, privacy, and user agency require truthful packaging and separation. No theory change is proposed.

## Relevant ADRs

ADR-0007, ADR-0009, and ADR-0011 remain unaffected.

## Current Implementation Context

R1 is implemented and automated-verified but failed Founder manual Step 3. It is not Founder-manually accepted, promoted, distributed, deployed, or released. The installed failed package is local manual evidence only.

## In Scope

Compile the final application binary with the exact Founder override supplied through Tauri's documented `TAURI_CONFIG` merge and with package-local Windows GUI linker flags, then bundle using the same override. Validate package script structure, embedded-config markers, GUI PE bytes, identity/profile contract, exact manifest, and canonical repository state.

## Out Of Scope

Any production/runtime source change, schema/provider/consent change, automatic install/uninstall, suppression of the failure evidence, or expanded package authority.

## Product Constraints

Normal config remains byte-unchanged; any direct final-binary compile must supply the exact Founder config through `TAURI_CONFIG`; environment state must be restored after the build; no dev server is started.

## Evidence And Provenance Constraints

No product evidence or provenance impact.

## Historical Context Constraints

No historical-context impact.

## Consent Constraints

No consent impact.

## AI-Role Constraints

No AI-role impact.

## Privacy Constraints

Distinct identifier/profile boundary remains mandatory and unchanged.

## User-Agency Constraints

Founder controls closing, uninstalling, reinstalling, relaunching, and acceptance. The tool does not alter installed state.

## Acceptance Criteria

1. Build script compiles only the final application binary with exact Founder `TAURI_CONFIG` plus GUI linker flags, then bundles with the same override.
2. Build script restores prior `TAURI_CONFIG` even on failure.
3. Contract tests reject a direct compile missing the Founder config, missing flags, or environment restoration.
4. Final release binary remains PE GUI subsystem 2 and installer manifest verifies.
5. Canonical verification passes with no protected-surface diff.
6. Founder repeats install and Step 3; only live evidence can close the embedded-config defect.

## Risks

Tauri may overwrite linker environment or the installed package may retain cached files. Script and manual reinstall must fail clearly; old installed package must be uninstalled by the Founder before retesting.

## Open Questions

none

## Human Decision Required

false; the correction is within the already authorized package-script surface and responds to direct manual evidence.

## Recommendation

Proceed with the bounded build-path correction, preserve all state distinctions, and return to manual Step 2R/3R without claiming acceptance.

## Review Status

approved_with_conditions
