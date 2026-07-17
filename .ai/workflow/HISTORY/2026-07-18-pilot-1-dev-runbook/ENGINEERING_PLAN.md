# Engineering Plan

Status: approved

- Sprint ID: 2026-07-18-pilot-1-dev-runbook
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: d68a24a2b5e91200b6bbbf43c0593f46d2b8879f
- Working-tree digest reviewed: 26b8e658a1671e711e1563bdc86ce3cb90bac56b64071d40bcc0ca197d63ef2b
- Created at: 2026-07-17T15:21:00Z
- Updated at: 2026-07-17T15:33:00Z

## Approved Product Boundary

Product Review status is `approved_with_conditions` after review cycle 1.
Change only the factual standard-command guidance in `docs/dev/05` and the
temporary-fixture initialization in `scripts/ai-workflow.node-test.mjs`;
preserve architecture/13 and every product/runtime boundary.

## Existing Implementation Understanding

- `scripts/verify.ps1` is the repository-owned deterministic verification path
  and includes the newly promoted workflow checks.
- `package.json` exposes `start:desktop`, which delegates to
  `scripts/start-life-os.ps1` and loads the local toolchain.
- `pnpm run tauri:dev` remains the direct Tauri development loop and is still
  appropriate for the runbook's hidden background procedure.
- `docs/dev/05` currently lists typecheck/build individually and does not name
  the canonical verifier or self-serve desktop command.

## Affected Modules

- `docs/dev/05_Development_Agent_Runbook.md`
- `scripts/ai-workflow.node-test.mjs`
- Current and archived `.ai/workflow/` control-plane artifacts

## Proposed Design

1. Bump the runbook from version 0.1 to 0.2 and update its date.
2. Add `docs/dev/08_Engineering_Harness.md` as a dependency.
3. Replace the standard command sequence with repository-local environment
   activation, dependency installation, canonical `verify.ps1`, and explicit
   `start:desktop` manual launch.
4. Explain that `tauri:dev` remains the lower-level direct development command.
5. Update the expected-result table without changing background launch or stop
   procedures.
6. Add one helper that restores only a copied fixture's current workflow files
   from `.ai/templates` and empties its copied `EVENTS.jsonl`.
7. Invoke the helper in both non-git and git-backed temporary fixtures before
   tests run or the fixture commit is created.
8. Add an explicit regression assertion that a copied fixture is idle even
   while the repository workflow is active.

## Alternatives Considered

- Update every older development document: rejected as too broad for the first
  pilot.
- Replace every `tauri:dev` reference: rejected because the direct command
  remains valid.
- Make no documentation change: rejected because current repository commands
  provide a demonstrably clearer canonical path.

## Data Lifecycle Impact

None. Only engineering documentation and workflow metadata change.

## SQLite Or Migration Impact

None. Schema remains v4; architecture/13 remains untracked and untouched.

## Provenance Impact

No product provenance impact. The workflow event chain records this pilot's
engineering handoffs and final archive.

## Historical Context Impact

None.

## Consent Impact

None.

## Provider Transmission Impact

None.

## Import And Export Impact

None.

## Test Strategy

- Confirm the documented commands exist in `package.json` and the named scripts.
- Run the workflow contract tests and state validation through canonical
  verification.
- Run `pnpm run test:workflow` during the active sprint to prove fixture
  independence before rerunning the canonical verifier.
- Use Markdown link, UTF-8, whitespace, and secret checks from `verify.ps1`.
- Confirm only the runbook plus workflow control-plane artifacts differ.

## Repository Verification Strategy

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

Record the exact successful snapshot during the validation phase before Theory
Alignment Review.

## Manual UI Verification

Not required for this documentation-only pilot. Do not start the desktop app or
claim a Founder UI result.

## Rollback Or Recovery Strategy

The runbook edit is one reversible documentation diff. Workflow interruption is
recoverable from `WORKFLOW_STATE.json`, `EVENTS.jsonl`, and current artifacts.
Do not reset the pre-existing architecture/13 file.

## Documentation Impact

Only `docs/dev/05` changes as documentation. The workflow test harness receives
one fixture-isolation correction. No Index, Book Zero, product, architecture,
or ADR document requires synchronization.

## ADR Impact

No new ADR and no ADR modification. This applies existing ADR-0008 behavior.

## Risk Level

Low. The changes are factual documentation plus test-only fixture isolation;
they are reversible and do not affect product runtime.

## Escalation Decision

No founder escalation required. Proceed with the bounded review-cycle-1 repair.
