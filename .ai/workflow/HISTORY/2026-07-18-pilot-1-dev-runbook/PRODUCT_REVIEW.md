# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-07-18-pilot-1-dev-runbook
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: d68a24a2b5e91200b6bbbf43c0593f46d2b8879f
- Working-tree digest reviewed: 26b8e658a1671e711e1563bdc86ce3cb90bac56b64071d40bcc0ca197d63ef2b
- Created at: 2026-07-17T15:18:00Z
- Updated at: 2026-07-17T15:31:00Z

## Mission Interpretation

Use the first Stage 1 pilot to make one factual, documentation-only correction
to the development agent command path while exercising every required role
handoff. The workflow evidence is part of the pilot; it is not product data.

## Problem Statement

`docs/dev/05_Development_Agent_Runbook.md` predates both the canonical
Engineering Harness verifier and the self-serve `start:desktop` command. Its
standard verification section still presents separate typecheck/build commands
as the main verification path and only names direct `tauri:dev` startup.

## User Value

Contributors receive one current, low-friction path for deterministic checks and
one current desktop startup path, reducing version and command ambiguity.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: higher authority remains unchanged.
- `docs/00_Index.md`: no Book Zero primary definition is affected.
- `docs/dev/08_Engineering_Harness.md`: canonical deterministic verification is
  `scripts/verify.ps1`.

## Relevant ADRs

- `docs/adr/ADR-0006-mvp-tech-stack.md`: preserves Tauri, React, SQLite, pnpm,
  and Rust.
- `docs/adr/ADR-0008-engineering-harness-governance-is-tool-independent.md`:
  shared repository commands remain canonical rather than tool-specific chat
  instructions.

## Current Implementation Context

- Implemented: `scripts/verify.ps1` runs workflow validation, Vitest,
  typecheck, frontend build, SQLite Rust tests, Rust check, repository hygiene,
  links, secrets, and Constitution diff reporting.
- Implemented: `package.json` exposes `start:desktop`, which invokes
  `scripts/start-life-os.ps1` and the repository-local environment loader.
- Implemented: direct `tauri:dev` remains valid for the standard development
  loop and the documented hidden background procedure.
- Proposed but not implemented: architecture/13 schema v5 migration plan; it is
  unrelated and must remain untouched.

## In Scope

- Update only the command guidance and factual metadata in
  `docs/dev/05_Development_Agent_Runbook.md`.
- Correct only the test-fixture initialization in
  `scripts/ai-workflow.node-test.mjs` so copied fixtures start from the idle
  workflow templates even when the repository itself has an active sprint.
- Preserve direct `tauri:dev` guidance where it remains accurate.
- Record and archive the pilot workflow evidence.

## Out Of Scope

Book Zero, Product Harness, runtime code, package scripts, dependencies, schema,
providers, architecture/13, Stage 2/3 orchestration, deployment, and live UI
testing.

## Product Constraints

The runbook must distinguish deterministic repository verification from manual
desktop runtime verification. It must not imply that automated checks prove the
desktop experience.

## Evidence And Provenance Constraints

No product Evidence or artifact provenance is affected. Workflow provenance
must remain repository-local and must preserve the pre-existing untracked
architecture/13 file without absorbing it into this sprint.

## Historical Context Constraints

None. This sprint does not read, select, assemble, or transmit Life OS history.

## Consent Constraints

None. No user content or provider call is involved.

## AI-Role Constraints

The documentation must describe commands factually. It must not expand AI
authority or treat an automated workflow result as founder approval.

## Privacy Constraints

Do not copy `.env.local`, credentials, logs containing secrets, runtime SQLite
data, or personal content into workflow artifacts.

## User-Agency Constraints

Desktop launch remains an explicit contributor action. Manual UI verification
must be reported separately and never fabricated.

## Acceptance Criteria

1. `docs/dev/05` identifies `scripts/verify.ps1` as the canonical deterministic
   verification path.
2. `docs/dev/05` identifies `pnpm run start:desktop` as the easiest self-serve
   desktop startup path while preserving `tauri:dev` for direct development use.
3. Automated verification and manual desktop verification remain distinct.
4. Document version, date, and dependencies are synchronized.
5. Workflow contract tests pass while the repository workflow is active;
   copied test fixtures reset only their own current control plane to idle and
   never mutate the live workflow.
6. No file outside `docs/dev/05`, `scripts/ai-workflow.node-test.mjs`, and
   current/archived workflow control-plane artifacts is changed by the pilot.
7. Canonical verification passes and architecture/13 remains untracked.

## Risks

- Overstating `start:desktop` as mandatory could hide the useful direct Tauri
  loop; mitigate by documenting both purposes.
- Treating `verify.ps1` as UI verification would be inaccurate; preserve the
  manual-runtime boundary.
- Workflow archive output will add tracked control-plane evidence; keep it free
  of secrets and user data.
- A fixture reset that touched the live `.ai/workflow` would corrupt sprint
  evidence; constrain the helper to the temporary copied root and cover it with
  a regression test.

## Open Questions

None.

## Human Decision Required

False. The correction applies already accepted Engineering Harness behavior and
does not change a founder-controlled boundary.

## Recommendation

Approve the bounded documentation correction plus the fixture-isolation repair
and return to engineering planning. This is an Engineering Harness reliability
correction, not an expansion of product or founder-controlled authority.

## Revision Note

Review cycle 1 was opened after canonical verification showed that the workflow
tests inherited the active repository sprint state. The scope is expanded only
to reset temporary copied fixtures from repository templates before each test.

## Review Status

approved_with_conditions
