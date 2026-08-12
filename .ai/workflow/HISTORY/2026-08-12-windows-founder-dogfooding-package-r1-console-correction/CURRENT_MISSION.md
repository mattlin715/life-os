# Current Mission

Status: ready

- Sprint ID: 2026-08-12-windows-founder-dogfooding-package-r1-console-correction
- Mission title: Correct the Founder package Windows subsystem without touching product runtime source
- Origin: founder_request
- Base branch: develop
- Starting commit: c7e767a397995c1d96daebcafa08eb1e578e356e
- Background: Post-archive PE inspection found the first review build used Windows CUI subsystem 3 despite the package requirement for no visible console.
- Problem: The package contract was incomplete; a successful NSIS build did not prove the embedded Life OS executable used Windows GUI subsystem 2.
- Intended outcome: Correct only the package build path to compile the final binary with Windows GUI subsystem, validate the PE header before bundling, rebuild the ignored artifact, reverify, and preserve product runtime source unchanged.
- Initial scope: Package build script, package contract module/tests, factual runbook and workflow evidence only.
- Explicit non-scope: No `src-tauri/src` or other product/runtime source change; no schema, provider, consent, install, promotion, distribution, deployment, or release.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; local-first and user-agency principles remain unchanged.
- Relevant ADRs: ADR-0007, ADR-0009, and ADR-0011 remain unchanged.
- Relevant architecture documents: `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md`.
- Relevant code areas: `scripts/build-founder-dogfood-package.ps1`; `scripts/founder-dogfood-package.mjs`; focused package tests and factual docs.
- Constraints: Follow AGENTS.md and .ai/workflow/WORKFLOW.md.
- Current owner: orchestrator
- Current phase: intake
- Created at: 2026-08-12T12:20:52.194Z
