import type { PersistedArtifactBundle } from "./types";

function ordered(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(ordered);
  if (value !== null && typeof value === "object") {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map(key => [key, ordered(record[key])]));
  }
  return value;
}
/** Local write guard only. Suggested questions are NOT evidence or provider context.
 * Include full relevant inputs, not only timestamps (two edits can share a clock tick).
 */
export function reflectionWritePrecondition(bundle: PersistedArtifactBundle): string {
  const byId = <T extends { id: string }>(items: T[]) => [...items].sort((a, b) => a.id.localeCompare(b.id));
  return JSON.stringify(ordered({ reflections: byId(bundle.reflections),
    confirmedEvidence: byId(bundle.evidence.filter(item => item.status === "confirmed")),
    answeredRecovery: byId(bundle.recoveryTurns.filter(item => item.status === "answered" && item.response?.trim())),
  }));
}
