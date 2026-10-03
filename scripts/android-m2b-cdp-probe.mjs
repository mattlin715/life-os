// Disposable debug WebView instrumentation. Never output source text/requests.
import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
const args = new Map();
for (let i = 2; i < process.argv.length; i += 2) args.set(process.argv[i], process.argv[i + 1]);
const port = Number(args.get("--port") ?? "9226");
const action = args.get("--action"); const text = args.get("--text") ?? process.env.LIFE_OS_M2B_PROBE_TEXT ?? "";
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
        const observed = await candidate.evaluate("document.querySelector('[data-runtime=\"android-m2b-disposable\"]')?.dataset.storageState ?? null");
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
const rootState = (state) => `document.querySelector('[data-runtime="android-m2b-disposable"]')?.dataset.saveState===${JSON.stringify(state)}`;
const metadata = (s) => ({ id: s.entry.id, revisionId: s.revisionId, revisionNumber: s.revisionNumber, predecessorRevisionId: s.predecessorRevisionId, authorship: s.authorship });
const request = async (s, body) => {
  const occurredAt = new Date(Math.max(Date.now(), Date.parse(s.entry.updatedAt) + 1)).toISOString();
  return cdp.evaluate(`(async()=>{const hash=async(v)=>[...new Uint8Array(await crypto.subtle.digest('SHA-256',new TextEncoder().encode(v)))].map(b=>b.toString(16).padStart(2,'0')).join('');const body=${JSON.stringify(body)};const parts=['life-os/m2b-request-v1',body===null?'delete':'update',${JSON.stringify(s.entry.id)},${JSON.stringify(s.revisionId)},${JSON.stringify(occurredAt)},body===null?'':await hash(body)];return {requestId:'m2b_'+await hash(JSON.stringify(parts)),operation:parts[1],id:parts[2],expectedRevisionId:parts[3],occurredAt:parts[4],body}})()`);
};
const out = (value) => process.stdout.write(JSON.stringify({ action, passed: true, ...value }) + "\n");
try {
  stage = "document-readiness";
  if (action === "state") {
    const state = await cdp.evaluate("document.querySelector('[data-runtime=\"android-m2b-disposable\"]')?.dataset.storageState ?? 'missing'");
    const code = await cdp.evaluate("window.__TAURI_INTERNALS__.invoke('m2b_storage_status').then(()=> 'ready',e=>typeof e==='string'&&/^m2[ab]_[a-z_]+$/.test(e)?e:'suppressed')");
    out({ storageState: state, storageCode: code });
  } else if (action === "blocked") {
    await wait("document.querySelector('[data-runtime=\"android-m2b-disposable\"]')?.dataset.storageState==='blocked'", "fail-closed state"); out({ blocked: true });
  } else {
    // Cold Android/WebView startup can outlast the shorter mutation timeout.
    // This only waits for an initial read-only state; it never retries writes.
    await wait("document.querySelector('[data-runtime=\"android-m2b-disposable\"]')?.dataset.storageState==='ready'", "ready state", 120000);
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
      await wait(`document.querySelector('[data-runtime="android-m2b-disposable"]')?.dataset.localePreferenceState==='saved'`, "status locale");
      const noDraftWarning = async () => assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2b-new-unsaved\"],[data-testid=\"m2b-edit-unsaved\"]')"), false);
      const feedback = async (text) => assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-state\"]')?.textContent"), text);
      await noDraftWarning(); assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2b-state\"]')"), false);
      await input('[data-testid="m2b-draft"]', "  \n "); await noDraftWarning();
      stage = "status-new-draft";
      await input('[data-testid="m2b-draft"]', "synthetic status fixture");
      await wait("!!document.querySelector('[data-testid=\"m2b-new-unsaved\"]')", "new draft warning");
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-new-unsaved\"]').textContent"), expected[0]);
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-new-unsaved\"]').closest('section')===document.querySelector('[data-testid=\"m2b-draft\"]').closest('section')"), true);
      await click('[data-testid="m2b-save"]'); await wait(rootState("saved"), "status create"); await feedback(expected[2]); await noDraftWarning();
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-state\"]').nextElementSibling.contains(document.querySelector('[data-testid=\"m2b-draft\"]'))"), true);
      const id = (await invoke("m2b_list_experiences"))[0].id;
      stage = "status-open-edit-cancel";
      await click('[data-testid="m2b-close"]'); await click('[data-testid="m2b-open"]'); await wait(rootState("draft"), "opened record"); await noDraftWarning();
      assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2b-state\"]')"), false);
      await click('[data-testid="m2b-edit"]'); await noDraftWarning();
      await input('[data-testid="m2b-edit-text"]', "synthetic changed status");
      await wait("!!document.querySelector('[data-testid=\"m2b-edit-unsaved\"]')", "changed edit warning");
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-edit-unsaved\"]').textContent"), expected[1]);
      assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-edit-unsaved\"]').closest('[data-testid=\"m2b-exact\"]')!==null"), true);
      await click('[data-testid="m2b-cancel-edit"]'); await noDraftWarning();
      assert.equal((await invoke("m2b_get_experience", { id })).revisionNumber, 1);
      await click('[data-testid="m2b-edit"]'); await input('[data-testid="m2b-edit-text"]', "synthetic changed status");
      await click('[data-testid="m2b-save-changes"]'); await wait(rootState("saved"), "status edit"); await feedback(expected[3]); await noDraftWarning();
      stage = "status-delete";
      await click('[data-testid="m2b-delete"]'); await click('[data-testid="m2b-cancel-delete"]'); await feedback(expected[3]);
      await click('[data-testid="m2b-delete"]'); await click('[data-testid="m2b-confirm-delete"]'); await wait(rootState("deleted"), "status delete"); await feedback(expected[4]); await noDraftWarning();
      assert.equal(await invoke("m2b_get_experience", { id }), null);
      stage = "status-conflict";
      await input('[data-testid="m2b-draft"]', "synthetic conflict fixture"); await click('[data-testid="m2b-save"]'); await wait(rootState("saved"), "conflict fixture");
      const entry = (await invoke("m2b_list_experiences"))[0]; const prior = await invoke("m2b_get_experience", { id: entry.id });
      await invoke("m2b_mutate_experience", { request: await request(prior, "synthetic external revision") });
      await click('[data-testid="m2b-edit"]'); await input('[data-testid="m2b-edit-text"]', "synthetic stale edit"); await click('[data-testid="m2b-save-changes"]');
      await wait(rootState("conflict"), "conflict visible"); await feedback(expected[5]); await noDraftWarning();
      stage = "status-unconfirmed";
      await click('[data-testid="m2b-open"]'); await click('[data-debug-phase="rollbackAfterProjection"]');
      await click('[data-testid="m2b-edit"]'); await input('[data-testid="m2b-edit-text"]', "synthetic rollback edit"); await click('[data-testid="m2b-save-changes"]');
      await wait(rootState("failed"), "unconfirmed visible"); await feedback(expected[6]); await noDraftWarning();
      assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2b-retry\"]') && document.querySelector('[data-testid=\"m2b-draft\"]').disabled"), true);
      assert.equal((await invoke("m2b_get_experience", { id: entry.id })).entry.body, "synthetic external revision");
      out({ locale, emptyAndOpenedNoFalseDraft: true, contextualDrafts: true, scopedCreateEditDelete: true, cancelPreserves: true, conflictAndUnconfirmedVisible: true });
    }
    else if (action === "copy-review") {
      await input('[data-testid="m2b-draft"]', "synthetic copy review fixture"); await click('[data-testid="m2b-save"]');
      await wait(rootState("saved"), "copy fixture saved");
      const before = await invoke("m2b_list_experiences"); assert.equal(before.length, 1);
      for (const [locale, revision, build, confirm, caveat] of [
        ["en", "Versions of this moment", "About this test build", "Delete this moment? This will remove its current text and the text of previously saved versions from the app.", "No secure erasure guarantee"],
        ["zh-TW", "這個片刻的版本", "關於此測試版", "要刪除這個片刻嗎？這會刪除它目前的文字，以及先前儲存版本的文字。", "不保證"],
        ["ja", "この瞬間のバージョン", "このテスト版について", "この瞬間を削除しますか？現在の文章と、以前に保存した版の文章がアプリから削除されます。", "保証しません"],
      ]) {
        await click(`[data-locale="${locale}"]`);
        await wait(`document.querySelector('[data-locale="${locale}"]')?.getAttribute('aria-pressed')==='true' && document.querySelector('[data-runtime="android-m2b-disposable"]')?.dataset.localePreferenceState==='saved'`, "copy locale saved");
        assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-revision-details\"] summary').textContent"), revision);
        assert.equal(await cdp.evaluate("document.querySelector('[data-testid=\"m2b-build-details\"] summary').textContent"), build);
        assert.equal(await cdp.evaluate("[...document.querySelectorAll('[data-testid=\"m2b-revision-details\"],[data-testid=\"m2b-build-details\"]')].every(d=>!d.open)"), true);
        assert.equal(await cdp.evaluate(`document.querySelector('[data-testid="m2b-build-details"]').textContent.includes(${JSON.stringify(caveat)})`), true);
        await click('[data-testid="m2b-delete"]');
        assert.equal(await cdp.evaluate("document.querySelector('[role=\"dialog\"] p').textContent"), confirm);
        await click('[data-testid="m2b-cancel-delete"]');
        assert.deepEqual(await invoke("m2b_list_experiences"), before);
      }
      out({ locales: 3, distinctHeadings: true, plainConfirmation: true, optionalCaveats: true, cancelPreserves: true });
    }
    else if (action === "locale") {
      const locale = args.get("--locale"); assert.ok(["en", "ja", "zh-TW"].includes(locale));
      if (args.get("--verify") !== "true") await click(`[data-locale="${locale}"]`);
      await wait(`document.querySelector('[data-locale="${locale}"]')?.getAttribute('aria-pressed')==='true' && document.querySelector('[data-runtime="android-m2b-disposable"]')?.dataset.localePreferenceState==='saved'`, "locale persistence"); out({ locale });
    } else if (action === "create") {
      await input('[data-testid="m2b-draft"]', text); await click('[data-testid="m2b-save"]');
      await cdp.evaluate("document.querySelector('[data-testid=\"m2b-save\"]').click()");
      await wait(rootState("saved"), "create commit and reread");
      const entries = await invoke("m2b_list_experiences"); const matching = entries.filter((e) => e.body === text); assert.equal(matching.length, 1);
      const saved = await invoke("m2b_get_experience", { id: matching[0].id }); assert.equal(saved.entry.body, text);
      if (args.get("--metadata")) writeFileSync(args.get("--metadata"), JSON.stringify(metadata(saved))); out(metadata(saved));
    } else if (["cancel-edit", "edit", "cancel-delete", "delete"].includes(action)) {
      const id = args.get("--id"); const prior = await invoke("m2b_get_experience", { id }); assert.ok(prior);
      await click(`[data-experience-id="${id}"] [data-testid="m2b-open"]`);
      await wait("!!document.querySelector('[data-testid=\"m2b-exact\"]')", "detail open");
      if (action === "edit" || action === "cancel-edit") {
        await click('[data-testid="m2b-edit"]'); await input('[data-testid="m2b-edit-text"]', text);
        if (action === "cancel-edit") { await click('[data-testid="m2b-cancel-edit"]'); const same = await invoke("m2b_get_experience", { id }); assert.deepEqual(same, prior); }
        else { await click('[data-testid="m2b-save-changes"]'); await wait(rootState("saved"), "update verified reread");
          const current = await invoke("m2b_get_experience", { id }); assert.equal(current.entry.body, text); assert.equal(current.entry.id, prior.entry.id);
          assert.equal(current.predecessorRevisionId, prior.revisionId); assert.equal(current.authorship, "user"); assert.equal(current.revisionNumber, prior.revisionNumber + 1); }
      } else {
        await click('[data-testid="m2b-delete"]');
        if (action === "cancel-delete") { await click('[data-testid="m2b-cancel-delete"]'); assert.deepEqual(await invoke("m2b_get_experience", { id }), prior); }
        else { await click('[data-testid="m2b-confirm-delete"]'); await wait(rootState("deleted"), "delete verified reread");
          assert.equal(await invoke("m2b_get_experience", { id }), null); assert.equal(await cdp.evaluate("!!document.querySelector('[data-testid=\"m2b-exact\"]')"), false);
          assert.equal(await cdp.evaluate(`document.body.innerText.includes(${JSON.stringify(prior.entry.body)})`), false); }
      }
      out({ id });
    } else if (action === "text" || action === "absent") {
      const actual = await invoke("m2b_get_experience", { id: args.get("--id") });
      if (action === "absent") { assert.equal(actual, null); assert.ok(!(await invoke("m2b_list_experiences")).some((e) => e.id === args.get("--id"))); }
      else assert.equal(actual?.entry.body, text); out({ id: args.get("--id") });
    } else if (action === "protocol") {
      const id = "m2b-protocol-001"; const original = "M2B_CONTENT_LEAK_CANARY_original"; const updated = "M2B_CONTENT_LEAK_CANARY_更新\n修正";
      await invoke("m2b_create_experience", { id, text: original }); const prior = await invoke("m2b_get_experience", { id });
      const first = await request(prior, updated); const second = await request(prior, "stale"); const staleDelete = await request(prior, null);
      assert.equal((await invoke("m2b_mutate_experience", { request: first })).acknowledgement, "committed");
      assert.equal((await invoke("m2b_mutate_experience", { request: first })).acknowledgement, "alreadyCommitted");
      const stale = async (r) => assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2b_mutate_experience',{request:${JSON.stringify(r)}}).then(()=>false,e=>e==='m2b_stale_revision_preserved')`), true);
      await stale(second); await stale(staleDelete); let current = await invoke("m2b_get_experience", { id }); assert.equal(current.revisionNumber, 2);
      const same = await request(current, updated); await invoke("m2b_mutate_experience", { request: same }); current = await invoke("m2b_get_experience", { id }); assert.equal(current.revisionNumber, 3);
      for (const body of ["rollback-canary", null]) {
        const rollback = await request(current, body);
        assert.equal(await cdp.evaluate(`window.__TAURI_INTERNALS__.invoke('m2b_mutate_experience',{request:${JSON.stringify(rollback)},debugPhase:'rollbackAfterProjection'}).then(()=>false,e=>e==='m2b_not_committed')`), true);
        assert.deepEqual(await invoke("m2b_get_experience", { id }), current);
      }
      const deletion = await request(current, null); const ack = await invoke("m2b_mutate_experience", { request: deletion }); assert.equal(ack.acknowledgement, "committed");
      assert.ok(!JSON.stringify(ack).includes("CANARY")); assert.equal((await invoke("m2b_mutate_experience", { request: deletion })).acknowledgement, "alreadyCommitted");
      await stale(second); assert.equal((await invoke("m2b_mutate_experience", { request: first })).acknowledgement, "committedNotCurrent");
      assert.equal(await invoke("m2b_get_experience", { id }), null); out({ revisions: 3, duplicateAndStale: true, rollback: true, logicalDelete: true });
    } else if (action === "arm") {
      assert.ok(["beforeCommit", "afterCommitBeforeAck"].includes(phase)); assert.ok(["update", "delete"].includes(operation));
      const id = `m2b-fault-${operation}-${phase}`; await invoke("m2b_create_experience", { id, text: "fault original synthetic" });
      const s = await invoke("m2b_get_experience", { id }); const r = await request(s, operation === "update" ? text : null);
      const { body, ...safe } = r; writeFileSync(args.get("--metadata"), JSON.stringify(safe));
      await cdp.evaluate(`window.__m2bArmed=window.__TAURI_INTERNALS__.invoke('m2b_mutate_experience',{request:${JSON.stringify(r)},debugPhase:${JSON.stringify(phase)}}).catch(()=>undefined);true`);
      out({ operation, phase, armed: true, id });
    } else if (action === "retry" || action === "retry-state") {
      stage = "read-frozen-request";
      const safe = JSON.parse(readFileSync(args.get("--metadata"), "utf8")); const r = { ...safe, body: safe.operation === "delete" ? null : text };
      stage = "read-current-snapshot";
      const actual = await invoke("m2b_get_experience", { id: safe.id });
      if (action === "retry-state") {
        out({ id: safe.id, actualRevision: actual?.revisionNumber ?? null, textMatches: actual?.entry.body === text, expectedPredecessorMatches: actual?.revisionId === safe.expectedRevisionId });
      } else {
      stage = "classify-native-pre-or-post-state";
      if (args.get("--committed") === "true") {
        if (safe.operation === "delete") assert.equal(actual, null); else { assert.equal(actual?.entry.body, text); assert.equal(actual.revisionNumber, 2); }
      } else assert.equal(actual?.revisionNumber, 1);
      const ack = await invoke("m2b_mutate_experience", { request: r });
      stage = "classify-exact-request-ack";
      assert.equal(ack.acknowledgement, args.get("--committed") === "true" ? "alreadyCommitted" : "committed");
      if (safe.operation === "delete") assert.equal(await invoke("m2b_get_experience", { id: safe.id }), null);
      else assert.equal((await invoke("m2b_get_experience", { id: safe.id })).revisionNumber, 2);
      out({ operation: safe.operation, exactIdentityReconciled: true });
      }
    } else throw new Error("Unknown bounded native probe action");
  }
} catch {
  process.stderr.write("Native probe failed: " + action + " at " + stage + " (all content suppressed)\n");
  process.exitCode = 1;
} finally { cdp.close(); }
