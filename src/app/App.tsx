import { useCallback, useEffect, useMemo, useState } from "react";

import { isTauri } from "@tauri-apps/api/core";
import {
  createExperienceExportFilename,
  serializeExperienceExportJson,
  serializeExperienceExportMarkdown,
} from "../shared/export/experienceExport";
import type { ExperienceExportFormat } from "../shared/export/types";
import { parseExperienceImportJson } from "../shared/import/experienceImport";
import {
  combineEvidenceCandidateSummaries,
  summarizeEvidenceCandidates,
} from "../shared/evidence/evidenceSummary";
import { summarizeReflectionPrompts } from "../shared/reflection/reflectionSummary";
import { createLocalEvidenceStore } from "../shared/storage";
import {
  evidenceKindLabel,
  languageOptions,
  statusLabel,
  uiText,
  type AppLanguage,
  type UiCopy,
} from "./i18n";
import { geminiProvider } from "../ai/providers/geminiProvider";
import {
  getAiRuntimeStatus,
  openaiProvider,
  type AiRuntimeStatus,
} from "../ai/providers/openaiProvider";
import { placeholderProvider } from "../ai/providers/placeholderProvider";
import type { AIProvider } from "../ai/providers/types";
import type {
  CandidateStatus,
  EvidenceCandidate,
  ExperienceEntry,
  PatternNote,
  ReflectionPrompt,
} from "../types/domain";

function downloadTextFile(filename: string, content: string, mimeType: string) {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");

  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  link.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 0);
}

interface SelectedTextFile {
  filename: string;
  content: string;
}

interface EvidenceCandidateEditState {
  entryId: string;
  candidateId: string;
  text: string;
}

type PendingAction =
  | "saving"
  | `evidence:${string}`
  | `reflection:${string}`
  | `pattern:${string}`;

function summarizePatternNotes(patternNotes: PatternNote[]) {
  return {
    total: patternNotes.length,
    candidate: patternNotes.filter((pattern) => pattern.status === "candidate")
      .length,
    confirmed: patternNotes.filter((pattern) => pattern.status === "confirmed")
      .length,
    rejected: patternNotes.filter((pattern) => pattern.status === "rejected")
      .length,
  };
}

function chooseProvider(aiRuntime: AiRuntimeStatus | null): AIProvider {
  if (aiRuntime?.provider === "gemini") {
    return geminiProvider;
  }

  if (aiRuntime?.provider === "openai") {
    return openaiProvider;
  }

  return placeholderProvider;
}

type ProviderErrorKind =
  | "missing_config"
  | "invalid_key"
  | "model_unavailable"
  | "quota_or_billing"
  | "rate_limited"
  | "network"
  | "unexpected_response"
  | "unknown";

const LANGUAGE_STORAGE_KEY = "life-os.language";

function readInitialLanguage(): AppLanguage {
  const savedLanguage = window.localStorage.getItem(LANGUAGE_STORAGE_KEY);

  if (
    savedLanguage === "en" ||
    savedLanguage === "zh-TW" ||
    savedLanguage === "ja"
  ) {
    return savedLanguage;
  }

  return "zh-TW";
}

function providerDisplayName(
  aiRuntime: AiRuntimeStatus | null,
  copy: UiCopy,
): string {
  if (aiRuntime?.provider === "gemini") {
    return "Gemini";
  }

  if (aiRuntime?.provider === "openai") {
    return "OpenAI";
  }

  return copy.providerGeneric;
}

function providerErrorText(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function redactProviderError(message: string): string {
  return message
    .replace(/sk-[A-Za-z0-9_-]+/g, "sk-[redacted]")
    .replace(/sk-proj-[A-Za-z0-9_-]+/g, "sk-proj-[redacted]")
    .replace(/AIza[0-9A-Za-z_-]+/g, "AIza[redacted]")
    .replace(/key_[A-Za-z0-9_-]+/g, "key_[redacted]");
}

function classifyProviderError(error: unknown): ProviderErrorKind {
  const message = providerErrorText(error).toLowerCase();

  if (
    message.includes("not configured") ||
    message.includes(" is empty") ||
    message.includes("api key is empty")
  ) {
    return "missing_config";
  }

  if (
    message.includes("401") ||
    message.includes("unauthorized") ||
    message.includes("invalid_api_key") ||
    message.includes("incorrect api key") ||
    message.includes("permission_denied")
  ) {
    return "invalid_key";
  }

  if (
    message.includes("model_not_found") ||
    message.includes("model") &&
      (message.includes("not available") ||
        message.includes("not found") ||
        message.includes("limited preview"))
  ) {
    return "model_unavailable";
  }

  if (
    message.includes("quota") ||
    message.includes("billing") ||
    message.includes("insufficient") ||
    message.includes("payment") ||
    message.includes("credit")
  ) {
    return "quota_or_billing";
  }

  if (
    message.includes("429") ||
    message.includes("rate limit") ||
    message.includes("rate_limit") ||
    message.includes("too many requests")
  ) {
    return "rate_limited";
  }

  if (
    message.includes("request failed") ||
    message.includes("network") ||
    message.includes("timeout") ||
    message.includes("dns") ||
    message.includes("connection")
  ) {
    return "network";
  }

  if (
    message.includes("json parse") ||
    message.includes("did not include") ||
    message.includes("returned no") ||
    message.includes("unexpected format")
  ) {
    return "unexpected_response";
  }

  return "unknown";
}

function providerErrorComfortCopy(
  error: unknown,
  aiRuntime: AiRuntimeStatus | null,
  copy: UiCopy,
): string {
  const providerName = providerDisplayName(aiRuntime, copy);
  const fallbackIntro = copy.fallbackIntro(providerName);

  switch (classifyProviderError(error)) {
    case "missing_config":
      return `${fallbackIntro} ${copy.errorMissingConfig}`;
    case "invalid_key":
      return `${fallbackIntro} ${copy.errorInvalidKey}`;
    case "model_unavailable":
      return `${fallbackIntro} ${copy.errorModelUnavailable}`;
    case "quota_or_billing":
      return `${fallbackIntro} ${copy.errorQuota}`;
    case "rate_limited":
      return `${fallbackIntro} ${copy.errorRateLimited}`;
    case "network":
      return `${fallbackIntro} ${copy.errorNetwork}`;
    case "unexpected_response":
      return `${fallbackIntro} ${copy.errorUnexpected}`;
    case "unknown":
    default:
      return `${fallbackIntro} ${copy.errorUnknown}`;
  }
}

function unavailableProviderMessage(
  error: unknown,
  aiRuntime: AiRuntimeStatus | null,
  copy: UiCopy,
): string {
  console.warn("Life OS AI provider fallback", {
    provider: aiRuntime?.provider ?? "mock",
    model: aiRuntime?.model,
    error: redactProviderError(providerErrorText(error)),
  });

  return providerErrorComfortCopy(error, aiRuntime, copy);
}

function isUnavailableProviderMessage(message: string | null): boolean {
  return (
    message?.includes("local mirror fallback") ||
    message?.includes("本機鏡像 fallback") ||
    message?.includes("ローカルミラー fallback") ||
    false
  );
}

function isRealAiActive(aiRuntime: AiRuntimeStatus | null): boolean {
  return aiRuntime?.provider === "gemini" || aiRuntime?.provider === "openai";
}

type GenerationStage = "evidence" | "reflection" | "pattern";

function successfulGenerationMessage(
  stage: GenerationStage,
  aiRuntime: AiRuntimeStatus | null,
  isFirstRealAiSuccess: boolean,
  copy: UiCopy,
): string {
  if (!isRealAiActive(aiRuntime)) {
    switch (stage) {
      case "evidence":
        return copy.successLocalEvidence;
      case "reflection":
        return copy.successLocalReflection;
      case "pattern":
        return copy.successLocalPattern;
    }
  }

  const providerName = providerDisplayName(aiRuntime, copy);
  const prefix = isFirstRealAiSuccess
    ? copy.firstMirror(providerName)
    : copy.mirror(providerName);

  switch (stage) {
    case "evidence":
      return `${prefix} ${copy.successEvidence}`;
    case "reflection":
      return `${prefix} ${copy.successReflection}`;
    case "pattern":
      return `${prefix} ${copy.successPattern}`;
  }
}

function pendingActionCopy(
  pendingAction: PendingAction | null,
  aiRuntime: AiRuntimeStatus | null,
  copy: UiCopy,
): string {
  const providerName = isRealAiActive(aiRuntime)
    ? providerDisplayName(aiRuntime, copy)
    : "Life OS";

  if (!pendingAction || pendingAction === "saving") {
    return copy.pendingHolding;
  }

  if (pendingAction.startsWith("evidence:")) {
    return copy.pendingEvidence(providerName);
  }

  if (pendingAction.startsWith("reflection:")) {
    return copy.pendingReflection(providerName);
  }

  if (pendingAction.startsWith("pattern:")) {
    return copy.pendingPattern(providerName);
  }

  return copy.pendingFallback;
}

function statusMessageClassName(message: string): string {
  if (isUnavailableProviderMessage(message)) {
    return "export-status export-status-fallback";
  }

  if (
    message.includes("mirror returned gently") ||
    message.includes("Local mirror prepared") ||
    message.includes("鏡像已") ||
    message.includes("ミラー")
  ) {
    return "export-status export-status-success";
  }

  return "export-status";
}

async function saveTextFile(
  filename: string,
  content: string,
  mimeType: string,
  format: ExperienceExportFormat,
  copy: UiCopy,
): Promise<string | null> {
  if (!isTauri()) {
    downloadTextFile(filename, content, mimeType);
    return filename;
  }

  const [{ save }, { writeTextFile }] = await Promise.all([
    import("@tauri-apps/plugin-dialog"),
    import("@tauri-apps/plugin-fs"),
  ]);
  const extension = format === "json" ? "json" : "md";
  const selectedPath = await save({
    title: copy.exportTitle,
    defaultPath: filename,
    filters: [
      {
        name: format === "json" ? "JSON" : "Markdown",
        extensions: [extension],
      },
    ],
  });

  if (!selectedPath) {
    return null;
  }

  await writeTextFile(selectedPath, content);
  return selectedPath;
}

async function readJsonImportFile(copy: UiCopy): Promise<SelectedTextFile | null> {
  if (!isTauri()) {
    return readBrowserTextFile(".json,application/json");
  }

  const [{ open }, { readTextFile }] = await Promise.all([
    import("@tauri-apps/plugin-dialog"),
    import("@tauri-apps/plugin-fs"),
  ]);
  const selectedPath = await open({
    title: copy.importTitle,
    multiple: false,
    filters: [
      {
        name: "JSON",
        extensions: ["json"],
      },
    ],
  });

  if (!selectedPath || Array.isArray(selectedPath)) {
    return null;
  }

  return {
    filename: selectedPath,
    content: await readTextFile(selectedPath),
  };
}

function readBrowserTextFile(accept: string): Promise<SelectedTextFile | null> {
  return new Promise((resolve, reject) => {
    const input = document.createElement("input");

    input.type = "file";
    input.accept = accept;
    input.style.display = "none";

    input.addEventListener("change", () => {
      const file = input.files?.[0];
      input.remove();

      if (!file) {
        resolve(null);
        return;
      }

      file
        .text()
        .then((content) => resolve({ filename: file.name, content }))
        .catch(reject);
    });
    input.addEventListener("cancel", () => {
      input.remove();
      resolve(null);
    });

    document.body.appendChild(input);
    input.click();
  });
}

export function App() {
  const store = useMemo(() => createLocalEvidenceStore(), []);
  const [language, setLanguage] = useState<AppLanguage>(() =>
    readInitialLanguage(),
  );
  const copy = uiText[language];
  const [body, setBody] = useState("");
  const [entries, setEntries] = useState<ExperienceEntry[]>([]);
  const [storageError, setStorageError] = useState<string | null>(null);
  const [portabilityStatus, setPortabilityStatus] = useState<string | null>(
    null,
  );
  const [aiRuntime, setAiRuntime] = useState<AiRuntimeStatus | null>(null);
  const [pendingAction, setPendingAction] = useState<PendingAction | null>(null);
  const [hasRealAiSuccess, setHasRealAiSuccess] = useState(false);
  const [evidenceCandidatesByEntryId, setEvidenceCandidatesByEntryId] =
    useState<Record<string, EvidenceCandidate[]>>({});
  const [reflectionPromptsByEntryId, setReflectionPromptsByEntryId] = useState<
    Record<string, ReflectionPrompt[]>
  >({});
  const [patternNotesByEntryId, setPatternNotesByEntryId] = useState<
    Record<string, PatternNote[]>
  >({});
  const [editingEvidenceCandidate, setEditingEvidenceCandidate] =
    useState<EvidenceCandidateEditState | null>(null);
  const [editingEntryId, setEditingEntryId] = useState<string | null>(null);
  const [editingBody, setEditingBody] = useState("");
  const evidenceSessionSummary = useMemo(
    () =>
      combineEvidenceCandidateSummaries(
        Object.values(evidenceCandidatesByEntryId).map((candidates) =>
          summarizeEvidenceCandidates(candidates),
        ),
      ),
    [evidenceCandidatesByEntryId],
  );

  useEffect(() => {
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, language);
    document.documentElement.lang = language === "zh-TW" ? "zh-Hant" : language;
  }, [language]);

  const refreshEntries = useCallback(async () => {
    try {
      setEntries(await store.listExperiences());
      setStorageError(null);
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
      setPortabilityStatus(null);
    }
  }, [store]);

  const saveExperience = useCallback(async () => {
    const trimmedBody = body.trim();

    if (!trimmedBody) {
      return;
    }

    try {
      setPendingAction("saving");
      await store.createExperience({ body: trimmedBody });
      setBody("");
      await refreshEntries();
      setStorageError(null);
      setPortabilityStatus(null);
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
      setPortabilityStatus(null);
    } finally {
      setPendingAction(null);
    }
  }, [body, refreshEntries, store]);

  const deleteExperience = useCallback(
    async (id: string) => {
      try {
        await store.deleteExperience(id);
        if (editingEntryId === id) {
          setEditingEntryId(null);
          setEditingBody("");
        }
        if (editingEvidenceCandidate?.entryId === id) {
          setEditingEvidenceCandidate(null);
        }
        setEvidenceCandidatesByEntryId((current) => {
          const next = { ...current };
          delete next[id];
          return next;
        });
        setReflectionPromptsByEntryId((current) => {
          const next = { ...current };
          delete next[id];
          return next;
        });
        setPatternNotesByEntryId((current) => {
          const next = { ...current };
          delete next[id];
          return next;
        });
        await refreshEntries();
        setStorageError(null);
        setPortabilityStatus(null);
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      }
    },
    [editingEntryId, editingEvidenceCandidate, refreshEntries, store],
  );

  const beginEdit = useCallback((entry: ExperienceEntry) => {
    setEditingEntryId(entry.id);
    setEditingBody(entry.body);
    setStorageError(null);
    setPortabilityStatus(null);
  }, []);

  const cancelEdit = useCallback(() => {
    setEditingEntryId(null);
    setEditingBody("");
    setStorageError(null);
    setPortabilityStatus(null);
  }, []);

  const saveEdit = useCallback(
    async (id: string) => {
      const trimmedBody = editingBody.trim();

      if (!trimmedBody) {
        return;
      }

      try {
        await store.updateExperience(id, { body: trimmedBody });
        setEditingEntryId(null);
        setEditingBody("");
        await refreshEntries();
        setStorageError(null);
        setPortabilityStatus(null);
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      }
    },
    [editingBody, refreshEntries, store],
  );

  const exportEntries = useCallback(
    async (format: ExperienceExportFormat) => {
      try {
        const currentEntries = await store.listExperiences();
        const exportedAt = new Date().toISOString();
        const content =
          format === "json"
            ? serializeExperienceExportJson(currentEntries, exportedAt)
            : serializeExperienceExportMarkdown(currentEntries, exportedAt);
        const mimeType =
          format === "json"
            ? "application/json;charset=utf-8"
            : "text/markdown;charset=utf-8";
        const savedPath = await saveTextFile(
          createExperienceExportFilename(format, exportedAt),
          content,
          mimeType,
          format,
          copy,
        );

        setStorageError(null);
        setPortabilityStatus(
          savedPath
            ? copy.exportSaved(savedPath)
            : copy.exportCancelled,
        );
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      }
    },
    [copy, store],
  );

  const importEntries = useCallback(async () => {
    try {
      const selectedFile = await readJsonImportFile(copy);

      if (!selectedFile) {
        setStorageError(null);
        setPortabilityStatus(copy.importCancelled);
        return;
      }

      const parsedImport = parseExperienceImportJson(selectedFile.content);
      const result = await store.importExperiences(parsedImport.entries);

      await refreshEntries();
      setStorageError(null);
      setPortabilityStatus(
        copy.importComplete(
          selectedFile.filename,
          result.importedCount,
          result.skippedCount,
        ),
      );
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
      setPortabilityStatus(null);
    }
  }, [copy, refreshEntries, store]);

  const generateEvidenceCandidates = useCallback(async (entry: ExperienceEntry) => {
    try {
      setPendingAction(`evidence:${entry.id}`);
      let candidates: EvidenceCandidate[];
      let usedLocalFallback = false;

      try {
        candidates = await chooseProvider(aiRuntime).extractEvidence(entry);
      } catch (providerError) {
        usedLocalFallback = true;
        candidates = await placeholderProvider.extractEvidence(entry);
        setPortabilityStatus(
          unavailableProviderMessage(providerError, aiRuntime, copy),
        );
      }

      setEvidenceCandidatesByEntryId((current) => ({
        ...current,
        [entry.id]: candidates,
      }));
      setReflectionPromptsByEntryId((current) => {
        const next = { ...current };
        delete next[entry.id];
        return next;
      });
      setPatternNotesByEntryId((current) => {
        const next = { ...current };
        delete next[entry.id];
        return next;
      });
      if (editingEvidenceCandidate?.entryId === entry.id) {
        setEditingEvidenceCandidate(null);
      }
      setStorageError(null);
      if (!usedLocalFallback && isRealAiActive(aiRuntime)) {
        const isFirstRealAiSuccess = !hasRealAiSuccess;
        setHasRealAiSuccess(true);
        setPortabilityStatus(
          successfulGenerationMessage(
            "evidence",
            aiRuntime,
            isFirstRealAiSuccess,
            copy,
          ),
        );
      } else {
        setPortabilityStatus((current) =>
          isUnavailableProviderMessage(current)
            ? current
            : successfulGenerationMessage("evidence", aiRuntime, false, copy),
        );
      }
    } catch (error) {
      setStorageError(error instanceof Error ? error.message : String(error));
      setPortabilityStatus(null);
    } finally {
      setPendingAction(null);
    }
  }, [aiRuntime, copy, editingEvidenceCandidate, hasRealAiSuccess]);

  const beginEvidenceCandidateEdit = useCallback(
    (entryId: string, candidate: EvidenceCandidate) => {
      if (candidate.status !== "candidate") {
        return;
      }

      setEditingEvidenceCandidate({
        entryId,
        candidateId: candidate.id,
        text: candidate.text,
      });
      setStorageError(null);
      setPortabilityStatus(null);
    },
    [],
  );

  const cancelEvidenceCandidateEdit = useCallback(() => {
    setEditingEvidenceCandidate(null);
    setStorageError(null);
    setPortabilityStatus(null);
  }, []);

  const saveEvidenceCandidateEdit = useCallback(() => {
    if (!editingEvidenceCandidate) {
      return;
    }

    const trimmedText = editingEvidenceCandidate.text.trim();

    if (!trimmedText) {
      return;
    }

    setEvidenceCandidatesByEntryId((current) => ({
      ...current,
      [editingEvidenceCandidate.entryId]:
        current[editingEvidenceCandidate.entryId]?.map((candidate) =>
          candidate.id === editingEvidenceCandidate.candidateId &&
          candidate.status === "candidate"
            ? {
                ...candidate,
                text: trimmedText,
                updatedAt: new Date().toISOString(),
              }
            : candidate,
        ) ?? [],
    }));
    setEditingEvidenceCandidate(null);
    setStorageError(null);
    setPortabilityStatus(copy.evidenceEditedStatus);
  }, [copy, editingEvidenceCandidate]);

  const updateEvidenceCandidateStatus = useCallback(
    (
      entryId: string,
      candidateId: string,
      status: Extract<CandidateStatus, "confirmed" | "rejected">,
    ) => {
      setEvidenceCandidatesByEntryId((current) => ({
        ...current,
        [entryId]:
          current[entryId]?.map((candidate) =>
            candidate.id === candidateId
              ? {
                  ...candidate,
                  status,
                  updatedAt: new Date().toISOString(),
                }
              : candidate,
          ) ?? [],
      }));
      if (
        editingEvidenceCandidate?.entryId === entryId &&
        editingEvidenceCandidate.candidateId === candidateId
      ) {
        setEditingEvidenceCandidate(null);
      }
      setPatternNotesByEntryId((current) => {
        const next = { ...current };
        delete next[entryId];
        return next;
      });
      setStorageError(null);
      setPortabilityStatus(copy.evidenceReviewUpdated);
    },
    [copy, editingEvidenceCandidate],
  );

  const generateReflectionPrompts = useCallback(
    async (entry: ExperienceEntry) => {
      const confirmedEvidence =
        evidenceCandidatesByEntryId[entry.id]?.filter(
          (candidate) => candidate.status === "confirmed",
        ) ?? [];

      if (confirmedEvidence.length === 0) {
        setStorageError(null);
        setPortabilityStatus(copy.reflectionNeedEvidence);
        return;
      }

      try {
        setPendingAction(`reflection:${entry.id}`);
        let prompts: ReflectionPrompt[];
        let usedLocalFallback = false;

        try {
          prompts = await chooseProvider(aiRuntime).generateReflectionPrompts(
            entry,
            confirmedEvidence,
          );
        } catch (providerError) {
          usedLocalFallback = true;
          prompts = await placeholderProvider.generateReflectionPrompts(
            entry,
            confirmedEvidence,
          );
          setPortabilityStatus(
            unavailableProviderMessage(providerError, aiRuntime, copy),
          );
        }

        setReflectionPromptsByEntryId((current) => ({
          ...current,
          [entry.id]: prompts,
        }));
        setPatternNotesByEntryId((current) => {
          const next = { ...current };
          delete next[entry.id];
          return next;
        });
        setStorageError(null);
        if (!usedLocalFallback && isRealAiActive(aiRuntime)) {
          const isFirstRealAiSuccess = !hasRealAiSuccess;
          setHasRealAiSuccess(true);
          setPortabilityStatus(
            successfulGenerationMessage(
              "reflection",
              aiRuntime,
              isFirstRealAiSuccess,
              copy,
            ),
          );
        } else {
          setPortabilityStatus((current) =>
            isUnavailableProviderMessage(current)
              ? current
              : successfulGenerationMessage("reflection", aiRuntime, false, copy),
          );
        }
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      } finally {
        setPendingAction(null);
      }
    },
    [aiRuntime, copy, evidenceCandidatesByEntryId, hasRealAiSuccess],
  );

  const updateReflectionPromptResponse = useCallback(
    (entryId: string, promptId: string, response: string) => {
      setReflectionPromptsByEntryId((current) => ({
        ...current,
        [entryId]:
          current[entryId]?.map((prompt) =>
            prompt.id === promptId && prompt.status !== "skipped"
              ? {
                  ...prompt,
                  response,
                  updatedAt: new Date().toISOString(),
                }
              : prompt,
          ) ?? [],
      }));
    },
    [],
  );

  const saveReflectionPromptAnswer = useCallback(
    (entryId: string, promptId: string) => {
      setReflectionPromptsByEntryId((current) => ({
        ...current,
        [entryId]:
          current[entryId]?.map((prompt) => {
            if (prompt.id !== promptId || prompt.status === "skipped") {
              return prompt;
            }

            const response = prompt.response?.trim();

            if (!response) {
              return prompt;
            }

            return {
              ...prompt,
              response,
              status: "answered",
              updatedAt: new Date().toISOString(),
            };
          }) ?? [],
      }));
      setPatternNotesByEntryId((current) => {
        const next = { ...current };
        delete next[entryId];
        return next;
      });
      setStorageError(null);
      setPortabilityStatus(copy.reflectionSaved);
    },
    [copy],
  );

  const skipReflectionPrompt = useCallback(
    (entryId: string, promptId: string) => {
      setReflectionPromptsByEntryId((current) => ({
        ...current,
        [entryId]:
          current[entryId]?.map((prompt) => {
            if (prompt.id !== promptId || prompt.status !== "suggested") {
              return prompt;
            }

            return {
              ...prompt,
              response: undefined,
              status: "skipped",
              updatedAt: new Date().toISOString(),
            };
          }) ?? [],
      }));
      setPatternNotesByEntryId((current) => {
        const next = { ...current };
        delete next[entryId];
        return next;
      });
      setStorageError(null);
      setPortabilityStatus(copy.reflectionSkipped);
    },
    [copy],
  );

  const generatePatternNotes = useCallback(
    async (entry: ExperienceEntry) => {
      const confirmedEvidence =
        evidenceCandidatesByEntryId[entry.id]?.filter(
          (candidate) => candidate.status === "confirmed",
        ) ?? [];
      const reflectionPrompts = reflectionPromptsByEntryId[entry.id] ?? [];

      if (confirmedEvidence.length === 0) {
        setStorageError(null);
        setPortabilityStatus(copy.patternNeedEvidence);
        return;
      }

      try {
        setPendingAction(`pattern:${entry.id}`);
        let patternNotes: PatternNote[];
        let usedLocalFallback = false;

        try {
          patternNotes = await chooseProvider(aiRuntime).suggestPatternNotes(
            entry,
            confirmedEvidence,
            reflectionPrompts,
          );
        } catch (providerError) {
          usedLocalFallback = true;
          patternNotes = await placeholderProvider.suggestPatternNotes(
            entry,
            confirmedEvidence,
            reflectionPrompts,
          );
          setPortabilityStatus(
            unavailableProviderMessage(providerError, aiRuntime, copy),
          );
        }

        setPatternNotesByEntryId((current) => ({
          ...current,
          [entry.id]: patternNotes,
        }));
        setStorageError(null);
        if (!usedLocalFallback && isRealAiActive(aiRuntime)) {
          const isFirstRealAiSuccess = !hasRealAiSuccess;
          setHasRealAiSuccess(true);
          setPortabilityStatus(
            successfulGenerationMessage(
              "pattern",
              aiRuntime,
              isFirstRealAiSuccess,
              copy,
            ),
          );
        } else {
          setPortabilityStatus((current) =>
            isUnavailableProviderMessage(current)
              ? current
              : successfulGenerationMessage("pattern", aiRuntime, false, copy),
          );
        }
      } catch (error) {
        setStorageError(error instanceof Error ? error.message : String(error));
        setPortabilityStatus(null);
      } finally {
        setPendingAction(null);
      }
    },
    [aiRuntime, copy, evidenceCandidatesByEntryId, reflectionPromptsByEntryId, hasRealAiSuccess],
  );

  const updatePatternNoteStatus = useCallback(
    (
      entryId: string,
      patternNoteId: string,
      status: Extract<CandidateStatus, "confirmed" | "rejected">,
    ) => {
      setPatternNotesByEntryId((current) => ({
        ...current,
        [entryId]:
          current[entryId]?.map((patternNote) =>
            patternNote.id === patternNoteId
              ? {
                  ...patternNote,
                  status,
                  updatedAt: new Date().toISOString(),
                }
              : patternNote,
          ) ?? [],
      }));
      setStorageError(null);
      setPortabilityStatus(copy.patternUpdated);
    },
    [copy],
  );

  useEffect(() => {
    void refreshEntries();
  }, [refreshEntries]);

  useEffect(() => {
    let cancelled = false;

    getAiRuntimeStatus()
      .then((status) => {
        if (!cancelled) {
          setAiRuntime(status);
        }
      })
      .catch((error) => {
        if (!cancelled) {
          setAiRuntime({
            provider: "mock",
            reason: error instanceof Error ? error.message : String(error),
          });
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  const aiProviderLabel =
    aiRuntime?.provider === "gemini"
      ? copy.providerActive("Gemini", aiRuntime.model)
      : aiRuntime?.provider === "openai"
        ? copy.providerActive("OpenAI", aiRuntime.model)
        : copy.localFallback;
  const aiProviderDetail =
    aiRuntime?.provider === "gemini"
      ? copy.aiAvailable("Gemini")
      : aiRuntime?.provider === "openai"
        ? copy.aiAvailable("OpenAI")
        : copy.aiFallbackDetail;
  const isSaving = pendingAction === "saving";

  return (
    <main className="app-shell">
      <div className="ambient ambient-one" />
      <div className="ambient ambient-two" />
      <section className="product-frame" aria-label={copy.appAria}>
        <header className="top-bar">
          <div>
            <p className="eyebrow">{copy.alpha}</p>
            <p className="top-bar-title">{copy.motto}</p>
          </div>
          <div className="top-bar-controls">
            <label className="language-switcher">
              <span>{copy.languageLabel}</span>
              <select
                value={language}
                onChange={(event) =>
                  setLanguage(event.target.value as AppLanguage)
                }
              >
                {languageOptions.map((option) => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}
              </select>
            </label>
            <div className="ai-status" title={aiRuntime?.reason ?? aiProviderDetail}>
              <span className={`status-dot status-dot-${aiRuntime?.provider ?? "mock"}`} />
              <span>{aiProviderLabel}</span>
            </div>
          </div>
        </header>

        <section className="welcome-card">
          <div className="welcome-copy">
            <p className="soft-label">{copy.welcome}</p>
            <h1>{copy.hero}</h1>
            <p className="welcome-subtitle">{copy.subtitle}</p>
          </div>
          <div className="reflection-composer">
            <textarea
              aria-label={copy.experienceAria}
              placeholder={copy.experiencePlaceholder}
              value={body}
              onChange={(event) => setBody(event.target.value)}
            />
            <div className="composer-footer">
              <p>{aiProviderDetail}</p>
              <button
                type="button"
                className="primary-button"
                disabled={!body.trim() || isSaving}
                onClick={saveExperience}
              >
                {isSaving ? copy.saving : copy.saveMoment}
              </button>
            </div>
          </div>
        </section>

        <section className="utility-row" aria-label={copy.importExportAria}>
          <button
            type="button"
            className="ghost-button"
            disabled={entries.length === 0}
            onClick={() => exportEntries("json")}
          >
            {copy.exportJson}
          </button>
          <button
            type="button"
            className="ghost-button"
            disabled={entries.length === 0}
            onClick={() => exportEntries("markdown")}
          >
            {copy.exportMarkdown}
          </button>
          <button type="button" className="ghost-button" onClick={importEntries}>
            {copy.importJson}
          </button>
        </section>

        {storageError ? (
          <p className="storage-error" role="alert">
            {copy.storageErrorPrefix} {storageError}
          </p>
        ) : null}
        {portabilityStatus ? (
          <p className={statusMessageClassName(portabilityStatus)} aria-live="polite">
            {portabilityStatus}
          </p>
        ) : null}
        {pendingAction && pendingAction !== "saving" ? (
          <div className="loading-card" role="status" aria-live="polite">
            <span className="spinner" />
            <span>{pendingActionCopy(pendingAction, aiRuntime, copy)}</span>
          </div>
        ) : null}

        <section className="session-summary" aria-label={copy.currentSession}>
          <div>
            <p className="summary-kicker">{copy.currentSession}</p>
            <p className="summary-line">
              {copy.summary(
                entries.length,
                evidenceSessionSummary.total,
                evidenceSessionSummary.confirmed,
                evidenceSessionSummary.rejected,
                evidenceSessionSummary.pending,
              )}
            </p>
          </div>
          <p>{copy.summaryNote}</p>
        </section>

        {entries.length === 0 ? (
          <section className="empty-state" aria-label={copy.emptyAria}>
            <div className="empty-orb">○</div>
            <h2>{copy.emptyTitle}</h2>
            <p>{copy.emptyBody}</p>
          </section>
        ) : (
          <section className="entry-list" aria-label={copy.savedExperiencesAria}>
            {entries.map((entry) => {
              const evidenceCandidates =
                evidenceCandidatesByEntryId[entry.id] ?? [];
              const evidenceSummary =
                summarizeEvidenceCandidates(evidenceCandidates);
              const confirmedEvidenceCandidates = evidenceCandidates.filter(
                (candidate) => candidate.status === "confirmed",
              );
              const reflectionPrompts =
                reflectionPromptsByEntryId[entry.id] ?? [];
              const reflectionSummary =
                summarizeReflectionPrompts(reflectionPrompts);
              const patternNotes = patternNotesByEntryId[entry.id] ?? [];
              const patternSummary = summarizePatternNotes(patternNotes);
              const isEvidencePending = pendingAction === `evidence:${entry.id}`;
              const isReflectionPending =
                pendingAction === `reflection:${entry.id}`;
              const isPatternPending = pendingAction === `pattern:${entry.id}`;

              return (
                <article className="entry" key={entry.id}>
                  <div className="entry-meta">
                    <div>
                      <p className="entry-kicker">{copy.moment}</p>
                      <time dateTime={entry.createdAt}>
                        {new Date(entry.createdAt).toLocaleString()}
                      </time>
                    </div>
                    {entry.updatedAt !== entry.createdAt ? (
                      <time dateTime={entry.updatedAt}>
                        {copy.updated} {new Date(entry.updatedAt).toLocaleString()}
                      </time>
                    ) : null}
                  </div>

                  {editingEntryId === entry.id ? (
                    <div className="edit-block">
                      <textarea
                        aria-label={copy.editExperienceAria}
                        className="edit-textarea"
                        value={editingBody}
                        onChange={(event) => setEditingBody(event.target.value)}
                      />
                      <div className="entry-actions">
                        <button
                          type="button"
                          className="primary-button compact"
                          disabled={!editingBody.trim()}
                          onClick={() => saveEdit(entry.id)}
                        >
                          {copy.saveEdit}
                        </button>
                        <button
                          type="button"
                          className="ghost-button compact"
                          onClick={cancelEdit}
                        >
                          {copy.cancel}
                        </button>
                      </div>
                    </div>
                  ) : (
                    <>
                      <p className="entry-body">{entry.body}</p>
                      <div className="entry-actions">
                        <button
                          type="button"
                          className="ghost-button compact"
                          onClick={() => beginEdit(entry)}
                        >
                          {copy.edit}
                        </button>
                        <button
                          type="button"
                          className="danger-button compact"
                          onClick={() => deleteExperience(entry.id)}
                        >
                          {copy.delete}
                        </button>
                      </div>
                    </>
                  )}

                  <section className="review-stack">
                    <section className="review-card evidence-review">
                      <div className="review-card-header">
                        <div>
                          <p className="review-step">{copy.evidenceStep}</p>
                          <h2>{copy.evidenceTitle}</h2>
                        </div>
                        <button
                          type="button"
                          className="secondary-button"
                          disabled={isEvidencePending}
                          onClick={() => generateEvidenceCandidates(entry)}
                        >
                          {isEvidencePending
                            ? copy.findingEvidence
                            : copy.generateEvidence}
                        </button>
                      </div>
                      <p className="next-step">
                        {isEvidencePending
                          ? copy.evidenceNextPending
                          : evidenceCandidates.length === 0
                            ? copy.evidenceNextEmpty
                            : evidenceSummary.pending > 0
                              ? copy.evidenceNextReview
                              : confirmedEvidenceCandidates.length > 0
                                ? copy.evidenceNextReady
                                : copy.evidenceNextNone}
                      </p>

                      {evidenceCandidates.length > 0 ? (
                        <>
                          <p className="review-summary">
                            {copy.evidenceSummary(
                              evidenceSummary.total,
                              evidenceSummary.confirmed,
                              evidenceSummary.rejected,
                              evidenceSummary.pending,
                            )}
                          </p>
                          <p className="evidence-note">
                            {copy.evidenceNote}
                          </p>
                          <div className="candidate-list">
                            {evidenceCandidates.map((candidate) => {
                              const isEditing =
                                editingEvidenceCandidate?.entryId === entry.id &&
                                editingEvidenceCandidate.candidateId ===
                                  candidate.id;
                              const isReviewed =
                                candidate.status !== "candidate";
                              const hasEditedText =
                                Boolean(candidate.originalText) &&
                                candidate.originalText !== candidate.text;

                              return (
                                <article
                                  className={`candidate candidate-${candidate.status}`}
                                  key={candidate.id}
                                >
                                  <div className="candidate-meta">
                                    <span>
                                      {evidenceKindLabel(candidate.kind, language)}
                                    </span>
                                    <span>
                                      {statusLabel(candidate.status, language)}
                                    </span>
                                  </div>
                                  {isEditing ? (
                                    <>
                                      <textarea
                                        aria-label={copy.editEvidenceAria}
                                        className="candidate-edit-textarea"
                                        value={editingEvidenceCandidate.text}
                                        onChange={(event) =>
                                          setEditingEvidenceCandidate({
                                            ...editingEvidenceCandidate,
                                            text: event.target.value,
                                          })
                                        }
                                      />
                                      <div className="entry-actions">
                                        <button
                                          type="button"
                                          className="primary-button compact"
                                          disabled={
                                            !editingEvidenceCandidate.text.trim()
                                          }
                                          onClick={saveEvidenceCandidateEdit}
                                        >
                                          {copy.save}
                                        </button>
                                        <button
                                          type="button"
                                          className="ghost-button compact"
                                          onClick={cancelEvidenceCandidateEdit}
                                        >
                                          {copy.cancel}
                                        </button>
                                      </div>
                                    </>
                                  ) : (
                                    <>
                                      <p>{candidate.text}</p>
                                      {hasEditedText ? (
                                        <p className="candidate-edited-note">
                                          {copy.editedFromAi}
                                        </p>
                                      ) : null}
                                      {isReviewed ? (
                                        <p className="reviewed-note">
                                          {candidate.status === "confirmed"
                                            ? copy.keptEvidence
                                            : copy.setAside}
                                        </p>
                                      ) : null}
                                      {!isReviewed ? (
                                        <div className="entry-actions review-actions" aria-label={copy.reviewEvidenceAria}>
                                          <button
                                            type="button"
                                            className="ghost-button compact"
                                            onClick={() =>
                                              beginEvidenceCandidateEdit(
                                                entry.id,
                                                candidate,
                                              )
                                            }
                                          >
                                            {copy.edit}
                                          </button>
                                          <button
                                            type="button"
                                            className="primary-button compact"
                                            onClick={() =>
                                              updateEvidenceCandidateStatus(
                                                entry.id,
                                                candidate.id,
                                                "confirmed",
                                              )
                                            }
                                          >
                                            {copy.confirm}
                                          </button>
                                          <button
                                            type="button"
                                            className="ghost-button compact"
                                            onClick={() =>
                                              updateEvidenceCandidateStatus(
                                                entry.id,
                                                candidate.id,
                                                "rejected",
                                              )
                                            }
                                          >
                                            {copy.reject}
                                          </button>
                                        </div>
                                      ) : null}
                                    </>
                                  )}
                                </article>
                              );
                            })}
                          </div>
                        </>
                      ) : (
                        <p className="session-note">
                          {copy.evidenceEmpty}
                        </p>
                      )}
                    </section>

                    <section
                      className="review-card reflection-review"
                      aria-label={copy.reflectionAria}
                    >
                      <div className="review-card-header">
                        <div>
                          <p className="review-step">{copy.reflectionStep}</p>
                          <h2>{copy.reflectionTitle}</h2>
                        </div>
                        <button
                          type="button"
                          className="secondary-button"
                          disabled={
                            confirmedEvidenceCandidates.length === 0 ||
                            isReflectionPending
                          }
                          onClick={() => generateReflectionPrompts(entry)}
                        >
                          {isReflectionPending
                            ? copy.shapingQuestions
                            : copy.generateReflection}
                        </button>
                      </div>
                      <p className="reflection-boundary">
                        {copy.reflectionBoundary}
                      </p>
                      <p className="next-step">
                        {confirmedEvidenceCandidates.length === 0
                          ? copy.reflectionNextNeedEvidence
                          : isReflectionPending
                            ? copy.reflectionNextPending
                            : reflectionPrompts.length === 0
                              ? copy.reflectionNextEmpty
                              : reflectionSummary.suggested > 0
                                ? copy.reflectionNextReview
                                : reflectionSummary.answered > 0
                                  ? copy.reflectionNextReady
                                  : copy.reflectionNextSkipped}
                      </p>
                      {confirmedEvidenceCandidates.length === 0 ? (
                        <p className="session-note">
                          {copy.reflectionNeedEvidence}
                        </p>
                      ) : null}
                      {reflectionPrompts.length > 0 ? (
                        <>
                          <p className="review-summary">
                            {copy.reflectionSummary(
                              reflectionSummary.suggested,
                              reflectionSummary.answered,
                              reflectionSummary.skipped,
                            )}
                          </p>
                          <div className="reflection-list">
                            {reflectionPrompts.map((prompt) => {
                              const response = prompt.response ?? "";
                              const isAnswered = prompt.status === "answered";
                              const isSkipped = prompt.status === "skipped";

                              return (
                                <article
                                  className={`reflection-prompt reflection-prompt-${prompt.status}`}
                                  key={prompt.id}
                                >
                                  <div className="candidate-meta">
                                    <span>{statusLabel(prompt.status, language)}</span>
                                    <span>
                                      {copy.evidenceCount(
                                        prompt.sourceEvidenceIds.length,
                                      )}
                                    </span>
                                  </div>
                                  <p>{prompt.question}</p>
                                  <textarea
                                    aria-label={copy.answerReflectionAria}
                                    className="reflection-response-textarea"
                                    disabled={isSkipped}
                                    placeholder={copy.reflectionPlaceholder}
                                    value={response}
                                    onChange={(event) =>
                                      updateReflectionPromptResponse(
                                        entry.id,
                                        prompt.id,
                                        event.target.value,
                                      )
                                    }
                                  />
                                  {isAnswered || isSkipped ? (
                                    <p className="reviewed-note">
                                      {isAnswered
                                        ? copy.answeredSession
                                        : copy.skippedNoJudgment}
                                    </p>
                                  ) : null}
                                  <div className="entry-actions review-actions" aria-label={copy.reviewReflectionAria}>
                                    <button
                                      type="button"
                                      className="primary-button compact"
                                      disabled={!response.trim() || isSkipped}
                                      onClick={() =>
                                        saveReflectionPromptAnswer(
                                          entry.id,
                                          prompt.id,
                                        )
                                      }
                                    >
                                      {copy.saveAnswer}
                                    </button>
                                    <button
                                      type="button"
                                      className="ghost-button compact"
                                      disabled={isAnswered || isSkipped}
                                      onClick={() =>
                                        skipReflectionPrompt(entry.id, prompt.id)
                                      }
                                    >
                                      {copy.skip}
                                    </button>
                                  </div>
                                </article>
                              );
                            })}
                          </div>
                        </>
                      ) : null}
                    </section>

                    <section
                      className="review-card pattern-review"
                      aria-label={copy.patternAria}
                    >
                      <div className="review-card-header">
                        <div>
                          <p className="review-step">{copy.patternStep}</p>
                          <h2>{copy.patternTitle}</h2>
                        </div>
                        <button
                          type="button"
                          className="secondary-button"
                          disabled={
                            confirmedEvidenceCandidates.length === 0 ||
                            isPatternPending
                          }
                          onClick={() => generatePatternNotes(entry)}
                        >
                          {isPatternPending
                            ? copy.formingPattern
                            : copy.generatePattern}
                        </button>
                      </div>
                      <p className="pattern-boundary">
                        {copy.patternBoundary}
                      </p>
                      <p className="next-step">
                        {confirmedEvidenceCandidates.length === 0
                          ? copy.patternNextNeedEvidence
                          : isPatternPending
                            ? copy.patternNextPending
                            : patternNotes.length === 0
                              ? copy.patternNextEmpty
                              : patternSummary.candidate > 0
                                ? copy.patternNextReview
                                : patternSummary.confirmed > 0
                                  ? copy.patternNextComplete
                                  : copy.patternNextRejected}
                      </p>
                      {confirmedEvidenceCandidates.length === 0 ? (
                        <p className="session-note">
                          {copy.patternNeedEvidence}
                        </p>
                      ) : null}
                      {patternNotes.length > 0 ? (
                        <>
                          <p className="review-summary">
                            {copy.patternSummary(
                              patternSummary.candidate,
                              patternSummary.confirmed,
                              patternSummary.rejected,
                            )}
                          </p>
                          <div className="pattern-list">
                            {patternNotes.map((patternNote) => {
                              const isReviewed =
                                patternNote.status !== "candidate";

                              return (
                                <article
                                  className={`pattern-note pattern-note-${patternNote.status}`}
                                  key={patternNote.id}
                                >
                                  <div className="candidate-meta">
                                    <span>
                                      {statusLabel(patternNote.status, language)}
                                    </span>
                                    <span>
                                      {copy.evidenceCount(
                                        patternNote.sourceEvidenceIds.length,
                                      )}
                                    </span>
                                  </div>
                                  <p>{patternNote.text}</p>
                                  {isReviewed ? (
                                    <p className="reviewed-note">
                                      {patternNote.status === "confirmed"
                                        ? copy.keptPattern
                                        : copy.setAside}
                                    </p>
                                  ) : null}
                                  {!isReviewed ? (
                                    <div className="entry-actions review-actions" aria-label={copy.reviewPatternAria}>
                                      <button
                                        type="button"
                                        className="primary-button compact"
                                        onClick={() =>
                                          updatePatternNoteStatus(
                                            entry.id,
                                            patternNote.id,
                                            "confirmed",
                                          )
                                        }
                                      >
                                        {copy.confirm}
                                      </button>
                                      <button
                                        type="button"
                                        className="ghost-button compact"
                                        onClick={() =>
                                          updatePatternNoteStatus(
                                            entry.id,
                                            patternNote.id,
                                            "rejected",
                                          )
                                        }
                                      >
                                        {copy.reject}
                                      </button>
                                    </div>
                                  ) : null}
                                </article>
                              );
                            })}
                          </div>
                        </>
                      ) : null}
                    </section>
                  </section>
                </article>
              );
            })}
          </section>
        )}
      </section>
    </main>
  );
}

