# SQLite Local Evidence Store

This folder contains the SQLite implementation boundary for `ExperienceEntry`.

The current persistence spike intentionally creates only one table:

```sql
experience_entries (
  id TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
)
```

Evidence, reflection, and pattern persistence are deferred.

Do not add a migration framework here until the local data model has stabilized.
