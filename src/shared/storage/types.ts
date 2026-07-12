import type { ContextRecoveryTurn, EvidenceCandidate, ExperienceEntry, PatternNote, ReflectionPrompt } from "../../types/domain";
import type { ExperienceImportEntry, ExperienceImportResult } from "../import/types";
import type { GenerationSnapshot } from "../../ai/harness/generationSnapshot";

export interface CreateExperienceInput { body: string; }
export interface UpdateExperiencePatch { body?: string; }

export interface PersistedArtifactBundle {
  evidence: EvidenceCandidate[];
  reflections: ReflectionPrompt[];
  patterns: PatternNote[];
  recoveryTurns: ContextRecoveryTurn[];
}

export interface SaveArtifactsOptions {
  /** Checked with the durable write in SQLite to prevent stale AI output. */
  expectedExperienceUpdatedAt?: string;
  /** Revalidated inside the serialized mutation queue before a generated result is applied. */
  generationSnapshot?: GenerationSnapshot;
}

export interface SaveArtifactsResult {
  status: "committed" | "stale_generation";
  bundle: PersistedArtifactBundle;
  validationIssues: string[];
}

/** All durable artifacts are source-scoped. Rejected evidence/patterns are deliberately excluded. */
export interface LocalEvidenceStore {
  createExperience(input: CreateExperienceInput): Promise<ExperienceEntry>;
  importExperiences(entries: ExperienceImportEntry[]): Promise<ExperienceImportResult>;
  listExperiences(): Promise<ExperienceEntry[]>;
  getExperience(id: string): Promise<ExperienceEntry | null>;
  /** Editing a body invalidates all derived artifacts in the same committed mutation. */
  updateExperience(id: string, patch: UpdateExperiencePatch): Promise<ExperienceEntry | null>;
  /** Deleting an experience removes all source-scoped dependent records atomically. */
  deleteExperience(id: string): Promise<void>;
  listArtifacts(entryId: string): Promise<PersistedArtifactBundle>;
  /** Replaces the complete source-scoped bundle atomically and returns what was committed. */
  saveArtifacts(
    entryId: string,
    bundle: PersistedArtifactBundle,
    options?: SaveArtifactsOptions,
  ): Promise<SaveArtifactsResult>;
}
