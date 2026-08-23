# Current Mission

Status: ready

- Sprint ID: 2026-08-18-desktop-schema-v5-ordinary-production-activation-r1
- Mission title: Activate schema v5 for the ordinary desktop through explicit governed migration
- Origin: founder_goal
- Base branch: develop
- Starting commit: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Background: Founder Candidate R1 is promoted and manually verified on isolated disposable profiles; ordinary com.lifeos.app remains schema v4.
- Problem: Ordinary desktop cannot yet initialize, migrate, recover, or route verified schema v5 while preserving explicit local control and fail-closed recovery.
- Intended outcome: Implement and verify the bounded ordinary desktop schema-v5 activation, create an unsigned disposable ordinary-identity review package, and stop before installation at Founder manual review.
- Initial scope: Shared activation core, ordinary desktop identity/build policy and UI integration, exact typed v5 routing, backup retention/restore controls, tests, factual documentation, and review package.
- Explicit non-scope: No real ordinary-profile access before explicit final manual authorization; no Phase 4, Android, provider, ContextPacket, consent-policy, distribution, deployment, release, stage, commit, push, merge, or PR.
- Relevant Book Zero definitions: `docs/00_Constitution.md` (human agency, privacy, evidence, and mirror boundary); `docs/03_Principles.md`; `docs/06_Memory.md`; `docs/Reflection.md`; `docs/09_AI.md`; `docs/10_Privacy.md`; `docs/11_MVP.md`; `docs/12_Roadmap.md`; `docs/appendix/Harness.md`.
- Relevant ADRs: ADR-0007 reviewed-artifact provenance; ADR-0009 exact historical consent, transport, and actual-use provenance; ADR-0011 append-only lifecycle, correction, deletion, projection, and non-destructive cutover policy.
- Relevant architecture documents: architecture/01 local store; architecture/09 and /10 Phase 3B; architecture/12 lifecycle foundation; architecture/13 migration/cutover; architecture/15 readiness gate; architecture/16 product-exit audit; architecture/17 promoted isolated Candidate R1; docs/dev/09 package boundary.
- Relevant code areas: `src-tauri/src/sqlite.rs`, filesystem and schema-v5 migration/runtime/writer modules, Tauri command registration/configuration, `src/shared/storage/` adapters, database startup and migration UI/i18n, package-contract/build scripts, and canonical verification.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-18T10:50:39.055Z
