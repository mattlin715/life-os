import type { ContextRecoveryTurn } from "../../types/domain";
import { decideContextGate, type ContextGateDecision } from "./gateDecision";

export type PatternAvailabilityReason =
  | "available"
  | "missing_confirmed_evidence"
  | "unsaved_reflection"
  | "insufficient_context"
  | "pending";

export interface PatternAvailability {
  available: boolean;
  reason: PatternAvailabilityReason;
  gate: ContextGateDecision;
}

export interface PatternAvailabilityInput {
  body: string;
  recoveryTurns: ContextRecoveryTurn[];
  confirmedEvidenceCount: number;
  hasDirtyReflectionDraft: boolean;
  isPatternPending: boolean;
}

/**
 * The one eligibility decision for both Pattern rendering and execution.
 * Reflection drafts never enrich source context; only saved recovery answers
 * can change the Context Gate result.
 */
export function decidePatternAvailability({
  body,
  recoveryTurns,
  confirmedEvidenceCount,
  hasDirtyReflectionDraft,
  isPatternPending,
}: PatternAvailabilityInput): PatternAvailability {
  const gate = decideContextGate(body, recoveryTurns);

  if (isPatternPending) return { available: false, reason: "pending", gate };
  if (confirmedEvidenceCount === 0) {
    return { available: false, reason: "missing_confirmed_evidence", gate };
  }
  if (hasDirtyReflectionDraft) {
    return { available: false, reason: "unsaved_reflection", gate };
  }
  if (!gate.allowPattern) {
    return { available: false, reason: "insufficient_context", gate };
  }
  return { available: true, reason: "available", gate };
}
