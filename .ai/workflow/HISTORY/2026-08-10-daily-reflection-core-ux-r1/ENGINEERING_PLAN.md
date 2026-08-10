# Engineering Plan

Status: approved

- Sprint ID: 2026-08-10-daily-reflection-core-ux-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: f8fbf94812bac2e1359367362e9c1e0db1aeaef1
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-10T08:33:00.000Z
- Updated at: 2026-08-10T08:33:00.000Z

## Approved Product Boundary

Product Review is `approved_with_conditions`. Implement only reversible presentation hierarchy, session-local local Experience filtering, native core-path terminology, and progressive ADR-0009 preflight disclosure. Preserve schema v4, existing handlers, storage/provider/ContextPacket/packet/consent/provenance behavior, historical retrieval, and Phase 4 fences.

## Existing Implementation Understanding

`src/app/App.tsx` owns all React state and mutation handlers. The current top bar directly exposes Database Readiness; a full exact technical historical preflight is expanded inline; local history precedes Evidence/Reflection; and every Experience renders its entire workflow. `entries` are already loaded through `LocalEvidenceStore`; `buildHistoricalSavedDateRange` and `isTimestampInHistoricalSavedDateRange` provide the promoted inclusive device-timezone saved-date contract. The preflight packet is already immutable after assembly and send uses that exact object. Therefore this slice can remain presentation-only plus a pure local list projection.

## Affected Modules

Anticipated product/document allowlist before implementation:

- `src/app/App.tsx`
- `src/app/i18n.ts`
- `src/app/i18n.test.ts`
- `src/styles.css`
- `src/app/DailyReflectionComposer.tsx`
- `src/app/DailyReflectionComposer.test.tsx`
- `src/app/SecondaryDataTools.tsx`
- `src/app/SecondaryDataTools.test.tsx`
- `src/app/ExperienceTimelineFilters.tsx`
- `src/app/ExperienceTimelineFilters.test.tsx`
- `src/app/experienceTimeline.ts`
- `src/app/experienceTimeline.test.ts`
- `src/app/HistoricalConsentPreflight.tsx`
- `src/app/HistoricalConsentPreflight.test.tsx`
- `src/app/dailyReflectionFlow.ts`
- `src/app/dailyReflectionFlow.test.ts`
- `docs/11_MVP.md`
- `docs/product/00_MVP_User_Flow.md`
- repository-required current `.ai/workflow/` artifacts and the terminal archive.

No Rust, SQLite, storage, provider, ContextPacket, governed packet-construction, migration, or Constitution path is anticipated.

## Proposed Design

1. Extract a semantic `DailyReflectionComposer` so the primary Experience action is testable and remains ahead of diagnostics.
2. Move portability and Database Readiness into a native `details`-based `SecondaryDataTools` surface. The existing readiness panel and explicit Check handler remain unchanged.
3. Add a pure `experienceTimeline` projection: NFKC/case-normalized keyword matching against Experience body; optional two-ended saved-date range reusing the promoted device-IANA start-inclusive/next-day-exclusive contract; invalid/incomplete range returns an explicit blocked result and no fallback; no state persistence or calls.
4. Render semantic `ExperienceTimelineFilters` with keyword, two saved-date inputs, clear, result count, and localized empty/error disclosure.
5. Add a pure next-action derivation from current Evidence and Reflection review state. Render it in a summary for each Experience; newest is expanded by default and older Experiences use native progressive disclosure.
6. Reorder Experience detail so Evidence and Reflection precede optional historical context; keep Pattern after Reflection and keep every existing action reachable.
7. Extract `HistoricalConsentPreflight`: concise default summary and send/cancel/adjust paths; exact IDs/revisions/content/reasons/retention/packet ID/digest/include-exclude inside an explicit `details` disclosure before the same send action. Details opening has no callback.
8. Replace unnecessary English domain words in ordinary Traditional Chinese/Japanese copy while retaining provider/model names and advanced exact identifiers. Add locale-key and leakage regressions.
9. Add responsive/focus-visible CSS and factual Book One synchronization.

## Alternatives Considered

- Keep all UI in `App.tsx`: rejected because the consent and filter contracts would be difficult to test without brittle source assertions.
- Persist filter preferences: rejected by scope and privacy minimization.
- Reuse the historical per-panel filter state for the timeline: rejected because timeline viewing and historical provider-source selection are separate user intentions.
- Hide packet details entirely: rejected because ADR-0009 requires exact disclosure before consent.
- Use a custom modal: rejected; native inline `details` preserves keyboard semantics and avoids new focus-trap complexity.

## Data Lifecycle Impact

None. New keyword/date/diagnostic/disclosure/open-state values remain React memory only and disappear on restart. Existing Experience and artifact lifecycle is untouched.

## SQLite Or Migration Impact

None. No SQL, Rust, migration, `SCHEMA_VERSION`, `user_version`, startup, backup, restore, or app-data path change.

## Provenance Impact

Presentation only. Existing actual-use provenance persistence and P1 validator are unchanged; ordinary labels become native while exact identifiers remain visible in advanced views.

## Historical Context Impact

The existing Phase 3A retrieval, ranking, caps, range control, ephemeral selection, and preflight invalidation remain unchanged. Timeline filters merely project the loaded Experience list and never alter the candidate input.

## Consent Impact

No semantic change. The same immutable disclosed packet is passed to the same consent/send handler. Opening preflight or details performs no consent action. Source adjustment cancels only the displayed preflight and preserves the existing ephemeral source-selection workflow. Cancellation remains no-call/no-record.

## Provider Transmission Impact

None. No provider or ContextPacket files change. The send handler, destination revalidation, no historical fallback, stale response rejection, and output evaluator remain unchanged.

## Import And Export Impact

No format change. Existing Experience-only import/export controls move into the secondary data surface and retain their handlers.

## Test Strategy

- Pure filter tests: keyword, NFKC/case behavior, inclusive saved-date/timezone boundary, combined filters, invalid/missing ranges, malformed timestamps, clear/full list, stable order, no mutation.
- Filter component tests: all locales, semantic inputs, result count, reset, empty/error copy.
- Next-action tests: no Evidence, pending Evidence, confirmed Evidence, suggested/answered/skipped Reflection.
- Composer and secondary-tools tests: Experience-first composition; readiness absent from primary component and present only inside secondary details; no write-capable readiness control.
- Preflight tests: concise summary, types/count, one-use consent, exact details before send, packet ID/digest, source/artifact controls, locale parity, no event on details rendering.
- i18n tests: exact key parity and bounded core-path English leakage denial for Traditional Chinese/Japanese.
- Existing Phase 3A/R1, Phase 3B packet, provider, provenance, storage, and no-Phase-4 regression suite through canonical verification.

## Repository Verification Strategy

Run focused Vitest during development, `pnpm typecheck`, `pnpm build`, and final `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`. Independently compare all tracked and non-ignored untracked paths with this allowlist, confirm no staged files, Constitution diff, schema v4, and unchanged provider/ContextPacket path set.

## Manual UI Verification

Founder-owned after automated completion. Use safe ordinary product data and the existing provider configuration. Verify primary action, create/review/reflect journey, locale terminology, timeline filters, preflight summary/details/source adjustment/cancel, and secondary Database Readiness. Do not fabricate manual pass status.

## Rollback Or Recovery Strategy

All product state additions are non-durable. Closing/reloading clears filters and disclosure state. Reverting the allowlisted React/i18n/CSS/docs diff restores prior hierarchy without database recovery. Existing storage/provider records are neither transformed nor removed.

## Documentation Impact

Factual Book One updates only in `docs/11_MVP.md` and `docs/product/00_MVP_User_Flow.md`. No Book Zero rewrite, architecture authority change, or new UX document is necessary.

## ADR Impact

No new ADR. This is an implementation of existing MVP/ADR-0009 presentation requirements and does not alter a durable data or product-policy decision.

## Risk Level

medium. No data authority changes, but the UI touches the core daily journey and pre-consent disclosure. Component isolation, exact handler reuse, behavior tests, canonical verification, and Founder manual review bound the risk.

## Escalation Decision

No escalation. The Founder explicitly authorized the product meaning and autonomous reversible implementation. Stop only if implementation would require a non-allowlisted semantic change such as weaker consent, schema activation, or new provider behavior.
