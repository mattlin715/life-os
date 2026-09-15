# Current Mission

Status: ready

- Sprint ID: 2026-08-27-desktop-schema-v5-prepared-state-recovery-r2
- Mission title: Desktop Schema-v5 Prepared-State Recovery and Real-Profile Migration R2
- Origin: founder_request
- Base branch: develop
- Starting commit: 8173d4fe76eeb6498fb8e5d5e3d6169c97e0b899
- Background: Ordinary Schema-v5 Production Activation R1 was promoted through a clean non-fast-forward merge after disposable migration/runtime review. Its one separately authorized real-profile attempt stopped before backup creation or schema mutation and preserved one exact-owned `prepared` operation, zero-byte staging, and verifier-created WAL/SHM beside a byte-unchanged schema-v4 database.
- Problem: The promoted runtime truthfully failed closed on retained pre-write evidence and later on an exact empty WAL/zero-frame SHM pair created when a historical schema-v4 binary refused a valid disposable schema-v5 profile. Neither state may be silently cleaned, checkpointed, ignored, repaired, restored, retried, or treated as consent.
- Intended outcome: Implement and verify the exact prepared-v4 path plus Founder-authorized A-R3 recovery for only the exact disposable `v5_ready` legacy-refusal sidecars; disclose each boundary in English, Traditional Chinese, and Japanese; preserve recovery, migration, backup, and restore as separate actions; build one ignored unsigned successor review package; and stop at Founder diff/manual review before any real-profile action.
- Initial scope: Repository-only evidence reconstruction; destructive-data threat analysis; two exact eligibility classifiers; explicit-open recovery UI; strict content-free receipts; bounded four-file or two-file quarantine; restart reconstruction; recovery/migration consent separation; synthetic refusal/TOCTOU/partial-failure matrices; architecture/runbook synchronization; focused, Clippy, canonical, and disposable package verification.
- Explicit non-scope: No autonomous access to `%APPDATA%\com.lifeos.app`; no retrieval/display/interpretation of personal content fields; no real-profile mutation, cleanup, checkpoint, retry, backup, restore, or migration; no general SQLite repair or file cleanup; no provider, ContextPacket, consent, Product Harness, schema/DDL, ADR-status, Phase 4, Android, staging, commit, push, merge, PR, deployment, distribution, or release authority.
- Relevant Book Zero definitions: `docs/00_Constitution.md`; `docs/02_Philosophy.md`; `docs/03_Principles.md`; `docs/06_Memory.md`; `docs/Reflection.md`; `docs/09_AI.md`; `docs/10_Privacy.md`.
- Relevant ADRs: `docs/adr/ADR-0004-local-first-mvp.md`; `docs/adr/ADR-0007-persist-reviewed-ai-artifacts-with-provenance.md`; `docs/adr/ADR-0009-govern-historical-context-use-with-explicit-consent-and-provenance.md`; `docs/adr/ADR-0011-establish-append-only-artifact-lifecycle-and-portable-provenance.md`.
- Relevant architecture documents: `docs/architecture/01_Local_Evidence_Store.md`; `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`; `docs/architecture/15_Phase_3C_Production_Activation_Readiness_Gate.md`; `docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md`; new architecture/19; `docs/dev/10_Windows_Ordinary_Schema_v5_Review_Package_R1.md`.
- Relevant code areas: `src-tauri/src/filesystem_safety.rs`; `src-tauri/src/schema_v5_founder_activation.rs`; `src-tauri/src/schema_v5_migration.rs`; `src-tauri/src/lib.rs`; `src/shared/storage/sqlite/founderSchemaV5.ts`; `src/shared/storage/createLocalEvidenceStore.ts`; `src/app/App.tsx`; recovery UI/i18n/tests; ordinary review package scripts/configuration.
- Constraints: Apply the roles sequentially in one writable checkout; use only synthetic/disposable fixtures; do not launch ordinary Life OS during implementation; fail closed on every predicate mismatch or TOCTOU change; do not rewrite workflow archives; guide Founder manual review one bounded step at a time.
- Current owner: orchestrator
- Current phase: human_decision_required (`DESKTOP-V5-PREPARED-RECOVERY-R2B-PROMOTION-001` active and unresolved; promotion not authorized)
- Created at: 2026-08-26T15:42:44.900Z
- Updated at: 2026-09-01T02:03:18+09:00

## R2B repository-only correction addendum — 2026-09-02

After separately authorized real Phase B recovery and one Phase C migration
attempt, the exact real profile reached committed schema v5 with one verified
schema-v4 backup but stopped fail closed at
`post_commit_schema_manifest_mismatch` before lifecycle writes were enabled.
Read-only diagnosis proved the historical frontend multiline
`experience_entries` schema representation was the only schema-object digest
difference and reproduced the exact derived 95-object manifest from unchanged
DDL without inspecting personal rows or content.

The Founder selected Option B and authorized one bounded repository-only R2B
correction cycle: recognize only the exact fixed historical source/derived
manifests, reject unknown source representations before migration, implement
an explicit fail-closed recovery for only that exact post-commit state, add
focused tests and synchronized architecture/workflow evidence, run canonical
verification, and build one ignored unsigned disposable package. Installation,
launch, all further access to the real `com.lifeos.app` profile, real recovery,
retry, restore, repair, checkpoint, schema decrement, backup deletion,
migration, promotion, staging, commit, push, merge, PR, deployment,
distribution, release, Phase 4 and Android remain unauthorized.

Current R2B stopping point: prepare the ignored unsigned package and return to
a disposable Founder manual-review gate. The real blocked profile remains
untouched by this correction cycle.

## R2B repository gate reached — 2026-09-02

The exact implementation, documentation synchronization, focused checks,
warnings-denied Clippy and canonical repository verification pass. Exactly one
ignored unsigned package was created with suffix
`r2b-post-commit-manifest-lock`; its verified size is `5805550` bytes and its
SHA-256 is
`30ec0a075f79728b6c87a2f9ba9c5ed3ec1d7188d85b96650fc23cd8715ade9b`.
The repository-only correction has reached its authorized stopping point.
Disposable installation/launch and all real-profile/Git/promotion/release/
Phase-4/Android actions remain unauthorized and were not performed.

## R2B disposable Founder acceptance — 2026-09-14

The Founder accepted the complete disposable BF1-BF8 matrix. The final state
proves the bounded recovery disclosure and Cancel review, exact recovery
completion on a separate clone, post-recovery restart, managed-backup
reconstruction, and one persisted synthetic schema-v5 Experience with no
provider transmission. The BF8 runner/before/after/verification/result digests
are respectively `3c2213fa433af2ed2643f205c69e53c2209b888485c92506ee6f7e852cec6963`,
`edd4194692e135c9419bfb6be533c8da62270128ee9b29f3051ca73295720151`,
`c3cf827f0df68a77e44465376f09a50d1587cc74b20082986b392c3a66204c59`,
`b0cfa0d830beec9557648bbf233f927e904a606e837890f6358c2b4a5dca547c`
and `d7ab885c5a2c1d88e994c10c69ce4f6fb7df31ab9e52294a17467027f42a7e99`.

The current repository-only task is evidence synchronization, fresh focused
and canonical verification, and preparation of one explicit promotion
decision. It does not authorize promotion. The real profile remains blocked
after the consumed Phase C attempt with lifecycle writes disabled; Android M0
remains parked behind the full desktop sequencing fence.

## R2B workflow-control correction completed — 2026-09-14

The bounded `refresh-verification-for-resume` CLI operation now resolves the
proven stale-verification control-plane cycle without reopening or rewriting the
old Founder decision. Focused and canonical verification pass. The official
workflow resumed through `theory_alignment_review`, recorded the factual final
review package, and activated
`DESKTOP-V5-PREPARED-RECOVERY-R2B-PROMOTION-001` as a new unresolved machine
decision. No promotion action, profile action or Android work occurred.
