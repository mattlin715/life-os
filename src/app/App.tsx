import { useCallback, useMemo, useState } from "react";

import { createInMemoryLocalEvidenceStore } from "../shared/storage";
import type { ExperienceEntry } from "../types/domain";

export function App() {
  const store = useMemo(() => createInMemoryLocalEvidenceStore(), []);
  const [body, setBody] = useState("");
  const [entries, setEntries] = useState<ExperienceEntry[]>([]);

  const refreshEntries = useCallback(async () => {
    setEntries(await store.listExperiences());
  }, [store]);

  const saveExperience = useCallback(async () => {
    const trimmedBody = body.trim();

    if (!trimmedBody) {
      return;
    }

    await store.createExperience({ body: trimmedBody });
    setBody("");
    await refreshEntries();
  }, [body, refreshEntries, store]);

  const deleteExperience = useCallback(
    async (id: string) => {
      await store.deleteExperience(id);
      await refreshEntries();
    },
    [refreshEntries, store],
  );

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
