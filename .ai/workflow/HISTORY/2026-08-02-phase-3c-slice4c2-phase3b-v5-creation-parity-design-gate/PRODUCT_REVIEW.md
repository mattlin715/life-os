# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-08-02-phase-3c-slice4c2-phase3b-v5-creation-parity-design-gate
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: `666518eb53ba1cebf64ce87e4add1d2467c8dcbb`
- Working-tree digest reviewed: `4e961904bd049b2f7f54ceaf663fe3be7cc298f7bc912cb880949bb5a2060e1c`
- Created at: 2026-08-01T20:57:02.587Z
- Updated at: 2026-08-01T21:18:00.000Z

## Mission Interpretation

Close one schema-v5 parity gap without adding a product capability: prove, on
exact disposable schema-v5 fixtures, that an already-authorized successful
Phase 3B Historical Reflection Question can be represented atomically in both
the existing ADR-0009 schema-v4 subsystem and the normalized schema-v5
lifecycle model. This review requests Founder authority before implementation.

## Problem Statement

Phase 3B production behavior currently persists consent, transmission, the
immutable packet snapshot, the generated Historical Question, and exact source
dependencies in schema v4. The promoted migration core can normalize existing
rows and Slice 4C-1 can remove normalized questions during source lifecycle
changes, but no private schema-v5 writer can yet create a new question in both
representations. Production schema-v5 activation would therefore be unsafe.

## User Value

The bounded work protects future user trust: a historical question remains
traceable to the exact packet the user saw and consented to, its exact sources,
and the actual successful provider use. It does not create a new visible
feature, provider call, consent action, or AI interpretation.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: the AI remains a mirror, never an authority.
- `docs/06_Memory.md`: historical use requires provenance, correction,
  deletion, and user control; more context increases responsibility.
- `docs/Reflection.md`: questions invite reflection and do not finalize meaning.
- `docs/09_AI.md`: uncertainty and source boundaries remain visible; no Phase 4
  conclusion, diagnosis, identity finalization, or sensitive inference.
- `docs/10_Privacy.md`: local persistence and provider use are distinct.
- `docs/appendix/Harness.md`: exact inputs, contracts, provenance, refusal, and
  provider-independent behavior govern the capability.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`:
  persisted AI artifacts retain authorship and provenance.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`:
  per-generation/per-purpose consent, immutable packet identity, successful
  actual-use provenance, exact source eligibility, revalidation, deletion, and
  the Historical Reflection Question output boundary are unchanged.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`:
  normalized append-only revision/provenance/dependency structures and guarded
  compatibility projection are Founder-approved design, not production-v5
  authority.

## Current Implementation Context

**Implemented, verified, and promoted:** Phase 3B schema-v4 consent,
transmission, packet snapshot, Historical Question persistence, exact
dependencies, deletion cascades, and the read-only P1 provenance inspector;
schema-v5 migration/backfill evidence; disposable Experience, Evidence,
Reflection, Pattern, Context Recovery writers; and Slice 4C-1 confirmed-Evidence
lifecycle cascades. Slice 4C-1 feature commit
`893a3f8e5f4123685d9934bea8bfd8fbe9556165` was merged non-fast-forward by
`666518eb53ba1cebf64ce87e4add1d2467c8dcbb`.

**Founder-approved design:** architecture/13 describes schema-v5 lifecycle
tables and the requirement that new Phase 3B writes preserve the authoritative
ADR-0009 rows while creating an exact normalized projection.

**Proposed only:** the Slice 4C-2 disposable creation-parity writer below.

**Not authorized:** production schema/user_version 5, real-user migration,
runtime activation, provider or ContextPacket changes, consent changes,
retention changes, Phase 4, deployment, or release. Production
`SCHEMA_VERSION` and startup maximum remain 4.

### Exact v4-to-v5 mapping

| Existing ADR-0009 record | Normalized v5 representation | Authority rule |
| --- | --- | --- |
| `historical_consent_events` | Referenced by the successful Historical Question revision provenance; not duplicated as a generic lifecycle event | Existing immutable consent row remains authoritative for consent meaning and scope |
| `historical_transmission_events` | Referenced by the same provenance and lifecycle link; unsuccessful attempts remain independent audit metadata | Existing transmission row remains authoritative for outcome/provider/model |
| `historical_question_artifacts.packet_snapshot` and `packet_digest` | `historical_question_lifecycle_links` links the normalized artifact to the v4 row; packet identity is retained by reference | Exact packet bytes and ADR-0009 digest remain authoritative and are never reconstructed or recanonicalized |
| `historical_question_artifacts.payload` | One `historical_question` head, immutable created revision, purgeable revision content, and AI provenance fingerprint | Generated question content is normalized; v4 payload remains the guarded compatibility projection |
| `historical_artifact_dependencies` | Exact `historical_current_experience` and `historical_packet_item` revision dependencies | Both representations must have per-source one-to-one parity before union, cascade, or commit |
| Question deletion through ADR-0009 source invalidation | Deletion of linked normalized head/revisions/content/provenance links/dependencies through the existing guarded bridge | Slice 4C-1 remains the authoritative consequence path; no second cascade policy |
| Failed/cancelled/no-question attempts | No generated artifact or actual-use provenance | Existing unsuccessful audit metadata remains governed by its unchanged 30-day retention boundary |

### Proposed transaction and reconciliation boundary

One private, unregistered, path/connection-injected Rust boundary would operate
only on disposable exact-v5 fixtures. One `BEGIN IMMEDIATE` transaction would:

1. prove exact schema-v5 contract/receipt, empty compatibility guard, and
   disabled lifecycle/export activation;
2. validate well-formed unique identifiers before any set comparison;
3. prove a consumed, unexpired, per-generation/per-purpose consent and one
   successful transmission have the exact packet digest, provider, model,
   purpose, task, and references supplied for persistence;
4. retain the already-persisted packet snapshot byte-for-byte and prove its
   digest and exact source-revision set; never rebuild it after consent;
5. reuse promoted Experience, Evidence, and complete Reflection verifier chains
   to prove exact current active/confirmed/answered eligibility and same-source
   dependencies; Pattern, Context Recovery, and all unsupported kinds fail
   closed;
6. validate the neutral question output and require at least one historical
   citation, with exact citation/source-set agreement;
7. create the schema-v4 Historical Question row and exact dependencies plus one
   normalized v5 head/revision/content/provenance fingerprint/dependency set and
   lifecycle link under the existing compatibility guard;
8. reconcile exact v4/v5 identifiers, packet bytes, provenance, dependencies,
   current-content rules, foreign keys, integrity, and guard emptiness before
   commit; and
9. close writable access and classify a generic COMMIT error only from exact
   read-only pre-state or post-state evidence. Anything else is
   `outcome_unknown`/`recovery_required`; there is no retry, replay, repair,
   cleanup, rebinding, or candidate selection.

Duplicate exact requests may return an idempotent already-committed result only
when every v4/v5 identity, packet byte, provenance field, dependency, and
content byte is identical. Reuse of an identifier with any mismatch fails
closed. Provider failure, cancellation, and valid no-question output create no
Historical Question or actual-use provenance. Source change after consent,
stale response, malformed dependency state, or eligibility drift aborts the
entire transaction unchanged.

## In Scope

- Minimal factual Slice 4C-1 promotion correction in architecture/13.
- Founder decision package for Alternative A.
- If separately authorized: one private, unregistered, disposable-only Rust
  creation-parity module and focused tests, connected only as a private module
  to the existing migration/test fixture boundary.
- Exact rollback/failure/restart classification and factual documentation.

## Out Of Scope

Production schema/user_version 5; real user databases or app-data; startup,
Tauri, renderer, UI, provider calls, ContextPacket changes, consent or retention
changes; new eligibility; packet reconstruction; lifecycle UI; export v2;
production backup/restore/recovery; Phase 4; Harness expansion; stage, commit,
push, merge, PR, deployment, or release.

## Product Constraints

The disclosed packet, consent-bound packet, transmitted packet, and persisted
packet are the same immutable object. Selection and panel opening are never
consent. Candidate caps, provider behavior, response contract, and source
eligibility are unchanged. The slice must not become a second Product Harness
policy or a parallel historical persistence system.

## Evidence And Provenance Constraints

Actual-use provenance exists only for a persisted generated artifact after an
authorized successful call. It retains exact provider/model/purpose,
consent/transmission references, packet identity, generation contract versions,
and exact source-revision IDs. The packet digest is not converted to the
artifact canonicalization digest. Provenance fingerprint reuse is allowed only
for byte-identical governed inputs.

## Historical Context Constraints

Only exact explicitly selected, packet-represented persisted Experience text,
confirmed Evidence, and eligible saved user-authored Reflection responses are
accepted. Whole-history loading, Pattern, Context Recovery, identity
hypotheses, rejected Evidence, unreviewed AI output, unsaved drafts, and
unrelated history remain excluded.

## Consent Constraints

Consent remains explicit per generation and per purpose. Source/revision,
task/purpose, provider/model, packet/digest, or expiration mismatch fails
closed. Local storage permission, selection, silence, or opening the panel does
not grant provider-use consent. No blanket consent or consent reuse is added.

## AI-Role Constraints

Output remains either zero to three neutral, source-citing reflective questions
or no question. At least one historical source must be cited when an artifact
is persisted. No summary, recurrence/contradiction/change-over-time conclusion,
diagnosis, sensitive inference, identity finalization, or Phase 4 pattern is
allowed.

## Privacy Constraints

The proposed writer is local, private, unregistered, and disposable-only. It
makes no provider call and cannot reach real app-data. Failed/cancelled and
no-question audit retention stays the existing 30-day minimal-metadata policy;
successful artifact provenance follows existing deletion cascades.

## User-Agency Constraints

No new user action or UI is introduced. Existing preflight include/exclude/
cancel, exact disclosure, explicit send consent, correction, and deletion
semantics are preserved. Normalization cannot increase AI authority or prevent
the user from deleting affected generated artifacts.

## Acceptance Criteria

1. Exact successful creation yields byte- and ID-consistent v4/v5 records,
   immutable packet equality, exact provenance, and exact-revision dependency
   closure.
2. Selection without consent and cancelled preflight create no rows.
3. Provider/model/purpose/digest mismatch, consent expiration/consumption
   mismatch, source edit/delete, stale response, or Reflection eligibility drift
   fails closed with no partial write.
4. Malformed, duplicate, orphaned, cross-source, unsupported, cyclic, extra, or
   missing dependencies fail closed; array IDs are validated for uniqueness
   before set equality.
5. Duplicate byte-identical persistence is idempotently classified; conflicting
   reuse fails closed.
6. Provider failure and valid no-question output create no artifact or
   actual-use provenance, while existing unsuccessful audit retention remains
   unchanged.
7. Failure injection before/after every v4 insert, v5 insert, dependency,
   provenance, lifecycle-link, reconciliation, and commit boundary proves exact
   logical rollback or conservative read-only pre/post classification.
8. Source correction/deletion after creation reaches direct and transitive
   Historical Questions through the promoted Slice 4C-1 bridge, deleting linked
   v4/v5 artifact, packet, dependencies, consent/transmission references, and
   orphaned successful provenance without touching independent unsuccessful
   audit rows.
9. Whole-history and Phase 4 outputs remain impossible; English, Traditional
   Chinese, and Japanese output-contract fixtures remain semantically equal.
10. Production schema/startup maximum remain 4; no runtime or real-user caller
    exists; canonical verification, Clippy, and Theory Alignment Review pass.

### Evaluation matrix

| Case | Required result |
| --- | --- |
| Selection without consent; cancelled preflight | No call-side persistence and no v4/v5 artifact |
| Provider/model/purpose/digest mismatch | Fail closed before mutation |
| Source edit/delete after consent; stale response | Revalidation fails; transaction unchanged |
| Reflection loses confirmed-Evidence dependency | Complete verifier chain refuses it |
| Malformed/duplicate/orphaned dependency | Refuse before set equality or writes |
| Exact duplicate persistence | Return already committed only after complete byte equality |
| Conflicting duplicate identifier | Fail closed; preserve prior state |
| No-question response | No artifact/actual-use provenance; audit boundary unchanged |
| Provider failure | No artifact/actual-use provenance; unsuccessful minimal metadata remains independently expirable |
| Successful creation | Exact packet, consent, transmission, provenance, citation, and source equality across v4/v5 |
| Each injected persistence failure | Complete logical rollback |
| Ambiguous COMMIT/restart | Read-only exact pre/post classification or `recovery_required`; never replay |
| v4/v5 projection parity | One-to-one identity and dependency equality before commit/cascade |
| Source correction/deletion | Promoted Slice 4C-1 direct/transitive closure deletes both representations |
| Orphan check | No successful packet/dependency/provenance orphan remains |
| Whole-history/Phase 4 temptation | Rejected by bounded source set and output evaluator |
| EN/zh-TW/ja fixtures | Same semantic eligibility/refusal and no-policy-drift result |

## Risks

- Dual representation can diverge if writes or cascades are ordered separately;
  one guarded transaction plus exact reconciliation is mandatory.
- Treating packet JSON as ordinary revision content could change consent-bound
  identity; it must remain authoritative in ADR-0009 storage.
- A weak set comparison can hide duplicate IDs; validate durable IDs and
  uniqueness first.
- Parallel validators can drift from promoted Evidence/Reflection logic; reuse
  authoritative verifier chains.
- Over-broad idempotency can accept conflicting persistence; equality must cover
  all bytes and references.
- A generic COMMIT error can be misclassified; outcome remains unknown unless
  exact read-only evidence proves one state.
- Passing disposable tests does not prove production restart safety, migration
  readiness, or real-user safety.

## Open Questions

None within the explicitly authorized Slice 4C-2 boundary. Production schema
activation, real-user migration, runtime integration, and later slices remain
separate Founder decisions.

## Human Decision Required

No. `PHASE3C-SLICE4C2-001` was resolved as Option A with the exact bounded
authority recorded in `.ai/workflow/DECISION_REQUIRED.md`.

## Recommendation

Proceed with authorized Alternative A. The repository supports a bounded design because the
authoritative ADR-0009 v4 transaction, shared schema-v5 contract, promoted
Experience/Evidence/Reflection verifiers, migration fixtures, compatibility
guard, lifecycle link, and Slice 4C-1 cascade already exist. The new code should
orchestrate these authorities, not redefine them. Stop after verification,
Theory Alignment Review, archive/reset, and Founder diff review.

## Review Status

`approved_with_conditions`
