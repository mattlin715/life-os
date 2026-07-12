import { sharedBehaviorProtocol } from "../harness/behaviorProtocol";
import type { ContextPacket } from "../harness/contextPacket";
import type { ArtifactProvenance, EvidenceCandidate, PatternNote, ReflectionPrompt } from "../../types/domain";
import type { AIProvider } from "./types";

type RequestJson = (instructions: string, input: unknown) => Promise<Record<string, unknown>>;
const kinds: EvidenceCandidate["kind"][] = ["observation", "emotion", "decision", "contradiction", "self_description", "other"];
const id = () => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
const stamp = () => new Date().toISOString();
const record = (value: unknown): Record<string, unknown> => {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("AI response was not a JSON object.");
  return value as Record<string, unknown>;
};
const provenance = (packet: ContextPacket, sourceArtifactIds: string[], generatedAt: string): ArtifactProvenance => ({
  origin: "ai", sourceEntryId: packet.currentExperience.id, sourceArtifactIds,
  provider: packet.provider, model: packet.model, harnessVersion: packet.harnessVersion,
  promptVersion: packet.promptVersion, generatedAt,
});
const rules = [sharedBehaviorProtocol, "Match the locale in the Context Packet.", "Treat clarification answers as user-authored context, never as part of the Experience body.", "Return strict JSON only."].join("\n");
const transportPacket = (packet: ContextPacket) => ({
  ...packet,
  currentExperience: { id: packet.currentExperience.id, body: packet.currentExperience.body },
  answeredClarificationTurns: packet.answeredClarificationTurns.map(({ id, question, response }) => ({ id, question, response })),
  confirmedEvidence: packet.confirmedEvidence.map(({ id, kind, text }) => ({ id, kind, text })),
  answeredReflectionResponses: packet.answeredReflectionResponses.map(({ id, question, response, sourceEvidenceIds }) => ({ id, question, response, sourceEvidenceIds, responseAuthor: "user" })),
});

export function createJsonProvider(requestJson: RequestJson): AIProvider {
  return {
    async extractEvidence(packet) {
      const data = await requestJson([rules, "Extract 2 to 4 direct, reviewable observations. If context depth is low, do not infer causes, traits, or recurring patterns.", 'Return {"candidates":[{"kind":"observation|emotion|decision|contradiction|self_description|other","text":"..."}]}'].join("\n"), transportPacket(packet));
      const candidates = Array.isArray(data.candidates) ? data.candidates : [];
      const result = candidates.map((item): EvidenceCandidate | null => {
        const value = record(item); const text = typeof value.text === "string" ? value.text.trim() : ""; if (!text) return null;
        const createdAt = stamp(); const kind = typeof value.kind === "string" && kinds.includes(value.kind as EvidenceCandidate["kind"]) ? value.kind as EvidenceCandidate["kind"] : "other";
        return { id: id(), sourceEntryId: packet.currentExperience.id, text, originalText: text, kind, status: "candidate", userEditable: true, provenance: provenance(packet, packet.answeredClarificationTurns.map((turn) => turn.id), createdAt), createdAt, updatedAt: createdAt };
      }).filter((item): item is EvidenceCandidate => Boolean(item));
      if (!result.length) throw new Error("AI provider returned no evidence candidates.");
      return result;
    },
    async generateReflectionPrompts(packet) {
      const data = await requestJson([rules, "Generate 2 to 3 calm questions based only on confirmed evidence and answered clarification.", 'Return {"questions":["..."]}'].join("\n"), transportPacket(packet));
      const questions = Array.isArray(data.questions) ? data.questions.filter((item): item is string => typeof item === "string" && Boolean(item.trim())).slice(0, 3) : [];
      if (!questions.length) throw new Error("AI provider returned no reflection questions.");
      return questions.map((question, index): ReflectionPrompt => {
        const evidence = packet.confirmedEvidence[index % packet.confirmedEvidence.length]; const createdAt = stamp(); const sourceEvidenceIds = evidence ? [evidence.id] : [];
        return { id: id(), sourceEntryId: packet.currentExperience.id, sourceEvidenceIds, question: question.trim(), status: "suggested", promptProvenance: provenance(packet, sourceEvidenceIds, createdAt), createdAt, updatedAt: createdAt };
      });
    },
    async suggestPatternNotes(packet) {
      const data = await requestJson([rules, "Propose exactly one tentative pattern hypothesis. It must stay uncertain and use only the validated packet.", 'Return {"text":"..."}'].join("\n"), transportPacket(packet));
      const text = typeof data.text === "string" ? data.text.trim() : ""; if (!text) throw new Error("AI provider returned no pattern candidate.");
      const createdAt = stamp(); const sourceEvidenceIds = packet.confirmedEvidence.map((item) => item.id); const sourceReflectionPromptIds = packet.answeredReflectionResponses.map((item) => item.id);
      return [{ id: id(), sourceEntryId: packet.currentExperience.id, sourceEvidenceIds, sourceReflectionPromptIds: sourceReflectionPromptIds.length ? sourceReflectionPromptIds : undefined, text, status: "candidate", provenance: provenance(packet, [...sourceEvidenceIds, ...sourceReflectionPromptIds, ...packet.answeredClarificationTurns.map((turn) => turn.id)], createdAt), createdAt, updatedAt: createdAt }];
    },
  };
}
