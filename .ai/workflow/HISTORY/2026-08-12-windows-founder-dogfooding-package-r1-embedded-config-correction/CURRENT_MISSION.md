# Current Mission

Status: ready

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-embedded-config-correction
- Mission title: Correct the installed Founder package embedded Tauri configuration
- Origin: founder_request
- Base branch: develop
- Starting commit: c7e767a397995c1d96daebcafa08eb1e578e356e
- Background: Founder manual Step 3 launched the installed R1 package and observed window title `Life OS` with `localhost` connection refusal. Process evidence showed the installed executable at `C:\Users\a7152\AppData\Local\Life OS Founder Dogfood\life-os.exe`.
- Problem: The split `cargo rustc` plus `tauri bundle` correction produced GUI subsystem 2 but compiled the binary with the default embedded Tauri configuration, so the installed application retained the default title and attempted the development URL.
- Intended outcome: Build the final binary through `tauri build --config <Founder override>` while applying package-local Windows GUI linker flags, verify the binary and manifest, preserve the failed manual evidence, and return to a clean reinstall/retest step.
- Initial scope: Correct only Founder package build/contract scripts, focused tests, factual runbook, ignored artifact, and repository workflow evidence.
- Explicit non-scope: No product React/Rust/SQLite source change; no schema, provider, ContextPacket, consent, migration, autonomous install/uninstall, promotion, distribution, deployment, or release.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/03_Principles.md`; local-first privacy and user agency remain unchanged.
- Relevant ADRs: ADR-0007, ADR-0009, and ADR-0011 remain unchanged.
- Relevant architecture documents: `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md`.
- Relevant code areas: `scripts/build-founder-dogfood-package.ps1`; `scripts/founder-dogfood-package.mjs`; focused package tests; package runbook.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-12T13:37:28.998Z
