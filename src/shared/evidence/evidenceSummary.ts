import type { EvidenceCandidate } from "../../types/domain";

export interface EvidenceCandidateSummary {
  total: number;
  confirmed: number;
  rejected: number;
  pending: number;
}

export function summarizeEvidenceCandidates(
  candidates: EvidenceCandidate[],
): EvidenceCandidateSummary {
  return candidates.reduce<EvidenceCandidateSummary>(
    (summary, candidate) => {
      summary.total += 1;

      if (candidate.status === "confirmed") {
        summary.confirmed += 1;
      } else if (candidate.status === "rejected") {
        summary.rejected += 1;
      } else {
        summary.pending += 1;
      }

      return summary;
    },
    {
      total: 0,
      confirmed: 0,
      rejected: 0,
      pending: 0,
    },
  );
}

export function combineEvidenceCandidateSummaries(
  summaries: EvidenceCandidateSummary[],
): EvidenceCandidateSummary {
  return summaries.reduce<EvidenceCandidateSummary>(
    (total, summary) => ({
      total: total.total + summary.total,
      confirmed: total.confirmed + summary.confirmed,
      rejected: total.rejected + summary.rejected,
      pending: total.pending + summary.pending,
    }),
    {
      total: 0,
      confirmed: 0,
      rejected: 0,
      pending: 0,
    },
  );
}
