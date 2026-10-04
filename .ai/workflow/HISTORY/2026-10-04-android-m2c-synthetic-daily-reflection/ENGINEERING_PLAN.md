# Engineering Plan

Status: approved

- Sprint ID: 2026-10-04-android-m2c-synthetic-daily-reflection
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: ae36fb6ae77fd6da582831a5dd122d81082f20f9
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-10-03T22:30:53.697Z
- Updated at: 2026-10-04T18:23:40.760Z

## Approved Product Boundary

Current PRODUCT_REVIEW approved under UX001. Presentation only; no new generation/review/persistence/provenance/consent authority. Historical initial implementation remains preserved in baseline copies.

## Existing Implementation Understanding

Source editor and candidate correction already use distinct canonical immutable request paths. Reuse them exactly. Only displayed copy, labels, accessibility attributes and CSS change. Initial candidate is exact quoted source; correction makes a pending revision; explicit adoption remains required before Reflection. Existing optional exact content/provenance disclosure remains.

## Affected Modules

Only AndroidM2CApp.tsx,journeyCopy.ts,ReflectionJourney.tsx,android-m2c.css; corresponding existing tests, android-m2c-contract.node-test.mjs and native CDP probe; architecture24/runbook15 and workflow Markdown. No source store/provider/domain/Rust/schema/config/icon change; current46-file allowlist stays bounded.

## Proposed Design

Add contextual source edit label/hint; separate Clues and reflection heading; Candidate clue · Local demo plus visible scope note; distinct correct/adopt/do-not-use labels and new-pending-revision editor help. Keep actual record.text exact, collapse only exact repeats/reviewed text as before. Auto-focus source/candidate textarea with associated label and help. No handler, hook, request, gate or persistence edits.

## Alternatives Considered

Do not fake v4 receipt or call migrated runtime; do not invent artifact schema, renderer persistence or overwrite SQL; do not copy broad desktop provider/history APIs; do not leave M2-B blanket refusal for legitimate M2-C. Explicit direct-origin adapter is the smallest semantically equivalent path.

## Data Lifecycle Impact

None relative accepted revision50. Pin accepted behavior blocks, all JSX input/mutation handlers, source store/shared Harness/provider/domain and Rust adapter/writers. Original generation question payload strings also pinned.

## SQLite Or Migration Impact

Canonical exact schema5/directFreshV5 only, fixed app-private filenames/receipt and no fake migration. No DDL/schema/desktop activation change.

## Provenance Impact

Mock candidate/prompt preserve local_mock/mock/model:null/shared Harness/Prompt versions and exact dependencies. User correction lineage and user response provenance remain distinct; confirmation never changes authorship or certifies fact.

## Historical Context Impact

None; no history/recovery/Pattern paths or packets, all unsupported rows/dependencies refused before mutation.

## Consent Impact

Explicit local user decisions only; drafts/cancellation no mutation. No implicit review or provider consent.

## Provider Transmission Impact

None; no INTERNET/provider/BYOK/imported transport. Bundle/static/native inspection.

## Import And Export Impact

None; unsupported and no commands/UI.

## Test Strategy

Extend existing language/SSR tests and Node contracts without removing existing assertions. Actual native source editor direct opening/cancellation, candidate review/correction/adoption leave full source snapshot unchanged; new revision stays pending. Preserve full reflection, rejection/skip/invalidation/draft/stale/duplicate/fault tests. Verify focus and one primary exact source plus optional details in all3 languages.

## Repository Verification Strategy

Baseline passed .artifacts/android-m2c/baseline-verification-result.json. Final canonical scripts/verify.ps1, Android debug build/inspection/native; final APK SHA256/non-workflow digest/exact paths/evidence after last changes.

## Manual UI Verification

Fresh Founder diff review and short6-step source edit -> candidate inspect -> correction -> adopt/reject -> optional response -> save/reopen. Earlier9/9 cannot transfer; failure evidence preserved without repair.

## Rollback Or Recovery Strategy

No rollback/reset authorized. Accepted raw46 copies with .baseline suffix, byte-identical accepted APK and old AVD/evidence are preserved. Any unexpected drift or essential policy change stops with evidence. Use separately owned new AVD5590 and evidence-mapped derived harness, preserve existing5586/5588.

## Documentation Impact

architecture24/runbook15 version0.4 presentation-only correction and revised manual gate, explicit historical acceptance and unchanged runtime boundaries. Workflow via official terminal-follow-up and artifact/transition CLI only; no kernel edits.

## ADR Impact

None. Accepted0007/0011/0012 unchanged; human adoption is not independent fact verification.

## Risk Level

high: lifecycle/provenance/purge transaction routing; mitigated by canonical writers, strict graph refusal, immutable reconciliation, rollback/reread and native tests; synthetic-only.

## Escalation Decision

Proceed only with UX001 scope. Stop at new Founder gate or preserve failure if verification exposes an unexpected/runtime defect.

## Authorized test-only synchronization follow-up

Founder directly authorized offered ANDROID-M2C-UX-NATIVE-SYNC-001 Option A with exact response「授權 test-only 同步修正與重新驗證」. Only native readiness guard, regression safeguard/evidence/docs; all150 original probe assertions unchanged. Wait for exact source ID/body/revision/authorship/lineage, idle journey and enabled controls with30000ms predicate timeout; no arbitrary delay/replacement request/auto-retry. Product/UI/store/Harness/provider/domain/Rust/schema/config/icon untouched. Rebuild/inspect/focused/full affected native/canonical on fresh owned5596/CDP9236 before new manual gate. Original failure cause unconfirmed, no retrospective proof claim.
## Founder003 Option C — exact implementation plan

Authoring role: senior_product_engineer. Implement only pure presentation helpers before pinned currentEvidence block, generated-button label expression and inactive rejected row grouping in ReflectionJourney.tsx; new EN/zh-TW/ja copy keys in journeyCopy.ts. Existing hooks, event/input handlers, generation predicate and frozen request path must remain byte-identical. Inactive Evidence/rejected rows only; preserve all other statuses. Optional native details list every existing rejected row and all returned non-content snapshot fields, excludes any payload/text. Add focused SSR/pure-helper tests, Node exact render-contract safeguard without replacing old assertions, and one dedicated native rejection-presentation probe with three-language repeated create/reject/new-ID/zero purged-content/unchanged original/disclosure-only invariance/processing lock checks. Existing native150 assertions and full31 groups are unchanged. New tracked safeguard scripts/android-m2c-rejection-cdp-probe.mjs is the only added candidate path; anticipated exact allowlist47. Architecture24/runbook15 synchronize policy-unchanged presentation and latest results. Build/inspect/focused/full native/canonical on absent new5598/CDP9238 AVD/run20261004134814; existing5588/5596 and all old failed/diagnostic AVDS inventory-only. Derived ignored harness only root/ports/authority mapping plus additive scenario/reset within new owned fixtures. No old-profile replay/repair/clear/reinstall. Failure preserves evidence and stops. Cycle3/max3/kernel unchanged; no automatic extension.

## Canonical failure exact-path synchronization

Direct CANONICAL-FAIL001 Option A permits only one full ACTIVE_SUCCESSOR_ALLOWLIST entry and one additive package contract test with positive/neighbor-negative cases. Preserve all existing source guard body, prefixes, allowlist members, test assertions and product/APK/native instrumentation bytes. Affected package/M2C contracts followed by full canonical on updated exact digest. Existing34/34 native bound only after raw evidence/runtime/APK checks; no profile operations or test replay. Old canonical exit1/log/receipt/bookkeeping refusal and83/1 preserved. Return Founder004 exact diff/manual gate with all6 fresh/not_run. Any new failure preserves/stops, cycle3/max3/kernel unchanged, unstaged/no acceptance/promotion.

## Authorized integration closeout engineering plan

Sequential Senior Product Engineer plan: A preserve47 raw accepted files and canonical product bytes; B change only current factual status in architecture24/runbook15 and workflow reports; C add exactly .ai/workflow/HISTORY/2026-10-04-android-m2c-synthetic-daily-reflection/ to existing HISTORY_PREFIXES plus additive exact/adjacent negative test, archive/reset via official CLI. Run affected package contracts/full canonical before terminal completion, revalidate archived bytes and idle, full canonical after archive, stage only actual normalized allowlist. Commit final candidate/archive/reset work, normal feature push, no-ff exact feature merge into develop, inspect both parents, full canonical then normal develop push. No APK build/launch/emulator/profile access. No automatic retries/source corrections; preserve partial state and stop on failure.
