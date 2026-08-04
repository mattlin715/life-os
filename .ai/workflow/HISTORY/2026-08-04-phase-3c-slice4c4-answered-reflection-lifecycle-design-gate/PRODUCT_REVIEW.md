# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-04-phase-3c-slice4c4-answered-reflection-lifecycle-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `7c09dd7d008773e157e7da38de2263661d91307a`
- Working-tree digest reviewed: `47528284534e0229716dd493462583c91f7460ffc873611c6742db72bd00c019`
- Created at: 2026-08-04T11:52:36.308Z
- Updated at: 2026-08-04T11:52:36.308Z

## Mission Interpretation

This is a Founder-gated design review for the smallest complete answered
Reflection lifecycle extension. It must decide whether exact-current response
correction and explicit answered-Reflection deletion can safely apply every
already-governed ordinary Pattern consequence and ADR-0009 Historical Question
consequence in one private, unregistered, disposable-only transaction. It may
correct Slice 4C-3 promotion wording but may not implement or infer authority.

## Problem Statement

The promoted Reflection writer already appends an immutable mixed-authorship
response correction, preserves the immutable prompt lineage, and preserves
exact Experience/Evidence dependencies. It deliberately refuses correction
when an inbound Pattern or Historical Question dependency exists and has no
explicit answered-Reflection deletion command.

Changing only the Reflection head would be unsafe. A Pattern could remain
eligible while bound to a superseded/deleted response, and a Historical
Question could retain generated content, packet snapshot, and actual-use
provenance after its selected Reflection changed or was deleted.

## User Value

- The user's corrected response remains their saved words without rewriting
  the AI/local-mock prompt or inventing confirmation.
- Existing Pattern hypotheses remain inspectable but visibly invalidated rather
  than being regenerated or silently rebound.
- Historical Questions obey the stricter ADR-0009 deletion lifecycle when an
  exact selected Reflection changes or is deleted.
- Explicit deletion removes every retained prompt/response content payload and
  prevents later reuse while preserving only authorized content-free facts.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: Human before AI and **We Build Mirrors, Not
  Oracles** remain controlling.
- `docs/03_Principles.md`: Evidence before Conclusion, Reflection before Answer,
  user control, revisability, and visible uncertainty.
- `docs/06_Memory.md`: durable memory must remain provenance-preserving,
  correctable, rejectable, and deletable; meaning remains user-owned.
- `docs/Reflection.md`: Reflection is user interpretation supported by evidence,
  not an AI answer or identity conclusion.
- `docs/09_AI.md`, `docs/10_Privacy.md`, and `docs/appendix/Harness.md`: exact
  context, authorship, bounded use, consent separation, deletion, and
  fail-closed behavior remain inspectable.

## Relevant ADRs

- ADR-0007 requires durable reviewed artifacts, exact provenance, and distinct
  AI/user authorship.
- ADR-0009 requires exact-source revalidation and cascade deletion of a
  dependent Historical Question's generated content and successful packet
  snapshot after source correction or deletion; consent cannot be reused.
- ADR-0011 Decisions 9A, 10B, 11A, and 12A require retained superseded content,
  visible ordinary-dependent invalidation, ADR-0009 Historical Question
  deletion, and content-free artifact deletion tombstones.

## Current Implementation Context

### Implemented, verified, Founder-accepted, and promoted

- Production schema v4 typed mutation and ADR-0009 behavior remain active.
- Private disposable schema-v5 migration/restart evidence and current-state
  writers cover Experience, Evidence, Reflection, Pattern, Context Recovery,
  confirmed-Evidence lifecycle, Historical Question creation parity, and
  confirmed-Pattern lifecycle.
- Slice 4C-3 was promoted through feature commit
  `eb4dde6ea3b0b4edb53ad3cea58771c7ccc9b12b` and non-fast-forward merge
  `7c09dd7d008773e157e7da38de2263661d91307a`.

### Founder-approved design, not production-authorized

- Architecture/13 defines normalized schema-v5 lifecycle and cutover policy.
  Production `SCHEMA_VERSION`, startup maximum, and user databases remain v4.

### Not implemented

- Dependent-aware answered Reflection correction.
- Explicit answered Reflection deletion and all-revision content purge.
- Production schema-v5 migration/runtime activation and remaining lifecycle,
  export, retention, and UI gaps.

## Exact Reflection Dependency Inventory

1. Each answered/corrected Reflection revision has one
   `derived_from_experience` edge to the exact current active Experience
   revision.
2. It has one or more `uses_evidence` edges to exact current confirmed,
   eligible, same-source Evidence revisions. The represented durable ID array
   must be individually valid and unique before exact-set equality.
3. It has exactly one self-artifact `answers_prompt` edge to immutable revision
   1. Prompt bytes and role `prompt` provenance remain identical; the current
   revision has separate role `response` user provenance and `mixed`
   authorship.
4. A legal ordinary inbound edge is only a same-source Pattern revision using
   `uses_reflection_response` and the exact Reflection revision actually used.
   An active current Pattern is invalidated; an already invalidated Pattern may
   remain bound to an older retained revision only with its exact dependency
   invalidation fact and absent v4 projection.
5. A legal historical inbound edge is only a normalized
   `historical_packet_item` with exact schema-v4
   `historical_artifact_dependencies` parity for the same current Reflection
   snapshot. Its linked Historical Question is cascade-deleted under ADR-0009.
6. Context Recovery has only exact Experience and self `answers_prompt`
   dependencies and is categorically excluded from historical memory. It may
   not depend on Reflection.
7. No other ordinary inbound relationship is authorized. Pattern itself has no
   legal inbound consumer, so invalidating it creates no transitive Phase 4
   cascade. Malformed, cyclic, cross-source, stale, unsupported, contradictory,
   or incomplete dependency evidence fails closed before mutation.

## In Scope

Prepare decision `PHASE3C-SLICE4C4-001` with these Option A transaction
contracts if explicitly authorized:

### Correction state transition

`active answered eligible rN -> active answered eligible rN+1`

- Require exact current Experience and Reflection revisions, exact current
  confirmed Evidence set, immutable prompt lineage/provenance, exact current
  v4/v5 parity, and complete inbound closure.
- Append immutable `mixed` rN+1 with exact predecessor rN, unchanged prompt
  bytes/provenance, new user response provenance, and unchanged exact outgoing
  dependencies; append corrected/superseded facts.
- Keep rN content/provenance as superseded and ineligible through non-current
  status. Do not add a confirmation event or confirmation requirement.
- Invalidate every active exact Pattern dependent on rN, keep its content and
  provenance, make it ineligible, remove its v4 projection, and never rebind or
  regenerate it.
- Cascade-delete every exact Historical Question selecting rN, including
  generated content, packet snapshot, lifecycle link, actual-use provenance,
  and exact dependencies. Preserve unrelated unsuccessful audit metadata.
- Write the corrected Reflection v4 projection only after normalized authority
  and consequences reconcile.

### Deletion state transition

`active answered eligible rN -> deleted ineligible with no current revision`

- Require the same exact source, prompt, projection, and inbound-closure proof.
- Append explicit user deletion and content-purge facts; clear the head; remove
  the Reflection v4 projection; purge content rows for every retained
  Reflection revision, including all prompt and response bytes.
- Retain only authorized immutable content-free revision, provenance,
  dependency, review/lifecycle metadata and one minimal artifact tombstone with
  no reusable text. Do not reinterpret skip or invalidation as deletion.
- Apply the same exact Pattern invalidation and ADR-0009 cascade before final
  reconciliation. Suggested, skipped, invalidated, or already-deleted
  Reflection deletion remains outside this initial command and fails closed.

### Transaction and recovery model

Use one `BEGIN IMMEDIATE` transaction and the existing compatibility guard.
Collect and validate the complete closure before writing; then apply lifecycle
authority, exact consequences, guarded v4 projection changes, current-content
checks, source/target operation manifests, foreign keys, integrity, and guard
removal. Inject deterministic failure at every meaningful boundary. A generic
COMMIT error is outcome-unknown unless read-only durable evidence exactly
matches the pre-state or complete post-state; any third state returns
`recovery_required` without retry, replay, rollback, repair, cleanup, rebinding,
regeneration, or candidate selection.

### Exact prospective implementation allowlist

Only these product files may change after explicit Option A authorization:

1. `src-tauri/src/schema_v5_reflection_write.rs`
2. `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

The repository workflow may update its standard current artifacts and create
the standard archive under
`.ai/workflow/HISTORY/2026-08-04-phase-3c-slice4c4-answered-reflection-lifecycle-design-gate/`.
The implementation may call or faithfully apply promoted private verifier and
cascade contracts, but may not edit their modules without a new Founder gate.
No DDL, Cargo, production SQLite, Tauri registration, TypeScript, renderer, UI,
provider, ContextPacket, ADR, Book Zero, or Harness-contract path is allowed.

## Out Of Scope

Production schema/user_version 5; migration/fresh-v5 activation; real user
data/app-data; runtime/Tauri/renderer/UI/startup; prompt generation; Evidence or
historical eligibility changes; provider/ContextPacket/consent/retention
changes; Pattern regeneration or source rebinding; suggested/skipped Reflection
deletion; generic future-dependent infrastructure; production
backup/restore/recovery; export v2; Phase 4; identity/sensitive inference;
Harness expansion; staging, commit, push, merge, PR, deployment, or release.

## Product Constraints

The prompt remains AI/local-mock-authored and immutable. The saved/corrected
response remains user-authored; `mixed` describes a combined revision, not AI
ownership of the response. Correction is authorship, not Evidence confirmation,
Pattern confirmation, diagnosis, or identity truth. Silence and navigation do
nothing. No dependent is rebound.

## Evidence And Provenance Constraints

Reuse the promoted complete Experience, Evidence, Reflection, Pattern, and
Historical Question verification contracts rather than weaken them. Validate
identifier uniqueness before set equality. Preserve prior prompt/response
provenance on superseded revisions. Each invalidated Pattern fact names the
exact `uses_reflection_response` dependency that caused it. Historical
representation must match one-to-one across v4 and v5 before deletion.

## Historical Context Constraints

Only an active answered Reflection with non-empty user response and current
exact dependencies is eligible. Correction or deletion invalidates every old
packet use; no consent, preflight, packet, or generated artifact is reused.
The unsuccessful 30-day transmission audit boundary is independent and remains
unchanged unless it is actually linked to the deleted generated artifact under
existing ADR-0009 semantics.

## Consent Constraints

No provider call, consent event, transmission, retry, or policy change occurs.
Future historical use of the corrected response requires new selection,
preflight, exact packet, and per-generation/per-purpose consent.

## AI-Role Constraints

No text generation, semantic reinterpretation, Pattern regeneration,
recurrence claim, cross-time conclusion, diagnosis, advice, sensitive
inference, or identity finalization is added.

## Privacy Constraints

Option A touches synthetic/disposable fixtures only. Correction retains prior
content because ADR-0011 Decision 9A authorizes visible superseded history;
explicit artifact deletion purges all Reflection content. Content-free metadata
must not contain question/response text. Production and real user data remain
unreachable.

## User-Agency Constraints

Correction and deletion require explicit exact-current commands. Deletion is
distinct from skip and invalidation. Stale or contradictory evidence fails
closed; the system never chooses a replacement source, rebinds a dependent,
or infers a user's intent.

## Acceptance Criteria

| Case | Required result |
| --- | --- |
| Correction, no inbound dependent | Append exact mixed successor; prompt unchanged; new user response provenance; remains eligible |
| One or multiple direct Pattern dependents | Retain content/provenance; mark each exact current Pattern invalidated/ineligible; remove v4 projections |
| Historical Question selects old Reflection | Complete ADR-0009 generated-artifact, packet, actual-use, lifecycle-link, and dependency cascade |
| Pattern plus Historical Question dependents | Both consequence classes commit atomically with correction/deletion |
| Old Reflection revision | Superseded and non-current; no dependent rebound |
| Explicit answered deletion | Clear head/projection; purge every prompt/response content row; retain only authorized content-free facts/tombstone |
| Unrelated unsuccessful audit | Remains unchanged |
| Stale/duplicate/conflicting command | Fail closed with no partial write |
| Suggested/skipped/invalidated/deleted Reflection | Refuse unchanged |
| Experience/Evidence drift | Complete verifier chain refuses unchanged |
| Missing/duplicate/malformed/cross-source dependency | Refuse before set equality or mutation |
| Incomplete Pattern v4/v5 state | Refuse without invalidation or mutation |
| Incomplete Historical Question v4/v5 state | Refuse without cascade or mutation |
| Context Recovery/future inbound edge | Refuse as unsupported |
| Every meaningful failure boundary | Exact logical pre-state and empty guard after definite rollback |
| Ambiguous COMMIT exact pre/post | Classify definite non-commit or committed without replay |
| Ambiguous COMMIT third state | Return `recovery_required`; no autonomous action |
| Reconciliation | Exact v4/v5 parity, current-content rules, `foreign_key_check`, `integrity_check`, and guard emptiness pass |
| Scope | Production schema/startup remain v4; no provider, runtime, real-data, or Phase 4 behavior |

## Risks

- Exact closure is more complex than the current zero-inbound refusal; a missed
  edge could preserve stale personal interpretation. Mitigation: narrow
  relationship allowlist, all-revision scan, v4/v5 parity, and fail closed.
- Duplicate cascade implementations can drift. Mitigation: follow the promoted
  Evidence lifecycle, Pattern verifier, and Historical Question bridge as the
  executable contract without creating a generic future-dependent framework.
- Purging only the current revision would make deletion misleading. Option A
  requires all retained Reflection content rows to be removed.
- Retained invalidated Pattern content is intentionally different from deleted
  Reflection content; UI meaning must not be inferred from storage alone.
- Disposable transaction evidence cannot prove production restart recovery,
  real-user safety, or production schema-v5 readiness.

## Open Questions

None. The Founder resolved `PHASE3C-SLICE4C4-001` as Option A. Any file,
relationship, artifact state, or production/runtime behavior outside the exact
recorded authorization requires a new decision.

## Human Decision Required

False. `PHASE3C-SLICE4C4-001` was explicitly resolved as Option A; promotion
remains a later separate Founder gate.

## Recommendation

Implement only the Founder-authorized Option A boundary. The legal closure is
bounded: exact Reflection-to-Pattern ordinary invalidation plus exact
Reflection-to-Historical-Question ADR-0009 cascade, with no legal Context
Recovery or downstream Pattern consumer. Stop and re-escalate if implementation
requires a new file, relationship, policy, or runtime surface.

## Review Status

`approved_with_conditions`
