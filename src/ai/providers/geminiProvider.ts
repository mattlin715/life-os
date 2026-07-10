import { invoke, isTauri } from "@tauri-apps/api/core";

import type { AIProvider } from "./types";
import type {
  EvidenceCandidate,
  ExperienceEntry,
  PatternNote,
  ReflectionPrompt,
} from "../../types/domain";

type EvidenceKind = EvidenceCandidate["kind"];

const evidenceKinds: EvidenceKind[] = [
  "observation",
  "emotion",
  "decision",
  "contradiction",
  "self_description",
  "other",
];

function createId(): string {
  return globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
}

function timestamp(): string {
  return new Date().toISOString();
}

function stripJsonFences(text: string): string {
  return text
    .trim()
    .replace(/^```(?:json)?/i, "")
    .replace(/```$/i, "")
    .trim();
}

function parseJsonObject(text: string): unknown {
  return JSON.parse(stripJsonFences(text));
}

function asRecord(value: unknown): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error("AI response was not a JSON object.");
  }

  return value as Record<string, unknown>;
}

function asStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }

  return value
    .map((item) => (typeof item === "string" ? item.trim() : ""))
    .filter(Boolean);
}

function normalizeEvidenceKind(kind: unknown): EvidenceKind {
  return typeof kind === "string" && evidenceKinds.includes(kind as EvidenceKind)
    ? (kind as EvidenceKind)
    : "other";
}

function normalizeForAnchor(text: string): string {
  return text
    .replace(/\s+/g, "")
    .replace(/[「」『』"“”'.,。！？!?：:；;、，]/gu, "")
    .toLowerCase();
}

function hasEvidenceAnchor(
  text: string,
  confirmedEvidence: EvidenceCandidate[],
): boolean {
  const normalizedText = normalizeForAnchor(text);

  return confirmedEvidence.some((evidence) => {
    const normalizedEvidence = normalizeForAnchor(evidence.text);

    if (normalizedEvidence.length < 4) {
      return false;
    }

    const anchor =
      normalizedEvidence.length > 24
        ? normalizedEvidence.slice(0, 24)
        : normalizedEvidence;

    return normalizedText.includes(anchor);
  });
}

async function requestGeminiJson(
  instructions: string,
  input: unknown,
): Promise<Record<string, unknown>> {
  if (!isTauri()) {
    throw new Error("Gemini provider is only enabled in the Tauri desktop runtime.");
  }

  const text = await invoke<string>("generate_gemini_response", {
    instructions,
    input: JSON.stringify(input, null, 2),
  });

  return asRecord(parseJsonObject(text));
}

function mirrorRules(): string {
  return [
    "You are helping Life OS, a product that builds mirrors, not oracles.",
    "Match the user's primary language and writing system for every user-facing string.",
    "Keep JSON keys in English, but write values such as text and questions in the same language as the experience.",
    "If the experience is written in Traditional Chinese, respond in Traditional Chinese.",
    "Never diagnose.",
    "Never infer MBTI, personality type, mental health labels, identity labels, or fixed traits.",
    "Never give advice or tell the user what to do.",
    "Never use certainty language like 'clearly', 'this proves', or 'you are'.",
    "Use gentle uncertainty language.",
    "Prefer questions before conclusions.",
    "Return strict JSON only. No markdown. No commentary.",
  ].join("\n");
}

export const geminiProvider: AIProvider = {
  async extractEvidence(entry: ExperienceEntry): Promise<EvidenceCandidate[]> {
    const data = await requestGeminiJson(
      [
        mirrorRules(),
        "Task: extract 2 to 4 evidence candidates from one user-authored experience.",
        "Each candidate should be observable or reviewable.",
        "Do not explain what the user's life means.",
        "Return shape: { \"candidates\": [{ \"kind\": \"observation|emotion|decision|contradiction|self_description|other\", \"text\": \"...\" }] }",
      ].join("\n"),
      {
        experience: entry.body,
      },
    );

    const candidates = Array.isArray(data.candidates) ? data.candidates : [];

    const normalizedCandidates = candidates
      .map((candidate): EvidenceCandidate | null => {
        const record = asRecord(candidate);
        const text = typeof record.text === "string" ? record.text.trim() : "";

        if (!text) {
          return null;
        }

        const now = timestamp();

        return {
          id: createId(),
          sourceEntryId: entry.id,
          text,
          originalText: text,
          kind: normalizeEvidenceKind(record.kind),
          status: "candidate",
          userEditable: true,
          createdAt: now,
          updatedAt: now,
        };
      })
      .filter((candidate): candidate is EvidenceCandidate => Boolean(candidate));

    if (normalizedCandidates.length === 0) {
      throw new Error("Gemini returned no evidence candidates.");
    }

    return normalizedCandidates;
  },

  async generateReflectionPrompts(
    entry: ExperienceEntry,
    confirmedEvidence: EvidenceCandidate[],
  ): Promise<ReflectionPrompt[]> {
    const data = await requestGeminiJson(
      [
        mirrorRules(),
        "Task: generate 2 to 3 calm reflection questions.",
        "The questions must be based only on confirmed evidence.",
        "Ask questions that help the user notice what stands out, what feels important, or what may have another interpretation.",
        "Do not answer for the user.",
        "Return shape: { \"questions\": [\"...\", \"...\"] }",
      ].join("\n"),
      {
        experience: entry.body,
        confirmedEvidence: confirmedEvidence.map((evidence) => ({
          id: evidence.id,
          kind: evidence.kind,
          text: evidence.text,
        })),
      },
    );

    const questions = asStringArray(data.questions).slice(0, 3);

    if (questions.length === 0) {
      throw new Error("Gemini returned no reflection questions.");
    }

    return questions.map((question, index) => {
      const now = timestamp();
      const evidence = confirmedEvidence[index % confirmedEvidence.length];

      return {
        id: createId(),
        sourceEntryId: entry.id,
        sourceEvidenceIds: evidence ? [evidence.id] : [],
        question,
        status: "suggested",
        createdAt: now,
        updatedAt: now,
      };
    });
  },

  async suggestPatternNotes(
    entry: ExperienceEntry,
    confirmedEvidence: EvidenceCandidate[],
    reflectionPrompts: ReflectionPrompt[],
  ): Promise<PatternNote[]> {
    const answeredReflection = reflectionPrompts
      .filter((prompt) => prompt.status === "answered" && prompt.response?.trim())
      .map((prompt) => ({
        id: prompt.id,
        question: prompt.question,
        response: prompt.response,
      }));

    const data = await requestGeminiJson(
      [
        mirrorRules(),
        "Task: propose exactly one pattern candidate as a hypothesis for user review.",
        "Use only the provided experience, confirmed evidence, and answered reflection.",
        "Do not introduce events, motives, causes, or contexts that are not present in the input.",
        "Anchor the hypothesis in one or two concrete phrases from the confirmed evidence.",
        "The text must include at least one exact short phrase from confirmed evidence.",
        "Keep it short: one sentence if possible, two sentences at most.",
        "It must begin with uncertainty language in the user's language.",
        "For English, examples include 'One possible pattern to review is...' or 'This may suggest...'.",
        "For Traditional Chinese, examples include '一個可以檢視的可能模式是...' or '這也許暗示...'.",
        "Do not create advice. Do not create a growth note. Do not label the user's identity.",
        "Return shape: { \"text\": \"...\" }",
      ].join("\n"),
      {
        experience: entry.body,
        confirmedEvidence: confirmedEvidence.map((evidence) => ({
          id: evidence.id,
          kind: evidence.kind,
          text: evidence.text,
        })),
        answeredReflection,
      },
    );

    const text = typeof data.text === "string" ? data.text.trim() : "";

    if (!text) {
      throw new Error("Gemini returned no pattern candidate.");
    }

    if (!hasEvidenceAnchor(text, confirmedEvidence)) {
      throw new Error(
        "Gemini pattern candidate was not anchored in confirmed evidence.",
      );
    }

    const now = timestamp();

    return [
      {
        id: createId(),
        sourceEntryId: entry.id,
        sourceEvidenceIds: confirmedEvidence.map((evidence) => evidence.id),
        sourceReflectionPromptIds:
          answeredReflection.length > 0
            ? answeredReflection.map((prompt) => prompt.id)
            : undefined,
        text,
        status: "candidate",
        createdAt: now,
        updatedAt: now,
      },
    ];
  },
};
