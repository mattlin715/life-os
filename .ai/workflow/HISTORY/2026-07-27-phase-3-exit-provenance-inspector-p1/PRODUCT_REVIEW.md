# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-27-phase-3-exit-provenance-inspector-p1
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: db43b8f47815b0c6ddb1bd8daa9a9503c11a2146
- Working-tree digest reviewed: 0136a4f4c038f8ebad19915c9506f1e6268333d0341d9d9e24db6cce22ff54f1
- Created at: 2026-07-26T16:20:08.464Z
- Updated at: 2026-07-26T16:20:08.464Z

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

Evaluate the smallest user-visible, local, read-only provenance inspection
slice supported by the exact successful Phase 3B packet snapshot already
persisted with a Historical Question artifact. The inspector must explain an
existing actual-use chain without interpreting the user's history, querying
current sources, creating consent, or adding storage authority.

## Problem Statement

The current Historical Question UI shows question text, cited Experience IDs,
provider/model, a shortened packet digest, and deletion. It does not let the
user inspect the exact packet identity, purpose, consent/transmission
references, included items, revisions, authorship/review state, or dependency
relationships already persisted under ADR-0009.

This creates an understandable-provenance gap: the database can enforce what
was used, while the user cannot inspect that chain without database knowledge.

## User Value

An explicit inspector would let the user answer four separate questions:

1. What sources and artifacts were selected into the successful packet?
2. What exact purpose and destination were consented?
3. What successful transmission reference was verified before persistence?
4. Which generated Historical Question artifact and citations were retained?

This strengthens user agency and deletion comprehension. It does not tell the
user what the historical material means.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: **We Build Mirrors, Not Oracles**, Human before
  AI, Evidence before Conclusion, Privacy before Profit, and documentation
  hierarchy. Provenance can expose evidence; it cannot confer truth.
- `docs/03_Principles.md`: evidence must remain traceable and distinguishable
  from conclusions.
- `docs/06_Memory.md`: longitudinal memory requires provenance, correction,
  deletion, selective context, and user control.
- `docs/Reflection.md`: reflection remains user-owned; inspecting a question's
  lineage is not Cross-Experience interpretation.
- `docs/09_AI.md`: AI output is bounded, uncertain, and non-authoritative.
- `docs/10_Privacy.md`: local inspection must minimize disclosure, honor
  deletion, and avoid hidden data reuse.

## Relevant ADRs

- `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md` requires
  durable AI artifacts to remain visibly distinct from user content and keep
  source, authorship, model/version, review, lifecycle, and deletion context.
- `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`
  keeps selection, consent, transmission, and actual use distinct. Exact
  successful packet snapshots share the generated artifact lifecycle.
- `docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md`
  keeps Phase 4 interpretation outside this inspector.
- `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`
  approves a broader future lifecycle/provenance foundation but does not make a
  schema-v4 P1 inspector a complete graph inspector.

## Current Implementation Context

### Implemented and verified

- `HistoricalQuestionArtifact` already hydrates `id`,
  `currentExperienceId`, generated questions and citations, the exact
  `HistoricalContextPacket`, `consentId`, `transmissionId`, and `generatedAt`.
- The packet already contains packet ID/digest/schema, task/purpose,
  destination, versions, current Experience snapshot, included Experience and
  artifact IDs, types, content, authorship, review state, revisions, relevance
  reasons, retrieval algorithm version, and consent reference/scope.
- Rust persistence validates the current Experience, exact historical source
  and artifact revisions, artifact eligibility, consumed consent, successful
  transmission, packet digest, provider/model, citations, and dependency
  uniqueness before committing the artifact, packet snapshot, and dependencies
  atomically.
- The current SQLite read path loads only `payload` and `packet_snapshot` for
  the current Experience. No additional consent/transmission query is used.
- Source edits, deletions, rejection, or eligibility loss cascade-delete
  dependent Historical Questions and packet snapshots. Deleting the Historical
  Question deletes its dependencies and referenced successful
  consent/transmission chain.
- R1 saved-date retrieval is promoted, local, session-only, and independent of
  this persisted packet.

### Current limitations

- The UI exposes only a compact provider/model/digest summary.
- SQLite hydration uses `JSON.parse` plus a TypeScript cast. A P1 view-model
  boundary must validate every displayed field and fail closed rather than
  interpreting malformed runtime data.
- The loaded artifact contains transmission and consent references, not their
  complete event payloads. P1 may truthfully describe the persistence invariant
  and show the IDs; it may not invent an event timestamp or reread an event
  outcome without separate storage authority.
- Schema v4 represents packet items rather than the complete future schema-v5
  provenance/dependency graph.

### Proposed only

The collapsed, local, read-only P1 inspector described below. This Product
Review does not authorize its implementation.

## In Scope

If the Founder chooses Option A:

1. One inspector attached to one persisted Historical Question artifact.
2. Collapsed by default; explicit open/close.
3. A pure validation/view-model boundary over the already-loaded artifact and
   packet snapshot.
4. Human-readable separation of selected packet contents, consent scope,
   successful-transmission reference, and persisted generated artifact.
5. Technical IDs, digest, versions, revisions, provider/model, requested
   purpose, citations, authorship/review state, relevance, and packet
   dependency relationships.
6. Exact outgoing content collapsed separately and revealed only through a
   second explicit local action.
7. Calm fail-closed handling for malformed, unsupported, incomplete, or
   contradictory data.
8. English, Traditional Chinese, and Japanese parity.
9. Focused tests and factual Book One synchronization.

## Out Of Scope

- New SQL, persistence API, table, migration, schema-v5 activation, retention,
  export, or lifecycle policy.
- Loading consent/transmission event payloads beyond the references already in
  the artifact.
- Rehydrating source content from current Experiences or other tables.
- Provider calls, consent creation, audit events, packet mutation, clipboard
  automation, or persisted inspector state.
- Phase 4 recurrence, contradiction, change-over-time, summary, causality,
  identity, diagnosis, sensitive inference, or meaning interpretation.
- A complete provenance/dependency graph inspector.
- Harness expansion, Stage 2/3, Git promotion, PR, deployment, or release.

## Product Constraints

- The inspector is a mirror of retained facts, not an oracle about meaning.
- Opening the inspector must be an explicit local action and must not imply
  consent.
- Selected, consented, transmitted, and persisted states must be labeled
  separately rather than collapsed into "AI used this."
- Human-readable labels lead; IDs/digests are expandable technical evidence.
- P1 must identify itself as partial schema-v4 actual-use inspection.

## Evidence And Provenance Constraints

- Display only fields that the validator can prove from the loaded artifact and
  exact packet snapshot.
- A persisted artifact permits the UI to state that its transmission reference
  was verified as successful by the persistence invariant. It does not permit
  an invented event time, provider receipt guarantee, or truth claim.
- Question citations must be validated against the current Experience ID plus
  packet historical source IDs and must cite at least one historical source.
- Packet consent reference must equal artifact consent ID. Packet digest,
  schema, task, purpose, destination, included-item identities, and revisions
  must satisfy the fixed P1 contract before display.
- Contradictory or unsupported data produces no partial provenance story.

## Historical Context Constraints

- The inspector does not perform retrieval.
- It must not load whole history or current source rows.
- Packet content is retained historical actual-use evidence, not a current
  source substitute.
- R1 saved-date filtering and Phase 3A selection remain unchanged.

## Consent Constraints

- Opening, closing, or revealing the inspector creates no consent.
- P1 displays the retained one-generation/one-purpose consent reference and
  scope only.
- No consent is reused, renewed, consumed, or invalidated by inspection.
- No selection state is reconstructed from the packet for a future generation.

## AI-Role Constraints

- No provider is called.
- No model output is generated, summarized, classified, or interpreted.
- Provenance labels must not imply that the generated question is correct,
  insightful, or endorsed by the user.
- No Phase 4 conclusion or sensitive inference is derived from IDs, content, or
  dependency structure.

## Privacy Constraints

- Exact outgoing content is hidden by default behind a second explicit reveal.
- Reveal stays local, is not persisted as UI state, is not automatically
  copied, and emits no audit or provider event.
- Deleted content is never reconstructed from current Experience or artifact
  records.
- Existing ADR-0009 cascade behavior is authoritative; when the Historical
  Question disappears, its inspector disappears.
- Provider receipt remains historically irreversible even though local
  inspection and deletion remain user-controlled.

## User-Agency Constraints

- The user chooses whether to inspect and whether to reveal content.
- Open, close, and reveal have no hidden side effects.
- The UI explains the four lifecycle stages without requiring database
  knowledge.
- Delete remains available under the existing lifecycle and must remove the
  inspector with the artifact.

## Acceptance Criteria

1. Inspector is collapsed by default for every Historical Question artifact.
2. Explicit open and close affect only that artifact and current session.
3. Valid view model shows exact generated artifact ID/time, questions,
   citations, packet identity/digest/schema/versions, destination, purpose,
   consent/transmission references, included items, revisions, authorship,
   review state, relevance, and packet relationships.
4. The four stages—selected, consented, transmitted, persisted—are visibly
   distinct and factually bounded.
5. Exact outgoing content is hidden by default and requires a second explicit
   reveal.
6. Reveal has no provider, storage, consent, audit, clipboard, or packet
   mutation side effect and resets on close/restart.
7. Unsupported schema, missing fields, malformed items, packet/artifact consent
   mismatch, duplicate dependencies, or contradictory citations fail closed
   with a calm localized message and no fallback to current source content.
8. Deleting the artifact removes the inspector through existing behavior.
9. English, Traditional Chinese, and Japanese expose equivalent terminology
   and disclosure.
10. Tests prove no provider call, no persistence mutation, R1 regression
    stability, Phase 3B packet/consent stability, and no Phase 4 language.
11. Production schema and `user_version` remain 4.

## Risks

- Dense technical provenance can overwhelm users or imply certainty.
  Mitigation: progressive disclosure and human-readable lifecycle labels.
- Exact packet content duplicates sensitive text by design under ADR-0009.
  Mitigation: keep it hidden by default and never reconstruct deleted content.
- A TypeScript-only cast could turn malformed legacy data into misleading UI.
  Mitigation: one pure fail-closed validator/view model.
- "Transmitted" can overclaim what is locally provable. Mitigation: describe a
  successful transmission reference verified before artifact persistence, not
  provider-side retention or receipt guarantees.
- P1 could be mistaken for the complete schema-v5 inspector. Mitigation:
  visibly state the partial schema-v4 boundary and leave broader graph
  inspection open.

## Open Questions

none. The Founder selected Option A and authorized only the exact bounded P1
contract recorded in `DECISION_REQUIRED.md`.

## Human Decision Required

No. `PHASE3-PROVENANCE-INSPECTOR-P1-001` is resolved as Option A.

## Recommendation

Proceed with the smallest authorized Option A implementation. Engineering
Planning must preserve the pure fail-closed validator/view-model boundary,
already-loaded-data-only rule, separately collapsed exact-content reveal,
ADR-0009 cascade semantics, three-locale parity, and every recorded non-scope.

## Review Status

approved_with_conditions
