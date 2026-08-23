# Current Mission

Status: ready

- Sprint ID: 2026-08-23-ordinary-v5-real-profile-phase-b-blocked-r1
- Mission title: Record and analyze the fail-closed real-profile Phase B pre-backup migration outcome without retry or repair
- Origin: founder_request
- Base branch: develop
- Starting commit: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Background: The Founder separately authorized Manual Phase B for one explicit migration attempt against the actual ordinary `com.lifeos.app` profile after the ordinary schema-v5 implementation passed canonical verification and disposable Manual Phase A. The exact reviewed executable and installer were hash-verified before launch. That single real-profile action failed closed before backup creation. The Founder then selected Option A for `ORDINARY-V5-PHASEB-WAL-VERIFIER-CORRECTION-001`, authorizing only a separate disposable persistent-WAL verifier correction and review while preserving the real profile and all recovery evidence unchanged.
- Problem: Content-free durable evidence showed the real main database remained byte-identical to its pre-action schema-v4 state while one exact-owned operation remained `prepared` with a zero-byte staging file, no backup, and WAL/SHM sidecars. The first disposable correction made verifier and backup-source inspection immutable and sidecar-free, but its packaged migration exposed a second bounded defect: normal activated-runtime reads could create empty WAL/SHM sidecars, and typed writers did not explicitly close their connection before durable verification. The disposable database committed as schema v5 with a verified v4 backup, then correctly surfaced `sqlite_sidecar_present` instead of claiming normal runtime success.
- Intended outcome: Preserve the real ordinary profile and its recovery evidence without retry or repair; complete the bounded immutable-read and explicit-writer-close correction; prove it through production-shaped automated coverage; build one ignored unsigned disposable review installer; and return to Founder-owned disposable migration, typed-write, normal-close, and restart review before Founder diff review.
- Initial scope: Immutable sidecar-prechecked source and stable-runtime reads; explicit verifier and typed-writer close; persistent-WAL disposable fixtures through real migration and typed facades; fail-closed malformed/uncheckpointed cases; package-contract synchronization; factual architecture, development, workflow, and verification evidence.
- Explicit non-scope: No real-profile retry, cleanup, checkpoint, repair, restore, sidecar deletion, operation-directory deletion, backup creation, migration, or content inspection. No mutation or retry of the failed disposable v5 forensic state except the explicit Founder-owned preservation and fresh disposable-fixture preparation step. No stage, commit, push, merge, PR, deployment, distribution, release, provider/ContextPacket/consent change, Phase 4, or Android work.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/03_Principles.md`; `docs/06_Memory.md`; `docs/10_Privacy.md`.
- Relevant ADRs: `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`; `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`; `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`; `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`.
- Relevant code areas: `src-tauri/src/filesystem_safety.rs`; `src-tauri/src/schema_v5_migration.rs`; `src-tauri/src/schema_v5_founder_activation.rs`; `src-tauri/src/schema_v5_runtime.rs`; the seven registered schema-v5 typed-writer modules; `scripts/founder-dogfood-package.mjs`; `src-tauri/src/sqlite.rs`; `.ai/workflow/`.
- Constraints: Follow `AGENTS.md` and `.ai/workflow/WORKFLOW.md`. Treat the real profile as sensitive even when recording only content-free metadata. Preserve the fail-closed evidence exactly and do not infer retry authority from a diagnosed cause.
- Current owner: chief_product_theorist
- Current phase: theory_alignment_review
- Created at: 2026-08-22T22:43:21.322Z
