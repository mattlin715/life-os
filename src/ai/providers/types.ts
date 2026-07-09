import type {
  EvidenceCandidate,
  ExperienceEntry,
  PatternNote,
  ReflectionPrompt,
} from "../../types/domain";

export interface AIProvider {
  extractEvidence(entry: ExperienceEntry): Promise<EvidenceCandidate[]>;
  generateReflectionPrompts(
    entry: ExperienceEntry,
    evidence: EvidenceCandidate[],
  ): Promise<ReflectionPrompt[]>;
  suggestPatternNotes(
    entry: ExperienceEntry,
    confirmedEvidence: EvidenceCandidate[],
    reflectionPrompts: ReflectionPrompt[],
  ): Promise<PatternNote[]>;
}
