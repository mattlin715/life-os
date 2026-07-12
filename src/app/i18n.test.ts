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
});
