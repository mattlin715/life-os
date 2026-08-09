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
  it("keeps the database readiness inspector explicit, read-only, and unavailable for migration in every language", () => {
    expect(uiText.en.databaseReadinessIntro).toContain("Check now");
    expect(uiText["zh-TW"].databaseReadinessIntro).toContain("立即檢查");
    expect(uiText.ja.databaseReadinessIntro).toContain("今すぐ確認");
    expect(uiText.en.databaseReadinessNoAction).toContain("does not create, migrate, back up, restore, repair, clean up, or change");
    expect(uiText["zh-TW"].databaseReadinessNoAction).toContain("不會建立、遷移、備份、還原、修復、清理或變更");
    expect(uiText.ja.databaseReadinessNoAction).toContain("作成、移行、バックアップ、復元、修復、クリーンアップ、変更を行いません");
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      expect(uiText[locale].databaseReadinessRecoveryRequired).toBeTruthy();
      expect(uiText[locale].databaseReadinessQuiescenceUnknown).toBeTruthy();
    }
  });
  it("keeps saved-date retrieval explicit, inclusive, and fail-closed in every language", () => {
    expect(uiText.en.historicalSavedDateFilterLabel).toContain("saved date");
    expect(uiText["zh-TW"].historicalSavedDateFilterLabel).toContain("儲存日期");
    expect(uiText.ja.historicalSavedDateFilterLabel).toContain("保存日");
    expect(uiText.en.historicalSavedDateReason("2026-01-01", "2026-01-02", "Asia/Tokyo")).toContain("inclusive");
    expect(uiText["zh-TW"].historicalSavedDateReason("2026-01-01", "2026-01-02", "Asia/Tokyo")).toContain("包含起訖日");
    expect(uiText.ja.historicalSavedDateReason("2026-01-01", "2026-01-02", "Asia/Tokyo")).toContain("両端を含む");
    expect(uiText.en.historicalSavedDateInverted).toContain("No history was retrieved");
    expect(uiText["zh-TW"].historicalSavedDateInverted).toContain("未檢索任何歷史紀錄");
    expect(uiText.ja.historicalSavedDateInverted).toContain("履歴は取得されていません");
  });
  it("keeps the provenance inspector local, read-only, explicit, and fail-closed in every language", () => {
    expect(uiText.en.historicalProvenanceLocalOnly).toContain("read-only");
    expect(uiText["zh-TW"].historicalProvenanceLocalOnly).toContain("唯讀");
    expect(uiText.ja.historicalProvenanceLocalOnly).toContain("読み取り専用");
    expect(uiText.en.historicalProvenanceExactContentWarning).toContain("does not send");
    expect(uiText["zh-TW"].historicalProvenanceExactContentWarning).toContain("不會傳送");
    expect(uiText.ja.historicalProvenanceExactContentWarning).toContain("送信");
    expect(uiText.en.historicalProvenanceInvalid).toContain("nothing was changed");
    expect(uiText["zh-TW"].historicalProvenanceInvalid).toContain("沒有變更任何資料");
    expect(uiText.ja.historicalProvenanceInvalid).toContain("データの変更は行っていません");
  });
});
