import { useEffect, useMemo, useRef, useState } from "react";
import type { ExperienceEntry } from "../types/domain";
import {
  androidM1ExperienceStore,
  androidM1LocalePreference,
  type AndroidM1DebugPhase,
  type AndroidM1ExperienceStore,
  type AndroidM1LocalePreference,
  type AndroidM1LocalePreferenceValue,
  type AndroidM1StorageStatus,
} from "./androidM1Store";

export type AndroidM1Locale = AndroidM1LocalePreferenceValue;
type SaveState = "draft" | "saving" | "committed" | "failed";
type LocalePreferenceState = "loading" | "saved" | "saving" | "failed";

export const ANDROID_M1_LOCALE_PREFERENCE_FILE = "android-m1-locale.pref";

type Copy = {
  language: string; eyebrow: string; title: string; summary: string; synthetic: string;
  draftHeading: string; draftLabel: string; placeholder: string; draftState: string;
  savingState: string; committedState: string; failedState: string; save: string; retry: string;
  listHeading: string; emptyList: string; open: string; exactHeading: string; close: string;
  loading: string; blocked: string; details: string; boundaries: readonly string[];
  supported: string; unsupported: string; debug: string; beforeCommit: string; afterCommit: string;
};

export const androidM1Copy: Record<AndroidM1Locale, Copy> = {
  en: {
    language: "Language", eyebrow: "Disposable native review · M1", title: "A small, local Experience",
    summary: "Write synthetic text, commit it explicitly, and reopen exactly what was saved.",
    synthetic: "Synthetic review data only · not production · no AI",
    draftHeading: "New synthetic Experience", draftLabel: "Text to save",
    placeholder: "For example: 今天，我想記住静かな勇気。", draftState: "Draft — not saved",
    savingState: "Saving — no success has been acknowledged yet",
    committedState: "Committed — SQLite confirmed the transaction",
    failedState: "Save failed — your draft is still here", save: "Save explicitly",
    retry: "Retry same request", listHeading: "Committed on this device",
    emptyList: "Nothing committed yet.", open: "Open exact text", exactHeading: "Exact committed text",
    close: "Close", loading: "Preparing app-private storage…",
    blocked: "Storage is blocked. No data was deleted or repaired.", details: "Technical boundaries",
    boundaries: [
      "Temporary application ID: com.lifeos.review.m1.",
      "App-private exact schema v5; no desktop import, v4 fallback, repair, migration, backup, or transfer.",
      "Only create, list, and get are supported. Update, delete, AI, Evidence, Patterns, and daily reflection are absent.",
      "Commit acknowledgement is not proof of emulator crash, host crash, or physical power-loss durability.",
    ],
    supported: "Supported: createExperience, listExperiences, getExperience.",
    unsupported: "Unsupported: update/delete/import, artifacts/history, recovery, export, sync.",
    debug: "Disposable debug probes", beforeCommit: "Hold next save before commit",
    afterCommit: "Hold next save after commit, before acknowledgement",
  },
  "zh-TW": {
    language: "語言", eyebrow: "Android M1 · 暫用原生審查版", title: "在本機留下一個小片刻",
    summary: "寫下一個合成測試片刻，明確儲存，再重新開啟資料庫實際保存的原文。",
    synthetic: "僅使用合成測試資料 · 尚非正式版 · 不含 AI", draftHeading: "記下一個合成測試片刻",
    draftLabel: "這次要儲存的文字", placeholder: "例如：今天，我想記住静かな勇気。",
    draftState: "草稿 — 尚未儲存", savingState: "儲存中 — 尚未確認成功",
    committedState: "已儲存 — SQLite 已確認交易完成", failedState: "未能儲存 — 草稿仍保留在這裡",
    save: "明確儲存", retry: "使用同一請求重試", listHeading: "此裝置已儲存的片刻",
    emptyList: "尚未儲存任何片刻。", open: "開啟已儲存的原文", exactHeading: "已儲存的原文",
    close: "關閉", loading: "正在準備應用程式專用的儲存空間…",
    blocked: "目前無法安全開啟本機儲存空間。資料未被修改。", details: "技術限制與範圍",
    boundaries: [
      "暫用應用程式 ID：com.lifeos.review.m1。",
      "使用應用程式專用的精確 schema v5；不會匯入桌面資料、不會回退至 v4，也不提供修復、遷移、備份或裝置轉移。",
      "僅支援建立、列出與讀取 Experience；不支援更新、刪除、AI、可觀察線索（Evidence）、模式假設（Pattern）或每日反思流程。",
      "交易提交確認只代表目前測試到的資料庫結果，不代表已證明模擬器／主機崩潰或實體斷電下的耐久性。",
    ],
    supported: "支援：createExperience、listExperiences、getExperience。",
    unsupported: "不支援：更新／刪除／匯入、產出物／歷史內容、復原、匯出、同步。",
    debug: "一次性除錯工具", beforeCommit: "讓下次儲存在提交前暫停", afterCommit: "讓下次儲存在提交後、確認前暫停",
  },
  ja: {
    language: "言語", eyebrow: "Android M1 · 一時利用のネイティブ審査版", title: "端末に残す、小さな瞬間",
    summary: "合成テスト用の文章を書き、明示的に保存して、データベースに保存された原文を開き直します。",
    synthetic: "合成テストデータのみ · 正式版ではありません · AI なし",
    draftHeading: "合成テスト用の瞬間を記録", draftLabel: "保存する文章",
    placeholder: "例：今天，我想記住静かな勇気。", draftState: "下書き — 未保存",
    savingState: "保存中 — 成功はまだ確認されていません",
    committedState: "保存済み — SQLite がトランザクションの完了を確認しました",
    failedState: "保存できませんでした — 下書きはそのまま残っています", save: "明示的に保存",
    retry: "同じリクエストで再試行", listHeading: "この端末に保存した瞬間",
    emptyList: "保存した瞬間はまだありません。", open: "保存した原文を開く",
    exactHeading: "保存した原文", close: "閉じる", loading: "アプリ専用ストレージを準備しています…",
    blocked: "ストレージを安全に開けません。データの削除や修復は行っていません。", details: "技術上の制限",
    boundaries: [
      "一時利用のアプリ ID：com.lifeos.review.m1。",
      "アプリ専用の正確な schema v5 を使用します。デスクトップデータの取り込み、v4 へのフォールバック、修復、移行、バックアップ、端末移行には対応していません。",
      "対応する操作は Experience の作成・一覧・取得のみです。更新、削除、AI、観察できる手がかり（Evidence）、パターン仮説（Pattern）、日次の振り返りには対応していません。",
      "トランザクションのコミット確認は、エミュレーター／ホストのクラッシュや実機の電源断に対する耐久性を証明するものではありません。",
    ],
    supported: "対応：createExperience、listExperiences、getExperience。",
    unsupported: "非対応：更新／削除／取り込み、生成物／履歴、復旧、書き出し、同期。",
    debug: "一時利用のデバッグ機能", beforeCommit: "次の保存をコミット前で一時停止",
    afterCommit: "次の保存をコミット後・確認前で一時停止",
  },
};

const localeLabels: Record<AndroidM1Locale, string> = { en: "English", "zh-TW": "繁體中文", ja: "日本語" };

export function normalizeAndroidM1Locale(value: unknown): AndroidM1Locale {
  return value === "zh-TW" || value === "ja" || value === "en" ? value : "en";
}

export function createM1RequestId(): string {
  return "m1_" + (globalThis.crypto?.randomUUID?.() ?? String(Date.now()) + "_" + Math.random().toString(16).slice(2));
}

function errorCode(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export function AndroidM1App({
  store = androidM1ExperienceStore,
  localePreference = androidM1LocalePreference,
}: {
  store?: AndroidM1ExperienceStore;
  localePreference?: AndroidM1LocalePreference;
}) {
  const [locale, setLocale] = useState<AndroidM1Locale>("en");
  const [localePreferenceState, setLocalePreferenceState] = useState<LocalePreferenceState>("loading");
  const [storage, setStorage] = useState<AndroidM1StorageStatus | null>(null);
  const [storageError, setStorageError] = useState<string | null>(null);
  const [entries, setEntries] = useState<ExperienceEntry[]>([]);
  const [selected, setSelected] = useState<ExperienceEntry | null>(null);
  const [draft, setDraft] = useState("");
  const [requestId, setRequestId] = useState(createM1RequestId);
  const [saveState, setSaveState] = useState<SaveState>("draft");
  const [saveError, setSaveError] = useState<string | null>(null);
  const [debugPhase, setDebugPhase] = useState<AndroidM1DebugPhase | undefined>();
  const savingRef = useRef(false);
  const copy = useMemo(() => androidM1Copy[locale], [locale]);

  useEffect(() => {
    let current = true;
    void localePreference.read()
      .then((stored) => {
        if (current) {
          setLocale(normalizeAndroidM1Locale(stored));
          setLocalePreferenceState("saved");
        }
      })
      .catch(() => {
        if (current) setLocalePreferenceState("failed");
      });
    return () => { current = false; };
  }, [localePreference]);

  useEffect(() => {
    document.documentElement.lang = locale === "zh-TW" ? "zh-Hant" : locale;
  }, [locale]);

  const selectLocale = (candidate: AndroidM1Locale) => {
    setLocale(candidate);
    setLocalePreferenceState("saving");
    void localePreference.write(candidate)
      .then(() => setLocalePreferenceState("saved"))
      .catch(() => setLocalePreferenceState("failed"));
  };

  const refresh = async () => setEntries(await store.listExperiences());

  useEffect(() => {
    let current = true;
    void (async () => {
      try {
        const status = await store.status();
        const loaded = await store.listExperiences();
        if (current) { setStorage(status); setEntries(loaded); }
      } catch (error) {
        if (current) setStorageError(errorCode(error));
      }
    })();
    return () => { current = false; };
  }, [store]);

  const changeDraft = (value: string) => {
    setDraft(value);
    if (saveState === "committed") setRequestId(createM1RequestId());
    setSaveState("draft");
    setSaveError(null);
  };

  const save = async () => {
    if (savingRef.current || !draft.trim() || !storage) return;
    savingRef.current = true;
    setSaveState("saving");
    setSaveError(null);
    try {
      const receipt = await store.createExperience({ body: draft }, requestId, debugPhase);
      setSelected(receipt.entry);
      await refresh();
      setSaveState("committed");
      setDebugPhase(undefined);
    } catch (error) {
      setSaveError(errorCode(error));
      setSaveState("failed");
    } finally {
      savingRef.current = false;
    }
  };

  const open = async (id: string) => setSelected(await store.getExperience(id));

  if (!storage || localePreferenceState === "loading") {
    return (
      <main className="android-m1 android-m1--center" data-runtime="android-m1-disposable" data-storage-state={storageError ? "blocked" : "loading"} data-locale-preference-state={localePreferenceState}>
        <p>{storageError ? copy.blocked : copy.loading}</p>
        {storageError ? <code>{storageError}</code> : null}
      </main>
    );
  }

  const stateText = saveState === "saving" ? copy.savingState
    : saveState === "committed" ? copy.committedState
      : saveState === "failed" ? copy.failedState : copy.draftState;

  return (
    <main className="android-m1" data-runtime="android-m1-disposable" data-storage-state="ready" data-save-state={saveState} data-locale-preference-state={localePreferenceState}>
      <div className="android-m1__shell">
        <nav className="android-m1__languages" aria-label={copy.language}>
          {(Object.keys(localeLabels) as AndroidM1Locale[]).map((candidate) => (
            <button type="button" data-locale={candidate} className={candidate === locale ? "is-active" : undefined} aria-pressed={candidate === locale} disabled={localePreferenceState === "saving"} onClick={() => selectLocale(candidate)} key={candidate}>{localeLabels[candidate]}</button>
          ))}
        </nav>
        <header>
          <p className="android-m1__eyebrow">{copy.eyebrow}</p>
          <h1>{copy.title}</h1><p>{copy.summary}</p><p className="android-m1__badge">{copy.synthetic}</p>
        </header>
        <section className="android-m1__card" aria-labelledby="m1-draft-heading">
          <h2 id="m1-draft-heading">{copy.draftHeading}</h2>
          <label htmlFor="m1-draft">{copy.draftLabel}</label>
          <textarea id="m1-draft" data-testid="m1-draft" rows={5} value={draft} placeholder={copy.placeholder} disabled={saveState === "saving"} onChange={(event) => changeDraft(event.currentTarget.value)} />
          <p className={"android-m1__state android-m1__state--" + saveState} role="status">{stateText}</p>
          {saveError ? <code className="android-m1__error">{saveError}</code> : null}
          <button className="android-m1__primary" data-testid="m1-save" type="button" disabled={!draft.trim() || saveState === "saving"} onClick={() => void save()}>{saveState === "failed" ? copy.retry : copy.save}</button>
        </section>
        <section className="android-m1__card" aria-labelledby="m1-list-heading">
          <h2 id="m1-list-heading">{copy.listHeading}</h2>
          {entries.length === 0 ? <p>{copy.emptyList}</p> : (
            <ol className="android-m1__list" data-testid="m1-list">
              {entries.map((entry) => (
                <li key={entry.id} data-experience-id={entry.id}>
                  <p>{entry.body}</p><button type="button" onClick={() => void open(entry.id)}>{copy.open}</button>
                </li>
              ))}
            </ol>
          )}
        </section>
        {selected ? (
          <section className="android-m1__card android-m1__exact" data-testid="m1-exact">
            <h2>{copy.exactHeading}</h2><p>{selected.body}</p><button type="button" onClick={() => setSelected(null)}>{copy.close}</button>
          </section>
        ) : null}
        <details className="android-m1__details">
          <summary>{copy.details}</summary>
          <ul>{copy.boundaries.map((boundary) => <li key={boundary}>{boundary}</li>)}</ul>
          <p>{copy.supported}</p><p>{copy.unsupported}</p>
          <code>schema={storage.schemaVersion} · file={storage.databaseFilename}</code>
          {import.meta.env.VITE_LIFE_OS_ANDROID_M1_DEBUG_HOOKS === "1" ? (
            <div className="android-m1__debug">
              <strong>{copy.debug}</strong>
              <button type="button" data-debug-phase="beforeCommit" aria-pressed={debugPhase === "beforeCommit"} onClick={() => setDebugPhase("beforeCommit")}>{copy.beforeCommit}</button>
              <button type="button" data-debug-phase="afterCommitBeforeAck" aria-pressed={debugPhase === "afterCommitBeforeAck"} onClick={() => setDebugPhase("afterCommitBeforeAck")}>{copy.afterCommit}</button>
            </div>
          ) : null}
        </details>
      </div>
    </main>
  );
}
