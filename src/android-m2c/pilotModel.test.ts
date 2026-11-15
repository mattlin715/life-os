import { describe, expect, it, vi } from "vitest";
import { createTrial, type ModelPreview, type TrialState } from "./pilotModel";

const preview: ModelPreview = {
  sourceId: "local-source", sourceRevisionId: "current-source", clueId: "local-evidence",
  clueRevisionId: "current-evidence", text: "  The synthetic clue.\nKeep spacing. ", locale: "zh-TW",
};

function fixture() {
  let state: TrialState = { step: "idle" };
  const model = {
    ask: vi.fn(async (_request: ModelPreview & { syntheticConfirmed: true }) => ({ ticket: "ticket-1", question: "那一刻你留意到什麼？" })),
    save: vi.fn(async (_ticket: string, _answer: string) => {}),
  };
  const trial = createTrial(model, (next) => { state = next; });
  return { trial, model, current: () => state };
}

describe("Founder-only model trial", () => {
  it("sends nothing without confirmation; sends the immutable exact clue and no other data", async () => {
    const { trial, model } = fixture();
    const extra = { ...preview, note: "never sent", history: ["never sent"], root: "never sent" };
    trial.preview(extra);
    await trial.confirm(false, () => true);
    expect(model.ask).not.toHaveBeenCalled();
    extra.text = "changed after preview";
    await trial.confirm(true, () => true);
    expect(model.ask.mock.calls).toEqual([[{ ...preview, syntheticConfirmed: true }]]);
  });

  it("drops a late reply after leaving and admits only one send", async () => {
    const { trial, model, current } = fixture();
    let resolve!: (value: { ticket: string; question: string }) => void;
    model.ask.mockReturnValueOnce(new Promise((done) => { resolve = done; }));
    trial.preview(preview);
    const pending = trial.confirm(true, () => true);
    await trial.confirm(true, () => true);
    trial.leave();
    resolve({ ticket: "late", question: "Must not show?" });
    await pending;
    expect(model.ask).toHaveBeenCalledTimes(1);
    expect(current()).toEqual({ step: "idle" });
  });

  it("refuses a stale preview before dispatch and another that changes while waiting", async () => {
    const { trial, model, current } = fixture();
    trial.preview(preview);
    await trial.confirm(true, () => false);
    expect(model.ask).not.toHaveBeenCalled();
    let matches = true;
    trial.preview(preview);
    const pending = trial.confirm(true, () => matches);
    matches = false;
    await pending;
    expect(current()).toEqual({ step: "idle" });
  });

  it("preserves the draft across settings; explicit Save does not call the provider again", async () => {
    const { trial, model, current } = fixture();
    const readBack = vi.fn(async () => {});
    trial.preview(preview);
    await trial.confirm(true, () => true);
    trial.answer("我的合成回答");
    trial.leave();
    expect(current()).toMatchObject({ step: "result", answer: "我的合成回答" });
    await trial.save(readBack);
    expect(model.ask).toHaveBeenCalledTimes(1);
    expect(model.save.mock.calls).toEqual([["ticket-1", "我的合成回答"]]);
    expect(readBack).toHaveBeenCalledExactlyOnceWith(expect.objectContaining({
      ticket: "ticket-1", question: "那一刻你留意到什麼？", answer: "我的合成回答",
    }));
    expect(current()).toEqual({ step: "idle" });
  });

  it("keeps the exact answer for retry after uncertain save and never retries generation", async () => {
    const { trial, model, current } = fixture();
    trial.preview(preview);
    await trial.confirm(true, () => true);
    trial.answer("First answer.");
    const readBack = vi.fn(async () => { throw new Error("private payload canary"); });
    await trial.save(readBack);
    expect(current()).toMatchObject({ step: "result", error: "save_unconfirmed", answer: "First answer." });
    trial.answer("Replacement.");
    await trial.save(async () => {});
    expect(model.save.mock.calls).toEqual([["ticket-1", "First answer."], ["ticket-1", "First answer."]]);
    expect(model.ask).toHaveBeenCalledTimes(1);
  });

  it("shows a bounded error rather than retrying or substituting a mock result", async () => {
    const { trial, model, current } = fixture();
    model.ask.mockRejectedValueOnce("private payload canary");
    trial.preview(preview);
    await trial.confirm(true, () => true);
    expect(current()).toEqual({ step: "error", error: "unavailable" });
    expect(model.ask).toHaveBeenCalledTimes(1);
  });

  it("Skip does not save the question or the draft", async () => {
    const { trial, model, current } = fixture();
    trial.preview(preview);
    await trial.confirm(true, () => true);
    trial.answer("Discard this draft.");
    trial.reset();
    expect(current()).toEqual({ step: "idle" });
    expect(model.save).not.toHaveBeenCalled();
  });

  it("finishes a confirmed local Save while Settings temporarily hides the pane", async () => {
    const { trial, model, current } = fixture();
    let finish!: () => void;
    model.save.mockReturnValueOnce(new Promise<void>((done) => { finish = done; }));
    trial.preview(preview);
    await trial.confirm(true, () => true);
    trial.answer("The exact local answer.");
    const readBack = vi.fn(async () => {});
    const pending = trial.save(readBack);
    trial.leave();
    finish();
    await pending;
    expect(readBack).toHaveBeenCalledTimes(1);
    expect(current()).toEqual({ step: "idle" });
    expect(model.ask).toHaveBeenCalledTimes(1);
  });
});
