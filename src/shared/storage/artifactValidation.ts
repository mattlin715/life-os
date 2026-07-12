import type { PersistedArtifactBundle } from "./types";
export function validateArtifactBundle(entryId: string, bundle: PersistedArtifactBundle) {
  const issues: string[] = [];
  const evidence = bundle.evidence.filter((item) => item.sourceEntryId === entryId && item.status !== "rejected");
  const evidenceIds = new Set(evidence.filter((item) => item.status === "confirmed").map((item) => item.id));
  const reflections = bundle.reflections.filter((item) => {
    const valid = item.sourceEntryId === entryId && item.sourceEvidenceIds.length > 0 && item.sourceEvidenceIds.every((id) => evidenceIds.has(id));
    if (!valid) issues.push(`invalid_reflection_dependency:${item.id}`);
    if (item.status === "answered" && !item.responseProvenance) issues.push(`legacy_or_missing_response_provenance:${item.id}`);
    return valid;
  });
  const answeredReflectionIds = new Set(reflections.filter((item) => item.status === "answered" && Boolean(item.response?.trim())).map((item) => item.id));
  const patterns = bundle.patterns.filter((item) => {
    const validEvidence = item.sourceEntryId === entryId && item.sourceEvidenceIds.every((id) => evidenceIds.has(id));
    const validReflection = !item.sourceReflectionPromptIds || item.sourceReflectionPromptIds.every((id) => answeredReflectionIds.has(id));
    const valid = item.status !== "rejected" && validEvidence && validReflection;
    if (!valid) issues.push(`invalid_pattern_dependency:${item.id}`);
    return valid;
  });
  const recoveryTurns = bundle.recoveryTurns.filter((item) => item.sourceEntryId === entryId);
  return { bundle: { evidence, reflections, patterns, recoveryTurns }, validationIssues: issues };
}
