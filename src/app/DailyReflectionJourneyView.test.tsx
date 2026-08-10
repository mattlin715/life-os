import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import type { PatternNote, ReflectionPrompt } from "../types/domain";
import { JourneyStage, ReflectionCompletionReview } from "./DailyReflectionJourneyView";
import { uiText } from "./i18n";

const stamp = "2026-08-11T00:00:00.000Z";
const reflection: ReflectionPrompt = {
  id: "reflection",
  sourceEntryId: "entry",
  sourceEvidenceIds: ["evidence"],
  question: "What mattered?",
  status: "answered",
  response: "I wanted to be heard.",
  promptProvenance: { origin: "ai", sourceEntryId: "entry", sourceArtifactIds: ["evidence"] },
  responseProvenance: { origin: "user", sourceEntryId: "entry", sourceArtifactIds: ["reflection"] },
  createdAt: stamp,
  updatedAt: stamp,
};
const pattern: PatternNote = {
  id: "pattern",
  sourceEntryId: "entry",
  sourceEvidenceIds: ["evidence"],
  sourceReflectionPromptIds: ["reflection"],
  text: "A tentative interpretation",
  status: "candidate",
  provenance: { origin: "ai", sourceEntryId: "entry", sourceArtifactIds: ["evidence", "reflection"] },
  createdAt: stamp,
  updatedAt: stamp,
};

describe("Daily Reflection journey surfaces", () => {
  it("renders only the open stage body and exposes an explicit reopen control", () => {
    const closed = renderToStaticMarkup(
      <JourneyStage id="stage" step="1" title="Evidence" summary="1 confirmed" statusLabel="Complete" active={false} open={false} available onOpen={vi.fn()}>
        <p>private stage detail</p>
      </JourneyStage>,
    );
    expect(closed).toContain("1 confirmed");
    expect(closed).toContain('aria-expanded="false"');
    expect(closed).not.toContain("private stage detail");
  });

  it.each(["en", "zh-TW", "ja"] as const)("keeps completion record-only and authorship-distinct in %s", (locale) => {
    const copy = uiText[locale];
    const html = renderToStaticMarkup(
      <ReflectionCompletionReview
        id="completion"
        copy={copy}
        experienceBody="Original moment"
        confirmedEvidence={["Confirmed clue"]}
        reflections={[reflection]}
        patterns={[pattern]}
        patternRejectedInSession={false}
        historicalQuestions={[]}
        onReopenEvidence={vi.fn()}
        onReopenReflection={vi.fn()}
        onOpenPattern={vi.fn()}
        onOpenHistory={vi.fn()}
        onRecordAnother={vi.fn()}
      />,
    );
    expect(html).toContain("Original moment");
    expect(html).toContain("Confirmed clue");
    expect(html).toContain("What mattered?");
    expect(html).toContain("I wanted to be heard.");
    expect(html).toContain(copy.completionPromptLabel("ai"));
    expect(html).toContain(copy.completionUserResponseLabel);
    expect(html).toContain(copy.completionCanRest);
  });

  it("labels local-mock prompts honestly rather than as provider AI success", () => {
    const localPrompt = { ...reflection, promptProvenance: { ...reflection.promptProvenance, origin: "local_mock" as const } };
    const html = renderToStaticMarkup(
      <ReflectionCompletionReview
        id="completion"
        copy={uiText.en}
        experienceBody="Original moment"
        confirmedEvidence={["Confirmed clue"]}
        reflections={[localPrompt]}
        patterns={[]}
        patternRejectedInSession={false}
        historicalQuestions={[]}
        onReopenEvidence={vi.fn()}
        onReopenReflection={vi.fn()}
        onOpenPattern={vi.fn()}
        onOpenHistory={vi.fn()}
        onRecordAnother={vi.fn()}
      />,
    );
    expect(html).toContain(uiText.en.completionPromptLabel("local_mock"));
    expect(html).not.toContain(uiText.en.completionAiPromptLabel);
  });

  it("does not introduce generated synthesis, recurrence, diagnosis, or identity claims", () => {
    const html = renderToStaticMarkup(
      <ReflectionCompletionReview
        id="completion"
        copy={uiText.en}
        experienceBody="Original moment"
        confirmedEvidence={["Confirmed clue"]}
        reflections={[reflection]}
        patterns={[]}
        patternRejectedInSession
        historicalQuestions={[]}
        onReopenEvidence={vi.fn()}
        onReopenReflection={vi.fn()}
        onOpenPattern={vi.fn()}
        onOpenHistory={vi.fn()}
        onRecordAnother={vi.fn()}
      />,
    );
    expect(html).toContain(uiText.en.completionPatternRejected);
    expect(html).not.toMatch(/recurr|diagnos|identity|personality/i);
    expect(html).not.toContain("completion-synthesis");
  });
});
