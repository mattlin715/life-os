import type { ContextRecoveryTurn } from "../../types/domain";
export function answerRecoveryTurn(turn: ContextRecoveryTurn, response: string, updatedAt = new Date().toISOString()): ContextRecoveryTurn {
  return { ...turn, response: response.trim(), status: "answered", responseProvenance: { origin: "user", sourceEntryId: turn.sourceEntryId, sourceArtifactIds: [turn.id], generatedAt: updatedAt }, updatedAt };
}
export function skipRecoveryTurnRecord(turn: ContextRecoveryTurn, updatedAt = new Date().toISOString()): ContextRecoveryTurn {
  return { ...turn, response: undefined, responseProvenance: undefined, status: "skipped", updatedAt };
}
