# Engineering Report

Status: human_decision_required

- Sprint ID: android-pilot-byo-gemini-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD implemented: 869a25e217cb1297bd0569964b2b69b56fe9650d
- Planning working-tree digest (before implementation): 8360c4d4dea5a26517c7d5f2c29eada69554932b7d3b6a6162c54de866af5576
- Created at: 2026-11-15 JST
- Updated at: 2026-11-15 JST

## Implementation Summary

Option A, synthetic M3: separate Android identity, direct-v5 storage and
locale preference; same phone Reflection flow. Settings retains drafts.
The native provider bridge holds a user-pasted key in memory, and accepts
a single exact previewed adopted clue. Save uses the native result ticket
and atomically records the Gemini question and user answer without Gemini.

## Files And Verification Boundaries

- Renderer/nav/orchestration: \`src/android-m2c\`; mode: \`vite.config.ts\`.
- Native provider, identity and canonical save: \`src-tauri\`; Android-only
  manifest and Tauri overlays are not applied in the offline build path.
- Host contracts and documentation: \`scripts/android-m2c-contract.node-test.mjs\`,
  Android architecture 24 and runbook 15.
- ADR-0012 unchanged; scope authorized via ANDROID-M3-GEMINI-LOCAL-SAVE-R1 A.

## Tests Executed

565 frontend tests, 45 Android source contract tests, TypeScript and four
focused Rust host tests passed. Native tests used a staged source tree, a
locked Cargo graph and a synthetic temporary SQLite database. Atomic failure
injection, same-answer retry, divergent retry rejection, provenance and
stale clue rejection are covered. No provider call was made.

## Verification Results

Full canonical verifier stopped before tool execution: project-local pnpm
absent. Do not record full verification as passed. The narrowed evidence
does not establish Android runtime, device/keyboard behavior or model quality.

## Git State

Working worktree only, with pre-existing changes preserved. No stage, commit,
push, package, signing, install, or deployment.

## Engineer Completion Status

Source and mock-host scope complete; packaging, Android merged manifest,
on-device settings/draft/playback, live Google test and practical question
quality are not yet validated. Stop for Founder package approval.

## R2 Revision Implementation (after the 3100 checkpoint)

- `src/android-m2c`: connected direct-to-question path with complete
  original, optional verbatim focus, confirmed local save and a single next
  action for feedback. Prior clues and demo stay in a secondary disclosure.
  Three languages and settings behavior are retained. Feedback is not saved.
- `src-tauri/src/android_pilot_model.rs`: distinct fixed-HTTPS payloads and
  separate consent; pre/post checks bind exact source and saved pair.
  Response validates shape and a literal grounding quote or admits insufficient
  material. This is not a guarantee of model judgment or quality.
- `schema_v5_{android_reflection,reflection_write}.rs`: verified atomic
  source-only R2 pair, ordinary source correction/delete and same-answer
  reconciliation without a duplicate write. No fabricated Evidence, DDL,
  desktop adapter, or offline policy change. ADRs unchanged.
- Book One architecture 24 and Runbook 15 now document the exact two
  transmissions and one Founder device review.

### R2 Host Evidence

TypeScript and 571 Vitest tests passed; 46 Android M1/M2-A/M2-B/M2-C
source contracts passed. The R2 focused Rust host run passed 7 tests on the
final native source (including rollback, provenance, correction, invalidation,
deletion, exact payload, stale pair rejection, and volatile key). Broader
Windows host runs during this revision passed 280 connected tests and
275 offline tests. No Android runtime or remote request was exercised.

Canonical verify exited 1 at preflight because repository-local pnpm was
missing. The optional historical M0 checks were also inspected: three expect
the M0-only generated paths/identity/empty command surface rather than the
M2-C surface already present on the base. They are not in the canonical
verifier and were not changed. Do not mark repository verification passed.

R2 remains working-tree only, based on
`26130b9106f9efe52496ca5de659879ddc85d3e8` in
`codex/android-pilot-arm64-r1`. No package, sign, install, live Gemini
request, commit, or push. Exact phone behavior and usefulness need one
Founder review with fictional data after separate approval.

## R2 authorized standalone package

Founder approved one connected R2 package on 2026-11-16 JST.
Verified unsigned delivery: `D:\Lin\Project\LifeOS-Zenfone-Model-Pilot-ARM64-R2-3101-20261116\LifeOS-Model-Pilot-3101-unsigned.apk`.
SHA-256 `6CA2D0A0099B10B94D24FD8D76249BCB6BDC204D85591CF6AAC7D46502C8F71D`.
The ARM64 package retains `com.lifeos.review.pilot.model` with versionCode
3101. Verified embedded R2 assets, 16 KiB ELF/ZIP alignment, unsigned and
non-debuggable status, and exact 3100 permission continuity. See the
standalone RESULT.md, retained rejected-artifact evidence, and generated
output reconciliation. No canonical-verifier PASS, signing, installation,
API use, or Git promotion is implied.

## R3 UI cleanup and consolidated proposal (2026-11-18 JST)

### Implementation and files

- `src/android-m2c/PilotModelPanels.tsx`: remove optional excerpt entry,
  drop any obsolete input focus from the renderer-created preview; distinguish
  question versus feedback. Current feedback remains honestly session-only.
- `ReflectionJourney.tsx`, `pilotModelCopy.ts`: remove connected demo
  controls, retain offline behavior, keep earlier genuine Gemini pairs
  readable and independently user-authored responses distinct. No DB cleanup.
- `ReflectionJourney.test.tsx` and Android source contract: nine new locale
  regressions; synchronize only the authorized readback-renderer block pin.
  Mutation handler and native storage pins remain unchanged.
- Product proposal `docs/product/01_Android_Model_Pilot_R3_Proposal.md`,
  architecture 24 and runbook 15: record Founder observations and all seven
  requests, proposed consolidated key/feedback/follow-up boundary, and the
  deferred time-range cross-moment idea routed through ADR-0010.
- .ai review/plan/report and journal: preserve the R2 decisions and failed
  canonical evidence; bounded R3 revision, no machine-state Android PASS.
  The scoped manual-registration command only supports Windows/mock/current-user;
  it was not repurposed to claim independent Android acceptance.

### Host checks

TypeScript, all 580 Vitest tests and 47 Android M1/M2-A/M2-B/M2-C source
contracts passed. Focused presentation/controller tests: 94 passed.
All 107 tracked native inputs match the frozen 3101 input manifest.
Git whitespace check passed. No native, schema, manifest or credential
implementation was changed by this UI-only follow-up.

Canonical verification again stopped with exit 1: project-local pnpm
missing. Full verification and theory-completion were not marked passed.
Installed 3101 does not contain these new UI changes; no new package,
signing, install, live provider call, staging, commit or push occurred.

### Deferred implementation and stop

Remembered key, durable reviewed feedback and same-moment supplements are
one proposed next implementation, not three intermediate APKs. Founder
decision required for expanded secret retention, canonical durable-feedback
lifecycle/upgrade, and disclosed per-turn provider input. No automatic
persistence, broad history, background Pattern analysis or accepted ADR
status change. The cross-moment idea is record-only for later discussion.

## Authorized R3 continuity work: partial completion and concrete schema gate

### Completed: item 1 source and host tests

Connected-only native Java vault and Rust bridge implement opt-in remembered
key with Android Keystore AES-GCM, app-private no-backup AtomicFile, session-
only replacement and explicit forget. No renderer getter, secret preference,
SQLite credential or plaintext fallback. Failures are bounded; disabled state
is written before wrapping-key destruction. Credential commands wait for
mobile-plugin work from a blocking worker rather than the WebView/UI thread.

Settings has the three-language opt-in, success/error states, metadata-only
status restore, no password refill and clearing even an unreadable credential.
App startup restores readiness without a stale startup reply overriding a
later key change. Existing question/answer and session-only feedback behavior
are retained. The standalone source-staging guard rejects missing, linked or
stale native Java files before a future connected build can proceed.

Files: AndroidM2CApp/settings/model copy and IPC, native model/lib/build,
new android_pilot_credentials.rs, android-pilot-online/java and README,
build_support/pilot_credentials_inputs.rs, Java host test and host runner.
Only exact feature-gated vault registration is exempted from old source
contracts; no general plugin exemption or schema verification relaxation.

### Not completed: items 5 and 6

Actual v5 constraints cannot represent independent retained feedback and
parent-feedback-bound supplements. Pattern requires confirmed Evidence;
the R2 question intentionally has none. Reflection is not arbitrary
conversation content. Fixed schema/version and dependency/provenance contracts
forbid appending improvised records while claiming exact v5. No fake Evidence,
answer overload, sidecar persistence, DB migration or accepted ADR amendment.

Proposed ADR-0013 describes isolated Model Pilot v6, normalized typed model
turns/provenance/dependencies/events, precise original/answer/turn revision
binding, explicit reviewed save, transitive invalidation/deletion and owned
upgrade/rollback constraints. It is Proposed, not accepted. Product proposal,
architecture 24 and runbook 15 reflect authorization and this actual partial
status. The broader R3 goal is NOT complete; no key-only APK is requested.

### Checks and evidence

- TypeScript passed; all 587 frontend tests and 50 Android contracts passed.
- Java injected-backend lifecycle: 26 checks; Android adapter API compilation
  and the repo-owned explicit-input host runner passed.
- Focused native model/staging host tests: 8 passed; offline identity: 1 passed.
- All baseline schema/journal-writer inputs still match the frozen 3101 input.
- Canonical verify again exits 1 at missing local pnpm. No full PASS; no
  theory/completion transition. Earlier failed run evidence is preserved.
- First javac command failed the host's default encoding; explicit UTF-8 is
  now required. First all-frontend run found the old session-IPC expectation
  missing the new explicit false argument; synchronized that expectation and
  added opt-in and restore/error regressions without weakening checks.
- Standalone evidence: `D:\Lin\Project\LifeOS-R3-Credentials-Host-20261118\RESULT.md`;
  source-inputs-final.json SHA-256
  `188A0A65691BBC50498E84D59CCB2F2BFFC778C0CA1AC1AAF06A4EE352DA3A4D`.

### Actual limits and Git state

No real Android Keystore, mobile plugin runtime, secret, actual provider,
APK build/sign/install, device action, commit or push. Work remains in the
existing dirty worktree with an empty index; protected original/036 untouched.
Host results do not prove phone crypto storage, update continuity or the
new persistence/follow-up journey. Stop for the exact schema/upgrade decision;
do not ask Founder to repeat the already approved user-flow decisions.
