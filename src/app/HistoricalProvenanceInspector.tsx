import { useRef, useState } from "react";
import type { HistoricalQuestionArtifact } from "../historicalContext/governedPacket";
import {
  inspectHistoricalQuestionProvenance,
  type HistoricalProvenanceValidation,
  type HistoricalProvenanceViewModel,
} from "../historicalContext/provenanceInspector";
import type { UiCopy } from "./i18n";

interface HistoricalProvenanceInspectorProps {
  readonly artifact: HistoricalQuestionArtifact;
  readonly copy: UiCopy;
}

interface HistoricalProvenanceInspectorPanelProps {
  readonly validation: HistoricalProvenanceValidation | null;
  readonly loading: boolean;
  readonly revealContent: boolean;
  readonly copy: UiCopy;
  readonly onRevealContent: () => void;
}

function Field({ label, value }: { readonly label: string; readonly value: string }) {
  return (
    <div>
      <dt>{label}</dt>
      <dd><code>{value}</code></dd>
    </div>
  );
}

function Stage({
  title,
  children,
}: {
  readonly title: string;
  readonly children: React.ReactNode;
}) {
  return (
    <li>
      <strong>{title}</strong>
      <div>{children}</div>
    </li>
  );
}

export function HistoricalProvenanceInspectorPanel({
  validation,
  loading,
  revealContent,
  copy,
  onRevealContent,
}: HistoricalProvenanceInspectorPanelProps) {
  if (loading) return <p role="status">{copy.historicalProvenanceLoading}</p>;
  if (!validation) return null;
  if (validation.status === "invalid") {
    return <p className="historical-provenance-invalid" role="alert">{copy.historicalProvenanceInvalid}</p>;
  }

  const provenance = validation.viewModel;
  return (
    <div className="historical-provenance-body">
      <p>{copy.historicalProvenanceLocalOnly}</p>
      <p className="historical-provenance-partial">{copy.historicalProvenancePartial}</p>

      <ol className="historical-provenance-stages">
        <Stage title={copy.historicalProvenanceSelected}>
          <code>{provenance.packet.id}</code> · {provenance.includedItems.length} {copy.historicalProvenanceIncluded}
        </Stage>
        <Stage title={copy.historicalProvenanceConsented}>
          <code>{provenance.packet.consentId}</code> · <code>{provenance.packet.consentScope}</code>
        </Stage>
        <Stage title={copy.historicalProvenanceTransmitted}>
          <code>{provenance.packet.transmissionId}</code> · {provenance.packet.provider} / {provenance.packet.model}
        </Stage>
        <Stage title={copy.historicalProvenancePersisted}>
          <code>{provenance.artifact.id}</code> · <time dateTime={provenance.artifact.generatedAt}>{provenance.artifact.generatedAt}</time>
        </Stage>
      </ol>

      <section>
        <h4>{copy.historicalProvenanceQuestions}</h4>
        {provenance.questions.length ? (
          <ul>
            {provenance.questions.map((question) => (
              <li key={question.id}>
                <span>{question.text}</span>{" "}
                <code>{question.sourceExperienceIds.join(", ")}</code>
              </li>
            ))}
          </ul>
        ) : <p>0</p>}
      </section>

      <section>
        <h4>{copy.historicalProvenancePacket}</h4>
        <dl className="historical-provenance-fields">
          <Field label={copy.historicalProvenancePacketId} value={provenance.packet.id} />
          <Field label={copy.historicalProvenanceDigest} value={provenance.packet.digest} />
          <Field label={copy.historicalProvenanceSchema} value={provenance.packet.schemaVersion} />
          <Field label={copy.historicalProvenanceDestination} value={`${provenance.packet.provider} / ${provenance.packet.model}`} />
          <Field label={copy.historicalProvenancePurpose} value={provenance.packet.purpose} />
          <Field label={copy.historicalProvenanceConsent} value={provenance.packet.consentId} />
          <Field label={copy.historicalProvenanceTransmission} value={provenance.packet.transmissionId} />
          <Field
            label={copy.historicalProvenanceVersions}
            value={provenance.packet.versions.map(({ name, value }) => `${name}=${value}`).join("; ")}
          />
        </dl>
      </section>

      <section>
        <h4>{copy.historicalProvenanceIncluded}</h4>
        <ul className="historical-provenance-items">
          {provenance.includedItems.map((item) => (
            <li key={`${item.itemType}:${item.artifactId ?? item.sourceExperienceId}`}>
              <dl className="historical-provenance-fields">
                <Field label={copy.historicalProvenanceItemType} value={item.itemType} />
                <Field label={copy.historicalProvenanceSourceId} value={item.sourceExperienceId} />
                <Field label={copy.historicalProvenanceArtifactReference} value={item.artifactId ?? "—"} />
                <Field label={copy.historicalProvenanceSnapshot} value={item.snapshotReference} />
                <Field label={copy.historicalProvenanceRetrieval} value={item.retrievalAlgorithmVersion} />
                <Field label={copy.historicalProvenanceAuthorship} value={item.authorship} />
                <Field label={copy.historicalProvenanceReview} value={item.reviewState} />
                <Field label={copy.historicalProvenanceRelevance} value={item.relevanceReason} />
              </dl>
            </li>
          ))}
        </ul>
      </section>

      <section>
        <h4>{copy.historicalProvenanceDependencies}</h4>
        <ul>
          {provenance.dependencies.map((dependency, index) => (
            <li key={`${dependency.kind}:${dependency.fromId}:${dependency.sourceExperienceId}:${index}`}>
              <code>{dependency.kind}</code>{" "}
              <code>{dependency.fromId}</code> → <code>{dependency.sourceExperienceId}</code>
            </li>
          ))}
        </ul>
      </section>

      <button type="button" className="ghost-button compact" onClick={onRevealContent}>
        {revealContent ? copy.historicalProvenanceHideContent : copy.historicalProvenanceRevealContent}
      </button>
      {revealContent ? <ExactOutgoingContent provenance={provenance} copy={copy} /> : null}
    </div>
  );
}

function ExactOutgoingContent({
  provenance,
  copy,
}: {
  readonly provenance: HistoricalProvenanceViewModel;
  readonly copy: UiCopy;
}) {
  return (
    <section className="historical-provenance-exact-content">
      <h4>{copy.historicalProvenanceExactContent}</h4>
      <p>{copy.historicalProvenanceExactContentWarning}</p>
      <article>
        <strong>{copy.historicalProvenanceCurrentExperience}</strong>
        <p><code>{provenance.outgoingContent.currentExperience.id}@{provenance.outgoingContent.currentExperience.revision}</code></p>
        <p>{provenance.outgoingContent.currentExperience.content}</p>
      </article>
      {provenance.outgoingContent.includedItems.map((item) => (
        <article key={`${item.type}:${item.id}`}>
          <p><code>{item.type}:{item.id}</code></p>
          <p>{item.content}</p>
        </article>
      ))}
    </section>
  );
}

export function HistoricalProvenanceInspector({
  artifact,
  copy,
}: HistoricalProvenanceInspectorProps) {
  const [validation, setValidation] = useState<HistoricalProvenanceValidation | null>(null);
  const [loading, setLoading] = useState(false);
  const [revealContent, setRevealContent] = useState(false);
  const request = useRef(0);

  const handleToggle = async (event: React.SyntheticEvent<HTMLDetailsElement>) => {
    const open = event.currentTarget.open;
    request.current += 1;
    const currentRequest = request.current;
    setRevealContent(false);
    setValidation(null);
    if (!open) {
      setLoading(false);
      return;
    }
    setLoading(true);
    const result = await inspectHistoricalQuestionProvenance(artifact);
    if (request.current === currentRequest) {
      setValidation(result);
      setLoading(false);
    }
  };

  return (
    <details className="historical-provenance-inspector" onToggle={handleToggle}>
      <summary>
        <span className="historical-provenance-open">{copy.historicalProvenanceOpen}</span>
        <span className="historical-provenance-close">{copy.historicalProvenanceClose}</span>
      </summary>
      <h3>{copy.historicalProvenanceTitle}</h3>
      <HistoricalProvenanceInspectorPanel
        validation={validation}
        loading={loading}
        revealContent={revealContent}
        copy={copy}
        onRevealContent={() => setRevealContent((current) => !current)}
      />
    </details>
  );
}
