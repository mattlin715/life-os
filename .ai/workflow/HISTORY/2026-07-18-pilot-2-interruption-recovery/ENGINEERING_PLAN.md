# Engineering Plan

Status: approved

- Sprint ID: 2026-07-18-pilot-2-interruption-recovery
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: af2d8736a20cc1820dacc566e25c5accd6b41d59
- Working-tree digest reviewed: bbfef1ccad16efa4b8a4ae327e727f94e81cbe365ed28ebc8a8f64997390c840
- Created at: 2026-07-17T17:24:51Z
- Updated at: 2026-07-17T18:10:00Z

## Approved Product Boundary

The Product Review is `approved_with_conditions`. This plan preserves every
condition:

- repository facts, not chat context, are the recovery authority;
- a state/event mismatch must fail closed and report evidence, never guess or
  repair;
- event-ahead and state-ahead scenarios may be exercised only in disposable
  copied fixtures;
- the live workflow state and journal must never be corruption fixtures;
- detection must not be described as safe repair or arbitrary crash recovery;
- no recovery command, replay, distributed lock, lease, concurrency mechanism,
  Stage 2/3 capability, product/runtime/schema/provider change, or
  architecture/13 work is in scope;
- implementation, commit, push, merge, deployment, and promotion remain
  separately controlled. This block stops after recording this plan.

Review cycle 1 adds one bounded kernel correction after a real `EPERM` exposed
that `writeAtomic` leaves its temp file on rename failure and that the
event/state pair lacks centralized write-failure rollback. This does not
authorize repair of an unknown mismatch. It only restores the exact originals
captured by the command whose own write failed.

## Existing Implementation Understanding

Repository-only recovery independently established the following checkpoint:

- branch `codex/orchestration-pilot-2-interruption-recovery` at HEAD
  `af2d8736a20cc1820dacc566e25c5accd6b41d59`, with no configured upstream;
- only the four active workflow artifacts were modified and
  `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`
  remained unrelated and untracked;
- revision 4 was `product_review`, with Product Review
  `approved_with_conditions`;
- event `2026-07-18-pilot-2-interruption-recovery:0004` had sequence 4 and hash
  `2aa7f62b1e297aff291f870712ca3754efd9741914dc171bbd28bdb98e8f033d`;
  an independent canonical re-hash matched;
- `node scripts/ai-workflow.mjs validate` passed before transition;
- expected sequence 4 transitioned through the repository script to
  `engineering_planning` at revision 5, and the transition reason recorded the
  recovery coordinates.

`writeAtomic` replaces each file atomically, but `appendEventAndState` writes
`EVENTS.jsonl` before `WORKFLOW_STATE.json`. A process interruption can
therefore leave an event-ahead projection. A state-ahead projection is not the
normal order of that function, but remains a useful fail-closed integrity
fixture. `validateWorkflow` already checks event count against state revision,
state last-event identity/hash against the journal tail, and state status/sprint
against the journal tail. Command-level rollback restores prior contents only
when the process remains alive to execute the rejection path; there is no
autonomous crash reconciliation by design.

The existing workflow test suite passed 13 of 13 tests. It covers copied-fixture
isolation, invalid transitions, artifact/state disagreement, rollback after a
rejected live command, decision resume, verification, and archive/reset. It
does not contain direct event-ahead or state-ahead torn-projection regression
cases.

A disposable OS-temp evaluation, using copied workflow artifacts and leaving
the live control plane unchanged, confirmed both projections are currently
rejected:

- event ahead: revision 4 versus 5 events, last-event mismatch, and
  status/sprint mismatch;
- state ahead: revision 5 versus 4 events, last-event mismatch, and
  status/sprint mismatch.

This proves current detection for these two synthetic projections. It does not
prove repair, arbitrary interruption recovery, or multi-writer safety.

## Affected Modules

Initial implementation scope was limited to:

- `scripts/ai-workflow.node-test.mjs` — add two focused disposable-fixture
  regression tests.

Review-cycle-1 implementation scope adds:

- `scripts/ai-workflow.mjs` — temp cleanup, pair rollback, and a test-only
  deterministic atomic-rename failure injection guarded by `NODE_ENV=test`;
- `scripts/ai-workflow.node-test.mjs` — first-write and second-write failure
  regression tests.

No workflow contract, product code, runtime storage, documentation,
architecture decision, or architecture/13 file is an implementation target.

## Proposed Design

In a separately authorized implementation phase, add two tests that use
`withGitWorkflow` and the real CLI to generate a valid pre-transition
`product_review` checkpoint and a valid post-transition
`engineering_planning` checkpoint:

1. Event-ahead test: combine the post-transition `EVENTS.jsonl` with the
   pre-transition `WORKFLOW_STATE.json`, call `validateWorkflow`, and assert the
   revision-count, last-event, and status/sprint mismatch diagnostics.
2. State-ahead test: combine the post-transition `WORKFLOW_STATE.json` with the
   pre-transition `EVENTS.jsonl`, call `validateWorkflow`, and assert the same
   three classes of diagnostics.

Both tests must operate only under the temporary directory created by the
existing test harness and must clean it in `finally`. Generate the projections
through the existing CLI rather than duplicating production hashing logic or
exporting new production internals solely for tests.

The initial plan required no change to `scripts/ai-workflow.mjs` because the
validator already rejected both synthetic mismatches. The real `EPERM` evidence
invalidated that assumption for command-owned temp cleanup and pair rollback;
autonomous repair remains out of scope.

### Review Cycle 1 Corrective Design

1. Wrap the temporary write plus rename in `writeAtomic` with cleanup. On any
   failure, remove only the temp path created by that invocation and rethrow the
   primary failure without replacing the target.
2. Add an internal, process-local rename-attempt counter. Inject a deterministic
   failure only when both `NODE_ENV=test` and
   `LIFE_OS_AI_WORKFLOW_TEST_FAIL_ATOMIC_RENAME_AT=<n>` are set. This is not a
   CLI option or workflow capability.
3. Make `appendEventAndState` capture the original serialized event journal and
   state before either replacement. If either write fails, attempt to restore
   both originals with fresh atomic writes, then rethrow. If rollback also
   fails, preserve the primary error and include rollback evidence; validation
   remains fail closed.
4. Do not promote or reuse the orphan candidate sequence 14. After verifying
   its exact path, size, and SHA-256 against Product Review evidence, delete
   only that orphan and recompute the repository snapshot.
5. Add two CLI regression cases from a valid revision-4 temporary checkpoint:
   fail atomic rename attempt 1 and attempt 2. Each asserts a non-zero command,
   unchanged original events/state, no `*.tmp-*` orphan, and valid workflow.
6. Rerun targeted and canonical verification. All earlier Pilot 2 evidence
   remains in the event chain; the terminal report must add cycle-1 evidence.

## Alternatives Considered

- Rely only on the operational disposable-fixture evaluation: rejected because
  the behavior would remain unprotected against regression.
- Hand-build and hash a synthetic event in the test: rejected because it would
  duplicate production canonicalization and could test the fixture builder
  more than the workflow CLI.
- Export internal event-building helpers for tests: rejected because it would
  enlarge the production module surface without a product need.
- Add a recovery or replay command: rejected as outside Stage 1 and contrary to
  the approved fail-closed boundary.
- Change to a two-file transaction, lock, or lease: rejected as disproportionate
  and outside this pilot.
- Ignore the real orphan and retry completion: rejected because the orphan
  changes the repository digest and would contaminate new evidence.
- Delete the orphan without a formal revision: rejected because it would hide
  the operational failure rather than preserve its provenance.

## Data Lifecycle Impact

None. The planned tests use synthetic workflow metadata in disposable OS-temp
directories and delete it after each case. No Life OS user data is read,
persisted, changed, or transmitted.

## SQLite Or Migration Impact

None. No SQLite code, schema version, migration, or runtime database is in
scope.

## Provenance Impact

No product provenance impact. The tests strengthen evidence that the workflow
projection and append-only journal must agree before progress is allowed.

## Historical Context Impact

None. No historical Life OS context is selected, assembled, or read.

## Consent Impact

None. No user-content consent contract is involved.

## Provider Transmission Impact

None. No provider call or payload is involved.

## Import And Export Impact

None. No Life OS import or export path changes.

## Test Strategy

The initial implementation added exactly the two focused mismatch tests.
Review cycle 1 adds exactly two atomic-write fault tests. Each fault test must
invoke the real CLI in a temporary copied repository and assert:

- the transition exits non-zero;
- original event and state bytes are restored;
- no workflow temp files remain;
- `validateWorkflow` returns no errors after the failed command.

The full workflow suite must remain green. No live
`.ai/workflow/WORKFLOW_STATE.json` or `EVENTS.jsonl` contents may be used as a
fault fixture.

## Repository Verification Strategy

After separately authorized implementation:

1. run the focused Node workflow test suite;
2. run `node scripts/ai-workflow.mjs validate` against the live control plane;
3. run the canonical
   `powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1`;
4. inspect `git diff`, `git status`, the event tail, and workflow revision to
   prove architecture/13 and unrelated work remain untouched.

This planning block runs no implementation verification claim beyond the
already reported 13-test baseline, disposable-fixture evaluation, and workflow
validation.

## Manual UI Verification

Not applicable. This is repository control-plane test coverage with no UI or
runtime behavior change.

## Rollback Or Recovery Strategy

The initial tests remain reversible. The corrective kernel change is reversible
through an ordinary reviewed diff, but the real failure evidence must remain in
the sprint archive. The verified orphan is removable only after exact evidence
checks. If cleanup or rollback touches any path other than its own temp or the
captured event/state pair, stop and report the exact diff.

## Documentation Impact

None planned. The Product Review and this Engineering Plan already state the
current implemented, tested, operationally observed, and intentionally absent
behaviors. A later factual document update would require separate evidence and
scope.

## ADR Impact

No new ADR and no ADR status change. Two regression tests apply the existing
Accepted ADR-0008 repository-owned governance and the implemented Stage 1
fail-closed boundary in architecture/14 without changing either decision.

## Risk Level

Medium-low. The correction changes workflow-kernel failure handling but not its
state machine or authority. Risks are rollback failure, fault injection leaking
outside tests, and masking the primary filesystem error. Guard injection with
`NODE_ENV=test`, preserve the primary error, keep validation fail closed, and
cover both write positions.

## Escalation Decision

No founder-controlled decision is required. The user's instruction authorized
the Harness to continue through subprocess-assisted development, and Product
Review approved this bounded correction. Record this revised plan before
entering implementation; commit, push, merge, deployment, Stage 2, and Stage 3
remain unauthorized.
