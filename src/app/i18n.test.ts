import { describe, expect, it } from "vitest";
import { uiText } from "./i18n";
describe("Context Recovery localization", () => {
  it("contains natural UTF-8 copy in all supported languages", () => {
    expect(uiText.en.recoveryQuestion).toContain("what happened");
    expect(uiText["zh-TW"].recoveryQuestion).toContain("發生了什麼");
    expect(uiText.ja.recoveryQuestion).toContain("何が起きたのか");
  });
  it("contains no question-mark replacement corruption", () => {
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      const copy = uiText[locale];
      for (const value of [copy.recoveryTitle, copy.recoveryNote, copy.recoveryQuestion]) expect(value).not.toMatch(/\?{4,}/u);
    }
  });
  it("has readable Pattern availability explanations and an optional Add Context action", () => {
    expect(uiText.en.patternContextLimited).toContain("event context");
    expect(uiText["zh-TW"].patternContextLimited).toContain("事件脈絡");
    expect(uiText.ja.patternContextLimited).toContain("出来事の文脈");
    expect(uiText.en.addContext).toBe("Add context");
    expect(uiText["zh-TW"].addContext).toBe("補充情境");
    expect(uiText.ja.addContext).toBe("状況を少し補足する");
  });
  it("states the local-only historical selection boundary in every language", () => {
    expect(uiText.en.historicalContextLocalOnly).toContain("has been sent to an AI provider");
    expect(uiText["zh-TW"].historicalContextLocalOnly).toContain("尚未傳送給 AI provider");
    expect(uiText.ja.historicalContextLocalOnly).toContain("AI provider に送信されていません");
  });
  it("provides a localized unavailable-date fallback", () => {
    expect(uiText.en.dateUnavailable).toBe("Date unavailable");
    expect(uiText["zh-TW"].dateUnavailable).toBe("日期無法使用");
    expect(uiText.ja.dateUnavailable).toBe("日付を表示できません");
  });
  it("keeps the local database refusal boundary equivalent in every language", () => {
    expect(uiText.en.databaseNewerSchema(5, 4)).toContain("Nothing was changed");
    expect(uiText["zh-TW"].databaseNewerSchema(5, 4)).toContain("資料未被修改");
    expect(uiText.ja.databaseNewerSchema(5, 4)).toContain("データは変更されていません");
    expect(uiText.en.databaseBlockedAction).toContain("will not repair, downgrade, or write");
    expect(uiText["zh-TW"].databaseBlockedAction).toContain("不會自動修復、降版或寫入");
    expect(uiText.ja.databaseBlockedAction).toContain("自動修復、ダウングレード、書き込みは行いません");
  });
});
