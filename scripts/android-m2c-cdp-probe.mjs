// Disposable debug WebView instrumentation. Never output source text/requests.
import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
const args = new Map();
for (let i = 2; i < process.argv.length; i += 2) args.set(process.argv[i], process.argv[i + 1]);
const port = Number(args.get("--port") ?? "9228");
const action = args.get("--action"); const text = args.get("--text") ?? process.env.LIFE_OS_M2C_PROBE_TEXT ?? "";
const phase = args.get("--phase"); const operation = args.get("--operation");
let stage = "connect";
class Cdp {
  constructor(url) {
    this.id = 0; this.pending = new Map();
    this.ready = new Promise((resolve, reject) => {
      this.socket = new WebSocket(url); this.socket.onopen = resolve; this.socket.onerror = reject;
      this.socket.onmessage = (event) => { const m = JSON.parse(event.data); const p = this.pending.get(m.id);
        if (!p) return; this.pending.delete(m.id); if (m.error) p.reject(new Error("CDP protocol failure")); else p.resolve(m.result); };
    });
  }
  async send(method, params = {}) {
    await this.ready; const id = ++this.id;
    const response = new Promise((resolve, reject) => this.pending.set(id, { resolve, reject }));
    this.socket.send(JSON.stringify({ id, method, params })); return response;
  }
  async evaluate(expression) {
    const r = await this.send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
    if (r.exceptionDetails) throw new Error("Native probe evaluation failed (content suppressed)");
    return r.result?.value;
  }
  close() { this.socket.close(); }
}
let cdp;
const deadline = Date.now() + 120000;
while (!cdp && Date.now() < deadline) {
  try { const targets = await (await fetch("http://127.0.0.1:" + port + "/json", { signal: AbortSignal.timeout(1000) })).json();
    // A cold activity can expose an initial blank WebView. Attach only to the
    // configured Tauri application document, not a stale/placeholder target.
    const page = targets.find((v) => v.type === "page" && v.webSocketDebuggerUrl && /^https?:\/\/tauri\.localhost(?:\/|$)/.test(v.url));
    if (page) {
      const candidate = new Cdp(page.webSocketDebuggerUrl);
      try {
        await Promise.race([candidate.ready, new Promise((_, reject) => setTimeout(() => reject(new Error("Local attach timeout")), 1500))]);
        await candidate.send("Runtime.enable");
        const expected = action === "blocked" ? "blocked" : action === "state" ? null : "ready";
        const observed = await candidate.evaluate("document.querySelector('[data-runtime=\"android-m2c-disposable\"]')?.dataset.storageState ?? null");
        // Android can replace the cold document/context after CDP attachment.
        // Reattach during this read-only readiness stage instead of holding a
        // stale context forever. No lifecycle IPC has been issued here.
        if (expected === null || observed === expected) cdp = candidate;
        else candidate.close();
      } catch { candidate.close(); }
    }
  } catch { /* retry only local connection establishment, never a mutation */ }
  if (!cdp) await new Promise((r) => setTimeout(r, 250));
}
if (!cdp) throw new Error("No debug WebView on exact forwarded port");
const wait = async (expression, label, timeout = 30000) => {
  const end = Date.now() + timeout;
  while (Date.now() < end) {
    try { if (await cdp.evaluate(expression)) return; } catch { /* document execution context may still be starting */ }
    await new Promise((r) => setTimeout(r, 100));
  }
  throw new Error("Native timeout: " + label + " (content suppressed)");
};
const invoke = (command, payload = {}) => cdp.evaluate(`window.__TAURI_INTERNALS__.invoke(${JSON.stringify(command)},${JSON.stringify(payload)})`);
const click = async (selector) => assert.equal(await cdp.evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e||e.disabled)return false;e.click();return true})()`), true);
const input = async (selector, value) => assert.equal(await cdp.evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});if(!e)return false;Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype,'value').set.call(e,${JSON.stringify(value)});e.dispatchEvent(new Event('input',{bubbles:true}));return true})()`), true);
const rootState = (state) => `document.querySelector('[data-runtime="android-m2c-disposable"]')?.dataset.saveState===${JSON.stringify(state)}`;
// Test-only readiness predicate. Observes exact source identity/content/revision
// and enabled controls; it never mutates, substitutes requests or retries writes.
export function exactSourceReadiness(source) {
  const revision = `revision=${source.revisionNumber} · authoredBy=${source.authorship} · id=${source.revisionId} · predecessor=${source.predecessorRevisionId ?? "none"}`;
  return `(()=>{const root=document.querySelector('[data-runtime="android-m2c-disposable"]');const original=document.querySelector('[data-testid="m2c-exact-text"]');const version=document.querySelector('[data-testid="m2c-revision"]');const journey=document.querySelector('[data-testid="m2c-journey"]');const edit=document.querySelector('[data-testid="m2c-edit"]');const fault=document.querySelector('[data-debug-phase="rollbackAfterProjection"]');return root?.dataset.storageState==='ready'&&root.dataset.saveState==='draft'&&!!document.querySelector(${JSON.stringify('[data-experience-id="'+source.entry.id+'"]')})&&original?.textContent===${JSON.stringify(source.entry.body)}&&version?.textContent===${JSON.stringify(revision)}&&journey?.dataset.artifactState==='idle'&&!!edit&&!edit.disabled&&!!fault&&!fault.disabled})()`;
}

const metadata = (s) => ({ id: s.entry.id, revisionId: s.revisionId, revisionNumber: s.revisionNumber, predecessorRevisionId: s.predecessorRevisionId, authorship: s.authorship });
const request = async (s, body) => {
  const occurredAt = new Date(Math.max(Date.now(), Date.parse(s.entry.updatedAt) + 1)).toISOString();
  return cdp.evaluate(`(async()=>{const hash=async(v)=>[...new Uint8Array(await crypto.subtle.digest('SHA-256',new TextEncoder().encode(v)))].map(b=>b.toString(16).padStart(2,'0')).join('');const body=${JSON.stringify(body)};const parts=['life-os/m2c-request-v1',body===null?'delete':'update',${JSON.stringify(s.entry.id)},${JSON.stringify(s.revisionId)},${JSON.stringify(occurredAt)},body===null?'':await hash(body)];return {requestId:'m2c_'+await hash(JSON.stringify(parts)),operation:parts[1],id:parts[2],expectedRevisionId:parts[3],occurredAt:parts[4],body}})()`);
};
const out = (value) => process.stdout.write(JSON.stringify({ action, passed: true, ...value }) + "\n");
const artifactRows = (id) => invoke("m2c_artifact_snapshot", { id });
const artifactReq = async (s, rows, op, target = null, value = null, locale = "en") => {
  const generated = ["candidate", "question"].includes(op);
  const refs = ["question", "answer", "skip"].includes(op) ? rows.filter(v => v.kind === "evidence" && v.reviewState === "confirmed" && v.lifecycleState === "active" && v.eligibilityState === "eligible").slice(0, 1).map(v => ({ artifactId: v.id, revisionId: v.revisionId })) : [];
  const latest = rows.reduce((time,v) => v.payload?.updatedAt > time ? v.payload.updatedAt : time, s.entry.updatedAt);
  const base = { operation: op, sourceId: s.entry.id, expectedSourceRevisionId: s.revisionId, artifactId: generated ? "m2c_" + crypto.randomUUID() : target.id, expectedArtifactRevisionId: generated ? null : target.revisionId, evidence: refs, text: value, locale, occurredAt: new Date(Math.max(Date.now(), Date.parse(latest) + 1)).toISOString() };
  return cdp.evaluate(`(async()=>{const r=${JSON.stringify(base)};const hash=async(v)=>[...new Uint8Array(await crypto.subtle.digest('SHA-256',new TextEncoder().encode(v)))].map(b=>b.toString(16).padStart(2,'0')).join('');const tuple=['life-os/m2c-artifact-request-v1',r.operation,r.sourceId,r.expectedSourceRevisionId,r.artifactId,r.expectedArtifactRevisionId,r.evidence.map(v=>[v.artifactId,v.revisionId]),r.text===null?null:await hash(r.text),r.locale,r.occurredAt];return {...r,requestId:'m2c_'+await hash(JSON.stringify(tuple))}})()`);
};
const mutateArtifact = (r, debugPhase) => invoke("m2c_mutate_artifact", { request: r, debugPhase });
const openJourney = async (id) => {
  const buttons = await cdp.evaluate(`[...document.querySelectorAll('[data-experience-id]')].map(v=>v.dataset.experienceId)`);
  assert.ok(buttons.includes(id)); await click(`[data-experience-id="${id}"] [data-testid="m2c-open"]`);
  await wait("document.querySelector('[data-testid=\"m2c-journey\"]')?.dataset.artifactState==='idle'", "opened journey");
};
async function reflectionProbe(action) {
  stage = action;
  const question = "What stands out when you read this evidence again?";
  if (action === "reflection-journey") {
    const locale = args.get("--locale");
    const body = "  Synthetic self-report: Today I felt uncertain. 誰かが『失敗する』と言った。\n我先停下來，選擇平靜回應。  ";
    await input('[data-testid="m2c-draft"]', body); await click('[data-testid="m2c-save"]');
    await wait(rootState("saved"), "reflection source saved");
    const [entry] = await invoke("m2c_list_experiences"); assert.equal(entry.body, body);
    await wait("document.querySelector('[data-testid=\"m2c-journey\"]')?.dataset.artifactState==='idle'", "reflection initial");
    stage = "source-editor-is-independent";
    const originalSource = await invoke("m2c_get_experience",{id:entry.id});
    const labels = {
      en: ["Edit original text","Your original text","Candidate clue · Local demo","Candidate text — not your original","Correct candidate","Use as a reflection clue","Do not use"],
      "zh-TW": ["編輯原文","你的原文","線索候選・本機示範","候選內容（不是原文）","修正候選內容","採用為反思線索","不採用"],
      ja: ["原文を編集","あなたの原文","手がかりの候補・端末内デモ","候補の文章（原文ではありません）","候補を修正","振り返りの手がかりに採用","採用しない"]
    }[locale];
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-edit"]\').textContent'),labels[0]);
    await click('[data-testid="m2c-edit"]');
    await wait('document.activeElement?.id==="m2c-edit"',"source editor focus");
    assert.equal(await cdp.evaluate('document.querySelector(\'label[for="m2c-edit"]\').textContent'),labels[1]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-source-hint"]\').checkVisibility()'),true);
    const sourcePng=await cdp.send("Page.captureScreenshot",{format:"png"});
    writeFileSync(args.get("--metadata")+"-source-editor.png",Buffer.from(sourcePng.data,"base64"));
    await click('[data-testid="m2c-cancel-edit"]');
    await wait("document.querySelector('[data-testid=\"m2c-journey\"]')?.dataset.artifactState==='idle'","source editor cancelled");
    assert.deepEqual(await invoke("m2c_get_experience",{id:entry.id}),originalSource);
    assert.deepEqual(await artifactRows(entry.id),[]);
    await click('[data-testid="m2c-generate-candidate"]');
    await wait("!!document.querySelector('[data-testid=\"m2c-candidate-text\"]')", "candidate visible");
    let rows = await artifactRows(entry.id); assert.equal(rows[0].reviewState, "pending");
    const candidate = rows[0]; assert.equal(candidate.payload.provenance.origin, "local_mock"); assert.equal(candidate.payload.provenance.provider, "mock"); assert.equal(candidate.payload.provenance.model, null);
    assert.ok(candidate.payload.text.includes(body.trim())); assert.ok(!candidate.payload.text.includes("Directly observable"));
    stage = "source-candidate-meaning-and-scope";
    assert.equal(await cdp.evaluate('document.querySelector("[data-review-state] h3").textContent'),labels[2]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-correct"]\').textContent'),labels[4]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-confirm"]\').textContent'),labels[5]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-reject"]\').textContent'),labels[6]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-candidate-scope"]\').checkVisibility() && document.querySelector(\'[data-testid="m2c-candidate-scope"]\').textContent.includes("AI")'),true);
    assert.equal(await cdp.evaluate('!!document.querySelector(\'[data-testid="m2c-generate-question"]\')'),false);
    assert.deepEqual(await invoke("m2c_get_experience",{id:entry.id}),originalSource);
    stage = "low-burden-pending-presentation";
    // Inspect the actual Android WebView, not a desktop reconstruction. Opening
    // optional presentation must not issue a write or alter persisted records.
    assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-candidate-content\"]')?.open"), false);
    assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-exact-source\"]')"), false);
    assert.equal(await cdp.evaluate(`document.querySelector('[data-testid="m2c-exact-text"]').textContent===${JSON.stringify(body)}`), true);
    assert.equal(await cdp.evaluate(`[...document.querySelectorAll('[data-testid="m2c-exact-text"],[data-testid="m2c-candidate-text"],[data-testid="m2c-open"]')].filter(e=>e.checkVisibility()&&e.textContent.includes(${JSON.stringify(body.trim())})).length`), 1);
    assert.equal(await cdp.evaluate("[...document.querySelectorAll('[data-debug-only]')].length===2 && [...document.querySelectorAll('[data-debug-only]')].every(e=>e.hidden&&!e.checkVisibility())"), true);
    assert.equal(await cdp.evaluate("[...document.querySelectorAll('pre')].every(e=>!e.checkVisibility())"), true);
    const shot = async (name) => {
      await cdp.evaluate("document.querySelector('[data-testid=\"m2c-journey\"]').scrollIntoView({block:'start'});true");
      const png = await cdp.send("Page.captureScreenshot", {format:"png"});
      writeFileSync(args.get("--metadata")+"-"+name+".png",Buffer.from(png.data,"base64"));
    };
    await shot("pending");
    await click('[data-testid="m2c-evidence-provenance"] > summary');
    await click('[data-testid="m2c-evidence-provenance"] [data-testid="m2c-technical-provenance"] > summary');
    assert.equal(await cdp.evaluate("[...document.querySelectorAll('[data-testid=\"m2c-evidence-provenance\"] dd')].map(e=>e.textContent).join('|')"), "local_mock|mock|null");
    assert.equal(await cdp.evaluate("[...document.querySelectorAll('[data-testid=\"m2c-evidence-provenance\"] dd')].every(e=>e.checkVisibility())"), true);
    await shot("optional-origin");
    await click('[data-testid="m2c-evidence-provenance"] [data-testid="m2c-technical-provenance"] > summary');
    await click('[data-testid="m2c-evidence-provenance"] > summary');
    await click('[data-testid="m2c-candidate-content"] > summary');
    assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-candidate-text\"]').checkVisibility()"), true);
    assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-candidate-text\"]').textContent"), candidate.payload.text);
    await click('[data-testid="m2c-candidate-content"] > summary');
    assert.deepEqual(await artifactRows(entry.id), rows);
    await click('[data-testid="m2c-correct"]');
    await wait('document.activeElement?.id==="m2c-correction"',"candidate editor focus");
    assert.equal(await cdp.evaluate('document.querySelector(\'label[for="m2c-correction"]\').textContent'),labels[3]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-correction-help"]\').checkVisibility() && document.querySelector(\'[data-testid="m2c-correction"]\').getAttribute("aria-describedby")==="m2c-correction-help"'),true);
    await shot("candidate-editor");
    await input('[data-testid="m2c-correction"]', "unsaved correction"); await click('[data-testid="m2c-cancel-correction"]');
    assert.deepEqual(await artifactRows(entry.id), rows);
    const corrected = "Synthetic review: this is my self-report, not an independently verified statement. 自分の言葉。";
    await click('[data-testid="m2c-correct"]'); await input('[data-testid="m2c-correction"]', corrected); await click('[data-testid="m2c-save-correction"]');
    await wait(`document.querySelector('[data-testid="m2c-candidate-text"]')?.textContent===${JSON.stringify(corrected)}`, "corrected pending");
    rows = await artifactRows(entry.id); assert.equal(rows[0].reviewState, "pending"); assert.notEqual(rows[0].revisionId, candidate.revisionId);
    assert.equal(rows[0].payload.originalText, candidate.payload.originalText);assert.equal(rows[0].payload.provenance.origin,"local_mock");
    assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-candidate-text\"]').checkVisibility() && !document.querySelector('[data-testid=\"m2c-candidate-content\"]')"), true);
    assert.deepEqual(await invoke("m2c_get_experience",{id:entry.id}),originalSource);
    assert.equal(await cdp.evaluate('!!document.querySelector(\'[data-testid="m2c-generate-question"]\')'),false);
    await shot("corrected-pending");
    await click('[data-testid="m2c-confirm"]'); await wait("!!document.querySelector('[data-review-state=\"confirmed\"]')", "explicit reviewed version");
    assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-candidate-content\"]')?.open"), false);
    assert.deepEqual(await invoke("m2c_get_experience",{id:entry.id}),originalSource);
    await click('[data-testid="m2c-generate-question"]'); await wait("!!document.querySelector('[data-testid=\"m2c-question\"]')", "question visible");
    rows = await artifactRows(entry.id); const prompt = rows.find(v=>v.kind==="reflection").payload;
    assert.equal(prompt.promptProvenance.origin,"local_mock");assert.equal(prompt.promptProvenance.provider,"mock");assert.equal(prompt.promptProvenance.model,null);
    assert.equal(prompt.status,"suggested");assert.equal(prompt.response,undefined);
    const response="  我選擇先停一下。静かに返事を選んだ。Synthetic response, my own meaning.  ";
    await input('[data-testid="m2c-response-draft"]',response);
    assert.equal((await artifactRows(entry.id)).find(v=>v.kind==="reflection").payload.response,undefined);
    await click('[data-testid="m2c-save-response"]');
    await wait(`document.querySelector('[data-testid="m2c-saved-response"]')?.textContent===${JSON.stringify(response.trim())}`,"saved user response");
    const saved=(await artifactRows(entry.id)).find(v=>v.kind==="reflection").payload;
    assert.equal(saved.responseProvenance.origin,"user");assert.equal(saved.promptProvenance.origin,"local_mock");
    assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-response-unsaved\"]')"),false);
    await shot("answered");
    assert.equal(await cdp.evaluate("document.documentElement.scrollWidth<=document.documentElement.clientWidth"), true);
    writeFileSync(args.get("--metadata"),JSON.stringify({id:entry.id,locale,sourceRevision:(await invoke("m2c_get_experience",{id:entry.id})).revisionId,artifactIds:rows.map(v=>v.id)}));
    out({locale,sourceCandidateDistinct:true,truthfulMockScope:true,sourceUnchangedByCandidateReview:true,accessibleEditors:true,explicitReview:true,responseSaved:true,distinctAuthorship:true,onePrimarySource:true,optionalExactContent:true,optionalExactProvenance:true,hiddenTestProbes:true,correctedPendingVisible:true,noHorizontalOverflow:true});
  } else if(action==="reflection-reopened") {
    const meta=JSON.parse(readFileSync(args.get("--metadata"),"utf8"));await openJourney(meta.id);
    const rows=await artifactRows(meta.id);assert.equal(rows.find(v=>v.kind==="evidence").reviewState,"confirmed");
    const prompt=rows.find(v=>v.kind==="reflection").payload;assert.equal(prompt.status,"answered");assert.ok(prompt.response);assert.equal(prompt.responseProvenance.origin,"user");
    assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-saved-response\"]').textContent"),prompt.response);out({reopened:true});
  } else if(action==="reflection-source-edit-delete") {
    const meta=JSON.parse(readFileSync(args.get("--metadata"),"utf8"));
    const s=await invoke("m2c_get_experience",{id:meta.id});const correction=await request(s,"Source correction: synthetic revised self-report. 我改了來源原文。背景也不同了。");
    await invoke("m2c_mutate_experience",{request:correction});
    const rows=await artifactRows(meta.id);assert.ok(rows.every(v=>v.lifecycleState==="invalidated"&&v.eligibilityState==="ineligible"));
    if (await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-close\"]')")) await click('[data-testid="m2c-close"]');
    await openJourney(meta.id);
    assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-saved-response\"],[data-testid=\"m2c-generate-question\"]')"),false);
    const before=await artifactRows(meta.id);await click('[data-testid="m2c-delete"]');await click('[data-testid="m2c-cancel-delete"]');assert.deepEqual(await artifactRows(meta.id),before);
    await click('[data-testid="m2c-delete"]');await click('[data-testid="m2c-confirm-delete"]');await wait(rootState("deleted"),"dependent parent deleted");
    assert.equal(await invoke("m2c_get_experience",{id:meta.id}),null);assert.ok((await artifactRows(meta.id)).every(v=>v.payload===null));out({sourceInvalidation:true,parentPurge:true,cancelNoMutation:true});
  } else if(action==="reflection-reject-skip") {
    const id="m2c-reject-skip-001";await invoke("m2c_create_experience",{id,text:"Synthetic context: today I paused before replying and recorded my own thoughts."});
    let s=await invoke("m2c_get_experience",{id});let r=await artifactReq(s,[],"candidate");await mutateArtifact(r);let rows=await artifactRows(id);
    const reject=await artifactReq(s,rows,"reject",rows[0]);await mutateArtifact(reject);assert.equal((await mutateArtifact(reject)).acknowledgement,"alreadyCommitted");rows=await artifactRows(id);
    assert.equal(rows[0].payload,null);assert.equal(rows[0].reviewState,"rejected");
    const rejectedQuestion=await artifactReq(s,rows,"question",null,question);assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2c_mutate_artifact',{request:${JSON.stringify(rejectedQuestion)}}).then(()=>false,()=>true)`),true);
    r=await artifactReq(s,rows,"candidate");await mutateArtifact(r);rows=await artifactRows(id);const active=rows.find(v=>v.lifecycleState==="active");await mutateArtifact(await artifactReq(s,rows,"confirm",active));rows=await artifactRows(id);
    await mutateArtifact(await artifactReq(s,rows,"question",null,question));rows=await artifactRows(id);
    const prompt=rows.find(v=>v.kind==="reflection");const before=structuredClone(rows);
    assert.deepEqual(await artifactRows(id),before);await mutateArtifact(await artifactReq(s,rows,"skip",prompt));
    rows=await artifactRows(id);const skipped=rows.find(v=>v.kind==="reflection").payload;assert.equal(skipped.status,"skipped");assert.equal(skipped.response,undefined);assert.equal(skipped.responseProvenance,undefined);out({rejectExcluded:true,skipNoResponse:true,noActionNoMutation:true});
  } else if(action==="reflection-protocol") {
    const id="m2c-artifact-protocol-001";await invoke("m2c_create_experience",{id,text:"Synthetic exact-revision fixture: I felt unsure, then paused before replying."});
    let s=await invoke("m2c_get_experience",{id});let rows=[];
    const candidate=await artifactReq(s,rows,"candidate");await mutateArtifact(candidate);assert.equal((await mutateArtifact(candidate)).acknowledgement,"alreadyCommitted");rows=await artifactRows(id);
    const stale=await artifactReq(s,rows,"confirm",rows[0]);const correction=await artifactReq(s,rows,"correct",rows[0],"My corrected synthetic words, still pending explicit review.");
    const before=await artifactRows(id);assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2c_mutate_artifact',{request:${JSON.stringify(correction)},debugPhase:'rollbackAfterProjection'}).then(()=>false,()=>true)`),true);assert.deepEqual(await artifactRows(id),before);
    await mutateArtifact(correction);const refused=async(r)=>assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2c_mutate_artifact',{request:${JSON.stringify(r)}}).then(()=>false,()=>true)`),true);
    await refused(stale);rows=await artifactRows(id);await mutateArtifact(await artifactReq(s,rows,"confirm",rows[0]));rows=await artifactRows(id);
    await refused(await artifactReq(s,rows,"correct",rows[0],"Confirmed correction is outside this slice."));
    const foreignId="m2c-native-foreign-source";
    await invoke("m2c_create_experience",{id:foreignId,text:"Another synthetic source; its Evidence cannot be silently substituted."});
    const foreign=await invoke("m2c_get_experience",{id:foreignId});
    await refused(await artifactReq(foreign,rows,"question",null,question));
    assert.deepEqual(await artifactRows(foreignId),[]);
    const q=await artifactReq(s,rows,"question",null,question);await mutateArtifact(q);assert.equal((await mutateArtifact(q)).acknowledgement,"alreadyCommitted");rows=await artifactRows(id);
    const answer=await artifactReq(s,rows,"answer",rows.find(v=>v.kind==="reflection"),"My own synthetic response.");const unchanged=await artifactRows(id);
    assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2c_mutate_artifact',{request:${JSON.stringify(answer)},debugPhase:'rollbackAfterProjection'}).then(()=>false,()=>true)`),true);assert.deepEqual(await artifactRows(id),unchanged);
    await mutateArtifact(answer);assert.equal((await mutateArtifact(answer)).acknowledgement,"alreadyCommitted");
    await refused({...answer,text:"tampered immutable request"});
    await invoke("m2c_mutate_experience",{request:await request(s,"A revised source that invalidates previous reviewed evidence. New synthetic context.")});
    await refused(await artifactReq(s,rows,"question",null,question));
    assert.ok((await artifactRows(id)).every(v=>v.lifecycleState==="invalidated"));out({duplicateReconciled:true,staleRefused:true,rollbackPreserved:true,identityTamperRefused:true,foreignRefused:true,confirmedCorrectionRefused:true});
  } else if(action==="reflection-arm") {
    assert.ok(["candidate","answer"].includes(operation));assert.ok(["beforeCommit","afterCommitBeforeAck"].includes(phase));
    const id=`m2c-artifact-fault-${operation}-${phase}`;
    await invoke("m2c_create_experience",{id,text:"Synthetic fault fixture: I paused today before replying, then wrote my own account."});
    const s=await invoke("m2c_get_experience",{id});let rows=[];
    if(operation==="answer") {
      await mutateArtifact(await artifactReq(s,rows,"candidate"));rows=await artifactRows(id);
      await mutateArtifact(await artifactReq(s,rows,"confirm",rows[0]));rows=await artifactRows(id);
      await mutateArtifact(await artifactReq(s,rows,"question",null,question));rows=await artifactRows(id);
    }
    const r=await artifactReq(s,rows,operation,operation==="answer"?rows.find(v=>v.kind==="reflection"):null,operation==="answer"?"Synthetic uncertain response, user authored.":null);
    const {text:content,...safe}=r;writeFileSync(args.get("--metadata"),JSON.stringify(safe));
    await cdp.evaluate(`window.__m2cArtifactArmed=window.__TAURI_INTERNALS__.invoke('m2c_mutate_artifact',{request:${JSON.stringify(r)},debugPhase:${JSON.stringify(phase)}}).catch(()=>undefined);true`);
    out({operation,phase,armed:true});
  } else if(action==="reflection-retry") {
    const safe=JSON.parse(readFileSync(args.get("--metadata"),"utf8"));const r={...safe,text:safe.operation==="answer"?"Synthetic uncertain response, user authored.":null};
    const before=await artifactRows(r.sourceId);const committed=args.get("--committed")==="true";
    if(safe.operation==="candidate")assert.equal(before.length,committed?1:0);
    else assert.equal(before.find(v=>v.kind==="reflection").payload.status,committed?"answered":"suggested");
    const ack=await mutateArtifact(r);assert.equal(ack.acknowledgement,committed?"alreadyCommitted":"committed");
    assert.equal((await mutateArtifact(r)).acknowledgement,"alreadyCommitted");
    const actual=await artifactRows(r.sourceId);if(safe.operation==="candidate")assert.equal(actual.length,1);else assert.equal(actual.find(v=>v.kind==="reflection").payload.responseProvenance.origin,"user");
    out({operation:safe.operation,exactRequestReconciled:true,duplicateNoWrite:true});
  } else throw new Error("Unknown Reflection probe");
}
try {
  stage = "document-readiness";
  if (action === "state") {
    const state = await cdp.evaluate("document.querySelector('[data-runtime=\"android-m2c-disposable\"]')?.dataset.storageState ?? 'missing'");
    const code = await cdp.evaluate("window.__TAURI_INTERNALS__.invoke('m2c_storage_status').then(()=> 'ready',e=>typeof e==='string'&&/^m2[ab]_[a-z_]+$/.test(e)?e:'suppressed')");
    out({ storageState: state, storageCode: code });
  } else if (action === "blocked") {
    await wait("document.querySelector('[data-runtime=\"android-m2c-disposable\"]')?.dataset.storageState==='blocked'", "fail-closed state"); out({ blocked: true });
  } else {
    // Cold Android/WebView startup can outlast the shorter mutation timeout.
    // This only waits for an initial read-only state; it never retries writes.
    await wait("document.querySelector('[data-runtime=\"android-m2c-disposable\"]')?.dataset.storageState==='ready'", "ready state", 120000);
    if (action === "ready") out({ ready: true });
    else if (action === "status-review") {
      const locale = args.get("--locale");
      const expected = {
        en: ["New moment — not saved", "Changes — not saved", "Moment saved and reopened", "Changes saved and reopened", "Moment deleted; no active text remains", "This revision changed. Reopen the current record before editing again.", "Outcome not confirmed. Do not submit a new request; reconcile the same request or reopen the current record."],
        "zh-TW": ["新片刻尚未儲存", "編輯內容尚未儲存", "這個片刻已儲存並重新讀取確認", "修改已儲存並重新讀取確認", "這個片刻已刪除；不再有可開啟的文字", "這個版本已變更。請先重新開啟目前的紀錄，再編輯。", "結果尚未確認。請勿另送新請求；請核對同一請求，或重新開啟目前的紀錄。"],
        ja: ["新しい瞬間はまだ保存していません", "変更はまだ保存していません", "この瞬間を保存し、読み直して確認しました", "変更を保存し、読み直して確認しました", "この瞬間を削除しました。開ける文章は残っていません", "この版は変更されています。現在の記録を開き直してから編集してください。", "結果を確認できていません。新しいリクエストは送らず、同じリクエストの結果を確認するか、現在の記録を開き直してください。"],
      }[locale]; assert.ok(expected);
      stage = "status-locale";
      await click(`[data-locale="${locale}"]`);
      await wait(`document.querySelector('[data-runtime="android-m2c-disposable"]')?.dataset.localePreferenceState==='saved'`, "status locale");
      const noDraftWarning = async () => assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-new-unsaved\"],[data-testid=\"m2c-edit-unsaved\"]')"), false);
      const feedback = async (text) => assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-state\"]')?.textContent"), text);
      await noDraftWarning(); assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-state\"]')"), false);
      await input('[data-testid="m2c-draft"]', "  \n "); await noDraftWarning();
      stage = "status-new-draft";
      await input('[data-testid="m2c-draft"]', "synthetic status fixture");
      await wait("!!document.querySelector('[data-testid=\"m2c-new-unsaved\"]')", "new draft warning");
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-new-unsaved\"]').textContent"), expected[0]);
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-new-unsaved\"]').closest('section')===document.querySelector('[data-testid=\"m2c-draft\"]').closest('section')"), true);
      await click('[data-testid="m2c-save"]'); await wait(rootState("saved"), "status create"); await feedback(expected[2]); await noDraftWarning();
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-state\"]').nextElementSibling.contains(document.querySelector('[data-testid=\"m2c-draft\"]'))"), true);
      const id = (await invoke("m2c_list_experiences"))[0].id;
      stage = "status-open-edit-cancel";
      const openedSource = await invoke("m2c_get_experience", { id });
      await click('[data-testid="m2c-close"]'); await click('[data-testid="m2c-open"]'); await wait(rootState("draft"), "opened record"); await noDraftWarning();
      await wait(exactSourceReadiness(openedSource), "exact source ready after reopen", 30000);
      assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-state\"]')"), false);
      await click('[data-testid="m2c-edit"]'); await noDraftWarning();
      await input('[data-testid="m2c-edit-text"]', "synthetic changed status");
      await wait("!!document.querySelector('[data-testid=\"m2c-edit-unsaved\"]')", "changed edit warning");
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-edit-unsaved\"]').textContent"), expected[1]);
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-edit-unsaved\"]').closest('[data-testid=\"m2c-exact\"]')!==null"), true);
      await click('[data-testid="m2c-cancel-edit"]'); await noDraftWarning();
      assert.equal((await invoke("m2c_get_experience", { id })).revisionNumber, 1);
      await click('[data-testid="m2c-edit"]'); await input('[data-testid="m2c-edit-text"]', "synthetic changed status");
      await click('[data-testid="m2c-save-changes"]'); await wait(rootState("saved"), "status edit"); await feedback(expected[3]); await noDraftWarning();
      stage = "status-delete";
      await click('[data-testid="m2c-delete"]'); await click('[data-testid="m2c-cancel-delete"]'); await feedback(expected[3]);
      await click('[data-testid="m2c-delete"]'); await click('[data-testid="m2c-confirm-delete"]'); await wait(rootState("deleted"), "status delete"); await feedback(expected[4]); await noDraftWarning();
      assert.equal(await invoke("m2c_get_experience", { id }), null);
      stage = "status-conflict";
      await input('[data-testid="m2c-draft"]', "synthetic conflict fixture"); await click('[data-testid="m2c-save"]'); await wait(rootState("saved"), "conflict fixture");
      const entry = (await invoke("m2c_list_experiences"))[0]; const prior = await invoke("m2c_get_experience", { id: entry.id });
      await invoke("m2c_mutate_experience", { request: await request(prior, "synthetic external revision") });
      await click('[data-testid="m2c-edit"]'); await input('[data-testid="m2c-edit-text"]', "synthetic stale edit"); await click('[data-testid="m2c-save-changes"]');
      await wait(rootState("conflict"), "conflict visible"); await feedback(expected[5]); await noDraftWarning();
      stage = "status-unconfirmed";
      const reopenedSource = await invoke("m2c_get_experience", { id: entry.id });
      await click('[data-testid="m2c-open"]');
      await wait(exactSourceReadiness(reopenedSource), "exact source ready before fault", 30000);
      await click('[data-debug-phase="rollbackAfterProjection"]');
      await click('[data-testid="m2c-edit"]'); await input('[data-testid="m2c-edit-text"]', "synthetic rollback edit"); await click('[data-testid="m2c-save-changes"]');
      await wait(rootState("failed"), "unconfirmed visible"); await feedback(expected[6]); await noDraftWarning();
      assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-retry\"]') && document.querySelector('[data-testid=\"m2c-draft\"]').disabled"), true);
      assert.equal((await invoke("m2c_get_experience", { id: entry.id })).entry.body, "synthetic external revision");
      out({ locale, emptyAndOpenedNoFalseDraft: true, contextualDrafts: true, scopedCreateEditDelete: true, cancelPreserves: true, conflictAndUnconfirmedVisible: true });
    }
    else if (action === "copy-review") {
      await input('[data-testid="m2c-draft"]', "synthetic copy review fixture"); await click('[data-testid="m2c-save"]');
      await wait(rootState("saved"), "copy fixture saved");
      const before = await invoke("m2c_list_experiences"); assert.equal(before.length, 1);
      for (const [locale, revision, build, confirm, caveat] of [
        ["en", "Versions of this moment", "About this test build", "Delete this moment? This will remove its current text and the text of previously saved versions from the app.", "No secure erasure guarantee"],
        ["zh-TW", "這個片刻的版本", "關於此測試版", "要刪除這個片刻嗎？這會刪除它目前的文字，以及先前儲存版本的文字。", "不保證"],
        ["ja", "この瞬間のバージョン", "このテスト版について", "この瞬間を削除しますか？現在の文章と、以前に保存した版の文章がアプリから削除されます。", "保証しません"],
      ]) {
        await click(`[data-locale="${locale}"]`);
        await wait(`document.querySelector('[data-locale="${locale}"]')?.getAttribute('aria-pressed')==='true' && document.querySelector('[data-runtime="android-m2c-disposable"]')?.dataset.localePreferenceState==='saved'`, "copy locale saved");
        assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-revision-details\"] summary').textContent"), revision);
        assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2c-build-details\"] summary').textContent"), build);
        assert.equal(await cdp.evaluate("[...document.querySelectorAll('[data-testid=\"m2c-revision-details\"],[data-testid=\"m2c-build-details\"]')].every(d=>!d.open)"), true);
        assert.equal(await cdp.evaluate(`document.querySelector('[data-testid="m2c-build-details"]').textContent.includes(${JSON.stringify(caveat)})`), true);
        await click('[data-testid="m2c-delete"]');
        assert.equal(await cdp.evaluate("document.querySelector('[role=\"dialog\"] p').textContent"), confirm);
        await click('[data-testid="m2c-cancel-delete"]');
        assert.deepEqual(await invoke("m2c_list_experiences"), before);
      }
      out({ locales: 3, distinctHeadings: true, plainConfirmation: true, optionalCaveats: true, cancelPreserves: true });
    }
    else if (action === "locale") {
      const locale = args.get("--locale"); assert.ok(["en", "ja", "zh-TW"].includes(locale));
      if (args.get("--verify") !== "true") await click(`[data-locale="${locale}"]`);
      await wait(`document.querySelector('[data-locale="${locale}"]')?.getAttribute('aria-pressed')==='true' && document.querySelector('[data-runtime="android-m2c-disposable"]')?.dataset.localePreferenceState==='saved'`, "locale persistence"); out({ locale });
    } else if (action === "create") {
      await input('[data-testid="m2c-draft"]', text); await click('[data-testid="m2c-save"]');
      await cdp.evaluate("document.querySelector('[data-testid=\"m2c-save\"]').click()");
      await wait(rootState("saved"), "create commit and reread");
      const entries = await invoke("m2c_list_experiences"); const matching = entries.filter((e) => e.body === text); assert.equal(matching.length, 1);
      const saved = await invoke("m2c_get_experience", { id: matching[0].id }); assert.equal(saved.entry.body, text);
      if (args.get("--metadata")) writeFileSync(args.get("--metadata"), JSON.stringify(metadata(saved))); out(metadata(saved));
    } else if (["cancel-edit", "edit", "cancel-delete", "delete"].includes(action)) {
      const id = args.get("--id"); const prior = await invoke("m2c_get_experience", { id }); assert.ok(prior);
      await click(`[data-experience-id="${id}"] [data-testid="m2c-open"]`);
      await wait("!!document.querySelector('[data-testid=\"m2c-exact\"]')", "detail open");
      if (action === "edit" || action === "cancel-edit") {
        await click('[data-testid="m2c-edit"]'); await input('[data-testid="m2c-edit-text"]', text);
        if (action === "cancel-edit") { await click('[data-testid="m2c-cancel-edit"]'); const same = await invoke("m2c_get_experience", { id }); assert.deepEqual(same, prior); }
        else { await click('[data-testid="m2c-save-changes"]'); await wait(rootState("saved"), "update verified reread");
          const current = await invoke("m2c_get_experience", { id }); assert.equal(current.entry.body, text); assert.equal(current.entry.id, prior.entry.id);
          assert.equal(current.predecessorRevisionId, prior.revisionId); assert.equal(current.authorship, "user"); assert.equal(current.revisionNumber, prior.revisionNumber + 1); }
      } else {
        await click('[data-testid="m2c-delete"]');
        if (action === "cancel-delete") { await click('[data-testid="m2c-cancel-delete"]'); assert.deepEqual(await invoke("m2c_get_experience", { id }), prior); }
        else { await click('[data-testid="m2c-confirm-delete"]'); await wait(rootState("deleted"), "delete verified reread");
          assert.equal(await invoke("m2c_get_experience", { id }), null); assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2c-exact\"]')"), false);
          assert.equal(await cdp.evaluate(`document.body.innerText.includes(${JSON.stringify(prior.entry.body)})`), false); }
      }
      out({ id });
    } else if (action === "text" || action === "absent") {
      const actual = await invoke("m2c_get_experience", { id: args.get("--id") });
      if (action === "absent") { assert.equal(actual, null); assert.ok(!(await invoke("m2c_list_experiences")).some((e) => e.id === args.get("--id"))); }
      else assert.equal(actual?.entry.body, text); out({ id: args.get("--id") });
    } else if (action === "protocol") {
      const id = "m2c-protocol-001"; const original = "M2C_CONTENT_LEAK_CANARY_original"; const updated = "M2C_CONTENT_LEAK_CANARY_更新\n修正";
      await invoke("m2c_create_experience", { id, text: original }); const prior = await invoke("m2c_get_experience", { id });
      const first = await request(prior, updated); const second = await request(prior, "stale"); const staleDelete = await request(prior, null);
      assert.equal((await invoke("m2c_mutate_experience", { request: first })).acknowledgement, "committed");
      assert.equal((await invoke("m2c_mutate_experience", { request: first })).acknowledgement, "alreadyCommitted");
      const stale = async (r) => assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2c_mutate_experience',{request:${JSON.stringify(r)}}).then(()=>false,e=>e==='m2c_stale_revision_preserved')`), true);
      await stale(second); await stale(staleDelete); let current = await invoke("m2c_get_experience", { id }); assert.equal(current.revisionNumber, 2);
      const same = await request(current, updated); await invoke("m2c_mutate_experience", { request: same }); current = await invoke("m2c_get_experience", { id }); assert.equal(current.revisionNumber, 3);
      for (const body of ["rollback-canary", null]) {
        const rollback = await request(current, body);
        assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2c_mutate_experience',{request:${JSON.stringify(rollback)},debugPhase:'rollbackAfterProjection'}).then(()=>false,e=>e==='m2c_not_committed')`), true);
        assert.deepEqual(await invoke("m2c_get_experience", { id }), current);
      }
      const deletion = await request(current, null); const ack = await invoke("m2c_mutate_experience", { request: deletion }); assert.equal(ack.acknowledgement, "committed");
      assert.ok(!JSON.stringify(ack).includes("CANARY")); assert.equal((await invoke("m2c_mutate_experience", { request: deletion })).acknowledgement, "alreadyCommitted");
      await stale(second); assert.equal((await invoke("m2c_mutate_experience", { request: first })).acknowledgement, "committedNotCurrent");
      assert.equal(await invoke("m2c_get_experience", { id }), null); out({ revisions: 3, duplicateAndStale: true, rollback: true, logicalDelete: true });
    } else if (action === "arm") {
      assert.ok(["beforeCommit", "afterCommitBeforeAck"].includes(phase)); assert.ok(["update", "delete"].includes(operation));
      const id = `m2c-fault-${operation}-${phase}`; await invoke("m2c_create_experience", { id, text: "fault original synthetic" });
      const s = await invoke("m2c_get_experience", { id }); const r = await request(s, operation === "update" ? text : null);
      const { body, ...safe } = r; writeFileSync(args.get("--metadata"), JSON.stringify(safe));
      await cdp.evaluate(`window.__m2cArmed=window.__TAURI_INTERNALS__.invoke('m2c_mutate_experience',{request:${JSON.stringify(r)},debugPhase:${JSON.stringify(phase)}}).catch(()=>undefined);true`);
      out({ operation, phase, armed: true, id });
    } else if (action === "retry" || action === "retry-state") {
      stage = "read-frozen-request";
      const safe = JSON.parse(readFileSync(args.get("--metadata"), "utf8")); const r = { ...safe, body: safe.operation === "delete" ? null : text };
      stage = "read-current-snapshot";
      const actual = await invoke("m2c_get_experience", { id: safe.id });
      if (action === "retry-state") {
        out({ id: safe.id, actualRevision: actual?.revisionNumber ?? null, textMatches: actual?.entry.body === text, expectedPredecessorMatches: actual?.revisionId === safe.expectedRevisionId });
      } else {
      stage = "classify-native-pre-or-post-state";
      if (args.get("--committed") === "true") {
        if (safe.operation === "delete") assert.equal(actual, null); else { assert.equal(actual?.entry.body, text); assert.equal(actual.revisionNumber, 2); }
      } else assert.equal(actual?.revisionNumber, 1);
      const ack = await invoke("m2c_mutate_experience", { request: r });
      stage = "classify-exact-request-ack";
      assert.equal(ack.acknowledgement, args.get("--committed") === "true" ? "alreadyCommitted" : "committed");
      if (safe.operation === "delete") assert.equal(await invoke("m2c_get_experience", { id: safe.id }), null);
      else assert.equal((await invoke("m2c_get_experience", { id: safe.id })).revisionNumber, 2);
      out({ operation: safe.operation, exactIdentityReconciled: true });
      }
    } else if (action.startsWith("reflection-")) { await reflectionProbe(action); }
    else throw new Error("Unknown bounded native probe action");
  }
} catch {
  process.stderr.write("Native probe failed: " + action + " at " + stage + " (all content suppressed)\n");
  process.exitCode = 1;
} finally { cdp.close(); }
