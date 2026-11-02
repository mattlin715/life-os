# Engineering Plan

## CURRENT AUTHORIZED FOLLOW-UP: scoped manual verification registration — 2026/11/03

Sprint ID: 2026-10-28-desktop-reflection-provenance-r1.
Founder explicitly authorizes a bounded Engineering Harness repair and subsequent
canonical verification, official closeout/archive and precise LOCAL commit.
Accepted27-file candidate source/protected bytes must remain unchanged. Existing
candidate acceptance is NOT acceptance of this new harness code. No app/profile/
process/account/provider/Android036 action, rebuild, Git integration or cleanup.
No Book Zero/ADR/product-policy change is needed or authorized (ADR-0008 applies).

Sequential product boundary review: APPROVED for engineering-metadata recording
only; no acceptance criteria/gates relaxed. Engineering plan: add one CLI command
using existing appendEventAndState rollback; validate explicit direct Founder
acceptance/scope plus cross-bound receipt/ledger/supplement/package/source; preserve
prior resolution and historical not_run; optional scoped projection stays backward
compatible. Reject duplicates/conflicts and stale/invalid/missing/hash-drift input.
Test isolated Git fixtures first, including write failure, legacy and archive.
After focused green and sequential source review, register existing real evidence.
Then fresh canonical, normal gates/archive, explicit staged diff and local commit.
If any exact gate remains blocked, preserve facts and STOP, never weaken checks.
Earlier STOP/acceptance sections below are historical and remain unchanged.

## Reflection projection-order contract and boundary — 2026/10/31

Sprint ID: 2026-10-28-desktop-reflection-provenance-r1. Actual product payload
capture is pinned in reflection_projection_ordering.json. Rust replay red001
exited101 for BOTH first multi-question creation and append to Q1; rereads prove
the complete batch committed in SQLite order before postcondition mismatch.
Raw DBs, payloads, output and pre-fix runtime/test snapshots are preserved under
`.artifacts/reflection-projection-ordering-r1/run-001/`.

Contract: ReflectionPrompt has no position/rank, and relevant Book One/Accepted
ADR-0007/0011 clauses require stable identities/provenance, not generation-array
order. persisted_artifacts.id and created_at are TEXT with default BINARY collation
in the fixed v4 base retained by v5. The canonical writer binds prompt.id and
prompt.created_at to BOTH payload keys and columns, without parsing/reformatting
time. Read SQL is ORDER BY created_at, id (ascending); equal timestamps tie-break
by the complete ID. These are stored-text/projection keys, not chronological
instant comparison or locale order. Different textual timestamp precision is
not representation-equivalent merely because it denotes the same instant.

Choose the typed runtime boundary, not frontend or adapters: it owns the actual
SQLite read contract for all Reflection commands. Before mutation, sort the full
proposed Reflection array using SQLite itself on the supplied JSON TEXT keys with
explicit BINARY collation in the same DB connection environment. Do not read a
post-write actual projection into desired. Do not duplicate sort comparators,
parse dates, alter IDs/timestamps/content, relax full Value comparison, or change
writer/schema/transaction/single-action/provenance/retention policy. Add explicit
Reflection read checks that column id/created_at match payload id/createdAt.

Ordering alone must not mask pre-existing create-projection inconsistencies:
validate each new suggested projection against the promoted DTO mapping and
compare the full expected bundle before the writer. Reject contradictions,
unknown fields and noncanonical nested provenance/source representation rather
than normalize them. Keep the accepted four absent/null response fields exactly
as before. Bound the adjacent audit to Reflection presentation; do not refactor
other artifact writers or lifecycle invalidation. Review source and focused
results before canonical; build a new suffix only if no in-scope blocker remains.

## Authorized additive-generation follow-up plan — 2026/10/31

Sprint ID: 2026-10-28-desktop-reflection-provenance-r1. This addendum is approved
under the separately recorded Founder authorization, not old diff acceptance.
Preserve all baseline/diagnostic evidence. Extract and wire the real App handler;
run red tests before semantic repair. Implement a synchronous per-source flight,
fresh durable ContextPacket, exact local Reflection/relevant-input precondition
outside provider context, queue and adapter rechecks, and atomic full-batch append
without touching any other artifact collection. Focus the newly committed
question, preserve drafts and existing error/readback behavior, and report success
only on confirmed commit. Keep native commands/writers and postconditions fixed.
Test the actual product coordinator and typed adapter with mocked IPC separately
from real Rust/SQLite A/B/correction/deletion and unchanged replacement refusals.
Run affected suites then the unmodified canonical verifier. Build/inspect with a
new ReviewSuffix only after passing. Bind both the incremental correction and
the cumulative candidate to source hashes; no installation, launch, profile,
provider, Git publication or036 actions. Keep workflow human/manual gate open.

Status: approved

- Sprint ID: 2026-10-28-desktop-reflection-provenance-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 7b9e7244ab39bb57f37d3856c366529efc4b25b6
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-10-28
- Updated at: 2026-10-28

## Approved Product Boundary

PRODUCT_REVIEW.md approved_with_conditions: reproduce before fix; preserve strict comparison and canonical writers; no runtime/profile access.

## Existing Implementation Understanding

App uses answerReflectionPrompt then typed save_founder_v5_artifacts. Runtime inferred transitions call canonical writers. Canonical response provenance has four nulls absent from helper. Recovery has an analogous validated normalization.

## Affected Modules

src-tauri/src/schema_v5_runtime.rs (including tests); src/shared/storage/sqlite/founderSchemaV5LocalEvidenceStore.test.ts; if required directly related frontend payload regression. architecture17/runbook09 factual update only.

## Proposed Design

First add failing first-answer/correction runtime regressions using the actual helper serialization. Preserve red raw output and retained synthetic SQLite outcome. Only if proven, validate exact response operation and normalize four allowed omitted/null fields before writer invocation; reject any other mismatch before writing. Keep full postcondition.

## Alternatives Considered

No weakened comparison; no changing canonical writer or shared helper/R1; no Android adapter or new lifecycle.

## Data Lifecycle Impact

None: canonical append-only correction, dependencies and parent deletion remain unchanged.

## SQLite Or Migration Impact

Host fixture databases only. No DDL/schema/receipt/storage-origin/migration implementation change.

## Provenance Impact

Representation equivalence only after user origin, exact source/prompt artifact/time and unchanged prompt fields pass. Contradictory or unknown fields rejected, never silently discarded.

## Historical Context Impact

No new historical behavior. Existing canonical dependent invalidation/cascade unchanged.

## Consent Impact

No new consent or retained content policy. Synthetic host fixtures only.

## Provider Transmission Impact

No provider requests/credentials. Verify future AI_PROVIDER=mock path with synthetic mocks; unsupported explanation and registered remote IPC remain disclosed.

## Import And Export Impact

None; no import/export implementation touched.

## Test Strategy

Focused red then green desktop runtime SQLite and actual frontend helper/adapter shape; first save/correct omitted/null, invalid authorship/relations/time/non-null/unknown/lineage, blank/unchanged/stale/no-write and canonical rollback; close/reopen and source-delete. Preserve red fixtures and stdout/stderr/actual child exits.

## Repository Verification Strategy

Focused then affected suites, full scripts/verify.ps1 with actual direct child exit and raw log. Existing candidate script with new suffix, source/binary/manifest inspection, no installation. Independent local tool copies; no new dependencies.

## Manual UI Verification

Founder is owner; not_run and unauthorized this sprint. Later controlled packaged journey and fresh disposable Windows profile.

## Rollback Or Recovery Strategy

Source-only bounded fix; no runtime rollback/recovery, no retries or DB cleanup. Preserve failures. Only close owned children normally.

## Documentation Impact

architecture17 metadata/factual follow-up and founder runbook09 future dedicated account, process-scoped mock launch and no installation/manual/native proof.

## ADR Impact

No new ADR: no policy/schema/identity/retention/architecture change; accepted ADR statuses untouched.

## Risk Level

medium: acknowledgement after durable writes and shared ordinary/candidate runtime; strict precommit rejection and feature-specific host tests mitigate.

## Escalation Decision

No additional implementation decision. Stop if reproduction fails or policy/storage/identity/guard weakening is needed; final human_decision_required for diff/new account/install checkpoint.
