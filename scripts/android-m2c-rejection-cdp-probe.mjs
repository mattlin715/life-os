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

const locale = args.get("--locale");
const labels = {
  en: ["Create a new local-demo candidate", "Earlier candidates were not adopted; their rejected text was removed.", "Not-adopted candidates (optional)"],
  "zh-TW": ["建立新的本機示範候選", "先前的候選未採用；被拒絕版本的文字已清除。", "不採用紀錄（選看）"],
  ja: ["端末内デモの候補を新しく作る", "以前の候補は採用していません。その版の文章は消去済みです。", "採用しなかった候補（任意）"]
}[locale];
const getRows = id => invoke("m2c_artifact_snapshot", {id});
const getSource = id => invoke("m2c_get_experience", {id});
const idle = 'document.querySelector(\'[data-testid="m2c-journey"]\')?.dataset.artifactState==="saved" || document.querySelector(\'[data-testid="m2c-journey"]\')?.dataset.artifactState==="idle"';
try {
  assert.ok(labels, "Only authorized three locales");
  assert.ok(args.get("--metadata"), "Owned evidence path required");
  const body = `Synthetic ${locale} rejection presentation: I paused before replying.\n我保留自己的原文與感受。`;
  stage = "new-source";
  await input('[data-testid="m2c-draft"]', body);
  await click('[data-testid="m2c-save"]');
  await wait(rootState("saved"), "exact new source saved");
  const entries = await invoke("m2c_list_experiences");
  assert.equal(entries.length, 1);
  const source = await getSource(entries[0].id);
  assert.equal(source.entry.body, body);
  await wait('document.querySelector(\'[data-testid="m2c-journey"]\')?.dataset.artifactState==="idle"', "new source journey ready");
  assert.deepEqual(await getRows(source.entry.id), []);
  assert.equal(await cdp.evaluate('!!document.querySelector(\'[data-testid="m2c-rejection-group"]\')'), false);
  const identities = [];
  const snapshots = [];
  for (let round=0; round<3; round++) {
    stage = `generate-distinct-${round+1}`;
    if (round) assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-generate-candidate"]\').textContent'), labels[0]);
    await click('[data-testid="m2c-generate-candidate"]');
    await wait(`(${idle}) && !!document.querySelector('[data-testid="m2c-reject"]') && !document.querySelector('[data-testid="m2c-reject"]').disabled`, "candidate mutation completed");
    let rows = await getRows(source.entry.id);
    const pending = rows.filter(v=>v.lifecycleState==="active");
    assert.equal(pending.length, 1);assert.equal(rows.length, round+1);
    assert.equal(pending[0].kind, "evidence");assert.equal(pending[0].reviewState, "pending");
    assert.equal(pending[0].eligibilityState, "ineligible");
    assert.ok(!identities.includes(pending[0].id));identities.push(pending[0].id);
    assert.equal(pending[0].payload.provenance.origin, "local_mock");
    assert.equal(pending[0].payload.provenance.provider, "mock");
    assert.equal(pending[0].payload.provenance.model, null);
    assert.ok(pending[0].payload.text.includes(body));
    assert.equal(await cdp.evaluate('!!document.querySelector(\'[data-testid="m2c-generate-question"]\')'), false);
    assert.equal(await cdp.evaluate('!!document.querySelector(\'[data-testid="m2c-generate-candidate"]\')'), false);
    assert.deepEqual(await getSource(source.entry.id), source);
    for(const row of rows.filter(v=>v.reviewState==="rejected")) {
      assert.equal(row.payload, null);assert.equal(row.lifecycleState, "content_purged");
      assert.equal(row.eligibilityState, "ineligible");
    }
    snapshots.push(rows);
    if (round===2) break;
    stage = `reject-exact-${round+1}`;
    await click('[data-testid="m2c-reject"]');
    await wait(`(${idle}) && !!document.querySelector('[data-testid="m2c-generate-candidate"]') && !document.querySelector('[data-testid="m2c-generate-candidate"]').disabled`, "rejection completed");
    rows = await getRows(source.entry.id);
    assert.equal(rows.length, round+1);
    assert.ok(rows.every(v=>v.reviewState==="rejected"&&v.lifecycleState==="content_purged"&&v.payload===null&&v.eligibilityState==="ineligible"));
    assert.equal(await cdp.evaluate('document.querySelectorAll(\'[data-testid="m2c-rejection-summary"]\').length'), 1);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-rejection-summary"]\').textContent'), labels[1]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-generate-candidate"]\').textContent'), labels[0]);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-rejection-history"]\').open'), false);
    assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-rejection-history"] summary\').textContent'), `${labels[2]} (${round+1})`);
    assert.equal(await cdp.evaluate('!!document.querySelector(\'[data-testid="m2c-candidate-text"]\')'), false);
    assert.equal(await cdp.evaluate('!!document.querySelector(\'[data-testid="m2c-generate-question"]\')'), false);
    assert.equal(await cdp.evaluate('[...document.querySelectorAll(\'[data-testid="m2c-rejection-history"] article\')].every(e=>!e.checkVisibility())'), true);
    assert.deepEqual(await getSource(source.entry.id), source);
  }
  stage = "optional-metadata-no-mutation";
  const before = await getRows(source.entry.id);
  assert.equal(before.length, 3);
  const rejected = before.filter(v=>v.reviewState==="rejected");
  assert.equal(rejected.length, 2);
  await click('[data-testid="m2c-rejection-history"] > summary');
  assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-rejection-history"]\').open'), true);
  assert.equal(await cdp.evaluate('document.querySelectorAll(\'[data-testid="m2c-rejection-history"] article\').length'), 2);
  for (const row of rejected) {
    const selector = `[data-testid="m2c-rejection-history"] [data-artifact-id="${row.id}"]`;
    assert.equal(await cdp.evaluate(`document.querySelector(${JSON.stringify(selector)}).checkVisibility()`), true);
    await click(selector+' [data-testid="m2c-rejection-metadata"] > summary');
    const rendered = await cdp.evaluate(`JSON.parse(document.querySelector(${JSON.stringify(selector+' pre')}).textContent)`);
    assert.deepEqual(rendered, {id:row.id,kind:row.kind,revisionId:row.revisionId,reviewState:row.reviewState,lifecycleState:row.lifecycleState,eligibilityState:row.eligibilityState});
    assert.deepEqual(Object.keys(rendered).sort(), ['id','kind','revisionId','reviewState','lifecycleState','eligibilityState'].sort());
    await click(selector+' [data-testid="m2c-rejection-metadata"] > summary');
  }
  await click('[data-testid="m2c-rejection-history"] > summary');
  assert.equal(await cdp.evaluate('document.querySelector(\'[data-testid="m2c-rejection-history"]\').open'), false);
  assert.equal(await cdp.evaluate('document.querySelectorAll(\'[data-testid="m2c-rejection-summary"]\').length'), 1);
  assert.deepEqual(await getRows(source.entry.id), before);
  assert.deepEqual(await getSource(source.entry.id), source);
  const png=await cdp.send("Page.captureScreenshot",{format:"png"});
  writeFileSync(args.get("--metadata")+"-collapsed.png",Buffer.from(png.data,"base64"));
  writeFileSync(args.get("--metadata"),JSON.stringify({locale,source,identities,snapshots,final:before,assertions:"Two purged rejected heads, one distinct pending candidate; one summary; optional complete six-field metadata; disclosure no writes"},null,2));
  process.stdout.write(JSON.stringify({passed:true,locale,newCandidates:3,rejected:2,oneSummary:true,disclosureNoMutation:true})+"\n");
} catch(error) {
  writeFileSync(args.get("--metadata")+"-failure.json",JSON.stringify({passed:false,stage,locale,errorClass:error.name},null,2));
  process.stderr.write(JSON.stringify({passed:false,stage,locale,errorClass:error.name})+"\n");
  process.exitCode=1;
} finally { cdp.close(); }
