import type { ReactNode } from "react";
import type { PatternNote, ReflectionPrompt } from "../types/domain";
import type { HistoricalQuestionArtifact } from "../historicalContext/governedPacket";
import type { UiCopy } from "./i18n";

interface JourneyStageProps {
  readonly id: string;
  readonly step: string;
  readonly title: string;
  readonly summary: string;
  readonly statusLabel: string;
  readonly active: boolean;
  readonly open: boolean;
  readonly available: boolean;
  readonly onOpen: () => void;
  readonly children: ReactNode;
}

export function JourneyStage({
  id,
  step,
  title,
  summary,
  statusLabel,
  active,
  open,
  available,
  onOpen,
  children,
}: JourneyStageProps) {
  return (
    <section
      id={id}
      className={`journey-stage${active ? " journey-stage-active" : ""}${open ? " journey-stage-open" : ""}`}
      aria-current={active ? "step" : undefined}
      tabIndex={-1}
    >
      <div className="journey-stage-heading">
        <div>
          <p className="review-step">{step}</p>
          <h2>{title}</h2>
          {!open ? <p className="journey-stage-summary">{summary}</p> : null}
        </div>
        <div className="journey-stage-controls">
          <span className="journey-stage-status">{statusLabel}</span>
          <button
            type="button"
            className="ghost-button compact"
            aria-expanded={open}
            aria-controls={`${id}-content`}
            disabled={!available}
            onClick={onOpen}
          >
            {open ? "−" : "+"}
            <span className="visually-hidden"> {title}</span>
          </button>
        </div>
      </div>
      {open ? <div id={`${id}-content`} className="journey-stage-content">{children}</div> : null}
    </section>
  );
}

interface ReflectionCompletionReviewProps {
  readonly id: string;
  readonly copy: UiCopy;
  readonly experienceBody: string;
  readonly confirmedEvidence: readonly string[];
  readonly reflections: readonly ReflectionPrompt[];
  readonly patterns: readonly PatternNote[];
  readonly patternRejectedInSession: boolean;
  readonly historicalQuestions: readonly HistoricalQuestionArtifact[];
  readonly onReopenEvidence: () => void;
  readonly onReopenReflection: () => void;
  readonly onOpenPattern: () => void;
  readonly onOpenHistory: () => void;
  readonly onRecordAnother: () => void;
}

export function ReflectionCompletionReview({
  id,
  copy,
  experienceBody,
  confirmedEvidence,
  reflections,
  patterns,
  patternRejectedInSession,
  historicalQuestions,
  onReopenEvidence,
  onReopenReflection,
  onOpenPattern,
  onOpenHistory,
  onRecordAnother,
}: ReflectionCompletionReviewProps) {
  const answered = reflections.filter(
    (prompt) => prompt.status === "answered" && Boolean(prompt.response?.trim()),
  );
  const skipped = reflections.filter((prompt) => prompt.status === "skipped");
  const pattern = patterns.find((item) => item.status === "confirmed")
    ?? patterns.find((item) => item.status === "candidate");
  const questions = historicalQuestions.flatMap((artifact) => artifact.questions);

  return (
    <section id={id} className="reflection-completion" aria-labelledby={`${id}-title`} tabIndex={-1}>
      <p className="review-step">{copy.completionStep}</p>
      <h2 id={`${id}-title`}>{copy.completionTitle}</h2>
      <p className="completion-intro">{copy.completionIntro}</p>

      <div className="completion-records">
        <section>
          <h3>{copy.completionExperienceLabel}</h3>
          <p>{experienceBody}</p>
        </section>
        <section>
          <h3>{copy.completionEvidenceLabel}</h3>
          <ul>
            {confirmedEvidence.map((text, index) => <li key={`${index}-${text}`}>{text}</li>)}
          </ul>
        </section>
        <section>
          <h3>{copy.completionReflectionLabel}</h3>
          {answered.map((prompt) => (
            <div className="completion-reflection" key={prompt.id}>
              <p><strong>{copy.completionPromptLabel(prompt.promptProvenance.origin)}:</strong> {prompt.question}</p>
              <p><strong>{copy.completionUserResponseLabel}:</strong> {prompt.response}</p>
            </div>
          ))}
          {skipped.length > 0 ? <p>{copy.completionSkipped(skipped.length)}</p> : null}
        </section>
      </div>

      <section className="completion-optional" aria-label={copy.completionOptionalLabel}>
        <h3>{copy.completionOptionalLabel}</h3>
        {pattern ? (
          <p>
            <strong>{copy.completionPatternLabel(pattern.status, pattern.provenance.origin)}:</strong> {pattern.text}
          </p>
        ) : patternRejectedInSession ? (
          <p>{copy.completionPatternRejected}</p>
        ) : (
          <p>{copy.completionPatternAbsent}</p>
        )}
        {questions.length > 0 ? (
          <div>
            <strong>{copy.completionHistoricalLabel}</strong>
            <ul>{questions.map((question) => <li key={question.id}>{question.text}</li>)}</ul>
          </div>
        ) : (
          <p>{copy.completionHistoricalAbsent}</p>
        )}
      </section>

      <p className="completion-rest">{copy.completionCanRest}</p>
      <div className="completion-actions" aria-label={copy.completionActionsLabel}>
        <button type="button" className="ghost-button compact" onClick={onReopenEvidence}>{copy.completionReopenEvidence}</button>
        <button type="button" className="ghost-button compact" onClick={onReopenReflection}>{copy.completionReopenReflection}</button>
        <button type="button" className="secondary-button compact" onClick={onOpenPattern}>{copy.completionExplorePattern}</button>
        <button type="button" className="ghost-button compact" onClick={onOpenHistory}>{copy.completionOpenHistory}</button>
        <button type="button" className="primary-button compact" onClick={onRecordAnother}>{copy.completionRecordAnother}</button>
      </div>
    </section>
  );
}
