import type { ExperienceImportEntry, ExperienceImportParseResult } from "./types";

const APP_NAME = "Life OS";

interface RawExperienceExportDocument {
  app?: unknown;
  version?: unknown;
  entries?: unknown;
}

interface RawExperienceExportEntry {
  id?: unknown;
  content?: unknown;
  createdAt?: unknown;
  updatedAt?: unknown;
}

export function parseExperienceImportJson(
  content: string,
): ExperienceImportParseResult {
  let parsed: unknown;

  try {
    parsed = JSON.parse(content);
  } catch {
    throw new Error("Import file is not valid JSON.");
  }

  if (!isRecord(parsed)) {
    throw new Error("Import file must be a Life OS export object.");
  }

  const document = parsed as RawExperienceExportDocument;

  if (document.app !== APP_NAME) {
    throw new Error('Import file must have app set to "Life OS".');
  }

  if (typeof document.version !== "string" || !document.version) {
    throw new Error("Import file must include a version.");
  }

  if (!Array.isArray(document.entries)) {
    throw new Error("Import file must include an entries array.");
  }

  return {
    entries: document.entries.map(parseEntry),
  };
}

function parseEntry(entry: unknown, index: number): ExperienceImportEntry {
  if (!isRecord(entry)) {
    throw new Error(`Entry ${index + 1} must be an object.`);
  }

  const rawEntry = entry as RawExperienceExportEntry;

  if (typeof rawEntry.id !== "string" || !rawEntry.id) {
    throw new Error(`Entry ${index + 1} must include string id.`);
  }

  if (typeof rawEntry.content !== "string") {
    throw new Error(`Entry ${index + 1} must include string content.`);
  }

  if (typeof rawEntry.createdAt !== "string" || !rawEntry.createdAt) {
    throw new Error(`Entry ${index + 1} must include string createdAt.`);
  }

  if (typeof rawEntry.updatedAt !== "string" || !rawEntry.updatedAt) {
    throw new Error(`Entry ${index + 1} must include string updatedAt.`);
  }

  return {
    id: rawEntry.id,
    body: rawEntry.content,
    createdAt: rawEntry.createdAt,
    updatedAt: rawEntry.updatedAt,
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
