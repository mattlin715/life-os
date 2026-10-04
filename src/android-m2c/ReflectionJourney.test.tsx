import { describe, expect, it } from "vitest";
import { currentEvidence, localQuestion, reflectionPacket } from "./ReflectionJourney";
import { artifactRequest } from "./androidM2CStore";
import type { ArtifactSnapshot, Snapshot } from "./androidM2CStore";
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
