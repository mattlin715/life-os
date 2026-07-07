import type { ExperienceEntry } from "../../types/domain";

export interface CreateExperienceInput {
  body: string;
}

export interface UpdateExperiencePatch {
  body?: string;
}

export interface LocalEvidenceStore {
  createExperience(input: CreateExperienceInput): Promise<ExperienceEntry>;
  listExperiences(): Promise<ExperienceEntry[]>;
  getExperience(id: string): Promise<ExperienceEntry | null>;
  updateExperience(
    id: string,
    patch: UpdateExperiencePatch,
  ): Promise<ExperienceEntry | null>;
  deleteExperience(id: string): Promise<void>;
}
