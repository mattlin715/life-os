# Engineering Plan

Status: approved

- Sprint ID: 2026-07-18-pilot-3-human-decision-resume
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: f15477271b1d0d1df1c080c1f7b99b2fd96ae477
- Working-tree digest reviewed: 26b8e658a1671e711e1563bdc86ce3cb90bac56b64071d40bcc0ca197d63ef2b
- Created at: 2026-07-17T20:31:58.6149543Z
- Updated at: 2026-07-17T20:31:58.6149543Z

## Approved Product Boundary

The Product Review is `approved_with_conditions`. All six conditions are
binding:

1. The exact Founder Option A response is the complete authority boundary.
2. No implementation or non-workflow documentation file may change.
3. architecture/13 remains untouched and excluded.
4. No autonomous repair, recover command, or event replay may be introduced.
5. Completion/archive grants no commit, push, merge, Stage 2, Stage 3, product
   functionality, migration, or deployment authority.
6. Stop at Founder diff review after archive/reset.

## Existing Implementation Understanding

Stage 1 uses `WORKFLOW_CONTRACT.json`, fixed Markdown artifacts, hash-chained
`EVENTS.jsonl`, projected `WORKFLOW_STATE.json`, and guarded commands in
`scripts/ai-workflow.mjs`. Events `:0008` through `:0011` preserve the exact
founder resolution, the authorized resume to `product_review`, the reconciled
Product Review, and entry to planning. Existing tests already cover decision
gating; this live pilot evaluates repository-only operational use. No code
defect or code change is authorized.

## Affected Modules

- Active workflow artifacts:
  `.ai/workflow/ENGINEERING_PLAN.md`, `ENGINEERING_REPORT.md`,
  `THEORY_ALIGNMENT_REVIEW.md`, `SPRINT_REPORT.md`, `WORKFLOW_STATE.json`, and
  `EVENTS.jsonl`.
- Previously completed Mission, Product Review, and Decision artifacts are
  observed inputs and later archived/reset.
- Terminal archive:
  `.ai/workflow/HISTORY/2026-07-18-pilot-3-human-decision-resume/`.
- `scripts/ai-workflow.mjs` and its tests are read-only observation surfaces.
- architecture/13 is excluded.

## Proposed Design

1. Write and record this plan using the observed expected sequence; validate.
2. Enter the workflow phase named `implementation` only as a
   no-implementation-file control-plane observation.
3. Inspect branch, HEAD, status, diff, state/event chain, exact founder evidence,
   and the exclusion allowlist.
4. Run existing workflow tests and validation without modifying them.
5. Write a truthful Engineering Report and record it.
6. Enter validation, run canonical verification, and record its exact result.
7. Perform theory review of the actual diff and complete the Sprint Report.
8. Transition terminal, archive/reset through the repository CLI, validate
   idle, and stop at Founder diff review.

Each mutation uses the observed expected sequence. No hidden chat state is
needed to resume any micro-block.

## Alternatives Considered

- Remain paused: safe, but loses live resume evidence.
- Use only synthetic tests: insufficient operational proof.
- Skip the `implementation` phase: violates the state-machine handoff.
- Create an implementation or documentation diff: unauthorized and unnecessary.

The proposed design is the smallest authorized path.

## Data Lifecycle Impact

None. Only repository control-plane metadata is affected; no Life OS user or
runtime data is used.

## SQLite Or Migration Impact

None. No SQLite, schema, migration, DDL, backup, compatibility, or database
version behavior changes.

## Provenance Impact

No product provenance impact. Engineering evidence retains event IDs and
hashes, branch, HEAD, digest, exact founder response/reference, verification,
and the terminal archive manifest.

## Historical Context Impact

None. No historical material or Context Packet is assembled.

## Consent Impact

None. Founder workflow authorization is not product or provider consent.

## Provider Transmission Impact

None. No provider/model request or payload is created.

## Import And Export Impact

None. No product import, export, or export-v2 behavior is involved.

## Test Strategy

Add or modify no tests. Run workflow status/validation at every handoff, inspect
state/hash/diff agreement, and execute the existing 17 workflow tests during
the observation phase. Canonical verification in validation must run the full
repository suite. After archive/reset, validate the idle projection and rerun
canonical verification for parity with the promoted pilots.

## Repository Verification Strategy

Verify branch/HEAD, event sequence/hash, working-tree digest, exact decision
evidence, no staged files, the workflow-only diff allowlist, architecture/13
exclusion, absence of product/code/schema/provider/Constitution changes,
`git diff --check`, canonical verification output, archive manifest, and the
reset idle projection.

## Manual UI Verification

Not applicable because no UI or runtime behavior changes. Founder diff review
and any later promotion authorization remain separate gates.

## Rollback Or Recovery Strategy

No product rollback exists. After interruption, trust only the last durable
repository state/event pair and independently validate branch, HEAD, digest,
and artifacts. On mismatch or failed command, stop and preserve evidence; do
not repair, replay, reset, discard, or overwrite. Archive/reset only after a
valid terminal state using the repository command.

## Documentation Impact

Only workflow evidence and its archive. Book Zero, ADR, architecture, product,
development, and navigation documents remain unchanged. The stale `unpiloted`
wording in `docs/dev/08_Engineering_Harness.md` is deferred to a separately
authorized factual correction.

## ADR Impact

No new ADR or ADR change. This is an operational application of Accepted
ADR-0008 and Implemented architecture/14.

## Risk Level

Medium operational-evidence risk but low product/data risk. A workflow error
could invalidate pilot evidence, while no user data or production behavior is
touched. Fail-closed checkpoints and the exact file allowlist bound the risk.

## Escalation Decision

No additional founder decision is needed inside Option A. Stop at
`human_decision_required` if any file change, authority expansion, mismatch,
destructive recovery, code correction, or new consequential choice becomes
necessary.
