import { invoke } from "@tauri-apps/api/core";
import type { ExperienceEntry } from "../types/domain";
import type { EvidenceCandidate, ReflectionPrompt } from "../types/domain";

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
  status(): Promise<{ state: "ready"; applicationId: "com.lifeos.review.m2c"; schemaVersion: 5; syntheticOnly: true }>;
  create(id: string, text: string): Promise<void>;
  list(): Promise<ExperienceEntry[]>;
  get(id: string): Promise<Snapshot | null>;
  mutate(request: LifecycleRequest, debugPhase?: DebugPhase): Promise<MutationAck>;
  readLocale(): Promise<Locale | null>;
  writeLocale(locale: Locale): Promise<void>;
  artifacts(id: string): Promise<ArtifactSnapshot[]>;
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
export type ArtifactRequest = Readonly<{
  requestId: string; operation: "candidate" | "correct" | "confirm" | "reject" | "question" | "answer" | "skip";
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
