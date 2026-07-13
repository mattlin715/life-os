import type { AppLanguage } from "../app/i18n";
import type { EvidenceCandidate, ExperienceEntry, ReflectionPrompt } from "../types/domain";

export const HISTORICAL_CONTEXT_ALGORITHM_VERSION = "local-lexical-v1";

export interface HistoricalSourceArtifacts {
  evidence: EvidenceCandidate[];
  reflections: ReflectionPrompt[];
}

export interface HistoricalContextCandidateReason {
  kind: "shared_visible_terms";
  terms: string[];
}

export interface HistoricalContextCandidate {
  sourceExperienceId: string;
  sourceCreatedAt: string;
  sourceUpdatedAt: string;
  sourceExcerpt: string;
  reasons: HistoricalContextCandidateReason[];
  confirmedEvidenceIds: string[];
  answeredReflectionIds: string[];
  ranking: {
    algorithmVersion: typeof HISTORICAL_CONTEXT_ALGORITHM_VERSION;
    score: number;
    matchedTermCount: number;
  };
}

export interface HistoricalContextRetrievalInput {
  currentExperience: ExperienceEntry;
  experiences: ExperienceEntry[];
  artifactsByEntryId: Record<string, HistoricalSourceArtifacts | undefined>;
  locale: AppLanguage;
  maxCandidates?: number;
}
