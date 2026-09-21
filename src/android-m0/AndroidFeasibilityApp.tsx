import { useMemo, useState } from "react";

export type AndroidM0Locale = "en" | "zh-TW" | "ja";

type AndroidM0Copy = {
  language: string;
  eyebrow: string;
  title: string;
  summary: string;
  boundariesHeading: string;
  boundaries: readonly string[];
  identity: string;
  previewHeading: string;
  previewLabel: string;
  previewPlaceholder: string;
  mirrorHeading: string;
  emptyMirror: string;
  clear: string;
  footer: string;
};

export const androidM0Copy: Record<AndroidM0Locale, AndroidM0Copy> = {
  en: {
    language: "Language",
    eyebrow: "Native review candidate · M0",
    title: "Life OS Android feasibility shell",
    summary: "A bounded native packaging test. We build mirrors, not oracles.",
    boundariesHeading: "What this build is — and is not",
    boundaries: [
      "Feasibility build only; it is not Android R0 or a release.",
      "No real AI, inference, diagnosis, or advice. The preview only mirrors the text below.",
      "No product database or persistent Life OS data is created.",
      "No desktop profile, history, provider, credential, or cloud connection exists.",
      "Preview text is session-only and may disappear after close or restart.",
    ],
    identity: "Temporary application ID: com.lifeos.feasibility.m0. Debug signing and future install/data continuity are not promised.",
    previewHeading: "Synthetic, session-only preview",
    previewLabel: "Type sample text",
    previewPlaceholder: "For example: I want to notice what matters today.",
    mirrorHeading: "Immediate mirror",
    emptyMirror: "Your sample text will appear here without analysis.",
    clear: "Clear preview",
    footer: "Offline runtime boundary: no provider call, persistence, backup, or device transfer.",
  },
  "zh-TW": {
    language: "語言",
    eyebrow: "原生審查候選版 · M0",
    title: "Life OS Android 可行性外殼",
    summary: "這是邊界明確的原生封裝測試。我們打造鏡子，而非神諭。",
    boundariesHeading: "這個版本是什麼，也不是什麼",
    boundaries: [
      "僅供可行性驗證；不是 Android R0，也不是正式發布。",
      "沒有真正的 AI、推論、診斷或建議；預覽只會原樣映照下方文字。",
      "不建立產品資料庫，也不保存任何 Life OS 資料。",
      "不連接桌面個人檔案、歷史、供應商、憑證或雲端。",
      "預覽文字只存在於本次工作階段，關閉或重新啟動後可能消失。",
    ],
    identity: "暫時應用程式 ID：com.lifeos.feasibility.m0。除錯簽章不承諾未來安裝或資料延續。",
    previewHeading: "合成、僅限本次工作階段的預覽",
    previewLabel: "輸入範例文字",
    previewPlaceholder: "例如：我想留意今天真正重要的事。",
    mirrorHeading: "即時鏡映",
    emptyMirror: "你的範例文字會顯示在這裡，不會受到分析。",
    clear: "清除預覽",
    footer: "離線執行邊界：不呼叫供應商、不持久化、不備份，也不進行裝置轉移。",
  },
  ja: {
    language: "言語",
    eyebrow: "ネイティブレビュー候補 · M0",
    title: "Life OS Android 実現可能性シェル",
    summary: "境界を限定したネイティブ・パッケージ検証です。神託ではなく、鏡をつくります。",
    boundariesHeading: "このビルドで行うこと・行わないこと",
    boundaries: [
      "実現可能性の確認専用です。Android R0 やリリース版ではありません。",
      "実際の AI、推論、診断、助言はありません。入力文をそのまま映すだけです。",
      "製品データベースや永続的な Life OS データは作成しません。",
      "デスクトップのプロフィール、履歴、プロバイダー、認証情報、クラウドには接続しません。",
      "プレビュー文はこのセッションだけに存在し、終了・再起動後に失われます。",
    ],
    identity: "一時的なアプリ ID：com.lifeos.feasibility.m0。デバッグ署名は将来のインストールやデータ継続を保証しません。",
    previewHeading: "合成・セッション限定プレビュー",
    previewLabel: "サンプル文を入力",
    previewPlaceholder: "例：今日、本当に大切なことに気づきたい。",
    mirrorHeading: "そのまま映す鏡",
    emptyMirror: "分析せず、入力したサンプル文をここに表示します。",
    clear: "プレビューを消去",
    footer: "オフライン実行境界：プロバイダー通信、永続化、バックアップ、端末間転送はありません。",
  },
};

const localeLabels: Record<AndroidM0Locale, string> = {
  en: "English",
  "zh-TW": "繁體中文",
  ja: "日本語",
};

export function AndroidFeasibilityApp() {
  const [locale, setLocale] = useState<AndroidM0Locale>("en");
  const [sample, setSample] = useState("");
  const copy = useMemo(() => androidM0Copy[locale], [locale]);

  return (
    <main className="android-m0" data-runtime="android-feasibility-m0">
      <section className="android-m0__shell" aria-labelledby="android-m0-title">
        <nav className="android-m0__languages" aria-label={copy.language}>
          {(Object.keys(localeLabels) as AndroidM0Locale[]).map((candidate) => (
            <button
              className={candidate === locale ? "is-active" : undefined}
              type="button"
              aria-pressed={candidate === locale}
              onClick={() => setLocale(candidate)}
              key={candidate}
            >
              {localeLabels[candidate]}
            </button>
          ))}
        </nav>

        <header className="android-m0__header">
          <p className="android-m0__eyebrow">{copy.eyebrow}</p>
          <h1 id="android-m0-title">{copy.title}</h1>
          <p>{copy.summary}</p>
        </header>

        <section className="android-m0__card" aria-labelledby="android-m0-boundaries">
          <h2 id="android-m0-boundaries">{copy.boundariesHeading}</h2>
          <ul>
            {copy.boundaries.map((boundary) => <li key={boundary}>{boundary}</li>)}
          </ul>
          <p className="android-m0__identity">{copy.identity}</p>
        </section>

        <section className="android-m0__card" aria-labelledby="android-m0-preview">
          <h2 id="android-m0-preview">{copy.previewHeading}</h2>
          <label htmlFor="android-m0-sample">{copy.previewLabel}</label>
          <textarea
            id="android-m0-sample"
            value={sample}
            placeholder={copy.previewPlaceholder}
            onChange={(event) => setSample(event.currentTarget.value)}
            rows={4}
          />
          <div className="android-m0__mirror" aria-live="polite">
            <h3>{copy.mirrorHeading}</h3>
            <p>{sample || copy.emptyMirror}</p>
          </div>
          <button className="android-m0__clear" type="button" onClick={() => setSample("")} disabled={!sample}>
            {copy.clear}
          </button>
        </section>

        <footer>{copy.footer}</footer>
      </section>
    </main>
  );
}
