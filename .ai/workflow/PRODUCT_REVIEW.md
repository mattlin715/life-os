# Product Review

Status: approved_with_conditions

- Sprint ID: 2026-09-22-android-m1-disposable-persistence-review
- Artifact schema: 1.0
- Authoring role: chief_product_theorist
- Repository HEAD reviewed: 04965a6d6f7a62d1c5ae4d2e2fcf91317f5fd5df
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-09-22T01:20:00+09:00
- Updated at: 2026-09-22T01:20:00+09:00

Allowed final status: `approved`, `approved_with_conditions`,
`revision_required`, `human_decision_required`, or `rejected`.

## Mission Interpretation

M1 is an evidence-producing decision sprint. It may implement a disposable Android persistence prototype and recommend a production direction, but it may not accept that direction or claim M2 persistence complete. The prototype must be a truthful mirror of explicit user-authored synthetic text: draft state is visibly uncommitted, success appears only after the database transaction commits, and reopened content is the exact stored value.

## Problem Statement

Android M0 established native shell feasibility, not storage authority. The current production store factory intentionally routes Android away from the desktop schema-v5 runtime, and Android has no app-private SQLite implementation. Copying desktop filesystem and recovery claims would be unsafe. Founder review therefore needs a narrowly isolated implementation plus native evidence and explicit limitations.

## User Value

The Founder can evaluate the core local-first save-to-relaunch journey on a real Android runtime before choosing the final application identity and production storage architecture. Failures remain visible rather than creating false confidence or risking any desktop/real profile.

## Relevant Primary Definitions

- `docs/00_Constitution.md`: human authority and “We Build Mirrors, Not Oracles.”
- `docs/01_Vision.md`, `docs/02_Philosophy.md`, `docs/03_Principles.md`: local-first, low-pressure, user-agency boundaries.
- `docs/04_Identity.md`, `docs/06_Memory.md`, `docs/07_Growth.md`, `docs/Reflection.md`: no diagnosis, identity finalization, Pattern inference, or daily-reflection completion claim.
- `docs/10_Privacy.md`: app-private, offline, consent-preserving treatment of data.

## Relevant ADRs

- ADR-0004 requires local-first persistence and user ownership.
- ADR-0006 accepts Tauri + React + SQLite for the desktop MVP while leaving mobile architecture undecided.
- ADR-0007 and ADR-0011 supply provenance and append-only concepts that may inform the prototype, but this Experience-only surface must not pretend to implement reviewed AI artifacts or their complete lifecycle.
- Any Android production-direction ADR created here remains `Proposed` and cannot override those Accepted records.

## Current Implementation Context

Implemented and verified before this sprint: M0 disposable application `com.lifeos.feasibility.m0`, offline shell build, and Founder native checklist 11/11. Implemented on desktop: schema-v5 runtime/founder adapters and Windows-specific recovery. Not implemented: an Android `LocalEvidenceStore`, Android schema initialization/durability/recovery, final Android identity, or M2 behavior. This sprint is Founder-authorized for a prototype only; neither silence nor successful tests would authorize production.

## In Scope

- Disposable identity `com.lifeos.review.m1` with app-private, synthetic-only fresh-v5 storage.
- Trilingual create/list/get/reopen/relaunch journey with progressive technical details.
- Honest save state, commit acknowledgement, duplicate-submit prevention, and fail-closed storage errors.
- Focused automated/fault-injection tests, Android 36 x86_64 debug APK, and dedicated disposable AVD evidence.
- Proposed architecture/ADR package separating shared domain logic from Android storage responsibility and later phases.

## Out Of Scope

Final identity selection; production activation; desktop database access; v4 import/fallback; migration, recovery, repair, retention, delete, export, restore, cloud sync, providers, credentials, analytics, AI analysis, Evidence extraction, Patterns, daily-reflection completion, release signing/distribution, physical devices, and M2–M4 implementation.

## Product Constraints

1. The UI labels the app and data as disposable/synthetic and never conflates draft with committed content.
2. Only create/list/get operations actually implemented by the experimental adapter may be exposed.
3. A database is either exact supported schema v5 or refused; malformed/newer/ambiguous state is preserved without automatic recreation.
4. Android-specific guarantees must be evidenced on Android and narrowly worded.
5. Technical/storage details remain behind progressive disclosure; primary action stays calm and low-friction.

## Evidence And Provenance Constraints

The stored Experience is explicitly user-authored synthetic input. The prototype must retain a stable ID, creation time, revision/commit marker, and exact content. It must not label AI output or inferred claims as Evidence. Test fixtures and fault injections must be distinguishable from Founder-created review records.

## Historical Context Constraints

No historical desktop or Founder data may be read, imported, migrated, restored, or transmitted. The new application ID and app-private directory are the isolation boundary. Existing M0 evidence is immutable input evidence, not a profile to modify.

## Consent Constraints

No provider or historical-context consent flow is introduced because there is no provider transmission. Backup/transfer is excluded rather than silently opting the user into OS-mediated copying. Any future change is a separate user-visible decision.

## AI-Role Constraints

No AI feature exists in this prototype. UI copy must not imply interpretation, advice, diagnosis, Evidence extraction, Pattern creation, or oracle authority.

## Privacy Constraints

No `INTERNET` permission, real user data, credentials, cloud, desktop import, or external storage. Debug logs and evidence must avoid storing entered text unless a synthetic fixture is deliberately part of a bounded test artifact.

## User-Agency Constraints

Saving is explicit. The user can see whether text is draft, saving, committed, or failed; failed saves keep the draft available for correction/retry. Duplicate taps cannot silently create duplicates. Reopening must show exact committed text, not a generated summary.

## Acceptance Criteria

1. The debug APK uses exactly `com.lifeos.review.m1`, has no `INTERNET` permission, and excludes app data from backup/transfer.
2. A clean dedicated API 36 x86_64 AVD creates an app-private exact-v5 database and the UI completes enter → explicit save → list → reopen exact CJK text → force-stop/relaunch → reopen.
3. Success is emitted only after commit; repeated/double submission produces one Experience; a failed write leaves no acknowledged phantom record.
4. Automated/native evidence covers or explicitly marks unsupported every test category named in the Founder mission, including before/after-acknowledgement process termination, atomic interruption, concurrent open, incompatible database, open/write failures, lifecycle variants, packaging, permissions, and desktop isolation.
5. Malformed/newer/ambiguous storage is refused without deletion, repair, migration, restore, memory fallback, or v4 fallback.
6. The UI is trilingual, labels synthetic/disposable scope, keeps technical detail progressive, and exposes no unsupported store operation.
7. Architecture docs and a Proposed ADR give one recommendation plus final identity options, ownership, shared/Android responsibility, durability semantics, backup consequences, fail-closed recovery boundary, test matrix, and M2/M3/M4 remainder.
8. Android-focused tests and the repository canonical verifier pass; APK path/hash/build coordinates and exact changed paths are reported.
9. Workflow stops at one unresolved Founder diff/manual UI gate with all changes unstaged and uncommitted.

## Risks

- Android SQLite/OS semantics may differ from the Windows desktop implementation; mitigation is an Android-owned adapter and scoped claims.
- Process-kill tests can be timing-sensitive; use deterministic fault hooks and distinguish them from actual power-loss evidence.
- A “schema-v5 prototype” could overclaim production compatibility; use a prototype-specific contract marker and Proposed documentation, never desktop migration authority.
- Debug-only hooks could escape into production; bind them to the disposable identity/debug build and document removal/review requirements.
- OS backup defaults could copy synthetic records; manifest and data-extraction rules must be inspected from the packaged APK.

## Open Questions

The final production application ID and acceptance of the recommended Android storage architecture remain Founder decisions. They are intentionally deferred to the consolidated gate after evidence exists; they do not block the disposable prototype.

## Human Decision Required

False during implementation. At handoff, open one consolidated decision for Founder diff acceptance, manual UI results, production-direction acceptance/revision, and final identity selection or deferral. No production activation follows automatically.

## Recommendation

Proceed with an Android-specific experimental app-private SQLite adapter behind a narrow Tauri command façade, while reusing pure shared Experience validation/types rather than desktop path/recovery code. Use a fresh exact-v5 prototype contract, explicit transactional acknowledgement, fail-closed open, and a temporary package identity. Record the production direction as Proposed only and stop after native evidence is prepared.

## Review Status

`approved_with_conditions`: all constraints and acceptance criteria above must be copied into the Engineering Plan; material scope or dependency changes require escalation rather than silent expansion.
