import type { ReflectionPrompt } from "../../types/domain";

export interface ReflectionPromptSummary {
  total: number;
  suggested: number;
  answered: number;
  skipped: number;
}

export function summarizeReflectionPrompts(
  prompts: ReflectionPrompt[],
): ReflectionPromptSummary {
  return prompts.reduce<ReflectionPromptSummary>(
    (summary, prompt) => {
      summary.total += 1;

      if (prompt.status === "answered") {
        summary.answered += 1;
      } else if (prompt.status === "skipped") {
        summary.skipped += 1;
      } else {
        summary.suggested += 1;
      }

      return summary;
    },
    {
      total: 0,
      suggested: 0,
      answered: 0,
      skipped: 0,
    },
  );
}
