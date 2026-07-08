import type { ExperienceEntry } from "../../types/domain";
import type {
  ExperienceImportEntry,
  ExperienceImportResult,
} from "../import/types";

export interface CreateExperienceInput {
  body: string;
}

export interface UpdateExperiencePatch {
  body?: string;
}

export interface LocalEvidenceStore {
  createExperience(input: CreateExperienceInput): Promise<ExperienceEntry>;
  importExperiences(
    entries: ExperienceImportEntry[],
  ): Promise<ExperienceImportResult>;
  listExperiences(): Promise<ExperienceEntry[]>;
  getExperience(id: string): Promise<ExperienceEntry | null>;
  updateExperience(
    id: string,
    patch: UpdateExperiencePatch,
  ): Promise<ExperienceEntry | null>;
  deleteExperience(id: string): Promise<void>;
}
