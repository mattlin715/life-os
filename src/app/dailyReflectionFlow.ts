export type DailyReflectionNextAction =
  | "generate_evidence"
  | "review_evidence"
  | "generate_reflection"
  | "answer_reflection"
  | "reflection_complete";

export interface DailyReflectionProgress {
  readonly evidenceTotal: number;
  readonly evidencePending: number;
  readonly evidenceConfirmed: number;
  readonly reflectionTotal: number;
  readonly reflectionSuggested: number;
  readonly reflectionAnswered: number;
}

export function nextDailyReflectionAction(
  progress: DailyReflectionProgress,
): DailyReflectionNextAction {
  if (progress.evidenceTotal === 0) return "generate_evidence";
  if (progress.evidencePending > 0 || progress.evidenceConfirmed === 0) {
    return "review_evidence";
  }
  if (progress.reflectionTotal === 0) return "generate_reflection";
  if (progress.reflectionSuggested > 0 && progress.reflectionAnswered === 0) {
    return "answer_reflection";
  }
  return "reflection_complete";
}
