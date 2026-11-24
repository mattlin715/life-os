import { invoke } from "@tauri-apps/api/core";
import type { Locale } from "./androidM2CStore";

export const connectedPilot = import.meta.env.VITE_LIFE_OS_ANDROID_PILOT_ONLINE === "1";
export const destination = "Google Gemini · generativelanguage.googleapis.com";

export type ModelPreview = Readonly<{
  sourceId: string;
  sourceRevisionId: string;
  original: string;
  focus?: string;
  locale: Locale;
}>;

export type FeedbackPreview = Readonly<Omit<ModelPreview, "focus"> & {
  reflectionId: string;
  reflectionRevisionId: string;
  question: string;
  answer: string;
}>;
export type ModelFeedback = Readonly<{ observation: string; interpretation: string; uncertainty: string; basisQuote: string }>;
export const modelSourcePrompt = "android-gemini-reflection-v2";
export const validFocus = (original: string, focus: string) => !!focus.trim() && original.includes(focus);
export type KeyStatus = Readonly<{ ready: boolean; remembered: boolean }>;

export interface ModelPort {
  hasKey(): Promise<boolean>;
  keyStatus?(): Promise<KeyStatus>;
  setKey(key: string, remember?: boolean): Promise<void>;
  clearKey(): Promise<void>;
  ask(request: ModelPreview & { syntheticConfirmed: true }): Promise<{ ticket: string; question: string }>;
  feedback(request: FeedbackPreview & { syntheticConfirmed: true }): Promise<ModelFeedback>;
  save(ticket: string, answer: string): Promise<void>;
}

export const pilotModel: ModelPort = {
  hasKey: () => invoke("pilot_model_has_key"),
  keyStatus: () => invoke("pilot_model_key_status"),
  setKey: (key, remember = false) => invoke("pilot_model_set_key", { key, remember }),
  clearKey: () => invoke("pilot_model_clear_key"),
  ask: (request) => invoke("pilot_model_ask", { request }),
  feedback: (request) => invoke("pilot_model_feedback", { request }),
  save: (ticket, answer) => invoke("pilot_model_save", { ticket, answer }),
};

export const modelErrors = ["no_key", "invalid_key", "credential_unavailable", "invalid_clue", "stale_clue", "busy", "connection", "access", "quota", "no_question", "no_feedback", "unavailable", "provider_error", "save_unconfirmed", "invalid_answer"] as const;
export type ModelError = (typeof modelErrors)[number];
export function publicModelError(value: unknown): ModelError {
  return modelErrors.find((code) => value === code) ?? "unavailable";
}

export type TrialState =
  | { step: "idle" }
  | { step: "preview"; preview: ModelPreview }
  | { step: "sending" }
  | { step: "result"; ticket: string; question: string; answer: string; saving?: boolean; error?: ModelError }
  | { step: "error"; error: ModelError };

// The preview is the single source of truth for the request. An epoch and an
// object identity check discard deliveries even if an IPC promise cannot abort.
export function createTrial(model: Pick<ModelPort, "ask" | "save">, publish: (state: TrialState) => void) {
  let state: TrialState = { step: "idle" };
  let epoch = 0, inFlight = false;
  const set = (next: TrialState) => { state = next; publish(next); };
  return {
    preview(input: ModelPreview) {
      if (input.focus !== undefined && !validFocus(input.original, input.focus)) return;
      if (!inFlight && state.step !== "result") set({ step: "preview", preview: Object.freeze({ ...input }) });
    },
    async confirm(synthetic: boolean, current: (preview: ModelPreview) => boolean) {
      if (!synthetic || inFlight || state.step !== "preview") return;
      const preview = state.preview;
      if (!current(preview)) { set({ step: "error", error: "stale_clue" }); return; }
      const operation = ++epoch;
      inFlight = true;
      set({ step: "sending" });
      let delivered = false;
      try {
        const { question, ticket } = await model.ask({
          sourceId: preview.sourceId, sourceRevisionId: preview.sourceRevisionId,
          original: preview.original, ...(preview.focus === undefined ? {} : { focus: preview.focus }),
          locale: preview.locale, syntheticConfirmed: true,
        });
        if (operation === epoch && current(preview)) { set({ step: "result", question, ticket, answer: "" }); delivered = true; }
      } catch (error) {
        if (operation === epoch && current(preview)) { set({ step: "error", error: publicModelError(error) }); delivered = true; }
      } finally {
        inFlight = false;
        if (operation === epoch && !delivered) set({ step: "idle" });
      }
    },
    answer(answer: string) {
      if (state.step === "result" && !state.saving && state.error !== "save_unconfirmed") set({ ...state, answer, error: undefined });
    },
    async save(readBack: (result: { ticket: string; question: string; answer: string }) => Promise<void>) {
      if (state.step !== "result" || state.saving || !state.answer.trim()) return;
      const result = state, operation = epoch;
      set({ ...result, saving: true, error: undefined });
      try {
        await model.save(result.ticket, result.answer);
        if (operation === epoch) await readBack(result);
        if (operation === epoch) set({ step: "idle" });
      } catch (error) {
        if (operation === epoch) set({ ...result, saving: false, error: publicModelError(error) === "unavailable" ? "save_unconfirmed" : publicModelError(error) });
      }
    },
    leave() {
      if (state.step === "preview" || state.step === "sending") {
        ++epoch;
        set({ step: "idle" });
      }
    },
    dispose() { ++epoch; state = { step: "idle" }; },
    reset() { ++epoch; set({ step: "idle" }); },
  };
}

export type FeedbackState =
  | { step: "idle" | "sending" }
  | { step: "preview"; preview: FeedbackPreview }
  | { step: "result"; result: ModelFeedback }
  | { step: "error"; error: ModelError };

export function createFeedback(model: Pick<ModelPort, "feedback">, publish: (state: FeedbackState) => void) {
  let state: FeedbackState = { step: "idle" };
  let epoch = 0, inFlight = false;
  const set = (next: FeedbackState) => { state = next; publish(next); };
  return {
    preview(input: FeedbackPreview) {
      if (inFlight) return;
      const { sourceId, sourceRevisionId, reflectionId, reflectionRevisionId, original, locale, question, answer } = input;
      set({ step: "preview", preview: Object.freeze({ sourceId, sourceRevisionId, reflectionId, reflectionRevisionId, original, locale, question, answer }) });
    },
    async confirm(synthetic: boolean, current: (preview: FeedbackPreview) => boolean) {
      if (!synthetic || inFlight || state.step !== "preview") return;
      const preview = state.preview;
      if (!current(preview)) { set({ step: "error", error: "stale_clue" }); return; }
      const operation = ++epoch;
      inFlight = true;
      set({ step: "sending" });
      try {
        const result = await model.feedback({ ...preview, syntheticConfirmed: true });
        if (operation === epoch) set(current(preview) ? { step: "result", result } : { step: "idle" });
      } catch (error) {
        if (operation === epoch) set(current(preview) ? { step: "error", error: publicModelError(error) } : { step: "idle" });
      } finally { inFlight = false; }
    },
    cancel() { ++epoch; set({ step: "idle" }); },
    dispose() { ++epoch; state = { step: "idle" }; },
  };
}
