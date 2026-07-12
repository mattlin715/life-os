import type { ContextRecoveryTurn } from "../../types/domain";

/** A new request never duplicates an already-open optional recovery turn. */
export function addSuggestedRecoveryTurn(
  turns: ContextRecoveryTurn[],
  suggestedTurn: ContextRecoveryTurn,
): ContextRecoveryTurn[] {
  return turns.some((turn) => turn.status === "suggested") ? turns : [...turns, suggestedTurn];
}

export function answerRecoveryTurn(turn: ContextRecoveryTurn, response: string, updatedAt = new Date().toISOString()): ContextRecoveryTurn {
  return { ...turn, response: response.trim(), status: "answered", responseProvenance: { origin: "user", sourceEntryId: turn.sourceEntryId, sourceArtifactIds: [turn.id], generatedAt: updatedAt }, updatedAt };
}
export function skipRecoveryTurnRecord(turn: ContextRecoveryTurn, updatedAt = new Date().toISOString()): ContextRecoveryTurn {
  return { ...turn, response: undefined, responseProvenance: undefined, status: "skipped", updatedAt };
}
