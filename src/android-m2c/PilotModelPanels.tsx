import { useEffect, useRef, useState, type ReactNode } from "react";
import type { Locale } from "./androidM2CStore";
import { createTrial, destination, publicModelError, type ModelError, type ModelPort, type ModelPreview, type TrialState } from "./pilotModel";
import { modelErrorMessage, pilotCopy } from "./pilotModelCopy";

export function PilotSettings({ locale, model, close, languages, onKeyChanged }: {
  locale: Locale; model?: ModelPort; close: () => void; languages: ReactNode; onKeyChanged: (ready: boolean) => void;
}) {
  const c = pilotCopy[locale];
  const [ready, setReady] = useState(false);
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
      void model.hasKey().then((ready) => { if (active.current) { setReady(ready); onKeyChanged(ready); } })
        .catch((error) => { if (active.current) setError(publicModelError(error)); })
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
      if (clear) await model.clearKey(); else await model.setKey(value);
      onKeyChanged(!clear);
      if (active.current) setReady(!clear);
    } catch (error) { if (active.current) setError(publicModelError(error)); }
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
      {ready ? <p role="status">{c.keySet}</p> : null}
      {error ? <p role="status">{modelErrorMessage(locale, error)}</p> : null}
      <div className="android-m2c__clue-actions">
        <button type="button" disabled={busy || !entered} onClick={() => void updateKey(false)}>{c.useKey}</button>
        <button type="button" disabled={busy || !ready} onClick={() => void updateKey(true)}>{c.clear}</button>
      </div>
    </> : <p>{c.offline}</p>}
  </section>;
}

export function PilotModelQuestion({ locale, model, hasKey, previews, visible, disabled, openSettings, readBack }: {
  locale: Locale; model: ModelPort; hasKey: boolean; previews: readonly ModelPreview[];
  visible: boolean; disabled: boolean; openSettings: () => void;
  readBack: (result: { ticket: string; question: string; answer: string }) => Promise<void>;
}) {
  const c = pilotCopy[locale];
  const [state, setState] = useState<TrialState>({ step: "idle" });
  const [choose, setChoose] = useState(false);
  const [consent, setConsent] = useState(false);
  const controllerRef = useRef<ReturnType<typeof createTrial> | null>(null);
  if (!controllerRef.current) controllerRef.current = createTrial(model, setState);
  const controller = controllerRef.current;
  const current = useRef({ visible, disabled, previews });
  current.current = { visible, disabled, previews };
  const identity = JSON.stringify(previews.map(({ sourceRevisionId, clueRevisionId, clueId }) => [sourceRevisionId, clueRevisionId, clueId]));
  useEffect(() => {
    setConsent(false); setChoose(false); controller.reset();
    return () => controller.dispose();
  }, [controller, identity]);
  useEffect(() => {
    if (!visible) { setChoose(false); setConsent(false); controller.leave(); }
  }, [controller, visible]);
  useEffect(() => {
    const onVisibility = () => {
      if (document.hidden) { setChoose(false); setConsent(false); controller.leave(); }
    };
    document.addEventListener("visibilitychange", onVisibility);
    return () => document.removeEventListener("visibilitychange", onVisibility);
  }, [controller]);

  function select(preview: ModelPreview) {
    setChoose(false); setConsent(false); controller.preview(preview);
  }
  const isCurrent = (preview: ModelPreview) => !document.hidden && current.current.visible && !current.current.disabled &&
    current.current.previews.some((v) => v.sourceId === preview.sourceId && v.sourceRevisionId === preview.sourceRevisionId &&
      v.clueId === preview.clueId && v.clueRevisionId === preview.clueRevisionId && v.text === preview.text && v.locale === preview.locale);
  if (!previews.length) return null;
  return <section className="android-m2c__model-question" data-testid="pilot-model-question">
    {state.step === "idle" || state.step === "error" ? <>
      {state.step === "error" ? <p role="status">{modelErrorMessage(locale, state.error)}</p> : null}
      {!choose ? <button className="android-m2c__journey-primary" disabled={disabled} onClick={() => {
        if (!hasKey) openSettings();
        else if (previews.length === 1) select(previews[0]);
        else setChoose(true);
      }}>{c.ask}</button> : null}
      {!hasKey ? <p>{c.configure}</p> : null}
      {choose ? <div>
        <p>{c.choose}</p>
        {previews.map((preview) => <button key={preview.clueId} disabled={disabled}
          onClick={() => select(preview)}>{preview.text}</button>)}
        <button onClick={() => setChoose(false)}>{c.cancel}</button>
      </div> : null}
    </> : state.step === "preview" ? <div>
      <h3>{c.preview}</h3>
      <p>{c.destination}: {destination}</p>
      <p className="android-m2c__clue-text" data-testid="pilot-outgoing-clue">{state.preview.text}</p>
      <p>{c.instruction}</p>
      <p>{c.privacy}</p>
      <label><input type="checkbox" checked={consent} onChange={(e) => setConsent(e.currentTarget.checked)} />{c.consent}</label>
      <button className="android-m2c__journey-primary" disabled={!consent || disabled}
        onClick={() => void controller.confirm(consent, isCurrent)}>{c.send}</button>
      <button onClick={() => controller.reset()}>{c.cancel}</button>
    </div> : state.step === "sending" ? <>
      <p role="status">{c.waiting}</p>
      <button onClick={() => controller.leave()}>{c.cancel}</button>
    </> : <>
      <div className="android-m2c__question-block"><h3>{c.generated}</h3><p>{state.question}</p></div>
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
