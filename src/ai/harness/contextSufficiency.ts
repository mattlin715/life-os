export type ContextSufficiency = "sufficient_for_observation" | "clarification_useful" | "sufficient_for_tentative_hypothesis" | "insufficient_for_pattern";

const marker = /\b(i|we|felt|feel|because|when|after|before|decided|said|work|family|today|yesterday)\b|[\u3002\uff01\uff1f\uff1b]|[\u3040-\u30ff\u3400-\u9fff]/iu;
export function assessContextSufficiency(input: string, clarificationResponses: string[] = []): ContextSufficiency {
  const text = [input, ...clarificationResponses].join(" ").replace(/\s+/g, " ").trim();
  if (!text) return "clarification_useful";
  const length = Array.from(text).length;
  if (length < 18) return "clarification_useful";
  if (clarificationResponses.some((value) => value.trim())) return "sufficient_for_tentative_hypothesis";
  if (length < 70 || !marker.test(text)) return "sufficient_for_observation";
  if (length >= 150) return "sufficient_for_tentative_hypothesis";
  return "insufficient_for_pattern";
}
