import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it, vi } from "vitest";
import { AndroidM2BApp, copy, DraftStatus, OperationStatus } from "./AndroidM2BApp";
import { createId, lifecycleRequest, nextTimestamp, sha256, m2bStore } from "./androidM2BStore";
import type { Snapshot } from "./androidM2BStore";

const invoke = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke }));
const snapshot: Snapshot = {
  entry: { id: "m2b-synthetic-001", body: "old", createdAt: "2026-10-03T00:00:00.000Z", updatedAt: "2026-10-03T00:00:00.000Z", userEditable: true },
  revisionId: "v5sr_exact", revisionNumber: 1, predecessorRevisionId: null, authorship: "user",
};

describe("Android M2-B synthetic lifecycle", () => {
  it.each([
    ["en", "New moment — not saved", "Changes — not saved", "Moment saved and reopened", "Changes saved and reopened", "Moment deleted; no active text remains"],
    ["zh-TW", "新片刻尚未儲存", "編輯內容尚未儲存", "這個片刻已儲存並重新讀取確認", "修改已儲存並重新讀取確認", "這個片刻已刪除；不再有可開啟的文字"],
    ["ja", "新しい瞬間はまだ保存していません", "変更はまだ保存していません", "この瞬間を保存し、読み直して確認しました", "変更を保存し、読み直して確認しました", "この瞬間を削除しました。開ける文章は残っていません"],
  ] as const)("pins status copy to the correct operation in %s", (locale, draft, edit, saved, changed, deleted) => {
    expect([copy[locale].unsaved, copy[locale].editUnsaved, copy[locale].saved, copy[locale].changesSaved, copy[locale].deleted]).toEqual([draft, edit, saved, changed, deleted]);
  });
  it.each(["en", "zh-TW", "ja"] as const)("renders contextual unsaved and verified operation feedback without hiding abnormal states in %s", (locale) => {
    const c = copy[locale];
    expect(renderToStaticMarkup(<OperationStatus state="draft" c={c} />)).toBe("");
    expect(renderToStaticMarkup(<DraftStatus kind="new" visible={false} c={c} />)).toBe("");
    expect(renderToStaticMarkup(<DraftStatus kind="edit" visible={false} c={c} />)).toBe("");
    expect(renderToStaticMarkup(<DraftStatus kind="new" visible c={c} />)).toContain(c.unsaved);
    expect(renderToStaticMarkup(<DraftStatus kind="edit" visible c={c} />)).toContain(c.editUnsaved);
    for (const [state, text] of [["saving", c.saving], ["failed", c.failed], ["conflict", c.conflict], ["deleted", c.deleted], ["saved", c.saved]]) {
      const html = renderToStaticMarkup(<OperationStatus state={state} c={c} />);
      expect(html).toContain('role="status"'); expect(html).toContain(text); expect(html).not.toContain(c.unsaved);
    }
    expect(renderToStaticMarkup(<OperationStatus state="saved" revisionNumber={2} c={c} />)).toContain(c.changesSaved);
    expect(c.saved).not.toBe(c.changesSaved); expect(c.unsaved).not.toBe(c.editUnsaved);
  });
  it("does not render success or saved content before storage readiness", () => {
    const html = renderToStaticMarkup(<AndroidM2BApp />);
    expect(html).toContain('data-storage-state="loading"'); expect(html).not.toContain("Saved and reopened");
  });
  it.each(["en", "zh-TW", "ja"] as const)("discloses revision retention and logical deletion in %s", (locale) => {
    expect(copy[locale].retention).toMatch(/Prior text|舊版文字|以前の文章/);
    expect(copy[locale].retention).toMatch(/secure erasure|安全抹除|物理消去/);
    expect(copy[locale].limits).toContain("com.lifeos.review.m2b");
    expect(copy[locale].limits).toContain("directFreshV5");
    expect(copy[locale].cancel).toBeTruthy();
  });
  it.each([
    ["en", "Versions of this moment", "About this test build", "Delete this moment? This will remove its current text and the text of previously saved versions from the app."],
    ["zh-TW", "這個片刻的版本", "關於此測試版", "要刪除這個片刻嗎？這會刪除它目前的文字，以及先前儲存版本的文字。"],
    ["ja", "この瞬間のバージョン", "このテスト版について", "この瞬間を削除しますか？現在の文章と、以前に保存した版の文章がアプリから削除されます。"],
  ] as const)("keeps distinct disclosures and plain complete deletion consequences in %s", (locale, revision, build, confirm) => {
    expect(copy[locale].revisionDetails).toBe(revision); expect(copy[locale].buildDetails).toBe(build);
    expect(revision).not.toBe(build); expect(copy[locale].confirm).toBe(confirm);
    expect(confirm).not.toMatch(/metadata|lifecycle|physical|SQLite|生命週期|識別碼|安全抹除|ダイジェスト|物理消去/);
    // Technical truth remains available in optional disclosure, not hidden or
    // converted into an unsupported secure-erasure guarantee.
    expect(copy[locale].retention).toMatch(/IDs|識別碼|ID/);
    expect(copy[locale].retention).toMatch(/No secure erasure guarantee|不保證|保証しません/);
  });
  it("binds exact CJK/multiline content, predecessor, operation and time to an immutable retry identity", async () => {
    const request = await lifecycleRequest(snapshot, "  合成\nテスト  ", "2026-10-03T00:00:01.000Z");
    expect(Object.isFrozen(request)).toBe(true);
    expect(request.requestId).toMatch(/^m2b_[a-f0-9]{64}$/);
    expect(request).toEqual(await lifecycleRequest(snapshot, request.body, request.occurredAt));
    expect((await lifecycleRequest(snapshot, "changed", request.occurredAt)).requestId).not.toBe(request.requestId);
    expect((await lifecycleRequest({ ...snapshot, revisionId: "v5sr_other" }, request.body, request.occurredAt)).requestId).not.toBe(request.requestId);
    expect((await lifecycleRequest(snapshot, null, request.occurredAt)).requestId).not.toBe(request.requestId);
    expect(request.expectedRevisionId).toBe(snapshot.revisionId);
    expect(await sha256("abc")).toBe("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
  });
  it("same-content edits still produce update requests, not silent no-ops", async () => {
    const request = await lifecycleRequest(snapshot, snapshot.entry.body);
    expect(request.operation).toBe("update"); expect(request.body).toBe("old");
  });
  it("delete descriptors contain no raw source body", async () => {
    const request = await lifecycleRequest({ ...snapshot, entry: { ...snapshot.entry, body: "source-secret-canary" } }, null);
    expect(JSON.stringify(request)).not.toContain("source-secret-canary"); expect(request.body).toBeNull();
  });
  it("advances the canonical timestamp despite clock skew", () => {
    expect(nextTimestamp(snapshot.entry.updatedAt, 0)).toBe("2026-10-03T00:00:00.001Z");
    expect(createId()).toMatch(/^m2b_[a-z0-9-]{36}$/);
  });
  it("only calls fixed synthetic IPC commands and forwards the same retry descriptor", async () => {
    invoke.mockReset(); invoke.mockResolvedValue({ acknowledgement: "alreadyCommitted" });
    const request = await lifecycleRequest(snapshot, null);
    await m2bStore.mutate(request); await m2bStore.mutate(request);
    expect(invoke.mock.calls).toEqual([["m2b_mutate_experience", { request, debugPhase: undefined }], ["m2b_mutate_experience", { request, debugPhase: undefined }]]);
    await m2bStore.get(snapshot.entry.id); expect(invoke).toHaveBeenLastCalledWith("m2b_get_experience", { id: snapshot.entry.id });
    await m2bStore.writeLocale("ja"); expect(invoke).toHaveBeenLastCalledWith("m2b_set_locale_preference", { locale: "ja" });
  });
});
