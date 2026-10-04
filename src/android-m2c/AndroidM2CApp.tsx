import { ReflectionJourney } from "./ReflectionJourney";
import { journeyCopy } from "./journeyCopy";
import { useEffect, useRef, useState } from "react";
import { createId, lifecycleRequest, m2cStore } from "./androidM2CStore";
import type { DebugPhase, LifecycleRequest, Locale, M2CStore, Snapshot } from "./androidM2CStore";
import type { ExperienceEntry } from "../types/domain";

export const copy = {
  en: {
    title: "A small moment, kept locally", summary: "Save, correct, and explicitly delete synthetic test text on this device.",
    badge: "Synthetic data only · Review build · No AI · Offline", draft: "Record a synthetic moment", text: "Text to save",
    save: "Save", saved: "Moment saved and reopened", changesSaved: "Changes saved and reopened", saving: "Working — success not yet verified", unsaved: "New moment — not saved", editUnsaved: "Changes — not saved",
    failed: "Outcome not confirmed. Do not submit a new request; reconcile the same request or reopen the current record.",
    conflict: "This revision changed. Reopen the current record before editing again.", list: "Saved on this device", empty: "Nothing saved yet.",
    open: "Open exact text", exact: "Exact saved text", edit: "Edit original text", sourceEditLabel: "Your original text", sourceHint: "You can edit your original text directly; you do not need to adopt a candidate first.", changes: "Save changes", cancel: "Cancel", close: "Close",
    del: "Delete", confirm: "Delete this moment? This will remove its current text and the text of previously saved versions from the app.",
    confirmDelete: "Confirm deletion", deleted: "Moment deleted; no active text remains", retry: "Reconcile same request",
    revisionDetails: "Versions of this moment", buildDetails: "About this test build", retention: "Editing creates a new user-authored revision, even for identical text. Prior text stays locally until deletion. Deletion purges logical text; IDs, revision digests, authorship, lineage and times remain. No secure erasure guarantee for SQLite pages or device storage.",
    limits: "com.lifeos.review.m2c · directFreshV5 · synthetic-only create/read/edit/delete. No AI, artifacts, providers, credentials, sync, import/export, backup, migration, repair or production activation. Android 36 x86_64 emulator evidence is not physical-device or power-loss proof.",
    loading: "Preparing app-private storage…", blocked: "Storage is blocked. No data was deleted or repaired.", localeError: "Language preference could not be saved.",
  },
  "zh-TW": {
    title: "在本機留下一個小片刻", summary: "在這部裝置儲存、修正，或明確刪除合成測試文字。",
    badge: "僅限合成測試資料 · 暫用審查版 · 不含 AI · 離線", draft: "記下一個合成測試片刻", text: "這次要儲存的文字",
    save: "儲存", saved: "這個片刻已儲存並重新讀取確認", changesSaved: "修改已儲存並重新讀取確認", saving: "處理中 — 尚未確認成功", unsaved: "新片刻尚未儲存", editUnsaved: "編輯內容尚未儲存",
    failed: "結果尚未確認。請勿另送新請求；請核對同一請求，或重新開啟目前的紀錄。",
    conflict: "這個版本已變更。請先重新開啟目前的紀錄，再編輯。", list: "此裝置已儲存的片刻", empty: "尚未儲存任何片刻。",
    open: "開啟已儲存的原文", exact: "已儲存的原文", edit: "編輯原文", sourceEditLabel: "你的原文", sourceHint: "可直接編輯原文，不必先採用線索候選。", changes: "儲存變更", cancel: "取消", close: "關閉",
    del: "刪除", confirm: "要刪除這個片刻嗎？這會刪除它目前的文字，以及先前儲存版本的文字。",
    confirmDelete: "確認刪除", deleted: "這個片刻已刪除；不再有可開啟的文字", retry: "核對同一請求",
    revisionDetails: "這個片刻的版本", buildDetails: "關於此測試版", retention: "編輯會建立由你撰寫的新版本，文字相同也一樣。舊版文字會留在本機，直到刪除。刪除清除邏輯內容，但保留識別碼、版本摘要、作者、前後版本關係與時間。不保證 SQLite 頁面或裝置儲存空間已安全抹除。",
    limits: "com.lifeos.review.m2c · directFreshV5 · 僅限合成測試資料的建立／讀取／編輯／刪除。不提供 AI、產出物、Provider、憑證、同步、匯入／匯出、備份、遷移、修復或正式版啟用。Android 36 x86_64 模擬器結果不代表實體手機或斷電耐久性。",
    loading: "正在準備應用程式專用的儲存空間…", blocked: "目前無法安全開啟本機儲存空間。資料未被刪除或修復。", localeError: "未能儲存語言偏好。",
  },
  ja: {
    title: "端末に残す、小さな瞬間", summary: "この端末で合成テスト用の文章を保存し、修正し、明示的に削除できます。",
    badge: "合成テストデータのみ · 一時審査版 · AI なし · オフライン", draft: "合成テスト用の瞬間を記録", text: "保存する文章",
    save: "保存", saved: "この瞬間を保存し、読み直して確認しました", changesSaved: "変更を保存し、読み直して確認しました", saving: "処理中 — 成功はまだ確認されていません", unsaved: "新しい瞬間はまだ保存していません", editUnsaved: "変更はまだ保存していません",
    failed: "結果を確認できていません。新しいリクエストは送らず、同じリクエストの結果を確認するか、現在の記録を開き直してください。",
    conflict: "この版は変更されています。現在の記録を開き直してから編集してください。", list: "この端末に保存した瞬間", empty: "保存した瞬間はまだありません。",
    open: "保存した原文を開く", exact: "保存した原文", edit: "原文を編集", sourceEditLabel: "あなたの原文", sourceHint: "候補を採用しなくても、原文は直接編集できます。", changes: "変更を保存", cancel: "キャンセル", close: "閉じる",
    del: "削除", confirm: "この瞬間を削除しますか？現在の文章と、以前に保存した版の文章がアプリから削除されます。",
    confirmDelete: "削除を確定", deleted: "この瞬間を削除しました。開ける文章は残っていません", retry: "同じリクエストを確認",
    revisionDetails: "この瞬間のバージョン", buildDetails: "このテスト版について", retention: "編集すると、同じ文章でもユーザー作成の新しい版が追加されます。以前の文章は削除するまで端末に残ります。削除後も ID、版のダイジェスト、作成者、版のつながり、時刻は残ります。SQLite ページや端末ストレージの安全な物理消去は保証しません。",
    limits: "com.lifeos.review.m2c · directFreshV5 · 合成テストデータの作成／読み出し／編集／削除のみ。AI、生成物、Provider、認証情報、同期、取り込み／書き出し、バックアップ、移行、修復、正式版の有効化には対応しません。Android 36 x86_64 エミュレーターの結果は実機や電源断の耐久性を証明しません。",
    loading: "アプリ専用ストレージを準備しています…", blocked: "ストレージを安全に開けません。データの削除や修復は行っていません。", localeError: "言語設定を保存できませんでした。",
  },
};

export function SourceListText({ entry, selectedId, locale }: { entry: ExperienceEntry; selectedId?: string; locale: Locale }) {
  return <p className="android-m2c__excerpt">{entry.id === selectedId ? journeyCopy[locale].currentSource : entry.body}</p>;
}

type StatusCopy = typeof copy[Locale];

// Presentation only: no lifecycle transition, request, timer or persistence.
export function OperationStatus({ state, c, revisionNumber = 1 }: { state: string; c: StatusCopy; revisionNumber?: number }) {
  const message = state === "saving" ? c.saving : state === "saved" ? (revisionNumber > 1 ? c.changesSaved : c.saved)
    : state === "deleted" ? c.deleted : state === "conflict" ? c.conflict : state === "failed" ? c.failed : null;
  return message ? <p role="status" data-testid="m2c-state">{message}</p> : null;
}

export function DraftStatus({ kind, visible, c }: { kind: "new" | "edit"; visible: boolean; c: StatusCopy }) {
  return visible ? <p role="status" data-testid={`m2c-${kind}-unsaved`}>{kind === "new" ? c.unsaved : c.editUnsaved}</p> : null;
}

export function AndroidM2CApp({ store = m2cStore }: { store?: M2CStore }) {
  const [locale, setLocale] = useState<Locale>("en");
  const [localeState, setLocaleState] = useState("loading");
  const [storage, setStorage] = useState("loading");
  const [entries, setEntries] = useState<ExperienceEntry[]>([]);
  const [selected, setSelected] = useState<Snapshot | null>(null);
  const [draft, setDraft] = useState("");
  const [editing, setEditing] = useState(false);
  const [editText, setEditText] = useState("");
  const [confirming, setConfirming] = useState(false);
  const [state, setState] = useState("draft");
  const [request, setRequest] = useState<LifecycleRequest | null>(null);
  const [createRequest, setCreateRequest] = useState<{ id: string; text: string } | null>(null);
  const [debugPhase, setDebugPhase] = useState<DebugPhase | undefined>();
  const busy = useRef(false);
  const [artifactLocked, setArtifactLocked] = useState(false);
  const c = { ...copy[locale], ...journeyCopy[locale].source };
  useEffect(() => {
    let active = true;
    void (async () => {
      try {
        const stored = await store.readLocale(); await store.status(); const loaded = await store.list();
        if (active) { setLocale(stored === "zh-TW" || stored === "ja" ? stored : "en"); setEntries(loaded); setStorage("ready"); setLocaleState("saved"); }
      } catch { if (active) { setStorage("blocked"); setLocaleState("failed"); } }
    })();
    return () => { active = false; };
  }, [store]);
  useEffect(() => { document.documentElement.lang = locale === "zh-TW" ? "zh-Hant" : locale; }, [locale]);

  const open = async (id: string) => {
    if (busy.current) return;
    busy.current = true;
    try { const actual = await store.get(id); setSelected(actual); setEditing(false); setEditText(""); setConfirming(false); setRequest(null); setState("draft"); }
    catch { setSelected(null); setState("failed"); }
    finally { busy.current = false; }
  };
  const save = async (retry = false) => {
    if (busy.current || (!retry && !draft.trim())) return;
    busy.current = true; setState("saving");
    const frozen = retry && createRequest ? createRequest : { id: createId(), text: draft };
    setCreateRequest(frozen);
    try {
      await store.create(frozen.id, frozen.text); const actual = await store.get(frozen.id); const loaded = await store.list();
      if (!actual || actual.entry.body !== frozen.text) throw new Error("unverified");
      setSelected(actual); setEntries(loaded); setDraft(""); setCreateRequest(null); setState("saved");
    } catch { setState("failed"); }
    finally { busy.current = false; }
  };
  const mutate = async (deleting: boolean, retry = false) => {
    if (busy.current || (!retry && (!selected || (!deleting && !editText.trim())))) return;
    busy.current = true; setState("saving");
    try {
      const frozen = retry && request ? request : await lifecycleRequest(selected!, deleting ? null : editText);
      setRequest(frozen); setConfirming(false);
      setSelected(null); setEntries([]); setEditText(""); setEditing(false);
      if (frozen.operation === "delete") { setSelected(null); setEntries([]); setEditText(""); setDraft(""); setEditing(false); setCreateRequest(null); }
      const ack = await store.mutate(frozen, debugPhase);
      if (ack.requestId !== frozen.requestId || ack.sourceId !== frozen.id) throw new Error("unverified");
      const actual = await store.get(frozen.id); const loaded = await store.list();
      if (frozen.operation === "delete" ? actual !== null || loaded.some((v) => v.id === frozen.id)
        : ack.acknowledgement === "committedNotCurrent" || !actual || actual.entry.body !== frozen.body
          || actual.predecessorRevisionId !== frozen.expectedRevisionId || actual.entry.updatedAt !== frozen.occurredAt) {
        setSelected(null); setEditText(""); setEditing(false); setEntries(loaded); setRequest(null); setState("conflict"); return;
      }
      setSelected(actual); setEntries(loaded); setEditText(""); setEditing(false); setRequest(null); setDebugPhase(undefined);
      setState(frozen.operation === "delete" ? "deleted" : "saved");
    } catch (error) {
      // No arbitrary IPC/backend error payload is rendered or logged.
      if (String(error).includes("m2c_stale_revision_preserved") || String(error).includes("m2c_request_identity_conflict")) {
        setSelected(null); setEditText(""); setEditing(false); setRequest(null); setState("conflict");
        try { setEntries(await store.list()); } catch { setEntries([]); }
      } else { setState("failed"); }
    } finally { busy.current = false; }
  };

  if (storage !== "ready") return <main className="android-m2c android-m2c--center" data-runtime="android-m2c-disposable" data-storage-state={storage}><p>{storage === "blocked" ? c.blocked : c.loading}</p></main>;
  const locked = artifactLocked || state === "saving" || request !== null || createRequest !== null;
  return <main className="android-m2c" data-runtime="android-m2c-disposable" data-storage-state="ready" data-save-state={state} data-locale-preference-state={localeState}>
    <div className="android-m2c__shell">
      <nav className="android-m2c__languages" aria-label="Language">
        {(["en", "zh-TW", "ja"] as Locale[]).map((lang) => <button key={lang} data-locale={lang} aria-pressed={locale === lang} disabled={locked || localeState === "saving"} onClick={() => {
          setLocale(lang); setLocaleState("saving"); void store.writeLocale(lang).then(() => setLocaleState("saved")).catch(() => setLocaleState("failed"));
        }}>{lang === "en" ? "English" : lang === "ja" ? "日本語" : "繁體中文"}</button>)}
      </nav>
      {localeState === "failed" ? <p role="status">{c.localeError}</p> : null}
      <header><p className="android-m2c__eyebrow">Android M2-C · Life OS</p><h1>{c.title}</h1><p>{c.summary}</p><p className="android-m2c__badge">{c.badge}</p></header>
      <OperationStatus state={state} c={c} revisionNumber={selected?.revisionNumber} />
      {state === "failed" && (request || createRequest) ? <>
        <button data-testid="m2c-retry" onClick={() => void (request ? mutate(request.operation === "delete", true) : save(true))}>{c.retry}</button>
        <button data-testid="m2c-reopen-current" onClick={() => {
          if (busy.current) return;
          const id = request?.id ?? createRequest!.id;
          busy.current = true;
          void (async () => {
            try {
              const actual = await store.get(id); const loaded = await store.list();
              setSelected(actual); setEntries(loaded); setRequest(null); setCreateRequest(null);
              setDraft(""); setEditText(""); setEditing(false); setState("draft");
            } catch { setSelected(null); setEntries([]); }
            finally { busy.current = false; }
          })();
        }}>{c.open}</button>
      </> : null}
      <section className="android-m2c__card"><h2>{c.draft}</h2><label htmlFor="m2c-draft">{c.text}</label>
        <textarea id="m2c-draft" data-testid="m2c-draft" value={draft} disabled={locked} onChange={(e) => { setDraft(e.target.value); setState("draft"); }} />
        <DraftStatus kind="new" visible={!!draft.trim() && !locked} c={c} />
        <button data-testid="m2c-save" disabled={locked || !draft.trim()} onClick={() => void save()}>{c.save}</button>
      </section>
      <section className="android-m2c__card"><h2>{c.list}</h2>{entries.length === 0 ? <p>{c.empty}</p> : <ul className="android-m2c__list">{entries.map((entry) => <li key={entry.id} data-experience-id={entry.id}><SourceListText entry={entry} selectedId={selected?.entry.id} locale={locale} /><button data-testid="m2c-open" disabled={locked} onClick={() => void open(entry.id)}>{c.open}</button></li>)}</ul>}</section>
      {selected ? <section className="android-m2c__card android-m2c__exact" data-testid="m2c-exact"><h2>{c.exact}</h2><p id="m2c-source-hint" data-testid="m2c-source-hint" className="android-m2c__secondary">{c.sourceHint}</p>
        {editing ? <><label htmlFor="m2c-edit">{c.sourceEditLabel}</label><textarea autoFocus aria-describedby="m2c-source-hint" id="m2c-edit" data-testid="m2c-edit-text" value={editText} disabled={locked} onChange={(e) => { setEditText(e.target.value); setState("draft"); }} /><DraftStatus kind="edit" visible={editText !== selected.entry.body && !locked} c={c} /><button data-testid="m2c-save-changes" disabled={locked || !editText.trim()} onClick={() => void mutate(false)}>{c.changes}</button><button data-testid="m2c-cancel-edit" disabled={locked} onClick={() => { setEditing(false); setEditText(""); }}>{c.cancel}</button></>
          : <><p id="m2c-source-content" data-testid="m2c-exact-text">{selected.entry.body}</p><button data-testid="m2c-edit" disabled={locked || confirming} onClick={() => { setEditText(selected.entry.body); setEditing(true); setState("draft"); }}>{c.edit}</button><button data-testid="m2c-delete" disabled={locked || confirming} onClick={() => setConfirming(true)}>{c.del}</button></>}
        {confirming ? <div role="dialog" aria-modal="false" aria-label={c.del}><p>{c.confirm}</p><button data-testid="m2c-confirm-delete" disabled={locked} onClick={() => void mutate(true)}>{c.confirmDelete}</button><button data-testid="m2c-cancel-delete" disabled={locked} onClick={() => setConfirming(false)}>{c.cancel}</button></div> : null}
        <button disabled={locked} data-testid="m2c-close" onClick={() => { setSelected(null); setEditing(false); setEditText(""); setConfirming(false); }}>{c.close}</button>
        <details className="android-m2c__secondary-details" data-testid="m2c-revision-details"><summary>{c.revisionDetails}</summary><code data-testid="m2c-revision">revision={selected.revisionNumber} · authoredBy={selected.authorship} · id={selected.revisionId} · predecessor={selected.predecessorRevisionId ?? "none"}</code></details>
        {!editing && !confirming ? <ReflectionJourney key={selected.revisionId} source={selected} locale={locale} store={store} onLock={setArtifactLocked} onReopen={() => void open(selected.entry.id)} /> : null}
      </section> : null}
      <details className="android-m2c__details" data-testid="m2c-build-details"><summary>{c.buildDetails}</summary><p>{journeyCopy[locale].disclosure}</p><p>{journeyCopy[locale].retention}</p><p>{c.retention}</p><p>{c.limits}</p><code>schema=5 · origin=directFreshV5 · file=android-m2c-disposable-v5.db</code></details>
      {import.meta.env.VITE_LIFE_OS_ANDROID_M2C_DEBUG_HOOKS === "1" ? <details hidden data-debug-only="source"><summary>Disposable debug probes</summary>{(["beforeCommit", "afterCommitBeforeAck", "rollbackAfterProjection"] as DebugPhase[]).map((phase) => <button key={phase} data-debug-phase={phase} disabled={locked} onClick={() => setDebugPhase(phase)}>{phase}</button>)}</details> : null}
    </div>
  </main>;
}
