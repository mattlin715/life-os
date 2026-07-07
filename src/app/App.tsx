import { useCallback, useEffect, useMemo, useState } from "react";

import { createLocalEvidenceStore } from "../shared/storage";
import type { ExperienceEntry } from "../types/domain";

export function App() {
  const store = useMemo(() => createLocalEvidenceStore(), []);
  const [body, setBody] = useState("");
  const [entries, setEntries] = useState<ExperienceEntry[]>([]);
  const [storageError, setStorageError] = useState<string | null>(null);

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
        await refreshEntries();
        setStorageError(null);
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
      }
    },
    [refreshEntries, store],
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
                <time dateTime={entry.createdAt}>
                  {new Date(entry.createdAt).toLocaleString()}
                </time>
                <p>{entry.body}</p>
                <button
                  type="button"
                  className="delete-button"
                  onClick={() => deleteExperience(entry.id)}
                >
                  Delete
                </button>
              </article>
            ))}
          </section>
        ) : null}
      </section>
    </main>
  );
}
