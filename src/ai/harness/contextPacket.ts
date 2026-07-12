import type {
  AiProviderName,
  ContextRecoveryTurn,
  EvidenceCandidate,
  ExperienceEntry,
  PatternNote,
  ReflectionPrompt,
} from "../../types/domain";
import { HARNESS_VERSION, PROMPT_VERSION } from "./version";

export type RequestedAiTask = "evidence" | "reflection" | "pattern";
export interface ContextPacket {
  currentExperience: ExperienceEntry;
  answeredClarificationTurns: ContextRecoveryTurn[];
  confirmedEvidence: EvidenceCandidate[];
  answeredReflectionResponses: ReflectionPrompt[];
  locale: "en" | "zh-TW" | "ja";
  harnessVersion: string;
  promptVersion: string;
  requestedTask: RequestedAiTask;
  provider: AiProviderName;
  model: string | null;
}
export type ContextPacketIssueCode =
  | "foreign_recovery_turn"
  | "foreign_evidence"
  | "foreign_reflection"
  | "reflection_missing_confirmed_evidence"
  | "reflection_response_provenance_unknown"
  | "orphaned_source_artifact";
export interface ContextPacketIssue { code: ContextPacketIssueCode; artifactId: string; }
export interface ContextPacketValidationResult { packet: ContextPacket; issues: ContextPacketIssue[]; }

export interface ContextPacketInput {
  currentExperience: ExperienceEntry;
  recoveryTurns?: ContextRecoveryTurn[];
  evidence?: EvidenceCandidate[];
  reflections?: ReflectionPrompt[];
  patterns?: PatternNote[];
  locale: ContextPacket["locale"];
  requestedTask: RequestedAiTask;
  provider: AiProviderName;
  model?: string | null;
}

export function createContextPacket(input: ContextPacketInput): ContextPacketValidationResult {
  const entryId = input.currentExperience.id;
  const issues: ContextPacketIssue[] = [];
  const confirmedEvidence = (input.evidence ?? []).filter((record) => {
    if (record.sourceEntryId !== entryId) { issues.push({ code: "foreign_evidence", artifactId: record.id }); return false; }
    return record.status === "confirmed";
  });
  const confirmedIds = new Set(confirmedEvidence.map((record) => record.id));
  const answeredClarificationTurns = (input.recoveryTurns ?? []).filter((turn) => {
    if (turn.sourceEntryId !== entryId) { issues.push({ code: "foreign_recovery_turn", artifactId: turn.id }); return false; }
    return turn.status === "answered" && Boolean(turn.response?.trim());
  });
  const answeredReflectionResponses = (input.reflections ?? []).filter((prompt) => {
    if (prompt.sourceEntryId !== entryId) { issues.push({ code: "foreign_reflection", artifactId: prompt.id }); return false; }
    if (prompt.status !== "answered" || !prompt.response?.trim()) return false;
    if (!prompt.responseProvenance || prompt.responseProvenance.origin !== "user") {
      issues.push({ code: "reflection_response_provenance_unknown", artifactId: prompt.id });
      return false;
    }
    if (prompt.sourceEvidenceIds.length === 0 || prompt.sourceEvidenceIds.some((id) => !confirmedIds.has(id))) {
      issues.push({ code: "reflection_missing_confirmed_evidence", artifactId: prompt.id });
      return false;
    }
    return true;
  });
  const validArtifactIds = new Set([
    ...confirmedEvidence.map((record) => record.id),
    ...answeredReflectionResponses.map((record) => record.id),
    ...answeredClarificationTurns.map((record) => record.id),
  ]);
  for (const pattern of input.patterns ?? []) {
    if (pattern.sourceEntryId !== entryId) continue;
    for (const id of pattern.provenance.sourceArtifactIds) {
      if (!validArtifactIds.has(id)) issues.push({ code: "orphaned_source_artifact", artifactId: pattern.id });
    }
  }
  return {
    packet: {
      currentExperience: input.currentExperience,
      answeredClarificationTurns,
      confirmedEvidence,
      answeredReflectionResponses,
      locale: input.locale,
      harnessVersion: HARNESS_VERSION,
      promptVersion: PROMPT_VERSION,
      requestedTask: input.requestedTask,
      provider: input.provider,
      model: input.model ?? null,
    },
    issues,
  };
}
