//! Canonical empty schema-v4 base used only to construct a fresh exact schema-v5 database.

pub(crate) const EMPTY_V4_BASE_SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;
CREATE TABLE experience_entries (
  id TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE TABLE persisted_artifacts (
  id TEXT PRIMARY KEY NOT NULL,
  source_entry_id TEXT NOT NULL,
  artifact_kind TEXT NOT NULL,
  payload TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE
);
CREATE INDEX idx_persisted_artifacts_source
  ON persisted_artifacts(source_entry_id, artifact_kind, created_at);
CREATE TABLE historical_consent_events (
  id TEXT PRIMARY KEY NOT NULL,
  packet_digest TEXT NOT NULL,
  payload TEXT NOT NULL,
  state TEXT NOT NULL CHECK(state IN ('granted','consumed','invalidated')),
  created_at TEXT NOT NULL,
  expires_at TEXT NOT NULL
);
CREATE TABLE historical_transmission_events (
  id TEXT PRIMARY KEY NOT NULL,
  consent_id TEXT NOT NULL,
  packet_digest TEXT NOT NULL,
  provider TEXT NOT NULL,
  model TEXT NOT NULL,
  outcome TEXT NOT NULL CHECK(outcome IN (
    'sent','failed','refused','cancelled_before_send','cancelled_after_send'
  )),
  created_at TEXT NOT NULL,
  expires_at TEXT NOT NULL,
  FOREIGN KEY(consent_id) REFERENCES historical_consent_events(id) ON DELETE CASCADE
);
CREATE TABLE historical_question_artifacts (
  id TEXT PRIMARY KEY NOT NULL,
  current_experience_id TEXT NOT NULL,
  packet_digest TEXT NOT NULL,
  payload TEXT NOT NULL,
  packet_snapshot TEXT NOT NULL,
  consent_id TEXT NOT NULL,
  transmission_id TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY(current_experience_id) REFERENCES experience_entries(id) ON DELETE CASCADE,
  FOREIGN KEY(consent_id) REFERENCES historical_consent_events(id) ON DELETE CASCADE,
  FOREIGN KEY(transmission_id) REFERENCES historical_transmission_events(id) ON DELETE CASCADE
);
CREATE TABLE historical_artifact_dependencies (
  historical_artifact_id TEXT NOT NULL,
  source_entry_id TEXT NOT NULL,
  source_artifact_id TEXT,
  source_revision TEXT NOT NULL,
  PRIMARY KEY(historical_artifact_id, source_entry_id, source_artifact_id),
  FOREIGN KEY(historical_artifact_id) REFERENCES historical_question_artifacts(id) ON DELETE CASCADE,
  FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE
);
CREATE INDEX idx_historical_dependencies_source
  ON historical_artifact_dependencies(source_entry_id, source_artifact_id);
CREATE INDEX idx_historical_questions_current
  ON historical_question_artifacts(current_experience_id, created_at);
CREATE TRIGGER delete_historical_artifacts_before_source_delete
BEFORE DELETE ON experience_entries
BEGIN
  DELETE FROM historical_question_artifacts
  WHERE id IN (
    SELECT historical_artifact_id FROM historical_artifact_dependencies
    WHERE source_entry_id = OLD.id
  );
END;
CREATE TRIGGER delete_historical_provenance_after_artifact_delete
AFTER DELETE ON historical_question_artifacts
BEGIN
  DELETE FROM historical_transmission_events WHERE id = OLD.transmission_id;
  DELETE FROM historical_consent_events WHERE id = OLD.consent_id;
END;
PRAGMA user_version = 4;
"#;
