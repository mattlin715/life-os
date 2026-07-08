---
status: Draft
version: 0.1
owner: LIN MENGLUNG
last_updated: 2026/07/08
depends:
  - docs/dev/03_Tauri_Runtime_Verification.md
  - docs/dev/05_Development_Agent_Runbook.md
  - docs/architecture/01_Local_Evidence_Store.md
referenced_by: []
---

# 06 Manual Persistence Verification

## Purpose

This document defines the manual verification flow for `ExperienceEntry` persistence.

It verifies only the local experience entry boundary.

It does not verify evidence extraction, reflection prompts, pattern notes, AI behavior, identity labels, or advice.

## Start The App

From the repository root:

```powershell
.\scripts\use-local-dev-env.ps1
pnpm run tauri:dev
```

Wait for the Life OS desktop window to open.

## Add An Experience

1. Type one experience into the main textarea.
2. Click `Save locally`.
3. Confirm the entry appears in the list below the textarea.

Expected result:

- The entry appears on screen.
- No AI analysis runs.
- No identity label appears.
- No pattern or advice is generated.

## Close The App

Close the Life OS desktop window.

Confirm the dev session exits or that port `1420` is no longer occupied.

## Reopen The App

Run again:

```powershell
.\scripts\use-local-dev-env.ps1
pnpm run tauri:dev
```

Expected result:

- The previously saved entry still appears.
- The entry content is unchanged.

## Edit An Experience

1. Click `Edit` on an existing entry.
2. Modify only the text content.
3. Click `Save edit`.
4. Confirm the updated content appears in the list.

Expected result:

- Only the entry content changes.
- `createdAt` remains unchanged.
- `updatedAt` is updated by the store.
- No AI analysis runs.
- No identity label appears.
- No pattern or advice is generated.

## Cancel An Edit

1. Click `Edit`.
2. Modify the text.
3. Click `Cancel`.

Expected result:

- The original entry remains unchanged.
- No database update is required.

## Verify Edit Persistence

1. Edit an entry.
2. Close the app.
3. Reopen the app.
4. Confirm the edited content is still present.

## Delete An Experience

1. Click `Delete` on an entry.
2. Confirm the entry disappears from the list.

Expected result:

- The entry is no longer visible.
- No related evidence, reflection, or pattern data is created.

## Verify Delete Persistence

1. Delete an entry.
2. Close the app.
3. Reopen the app.
4. Confirm the entry does not return.

## Export JSON

1. Create at least two experience entries.
2. Click `Export JSON`.
3. Choose a save location in the system save dialog.
4. Open the saved `.json` file.

Expected result:

- The file includes `exportedAt`, `app`, `version`, and `entries`.
- Each entry includes `id`, `content`, `createdAt`, and `updatedAt`.
- The file does not include identity labels.
- The file does not include evidence, reflection, pattern, advice, or AI interpretation.

## Export Markdown

1. Create at least two experience entries.
2. Click `Export Markdown`.
3. Choose a save location in the system save dialog.
4. Open the saved `.md` file.

Expected result:

- The file starts with `# Life OS Experience Export`.
- The file includes an export timestamp.
- Each entry is readable as plain Markdown.
- The file does not include identity labels.
- The file does not include evidence, reflection, pattern, advice, or AI interpretation.

## Import JSON

1. Create at least two experience entries.
2. Click `Export JSON`.
3. Save the `.json` file.
4. Delete one or all of the exported entries.
5. Click `Import JSON`.
6. Choose the previously exported `.json` file.

Expected result:

- The imported entries appear in the list.
- Imported entries preserve their original `createdAt` and `updatedAt`.
- Import reports the imported count.
- No AI analysis runs.
- No identity label appears.
- No pattern or advice is generated.

## Verify Duplicate Handling

1. Import the same JSON file again.
2. Confirm existing entries are not overwritten.

Expected result:

- Duplicate entries are skipped.
- The UI reports the skipped count.
- Existing edited entries remain unchanged.

## Verify Invalid Import Handling

1. Create or select a JSON file that is not a Life OS export.
2. Click `Import JSON`.
3. Choose the invalid file.

Expected result:

- The UI shows an error message.
- No entries are imported.
- The SQLite schema remains unchanged.

## SQLite DB Path Examples

Windows:

```text
C:\Users\<USER>\AppData\Roaming\com.lifeos.app\life-os.db
```

macOS:

```text
/Users/<USER>/Library/Application Support/com.lifeos.app/life-os.db
```

The current table is intentionally minimal:

```sql
experience_entries (
  id TEXT PRIMARY KEY NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
)
```

## Verification Log Template

```md
## Manual Persistence Verification Log

Date:
Machine:
OS:
Node:
pnpm:
Rust:
Cargo:

Add entry:
Close and reopen after add:
Edit entry:
Close and reopen after edit:
Delete entry:
Close and reopen after delete:
Export JSON:
Export Markdown:
Import JSON:
Duplicate import skipped:
Invalid import rejected:

SQLite DB path:
Result:
Notes:
```
