import { useCallback, useEffect, useMemo, useState } from "react";

import { isTauri } from "@tauri-apps/api/core";
import {
  createExperienceExportFilename,
  serializeExperienceExportJson,
  serializeExperienceExportMarkdown,
} from "../shared/export/experienceExport";
import type { ExperienceExportFormat } from "../shared/export/types";
import { parseExperienceImportJson } from "../shared/import/experienceImport";
import { createLocalEvidenceStore } from "../shared/storage";
import type { ExperienceEntry } from "../types/domain";

function downloadTextFile(filename: string, content: string, mimeType: string) {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");

  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  link.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 0);
}

interface SelectedTextFile {
  filename: string;
  content: string;
}

async function saveTextFile(
  filename: string,
  content: string,
  mimeType: string,
  format: ExperienceExportFormat,
): Promise<string | null> {
  if (!isTauri()) {
    downloadTextFile(filename, content, mimeType);
    return filename;
  }

  const [{ save }, { writeTextFile }] = await Promise.all([
    import("@tauri-apps/plugin-dialog"),
    import("@tauri-apps/plugin-fs"),
  ]);
  const extension = format === "json" ? "json" : "md";
  const selectedPath = await save({
    title: "Export Life OS experiences",
    defaultPath: filename,
    filters: [
      {
        name: format === "json" ? "JSON" : "Markdown",
        extensions: [extension],
      },
    ],
  });

  if (!selectedPath) {
    return null;
  }

  await writeTextFile(selectedPath, content);
  return selectedPath;
}

async function readJsonImportFile(): Promise<SelectedTextFile | null> {
  if (!isTauri()) {
    return readBrowserTextFile(".json,application/json");
  }

  const [{ open }, { readTextFile }] = await Promise.all([
    import("@tauri-apps/plugin-dialog"),
    import("@tauri-apps/plugin-fs"),
  ]);
  const selectedPath = await open({
    title: "Import Life OS experiences",
    multiple: false,
    filters: [
      {
        name: "JSON",
        extensions: ["json"],
      },
    ],
  });

  if (!selectedPath || Array.isArray(selectedPath)) {
    return null;
  }

  return {
    filename: selectedPath,
    content: await readTextFile(selectedPath),
  };
}

function readBrowserTextFile(accept: string): Promise<SelectedTextFile | null> {
  return new Promise((resolve, reject) => {
    const input = document.createElement("input");

    input.type = "file";
    input.accept = accept;
    input.style.display = "none";

    input.addEventListener("change", () => {
      const file = input.files?.[0];
      input.remove();

      if (!file) {
        resolve(null);
        return;
      }

      file
        .text()
        .then((content) => resolve({ filename: file.name, content }))
        .catch(reject);
    });
    input.addEventListener("cancel", () => {
      input.remove();
      resolve(null);
    });

    document.body.appendChild(input);
    input.click();
  });
}

export function App() {
  const store = useMemo(() => createLocalEvidenceStore(), []);
  const [body, setBody] = useState("");
  const [entries, setEntries] = useState<ExperienceEntry[]>([]);
  const [storageError, setStorageError] = useState<string | null>(null);
  const [portabilityStatus, setPortabilityStatus] = useState<string | null>(
    null,
  );
  const [editingEntryId, setEditingEntryId] = useState<string | null>(null);
  const [editingBody, setEditingBody] = useState("");

  const refreshEntries = useCallback(async () => {
    try {
      setEntries(await store.listExperiences());
      setStorageError(null);
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
      setPortabilityStatus(null);
    }
  }, [store]);

  const saveExperience = useCallback(async () => {
    const trimmedBody = body.trim();

    if (!trimmedBody) {
      return;
    }

    try {
      await store.createExperience({ body: trimmedBody });
      setBody("");
      await refreshEntries();
      setStorageError(null);
      setPortabilityStatus(null);
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
      setPortabilityStatus(null);
    }
  }, [body, refreshEntries, store]);

  const deleteExperience = useCallback(
    async (id: string) => {
      try {
        await store.deleteExperience(id);
        if (editingEntryId === id) {
          setEditingEntryId(null);
          setEditingBody("");
        }
        await refreshEntries();
        setStorageError(null);
        setPortabilityStatus(null);
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      }
    },
    [editingEntryId, refreshEntries, store],
  );

  const beginEdit = useCallback((entry: ExperienceEntry) => {
    setEditingEntryId(entry.id);
    setEditingBody(entry.body);
    setStorageError(null);
    setPortabilityStatus(null);
  }, []);

  const cancelEdit = useCallback(() => {
    setEditingEntryId(null);
    setEditingBody("");
    setStorageError(null);
    setPortabilityStatus(null);
  }, []);

  const saveEdit = useCallback(
    async (id: string) => {
      const trimmedBody = editingBody.trim();

      if (!trimmedBody) {
        return;
      }

      try {
        await store.updateExperience(id, { body: trimmedBody });
        setEditingEntryId(null);
        setEditingBody("");
        await refreshEntries();
        setStorageError(null);
        setPortabilityStatus(null);
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      }
    },
    [editingBody, refreshEntries, store],
  );

  const exportEntries = useCallback(
    async (format: ExperienceExportFormat) => {
      try {
        const currentEntries = await store.listExperiences();
        const exportedAt = new Date().toISOString();
        const content =
          format === "json"
            ? serializeExperienceExportJson(currentEntries, exportedAt)
            : serializeExperienceExportMarkdown(currentEntries, exportedAt);
        const mimeType =
          format === "json"
            ? "application/json;charset=utf-8"
            : "text/markdown;charset=utf-8";
        const savedPath = await saveTextFile(
          createExperienceExportFilename(format, exportedAt),
          content,
          mimeType,
          format,
        );

        setStorageError(null);
        setPortabilityStatus(
          savedPath
            ? `Export saved: ${savedPath}`
            : "Export cancelled. No file was written.",
        );
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      }
    },
    [store],
  );

  const importEntries = useCallback(async () => {
    try {
      const selectedFile = await readJsonImportFile();

      if (!selectedFile) {
        setStorageError(null);
        setPortabilityStatus("Import cancelled. No file was read.");
        return;
      }

      const parsedImport = parseExperienceImportJson(selectedFile.content);
      const result = await store.importExperiences(parsedImport.entries);

      await refreshEntries();
      setStorageError(null);
      setPortabilityStatus(
        `Import complete from ${selectedFile.filename}: ${result.importedCount} imported, ${result.skippedCount} skipped.`,
      );
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
      setPortabilityStatus(null);
    }
  }, [refreshEntries, store]);

  useEffect(() => {
    void refreshEntries();
  }, [refreshEntries]);

  return (
    <main className="app-shell">
      <section className="hero">
        <p className="eyebrow">Life OS</p>
        <h1>Life OS</h1>
        <p className="principle">We Build Mirrors, Not Oracles.</p>
        <textarea
          aria-label="Experience"
          placeholder="Write one experience you want to understand."
          value={body}
          onChange={(event) => setBody(event.target.value)}
        />
        <div className="actions">
          <button
            type="button"
            disabled={!body.trim()}
            onClick={saveExperience}
          >
            Save locally
          </button>
          <button
            type="button"
            className="secondary-button"
            disabled={entries.length === 0}
            onClick={() => exportEntries("json")}
          >
            Export JSON
          </button>
          <button
            type="button"
            className="secondary-button"
            disabled={entries.length === 0}
            onClick={() => exportEntries("markdown")}
          >
            Export Markdown
          </button>
          <button
            type="button"
            className="secondary-button"
            onClick={importEntries}
          >
            Import JSON
          </button>
        </div>
        {storageError ? (
          <p className="storage-error" role="alert">
            Local storage error: {storageError}
          </p>
        ) : null}
        {portabilityStatus ? (
          <p className="export-status">{portabilityStatus}</p>
        ) : null}
        {entries.length > 0 ? (
          <section className="entry-list" aria-label="Saved experiences">
            {entries.map((entry) => (
              <article className="entry" key={entry.id}>
                <div className="entry-meta">
                  <time dateTime={entry.createdAt}>
                    Created {new Date(entry.createdAt).toLocaleString()}
                  </time>
                  {entry.updatedAt !== entry.createdAt ? (
                    <time dateTime={entry.updatedAt}>
                      Updated {new Date(entry.updatedAt).toLocaleString()}
                    </time>
                  ) : null}
                </div>
                {editingEntryId === entry.id ? (
                  <>
                    <textarea
                      aria-label="Edit experience"
                      className="edit-textarea"
                      value={editingBody}
                      onChange={(event) => setEditingBody(event.target.value)}
                    />
                    <div className="entry-actions">
                      <button
                        type="button"
                        disabled={!editingBody.trim()}
                        onClick={() => saveEdit(entry.id)}
                      >
                        Save edit
                      </button>
                      <button
                        type="button"
                        className="secondary-button"
                        onClick={cancelEdit}
                      >
                        Cancel
                      </button>
                    </div>
                  </>
                ) : (
                  <>
                    <p>{entry.body}</p>
                    <div className="entry-actions">
                      <button type="button" onClick={() => beginEdit(entry)}>
                        Edit
                      </button>
                      <button
                        type="button"
                        className="delete-button"
                        onClick={() => deleteExperience(entry.id)}
                      >
                        Delete
                      </button>
                    </div>
                  </>
                )}
              </article>
            ))}
          </section>
        ) : null}
      </section>
    </main>
  );
}
