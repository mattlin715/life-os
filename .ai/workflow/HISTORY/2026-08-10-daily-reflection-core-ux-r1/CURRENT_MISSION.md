# Current Mission

Status: ready

- Sprint ID: 2026-08-10-daily-reflection-core-ux-r1
- Mission title: Daily Reflection Core UX R1
- Origin: explicit_founder_authorization
- Base branch: develop
- Starting commit: f8fbf94812bac2e1359367362e9c1e0db1aeaef1
- Background: The promoted schema-v4 product contains the governed Experience, Evidence, Reflection, local-history, consent-preflight, provenance, and Database Readiness capabilities, but the ordinary first-screen hierarchy is dense and exposes diagnostics before the daily reflection journey. The Founder authorized a bounded product-first UX re-composition without changing persistence, provider, ContextPacket, consent, or Phase 4 behavior.
- Problem: The primary screen does not sufficiently distinguish the daily Experience-to-Reflection journey from optional history, diagnostics, portability, and technical disclosure. Traditional Chinese and Japanese core-path copy also retains unnecessary English domain terminology, and the timeline lacks session-local keyword and saved-date retrieval controls.
- Intended outcome: Deliver one coherent Daily Reflection Core UX R1 that prioritizes Experience entry and next action, places database readiness in a secondary diagnostics surface, adds deterministic session-only local Experience filtering, provides native-language core terminology, and presents ADR-0009 preflight as a concise summary with exact details available before consent.
- Initial scope: Reversible React/CSS/i18n composition; pure local Experience keyword and saved-date filtering; progressive disclosure for older/completed stages and consent details; secondary diagnostics access; behavior-level regression tests; minimal factual Book One/product documentation; Harness review and canonical verification.
- Explicit non-scope: Constitution or Book Zero changes; ADR changes; schema or persistence changes; real-user migration; provider or ContextPacket changes; weakened consent/provenance; Phase 4; embeddings, vector search, memory graphs, cloud sync, telemetry, deployment, release, PR, stage, commit, push, or merge.
- Relevant Book Zero definitions: `docs/00_Constitution.md`, `docs/03_Principles.md`, `docs/06_Memory.md`, `docs/Reflection.md`, `docs/09_AI.md`, and `docs/10_Privacy.md` govern mirrors-not-oracles, evidence, reflection, user ownership, uncertainty, local control, and consent.
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`, `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`, and `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`; no ADR status change is authorized or expected.
- Relevant architecture documents: `docs/architecture/00_MVP_Architecture.md`, `docs/architecture/08_Local_Historical_Context_Selection_Foundation.md`, `docs/architecture/09_Governed_Historical_Context_Assembly_and_Consent.md`, `docs/architecture/10_Historical_Question_Persistence_Vertical_Slice.md`, and `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`.
- Relevant code areas: `src/app/App.tsx`, `src/app/i18n.ts`, `src/styles.css`, existing historical date/range and governed packet modules, presentation components/tests, and no storage/provider mutation module.
- Constraints: Use the existing sequential repository Harness; preserve schema v4 and every current consent, packet, transport, stale-work, deletion, and provenance invariant; use no concurrent writable agents; keep all filter and disclosure UI state session-only; stop only at Founder diff/manual UI review readiness.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-10T08:18:24.085Z
- Updated at: 2026-08-10T08:27:00.000Z
