export type DailyReflectionStage = "evidence" | "reflection" | "pattern" | "completion";

export type DailyReflectionStageOverride =
  | DailyReflectionStage
  | "collapsed"
  | null
  | undefined;

export type DailyReflectionNextAction =
  | "answer_context"
  | "generate_evidence"
  | "review_evidence"
  | "generate_reflection"
  | "resolve_reflection"
  | "save_reflection"
  | "review_completion";

export type EvidenceJourneyState =
  | "not_generated"
  | "awaiting_review"
  | "no_confirmed"
  | "complete";

export type ReflectionJourneyState =
  | "unavailable"
  | "not_generated"
  | "awaiting_resolution"
  | "dirty_unsaved"
  | "complete";

export type PatternJourneyState =
  | "unavailable"
  | "available"
  | "awaiting_review"
  | "confirmed"
  | "rejected";

export interface DailyReflectionJourneyInput {
  readonly experienceSaved: boolean;
  readonly contextClarificationInvited: boolean;
  readonly evidenceTotal: number;
  readonly evidencePending: number;
  readonly evidenceConfirmed: number;
  readonly reflectionTotal: number;
  readonly reflectionSuggested: number;
  readonly reflectionAnswered: number;
  readonly reflectionSkipped: number;
  readonly hasDirtyReflectionDraft: boolean;
  readonly patternAvailable: boolean;
  readonly patternCandidates: number;
  readonly patternConfirmed: number;
  readonly patternRejectedInSession?: boolean;
  readonly historicalQuestionCount: number;
}

export interface DailyReflectionJourney {
  readonly experienceSaved: boolean;
  readonly contextClarificationInvited: boolean;
  readonly evidence: EvidenceJourneyState;
  readonly reflection: ReflectionJourneyState;
  readonly pattern: PatternJourneyState;
  readonly historicalReflectionAvailable: boolean;
  readonly coreReflectionComplete: boolean;
  readonly activeStage: DailyReflectionStage;
  readonly nextAction: DailyReflectionNextAction;
}

function resolveEvidence(input: DailyReflectionJourneyInput): EvidenceJourneyState {
  if (input.evidenceTotal === 0) return "not_generated";
  if (input.evidencePending > 0) return "awaiting_review";
  if (input.evidenceConfirmed === 0) return "no_confirmed";
  return "complete";
}

function resolveReflection(
  input: DailyReflectionJourneyInput,
  evidence: EvidenceJourneyState,
): ReflectionJourneyState {
  if (evidence !== "complete") return "unavailable";
  if (input.reflectionTotal === 0) return "not_generated";
  if (input.hasDirtyReflectionDraft) return "dirty_unsaved";
  if (input.reflectionSuggested > 0) return "awaiting_resolution";
  if (
    input.reflectionAnswered + input.reflectionSkipped >= input.reflectionTotal
  ) {
    return "complete";
  }
  return "awaiting_resolution";
}

function resolvePattern(
  input: DailyReflectionJourneyInput,
  coreReflectionComplete: boolean,
): PatternJourneyState {
  if (input.patternCandidates > 0) return "awaiting_review";
  if (input.patternConfirmed > 0) return "confirmed";
  if (input.patternRejectedInSession) return "rejected";
  if (coreReflectionComplete && input.patternAvailable) return "available";
  return "unavailable";
}

export function resolveDailyReflectionJourney(
  input: DailyReflectionJourneyInput,
): DailyReflectionJourney {
  const evidence = resolveEvidence(input);
  const reflection = resolveReflection(input, evidence);
  const coreReflectionComplete = reflection === "complete";
  const pattern = resolvePattern(input, coreReflectionComplete);

  let activeStage: DailyReflectionStage = "evidence";
  let nextAction: DailyReflectionNextAction = "generate_evidence";

  if (evidence === "not_generated") {
    nextAction = input.contextClarificationInvited
      ? "answer_context"
      : "generate_evidence";
  } else if (evidence === "awaiting_review" || evidence === "no_confirmed") {
    nextAction = "review_evidence";
  } else if (reflection === "not_generated") {
    activeStage = "reflection";
    nextAction = "generate_reflection";
  } else if (reflection === "dirty_unsaved") {
    activeStage = "reflection";
    nextAction = "save_reflection";
  } else if (reflection === "awaiting_resolution") {
    activeStage = "reflection";
    nextAction = "resolve_reflection";
  } else if (coreReflectionComplete) {
    activeStage = "completion";
    nextAction = "review_completion";
  }

  return {
    experienceSaved: input.experienceSaved,
    contextClarificationInvited: input.contextClarificationInvited,
    evidence,
    reflection,
    pattern,
    historicalReflectionAvailable: input.historicalQuestionCount > 0,
    coreReflectionComplete,
    activeStage,
    nextAction,
  };
}

export function stageForNextAction(
  nextAction: DailyReflectionNextAction,
): DailyReflectionStage {
  if (
    nextAction === "answer_context" ||
    nextAction === "generate_evidence" ||
    nextAction === "review_evidence"
  ) {
    return "evidence";
  }
  if (
    nextAction === "generate_reflection" ||
    nextAction === "resolve_reflection" ||
    nextAction === "save_reflection"
  ) {
    return "reflection";
  }
  return "completion";
}

export function resolveDisplayedJourneyStage(
  override: DailyReflectionStageOverride,
  journey: Pick<DailyReflectionJourney, "activeStage" | "coreReflectionComplete">,
): DailyReflectionStage | null {
  if (override === "collapsed") return null;
  return override ?? (journey.coreReflectionComplete ? null : journey.activeStage);
}
