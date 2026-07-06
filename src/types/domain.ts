export type ISODateTime = string;

export interface ExperienceEntry {
  id: string;
  body: string;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}

export interface EvidenceCandidate {
  id: string;
  entryId: string;
  text: string;
  kind: "event" | "emotion" | "decision" | "value" | "contradiction" | "self_description";
  accepted: boolean;
}

export interface ReflectionPrompt {
  id: string;
  entryId: string;
  question: string;
}

export interface PatternNote {
  id: string;
  title: string;
  summary: string;
  evidenceIds: string[];
  accepted: boolean;
}
