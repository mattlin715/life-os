export type ISODateTime = string;

export type CandidateStatus = "candidate" | "confirmed" | "rejected";
export type ReflectionPromptStatus = "suggested" | "answered" | "skipped";

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
  originalText?: string;
  kind:
    | "observation"
    | "emotion"
    | "decision"
    | "contradiction"
    | "self_description"
    | "other";
  status: CandidateStatus;
  userEditable: boolean;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}

export interface ReflectionPrompt {
  id: string;
  sourceEntryId: string;
  sourceEvidenceIds: string[];
  question: string;
  status: ReflectionPromptStatus;
  response?: string;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}

export interface PatternNote {
  id: string;
  sourceEntryId: string;
  sourceEvidenceIds: string[];
  sourceReflectionPromptIds?: string[];
  text: string;
  status: CandidateStatus;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}
