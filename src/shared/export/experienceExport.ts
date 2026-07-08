import type { ExperienceEntry } from "../../types/domain";
import type {
  ExperienceExportDocument,
  ExperienceExportEntry,
  ExperienceExportFormat,
} from "./types";

const APP_NAME = "Life OS";
const EXPORT_VERSION = "0.1";

export function createExperienceExportDocument(
  entries: ExperienceEntry[],
  exportedAt = new Date().toISOString(),
): ExperienceExportDocument {
  return {
    exportedAt,
    app: APP_NAME,
    version: EXPORT_VERSION,
    entries: entries.map(toExportEntry),
  };
}

export function serializeExperienceExportJson(
  entries: ExperienceEntry[],
  exportedAt?: string,
): string {
  return `${JSON.stringify(createExperienceExportDocument(entries, exportedAt), null, 2)}\n`;
}

export function serializeExperienceExportMarkdown(
  entries: ExperienceEntry[],
  exportedAt = new Date().toISOString(),
): string {
  const lines = [
    "# Life OS Experience Export",
    "",
    `Exported at: ${exportedAt}`,
    "",
  ];

  for (const entry of entries) {
    lines.push(`## Entry - ${entry.createdAt}`);
    lines.push("");
    lines.push(entry.body);
    lines.push("");
    lines.push(`Updated at: ${entry.updatedAt}`);
    lines.push("");
  }

  return `${lines.join("\n").trimEnd()}\n`;
}

export function createExperienceExportFilename(
  format: ExperienceExportFormat,
  exportedAt = new Date().toISOString(),
): string {
  const safeTimestamp = exportedAt.replace(/[:.]/g, "-");
  const extension = format === "json" ? "json" : "md";

  return `life-os-experience-export-${safeTimestamp}.${extension}`;
}

function toExportEntry(entry: ExperienceEntry): ExperienceExportEntry {
  return {
    id: entry.id,
    content: entry.body,
    createdAt: entry.createdAt,
    updatedAt: entry.updatedAt,
  };
}
