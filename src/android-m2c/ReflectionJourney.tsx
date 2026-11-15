import { useEffect, useRef, useState } from "react";
import { createContextPacket } from "../ai/harness/contextPacket";
import { decideContextGate } from "../ai/harness/gateDecision";
import { HARNESS_VERSION, PROMPT_VERSION } from "../ai/harness/version";
import { generateLocalReflectionPrompts } from "../ai/providers/placeholderProvider";
import { answerReflectionPrompt } from "../ai/harness/reflectionResponse";
import type { ArtifactProvenance, EvidenceCandidate, ReflectionPrompt } from "../types/domain";
import { artifactRequest, nextTimestamp, questionBatchRequest, reflectionSet } from "./androidM2CStore";
import type { ArtifactRequest, ArtifactSnapshot, DebugPhase, EvidenceRef, Locale, M2CStore, QuestionBatchRequest, Snapshot } from "./androidM2CStore";
import { journeyCopy } from "./journeyCopy";
import { PilotModelQuestion } from "./PilotModelPanels";
import { connectedPilot, type ModelPort, type ModelPreview } from "./pilotModel";
import { pilotCopy } from "./pilotModelCopy";

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

function QuestionOrigin({ provenance, locale }: { provenance?: ArtifactProvenance; locale: Locale }) {
  const c = journeyCopy[locale];
  const local = provenance?.origin === "local_mock" && provenance.provider === "mock" && provenance.model === null;
  const gemini = provenance?.origin === "ai" && provenance.provider === "gemini" && provenance.promptVersion === "android-gemini-reflection-v1";
  return <p className="android-m2c__origin-label" data-testid="m2c-question-origin">{local ? c.demoQuestion : gemini ? pilotCopy[locale].generated : c.unknownOrigin}</p>;
}

export function ArtifactDetails({ provenance, response, edited, note, locale, testId }: {
  provenance?: ArtifactProvenance; response?: ArtifactProvenance; edited?: boolean; note?: string; locale: Locale; testId: string;
}) {
  const c = journeyCopy[locale];
  const origin = (p?: ArtifactProvenance) => p?.origin === "local_mock" ? c.localOrigin : p?.origin === "user" ? c.userOrigin
    : p?.origin === "ai" && p.provider === "gemini" && p.promptVersion === "android-gemini-reflection-v1" ? pilotCopy[locale].generated : c.unknownOrigin;
  return <details className="android-m2c__secondary-details" data-testid={testId}>
    <summary>{c.details}</summary>
    <p>{origin(provenance)}</p>
    {edited ? <p>{c.editedOrigin}</p> : null}
    {note ? <p>{note}</p> : null}
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

function isSupersededReflection(row: ArtifactSnapshot): boolean {
  return row.lifecycleState === "invalidated" && (row.kind === "evidence" || row.kind === "reflection") && !isRejectedCandidate(row);
}

function supersededReflectionNotice(rows: readonly ArtifactSnapshot[], locale: Locale) {
  const superseded = rows.filter(isSupersededReflection);
  if (!superseded.length) return null;
  const c = journeyCopy[locale];
  const questions = superseded.filter((row) => row.kind === "reflection").length;
  const hasCurrentWork = rows.some((row) => row.lifecycleState === "active" && (row.kind === "evidence" || row.kind === "reflection"));
  return <aside className="android-m2c__source-update" aria-label={c.sourceUpdatedTitle} data-testid="m2c-source-update">
    <h3>{c.sourceUpdatedTitle}</h3>
    <p>{questions ? c.supersededQuestions(questions) : c.supersededClues}</p>
    {!hasCurrentWork ? <p>{c.reflectOnUpdatedSource}</p> : null}
  </aside>;
}

export function currentEvidence(rows: ArtifactSnapshot[]): ArtifactSnapshot[] {
  return rows.filter((v) => v.kind === "evidence" && v.lifecycleState === "active" && v.reviewState === "confirmed" && v.eligibilityState === "eligible" && v.revisionId && v.payload);
}

export function canCorrectResponse(row: ArtifactSnapshot, source: Snapshot, rows: ArtifactSnapshot[]): boolean {
  const prompt = row.payload as ReflectionPrompt | null;
  const evidence = currentEvidence(rows);
  return row.kind === "reflection" && row.lifecycleState === "active" && row.eligibilityState === "eligible"
    && row.reviewState === "not_applicable" && !!row.revisionId && prompt?.status === "answered"
    && prompt.id === row.id && prompt.sourceEntryId === source.entry.id
    && prompt.responseProvenance?.origin === "user" && !!prompt.response?.trim()
    && prompt.sourceEvidenceIds.length === 1 && evidence.some((v) => v.id === prompt.sourceEvidenceIds[0]);
}

export function reflectionPacket(source: Snapshot, rows: ArtifactSnapshot[], locale: Locale) {
  return createContextPacket({ currentExperience: source.entry,
    evidence: currentEvidence(rows).map((v) => v.payload as EvidenceCandidate),
    // No historical, orphaned, skipped, foreign or unsaved response is supplied.
    locale, requestedTask: "reflection", provider: "mock", model: null });
}

function localQuestionEvidence(source: Snapshot, rows: ArtifactSnapshot[], locale: Locale): EvidenceRef[] {
  const { packet, issues } = reflectionPacket(source, rows, locale);
  if (issues.length || !decideContextGate(source.entry.body, []).allowReflection) return [];
  const evidence = new Map(currentEvidence(rows).map((row) => [row.id, row.revisionId!]));
  const refs: EvidenceRef[] = [];
  for (const prompt of generateLocalReflectionPrompts(packet)) {
    const artifactId = prompt.sourceEvidenceIds[0];
    const revisionId = evidence.get(artifactId);
    if (prompt.sourceEvidenceIds.length !== 1 || !revisionId) return [];
    refs.push({ artifactId, revisionId });
  }
  return refs;
}

// The same localized offline mock text is constructed and verified natively.
export function localQuestion(source: Snapshot, rows: ArtifactSnapshot[], locale: Locale): string | null {
  const { packet, issues } = reflectionPacket(source, rows, locale);
  return !issues.length && packet.confirmedEvidence.length && decideContextGate(source.entry.body, []).allowReflection
    ? journeyCopy[locale].question : null;
}

type JourneyRequest = ArtifactRequest | QuestionBatchRequest;

function batchVisible(request: QuestionBatchRequest, rows: ArtifactSnapshot[]): boolean {
  const ids = new Set(request.items.map((item) => item.artifactId));
  if (reflectionSet(rows.filter((row) => !ids.has(row.id))) !== request.expectedReflectionSet) return false;
  const eligible = currentEvidence(rows);
  return request.items.every(({ artifactId, evidence }) => {
    const row = rows.find((item) => item.id === artifactId);
    const prompt = row?.payload as ReflectionPrompt | null | undefined;
    const provenance = prompt?.promptProvenance;
    return row?.kind === "reflection" && row.revisionId && row.lifecycleState === "active"
      && row.reviewState === "pending" && row.eligibilityState === "ineligible"
      && prompt?.status === "suggested" && prompt.id === artifactId
      && prompt.response === undefined && prompt.responseProvenance === undefined
      && provenance?.origin === "local_mock" && provenance.provider === "mock" && provenance.model === null
      && provenance.harnessVersion === HARNESS_VERSION && provenance.promptVersion === PROMPT_VERSION
      && provenance.sourceEntryId === request.sourceId && provenance.generatedAt === request.occurredAt
      && provenance.sourceArtifactIds.length === 1 && provenance.sourceArtifactIds[0] === evidence.artifactId
      && eligible.some((item) => item.id === evidence.artifactId && item.revisionId === evidence.revisionId)
      && prompt.sourceEntryId === request.sourceId && prompt.question === journeyCopy[request.locale].question
      && prompt.createdAt === request.occurredAt && prompt.updatedAt === request.occurredAt
      && prompt.sourceEvidenceIds.length === 1 && prompt.sourceEvidenceIds[0] === evidence.artifactId;
  });
}

export function ReflectionJourney({ source, locale, store, onLock, onReopen, guided = false, model, hasModelKey = false, modelVisible = false, openModelSettings }: {
  source: Snapshot; locale: Locale; store: M2CStore; onLock: (value: boolean) => void; onReopen: () => void;
  guided?: boolean;
  model?: ModelPort; hasModelKey?: boolean; modelVisible?: boolean; openModelSettings?: () => void;
}) {
  const c = journeyCopy[locale];
  const [rows, setRows] = useState<ArtifactSnapshot[]>([]);
  const [state, setState] = useState("loading");
  const [frozen, setFrozen] = useState<JourneyRequest | null>(null);
  const [correction, setCorrection] = useState<{ id: string; text: string } | null>(null);
  const [responses, setResponses] = useState<Record<string, string>>({});
  const [responseCorrection, setResponseCorrection] = useState<{ target: ArtifactSnapshot; text: string } | null>(null);
  const [inspected, setInspected] = useState<{ source: Snapshot | null; rows: ArtifactSnapshot[] } | null>(null);
  const [debugPhase, setDebugPhase] = useState<DebugPhase | undefined>();
  const busy = useRef(false);
  const mounted = useRef(true);
  const savedClueOperation = useRef(false);
  useEffect(() => {
    mounted.current = true;
    let active = true;
    void store.artifacts(source.entry.id).then((values) => { if (active) { setRows(values); setState("idle"); } })
      .catch(() => { if (active) { setState("failed"); onLock(true); } });
    return () => { active = false; mounted.current = false; onLock(false); };
  }, [source, store, onLock]);
  const locked = state === "loading" || state === "working" || state === "failed" || state === "conflict" || frozen !== null;
  const otherLocked = locked || responseCorrection !== null;
  const gate = decideContextGate(source.entry.body, []);
  const evidence = currentEvidence(rows);
  const execute = async (request: JourneyRequest) => {
    if (busy.current) return;
    busy.current = true; onLock(true); setFrozen(request); setState("working");
    try {
      // Await backend verified canonical commit/reread; never save a UI-only bundle.
      const ack = await ("items" in request
        ? store.generateQuestions(request, debugPhase) : store.artifactMutate(request, debugPhase));
      if (ack.requestId !== request.requestId || ack.acknowledgement === "committedNotCurrent") throw new Error("m2c_stale_revision_preserved");
      const current = await store.get(source.entry.id);
      const actual = await store.artifacts(source.entry.id);
      if (!current || current.revisionId !== request.expectedSourceRevisionId) throw new Error("m2c_stale_revision_preserved");
      if ("items" in request) {
        if ((ack.acknowledgement !== "committed" && ack.acknowledgement !== "alreadyCommitted")
          || !batchVisible(request, actual)) throw new Error("m2c_batch_reread_unconfirmed");
      } else if (request.operation === "correctResponse") {
        const saved = actual.find((v) => v.id === request.artifactId);
        const prompt = saved?.payload as ReflectionPrompt | undefined;
        const original = responseCorrection?.target.payload as ReflectionPrompt | undefined;
        if (!saved || !prompt || !canCorrectResponse(saved, current, actual) || saved.revisionId === request.expectedArtifactRevisionId
          || prompt.response !== request.text?.trim() || prompt.question !== original?.question
          || JSON.stringify(prompt.promptProvenance) !== JSON.stringify(original?.promptProvenance)) {
          throw new Error("m2c_stale_revision_preserved");
        }
      }
      if (mounted.current) {
        savedClueOperation.current = !("items" in request) && ["candidate", "correct", "confirm", "reject"].includes(request.operation);
        setRows(actual); setFrozen(null); setInspected(null); setDebugPhase(undefined); setState("saved"); onLock(false);
        if (!("items" in request)) {
          if (request.operation === "correct") setCorrection(null);
          if (request.operation === "correctResponse") setResponseCorrection(null);
          if (request.operation === "answer" || request.operation === "skip") {
            setResponses((previous) => {
              const { [request.artifactId]: _discarded, ...otherDrafts } = previous;
              return otherDrafts;
            });
          }
        }
      }
    } catch (error) {
      // Keep the immutable request and suppress arbitrary backend text.
      if (mounted.current) setState(String(error).includes("stale") || String(error).includes("identity_conflict") ? "conflict" : "failed");
    } finally { busy.current = false; }
  };
  const submit = async (operation: Exclude<ArtifactRequest["operation"], "question">, target: ArtifactSnapshot | null, text: string | null = null) => {
    if (busy.current || locked || (responseCorrection && operation !== "correctResponse")) return;
    if (operation === "correctResponse" && (!target || !canCorrectResponse(target, source, rows)
      || !text?.trim() || text.trim() === (target.payload as ReflectionPrompt).response?.trim())) return;
    // Bind only exact IDs cited by this prompt; another eligible artifact must
    // never be silently substituted while saving a response.
    const prompt = target?.kind === "reflection" ? target.payload as ReflectionPrompt : null;
    const selected = prompt ? evidence.filter((v) => prompt.sourceEvidenceIds.includes(v.id)) : [];
    if (prompt && selected.length !== prompt.sourceEvidenceIds.length) { setState("conflict"); onLock(true); return; }
    const latest = rows.reduce((time, v) => v.payload && v.payload.updatedAt > time ? v.payload.updatedAt : time, source.entry.updatedAt);
    // Fence the asynchronous hashing window as well as the IPC window.
    busy.current = true; onLock(true); setState("working");
    let request: ArtifactRequest;
    try {
      const occurredAt = nextTimestamp(latest);
      const response = (operation === "answer" || operation === "correctResponse") && prompt ? answerReflectionPrompt(prompt, text ?? "", occurredAt).response! : text;
      request = await artifactRequest(source, operation, locale, target, response,
        selected.map((v) => ({ artifactId: v.id, revisionId: v.revisionId! })), occurredAt);
    } catch { busy.current = false; setState("failed"); return; }
    busy.current = false;
    await execute(request);
  };
  const generate = async () => {
    if (busy.current || otherLocked) return;
    // Lock before the first await, including hashing. The frozen request never contains drafts.
    busy.current = true; onLock(true); setState("working");
    let request: QuestionBatchRequest;
    try {
      const refs = localQuestionEvidence(source, rows, locale);
      if (refs.length === 0) { setState("idle"); onLock(false); return; }
      const latest = rows.reduce((time, row) => row.payload && row.payload.updatedAt > time ? row.payload.updatedAt : time, source.entry.updatedAt);
      request = await questionBatchRequest(source, rows, refs, locale, nextTimestamp(latest));
    } catch { setState("failed"); return; }
    finally { busy.current = false; }
    await execute(request);
  };
  const inspectCurrent = async () => {
    if (busy.current) return;
    busy.current = true; setState("working");
    try {
      const current = await store.get(source.entry.id);
      const actual = await store.artifacts(source.entry.id);
      if (mounted.current) { setInspected({ source: current, rows: actual }); setState("conflict"); }
    } catch { if (mounted.current) setState("failed"); }
    finally { busy.current = false; }
    // Inspection is not reconciliation, success, a new request, or draft discard.
  };
  const hasQuestions = rows.some((row) => row.kind === "reflection" && row.lifecycleState === "active");
  const modelPreviews: ModelPreview[] = model ? evidence.map((row) => ({
    sourceId: source.entry.id, sourceRevisionId: source.revisionId,
    clueId: row.id, clueRevisionId: row.revisionId!, text: (row.payload as EvidenceCandidate).text, locale,
  })) : [];
  return <section data-testid="m2c-journey" data-artifact-state={state} className="android-m2c__journey">
    <h2 id="m2c-reflection-start" tabIndex={-1}>{guided && !hasQuestions ? c.clueReview : c.title}</h2>
    {guided && state === "loading" ? <p role="status">{c.loadingReflection}</p> : null}
    {state === "working" || (state === "saved" && (!guided || !savedClueOperation.current)) || state === "failed" || state === "conflict" ? <p role="status" data-testid="m2c-artifact-feedback">{state === "working" ? c.working : state === "saved" ? c.saved : state === "conflict" ? c.conflict : c.failed}</p> : null}
    {(state === "failed" || (state === "conflict" && frozen && !("items" in frozen))) && frozen ? <button data-testid="m2c-artifact-reconcile" disabled={busy.current} onClick={() => void execute(frozen)}>{c.retry}</button> : null}
    {(state === "failed" || state === "conflict") && responseCorrection ? <button data-testid="m2c-inspect-response" disabled={busy.current} onClick={() => void inspectCurrent()}>{c.inspectResponse}</button> : null}
    {state === "failed" || state === "conflict" ? <button data-testid="m2c-artifact-reopen" disabled={busy.current} onClick={() => { onLock(false); setFrozen(null); setResponses({}); setCorrection(null); setResponseCorrection(null); setInspected(null); onReopen(); }}>{responseCorrection ? c.discardResponseDraft : Object.values(responses).some((draft) => draft.trim()) ? c.discardDrafts : c.reopen}</button> : null}
    {inspected ? <aside data-testid="m2c-inspected-response"><h3>{c.currentSavedState}</h3>
      {inspected.source ? <div className="android-m2c__authored-block"><h4>{c.yourMoment}</h4><p>{inspected.source.entry.body}</p></div> : <p>{c.sourceUnavailable}</p>}
      {inspected.rows.filter((v) => v.id === responseCorrection?.target.id).map((v) => {
        const p = v.payload as ReflectionPrompt | null;
        return <div key={v.id}>
          {p?.question ? <div className="android-m2c__question-block"><QuestionOrigin provenance={p.promptProvenance} locale={locale} /><p>{p.question}</p></div> : null}
          {p?.response != null ? <div className="android-m2c__authored-block"><h4>{p.responseProvenance?.origin === "user" ? c.response : c.unknownOrigin}</h4><p>{p.response}</p></div> : <p>{c.sourceUnavailable}</p>}
          <details className="android-m2c__secondary-details"><summary>{c.technical}</summary><pre>{JSON.stringify({ revisionId: v.revisionId, lifecycleState: v.lifecycleState, eligibilityState: v.eligibilityState })}</pre></details>
        </div>;
      })}
    </aside> : null}
    {guided ? supersededReflectionNotice(rows, locale) : null}
    <RejectedCandidateHistory rows={rows} locale={locale} />
    {(!guided || (state !== "loading" && state !== "failed" && state !== "conflict")) && rows.every((v) => v.kind !== "evidence" || v.lifecycleState !== "active") ? <>
      {!gate.allowObservation ? <p>{c.sparse}</p> : null}
      <button className={guided ? "android-m2c__journey-primary" : undefined} data-testid="m2c-generate-candidate" disabled={otherLocked || !gate.allowObservation} onClick={() => void submit("candidate", null)}>{guided ? c.createDemoClue : candidateGenerationLabel(rows, locale)}</button>
    </> : null}
    {rows.map((row) => {
      const active = row.lifecycleState === "active";
      if (isRejectedCandidate(row)) return null;
      if (guided && isSupersededReflection(row)) return null;
      if (!active) return <article key={row.id} data-artifact-id={row.id} data-lifecycle={row.lifecycleState}><p>{row.reviewState === "rejected" ? c.rejected : c.invalidated}</p></article>;
      if (!row.payload) return null;
      if (row.kind === "evidence") {
        const record = row.payload as EvidenceCandidate;
        const edited = !!record.originalText && record.originalText !== record.text;
        const adopted = row.reviewState === "confirmed";
        const refersToOriginal = guided && (candidateRepeatsSource(record, source) || record.text === source.entry.body.trim());
        const editClue = <button data-testid="m2c-correct" disabled={otherLocked} onClick={() => setCorrection({ id: row.id, text: refersToOriginal ? "" : record.text })}>{refersToOriginal ? c.useExcerpt : guided ? c.editThisClue : c.correct}</button>;
        const useClue = <button className={guided ? "android-m2c__journey-primary" : undefined} data-testid="m2c-confirm" disabled={otherLocked} onClick={() => void submit("confirm", row)}>{refersToOriginal ? c.useOriginal : guided ? c.useThisClue : c.confirm}</button>;
        return <article key={row.id} data-artifact-id={row.id} data-review-state={row.reviewState} className={guided ? "android-m2c__clue-card" : undefined}>
          <div className="android-m2c__review-heading">
            <h3 data-testid={guided ? "m2c-clue-title" : undefined}>{guided ? edited ? c.editedClueTitle : c.clueTitle : c.candidate}</h3>
            <span data-testid={guided ? "m2c-clue-status" : undefined}>{guided ? adopted ? c.clueAdopted : c.clueToReview : adopted ? c.confirmed : c.pending}</span>
          </div>
          {guided ? refersToOriginal ? <p className="android-m2c__scope-note" data-testid="m2c-candidate-scope">{c.originalReference}</p>
          : correction?.id !== row.id ? <p className="android-m2c__clue-text" data-testid="m2c-candidate-text">{record.text}</p> : null : <>
            <CandidateScopeNote locale={locale} />
            <CandidateContent record={record} source={source} reviewed={adopted} locale={locale} />
          </>}
          {row.reviewState === "pending" ? correction?.id === row.id ? <>
            <label htmlFor="m2c-correction">{refersToOriginal ? c.excerptLabel : c.correctionLabel}</label><p id="m2c-correction-help" data-testid="m2c-correction-help" className="android-m2c__secondary">{refersToOriginal ? c.excerptHelp : c.correctionHelp}</p><textarea autoFocus aria-describedby="m2c-correction-help" id="m2c-correction" data-testid="m2c-correction" disabled={otherLocked} value={correction.text} onChange={(e) => setCorrection({ id: row.id, text: e.target.value })} />
            <button data-testid="m2c-save-correction" disabled={otherLocked || !correction.text.trim()} onClick={() => void submit("correct", row, correction.text)}>{c.saveCorrection}</button>
            <button data-testid="m2c-cancel-correction" disabled={otherLocked} onClick={() => setCorrection(null)}>{c.cancel}</button>
          </> : <div className={guided ? "android-m2c__clue-actions" : undefined}>
            {guided ? <>{useClue}{editClue}</> : <>{editClue}{useClue}</>}
            <button data-testid="m2c-reject" disabled={otherLocked} onClick={() => void submit("reject", row)}>{guided ? c.rejectThisClue : c.reject}</button>
          </div> : null}
          <ArtifactDetails provenance={record.provenance} edited={edited} note={guided && adopted ? c.clueAdoptionLimit : undefined} locale={locale} testId="m2c-evidence-provenance" />
        </article>;
      }
      const prompt = row.payload as ReflectionPrompt;
      return <article key={row.id} data-artifact-id={row.id} data-response-state={prompt.status}>
        <div className="android-m2c__question-block">
          <h3>{c.reflection}</h3><QuestionOrigin provenance={prompt.promptProvenance} locale={locale} />
          <p data-testid="m2c-question">{prompt.question}</p>
        </div>
        <div className="android-m2c__authored-block">
          {prompt.status === "suggested" ? <>
            <label htmlFor={`response-${row.id}`}>{c.answerLabel}</label><textarea id={`response-${row.id}`} data-testid="m2c-response-draft" disabled={otherLocked} value={responses[row.id] ?? ""} onChange={(e) => setResponses({ ...responses, [row.id]: e.target.value })} />
            {responses[row.id]?.trim() ? <p data-testid="m2c-response-unsaved">{c.draft}</p> : null}
            <button data-testid="m2c-save-response" disabled={otherLocked || !responses[row.id]?.trim()} onClick={() => void submit("answer", row, responses[row.id])}>{c.saveAnswer}</button>
            <button data-testid="m2c-skip" disabled={otherLocked} onClick={() => void submit("skip", row)}>{c.skip}</button>
          </> : prompt.status === "answered" ? <><h4>{prompt.responseProvenance?.origin === "user" ? (prompt.promptProvenance?.promptVersion === "android-gemini-reflection-v1" ? pilotCopy[locale].answer : c.response) : c.unknownOrigin}</h4><p data-testid="m2c-saved-response">{prompt.response}</p>
            {responseCorrection?.target.id === row.id ? <>
              <label htmlFor={`correct-response-${row.id}`}>{c.editResponseLabel}</label>
              <textarea autoFocus id={`correct-response-${row.id}`} data-testid="m2c-response-correction" readOnly={locked} value={responseCorrection.text} onChange={(e) => { if (!locked) setResponseCorrection({ ...responseCorrection, text: e.target.value }); }} />
              <p data-testid="m2c-response-unsaved">{c.draft}</p>
              <button data-testid="m2c-save-response-correction" disabled={locked || !responseCorrection.text.trim() || responseCorrection.text.trim() === prompt.response?.trim()} onClick={() => void submit("correctResponse", responseCorrection.target, responseCorrection.text)}>{c.saveResponseCorrection}</button>
              <button data-testid="m2c-cancel-response-correction" disabled={locked} onClick={() => { setResponseCorrection(null); setState("idle"); onLock(false); }}>{c.cancel}</button>
            </> : canCorrectResponse(row, source, rows) ? <button data-testid="m2c-edit-response" disabled={otherLocked} onClick={() => { if (busy.current || otherLocked) return; setResponseCorrection({ target: row, text: prompt.response! }); setState("idle"); onLock(true); }}>{c.editResponse}</button> : null}
          </> : <p data-testid="m2c-skipped">{c.skipped}</p>}
        </div>
        <ArtifactDetails provenance={prompt.promptProvenance} response={prompt.responseProvenance} locale={locale} testId="m2c-reflection-provenance" />
      </article>;
    })}
    {connectedPilot && model && openModelSettings && gate.allowReflection ? <PilotModelQuestion locale={locale} model={model} hasKey={hasModelKey}
      previews={modelPreviews} visible={modelVisible} disabled={otherLocked} openSettings={openModelSettings}
      readBack={async ({ ticket, question, answer }) => {
        const current = await store.get(source.entry.id);
        const actual = await store.artifacts(source.entry.id);
        if (current?.revisionId !== source.revisionId) throw "stale_clue";
        if (!actual.some((row) => {
          const prompt = row.payload as ReflectionPrompt | null;
          return row.id === ticket && row.kind === "reflection" && row.lifecycleState === "active" &&
            prompt?.status === "answered" && prompt.question === question && prompt.response === answer.trim() &&
            prompt.promptProvenance?.origin === "ai" && prompt.promptProvenance.provider === "gemini" &&
            prompt.responseProvenance?.origin === "user";
        })) throw "save_unconfirmed";
        if (mounted.current) setRows(actual);
      }} /> : null}
    {evidence.length > 0 ? <button className={guided && !model && !hasQuestions ? "android-m2c__journey-primary" : undefined} data-testid="m2c-generate-question" disabled={otherLocked || !localQuestion(source, rows, locale)} onClick={() => void generate()}>{hasQuestions ? c.addQuestion : c.ask}</button> : null}
    {import.meta.env.VITE_LIFE_OS_ANDROID_M2C_DEBUG_HOOKS === "1" ? <details hidden data-debug-only="artifact"><summary>Disposable artifact debug probes</summary>{(["beforeCommit", "afterCommitBeforeAck", "rollbackAfterProjection"] as DebugPhase[]).map((phase) => <button key={phase} data-artifact-debug-phase={phase} disabled={otherLocked} onClick={() => setDebugPhase(phase)}>{phase}</button>)}</details> : null}
  </section>;
}
