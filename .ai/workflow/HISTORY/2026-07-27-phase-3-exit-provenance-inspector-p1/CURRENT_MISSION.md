# Current Mission

Status: ready

- Sprint ID: 2026-07-27-phase-3-exit-provenance-inspector-p1
- Mission title: Phase 3 Exit Provenance Inspector P1
- Origin: founder_request
- Base branch: develop
- Starting commit: db43b8f47815b0c6ddb1bd8daa9a9503c11a2146
- Background: Phase 3B already persists Historical Question artifacts with an immutable successful packet snapshot, consent/transmission references, exact source and artifact revisions, and deletion dependencies. The promoted R1 saved-date retrieval slice does not expose this durable actual-use chain beyond a compact provider/model/digest summary.
- Problem: Users can delete a Historical Question but cannot locally inspect what was selected, consented, transmitted, and persisted. That weakens understandable provenance even though the exact evidence already exists in schema v4.
- Intended outcome: Correct confirmed post-promotion factual drift, evaluate one explicit local read-only Historical Question actual-use inspector, and stop at a Founder decision before any production UI implementation.
- Initial scope: Factual Book One synchronization; Product Review; decision PHASE3-PROVENANCE-INSPECTOR-P1-001 with a bounded, schema-neutral, read-only P1 contract; exact acceptance and manual-review criteria.
- Explicit non-scope: No inspector implementation before Founder approval; no persistence API, SQL, schema, migration, retention, provider, ContextPacket, consent-policy, Phase 4, Harness, Stage 2/3, Git promotion, PR, or deployment change.
- Relevant Book Zero definitions: docs/00_Constitution.md; docs/03_Principles.md; docs/06_Memory.md; docs/Reflection.md; docs/09_AI.md; docs/10_Privacy.md.
- Relevant ADRs: ADR-0007, ADR-0009, ADR-0010, and ADR-0011.
- Relevant architecture documents: architecture/09, architecture/10, architecture/11, architecture/12, and architecture/13.
- Relevant code areas: src/historicalContext/governedPacket.ts; src/shared/storage/; src-tauri/src/sqlite.rs; src/app/App.tsx; src/app/i18n.ts; current Historical Question and R1 tests.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md; one writable orchestrator; use repository workflow commands; preserve deleted-content non-resurrection and ADR-0009 cascade semantics; stop at human_decision_required.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-07-26T16:17:55.255Z
