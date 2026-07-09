---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/09
depends:
  - docs/dev/05_Development_Agent_Runbook.md
  - docs/architecture/04_Evidence_Candidate_Boundary.md
  - docs/architecture/05_Reflection_Prompt_Boundary.md
  - docs/architecture/01_Local_Evidence_Store.md
referenced_by: []
---

# 07 Reflection Boundary Verification

## Purpose

This document defines the manual verification flow for the first Reflection Prompt mock boundary in the Tauri runtime.

The goal is to confirm the product boundary:

Confirmed Evidence  
-> Reflection Prompt  
-> User Response Optional

This verification does not test real AI quality.

It tests local persistence and session-only behavior.

## Expected Persistence Boundary

The current expected behavior is:

- `ExperienceEntry` persists in SQLite.
- `EvidenceCandidate` is session-only.
- `ReflectionPrompt` is session-only.
- Reflection prompt responses are session-only.

This means closing and reopening the Tauri app should preserve experiences, but should clear evidence candidates, reflection prompts, and prompt responses.

This is expected for the current sprint.

It is not data loss for evidence or reflection because those records are not persistent records yet.

## Prompt State Transition Rule

The current prompt state rule is intentionally minimal:

- `suggested` can be answered.
- `suggested` can be skipped.
- `answered` can be edited and saved again.
- `answered` cannot be skipped.
- `skipped` cannot be answered in this sprint.

Skipped prompt reopening is deferred.

Do not add persistence or a reopen flow during this verification sprint.

## UX Copy Checks

The UI should clearly state:

- Reflection prompts are questions, not conclusions.
- Responses are session-only for now.
- Evidence and reflection are not persisted yet.

The copy should stay short.

The UI should not present advice, diagnosis, identity labels, MBTI, pattern notes, growth notes, or scores.

## Standard Commands

From the repository root:

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command ". 'D:\Lin\Project\LifeOperatingSystem\scripts\use-local-dev-env.ps1'; pnpm run typecheck"
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command ". 'D:\Lin\Project\LifeOperatingSystem\scripts\use-local-dev-env.ps1'; pnpm run build"
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command ". 'D:\Lin\Project\LifeOperatingSystem\scripts\use-local-dev-env.ps1'; pnpm run tauri:dev"
```

## Manual Tauri Runtime Flow

1. Open the Tauri app.
2. Add one new experience.
3. Close the app.
4. Reopen the app.
5. Confirm the experience persists.
6. Click `Generate evidence candidates`.
7. Confirm at least one evidence candidate.
8. Click `Generate reflection prompts`.
9. Confirm every prompt is a question, not an answer.
10. Write an answer for one prompt.
11. Click `Save answer`.
12. Confirm the prompt becomes `answered`.
13. Confirm the answered prompt can be edited and saved again.
14. Confirm the answered prompt cannot be skipped.
15. Skip one suggested prompt if another prompt is available.
16. Confirm the skipped prompt cannot be answered.
17. Confirm the reflection summary updates:
    - suggested count
    - answered count
    - skipped count
18. Close the app.
19. Reopen the app.
20. Confirm the experience persists.
21. Confirm evidence candidates disappeared.
22. Confirm reflection prompts disappeared.
23. Confirm prompt answers disappeared.

## Boundary Checks

During verification, confirm the app does not create:

- Pattern schema.
- Pattern notes.
- Growth notes.
- Advice.
- Diagnosis.
- MBTI.
- Identity labels.
- Scores.
- Real AI provider calls.

## Minimal Test Policy

Do not add a large test framework for this sprint.

The pure summary utility can be checked by TypeScript compilation and the manual UI flow above.

A future lightweight unit test runner may be considered only after the MVP has more stable utility boundaries.

## Result Template

Use this template after manual verification:

```text
Date:
Runtime:
typecheck:
build:
tauri:dev:

Experience persists after restart:
Evidence candidates disappear after restart:
Reflection prompts disappear after restart:
Reflection responses disappear after restart:

Suggested can answer:
Suggested can skip:
Answered can edit and save again:
Answered cannot skip:
Skipped cannot answer:

No advice / diagnosis / identity label / pattern:

Notes:
```
