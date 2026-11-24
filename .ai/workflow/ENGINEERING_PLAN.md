# Engineering Plan

Status: approved

- Sprint ID: android-pilot-byo-gemini-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 869a25e217cb1297bd0569964b2b69b56fe9650d
- Working-tree digest reviewed: ac4029e060d2349c1433b1eddd81abf3e1b21b3d6747bd5ebc29f48f5592aff4
- Created at: 2026-11-15 JST
- Updated at: 2026-11-15 JST

## Approved Product Boundary

Product Review approved_with_conditions; all synthetic-only, explicit-consent and no-persistence conditions carry through.

## Existing Implementation Understanding

Reuse Android M2-C locale and existing approval/persistence; desktop AIProvider deliberately excluded.

## Affected Modules

AndroidM2CApp and ReflectionJourney; isolated model settings/preview adapter; Android-only native transport and release manifest overlay; add one M3-gated atomic operation reusing canonical writes.

## Proposed Design

Keep the main subtree mounted under Settings. Gate the narrow IPC and network manifest separately for android-pilot-online. Store key only in native state; exact preview text and locale sent to native only once. Block stale replies and concurrent sends.

## Alternatives Considered

Desktop AIProvider exports excess data; WebView fetch would expose credentials in renderer/session and depend on CORS; proxy expressly excluded.

## Data Lifecycle Impact

Session-only key and unsaved draft; explicit Save atomically persists the exact Gemini question and user response in a new M3-only synthetic store. Offline identity and data remain untouched.

## SQLite Or Migration Impact

No DDL or migration; reuse canonical direct-v5 transaction to create the prompt and save its response in one commit; verified reread and exact retry identity.

## Provenance Impact

Persist exact prompt provenance (AI/Gemini/pinned model) and separate user response provenance. No promotion to evidence.

## Historical Context Impact

None; narrow request shape lacks history.

## Consent Impact

Explicit synthetic-only acknowledgement and frozen destination/text preview; no automatic fallback or resend.

## Provider Transmission Impact

One fixed HTTPS destination, endpoint, and default model; POST exactly one clue plus bounded instruction; mock only this turn.

## Import And Export Impact

None; no changes to portability or backups.

## Test Strategy

Exercise consent, no retries/fallback, exact payload, stale fencing, key clear, state continuity, and locale persistence in mocks; inspect offline exclusions.

## Repository Verification Strategy

Canonical preflight where permitted; TS, Vitest, contract tests, host native checks, Git whitespace. No APK/build packaging.

## Manual UI Verification

Founder after separate package and device gates; no current live test.

## Rollback Or Recovery Strategy

Offline Pilot unchanged; omit connected feature/overlay; no DB changes and key not persisted.

## Documentation Impact

Record separate connected scope and packaging gates in Android Book One architecture/runbook.

## ADR Impact

No ADR change: Founder approved isolated synthetic M3 Option A via direct Founder chat. Actual outbound test/packaging still separately gated.

## Risk Level

Medium: new secret and external transmission surface, isolated from the offline artifact.

## Escalation Decision

Pause for package and actual network/device approval; do not claim runtime or production authority.

## R2 Revision Plan

The preceding design records R1. R2 retains the connected identity and
strict fixed transport. A source-revision-bound question request sends
the exact original and optional exact excerpt. A separately consented
feedback request binds exact source and saved Reflection revisions;
only original, question and saved answer are transmitted.

Only the feature-gated atomic Gemini version-2 answered-pair operation
can create a canonical record without a uses_evidence edge. It retains
the exact derived_from_experience edge and answers_prompt lineage; the
normal Evidence gate remains in place for offline/desktop generation and
unqualified writes. No DDL, side store, or ADR amendment. Corrections use
the same compare-and-set writer. Feedback remains renderer session state.

Validate payload and consent, asynchronous invalidation, key lifetime,
atomic rollback/retry/readback, response correction, and source
replacement/deletion with host fixtures. No APK or live request.

## R3 UI source cleanup and integrated proposal

Only renderer presentation and regression tests change now. Remove the focus
entry/preview field, do not send an optional focus from the connected screen,
and remove the connected legacy renderer branch rather than hiding it in a
collapsed control. Retain the offline renderer. Genuine earlier saved model
questions/answers remain readable without generation or adoption controls;
no artifact/SQLite cleanup. Question and feedback use distinct localized
origin labels; the current feedback still truthfully states session-only.

Document one R3 implementation proposal for opt-in encrypted credential
retention, explicit reviewed local feedback save, and same-moment follow-up
with new preview/consent per request. Specify source revisions, provenance,
correction/deletion and upgrade continuity before implementing new storage.
Do not add a new schema or silently reuse Pattern or generic JSON sidecars.

Tests: all three locales, original-only preview, no connected mock actions,
earlier Gemini readback, distinct origin labels, plus existing frontend/
Android contracts. Do not build, sign, install, send, or promote Git. Canonical
verification remains independently required; do not weaken its preflight.

## R3 authorized continuity: exact feasibility findings

Founder authorizes items 1,5,6 sources and host tests; no packaging. A native-only
Tauri mobile bridge will manage opt-in Android Keystore AES-GCM wrapping into
an AtomicFile under noBackupFilesDir. There is no renderer credential getter,
plaintext persistence, SQLite secret or APK key. Session-only default and
clear/forget write a durable disabled marker before destroying the wrapping
key, so cleanup failure cannot silently restore an old credential.

Schema v5 artifact_heads restricts kinds to evidence/reflection/pattern/
recovery_turn/historical_question; roles and relationship types are similarly
closed. Pattern creation requires confirmed Evidence; the R2 source-only
question intentionally has none. Durable independent feedback and parent-
feedback-bound supplements cannot be represented without a typed extension.
Do not masquerade them as Evidence/Reflection or widen accepted Pattern rules.
Draft the exact next-schema/lifecycle ADR for separate Founder review as the
approved proposal requires. Keep angle results session-only and supplementation
unimplemented until that checkpoint. Credential work can proceed independently
without modifying SQLite, so no interim APK is required.

Host: pure Java injected-backend crypto/lifecycle tests (not real Keystore),
Java compile of the Android bridge against pinned APIs, TypeScript/frontend
state tests, and focused native tests/static mobile registration. Actual Android
Keystore, secret update continuity and secure storage cannot be proven on host.
