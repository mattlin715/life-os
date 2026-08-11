import type { EvidenceCandidate, ReflectionPrompt } from "../types/domain";
import { isTimestampInHistoricalSavedDateRange } from "./savedDateRange";
import {
  HISTORICAL_CONTEXT_ALGORITHM_VERSION,
  type HistoricalContextCandidate,
  type HistoricalContextRetrievalInput,
  type HistoricalSourceArtifacts,
  type HistoricalContextVisibleMatch,
} from "./types";

const DEFAULT_LIMIT = 3;
const MAX_LIMIT = 10;
const ENGLISH_STOP_WORDS = new Set([
  "about", "after", "again", "also", "because", "before", "been", "being",
  "could", "from", "have", "into", "just", "more", "over", "that", "their",
  "then", "there", "these", "they", "this", "were", "what", "when", "with",
  "would", "your",
]);

const CJK_STOP_TERMS = new Set([
  // Traditional Chinese function words and generic self-report scaffolding.
  "一個", "一直", "但是", "因為", "所以", "今天", "昨天", "沒有", "以及",
  "感到", "感覺", "覺得", "覺得很", "認為", "需要", "這個", "那個", "這些", "那些",
  // Japanese particles, auxiliaries, and generic self-report scaffolding.
  "から", "こと", "これ", "それ", "ため", "です", "でした", "ます", "ました",
  "感じ", "感じる", "思う", "思った", "いる", "ある", "する", "した", "だった",
]);

interface WordSegment {
  segment: string;
  isWordLike?: boolean;
}

interface SegmenterLike {
  segment(input: string): Iterable<WordSegment>;
}

type SegmenterConstructor = new (
  locale: string,
  options: { granularity: "word" },
) => SegmenterLike;

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

function meaningfulCjkTerm(term: string): boolean {
  const characters = [...term];
  if (characters.length < 2 || CJK_STOP_TERMS.has(term)) return false;
  if (/^[\p{Script=Latin}\p{N}]+$/u.test(term)) {
    return characters.length >= 3 && !ENGLISH_STOP_WORDS.has(term);
  }
  return true;
}

function cjkVisibleTerms(normalized: string, locale: "zh-TW" | "ja"): Set<string> {
  const Segmenter = (Intl as unknown as { Segmenter?: SegmenterConstructor }).Segmenter;
  if (!Segmenter) {
    // Conservative fallback: keep complete visible runs rather than inventing
    // adjacent-character fragments across unknown word boundaries.
    return new Set(
      (normalized.match(/[\p{L}\p{N}]+/gu) ?? []).filter(meaningfulCjkTerm),
    );
  }

  const segmenter = new Segmenter(localeForCaseNormalization(locale), {
    granularity: "word",
  });
  return new Set(
    [...segmenter.segment(normalized)]
      .filter((part) => part.isWordLike !== false)
      .map((part) => part.segment.trim())
      .filter(meaningfulCjkTerm),
  );
}

function visibleTerms(text: string, locale: HistoricalContextRetrievalInput["locale"]): Set<string> {
  const normalized = text.normalize("NFKC").toLocaleLowerCase(localeForCaseNormalization(locale));
  if (locale === "en") {
    return new Set(
      (normalized.match(/[\p{L}\p{N}]{3,}/gu) ?? [])
        .filter((term) => !ENGLISH_STOP_WORDS.has(term)),
    );
  }
  return cjkVisibleTerms(normalized, locale);
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

function matchingArtifactDisclosures<T extends EvidenceCandidate | ReflectionPrompt>(
  records: T[],
  currentTerms: Set<string>,
  locale: HistoricalContextRetrievalInput["locale"],
  text: (record: T) => string,
  kind: HistoricalContextVisibleMatch["kind"],
): HistoricalContextVisibleMatch[] {
  return records
    .map((record): HistoricalContextVisibleMatch | null => {
      const content = text(record);
      const terms = sharedTerms(currentTerms, content, locale);
      return terms.length > 0
        ? { kind, artifactId: record.id, terms, excerpt: excerpt(content) }
        : null;
    })
    .filter((match): match is HistoricalContextVisibleMatch => match !== null)
    .sort((left, right) => stableCompare(left.artifactId ?? "", right.artifactId ?? ""));
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
    .filter(
      (entry) =>
        !input.savedDateRange ||
        isTimestampInHistoricalSavedDateRange(entry.createdAt, input.savedDateRange),
    )
    .map((entry): HistoricalContextCandidate | null => {
      const { confirmedEvidence, answeredReflections } = sourceArtifacts(
        entry.id,
        input.artifactsByEntryId[entry.id],
      );
      const bodyMatches = sharedTerms(currentTerms, entry.body, input.locale);
      const evidenceDisclosures = matchingArtifactDisclosures(
        confirmedEvidence,
        currentTerms,
        input.locale,
        (evidence) => evidence.text,
        "confirmed_evidence",
      );
      const reflectionDisclosures = matchingArtifactDisclosures(
        answeredReflections,
        currentTerms,
        input.locale,
        (reflection) => reflection.response ?? "",
        "saved_reflection",
      );
      const matchedTerms = [...new Set([
        ...bodyMatches,
        ...evidenceDisclosures.flatMap((match) => match.terms),
        ...reflectionDisclosures.flatMap((match) => match.terms),
      ])]
        .sort(stableCompare);
      if (matchedTerms.length === 0) return null;

      const confirmedEvidenceIds = evidenceDisclosures.map((match) => match.artifactId!);
      const answeredReflectionIds = reflectionDisclosures.map((match) => match.artifactId!);
      const score = matchedTerms.length * 10 + confirmedEvidenceIds.length * 2 + answeredReflectionIds.length * 2;

      return {
        sourceExperienceId: entry.id,
        sourceCreatedAt: entry.createdAt,
        sourceUpdatedAt: entry.updatedAt,
        sourceExcerpt: excerpt(entry.body),
        reasons: [{ kind: "shared_visible_terms" as const, terms: matchedTerms }],
        visibleMatches: [
          ...(bodyMatches.length > 0
            ? [{ kind: "experience" as const, terms: bodyMatches, excerpt: excerpt(entry.body) }]
            : []),
          ...evidenceDisclosures,
          ...reflectionDisclosures,
        ],
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
