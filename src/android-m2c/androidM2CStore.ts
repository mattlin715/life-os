import { invoke } from "@tauri-apps/api/core";
import type { ExperienceEntry } from "../types/domain";
import type { EvidenceCandidate, ReflectionPrompt } from "../types/domain";

export const isAndroidPilot = import.meta.env.VITE_LIFE_OS_ANDROID_PILOT === "1";
const connected = import.meta.env.VITE_LIFE_OS_ANDROID_PILOT_ONLINE === "1";
export const m2cApplicationId = connected ? "com.lifeos.review.pilot.model" : isAndroidPilot ? "com.lifeos.review.pilot" : "com.lifeos.review.m2c";
export const m2cDatabaseFilename = connected ? "android-model-pilot-disposable-v5.db" : isAndroidPilot ? "android-pilot-disposable-v5.db" : "android-m2c-disposable-v5.db";

export type Locale = "en" | "zh-TW" | "ja";
export type Snapshot = {
  entry: ExperienceEntry;
  revisionId: string;
  revisionNumber: number;
  predecessorRevisionId: string | null;
  authorship: "user";
};
export type LifecycleRequest = Readonly<{
  requestId: string; operation: "update" | "delete"; id: string;
  expectedRevisionId: string; occurredAt: string; body: string | null;
}>;
export type MutationAck = {
  acknowledgement: "committed" | "alreadyCommitted" | "committedNotCurrent";
  requestId: string; sourceId: string;
};
export type DebugPhase = "beforeCommit" | "afterCommitBeforeAck" | "rollbackAfterProjection";
export interface M2CStore {
  status(): Promise<{ state: "ready"; applicationId: typeof m2cApplicationId; schemaVersion: 5; syntheticOnly: true }>;
  create(id: string, text: string): Promise<void>;
  list(): Promise<ExperienceEntry[]>;
  get(id: string): Promise<Snapshot | null>;
  mutate(request: LifecycleRequest, debugPhase?: DebugPhase): Promise<MutationAck>;
  readLocale(): Promise<Locale | null>;
  writeLocale(locale: Locale): Promise<void>;
  artifacts(id: string): Promise<ArtifactSnapshot[]>;
  generateQuestions(request: QuestionBatchRequest, debugPhase?: DebugPhase): Promise<ArtifactAck>;
  artifactMutate(request: ArtifactRequest, debugPhase?: DebugPhase): Promise<ArtifactAck>;
}

export const m2cStore: M2CStore = {
  status: () => invoke("m2c_storage_status"),
  create: (id, text) => invoke("m2c_create_experience", { id, text }),
  list: () => invoke("m2c_list_experiences"),
  get: (id) => invoke("m2c_get_experience", { id }),
  mutate: (request, debugPhase) => invoke("m2c_mutate_experience", { request, debugPhase }),
  readLocale: () => invoke("m2c_get_locale_preference"),
  writeLocale: (locale) => invoke("m2c_set_locale_preference", { locale }),
  artifacts: (id) => invoke("m2c_artifact_snapshot", { id }),
  generateQuestions: (request, debugPhase) => invoke("m2c_generate_questions", { request, debugPhase }),
  artifactMutate: (request, debugPhase) => invoke("m2c_mutate_artifact", { request, debugPhase }),
};

export async function sha256(value: string): Promise<string> {
  const bytes = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(value));
  return Array.from(new Uint8Array(bytes), (v) => v.toString(16).padStart(2, "0")).join("");
}

export function nextTimestamp(previous: string, now = Date.now()): string {
  return new Date(Math.max(now, Date.parse(previous) + 1)).toISOString();
}

export async function lifecycleRequest(snapshot: Snapshot, body: string | null, occurredAt = nextTimestamp(snapshot.entry.updatedAt)): Promise<LifecycleRequest> {
  const operation = body === null ? "delete" : "update";
  const digest = body === null ? "" : await sha256(body);
  const requestId = "m2c_" + await sha256(JSON.stringify([
    "life-os/m2c-request-v1", operation, snapshot.entry.id, snapshot.revisionId, occurredAt, digest,
  ]));
  return Object.freeze({ requestId, operation, id: snapshot.entry.id,
    expectedRevisionId: snapshot.revisionId, occurredAt, body });
}

export function createId(): string { return "m2c_" + globalThis.crypto.randomUUID(); }

export type ArtifactSnapshot = {
  id: string; kind: "evidence" | "reflection"; revisionId: string | null;
  reviewState: string; lifecycleState: string; eligibilityState: string;
  payload: EvidenceCandidate | ReflectionPrompt | null;
};
export type EvidenceRef = Readonly<{ artifactId: string; revisionId: string }>;
export type QuestionBatchRequest = Readonly<{
  requestId: string; sourceId: string; expectedSourceRevisionId: string; expectedReflectionSet: string;
  items: readonly Readonly<{ artifactId: string; evidence: EvidenceRef }>[];
  locale: Locale; occurredAt: string;
}>;

// Order is the native snapshot order, not JS locale/UTF-16 sort. Skip can
// change status without changing the revision ID; bind both.
export function reflectionSet(rows: readonly ArtifactSnapshot[]): string {
  return JSON.stringify(rows.filter((row) => row.kind === "reflection").map((row) => [
    row.id, row.revisionId, row.reviewState, row.lifecycleState, row.eligibilityState,
    (row.payload as ReflectionPrompt | null)?.status ?? null,
  ]));
}

export async function questionBatchRequest(source: Snapshot, rows: readonly ArtifactSnapshot[],
  evidence: readonly EvidenceRef[], locale: Locale, occurredAt: string): Promise<QuestionBatchRequest> {
  const expectedReflectionSet = reflectionSet(rows);
  const items = Object.freeze(evidence.map((edge) => Object.freeze({
    artifactId: createId(), evidence: Object.freeze({ ...edge }),
  })));
  const tuple = ["life-os/m2c-question-batch-v1", source.entry.id, source.revisionId, expectedReflectionSet,
    items.map(({ artifactId, evidence }) => [artifactId, evidence.artifactId, evidence.revisionId]), locale, occurredAt];
  return Object.freeze({ requestId: "m2c_" + await sha256(JSON.stringify(tuple)), sourceId: source.entry.id,
    expectedSourceRevisionId: source.revisionId, expectedReflectionSet, items, locale, occurredAt });
}

export type ArtifactRequest = Readonly<{
  requestId: string; operation: "candidate" | "correct" | "confirm" | "reject" | "question" | "answer" | "correctResponse" | "skip";
  sourceId: string; expectedSourceRevisionId: string; artifactId: string; expectedArtifactRevisionId: string | null;
  evidence: readonly EvidenceRef[]; text: string | null; locale: Locale; occurredAt: string;
}>;
export type ArtifactAck = { acknowledgement: "committed" | "alreadyCommitted" | "committedNotCurrent"; requestId: string };
export async function artifactRequest(source: Snapshot, operation: ArtifactRequest["operation"], locale: Locale,
  target: ArtifactSnapshot | null, text: string | null, evidence: readonly EvidenceRef[] = [], occurredAt = nextTimestamp(source.entry.updatedAt)): Promise<ArtifactRequest> {
  const artifactId = target?.id ?? createId();
  const expectedArtifactRevisionId = target?.revisionId ?? null;
  const frozenRefs = Object.freeze(evidence.map((v) => Object.freeze({ ...v })));
  const tuple = ["life-os/m2c-artifact-request-v1", operation, source.entry.id, source.revisionId, artifactId,
    expectedArtifactRevisionId, frozenRefs.map((v) => [v.artifactId, v.revisionId]), text === null ? null : await sha256(text), locale, occurredAt];
  return Object.freeze({ requestId: "m2c_" + await sha256(JSON.stringify(tuple)), operation, sourceId: source.entry.id,
    expectedSourceRevisionId: source.revisionId, artifactId, expectedArtifactRevisionId, evidence: frozenRefs, text, locale, occurredAt });
}
