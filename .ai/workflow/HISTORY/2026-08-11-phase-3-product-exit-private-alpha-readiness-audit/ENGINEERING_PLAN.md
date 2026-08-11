# Engineering Plan

Status: approved

- Sprint ID: 2026-08-11-phase-3-product-exit-private-alpha-readiness-audit
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 76bc4addd954cd14a4ab82f3e4a2369efaab8820
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-11
- Updated at: 2026-08-11

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. This is documentation and
readiness evidence only: reconcile R2, create Proposed architecture/16, audit
existing product/code/tests, recommend one slice, verify, and prepare the app
for manual review. Do not implement the recommendation or change authority.

## Existing Implementation Understanding

Production is Tauri + React + SQLite schema v4. Renderer mutation paths use
typed Rust commands for Experience and artifact/historical writes. Current
export/import is Experience-only. Private v5 modules are unregistered and use
disposable fixtures. R2 is promoted by merge `76bc4add...`; its archived
manual UI field is `not_run`.

## Affected Modules

- `docs/11_MVP.md`
- `docs/product/00_MVP_User_Flow.md`
- `docs/architecture/16_Phase_3_Product_Exit_and_Private_Alpha_Readiness_Audit.md` (new)
- repository-required `.ai/workflow/` artifacts and archive

## Proposed Design

Use a single Proposed audit document as the traceability hub. Keep its matrix
compact but exhaustive, then add focused conclusions for dogfooding, Private
Alpha, v4/v5, walkthrough quality, blockers, alternatives, and the one next
slice. Product documents receive only the smallest R2 status correction.

## Alternatives Considered

- A new ADR was rejected because no new durable moral or data-governance policy
  is introduced.
- Production v5 activation first was rejected as too coupled and risky.
- A bounded v4 complete-artifact export was rejected because ADR-0011 selects
  complete portable provenance and current v4 cannot honestly supply revision,
  rejection, tombstone, and full dependency history.
- Phase 4 was rejected because its entry and implementation authority remain
  separate.

## Data Lifecycle Impact

None. Documentation only; no database or stored UI state changes.

## SQLite Or Migration Impact

None. Production `SCHEMA_VERSION` and `user_version` remain 4; no DDL or
migration execution.

## Provenance Impact

None. Existing provenance is audited but not modified.

## Historical Context Impact

None. Existing local retrieval and governed history are described accurately;
no new loading, selection, or inference.

## Consent Impact

None. No consent or authorization state changes.

## Provider Transmission Impact

None. No provider call path or outgoing packet changes.

## Import And Export Impact

No implementation impact. The audit records Experience-only v1 portability as
a dogfooding limitation and Private Alpha blocker; complete export v2 remains
schema-v5/lifecycle work under ADR-0011.

## Test Strategy

Use existing canonical tests as traceability evidence. Do not add hidden audit
helpers unless a factual claim cannot otherwise be verified; none is expected.
Audit Matrix claims against source, tests, docs, Git history, and archived
workflow evidence.

## Repository Verification Strategy

Run `scripts/verify.ps1`, inspect the complete diff and non-ignored untracked
paths, confirm no Constitution/schema/product/provider/ContextPacket/consent
diff, no staged files, and workflow validity before archive/reset.

## Manual UI Verification

Founder-owned and not acceptance yet. Start the desktop development app and
provide a compact non-destructive walkthrough including one R2 confirmation,
core journey, restart reconstruction, history/consent, lifecycle controls,
export limitation, and EN/zh-TW/ja review.

## Rollback Or Recovery Strategy

Documentation edits are trivially reversible. The recommended packaging slice
is not implemented. If its future evidence fails, retain development startup
and schema-v4 data unchanged; do not distribute the package.

## Documentation Impact

Minimal Book One factual corrections plus new Proposed architecture/16.
Archived R2 evidence is immutable and remains historically truthful.

## ADR Impact

No new ADR. Accepted ADR-0007, ADR-0009, ADR-0010, and ADR-0011 already govern
the relevant provenance, historical consent, Phase 4, lifecycle, and export
boundaries. Architecture/16 proposes an operational readiness conclusion only.

## Risk Level

low. Only documentation and workflow evidence change; the main risk is status
overclaiming, addressed by explicit vocabulary and source citations.

## Escalation Decision

No escalation during implementation. Any discovered need to change product
code, ADR status, Book Zero, schema, provider, consent, or Phase 4 returns to a
Founder decision rather than being inferred.
