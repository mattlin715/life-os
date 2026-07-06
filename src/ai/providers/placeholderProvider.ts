import type { AIProvider } from "./types";

export const placeholderProvider: AIProvider = {
  async extractEvidence() {
    return [];
  },
  async generateReflectionPrompts() {
    return [];
  },
  async suggestPatternNotes() {
    return [];
  },
};
