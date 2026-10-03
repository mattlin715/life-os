import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import {
  ANDROID_M2A_LOCALE_PREFERENCE_FILE,
  AndroidM2AApp,
  androidM2ACopy,
  createM2ARequestId,
  normalizeAndroidM2ALocale,
} from "./AndroidM2AApp";
import { androidM2ASupportedOperations } from "./androidM2AStore";

describe("Android M2-A disposable persistence review surface", () => {
  it("renders a truthful loading boundary before storage readiness", () => {
    const html = renderToStaticMarkup(<AndroidM2AApp />);
    expect(html).toContain('data-runtime="android-m2a-disposable"');
    expect(html).toContain('data-storage-state="loading"');
    expect(html).not.toContain("Committed —");
  });

  it.each(["en", "zh-TW", "ja"] as const)("keeps scope and durability limits explicit in %s", (locale) => {
    const copy = androidM2ACopy[locale];
    expect(copy.boundaries).toHaveLength(4);
    expect(copy.boundaries.join(" ")).toContain("com.lifeos.review.m2a");
    expect(copy.supported).toContain("createExperience");
    expect(copy.unsupported.length).toBeGreaterThan(20);
  });

  it("exposes only the honest LocalEvidenceStore subset", () => {
    expect(androidM2ASupportedOperations).toEqual(["createExperience", "listExperiences", "getExperience"]);
  });

  it("creates stable, backend-valid idempotency identifiers", () => {
    expect(createM2ARequestId()).toMatch(/^m2a_[A-Za-z0-9_-]{5,}$/);
  });

  it.each(["en", "zh-TW", "ja"] as const)("accepts the bounded non-content locale preference %s", (locale) => {
    expect(normalizeAndroidM2ALocale(locale)).toBe(locale);
    expect(ANDROID_M2A_LOCALE_PREFERENCE_FILE).toBe("android-m2a-locale.pref");
  });

  it("fails safely to English when the locale preference is absent or invalid", () => {
    expect(normalizeAndroidM2ALocale(null)).toBe("en");
    expect(normalizeAndroidM2ALocale("fr")).toBe("en");
  });

  it("aligns Traditional Chinese and Japanese product terms without widening M2-A", () => {
    expect(androidM2ACopy["zh-TW"].title).toContain("片刻");
    expect(androidM2ACopy["zh-TW"].save).toContain("儲存");
    expect(androidM2ACopy.ja.title).toContain("瞬間");
    expect(androidM2ACopy.ja.open).toContain("原文");
    for (const locale of ["zh-TW", "ja"] as const) {
      expect(androidM2ACopy[locale].synthetic).toMatch(/合成/);
      expect(androidM2ACopy[locale].boundaries.join(" ")).toMatch(/AI/);
    }
  });
});
