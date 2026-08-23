---
status: Founder-approved
version: 0.5
owner: product-and-engineering
last_updated: 2026/08/18
depends:
  - docs/00_Constitution.md
  - docs/03_Principles.md
  - docs/06_Memory.md
  - docs/Reflection.md
  - docs/09_AI.md
  - docs/10_Privacy.md
  - docs/11_MVP.md
  - docs/12_Roadmap.md
  - docs/product/00_MVP_User_Flow.md
  - docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md
  - docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md
  - docs/adr/ADR-0010-govern-cross-experience-reflection-as-user-owned-hypothesis.md
  - docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md
  - docs/architecture/08_Local_Historical_Context_Selection_Foundation.md
  - docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md
  - docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md
  - docs/architecture/11_Cross_Experience_Reflection_Design_Gate.md
  - docs/architecture/12_Phase_3C_Revision_Lifecycle_Provenance_and_Export_Foundation.md
  - docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md
  - docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md
referenced_by:
  - docs/00_Index.md
---

# 16 Phase 3 Product Exit And Private Alpha Readiness Audit

## Purpose And Authority

This **Founder-approved** Book One audit reconciles the production schema-v4 product,
private schema-v5 evidence, Phase 3 exit criteria, and readiness for bounded
Founder dogfooding or later Private Alpha preparation.

It does not authorize implementation, schema activation, migration, backup,
restore, provider or consent changes, Phase 4, packaging, distribution,
deployment, or release. Passing tests does not change those states. The Founder
remains the final authority.

Founder approval accepted the bounded conclusions and next-step direction below.
The reviewed audit package was later promoted through feature commit
`af3f673d` and non-fast-forward merge `c7e767a`. Promotion did not authorize
Private Alpha distribution, deployment, or release.

## Founder Acceptance And Independent Manual Evidence

The Founder accepted this audit on 2026/08/12 after a bounded live walkthrough
performed across 2026/08/11 and 2026/08/12. The walkthrough is new independent
evidence. It does not retrospectively change the archived Daily Reflection
Completion UX R2 state, which truthfully remains `manual_ui.status = not_run`.

The accepted decisions are:

1. Life OS may begin bounded Founder dogfooding on schema v4. This is not
   Private Alpha readiness, schema-v5 production readiness, deployment, or
   release authority.
2. A distributable Private Alpha is not ready. Only bounded Founder dogfooding
   is accepted; external distribution, deployment, and release remain blocked.
3. Cross-Experience remains a distributable Private Alpha blocker under the
   current source of truth. Phase 4 implementation or a Roadmap scope revision
   requires a separate Founder gate after bounded dogfooding evidence.
4. Windows Founder Dogfooding Package R1 is the single next implementation
   direction. Implementation requires a separate explicit authorization after
   this audit package is promoted; distribution, deployment, and release remain
   unauthorized.
5. The R2 manual-record gap is closed only for this audit by the new bounded
   walkthrough. The old workflow archive is not rewritten.

The live walkthrough also found and Founder-accepted four bounded corrections:
journey-summary deduplication and collapse behavior, historical relevance-origin
disclosure, historical preflight cancel/adjust focus restoration, and collapsed
timeline keyboard focus visibility. They were implemented, verified, manually
accepted, and promoted as part of feature commit `af3f673d` and merge
`c7e767a`; they are not deployed or released.

## Evidence Baseline And State Vocabulary

Intake was performed from feature branch
`codex/phase-3-product-exit-private-alpha-readiness-audit` at promoted merge
`76bc4addd954cd14a4ab82f3e4a2369efaab8820`. Its parents are
`c329edb80e0ab3bd5bcbcc4dedd33c9c3af6bf12` and
`a8dd26b0652fd284f6d18fd726a87577b79e9b32`; local `develop` and
`origin/develop` matched. The tree and index were clean, workflow validated
idle, production `SCHEMA_VERSION` was 4, the Constitution was unchanged, and
canonical intake verification passed.

The completed audit package contains 87 promoted files. Git records feature
commit `af3f673d` and non-fast-forward merge `c7e767a`; the latter is the
current Founder-package sprint baseline. These promotion facts do not imply
deployment, release, Private Alpha readiness, or schema-v5 authority.

R2 is implemented by feature commit `a8dd26b0652fd284f6d18fd726a87577b79e9b32`
and promoted by the merge above. It is not deployed or released. Its archived
workflow state records `manual_ui.status = not_run`. This audit does not infer
manual acceptance from promotion or rewrite that historical record. The new
2026/08/11-12 bounded Founder walkthrough independently closes the current
audit's R2 manual-evidence gap.

The following words are intentionally distinct:

| State | Meaning |
| --- | --- |
| Designed | A document or contract describes behavior. |
| Founder-approved | The Founder accepted a bounded decision; this is not implementation. |
| Implemented privately | Code exists only behind private/unregistered disposable seams. |
| Production-reachable | The current desktop runtime can invoke the behavior. |
| Automated-verified | Repository tests passed for the stated boundary. |
| Founder-manually verified | A bounded live review is explicitly recorded. |
| Promoted | A reviewed diff was committed and merged to `develop`. |
| Deployed or released | An installable build was intentionally distributed; no audited feature has this state merely because it is promoted. |

## Central Conclusion

1. **Bounded Founder dogfooding may begin on schema v4.** The primary daily
   reflection journey, local persistence, reviewed current-state artifacts,
   bounded history, exact consent, and provenance inspection are coherent
   enough for a single informed Founder who accepts the limitations below.
2. **Private Alpha preparation may begin, but a distributable Private Alpha is
   not ready.** Installation/startup evidence, complete artifact portability,
   complete lifecycle controls/history, and the current Roadmap's
   cross-experience requirement remain open.
3. **Schema v5 is not required to validate current daily-reflection value.** It
   is required for the already accepted append-only revision/lifecycle model,
   normalized dependency/tombstone inspection, and complete export v2. Private
   fixture evidence is not production activation authority.
4. The lowest-risk next user-value step is a **Windows Founder Dogfooding
   Package R1**, not v5 activation or Phase 4. It closes the access/install
   barrier while keeping current data semantics explicit and unchanged.

## Phase 3 Exit Evidence Matrix

The two tables share capability IDs. `Exit` means the current Phase 3 criteria
in `docs/11_MVP.md`, `docs/12_Roadmap.md`, ADR-0010's Phase 3 entry dependency,
and ADR-0011. `Dogfood` and `Alpha` mean blocking for bounded Founder use and a
distributable Private Alpha respectively.

### Actual behavior and missing integration

| ID | Capability and user value | Source-of-truth requirement | Production schema-v4 behavior | Private/disposable schema-v5 evidence | Missing production integration |
| --- | --- | --- | --- | --- | --- |
| E1 | Experience create/edit/delete | MVP core flow; Roadmap source control | Typed Rust commands; edit/delete invalidate source-scoped artifacts and governed history | Append-only correction, source consequences, parent deletion, migrated baseline parity | v5 routing and revision-history UI |
| E2 | Context Recovery persistence/review | Context Before Insight; low-burden recovery | Suggested local-mock turn, answer, skip persist per Experience; no standalone correction/delete | Create, answer, skip and migrated-successor parity; historically ineligible | Normalized runtime routing; optional future lifecycle controls |
| E3 | Evidence generate/correct/confirm/reject/delete | Evidence before Conclusion; ADR-0007 | AI/local fallback candidates; pending edit; confirm; rejection removes durable candidate; parent edit/delete removes bundle | Candidate and confirmed lifecycle, correction, deletion, purge, exact dependent consequences | Confirmed correction/delete UI and v5 runtime routing |
| E4 | Reflection generate/answer/skip/correct/delete | Reflection before Answer; durable user response | Generate from confirmed Evidence; answer/skip; answered response can be edited and saved; parent mutation removes bundle | Full prompt/response lineage, answer correction/deletion, Pattern/Historical Question consequences, migrated parity | Standalone answered delete/history UI and v5 routing |
| E5 | Pattern generate/confirm/reject/correct/delete | Revisable hypothesis; no identity finalization | One-Experience candidate from exact current context; confirm/reject; no confirmed edit/delete | Candidate and confirmed lifecycle, exact dependency sets, correction/reconfirm/delete | Confirmed lifecycle UI and v5 routing |
| E6 | Historical candidate retrieval/explanation | Selective, explainable retrieval | Explicit-open local lexical v2; keyword overlap; session-only inclusive saved-date filter; caps unchanged | Not a schema-v5 requirement | Theme/emotion/relationship/value retrieval remains unimplemented and sensitive taxonomy needs a separate gate |
| E7 | Historical source selection | Selection is not consent | Exact-source include/exclude is ephemeral per panel/session | Exact revision dependency model exists privately | No durable selection by design; none needed for current boundary |
| E8 | Governed consent/transmission | ADR-0009 | Exact-content preflight; one-generation/one-purpose consent; OpenAI/Gemini; no silent fallback; stale revalidation | Normalized creation parity and lifecycle consequences | v5 production routing only; current v4 path is complete for Phase 3B task |
| E9 | Historical Reflection Question persistence/deletion | ADR-0009 actual-use provenance | Persists neutral source-citing questions, exact packet, consent/transmission refs/dependencies; explicit or source cascade deletion | Exact v4/v5 creation parity and cascade evidence | Normalized runtime routing; no Phase 4 conclusion generation |
| E10 | Provenance inspection | ADR-0007/0009; inspectability | P1 read-only inspector for one already-loaded Historical Question packet; second reveal for outgoing content | Normalized graph contract and lifecycle data exist privately | Full cross-artifact revision/review/tombstone/dependency inspector |
| E11 | Exact invalidation/deletion | Correction/deletion/source removal | v4 Experience edit/delete removes its source artifacts and affected Historical Questions; Evidence rejection removes dependent Reflection and Patterns | Exact direct/transitive invalidation and ADR-0009 cascades extensively verified | Full normalized runtime routing and user-facing consequence preview/history |
| E12 | Export | User ownership; ADR-0007/0011 | Accurately labelled Experience-only JSON/Markdown | Export-v2 contract/design only; production flag disabled | Complete graph serializer, atomic file contract, UI, manual parity |
| E13 | Import | Current data portability boundary | Atomic duplicate-skipping Experience-only JSON import | Migrated baseline/current-action parity tests | Complete artifact import is explicitly unapproved and separately designed later |
| E14 | Revision/rejection history | ADR-0011 append-only lifecycle | Current-state JSON payloads; rejected Evidence/Pattern text is removed; no complete user-visible history | Append-only revisions/events/tombstones/guards verified privately | Production v5 migration/routing/inspector |
| E15 | Multilingual parity | EN/Traditional Chinese/Japanese consistency | Main flow, readiness, history, consent, provenance copy and tests cover three locales | Future migration copy specified, not activated | Native-language dogfooding and release-level manual matrix remain |
| E16 | Provider parity/fallback | Provider-independent Harness | OpenAI/Gemini share Product Harness; local mock keeps journaling/reflection usable; Phase 3B has no silent provider fallback | No additional provider work required | Live release smoke evidence for both providers; credentials remain BYOK |
| E17 | Evaluation coverage | Harness regression and safety | 310 Vitest and 189 Rust tests plus workflow/integration suites; Phase 4/sensitive output rejection | Deep disposable migration/lifecycle/failure coverage | Governed long-term dogfooding dataset and release-environment evidence |
| E18 | Error/stale work | Fail closed; no hidden overwrite | Generation snapshots and serialized expected revision; database incompatibility blocks store; readiness is read-only | Boundary failure injection, rollback, ambiguous commit classification | Production v5 restart/recovery routing; crash-level package evidence |
| E19 | Packaging/installation | MVP Private Alpha criterion | Tauri bundling configured; development startup and production frontend build pass | No schema relevance | No verified Windows installer lifecycle; Windows/macOS release artifacts not both verified |
| E20 | Startup/crash/recovery | Local-first safety | v2/v3/v4 startup supported; newer/malformed DB refuses; readiness reports sidecars/unknown quiescence without repair | Backup/migration/restart/restore contracts verified on disposable fixtures | Production backup/migration/recovery activation and packaged crash/restart evidence |
| E21 | Daily reflection journey/completion | Primary user value | R1/R2 provide low-friction composer, one active step, deterministic completion review, optional Pattern/history, session-only presentation | Not a schema-v5 requirement | R2 manual acceptance is not recorded in repository; bounded confirmation remains |
| E22 | Cross-experience analysis | Current `docs/11_MVP.md` Private Alpha list; ADR-0010 | Not implemented; Phase 3B only asks neutral comparison questions | Phase 4 is design-approved only | Separate Phase 4 authority after Phase 3 entry conditions or explicit Roadmap revision |

### Classification, deferral risk, and smallest safe closure

| ID | Current state | Exit | Dogfood | Alpha | Risk of deferral | Smallest safe closure |
| --- | --- | --- | --- | --- | --- | --- |
| E1 | Reachable, verified, promoted; current-state only | Partial | No | Yes | Users cannot inspect prior corrections | v5 migration/routing plus revision inspector after separate gate |
| E2 | Reachable, verified, promoted | Satisfied for current action set | No | No | Limited recovery editing | Defer standalone lifecycle until observed user need |
| E3 | Reachable basic review; private full lifecycle | Partial | No with disclosure | Yes | Confirmed mistakes require parent edit/delete | Production lifecycle routing and controls after v5 gate |
| E4 | Reachable answer/skip/correct; private deletion history | Partial | No | Yes | No standalone deletion/history | Production lifecycle routing and controls after v5 gate |
| E5 | Reachable review; private full lifecycle | Partial | No | Yes | Confirmed hypothesis cannot be independently revised/deleted | Production lifecycle routing and controls after v5 gate |
| E6 | Reachable lexical/date retrieval | Partial | No | Conditional | Lexical misses and noise remain | Dogfood v2; separately gate any sensitive structured taxonomy |
| E7 | Reachable and intentionally ephemeral | Satisfied | No | No | Re-selection cost only | No closure required |
| E8 | Reachable, verified, manually reviewed in prior Phase 3B gate | Satisfied | No | No | Provider policy drift | Release smoke and provider disclosure review |
| E9 | Reachable and exact | Satisfied | No | No | v4 graph remains specialized | Keep v4 until v5 routing is authorized |
| E10 | Reachable partial inspector | Partial | No | Yes | Most ordinary artifact lineage remains opaque | Full v5 graph inspector after routing |
| E11 | Reachable coarse cascades; private exact lifecycle | Partial | No with disclosure | Yes | Broad v4 deletion hides lifecycle explanation | v5 routing plus consequence disclosure |
| E12 | Experience-only | Blocker | No with explicit warning | Yes | Artifact lock-in and incomplete ownership | Complete export v2 after v5 lifecycle authority; do not invent partial v4 completeness |
| E13 | Experience-only atomic import | Partial | No | Conditional | Artifact import not restorable | Keep accurately labelled; design import only after export v2 |
| E14 | Private only | Blocker | No with disclosure | Yes | No honest audit trail for corrections/rejections | Production v5 lifecycle and inspector |
| E15 | Reachable/automated; partial manual | Partial | No | Yes | Unnatural or divergent release copy | Bounded three-language package walkthrough |
| E16 | Reachable/contract verified | Satisfied | No | Conditional | Live provider/account policy failures | Package smoke with local mock; later opt-in provider smoke |
| E17 | Strong automated, weak longitudinal/product evidence | Partial | No | Yes | Tests may not predict reflective value | Structured no-telemetry dogfooding protocol |
| E18 | Reachable v4 fail-closed; private v5 only | Partial | No | Yes | Packaged crash/permission behavior unknown | Package/startup/crash evidence first; v5 recovery later |
| E19 | Configured, not install-verified | Blocker for Alpha | No | Yes | Founder remains tied to dev tooling | **Windows Founder Dogfooding Package R1** |
| E20 | Safe v4 startup; no production v5 recovery | Partial | No | Conditional | No tested package restart/crash envelope | Package R1 for v4; separate v5 activation gate later |
| E21 | Reachable/verified/promoted; independent manual evidence recorded by this audit | Satisfied | No | Conditional | Later UX regression could go unnoticed | Continue bounded dogfooding; preserve the historical R2 archive |
| E22 | Designed only | Blocker under current MVP wording | No | Yes under current wording | Delays longitudinal value; premature work risks oracle behavior | Keep Phase 4 deferred or separately revise Alpha criterion through Founder authority |

## Product Walkthrough Audit

| Area | Evidence-based assessment | Readiness consequence |
| --- | --- | --- |
| First use and startup friction | `DailyReflectionComposer` leads with one small Experience action; diagnostics are secondary. | Suitable for bounded dogfooding. |
| One-step progression | `dailyReflectionJourney.ts` derives Evidence, Reflection, completion, and optional Pattern from loaded records; navigation does not generate. | Strong current-flow clarity; independently Founder-confirmed in the 2026/08/11-12 walkthrough. |
| Completion feedback | `ReflectionCompletionReview` uses saved source, confirmed Evidence, user responses, skips, and optional existing artifacts only. | Honest rest point; no AI summary or completion pressure. |
| Authorship clarity | Candidate status and P1 provenance are visible, and stored records carry provenance. Ordinary cards do not expose the full lifecycle graph. | Adequate for Founder dogfooding; partial for Alpha trust. |
| Correction/rejection/deletion | Pending Evidence edit/reject is visible; answered Reflection can be resaved; Experience edit/delete is visible and cascades. Confirmed Evidence/Pattern and ordinary artifact standalone delete are not production controls. | Major disclosed Alpha limitation. |
| Provider failure | Raw Experience remains local; generation falls back to local mock with calm status. Phase 3B never silently changes destination. | Suitable for dogfooding; packaged live smoke still absent. |
| Restart | Entries and source artifacts reload from SQLite; R2 presentation and panel state are recomputed/session-only. | Suitable on v4; package restart evidence absent. |
| History | Every open Experience has a top entry point; keyword/date filters are local; relevance is visible. | Findable and bounded; lexical quality needs real use. |
| Consent | Preflight summarizes exact sources, destination, purpose, content and second-level detail; selection is not consent. | Strong Phase 3B boundary. |
| Language | EN/zh-TW/ja copy and parity tests cover the promoted surfaces. | Bounded manual package review remains. |
| Narrow window and keyboard | R2 focus helpers and component tests cover top-of-stage focus and navigation; production window has minimum dimensions. | Needs packaged manual check; no accessibility certification is claimed. |

No telemetry or silent dogfooding collection is proposed. Product value should
be recorded by the Founder deliberately outside the application until a
separately governed evaluation-data policy exists.

## Dogfooding And Private Alpha Readiness

### Founder dogfooding on schema v4: ready with explicit limitations

The Founder can safely use the current desktop development app for ordinary
single-user reflection because schema-v4 CRUD and reviewed current-state
artifacts are production-reachable, failures block or fall back without losing
the raw Experience, history use is explicit, and Phase 4 is absent. The Founder
completed and accepted the compact R2 confirmation in this audit.

The Founder must understand that export is Experience-only, confirmed ordinary
artifacts lack complete independent lifecycle controls/history, local lexical
retrieval is not semantic understanding, and no cross-experience conclusion is
available.

### Distributable Private Alpha: not ready

Blocking evidence is missing for Windows installation/startup/restart/uninstall
and for a release support boundary. Complete artifact portability and honest
append-only revision/lifecycle inspection are also open. Finally,
`docs/11_MVP.md` currently lists cross-experience analysis as a Must Have Before
Private Alpha; this audit cannot silently waive that requirement. Private Alpha
preparation may proceed, but distribution requires either satisfying it or a
separate Founder-approved scope revision after dogfooding evidence.

## Alternatives

### Option A — Schema-v4 product completion first

Current v4 can support clearer copy and bounded dogfooding, but cannot honestly
provide ADR-0011's complete append-only history or export v2. A current-state
artifact export would conflict with the accepted complete-portability direction
if presented as closure. Continue v4 for validation, not for simulated v5
guarantees.

### Option B — Production schema-v5 activation first

This would combine quiescence, backup, migration, ambiguous-commit recovery,
read/write routing, UI disclosure, retention, restore, and real-data support.
Private tests are deep but the production integration and manual gates are
explicitly incomplete. The risk is unjustified before the core experience has
bounded dogfooding evidence.

### Option C — Private Alpha packaging first

A Windows-only, non-released Founder package closes the immediate access
barrier and tests the actual installation/startup envelope without changing
data semantics. It must use a disposable/private-alpha identity or profile,
remain unsigned and undistributed, and disclose Experience-only export and
current lifecycle limits. This is the recommended next slice.

### Option D — Begin Phase 4

ADR-0010 is Accepted as design, not implementation authority. Current Phase 3
entry gaps and the absence of dogfooding evidence make Cross-Experience
Reflection premature. More context would increase responsibility, not AI
authority.

### Option E — Another bounded product slice

A lifecycle or export slice would provide direct control but depends on v5
production routing to meet the accepted semantics. Another private fixture
slice would add less user value than making the current safe product installable
for bounded Founder use. No stronger alternative was found.

## Single Recommended Next Slice

### Windows Founder Dogfooding Package R1

**Founder-approved and separately authorized for bounded R1 implementation.**
Keep production schema v4. Add one Windows-only
local build path that produces an unsigned,
non-distributed Founder package with a distinct private-alpha application
identity/profile so installation tests cannot open or mutate the ordinary
development database. Verify clean install, first start, normal schema-v4 daily
reflection, restart reconstruction, local mock fallback, controlled app close,
and uninstall behavior in a disposable Windows account or sandbox. Produce a
content-free build manifest and checksum, but no updater, signing, telemetry,
cloud service, migration, backup/restore activation, or release automation.

Expected surfaces are limited to Tauri package configuration override,
repository build/verification scripts, package-specific factual documentation,
and test fixtures. Product React/Rust/SQLite/provider/ContextPacket/consent code
should not change. Any need for such a change returns to a Founder gate.

Automated acceptance:

1. Base `tauri.conf.json`, product identifier, and schema-v4 runtime remain
   unchanged; the private package override is explicit and reproducible.
2. Package build succeeds on Windows and outputs only ignored build artifacts.
3. Manifest contains build version, Git SHA, target, artifact filename, size,
   and SHA-256 but no user path, content, secret, or database metadata.
4. Static checks prove no updater, network deployment, signing secret,
   migration, backup/restore, schema, provider, consent, or Phase 4 change.
5. Canonical verification remains green before and after the package build.

Founder manual matrix:

1. Confirm the installer identifies itself as a private Founder dogfooding
   build and does not replace the development app.
2. Install in a disposable Windows account/profile; acknowledge the unsigned
   local-build warning without distributing the artifact.
3. Start from the installed shortcut and confirm the empty-state Experience
   composer appears without a console or development server.
4. Complete one local-mock Experience → Evidence → Reflection → completion
   journey; keep Pattern/history optional.
5. Close normally, restart, and confirm saved schema-v4 records reconstruct
   while session-only panels remain closed.
6. Confirm Experience-only export is labelled accurately and no Upgrade,
   migration, backup, restore, updater, telemetry, or release control appears.
7. Repeat the core disclosure in Traditional Chinese and Japanese and inspect a
   narrow window plus keyboard focus.
8. Uninstall the private package; record whether its disposable local data is
   retained or removed, without touching the development profile.

The slice stops at Founder diff and manual install review. It does not stage,
commit, push, merge, deploy, distribute, or release without separate authority.

### R1 implementation state

Windows Founder Dogfooding Package R1 is implemented in the current feature
working tree as an additive Tauri override, repository build/verification
scripts, focused contract tests, ignored review output, and a content-free
manifest. It remains unpromoted and not Founder-manually accepted until the
separate diff and install review completes. The ordinary Tauri identifier stays
`com.lifeos.app`; the Founder package uses `com.lifeos.founderdogfood`, so the
relative `sqlite:life-os.db` resolves under a distinct Tauri application config
directory. Production `SCHEMA_VERSION` remains 4.

## Copy-Ready Next `/goal`

```text
/goal Complete Life OS Windows Founder Dogfooding Package R1 autonomously.

Repository:
D:\Lin\Project\LifeOperatingSystem

Begin from clean develop only after architecture/16 and this exact slice are
Founder-approved. Use the existing repository Engineering Harness sequentially.
Do not create a worktree.

Implement only one Windows-only, unsigned, non-distributed Founder dogfooding
package path for the current schema-v4 product. Use a distinct private-alpha
application identity/profile so install and runtime checks cannot open or
mutate the ordinary development database. Preserve the normal Tauri config,
product identifier, production SCHEMA_VERSION = 4, provider behavior,
ContextPacket, consent, and all current product semantics.

Scope:
- an explicit Tauri Windows package configuration override;
- a deterministic repository script that builds the local package;
- a content-free manifest containing version, Git SHA, target, artifact name,
  size, and SHA-256;
- automated checks proving separation from the development profile and absence
  of updater, deployment, signing secrets, schema/provider/consent/Phase 4
  changes;
- factual Book One documentation;
- canonical verification, Theory Alignment Review, workflow archive/reset;
- stop at Founder diff and a step-by-step manual install/start/restart/uninstall
  review.

Do not add an updater, telemetry, cloud service, signing secret, auto-start,
migration, fresh-v5 initialization, backup/restore activation, retention,
provider or ContextPacket changes, consent changes, Phase 4, macOS packaging,
public distribution, PR, deployment, or release. Use only disposable Windows
test data. Do not touch real user data.

Before implementation, verify repository truth and present an exact file
allowlist. If a product React/Rust/SQLite/provider/consent file is required,
stop at human_decision_required rather than expanding scope.

Run focused checks and:
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1

Leave build artifacts ignored. Stop before stage, commit, push, merge, PR,
deployment, distribution, or release. At the end, guide the Founder through the
eight-step manual matrix in architecture/16 one step at a time.
```

## Founder Decisions

All five requested decisions were explicitly accepted on 2026/08/12 and are
recorded in `Founder Acceptance And Independent Manual Evidence`. Their scope
does not include promotion or implementation authority.

## Scope Confirmation

The Constitution, Book Zero, ADR status, production code, schema, provider,
ContextPacket, consent policy, historical behavior, Phase 4, Engineering
Harness contract, deployment, and release are unchanged. This document is
Founder-approved and promoted through `af3f673d` / `c7e767a`. Windows Founder
Dogfooding Package R1 is a later separately authorized implementation working
tree and is not promoted, distributed, deployed, or released by this document.
> 2026/08/13 factual note: the next bounded direction selected here now has an
> implemented but unpromoted isolated schema-v5 Founder candidate in
> architecture/17. Private Alpha distribution remains unauthorized.
>
> 2026/08/18 factual note: the isolated Candidate was subsequently
> Founder-manually accepted and promoted. Architecture/18 records the current
> unpromoted ordinary desktop activation implementation. Neither state
> authorizes Private Alpha distribution, deployment, release, Phase 4, or
> migration of the Founder's real ordinary profile.
