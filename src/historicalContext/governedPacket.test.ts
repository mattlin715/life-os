import { describe, expect, it } from "vitest";
import type { AppLanguage } from "../app/i18n";
import type { EvidenceCandidate, ExperienceEntry, ReflectionPrompt } from "../types/domain";
import type { HistoricalContextCandidate, HistoricalSourceArtifacts } from "./types";
import {
  MAX_HISTORICAL_CONTENT_CHARACTERS,
  assembleHistoricalContextPacket,
  createHistoricalConsentEvent,
  eligibleHistoricalArtifacts,
  isHistoricalPacketCurrent,
  isHistoricalDestinationCurrent,
  isHistoricalTransportAuthorized,
  sendAuthorizedHistoricalPacket,
  transportHistoricalPacket,
  validateHistoricalPacketDigest,
  validateHistoricalQuestionOutput,
} from "./governedPacket";

const at = (day: number) => `2026-07-${String(day).padStart(2, "0")}T00:00:00.000Z`;
const experience = (id: string, body: string, day: number): ExperienceEntry => ({ id, body, createdAt: at(day), updatedAt: at(day), userEditable: true });
const current = experience("current", "A project conversation stayed with me.", 14);
const source = experience("source", "An earlier project conversation felt unfinished.", 10);
const provenance = (origin: "ai" | "user") => ({ origin, sourceEntryId: source.id, sourceArtifactIds: [], provider: origin === "ai" ? "openai" as const : undefined, model: origin === "ai" ? "model" : null, generatedAt: at(10) });
const evidence: EvidenceCandidate = { id: "evidence", sourceEntryId: source.id, text: "The conversation ended early.", kind: "observation", status: "confirmed", userEditable: true, provenance: provenance("ai"), createdAt: at(10), updatedAt: at(10) };
const reflection: ReflectionPrompt = { id: "reflection", sourceEntryId: source.id, sourceEvidenceIds: [evidence.id], question: "What mattered?", response: "I wanted more time to explain.", status: "answered", promptProvenance: provenance("ai"), responseProvenance: provenance("user"), createdAt: at(10), updatedAt: at(10) };
const candidate = (entry = source): HistoricalContextCandidate => ({ sourceExperienceId: entry.id, sourceCreatedAt: entry.createdAt, sourceUpdatedAt: entry.updatedAt, sourceExcerpt: entry.body, reasons: [{ kind: "shared_visible_terms", terms: ["project", "conversation"] }], confirmedEvidenceIds: [evidence.id], answeredReflectionIds: [reflection.id], ranking: { algorithmVersion: "local-lexical-v1", score: 10, matchedTermCount: 2 } });
const artifacts = (entry = source, overrides: Partial<HistoricalSourceArtifacts> = {}): HistoricalSourceArtifacts => ({ experience: entry, evidence: [evidence], reflections: [reflection], ...overrides });
const input = (options: { locale?: AppLanguage; selected?: HistoricalContextCandidate[]; included?: string[]; sourceArtifacts?: HistoricalSourceArtifacts; provider?: "openai" | "gemini"; model?: string } = {}) => ({ currentExperience: current, selectedCandidates: options.selected ?? [candidate()], artifactsByEntryId: { source: options.sourceArtifacts ?? artifacts() }, explicitlyIncludedArtifactIds: new Set(options.included ?? []), locale: options.locale ?? "en", provider: options.provider ?? "openai", model: options.model ?? "model", retentionDisclosure: "disclosed" } as const);

describe("Phase 3B governed historical packet", () => {
  it("does not authorize selection, panel opening, or silence without exact consent", async () => {
    const packet = await assembleHistoricalContextPacket(input());
    expect(await isHistoricalTransportAuthorized(packet, null)).toBe(false);
    expect(await isHistoricalTransportAuthorized(packet, { ...createHistoricalConsentEvent(packet), state: "invalidated" })).toBe(false);
    let calls = 0;
    await expect(sendAuthorizedHistoricalPacket(packet, null, async () => { calls += 1; })).rejects.toThrow("historical_transport_not_authorized");
    expect(calls).toBe(0);
  });

  it("binds one consent to exact content, purpose, destination, versions, and digest", async () => {
    const packet = await assembleHistoricalContextPacket(input({ included: [evidence.id, reflection.id] }));
    const consentTime = new Date("2026-07-14T12:00:00.000Z");
    const consent = createHistoricalConsentEvent(packet, consentTime);
    expect(consent.createdAt).toBe(consentTime.toISOString());
    expect(await validateHistoricalPacketDigest(packet)).toBe(true);
    expect(await isHistoricalTransportAuthorized(packet, consent)).toBe(true);
    expect(await isHistoricalTransportAuthorized({ ...packet, destination: { ...packet.destination, model: "changed" } }, consent)).toBe(false);
    expect(await isHistoricalTransportAuthorized({ ...packet, purpose: "changed-purpose" } as unknown as typeof packet, consent)).toBe(false);
    expect(await isHistoricalTransportAuthorized({ ...packet, task: "changed-task" } as unknown as typeof packet, consent)).toBe(false);
    expect(await isHistoricalTransportAuthorized({ ...packet, versions: { ...packet.versions, prompt: "changed" } } as unknown as typeof packet, consent)).toBe(false);
    expect(await isHistoricalTransportAuthorized({ ...packet, currentExperience: { ...packet.currentExperience, content: "changed" } }, consent)).toBe(false);
    expect(isHistoricalDestinationCurrent(packet, { provider: "gemini", model: "model" }, "en")).toBe(false);
    expect(isHistoricalDestinationCurrent(packet, { provider: "openai", model: "changed" }, "en")).toBe(false);
    expect(isHistoricalDestinationCurrent(packet, { provider: "openai", model: "model" }, "ja")).toBe(false);
  });

  it("canonicalizes the immutable disclosure independently of object insertion order", async () => {
    const packet = await assembleHistoricalContextPacket(input({ included: [evidence.id, reflection.id] }));
    const reordered = Object.fromEntries(Object.entries(packet).reverse()) as unknown as typeof packet;
    expect(await validateHistoricalPacketDigest(reordered)).toBe(true);
    expect(packet.consent).not.toHaveProperty("consentedAt");
  });

  it("includes exact IDs, revisions, bounded text, relevance, destination, consent, and versions without the candidate collection", async () => {
    const packet = await assembleHistoricalContextPacket(input({ included: [evidence.id, reflection.id] }));
    expect(packet.includedItems.map((item) => [item.itemType, item.artifactId])).toEqual([["experience", null], ["evidence", evidence.id], ["reflection_response", reflection.id]]);
    expect(packet.includedItems.every((item) => item.relevanceReason === "project, conversation")).toBe(true);
    expect(packet).toMatchObject({ currentExperience: { id: current.id, revision: current.updatedAt, content: current.body }, destination: { provider: "openai", model: "model" }, consent: { scope: "one_generation_one_purpose" }, versions: { harness: "harness-v1" } });
    expect(JSON.stringify(transportHistoricalPacket(packet))).not.toContain("sourceExcerpt");
  });

  it("excludes drafts, rejected or unreviewed evidence, and non-user/orphaned reflection responses", () => {
    const rejected = { ...evidence, id: "rejected", status: "rejected" as const };
    const candidateEvidence = { ...evidence, id: "candidate", status: "candidate" as const };
    const aiAnswer = { ...reflection, id: "ai-answer", responseProvenance: provenance("ai") };
    const orphan = { ...reflection, id: "orphan", sourceEvidenceIds: ["missing"] };
    const eligible = eligibleHistoricalArtifacts(source.id, artifacts(source, { evidence: [evidence, rejected, candidateEvidence], reflections: [reflection, aiAnswer, orphan] }));
    expect(eligible.evidence.map((item) => item.id)).toEqual([evidence.id]);
    expect(eligible.reflections.map((item) => item.id)).toEqual([reflection.id]);
  });

  it("invalidates before send or persistence when a source or artifact changes, is deleted, rejected, or orphaned", async () => {
    const packet = await assembleHistoricalContextPacket(input({ included: [evidence.id, reflection.id] }));
    expect(isHistoricalPacketCurrent(packet, [current, source], { source: artifacts() })).toBe(true);
    expect(isHistoricalPacketCurrent(packet, [current, { ...source, body: "edited", updatedAt: at(11) }], { source: artifacts() })).toBe(false);
    expect(isHistoricalPacketCurrent(packet, [current, source], { source: artifacts(source, { evidence: [{ ...evidence, status: "rejected" }], reflections: [reflection] }) })).toBe(false);
    expect(isHistoricalPacketCurrent(packet, [current], {})).toBe(false);
  });

  it("fails closed for malformed, stale, missing-destination, whole-history, and size-limit inputs", async () => {
    await expect(assembleHistoricalContextPacket(input({ sourceArtifacts: artifacts({ ...source, updatedAt: at(11) }) }))).rejects.toThrow("historical_source_stale");
    await expect(assembleHistoricalContextPacket(input({ model: "" }))).rejects.toThrow("historical_destination_missing");
    const many = [1, 2, 3, 4].map((day) => experience(`source-${day}`, "project conversation", day));
    await expect(assembleHistoricalContextPacket({ ...input(), selectedCandidates: many.map(candidate), artifactsByEntryId: Object.fromEntries(many.map((entry) => [entry.id, artifacts(entry, { evidence: [], reflections: [] })])) })).rejects.toThrow("historical_source_limit_exceeded");
    const huge = experience("source", "x".repeat(MAX_HISTORICAL_CONTENT_CHARACTERS), 10);
    await expect(assembleHistoricalContextPacket(input({ selected: [candidate(huge)], sourceArtifacts: artifacts(huge, { evidence: [], reflections: [] }) }))).rejects.toThrow("historical_content_limit_exceeded");
  });

  it.each(["en", "zh-TW", "ja"] as const)("keeps the same disclosure packet semantics in %s", async (locale) => {
    const packet = await assembleHistoricalContextPacket(input({ locale }));
    expect(packet.locale).toBe(locale);
    expect(packet.responseLanguage).toBe(locale);
    expect(packet.includedItems.map((item) => item.sourceExperienceId)).toEqual([source.id]);
    expect(packet.task).toBe("historical_reflection_questions");
  });

  it("accepts source-citing neutral questions and an honest no-question result", async () => {
    const packet = await assembleHistoricalContextPacket(input());
    expect(validateHistoricalQuestionOutput({ questions: [] }, packet)).toEqual([]);
    expect(validateHistoricalQuestionOutput({ questions: [{ text: "What feels important when you read these two moments?", sourceExperienceIds: [source.id] }] }, packet)).toHaveLength(1);
    expect(validateHistoricalQuestionOutput({ questions: [{ text: "What feels important when you read these two moments?", sourceExperienceIds: [current.id, source.id] }] }, packet)).toHaveLength(1);
    expect(() => validateHistoricalQuestionOutput({ questions: [{ text: "What feels important in this moment?", sourceExperienceIds: [current.id] }] }, packet)).toThrow("historical_output_prohibited");
  });

  it.each([
    "Why does this pattern keep recurring?",
    "What diagnosis explains these moments?",
    "你為什麼反覆出現這個模式？",
    "このパターンを繰り返す原因は何ですか？",
  ])("rejects Phase 4 conclusions or sensitive inference disguised as a question: %s", async (text) => {
    const packet = await assembleHistoricalContextPacket(input());
    expect(() => validateHistoricalQuestionOutput({ questions: [{ text, sourceExperienceIds: [source.id] }] }, packet)).toThrow("historical_output_prohibited");
  });

  it("rejects uncited, foreign-source, statement, or over-limit output", async () => {
    const packet = await assembleHistoricalContextPacket(input());
    expect(() => validateHistoricalQuestionOutput({ questions: [{ text: "What matters?", sourceExperienceIds: [] }] }, packet)).toThrow();
    expect(() => validateHistoricalQuestionOutput({ questions: [{ text: "What matters?", sourceExperienceIds: ["foreign"] }] }, packet)).toThrow();
    expect(() => validateHistoricalQuestionOutput({ questions: [{ text: "This matters.", sourceExperienceIds: [source.id] }] }, packet)).toThrow();
    expect(() => validateHistoricalQuestionOutput({ questions: Array.from({ length: 4 }, () => ({ text: "What matters?", sourceExperienceIds: [source.id] })) }, packet)).toThrow("historical_output_limit_exceeded");
  });
});
