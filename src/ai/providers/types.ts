import type { ContextPacket } from "../harness/contextPacket";
import type { EvidenceCandidate, PatternNote, ReflectionPrompt } from "../../types/domain";
export interface AIProvider {
  extractEvidence(packet: ContextPacket): Promise<EvidenceCandidate[]>;
  generateReflectionPrompts(packet: ContextPacket): Promise<ReflectionPrompt[]>;
  suggestPatternNotes(packet: ContextPacket): Promise<PatternNote[]>;
}
