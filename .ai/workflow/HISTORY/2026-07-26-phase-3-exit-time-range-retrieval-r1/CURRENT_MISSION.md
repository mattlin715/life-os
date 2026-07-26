# Current Mission

Status: ready

- Sprint ID: 2026-07-26-phase-3-exit-time-range-retrieval-r1
- Mission title: Phase 3 Exit Structured Retrieval R1
- Origin: founder_request
- Base branch: develop
- Starting commit: 5714b3eeeeb4c9612e445dbd78e4f606df41fa27
- Background: Phase 3A already provides explicit-panel, local-only, bounded lexical retrieval and ephemeral exact-ID selection. The accepted Phase 4 gate still identifies structured retrieval as a blocking Phase 3 exit gap. Slice 3B migration-restart evidence was promoted at the starting commit, while production schema support remains v4.
- Problem: The historical-context panel cannot yet let the user explicitly narrow locally retrieved lexical candidates by a chosen time range. Separately, architecture/13 contains stale pre-promotion wording about the already-promoted Slice 3B.
- Intended outcome: Correct only the confirmed Slice 3B factual drift, evaluate one user-visible local time-range retrieval vertical slice, and stop at `human_decision_required` with decision `PHASE3-RETRIEVAL-R1-001` before any production implementation.
- Initial scope: Repository evidence review; minimal architecture/13 factual correction; Product Review; exact temporal contract; alternatives, risks, acceptance criteria, and Founder decision package.
- Explicit non-scope: Production implementation before Founder approval; Harness expansion; schema/provider/ContextPacket/persistence changes; inferred event dates or taxonomies; Phase 4; staging, commit, push, merge, PR, or deployment.
- Relevant Book Zero definitions: `docs/06_Memory.md` (selective, explainable historical retrieval and a user-chosen time range); `docs/03_Principles.md` (Context Before Insight, agency, bounded Context Recovery); `docs/Reflection.md` (user-owned cross-time meaning); `docs/09_AI.md` (Context Steward); `docs/10_Privacy.md` (ongoing control and selective context).
- Relevant ADRs: ADR-0009 (selection/consent separation and bounded historical use); ADR-0010 (structured retrieval remains a Phase 3 exit dependency; Phase 4 stays blocked); ADR-0011 (lifecycle/provenance design does not authorize schema-v5 activation).
- Relevant architecture documents: architecture/08, architecture/09, architecture/11, architecture/12, and architecture/13.
- Relevant code areas: `src/historicalContext/`, historical panel/preflight state in `src/app/App.tsx`, `src/app/i18n.ts`, `src/styles.css`, storage interfaces only for impact inspection, and existing historical-context/evaluation tests.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-26T14:06:04.116Z
