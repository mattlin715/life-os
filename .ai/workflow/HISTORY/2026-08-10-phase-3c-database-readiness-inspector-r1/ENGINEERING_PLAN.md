# Engineering Plan

Status: approved

- Sprint ID: 2026-08-10-phase-3c-database-readiness-inspector-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: bed87283f9141144a1c11500457363d5a8081aed
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-09T20:32:00Z
- Updated at: 2026-08-09T20:32:00Z

## Approved Product Boundary

Product Review is `approved`. Implement only architecture/15 Option A: explicit-open, session-only, read-only production-path readiness inspection and three-language disclosure. No write-capable behavior or broader Phase 3C activation.

## Existing Implementation Understanding

`src-tauri/src/sqlite.rs` owns schema-v4 startup inspection/initialization and must keep those semantics. `src-tauri/src/filesystem_safety.rs` contains private path, ownership, sidecar, and operation-state safety primitives. `src-tauri/src/lib.rs` registers Tauri commands. `src/app/App.tsx` owns startup gating and session UI state. Existing React tests use static rendering and storage adapters use injected `invoke` functions for deterministic testing.

## Affected Modules

Exactly the architecture/15 allowlist: `src-tauri/src/filesystem_safety.rs`, `src-tauri/src/sqlite.rs`, `src-tauri/src/lib.rs`, new `src/shared/storage/sqlite/databaseReadiness.ts` and test, new `src/app/DatabaseReadinessPanel.tsx` and test, `src/app/App.tsx`, `src/app/i18n.ts` and its test, `src/styles.css`, architecture/13, architecture/15, and repository-required workflow artifacts.

## Proposed Design

1. Add a read-only filesystem snapshot helper that validates the canonical owned database boundary, detects WAL/SHM/journal presence, and conservatively detects owned-operation evidence without writing or selecting a candidate.
2. Add a path-injected readiness core plus a Tauri command that returns a fixed, content-free result union. Use create-disabled read-only immutable SQLite inspection. Map all internal errors to bounded classifications; never return raw paths or errors.
3. Add a typed TypeScript adapter with strict runtime validation and fail-closed normalization.
4. Add a closed-by-default panel. Opening does not invoke the command; `Check now` performs one fresh inspection. Results and open state remain in React memory only.
5. Add equivalent English, Traditional Chinese, and Japanese copy and focused static UI, adapter, lifecycle, and Rust fixture tests.

## Alternatives Considered

- Reuse startup inspection directly: rejected because it omits architecture/15 path/sidecar/operation-evidence disclosure and conflates startup with explicit inspection.
- Expose raw filesystem/SQLite errors: rejected for privacy and stable-contract reasons.
- Add upgrade or recovery controls: rejected as unauthorized and unsafe.

## Data Lifecycle Impact

None. The command and UI persist no result, preference, event, receipt, manifest, operation state, or audit record.

## SQLite Or Migration Impact

No schema or migration impact. `SCHEMA_VERSION` and supported maximum remain 4. Inspection uses only read-only/create-disabled operations against an existing file after explicit user action.

## Provenance Impact

None. No provenance-bearing record is created, changed, or deleted.

## Historical Context Impact

None. No historical source, packet, artifact, or provider path is read or changed.

## Consent Impact

None. The local button action is not historical-context consent and creates no durable authorization.

## Provider Transmission Impact

None. No provider or network call is introduced.

## Import And Export Impact

None.

## Test Strategy

- Rust disposable fixtures: missing path, v3, v4, v5, malformed, unreadable where platform-reliable, unsafe path/link/hard-link, sidecars, owned-operation ambiguity, repeated checks, metadata/byte immutability, and schema constant.
- TypeScript adapter: no import-time invocation, exact command name, valid result parsing, malformed/unexpected payload fail closed, and raw error suppression.
- UI/i18n: collapsed/open/check lifecycle, read-only controls, classification disclosure, no prohibited action labels, close/session behavior, and locale parity.
- Regression: existing startup compatibility, v4 product, R1 historical retrieval, provenance inspector, and canonical suite.

## Repository Verification Strategy

Run focused Vitest and Rust tests, `cargo clippy --all-targets -- -D warnings`, then `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Confirm Constitution unchanged, schema v4, no unexpected allowlist paths, and workflow validity.

## Manual UI Verification

Founder review is required after implementation. Verify closed default, explicit check, each locale, v4 disclosure, blocked/uncertain synthetic evidence only where safely reproducible, no write controls, close/reset behavior, and unchanged normal product flow.

## Rollback Or Recovery Strategy

The feature has no durable state. Closing the panel removes the disclosure from view; reverting the allowlisted code removes the inspector. Backend uncertainty returns a bounded blocked classification and performs no recovery action.

## Documentation Impact

Factual synchronization only in architecture/13 and architecture/15: promoted audit gate facts, implemented R1 evidence after verification, and unchanged production activation fences.

## ADR Impact

No new ADR. ADR-0007, ADR-0009, ADR-0011, and Founder-approved architecture/15 already govern the bounded behavior.

## Risk Level

medium: the feature is read-only and reversible, but it touches the production database path and must prove that no filesystem or SQLite API produces side effects.

## Escalation Decision

No escalation required. Stop at `human_decision_required` if implementation reveals a consequential ambiguity not resolved by architecture/15; otherwise proceed and stop at Founder diff/manual review.
