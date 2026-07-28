# Engineering Plan

Status: approved

- Sprint ID: 2026-07-27-phase-3-exit-provenance-inspector-p1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: db43b8f47815b0c6ddb1bd8daa9a9503c11a2146
- Working-tree digest reviewed: 0136a4f4c038f8ebad19915c9506f1e6268333d0341d9d9e24db6cce22ff54f1
- Created at: 2026-07-27T16:34:42.9511933Z
- Updated at: 2026-07-27T16:34:42.9511933Z

Allowed final status: pproved, evision_required, or
human_decision_required.

## Approved Product Boundary

Product Review is pproved_with_conditions after the Founder resolved
PHASE3-PROVENANCE-INSPECTOR-P1-001 with Option A. Implement only a
collapsed-by-default, explicit-open, local, read-only inspector for one already
loaded schema-v4 HistoricalQuestionArtifact and its exact persisted packet
snapshot. Exact outgoing content requires a second explicit reveal. The
inspector must fail closed in English, Traditional Chinese, and Japanese and
must not add persistence, queries, provider behavior, consent behavior,
schema changes, Phase 4 interpretation, or Harness functionality.

## Existing Implementation Understanding

The existing schema-v4 storage path hydrates each Historical Question from its
persisted payload and exact packet snapshot. ADR-0009 persistence already
requires an authorized, successful transmission; exact packet digest,
provider/model, source revision and eligibility revalidation; valid citations;
and exact dependencies. Source mutation/deletion and Historical Question
deletion already enforce the governed cascades. The current UI renders only a
short provenance summary and questions. Runtime hydration uses a TypeScript
cast, so the new read-only surface needs an explicit runtime validator before
showing detailed metadata.

## Affected Modules

- src/historicalContext/provenanceInspector.ts: pure validator and view-model builder.
- src/historicalContext/provenanceInspector.test.ts: malformed, contradictory, privacy, and boundary tests.
- src/app/HistoricalProvenanceInspector.tsx: collapsed local inspector and separate exact-content reveal.
- src/app/HistoricalProvenanceInspector.test.tsx: UI disclosure and fail-closed rendering tests.
- src/app/App.tsx: render the inspector for an already-loaded Historical Question.
- src/app/i18n.ts and locale tests: equivalent English, Traditional Chinese, and Japanese copy.
- src/styles.css: bounded inspector presentation.
- Factual Book One documents describing the implemented but unpromoted P1 state.
- Repository workflow artifacts required by the current sprint.

## Proposed Design

1. Accept the already-loaded artifact as unknown at the validator boundary.
2. Validate required artifact, question, citation, packet, destination, consent,
   version, included-item, dependency, authorship, review-state, relevance, and
   snapshot fields against the already accepted Phase 3B contract.
3. Reject unsupported versions, malformed data, duplicate or contradictory
   identities/dependencies, invalid citations, and digest mismatch. Never
   partially render invalid provenance.
4. Produce an immutable presentation view model containing four distinct stages:
   selected packet content, consent reference, successful-transmission reference,
   and persisted generated artifact.
5. Keep the inspector collapsed until explicit open. Keep exact outgoing content
   under a second collapsed control. Closing or unmounting discards component
   disclosure state; no state is persisted.
6. Give the component no store, provider, consent, clipboard, or mutation
   capability. It consumes only the artifact prop.
7. Preserve current retrieval, R1, provider, packet assembly, and deletion paths.

## Alternatives Considered

- Display raw JSON: rejected because it is not calm, comprehensible, or safely
  fail-closed and would blur the four lifecycle stages.
- Re-query or rehydrate current sources: rejected because it can resurrect
  deleted content and would not represent actual-use evidence.
- Add schema-v5 graph inspection now: rejected as outside P1 and unauthorized.
- Continue showing only the current short summary: insufficient for the approved
  actual-use transparency gap.

## Data Lifecycle Impact

None. The component reads the already-loaded in-memory artifact only. Opening,
closing, and revealing content create no durable state, audit event, consent,
clipboard entry, or provider call. Existing deletion and invalidation cascades
remain unchanged.

## SQLite Or Migration Impact

None. No query, command, table, index, trigger, migration, SCHEMA_VERSION, or
user_version change is planned.

## Provenance Impact

The UI exposes existing exact packet and generated-artifact provenance without
creating or modifying provenance. Runtime validation prevents malformed or
contradictory evidence from being presented as trustworthy. P1 remains partial
and does not replace the future schema-v5 provenance/dependency graph inspector.

## Historical Context Impact

None to retrieval, selection, assembly, or transmission. The inspector reads a
single already-loaded persisted Historical Question and its packet snapshot; it
never loads history or current source content.

## Consent Impact

None. Existing consent references are disclosed as evidence. Opening or
revealing never requests, consumes, renews, or implies consent.

## Provider Transmission Impact

None. No provider or ContextPacket code is changed and no network-capable
interface is supplied to the inspector.

## Import And Export Impact

None. No clipboard, import, export, download, or copy behavior is added.

## Test Strategy

- Pure validator/view-model tests for one valid artifact and each malformed,
  unsupported, incomplete, duplicate, digest, citation, dependency, and
  lifecycle contradiction boundary.
- UI tests for collapsed default, explicit open, local-only notice, four-stage
  distinction, separate exact-content reveal, close/reset, and generic
  fail-closed message without partial disclosure.
- Locale-key parity and exact English/Traditional Chinese/Japanese messages.
- Existing R1, Phase 3B, storage cascade, privacy, evaluation, and no-Phase-4
  regressions continue through canonical verification.

## Repository Verification Strategy

Run focused Vitest files during implementation, then run:

powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1

Record the canonical result through the repository workflow before Theory
Alignment Review.

## Manual UI Verification

Owner: Founder. After automated verification and archive/reset, provide a
stepwise matrix covering collapsed default, explicit local open, four stages,
exact citations and packet fields, separately hidden/revealed outgoing content,
three locales, malformed fail-closed fixture where practical, R1 regression,
and deletion behavior. Stop before promotion.

## Rollback Or Recovery Strategy

The change is schema-neutral and additive at the presentation layer. Reverting
the component, validator, integration, locale copy, tests, and factual docs
restores the prior summary UI without database recovery. No data rollback is
required.

## Documentation Impact

Synchronize only factual Book One product/architecture/roadmap material. Mark
P1 as implemented and verified in the working tree only after evidence exists,
and clearly distinguish Founder review/promotion/deployment states. Do not
change Book Zero, the Constitution, or archived history.

## ADR Impact

No new ADR and no ADR status change. The implementation applies existing
ADR-0009 actual-use provenance and deletion semantics without changing policy.

## Risk Level

Medium. The surface is read-only and schema-neutral, but a permissive validator
could misrepresent incomplete or contradictory actual-use evidence, while an
overly strict validator could hide valid historical records. Focused fixtures
and canonical regressions bound both risks.

## Escalation Decision

No new Founder escalation is required. The exact P1 authority is resolved. If
implementation reveals a contradiction requiring new storage access, a packet
contract change, consent-policy change, or schema-v5 behavior, stop at `human_decision_required` rather than expanding scope.
