import { invoke } from "@tauri-apps/api/core";
import type { Locale } from "./androidM2CStore";

export const connectedPilot = import.meta.env.VITE_LIFE_OS_ANDROID_PILOT_ONLINE === "1";
export const destination = "Google Gemini · generativelanguage.googleapis.com";

export type ModelPreview = Readonly<{
  sourceId: string;
  sourceRevisionId: string;
  clueId: string;
  clueRevisionId: string;
  text: string;
  locale: Locale;
}>;

export interface ModelPort {
  hasKey(): Promise<boolean>;
  setKey(key: string): Promise<void>;
  clearKey(): Promise<void>;
  ask(request: ModelPreview & { syntheticConfirmed: true }): Promise<{ ticket: string; question: string }>;
  save(ticket: string, answer: string): Promise<void>;
}

export const pilotModel: ModelPort = {
  hasKey: () => invoke("pilot_model_has_key"),
  setKey: (key) => invoke("pilot_model_set_key", { key }),
  clearKey: () => invoke("pilot_model_clear_key"),
  ask: (request) => invoke("pilot_model_ask", { request }),
  save: (ticket, answer) => invoke("pilot_model_save", { ticket, answer }),
};

export const modelErrors = ["no_key", "invalid_key", "invalid_clue", "stale_clue", "busy", "connection", "access", "quota", "no_question", "unavailable", "provider_error", "save_unconfirmed", "invalid_answer"] as const;
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
          clueId: preview.clueId, clueRevisionId: preview.clueRevisionId,
          text: preview.text, locale: preview.locale, syntheticConfirmed: true,
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
