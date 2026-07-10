import type {
  CandidateStatus,
  EvidenceCandidate,
  ReflectionPrompt,
} from "../types/domain";

export type AppLanguage = "en" | "zh-TW" | "ja";

export const languageOptions: Array<{
  value: AppLanguage;
  label: string;
}> = [
  { value: "en", label: "English" },
  { value: "zh-TW", label: "繁體中文" },
  { value: "ja", label: "日本語" },
];

type EvidenceKind = EvidenceCandidate["kind"];
type ReflectionStatus = ReflectionPrompt["status"];

export const uiText = {
  en: {
    languageLabel: "Language",
    appAria: "Life OS reflection space",
    alpha: "Life OS Alpha",
    motto: "We Build Mirrors, Not Oracles.",
    welcome: "Welcome.",
    hero: "What happened today that stayed with you?",
    subtitle:
      "Write one moment. Life OS will help you turn it into evidence, reflection questions, and possible patterns - never conclusions.",
    experienceAria: "Experience",
    experiencePlaceholder:
      "A conversation, a decision, a feeling, a small moment you keep returning to...",
    saving: "Saving...",
    saveMoment: "Save this moment",
    importExportAria: "Import and export",
    exportJson: "Export JSON",
    exportMarkdown: "Export Markdown",
    importJson: "Import JSON",
    storageErrorPrefix: "Local storage error:",
    currentSession: "Current session",
    summary: (entries: number, total: number, confirmed: number, rejected: number, pending: number) =>
      `${entries} entries / ${total} evidence candidates / ${confirmed} confirmed / ${rejected} rejected / ${pending} pending`,
    summaryNote: "A lightweight mirror of review state. No scores. No judgment.",
    emptyAria: "Empty timeline",
    emptyTitle: "Your timeline is quiet.",
    emptyBody:
      "Start with one honest moment. It does not need to be important. It only needs to be real enough to look at gently.",
    savedExperiencesAria: "Saved experiences",
    moment: "Moment",
    updated: "Updated",
    editExperienceAria: "Edit experience",
    saveEdit: "Save edit",
    cancel: "Cancel",
    edit: "Edit",
    delete: "Delete",
    save: "Save",
    confirm: "Confirm",
    reject: "Reject",
    evidenceStep: "1 / Evidence",
    evidenceTitle: "What can be observed?",
    findingEvidence: "Finding observable pieces...",
    generateEvidence: "Generate evidence",
    evidenceNextPending:
      "Stay with the moment. Life OS is only separating observable pieces.",
    evidenceNextEmpty:
      "Next: generate a few candidates, then keep only what feels accurate.",
    evidenceNextReview:
      "Next: review one candidate. Confirm, edit, or set it aside.",
    evidenceNextReady:
      "Enough evidence is confirmed. Reflection is ready when you are.",
    evidenceNextNone:
      "No evidence is confirmed yet. You can generate again or leave this moment unprocessed.",
    evidenceSummary: (total: number, confirmed: number, rejected: number, pending: number) =>
      `${total} candidates / ${confirmed} confirmed / ${rejected} rejected / ${pending} pending`,
    evidenceNote:
      "You can edit candidates before confirming. They are not facts until you accept them.",
    editEvidenceAria: "Edit evidence candidate",
    editedFromAi: "Edited from AI output.",
    keptEvidence: "Kept as usable evidence for this session.",
    setAside: "Set aside for this session.",
    reviewEvidenceAria: "Review this evidence candidate",
    evidenceEmpty: "Evidence candidates will appear here for review.",
    reflectionAria: "Reflection prompt review",
    reflectionStep: "2 / Reflection",
    reflectionTitle: "What question opens the mirror?",
    shapingQuestions: "Shaping questions...",
    generateReflection: "Generate reflection",
    reflectionBoundary:
      "Reflection prompts are questions, not conclusions.",
    reflectionNextNeedEvidence: "Next: confirm one evidence candidate first.",
    reflectionNextPending:
      "Stay unhurried. Life OS is shaping questions, not answers.",
    reflectionNextEmpty:
      "Next: generate a few questions from confirmed evidence.",
    reflectionNextReview:
      "Next: answer one question only if it opens something useful. Skipping is allowed.",
    reflectionNextReady:
      "You have enough reflection context for a pattern hypothesis.",
    reflectionNextSkipped:
      "All questions were skipped. That is still a valid review outcome.",
    reflectionNeedEvidence:
      "Confirm at least one evidence candidate before generating reflection prompts.",
    reflectionSummary: (suggested: number, answered: number, skipped: number) =>
      `${suggested} suggested / ${answered} answered / ${skipped} skipped`,
    evidenceCount: (count: number) => `${count} evidence`,
    answerReflectionAria: "Answer reflection prompt",
    reflectionPlaceholder:
      "Optional. Answer only if something feels true enough to write.",
    answeredSession: "Answered for this session only.",
    skippedNoJudgment: "Skipped without judgment.",
    reviewReflectionAria: "Review this reflection question",
    saveAnswer: "Save answer",
    skip: "Skip",
    patternAria: "Pattern candidate review",
    patternStep: "3 / Pattern",
    patternTitle: "Is there a hypothesis worth reviewing?",
    formingPattern: "Forming one hypothesis...",
    generatePattern: "Generate pattern",
    patternBoundary:
      "Pattern candidates are hypotheses for review, not conclusions.",
    patternNextNeedEvidence: "Next: confirm evidence before asking for a pattern.",
    patternNextPending:
      "Stay spacious. Life OS is forming one hypothesis, not a label.",
    patternNextEmpty:
      "Next: generate one hypothesis only when you want another angle.",
    patternNextReview:
      "Next: confirm only if the hypothesis feels worth revisiting; otherwise reject it.",
    patternNextComplete: "Pattern review is complete for this session.",
    patternNextRejected: "The hypothesis was set aside. Nothing else is required.",
    patternNeedEvidence:
      "Confirm at least one evidence candidate before generating a pattern candidate.",
    patternSummary: (candidate: number, confirmed: number, rejected: number) =>
      `${candidate} candidate / ${confirmed} confirmed / ${rejected} rejected`,
    keptPattern: "Kept as a hypothesis to revisit.",
    reviewPatternAria: "Review this pattern hypothesis",
    aiAvailable: (provider: string) =>
      `${provider} is available. Outputs still require your review.`,
    aiFallbackDetail:
      "Set AI_PROVIDER and an API key before launch to enable real AI.",
    localFallback: "Local mirror fallback",
    providerActive: (provider: string, model?: string) =>
      `${provider} active${model ? ` - ${model}` : ""}`,
    providerGeneric: "the AI provider",
    exportTitle: "Export Life OS experiences",
    importTitle: "Import Life OS experiences",
    exportSaved: (path: string) => `Export saved: ${path}`,
    exportCancelled: "Export cancelled. No file was written.",
    importCancelled: "Import cancelled. No file was read.",
    importComplete: (filename: string, imported: number, skipped: number) =>
      `Import complete from ${filename}: ${imported} imported, ${skipped} skipped.`,
    evidenceEditedStatus:
      "Evidence candidate edited. It remains a candidate until you confirm it.",
    evidenceReviewUpdated:
      "Evidence review updated. Candidates remain session-only.",
    reflectionSaved: "Reflection answer saved for this session only.",
    reflectionSkipped: "Reflection prompt skipped for this session only.",
    patternUpdated:
      "Pattern review updated. Pattern candidates remain session-only.",
    pendingHolding: "Holding the moment gently.",
    pendingEvidence: (provider: string) =>
      `${provider} is looking for observable pieces. You will choose what is true.`,
    pendingReflection: (provider: string) =>
      `${provider} is shaping questions, not answers.`,
    pendingPattern: (provider: string) =>
      `${provider} is forming one hypothesis for review, not a label.`,
    pendingFallback: "Reflecting gently. Keeping this as a hypothesis.",
    successLocalEvidence:
      "Local mirror prepared evidence candidates. You decide what is true.",
    successLocalReflection:
      "Local mirror prepared reflection questions. They are invitations, not conclusions.",
    successLocalPattern:
      "Local mirror prepared one pattern hypothesis. Review it slowly before accepting it.",
    firstMirror: (provider: string) => `First ${provider} mirror returned gently.`,
    mirror: (provider: string) => `${provider} mirror returned gently.`,
    successEvidence:
      "Evidence candidates are ready for your review; none are facts until you confirm them.",
    successReflection:
      "Reflection questions are ready. Let them open space, not pressure.",
    successPattern:
      "One pattern hypothesis is ready. It is only something to examine, not a conclusion.",
    fallbackIntro: (provider: string) =>
      `${provider} could not respond right now, so Life OS used the local mirror fallback.`,
    errorMissingConfig:
      "To enable real AI, check your .env.local key and restart the app.",
    errorInvalidKey:
      "The API key was not accepted. Copy the secret key itself, not the key name or tracking ID, then restart.",
    errorModelUnavailable:
      "The selected model is not available for this account. Try a broadly available model such as gemini-3.1-flash-lite for Gemini or gpt-5-nano for OpenAI.",
    errorQuota:
      "The account may be out of free credits, quota, or billing access. Check the provider console when you are ready.",
    errorRateLimited:
      "The provider is rate-limiting requests. Waiting a little and trying again is usually enough.",
    errorNetwork:
      "This looks like a connection or provider availability issue. Your local reflection flow is still safe to continue.",
    errorUnexpected:
      "The provider returned something Life OS could not safely read. The fallback kept the flow reviewable.",
    errorUnknown:
      "Nothing was saved to the provider by Life OS; you can keep reflecting locally and try real AI again later.",
  },
  "zh-TW": {
    languageLabel: "語言",
    appAria: "Life OS 反思空間",
    alpha: "Life OS Alpha",
    motto: "我們建造鏡子，不建造神諭。",
    welcome: "歡迎。",
    hero: "今天有什麼事情，仍然停留在你心裡？",
    subtitle:
      "寫下一個片刻。Life OS 會幫你把它整理成 evidence、reflection question、possible pattern —— 但永遠不是結論。",
    experienceAria: "經驗紀錄",
    experiencePlaceholder:
      "一段對話、一個決定、一種感受，或一個你反覆想起的小片刻……",
    saving: "儲存中...",
    saveMoment: "儲存這個片刻",
    importExportAria: "匯入與匯出",
    exportJson: "匯出 JSON",
    exportMarkdown: "匯出 Markdown",
    importJson: "匯入 JSON",
    storageErrorPrefix: "本機儲存錯誤：",
    currentSession: "目前 session",
    summary: (entries: number, total: number, confirmed: number, rejected: number, pending: number) =>
      `${entries} 筆紀錄 / ${total} 個 evidence candidates / ${confirmed} 已確認 / ${rejected} 已排除 / ${pending} 待檢視`,
    summaryNote: "這只是目前 review 狀態的輕量鏡子。沒有分數，沒有評判。",
    emptyAria: "空的時間線",
    emptyTitle: "你的時間線現在很安靜。",
    emptyBody:
      "從一個真實的片刻開始。它不需要很重要，只需要足夠真實，讓你可以溫柔地看一看。",
    savedExperiencesAria: "已儲存的經驗",
    moment: "片刻",
    updated: "更新於",
    editExperienceAria: "編輯經驗",
    saveEdit: "儲存編輯",
    cancel: "取消",
    edit: "編輯",
    delete: "刪除",
    save: "儲存",
    confirm: "確認",
    reject: "排除",
    evidenceStep: "1 / Evidence",
    evidenceTitle: "有什麼是可以被觀察的？",
    findingEvidence: "正在尋找可觀察的片段...",
    generateEvidence: "產生 evidence",
    evidenceNextPending:
      "先停留在這個片刻。Life OS 只是在分離可觀察的部分。",
    evidenceNextEmpty:
      "下一步：產生幾個 candidates，然後只留下感覺準確的部分。",
    evidenceNextReview:
      "下一步：檢視一個 candidate。確認、編輯，或先放下。",
    evidenceNextReady:
      "已經有足夠 evidence。當你準備好時，可以進入 reflection。",
    evidenceNextNone:
      "目前還沒有確認的 evidence。你可以再產生一次，也可以讓這個片刻先停在這裡。",
    evidenceSummary: (total: number, confirmed: number, rejected: number, pending: number) =>
      `${total} candidates / ${confirmed} 已確認 / ${rejected} 已排除 / ${pending} 待檢視`,
    evidenceNote:
      "你可以在確認前編輯 candidates。只有你接受之後，它們才會成為本次 session 的 usable evidence。",
    editEvidenceAria: "編輯 evidence candidate",
    editedFromAi: "已從 AI 輸出編輯。",
    keptEvidence: "已保留為本次 session 可使用的 evidence。",
    setAside: "已在本次 session 中放下。",
    reviewEvidenceAria: "檢視這個 evidence candidate",
    evidenceEmpty: "Evidence candidates 會出現在這裡等待你檢視。",
    reflectionAria: "Reflection prompt review",
    reflectionStep: "2 / Reflection",
    reflectionTitle: "哪一個問題能打開鏡子？",
    shapingQuestions: "正在形成問題...",
    generateReflection: "產生 reflection",
    reflectionBoundary:
      "Reflection prompts 是問題，不是結論。",
    reflectionNextNeedEvidence: "下一步：先確認一個 evidence candidate。",
    reflectionNextPending:
      "不用急。Life OS 正在形成問題，不是在給答案。",
    reflectionNextEmpty:
      "下一步：從已確認的 evidence 產生幾個問題。",
    reflectionNextReview:
      "下一步：只有在它打開了某些東西時，回答一個問題即可。跳過也可以。",
    reflectionNextReady:
      "你已經有足夠的 reflection context 可以形成 pattern hypothesis。",
    reflectionNextSkipped:
      "所有問題都被跳過了。這仍然是有效的 review 結果。",
    reflectionNeedEvidence:
      "請先確認至少一個 evidence candidate，再產生 reflection prompts。",
    reflectionSummary: (suggested: number, answered: number, skipped: number) =>
      `${suggested} 建議 / ${answered} 已回答 / ${skipped} 已跳過`,
    evidenceCount: (count: number) => `${count} 個 evidence`,
    answerReflectionAria: "回答 reflection prompt",
    reflectionPlaceholder:
      "可選。只有當某些東西感覺足夠真實時才寫下來。",
    answeredSession: "已回答，僅保留於本次 session。",
    skippedNoJudgment: "已跳過，沒有評判。",
    reviewReflectionAria: "檢視這個 reflection question",
    saveAnswer: "儲存回答",
    skip: "跳過",
    patternAria: "Pattern candidate review",
    patternStep: "3 / Pattern",
    patternTitle: "是否有值得檢視的假設？",
    formingPattern: "正在形成一個 hypothesis...",
    generatePattern: "產生 pattern",
    patternBoundary:
      "Pattern candidates 是供你檢視的假設，不是結論。",
    patternNextNeedEvidence: "下一步：先確認 evidence，再請 Life OS 形成 pattern。",
    patternNextPending:
      "保持一點空間。Life OS 正在形成一個假設，不是在貼標籤。",
    patternNextEmpty:
      "下一步：只有當你想要另一個角度時，才產生一個 hypothesis。",
    patternNextReview:
      "下一步：只有當這個 hypothesis 值得未來再看時才確認；否則排除。",
    patternNextComplete: "本次 session 的 pattern review 已完成。",
    patternNextRejected: "這個 hypothesis 已先放下。現在不需要做其他事。",
    patternNeedEvidence:
      "請先確認至少一個 evidence candidate，再產生 pattern candidate。",
    patternSummary: (candidate: number, confirmed: number, rejected: number) =>
      `${candidate} candidate / ${confirmed} 已確認 / ${rejected} 已排除`,
    keptPattern: "已保留為未來可回看的 hypothesis。",
    reviewPatternAria: "檢視這個 pattern hypothesis",
    aiAvailable: (provider: string) =>
      `${provider} 已連線。輸出仍然需要由你檢視。`,
    aiFallbackDetail:
      "啟動前設定 AI_PROVIDER 與 API key，即可啟用真實 AI。",
    localFallback: "本機鏡像 fallback",
    providerActive: (provider: string, model?: string) =>
      `${provider} 已啟用${model ? ` - ${model}` : ""}`,
    providerGeneric: "AI provider",
    exportTitle: "匯出 Life OS experiences",
    importTitle: "匯入 Life OS experiences",
    exportSaved: (path: string) => `已匯出：${path}`,
    exportCancelled: "已取消匯出，沒有寫入檔案。",
    importCancelled: "已取消匯入，沒有讀取檔案。",
    importComplete: (filename: string, imported: number, skipped: number) =>
      `已從 ${filename} 匯入完成：${imported} 筆匯入，${skipped} 筆略過。`,
    evidenceEditedStatus:
      "Evidence candidate 已編輯。在你確認前，它仍然只是 candidate。",
    evidenceReviewUpdated:
      "Evidence review 已更新。Candidates 仍然只保留於本次 session。",
    reflectionSaved: "Reflection answer 已儲存，僅保留於本次 session。",
    reflectionSkipped: "Reflection prompt 已跳過，僅保留於本次 session。",
    patternUpdated:
      "Pattern review 已更新。Pattern candidates 仍然只保留於本次 session。",
    pendingHolding: "正在溫柔地承接這個片刻。",
    pendingEvidence: (provider: string) =>
      `${provider} 正在尋找可觀察的片段。最後由你決定什麼是真實的。`,
    pendingReflection: (provider: string) =>
      `${provider} 正在形成問題，不是在給答案。`,
    pendingPattern: (provider: string) =>
      `${provider} 正在形成一個供你檢視的 hypothesis，不是在貼標籤。`,
    pendingFallback: "正在溫柔地反映。這仍然只是一個 hypothesis。",
    successLocalEvidence:
      "本機鏡像已準備好 evidence candidates。由你決定什麼是真實的。",
    successLocalReflection:
      "本機鏡像已準備好 reflection questions。它們是邀請，不是結論。",
    successLocalPattern:
      "本機鏡像已準備好一個 pattern hypothesis。請慢慢檢視，再決定是否接受。",
    firstMirror: (provider: string) => `第一次 ${provider} 鏡像已溫柔返回。`,
    mirror: (provider: string) => `${provider} 鏡像已溫柔返回。`,
    successEvidence:
      "Evidence candidates 已準備好等待你檢視；在你確認前，它們都不是事實。",
    successReflection:
      "Reflection questions 已準備好。讓它們打開空間，而不是形成壓力。",
    successPattern:
      "一個 pattern hypothesis 已準備好。它只是可檢視的東西，不是結論。",
    fallbackIntro: (provider: string) =>
      `${provider} 現在無法回應，所以 Life OS 使用了本機鏡像 fallback。`,
    errorMissingConfig:
      "若要啟用真實 AI，請檢查 .env.local key 並重新啟動 app。",
    errorInvalidKey:
      "API key 未被接受。請複製 secret key 本身，不是 key name 或 tracking ID，然後重新啟動。",
    errorModelUnavailable:
      "目前帳號無法使用選定模型。Gemini 可嘗試 gemini-3.1-flash-lite；OpenAI 可嘗試 gpt-5-nano。",
    errorQuota:
      "帳號可能沒有可用免費額度、quota 或 billing 權限。準備好時可以到 provider console 檢查。",
    errorRateLimited:
      "Provider 正在限制請求頻率。通常稍等一下再試即可。",
    errorNetwork:
      "這看起來像連線或 provider 可用性問題。你的本機 reflection flow 仍然可以安全繼續。",
    errorUnexpected:
      "Provider 回傳了 Life OS 無法安全讀取的內容。Fallback 讓流程仍可檢視。",
    errorUnknown:
      "Life OS 沒有把任何內容儲存到 provider；你可以先在本機繼續 reflection，稍後再試真實 AI。",
  },
  ja: {
    languageLabel: "言語",
    appAria: "Life OS リフレクション空間",
    alpha: "Life OS Alpha",
    motto: "私たちは神託ではなく、鏡をつくる。",
    welcome: "ようこそ。",
    hero: "今日、心に残っていることは何ですか？",
    subtitle:
      "ひとつの瞬間を書いてください。Life OS はそれを evidence、reflection question、possible pattern に整理します。ただし結論にはしません。",
    experienceAria: "経験",
    experiencePlaceholder:
      "会話、決断、感情、何度も思い出す小さな瞬間...",
    saving: "保存中...",
    saveMoment: "この瞬間を保存",
    importExportAria: "インポートとエクスポート",
    exportJson: "JSON をエクスポート",
    exportMarkdown: "Markdown をエクスポート",
    importJson: "JSON をインポート",
    storageErrorPrefix: "ローカル保存エラー:",
    currentSession: "現在の session",
    summary: (entries: number, total: number, confirmed: number, rejected: number, pending: number) =>
      `${entries} 件 / ${total} evidence candidates / ${confirmed} 確認済み / ${rejected} 除外 / ${pending} 保留`,
    summaryNote: "これは review 状態の軽い鏡です。スコアも評価もありません。",
    emptyAria: "空のタイムライン",
    emptyTitle: "タイムラインは静かです。",
    emptyBody:
      "ひとつの正直な瞬間から始めましょう。重要である必要はありません。やさしく見つめられる程度に、本物であれば十分です。",
    savedExperiencesAria: "保存された経験",
    moment: "瞬間",
    updated: "更新",
    editExperienceAria: "経験を編集",
    saveEdit: "編集を保存",
    cancel: "キャンセル",
    edit: "編集",
    delete: "削除",
    save: "保存",
    confirm: "確認",
    reject: "除外",
    evidenceStep: "1 / Evidence",
    evidenceTitle: "何が観察できますか？",
    findingEvidence: "観察できる部分を探しています...",
    generateEvidence: "Evidence を生成",
    evidenceNextPending:
      "その瞬間に留まります。Life OS は観察できる部分を分けているだけです。",
    evidenceNextEmpty:
      "次: candidates をいくつか生成し、正確に感じるものだけ残します。",
    evidenceNextReview:
      "次: candidate をひとつ確認、編集、または脇に置きます。",
    evidenceNextReady:
      "十分な evidence が確認されました。準備ができたら reflection に進めます。",
    evidenceNextNone:
      "まだ確認済みの evidence はありません。再生成しても、この瞬間を未処理のままにしても大丈夫です。",
    evidenceSummary: (total: number, confirmed: number, rejected: number, pending: number) =>
      `${total} candidates / ${confirmed} 確認済み / ${rejected} 除外 / ${pending} 保留`,
    evidenceNote:
      "確認する前に candidates を編集できます。あなたが受け入れるまで、それらは事実ではありません。",
    editEvidenceAria: "Evidence candidate を編集",
    editedFromAi: "AI 出力から編集済み。",
    keptEvidence: "この session で使える evidence として保持しました。",
    setAside: "この session では脇に置きました。",
    reviewEvidenceAria: "この evidence candidate を確認",
    evidenceEmpty: "Evidence candidates はここに表示されます。",
    reflectionAria: "Reflection prompt review",
    reflectionStep: "2 / Reflection",
    reflectionTitle: "どの問いが鏡を開きますか？",
    shapingQuestions: "問いを形にしています...",
    generateReflection: "Reflection を生成",
    reflectionBoundary:
      "Reflection prompts は問いであり、結論ではありません。",
    reflectionNextNeedEvidence: "次: まず evidence candidate をひとつ確認します。",
    reflectionNextPending:
      "急がなくて大丈夫です。Life OS は答えではなく、問いを形にしています。",
    reflectionNextEmpty:
      "次: 確認済み evidence からいくつかの問いを生成します。",
    reflectionNextReview:
      "次: 何かが開くと感じる問いだけ答えてください。スキップしても大丈夫です。",
    reflectionNextReady:
      "Pattern hypothesis に進むための reflection context は十分です。",
    reflectionNextSkipped:
      "すべての問いをスキップしました。それも有効な review 結果です。",
    reflectionNeedEvidence:
      "Reflection prompts を生成する前に、少なくともひとつの evidence candidate を確認してください。",
    reflectionSummary: (suggested: number, answered: number, skipped: number) =>
      `${suggested} 提案 / ${answered} 回答済み / ${skipped} スキップ`,
    evidenceCount: (count: number) => `${count} evidence`,
    answerReflectionAria: "Reflection prompt に回答",
    reflectionPlaceholder:
      "任意です。書くに足る真実味を感じるときだけ答えてください。",
    answeredSession: "この session のみ回答を保持しました。",
    skippedNoJudgment: "評価せずにスキップしました。",
    reviewReflectionAria: "この reflection question を確認",
    saveAnswer: "回答を保存",
    skip: "スキップ",
    patternAria: "Pattern candidate review",
    patternStep: "3 / Pattern",
    patternTitle: "確認する価値のある仮説はありますか？",
    formingPattern: "ひとつの hypothesis を形にしています...",
    generatePattern: "Pattern を生成",
    patternBoundary:
      "Pattern candidates は確認のための仮説であり、結論ではありません。",
    patternNextNeedEvidence: "次: pattern を生成する前に evidence を確認します。",
    patternNextPending:
      "余白を保ちましょう。Life OS はラベルではなく、仮説を形にしています。",
    patternNextEmpty:
      "次: 別の角度が欲しいときだけ、ひとつの hypothesis を生成します。",
    patternNextReview:
      "次: 後で見返す価値があると感じる場合だけ確認し、そうでなければ除外します。",
    patternNextComplete: "この session の pattern review は完了です。",
    patternNextRejected: "その hypothesis は脇に置かれました。他に必要なことはありません。",
    patternNeedEvidence:
      "Pattern candidate を生成する前に、少なくともひとつの evidence candidate を確認してください。",
    patternSummary: (candidate: number, confirmed: number, rejected: number) =>
      `${candidate} candidate / ${confirmed} 確認済み / ${rejected} 除外`,
    keptPattern: "後で見返せる hypothesis として保持しました。",
    reviewPatternAria: "この pattern hypothesis を確認",
    aiAvailable: (provider: string) =>
      `${provider} が利用できます。出力は引き続きあなたの確認が必要です。`,
    aiFallbackDetail:
      "起動前に AI_PROVIDER と API key を設定すると、実 AI を有効化できます。",
    localFallback: "ローカルミラー fallback",
    providerActive: (provider: string, model?: string) =>
      `${provider} 有効${model ? ` - ${model}` : ""}`,
    providerGeneric: "AI provider",
    exportTitle: "Life OS experiences をエクスポート",
    importTitle: "Life OS experiences をインポート",
    exportSaved: (path: string) => `エクスポートしました: ${path}`,
    exportCancelled: "エクスポートをキャンセルしました。ファイルは書き込まれていません。",
    importCancelled: "インポートをキャンセルしました。ファイルは読み込まれていません。",
    importComplete: (filename: string, imported: number, skipped: number) =>
      `${filename} からインポート完了: ${imported} 件インポート、${skipped} 件スキップ。`,
    evidenceEditedStatus:
      "Evidence candidate を編集しました。確認するまでは candidate のままです。",
    evidenceReviewUpdated:
      "Evidence review を更新しました。Candidates はこの session のみ保持されます。",
    reflectionSaved: "Reflection answer を保存しました。この session のみ保持されます。",
    reflectionSkipped: "Reflection prompt をスキップしました。この session のみ保持されます。",
    patternUpdated:
      "Pattern review を更新しました。Pattern candidates はこの session のみ保持されます。",
    pendingHolding: "この瞬間をやさしく受け止めています。",
    pendingEvidence: (provider: string) =>
      `${provider} が観察できる部分を探しています。何が真実かはあなたが選びます。`,
    pendingReflection: (provider: string) =>
      `${provider} が答えではなく、問いを形にしています。`,
    pendingPattern: (provider: string) =>
      `${provider} が確認のための hypothesis をひとつ形にしています。ラベルではありません。`,
    pendingFallback: "やさしく映しています。これはまだ hypothesis です。",
    successLocalEvidence:
      "ローカルミラーが evidence candidates を準備しました。何が真実かはあなたが決めます。",
    successLocalReflection:
      "ローカルミラーが reflection questions を準備しました。それらは結論ではなく招待です。",
    successLocalPattern:
      "ローカルミラーが pattern hypothesis をひとつ準備しました。受け入れる前にゆっくり確認してください。",
    firstMirror: (provider: string) => `初めての ${provider} ミラーがやさしく返ってきました。`,
    mirror: (provider: string) => `${provider} ミラーがやさしく返ってきました。`,
    successEvidence:
      "Evidence candidates の準備ができました。確認するまで事実ではありません。",
    successReflection:
      "Reflection questions の準備ができました。圧力ではなく、余白を開くものとして扱ってください。",
    successPattern:
      "Pattern hypothesis がひとつ準備できました。これは結論ではなく、確認するためのものです。",
    fallbackIntro: (provider: string) =>
      `${provider} は今応答できなかったため、Life OS はローカルミラー fallback を使いました。`,
    errorMissingConfig:
      "実 AI を有効にするには、.env.local key を確認して app を再起動してください。",
    errorInvalidKey:
      "API key が受け付けられませんでした。key name や tracking ID ではなく secret key 本体をコピーし、再起動してください。",
    errorModelUnavailable:
      "選択したモデルはこのアカウントでは利用できません。Gemini は gemini-3.1-flash-lite、OpenAI は gpt-5-nano を試してください。",
    errorQuota:
      "無料枠、quota、または billing 権限が不足している可能性があります。準備ができたら provider console を確認してください。",
    errorRateLimited:
      "Provider がリクエストを制限しています。少し待ってから再試行すると改善することがあります。",
    errorNetwork:
      "接続または provider の可用性の問題に見えます。ローカルの reflection flow は安全に続けられます。",
    errorUnexpected:
      "Provider が Life OS で安全に読めない内容を返しました。Fallback により review 可能な状態を保ちました。",
    errorUnknown:
      "Life OS は provider に何も保存していません。ローカルで reflection を続け、後でもう一度試せます。",
  },
};

export type UiCopy = (typeof uiText)["en"];

export function statusLabel(
  status: CandidateStatus | ReflectionStatus,
  language: AppLanguage,
): string {
  const labels: Record<AppLanguage, Record<string, string>> = {
    en: {
      candidate: "candidate",
      confirmed: "confirmed",
      rejected: "rejected",
      suggested: "suggested",
      answered: "answered",
      skipped: "skipped",
    },
    "zh-TW": {
      candidate: "待檢視",
      confirmed: "已確認",
      rejected: "已排除",
      suggested: "建議",
      answered: "已回答",
      skipped: "已跳過",
    },
    ja: {
      candidate: "候補",
      confirmed: "確認済み",
      rejected: "除外",
      suggested: "提案",
      answered: "回答済み",
      skipped: "スキップ",
    },
  };

  return labels[language][status] ?? status;
}

export function evidenceKindLabel(
  kind: EvidenceKind,
  language: AppLanguage,
): string {
  const labels: Record<AppLanguage, Record<EvidenceKind, string>> = {
    en: {
      observation: "observation",
      emotion: "emotion",
      decision: "decision",
      contradiction: "contradiction",
      self_description: "self description",
      other: "other",
    },
    "zh-TW": {
      observation: "觀察",
      emotion: "情緒",
      decision: "決定",
      contradiction: "矛盾",
      self_description: "自我描述",
      other: "其他",
    },
    ja: {
      observation: "観察",
      emotion: "感情",
      decision: "決定",
      contradiction: "矛盾",
      self_description: "自己記述",
      other: "その他",
    },
  };

  return labels[language][kind] ?? kind;
}
