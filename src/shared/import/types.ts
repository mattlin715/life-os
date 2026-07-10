export interface ExperienceImportEntry {
  id: string;
  body: string;
  createdAt: string;
  updatedAt: string;
}

export interface ExperienceImportParseResult {
  entries: ExperienceImportEntry[];
}

export interface ExperienceImportResult {
  importedCount: number;
  skippedCount: number;
}
