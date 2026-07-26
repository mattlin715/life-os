import type { AppLanguage } from "../app/i18n";
import type { ExperienceEntry } from "../types/domain";
import { findHistoricalContextCandidates } from "./retrieve";
import type { HistoricalSavedDateRangeConstraint } from "./savedDateRange";
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
  /** Missing entries are inactive, preserving the Phase 3A retrieval path. */
  savedDateRangeConstraintsByCurrentExperienceId?: ReadonlyMap<
    string,
    HistoricalSavedDateRangeConstraint
  >;
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
  savedDateRangeConstraintsByCurrentExperienceId,
  retrieveCandidates = findHistoricalContextCandidates,
}: OpenHistoricalPanelRetrievalInput): ReadonlyMap<string, HistoricalContextCandidate[]> {
  const candidatesByCurrentExperienceId = new Map<string, HistoricalContextCandidate[]>();

  for (const currentExperience of experiences) {
    if (!openPanelExperienceIds.has(currentExperience.id) || candidatesByCurrentExperienceId.has(currentExperience.id)) {
      continue;
    }

    const constraint = savedDateRangeConstraintsByCurrentExperienceId?.get(
      currentExperience.id,
    );
    if (constraint?.status === "blocked") {
      candidatesByCurrentExperienceId.set(currentExperience.id, []);
      continue;
    }

    candidatesByCurrentExperienceId.set(currentExperience.id, retrieveCandidates({
      currentExperience,
      experiences,
      artifactsByEntryId,
      locale,
      maxCandidates,
      ...(constraint?.status === "applied"
        ? { savedDateRange: constraint.range }
        : {}),
    }));
  }

  return candidatesByCurrentExperienceId;
}
