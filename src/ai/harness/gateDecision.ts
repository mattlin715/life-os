import type { ContextRecoveryTurn } from "../../types/domain";
import { assessContextSufficiency, type ContextSufficiency } from "./contextSufficiency";

export interface ContextGateDecision {
  sufficiency: ContextSufficiency;
  inviteClarification: boolean;
  allowObservation: boolean;
  allowReflection: boolean;
  allowPattern: boolean;
  limitation: "none" | "observation_only" | "pattern_requires_more_context";
}
export function decideContextGate(body: string, turns: ContextRecoveryTurn[]): ContextGateDecision {
  const answers = turns.filter((turn) => turn.status === "answered" && turn.response?.trim()).map((turn) => turn.response!.trim());
  const sufficiency = assessContextSufficiency(body, answers);
  const hasPriorInvitation = turns.length > 0;
  if (sufficiency === "clarification_useful") return {
    sufficiency,
    inviteClarification: !hasPriorInvitation,
    allowObservation: hasPriorInvitation,
    allowReflection: hasPriorInvitation,
    allowPattern: false,
    limitation: "observation_only",
  };
  if (sufficiency === "sufficient_for_tentative_hypothesis") return { sufficiency, inviteClarification: false, allowObservation: true, allowReflection: true, allowPattern: true, limitation: "none" };
  return { sufficiency, inviteClarification: false, allowObservation: true, allowReflection: true, allowPattern: false, limitation: "pattern_requires_more_context" };
}
