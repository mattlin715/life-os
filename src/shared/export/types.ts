export interface ExperienceExportEntry {
  id: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

export interface ExperienceExportDocument {
  exportedAt: string;
  app: "Life OS";
  version: "0.1";
  entries: ExperienceExportEntry[];
}

export type ExperienceExportFormat = "json" | "markdown";
