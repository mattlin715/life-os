# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-09-phase-3c-slice4c6b-legacy-successor-parity
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `6f2c64c13a6085e73cc4f4c5dccf6d51a73e543d`
- Working-tree digest reviewed: `6918f007203286c6d37e4848622eb1430df6fd23`
- Created at: 2026-08-09T19:24:05+09:00
- Updated at: 2026-08-09T19:24:05+09:00

## Mission Interpretation

Complete only the final currently reachable successor-producing actions for migrated `legacy-v4-raw` artifacts. This is private disposable contract evidence, not production schema-v5 activation. The user action creates a new fact; migration does not rewrite history or silently perform the action.

## Problem Statement

The promoted migration core preserves exact schema-v4 bytes, but the private Evidence, Reflection, and Context Recovery writers currently require `canonical-json-v1` for the authorized actions. A future cutover would therefore break buttons already reachable in the schema-v4 product unless the legacy predecessor and canonical successor contract is proved first.

## User Value

An upgrade must preserve edit, answer, and skip actions without fabricating provenance or replacing old records. Users retain the ability to correct and respond, while exact old content and unknown provenance remain honest and inspectable.

## Relevant Primary Definitions

- `docs/03_Principles.md`: Evidence before conclusion; agency and reversibility.
- `docs/06_Memory.md`: provenance, correction, deletion, and user control.
- `docs/Reflection.md`: Reflection is user-owned and grounded in confirmed Evidence.
- `docs/09_AI.md`: AI prompts remain distinct from user responses and are never finalized identity.
- `docs/10_Privacy.md`: local control and no silent rewriting or retention expansion.
- `docs/appendix/Harness.md`: exact source, authorship, review state, and eligibility remain governed.

## Relevant ADRs

- ADR-0007 requires durable reviewed artifacts with explicit provenance.
- ADR-0009 requires exact Historical Question dependencies and cascade behavior; Context Recovery remains ineligible.
- ADR-0011 requires append-only correction, exact revision dependencies, content/authorship separation, and fail-closed lifecycle behavior.

## Current Implementation Context

- Slice 4C-6A is promoted at feature `5c8c8755` and no-ff merge `6f2c64c`; architecture/13 still has pre-promotion wording requiring factual correction.
- Production `SCHEMA_VERSION` and startup maximum are 4.
- `src/app/App.tsx` exposes pending Evidence edit only for candidate state; suggested Reflection answer/skip; answered Reflection response editing and save; suggested Context Recovery answer/skip. Recovery answered state is disabled.
- Existing private writers already provide one transaction, guarded v4 projection, deterministic failure injection, read-only reconciliation, and conservative COMMIT classification for canonical revisions.
- Migration stores the legacy predecessor as `legacy-v4-raw`, preserves exact bytes/digest, records `legacy_import` facts, and uses `legacy_unknown` when provenance is unknowable.

## Current-Action Reachability Matrix

| Migrated state | Current action | Reachable | Slice result |
| --- | --- | --- | --- |
| Evidence pending/candidate | user correction | yes | append canonical user successor; pending/ineligible; exact reconfirmation required |
| Reflection suggested | answer | yes | append canonical mixed successor with immutable legacy prompt lineage and new user response provenance |
| Reflection suggested | skip | yes | preserve legacy revision/content; append exact skip review/head state; no response content |
| Reflection answered | response correction | yes | append canonical mixed successor with unchanged legacy prompt lineage and new response provenance |
| Context Recovery suggested | answer | yes | append canonical mixed successor; current-task-only and historically ineligible |
| Context Recovery suggested | skip | yes | preserve legacy content; terminal skip; no response revision/content |
| Context Recovery answered/skipped | correction/deletion | no | intentionally omitted |
| Reflection skipped | response/delete | no | intentionally omitted |

## In Scope

The six reachable actions above, artifact-specific strict legacy parsing, exact predecessor preservation, canonical successors where content changes, guarded v4 parity, deterministic tests, architecture/13 factual synchronization, Product/Theory review, and workflow archive.

## Out Of Scope

Production schema v5, DDL, real data, runtime/Tauri/UI/startup registration, new actions, generic lifecycle infrastructure, provider/ContextPacket/consent change, backup/restore activation, export v2, Phase 4, Git promotion, deployment, and release.

## Product Constraints

- Never rewrite or recanonicalize the legacy predecessor.
- Do not add actions absent from current product behavior.
- Reflection and Context Recovery preserve prompt/user-response authorship separation.
- Context Recovery stays task-scoped and historically ineligible.

## Evidence And Provenance Constraints

Raw predecessor bytes, digest, revision identity, serialization, imported lifecycle/review facts, provenance representation, and exact dependencies remain unchanged. A new response/correction records new user provenance at the injected action time. `legacy_unknown` remains unknown.

## Historical Context Constraints

Confirmed Evidence dependencies must be exact and current before a Reflection successor. Context Recovery cannot become a Phase 3B source. Existing ADR-0009 behavior is unchanged.

## Consent Constraints

No consent event, scope, packet, transmission, or policy changes.

## AI-Role Constraints

No provider call, generation, semantic inference, diagnosis, Pattern conclusion, or identity inference. Prompt content stays attributed to its original AI/local-mock/legacy-unknown provenance; user responses remain user-authored.

## Privacy Constraints

No real data or app-data path. Skip creates no response content. Malformed or contradictory history fails closed without repair, regeneration, retry, or rebinding.

## User-Agency Constraints

Only explicit user edit/answer/skip actions create new facts. Unsaved drafts remain non-durable. Reconfirmation remains required after Evidence correction.

## Acceptance Criteria

1. Every fixture begins as exact schema v4 and reaches v5 only through the promoted private migration core.
2. The six reachable actions follow the matrix; omitted actions remain absent.
3. Predecessor bytes/digest/provenance/dependencies/imported facts remain exact.
4. Content-changing actions append deterministic canonical successors with honest authorship/provenance.
5. Skip changes review/head/projection only and creates no response content.
6. Exact current Experience and confirmed Evidence revisions are revalidated; no rebinding occurs.
7. Unknown fields, malformed JSON, stale/deleted/rejected/orphaned/cross-source/duplicate/contradictory state fail closed.
8. Every meaningful write boundary rolls back logically to the exact pre-state.
9. Ambiguous COMMIT accepts only exact pre or post state; third state is `recovery_required` without automation.
10. Production `SCHEMA_VERSION` remains 4 and no runtime surface is registered.
11. Focused tests, Clippy, canonical verification, Product/Theory review, archive/reset, and Founder diff review complete.

## Anticipated Final File Allowlist

- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_reflection_write.rs`
- `src-tauri/src/schema_v5_context_recovery_write.rs`
- the 11 repository-required archive artifacts under `.ai/workflow/HISTORY/2026-08-09-phase-3c-slice4c6b-legacy-successor-parity/`

No additional product/document path is anticipated. Stop for Founder direction if one becomes necessary.

## Risks

- A permissive parser could bless malformed legacy history.
- Reusing canonical helpers without legacy-specific proof could silently normalize the predecessor.
- Response correction has dependent Pattern/Historical Question consequences; exact promoted verifier chains must remain authoritative.
- A passing disposable suite could be mislabeled as production readiness.

## Open Questions

None. Current UI inspection proves answered Reflection correction is reachable and standalone Context Recovery correction/deletion is not.

## Human Decision Required

False. The Founder explicitly authorized Slice 4C-6B with this exact reachability boundary. Any required expansion must transition to `human_decision_required`.

## Recommendation

Proceed under the anticipated allowlist and reuse the existing artifact-specific writers. Do not build shared generic legacy mutation infrastructure.

## Review Status

approved_with_conditions
