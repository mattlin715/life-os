# Current Mission

Status: ready

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1
- Mission title: Build the bounded Windows Founder dogfooding package without changing schema-v4 product semantics
- Origin: founder_request
- Base branch: develop
- Starting commit: c7e767a397995c1d96daebcafa08eb1e578e356e
- Background: The Founder-approved Phase 3 exit audit permits bounded Founder dogfooding on schema v4 and identifies a Windows Founder Dogfooding Package R1 as the next implementation slice. The audit was promoted at `c7e767a397995c1d96daebcafa08eb1e578e356e`.
- Problem: The current product is usable through development tooling but lacks an installable, isolated Windows package suitable for bounded Founder dogfooding without risking the ordinary development data profile.
- Intended outcome: Produce one unsigned, local, non-distributed Windows installer with a distinct application identity and data profile, a content-free integrity manifest, deterministic repository build and verification commands, factual Book One documentation, and a complete Founder manual-review package.
- Initial scope: Additive Tauri Windows packaging override, package scripts and focused tests, ignored output, content-free manifest, architecture/16 factual synchronization, documentation, canonical verification, and workflow evidence.
- Explicit non-scope: No product behavior changes; no schema or migration change; no real-user database access; no updater, telemetry, cloud service, signing secret, auto-start, provider, ContextPacket, consent, Phase 4, Android, distribution, deployment, release, stage, commit, push, merge, or PR.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/03_Principles.md`; `docs/06_Memory.md`; `docs/09_AI.md`; `docs/10_Privacy.md`.
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md`; `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`; `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`.
- Relevant code areas: `src-tauri/tauri.conf.json`; additive package override under `src-tauri/`; `scripts/`; `.gitignore`; `package.json`; factual Book One documentation only.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-12T11:46:27.842Z
