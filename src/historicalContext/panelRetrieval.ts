import type { AppLanguage } from "../app/i18n";
import type { ExperienceEntry } from "../types/domain";
import { findHistoricalContextCandidates } from "./retrieve";
import type {
  HistoricalContextCandidate,
  HistoricalContextRetrievalInput,
  HistoricalSourceArtifacts,
} from "./types";

export type HistoricalContextCandidateRetriever = (
  input: HistoricalContextRetrievalInput,
) => HistoricalContextCandidate[];

export interface OpenHistoricalPanelRetrievalInput {
  /** Exact IDs of panels which the user has explicitly opened this session. */
  openPanelExperienceIds: ReadonlySet<string>;
  /** Current durable Experience list. Artifact-only IDs cannot be retrieved. */
  experiences: ExperienceEntry[];
  artifactsByEntryId: Record<string, HistoricalSourceArtifacts | undefined>;
  locale: AppLanguage;
  maxCandidates?: number;
  /** Injectable only for deterministic lifecycle tests; production uses local retrieval. */
  retrieveCandidates?: HistoricalContextCandidateRetriever;
}

/**
 * Explicit-panel retrieval boundary. Closed panels do not invoke retrieval;
 * every returned map key is an exact ID from the current Experience list.
 */
export function retrieveCandidatesForOpenHistoricalPanels({
  openPanelExperienceIds,
  experiences,
  artifactsByEntryId,
  locale,
  maxCandidates,
  retrieveCandidates = findHistoricalContextCandidates,
}: OpenHistoricalPanelRetrievalInput): ReadonlyMap<string, HistoricalContextCandidate[]> {
  const candidatesByCurrentExperienceId = new Map<string, HistoricalContextCandidate[]>();

  for (const currentExperience of experiences) {
    if (!openPanelExperienceIds.has(currentExperience.id) || candidatesByCurrentExperienceId.has(currentExperience.id)) {
      continue;
    }

    candidatesByCurrentExperienceId.set(currentExperience.id, retrieveCandidates({
      currentExperience,
      experiences,
      artifactsByEntryId,
      locale,
      maxCandidates,
    }));
  }

  return candidatesByCurrentExperienceId;
}
