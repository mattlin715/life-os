import { useEffect, useRef, useState, type ReactNode } from "react";
import type { Locale } from "./androidM2CStore";
import { createFeedback, createTrial, destination, publicModelError, type FeedbackPreview, type FeedbackState, type ModelError, type ModelPort, type ModelPreview, type TrialState } from "./pilotModel";
import { modelErrorMessage, pilotCopy } from "./pilotModelCopy";

export function PilotSettings({ locale, model, close, languages, onKeyChanged }: {
  locale: Locale; model?: ModelPort; close: () => void; languages: ReactNode; onKeyChanged: (ready: boolean) => void;
}) {
  const c = pilotCopy[locale];
  const [ready, setReady] = useState(false);
  const [remember, setRemember] = useState(false);
  const [remembered, setRemembered] = useState(false);
  const [busy, setBusy] = useState(false);
  const [entered, setEntered] = useState(false);
  const [error, setError] = useState<ModelError>();
  const heading = useRef<HTMLHeadingElement>(null);
  const password = useRef<HTMLInputElement>(null);
  const active = useRef(false);
  const reentry = useRef(false);
  useEffect(() => {
    active.current = true;
    heading.current?.focus();
    if (model) {
      setBusy(true);
      void (model.keyStatus ? model.keyStatus() : model.hasKey().then((ready) => ({ ready, remembered: false })))
        .then((status) => { if (active.current) { setReady(status.ready); setRemembered(status.remembered); setRemember(status.remembered); onKeyChanged(status.ready); } })
        .catch((error) => { if (active.current) { onKeyChanged(false); setError(publicModelError(error)); } })
        .finally(() => { if (active.current) setBusy(false); });
    }
    return () => { active.current = false; };
  }, [model, onKeyChanged]);

  async function updateKey(clear: boolean) {
    if (!model || busy || reentry.current) return;
    const input = password.current;
    const value = input?.value ?? "";
    if (!clear && !value.trim()) { setError("invalid_key"); return; }
    if (input) input.value = "";
    setEntered(false);
    reentry.current = true;
    setBusy(true); setError(undefined);
    try {
      if (clear) await model.clearKey(); else await model.setKey(value, remember);
      onKeyChanged(!clear);
      if (active.current) { setReady(!clear); setRemembered(!clear && remember); if (clear) setRemember(false); }
    } catch (error) {
      onKeyChanged(false);
      if (active.current) { setReady(false); setError(publicModelError(error)); }
    }
    finally {
      reentry.current = false;
      if (active.current) setBusy(false);
    }
  }
  return <section className="android-m2c__card android-m2c__pilot-settings" data-testid="pilot-settings">
    <button type="button" onClick={close}>{c.back}</button>
    <h2 ref={heading} tabIndex={-1}>{c.settings}</h2>
    <h3>{c.language}</h3>{languages}
    <h3>{c.connection}</h3>
    {model ? <>
      <p>{c.provider}: Gemini</p>
      <label htmlFor="pilot-api-key">{c.key}</label>
      <input id="pilot-api-key" ref={password} type="password" autoComplete="off" autoCapitalize="off" spellCheck={false}
        data-testid="pilot-api-key" aria-describedby="pilot-key-help" disabled={busy}
        onChange={(event) => setEntered(event.currentTarget.value.length > 0)} />
      <p id="pilot-key-help">{c.keyHelp}</p>
      <label><input type="checkbox" data-testid="pilot-remember-key" checked={remember} disabled={busy}
        onChange={(event) => setRemember(event.currentTarget.checked)} />{c.rememberKey}</label>
      <p>{c.rememberHelp}</p>
      {ready ? <p role="status">{remembered ? c.keyRemembered : c.keySet}</p> : null}
      {error ? <p role="status">{modelErrorMessage(locale, error)}</p> : null}
      <div className="android-m2c__clue-actions">
        <button type="button" disabled={busy || !entered} onClick={() => void updateKey(false)}>{remember ? c.saveKey : c.useKey}</button>
        <button type="button" disabled={busy} onClick={() => void updateKey(true)}>{c.clear}</button>
      </div>
    </> : <p>{c.offline}</p>}
  </section>;
}

function Outgoing({ label, text }: { label: string; text: string }) {
  return <div className="android-m2c__authored-block"><h4>{label}</h4><p className="android-m2c__clue-text">{text}</p></div>;
}

function Consent({ locale, disabled, send, cancel, children }: {
  locale: Locale; disabled: boolean; send: (confirmed: boolean) => void; cancel: () => void; children: ReactNode;
}) {
  const c = pilotCopy[locale];
  const [confirmed, setConfirmed] = useState(false);
  return <div>
    <h3>{c.preview}</h3><p>{c.destination}: {destination}</p>
    {children}
    <p>{c.privacy}</p>
    <label><input type="checkbox" checked={confirmed} onChange={(e) => setConfirmed(e.currentTarget.checked)} />{c.consent}</label>
    <button className="android-m2c__journey-primary" disabled={!confirmed || disabled} onClick={() => send(confirmed)}>{c.send}</button>
    <button onClick={cancel}>{c.cancel}</button>
  </div>;
}

export function PilotModelQuestion({ locale, model, hasKey, input, visible, disabled, openSettings, readBack }: {
  locale: Locale; model: ModelPort; hasKey: boolean; input: ModelPreview;
  visible: boolean; disabled: boolean; openSettings: () => void;
  readBack: (result: { ticket: string; question: string; answer: string }) => Promise<void>;
}) {
  const c = pilotCopy[locale];
  const [state, setState] = useState<TrialState>({ step: "idle" });
  const controllerRef = useRef<ReturnType<typeof createTrial> | null>(null);
  if (!controllerRef.current) controllerRef.current = createTrial(model, setState);
  const controller = controllerRef.current;
  const current = useRef({ visible, disabled, input });
  current.current = { visible, disabled, input };
  useEffect(() => () => controller.dispose(), [controller]);
  useEffect(() => { if (!visible || disabled) controller.leave(); }, [controller, visible, disabled]);
  useEffect(() => {
    const onVisibility = () => { if (document.hidden) controller.leave(); };
    document.addEventListener("visibilitychange", onVisibility);
    return () => document.removeEventListener("visibilitychange", onVisibility);
  }, [controller]);
  const isCurrent = (preview: ModelPreview) => {
    const { input, visible, disabled } = current.current;
    return !document.hidden && visible && !disabled && input.sourceId === preview.sourceId &&
      input.sourceRevisionId === preview.sourceRevisionId && input.original === preview.original && input.locale === preview.locale;
  };
  const heading = useRef<HTMLHeadingElement>(null);
  useEffect(() => {
    if (state.step === "preview" || state.step === "result") heading.current?.focus();
  }, [state.step]);
  return <section className="android-m2c__model-question" data-testid="pilot-model-question">
    {state.step === "idle" || state.step === "error" ? <>
      {state.step === "error" ? <p role="status">{modelErrorMessage(locale, state.error)}</p> : null}
      <button className="android-m2c__journey-primary" disabled={disabled} onClick={() => {
        if (!hasKey) openSettings();
        else controller.preview({ sourceId: input.sourceId, sourceRevisionId: input.sourceRevisionId,
          original: input.original, locale: input.locale });
      }}>{c.ask}</button>
      {!hasKey ? <p>{c.configure}</p> : null}
    </> : state.step === "preview" ? <div>
      <h4 className="android-m2c__origin-label" tabIndex={-1} ref={heading}>{c.ask}</h4>
      <Consent locale={locale} disabled={disabled} cancel={() => controller.reset()} send={(confirmed) => void controller.confirm(confirmed, isCurrent)}>
        <Outgoing label={c.original} text={state.preview.original} />
        <p>{c.instruction}</p>
      </Consent>
    </div> : state.step === "sending" ? <>
      <p role="status">{c.waiting}</p><button onClick={() => controller.leave()}>{c.stopWaiting}</button>
    </> : <>
      <div className="android-m2c__question-block"><h3 ref={heading} tabIndex={-1}>{c.questionOrigin}</h3><p>{state.question}</p></div>
      <label htmlFor="pilot-model-answer">{c.answer}</label>
      <textarea id="pilot-model-answer" value={state.answer} readOnly={disabled || state.saving || state.error === "save_unconfirmed"}
        onChange={(e) => controller.answer(e.currentTarget.value)} />
      <p role="status">{state.saving ? c.saving : state.error ? modelErrorMessage(locale, state.error) : c.draft}</p>
      <button className="android-m2c__journey-primary" disabled={state.saving || !state.answer.trim() || disabled}
        onClick={() => void controller.save(readBack)}>{c.save}</button>
      <button disabled={state.saving || state.error === "save_unconfirmed"} onClick={() => controller.reset()}>{c.skip}</button>
    </>}
  </section>;
}

export function PilotModelFeedback({ locale, model, hasKey, input, visible, disabled, openSettings, focusOnMount }: {
  locale: Locale; model: ModelPort; hasKey: boolean; input: FeedbackPreview; focusOnMount: boolean;
  visible: boolean; disabled: boolean; openSettings: () => void;
}) {
  const c = pilotCopy[locale];
  const [state, setState] = useState<FeedbackState>({ step: "idle" });
  const controllerRef = useRef<ReturnType<typeof createFeedback> | null>(null);
  if (!controllerRef.current) controllerRef.current = createFeedback(model, setState);
  const controller = controllerRef.current;
  const current = useRef({ input, visible, disabled });
  current.current = { input, visible, disabled };
  const active = visible && !disabled;
  useEffect(() => () => controller.dispose(), [controller]);
  useEffect(() => { if (!active && (state.step === "preview" || state.step === "sending")) controller.cancel(); }, [active, controller, state.step]);
  useEffect(() => {
    const onVisibility = () => {
      if (document.hidden && (state.step === "preview" || state.step === "sending")) controller.cancel();
    };
    document.addEventListener("visibilitychange", onVisibility);
    return () => document.removeEventListener("visibilitychange", onVisibility);
  }, [controller, state.step]);
  const isCurrent = (preview: FeedbackPreview) => {
    const { input, visible, disabled } = current.current;
    return !document.hidden && visible && !disabled && preview.sourceId === input.sourceId &&
      preview.sourceRevisionId === input.sourceRevisionId && preview.reflectionId === input.reflectionId &&
      preview.reflectionRevisionId === input.reflectionRevisionId && preview.original === input.original &&
      preview.question === input.question && preview.answer === input.answer && preview.locale === input.locale;
  };
  const heading = useRef<HTMLHeadingElement>(null);
  const next = useRef<HTMLButtonElement>(null);
  useEffect(() => { heading.current?.focus(); }, [state.step]);
  useEffect(() => { if (focusOnMount && active) next.current?.focus(); }, [focusOnMount, active]);
  return <section className="android-m2c__model-question" data-testid="pilot-model-feedback">
    {state.step === "idle" || state.step === "error" ? <>
      {state.step === "error" ? <p role="status">{modelErrorMessage(locale, state.error)}</p> : null}
      <button ref={next} className="android-m2c__journey-primary" disabled={disabled} onClick={() => {
        if (hasKey) controller.preview(input); else openSettings();
      }}>{c.seeAngle}</button>
      <p>{c.feedbackOptional}</p>
      {!hasKey ? <p>{c.configure}</p> : null}
    </> : state.step === "preview" ? <>
      <h4 tabIndex={-1} ref={heading}>{c.seeAngle}</h4>
      <Consent locale={locale} disabled={disabled} cancel={() => controller.cancel()} send={(confirmed) => void controller.confirm(confirmed, isCurrent)}>
        <Outgoing label={c.original} text={state.preview.original} />
        <Outgoing label={c.questionOrigin} text={state.preview.question} />
        <Outgoing label={c.answer} text={state.preview.answer} />
        <p>{c.feedbackInstruction}</p>
      </Consent>
    </> : state.step === "sending" ? <>
      <p role="status">{c.feedbackWaiting}</p><button onClick={() => controller.cancel()}>{c.stopWaiting}</button>
    </> : state.step === "result" ? <div className="android-m2c__question-block">
      <h3 tabIndex={-1} ref={heading}>{c.seeAngle}</h3>
      <p>{c.feedbackOrigin} · {c.sessionFeedback}</p>
      {state.result.observation ? <Outgoing label={c.observation} text={state.result.observation} /> : null}
      {state.result.basisQuote ? <Outgoing label={c.basis} text={state.result.basisQuote} /> : null}
      {state.result.interpretation ? <Outgoing label={c.possibility} text={state.result.interpretation} /> : null}
      <Outgoing label={c.uncertainty} text={state.result.uncertainty} />
    </div> : null}
  </section>;
}
