# Engineering Plan

Status: approved

- Sprint ID: 2026-08-13-desktop-schema-v5-founder-dogfood-activation-r1
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: 9a226f7081aabc071571f4a745e1343dbdb7d927
- Working-tree digest reviewed: 26192cb490b4e07f11a113b9e218e7a6db110179666643bf9ef7598dc54b0a6e
- Created at: 2026-08-13T12:05:00Z
- Updated at: 2026-08-15T10:55:00+09:00

## Approved Product Boundary

Product Review is `approved_with_conditions`: implement only the compile-time and runtime isolated Founder candidate. Never inspect or derive the ordinary profile path. All destructive evidence is disposable; manual review uses an intentionally disposable Founder dataset.

## Existing Implementation Understanding

`sqlite.rs` owns production v4 startup and typed commands. `filesystem_safety.rs` owns exact-operation, sidecar, backup, replacement, restart, and ownership primitives. `schema_v5_migration.rs` owns accepted DDL, manifests, receipt, durable classification, and private child writers. The React store currently reads guarded v4 projections and sends typed high-level mutations. Founder Package R1 already provides an isolated identity and reproducible package verification.

## Affected Modules

Only the exact allowlist in `CURRENT_MISSION.md`: existing storage/migration/writer modules, one Founder activation module, typed frontend adapter/panel, Founder candidate config/build verifier, factual Book One/runbook/index updates, focused tests, and required workflow evidence.

## Proposed Design

1. Add Cargo feature `founder-schema-v5` and compile candidate commands only with that feature. Each command also verifies Tauri identifier `com.lifeos.founderdogfood`; the ordinary build has no callable surface.
2. Add one Founder activation module/state machine. It derives only `app_data_dir()/life-os.db`, classifies before any writable open, and reconstructs state from the live file plus exact-owned operation evidence.
3. Fresh initialization builds an empty exact-v4 staging database, runs the accepted migration core without a migration backup, verifies exact v5 closed/read-only, then publishes only into an absent live path. Existing exact v4 requires explicit authorization and one owned verified backup before the accepted orchestration.
4. Reuse filesystem ownership/state contracts, accepted migration receipt/manifests, and conservative ambiguous-COMMIT classification. No autonomous recovery action.
5. Add a typed v5 runtime façade that reads only after contract/projection reconciliation and translates currently reachable domain mutations into the promoted private writer commands. Guarded v4 rows remain projections used by the unchanged renderer read model; writes originate in v5 authority.
6. Historical consent/transmission/audit rows remain ADR-0009 authoritative records but are mutated only inside a typed guarded transaction. Historical Question creation uses the promoted normalized writer; deletion uses the accepted guarded cascade bridge.
7. Add explicit migration, backup lifecycle, delete-now, and restore commands plus a session UI. All commands are one-operation, identity-bound, and path-derived in Rust. Restore revalidates live and backup identity and requires fresh explicit authorization.
8. Add a separate Founder-v5 Tauri override and package script that passes the feature at compile time and verifies embedded identity/custom protocol/private title/absence of development server.

## Alternatives Considered

- Runtime renderer flag: rejected because it could escape the package boundary.
- Change global `SCHEMA_VERSION` to 5: rejected because ordinary profile authority is withheld.
- Duplicate migration/backup logic: rejected in favor of promoted primitives.
- Continue writing only v4 projections on a v5 DB: rejected because v5 must be authoritative.
- Automatic restore/retry: rejected by accepted fail-closed policy.

## Data Lifecycle Impact

Only the isolated Founder profile changes. Fresh profile gains exact v5. Migrated profile retains one exact-owned v4 backup for at most 30 days unless explicitly deleted. Restore replaces only the isolated live file after revalidation; it does not decrement or mutate a database in place.

## SQLite Or Migration Impact

Ordinary `SCHEMA_VERSION` remains 4. Candidate maximum is 5 only behind compile feature plus app identity. The accepted fixed DDL and migration core are reused; user_version 5 is last and exact v4 is the only migration source.

## Provenance Impact

Existing provenance is preserved. New v5 mutations use promoted append-only provenance-bearing writers. No INSERT OR REPLACE is introduced for provenance records.

## Historical Context Impact

No retrieval policy change. Existing exact packet/dependency data is migrated and current creation/deletion routes preserve ADR-0009. No whole-history loading or Phase 4 conclusion.

## Consent Impact

No policy change. Typed guarded persistence preserves per-generation/per-purpose consent and atomic consumption. Local migration authorization is not provider-use consent.

## Provider Transmission Impact

None. Provider commands, packet content/digest, store:false behavior, and no-fallback rules are unchanged.

## Import And Export Impact

Current Experience-only import/export behavior remains. Import routes through v5 Experience parity. Export v2 remains disabled.

## Test Strategy

Add path-injected Rust state-machine tests for all authorized startup, migration, interruption, receipt, restart, v5 routing, drift, retention/delete/restore, and identity cases. Add TypeScript adapter/UI/i18n/lifecycle tests. Extend package tests to prove the compile feature and isolated identity. Use only disposable roots and assert ordinary sentinel bytes unchanged.

## Repository Verification Strategy

Run focused Vitest/Node/Rust tests, `cargo clippy --all-targets --features founder-schema-v5 -- -D warnings`, candidate package verification, and canonical `scripts/verify.ps1`. Compare tracked and non-ignored untracked paths to the allowlist; confirm ignored package/test artifacts and no Constitution diff.

## Manual UI Verification

Founder-owned, one step at a time, after package preparation. Use only the isolated disposable Founder profile and the promoted v4 Founder package; confirm cancel, migration, restart, v5 journey, deletion consequences, backup lifecycle, one bounded recovery case, old-v4 refusal, fresh profile, locale/focus/narrow layout, ordinary-profile non-access, and uninstall retention.

## Rollback Or Recovery Strategy

Before cutover the verified v4 backup is preserved. Ambiguity becomes recovery-required. Restore is separate and explicit, verifies exact ownership/digests/manifests and the live data-loss window, and performs no down migration. Feature disable removes the candidate build; it does not modify v5 data.

## Documentation Impact

Create the next architecture document for candidate states and threat model. Factually synchronize architecture/13, /15, /16, Index, and the Founder package runbook without claiming manual acceptance, promotion, ordinary enablement, real-user migration, deployment, or release.

## ADR Impact

No new ADR. ADR-0007, ADR-0009, ADR-0011 and Founder-approved architecture documents provide authority. Stop if implementation exposes a new policy decision.

## Risk Level

high: this is isolated and reversible but exercises real production-quality migration/replacement and broad mutation routing. Identity, ownership, atomicity, and durable-state evidence must all fail closed.

## Escalation Decision

Proceed. Escalate only if the isolated identity cannot be enforced, ordinary data would need access, destructive down migration is required, or accepted governance is insufficient.

## Corrective Cycle 2 Plan

Founder Step 7B-R5 exposed a second bounded implementation defect after the
schema-v5 transaction committed: the synthetic v4 fixture and the promoted v4
runtime create the same eleven legacy SQLite objects using different SQL
whitespace, while the full schema-object manifest hashes `sqlite_master.sql`
bytes after only line-ending/trailing-space normalization. The exact promoted-
runtime representation therefore failed the fixture-derived digest even though
all names, records, receipt, foreign keys, and integrity were valid.

The correction will:

1. preserve the fixed schema-v5 DDL digest and existing fixture manifest;
2. add one second exact full-schema manifest for the promoted production-v4
   SQL representation, accepting no third representation;
3. add a regression that creates v4 through `sqlite::migrate_connection`, then
   performs the candidate backup/migration and durable verification;
4. expose explicit restore only when durable operation evidence is exactly
   `V5BlockedRestoreAvailable`, the live database is schema v5, and the exact
   owned v4 backup revalidates; blocked restore never retries migration or
   repairs/reclassifies the v5 database;
5. preserve the current live blocked-v5 forensic database and backup until the
   corrected package is built and the Founder explicitly chooses the recovery
   action.

No new file family is required. The existing allowlist already includes
`schema_v5_migration.rs`, `schema_v5_founder_activation.rs`, the typed Founder
state adapter, `App.tsx`, Founder backup UI/tests, i18n, architecture/13 and /17,
and workflow artifacts. No contract fixture, fixed DDL, ordinary profile,
provider, ContextPacket, consent, Phase 4, Android, or deployment path changes.

## Corrective Cycle 3 Plan

Founder Step 11A proved that the cycle-2 package can migrate and restart the
promoted runtime-v4 representation, while the first typed Experience write
still fails closed before `BEGIN IMMEDIATE` because
`schema_v5_experience_write::verify_exact_v5` retained the old single fixture
digest comparison.

The final bounded correction will:

1. reuse the migration module's exact two-manifest predicate in the Experience
   verifier, which is the shared verifier chain for the other typed writers;
2. accept no third schema-object representation and change no fixed DDL;
3. extend the production-initialized-v4 activation regression through one
   typed Experience create and durable v5 reconciliation;
4. rerun focused Rust tests, Clippy, canonical verification, and package build;
5. return only to manual Step 11A with the cycle-3 package, preserving the
   existing durable v5 database and verified v4 backup without automatic
   restore, retry, cleanup, or data mutation.
