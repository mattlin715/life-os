import type { HistoricalContextCandidate } from "../historicalContext/types";
import type { UiCopy } from "./i18n";

interface HistoricalCandidateRelevanceProps {
  readonly candidate: HistoricalContextCandidate;
  readonly copy: UiCopy;
}

export function HistoricalCandidateRelevance({
  candidate,
  copy,
}: HistoricalCandidateRelevanceProps) {
  if (!candidate.visibleMatches?.length) {
    return (
      <p className="historical-context-reason">
        {copy.historicalContextReason(
          candidate.reasons.flatMap((reason) => reason.terms).join(", "),
        )}
      </p>
    );
  }

  return (
    <div className="historical-context-relevance">
      <strong>{copy.historicalContextMatchLocations}</strong>
      <ul>
        {candidate.visibleMatches.map((match, index) => {
          const terms = match.terms.join(", ");
          const label = match.kind === "experience"
            ? copy.historicalContextExperienceMatch(terms)
            : match.kind === "confirmed_evidence"
              ? copy.historicalContextEvidenceMatch(terms)
              : copy.historicalContextReflectionMatch(terms);
          return (
            <li key={`${match.kind}:${match.artifactId ?? "experience"}:${index}`}>
              <p className="historical-context-reason">{label}</p>
              <blockquote>{match.excerpt}</blockquote>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
