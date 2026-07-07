export type ISODateTime = string;

export type CandidateStatus = "candidate" | "confirmed" | "rejected";

export interface ExperienceEntry {
  id: string;
  body: string;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
  userEditable: boolean;
}

export interface EvidenceCandidate {
  id: string;
  sourceEntryId: string;
  text: string;
  kind: "event" | "emotion" | "decision" | "value" | "contradiction" | "self_description";
  status: CandidateStatus;
  userEditable: boolean;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}

export interface ReflectionPrompt {
  id: string;
  sourceEntryId: string;
  question: string;
  status: CandidateStatus;
  userEditable: boolean;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}

export interface PatternNote {
  id: string;
  title: string;
  summary: string;
  sourceEntryIds: string[];
  evidenceIds: string[];
  status: CandidateStatus;
  userEditable: boolean;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}
