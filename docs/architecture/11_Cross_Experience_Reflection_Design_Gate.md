---
status: Founder-approved
version: 0.3
owner: product-and-engineering
last_updated: 2026/08/11
depends:
  - docs/00_Constitution.md
  - docs/02_Philosophy.md
  - docs/03_Principles.md
  - docs/05_Identity.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/07_Awareness.md
  - docs/08_Growth.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/appendix/Harness.md
  - docs/architecture/08_Local_Historical_Context_Selection_Foundation.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
  - docs/adr/ADR-0003-identity-is-emergent.md
  - docs/adr/ADR-0005-ai-provider-abstraction.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
referenced_by:
  - docs/00_Index.md
  - docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md
---

# 11 Cross-Experience Reflection Design Gate (Founder-Approved)

## Purpose And Decision State

This document is a design gate for Phase 4 Cross-Experience Reflection. It audits the actual Phase 3 exit state and proposes a bounded relationship between AI, selected history, and user-owned meaning.

It does not authorize implementation. It does not authorize a migration, provider call, production packet, UI activation, or reuse of Phase 3B consent. Every proposal below remains subject to explicit founder decisions and acceptance of ADR-0010.

Decision states remain separate:

| State | Meaning |
| --- | --- |
| Implemented | Verified current repository behavior. |
| Proposed | A recommendation in this design, with no production authority. |
| Founder-approved | A proposal explicitly accepted by the founder. |
| Implemented and verified | Approved behavior delivered in code and accepted through automated and founder-manual gates. |

**We Build Mirrors, Not Oracles.** More context increases responsibility, not AI authority.

## Source-Of-Truth Conclusions

- `docs/Reflection.md` permits Cross-Experience Reflection to compare selected experiences and propose repeated themes or contradictions when sources remain visible. It does not transfer ownership of meaning to AI.
- `docs/06_Memory.md` requires selective, explainable retrieval, provenance, correction, deletion, and no silent identity accumulation.
- `docs/09_AI.md` permits observation, comparison, and hypothesis while requiring uncertainty, alternative explanation, evidence citation, and user ownership.
- `docs/07_Awareness.md` allows cross-experience pattern hypotheses but treats honest no-pattern and insufficient-context outcomes as necessary restraint.
- `docs/05_Identity.md` and ADR-0003 make identity emergent. A pattern hypothesis cannot become an identity conclusion.
- `docs/10_Privacy.md` makes context-use consent ongoing. Storage consent and Phase 3B consent do not authorize Phase 4.
- ADR-0005 requires one conceptual contract across providers or refusal.
- ADR-0007 requires reviewed durable artifacts to retain provenance, lifecycle, correction, deletion, and authorship distinctions.
- ADR-0009 authorizes only Historical Reflection Questions. Recurrence, contradiction, change-over-time, historical summary, and Cross-Experience Pattern hypotheses remain outside that consent and task.

No Constitution or Book Zero redefinition is proposed.

## Phase 3 Exit Audit

The audit uses the Phase 3 capabilities and exit criteria in `docs/12_Roadmap.md`, the production UI, storage interfaces, SQLite schema v4, export code, local retrieval code, and Phase 3B provenance path.

| Capability or exit criterion | State | Repository evidence and gap |
| --- | --- | --- |
| Artifact and source inspection | **Partially implemented** | Experiences and persisted Evidence, Reflection, Pattern, Context Recovery, and Historical Question artifacts rehydrate into the timeline. Historical questions show provider/model, digest prefix, question text, and source IDs. The UI does not expose complete provenance, packet snapshots, dependency graphs, or artifact revision history as a general inspection surface. |
| Correction and deletion | **Partially implemented** | Experience edit/delete, pre-review Evidence edit/reject, answered Reflection revision, Pattern candidate review, and Historical Question delete exist. Confirmed Evidence and confirmed Pattern records lack general post-review edit/delete controls. Experience edits currently invalidate all source-scoped derived artifacts rather than preserve a revisable lineage. |
| Artifact/source export | **Partially implemented** | JSON and Markdown export preserve Experience records only. Evidence, Reflection, Pattern, Context Recovery, Historical Questions, consent/transmission references, packet snapshots, dependencies, and provenance are absent. Artifact export is therefore **absent** even though source Experience export exists. |
| Revision history | **Absent** | Records carry `createdAt` and `updatedAt`; successful Phase 3B packets preserve exact used revisions. There is no general append-only revision history, correction lineage, prior-value inspection, or exportable revision chain. Timestamps and snapshots are not a revision-history capability. |
| Provenance | **Implemented for current persisted artifacts; partial as a user-facing capability** | Source-scoped artifacts retain authorship/provider/model/Harness/prompt/source IDs. Phase 3B persists packet, consent, transmission, revisions, and dependencies. Legacy hydration can remain `legacy_unknown`, and the UI/export do not make the full chain generally inspectable. |
| Selective retrieval | **Implemented narrowly** | `local-lexical-v2` is explicit-panel, bounded, deterministic, local, capped, and filters fragmentary/generic CJK overlap. Previously persisted v1 provenance remains readable. It does not load whole history. |
| Explainable retrieval | **Implemented narrowly** | Visible shared lexical terms explain each candidate. The reason is lexical overlap, not a semantic claim. |
| Retrieval by theme | **Partially implemented** | Shared visible terms can approximate themes, but there is no governed theme taxonomy, user theme selector, or independently evaluated theme signal. |
| Retrieval by emotion | **Absent as an explicit retrieval signal** | Evidence may carry kind `emotion`, but retrieval only compares lexical terms and has no emotion filter or explanation type. |
| Retrieval by relationship | **Absent as an explicit retrieval signal** | No relationship entity/filter/reason contract exists. Lexical overlap may incidentally match a relationship word. |
| Retrieval by value conflict | **Absent as an explicit retrieval signal** | No value/conflict representation or filter exists. Evidence kind `contradiction` does not implement value-conflict retrieval. |
| User-selected time range | **Absent** | Source timestamps affect deterministic sorting, but retrieval input has no time-range control or filter. |
| Consent awareness | **Partially implemented** | Phase 3B has exact, one-generation, one-purpose consent for Historical Reflection Questions. Selection is not consent. There is no Phase 4 consent, no general longitudinal-use authorization, and no durable future-use permission. |
| Preserve confirmation and rejection | **Partially implemented** | Confirmed artifacts persist. Rejected Evidence and Pattern records are deliberately removed rather than retained as review history, so rejection history is not preserved as Phase 3 describes. |

### Exit conclusion

Phase 3B is correctly implemented and promoted, but the broader Phase 3 exit criteria are not fully met. The blocking gaps are:

1. complete artifact/provenance export;
2. general revision and rejection history;
3. post-review correction/deletion for durable artifacts;
4. explicit emotion, relationship, value-conflict, and time-range retrieval;
5. a user-facing provenance/dependency inspection surface sufficient for Cross-Experience hypotheses.

Phase 4 design is founder-approved. Under accepted Decision 13A, all five gaps remain blocking and must be completed and verified before Phase 4 production implementation; no deferral is approved.

## Phase 3 Exit Alternatives

### Alternative A — Complete Phase 3 before any Phase 4 implementation

Finish every identified Phase 3 gap, verify the exit criteria, then implement Phase 4.

- Benefit: strongest provenance, correction, portability, and retrieval foundation.
- Risk: delays learning about whether Cross-Experience Reflection creates value; may overbuild revision and retrieval systems before the Phase 4 artifact is validated.

### Alternative B — Permit Phase 4 design now; block implementation until Phase 3 exits

Complete theory, threat model, consent, artifact, evaluation, and migration design now. Keep every production gate closed until the blocking Phase 3 capabilities are delivered and verified.

- Benefit: exposes Phase 4 requirements early without weakening the exit gate or committing schema/code prematurely.
- Risk: design assumptions may change as Phase 3 gaps are implemented.

### Alternative C — Explicitly defer selected Phase 3 criteria

The founder identifies which exit criteria are not required for the first Phase 4 vertical slice. After approval, update the Roadmap explicitly before implementation.

- Benefit: permits a smaller experiment.
- Risk: portability, revision, or retrieval debt could become embedded in a sensitive longitudinal feature; deferral can be mistaken for completion.

### Accepted approach

The founder accepted **Alternative B** for phase sequencing: complete and promote the design now while keeping production gates closed. Decision 13A further establishes that all five Phase 3 gaps must be completed and verified; Alternative C and any Roadmap deferral were not approved.

## Founder-Approved Phase 4 Product Boundary

Phase 4 may help the user examine selected records across time. It may not tell the user who they are or what their history finally means.

### Founder-approved permitted output

A Phase 4 output may be a **Cross-Experience Hypothesis** containing:

1. a source-bound observation of what appears similar, different, or in tension;
2. exact supporting source citations;
3. exact counter-evidence or non-matching sources from the selected packet;
4. at least one plausible alternative explanation;
5. a qualitative confidence basis tied to evidence sufficiency, never model fluency;
6. a direct invitation for the user to confirm, edit, reject, or add context;
7. an honest `no hypothesis` or `insufficient context` outcome.

Permitted phrasing remains tentative:

> Across these selected records, a possible repeated theme is ____. Source A and Source B support it; Source C may point another way. Does this fit your understanding, or is another explanation closer?

### Founder-approved recurrence boundary

AI may say that a theme **appears in several selected records**. It may not say a behavior always recurs, reveal an underlying cause, predict future behavior, or turn frequency into identity.

### Founder-approved contradiction boundary

AI may surface a **visible tension between exact sources**. It must preserve both sides and allow context, time, role, or changed priorities as alternatives. It may not resolve the contradiction, accuse the user of inconsistency, or decide which source is the true self.

### Founder-approved change-over-time boundary

AI may describe **source-bound differences at user-selected time points**. It may not claim improvement, decline, healing, regression, cause, or a completed personal narrative unless the user supplies that meaning.

### Founder-approved summary boundary

A bounded chronology or comparison table may organize exact selected sources. A narrative summary that replaces source inspection or user interpretation is prohibited. No output may become an automatic life story.

### Categorically prohibited output

- diagnosis or treatment framing;
- sexuality, religious, or political identity inference;
- moral-character judgment;
- immutable personality labels;
- identity finalization;
- silent profiling or background scoring;
- deterministic recurrence claims;
- deterministic contradiction resolution;
- authoritative change-over-time narratives;
- causation inferred from recurrence;
- advice presented as the conclusion of historical analysis;
- automatic Evidence, Awareness, Growth, or Identity conversion;
- whole-history summaries or hidden whole-history loading.

## Founder-Approved Lifecycle And Retention States

The following matrix describes the **founder-approved design policy**. It is not implementation authority. “Session” means memory scoped to the active review flow, with a defensive maximum lifetime of 24 hours. “Minimal tombstone” means IDs, timestamps, purpose, outcome/error class, digest, provider/model, schema versions, and cleanup state only. It contains no source text, packet content, generated interpretation, user revision, prompt, or provider response.

| State | Content persisted? | Metadata persisted? | Retention | Deletion trigger | Dependency behavior | Future-context eligibility | Export visibility | Provider receipt reversible? | Feature disabled |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1. Ephemeral source selection | No. | Session-only selected IDs and visible reasons. | Until panel close, selection change, source mutation, feature disable, or session expiry. | Any listed event or explicit cancel. | Mutation immediately closes disclosure/preflight. | Never. | Not exported. | No receipt exists. | Clear selection. |
| 2. Unconsumed consent | No durable packet or consent content. | Session-only consent intent bound to one digest; it is not a consent event until the explicit action. | Until explicit consent, cancel, invalidation, or preflight expiry. | Cancel, mutation, destination/locale/version change, or expiry. | Any mismatch invalidates the preflight. | Never. | Not exported. | No receipt exists. | Invalidate and clear. |
| 3. Consumed consent | Packet content is retained only if an accepted artifact is later committed under the same transaction chain. | Yes, minimal consent and consumption metadata. | 30 days when no artifact is accepted; otherwise linked to the accepted artifact/tombstone lifecycle. | Expiry, artifact deletion, or user audit purge, subject to Decision 15. | Bound to exactly one transmission attempt; never reusable. | Never. | Metadata only, unless exported as part of an accepted artifact audit. | No; consumption records an attempted external action. | No new use; cleanup continues. |
| 4. Unsuccessful transmission | No provider output or packet content retained durably. | Minimal outcome/error-class tombstone only after consent was consumed. | 30 days, then cleanup. | Expiry or user audit purge. | No hypothesis dependency is created; retry requires new disclosure and consent. | Never. | Metadata only while retained. | If nothing was received by the provider, not applicable; uncertain receipt must be disclosed as irreversible. | Retry disabled; cleanup continues. |
| 5. Successful transmission without user acceptance | Provider output and packet remain session-only. | Minimal successful-transmission tombstone; no generated interpretation. | Content: session/24-hour maximum. Metadata: 30 days. | Accept, reject, session expiry, explicit cancel, or feature disable. | Source mutation makes the candidate stale and purges session content; acceptance is blocked. | Never. | Metadata only; candidate content is not exportable before acceptance. | No. Provider receipt cannot be undone. | Purge session content; retain only approved minimal metadata. |
| 6. Session-only candidate output | Yes, in volatile session state only. | Session-only source links, output validation result, and review state. | Until accept, edit-and-accept, reject, source mutation, session/24-hour expiry, or feature disable. | Any listed event. | Revalidate dependencies transactionally before acceptance. | Never. | Not exported. | No. | Purge candidate content. |
| 7. Rejected candidate | No durable candidate, edit, or generated interpretation. | Minimal rejection tombstone only if a consumed provider attempt must remain auditable. Silence never creates this state. | 30 days for tombstone; otherwise none. | Explicit reject starts immediate content purge; tombstone expires or is user-purged. | No dependency graph or reusable artifact remains. | Never. | Metadata only while retained. | No. | Cleanup continues; no content remains. |
| 8. Accepted or edited-and-accepted hypothesis | Yes: AI original and user-accepted revision remain distinct. The exact packet is initially retained only to support provenance. | Full accepted-artifact provenance and dependencies. | Until user deletion, invalidation cleanup, source lifecycle action, or an approved retention limit. | Hypothesis delete/reject, final dependency deletion, or source policy under Decision 16. | Current dependencies are exact and inspectable; any mutation invalidates rather than rewrites. | Only while accepted, current, non-invalidated, explicitly selected, disclosed, and newly consented. Never Evidence. | Full user-facing artifact and provenance export, subject to purged-content markers. | No. | Remains inspectable/exportable but cannot be transmitted or newly accepted. |
| 9. Invalidated hypothesis | Recommended: retain user-accepted hypothesis and AI original, but purge packet/source copies when invalidation arose from source correction/deletion. | Non-content provenance tombstone, invalidation reason, affected IDs/revisions, and purge state. | Until user deletes the hypothesis or an approved artifact-retention limit. | User deletion/rejection, final dependency deletion policy, or expiry. | Frozen; never silently recalculated or restored. Regeneration creates a new artifact and consent chain. | Never. | Exported as invalidated, with missing/purged source content explicit. | No. | Remains inspectable/exportable; no reuse. |
| 10. Deleted Experience or source artifact | No deleted source text may survive in a hidden Phase 4 packet under the recommended policy. | Minimal deletion/invalidation tombstone may retain source ID, revision/digest, timestamps, and cleanup state. | Until dependent cleanup and audit expiry complete. | Source deletion request. | Invalidate dependents, purge their content-bearing packet copies, and apply final-dependency policy. | Never. | Deleted content absent; tombstone and affected-artifact state visible while retained. | No. | Deletion and cleanup must still run. |
| 11. Deleted Phase 4 hypothesis | No hypothesis, AI original, user revision, packet, or content-bearing provenance. | Minimal deletion tombstone only for audit/cleanup integrity. | 30 days or until cleanup succeeds, whichever is later; user may request audit purge after integrity completion. | Explicit hypothesis deletion. | Delete dependency edges and content; never cascade into source deletion. | Never. | At most a non-content deletion record while retained. | No. | Deletion remains available; cleanup continues. |
| 12. Content-bearing packet snapshot | Yes, but only after explicit hypothesis acceptance and only while all included sources remain current and retained. | Digest, schema/version, consent/transmission links, and dependency roles. | Coextensive with the current accepted hypothesis; purge on source edit/delete/rejection/supersession, hypothesis deletion/rejection, final dependency deletion, or approved expiry. | Any listed lifecycle event. | Exact snapshot never silently changes. Purging replaces it with a non-content tombstone. | Never directly; it is provenance, not a reusable source. | Export only through explicit provenance export while content still exists. | No. | Retain for accepted artifacts unless source lifecycle requires purge; never transmit. |
| 13. Minimal non-content audit tombstone | No. | Yes, only the minimal fields defined above plus cleanup attempts/last error. | 30 days for nonaccepted activity; accepted/invalidated artifact tombstones follow artifact retention. Cleanup-failure tombstones persist until successful cleanup, then resume normal expiry. | Expiry, explicit audit purge after cleanup, or artifact deletion policy. | Can prove an action/outcome but cannot reconstruct or seed content. | Never. | Export as audit metadata when linked to a retained artifact; otherwise only through an explicit local audit export. | No. | Cleanup and expiry continue. |

Cancelled preflight, panel opening, selection, silence, and an unconsumed consent intent create no durable record. A cleanup failure must be visible, retryable, idempotent, and unable to restore content or reopen eligibility.

## Founder-Approved Evidence Sufficiency

The initial recommendation is deliberately conservative:

- manually select at least **three persisted Experiences** for recurrence or change-over-time review;
- use at least **two distinct time points**, with the user choosing the time range;
- require at least two sources to contain confirmed Evidence or saved user-authored Reflection responses relevant to the proposed comparison;
- permit a two-source contradiction comparison only when the output is explicitly a tension between exact statements, not a recurring pattern;
- include every selected source in the evaluator's counter-evidence scan, even if it does not support the candidate hypothesis;
- fail with `insufficient context` when relevance, independence, timestamps, or provenance are weak.

Record count alone never establishes sufficiency. Duplicate retellings, copied text, one event split into many entries, or sources generated from one parent must not be counted as independent support.

## Founder-Approved Eligibility

| Record | Founder-approved Phase 4 rule |
| --- | --- |
| Persisted Experience | Eligible only through manual exact selection and disclosure. |
| Confirmed Evidence | Eligible when current, source-scoped, disclosed, and explicitly included. It remains reviewed evidence, not objective truth. |
| Saved user-authored Reflection | Eligible when answered, current, dependencies remain eligible, and explicitly included. |
| Single-experience Pattern | A confirmed Pattern may be shown only as a prior hypothesis to revisit or counter; it cannot count as Evidence or silently seed a new conclusion. |
| Rejected Pattern or Evidence | Ineligible for generation and future context. Its rejection may remain as lifecycle history if the founder approves retention, but its content must not influence the model. |
| Edited/superseded artifact | Old revision is ineligible except for an explicit user-requested revision comparison under a separately disclosed purpose. |
| Historical Reflection Question | Ineligible as evidence or support. It is a generated question, not user meaning. |
| Phase 4 hypothesis | Eligible for future display only after explicit acceptance; provider reuse always requires a new disclosure, purpose, packet, and consent. |
| Draft, skipped, orphaned, legacy-unknown, or unreviewed output | Ineligible. |

## Founder-Approved Consent Contract

Consent should be:

- explicit per generation;
- explicit per comparison purpose (`recurrence_review`, `tension_review`, or `change_review`), with mixed purposes prohibited in the first slice;
- bound to one immutable packet digest and exact source/artifact revisions;
- bound to output schema, evaluator, provider, model, locale, prompt, Harness, and safety-contract versions;
- invalidated by any content, source, selection, purpose, destination, language, version, eligibility, or feature-state change;
- consumed by one transport attempt;
- requested again for retry, regeneration, alternative-purpose generation, persistence after a stale response, and all future provider reuse.

Phase 3B consent, local storage consent, panel opening, selection, prior acceptance, silence, and inactivity are never Phase 4 consent.

## Founder-Approved Provider-Independent Packet

If implementation is later approved, use a separate immutable Phase 4 packet containing:

- packet ID, digest, schema, timestamps, expiry, and declared comparison purpose;
- exact current and selected Experience IDs, revisions, bounded content, and user-selected time range;
- exact included artifact IDs, types, revisions, authorship, review state, and bounded content;
- visible retrieval reasons and selection source (`manual`);
- dependency and independence metadata needed for sufficiency checks;
- counter-evidence search scope limited to the exact selected packet;
- provider/model destination and provider-retention disclosure;
- Harness, prompt, output, evaluator, and safety versions;
- Phase 4-specific one-shot consent reference;
- strict source, content, and token limits.

The packet must exclude unselected history, drafts, rejected content, hidden profiles, Phase 3B consent, unrelated sensitive history, and inferred identity fields.

## Founder-Approved Artifact And Provenance Model

A durable accepted hypothesis should preserve:

- artifact ID and type `cross_experience_hypothesis`;
- purpose and hypothesis kind;
- original model output and the user's accepted/edited text as distinct fields;
- state: `candidate`, `accepted`, `edited`, `rejected`, `superseded`, or `invalidated`;
- supporting and counter-evidence source IDs, artifact IDs, types, and exact revisions;
- alternative explanations and qualitative confidence basis;
- packet digest/schema and the exact successful packet snapshot initially retained at acceptance, with later content purge represented explicitly under Decision 16;
- consent and transmission references;
- provider/model, Harness/prompt/output/evaluator/safety versions;
- generated, reviewed, revised, invalidated, and deleted timestamps;
- dependency graph and invalidation reason;
- explicit rule that confirmation means `useful to the user`, not `objectively true`.

## Correction, Deletion, And Stale Work

### Before transport

Re-read every selected Experience and artifact. Revalidate revisions, eligibility, purpose, time range, source independence, destination, versions, digest, feature state, and unconsumed Phase 4 consent. Any mismatch closes the preflight and sends nothing.

### Before persistence

Inside one serialized transaction, revalidate the same chain plus consent consumption, successful transmission, exact output contract, and dependency set. Any mismatch returns `stale_generation` and persists no hypothesis.

### After persistence

- Editing or deleting a contributing Experience invalidates dependent hypotheses.
- Editing, rejecting, deleting, or superseding an included Evidence, Reflection, or Pattern invalidates dependent hypotheses.
- Removing support must not silently recalculate confidence or rewrite the artifact.
- Under the recommended source-lifecycle policy, invalidation caused by source correction or deletion also purges the old source text from every content-bearing packet snapshot. The accepted hypothesis may remain visibly invalidated, with only non-content provenance tombstones. Exact audit reconstruction is intentionally reduced unless the founder separately authorizes revision-content retention.
- The user may explicitly regenerate through a new packet and consent.
- Deleting a hypothesis purges the hypothesis text, AI original, user revision, successful packet snapshot, and all content-bearing provenance. Only an approved minimal non-content deletion tombstone may remain. Provider receipt cannot be undone.
- Rejected or invalidated hypotheses never re-enter model context.

### Source correction and deletion matrix

This matrix records the founder-approved Decision 16 design policy. Current schema v4 does **not** provide this Phase 4 lifecycle: Experience deletion cascades current source-scoped artifacts; Phase 3B has deletion triggers for dependent historical-question artifacts and their consent/transmission chain; rejected Evidence and Pattern candidates are removed; and no general revision history exists.

| Action | Dependent hypothesis | Packet/content-bearing provenance | Non-content audit | Future eligibility |
| --- | --- | --- | --- | --- |
| Experience edit | Invalidate; never rewrite. | Purge old Experience text and affected artifact copies from snapshots; replace with tombstone. | Keep IDs, old revision/digest, new revision, reason, timestamps, cleanup state. | Old hypothesis ineligible; edited Experience requires new selection/disclosure/consent. |
| Experience delete | Invalidate; if it was the final dependency, apply founder-approved final-dependency deletion policy. | Purge deleted Experience text from all snapshots. | Keep only minimal deletion/dependency tombstone. | Deleted source and dependent hypothesis ineligible. |
| Evidence/Reflection/Pattern edit | Invalidate dependents; edited artifact becomes a new revision. | Purge prior artifact content from snapshots unless explicit revision-retention consent exists. | Keep artifact ID, old/new revision digests, reason, timestamps. | Old revision ineligible; new revision requires new selection and consent. |
| Artifact rejection | Invalidate dependents. Silence is not rejection. | Purge rejected artifact content from snapshots. | Minimal explicit-rejection and dependency tombstone only. | Rejected artifact and dependents ineligible. |
| Artifact deletion | Invalidate dependents; final dependency follows approved policy. | Purge deleted artifact content from snapshots. | Minimal deletion/dependency tombstone only. | Never eligible. |
| Artifact supersession | Invalidate dependents; do not substitute the replacement silently. | Purge superseded content from hidden snapshots absent revision-retention consent. | Link old/new IDs or revisions without content. | Superseded revision ineligible; replacement needs new consent. |
| Phase 4 hypothesis edit | Create a user revision; preserve AI original distinctly; accepted edit remains a hypothesis, not Evidence. | Keep the exact packet only while sources remain current. | Record revision authorship and timestamps. | Edited-and-accepted current version may be eligible with new consent. |
| Phase 4 hypothesis rejection | Purge hypothesis content, AI original, user revision, and packet. | Purge all content-bearing provenance. | Minimal explicit-rejection tombstone only if Decision 15 approves it. | Never eligible. |
| Phase 4 hypothesis deletion | Purge the complete artifact and all content-bearing provenance. | Purge packet and dependency content; do not delete sources. | Minimal deletion/cleanup tombstone only if approved. | Never eligible. |

### Source deletion alternatives

- **A — Cascade-delete dependent hypothesis and all content-bearing provenance.** Strongest erasure and simplest mental model; weakest continuity and auditability because even user-accepted meaning disappears. A mistaken source deletion can destroy the dependent reflective artifact.
- **B — Retain the user-accepted hypothesis text as visibly invalidated, purge packet/source content, and retain only a non-content tombstone.** Strong local-first user control and avoids hidden duplicated source text. Audit can prove which IDs/revisions were used and why invalidation happened, but cannot reconstruct the exact disclosed source content after purge.
- **C — Retain the full invalidated hypothesis and exact packet snapshot.** Strongest forensic reconstruction, but deleted or corrected personal text survives invisibly in duplicated storage and weakens the meaning of deletion.

**Recommendation: B**, with an always-visible option to delete the invalidated hypothesis too. Until the founder explicitly authorizes revision-content retention, source correction/deletion should not leave hidden duplicated source content. Provider receipt remains irreversible and must be disclosed.

### Candidate and rejection persistence alternatives

- **A — Persist every candidate and lifecycle transition.** Best debugging history, but silently creates durable generated personal interpretations before user acceptance and risks rejected-output resurrection.
- **B — Keep provider output session-only until explicit acceptance; purge rejected or expired candidate content; retain only approved minimal non-content audit metadata.** Preserves the acceptance boundary and supports bounded operational audit without building a hidden interpretation archive.
- **C — Persist no Phase 4 output, even after acceptance.** Smallest privacy surface, but no durable reflection, revision, inspection, or export.

**Recommendation: B.** The durable artifact lifecycle begins at explicit acceptance or edit-and-accept. `candidate` and `rejected` may be transient interaction labels, not durable reusable artifact states. The accepted AI original and the user's accepted revision remain distinct. Silence is neither acceptance nor rejection.

## Threat Model

| Threat | Harm | Founder-approved control |
| --- | --- | --- |
| Small-sample overfitting | Ordinary coincidence becomes a personal truth. | Minimum sufficiency, source independence, counter-evidence, alternatives, no-hypothesis outcome. |
| Confirmation bias | Retrieval selects only material supporting the current concern. | Manual source visibility, explainable retrieval, explicit exclusions, counter-evidence scan across selected sources. |
| Sensitive-context leakage | Unrelated intimate history reaches a provider. | Exact preflight, artifact-level inclusion, bounded packet, sensitive warning, no whole-history access. |
| Consent laundering | Phase 3B or past consent is reused for Phase 4. | New purpose namespace, packet schema, consent type, UI text, and transactional verification. |
| Oracle phrasing | Fluent output is perceived as identity truth. | Structured tentative schema, alternative explanation, confidence basis, prohibited-language evaluator, user acceptance gate. |
| Contradiction as accusation | Contextual differences become moral inconsistency. | Preserve both sources, neutral `tension` vocabulary, no resolution, contextual alternatives. |
| Change narrative fabrication | Differences become claims of growth, decline, or cause. | Source-bound chronology, user-owned meaning, no causation or evaluative trajectory. |
| Pattern-to-identity escalation | Repetition becomes a fixed label. | Artifact taxonomy separation, no automatic Evidence/Identity conversion, explicit non-identity copy. |
| Rejected-output resurrection | A rejected hypothesis silently influences later output. | Hard eligibility exclusion, lifecycle state check, dependency and context-packet tests. |
| Stale-source persistence | A response is saved after source correction. | Revalidation before transport and transactionally before persistence. |
| Provider drift/fallback | Destination or behavior changes after consent. | Provider-independent contract, version binding, no silent fallback, parity evaluation. |
| Packet snapshot accumulation | Sensitive duplicated text outlives its purpose. | Persist only with accepted artifact, tied deletion lifecycle, explicit retention decision. |
| Whole-history profiling | Background retrieval builds an unseen personal model. | Explicit user action, manual selection recommendation, bounded candidates, no background job/profile table. |
| Reviewer fatigue | Complex disclosure leads to mechanical consent. | Small source limits, one purpose per generation, calm exact disclosure, cancellation without penalty. |

## Founder-Approved Evaluation Matrix

Every supported provider must pass the same conceptual cases using synthetic or explicitly consented fixtures.

| Case | Required result |
| --- | --- |
| Phase 3B consent supplied to Phase 4 | Refuse before transport. |
| Panel open, selection, or silence | No consent, call, or durable record. |
| Fewer than required independent sources | `insufficient context`; no hypothesis. |
| Duplicate retellings presented as multiple sources | Do not count as independent support. |
| Strong supporting and clear counter-evidence | Cite both and reduce confidence; do not omit counter-evidence. |
| Two exact statements in tension | Neutral tension hypothesis allowed; no accusation or resolution. |
| Difference across time points | Source-bound difference allowed; no growth/decline/causation narrative. |
| No meaningful recurrence | Honest no-hypothesis result. |
| Sensitive identity inference request | Refuse prohibited inference even with selected sources. |
| Unselected sensitive source exists locally | It never enters packet, prompt, output, or logs. |
| Rejected/invalidated prior hypothesis | Never re-enters context. |
| Source edit before send | Preflight closes; no call. |
| Source edit during call | Response discarded; no artifact. |
| Source edit after acceptance | Dependent hypothesis becomes invalidated; no silent rewrite. |
| Source deletion after accepted hypothesis | Invalidate the hypothesis, purge deleted source text from every packet copy, retain only the founder-approved artifact text/tombstone policy, and never reuse it. |
| Source edit while an old exact packet exists | Invalidate the hypothesis and purge the old source content from the snapshot unless explicit revision-content retention was separately approved. |
| Provider/model/locale/purpose change | New disclosure and consent required. |
| Provider failure or fallback opportunity | No silent destination change; retry requires new consent. |
| User edits candidate before acceptance | Persist AI original and user revision distinctly only after explicit acceptance. |
| User rejects candidate | Purge candidate content; no durable reusable hypothesis; only Decision 15-approved minimal non-content metadata may remain. |
| Successful provider response without acceptance | Output and packet remain session-only, expire on session/24-hour boundary, and never become reusable or exportable content. |
| Rejected candidate storage inspection | No generated interpretation, edit, prompt, response, or content-bearing packet remains; at most minimal non-content metadata exists. |
| User deletes accepted hypothesis | Snapshot and dependencies follow approved deletion lifecycle. |
| Retention expiry cleanup | Expired session content and tombstones are removed without affecting source records or resurrecting eligibility. |
| Cleanup failure and retry | Failure is visible; retry is idempotent; minimal cleanup metadata persists until success; no content becomes eligible during failure. |
| Final dependency deletion | Apply the approved cascade-or-invalidated-text policy, purge source copies, and leave no dangling eligible artifact. |
| Export after invalidation | Mark the hypothesis invalidated, omit purged source content, and expose tombstone/dependency state without implying current support. |
| Feature disabled with accepted artifacts | No transport or reuse; retained accepted artifacts remain inspectable/exportable/deletable under policy. |
| Hidden snapshot scan after source deletion | No deleted source text remains in Phase 4 packet snapshots unless the founder explicitly approved full revision-content retention. |
| English, Traditional Chinese, Japanese | Same consent semantics, uncertainty, citations, and prohibited-output behavior. |
| Unsupported output field or uncited claim | Reject entire output before display/persistence. |
| Feature disabled | Transport gate closed; Phase 3A and Phase 3B remain independently governed. |

## Migration Alternatives

No migration implementation is authorized by this founder-approved design.

### M1 — No durable Phase 4 artifacts

Keep candidate output session-only. Reuse Phase 3B tables only for nothing; do not overload them.

- Benefit: no schema change and smallest privacy footprint.
- Cost: no reviewed continuity, correction history, or later hypothesis inspection.

### M2 — Additive normalized schema v5

Add separate Phase 4 consent, transmission, hypothesis, revision, and dependency tables. Preserve v4 intact. Use an additive migration with rollback-on-failure and no destructive down migration.

- Benefit: correct separation, explicit lifecycle, queryable provenance, and future export.
- Cost: highest implementation and privacy complexity; requires Phase 3 revision/export decisions first.

### M3 — Store accepted Phase 4 artifacts in the generic artifact table

Add a new artifact kind and put packet/provenance in JSON payloads.

- Benefit: less schema surface.
- Cost: weak normalized consent/dependency/revision enforcement and greater risk of mixing hypothesis with source-scoped Evidence.

### Recommendation

Decision 11B approves **M2** as the durable-artifact design direction, but only after the Phase 3 revision/export foundation is resolved. It does not authorize an exact schema or migration implementation. M1 remains the safe no-migration fallback; M3 remains rejected because it weakens taxonomy and transactional provenance.

## Rollback And Feature Disable

The founder-approved rollback direction is non-destructive; exact rollback implementation still requires its separate checkpoint:

1. close only the Phase 4 disclosure, consent, transport, and acceptance gates;
2. keep Phase 3A local retrieval and Phase 3B Historical Reflection Questions governed independently;
3. leave accepted Phase 4 artifacts inspectable but ineligible for provider reuse;
4. do not silently downgrade or reinterpret Phase 4 artifacts as Evidence or Pattern;
5. retain or delete consent/transmission metadata only under the approved retention rule;
6. never decrement schema version or drop tables without a separate destructive-migration founder checkpoint.

## Founder Decision Package

All sixteen decisions have explicit founder answers recorded below. On 2026/07/16, the founder accepted ADR-0010 and approved this architecture as `Founder-approved`. This status does not authorize migration, retention cleanup, provider transmission, or Phase 4 production work; all five Phase 3 exit gaps remain blocking.

### 1. Output authority

- **A:** questions only.
- **B:** tentative Cross-Experience Hypotheses with sources, counter-evidence, alternatives, and user review.
- **C:** direct conclusions.
- **Recommendation:** B. It tests Phase 4 while preserving a user-owned hypothesis boundary. Reject C as oracle behavior.
- **Privacy and auditability risks:** A minimizes interpretive data but cannot test the intended value; B creates sensitive interpretation and provenance requiring lifecycle controls; C maximizes persuasive-authority harm and cannot be made safe by audit alone.
- **Founder question:** Do you choose **1A questions only**, **1B tentative user-owned hypotheses**, or **1C direct conclusions**?
- **Founder answer (2026/07/15):** **1B approved.** Phase 4 may propose tentative, source-citing, user-owned hypotheses with counter-evidence, alternatives, uncertainty, and explicit user review. It may not issue direct conclusions or own final meaning.

### 2. Minimum source sufficiency

- **A:** two selected Experiences for every purpose.
- **B:** three independent Experiences for recurrence/change; two exact sources only for a bounded tension comparison.
- **C:** model decides dynamically without a floor.
- **Recommendation:** B. Count alone remains insufficient; relevance and provenance must also pass.
- **Privacy and auditability risks:** A may overfit two records; B transmits more selected content but provides a clearer sufficiency floor; C is difficult to audit and lets provider behavior redefine product policy.
- **Founder question:** Do you choose **2A two Experiences**, **2B three independent Experiences for recurrence/change and two exact sources for tension**, or **2C model-decided sufficiency**?
- **Founder answer (2026/07/15):** **2B approved.** Recurrence and change review require at least three independent Experiences; a bounded tension review may use two exact sources. Count alone is insufficient: relevance, independence, timestamps, and provenance must also pass.

### 3. Eligible records

- **A:** Experience only.
- **B:** Experience, confirmed Evidence, saved user Reflection, and confirmed single-experience Pattern only as a prior hypothesis.
- **C:** all stored records.
- **Recommendation:** B with artifact-level disclosure. Pattern never counts as Evidence.
- **Privacy and auditability risks:** A has the smallest artifact surface but omits user-reviewed context; B increases disclosure complexity and requires exact artifact lifecycle checks; C risks rejected, draft, or unrelated content entering the packet.
- **Founder question:** Do you choose **3A Experience only**, **3B the narrow reviewed record set**, or **3C all stored records**?
- **Founder answer (2026/07/15):** **3B approved.** Eligibility is limited to persisted Experience text, confirmed Evidence, saved user-authored Reflection responses, and confirmed single-Experience Pattern only as a visibly prior hypothesis. Every included artifact requires exact selection, disclosure, current eligibility, and new Phase 4 consent.

### 4. Counter-evidence and contradiction

- **A:** show supporting sources only.
- **B:** require counter-evidence scan, preserve both sides, show alternatives, and never resolve the tension.
- **C:** let the model select the strongest narrative.
- **Recommendation:** B.
- **Privacy and auditability risks:** A and C produce selective narratives that are difficult to challenge; B discloses and processes more of the selected packet but makes omission and uncertainty inspectable.
- **Founder question:** Do you choose **4A supporting sources only**, **4B mandatory counter-evidence and alternatives**, or **4C model-selected narrative**?
- **Founder answer (2026/07/15):** **4B approved.** Every selected source participates in the bounded counter-evidence scan; supporting and counter-evidence remain visible, at least one alternative explanation is required, tension is never resolved for the user, and weak support must yield no hypothesis or insufficient context.

### 5. Consent granularity

- **A:** reuse Phase 3B consent.
- **B:** new consent per generation, comparison purpose, immutable packet, and destination.
- **C:** persistent longitudinal preference.
- **Recommendation:** B. A and C violate ongoing purpose-specific consent.
- **Privacy and auditability risks:** A launders narrower consent; B adds consent friction but yields an exact auditable action; C enables background authorization and purpose drift.
- **Founder question:** Do you choose **5A reuse Phase 3B consent**, **5B new one-shot Phase 4 consent**, or **5C persistent longitudinal consent**?
- **Founder answer (2026/07/15):** **5B approved.** Phase 4 requires new explicit consent for one generation, one declared comparison purpose, one immutable exact-content packet, and one disclosed destination. Phase 3B consent and all implicit interaction states are ineligible; every retry or reuse requires new disclosure and consent.

### 6. Source selection

- **A:** always manually selected sources in the first Phase 4 slice.
- **B:** model-selected sources with optional review.
- **C:** background whole-history retrieval.
- **Recommendation:** A. Automated suggestions may remain local candidates, but explicit inclusion stays manual.
- **Privacy and auditability risks:** A can miss relevant sources but keeps inclusion legible; B can bias selection and obscure agency; C is whole-history profiling.
- **Founder question:** Do you choose **6A manual exact selection**, **6B model selection with review**, or **6C background whole-history retrieval**?
- **Founder answer (2026/07/15):** **6A approved.** The first Phase 4 slice uses manual exact source and artifact inclusion. Local bounded explainable suggestions may assist discovery, but suggestion, selection, panel opening, and silence never become inclusion or consent automatically.

### 7. Persistence acceptance

- **A:** persist every provider output.
- **B:** persist only after explicit user acceptance or user edit-and-accept.
- **C:** never persist Phase 4 output.
- **Recommendation:** B, with C as the safe fallback if migration is deferred.
- **Privacy and auditability risks:** A silently archives unowned interpretations; B requires a rigorous acceptance transaction and deletion/export lifecycle; C prevents continuity and review history.
- **Founder question:** Do you choose **7A persist every output**, **7B persist only explicit accept/edit-and-accept**, or **7C never persist Phase 4 output**?
- **Founder answer (2026/07/15):** **7B approved.** Provider output remains session-only until explicit accept or edit-and-accept. Only that explicit action creates a durable hypothesis; AI original and user revision remain distinct. Silence, panel closure, and inactivity are neither acceptance nor rejection.

### 8. Rejected and edited artifact reuse

- **A:** rejected or superseded versions may re-enter as counter-evidence.
- **B:** rejected/invalidated versions never re-enter; an edited accepted version may re-enter only as a prior user-reviewed hypothesis with new consent.
- **C:** any past output may be reused.
- **Recommendation:** B.
- **Privacy and auditability risks:** A/C can resurrect content the user set aside; B reduces reconstruction of rejected states but preserves the user's expressed boundary.
- **Founder question:** Do you choose **8A allow rejected/superseded reuse**, **8B exclude it and allow only current accepted edited hypotheses with new consent**, or **8C allow any past output**?
- **Founder answer (2026/07/15):** **8B approved.** Rejected, invalidated, and superseded versions never re-enter context. Only a current edited-and-accepted hypothesis may later be manually selected as a visibly prior user-reviewed hypothesis, never as Evidence, and only through a new disclosure, packet, purpose, and consent.

### 9. Durable provenance

- **A:** IDs only.
- **B:** exact successful packet snapshot at acceptance, digest, consent/transmission, source/artifact revisions, support/counter-evidence roles, provider/model, versions, original AI text, user revision, lifecycle, and explicit content-purge tombstones under Decision 16.
- **C:** final accepted text only.
- **Recommendation:** B at acceptance, modified by Decision 16: exact content remains only while sources are current; correction/deletion purges source copies and leaves non-content tombstones under the recommended policy.
- **Privacy and auditability risks:** A cannot reconstruct actual use; B gives strongest live audit but duplicates sensitive text until lifecycle purge; C hides the generation chain and authorship distinction.
- **Founder question:** Do you choose **9A IDs only**, **9B exact accepted-artifact provenance with Decision 16 lifecycle purge**, or **9C final text only**?
- **Founder answer (2026/07/15):** **9B approved.** Acceptance initially preserves the exact successful packet and complete actual-use provenance, including AI original versus user revision. This does not authorize permanent retention of corrected or deleted source text; Decision 16 governs later content purge and non-content tombstones.

### 10. Dependency invalidation

- **A:** silently update dependent hypotheses.
- **B:** source/artifact edit, rejection, deletion, supersession, or eligibility loss invalidates dependents; regeneration requires new consent.
- **C:** preserve dependents as current regardless of source changes.
- **Recommendation:** B.
- **Privacy and auditability risks:** A fabricates continuity; B can leave invalidated artifacts that require visible status and cleanup; C presents stale interpretations as current.
- **Founder question:** Do you choose **10A silent rewrite**, **10B explicit invalidation and regeneration**, or **10C keep dependents current**?
- **Founder answer (2026/07/15):** **10B approved.** Any included source edit, rejection, deletion, supersession, or eligibility loss explicitly invalidates dependents without rewriting, confidence recalculation, or source substitution. Regeneration requires a completely new governed chain.

### 11. Schema migration

- **A:** no migration; session-only output.
- **B:** additive normalized schema v5 after Phase 3 revision/export decisions.
- **C:** overload schema v4 or generic source-scoped artifacts.
- **Recommendation:** B if durable artifacts are approved; otherwise A. No migration before separate founder approval.
- **Privacy and auditability risks:** A avoids durable data but loses continuity; B adds tables, cleanup paths, and migration risk while enabling enforceable provenance; C weakens taxonomy and transactional guarantees.
- **Founder question:** Do you choose **11A session-only/no migration**, **11B additive normalized schema after Phase 3 foundations**, or **11C overload schema v4/generic artifacts**?
- **Founder answer (2026/07/15):** **11B approved as the design direction; migration implementation is not authorized.** If durable Phase 4 artifacts proceed, use a separate additive normalized schema after the Phase 3 revision/export foundations. Exact schema, migration, rollback, retention, and destructive behavior require a separate founder checkpoint.

### 12. Sensitive and identity inference

- **A:** categorically prohibit diagnosis, sexuality/religious/political identity inference, moral character, immutable personality, identity finalization, and silent profiling.
- **B:** allow when user asks.
- **C:** provider policy decides.
- **Recommendation:** A.
- **Privacy and auditability risks:** A intentionally limits capability to prevent intimate classification; B makes consent appear to legitimize harmful authority; C outsources constitutional boundaries to providers.
- **Founder question:** Do you choose **12A categorical prohibition**, **12B allow on user request**, or **12C provider-policy control**?
- **Founder answer (2026/07/16):** **12A approved.** Users may explore sensitive subjects, but Phase 4 must not infer or determine sensitive identity. Diagnosis, sensitive identity inference, moral-character judgment, immutable labels, identity finalization, silent profiling, deterministic recurrence or contradiction, and authoritative change narratives remain categorically prohibited.

### 13. Phase 3 blockers

- **A:** all five audited gaps block Phase 4 implementation.
- **B:** only provenance inspection, revision history, and artifact export block; structured retrieval signals may be explicitly deferred for a manual-selection slice.
- **C:** no Phase 3 gaps block implementation.
- **Recommendation:** B only if the founder explicitly updates the Roadmap under Alternative C; otherwise A. Phase 4 design may continue either way.
- **Privacy and auditability risks:** A is slowest but closes known lifecycle/export gaps; B embeds explicit debt and requires a truthful Roadmap change; C hides known exit failures and weakens user control.
- **Founder question:** Do you choose **13A all five gaps block**, **13B explicitly defer structured retrieval only after a Roadmap decision**, or **13C no Phase 3 blockers**?
- **Founder answer (2026/07/16):** **13A approved.** All five audited Phase 3 exit gaps block Phase 4 production implementation. Phase 4 design may continue, but no gap is deferred and no Roadmap deferral is authorized.

### 14. Automated and founder-manual gates

- **A:** automated contract tests only.
- **B:** provider-parity, packet, consent, staleness, provenance, migration, multilingual, sensitive-inference, and lifecycle tests plus a founder manual matrix before activation.
- **C:** dogfooding without a fixed gate.
- **Recommendation:** B.
- **Privacy and auditability risks:** A misses provider/UI/lifecycle failures; B costs more but tests both enforceable contracts and comprehensible consent; C produces unrepeatable evidence.
- **Founder question:** Do you choose **14A automated tests only**, **14B automated plus founder-manual gates**, or **14C ungated dogfooding**?
- **Founder answer (2026/07/16):** **14B approved.** Automated contract and provider-parity tests must prove the enforceable boundary, and a fixed founder-manual matrix must prove that disclosure, consent, candidate authority, lifecycle, cleanup, and user control are understandable before activation.

### 15. Retention for nonaccepted Phase 4 activity

- **A:** retain no durable record for cancelled, failed, stale, successful-but-unaccepted, or rejected activity; purge all content and metadata when the session ends.
- **B:** cancellation and unconsumed consent create no durable record; consumed failed/stale/successful-but-unaccepted/rejected content remains session-only with a 24-hour defensive maximum; retain only a minimal non-content audit tombstone for 30 days; retain cleanup-failure metadata until cleanup succeeds, then resume expiry.
- **C:** retain full packets, provider outputs, edits, and lifecycle records for 90 days or indefinitely regardless of acceptance.
- **Recommendation:** B. It supports bounded operational audit without creating a reusable archive of unaccepted personal interpretation. A is the privacy-maximal fallback; C is incompatible with the acceptance boundary.
- **Privacy and auditability risks:** A makes incident reconstruction and cleanup verification difficult; B adds small metadata exposure and requires reliable expiry; C leaves rejected and unowned interpretations durable and increases hidden-content risk.
- **Founder question:** Do you choose **15A no durable nonaccepted activity**, **15B session-only content plus 30-day minimal tombstones**, or **15C full-content retention**?
- **Founder answer (2026/07/16):** **15B approved as the retention design policy; implementation is not authorized.** Cancelled and unconsumed activity creates no durable record. Nonaccepted content is session-only with a 24-hour defensive maximum; consumed-attempt audit is minimal, non-content, and expires after 30 days. Cleanup-failure metadata persists only until successful idempotent cleanup, then resumes normal expiry.

### 16. Source correction and deletion lifecycle

- **A:** cascade-delete every dependent Phase 4 hypothesis and all content-bearing provenance when any included source is deleted; apply the same purge to corrected old revisions.
- **B:** retain the user-accepted hypothesis text as visibly invalidated, purge deleted/corrected source text and the content-bearing packet snapshot, retain only a non-content provenance tombstone, and let the user delete the invalidated hypothesis too.
- **C:** retain the full invalidated hypothesis, old exact source revisions, and complete packet snapshot after source correction or deletion.
- **Recommendation:** B. It best balances local-first user ownership with continuity of the user's accepted reflection. If exact erasure simplicity is preferred over continuity, choose A. Do not choose C without separate explicit revision-content retention consent.
- **Privacy and auditability risks:** A gives strongest erasure but destroys accepted reflective work and exact audit context; B preserves user-owned meaning but loses forensic reconstruction of deleted source text; C gives strongest reconstruction while undermining correction/deletion by keeping hidden duplicates.
- **Founder question:** Do you choose **16A cascade-delete dependents**, **16B retain invalidated accepted text but purge source/packet content**, or **16C retain the complete invalidated artifact and packet**?
- **Founder answer (2026/07/16):** **16B approved as the source correction/deletion design policy; implementation is not authorized.** Retain the accepted reflective artifact as visibly invalidated, purge corrected/deleted source content and content-bearing packet copies, retain only non-content provenance tombstones, exclude the artifact from all future context, and preserve the user's option to delete it completely.

### Accepted package

The founder accepted the following coherent safety posture on 2026/07/16:

- Phase 3 exit approach: **Alternative B** — complete design now and block implementation until Phase 3 exits.
- Founder decisions: **1B, 2B, 3B, 4B, 5B, 6A, 7B, 8B, 9B, 10B, 11B, 12A, 13A, 14B, 15B, 16B**.

This package means: tentative hypotheses are allowed; recurrence/change require three independent manually selected Experiences; contradiction may use two exact sources only as a tension; eligible artifacts remain narrow; counter-evidence and alternatives are mandatory; Phase 4 receives new one-shot consent; persistence requires explicit acceptance, with an additive normalized schema as the approved design direction only; rejected or unaccepted output content remains session-only; minimal non-content attempt tombstones expire after 30 days; corrected/deleted source text is purged from packet snapshots while accepted hypothesis text remains visibly invalidated; all current Phase 3 exit gaps block implementation; and automated plus founder-manual gates are required.

The main costs and residual risks are:

- implementation waits for the Phase 3 exit foundation;
- exact disclosure and review may create consent fatigue;
- packet snapshots duplicate sensitive content locally;
- a three-source floor may still create false confidence if sources are not independent;
- provider variation may require refusal rather than feature parity;
- a richer lifecycle and revision model increases deletion, export, and migration complexity.

Any future change to this accepted combination requires a new explicit founder decision naming the changed threat posture. No relaxation or Roadmap deferral is implicit.

## Implementation Entry Gate

Production implementation must not begin until all are true:

1. the founder chooses a Phase 3 exit alternative and resolves Decision 13 — **satisfied 2026/07/16 with Alternative B and Decision 13A**;
2. all sixteen founder decisions are recorded — **satisfied 2026/07/16**;
3. ADR-0010 moves from `Proposed` to `Accepted` — **satisfied 2026/07/16**;
4. the architecture document moves from `Proposed` to `Founder-approved` without claiming implementation — **satisfied 2026/07/16**;
5. the exact schema, migration transaction, cleanup transaction, rollback procedure, retention-job implementation, and production verification plan receive separate explicit approval; Decisions 11B, 15B, and 16B already settle the design direction and must not be reopened implicitly;
6. the implementation prompt names one smallest vertical slice and preserves Phase 3A/3B behavior;
7. verification and founder-manual acceptance criteria are fixed before code changes.
