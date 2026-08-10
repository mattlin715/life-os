import type { HistoricalContextPacket } from "../historicalContext/governedPacket";
import type { UiCopy } from "./i18n";

export interface EligibleHistoricalArtifactRecord {
  readonly id: string;
  readonly sourceExperienceId: string;
  readonly revision: string;
  readonly type: "evidence" | "reflection_response";
  readonly content: string;
  readonly relevanceReason: string;
}

interface HistoricalConsentPreflightProps {
  readonly copy: UiCopy;
  readonly packet: HistoricalContextPacket;
  readonly eligibleArtifacts: readonly EligibleHistoricalArtifactRecord[];
  readonly pending: boolean;
  readonly onToggleSource: (sourceExperienceId: string) => void;
  readonly onToggleArtifact: (artifactId: string) => void;
  readonly onSend: () => void;
  readonly onCancel: () => void;
  readonly onAdjustSources: () => void;
}

function typeLabel(type: HistoricalContextPacket["includedItems"][number]["itemType"], copy: UiCopy) {
  if (type === "experience") return copy.historicalItemExperience;
  if (type === "evidence") return copy.historicalItemEvidence;
  return copy.historicalItemReflection;
}

export function HistoricalConsentPreflight({
  copy,
  packet,
  eligibleArtifacts,
  pending,
  onToggleSource,
  onToggleArtifact,
  onSend,
  onCancel,
  onAdjustSources,
}: HistoricalConsentPreflightProps) {
  const experienceCount = packet.includedItems.filter((item) => item.itemType === "experience").length;
  const evidenceCount = packet.includedItems.filter((item) => item.itemType === "evidence").length;
  const reflectionCount = packet.includedItems.filter((item) => item.itemType === "reflection_response").length;

  return (
    <section className="historical-preflight" aria-label={copy.historicalPreflightAria}>
      <div className="historical-preflight-summary">
        <h3>{copy.historicalPreflightTitle}</h3>
        <p>{copy.historicalPreflightSummary}</p>
        <p>{copy.historicalPreflightDestination(packet.destination.provider, packet.destination.model)}</p>
        <p>{copy.historicalPreflightPurpose(packet.purpose)}</p>
        <p>{copy.historicalPreflightIncludedSummary(experienceCount, evidenceCount, reflectionCount)}</p>
        <p className="historical-sensitive-warning">{copy.historicalPreflightSensitiveWarning}</p>
        <p className="summary-note">{copy.historicalPreflightConsentScope}</p>
      </div>

      <details className="historical-preflight-details">
        <summary>{copy.historicalViewDetails}</summary>
        <p>{copy.historicalPreflightRetention}: {packet.destination.retentionDisclosure}</p>
        <dl className="historical-preflight-meta">
          <div><dt>{copy.historicalPreflightPacketId}</dt><dd>{packet.packetId}</dd></div>
          <div><dt>{copy.historicalPreflightPacketDigest}</dt><dd>{packet.packetDigest}</dd></div>
          <div><dt>{copy.historicalPreflightPacketSchema}</dt><dd>{packet.schemaVersion}</dd></div>
          <div><dt>{copy.historicalPreflightCurrentExperience}</dt><dd>{packet.currentExperience.id} @ {packet.currentExperience.revision}</dd></div>
          <div><dt>{copy.historicalPreflightVersions}</dt><dd>{Object.entries(packet.versions).map(([key, value]) => `${key}=${value}`).join("; ")}</dd></div>
        </dl>
        <p className="historical-preflight-content">{packet.currentExperience.content}</p>
        <p className="summary-note">{copy.historicalPacketImmutable(packet.packetId, packet.packetDigest)}</p>

        <div className="historical-preflight-items">
          {packet.includedItems.map((item) => (
            <article key={`${item.itemType}:${item.artifactId ?? item.sourceExperienceId}`}>
              <label>
                <input
                  type="checkbox"
                  checked
                  onChange={() => item.itemType === "experience"
                    ? onToggleSource(item.sourceExperienceId)
                    : item.artifactId && onToggleArtifact(item.artifactId)}
                />
                <strong>{typeLabel(item.itemType, copy)}</strong>
              </label>
              <p>{item.content}</p>
              <p className="summary-note">
                {item.sourceExperienceId} @ {item.revision}
                {item.artifactId ? ` · ${item.artifactId}` : ""}
              </p>
              <p className="summary-note">
                {copy.historicalPreflightAuthorship}: {item.authorship} · {copy.historicalPreflightReviewState}: {item.reviewState}
              </p>
              <p className="summary-note">{copy.historicalPreflightRelevance(item.relevanceReason)}</p>
            </article>
          ))}
        </div>

        {eligibleArtifacts.length ? (
          <div className="historical-preflight-eligible">
            <h4>{copy.historicalPreflightEligibleArtifacts}</h4>
            {eligibleArtifacts.map((artifact) => {
              const included = packet.includedItems.some((item) => item.artifactId === artifact.id);
              return (
                <label key={artifact.id}>
                  <input type="checkbox" checked={included} onChange={() => onToggleArtifact(artifact.id)} />
                  <span>{artifact.type === "evidence" ? copy.historicalItemEvidence : copy.historicalItemReflection}: {artifact.content}</span>
                </label>
              );
            })}
          </div>
        ) : null}
      </details>

      <div className="entry-actions historical-preflight-actions">
        <button type="button" className="primary-button" disabled={pending} onClick={onSend}>
          {pending ? copy.historicalSending : copy.historicalConsentAndSend}
        </button>
        <button type="button" className="ghost-button" disabled={pending} onClick={onAdjustSources}>
          {copy.historicalAdjustSources}
        </button>
        <button type="button" className="ghost-button" disabled={pending} onClick={onCancel}>
          {copy.historicalCancel}
        </button>
      </div>
    </section>
  );
}
