import type { AIProvider } from "./types";
import type { EvidenceCandidate, ExperienceEntry } from "../../types/domain";

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
    kind,
    status: "candidate",
    userEditable: true,
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

export function extractEvidenceCandidatesFromExperience(
  entry: ExperienceEntry,
): EvidenceCandidate[] {
  const body = entry.body.trim();

  if (!body) {
    return [];
  }

  const candidates = [
    createCandidate(entry, `Observation from entry: ${firstFragment(body)}`, "observation"),
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
      "開心",
      "難過",
      "生氣",
      "焦慮",
      "害怕",
      "壓力",
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
      "打算",
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

export const placeholderProvider: AIProvider = {
  async extractEvidence(entry) {
    return extractEvidenceCandidatesFromExperience(entry);
  },
  async generateReflectionPrompts() {
    return [];
  },
  async suggestPatternNotes() {
    return [];
  },
};
