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
