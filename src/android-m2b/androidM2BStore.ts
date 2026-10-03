import { invoke } from "@tauri-apps/api/core";
import type { ExperienceEntry } from "../types/domain";

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
export interface M2BStore {
  status(): Promise<{ state: "ready"; applicationId: "com.lifeos.review.m2b"; schemaVersion: 5; syntheticOnly: true }>;
  create(id: string, text: string): Promise<void>;
  list(): Promise<ExperienceEntry[]>;
  get(id: string): Promise<Snapshot | null>;
  mutate(request: LifecycleRequest, debugPhase?: DebugPhase): Promise<MutationAck>;
  readLocale(): Promise<Locale | null>;
  writeLocale(locale: Locale): Promise<void>;
}

export const m2bStore: M2BStore = {
  status: () => invoke("m2b_storage_status"),
  create: (id, text) => invoke("m2b_create_experience", { id, text }),
  list: () => invoke("m2b_list_experiences"),
  get: (id) => invoke("m2b_get_experience", { id }),
  mutate: (request, debugPhase) => invoke("m2b_mutate_experience", { request, debugPhase }),
  readLocale: () => invoke("m2b_get_locale_preference"),
  writeLocale: (locale) => invoke("m2b_set_locale_preference", { locale }),
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
  const requestId = "m2b_" + await sha256(JSON.stringify([
    "life-os/m2b-request-v1", operation, snapshot.entry.id, snapshot.revisionId, occurredAt, digest,
  ]));
  return Object.freeze({ requestId, operation, id: snapshot.entry.id,
    expectedRevisionId: snapshot.revisionId, occurredAt, body });
}

export function createId(): string { return "m2b_" + globalThis.crypto.randomUUID(); }
