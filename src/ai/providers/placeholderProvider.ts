import type { AIProvider } from "./types";
import type {
  EvidenceCandidate,
  ExperienceEntry,
  PatternNote,
  ReflectionPrompt,
} from "../../types/domain";

function createId(): string {
  return globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
}

function now(): string {
  return new Date().toISOString();
}

function firstFragment(body: string): string {
  const [fragment] = body
    .split(/(?<=[.!?。！？])\s+|\n+/u)
    .map((part) => part.trim())
    .filter(Boolean);

  return fragment ?? body.trim();
}

function isLikelyChinese(text: string): boolean {
  return /[\u3400-\u9fff]/u.test(text);
}

function shortText(text: string, maxLength = 160): string {
  const normalized = text.replace(/\s+/g, " ").trim();

  if (normalized.length <= maxLength) {
    return normalized;
  }

  return `${normalized.slice(0, maxLength - 1)}...`;
}

function containsAny(body: string, words: string[]): boolean {
  const normalized = body.toLowerCase();

  return words.some((word) => normalized.includes(word.toLowerCase()));
}

function createCandidate(
  entry: ExperienceEntry,
  text: string,
  kind: EvidenceCandidate["kind"],
): EvidenceCandidate {
  const timestamp = now();

  return {
    id: createId(),
    sourceEntryId: entry.id,
    text,
    originalText: text,
    kind,
    status: "candidate",
    userEditable: true,
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

function createReflectionPrompt(
  entry: ExperienceEntry,
  evidence: EvidenceCandidate,
  question: string,
): ReflectionPrompt {
  const timestamp = now();

  return {
    id: createId(),
    sourceEntryId: entry.id,
    sourceEvidenceIds: [evidence.id],
    question,
    status: "suggested",
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

function createPatternNote(
  entry: ExperienceEntry,
  confirmedEvidence: EvidenceCandidate[],
  answeredReflectionPrompts: ReflectionPrompt[],
  text: string,
): PatternNote {
  const timestamp = now();
  const sourceReflectionPromptIds = answeredReflectionPrompts.map(
    (prompt) => prompt.id,
  );

  return {
    id: createId(),
    sourceEntryId: entry.id,
    sourceEvidenceIds: confirmedEvidence.map((evidence) => evidence.id),
    sourceReflectionPromptIds:
      sourceReflectionPromptIds.length > 0 ? sourceReflectionPromptIds : undefined,
    text,
    status: "candidate",
    createdAt: timestamp,
    updatedAt: timestamp,
  };
}

export function extractEvidenceCandidatesFromExperience(
  entry: ExperienceEntry,
): EvidenceCandidate[] {
  const body = entry.body.trim();
  const useChinese = isLikelyChinese(body);

  if (!body) {
    return [];
  }

  const candidates = [
    createCandidate(
      entry,
      useChinese
        ? `來自紀錄的觀察：${firstFragment(body)}`
        : `Observation from entry: ${firstFragment(body)}`,
      "observation",
    ),
  ];

  if (
    containsAny(body, [
      "feel",
      "felt",
      "angry",
      "sad",
      "happy",
      "afraid",
      "anxious",
      "worry",
      "pressure",
      "stress",
      "感覺",
      "覺得",
      "難過",
      "生氣",
      "焦慮",
      "壓力",
      "失望",
      "開心",
    ])
  ) {
    candidates.push(
      createCandidate(
        entry,
        useChinese
          ? "這段紀錄中可能提到了一個情緒。請先檢視，再決定是否確認。"
          : "Possible emotion mentioned in this entry. Review before confirming.",
        "emotion",
      ),
    );
  }

  if (
    containsAny(body, [
      "decide",
      "decided",
      "choose",
      "chose",
      "choice",
      "決定",
      "選擇",
      "打算",
    ])
  ) {
    candidates.push(
      createCandidate(
        entry,
        useChinese
          ? "這段紀錄中可能提到了一個決定或選擇。請先檢視，再決定是否確認。"
          : "Possible decision or choice mentioned in this entry. Review before confirming.",
        "decision",
      ),
    );
  }

  if (candidates.length === 1) {
    candidates.push(
      createCandidate(
        entry,
        useChinese
          ? "可能的 evidence candidate。只有在它有助於你檢視這段經驗時才保留。"
          : "Possible evidence candidate. Keep only if this helps you review the experience.",
        "other",
      ),
    );
  }

  return candidates;
}

const reflectionQuestionTemplates = [
  "What stands out to you when you read this evidence again?",
  "What feels most important here, and what still feels unclear?",
  "Could there be another way to understand this moment?",
  "What part of this experience feels worth holding gently for a little longer?",
];

const chineseReflectionQuestionTemplates = [
  "當你再次閱讀這個 evidence 時，什麼最讓你注意到？",
  "這裡什麼感覺最重要？又有什麼仍然不清楚？",
  "是否還有另一種方式可以理解這個片刻？",
  "這段經驗中，哪一部分值得你再溫柔地停留一下？",
];

export function generateReflectionPromptsFromConfirmedEvidence(
  entry: ExperienceEntry,
  confirmedEvidence: EvidenceCandidate[],
): ReflectionPrompt[] {
  const templates = isLikelyChinese(entry.body)
    ? chineseReflectionQuestionTemplates
    : reflectionQuestionTemplates;

  return confirmedEvidence
    .filter((evidence) => evidence.status === "confirmed")
    .slice(0, 3)
    .map((evidence, index) =>
      createReflectionPrompt(
        entry,
        evidence,
        templates[index % templates.length],
      ),
    );
}

export function suggestPatternNotesFromReview(
  entry: ExperienceEntry,
  confirmedEvidence: EvidenceCandidate[],
  reflectionPrompts: ReflectionPrompt[],
): PatternNote[] {
  const reviewedEvidence = confirmedEvidence.filter(
    (evidence) => evidence.status === "confirmed",
  );

  if (reviewedEvidence.length === 0) {
    return [];
  }

  const answeredReflectionPrompts = reflectionPrompts.filter(
    (prompt) => prompt.status === "answered" && prompt.response?.trim(),
  );
  const useChinese = isLikelyChinese(entry.body);
  const evidenceContext = shortText(reviewedEvidence[0].text);
  const reflectionContext = answeredReflectionPrompts[0]?.response
    ? useChinese
      ? ` 這也許可以連回你的 reflection response：「${shortText(
          answeredReflectionPrompts[0].response,
          120,
        )}」。`
      : ` This may connect with your own reflection response: "${shortText(
          answeredReflectionPrompts[0].response,
          120,
        )}".`
    : useChinese
      ? " 在尚未回答 reflection 的情況下，這應該被視為更加暫時的假設。"
      : " Without an answered reflection yet, this should be treated as especially tentative.";
  const patternText = useChinese
    ? `一個可以檢視的可能模式是：這段經驗是否包含圍繞「${evidenceContext}」的重複主題。${reflectionContext} 你可以先把它視為待檢查的假設，而不是結論。`
    : `One possible pattern to review is whether this experience contains a repeated theme around: "${evidenceContext}".${reflectionContext} You may want to examine whether this belongs to a broader pattern or only to this single moment.`;

  return [
    createPatternNote(
      entry,
      reviewedEvidence,
      answeredReflectionPrompts,
      patternText,
    ),
  ];
}

export const placeholderProvider: AIProvider = {
  async extractEvidence(entry) {
    return extractEvidenceCandidatesFromExperience(entry);
  },
  async generateReflectionPrompts(entry, confirmedEvidence) {
    return generateReflectionPromptsFromConfirmedEvidence(
      entry,
      confirmedEvidence,
    );
  },
  async suggestPatternNotes(entry, confirmedEvidence, reflectionPrompts) {
    return suggestPatternNotesFromReview(
      entry,
      confirmedEvidence,
      reflectionPrompts,
    );
  },
};
