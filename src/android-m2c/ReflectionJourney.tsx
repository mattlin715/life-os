import { useEffect, useRef, useState } from "react";
import { createContextPacket } from "../ai/harness/contextPacket";
import { decideContextGate } from "../ai/harness/gateDecision";
import { generateLocalReflectionPrompts } from "../ai/providers/placeholderProvider";
import { answerReflectionPrompt } from "../ai/harness/reflectionResponse";
import type { ArtifactProvenance, EvidenceCandidate, ReflectionPrompt } from "../types/domain";
import { artifactRequest, nextTimestamp } from "./androidM2CStore";
import type { ArtifactRequest, ArtifactSnapshot, DebugPhase, Locale, M2CStore, Snapshot } from "./androidM2CStore";
import { journeyCopy } from "./journeyCopy";

// Hide a duplicate only when the persisted candidate is exactly the canonical
// quote. Never summarize, strip, or substitute corrected/unknown content.
export function candidateRepeatsSource(record: EvidenceCandidate, source: Snapshot): boolean {
  return ["Your own words", "你寫下的原文", "あなたが書いた原文"]
    .some((label) => record.text === `${label}:\n${source.entry.body.trim()}`);
}

export function CandidateContent({ record, source, reviewed, locale }: {
  record: EvidenceCandidate; source: Snapshot; reviewed: boolean; locale: Locale;
}) {
  const c = journeyCopy[locale];
  const repeat = candidateRepeatsSource(record, source);
  const content = <p data-testid="m2c-candidate-text">{record.text}</p>;
  return <>
    {repeat ? <p data-testid="m2c-source-reference" className="android-m2c__secondary">{c.sameSource}</p> : null}
    {repeat || reviewed ? <details className="android-m2c__secondary-details" data-testid="m2c-candidate-content"><summary>{c.viewContent}</summary>{content}</details> : content}
  </>;
}

// Presentation only: names the separate candidate, never changes its content or state.
export function CandidateScopeNote({ locale }: { locale: Locale }) {
  return <p className="android-m2c__scope-note" data-testid="m2c-candidate-scope">{journeyCopy[locale].candidateExplanation}</p>;
}

export function ArtifactDetails({ provenance, response, edited, locale, testId }: {
  provenance?: ArtifactProvenance; response?: ArtifactProvenance; edited?: boolean; locale: Locale; testId: string;
}) {
  const c = journeyCopy[locale];
  const origin = (p?: ArtifactProvenance) => p?.origin === "local_mock" ? c.localOrigin : p?.origin === "user" ? c.userOrigin : c.unknownOrigin;
  return <details className="android-m2c__secondary-details" data-testid={testId}>
    <summary>{c.details}</summary>
    <p>{origin(provenance)}</p>
    {edited ? <p>{c.editedOrigin}</p> : null}
    {response ? <p>{c.responseOrigin}: {origin(response)}</p> : null}
    <a href="#m2c-source-content">{c.sourceLabel}</a>
    <details data-testid="m2c-technical-provenance"><summary>{c.technical}</summary>
      <dl className="android-m2c__metadata">
        <dt>origin</dt><dd>{provenance?.origin ?? c.unknownOrigin}</dd>
        <dt>provider</dt><dd>{provenance?.provider ?? c.unknownOrigin}</dd>
        <dt>model</dt><dd>{provenance?.model === undefined ? c.unknownOrigin : JSON.stringify(provenance.model)}</dd>
      </dl>
      <pre>{JSON.stringify({ promptOrCandidate: provenance ?? null, userResponse: response ?? null }, null, 2)}</pre>
    </details>
  </details>;
}

// Presentation only. These historical rows never become eligible Evidence and
// displaying them does not generate, restore or mutate a candidate.
export function isRejectedCandidate(row: ArtifactSnapshot): boolean {
  return row.kind === "evidence" && row.lifecycleState !== "active" && row.reviewState === "rejected";
}

export function candidateGenerationLabel(rows: readonly ArtifactSnapshot[], locale: Locale): string {
  return rows.some(isRejectedCandidate) ? journeyCopy[locale].recreate : journeyCopy[locale].generate;
}

export function RejectedCandidateHistory({ rows, locale }: { rows: readonly ArtifactSnapshot[]; locale: Locale }) {
  const rejected = rows.filter(isRejectedCandidate);
  if (!rejected.length) return null;
  const c = journeyCopy[locale];
  return <div data-testid="m2c-rejection-group">
    <p data-testid="m2c-rejection-summary">{c.rejectedSummary}</p>
    <details className="android-m2c__secondary-details" data-testid="m2c-rejection-history">
      <summary>{c.rejectedHistory} ({rejected.length})</summary>
      {rejected.map((row) => <article key={row.id} data-artifact-id={row.id} data-lifecycle={row.lifecycleState}>
        <p>{c.rejected}</p>
        <details data-testid="m2c-rejection-metadata"><summary>{c.technical}</summary>
          <pre>{JSON.stringify({ id: row.id, kind: row.kind, revisionId: row.revisionId, reviewState: row.reviewState,
            lifecycleState: row.lifecycleState, eligibilityState: row.eligibilityState }, null, 2)}</pre>
        </details>
      </article>)}
    </details>
  </div>;
}

export function currentEvidence(rows: ArtifactSnapshot[]): ArtifactSnapshot[] {
  return rows.filter((v) => v.kind === "evidence" && v.lifecycleState === "active" && v.reviewState === "confirmed" && v.eligibilityState === "eligible" && v.revisionId && v.payload);
}

export function reflectionPacket(source: Snapshot, rows: ArtifactSnapshot[], locale: Locale) {
  return createContextPacket({ currentExperience: source.entry,
    evidence: currentEvidence(rows).map((v) => v.payload as EvidenceCandidate),
    // No historical, orphaned, skipped, foreign or unsaved response is supplied.
    locale, requestedTask: "reflection", provider: "mock", model: null });
}

// Translation of the existing placeholder question, not a different AI policy.
export function localQuestion(source: Snapshot, rows: ArtifactSnapshot[], locale: Locale): string | null {
  const { packet, issues } = reflectionPacket(source, rows, locale);
  if (issues.length || !decideContextGate(source.entry.body, []).allowReflection) return null;
  const generated = generateLocalReflectionPrompts(packet);
  return generated.length ? journeyCopy[locale].question : null;
}

export function ReflectionJourney({ source, locale, store, onLock, onReopen }: {
  source: Snapshot; locale: Locale; store: M2CStore; onLock: (value: boolean) => void; onReopen: () => void;
}) {
  const c = journeyCopy[locale];
  const [rows, setRows] = useState<ArtifactSnapshot[]>([]);
  const [state, setState] = useState("loading");
  const [frozen, setFrozen] = useState<ArtifactRequest | null>(null);
  const [correction, setCorrection] = useState<{ id: string; text: string } | null>(null);
  const [responses, setResponses] = useState<Record<string, string>>({});
  const [debugPhase, setDebugPhase] = useState<DebugPhase | undefined>();
  const busy = useRef(false);
  const mounted = useRef(true);
  useEffect(() => {
    mounted.current = true;
    let active = true;
    void store.artifacts(source.entry.id).then((values) => { if (active) { setRows(values); setState("idle"); } })
      .catch(() => { if (active) { setState("failed"); onLock(true); } });
    return () => { active = false; mounted.current = false; onLock(false); };
  }, [source, store, onLock]);
  const locked = state === "loading" || state === "working" || state === "failed" || state === "conflict" || frozen !== null;
  const gate = decideContextGate(source.entry.body, []);
  const evidence = currentEvidence(rows);
  const execute = async (request: ArtifactRequest) => {
    if (busy.current) return;
    busy.current = true; onLock(true); setFrozen(request); setState("working");
    try {
      // Await backend verified canonical commit/reread; never save a UI-only bundle.
      const ack = await store.artifactMutate(request, debugPhase);
      if (ack.requestId !== request.requestId || ack.acknowledgement === "committedNotCurrent") throw new Error("m2c_stale_revision_preserved");
      const current = await store.get(source.entry.id);
      const actual = await store.artifacts(source.entry.id);
      if (!current || current.revisionId !== request.expectedSourceRevisionId) throw new Error("m2c_stale_revision_preserved");
      if (mounted.current) {
        setRows(actual); setFrozen(null); setCorrection(null); setResponses({}); setDebugPhase(undefined); setState("saved"); onLock(false);
      }
    } catch (error) {
      // Keep the immutable request and suppress arbitrary backend text.
      if (mounted.current) setState(String(error).includes("stale") || String(error).includes("identity_conflict") ? "conflict" : "failed");
    } finally { busy.current = false; }
  };
  const submit = async (operation: ArtifactRequest["operation"], target: ArtifactSnapshot | null, text: string | null = null) => {
    if (busy.current || locked) return;
    // Bind only exact IDs cited by this prompt; another eligible artifact must
    // never be silently substituted while saving a response.
    const prompt = target?.kind === "reflection" ? target.payload as ReflectionPrompt : null;
    const selected = operation === "question" ? evidence.slice(0, 1) : prompt ? evidence.filter((v) => prompt.sourceEvidenceIds.includes(v.id)) : [];
    if (prompt && selected.length !== prompt.sourceEvidenceIds.length) { setState("conflict"); onLock(true); return; }
    const latest = rows.reduce((time, v) => v.payload && v.payload.updatedAt > time ? v.payload.updatedAt : time, source.entry.updatedAt);
    // Fence the asynchronous hashing window as well as the IPC window.
    busy.current = true; onLock(true); setState("working");
    let request: ArtifactRequest;
    try {
      const occurredAt = nextTimestamp(latest);
      const response = operation === "answer" && prompt ? answerReflectionPrompt(prompt, text ?? "", occurredAt).response! : text;
      request = await artifactRequest(source, operation, locale, target, response,
        selected.map((v) => ({ artifactId: v.id, revisionId: v.revisionId! })), occurredAt);
    } catch { busy.current = false; setState("failed"); return; }
    busy.current = false;
    await execute(request);
  };
  return <section data-testid="m2c-journey" data-artifact-state={state} className="android-m2c__journey">
    <h2>{c.title}</h2>
    {state === "working" || state === "saved" || state === "failed" || state === "conflict" ? <p role="status" data-testid="m2c-artifact-feedback">{state === "working" ? c.working : state === "saved" ? c.saved : state === "conflict" ? c.conflict : c.failed}</p> : null}
    {(state === "failed" || state === "conflict") && frozen ? <button data-testid="m2c-artifact-reconcile" disabled={busy.current} onClick={() => void execute(frozen)}>{c.retry}</button> : null}
    {state === "failed" || state === "conflict" ? <button data-testid="m2c-artifact-reopen" disabled={busy.current} onClick={() => { onLock(false); setFrozen(null); setResponses({}); setCorrection(null); onReopen(); }}>{c.reopen}</button> : null}
    <RejectedCandidateHistory rows={rows} locale={locale} />
    {rows.every((v) => v.kind !== "evidence" || v.lifecycleState !== "active") ? <>
      {!gate.allowObservation ? <p>{c.sparse}</p> : null}
      <button data-testid="m2c-generate-candidate" disabled={locked || !gate.allowObservation} onClick={() => void submit("candidate", null)}>{candidateGenerationLabel(rows, locale)}</button>
    </> : null}
    {rows.map((row) => {
      const active = row.lifecycleState === "active";
      if (isRejectedCandidate(row)) return null;
      if (!active) return <article key={row.id} data-artifact-id={row.id} data-lifecycle={row.lifecycleState}><p>{row.reviewState === "rejected" ? c.rejected : c.invalidated}</p></article>;
      if (!row.payload) return null;
      if (row.kind === "evidence") {
        const record = row.payload as EvidenceCandidate;
        return <article key={row.id} data-artifact-id={row.id} data-review-state={row.reviewState}>
          <div className="android-m2c__review-heading"><h3>{c.candidate}</h3><span>{row.reviewState === "confirmed" ? c.confirmed : c.pending}</span></div>
          <CandidateScopeNote locale={locale} />
          <CandidateContent record={record} source={source} reviewed={row.reviewState === "confirmed"} locale={locale} />
          {row.reviewState === "pending" ? correction?.id === row.id ? <>
            <label htmlFor="m2c-correction">{c.correctionLabel}</label><p id="m2c-correction-help" data-testid="m2c-correction-help" className="android-m2c__secondary">{c.correctionHelp}</p><textarea autoFocus aria-describedby="m2c-correction-help" id="m2c-correction" data-testid="m2c-correction" disabled={locked} value={correction.text} onChange={(e) => setCorrection({ id: row.id, text: e.target.value })} />
            <button data-testid="m2c-save-correction" disabled={locked || !correction.text.trim()} onClick={() => void submit("correct", row, correction.text)}>{c.saveCorrection}</button>
            <button data-testid="m2c-cancel-correction" disabled={locked} onClick={() => setCorrection(null)}>{c.cancel}</button>
          </> : <>
            <button data-testid="m2c-correct" disabled={locked} onClick={() => setCorrection({ id: row.id, text: record.text })}>{c.correct}</button>
            <button data-testid="m2c-confirm" disabled={locked} onClick={() => void submit("confirm", row)}>{c.confirm}</button>
            <button data-testid="m2c-reject" disabled={locked} onClick={() => void submit("reject", row)}>{c.reject}</button>
          </> : null}
          <ArtifactDetails provenance={record.provenance} edited={!!record.originalText && record.originalText !== record.text} locale={locale} testId="m2c-evidence-provenance" />
        </article>;
      }
      const prompt = row.payload as ReflectionPrompt;
      return <article key={row.id} data-artifact-id={row.id} data-response-state={prompt.status}>
        <h3>{c.reflection}</h3><p data-testid="m2c-question">{prompt.question}</p>
        {prompt.status === "suggested" ? <>
          <label htmlFor={`response-${row.id}`}>{c.answerLabel}</label><textarea id={`response-${row.id}`} data-testid="m2c-response-draft" disabled={locked} value={responses[row.id] ?? ""} onChange={(e) => setResponses({ ...responses, [row.id]: e.target.value })} />
          {responses[row.id]?.trim() ? <p data-testid="m2c-response-unsaved">{c.draft}</p> : null}
          <button data-testid="m2c-save-response" disabled={locked || !responses[row.id]?.trim()} onClick={() => void submit("answer", row, responses[row.id])}>{c.saveAnswer}</button>
          <button data-testid="m2c-skip" disabled={locked} onClick={() => void submit("skip", row)}>{c.skip}</button>
        </> : prompt.status === "answered" ? <><h4>{c.response}</h4><p data-testid="m2c-saved-response">{prompt.response}</p></> : <p data-testid="m2c-skipped">{c.skipped}</p>}
        <ArtifactDetails provenance={prompt.promptProvenance} response={prompt.responseProvenance} locale={locale} testId="m2c-reflection-provenance" />
      </article>;
    })}
    {evidence.length > 0 && !rows.some((v) => v.kind === "reflection" && v.lifecycleState === "active") ? <button data-testid="m2c-generate-question" disabled={locked || !localQuestion(source, rows, locale)} onClick={() => void submit("question", null, localQuestion(source, rows, locale))}>{c.ask}</button> : null}
    {import.meta.env.VITE_LIFE_OS_ANDROID_M2C_DEBUG_HOOKS === "1" ? <details hidden data-debug-only="artifact"><summary>Disposable artifact debug probes</summary>{(["beforeCommit", "afterCommitBeforeAck", "rollbackAfterProjection"] as DebugPhase[]).map((phase) => <button key={phase} data-artifact-debug-phase={phase} disabled={locked} onClick={() => setDebugPhase(phase)}>{phase}</button>)}</details> : null}
  </section>;
}
