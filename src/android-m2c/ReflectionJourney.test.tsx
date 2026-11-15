import { describe, expect, it, vi } from "vitest";
import { canCorrectResponse, ReflectionJourney, currentEvidence, localQuestion, reflectionPacket } from "./ReflectionJourney";
import { artifactRequest, m2cStore } from "./androidM2CStore";
import type { ArtifactAck, ArtifactRequest, M2CStore, ArtifactSnapshot, QuestionBatchRequest, Snapshot } from "./androidM2CStore";
import { journeyCopy } from "./journeyCopy";
import { generateLocalReflectionPrompts, placeholderProvider } from "../ai/providers/placeholderProvider";
import { renderToStaticMarkup } from "react-dom/server";
import { ArtifactDetails, CandidateContent, candidateRepeatsSource } from "./ReflectionJourney";
import { CandidateScopeNote } from "./ReflectionJourney";
import { candidateGenerationLabel, isRejectedCandidate, RejectedCandidateHistory } from "./ReflectionJourney";
import { copy as sourceCopy } from "./AndroidM2CApp";
import { AndroidM2CApp, MomentActions, ReflectionDisclosure, SourceListText } from "./AndroidM2CApp";
import { PilotSettings } from "./PilotModelPanels";
import { m2cApplicationId } from "./androidM2CStore";
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
type Element = { props: Record<string, any>; ref?: { current: unknown } }; // JSX handler inspection only; not a mock persistence layer.
function elements(tree: any): Element[] {
  if (Array.isArray(tree)) return tree.flatMap(elements);
  if (!tree || typeof tree !== "object" || !tree.props) return [];
  return [tree, ...elements(tree.props.children)];
}

describe("Pilot Android save-to-reflection handoff", () => {
  it("keeps the selected moment and reflection mounted behind settings and reloads all language preferences", async () => {
    let locale: import("./androidM2CStore").Locale = "en";
    const history = { state: null as unknown, pushState: vi.fn(), back: vi.fn() };
    vi.stubGlobal("window", { history, addEventListener: vi.fn(), removeEventListener: vi.fn() });
    vi.stubGlobal("document", { documentElement: { lang: "" } });
    try {
      const original = structuredClone(source);
      const store = {
        status: async () => ({ state: "ready", applicationId: m2cApplicationId, syntheticOnly: true, schemaVersion: 5 }),
        list: async () => [source.entry], get: async () => structuredClone(source),
        readLocale: async () => locale, writeLocale: vi.fn(async (value) => { locale = value; }),
      } as unknown as M2CStore;
      let tree: unknown;
      const reset = () => { hooks.values = []; hooks.effects = []; hooks.deps = []; };
      const render = () => {
        hooks.cursor = 0; hooks.active = true;
        try { tree = AndroidM2CApp({ store, pilot: true }); }
        finally { hooks.active = false; }
        hooks.effects.splice(0).forEach((effect) => effect());
      };
      const byId = (id: string) => elements(tree).find((v) => v.props["data-testid"] === id)!;
      const findType = (type: unknown) => elements(tree).find((element) => (element as unknown as { type: unknown }).type === type)!;
      for (const code of ["zh-TW", "ja", "en"] as const) {
        reset(); render(); await tick(); render();
        byId("m2c-open").props.onClick(); await tick(); render();
        byId("m2c-draft").props.onChange({ target: { value: "Local draft" } }); render();
        const reflection = findType(ReflectionDisclosure) as unknown as { key: string };
        byId("pilot-settings-open").props.onClick(); render();
        expect(byId("pilot-main").props.hidden).toBe(true);
        expect((findType(ReflectionDisclosure) as unknown as { key: string }).key).toBe(reflection.key);
        const settings = findType(PilotSettings);
        const buttons = elements(settings.props.languages);
        buttons.find((e) => e.props["data-locale"] === code)!.props.onClick(); await tick(); render();
        expect(findType(PilotSettings).props.locale).toBe(code);
        findType(PilotSettings).props.close(); render();
        expect(byId("pilot-main").props.hidden).toBe(false);
        expect(byId("m2c-exact-text").props.children).toBe(original.entry.body);
        expect(byId("m2c-draft").props.value).toBe("Local draft");
        expect((findType(ReflectionDisclosure) as unknown as { key: string }).key).toBe(reflection.key);
        reset(); render(); await tick(); render();
        byId("pilot-settings-open").props.onClick(); render();
        expect(findType(PilotSettings).props.locale).toBe(code);
      }
      expect(store.writeLocale).toHaveBeenCalledTimes(3);
      expect(source).toEqual(original);
    } finally { vi.unstubAllGlobals(); }
  });

  it.each(["verified", "short moment", "uncertain create", "read failure", "wrong source", "wrong body"] as const)(
    "moves focus only after verified create readback: %s", async (outcome) => {
      hooks.cursor = 0; hooks.values = []; hooks.effects = []; hooks.deps = [];
      vi.stubGlobal("document", { documentElement: { lang: "" } });
      try {
        const heading = { focus: vi.fn(), scrollIntoView: vi.fn() };
        const written = outcome === "short moment" ? "下雨了。" : "A synthetic moment, saved for reflection.";
        const older = structuredClone(source);
        let persisted: Snapshot | null = null;
        let finishCreate!: () => void;
        const create = vi.fn((id: string, body: string) => new Promise<void>((resolve, reject) => {
          finishCreate = () => {
            persisted = { ...source, entry: { ...source.entry, id, body } };
            if (outcome === "uncertain create") reject(new Error("unverified")); else resolve();
          };
        }));
        const get = vi.fn(async (id: string) => {
          if (id === older.entry.id) return older;
          if (outcome === "read failure") throw new Error("read failed");
          if (!persisted) return null;
          if (outcome === "wrong source") return { ...persisted, entry: { ...persisted.entry, id: older.entry.id } };
          if (outcome === "wrong body") return { ...persisted, entry: { ...persisted.entry, body: "Different text" } };
          return persisted;
        });
        const store = {
          create, get, list: vi.fn(async () => persisted ? [persisted.entry, older.entry] : [older.entry]),
          readLocale: vi.fn(async () => "zh-TW"), status: vi.fn(async () => ({})),
          mutate: vi.fn(async (request: import("./androidM2CStore").LifecycleRequest) => {
            if (!persisted || request.operation !== "update" || request.body === null) throw new Error("unexpected request");
            persisted = {
              ...persisted, revisionId: "updated-synthetic", predecessorRevisionId: persisted.revisionId, revisionNumber: 2,
              entry: { ...persisted.entry, body: request.body, updatedAt: request.occurredAt },
            };
            return { requestId: request.requestId, sourceId: request.id, acknowledgement: "committed" as const };
          }),
          artifacts: vi.fn(), artifactMutate: vi.fn(), generateQuestions: vi.fn(),
        };
        let tree: unknown;
        const byId = (id: string) => elements(tree).find((e) => e.props.id === id);
        const findTest = (id: string) => elements(tree).find((e) => e.props["data-testid"] === id);
        const byTest = (id: string) => findTest(id)!;
        const actions = () => elements(tree).find((e) => (e as unknown as { type: unknown }).type === MomentActions)!;
        const link = () => elements(tree).find((e) => e.props.href === "#m2c-reflection-start");
        const c = journeyCopy["zh-TW"];
        const render = () => {
          hooks.cursor = 0; hooks.active = true;
          try { tree = AndroidM2CApp({ store: store as unknown as M2CStore }); }
          finally { hooks.active = false; }
          const title = byId("m2c-saved-moment-heading");
          if (title?.ref) title.ref.current = heading;
          hooks.effects.splice(0).forEach((effect) => effect());
        };
        render(); await tick(); render();
        byTest("m2c-open").props.onClick(); await tick(); render();
        expect(byTest("m2c-exact-text").props.children).toBe(older.entry.body);
        expect(actions().props.compact).toBe(true);
        expect(link()?.props.className).toContain("android-m2c__reflection-link--primary");
        expect(byId("m2c-saved-moment-heading")?.props.children).toBe(c.yourMoment);
        expect(heading.focus).toHaveBeenCalledTimes(1);
        heading.focus.mockClear(); heading.scrollIntoView.mockClear();
        byTest("m2c-draft").props.onChange({ target: { value: written } }); render();
        byTest("m2c-save").props.onClick(); render();
        expect(heading.scrollIntoView).not.toHaveBeenCalled();
        const saveState = () => elements(tree).find((e) => e.props["data-save-state"])?.props["data-save-state"];
        expect(saveState()).toBe("saving");
        finishCreate(); await tick(); render(); render();
        if (outcome === "verified" || outcome === "short moment") {
          expect(byTest("m2c-exact-text").props.children).toBe(written);
          expect(byId("m2c-saved-moment-heading")?.props.children).toBe(c.momentSaved);
          const expectNextAction = () => {
            if (outcome === "short moment") {
              expect(link()).toBeUndefined();
              expect(byTest("m2c-reflection-context").props.children).toBe(c.contextNeeded);
              expect(byTest("m2c-add-context").props.children).toBe(c.addContext);
              expect(byTest("m2c-add-context").props["aria-describedby"]).toBe(byTest("m2c-reflection-context").props.id);
            } else {
              expect(link()?.props.children).toBe(c.startReflection);
              expect(link()?.props.className).toContain("android-m2c__reflection-link--primary");
              expect(link()?.props.onClick).toBeUndefined();
              expect(findTest("m2c-add-context")).toBeUndefined();
            }
          };
          expectNextAction();
          expect(actions().props.compact).toBe(true);
          expect(actions().props.label).toBe(c.moreMomentActions);
          const secondary = elements(actions()).map((e) => e.props["data-testid"]);
          expect(secondary).toEqual(expect.arrayContaining(["m2c-edit", "m2c-delete", "m2c-close", "m2c-revision-details"]));
          expect(findTest("m2c-source-hint")).toBeUndefined();
          expect(heading.focus).toHaveBeenCalledExactlyOnceWith({ preventScroll: true });
          expect(heading.scrollIntoView).toHaveBeenCalledExactlyOnceWith({ block: "start", behavior: "instant" });

          byTest(outcome === "short moment" ? "m2c-add-context" : "m2c-edit").props.onClick(); render();
          expect(actions().props.compact).toBe(false);
          expect(byTest("m2c-edit-text").props["aria-describedby"]).toBe(findTest("m2c-source-hint")?.props.id);
          expect(byTest("m2c-edit-text").props.value).toBe(written);
          expect(link()).toBeUndefined();
          byTest("m2c-cancel-edit").props.onClick(); render();
          expect(actions().props.compact).toBe(true);
          expect(findTest("m2c-source-hint")).toBeUndefined();
          expectNextAction();
          expect(store.mutate).not.toHaveBeenCalled();

          byTest("m2c-delete").props.onClick(); render();
          expect(actions().props.compact).toBe(false);
          expect(findTest("m2c-confirm-delete")).toBeDefined();
          expect(link()).toBeUndefined();
          byTest("m2c-cancel-delete").props.onClick(); render();
          expect(actions().props.compact).toBe(true);
          expect(byTest("m2c-exact-text").props.children).toBe(written);
          const beforeReopen = (actions() as unknown as { key: string }).key;
          byTest("m2c-open").props.onClick(); await tick(); render();
          expect(actions().props.compact).toBe(true);
          expect((actions() as unknown as { key: string }).key).not.toBe(beforeReopen);
          expect(byId("m2c-saved-moment-heading")?.props.children).toBe(c.yourMoment);
          expectNextAction();
          expect(heading.scrollIntoView).toHaveBeenCalledTimes(2);
          if (outcome === "short moment") {
            const expanded = "今天下班後下雨，我在公車站等了十分鐘，最後和同事一起走回家。";
            byTest("m2c-add-context").props.onClick(); render();
            byTest("m2c-edit-text").props.onChange({ target: { value: expanded } }); render();
            expect(link()).toBeUndefined();
            byTest("m2c-save-changes").props.onClick(); render();
            await vi.waitFor(() => { render(); expect(byTest("m2c-exact-text").props.children).toBe(expanded); });
            expect(link()?.props.children).toBe(c.startReflection);
            expect(findTest("m2c-reflection-context")).toBeUndefined();
            expect(actions().props.compact).toBe(true);
            expect(store.mutate).toHaveBeenCalledTimes(1);
            expect(heading.scrollIntoView).toHaveBeenCalledTimes(3);
          }
        } else {
          expect(byTest("m2c-exact-text").props.children).toBe(older.entry.body);
          expect(saveState()).toBe("failed");
          expect(heading.focus).not.toHaveBeenCalled();
          expect(heading.scrollIntoView).not.toHaveBeenCalled();
          expect(byTest("m2c-draft").props.value).toBe(written);
          expect(actions().props.compact).toBe(true);
          expect(link()).toBeUndefined();
          expect(findTest("m2c-add-context")).toBeUndefined();
        }
        expect(create).toHaveBeenCalledTimes(1);
        expect(store.artifactMutate).not.toHaveBeenCalled();
        expect(store.generateQuestions).not.toHaveBeenCalled();
      } finally { vi.unstubAllGlobals(); }
    },
  );
  it.each(["en", "zh-TW", "ja"] as const)("uses a closed native disclosure for additional actions in %s", (locale) => {
    const label = journeyCopy[locale].moreMomentActions;
    const html = renderToStaticMarkup(<MomentActions compact label={label}><button>Secondary action</button></MomentActions>);
    expect(html).toContain(`<summary>${label}</summary>`);
    expect(html).toContain("<details");
    expect(html).not.toMatch(/<details[^>]*\sopen(?:=|\s|>)/);
    expect(html).toContain("<button>Secondary action</button>");
    expect(renderToStaticMarkup(<MomentActions compact={false} label={label}><button>Secondary action</button></MomentActions>))
      .toBe("<button>Secondary action</button>");
  });
});
const savedPrompt = (): ArtifactSnapshot => ({ id: "m2c-response-synthetic", kind: "reflection", revisionId: "v5ar_answer",
  reviewState: "not_applicable", lifecycleState: "active", eligibilityState: "eligible", payload: {
    id: "m2c-response-synthetic", sourceEntryId: source.entry.id, question: "Persisted original question?", status: "answered", response: "My saved words.",
    sourceEvidenceIds: [row.id], createdAt: source.entry.createdAt, updatedAt: "2026-10-04T00:00:01.000Z",
    promptProvenance: { origin: "local_mock", provider: "mock", model: null, sourceEntryId: source.entry.id, sourceArtifactIds: [row.id] },
    responseProvenance: { origin: "user", sourceEntryId: source.entry.id, sourceArtifactIds: ["m2c-response-synthetic"] },
  } });
async function component(locale: "en" | "zh-TW" | "ja" = "en", initial = [row, savedPrompt()], guided = false) {
  hooks.cursor = 0; hooks.values = []; hooks.effects = []; hooks.deps = [];
  let rows = structuredClone(initial);
  const store = { artifacts: vi.fn(async () => structuredClone(rows)), get: vi.fn(async () => structuredClone(source)),
    generateQuestions: vi.fn(async (r: QuestionBatchRequest): Promise<ArtifactAck> => {
      rows = [...rows, ...r.items.map((_, i) => suggestedPrompt(r, i))];
      return { requestId: r.requestId, acknowledgement: "committed" as const };
    }),
    artifactMutate: vi.fn(async (r: ArtifactRequest) => {
      const old = rows[1]; rows = [rows[0], { ...old, revisionId: old.revisionId + "_next", payload: { ...old.payload!, response: r.text, updatedAt: r.occurredAt } } as ArtifactSnapshot];
      return { requestId: r.requestId, acknowledgement: "committed" as const };
    }) };
  let tree: any;
  const onLock = vi.fn(), onReopen = vi.fn();
  const render = () => {
    hooks.cursor = 0; hooks.active = true;
    try { tree = ReflectionJourney({ source, locale, store: store as unknown as M2CStore, onLock, onReopen, guided }); }
    finally { hooks.active = false; }
    hooks.effects.splice(0).forEach((effect) => effect());
  };
  const find = (id: string) => elements(tree).find((e) => e.props["data-testid"] === id);
  const get = (id: string) => { const e = find(id); expect(e, id).toBeTruthy(); return e!; };
  const click = (id: string) => { const e = get(id); expect(e.props.disabled, id + " disabled").not.toBe(true); e.props.onClick(); render(); };
  const change = (text: string) => { get("m2c-response-correction").props.onChange({ target: { value: text } }); render(); };
  render(); await tick(); render();
  return { store, get, find, click, change, render, onLock, onReopen, all: (id: string) => elements(tree).filter((e) => e.props["data-testid"] === id), setRows: (v: ArtifactSnapshot[]) => { rows = v; } };
}
describe("Pilot Android guided clue review", () => {
  it.each(["en", "zh-TW", "ja"] as const)("groups superseded questions once without counting clues or resurfacing old answers in %s", async (locale) => {
    const c = journeyCopy[locale];
    const invalidClue: ArtifactSnapshot = { ...row, lifecycleState: "invalidated", eligibilityState: "ineligible", payload: null };
    const invalidQuestions: ArtifactSnapshot[] = Array.from({ length: 3 }, (_, i) => ({
      ...savedPrompt(), id: `old-question-${i}`, lifecycleState: "invalidated", eligibilityState: "ineligible",
    }));
    const rejected: ArtifactSnapshot = { ...invalidClue, id: "other-clue", reviewState: "rejected" };
    const ui = await component(locale, [invalidClue, ...invalidQuestions, rejected], true);
    const notice = ui.get("m2c-source-update");
    const html = renderToStaticMarkup(notice as unknown as React.ReactElement);
    expect(ui.all("m2c-source-update")).toHaveLength(1);
    for (const label of [c.sourceUpdatedTitle, c.supersededQuestions(3), c.reflectOnUpdatedSource]) expect(html).toContain(label);
    expect(html).not.toContain(c.supersededClues);
    expect(ui.find("m2c-question")).toBeUndefined();
    expect(ui.find("m2c-saved-response")).toBeUndefined();
    expect(ui.get("m2c-generate-candidate").props.disabled).toBe(false);
    expect(ui.find("m2c-generate-question")).toBeUndefined();
    const journey = renderToStaticMarkup(ui.get("m2c-journey") as unknown as React.ReactElement);
    expect(journey).not.toContain(c.invalidated);
    expect(journey.indexOf('data-testid="m2c-source-update"')).toBeLessThan(journey.indexOf('data-testid="m2c-generate-candidate"'));
    expect(ui.store.artifactMutate).not.toHaveBeenCalled();
    expect(ui.store.generateQuestions).not.toHaveBeenCalled();
  });

  it("uses clue-only copy when no question was superseded and keeps active work visible", async () => {
    const c = journeyCopy["zh-TW"];
    const invalidClue: ArtifactSnapshot = { ...row, id: "old-clue", lifecycleState: "invalidated", eligibilityState: "ineligible", payload: null };
    const ui = await component("zh-TW", [invalidClue, row, savedPrompt()], true);
    const notice = renderToStaticMarkup(ui.get("m2c-source-update") as unknown as React.ReactElement);
    expect(notice).toContain(c.supersededClues);
    expect(notice).not.toContain(c.reflectOnUpdatedSource);
    expect(ui.get("m2c-question").props.children).toBe("Persisted original question?");
    expect(ui.get("m2c-saved-response").props.children).toBe("My saved words.");
    expect(ui.get("m2c-generate-question").props.children).toBe(c.addQuestion);
  });

  it("preserves the historical non-Pilot presentation for invalidated rows", async () => {
    const invalid: ArtifactSnapshot = { ...savedPrompt(), lifecycleState: "invalidated", eligibilityState: "ineligible", payload: null };
    const ui = await component("en", [invalid]);
    expect(ui.find("m2c-source-update")).toBeUndefined();
    const html = renderToStaticMarkup(ui.get("m2c-journey") as unknown as React.ReactElement);
    expect(html).toContain(journeyCopy.en.invalidated);
  });

  it("renders a closed disclosure with the next step inside, not a fragment link", () => {
    const c = journeyCopy["zh-TW"];
    const html = renderToStaticMarkup(<ReflectionDisclosure label={c.startReflection} collapseLabel={c.collapseReflection}>
      <p>{c.createDemoClue}</p>
    </ReflectionDisclosure>);
    expect(html).toMatch(/^<details[^>]+><summary[^>]+>/);
    expect(html).not.toMatch(/\bopen=""|href=/);
    for (const label of [c.startReflection, c.collapseReflection, c.createDemoClue]) expect(html).toContain(label);
  });

  it("brings the opened step into view without reacting to nested details or collapse", () => {
    const c = journeyCopy["zh-TW"];
    const disclosure = ReflectionDisclosure({ label: c.startReflection, collapseLabel: c.collapseReflection, children: null });
    const scrollIntoView = vi.fn();
    const target = { open: true, firstElementChild: { scrollIntoView } };
    disclosure.props.onToggle({ currentTarget: target, target: {} });
    disclosure.props.onToggle({ currentTarget: target, target });
    target.open = false;
    disclosure.props.onToggle({ currentTarget: target, target });
    expect(scrollIntoView).toHaveBeenCalledExactlyOnceWith({ block: "start", behavior: "instant" });
  });

  it("requires separate actions to create, review and use the exact clue before offering a question", async () => {
    const ui = await component("zh-TW", [], true);
    const c = journeyCopy["zh-TW"];
    const clueText = "你寫下的原文:\n" + source.entry.body;
    const operations: ArtifactRequest[] = [];
    ui.store.artifactMutate.mockImplementation(async (request) => {
      operations.push(request);
      const confirmed = request.operation === "confirm";
      const payload: EvidenceCandidate = {
        ...row.payload as EvidenceCandidate, id: request.artifactId, text: clueText, originalText: clueText,
        status: confirmed ? "confirmed" : "candidate",
      };
      ui.setRows([{ ...row, id: request.artifactId, revisionId: confirmed ? "v5ar_confirmed" : "v5ar_pending",
        reviewState: confirmed ? "confirmed" : "pending", eligibilityState: confirmed ? "eligible" : "ineligible", payload }]);
      return { requestId: request.requestId, acknowledgement: "committed" };
    });

    expect(ui.store.artifactMutate).not.toHaveBeenCalled();
    expect(ui.get("m2c-generate-candidate").props.children).toBe(c.createDemoClue);
    expect(ui.find("m2c-generate-question")).toBeUndefined();
    ui.click("m2c-generate-candidate");
    await vi.waitFor(() => {
      ui.render(); expect(ui.get("m2c-candidate-scope").props.children).toBe(c.originalReference);
    });
    expect(ui.get("m2c-clue-title").props.children).toBe(c.clueTitle);
    expect(ui.get("m2c-clue-status").props.children).toBe(c.clueToReview);
    expect(ui.find("m2c-candidate-text")).toBeUndefined();
    expect(ui.find("m2c-candidate-content")).toBeUndefined();
    expect(ui.find("m2c-artifact-feedback")).toBeUndefined();
    expect(ui.find("m2c-generate-question")).toBeUndefined();
    expect(ui.get("m2c-confirm").props).toMatchObject({ children: c.useOriginal, className: "android-m2c__journey-primary" });
    expect(ui.get("m2c-correct").props.children).toBe(c.useExcerpt);
    expect(ui.get("m2c-reject").props.children).toBe(c.rejectThisClue);
    ui.click("m2c-correct");
    expect(ui.get("m2c-correction").props.value).toBe("");
    expect(ui.get("m2c-save-correction").props.disabled).toBe(true);
    ui.click("m2c-cancel-correction");
    expect(ui.get("m2c-candidate-scope").props.children).toBe(c.originalReference);
    expect(operations.map((request) => request.operation)).toEqual(["candidate"]);

    ui.click("m2c-confirm");
    await vi.waitFor(() => {
      ui.render();
      expect(ui.get("m2c-generate-question").props).toMatchObject({ children: c.ask, className: "android-m2c__journey-primary", disabled: false });
    });
    expect(operations.map((request) => request.operation)).toEqual(["candidate", "confirm"]);
    expect(operations[1]).toMatchObject({ sourceId: source.entry.id, expectedSourceRevisionId: source.revisionId,
      artifactId: operations[0].artifactId, expectedArtifactRevisionId: "v5ar_pending" });
    expect(ui.get("m2c-candidate-scope").props.children).toBe(c.originalReference);
    expect(ui.get("m2c-clue-status").props.children).toBe(c.clueAdopted);
    expect(ui.find("m2c-candidate-text")).toBeUndefined();
    expect(ui.find("m2c-artifact-feedback")).toBeUndefined();
    expect(ui.store.generateQuestions).not.toHaveBeenCalled();
  });

  it("keeps the exact edited clue in the same card before and after adoption", async () => {
    const quote = "你寫下的原文:\n" + source.entry.body;
    const payload: EvidenceCandidate = { ...row.payload as EvidenceCandidate, text: quote, originalText: quote, status: "candidate" };
    const pending = { ...row, payload, reviewState: "pending", eligibilityState: "ineligible" };
    const ui = await component("zh-TW", [pending], true);
    const c = journeyCopy["zh-TW"];
    const excerpt = "Today I felt unsure and paused before replying.";
    ui.store.artifactMutate.mockImplementation(async (request) => {
      if (request.operation === "correct") {
        ui.setRows([{ ...pending, revisionId: "v5ar_excerpt", payload: { ...payload, text: request.text! } }]);
      } else if (request.operation === "confirm") {
        ui.setRows([{ ...pending, revisionId: "v5ar_adopted", reviewState: "confirmed", eligibilityState: "eligible",
          payload: { ...payload, text: excerpt, status: "confirmed" } }]);
      } else throw new Error("Unexpected test operation");
      return { requestId: request.requestId, acknowledgement: "committed" };
    });
    ui.click("m2c-correct");
    expect(ui.store.artifactMutate).not.toHaveBeenCalled();
    ui.get("m2c-correction").props.onChange({ target: { value: excerpt } });
    ui.render();
    ui.click("m2c-save-correction");
    await vi.waitFor(() => {
      ui.render(); expect(ui.get("m2c-candidate-text").props.children).toBe(excerpt);
    });
    expect(ui.find("m2c-candidate-scope")).toBeUndefined();
    expect(ui.get("m2c-clue-title").props.children).toBe(c.editedClueTitle);
    expect(ui.get("m2c-clue-status").props.children).toBe(c.clueToReview);
    expect(ui.get("m2c-correct").props.children).toBe(c.editThisClue);
    expect(ui.get("m2c-confirm").props.children).toBe(c.useThisClue);
    expect(ui.find("m2c-generate-question")).toBeUndefined();
    const [[saved]] = ui.store.artifactMutate.mock.calls;
    expect(saved).toMatchObject({ operation: "correct", text: excerpt, sourceId: source.entry.id,
      expectedSourceRevisionId: source.revisionId, artifactId: row.id, expectedArtifactRevisionId: row.revisionId });

    ui.click("m2c-confirm");
    await vi.waitFor(() => {
      ui.render(); expect(ui.get("m2c-clue-status").props.children).toBe(c.clueAdopted);
    });
    expect(ui.get("m2c-clue-title").props.children).toBe(c.editedClueTitle);
    const card = elements(ui.get("m2c-journey")).find((e) => e.props["data-artifact-id"] === row.id)!;
    const html = renderToStaticMarkup(card as unknown as React.ReactElement);
    expect(html).toMatch(/<p [^>]*data-testid="m2c-candidate-text">Today I felt unsure and paused before replying\.<\/p><details/);
    expect(html).toContain(c.editedOrigin);
    expect(html).toContain(c.clueAdoptionLimit);
    expect(html).not.toContain(quote);
    expect(ui.find("m2c-confirm")).toBeUndefined();
    expect(ui.get("m2c-generate-question").props.children).toBe(c.ask);
    expect(ui.store.artifactMutate.mock.calls[1][0]).toMatchObject({ operation: "confirm", artifactId: row.id,
      expectedArtifactRevisionId: "v5ar_excerpt", expectedSourceRevisionId: source.revisionId, text: null });
    expect(ui.store.generateQuestions).not.toHaveBeenCalled();
    expect((await ui.store.get()).entry.body).toBe(source.entry.body);
  });

  it("references the original once even when the saved clue is the bare original without a quote label", async () => {
    const ui = await component("zh-TW", [{ ...row, payload: { ...row.payload as EvidenceCandidate, text: source.entry.body } }], true);
    expect(ui.get("m2c-candidate-scope").props.children).toBe(journeyCopy["zh-TW"].originalReference);
    expect(ui.get("m2c-clue-status").props.children).toBe(journeyCopy["zh-TW"].clueAdopted);
    expect(ui.find("m2c-candidate-text")).toBeUndefined();
    expect(ui.store.artifactMutate).not.toHaveBeenCalled();
  });

  it("retains failure feedback and does not offer another creation when the result is uncertain", async () => {
    const ui = await component("zh-TW", [], true);
    ui.store.artifactMutate.mockRejectedValueOnce(new Error("private backend detail"));
    ui.click("m2c-generate-candidate");
    await vi.waitFor(() => {
      ui.render(); expect(ui.get("m2c-artifact-feedback").props.children).toBe(journeyCopy["zh-TW"].failed);
    });
    expect(ui.find("m2c-generate-candidate")).toBeUndefined();
    expect(ui.find("m2c-generate-question")).toBeUndefined();
    expect(ui.store.artifactMutate).toHaveBeenCalledTimes(1);
  });
});
describe("Pilot Android prompt and response ownership", () => {
  it("labels a reopened Gemini question and its saved answer separately", async () => {
    const saved = savedPrompt();
    const prompt = saved.payload as import("../types/domain").ReflectionPrompt;
    const ui = await component("zh-TW", [row, { ...saved, payload: { ...prompt,
      promptProvenance: { ...prompt.promptProvenance, origin: "ai", provider: "gemini",
        model: "gemini-3.1-flash-lite", promptVersion: "android-gemini-reflection-v1" },
    } }], true);
    const html = renderToStaticMarkup(ui.get("m2c-journey") as unknown as React.ReactElement);
    expect(html).toContain('data-testid="m2c-question-origin">Gemini 生成');
    expect(html).toContain('<h4>由你撰寫</h4>');
    expect(html).toContain("Persisted original question?");
    expect(html).toContain("My saved words.");
  });

  it.each(["en", "zh-TW", "ja"] as const)("renders separate origin labels in %s", async (locale) => {
    const ui = await component(locale);
    const question = ui.get("m2c-question");
    const response = ui.get("m2c-saved-response");
    const c = journeyCopy[locale];
    const markup = (e: Element) => renderToStaticMarkup(e as unknown as React.ReactElement);
    const questionBlock = elements(ui.get("m2c-journey")).find((e) => e.props.className === "android-m2c__question-block")!;
    const responseBlock = elements(ui.get("m2c-journey")).find((e) => e.props.className === "android-m2c__authored-block")!;
    expect(markup(questionBlock)).toContain(`data-testid="m2c-question-origin">${c.demoQuestion}`);
    expect(elements(questionBlock)).toContain(question);
    expect(elements(questionBlock)).not.toContain(response);
    expect(elements(responseBlock)).toContain(response);
    expect(elements(responseBlock).some((e) => e.props.children === c.response)).toBe(true);
    expect(elements(ui.get("m2c-journey")).find((e) => e.props.id === "m2c-reflection-start")?.props.tabIndex).toBe(-1);
  });
  it("does not label unverified provenance as a demo or as the user's response", async () => {
    const existing = savedPrompt();
    const prompt = existing.payload as import("../types/domain").ReflectionPrompt;
    const payload = { ...prompt, promptProvenance: { ...prompt.promptProvenance, origin: "legacy_unknown" as const }, responseProvenance: undefined };
    const ui = await component("en", [row, { ...existing, payload }]);
    const question = elements(ui.get("m2c-journey")).find((e) => e.props.className === "android-m2c__question-block")!;
    const html = renderToStaticMarkup(question as unknown as React.ReactElement);
    expect(html).toContain(`data-testid="m2c-question-origin">${journeyCopy.en.unknownOrigin}`);
    expect(html).not.toContain(journeyCopy.en.demoQuestion);
    const reply = elements(ui.get("m2c-journey")).find((e) => e.props.className === "android-m2c__authored-block")!;
    expect(elements(reply).some((e) => e.props.children === journeyCopy.en.unknownOrigin)).toBe(true);
  });
});
const suggestedPrompt = (batch: QuestionBatchRequest, index: number): ArtifactSnapshot => {
  const { artifactId, evidence } = batch.items[index];
  return {
    id: artifactId, kind: "reflection", revisionId: "revision-" + artifactId, reviewState: "pending",
    lifecycleState: "active", eligibilityState: "ineligible",
    payload: {
      id: artifactId, sourceEntryId: batch.sourceId, sourceEvidenceIds: [evidence.artifactId],
      question: journeyCopy[batch.locale].question, status: "suggested",
      createdAt: batch.occurredAt, updatedAt: batch.occurredAt,
      promptProvenance: { origin: "local_mock", provider: "mock", model: null, sourceEntryId: batch.sourceId, sourceArtifactIds: [evidence.artifactId],
        harnessVersion: "harness-v1", promptVersion: "v1", generatedAt: batch.occurredAt },
    },
  };
};

describe("explicit first/append Reflection actions", () => {
  const second: ArtifactSnapshot = {
    ...row, id: "m2c-second-evidence", revisionId: "v5ar_second-evidence",
    payload: { ...row.payload!, id: "m2c-second-evidence" },
  };
  const generate = "m2c-generate-question";
  const feedback = "m2c-artifact-feedback";
  const drafts = "m2c-response-draft";

  it("initially creates a full batch, then adds while preserving the original draft and verified order", async () => {
    const ui = await component("zh-TW", [row, second]), c = journeyCopy["zh-TW"];
    expect(ui.get(generate).props.children).toBe(c.ask);
    expect(ui.store.generateQuestions).not.toHaveBeenCalled();
    const handler = ui.get(generate).props.onClick;
    handler(); handler(); ui.render();
    expect(ui.get(generate).props.disabled).toBe(true);
    await vi.waitFor(() => { ui.render(); expect(ui.get(feedback).props.children).toBe(c.saved); });
    const first = ui.store.generateQuestions.mock.calls[0][0];
    expect(ui.store.generateQuestions).toHaveBeenCalledTimes(1);
    expect(first.items.map((item) => item.evidence.artifactId)).toEqual([row.id, second.id]);
    expect(first.expectedReflectionSet).toBe("[]");
    expect(Object.isFrozen(first.items[0].evidence)).toBe(true);
    expect(ui.get(generate).props.children).toBe(c.addQuestion);
    const original = ui.all(drafts).map((e) => e.props.id);
    ui.all(drafts)[0].props.onChange({ target: { value: "仍未儲存的文字" } }); ui.render();
    ui.click(generate);
    await vi.waitFor(() => { ui.render(); expect(ui.get(feedback).props.children).toBe(c.saved); });
    const added = ui.store.generateQuestions.mock.calls[1][0];
    expect(added.requestId).not.toBe(first.requestId);
    expect(added.expectedReflectionSet).toContain(first.items[0].artifactId);
    expect(JSON.stringify(added)).not.toContain("仍未儲存的文字");
    expect(ui.all(drafts).slice(0, 2).map((e) => e.props.id)).toEqual(original);
    expect(ui.all(drafts).map((e) => e.props.value)).toEqual(["仍未儲存的文字", "", "", ""]);
  });

  it("waits for complete reread and reconciles the same request without discarding existing drafts", async () => {
    const ui = await component("en", [row, second, savedPrompt()]);
    let complete!: ArtifactSnapshot[];
    ui.store.generateQuestions.mockImplementationOnce(async (r) => {
      complete = [row, second, savedPrompt(), ...r.items.map((_, i) => suggestedPrompt(r, i))];
      ui.setRows(complete.slice(0, -1));
      return { requestId: r.requestId, acknowledgement: "committed" };
    });
    ui.click(generate);
    await vi.waitFor(() => { ui.render(); expect(ui.get(feedback).props.children).toBe(journeyCopy.en.failed); });
    expect(ui.get("m2c-saved-response").props.children).toBe("My saved words.");
    expect(ui.all(drafts)).toHaveLength(0);
    ui.setRows(complete);
    ui.store.generateQuestions.mockImplementationOnce(async (r) => ({ requestId: r.requestId, acknowledgement: "alreadyCommitted" }));
    ui.click("m2c-artifact-reconcile");
    await vi.waitFor(() => { ui.render(); expect(ui.get(feedback).props.children).toBe(journeyCopy.en.saved); });
    expect(ui.store.generateQuestions.mock.calls[1][0]).toBe(ui.store.generateQuestions.mock.calls[0][0]);
    expect(ui.all(drafts)).toHaveLength(2);
    ui.all(drafts)[0].props.onChange({ target: { value: "Keep this draft" } }); ui.render();
    ui.store.generateQuestions.mockRejectedValueOnce(new Error("m2c_stale_revision_preserved"));
    ui.click(generate);
    await vi.waitFor(() => { ui.render(); expect(ui.get(feedback).props.children).toBe(journeyCopy.en.conflict); });
    expect(ui.all(drafts)[0].props.value).toBe("Keep this draft");
    expect(ui.find("m2c-artifact-reconcile")).toBeUndefined();
    expect(ui.get("m2c-artifact-reopen").props.children).toBe(journeyCopy.en.discardDrafts);
  });
});

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
