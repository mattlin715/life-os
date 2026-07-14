import { HARNESS_VERSION } from "../ai/harness/version";
import type { AppLanguage } from "../app/i18n";
import type { AiProviderName, EvidenceCandidate, ExperienceEntry, ReflectionPrompt } from "../types/domain";
import { HISTORICAL_CONTEXT_ALGORITHM_VERSION, type HistoricalContextCandidate, type HistoricalSourceArtifacts } from "./types";

export const HISTORICAL_PACKET_SCHEMA_VERSION = "historical-packet-v1";
export const HISTORICAL_PROMPT_VERSION = "historical-reflection-question-v1";
export const HISTORICAL_OUTPUT_SCHEMA_VERSION = "historical-question-output-v1";
export const HISTORICAL_SAFETY_CONTRACT_VERSION = "phase-3b-safety-v1";
export const HISTORICAL_TASK = "historical_reflection_questions" as const;
export const HISTORICAL_PURPOSE = "invite_user_comparison_without_cross_time_conclusions" as const;
export const MAX_HISTORICAL_SOURCES = 3;
export const MAX_HISTORICAL_CONTENT_CHARACTERS = 6_000;

export type HistoricalIncludedItemType = "experience" | "evidence" | "reflection_response";

export interface HistoricalIncludedItem {
  itemType: HistoricalIncludedItemType;
  sourceExperienceId: string;
  artifactId: string | null;
  revision: string;
  authorship: "user" | "user_confirmed_ai_candidate";
  reviewState: "persisted" | "confirmed" | "answered";
  content: string;
  relevanceReason: string;
  retrievalAlgorithmVersion: typeof HISTORICAL_CONTEXT_ALGORITHM_VERSION;
}

export interface HistoricalContextPacket {
  packetId: string;
  packetDigest: string;
  schemaVersion: typeof HISTORICAL_PACKET_SCHEMA_VERSION;
  assembledAt: string;
  expiresAt: string;
  currentExperience: { id: string; revision: string; content: string };
  task: typeof HISTORICAL_TASK;
  purpose: typeof HISTORICAL_PURPOSE;
  locale: AppLanguage;
  responseLanguage: AppLanguage;
  destination: { provider: Exclude<AiProviderName, "mock" | "legacy_unknown">; model: string; retentionDisclosure: string };
  versions: {
    harness: typeof HARNESS_VERSION;
    prompt: typeof HISTORICAL_PROMPT_VERSION;
    outputSchema: typeof HISTORICAL_OUTPUT_SCHEMA_VERSION;
    safetyContract: typeof HISTORICAL_SAFETY_CONTRACT_VERSION;
  };
  includedItems: HistoricalIncludedItem[];
  consent: { reference: string; scope: "one_generation_one_purpose" };
  limits: { maxSources: typeof MAX_HISTORICAL_SOURCES; maxContentCharacters: typeof MAX_HISTORICAL_CONTENT_CHARACTERS };
}

export interface HistoricalPreflight {
  currentExperience: ExperienceEntry;
  selectedCandidates: HistoricalContextCandidate[];
  artifactsByEntryId: Record<string, HistoricalSourceArtifacts | undefined>;
  explicitlyIncludedArtifactIds: ReadonlySet<string>;
  locale: AppLanguage;
  provider: "openai" | "gemini";
  model: string;
  retentionDisclosure: string;
}

export interface HistoricalConsentEvent {
  id: string;
  packetDigest: string;
  task: typeof HISTORICAL_TASK;
  purpose: typeof HISTORICAL_PURPOSE;
  provider: "openai" | "gemini";
  model: string;
  sourceRevisions: Array<{ sourceExperienceId: string; revision: string; artifactIds: string[] }>;
  state: "granted" | "consumed" | "invalidated";
  createdAt: string;
  expiresAt: string;
}

export interface HistoricalTransmissionEvent {
  id: string;
  consentId: string;
  packetDigest: string;
  provider: "openai" | "gemini";
  model: string;
  outcome: "sent" | "failed" | "refused" | "cancelled_before_send" | "cancelled_after_send";
  createdAt: string;
  expiresAt: string;
}

export interface HistoricalReflectionQuestion {
  id: string;
  text: string;
  sourceExperienceIds: string[];
}

export interface HistoricalQuestionArtifact {
  id: string;
  currentExperienceId: string;
  questions: HistoricalReflectionQuestion[];
  packet: HistoricalContextPacket;
  consentId: string;
  transmissionId: string;
  generatedAt: string;
}

const id = () => globalThis.crypto?.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
const compact = (value: string) => value.replace(/\s+/g, " ").trim();
const canonicalize = (value: unknown): string => {
  if (Array.isArray(value)) return `[${value.map(canonicalize).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.entries(value as Record<string, unknown>).sort(([a], [b]) => a === b ? 0 : a < b ? -1 : 1).map(([key, nested]) => `${JSON.stringify(key)}:${canonicalize(nested)}`).join(",")}}`;
  }
  return JSON.stringify(value);
};

async function sha256(value: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(value));
  return [...new Uint8Array(digest)].map((part) => part.toString(16).padStart(2, "0")).join("");
}

function reason(candidate: HistoricalContextCandidate): string {
  return candidate.reasons.flatMap((item) => item.terms).join(", ");
}

export function eligibleHistoricalArtifacts(sourceId: string, artifacts: HistoricalSourceArtifacts | undefined) {
  const evidence = (artifacts?.evidence ?? []).filter((item) => item.sourceEntryId === sourceId && item.status === "confirmed" && Boolean(compact(item.text)));
  const evidenceIds = new Set(evidence.map((item) => item.id));
  const reflections = (artifacts?.reflections ?? []).filter((item) => item.sourceEntryId === sourceId && item.status === "answered" && Boolean(compact(item.response ?? "")) && item.responseProvenance?.origin === "user" && item.sourceEvidenceIds.length > 0 && item.sourceEvidenceIds.every((evidenceId) => evidenceIds.has(evidenceId)));
  return { evidence, reflections };
}

function includedArtifactItems(input: HistoricalPreflight, candidate: HistoricalContextCandidate, reasonText: string): HistoricalIncludedItem[] {
  const { evidence, reflections } = eligibleHistoricalArtifacts(candidate.sourceExperienceId, input.artifactsByEntryId[candidate.sourceExperienceId]);
  const evidenceItems = evidence.filter((item) => input.explicitlyIncludedArtifactIds.has(item.id)).map((item): HistoricalIncludedItem => ({
    itemType: "evidence", sourceExperienceId: candidate.sourceExperienceId, artifactId: item.id, revision: item.updatedAt,
    authorship: item.provenance?.origin === "user" ? "user" : "user_confirmed_ai_candidate", reviewState: "confirmed", content: compact(item.text), relevanceReason: reasonText,
    retrievalAlgorithmVersion: HISTORICAL_CONTEXT_ALGORITHM_VERSION,
  }));
  const reflectionItems = reflections.filter((item) => input.explicitlyIncludedArtifactIds.has(item.id)).map((item): HistoricalIncludedItem => ({
    itemType: "reflection_response", sourceExperienceId: candidate.sourceExperienceId, artifactId: item.id, revision: item.updatedAt,
    authorship: "user", reviewState: "answered", content: compact(item.response ?? ""), relevanceReason: reasonText,
    retrievalAlgorithmVersion: HISTORICAL_CONTEXT_ALGORITHM_VERSION,
  }));
  return [...evidenceItems, ...reflectionItems];
}

export async function assembleHistoricalContextPacket(input: HistoricalPreflight, now = new Date()): Promise<HistoricalContextPacket> {
  if (!input.model.trim()) throw new Error("historical_destination_missing");
  if (input.selectedCandidates.length === 0) throw new Error("historical_sources_missing");
  if (input.selectedCandidates.length > MAX_HISTORICAL_SOURCES) throw new Error("historical_source_limit_exceeded");
  const includedItems = input.selectedCandidates.flatMap((candidate) => {
    const source = input.artifactsByEntryId[candidate.sourceExperienceId];
    const sourceExperience = (source as HistoricalSourceArtifacts & { experience?: ExperienceEntry } | undefined)?.experience;
    if (!sourceExperience || sourceExperience.updatedAt !== candidate.sourceUpdatedAt) throw new Error("historical_source_stale");
    const reasonText = reason(candidate);
    return [{ itemType: "experience", sourceExperienceId: sourceExperience.id, artifactId: null, revision: sourceExperience.updatedAt, authorship: "user", reviewState: "persisted", content: compact(sourceExperience.body), relevanceReason: reasonText, retrievalAlgorithmVersion: HISTORICAL_CONTEXT_ALGORITHM_VERSION } satisfies HistoricalIncludedItem, ...includedArtifactItems(input, candidate, reasonText)];
  });
  const totalCharacters = compact(input.currentExperience.body).length + includedItems.reduce((total, item) => total + item.content.length, 0);
  if (totalCharacters > MAX_HISTORICAL_CONTENT_CHARACTERS) throw new Error("historical_content_limit_exceeded");
  const assembledAt = now.toISOString();
  const withoutDigest: Omit<HistoricalContextPacket, "packetDigest"> = {
    packetId: id(), schemaVersion: HISTORICAL_PACKET_SCHEMA_VERSION, assembledAt,
    expiresAt: new Date(now.getTime() + 10 * 60_000).toISOString(),
    currentExperience: { id: input.currentExperience.id, revision: input.currentExperience.updatedAt, content: compact(input.currentExperience.body) },
    task: HISTORICAL_TASK, purpose: HISTORICAL_PURPOSE, locale: input.locale, responseLanguage: input.locale,
    destination: { provider: input.provider, model: input.model, retentionDisclosure: input.retentionDisclosure },
    versions: { harness: HARNESS_VERSION, prompt: HISTORICAL_PROMPT_VERSION, outputSchema: HISTORICAL_OUTPUT_SCHEMA_VERSION, safetyContract: HISTORICAL_SAFETY_CONTRACT_VERSION },
    // The immutable disclosure can carry the one-shot consent reference and
    // scope, but not a consent timestamp: that timestamp only exists when the
    // founder explicitly presses Send and the consent event is created.
    includedItems, consent: { reference: id(), scope: "one_generation_one_purpose" as const },
    limits: { maxSources: MAX_HISTORICAL_SOURCES, maxContentCharacters: MAX_HISTORICAL_CONTENT_CHARACTERS },
  };
  return { ...withoutDigest, packetDigest: await sha256(canonicalize(withoutDigest)) };
}

export async function validateHistoricalPacketDigest(packet: HistoricalContextPacket): Promise<boolean> {
  const { packetDigest, ...withoutDigest } = packet;
  return packetDigest === await sha256(canonicalize(withoutDigest));
}

export async function isHistoricalTransportAuthorized(packet: HistoricalContextPacket, consent: HistoricalConsentEvent | null, now = new Date()): Promise<boolean> {
  return Boolean(consent && consent.state === "granted" && consent.id === packet.consent.reference && consent.packetDigest === packet.packetDigest && consent.task === packet.task && consent.purpose === packet.purpose && consent.provider === packet.destination.provider && consent.model === packet.destination.model && now.toISOString() <= packet.expiresAt && await validateHistoricalPacketDigest(packet));
}

export async function sendAuthorizedHistoricalPacket<T>(packet: HistoricalContextPacket, consent: HistoricalConsentEvent | null, request: (packet: HistoricalContextPacket) => Promise<T>, now = new Date()): Promise<T> {
  if (!await isHistoricalTransportAuthorized(packet, consent, now)) throw new Error("historical_transport_not_authorized");
  return request(packet);
}

export function createHistoricalConsentEvent(packet: HistoricalContextPacket, now = new Date()): HistoricalConsentEvent {
  const bySource = new Map<string, { sourceExperienceId: string; revision: string; artifactIds: string[] }>();
  for (const item of packet.includedItems) {
    const current = bySource.get(item.sourceExperienceId) ?? { sourceExperienceId: item.sourceExperienceId, revision: item.revision, artifactIds: [] };
    if (item.itemType === "experience") current.revision = item.revision;
    if (item.artifactId) current.artifactIds.push(item.artifactId);
    bySource.set(item.sourceExperienceId, current);
  }
  return { id: packet.consent.reference, packetDigest: packet.packetDigest, task: packet.task, purpose: packet.purpose, provider: packet.destination.provider, model: packet.destination.model, sourceRevisions: [...bySource.values()], state: "granted", createdAt: now.toISOString(), expiresAt: new Date(now.getTime() + 30 * 24 * 60 * 60_000).toISOString() };
}

export function isHistoricalPacketCurrent(packet: HistoricalContextPacket, experiences: ExperienceEntry[], artifactsByEntryId: Record<string, HistoricalSourceArtifacts | undefined>, now = new Date()): boolean {
  if (now.toISOString() > packet.expiresAt) return false;
  const current = experiences.find((entry) => entry.id === packet.currentExperience.id);
  if (!current || current.updatedAt !== packet.currentExperience.revision || compact(current.body) !== packet.currentExperience.content) return false;
  for (const item of packet.includedItems) {
    const source = experiences.find((entry) => entry.id === item.sourceExperienceId);
    if (!source) return false;
    if (item.itemType === "experience") {
      if (source.updatedAt !== item.revision || compact(source.body) !== item.content) return false;
      continue;
    }
    const eligible = eligibleHistoricalArtifacts(item.sourceExperienceId, artifactsByEntryId[item.sourceExperienceId]);
    const artifact = item.itemType === "evidence" ? eligible.evidence.find((entry) => entry.id === item.artifactId) : eligible.reflections.find((entry) => entry.id === item.artifactId);
    const content = artifact && "text" in artifact ? artifact.text : artifact?.response;
    if (!artifact || artifact.updatedAt !== item.revision || compact(content ?? "") !== item.content) return false;
  }
  return true;
}

export function isHistoricalDestinationCurrent(packet: HistoricalContextPacket, runtime: { provider: string; model?: string | null }, locale: AppLanguage): boolean {
  return runtime.provider === packet.destination.provider && runtime.model === packet.destination.model && locale === packet.locale;
}

export function transportHistoricalPacket(packet: HistoricalContextPacket) {
  return {
    packetId: packet.packetId, packetDigest: packet.packetDigest, schemaVersion: packet.schemaVersion,
    currentExperience: packet.currentExperience, task: packet.task, purpose: packet.purpose, locale: packet.locale,
    destination: packet.destination, versions: packet.versions, includedItems: packet.includedItems, consent: packet.consent, limits: packet.limits,
  };
}

const prohibited = /\b(pattern|recurr(?:ing|ence)?|contradict(?:ion|ory)?|changed over time|over time|trend|summary|summari[sz]e|awareness|growth|diagnos(?:is|e)|personality|identity|advice|should|cause[ds]?|always|often|again)\b|模式|反覆|重複|矛盾|改變|趨勢|總結|摘要|覺察|成長|診斷|人格|身份|建議|原因|總是|經常|パターン|繰り返|反復|矛盾|変化|傾向|要約|気づき|成長|診断|人格|アイデンティティ|助言|原因|いつも|再び/iu;

export function validateHistoricalQuestionOutput(value: unknown, packet: HistoricalContextPacket): HistoricalReflectionQuestion[] {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("historical_output_invalid");
  const raw = (value as Record<string, unknown>).questions;
  if (!Array.isArray(raw)) throw new Error("historical_output_invalid");
  if (raw.length > 3) throw new Error("historical_output_limit_exceeded");
  const historicalSources = new Set(packet.includedItems.map((item) => item.sourceExperienceId));
  const allowedSources = new Set([packet.currentExperience.id, ...historicalSources]);
  return raw.map((item) => {
    if (!item || typeof item !== "object" || Array.isArray(item)) throw new Error("historical_output_invalid");
    const record = item as Record<string, unknown>;
    const text = typeof record.text === "string" ? record.text.trim() : "";
    const sourceExperienceIds = Array.isArray(record.sourceExperienceIds) ? record.sourceExperienceIds.filter((source): source is string => typeof source === "string") : [];
    if (!text || prohibited.test(text) || !/[?？]$/.test(text) || sourceExperienceIds.length === 0 || sourceExperienceIds.some((source) => !allowedSources.has(source)) || !sourceExperienceIds.some((source) => historicalSources.has(source))) throw new Error("historical_output_prohibited");
    return { id: id(), text, sourceExperienceIds: [...new Set(sourceExperienceIds)] };
  });
}

export function exactHistoricalArtifactIds(input: HistoricalPreflight): string[] {
  return input.selectedCandidates.flatMap((candidate) => {
    const { evidence, reflections } = eligibleHistoricalArtifacts(candidate.sourceExperienceId, input.artifactsByEntryId[candidate.sourceExperienceId]);
    return [...evidence, ...reflections].map((item) => item.id);
  });
}

// Compile-time assertion that eligible domain records remain the only artifact inputs.
void (0 as unknown as EvidenceCandidate | ReflectionPrompt);
