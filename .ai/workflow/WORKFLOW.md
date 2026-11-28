# Life OS Sprint Workflow

## Purpose

This is the repository-mediated lifecycle for one bounded multi-role sprint. It
controls engineering work; it does not define product theory or store runtime
Life OS data.

`WORKFLOW_CONTRACT.json` is the machine-readable vocabulary and transition
contract. `EVENTS.jsonl` is the append-only current-sprint transition journal.
Use `node scripts/ai-workflow.mjs` for state transitions; do not hand-edit the
JSON state or journal to bypass validation.
The current implementation and future event-driven gate are documented in
[`architecture/14`](../../docs/architecture/14_AI_Orchestration_Evolution.md).

Write the substantive Markdown review first, including its active Sprint ID and
completed required sections. Then use `record-artifact` to reconcile its Status
into state and the event journal before requesting the next transition. Every
mutating command requires the currently observed event sequence so stale work
fails closed.

## State Vocabulary

`WORKFLOW_STATE.json.status` must be one of:

- `idle`
- `intake`
- `product_review`
- `awaiting_product_clarification`
- `engineering_planning`
- `implementation`
- `validation`
- `theory_alignment_review`
- `revision_required`
- `human_decision_required`
- `completed`
- `failed`
- `cancelled`

`current_phase` uses the same phase names, except an idle state may keep
`intake` as the next phase. Artifact statuses are defined by their templates and
must not be substituted for workflow statuses.

## Allowed Transitions

| From | Allowed next state |
| --- | --- |
| `idle` | `intake` |
| `intake` | `product_review`, `human_decision_required`, `cancelled`, `failed` |
| `product_review` | `engineering_planning`, `awaiting_product_clarification`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `awaiting_product_clarification` | `product_review`, `human_decision_required`, `cancelled` |
| `engineering_planning` | `implementation`, `product_review`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `implementation` | `validation`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `validation` | `theory_alignment_review`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `theory_alignment_review` | `completed`, `revision_required`, `human_decision_required`, `cancelled`, `failed` |
| `revision_required` | `product_review`, `engineering_planning`, `implementation`, `human_decision_required`, `failed`, `cancelled` |
| `human_decision_required` | the recorded blocked phase, `cancelled`, or `failed`, after an explicit founder response |
| `completed`, `failed`, `cancelled` | none; `archive` creates the terminal archive and resets a new idle projection |

No transition may bypass the evidence needed by the destination phase. Intake
cannot transition directly to implementation or completion.

### Authorized completed-state follow-up

An already completed sprint whose candidate changed after terminal verification
cannot use an ordinary transition or the resolved-decision refresh. The bounded
`start-terminal-follow-up` operation may return only a `completed` sprint to
`validation` when all of the following are supplied and revalidated:

- the exact sprint ID, repository HEAD, branch, and current event sequence;
- an explicit authorization reference that appears verbatim in a named,
  repository-contained evidence file; and
- a concrete reason for the follow-up.

The operation validates the existing event chain before writing, appends one
`terminal_follow_up_started` event, preserves the full terminal coordinates and
the prior verification/decision evidence, and marks repository verification
pending. It does not mark manual UI review, theory review, completion, archive,
promotion, or a later product phase as passed. State and event writes are
atomic and roll back together. The normal validation, theory-review, and human
decision gates apply afterward.

## Phase 0: Intake

1. Inspect branch, HEAD, upstream, status, untracked files, recent history, and
   diff.
2. Confirm no unfinished sprint would be overwritten.
3. Create a unique sprint ID and complete `CURRENT_MISSION.md`.
4. Change its Status from `draft` to `ready` only after every `[required]`
   field is resolved, then record it with `workflow:record-artifact`.
5. Record actual repository coordinates in `WORKFLOW_STATE.json`.
6. Route relevant primary definitions, ADRs, architecture, product, development,
   code, and tests.
7. Complete a Product Review for every sprint using this multi-role workflow.
   A routine mechanical change that does not benefit from role-separated review
   may use the ordinary Engineering Harness instead of opening a multi-role
   sprint; once this workflow is started, Product Review is not skipped.

## Phase 1: Product Review

Apply the Chief Product Theorist role and complete `PRODUCT_REVIEW.md`.

- `approved` -> Engineering Planning.
- `approved_with_conditions` -> Engineering Planning with every condition copied
  into the plan.
- `revision_required` -> revise the mission or review.
- `human_decision_required` -> stop and populate `DECISION_REQUIRED.md`.
- `rejected` -> cancel the sprint unless the founder supplies a new mission.

## Phase 2: Engineering Planning

Apply the Senior Product Engineer role and complete `ENGINEERING_PLAN.md` before
editing. The plan must cite the approved product boundary, affected modules,
alternatives, data and lifecycle effects, tests, verification, recovery,
documentation, and escalation result. If implementation would change product
meaning, return to Product Review; if it needs founder authority, stop.

## Phase 3: Implementation

Implement the smallest approved change. Preserve unrelated work, avoid
speculative scaffolding, add focused tests, synchronize factual documentation,
and complete `ENGINEERING_REPORT.md`. Do not merge, push, deploy, or take an
unapproved destructive action.

## Phase 4: Validation

The orchestrator independently inspects the actual diff, test files, command
output, architecture synchronization, ADR implications, migrations, runtime
claims, and git state. Reports must be corrected if they disagree with
repository reality. Run the canonical verification path. Record manual checks as
passed only when actually performed.

### Resolved-decision verification refresh

`record-verification` remains limited to `validation`. A resolved
`human_decision_required` gate may use the narrower
`refresh-verification-for-resume` operation only when its recorded
`resume_phase` is `theory_alignment_review`. The command requires the current
event sequence, the exact stored decision evidence reference, `passed` status,
exit code `0`, and the canonical repository verification command. It records
the current HEAD, branch, and non-workflow working-tree digest in one explicit
`resume_verification_refreshed` event and the state projection.

The operation leaves the workflow at `human_decision_required`, preserves the
active decision IDs, exact resolution evidence, and resume phase, and rolls
back the event/state pair on a write or validation failure. It cannot be used
for an unresolved decision, another resume phase, failed/skipped verification,
or arbitrary post-decision state rewriting. The subsequent `transition` still
requires the recorded decision reference and fresh repository evidence; this
refresh grants no new Founder authority.

## Phase 5: Theory Alignment Review

### Scoped manual verification registration (Engineering Harness follow-up, 2026/11/03)

Use `node scripts/ai-workflow.mjs record-scoped-manual-verification` with
`--sprint-id`, `--expected-sequence`, `--evidence`, `--evidence-sha256`,
`--acceptance-source direct-founder-chat`, `--founder-response` and `--scope`.
It registers a previously observed and explicitly accepted Windows, synthetic,
mock, current-user candidate journey in an active validation/theory-review/human
decision state. It does not run an app, infer acceptance or complete/archive.

The repository-contained JSON request has format
`life-os/scoped-manual-verification-v1`, `sprintId`, `acceptanceSource`, exact
`scope` and `scopeQuote`, and a `target` object (`reviewSuffix`, `identity`,
`platform: windows`, `dataClass: synthetic`, `providerMode: mock`,
`installMode: current-user`). Its `evidence` holds `{path, sha256}` references
for `receipt`, `sourceBinding`, `installer`, `manifest`, `ledger`, `supplement`.
All paths are repository-relative, realpath-contained, existing files; mutable
current workflow files cannot serve as authorization evidence.

This narrow receipt adapter accepts existing explicit scoped Founder chat
acceptance receipts: `decision: ACCEPTED`, `exactFounderResponse`, exact
`acceptedScope`/`precedingFounderScopeQuote` and
`evidenceSource: Founder explicit chat reply after scoped acceptance clarification`.
It validates all candidate/source/package/ledger/supplement cross-bindings,
accepted source and protected file bytes, and referenced raw receipt hashes.
Both reported and directly observed steps and explicit limits are retained.
An agent recommendation, silence, host PASS or a hash is NOT human authority:
the caller must have the direct Founder reply and identify it explicitly;
the CLI cannot cryptographically authenticate a human author.

Optional `manual_ui.scoped_evidence` is a backward-compatible extension; old
not_run records/archives remain unchanged. `completed_at` and `registered_at`
are the CURRENT registration time, with the historical acceptance receipt time
separate. Prior manual verification and old decision resolution are preserved.
The new hash-chained event contains the full scoped projection; validation
requires that exact event/projection match, without reopening ignored artifacts
when verifying an archive. Missing/stale/malformed/mismatched inputs and all
duplicates/conflicts refuse without writes. No dry-run option is provided.

Writes reuse the existing `appendEventAndState` atomic-file/paired rollback
mechanism and validation rollback. The existing single-writer boundary remains;
this does not add locks, concurrent leases or a new storage protocol. Fault
injection covers both event and state rename failures. Archive copies the scoped
state/event and formal Markdown summary before the usual validated idle reset.
No acceptance standard or other completion gate is relaxed.

Apply the Chief Product Theorist role to the actual diff and completed
Engineering Report. Complete `THEORY_ALIGNMENT_REVIEW.md` against acceptance
criteria, Constitution, primary definitions, ADRs, user agency, privacy,
psychological safety, provenance, consent, lifecycle, and scope.

A `rejected` Theory Alignment Review routes to `failed` with evidence, or to
`cancelled` when the founder cancels. It is not silently converted into an
approval or an unbounded revision.

## Phase 6: Bounded Revision

For `revision_required`:

1. increment `review_cycle` exactly once;
2. require a concrete transition reason so the append-only event journal records
   the failed criterion, evidence, owner, and required correction;
3. append the cycle to `THEORY_ALIGNMENT_REVIEW.md` only when the correction
   originated in theory alignment; earlier product or plan corrections remain
   visible in the event journal and final Sprint Report;
4. route to the smallest responsible phase;
5. validate again and repeat theory review.

The maximum is three cycles. A fourth correction attempt is forbidden. If the
remaining issue is consequential or ambiguous, stop at
`human_decision_required`; otherwise mark `failed` with reproducible evidence.

## Phase 7: Completion Or Stop

Complete `SPRINT_REPORT.md`, reconcile the JSON state, and report one terminal
result. `completed_with_follow_up` is a report-level result represented by JSON
status `completed` plus documented deferred items. Completion never implies
commit, push, pull request, merge, deployment, or release.

Run `pnpm workflow:archive` to promote terminal artifacts under
`HISTORY/<sprint-id>/` before resetting the current files to their idle
templates. The archive must exclude secrets and runtime user data. Clear
`DECISION_REQUIRED.md` only after its decision is resolved or the terminal state
records why it remains unresolved.

## Human Decision Package

Every escalation includes observed evidence, affected high-authority sources,
available options, benefits, risks, reversibility, data/privacy impact,
orchestrator recommendation, safe default, blocked files or phases, and the
exact founder response needed. Silence is never approval.

The state records `blocked_phase`, `resume_phase`, active decision IDs, and the
decision-resolution evidence reference. `DECISION_REQUIRED.md` preserves the
exact founder response, selected option, authorized scope, response time, and
resume phase before work continues. Role approval never changes an ADR or
architecture decision state and never grants implementation authority that a
governing document explicitly withheld.

## Control-Plane Persistence

Current `.ai/workflow/` artifacts are tracked repository control-plane metadata,
not product data. The working-tree digest excludes current workflow artifacts to
avoid self-referential hashes, while including mission code, documentation,
roles, templates, contracts, and unrelated untracked files. Intermediate
workflow artifacts may be committed only when the task authorizes a commit.
They never authorize merge, push, deployment, or protected-branch actions.

The event journal uses monotonic sequences, previous-event hashes, idempotency
keys, repository HEAD, branch, and working-tree digest. Terminal archives remain
required before reset and are created through a temporary-directory promotion
plus validated reset. Cross-machine leases remain outside the current kernel
until separately implemented and verified.

## Recovery After Interruption

Treat the repository as authoritative: compare the state file and artifacts with
the actual branch, HEAD, diff, and check output. Resume from the last phase whose
evidence can be independently verified. Record discrepancies rather than
guessing. Never reset, discard, or overwrite unrelated changes to force a clean
state.

## Independent sprint disposition and bounded recovery (2026-11-28)

`cancel-sprint` is an explicit disposition path for an **unresolved**
`human_decision_required` sprint. It does not use `resolve-decision` and does
not relax ordinary `transition` (including `failed` or unresolved resume).
It requires an independently authorized Founder disposition; merely reading
this documentation, a template, a hash, a host PASS or a recommendation grants
no authority. The tool checks integrity and exact scope, not human identity.

The repository-relative JSON evidence has format
`life-os/sprint-disposition-v1` (see the
[unfilled template](../templates/sprint-disposition.template.json)). It must
bind the exact `sprint_id`, integer `expected_sequence`,
`disposition: cancelled`, `scope: cancel-current-sprint-only`,
`authorization_source: direct-founder-chat`, a nonempty
`authorization_reference`, `exact_founder_response`, and the exact
`retained_constraints` object in that template. The command requires the same
sprint, sequence, scope and response, plus SHA-256 of the evidence's exact bytes.
Paths must be existing repository-contained files; current mutable workflow
files cannot serve as authorization. Duplicate cancellation refuses.

The optional `sprint_disposition` state field and `sprint_cancelled` hash-chained
event contain the same disposition, raw evidence bytes/hash and complete
pre-cancellation state checkpoint. Validation checks the projection and event
prefix. Original events, decision Markdown, reports, active decision IDs,
blocked/resume phases, blocking reason, last completed phase and verification
history remain unchanged. Historical reports may still say
`human_decision_required`: the separate disposition records how the sprint
ended. This is cancellation, not completed work, PASS, product rejection,
resolved schema, ADR acceptance, or permission for restricted capabilities.
An unresolved gate in the checkpoint remains unresolved after disposition.
No existing history is required to invent this optional evidence.

### Archive format and interruption protocol

New archives have manifest schema `2.0`, the existing `files` list and terminal
coordinates, and `hashes` for every payload file. They retain all current
artifacts, the complete event/verification history and contract. A new
cancellation archive also contains `CANCELLATION_CHECKPOINT.json`,
`SPRINT_DISPOSITION.json`, and the exact `SPRINT_DISPOSITION_EVIDENCE.json`.
Manifest metadata is bound by the operation record hash; it cannot contain
its own hash. An archive records external raw logs as `not_acquired` because
this operation does not read or copy external log locations. Embedded old
verification references are history, not proof that unavailable logs were
archived. Existing manifest `1.0` archives remain untouched and readable;
existing completed-sprint archive invocation remains supported.

Cancellation and archive/reset use only one operation's persistent recovery
record at `.ai/workflow/.governance-operation/operation.json`. It contains fixed
before/after bytes and hashes. Recovery requires its exact SHA-256. Preparation
validates the destination images in a disposable isolated candidate before
publishing the record. Archive preparation also validates a complete terminal
archive and idle candidate **before changing any current file**. Copy and
publication verify all archive bytes; publication precedes every reset write.
The journal blocks all ordinary CLI mutations and `start`, even if a partial
reset already looks idle. `validate` reports recovery required; `status` remains
read-only. This record is not an alternative mission or a parallel workflow.

Recovery completes the recorded forward operation only. Before any publication
or control-plane write it checks all current files against the known before or
after image, verifies the record and its internal hashes, validates both
candidates, and checks staged/published archives for unknown files or changed
hashes. For cancellation, the after disposition checkpoint must deeply equal
**the complete before state**, including historical verification. The after
event bytes must retain the exact before event prefix and append only one
legal `sprint_cancelled` event, with a single boundary newline if the old log
was unterminated. Both images being independently valid, or supplying a newly
matching operation hash, cannot replace these cross-image checks (R3-GOV-01).
Contradictions refuse before any current-file write and preserve pending and
all recovery materials. It refuses unknown content, conflicts, malformed records, duplicate
archives and unexpected operation files. There is no force, discard, automatic
rollback or arbitrary-path recovery. On a second interruption or error retain
the record and every known recovery image. Successfully finishing moves the
record to `.governance-receipts/<operation-id>/`; it never deletes the only
recovery copy. No command starts a mission as a side effect.

This remains a local, single-writer operation, without a multi-task scheduler,
concurrent lock/lease protocol or general backup framework. Tests cover Linux
process termination (`SIGKILL`) and injected exceptions; file fsync and atomic
renames do not establish cross-platform power-loss durability. A crash during
initial record creation can leave `.governance-preparing-*`; mutations refuse
and the incomplete/unknown copy is preserved for independent review, with no
automatic removal. Unknown or damaged recovery content requires independent
review; do not hand-edit state, delete recovery files, or force a new mission.

### Future authorized operation runbook — not executed by this maintenance

1. Obtain a separate explicit authorization for real disposition, archive/reset,
   and any recovery. Review the tool candidate independently first. Recheck
   exact HEAD, index/worktree, single writer, current sprint/sequence,
   unresolved schema, ADR status, protection hashes, reports and historical
   verification. Do not execute against a copied real control plane as a test.
2. Save actual independent Founder disposition evidence in an appropriate
   repository-relative evidence file. Fill no field by inference; compare its
   bytes, scope and reference to the authorization. Keep a read-only inventory
   of the original control plane and archives outside the repository.
3. Invoke the official tool with explicit bindings, only after that authorization:

   ```text
   node scripts/ai-workflow.mjs cancel-sprint --sprint-id <exact-id> --expected-sequence <n> --evidence <relative-json> --evidence-sha256 <sha256> --founder-response <exact-text> --scope cancel-current-sprint-only
   node scripts/ai-workflow.mjs validate
   ```

4. Compare original event prefix, decision, reports and verification, inspect
   the new disposition/checkpoint, then invoke `node scripts/ai-workflow.mjs
   archive` only within the independently authorized archive/reset scope. Check
   each manifest hash and the validated idle result; retain external evidence.
5. If interrupted, stop other writers and preserve the full operation directory.
   Read and independently inspect `operation.json`; compute its SHA-256 and
   compare it to the intended operation/sprint/sequence and before/after
   inventory. With separately applicable recovery authorization use:

   ```text
   node scripts/ai-workflow.mjs recover-governance --operation-sha256 <exact-record-sha256>
   node scripts/ai-workflow.mjs validate
   ```

   Refusal means preserve evidence and stop for review. Successful reset is
   idle only. A new mission requires its own explicit scope, fresh reviews and
   verification; it inherits no R3 PASS, unresolved-decision resolution or A-F1
   product authority. Do not run `start` merely because reset succeeded.
