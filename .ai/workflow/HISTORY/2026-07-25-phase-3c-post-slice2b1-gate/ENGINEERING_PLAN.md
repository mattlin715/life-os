# Engineering Plan

Status: approved

- Sprint ID: 2026-07-25-phase-3c-post-slice2b1-gate
- Artifact schema: 1.0
- Authoring role: senior_product_engineer
- Repository HEAD reviewed: a476c38ba5c4a9b19a81fbb14973aacc4adf25bd
- Working-tree digest reviewed: c75187b937c4004a1326265784139b8ea0e9bbce4213c3d7c08aa27563db8087
- Created at: 2026-07-25T12:40:00+09:00
- Updated at: 2026-07-25T12:40:00+09:00

Allowed final status: `approved`, `revision_required`, or
`human_decision_required`.

## Approved Product Boundary

Product Review is `approved_with_conditions`. Founder decision
`PHASE3C-SLICE2B2-001` authorizes only production-quality, path-injected Rust
filesystem-safety primitives exercised exclusively against synthetic disposable
app-like directories. The module must remain unregistered and unreachable from
Tauri, renderer, UI, startup, app-data, and real user databases. It must stop at
Founder diff review after canonical verification, Theory Alignment Review, and
archive/reset.

## Existing Implementation Understanding

- Production schema and `user_version` remain 4.
- Slice 2A and Slice 2B-1 are private integration-test-local code in
  `src-tauri/tests/schema_v5_backup.rs`.
- Slice 2B-1 validates synthetic schema-v4 backups and simulates logical
  replacement by copying fixture bytes, then rolls fixture bytes back on
  injected failure.
- No reusable production filesystem module, exclusive-operation contract,
  sidecar refusal, canonical ownership model, platform replacement outcome,
  or restart classifier exists.
- `src-tauri/src/lib.rs` registers only existing AI and SQLite commands.
- `sha2` is currently a dev dependency because only integration tests compute
  file digests.

## Affected Modules

- `src-tauri/src/filesystem_safety.rs`: new compile-time production module and
  its synthetic/disposable unit tests.
- `src-tauri/src/lib.rs`: private module declaration only; no command or runtime
  call site.
- `src-tauri/Cargo.toml` and `src-tauri/Cargo.lock`: move `sha2` to regular
  dependencies because production code owns digest verification.
- `docs/architecture/13_Phase_3C_Schema_v5_Migration_and_Cutover_Plan.md`:
  factual Option A authority, implementation/evidence level, and limitations.
- `.ai/workflow/`: repository-native plan, implementation report, verification,
  theory review, sprint report, event chain, and archive.

## Proposed Design

1. Add a private `filesystem_safety` Rust module compiled into the library but
   not registered or called by any production entry point.
2. Model explicit database activity as `Quiescent`, `Active`, or `Unknown`.
   Constructing an exclusive-operation guard succeeds only for `Quiescent`.
3. Open a canonical existing owned root and refuse roots or supplied live paths
   containing traversal, symlink/reparse components, aliases, or paths outside
   the direct owned root.
4. Detect `live-wal`, `live-shm`, and `live-journal` before creating operation
   files and again immediately before replacement. Presence refuses without
   changing any sidecar.
5. Generate SHA-256-derived operation IDs from process/time/counter input and
   reserve backup, staging, and content-free state files with create-new
   semantics. A test-only fixed-ID seam proves collision refusal.
6. Represent expected candidate evidence as exact database SHA-256,
   source-manifest digest, schema version 4, foreign-key success, integrity
   success, and exact-record digest. A typed async verifier boundary supplies
   SQLite/domain evidence; filesystem code recomputes the file digest itself
   and rejects any mismatch.
7. Copy only from an owned verified backup into owned staging, flush staging,
   validate staging, revalidate the backup immediately before replacement, and
   recheck quiescence/sidecars.
8. Inject two compile-time abstractions:
   - a platform-aware durability adapter for file and parent-directory sync;
   - a replacement adapter returning exactly `Committed`,
     `FailedUnchanged`, or `OutcomeUnknown`.
   There is no default product invocation or generic claim that rename is
   atomic.
9. Persist only content-free operation phase/outcome metadata in the exact
   owned state file. Restart inspection classifies prepared, staged,
   committed, completed-with-cleanup, or recovery-required without modifying
   any file.
10. Known pre-commit failures verify the live file stayed byte-identical and
    clean only exact owned incomplete staging. Unknown or post-commit failures
    preserve all evidence and return `recovery_required`.
11. Cleanup accepts only the exact operation-owned backup, staging, or state
    path and refuses live, sidecar, manifest, or unrelated paths.

## Alternatives Considered

- Extend the existing integration test only: rejected because it would not
  materially advance production-quality filesystem contracts.
- Implement real platform replacement and app-data integration now: rejected
  because the Founder explicitly withheld production invocation and real data.
- Add Tauri commands behind an unused flag: rejected because registration
  expands the product attack surface and exceeds authority.
- Infer quiescence from absent sidecars: rejected because absence is not proof
  that connections or writes are inactive.
- Automatically rollback unknown outcomes: rejected because it can overwrite
  the user's actual latest state.

## Data Lifecycle Impact

Synthetic disposable database files only. Production code defines owned
operation metadata and cleanup rules but has no runtime caller. State metadata
is content-free: operation ID, relative filenames, phase, outcome class, and
non-content error class. No Experience, artifact, packet, credential, or raw
provider error is written.

## SQLite Or Migration Impact

No migration and no SQLite DDL. `SCHEMA_VERSION` and `user_version` remain 4.
Tests may create/open disposable schema-v4 files only. No app-data or real user
database is eligible.

## Provenance Impact

No provenance semantics change. Test verification must preserve exact synthetic
record digests and ADR-0009 relationships. The filesystem module cannot create,
repair, reinterpret, or reconstruct provenance.

## Historical Context Impact

None. Historical rows appear only as opaque synthetic SQLite records used to
prove exact candidate verification. No retrieval, selection, Context Packet,
question generation, cross-experience analysis, or Phase 4 behavior.

## Consent Impact

None. No consent is created, consumed, reused, or transmitted. A future
user-facing restore requires a separate disclosure and authorization design.

## Provider Transmission Impact

None. Provider and ContextPacket code are out of scope and must have no diff.

## Import And Export Impact

None. Import/export behavior and formats remain unchanged.

## Test Strategy

Synthetic/disposable unit tests in the new module will cover:

1. quiescent guard success plus active/unknown refusal;
2. WAL/SHM/journal refusal with byte-identical sidecars;
3. canonical root/live path, traversal, outside-root, alias, and reparse
   refusal;
4. generated create-new ownership plus fixed-ID destination collision refusal;
5. digest, source-manifest, schema, FK, integrity, and exact-record mismatch;
6. staged file flush and explicit supported/unsupported/failed parent
   durability;
7. replacement `FailedUnchanged`, `OutcomeUnknown`, and `Committed` branches;
8. interruption before and after replacement and post-commit verification
   failure;
9. restart classification for prepared, staged, committed, cleanup-needed,
   corrupt, missing, and contradictory state;
10. exact-owned cleanup and refusal to remove live, sidecar, manifest, or
    unrelated files;
11. source/backup immutability and byte-identical live data for every known
    pre-commit failure;
12. schema-v4 exact-record success using a disposable SQLite verifier.

Existing Slice 2A/2B-1 and schema contract tests remain unchanged.

## Repository Verification Strategy

Run focused Rust formatting/check/tests first, then:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\verify.ps1
```

Confirm workflow validation, Vitest, all Rust suites, typecheck/build, UTF-8,
whitespace, secret, Markdown-link, and Constitution checks pass. Confirm no
Tauri registration, UI, provider, ContextPacket, startup, app-data,
`SCHEMA_VERSION`, or migration diff.

## Manual UI Verification

Not applicable. The authorized module has no UI or runtime entry point and only
disposable tests invoke it. Founder diff review remains mandatory.

## Rollback Or Recovery Strategy

Before promotion, all work is unstaged and removable as one bounded branch
diff. After any future promotion, the unregistered module can be disabled or
removed without database mutation. During tests, known failures clean only
exact owned incomplete staging; ambiguous replacement outcomes preserve every
candidate and return `recovery_required`. No automatic replay, rollback,
repair, sidecar cleanup, or candidate selection exists.

## Documentation Impact

Update architecture/13 factually to record the exact Option A authorization,
current implementation evidence, and strict limitation that this is not
production activation, real-user recovery, cross-platform atomicity proof,
schema-v5 migration, or Phase 4.

## ADR Impact

No ADR added or changed. ADR-0007, ADR-0009, ADR-0010, and ADR-0011 decisions
remain unchanged.

## Risk Level

Medium. The module is production-capable filesystem code and platform behavior
is subtle, but no product call site, app-data path, real data, schema change, or
automatic recovery is allowed. Explicit outcome and restart ambiguity reduce
the primary destructive risk.

## Escalation Decision

No new decision is required. The plan fits the exact Founder-authorized Option
A. Any need for a real app-data path, Tauri/startup/UI integration, real
database, sidecar deletion/checkpoint, default platform replacement invocation,
retention, schema v5, or autonomous recovery must stop at a new Founder gate.
