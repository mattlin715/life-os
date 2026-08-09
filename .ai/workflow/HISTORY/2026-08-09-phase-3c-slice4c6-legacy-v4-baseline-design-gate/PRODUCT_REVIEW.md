# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-09-phase-3c-slice4c6-legacy-v4-baseline-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `a9bbf7cc687f0d8dcc4bda9e1d402bf309fac590`
- Working-tree digest reviewed: `10678a4018ce9cb77707b2016928149c2c8f86aa5ac9ca033ec942bf88bbcf17`
- Created at: 2026-08-08T20:13:52.826Z
- Updated at: 2026-08-08T20:13:52.826Z

## Mission Interpretation

This is a bounded design and Founder authorization gate. It asks which actions
already exposed by the schema-v4 product must remain usable for honestly
migrated `legacy_v4_baseline` records. It does not authorize production schema
v5, add product actions, or require every private normalized lifecycle command
to support legacy records.

Current UI reachability is the parity boundary. A refusal is a production
cutover blocker only when an existing user action would encounter it after a
future migration. Passing private disposable tests remains contract evidence,
not runtime or real-user authority.

## Problem Statement

The promoted migration core preserves schema-v4 artifact bytes as
`legacy-v4-raw`, imports only review facts schema v4 actually retained, and
uses `legacy_unknown` when provenance is absent. The canonical private v5
writers do not all accept those baselines:

- Pattern confirmation and rejection explicitly return
  `pattern_legacy_review_requires_later_slice`.
- Reflection first answer, answered-response correction, and skip require
  `canonical-json-v1`.
- Context Recovery first answer and skip require `canonical-json-v1`.
- pending Evidence correction requires `canonical-json-v1`.
- Evidence confirmation and rejection already traverse a verifier that accepts
  `legacy-v4-raw`, but no focused migrated-candidate review suite proves the
  complete state, purge, projection, rollback, and reconciliation contract.

Proceeding to cutover with these gaps would leave current buttons unusable.
Making all normalized lifecycle commands legacy-aware in one slice would be a
large, unnecessary state matrix and would elevate absent UI actions into false
blockers.

## User Value

A user who upgrades must not lose ordinary actions merely because Life OS
honestly preserved old bytes. At the same time, upgrade compatibility must not
rewrite history, fabricate provenance, infer dependencies, or pretend the
migration timestamp was the original decision time. New explicit decisions
should be recorded at their real action time; retained baseline bytes should
remain exact until an authorized purge action removes them.

## Relevant Primary Definitions

- `docs/02_Philosophy.md`: context precedes interpretation and meaning remains
  user-owned.
- `docs/03_Principles.md`: evidence stays distinguishable from conclusions;
  agency and reversibility are product constraints.
- `docs/06_Memory.md`: durable memory requires provenance, correction,
  deletion, consent, and no silent identity accumulation.
- `docs/Reflection.md`: Reflection begins with evidence, remains uncertain, and
  cannot be automated into a conclusion.
- `docs/09_AI.md`: AI artifacts remain revisable hypotheses; confirmation is a
  user review decision, not objective truth.
- `docs/10_Privacy.md`: local ownership, transparent retention, ongoing consent,
  and deletion prohibit silent normalization or hidden preservation.
- `docs/appendix/Harness.md`: output and memory eligibility remain governed by
  exact type, source, review state, and provenance.

## Relevant ADRs

- ADR-0007 requires reviewed AI and user-authored Reflection artifacts to keep
  explicit provenance and review distinctions.
- ADR-0009 requires exact source revalidation and cascade deletion of dependent
  Historical Questions and successful packet snapshots; packet bytes and
  digests are not general revision storage.
- ADR-0011 Decisions 5B, 6B, 7B, 8B, 10B, 11A, and 12A require exact-revision
  review, append-only correction, no confirmation carry-forward, rejected
  content purge, ordinary invalidation without rebinding, ADR-0009 cascade, and
  content-free deletion facts.

## Current Implementation Context

### Repository and promotion evidence

- `develop`, `origin/develop`, and starting HEAD were
  `a9bbf7cc687f0d8dcc4bda9e1d402bf309fac590`, with ahead/behind `0/0` and a
  clean tree before this sprint.
- The merge has parents `1428f610c161eb89b57d4c9d44da6fd99be7682b`
  and `f62050a6b96814376bc35f4924b996537e259bc1`.
- Slice 4C-5 was verified, Founder-accepted, and promoted. The minimal factual
  correction is now in architecture/13 version 4.2.
- Baseline canonical verification passed 17 workflow tests, 26 Vitest files / 204
  tests, 162 Rust library tests, 12 backup/restore integration tests, 8 schema
  contract tests, typecheck, production build, Rust check, hygiene checks, and
  the no-Constitution-diff check.
- Production `SCHEMA_VERSION` and startup maximum remain 4.

### Migration facts

`schema_v5_migration.rs` and the fixed DDL create one honest baseline revision:

- `revision_reason = legacy_v4_baseline`;
- `serialization_version = legacy-v4-raw`;
- exact original bytes and their SHA-256 digest;
- `legacy_unknown` authorship/provenance when v4 lacks evidence;
- `legacy_import` review facts only for statuses v4 actually retained;
- `record_updated_at_not_decision_time` rather than an invented click time;
- no reconstructed rejection history or missing dependency.

### Current UI reachability evidence

- Experience edit/delete call typed storage operations in `src/app/App.tsx`.
- Evidence candidate edit, confirm, and reject are visible only while the item
  is unreviewed/candidate.
- Reflection suggested prompts can be answered or skipped; answered responses
  remain editable and savable; answered deletion is not exposed.
- Pattern candidates can be confirmed or rejected; confirmed correction and
  deletion are not exposed.
- Context Recovery suggested turns can be answered or skipped; answered turns
  are disabled and no standalone correction/deletion is exposed.
- Historical Question generated artifacts have explicit deletion; source edits,
  rejection, eligibility loss, or deletion use ADR-0009 cascade behavior.

## Migrated-State Current-Action Matrix

| Artifact and migrated state | Current product action | Reachable now | Current private-v5 evidence | Legal result | Cutover class |
| --- | --- | --- | --- | --- | --- |
| Experience baseline | correction | yes | Slice 4C-5 accepts exact source correction and ordinary consequences | append exact new user source revision; preserve baseline; invalidate ordinary dependents; ADR-0009 cascade | covered |
| Experience baseline | deletion | yes | promoted parent-deletion regression | purge source-scoped content and approved tombstones; ADR-0009 cascade | covered |
| Evidence pending/candidate | correction | yes | explicit canonical-only fence | append canonical user successor; preserve raw predecessor; pending/ineligible; later confirmation required | **4C-6B blocker** |
| Evidence pending/candidate | confirmation | yes | verifier appears legacy-compatible but lacks focused migrated proof | preserve raw bytes/digest; append exact user confirmation; confirmed/eligible | **4C-6A blocker** |
| Evidence pending/candidate | rejection | yes | verifier appears legacy-compatible but lacks focused migrated proof | purge raw content; remove v4 projection; append exact rejection and content-free history | **4C-6A blocker** |
| Evidence confirmed/imported | correction | no | private lifecycle writer supports legacy baseline | canonical user successor, pending/ineligible, imported confirmation unchanged | safely deferrable |
| Evidence confirmed/imported | deletion | no | private lifecycle writer supports legacy baseline | explicit purge/tombstone and dependent consequences | safely deferrable |
| Reflection suggested | answer | yes | canonical-only fence | preserve raw prompt baseline; append canonical mixed prompt/user-response successor and exact lineage | **4C-6B blocker** |
| Reflection suggested | skip | yes | canonical-only fence | preserve raw content; append exact user skip review/head state only; no rewrite | **4C-6B blocker** |
| Reflection answered/imported | correction | yes | canonical-only fence | preserve raw predecessor and prompt provenance; append canonical mixed successor with new user-response provenance | **4C-6B blocker** |
| Reflection answered | deletion | no | private canonical lifecycle action only | not required for current-action parity | safely deferrable |
| Pattern candidate | confirmation | yes | explicit legacy-review fence | preserve raw bytes/digest/provenance; append exact user confirmation; useful-for-reflection but still a hypothesis | **4C-6A blocker** |
| Pattern candidate | rejection | yes | explicit legacy-review fence | purge raw content; remove v4 projection; append exact rejection and content-free history | **4C-6A blocker** |
| Pattern confirmed/imported | correction | no | private lifecycle writer supports legacy baseline | canonical user successor, pending/ineligible, exact source set unchanged | safely deferrable |
| Pattern confirmed/imported | deletion | no | private lifecycle writer supports legacy baseline | explicit purge/tombstone | safely deferrable |
| Context Recovery suggested | answer | yes | canonical-only fence | preserve raw prompt; append canonical prompt/user-response successor; remain current-task-only and historically excluded | **4C-6B blocker** |
| Context Recovery suggested | skip | yes | canonical-only fence | preserve raw content; append explicit user skip without historical eligibility | **4C-6B blocker** |
| Context Recovery answered/skipped | correction/deletion | no | not implemented as current product actions | do not manufacture a migration blocker or add UI | safely deferrable |
| Historical Question | explicit generated-artifact deletion | yes | promoted exact v4/v5 parity and deletion | delete generated content, packet snapshot, actual-use chain, links, and dependencies | covered |
| Historical Question | source-driven deletion | yes | promoted Experience/Evidence/Reflection consequences | exact parity then ADR-0009 cascade; unrelated failed audit metadata unchanged | covered |

## Parsing And Preservation Contract

1. **Parsing is read-only validation.** A legacy payload may be decoded only to
   prove it is the exact supported schema-v4 object for its artifact kind.
2. **The baseline remains opaque authority.** Its original UTF-8 bytes,
   `legacy-v4-raw` marker, digest, predecessor identity, imported provenance,
   and imported review event remain unchanged for every non-purge action.
3. **Strict shape.** The parser must prove an object with the required v4 fields,
   allowed optional fields, well-formed unique IDs, exact artifact/source IDs,
   legal status, exact dependency set, and exact v4 projection byte equality.
   Unknown fields, unsupported old shapes, malformed JSON, duplicate source IDs,
   or contradictory status fail closed.
4. **Unknown is an honest value.** A governed `legacy_unknown` provenance record
   is preserved as unknown; it is not rejected merely for being unknown and is
   never upgraded to AI, local-mock, or user provenance. Missing or contradictory
   provenance that is not represented by the migration contract fails closed.
5. **Successors are new facts.** A later authorized correction/answer may create
   a `canonical-json-v1` successor with exact new authorship/provenance and an
   exact predecessor relation. It cannot recanonicalize or replace the baseline.
6. **Purge is explicit.** Rejection/deletion may remove raw content only under
   the accepted artifact-specific policy while retaining only authorized
   content-free facts. No rejected text is reconstructed.
7. **No repair.** Digest mismatch, raw/projection mismatch, missing/orphaned or
   cross-source dependencies, stale revisions, duplicate IDs, incomplete guard
   evidence, or unsupported inbound relationships return unchanged fail-closed
   outcomes or `recovery_required`; no guessing, rebinding, or silent repair.

## Exact Review And Successor Semantics

### Slice 4C-6A review actions

- Confirming legacy Evidence or Pattern changes review/head eligibility only.
  It does not rewrite content, serialization, provenance, or dependency edges.
- The new event names the exact baseline revision, actor `user`, explicit action,
  and actual injected action time. It does not replace a prior `legacy_import`
  event. Duplicate or stale review attempts fail unchanged.
- Evidence confirmation makes that exact revision eligible under existing
  Evidence rules. Pattern confirmation records useful-for-reflection only; the
  content remains a revisable hypothesis, never fact or identity.
- Rejection is an explicit purge action. It removes all retained content for the
  rejected artifact, removes the guarded v4 projection, records exact review and
  lifecycle facts, and retains only ADR-0011-authorized content-free metadata.
- Rejection applies complete exact dependent consequences and ADR-0009 deletion
  where legally reachable. Candidate Evidence/Pattern must otherwise have no
  unsupported inbound state; malformed inbound evidence fails closed.

### Deferred Slice 4C-6B successor actions

- Evidence candidate correction creates a user-authored canonical successor,
  leaves it pending/ineligible, and requires new confirmation.
- Reflection answer/correction creates a new exact user-response provenance fact
  while preserving immutable prompt bytes/provenance and exact answers-prompt,
  Experience, and Evidence edges. Saving a response is authorship, not prompt
  confirmation.
- Reflection skip is an exact user review action without content rewrite.
- Context Recovery answer/skip keeps supporting conversation categorically
  excluded from Phase 3B retrieval, consent, transmission, durable longitudinal
  memory, and Phase 4 inference.
- Every successor retains the untouched raw predecessor and exact predecessor
  relationship; dependencies are validated, never inferred or rebound.

## Evaluation Matrix

| Required case | 4C-6A responsibility | Later parity responsibility |
| --- | --- | --- |
| byte-identical legacy predecessor and exact digest after non-purge | Evidence/Pattern confirm | repeat for every 4C-6B non-purge action |
| legacy candidate Evidence confirmation | exact new review time; baseline unchanged; eligible | none |
| legacy candidate Evidence rejection | purge, projection removal, content-free facts | none |
| legacy confirmed Evidence correction requiring reconfirmation | regression that imported event remains unchanged | already private-supported; UI absent, not cutover blocker |
| legacy candidate Pattern confirmation | exact review; remains hypothesis | none |
| legacy candidate Pattern rejection | purge, projection removal, content-free facts | none |
| legacy confirmed Pattern correction | regression only | already private-supported; UI absent |
| migrated suggested Reflection answer/skip | preserve refusal during 4C-6A | **4C-6B required** |
| migrated answered Reflection correction | preserve refusal during 4C-6A | **4C-6B required** |
| migrated suggested Context Recovery answer/skip | preserve refusal during 4C-6A | **4C-6B required** |
| imported review event | byte-identical and unchanged | unchanged across successors |
| `legacy_unknown` provenance | stays visibly unknown | stays unknown on predecessor; successor gets honest new provenance |
| no content rewrite during confirmation/skip | Evidence/Pattern confirmation | Reflection/Recovery skip |
| stale/duplicate action | exact unchanged refusal | exact unchanged refusal |
| malformed JSON/unknown field/unsupported shape | fail unchanged | fail unchanged |
| missing/duplicate/orphaned/cross-source dependency | fail unchanged | fail unchanged |
| raw/projection or digest mismatch | `recovery_required`, no mutation | same |
| Experience revision drift | fail unchanged | fail unchanged |
| dependent Pattern invalidation | Evidence consequences where legal | Reflection correction consequences |
| Historical Question ADR-0009 cascade | Evidence rejection where applicable | Evidence/Reflection source change where applicable |
| rollback at every write boundary | deterministic injection | deterministic injection |
| ambiguous COMMIT exact pre/exact post/third state | accept exact pre or post only; third is `recovery_required` | same |
| guard emptiness, `foreign_key_check`, `integrity_check` | required | required |
| production schema/provider/consent/Phase 4 | prove unchanged | prove unchanged |

## Slice Alternatives

### Option A — All reachable legacy actions in one slice

Touches Evidence, Pattern, Reflection, Context Recovery, their cross-artifact
consequences, and a much larger failure matrix. It reduces naming overhead but
couples review-only purge risk with successor/provenance construction. Reject:
too large for one bounded review cycle.

### Option B — Evidence and Pattern review actions first

Implement/prove only exact legacy candidate confirmation and rejection. This
isolates review/head-only and purge semantics, directly removes the explicit
Pattern fence, and adds missing migrated Evidence proof. Defer every
successor-producing current action to one final Slice 4C-6B. **Recommended.**

### Option C — Successor-producing actions first

Would enable edits and prompt responses while current Pattern confirm/reject
buttons still fail. Reject: it leaves the simplest explicit current-action
review blocker in place and mixes more provenance/dependency states first.

### Option D — Keep migrated artifacts read-only

Would require visible refusal behavior and intentionally disable buttons users
already have. Reject: current UI reachability proves this is not parity.

### Option E — Defer production v5 until the complete matrix exists

Accept as the cutover rule, not as a substitute for a next slice. Production
migration/recovery authorization must remain blocked until both 4C-6A and the
final successor slice 4C-6B are promoted and the matrix is re-audited.

## Recommended Minimum Slice And Allowlist

Recommend **Option B: Slice 4C-6A**. Exactly two implementation slices remain:

1. 4C-6A — migrated Evidence/Pattern candidate confirm/reject.
2. 4C-6B — migrated Evidence candidate edit, Reflection answer/skip/correction,
   and Context Recovery answer/skip.

Proposed product/document implementation allowlist for 4C-6A:

- `src-tauri/src/schema_v5_evidence_write.rs`
- `src-tauri/src/schema_v5_pattern_write.rs`
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`

Repository-native workflow state/artifacts and the eventual archive for this
same sprint may also change only as required by `.ai/workflow/WORKFLOW.md`.
No DDL, migration core, production SQLite, Tauri registration, renderer,
storage adapter, UI, provider, ContextPacket, or Book Zero file is allowed.
If implementation evidence requires any fourth product/document path, stop for
a revised Founder decision rather than expanding scope.

## Updated Phase 3C Roadmap And Exit Pressure

1. Promote no design automatically; obtain explicit Founder authority for
   4C-6A.
2. Implement, verify, Founder-review, and separately promote 4C-6A.
3. Return for one exact 4C-6B Founder gate; implement only the remaining
   currently reachable successor actions.
4. Lifecycle-parity work ends when every current UI action in the matrix is
   either covered by promoted disposable evidence or explicitly prohibited by
   existing product policy, while all currently absent actions remain deferred.
5. Only then run a bounded production migration/recovery Founder Gate covering
   real startup integration, verified backup/restart behavior, disclosure,
   command registration, real-user safeguards, manual matrix, and rollback
   authority. Neither 4C-6 slice authorizes that gate's implementation.
6. Export v2, lifecycle UI, prior-revision purge controls, absent Context
   Recovery actions, and Phase 4 remain separately value-gated work; they do not
   prolong current-action parity.

## In Scope

- Current-action matrix, strict legacy contract, 4C-5 factual synchronization,
  alternatives, 4C-6A recommendation, evaluation plan, allowlist, exit
  condition, and Founder decision package.

## Out Of Scope

- Any legacy writer implementation before approval; production schema v5;
  real data; migration/startup/runtime/UI changes; new actions; DDL; generic
  legacy mutation framework; automatic repair; provider, ContextPacket,
  consent, retention, export v2, Phase 4, Harness changes, Git promotion,
  deployment, or release.

## Product Constraints

- Preserve Mirrors, Not Oracles; exact evidence/review distinctions; Pattern as
  hypothesis; Reflection response as user authorship; Context Recovery as
  supporting conversation; no identity or sensitive inference.

## Evidence And Provenance Constraints

- Baseline raw bytes, digest, serialization, imported review fact, uncertain
  timestamp quality, authorship, provenance, and exact dependencies remain
  immutable unless an explicit accepted purge action removes content.
- New actions create new exact facts and never retroactively improve imported
  provenance.

## Historical Context Constraints

- Context Recovery stays excluded. Evidence/Reflection eligibility changes use
  exact artifact-specific ADR-0009 cascade after v4/v5 parity. Packet bytes and
  digests remain unchanged and are never rehydrated as revision history.

## Consent Constraints

- No consent is created, reused, widened, or reinterpreted. Existing consumed
  consent/transmission facts follow only the already-accepted generated-artifact
  deletion lifecycle.

## AI-Role Constraints

- Confirmation records the user's review of usefulness; it does not make AI
  content true. No provider call, semantic rewrite, regeneration, inference,
  diagnosis, recurrence claim, or identity finalization occurs.

## Privacy Constraints

- Rejection/deletion purge only under explicit accepted actions. Non-purge
  actions cannot silently rewrite raw content. Unknown provenance remains
  visible rather than guessed.

## User-Agency Constraints

- Edit, confirm, reject, answer, skip, and delete remain distinct actions.
  Migration itself performs none of them. A disabled future action may fail
  closed, but a currently reachable action cannot be silently removed at
  cutover without a separate product decision.

## Acceptance Criteria For An Authorized 4C-6A

1. Exact-v5 fixtures are produced only through the promoted exact-v4 migration.
2. Legacy candidate Evidence and Pattern confirm without changing raw bytes,
   digest, serialization, provenance, dependencies, or imported facts.
3. Confirmation appends one explicit-user event at the injected action time;
   Evidence becomes eligible and Pattern remains a user-reviewed hypothesis.
4. Legacy candidate rejection purges all artifact content, removes guarded v4
   projection, and retains only accepted content-free review/lifecycle facts.
5. Exact legacy object shape, unique IDs, source identity, source revision,
   dependencies, projection parity, provenance representation, and digest are
   revalidated immediately before mutation and read-only after transaction.
6. Malformed/unknown/unsupported payloads, stale or duplicate actions, missing
   or duplicate IDs, orphaned/cross-source edges, mismatched projection/digest,
   unsupported inbound state, or incomplete guard evidence fail closed.
7. Imported `legacy_import` events and `legacy_unknown` provenance are preserved
   exactly; no review time, actor, source, or provider is invented.
8. Relevant Pattern invalidation and Historical Question ADR-0009 cascade are
   exact where legally reachable; unrelated audit metadata remains unchanged.
9. Failure injection proves logical rollback at every new write boundary.
10. Ambiguous COMMIT accepts only exact pre-state or post-state; a third durable
    state returns `recovery_required` without retry, repair, or cleanup.
11. Focused tests, Clippy with warnings denied, canonical verification, Product
    and Theory Alignment Review, archive/reset, and Founder diff review pass.
12. Production `SCHEMA_VERSION`, startup maximum, DDL, user data, UI, provider,
    consent, retention, and Phase 4 remain unchanged.

## Risks

- Legacy raw objects require strict artifact-specific validation; a permissive
  generic parser could bless malformed history.
- Rejection is destructive for artifact content and needs stronger rollback and
  exact ownership proof than confirmation.
- Evidence currently looks compatible, but treating code inspection as proof
  would leave untested migrated behavior in the cutover path.
- Over-generalizing legacy mutations would create a second lifecycle framework.
- Completing 4C-6A alone could be mislabeled as cutover readiness; the successor
  slice remains a hard blocker.

## Open Questions

None inside Slice 4C-6A. Slice 4C-6B and every production migration/recovery
decision remain separate future Founder gates.

## Human Decision Required

No. `PHASE3C-SLICE4C6-001` was resolved as Option B with the exact bounded
Slice 4C-6A scope recorded in `DECISION_REQUIRED.md`.

## Recommendation

Implement only Founder-authorized **Option B / Slice 4C-6A** under the
three-path product/document allowlist. Do not infer Slice 4C-6B or production
cutover authority. After a separately reviewed promotion, return once for the
final successor-action slice.

## Review Status

approved_with_conditions
