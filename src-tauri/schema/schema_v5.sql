PRAGMA foreign_keys = ON;
PRAGMA defer_foreign_keys = ON;

CREATE TABLE schema_migration_receipts (
  migration_id TEXT PRIMARY KEY NOT NULL,
  from_version INTEGER NOT NULL,
  to_version INTEGER NOT NULL,
  state TEXT NOT NULL CHECK(state = 'committed'),
  application_version TEXT NOT NULL,
  backup_id TEXT,
  source_manifest_digest TEXT NOT NULL
    CHECK(length(source_manifest_digest) = 64
      AND source_manifest_digest NOT GLOB '*[^0-9a-f]*'),
  target_manifest_digest TEXT NOT NULL
    CHECK(length(target_manifest_digest) = 64
      AND target_manifest_digest NOT GLOB '*[^0-9a-f]*'),
  started_at TEXT NOT NULL,
  committed_at TEXT NOT NULL,
  CHECK(from_version = 4 AND to_version = 5)
);

CREATE TABLE database_contract (
  singleton INTEGER PRIMARY KEY NOT NULL CHECK(singleton = 1),
  authoritative_schema INTEGER NOT NULL CHECK(authoritative_schema = 5),
  minimum_application_version TEXT NOT NULL,
  compatibility_projection TEXT NOT NULL
    CHECK(compatibility_projection IN ('enabled','disabled')),
  lifecycle_writes TEXT NOT NULL
    CHECK(lifecycle_writes IN ('disabled','enabled')),
  export_v2 TEXT NOT NULL
    CHECK(export_v2 IN ('disabled','enabled')),
  updated_at TEXT NOT NULL
);

-- This is a transaction-scoped compatibility guard, not a security boundary.
-- A typed Rust write inserts one unpredictable token, performs the v5 and v4
-- projection writes on the same connection, deletes the token, then commits.
CREATE TABLE v5_compatibility_write_guard (
  token TEXT PRIMARY KEY NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE provenance_records (
  id TEXT PRIMARY KEY NOT NULL,
  fingerprint TEXT NOT NULL UNIQUE
    CHECK(length(fingerprint) = 64
      AND fingerprint NOT GLOB '*[^0-9a-f]*'),
  origin TEXT NOT NULL
    CHECK(origin IN ('user','ai','local_mock','legacy_unknown')),
  provider TEXT
    CHECK(provider IS NULL OR provider IN ('openai','gemini','mock','legacy_unknown')),
  model TEXT,
  harness_version TEXT,
  prompt_version TEXT,
  generated_at TEXT,
  canonical_payload TEXT NOT NULL CHECK(json_valid(canonical_payload)),
  created_at TEXT NOT NULL
);

-- Created before source_heads intentionally. SQLite resolves the deferred
-- cyclic relationship after both tables exist and the transaction commits.
CREATE TABLE source_revisions (
  id TEXT PRIMARY KEY NOT NULL,
  source_id TEXT NOT NULL,
  revision_number INTEGER NOT NULL CHECK(revision_number >= 1),
  predecessor_revision_id TEXT,
  authorship TEXT NOT NULL
    CHECK(authorship IN ('user','legacy_unknown')),
  revision_reason TEXT NOT NULL
    CHECK(revision_reason IN ('created','corrected','legacy_v4_baseline')),
  serialization_version TEXT NOT NULL
    CHECK(serialization_version IN ('utf8-text-v1','legacy-v4-raw')),
  content_digest TEXT NOT NULL
    CHECK(length(content_digest) = 64
      AND content_digest NOT GLOB '*[^0-9a-f]*'),
  created_at TEXT NOT NULL,
  UNIQUE(source_id, revision_number),
  UNIQUE(source_id, id),
  FOREIGN KEY(source_id) REFERENCES source_heads(id)
    ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
  FOREIGN KEY(source_id, predecessor_revision_id)
    REFERENCES source_revisions(source_id, id)
    DEFERRABLE INITIALLY DEFERRED,
  CHECK((revision_number = 1 AND predecessor_revision_id IS NULL)
     OR (revision_number > 1 AND predecessor_revision_id IS NOT NULL))
);

CREATE TABLE source_heads (
  id TEXT PRIMARY KEY NOT NULL,
  current_revision_id TEXT,
  lifecycle_state TEXT NOT NULL
    CHECK(lifecycle_state IN ('active','deleted')),
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(id, current_revision_id)
    REFERENCES source_revisions(source_id, id)
    DEFERRABLE INITIALLY DEFERRED,
  CHECK((lifecycle_state = 'active' AND current_revision_id IS NOT NULL)
     OR (lifecycle_state = 'deleted' AND current_revision_id IS NULL))
);

CREATE TABLE source_revision_content (
  revision_id TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  byte_length INTEGER NOT NULL CHECK(byte_length >= 0),
  FOREIGN KEY(revision_id) REFERENCES source_revisions(id) ON DELETE CASCADE
);

CREATE TABLE source_revision_provenance (
  source_revision_id TEXT NOT NULL,
  role TEXT NOT NULL CHECK(role = 'content'),
  provenance_id TEXT NOT NULL,
  PRIMARY KEY(source_revision_id, role),
  FOREIGN KEY(source_revision_id) REFERENCES source_revisions(id) ON DELETE CASCADE,
  FOREIGN KEY(provenance_id) REFERENCES provenance_records(id)
);

CREATE TABLE artifact_revisions (
  id TEXT PRIMARY KEY NOT NULL,
  artifact_id TEXT NOT NULL,
  source_id TEXT NOT NULL,
  revision_number INTEGER NOT NULL CHECK(revision_number >= 1),
  predecessor_revision_id TEXT,
  authorship TEXT NOT NULL
    CHECK(authorship IN ('user','ai','local_mock','legacy_unknown','mixed')),
  revision_reason TEXT NOT NULL
    CHECK(revision_reason IN (
      'created','corrected','answered','legacy_v4_baseline'
    )),
  serialization_version TEXT NOT NULL
    CHECK(serialization_version IN ('canonical-json-v1','legacy-v4-raw')),
  content_digest TEXT NOT NULL
    CHECK(length(content_digest) = 64
      AND content_digest NOT GLOB '*[^0-9a-f]*'),
  created_at TEXT NOT NULL,
  UNIQUE(artifact_id, revision_number),
  UNIQUE(artifact_id, id),
  FOREIGN KEY(artifact_id) REFERENCES artifact_heads(id)
    ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
  FOREIGN KEY(source_id) REFERENCES source_heads(id) ON DELETE CASCADE,
  FOREIGN KEY(artifact_id, predecessor_revision_id)
    REFERENCES artifact_revisions(artifact_id, id)
    DEFERRABLE INITIALLY DEFERRED,
  CHECK((revision_number = 1 AND predecessor_revision_id IS NULL)
     OR (revision_number > 1 AND predecessor_revision_id IS NOT NULL))
);

CREATE TABLE artifact_heads (
  id TEXT PRIMARY KEY NOT NULL,
  source_id TEXT NOT NULL,
  artifact_kind TEXT NOT NULL CHECK(artifact_kind IN (
    'evidence','reflection','pattern','recovery_turn','historical_question'
  )),
  current_revision_id TEXT,
  review_state TEXT NOT NULL CHECK(review_state IN (
    'pending','confirmed','rejected','skipped','not_applicable'
  )),
  lifecycle_state TEXT NOT NULL CHECK(lifecycle_state IN (
    'active','invalidated','content_purged','deleted'
  )),
  eligibility_state TEXT NOT NULL
    CHECK(eligibility_state IN ('eligible','ineligible')),
  eligibility_reason TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE(id, source_id),
  FOREIGN KEY(source_id) REFERENCES source_heads(id) ON DELETE CASCADE,
  FOREIGN KEY(id, current_revision_id)
    REFERENCES artifact_revisions(artifact_id, id)
    DEFERRABLE INITIALLY DEFERRED,
  CHECK((lifecycle_state IN ('active','invalidated') AND current_revision_id IS NOT NULL)
     OR (lifecycle_state IN ('content_purged','deleted') AND current_revision_id IS NULL)),
  CHECK(lifecycle_state = 'active' OR eligibility_state = 'ineligible')
);

CREATE TABLE artifact_revision_content (
  revision_id TEXT PRIMARY KEY NOT NULL,
  payload TEXT NOT NULL CHECK(json_valid(payload)),
  byte_length INTEGER NOT NULL CHECK(byte_length >= 0),
  FOREIGN KEY(revision_id) REFERENCES artifact_revisions(id) ON DELETE CASCADE
);

CREATE TABLE artifact_revision_provenance (
  artifact_revision_id TEXT NOT NULL,
  role TEXT NOT NULL CHECK(role IN ('content','prompt','response')),
  provenance_id TEXT NOT NULL,
  PRIMARY KEY(artifact_revision_id, role),
  FOREIGN KEY(artifact_revision_id) REFERENCES artifact_revisions(id) ON DELETE CASCADE,
  FOREIGN KEY(provenance_id) REFERENCES provenance_records(id)
);

CREATE TABLE artifact_review_events (
  id TEXT PRIMARY KEY NOT NULL,
  artifact_id TEXT NOT NULL,
  subject_revision_id TEXT NOT NULL,
  decision TEXT NOT NULL CHECK(decision IN ('confirmed','rejected','skipped')),
  actor TEXT NOT NULL CHECK(actor IN ('user','legacy_import')),
  event_origin TEXT NOT NULL
    CHECK(event_origin IN ('explicit_user_action','legacy_v4_baseline')),
  occurred_at TEXT NOT NULL,
  timestamp_quality TEXT NOT NULL CHECK(timestamp_quality IN (
    'exact_action_time','record_updated_at_not_decision_time'
  )),
  UNIQUE(artifact_id, subject_revision_id),
  FOREIGN KEY(artifact_id, subject_revision_id)
    REFERENCES artifact_revisions(artifact_id, id) ON DELETE CASCADE,
  CHECK((actor = 'user'
      AND event_origin = 'explicit_user_action'
      AND timestamp_quality = 'exact_action_time')
     OR (actor = 'legacy_import'
      AND event_origin = 'legacy_v4_baseline'
      AND timestamp_quality = 'record_updated_at_not_decision_time'))
);

CREATE TABLE artifact_lifecycle_events (
  id TEXT PRIMARY KEY NOT NULL,
  artifact_id TEXT NOT NULL,
  subject_revision_id TEXT,
  related_revision_id TEXT,
  dependency_id TEXT,
  event_type TEXT NOT NULL CHECK(event_type IN (
    'baseline_imported','created','corrected','superseded',
    'invalidated','deleted','content_purged'
  )),
  actor TEXT NOT NULL CHECK(actor IN ('user','system','legacy_import')),
  reason_code TEXT NOT NULL,
  occurred_at TEXT NOT NULL,
  FOREIGN KEY(artifact_id) REFERENCES artifact_heads(id) ON DELETE CASCADE,
  FOREIGN KEY(artifact_id, subject_revision_id)
    REFERENCES artifact_revisions(artifact_id, id) ON DELETE CASCADE,
  FOREIGN KEY(artifact_id, related_revision_id)
    REFERENCES artifact_revisions(artifact_id, id) ON DELETE CASCADE,
  FOREIGN KEY(dependency_id) REFERENCES artifact_dependencies(id)
    DEFERRABLE INITIALLY DEFERRED,
  CHECK(event_type NOT IN ('corrected','superseded')
     OR (subject_revision_id IS NOT NULL AND related_revision_id IS NOT NULL)),
  CHECK(event_type <> 'content_purged' OR subject_revision_id IS NOT NULL),
  CHECK(event_type <> 'invalidated' OR dependency_id IS NOT NULL)
);

CREATE TABLE artifact_dependencies (
  id TEXT PRIMARY KEY NOT NULL,
  dependent_artifact_id TEXT NOT NULL,
  dependent_revision_id TEXT NOT NULL,
  relationship_type TEXT NOT NULL CHECK(relationship_type IN (
    'derived_from_experience','uses_evidence','answers_prompt',
    'uses_reflection_response','historical_current_experience',
    'historical_packet_item'
  )),
  source_revision_id TEXT,
  source_artifact_id TEXT,
  source_artifact_revision_id TEXT,
  created_at TEXT NOT NULL,
  FOREIGN KEY(dependent_artifact_id, dependent_revision_id)
    REFERENCES artifact_revisions(artifact_id, id) ON DELETE CASCADE,
  FOREIGN KEY(source_revision_id) REFERENCES source_revisions(id),
  FOREIGN KEY(source_artifact_id, source_artifact_revision_id)
    REFERENCES artifact_revisions(artifact_id, id),
  CHECK((source_revision_id IS NOT NULL
      AND source_artifact_id IS NULL
      AND source_artifact_revision_id IS NULL)
     OR (source_revision_id IS NULL
      AND source_artifact_id IS NOT NULL
      AND source_artifact_revision_id IS NOT NULL))
);

CREATE TABLE content_tombstones (
  id TEXT PRIMARY KEY NOT NULL,
  subject_type TEXT NOT NULL CHECK(subject_type IN (
    'source_revision','artifact_revision','artifact'
  )),
  source_id TEXT,
  source_revision_id TEXT,
  artifact_id TEXT,
  artifact_revision_id TEXT,
  content_digest TEXT
    CHECK(content_digest IS NULL OR (
      length(content_digest) = 64
      AND content_digest NOT GLOB '*[^0-9a-f]*'
    )),
  reason_code TEXT NOT NULL CHECK(reason_code IN (
    'user_purged_revision','user_deleted_artifact','rejected_content_purged'
  )),
  purged_at TEXT NOT NULL,
  CHECK((subject_type = 'source_revision'
      AND source_id IS NOT NULL AND source_revision_id IS NOT NULL
      AND artifact_id IS NULL AND artifact_revision_id IS NULL)
     OR (subject_type = 'artifact_revision'
      AND source_id IS NULL AND source_revision_id IS NULL
      AND artifact_id IS NOT NULL AND artifact_revision_id IS NOT NULL)
     OR (subject_type = 'artifact'
      AND source_id IS NULL AND source_revision_id IS NULL
      AND artifact_id IS NOT NULL AND artifact_revision_id IS NULL)),
  FOREIGN KEY(source_id, source_revision_id)
    REFERENCES source_revisions(source_id, id) ON DELETE CASCADE,
  FOREIGN KEY(artifact_id) REFERENCES artifact_heads(id) ON DELETE CASCADE,
  FOREIGN KEY(artifact_id, artifact_revision_id)
    REFERENCES artifact_revisions(artifact_id, id) ON DELETE CASCADE
);

CREATE TABLE historical_question_lifecycle_links (
  historical_artifact_id TEXT PRIMARY KEY NOT NULL,
  artifact_id TEXT NOT NULL UNIQUE,
  FOREIGN KEY(historical_artifact_id)
    REFERENCES historical_question_artifacts(id) ON DELETE CASCADE,
  FOREIGN KEY(artifact_id) REFERENCES artifact_heads(id) ON DELETE CASCADE
);

CREATE INDEX idx_source_revisions_source
  ON source_revisions(source_id, revision_number DESC);
CREATE INDEX idx_artifact_heads_source_kind
  ON artifact_heads(source_id, artifact_kind, created_at);
CREATE INDEX idx_artifact_heads_current_state
  ON artifact_heads(lifecycle_state, review_state, eligibility_state);
CREATE INDEX idx_artifact_revisions_artifact
  ON artifact_revisions(artifact_id, revision_number DESC);
CREATE INDEX idx_review_events_artifact_time
  ON artifact_review_events(artifact_id, occurred_at);
CREATE INDEX idx_lifecycle_events_artifact_time
  ON artifact_lifecycle_events(artifact_id, occurred_at);
CREATE INDEX idx_dependencies_dependent
  ON artifact_dependencies(dependent_artifact_id, dependent_revision_id);
CREATE INDEX idx_dependencies_source_revision
  ON artifact_dependencies(source_revision_id)
  WHERE source_revision_id IS NOT NULL;
CREATE INDEX idx_dependencies_source_artifact_revision
  ON artifact_dependencies(source_artifact_id, source_artifact_revision_id)
  WHERE source_artifact_revision_id IS NOT NULL;
CREATE UNIQUE INDEX uq_dependencies_source_revision
  ON artifact_dependencies(
    dependent_revision_id, relationship_type, source_revision_id
  ) WHERE source_revision_id IS NOT NULL;
CREATE UNIQUE INDEX uq_dependencies_source_artifact_revision
  ON artifact_dependencies(
    dependent_revision_id, relationship_type,
    source_artifact_id, source_artifact_revision_id
  ) WHERE source_artifact_revision_id IS NOT NULL;
CREATE INDEX idx_tombstones_artifact
  ON content_tombstones(artifact_id, purged_at)
  WHERE artifact_id IS NOT NULL;

CREATE TRIGGER source_revisions_immutable_update
BEFORE UPDATE ON source_revisions
BEGIN
  SELECT RAISE(ABORT, 'source_revision_immutable');
END;

CREATE TRIGGER artifact_revisions_immutable_update
BEFORE UPDATE ON artifact_revisions
BEGIN
  SELECT RAISE(ABORT, 'artifact_revision_immutable');
END;

CREATE TRIGGER provenance_records_immutable_update
BEFORE UPDATE ON provenance_records
BEGIN
  SELECT RAISE(ABORT, 'provenance_immutable');
END;

CREATE TRIGGER review_events_immutable_update
BEFORE UPDATE ON artifact_review_events
BEGIN
  SELECT RAISE(ABORT, 'review_event_immutable');
END;

CREATE TRIGGER lifecycle_events_immutable_update
BEFORE UPDATE ON artifact_lifecycle_events
BEGIN
  SELECT RAISE(ABORT, 'lifecycle_event_immutable');
END;

CREATE TRIGGER dependencies_immutable_update
BEFORE UPDATE ON artifact_dependencies
BEGIN
  SELECT RAISE(ABORT, 'dependency_immutable');
END;

CREATE TRIGGER source_revision_content_immutable_update
BEFORE UPDATE ON source_revision_content
BEGIN
  SELECT RAISE(ABORT, 'source_revision_content_immutable');
END;

CREATE TRIGGER artifact_revision_content_immutable_update
BEFORE UPDATE ON artifact_revision_content
BEGIN
  SELECT RAISE(ABORT, 'artifact_revision_content_immutable');
END;

CREATE TRIGGER tombstones_immutable_update
BEFORE UPDATE ON content_tombstones
BEGIN
  SELECT RAISE(ABORT, 'content_tombstone_immutable');
END;

CREATE TRIGGER migration_receipts_immutable_update
BEFORE UPDATE ON schema_migration_receipts
BEGIN
  SELECT RAISE(ABORT, 'schema_migration_receipt_immutable');
END;

CREATE TRIGGER migration_receipts_immutable_delete
BEFORE DELETE ON schema_migration_receipts
BEGIN
  SELECT RAISE(ABORT, 'schema_migration_receipt_immutable');
END;

CREATE TRIGGER source_revision_provenance_immutable_update
BEFORE UPDATE ON source_revision_provenance
BEGIN
  SELECT RAISE(ABORT, 'source_revision_provenance_immutable');
END;

CREATE TRIGGER artifact_revision_provenance_immutable_update
BEFORE UPDATE ON artifact_revision_provenance
BEGIN
  SELECT RAISE(ABORT, 'artifact_revision_provenance_immutable');
END;

CREATE TRIGGER historical_lifecycle_links_immutable_update
BEFORE UPDATE ON historical_question_lifecycle_links
BEGIN
  SELECT RAISE(ABORT, 'historical_lifecycle_link_immutable');
END;

-- database_contract is intentionally a mutable operational projection. Only a
-- typed transaction holding the compatibility token may create or change it.
CREATE TRIGGER database_contract_insert_requires_guard
BEFORE INSERT ON database_contract
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'database_contract_requires_lifecycle_transaction');
END;

CREATE TRIGGER database_contract_update_requires_guard
BEFORE UPDATE ON database_contract
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'database_contract_requires_lifecycle_transaction');
END;

CREATE TRIGGER database_contract_delete_requires_guard
BEFORE DELETE ON database_contract
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'database_contract_requires_lifecycle_transaction');
END;

-- Guard rows are intentionally ephemeral transaction-local capability records:
-- INSERT and DELETE are required, but rewriting a token in place is forbidden.
CREATE TRIGGER compatibility_guard_immutable_update
BEFORE UPDATE ON v5_compatibility_write_guard
BEGIN
  SELECT RAISE(ABORT, 'compatibility_guard_token_immutable');
END;

CREATE TRIGGER source_revisions_delete_requires_guard
BEFORE DELETE ON source_revisions
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'source_revision_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER artifact_revisions_delete_requires_guard
BEFORE DELETE ON artifact_revisions
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'artifact_revision_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER provenance_delete_requires_guard
BEFORE DELETE ON provenance_records
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'provenance_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER review_event_delete_requires_guard
BEFORE DELETE ON artifact_review_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'review_event_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER lifecycle_event_delete_requires_guard
BEFORE DELETE ON artifact_lifecycle_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'lifecycle_event_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER dependency_delete_requires_guard
BEFORE DELETE ON artifact_dependencies
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'dependency_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER tombstone_delete_requires_guard
BEFORE DELETE ON content_tombstones
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'tombstone_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER source_revision_content_delete_requires_guard
BEFORE DELETE ON source_revision_content
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'source_content_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER artifact_revision_content_delete_requires_guard
BEFORE DELETE ON artifact_revision_content
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'artifact_content_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER source_revision_provenance_delete_requires_guard
BEFORE DELETE ON source_revision_provenance
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'source_revision_provenance_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER artifact_revision_provenance_delete_requires_guard
BEFORE DELETE ON artifact_revision_provenance
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'artifact_revision_provenance_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER historical_lifecycle_link_delete_requires_guard
BEFORE DELETE ON historical_question_lifecycle_links
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'historical_lifecycle_link_delete_requires_lifecycle_transaction');
END;

CREATE TRIGGER source_head_insert_requires_content
BEFORE INSERT ON source_heads
WHEN NEW.current_revision_id IS NOT NULL
 AND NOT EXISTS (
   SELECT 1 FROM source_revision_content
   WHERE revision_id = NEW.current_revision_id
 )
BEGIN
  SELECT RAISE(ABORT, 'current_source_revision_content_missing');
END;

CREATE TRIGGER source_head_current_requires_content
BEFORE UPDATE ON source_heads
WHEN NEW.current_revision_id IS NOT NULL
 AND NOT EXISTS (
   SELECT 1 FROM source_revision_content
   WHERE revision_id = NEW.current_revision_id
 )
BEGIN
  SELECT RAISE(ABORT, 'current_source_revision_content_missing');
END;

CREATE TRIGGER artifact_head_insert_requires_content
BEFORE INSERT ON artifact_heads
WHEN NEW.current_revision_id IS NOT NULL
 AND NOT EXISTS (
   SELECT 1 FROM artifact_revision_content
   WHERE revision_id = NEW.current_revision_id
 )
BEGIN
  SELECT RAISE(ABORT, 'current_artifact_revision_content_missing');
END;

CREATE TRIGGER artifact_head_current_requires_content
BEFORE UPDATE ON artifact_heads
WHEN NEW.current_revision_id IS NOT NULL
 AND NOT EXISTS (
   SELECT 1 FROM artifact_revision_content
   WHERE revision_id = NEW.current_revision_id
 )
BEGIN
  SELECT RAISE(ABORT, 'current_artifact_revision_content_missing');
END;

CREATE TRIGGER current_source_content_cannot_be_purged
BEFORE DELETE ON source_revision_content
WHEN EXISTS (
  SELECT 1 FROM source_heads
  WHERE current_revision_id = OLD.revision_id
)
BEGIN
  SELECT RAISE(ABORT, 'cannot_purge_current_source_revision');
END;

CREATE TRIGGER current_artifact_content_cannot_be_purged
BEFORE DELETE ON artifact_revision_content
WHEN EXISTS (
  SELECT 1 FROM artifact_heads
  WHERE current_revision_id = OLD.revision_id
)
BEGIN
  SELECT RAISE(ABORT, 'cannot_purge_current_artifact_revision');
END;

-- Existing v4/current-state tables become guarded compatibility projections.
CREATE TRIGGER guard_experience_entries_insert
BEFORE INSERT ON experience_entries
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_experience_entries_update
BEFORE UPDATE ON experience_entries
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_experience_entries_delete
BEFORE DELETE ON experience_entries
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;

CREATE TRIGGER guard_persisted_artifacts_insert
BEFORE INSERT ON persisted_artifacts
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_persisted_artifacts_update
BEFORE UPDATE ON persisted_artifacts
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_persisted_artifacts_delete
BEFORE DELETE ON persisted_artifacts
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;

CREATE TRIGGER guard_historical_consent_events_insert
BEFORE INSERT ON historical_consent_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_consent_events_update
BEFORE UPDATE ON historical_consent_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_consent_events_delete
BEFORE DELETE ON historical_consent_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;

CREATE TRIGGER guard_historical_transmission_events_insert
BEFORE INSERT ON historical_transmission_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_transmission_events_update
BEFORE UPDATE ON historical_transmission_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_transmission_events_delete
BEFORE DELETE ON historical_transmission_events
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;

CREATE TRIGGER guard_historical_question_artifacts_insert
BEFORE INSERT ON historical_question_artifacts
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_question_artifacts_update
BEFORE UPDATE ON historical_question_artifacts
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_question_artifacts_delete
BEFORE DELETE ON historical_question_artifacts
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;

CREATE TRIGGER guard_historical_artifact_dependencies_insert
BEFORE INSERT ON historical_artifact_dependencies
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_artifact_dependencies_update
BEFORE UPDATE ON historical_artifact_dependencies
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;
CREATE TRIGGER guard_historical_artifact_dependencies_delete
BEFORE DELETE ON historical_artifact_dependencies
WHEN NOT EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  SELECT RAISE(ABORT, 'schema_v5_requires_compatible_application');
END;

-- This bridge preserves ADR-0009. The guarded v5 delete command deletes the
-- historical row; the existing v4 provenance trigger removes its consent and
-- transmission, and this trigger removes the lifecycle projection.
CREATE TRIGGER delete_v5_historical_projection_before_historical_delete
BEFORE DELETE ON historical_question_artifacts
WHEN EXISTS (SELECT 1 FROM v5_compatibility_write_guard)
BEGIN
  DELETE FROM artifact_heads
  WHERE id = (
    SELECT artifact_id FROM historical_question_lifecycle_links
    WHERE historical_artifact_id = OLD.id
  );
END;
