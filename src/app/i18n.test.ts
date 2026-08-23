import { describe, expect, it } from "vitest";
import { uiText } from "./i18n";
describe("Context Recovery localization", () => {
  it("contains natural UTF-8 copy in all supported languages", () => {
    expect(uiText.en.recoveryQuestion).toContain("what happened");
    expect(uiText["zh-TW"].recoveryQuestion).toContain("發生了什麼");
    expect(uiText.ja.recoveryQuestion).toContain("何が起きたのか");
  });
  it("discloses Context Recovery persistence failure calmly and equivalently", () => {
    expect(uiText.en.recoveryMutationFailed).toContain("not saved");
    expect(uiText.en.recoveryMutationFailed).toContain("unchanged");
    expect(uiText["zh-TW"].recoveryMutationFailed).toContain("未被儲存");
    expect(uiText["zh-TW"].recoveryMutationFailed).toContain("狀態不變");
    expect(uiText.ja.recoveryMutationFailed).toContain("保存されませんでした");
    expect(uiText.ja.recoveryMutationFailed).toContain("変更していません");
  });
  it("contains no question-mark replacement corruption", () => {
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      const copy = uiText[locale];
      for (const value of [copy.recoveryTitle, copy.recoveryNote, copy.recoveryQuestion]) expect(value).not.toMatch(/\?{4,}/u);
    }
  });
  it("explains why saved clues and reflections do not replace missing event context", () => {
    expect(uiText.en.patternContextLimited).toContain("event context");
    expect(uiText.en.patternContextLimited).toContain("saved reflections still count");
    expect(uiText["zh-TW"].patternContextLimited).toContain("事件脈絡");
    expect(uiText["zh-TW"].patternContextLimited).toContain("都有效");
    expect(uiText.ja.patternContextLimited).toContain("出来事そのもの");
    expect(uiText.ja.patternContextLimited).toContain("有効");
    expect(uiText.en.addContext).toContain("unlock");
    expect(uiText["zh-TW"].addContext).toContain("解鎖");
    expect(uiText.ja.addContext).toContain("利用可能");
  });
  it("distinguishes the three reflection stages and does not imply that clues unlock patterns", () => {
    expect(uiText.en.reflectionFlowGuide).toContain("Three distinct stages");
    expect(uiText.en.evidenceNextReady).toContain("does not mean Stage 3");
    expect(uiText["zh-TW"].reflectionFlowGuide).toContain("三個不同階段");
    expect(uiText["zh-TW"].evidenceNextReady).toContain("不代表第 3 階段");
    expect(uiText.ja.reflectionFlowGuide).toContain("三つの異なる段階");
    expect(uiText.ja.evidenceNextReady).toContain("意味ではなく");
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      expect(uiText[locale].evidencePurpose).toBeTruthy();
      expect(uiText[locale].reflectionPurpose).toBeTruthy();
      expect(uiText[locale].patternPurpose).toBeTruthy();
    }
  });
  it("offers a top local-history shortcut without implying selection or consent", () => {
    expect(uiText.en.historicalContextShortcutBody).toContain("does not select or send");
    expect(uiText["zh-TW"].historicalContextShortcutBody).toContain("不代表同意傳送");
    expect(uiText.ja.historicalContextShortcutBody).toContain("送信同意も行われません");
  });
  it("states the local-only historical selection boundary in every language", () => {
    expect(uiText.en.historicalContextLocalOnly).toContain("has been sent to an AI provider");
    expect(uiText["zh-TW"].historicalContextLocalOnly).toContain("尚未傳送給 AI 供應商");
    expect(uiText.ja.historicalContextLocalOnly).toContain("AI 提供元に送信されていません");
  });
  it("labels each local historical relevance origin in every language", () => {
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      const copy = uiText[locale];
      expect(copy.historicalContextMatchLocations).toBeTruthy();
      expect(copy.historicalContextExperienceMatch("term")).toContain("term");
      expect(copy.historicalContextEvidenceMatch("term")).toContain("term");
      expect(copy.historicalContextReflectionMatch("term")).toContain("term");
    }
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
  it("keeps the database readiness inspector explicit and read-only without granting migration authority", () => {
    expect(uiText.en.databaseReadinessIntro).toContain("Check now");
    expect(uiText["zh-TW"].databaseReadinessIntro).toContain("立即檢查");
    expect(uiText.ja.databaseReadinessIntro).toContain("今すぐ確認");
    expect(uiText.en.databaseReadinessNoAction).toContain("does not create, migrate, back up, restore, repair, clean up, or change");
    expect(uiText["zh-TW"].databaseReadinessNoAction).toContain("不會建立、遷移、備份、還原、修復、清理或變更");
    expect(uiText.ja.databaseReadinessNoAction).toContain("作成、移行、バックアップ、復元、修復、クリーンアップ、変更を行いません");
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      expect(uiText[locale].databaseReadinessRecoveryRequired).toBeTruthy();
      expect(uiText[locale].databaseReadinessQuiescenceUnknown).toBeTruthy();
      expect(uiText[locale].databaseReadinessExactV5).toBeTruthy();
    }
  });
  it("keeps ordinary schema-v5 migration explicit and distinct from the isolated Founder profile", () => {
    expect(uiText.en.ordinaryV5Title).toContain("local Life OS database");
    expect(uiText["zh-TW"].ordinaryV5Title).toContain("Life OS 本機資料庫");
    expect(uiText.ja.ordinaryV5Title).toContain("Life OS ローカルデータベース");
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      const copy = uiText[locale];
      expect(copy.ordinaryV5Intro).toMatch(/schema(?:-| )v4/u);
      expect(copy.ordinaryV5Purpose).toBeTruthy();
      expect(copy.ordinaryV5BackupSensitivity).toBeTruthy();
      expect(copy.ordinaryV5OlderBinaryBoundary).toMatch(/schema(?:-| )v4/u);
      expect(copy.ordinaryV5BackupPurpose).not.toContain("Founder");
      expect(copy.founderV5CancelBoundary).toBeTruthy();
      expect(copy.founderV5ProviderBoundary).toBeTruthy();
    }
    expect(uiText.en.ordinaryV5Purpose).toContain("does not make AI output more true or authoritative");
    expect(uiText["zh-TW"].ordinaryV5BackupSensitivity).toContain("相同的本機個人資料");
    expect(uiText.ja.ordinaryV5OlderBinaryBoundary).toContain("書き込みを拒否");
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
  it("does not leak canonical English domain labels into the ordinary Chinese or Japanese flow", () => {
    const ordinaryKeys = [
      "subtitle",
      "currentSession",
      "summaryNote",
      "evidenceStep",
      "generateEvidence",
      "reflectionStep",
      "generateReflection",
      "patternStep",
      "generatePattern",
      "currentReflection",
      "olderReflection",
      "nextActionLabel",
      "completionStep",
      "completionTitle",
      "completionEvidenceLabel",
      "completionReflectionLabel",
      "completionOptionalLabel",
      "completionCanRest",
    ] as const;
    const canonicalEnglish = /\b(?:Experience|Evidence|Reflection|Pattern|candidate|hypothesis|session|review|provenance|artifact)\b/i;
    for (const locale of ["zh-TW", "ja"] as const) {
      for (const key of ordinaryKeys) {
        expect(uiText[locale][key]).not.toMatch(canonicalEnglish);
      }
    }
  });

  it("keeps the completion journey equivalent in every supported language", () => {
    for (const locale of ["en", "zh-TW", "ja"] as const) {
      const copy = uiText[locale];
      expect(copy.completionTitle).toBeTruthy();
      expect(copy.completionIntro).toBeTruthy();
      expect(copy.completionUserResponseLabel).toBeTruthy();
      expect(copy.completionPatternAbsent).toBeTruthy();
      expect(copy.completionHistoricalAbsent).toBeTruthy();
      expect(copy.completionCanRest).toBeTruthy();
      expect(copy.completionRecordAnother).toBeTruthy();
      expect(copy.nextActionAnswerContext).toBeTruthy();
      expect(copy.nextActionSaveReflection).toBeTruthy();
      expect(copy.nextActionReviewCompletion).toBeTruthy();
    }
  });
});
