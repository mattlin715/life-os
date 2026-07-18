PRAGMA foreign_keys = ON;

CREATE TABLE experience_entries (
  id TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE evidence_candidates (
  id TEXT PRIMARY KEY NOT NULL,
  source_entry_id TEXT NOT NULL,
  payload TEXT NOT NULL,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE
);

CREATE TABLE reflection_prompts (
  id TEXT PRIMARY KEY NOT NULL,
  source_entry_id TEXT NOT NULL,
  payload TEXT NOT NULL,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE
);

CREATE TABLE pattern_notes (
  id TEXT PRIMARY KEY NOT NULL,
  source_entry_id TEXT NOT NULL,
  payload TEXT NOT NULL,
  status TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(source_entry_id) REFERENCES experience_entries(id) ON DELETE CASCADE
);

INSERT INTO experience_entries VALUES (
  'fixture-v2-experience', 'v2 fixture content',
  '2026-01-01T00:00:00.000Z', '2026-01-01T00:00:00.000Z'
);
INSERT INTO evidence_candidates VALUES (
  'fixture-v2-evidence', 'fixture-v2-experience', '{"text":"observed"}',
  'confirmed', '2026-01-01T00:00:01.000Z', '2026-01-01T00:00:01.000Z'
);
INSERT INTO reflection_prompts VALUES (
  'fixture-v2-reflection', 'fixture-v2-experience', '{"prompt":"what mattered?"}',
  'pending', '2026-01-01T00:00:02.000Z', '2026-01-01T00:00:02.000Z'
);
INSERT INTO pattern_notes VALUES (
  'fixture-v2-rejected-pattern', 'fixture-v2-experience', '{"text":"rejected"}',
  'rejected', '2026-01-01T00:00:03.000Z', '2026-01-01T00:00:03.000Z'
);

PRAGMA user_version = 2;
