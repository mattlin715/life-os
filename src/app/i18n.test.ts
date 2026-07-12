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
});
