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

INSERT INTO experience_entries VALUES (
  'fixture-v3-experience', 'v3 fixture content',
  '2026-01-02T00:00:00.000Z', '2026-01-02T00:00:00.000Z'
);
INSERT INTO persisted_artifacts VALUES (
  'fixture-v3-evidence', 'fixture-v3-experience', 'evidence',
  '{"status":"confirmed","text":"observed"}',
  '2026-01-02T00:00:01.000Z', '2026-01-02T00:00:01.000Z'
);

PRAGMA user_version = 3;
