import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import type { HistoricalContextCandidate } from "../historicalContext/types";
import { HistoricalCandidateRelevance } from "./HistoricalCandidateRelevance";
import { uiText } from "./i18n";

const candidate: HistoricalContextCandidate = {
  sourceExperienceId: "source",
  sourceCreatedAt: "2026-08-01T00:00:00.000Z",
  sourceUpdatedAt: "2026-08-01T00:00:00.000Z",
  sourceExcerpt: "A project note.",
  reasons: [{ kind: "shared_visible_terms", terms: ["project", "feedback", "response"] }],
  visibleMatches: [
    { kind: "experience", terms: ["project"], excerpt: "A project note." },
    { kind: "confirmed_evidence", artifactId: "evidence", terms: ["feedback"], excerpt: "The feedback was specific." },
    { kind: "saved_reflection", artifactId: "reflection", terms: ["response"], excerpt: "My response was calm." },
  ],
  confirmedEvidenceIds: ["evidence"],
  answeredReflectionIds: ["reflection"],
  ranking: { algorithmVersion: "local-lexical-v2", score: 34, matchedTermCount: 3 },
};

describe("HistoricalCandidateRelevance", () => {
  it.each(["en", "zh-TW", "ja"] as const)(
    "shows exact local match origins and excerpts in %s",
    (locale) => {
      const copy = uiText[locale];
      const html = renderToStaticMarkup(
        <HistoricalCandidateRelevance candidate={candidate} copy={copy} />,
      );

      expect(html).toContain(copy.historicalContextMatchLocations);
      expect(html).toContain(copy.historicalContextExperienceMatch("project"));
      expect(html).toContain(copy.historicalContextEvidenceMatch("feedback"));
      expect(html).toContain(copy.historicalContextReflectionMatch("response"));
      expect(html).toContain("A project note.");
      expect(html).toContain("The feedback was specific.");
      expect(html).toContain("My response was calm.");
    },
  );

  it("preserves the aggregate relevance fallback for legacy candidate fixtures", () => {
    const legacy = { ...candidate, visibleMatches: undefined };
    const html = renderToStaticMarkup(
      <HistoricalCandidateRelevance candidate={legacy} copy={uiText.en} />,
    );
    expect(html).toContain(uiText.en.historicalContextReason("project, feedback, response"));
  });
});
