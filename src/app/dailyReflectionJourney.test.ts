import { describe, expect, it } from "vitest";
import {
  resolveDailyReflectionJourney,
  resolveDisplayedJourneyStage,
  stageForNextAction,
  type DailyReflectionJourneyInput,
} from "./dailyReflectionJourney";

const base: DailyReflectionJourneyInput = {
  experienceSaved: true,
  contextClarificationInvited: false,
  evidenceTotal: 0,
  evidencePending: 0,
  evidenceConfirmed: 0,
  reflectionTotal: 0,
  reflectionSuggested: 0,
  reflectionAnswered: 0,
  reflectionSkipped: 0,
  hasDirtyReflectionDraft: false,
  patternAvailable: false,
  patternCandidates: 0,
  patternConfirmed: 0,
  historicalQuestionCount: 0,
};

describe("resolveDailyReflectionJourney", () => {
  it("allows the derived active stage to be explicitly collapsed and reopened", () => {
    const journey = resolveDailyReflectionJourney(base);

    expect(resolveDisplayedJourneyStage(undefined, journey)).toBe("evidence");
    expect(resolveDisplayedJourneyStage("collapsed", journey)).toBeNull();
    expect(resolveDisplayedJourneyStage("evidence", journey)).toBe("evidence");
  });

  it("starts a saved Experience at explicit Evidence generation", () => {
    expect(resolveDailyReflectionJourney(base)).toMatchObject({
      experienceSaved: true,
      evidence: "not_generated",
      activeStage: "evidence",
      nextAction: "generate_evidence",
    });
  });

  it("keeps sparse input in Evidence while clarification is invited", () => {
    expect(resolveDailyReflectionJourney({ ...base, contextClarificationInvited: true })).toMatchObject({
      contextClarificationInvited: true,
      activeStage: "evidence",
      nextAction: "answer_context",
    });
  });

  it("keeps pending or wholly unconfirmed Evidence active", () => {
    expect(resolveDailyReflectionJourney({ ...base, evidenceTotal: 2, evidencePending: 1 }).evidence).toBe("awaiting_review");
    expect(resolveDailyReflectionJourney({ ...base, evidenceTotal: 2 }).evidence).toBe("no_confirmed");
    expect(resolveDailyReflectionJourney({ ...base, evidenceTotal: 2 }).nextAction).toBe("review_evidence");
  });

  it("moves from confirmed Evidence to Reflection generation", () => {
    expect(resolveDailyReflectionJourney({ ...base, evidenceTotal: 1, evidenceConfirmed: 1 })).toMatchObject({
      evidence: "complete",
      reflection: "not_generated",
      activeStage: "reflection",
      nextAction: "generate_reflection",
    });
  });

  it("keeps unanswered prompts and dirty drafts in Reflection", () => {
    const reflection = { ...base, evidenceTotal: 1, evidenceConfirmed: 1, reflectionTotal: 1, reflectionSuggested: 1 };
    expect(resolveDailyReflectionJourney(reflection).nextAction).toBe("resolve_reflection");
    expect(resolveDailyReflectionJourney({ ...reflection, hasDirtyReflectionDraft: true })).toMatchObject({
      reflection: "dirty_unsaved",
      activeStage: "reflection",
      nextAction: "save_reflection",
      coreReflectionComplete: false,
    });
  });

  it.each([
    { reflectionAnswered: 1, reflectionSkipped: 0 },
    { reflectionAnswered: 0, reflectionSkipped: 1 },
  ])("treats answered and explicitly skipped Reflection as honest core completion", (resolution) => {
    expect(resolveDailyReflectionJourney({
      ...base,
      evidenceTotal: 1,
      evidenceConfirmed: 1,
      reflectionTotal: 1,
      ...resolution,
    })).toMatchObject({
      reflection: "complete",
      coreReflectionComplete: true,
      activeStage: "completion",
      nextAction: "review_completion",
    });
  });

  it("keeps Pattern optional and represents each review state without changing core completion", () => {
    const complete = { ...base, evidenceTotal: 1, evidenceConfirmed: 1, reflectionTotal: 1, reflectionAnswered: 1, patternAvailable: true };
    expect(resolveDailyReflectionJourney(complete)).toMatchObject({ pattern: "available", coreReflectionComplete: true });
    expect(resolveDailyReflectionJourney({ ...complete, patternCandidates: 1 }).pattern).toBe("awaiting_review");
    expect(resolveDailyReflectionJourney({ ...complete, patternConfirmed: 1 }).pattern).toBe("confirmed");
    expect(resolveDailyReflectionJourney({ ...complete, patternRejectedInSession: true }).pattern).toBe("rejected");
  });

  it("reports only already persisted historical-question availability", () => {
    expect(resolveDailyReflectionJourney({ ...base, historicalQuestionCount: 1 }).historicalReflectionAvailable).toBe(true);
  });

  it("reconstructs the same durable journey from the same persisted counts", () => {
    const persisted = { ...base, evidenceTotal: 2, evidenceConfirmed: 1, reflectionTotal: 2, reflectionAnswered: 1, reflectionSkipped: 1, patternConfirmed: 1, historicalQuestionCount: 2 };
    expect(resolveDailyReflectionJourney({ ...persisted })).toEqual(resolveDailyReflectionJourney({ ...persisted }));
  });

  it("maps next actions to focus stages without introducing an action executor", () => {
    expect(stageForNextAction("answer_context")).toBe("evidence");
    expect(stageForNextAction("review_evidence")).toBe("evidence");
    expect(stageForNextAction("save_reflection")).toBe("reflection");
    expect(stageForNextAction("review_completion")).toBe("completion");
  });

  it("contains no Phase 4 conclusion or identity field", () => {
    const journey = resolveDailyReflectionJourney(base);
    expect(Object.keys(journey)).not.toEqual(expect.arrayContaining(["recurrence", "contradiction", "identity", "diagnosis", "summary"]));
  });
});
