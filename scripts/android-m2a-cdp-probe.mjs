import assert from "node:assert/strict";

const args = new Map();
for (let index = 2; index < process.argv.length; index += 2) args.set(process.argv[index], process.argv[index + 1]);
const port = Number(args.get("--port") ?? "9223");
const action = args.get("--action");
const text = args.get("--text") ?? process.env.LIFE_OS_M2A_PROBE_TEXT ?? "";
const phase = args.get("--phase");
const locale = args.get("--locale");

async function targets() {
  const response = await fetch("http://127.0.0.1:" + port + "/json", {
    signal: AbortSignal.timeout(1_000),
  });
  if (!response.ok) throw new Error("CDP target discovery failed: " + response.status);
  return response.json();
}

class Cdp {
  constructor(url) {
    this.nextId = 1;
    this.pending = new Map();
    this.ready = new Promise((resolve, reject) => {
      this.socket = new WebSocket(url);
      this.socket.onopen = resolve;
      this.socket.onerror = reject;
      this.socket.onmessage = (event) => {
        const message = JSON.parse(event.data);
        if (!message.id) return;
        const waiter = this.pending.get(message.id);
        if (!waiter) return;
        this.pending.delete(message.id);
        if (message.error) waiter.reject(new Error(message.error.message));
        else waiter.resolve(message.result);
      };
    });
  }
  async send(method, params = {}) {
    await this.ready;
    const id = this.nextId++;
    const result = new Promise((resolve, reject) => this.pending.set(id, { resolve, reject }));
    this.socket.send(JSON.stringify({ id, method, params }));
    return result;
  }
  async evaluate(expression) {
    const response = await this.send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
    if (response.exceptionDetails) throw new Error(response.exceptionDetails.text);
    return response.result?.value;
  }
  close() { this.socket.close(); }
}

async function connect() {
  const deadline = Date.now() + 30_000;
  let lastError;
  while (Date.now() < deadline) {
    try {
      const list = await targets();
      const target = list.find((candidate) => candidate.type === "page" && candidate.webSocketDebuggerUrl);
      if (target) return new Cdp(target.webSocketDebuggerUrl);
    } catch (error) { lastError = error; }
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw lastError ?? new Error("No debuggable WebView page appeared.");
}

async function waitFor(cdp, expression, label, timeout = 30_000) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    if (await cdp.evaluate(expression)) return;
    await new Promise((resolve) => setTimeout(resolve, 200));
  }
  const diagnostic = await cdp.evaluate("(() => { const root=document.querySelector('[data-runtime=\\\"android-m2a-disposable\\\"]'); return { storageState: root?.dataset.storageState ?? null, localePreferenceState: root?.dataset.localePreferenceState ?? null, text: document.body?.innerText ?? '' }; })()");
  throw new Error("Timed out waiting for " + label + ": " + JSON.stringify(diagnostic));
}

const setDraft = (value) => "(() => {" +
  "const element=document.querySelector('[data-testid=\"m2a-draft\"]');" +
  "if(!element)return false;" +
  "const setter=Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype,'value').set;" +
  "setter.call(element," + JSON.stringify(value) + ");" +
  "element.dispatchEvent(new Event('input',{bubbles:true}));return true;})()";
const click = (selector) => "(() => {const element=document.querySelector(" +
  JSON.stringify(selector) + ");if(!element)return false;element.click();return true;})()";

const cdp = await connect();
try {
  await cdp.send("Runtime.enable");
  if (action === "verify-blocked") {
    await waitFor(cdp, "document.querySelector('[data-runtime=\"android-m2a-disposable\"]')?.dataset.storageState==='blocked'", "fail-closed storage state");
    const visible = await cdp.evaluate("document.body.innerText");
    assert.match(visible, new RegExp(text));
    process.stdout.write(JSON.stringify({ action, blocked: true, reason: text }));
    process.exitCode = 0;
  } else {
  await waitFor(cdp, "document.querySelector('[data-runtime=\"android-m2a-disposable\"]')?.dataset.storageState==='ready'", "ready app-private storage");

  if (action === "journey") {
    assert.equal(await cdp.evaluate(setDraft(text)), true);
    await waitFor(cdp, "!document.querySelector('[data-testid=\"m2a-save\"]')?.disabled", "enabled save");
    assert.equal(await cdp.evaluate(click('[data-testid="m2a-save"]')), true);
    await cdp.evaluate(click('[data-testid="m2a-save"]'));
    await waitFor(cdp, "document.querySelector('[data-runtime=\"android-m2a-disposable\"]')?.dataset.saveState==='committed'", "commit acknowledgement");
    const texts = await cdp.evaluate("[...document.querySelectorAll('[data-testid=\"m2a-list\"] li > p')].map((node)=>node.textContent)");
    assert.deepEqual(texts.filter((value) => value === text), [text]);
    assert.equal(await cdp.evaluate(click('[data-testid="m2a-list"] li button')), true);
    await waitFor(cdp, "document.querySelector('[data-testid=\"m2a-exact\"] p')?.textContent===" + JSON.stringify(text), "exact reopened text");
    process.stdout.write(JSON.stringify({ action, committed: true, exact: true, count: texts.length }));
  } else if (action === "set-locale" || action === "verify-locale") {
    assert.ok(locale === "en" || locale === "zh-TW" || locale === "ja");
    const selector = '[data-locale="' + locale + '"]';
    if (action === "set-locale") assert.equal(await cdp.evaluate(click(selector)), true);
    const expectedLang = locale === "zh-TW" ? "zh-Hant" : locale;
    await waitFor(
      cdp,
      "document.querySelector(" + JSON.stringify(selector) + ")?.getAttribute('aria-pressed')==='true' && document.documentElement.lang===" + JSON.stringify(expectedLang) + " && document.querySelector('[data-runtime=\"android-m2a-disposable\"]')?.dataset.localePreferenceState==='saved'",
      "persisted locale " + locale,
    );
    process.stdout.write(JSON.stringify({ action, locale, htmlLang: expectedLang }));
  } else if (action === "verify-present" || action === "verify-absent") {
    const texts = await cdp.evaluate("[...document.querySelectorAll('[data-testid=\"m2a-list\"] li > p')].map((node)=>node.textContent)");
    const present = texts.includes(text);
    assert.equal(present, action === "verify-present");
    process.stdout.write(JSON.stringify({ action, present, count: texts.length }));
  } else if (action === "arm-save") {
    assert.ok(phase === "beforeCommit" || phase === "afterCommitBeforeAck");
    const requestId = phase === "beforeCommit"
      ? "m2a_probe_before_commit_0001"
      : "m2a_probe_after_commit_0001";
    const invocation = "window.__m2aNativeProbe=window.__TAURI_INTERNALS__.invoke(" +
      "'m2a_create_experience'," +
      JSON.stringify({ requestId, body: text, debugPhase: phase, debugHoldMs: 30000 }) +
      ");'started'";
    assert.equal(await cdp.evaluate(invocation), "started");
    process.stdout.write(JSON.stringify({ action, phase, saving: true }));
    process.exit(0);
  } else {
    throw new Error("Unknown --action " + action);
  }
  }
} finally {
  cdp.close();
}
