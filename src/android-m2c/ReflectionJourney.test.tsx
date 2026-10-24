import { describe, expect, it, vi } from "vitest";
import { canCorrectResponse, ReflectionJourney, currentEvidence, localQuestion, reflectionPacket } from "./ReflectionJourney";
import { artifactRequest, m2cStore } from "./androidM2CStore";
import type { ArtifactRequest, M2CStore, ArtifactSnapshot, Snapshot } from "./androidM2CStore";
import { journeyCopy } from "./journeyCopy";
import { generateLocalReflectionPrompts, placeholderProvider } from "../ai/providers/placeholderProvider";
import { renderToStaticMarkup } from "react-dom/server";
import { ArtifactDetails, CandidateContent, candidateRepeatsSource } from "./ReflectionJourney";
import { CandidateScopeNote } from "./ReflectionJourney";
import { candidateGenerationLabel, isRejectedCandidate, RejectedCandidateHistory } from "./ReflectionJourney";
import { copy as sourceCopy } from "./AndroidM2CApp";
import { SourceListText } from "./AndroidM2CApp";
import type { EvidenceCandidate } from "../types/domain";
const source: Snapshot = { entry: { id: "m2c-source-synthetic", body: "Today I felt unsure and paused before replying. This is my account, not independent proof.", createdAt: "2026-10-04T00:00:00.000Z", updatedAt: "2026-10-04T00:00:00.000Z", userEditable: true }, revisionId: "v5sr_exact-source", revisionNumber: 1, predecessorRevisionId: null, authorship: "user" };
const row: ArtifactSnapshot = { id: "m2c-evidence-synthetic", kind: "evidence", revisionId: "v5ar_exact-evidence", reviewState: "confirmed", lifecycleState: "active", eligibilityState: "eligible", payload: { id: "m2c-evidence-synthetic", sourceEntryId: source.entry.id, text: "Your own words", originalText: "Your own words", kind: "other", userEditable: true, status: "confirmed", createdAt: source.entry.createdAt, updatedAt: source.entry.updatedAt, provenance: { origin: "local_mock", provider: "mock", model: null, sourceEntryId: source.entry.id, sourceArtifactIds: [], harnessVersion: "harness-v1", promptVersion: "v1", generatedAt: source.entry.createdAt } } };
describe("Founder003 rejected-candidate presentation only", () => {
  const rejected = (id: string): ArtifactSnapshot => ({ ...row, id, revisionId: null, reviewState: "rejected", lifecycleState: "content_purged", eligibilityState: "ineligible", payload: null });
  it.each(["en", "zh-TW", "ja"] as const)("names new creation and groups complete non-content history in %s", (locale) => {
    const rows = [rejected("rejected-one"), rejected("rejected-two"), { ...row, id: "active-new", reviewState: "pending", eligibilityState: "ineligible" }];
    const before = structuredClone(rows);
    expect(candidateGenerationLabel([], locale)).toBe(journeyCopy[locale].generate);
    expect(candidateGenerationLabel([row], locale)).toBe(journeyCopy[locale].generate);
    expect(candidateGenerationLabel(rows, locale)).toBe(journeyCopy[locale].recreate);
    expect(journeyCopy[locale].recreate).not.toBe(journeyCopy[locale].generate);
    const html = renderToStaticMarkup(<RejectedCandidateHistory rows={rows} locale={locale} />);
    expect(html.match(/data-testid="m2c-rejection-summary"/g)).toHaveLength(1);
    expect(html).toContain(journeyCopy[locale].rejectedSummary);
    expect(html).toContain(journeyCopy[locale].rejectedHistory); expect(html).toContain('(2)');
    expect(html).not.toContain(' open=""');
    for (const item of rows.slice(0, 2)) {
      expect(html).toContain(`data-artifact-id="${item.id}"`);
      for (const key of ["id", "kind", "revisionId", "reviewState", "lifecycleState", "eligibilityState"]) expect(html).toContain(key);
    }
    expect(html).not.toContain('data-artifact-id="active-new"');
    expect(html).not.toContain(source.entry.body); expect(html).not.toContain('payload');
    expect(rows).toEqual(before); expect(currentEvidence(rows)).toEqual([]);
  });
  it("does not regroup active, invalidated or non-Evidence states or leak unexpected payload", () => {
    for (const item of [row, { ...row, reviewState: "rejected" }, { ...row, lifecycleState: "invalidated", reviewState: "confirmed" }, { ...rejected("reflection"), kind: "reflection" as const }]) {
      expect(isRejectedCandidate(item)).toBe(false);
      expect(renderToStaticMarkup(<RejectedCandidateHistory rows={[item]} locale="en" />)).toBe("");
    }
    const unusual = { ...rejected("unexpected-record"), payload: { ...row.payload!, text: "MUST_NOT_DISCLOSE_REJECTED_TEXT" } } as ArtifactSnapshot;
    const html = renderToStaticMarkup(<RejectedCandidateHistory rows={[unusual]} locale="en" />);
    expect(html).not.toContain("MUST_NOT_DISCLOSE_REJECTED_TEXT"); expect(html).not.toContain("payload");
  });
  it("keeps a hundred rejected rows inspectable without a hundred primary status lines", () => {
    const rows = Array.from({ length: 100 }, (_, i) => rejected(`rejected-${i}`));
    const html = renderToStaticMarkup(<RejectedCandidateHistory rows={rows} locale="zh-TW" />);
    expect(html.match(/data-testid="m2c-rejection-summary"/g)).toHaveLength(1);
    expect(html.match(/data-artifact-id=/g)).toHaveLength(100);
    expect(html).toContain('(100)'); expect(html).not.toContain(' open=""');
    expect(currentEvidence(rows)).toEqual([]);
  });
});
describe("bounded offline synthetic Reflection", () => {
  it.each(["pending", "rejected"])("excludes %s Evidence", (reviewState) => expect(currentEvidence([{ ...row, reviewState }])).toEqual([]));
  it.each(["invalidated", "deleted", "content_purged"])("excludes %s artifact lifecycle", (lifecycleState) => expect(currentEvidence([{ ...row, lifecycleState }])).toEqual([]));
  it("never substitutes ineligible or missing content", () => {
    expect(currentEvidence([{ ...row, eligibilityState: "ineligible" }, { ...row, payload: null }])).toEqual([]);
    expect(currentEvidence([row])).toEqual([row]);
  });
  it("shared packet rejects foreign Evidence; drafts/skips/history are not supplied", () => {
    const foreign = { ...row, payload: { ...row.payload!, sourceEntryId: "foreign-source" } } as ArtifactSnapshot;
    const { packet, issues } = reflectionPacket(source, [foreign], "en");
    expect(issues[0].code).toBe("foreign_evidence");expect(packet.confirmedEvidence).toEqual([]);
    expect(packet.answeredReflectionResponses).toEqual([]);expect(packet.answeredClarificationTurns).toEqual([]);
    expect(localQuestion(source, [foreign], "en")).toBeNull();
  });
  it.each(["en", "zh-TW", "ja"] as const)("reuses neutral placeholder semantics with %s presentation", async (locale) => {
    expect(localQuestion(source, [row], locale)).toBe(journeyCopy[locale].question);
    const { packet } = reflectionPacket(source, [row], locale);
    const shared = generateLocalReflectionPrompts(packet)[0];const provider = (await placeholderProvider.generateReflectionPrompts(packet))[0];
    expect(shared.question).toBe(provider.question);expect(shared.promptProvenance.origin).toBe("local_mock");expect(shared.promptProvenance.model).toBeNull();
    expect(shared.sourceEvidenceIds).toEqual([row.id]);
    expect(Object.keys(journeyCopy[locale])).toEqual(Object.keys(journeyCopy.en));
    expect(localQuestion(source, [], locale)).toBeNull();
  });
  it("applies existing sparse-context gate without inventing recovery content", () => {
    expect(localQuestion({ ...source, entry: { ...source.entry, body: "short" } }, [row], "en")).toBeNull();
  });
  it("freezes exact source/artifact/dependency identity, localized payload and time", async () => {
    const request = await artifactRequest(source, "answer", "ja", row, "  自分の言葉。  ", [{ artifactId: row.id, revisionId: row.revisionId! }], "2026-10-04T00:00:01.000Z");
    expect(Object.isFrozen(request)).toBe(true);expect(Object.isFrozen(request.evidence)).toBe(true);expect(Object.isFrozen(request.evidence[0])).toBe(true);
    expect(request.text).toBe("  自分の言葉。  ");expect(request.expectedSourceRevisionId).toBe(source.revisionId);expect(request.expectedArtifactRevisionId).toBe(row.revisionId);
    const next = await artifactRequest(source, "answer", "ja", row, "different", request.evidence, request.occurredAt);
    expect(next.requestId).not.toBe(request.requestId);
  });
});

describe("desktop-informed, low-burden M2-C presentation", () => {
  it.each(["en", "zh-TW", "ja"] as const)("shows one primary source and collapsed exact duplicate/provenance in %s", (locale) => {
    const label = locale === "en" ? "Your own words" : locale === "zh-TW" ? "你寫下的原文" : "あなたが書いた原文";
    const record: EvidenceCandidate = { ...row.payload as EvidenceCandidate, text: `${label}:\n${source.entry.body.trim()}` };
    const list = renderToStaticMarkup(<SourceListText entry={source.entry} selectedId={source.entry.id} locale={locale} />);
    expect(list).not.toContain(source.entry.body);expect(list).toContain(journeyCopy[locale].currentSource);
    const body = renderToStaticMarkup(<CandidateContent record={record} source={source} reviewed={false} locale={locale} />);
    expect(body).toContain(journeyCopy[locale].sameSource);expect(body).toContain('data-testid="m2c-candidate-content"');
    expect(body).not.toContain(' open=""');expect(body).toContain('data-testid="m2c-candidate-text"');
    const info = renderToStaticMarkup(<ArtifactDetails provenance={record.provenance} response={{ origin: "user", sourceEntryId: source.entry.id, sourceArtifactIds: [] }} edited locale={locale} testId="m2c-evidence-provenance" />);
    expect(info).toContain(journeyCopy[locale].localOrigin);expect(info).toContain(journeyCopy[locale].userOrigin);expect(info).toContain(journeyCopy[locale].editedOrigin);
    expect(info).toContain('data-testid="m2c-technical-provenance"');expect(info).not.toContain(' open=""');
    expect(info).toContain('<dt>origin</dt><dd>local_mock</dd>');expect(info).toContain('<dt>provider</dt><dd>mock</dd>');expect(info).toContain('<dt>model</dt><dd>null</dd>');
    expect(info).not.toContain(source.entry.body);expect(info).toContain('href="#m2c-source-content"');
  });
  it("never collapses corrected pending words or hides changed/unknown content as an identical source", () => {
    const original: EvidenceCandidate = { ...row.payload as EvidenceCandidate, text: `Your own words:\n${source.entry.body.trim()}` };
    expect(candidateRepeatsSource(original, source)).toBe(true);
    const corrected: EvidenceCandidate = { ...row.payload as EvidenceCandidate, text: `Your own words:\n${source.entry.body.trim()}\nMy correction.` };
    expect(candidateRepeatsSource(corrected, source)).toBe(false);
    expect(renderToStaticMarkup(<CandidateContent record={corrected} source={source} reviewed={false} locale="en" />)).not.toContain('<details');
    expect(renderToStaticMarkup(<CandidateContent record={corrected} source={source} reviewed locale="en" />)).toContain('<details');
    expect(candidateRepeatsSource({ ...row.payload as EvidenceCandidate, text: source.entry.body }, source)).toBe(false);
  });
  it("does not fabricate mock or model metadata when absent", () => {
    const html = renderToStaticMarkup(<ArtifactDetails locale="en" testId="missing-origin" />);
    expect(html).toContain('<dt>model</dt><dd>Origin not supplied</dd>');expect(html).not.toContain('<dd>mock</dd>');expect(html).not.toContain('<dd>local_mock</dd>');
  });
});

describe("UX001 separate source, candidate and reflection", () => {
  it.each(["en", "zh-TW", "ja"] as const)("names separate operations and visible truthful demo scope in %s", (locale) => {
    const c=journeyCopy[locale], s=sourceCopy[locale];
    expect(s.edit).not.toBe(c.correct); expect(s.sourceEditLabel).not.toBe(c.correctionLabel);
    expect(c.confirm).not.toBe(s.save); expect(c.saveCorrection).not.toBe(s.changes);
    expect(Object.keys(sourceCopy[locale])).toEqual(Object.keys(sourceCopy.en));
    expect(c.candidateExplanation).toContain("AI");
    const html=renderToStaticMarkup(<CandidateScopeNote locale={locale} />);
    expect(html).toContain(c.candidateExplanation); expect(html).toContain('data-testid="m2c-candidate-scope"');
    expect(html).not.toContain("<details"); expect(html).not.toContain(source.entry.body);
    expect(html).not.toContain("local_mock"); expect(html).not.toContain("<pre");
    expect(c.correctionHelp.length).toBeGreaterThan(20); expect(s.sourceHint.length).toBeGreaterThan(15);
  });
  it("preserves generated question payloads, not just translated UI labels", () => {
    expect([journeyCopy.en.question,journeyCopy["zh-TW"].question,journeyCopy.ja.question]).toEqual([
      "What stands out when you read this evidence again?","再次閱讀這段紀錄時，什麼最讓你留意？","この記録を読み返すと、何が最も気になりますか？"
    ]);
    expect(sourceCopy["zh-TW"].edit).toBe("編輯原文");
    expect(journeyCopy["zh-TW"].candidate).toBe("線索候選・本機示範");
    expect(journeyCopy["zh-TW"].confirm).toBe("採用為反思線索");
  });
});


const artifactInvoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: artifactInvoke }));

// Drives the actual component hooks and rendered handlers, without a new DOM
// dependency. This does not claim browser layout/focus or SQLite persistence.
const hooks = vi.hoisted(() => ({ active: false, cursor: 0, values: [] as unknown[], effects: [] as (() => void)[], deps: [] as unknown[][] }));
vi.mock("react", async (original) => {
  const actual = await original<typeof import("react")>();
  return { ...actual,
    useState: (initial: unknown) => {
      if (!hooks.active) return actual.useState(initial);
      const i = hooks.cursor++;
      if (!(i in hooks.values)) hooks.values[i] = initial;
      return [hooks.values[i], (v: unknown) => { hooks.values[i] = typeof v === "function" ? v(hooks.values[i]) : v; }];
    },
    useRef: (initial: unknown) => {
      if (!hooks.active) return actual.useRef(initial);
      const i = hooks.cursor++;
      if (!(i in hooks.values)) hooks.values[i] = { current: initial };
      return hooks.values[i];
    },
    useEffect: (effect: () => void, deps: unknown[]) => {
      if (!hooks.active) return actual.useEffect(effect, deps);
      const i = hooks.cursor++, prior = hooks.deps[i];
      if (!prior || deps.some((v, n) => v !== prior[n])) { hooks.deps[i] = deps; hooks.effects.push(effect); }
    },
  };
});
const tick = async () => { for (let i = 0; i < 15; i++) await Promise.resolve(); };
type Element = { props: Record<string, any> }; // JSX handler inspection only; not a mock persistence layer.
function elements(tree: any): Element[] {
  if (Array.isArray(tree)) return tree.flatMap(elements);
  if (!tree || typeof tree !== "object" || !tree.props) return [];
  return [tree, ...elements(tree.props.children)];
}
const savedPrompt = (): ArtifactSnapshot => ({ id: "m2c-response-synthetic", kind: "reflection", revisionId: "v5ar_answer",
  reviewState: "not_applicable", lifecycleState: "active", eligibilityState: "eligible", payload: {
    id: "m2c-response-synthetic", sourceEntryId: source.entry.id, question: "Persisted original question?", status: "answered", response: "My saved words.",
    sourceEvidenceIds: [row.id], createdAt: source.entry.createdAt, updatedAt: "2026-10-04T00:00:01.000Z",
    promptProvenance: { origin: "local_mock", provider: "mock", model: null, sourceEntryId: source.entry.id, sourceArtifactIds: [row.id] },
    responseProvenance: { origin: "user", sourceEntryId: source.entry.id, sourceArtifactIds: ["m2c-response-synthetic"] },
  } });
async function component(locale: "en" | "zh-TW" | "ja" = "en") {
  hooks.cursor = 0; hooks.values = []; hooks.effects = []; hooks.deps = [];
  let rows = [structuredClone(row), savedPrompt()];
  const store = { artifacts: vi.fn(async () => structuredClone(rows)), get: vi.fn(async () => structuredClone(source)),
    artifactMutate: vi.fn(async (r: ArtifactRequest) => {
      const old = rows[1]; rows = [rows[0], { ...old, revisionId: old.revisionId + "_next", payload: { ...old.payload!, response: r.text, updatedAt: r.occurredAt } } as ArtifactSnapshot];
      return { requestId: r.requestId, acknowledgement: "committed" as const };
    }) };
  let tree: any;
  const onLock = vi.fn(), onReopen = vi.fn();
  const render = () => {
    hooks.cursor = 0; hooks.active = true;
    try { tree = ReflectionJourney({ source, locale, store: store as unknown as M2CStore, onLock, onReopen }); }
    finally { hooks.active = false; }
    hooks.effects.splice(0).forEach((effect) => effect());
  };
  const find = (id: string) => elements(tree).find((e) => e.props["data-testid"] === id);
  const get = (id: string) => { const e = find(id); expect(e, id).toBeTruthy(); return e!; };
  const click = (id: string) => { const e = get(id); expect(e.props.disabled, id + " disabled").not.toBe(true); e.props.onClick(); render(); };
  const change = (text: string) => { get("m2c-response-correction").props.onChange({ target: { value: text } }); render(); };
  render(); await tick(); render();
  return { store, get, find, click, change, render, onLock, onReopen, setRows: (v: ArtifactSnapshot[]) => { rows = v; } };
}
describe("saved-response correction R1 actual component handlers", () => {
  it.each(["en", "zh-TW", "ja"] as const)("prefills, labels saved/draft, cancels without a request, saves and corrects again in %s", async (locale) => {
    const ui = await component(locale), c = journeyCopy[locale];
    expect(ui.get("m2c-saved-response").props.children).toBe("My saved words.");
    expect(ui.get("m2c-question").props.children).toBe("Persisted original question?");
    expect(ui.get("m2c-edit-response").props.children).toBe(c.editResponse);
    ui.click("m2c-edit-response");
    expect(ui.get("m2c-response-correction").props.value).toBe("My saved words.");
    expect(ui.get("m2c-save-response-correction").props.disabled).toBe(true);
    ui.change("  "); expect(ui.get("m2c-save-response-correction").props.disabled).toBe(true);
    ui.change(" My saved words. "); expect(ui.get("m2c-save-response-correction").props.disabled).toBe(true);
    ui.change("Discarded draft");
    expect(ui.get("m2c-response-unsaved").props.children).toBe(c.draft);
    ui.click("m2c-cancel-response-correction");
    expect(ui.store.artifactMutate).not.toHaveBeenCalled(); expect(ui.find("m2c-response-correction")).toBeUndefined();
    expect(ui.get("m2c-saved-response").props.children).toBe("My saved words.");
    let count = 0;
    for (const text of ["我的新回答。静かに選ぶ。", "A later correction."]) {
      ui.click("m2c-edit-response"); ui.change("  " + text + "  "); ui.click("m2c-save-response-correction");
      expect(ui.get("m2c-artifact-feedback").props.children).toBe(c.working);
      count++; await vi.waitFor(() => expect(ui.store.artifactMutate).toHaveBeenCalledTimes(count));
      await vi.waitFor(() => { ui.render(); expect(ui.get("m2c-artifact-feedback").props.children).toBe(c.saved); });
      expect(ui.get("m2c-saved-response").props.children).toBe(text);
      expect(ui.get("m2c-artifact-feedback").props.children).toBe(c.saved);
      expect(ui.find("m2c-response-correction")).toBeUndefined();
    }
    expect(ui.store.artifactMutate).toHaveBeenCalledTimes(2);
    const [first, second] = ui.store.artifactMutate.mock.calls.map(([r]) => r);
    expect(first.operation).toBe("correctResponse"); expect(first.text).toBe("我的新回答。静かに選ぶ。");
    expect(first.artifactId).toBe(savedPrompt().id); expect(first.expectedArtifactRevisionId).toBe("v5ar_answer");
    expect(second.expectedArtifactRevisionId).toBe("v5ar_answer_next");
    expect(first.evidence).toEqual([{ artifactId: row.id, revisionId: row.revisionId }]);
    expect(first.expectedSourceRevisionId).toBe(source.revisionId);
  });
  it("fences hashing, duplicate clicks and IPC; success waits for verified reread", async () => {
    const ui = await component(); let resolve!: (ack: any) => void;
    ui.store.artifactMutate.mockImplementationOnce((r) => new Promise((done) => { resolve = (ack) => done({ requestId: r.requestId, ...ack }); }));
    ui.click("m2c-edit-response"); ui.change("Not committed yet");
    const handler = ui.get("m2c-save-response-correction").props.onClick;
    handler(); handler(); ui.render();
    expect(ui.get("m2c-save-response-correction").props.disabled).toBe(true);
    expect(ui.get("m2c-cancel-response-correction").props.disabled).toBe(true);
    await vi.waitFor(() => expect(ui.store.artifactMutate).toHaveBeenCalledTimes(1));
    expect(ui.get("m2c-artifact-feedback").props.children).toBe(journeyCopy.en.working);
    resolve({ acknowledgement: "committed" }); await tick(); ui.render();
    // An acknowledgement without matching durable reread is not success.
    expect(ui.get("m2c-artifact-feedback").props.children).toBe(journeyCopy.en.conflict);
    expect(ui.get("m2c-response-correction").props.value).toBe("Not committed yet");
  });
  it.each(["m2c_not_committed", "m2c_stale_revision_preserved", "m2c_outcome_unconfirmed"])("retains draft and frozen identity after %s; inspection is read-only", async (failure) => {
    const ui = await component(); ui.store.artifactMutate.mockRejectedValueOnce(new Error(failure));
    ui.click("m2c-edit-response"); ui.change("Keep these unsaved words"); ui.click("m2c-save-response-correction");
    await vi.waitFor(() => expect(ui.store.artifactMutate).toHaveBeenCalledTimes(1)); await tick(); ui.render();
    expect(ui.get("m2c-artifact-feedback").props.children).not.toBe(journeyCopy.en.saved);
    expect(ui.get("m2c-response-correction").props.value).toBe("Keep these unsaved words");
    expect(ui.get("m2c-saved-response").props.children).toBe("My saved words.");
    expect(ui.get("m2c-artifact-reopen").props.children).toBe(journeyCopy.en.discardResponseDraft);
    ui.click("m2c-inspect-response"); await tick(); ui.render();
    expect(ui.get("m2c-inspected-response")).toBeTruthy(); expect(ui.store.artifactMutate).toHaveBeenCalledTimes(1);
    expect(ui.get("m2c-response-correction").props.value).toBe("Keep these unsaved words");
    ui.click("m2c-artifact-reconcile"); await tick(); ui.render();
    expect(ui.store.artifactMutate).toHaveBeenCalledTimes(2);
    expect(ui.store.artifactMutate.mock.calls[1][0]).toBe(ui.store.artifactMutate.mock.calls[0][0]);
    expect(ui.get("m2c-artifact-feedback").props.children).toBe(journeyCopy.en.saved);
  });
  it("does not offer correction for invalid lifecycle/eligibility, missing evidence or wrong authorship", () => {
    const prompt = savedPrompt(); expect(canCorrectResponse(prompt, source, [row, prompt])).toBe(true);
    for (const invalid of [{ ...prompt, lifecycleState: "invalidated" }, { ...prompt, eligibilityState: "ineligible" }, { ...prompt, revisionId: null }, { ...prompt, reviewState: "pending" }, { ...prompt, payload: null }]) expect(canCorrectResponse(invalid, source, [row, invalid])).toBe(false);
    expect(canCorrectResponse(prompt, source, [prompt])).toBe(false);
    expect(canCorrectResponse(prompt, { ...source, entry: { ...source.entry, id: "foreign" } }, [row, prompt])).toBe(false);
  });

  it("forwards the identical correction descriptor through the existing artifact IPC", async () => {
    const request = await artifactRequest(source, "correctResponse", "ja", savedPrompt(), "修正", [{ artifactId: row.id, revisionId: row.revisionId! }]);
    artifactInvoke.mockReset(); artifactInvoke.mockResolvedValue({ requestId: request.requestId, acknowledgement: "alreadyCommitted" });
    await m2cStore.artifactMutate(request); await m2cStore.artifactMutate(request);
    expect(artifactInvoke.mock.calls).toEqual([["m2c_mutate_artifact", { request, debugPhase: undefined }], ["m2c_mutate_artifact", { request, debugPhase: undefined }]]);
    expect(artifactInvoke.mock.calls[0][1].request).toBe(request);
  });
  it.each(["reread-fails", "source-missing", "source-changed", "not-current", "wrong-ack", "provenance-changed", "artifacts-fail"])("keeps draft and withholds success when %s", async (caseName) => {
    const ui = await component();
    if (caseName === "reread-fails") ui.store.get.mockRejectedValueOnce(new Error("read failed"));
    if (caseName === "source-missing") ui.store.get.mockResolvedValueOnce(null as unknown as Snapshot);
    if (caseName === "source-changed") ui.store.get.mockResolvedValueOnce({ ...source, revisionId: "other-source-revision" });
    if (caseName === "artifacts-fail") ui.store.artifacts.mockRejectedValueOnce(new Error("read failed"));
    if (caseName === "not-current") ui.store.artifactMutate.mockImplementationOnce(async (r) => ({ requestId: r.requestId, acknowledgement: "committedNotCurrent" as "committed" }));
    if (caseName === "wrong-ack") ui.store.artifactMutate.mockImplementationOnce(async () => ({ requestId: "other-request", acknowledgement: "committed" }));
    if (caseName === "provenance-changed") ui.store.artifactMutate.mockImplementationOnce(async (r) => {
      const p = savedPrompt(); ui.setRows([row, { ...p, revisionId: "other-answer", payload: { ...p.payload!, response: r.text, promptProvenance: { origin: "user", sourceEntryId: source.entry.id, sourceArtifactIds: [] } } } as ArtifactSnapshot]);
      return { requestId: r.requestId, acknowledgement: "committed" };
    });
    ui.click("m2c-edit-response"); ui.change("Retain on uncertain reread"); ui.click("m2c-save-response-correction");
    await vi.waitFor(() => { ui.render(); expect(ui.find("m2c-artifact-reconcile")).toBeTruthy(); });
    expect(ui.get("m2c-response-correction").props.value).toBe("Retain on uncertain reread");
    expect(ui.get("m2c-artifact-feedback").props.children).not.toBe(journeyCopy.en.saved);
    expect(ui.store.artifactMutate).toHaveBeenCalledTimes(1); expect(ui.onReopen).not.toHaveBeenCalled();
  });

  it("binds every correction coordinate and operation into immutable identity", async () => {
    const target = savedPrompt(), refs = [{ artifactId: row.id, revisionId: row.revisionId! }], time = "2026-10-04T00:00:02.000Z";
    const r = await artifactRequest(source, "correctResponse", "en", target, "changed", refs, time);
    expect(r.requestId).toBe("m2c_81dc40dbb5a20883d6413b865c2eb805e486b16c6c042b9af3757502c6bd5fcd");
    expect(r).toEqual(await artifactRequest(source, "correctResponse", "en", target, "changed", refs, time));
    expect(Object.isFrozen(r)).toBe(true); expect(Object.isFrozen(r.evidence[0])).toBe(true);
    const variants = await Promise.all([
      artifactRequest(source, "answer", "en", target, "changed", refs, time),
      artifactRequest({ ...source, revisionId: "other-source" }, "correctResponse", "en", target, "changed", refs, time),
      artifactRequest(source, "correctResponse", "en", { ...target, revisionId: "other-target" }, "changed", refs, time),
      artifactRequest(source, "correctResponse", "en", target, "changed", [{ ...refs[0], revisionId: "other-evidence" }], time),
      artifactRequest(source, "correctResponse", "ja", target, "changed", refs, time),
      artifactRequest(source, "correctResponse", "en", target, "other text", refs, time),
    ]); for (const v of variants) expect(v.requestId).not.toBe(r.requestId);
    expect(reflectionPacket(source, [row, target], "en").packet.answeredReflectionResponses).toEqual([]);
  });
});
