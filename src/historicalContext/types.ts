import type { AppLanguage } from "../app/i18n";
import type { EvidenceCandidate, ExperienceEntry, ReflectionPrompt } from "../types/domain";
import type { HistoricalSavedDateRange } from "./savedDateRange";

export const HISTORICAL_CONTEXT_ALGORITHM_VERSION = "local-lexical-v1";

export interface HistoricalSourceArtifacts {
  /** Supplied only to the governed Phase 3B assembler; Phase 3A retrieval ignores it. */
  experience?: ExperienceEntry;
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
  /** Optional explicit source-Experience createdAt constraint; never persisted or transmitted. */
  savedDateRange?: HistoricalSavedDateRange;
}
