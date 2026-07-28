import { HARNESS_VERSION } from "../ai/harness/version";
import {
  HISTORICAL_OUTPUT_SCHEMA_VERSION,
  HISTORICAL_PACKET_SCHEMA_VERSION,
  HISTORICAL_PROMPT_VERSION,
  HISTORICAL_PURPOSE,
  HISTORICAL_SAFETY_CONTRACT_VERSION,
  HISTORICAL_TASK,
  MAX_HISTORICAL_CONTENT_CHARACTERS,
  MAX_HISTORICAL_SOURCES,
  validateHistoricalPacketDigest,
  validateHistoricalQuestionOutput,
  type HistoricalIncludedItem,
  type HistoricalQuestionArtifact,
} from "./governedPacket";
import { HISTORICAL_CONTEXT_ALGORITHM_VERSION } from "./types";

type ProvenanceInvalidReason =
  | "malformed"
  | "unsupported"
  | "incomplete"
  | "contradictory"
  | "digest_mismatch";

export interface HistoricalProvenanceDependency {
  readonly kind: "included_item_to_source" | "question_citation_to_source";
  readonly fromId: string;
  readonly sourceExperienceId: string;
}

export interface HistoricalProvenanceViewModel {
  readonly artifact: {
    readonly id: string;
    readonly generatedAt: string;
    readonly currentExperienceId: string;
  };
  readonly questions: ReadonlyArray<{
    readonly id: string;
    readonly text: string;
    readonly sourceExperienceIds: readonly string[];
  }>;
  readonly packet: {
    readonly id: string;
    readonly digest: string;
    readonly schemaVersion: string;
    readonly assembledAt: string;
    readonly expiresAt: string;
    readonly task: string;
    readonly purpose: string;
    readonly locale: string;
    readonly responseLanguage: string;
    readonly provider: string;
    readonly model: string;
    readonly retentionDisclosure: string;
    readonly versions: ReadonlyArray<{ readonly name: string; readonly value: string }>;
    readonly consentId: string;
    readonly consentScope: string;
    readonly transmissionId: string;
  };
  readonly includedItems: ReadonlyArray<HistoricalIncludedItem & {
    readonly snapshotReference: string;
  }>;
  readonly dependencies: readonly HistoricalProvenanceDependency[];
  readonly outgoingContent: {
    readonly currentExperience: {
      readonly id: string;
      readonly revision: string;
      readonly content: string;
    };
    readonly includedItems: ReadonlyArray<{
      readonly id: string;
      readonly type: string;
      readonly content: string;
    }>;
  };
}

export type HistoricalProvenanceValidation =
  | { readonly status: "valid"; readonly viewModel: HistoricalProvenanceViewModel }
  | { readonly status: "invalid"; readonly reason: ProvenanceInvalidReason };

const locales = new Set(["en", "zh-TW", "ja"]);
const providers = new Set(["openai", "gemini"]);

function record(value: unknown): Record<string, unknown> | null {
  return value !== null && typeof value === "object" && !Array.isArray(value)
    ? value as Record<string, unknown>
    : null;
}

function nonEmptyString(value: unknown): value is string {
  return typeof value === "string" && value.trim().length > 0;
}

function validInstant(value: unknown): value is string {
  return nonEmptyString(value) && Number.isFinite(Date.parse(value));
}

function exactStringArray(value: unknown): value is string[] {
  return Array.isArray(value) && value.every(nonEmptyString);
}

function invalid(reason: ProvenanceInvalidReason): HistoricalProvenanceValidation {
  return { status: "invalid", reason };
}

function includedItem(value: unknown): HistoricalIncludedItem | null {
  const item = record(value);
  if (!item
    || !["experience", "evidence", "reflection_response"].includes(String(item.itemType))
    || !nonEmptyString(item.sourceExperienceId)
    || !nonEmptyString(item.revision)
    || !["user", "user_confirmed_ai_candidate"].includes(String(item.authorship))
    || !["persisted", "confirmed", "answered"].includes(String(item.reviewState))
    || !nonEmptyString(item.content)
    || !nonEmptyString(item.relevanceReason)
    || item.retrievalAlgorithmVersion !== HISTORICAL_CONTEXT_ALGORITHM_VERSION) {
    return null;
  }

  if (item.itemType === "experience") {
    if (item.artifactId !== null || item.authorship !== "user" || item.reviewState !== "persisted") return null;
  } else if (!nonEmptyString(item.artifactId)) {
    return null;
  } else if (item.itemType === "evidence" && item.reviewState !== "confirmed") {
    return null;
  } else if (item.itemType === "reflection_response"
    && (item.authorship !== "user" || item.reviewState !== "answered")) {
    return null;
  }

  return item as unknown as HistoricalIncludedItem;
}

/**
 * Validates only the already-loaded Historical Question and its exact packet
 * snapshot. It deliberately has no storage, provider, consent, or mutation
 * dependency.
 */
export async function inspectHistoricalQuestionProvenance(
  value: unknown,
): Promise<HistoricalProvenanceValidation> {
  const artifact = record(value);
  const packet = record(artifact?.packet);
  const currentExperience = record(packet?.currentExperience);
  const destination = record(packet?.destination);
  const versions = record(packet?.versions);
  const consent = record(packet?.consent);
  const limits = record(packet?.limits);

  if (!artifact || !packet || !currentExperience || !destination || !versions || !consent || !limits) {
    return invalid("malformed");
  }
  if (!nonEmptyString(artifact.id)
    || !nonEmptyString(artifact.currentExperienceId)
    || !nonEmptyString(artifact.consentId)
    || !nonEmptyString(artifact.transmissionId)
    || !validInstant(artifact.generatedAt)
    || !nonEmptyString(packet.packetId)
    || !nonEmptyString(packet.packetDigest)
    || !validInstant(packet.assembledAt)
    || !validInstant(packet.expiresAt)
    || !nonEmptyString(currentExperience.id)
    || !nonEmptyString(currentExperience.revision)
    || !nonEmptyString(currentExperience.content)
    || !nonEmptyString(destination.model)
    || !nonEmptyString(destination.retentionDisclosure)
    || !Array.isArray(packet.includedItems)
    || !Array.isArray(artifact.questions)) {
    return invalid("incomplete");
  }

  if (packet.schemaVersion !== HISTORICAL_PACKET_SCHEMA_VERSION
    || packet.task !== HISTORICAL_TASK
    || packet.purpose !== HISTORICAL_PURPOSE
    || !locales.has(String(packet.locale))
    || packet.responseLanguage !== packet.locale
    || !providers.has(String(destination.provider))
    || versions.harness !== HARNESS_VERSION
    || versions.prompt !== HISTORICAL_PROMPT_VERSION
    || versions.outputSchema !== HISTORICAL_OUTPUT_SCHEMA_VERSION
    || versions.safetyContract !== HISTORICAL_SAFETY_CONTRACT_VERSION
    || consent.scope !== "one_generation_one_purpose"
    || limits.maxSources !== MAX_HISTORICAL_SOURCES
    || limits.maxContentCharacters !== MAX_HISTORICAL_CONTENT_CHARACTERS) {
    return invalid("unsupported");
  }

  if (artifact.currentExperienceId !== currentExperience.id
    || artifact.consentId !== consent.reference
    || Date.parse(packet.expiresAt as string) < Date.parse(packet.assembledAt as string)
    || Date.parse(artifact.generatedAt) < Date.parse(packet.assembledAt as string)) {
    return invalid("contradictory");
  }

  const items = packet.includedItems.map(includedItem);
  if (items.some((item) => item === null)) return invalid("malformed");
  const includedItems = items as HistoricalIncludedItem[];
  const sourceExperiences = new Map<string, HistoricalIncludedItem>();
  const itemIdentities = new Set<string>();

  for (const item of includedItems) {
    if (item.sourceExperienceId === currentExperience.id) return invalid("contradictory");
    const identity = item.itemType === "experience"
      ? `experience:${item.sourceExperienceId}`
      : `${item.itemType}:${item.artifactId}`;
    if (itemIdentities.has(identity)) return invalid("contradictory");
    itemIdentities.add(identity);
    if (item.itemType === "experience") sourceExperiences.set(item.sourceExperienceId, item);
  }

  if (sourceExperiences.size === 0 || sourceExperiences.size > MAX_HISTORICAL_SOURCES) {
    return invalid("contradictory");
  }
  if (includedItems.some((item) => !sourceExperiences.has(item.sourceExperienceId))) {
    return invalid("incomplete");
  }
  const totalCharacters = (currentExperience.content as string).length
    + includedItems.reduce((total, item) => total + item.content.length, 0);
  if (totalCharacters > MAX_HISTORICAL_CONTENT_CHARACTERS) return invalid("contradictory");

  const allowedSources = new Set([currentExperience.id as string, ...sourceExperiences.keys()]);
  const questionIds = new Set<string>();
  const dependencies: HistoricalProvenanceDependency[] = includedItems.map((item) => ({
    kind: "included_item_to_source",
    fromId: item.artifactId ?? item.sourceExperienceId,
    sourceExperienceId: item.sourceExperienceId,
  }));
  const questions: HistoricalProvenanceViewModel["questions"][number][] = [];

  if (artifact.questions.length > 3) return invalid("contradictory");
  for (const valueQuestion of artifact.questions) {
    const question = record(valueQuestion);
    if (!question
      || !nonEmptyString(question.id)
      || !nonEmptyString(question.text)
      || !exactStringArray(question.sourceExperienceIds)
      || question.sourceExperienceIds.length === 0
      || new Set(question.sourceExperienceIds).size !== question.sourceExperienceIds.length
      || question.sourceExperienceIds.some((sourceId) => !allowedSources.has(sourceId))
      || !question.sourceExperienceIds.some((sourceId) => sourceExperiences.has(sourceId))
      || questionIds.has(question.id)) {
      return invalid("contradictory");
    }
    questionIds.add(question.id);
    questions.push({
      id: question.id,
      text: question.text,
      sourceExperienceIds: [...question.sourceExperienceIds],
    });
    dependencies.push(...question.sourceExperienceIds.map((sourceExperienceId) => ({
      kind: "question_citation_to_source" as const,
      fromId: question.id as string,
      sourceExperienceId,
    })));
  }

  try {
    validateHistoricalQuestionOutput(
      { questions: artifact.questions },
      packet as unknown as HistoricalQuestionArtifact["packet"],
    );
    if (!await validateHistoricalPacketDigest(packet as unknown as HistoricalQuestionArtifact["packet"])) {
      return invalid("digest_mismatch");
    }
  } catch {
    return invalid("malformed");
  }

  return {
    status: "valid",
    viewModel: {
      artifact: {
        id: artifact.id,
        generatedAt: artifact.generatedAt,
        currentExperienceId: artifact.currentExperienceId,
      },
      questions,
      packet: {
        id: packet.packetId,
        digest: packet.packetDigest,
        schemaVersion: packet.schemaVersion as string,
        assembledAt: packet.assembledAt as string,
        expiresAt: packet.expiresAt as string,
        task: packet.task as string,
        purpose: packet.purpose as string,
        locale: packet.locale as string,
        responseLanguage: packet.responseLanguage as string,
        provider: destination.provider as string,
        model: destination.model,
        retentionDisclosure: destination.retentionDisclosure,
        versions: [
          { name: "harness", value: versions.harness as string },
          { name: "prompt", value: versions.prompt as string },
          { name: "outputSchema", value: versions.outputSchema as string },
          { name: "safetyContract", value: versions.safetyContract as string },
        ],
        consentId: artifact.consentId,
        consentScope: consent.scope as string,
        transmissionId: artifact.transmissionId,
      },
      includedItems: includedItems.map((item) => ({
        ...item,
        snapshotReference: `${item.sourceExperienceId}@${item.revision}`,
      })),
      dependencies,
      outgoingContent: {
        currentExperience: {
          id: currentExperience.id,
          revision: currentExperience.revision,
          content: currentExperience.content,
        },
        includedItems: includedItems.map((item) => ({
          id: item.artifactId ?? item.sourceExperienceId,
          type: item.itemType,
          content: item.content,
        })),
      },
    },
  };
}
