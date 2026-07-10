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

function createCandidate(
  entry: ExperienceEntry,
  text: string,
  kind: EvidenceCandidate["kind"],
): EvidenceCandidate {
  const timestamp = new Date().toISOString();

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
  const timestamp = new Date().toISOString();

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
  const timestamp = new Date().toISOString();
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

function firstFragment(body: string): string {
  const [fragment] = body
    .split(/(?<=[.!?。！？])\s+|\n+/u)
    .map((part) => part.trim())
    .filter(Boolean);

  return fragment ?? body.trim();
}

function containsAny(body: string, words: string[]): boolean {
  const normalized = body.toLowerCase();

  return words.some((word) => normalized.includes(word.toLowerCase()));
}

function shortText(text: string, maxLength = 160): string {
  const normalized = text.replace(/\s+/g, " ").trim();

  if (normalized.length <= maxLength) {
    return normalized;
  }

  return `${normalized.slice(0, maxLength - 1)}…`;
}

export function extractEvidenceCandidatesFromExperience(
  entry: ExperienceEntry,
): EvidenceCandidate[] {
  const body = entry.body.trim();

  if (!body) {
    return [];
  }

  const candidates = [
    createCandidate(
      entry,
      `Observation from entry: ${firstFragment(body)}`,
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
      "感覺",
      "覺得",
      "生氣",
      "難過",
      "壓力",
      "擔心",
    ])
  ) {
    candidates.push(
      createCandidate(
        entry,
        "Possible emotion mentioned in this entry. Review before confirming.",
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
      "選擇",
      "決定",
      "取捨",
    ])
  ) {
    candidates.push(
      createCandidate(
        entry,
        "Possible decision or choice mentioned in this entry. Review before confirming.",
        "decision",
      ),
    );
  }

  if (candidates.length === 1) {
    candidates.push(
      createCandidate(
        entry,
        "Possible evidence candidate. Keep only if this helps you review the experience.",
        "other",
      ),
    );
  }

  return candidates;
}

const reflectionQuestionTemplates = [
  "What does this evidence make you notice about your current situation?",
  "Is there anything in this experience that feels familiar from the past?",
  "What part of this feels most important for you to understand?",
  "What feels unclear or unresolved when you read this evidence?",
];

export function generateReflectionPromptsFromConfirmedEvidence(
  entry: ExperienceEntry,
  confirmedEvidence: EvidenceCandidate[],
): ReflectionPrompt[] {
  return confirmedEvidence
    .filter((evidence) => evidence.status === "confirmed")
    .map((evidence, index) =>
      createReflectionPrompt(
        entry,
        evidence,
        reflectionQuestionTemplates[index % reflectionQuestionTemplates.length],
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
  const evidenceContext = shortText(reviewedEvidence[0].text);
  const reflectionContext = answeredReflectionPrompts[0]?.response
    ? ` This may connect with your own reflection response: "${shortText(
        answeredReflectionPrompts[0].response,
        120,
      )}".`
    : " Without an answered reflection yet, this should be treated as especially tentative.";

  return [
    createPatternNote(
      entry,
      reviewedEvidence,
      answeredReflectionPrompts,
      `One possible pattern to review is whether this experience contains a repeated theme around: "${evidenceContext}".${reflectionContext} You may want to examine whether this belongs to a broader pattern or only to this single moment.`,
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
