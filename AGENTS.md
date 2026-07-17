# Life OS Engineering Harness

`AGENTS.md` is the repository execution entry point. It is **not** the Life OS Constitution and cannot override it.

## Authority order

1. `docs/00_Constitution.md`
2. Book Zero primary definitions, routed through `docs/00_Index.md`
3. Accepted ADRs in `docs/adr/`
4. Book One architecture and product specifications
5. This Engineering Harness, skills, and implementation code

The human founder is the final constitutional authority. Preserve: **We Build Mirrors, Not Oracles.**

## Read the route that fits the task

| Task | Read first |
| --- | --- |
| Product theory / Book Zero | `AI_CONTRIBUTOR_GUIDE.md`, `docs/00_Index.md`, then the concept's primary definition |
| AI / Product Harness | `docs/02_Philosophy.md`, `docs/03_Principles.md`, `docs/06_Memory.md`, `docs/Reflection.md`, `docs/09_AI.md`, `docs/10_Privacy.md`, `docs/appendix/Harness.md` |
| Persistence / migrations | `docs/architecture/01_Local_Evidence_Store.md`, `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`, `src/shared/storage/`, `src-tauri/` |
| UI / product flow | `docs/11_MVP.md`, `docs/product/`, relevant architecture boundary, and `src/app/` |
| Documentation | `docs/00_Index.md`, affected document metadata, hierarchy, and ADR implications |
| Multi-role sprint orchestration | `.ai/README.md`, `.ai/workflow/WORKFLOW.md`, then the applicable role specification |
| Sprint completion | `docs/dev/08_Engineering_Harness.md` and `scripts/verify.ps1` |

For substantive work, read `AI_CONTRIBUTOR_GUIDE.md` before editing.

## Multi-Role Sprint Orchestration

For a substantive sprint that benefits from explicit product, engineering, and
alignment handoffs, use the tool-independent artifacts in `.ai/`. They remain
below this file and the Engineering Harness in authority. Apply the roles
sequentially in one writable worktree, record workflow truth in the repository,
and stop at `human_decision_required` rather than resolving a consequential
founder decision. `.ai/README.md` is navigation, not a second execution entry
point.

## Hard constraints

- Do not modify the Constitution or redefine canonical principles without explicit founder authorization.
- Do not mix AI inference with user-confirmed evidence, weaken provenance, or make AI an oracle.
- Do not transmit historical context to a provider without governed, user-controlled consent.
- Do not expose secrets or commit local credential files.
- Do not reset, discard, overwrite unrelated work, silently commit, or push when the task forbids it.
- Do not treat silence as rejection or implement diagnosis, identity finalization, or background profiling.

## Verification

Use the repository-owned deterministic path from the root:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

It does not replace founder manual UI verification.

## Founder checkpoints

Require explicit founder review before changing the Constitution, a Book Zero
primary definition, Product Harness behavior policy, ADR acceptance/status, any
implementation authority explicitly withheld by an ADR or architecture
document, sensitive inference, longitudinal-memory consent, destructive
migration policy, historical provider transmission, a new product worldview,
or identity-finalization behavior.

## Completion report

Report files changed, behavior changed, automated checks, documentation synchronization, ADR decision, remaining gaps, manual founder checks, and git state.
