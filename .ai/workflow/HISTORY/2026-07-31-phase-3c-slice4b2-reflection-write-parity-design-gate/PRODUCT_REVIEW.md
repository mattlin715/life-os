# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-31-phase-3c-slice4b2-reflection-write-parity-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `bb7ef2a6b37f8b4c7fdeab6ee0063d469dd8f011`
- Working-tree digest reviewed: `26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e`
- Created at: 2026-07-31T00:30:00+09:00
- Updated at: 2026-07-31T01:10:00+09:00

## Mission Interpretation

Implement the Founder-authorized smallest private disposable Reflection write-parity boundary that preserves separate AI/local-mock prompt authorship, user response authorship, exact dependencies, explicit skip, append-only response correction, guarded v4 projection, and fail-closed transaction evidence.

## Problem Statement

Production schema remains v4. The promoted disposable v5 foundation now covers Experience and bounded Evidence writes, while Reflection remains migration-baseline evidence only. Current v4 stores prompt, response, status and two provenance roles in one mutable JSON record and whole-bundle replacement can erase lineage. A v5 boundary must split immutable revision facts from current projection without treating the AI prompt as user content or a saved response as approval of the prompt.

## User Value

The design protects the user's exact words, preserves who authored each part, makes skip explicit, prevents stale/deleted/rejected Evidence from silently supporting Reflection, and keeps correction revisable without rewriting history.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI; Reflection before Answer; Evidence before Conclusion.
- `docs/03_Principles.md`: AI remains a mirror and visible uncertainty preserves user judgment.
- `docs/06_Memory.md`: records serve evidence and meaning remains user-owned.
- `docs/Reflection.md`: Reflection begins with Evidence and must not become an AI conclusion.
- `docs/09_AI.md`: AI may offer questions, not identity authority.
- `docs/10_Privacy.md`: local-first, bounded use, deletion and provenance remain user-controlled.

## Relevant ADRs

- ADR-0007: persist reviewed artifacts with exact provenance and keep AI/user authorship distinct.
- ADR-0009: answered Reflection can be eligible historical context only with exact user response provenance and current confirmed Evidence; this design does not implement v5 historical cascades.
- ADR-0011: immutable revisions, exact review events, correction/supersession, exact dependencies, guarded current projection, and content-free lifecycle facts.

## Current Implementation Context

Implemented and promoted: fixed schema-v5 DDL and disposable migration/restart evidence; Slice 4A Experience writes; Slice 4B-1 bounded Evidence candidate/review writes. Current v4 UI generates AI or local-mock prompts, keeps drafts ephemeral until save, saves user response provenance, and explicitly records skipped current state through typed whole-bundle persistence. Proposed only: Slice 4B-2. Production-authorized state remains schema v4; no v5 runtime registration exists.

## In Scope

Propose one private, unregistered, path/connection-injected Rust boundary against exact-v5 disposable fixtures with four typed operations:

1. `CreateSuggestedPrompt`: one AI/local-mock prompt revision with exact prompt provenance, exact current Experience revision dependency, and one or more exact current confirmed Evidence revision dependencies.
2. `SaveResponse`: require the exact current suggested prompt revision and unchanged eligible dependencies; append a mixed-content `answered` revision whose question bytes remain AI/local-mock-owned and whose non-empty response has exact user provenance.
3. `CorrectResponse`: require the exact current answered revision, unchanged question, changed non-empty response, current eligible dependencies and no inbound dependents; append a mixed `corrected` revision plus corrected/superseded lifecycle events without rebinding prior dependents.
4. `SkipSuggestedPrompt`: require the exact current suggested prompt revision; append one exact-revision `skipped` review event, retain prompt content, create no response provenance, and project skipped current state without creating a response revision.

## Out Of Scope

Production v5, real data/app-data, Tauri/renderer/UI/startup, confirmed-Evidence correction/deletion, background dependent invalidation, Pattern/Recovery/Historical Question parity, Phase 3B v5 cascade writes, providers/ContextPacket/consent changes, Phase 4, export v2, retention, backup/restore, autonomous recovery, Harness work, Git promotion, deployment and release.

## Product Constraints

- Saving or correcting a response is authorship, not confirmation of the prompt as truth.
- Skip requires explicit user action; silence, close, timeout or navigation never imply skip.
- Suggested/skipped prompts are ineligible as historical context. Only an answered current revision with non-empty user response provenance and valid exact dependencies may be eligible.
- The prompt question and its prompt provenance are immutable across answer/correction.
- No operation performs whole-bundle replacement.

## Evidence And Provenance Constraints

- Initial prompt revision authorship is exactly `ai` or `local_mock`.
- Answered/corrected combined revisions use `mixed` authorship; role `prompt` retains the exact original prompt provenance and role `response` is exact user provenance.
- Exact provenance fingerprints may deduplicate only identical canonical facts.
- Every revision repeats exact `derived_from_experience` and `uses_evidence` edges; answered/corrected revisions also use `answers_prompt` to the immutable initial prompt revision.
- No dependency is rebound to a newer Experience or Evidence revision.

## Historical Context Constraints

The boundary may project a valid answered Reflection for existing v4-compatible behavior in disposable fixtures, but it does not create Phase 3B packet, consent, transmission, Historical Question, or v5 cascade writes. Any existing inbound normalized or ADR-0009 dependent blocks response correction because required invalidation is outside this slice.

## Consent Constraints

No new consent, provider use or historical transmission. Selection and storage do not imply consent.

## AI-Role Constraints

AI/local-mock authors only the prompt. The response is always user-authored. The boundary records no interpretation, Pattern, diagnosis, identity statement or cross-experience conclusion.

## Privacy Constraints

Synthetic/disposable exact-v5 fixtures only; no app-data, network, provider, telemetry, clipboard, export, real user text or production activation.

## User-Agency Constraints

Exact-current-revision checks prevent stale overwrite. Skip is explicit. Correction appends rather than mutates. Failure and ambiguous COMMIT classification never retry, repair, select a candidate or infer intent.

## Local-First Data Architecture Analysis

### State mapping

| Product state | v5 authority | v4 projection |
| --- | --- | --- |
| Suggested | AI/local-mock prompt revision; head `pending`, active, ineligible | `status=suggested`, no response |
| Answered | new mixed `answered` revision; prompt and response provenance roles; head `not_applicable`, active, eligible only while dependencies match | `status=answered`, exact response and both provenance objects |
| Corrected response | new mixed `corrected` revision; predecessor answered revision; corrected/superseded events | answered projection with latest response; old revision remains non-current |
| Skipped | unchanged prompt revision plus exact `skipped` review event; head skipped/ineligible | `status=skipped`, no response or response provenance |

### Transaction contract

Each operation must: verify exact schema/receipt/contract and empty guard; `BEGIN IMMEDIATE`; revalidate source, prompt and confirmed-Evidence heads/revisions; refuse malformed/cross-source/stale/deleted/rejected/ineligible or inbound-dependent state; claim one deterministic guard; append v5 authority/events/dependencies/provenance; write guarded v4 projection last; reconcile exact operation manifest, foreign keys and integrity; commit through injectable outcome adapter; close writable connection; classify only exact durable pre/post state read-only; never retry or repair.

### Reversibility

The module is private and unregistered, so removing it and its tests leaves production unchanged. Disposable fixture transactions roll back logically on definite pre-commit failure. Ambiguous outcomes are classified, not reversed. No down migration is introduced.

## Acceptance Criteria

1. Exact-v5 fixtures originate only through the promoted migration core.
2. AI and local-mock prompt provenance/authorship remain distinct and immutable.
3. Prompt creation requires exact current Experience and at least one exact confirmed same-source Evidence revision.
4. Save response preserves question bytes, adds non-empty user response provenance, `mixed` revision authorship and `answers_prompt` linkage.
5. Response correction appends a new revision and corrected/superseded events; old content/provenance stays governed and no dependent is rebound.
6. Skip adds exactly one explicit review event, no response revision/provenance, and is terminal within this slice.
7. Stale source/prompt/Evidence, deleted/rejected/ineligible/cross-source dependency, duplicate/conflicting operation, malformed provenance/content, unsupported schema, or any correction inbound dependent fails closed unchanged.
8. v5 authority and v4 projection commit together under one guard; projection matches current authority.
9. Failure injection covers every meaningful write/reconciliation boundary; exact logical pre-state returns on definite failure.
10. Generic COMMIT errors are outcome-unknown unless definite non-commit is proven; read-only pre/post manifests classify without retry.
11. Receipt, schema digest, user_version 5 disposable fixture contract, foreign keys and integrity remain valid; production `SCHEMA_VERSION` stays 4.
12. Focused tests cover creation origins, answer, correction, skip, exact dependencies, invalid states, rollback, ambiguity, deterministic IDs/manifests and no Phase 4/Phase 3B writes.
13. Canonical verification, Clippy and Theory Alignment Review pass before Founder diff review.
14. No staging, commit, push, merge, PR, deployment or release occurs without separate authority.

## Evaluation Matrix

| Case | Expected result |
| --- | --- |
| AI/local-mock prompt with exact confirmed Evidence | prompt revision and projection commit |
| no Evidence, candidate/rejected/deleted/cross-source/stale Evidence | fail closed, no rows changed |
| answer exact suggested prompt | one mixed answered revision; exact prompt/user provenance |
| blank response or changed question | refuse unchanged |
| correct exact answered response with no inbound dependents | append correction and supersession |
| correction with normalized/ADR-0009 inbound dependent | refuse; no cascade or rebinding |
| explicit skip exact suggested prompt | one skipped event and skipped projection |
| duplicate skip, answer after skip, skip after answer | refuse unchanged |
| failure at each boundary | guard empty and exact logical pre-manifest |
| COMMIT ambiguity | exact pre/post classification or recovery_required; no retry |
| malformed/v4/newer/inconsistent database | refuse without mutation |
| production surface search | no caller/registration/app-data/UI/provider path |

## Risks

- Combined Reflection payloads can obscure authorship; mitigated by `mixed` revision authorship plus immutable prompt/response role provenance.
- Correcting an answered response can invalidate Pattern or historical dependents; this slice refuses any inbound-dependent correction rather than silently cascading or rebinding.
- Eligibility could be mistaken for consent; documentation and code must keep local eligibility separate from Phase 3B consent/transmission.
- Disposable transaction evidence does not prove real-user restart safety or production readiness.
- Skip projection changes current state without a new content revision; exact review event and immutable prompt revision are therefore mandatory.

## Open Questions

Founder decision `PHASE3C-SLICE4B2-001` Option A explicitly authorizes the exact four-operation private disposable contract. No constitutional change is proposed and every excluded production/runtime/later-slice boundary remains binding.

## Human Decision Required

false ? resolved by exact Founder Option A authorization on 2026-07-31.

## Recommendation

Implement only the Founder-authorized bounded four-operation private disposable slice above. Stop and re-escalate if repository evidence requires DDL, production activation, broader dependent invalidation, or any excluded capability.

## Review Status

`approved_with_conditions`: Founder Option A authorizes only the exact private disposable implementation and verification boundary; production/runtime/schema activation and every listed exclusion remain unauthorized.
