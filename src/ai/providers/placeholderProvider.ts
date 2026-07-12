import { HARNESS_VERSION, PROMPT_VERSION } from "../harness/version";
import type { ContextPacket } from "../harness/contextPacket";
import type { ArtifactProvenance, EvidenceCandidate, PatternNote, ReflectionPrompt } from "../../types/domain";
import type { AIProvider } from "./types";
const id = () => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
const stamp = () => new Date().toISOString();
const isChinese = (text: string) => /[\u3400-\u9fff]/u.test(text);
const isJapanese = (text: string) => /[\u3040-\u30ff]/u.test(text);
const provenance = (packet: ContextPacket, sources: string[], generatedAt: string): ArtifactProvenance => ({ origin: "local_mock", sourceEntryId: packet.currentExperience.id, sourceArtifactIds: sources, provider: "mock", model: null, harnessVersion: HARNESS_VERSION, promptVersion: PROMPT_VERSION, generatedAt });
const candidate = (packet: ContextPacket, text: string): EvidenceCandidate => { const createdAt = stamp(); return { id: id(), sourceEntryId: packet.currentExperience.id, text, originalText: text, kind: "observation", status: "candidate", userEditable: true, provenance: provenance(packet, packet.answeredClarificationTurns.map((turn) => turn.id), createdAt), createdAt, updatedAt: createdAt }; };
export function extractEvidenceCandidates(packet: ContextPacket): EvidenceCandidate[] {
  const body = packet.currentExperience.body.trim(); if (!body) return [];
  const clarification = packet.answeredClarificationTurns[0]?.response?.trim();
  const text = isJapanese(body) ? `記録から直接確認できること：${body}` : isChinese(body) ? `從紀錄中可直接確認：${body}` : `Directly observable from the entry: ${body}`;
  return [candidate(packet, clarification ? `${text} (${clarification})` : text)];
}
export const placeholderProvider: AIProvider = {
  async extractEvidence(packet) { return extractEvidenceCandidates(packet); },
  async generateReflectionPrompts(packet) {
    return packet.confirmedEvidence.slice(0, 3).map((evidence): ReflectionPrompt => { const createdAt = stamp(); const question = isJapanese(packet.currentExperience.body) ? "この記録を読み返すと、何が最も気になりますか？" : isChinese(packet.currentExperience.body) ? "再次閱讀這段紀錄時，什麼最讓你留意？" : "What stands out when you read this evidence again?"; return { id: id(), sourceEntryId: packet.currentExperience.id, sourceEvidenceIds: [evidence.id], question, status: "suggested", promptProvenance: provenance(packet, [evidence.id], createdAt), createdAt, updatedAt: createdAt }; });
  },
  async suggestPatternNotes(packet) {
    const evidence = packet.confirmedEvidence[0]; if (!evidence) return [];
    const createdAt = stamp(); const text = isJapanese(packet.currentExperience.body) ? `検討できる仮説のひとつは、「${evidence.text}」に表れたテーマが別の場面にもあるかもしれない、ということです。` : isChinese(packet.currentExperience.body) ? `一個可以檢視的假設是：「${evidence.text}」呈現的主題，也許也出現在其他時刻。` : `One tentative hypothesis to review is whether the theme in "${evidence.text}" may also appear in other moments.`;
    const reflectionIds = packet.answeredReflectionResponses.map((item) => item.id); const note: PatternNote = { id: id(), sourceEntryId: packet.currentExperience.id, sourceEvidenceIds: packet.confirmedEvidence.map((item) => item.id), sourceReflectionPromptIds: reflectionIds.length ? reflectionIds : undefined, text, status: "candidate", provenance: provenance(packet, [...packet.confirmedEvidence.map((item) => item.id), ...reflectionIds], createdAt), createdAt, updatedAt: createdAt }; return [note];
  },
};
