import type { ContextPacket } from "../harness/contextPacket";
import type { EvidenceCandidate, PatternNote, ReflectionPrompt } from "../../types/domain";
import type { HistoricalContextPacket, HistoricalReflectionQuestion } from "../../historicalContext/governedPacket";
export interface AIProvider {
  extractEvidence(packet: ContextPacket): Promise<EvidenceCandidate[]>;
  generateReflectionPrompts(packet: ContextPacket): Promise<ReflectionPrompt[]>;
  suggestPatternNotes(packet: ContextPacket): Promise<PatternNote[]>;
  generateHistoricalReflectionQuestions(packet: HistoricalContextPacket): Promise<HistoricalReflectionQuestion[]>;
}
