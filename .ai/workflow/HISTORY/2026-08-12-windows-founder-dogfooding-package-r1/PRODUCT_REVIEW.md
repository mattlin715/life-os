# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: c7e767a397995c1d96daebcafa08eb1e578e356e
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-12T11:55:00+09:00
- Updated at: 2026-08-12T11:55:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

The mission is a bounded access and packaging slice for one informed Founder. It converts the already-promoted schema-v4 desktop product into an isolated Windows installer without changing product meaning, data semantics, AI behavior, or distribution status.

## Problem Statement

The ordinary Tauri configuration builds a generic Life OS application using identifier `com.lifeos.app`, and development startup requires repository tooling. Installing that identity for dogfooding could resolve the same application data location as ordinary development. No verified Founder-only installer lifecycle currently exists.

## User Value

The Founder can use the current daily-reflection product without a dev server while keeping dogfooding data physically separated from the ordinary development profile. A content-free checksum manifest makes the exact reviewed installer identifiable without collecting user data.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: the package remains a user-controlled mirror and does not add AI authority.
- `docs/03_Principles.md`: local-first control and evidence-before-conclusion remain unchanged.
- `docs/06_Memory.md`: the package must not blur or silently combine local data profiles.
- `docs/09_AI.md` and `docs/10_Privacy.md`: no provider, consent, telemetry, or data-transmission changes are allowed.

## Relevant ADRs

- ADR-0007: reviewed artifacts and provenance semantics remain unchanged.
- ADR-0009: governed historical context and consent remain unchanged.
- ADR-0011: lifecycle and portability authority is not expanded by packaging.

## Current Implementation Context

The schema-v4 product and its Tauri configuration are implemented and promoted. Architecture/16 is Founder-approved and promoted at merge `c7e767a`; it authorizes the direction but distinguishes packaging implementation, manual acceptance, promotion, distribution, deployment, and release. Production `SCHEMA_VERSION` remains 4. Private schema-v5 work is evidence only and is not activated.

## In Scope

- One additive Windows Tauri configuration override with a distinct product name, bundle identity, window title, and current-user NSIS packaging.
- Repository build and verification scripts that emit an ignored package directory and a six-field content-free manifest.
- Static and synthetic focused tests for identity/profile separation, unchanged normal configuration, manifest integrity, output ignore rules, and forbidden capabilities.
- Minimal architecture/16 promotion reconciliation and a factual operator guide.
- Canonical verification, Product/Theory review, workflow archive, and Founder manual-review instructions.

## Out Of Scope

All React product code, Rust runtime/SQLite logic, storage, schema, migration, providers, ContextPacket, consent, Book Zero, updater, telemetry, signing, distribution, deployment, release, Android, and schema-v5 activation. No install or uninstall is performed autonomously.

## Product Constraints

- Preserve normal `src-tauri/tauri.conf.json` byte-for-byte.
- Keep `SCHEMA_VERSION = 4` and ordinary development behavior unchanged.
- The package is explicitly Founder-only, unsigned, local, and non-distributed.
- The dogfooding identifier must cause Tauri app-data resolution to differ from `com.lifeos.app`; the package must not accept a configurable database path.
- Package output must be ignored and manifest content-free.

## Evidence And Provenance Constraints

No product evidence or provenance changes. The manifest proves only installer artifact identity using version, Git SHA, target, filename, size, and SHA-256; it contains no user path, database metadata, content, secret, or runtime state.

## Historical Context Constraints

No retrieval, selection, consent, packet, transmission, or provenance behavior changes.

## Consent Constraints

No consent behavior changes. Packaging must not imply provider or historical-use consent.

## AI-Role Constraints

No AI-role change. Local mock and configured providers retain their current behavior; the package does not add a provider, fallback, prompt, evaluator, or model call.

## Privacy Constraints

Distinct bundle identity is mandatory so the installed package cannot resolve the ordinary development application data directory. No telemetry, logs upload, cloud service, updater, or user-data manifest content is allowed.

## User-Agency Constraints

Installation and manual use are Founder-controlled. The script builds but does not install, launch, uninstall, distribute, sign, or approve the package. Uninstall data retention must be observed and recorded rather than inferred.

## Acceptance Criteria

1. Normal Tauri config and identifier remain unchanged.
2. Founder override names the package clearly and uses a distinct identifier/profile.
3. A repository command builds one unsigned Windows NSIS installer without a dev server requirement after installation.
4. Output is ignored; a deterministic six-field manifest matches the closed installer bytes.
5. Focused tests reject identity collisions, malformed manifests, filename/path leakage, size/digest mismatch, forbidden override keys, and non-ignored output.
6. Static diff inspection finds no product, schema, provider, ContextPacket, consent, updater, signing-secret, telemetry, deployment, Phase 4, or Android changes.
7. Canonical verification passes before manual review.
8. Founder manual install/start/restart/uninstall review remains a distinct pending gate.

## Risks

- Windows unsigned installer warnings are expected and must be disclosed.
- Tauri/NSIS output naming or target metadata could drift; scripts must fail closed on ambiguity.
- Identifier separation does not by itself prove uninstall data removal; manual review records observed retention.
- The installer is built from an unstaged sprint diff over the recorded Git SHA; documentation must not represent it as promoted or releasable.
- A package build can be toolchain-sensitive and is not claimed byte-reproducible across machines.

## Open Questions

None. Founder authority and the exact bounded package contract are already explicit.

## Human Decision Required

false; no unresolved consequential decision.

## Recommendation

Proceed with the smallest package-only implementation. Conditions: preserve base config and schema v4; enforce profile separation and exact manifest validation; keep outputs ignored; make no runtime/product changes; stop before promotion and before any autonomous install.

## Review Status

approved_with_conditions
