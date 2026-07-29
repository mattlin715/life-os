---
status: Implemented
version: 1.2
owner: product-and-engineering
last_updated: 2026/07/28
depends:
  - docs/architecture/01_Local_Evidence_Store.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
referenced_by:
  - docs/00_Index.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
---

# 10 Historical Question Persistence Vertical Slice

## Scope

This document records the founder-approved additive migration and rollback design for the Phase 3B Historical Reflection Question vertical slice. It implements ADR-0009; it does not broaden the task into Phase 4 interpretation.

## Schema v4

Schema v4 leaves `experience_entries` and `persisted_artifacts` intact and adds four tables:

| Table | Boundary |
| --- | --- |
| `historical_consent_events` | Minimal local consent metadata and immutable packet binding. It is not a preference. |
| `historical_transmission_events` | Minimal destination and outcome metadata. Provider error bodies are never copied. |
| `historical_question_artifacts` | Accepted questions, exact successful packet snapshot, and actual-use references. |
| `historical_artifact_dependencies` | Exact source Experience/artifact IDs and revisions used by an accepted artifact. |

Consent and transmission records that are not referenced by an accepted artifact expire after 30 days. Successful packet snapshots and provenance share the generated artifact lifecycle. No additional transport tombstone is retained.

## Provider request retention controls

Implementation verification on 2026/07/14 confirmed that both supported request paths explicitly send `store: false`:

- OpenAI `/v1/responses`: `store: false` prevents Responses application-state retention, while separate abuse-monitoring retention may still apply unless the active project has approved retention controls. See [OpenAI data controls](https://developers.openai.com/api/docs/guides/your-data#default-usage-policies-by-endpoint).
- Gemini `models.generateContent`: the request-level `store: false` overrides project request logging, while separate abuse-monitoring retention and account terms may still apply. See [Gemini logs and datasets](https://ai.google.dev/gemini-api/docs/logs-datasets) and the [GenerateContent API](https://ai.google.dev/api/generate-content).

The preflight states these as separate provider-side facts. Life OS does not claim that local deletion can undo provider receipt or that BYOK implies zero retention.

## Serialized persistence boundary

Immediately before insertion, the Rust transaction re-reads every expected Experience revision, every included persisted artifact revision and eligibility state, and the existing consumed consent plus successful transmission. Consent ID, transmission ID, packet digest, provider, model, and successful outcome must match the packet. Any mismatch returns `stale_generation` without inserting the artifact. The accepted questions, exact packet snapshot, and dependencies are then committed atomically against that validated actual-use chain.

Source Experience edits or deletes and source artifact mutations delete dependent historical question artifacts and their successful packet snapshots. A changed source never silently rewrites a prior generated artifact.

## Rollback and feature disable

The approved rollback is non-destructive:

1. Close the historical transport UI and transport gate.
2. Keep Phase 3A local retrieval and ephemeral selection available.
3. Leave schema v4 tables and records readable; do not decrement `user_version`.
4. Keep prior generated artifacts inspectable but ineligible for reuse while the compatible feature is disabled.
5. Re-enable only with the same or a founder-approved compatible packet and Harness contract.

There is deliberately no automatic down migration. Dropping schema v4 tables would destroy consent and provenance records and therefore requires a new founder checkpoint, an explicit export/retention plan, and separate destructive-migration approval.

## Verification boundary

Automated tests cover additive v3-to-v4 preservation, transaction rollback on injected failure, revision and actual-use-provenance checks, eligibility checks, dependency invalidation, and the architecture/09 evaluation matrix. Founder manual verification completed on 2026/07/14 and passed migration preservation, selection-versus-consent separation, exact-content preflight, cancellation, Gemini governed generation, source-citing output, eligibility, provenance, deletion lifecycle, multilingual parity, and source-revision fail-closed behavior.

The corrective founder retest on 2026/07/14 also passed: an included saved Reflection mutation immediately closed the old preflight without transmission; a locale change required a new disclosure and consent; one unchanged Gemini preflight produced a neutral source-citing question with matching packet, consumed consent, successful transmission, provider/model, digest, revisions, dependencies, and durable artifact provenance; and cancellation created no provider call or new local records.

Residual verification boundaries are explicit rather than implied as complete: the OpenAI live path, an actual source edit during an in-flight provider response, a naturally returned live no-question result, and elapsed 30-day expiry were covered by shared contracts or regression tests rather than this manual run.

## Phase 3 Exit Provenance Inspector P1

Promoted Provenance Inspector P1 adds the Founder-authorized presentation
boundary over one already-loaded
`HistoricalQuestionArtifact` and its exact persisted packet snapshot. The
inspector is collapsed by default, opens only on an explicit local action, and
separately hides exact outgoing content until a second explicit reveal. It
distinguishes packet selection, one-generation consent, successful-transmission
reference, and persisted generated artifact while showing exact questions,
citations, versions, source/artifact revisions, review states, relevance
reasons, snapshot references, and packet-represented dependencies.

P1 adds runtime validation because schema-v4 hydration previously relied on a
TypeScript cast. Malformed, unsupported, incomplete, digest-mismatched, or
contradictory records show one calm fail-closed message instead of partial
provenance. The inspector receives no store, provider, consent, clipboard, or
mutation capability; it neither re-queries nor rehydrates deleted sources.
Existing source invalidation and Historical Question deletion cascades remain
unchanged.

This is schema-neutral and read-only. It creates no query, table, migration,
provider call, consent event, audit event, UI-state record, packet mutation, or
Phase 4 interpretation. It is a partial schema-v4 actual-use view and does not
complete or replace the future schema-v5 provenance/dependency graph inspector.
It is implemented and canonically verified: 26 Vitest files / 204 tests, 70 Rust library tests, 12
backup/restore tests, 8 schema-contract tests, typecheck, frontend build, Rust
check, and repository policy checks passed. The Founder accepted the P1 diff
and completed the stepwise manual UI review on 2026/07/29,
including collapsed/open behavior, four-stage provenance, exact-content
reveal/hide, three-locale parity, R1 coexistence, and restart reset. Destructive
malformed-data and deletion checks relied on the accepted canonical automated
evidence rather than altering real user data. Feature commit
`824a2294f2090c541eff0530063fc0730c18cc63` was promoted to `develop` through
non-fast-forward merge commit
`cf7633780a1a0a72efcad7558e463ceb094468c4`. P1 is not deployed or released and
remains a partial schema-v4 actual-use inspector rather than the future
schema-v5 provenance/dependency graph inspector.
