import type { ContextPacket } from "./contextPacket";
import type { ExperienceEntry } from "../../types/domain";
import type { PersistedArtifactBundle } from "../../shared/storage/types";
import { HARNESS_VERSION, PROMPT_VERSION } from "./version";
export interface GenerationSnapshot { sourceEntryId: string; sourceExperienceUpdatedAt: string; requestedTask: ContextPacket["requestedTask"]; recoveryTurns: Array<{ id: string; updatedAt: string }>; confirmedEvidence: Array<{ id: string; updatedAt: string }>; answeredReflections: Array<{ id: string; updatedAt: string }>; harnessVersion: string; promptVersion: string; }
export function createGenerationSnapshot(packet: ContextPacket): GenerationSnapshot { return { sourceEntryId: packet.currentExperience.id, sourceExperienceUpdatedAt: packet.currentExperience.updatedAt, requestedTask: packet.requestedTask, recoveryTurns: packet.answeredClarificationTurns.map(({ id, updatedAt }) => ({ id, updatedAt })), confirmedEvidence: packet.confirmedEvidence.map(({ id, updatedAt }) => ({ id, updatedAt })), answeredReflections: packet.answeredReflectionResponses.map(({ id, updatedAt }) => ({ id, updatedAt })), harnessVersion: packet.harnessVersion, promptVersion: packet.promptVersion }; }
const same = (expected: Array<{ id: string; updatedAt: string }>, actual: Array<{ id: string; updatedAt: string }>) => expected.length === actual.length && expected.every((item) => actual.some((other) => other.id === item.id && other.updatedAt === item.updatedAt));
export function isGenerationSnapshotCurrent(snapshot: GenerationSnapshot, experience: ExperienceEntry | null, bundle: PersistedArtifactBundle): boolean {
  if (!experience || experience.id !== snapshot.sourceEntryId || experience.updatedAt !== snapshot.sourceExperienceUpdatedAt) return false;
  const confirmed = bundle.evidence.filter((item) => item.status === "confirmed").map(({ id, updatedAt }) => ({ id, updatedAt }));
  const answered = bundle.reflections.filter((item) => item.status === "answered" && item.response?.trim() && item.responseProvenance?.origin === "user").map(({ id, updatedAt }) => ({ id, updatedAt }));
  const turns = bundle.recoveryTurns.filter((item) => item.status === "answered" && item.response?.trim()).map(({ id, updatedAt }) => ({ id, updatedAt }));
  return same(snapshot.confirmedEvidence, confirmed) && same(snapshot.answeredReflections, answered) && same(snapshot.recoveryTurns, turns);
}

/** A result created under an older Harness/prompt contract must not be committed. */
export function isGenerationSnapshotCompatible(snapshot: GenerationSnapshot): boolean {
  return snapshot.harnessVersion === HARNESS_VERSION && snapshot.promptVersion === PROMPT_VERSION;
}
