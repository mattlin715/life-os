# Engineering Plan

Status: approved

- Sprint ID: 2026-08-23-ordinary-v5-disclosure-manual-correction-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 44ec6d56d645829488aa73d0b92bcf72b19487f4
- Working-tree digest reviewed: see current workflow state
- Created at: 2026-08-23T05:00:00+09:00
- Updated at: 2026-08-23T05:00:00+09:00

## Approved Product Boundary

Product Review is approved after exact Founder resolution
ORDINARY-V5-DISCLOSURE-CORRECTION-001 Option A. Only the ordinary
migration-required disclosure, its focused tests, factual architecture/18
synchronization, workflow artifacts, canonical verification, and one ignored
unsigned installer rebuild are authorized.

## Existing Implementation Understanding

FounderSchemaV5MigrationPanel already selects ordinary title, introduction, and
local-only copy while sharing migration, backup, provider-boundary, cancel, and
button behavior. The three missing facts can therefore be added as ordinary-only
copy without altering callbacks, state, storage, commands, or Rust.

## Affected Modules

Exact anticipated implementation allowlist:

- src/app/FounderSchemaV5MigrationPanel.tsx
- src/app/FounderSchemaV5MigrationPanel.test.tsx
- src/app/i18n.ts
- src/app/i18n.test.ts
- docs/architecture/18_Desktop_Schema_v5_Ordinary_Production_Activation_R1.md
- repository-required workflow artifacts
- one ignored rebuilt installer and manifest under the existing review artifact
  boundary

## Proposed Design

Add three UiCopy values for bounded schema-v5 purpose, equal backup sensitivity,
and legacy schema-v4 write refusal. Render them inside the existing list only
when ordinary is true. Keep Founder Candidate markup byte-behavior equivalent.
Do not add controls, events, callbacks, state, persistence, or command calls.

## Alternatives Considered

Reuse shared Founder copy was rejected because it would change the explicitly
preserved Founder Candidate disclosure. Continuing without the copy was rejected
by the Founder-approved product contract. A new panel was rejected as needless
duplication.

## Data Lifecycle Impact

None. Text rendering does not read, create, mutate, back up, restore, or delete
database content or operation evidence.

## SQLite Or Migration Impact

None. No Rust, SQL, schema, DDL, version, migration state, or backup/restore
behavior is changed.

## Provenance Impact

None. Copy explains preservation goals but does not create or alter provenance.

## Historical Context Impact

None.

## Consent Impact

No provider consent change. Existing explicit migration authorization remains
separate and unchanged.

## Provider Transmission Impact

None.

## Import And Export Impact

None.

## Test Strategy

- Assert all three ordinary-only disclosures in English, Traditional Chinese,
  and Japanese.
- Assert Founder Candidate rendering excludes the new ordinary strings.
- Preserve existing explicit authorize/cancel and cancelled-state tests.
- Extend locale parity assertions for the new keys.
- Run the affected Vitest files, TypeScript typecheck, and canonical verify.

## Repository Verification Strategy

Run focused Vitest, then
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1.
Confirm no Rust or schema delta was introduced by this correction.

## Manual UI Verification

Founder-owned. Rebuild one ignored unsigned ordinary review installer with a new
exact hash, then resume only Manual Phase A Step 6A against the existing
disposable exact-v4 profile.

## Rollback Or Recovery Strategy

Before promotion, revert only the five implementation/documentation paths and
workflow evidence. The database remains untouched; no data rollback applies.

## Documentation Impact

Append a factual architecture/18 manual-evidence note recording the incomplete
Step 6A disclosure, the no-mutation Step 6A-D1 evidence, the exact Founder
resolution, and the bounded correction.

## ADR Impact

No new ADR and no ADR status change. Existing authority remains sufficient.

## Risk Level

Low implementation risk because the change is ordinary-only copy plus tests.
Product risk is controlled by direct three-language wording and Founder manual
retest. Database risk is none from this correction.

## Escalation Decision

No unresolved consequential question remains. Proceed to implementation within
the exact allowlist; stop if any non-copy runtime change appears necessary.
