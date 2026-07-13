import type { EvidenceCandidate, ReflectionPrompt } from "../types/domain";
import {
  HISTORICAL_CONTEXT_ALGORITHM_VERSION,
  type HistoricalContextCandidate,
  type HistoricalContextRetrievalInput,
  type HistoricalSourceArtifacts,
} from "./types";

const DEFAULT_LIMIT = 3;
const MAX_LIMIT = 10;
const ENGLISH_STOP_WORDS = new Set([
  "about", "after", "again", "also", "because", "before", "been", "being",
  "could", "from", "have", "into", "just", "more", "over", "that", "their",
  "then", "there", "these", "they", "this", "were", "what", "when", "with",
  "would", "your",
]);

function stableCompare(left: string, right: string): number {
  return left === right ? 0 : left < right ? -1 : 1;
}

function localeForCaseNormalization(locale: HistoricalContextRetrievalInput["locale"]): string {
  switch (locale) {
    case "en":
      return "en-US";
    case "zh-TW":
      return "zh-TW";
    case "ja":
      return "ja-JP";
  }
}

function visibleTerms(text: string, locale: HistoricalContextRetrievalInput["locale"]): Set<string> {
  const normalized = text.normalize("NFKC").toLocaleLowerCase(localeForCaseNormalization(locale));
  if (locale === "en") {
    return new Set(
      (normalized.match(/[\p{L}\p{N}]{3,}/gu) ?? [])
        .filter((term) => !ENGLISH_STOP_WORDS.has(term)),
    );
  }

  const terms = new Set<string>();
  for (const run of normalized.match(/[\p{L}\p{N}]+/gu) ?? []) {
    const characters = [...run];
    for (let index = 0; index < characters.length - 1; index += 1) {
      terms.add(`${characters[index]}${characters[index + 1]}`);
    }
  }
  return terms;
}

function sharedTerms(currentTerms: Set<string>, sourceText: string, locale: HistoricalContextRetrievalInput["locale"]): string[] {
  return [...visibleTerms(sourceText, locale)]
    .filter((term) => currentTerms.has(term))
    .sort(stableCompare);
}

function sourceArtifacts(entryId: string, artifacts: HistoricalSourceArtifacts | undefined) {
  const confirmedEvidence = (artifacts?.evidence ?? []).filter(
    (evidence) => evidence.sourceEntryId === entryId && evidence.status === "confirmed",
  );
  const confirmedEvidenceIds = new Set(confirmedEvidence.map((evidence) => evidence.id));
  const answeredReflections = (artifacts?.reflections ?? []).filter(
    (reflection) =>
      reflection.sourceEntryId === entryId &&
      reflection.status === "answered" &&
      Boolean(reflection.response?.trim()) &&
      reflection.responseProvenance?.origin === "user" &&
      reflection.sourceEvidenceIds.length > 0 &&
      reflection.sourceEvidenceIds.every((id) => confirmedEvidenceIds.has(id)),
  );
  return { confirmedEvidence, answeredReflections };
}

function excerpt(body: string): string {
  const normalized = body.replace(/\s+/g, " ").trim();
  return normalized.length > 180 ? `${normalized.slice(0, 177)}…` : normalized;
}

function matchingArtifactIds<T extends EvidenceCandidate | ReflectionPrompt>(
  records: T[],
  currentTerms: Set<string>,
  locale: HistoricalContextRetrievalInput["locale"],
  text: (record: T) => string,
): string[] {
  return records
    .filter((record) => sharedTerms(currentTerms, text(record), locale).length > 0)
    .map((record) => record.id)
    .sort(stableCompare);
}

/**
 * Local-only, bounded lexical retrieval. It deliberately receives no provider,
 * Context Packet, or persisted selection state.
 */
export function findHistoricalContextCandidates(
  input: HistoricalContextRetrievalInput,
): HistoricalContextCandidate[] {
  const currentTerms = visibleTerms(input.currentExperience.body, input.locale);
  if (currentTerms.size === 0) return [];

  const candidates = input.experiences
    .filter((entry) => entry.id !== input.currentExperience.id)
    .map((entry) => {
      const { confirmedEvidence, answeredReflections } = sourceArtifacts(
        entry.id,
        input.artifactsByEntryId[entry.id],
      );
      const bodyMatches = sharedTerms(currentTerms, entry.body, input.locale);
      const evidenceMatches = confirmedEvidence.flatMap((evidence) =>
        sharedTerms(currentTerms, evidence.text, input.locale),
      );
      const reflectionMatches = answeredReflections.flatMap((reflection) =>
        sharedTerms(currentTerms, reflection.response ?? "", input.locale),
      );
      const matchedTerms = [...new Set([...bodyMatches, ...evidenceMatches, ...reflectionMatches])]
        .sort(stableCompare);
      if (matchedTerms.length === 0) return null;

      const confirmedEvidenceIds = matchingArtifactIds(
        confirmedEvidence,
        currentTerms,
        input.locale,
        (evidence) => evidence.text,
      );
      const answeredReflectionIds = matchingArtifactIds(
        answeredReflections,
        currentTerms,
        input.locale,
        (reflection) => reflection.response ?? "",
      );
      const score = matchedTerms.length * 10 + confirmedEvidenceIds.length * 2 + answeredReflectionIds.length * 2;

      return {
        sourceExperienceId: entry.id,
        sourceCreatedAt: entry.createdAt,
        sourceUpdatedAt: entry.updatedAt,
        sourceExcerpt: excerpt(entry.body),
        reasons: [{ kind: "shared_visible_terms" as const, terms: matchedTerms }],
        confirmedEvidenceIds,
        answeredReflectionIds,
        ranking: {
          algorithmVersion: HISTORICAL_CONTEXT_ALGORITHM_VERSION,
          score,
          matchedTermCount: matchedTerms.length,
        },
      } satisfies HistoricalContextCandidate;
    })
    .filter((candidate): candidate is HistoricalContextCandidate => candidate !== null)
    .sort((left, right) =>
      right.ranking.score - left.ranking.score ||
      stableCompare(right.sourceCreatedAt, left.sourceCreatedAt) ||
      stableCompare(left.sourceExperienceId, right.sourceExperienceId),
    );

  const limit = Math.min(Math.max(input.maxCandidates ?? DEFAULT_LIMIT, 0), MAX_LIMIT);
  return candidates.slice(0, limit);
}
