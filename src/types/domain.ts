export type ISODateTime = string;

export type ArtifactOrigin = "user" | "ai" | "local_mock" | "legacy_unknown";
export type AiProviderName = "openai" | "gemini" | "mock" | "legacy_unknown";

/** Records where an artifact came from without turning it into an authority. */
export interface ArtifactProvenance {
  origin: ArtifactOrigin;
  sourceEntryId: string;
  sourceArtifactIds: string[];
  provider?: AiProviderName;
  model?: string | null;
  harnessVersion?: string | null;
  promptVersion?: string | null;
  generatedAt?: ISODateTime | null;
}

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
  provenance?: ArtifactProvenance;
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
  /** Authorship and model metadata for the generated question. */
  promptProvenance: ArtifactProvenance;
  /** Present only for a user-authored answered response. */
  responseProvenance?: ArtifactProvenance;
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
  provenance: ArtifactProvenance;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}

export interface ContextRecoveryTurn {
  id: string;
  sourceEntryId: string;
  question: string;
  response?: string;
  status: "suggested" | "answered" | "skipped";
  locale: "en" | "zh-TW" | "ja";
  /** Authorship of the deterministic/AI question. */
  promptProvenance: ArtifactProvenance;
  /** Present only when the user writes an answer. */
  responseProvenance?: ArtifactProvenance;
  createdAt: ISODateTime;
  updatedAt: ISODateTime;
}
