import { useCallback, useEffect, useMemo, useState } from "react";

import { createLocalEvidenceStore } from "../shared/storage";
import type { ExperienceEntry } from "../types/domain";

export function App() {
  const store = useMemo(() => createLocalEvidenceStore(), []);
  const [body, setBody] = useState("");
  const [entries, setEntries] = useState<ExperienceEntry[]>([]);
  const [storageError, setStorageError] = useState<string | null>(null);
  const [editingEntryId, setEditingEntryId] = useState<string | null>(null);
  const [editingBody, setEditingBody] = useState("");

  const refreshEntries = useCallback(async () => {
    try {
      setEntries(await store.listExperiences());
      setStorageError(null);
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
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
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
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
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
      }
    },
    [editingEntryId, refreshEntries, store],
  );

  const beginEdit = useCallback((entry: ExperienceEntry) => {
    setEditingEntryId(entry.id);
    setEditingBody(entry.body);
    setStorageError(null);
  }, []);

  const cancelEdit = useCallback(() => {
    setEditingEntryId(null);
    setEditingBody("");
    setStorageError(null);
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
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
      }
    },
    [editingBody, refreshEntries, store],
  );

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
        </div>
        {storageError ? (
          <p className="storage-error" role="alert">
            Local storage error: {storageError}
          </p>
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
